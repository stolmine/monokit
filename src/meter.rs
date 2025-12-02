use crate::types::{CpuData, MeterData, MetroEvent, ScopeData, SpectrumData, SCOPE_SAMPLES, SPECTRUM_BANDS};
use rosc::{decoder, OscPacket, OscType};
use socket2::{Domain, Protocol, Socket, Type};
use std::net::{SocketAddr, UdpSocket};
use std::sync::mpsc;
use std::time::Duration;

const METER_PORT: u16 = 57121;
const RECV_TIMEOUT_MS: u64 = 100;
const PEAK_HOLD_DECAY_RATE: f32 = 0.95;
const CLIP_THRESHOLD: f32 = 0.99;

pub fn meter_thread(event_tx: mpsc::Sender<MetroEvent>) {
    let socket = match Socket::new(Domain::IPV4, Type::DGRAM, Some(Protocol::UDP)) {
        Ok(s) => s,
        Err(e) => {
            let _ = event_tx.send(MetroEvent::Error(format!("ERROR: SOCKET CREATE FAILED: {}", e)));
            return;
        }
    };

    let bind_addr: SocketAddr = format!("0.0.0.0:{}", METER_PORT).parse().unwrap();
    if let Err(e) = socket.bind(&bind_addr.into()) {
        let _ = event_tx.send(MetroEvent::Error(format!("ERROR: METER BIND PORT {} FAIL: {}", METER_PORT, e)));
        return;
    }

    let socket: UdpSocket = socket.into();

    if let Err(e) = socket.set_read_timeout(Some(Duration::from_millis(RECV_TIMEOUT_MS))) {
        let _ = event_tx.send(MetroEvent::Error(format!("ERROR: METER SOCKET TIMEOUT FAIL: {}", e)));
        return;
    }

    let mut meter_data = MeterData::default();
    let mut spectrum_data = SpectrumData::default();
    let mut buf = [0u8; 1024];

    loop {
        match socket.recv_from(&mut buf) {
            Ok((size, _addr)) => {
                if let Ok((_, packet)) = decoder::decode_udp(&buf[..size]) {
                    if let OscPacket::Message(msg) = packet {
                        if msg.addr == "/monokit/meter" {
                            if let Some(update) = parse_meter_message(&msg.args) {
                                apply_meter_update(&mut meter_data, update);

                                if let Err(e) = event_tx.send(MetroEvent::MeterUpdate(meter_data.clone())) {
                                    let _ = event_tx.send(MetroEvent::Error(format!("ERROR: METER UPDATE SEND FAIL: {}", e)));
                                    return;
                                }
                            }
                        } else if msg.addr == "/monokit/spectrum" {
                            if let Some(update) = parse_spectrum_message(&msg.args) {
                                apply_spectrum_update(&mut spectrum_data, update);

                                if let Err(e) = event_tx.send(MetroEvent::SpectrumUpdate(spectrum_data.clone())) {
                                    let _ = event_tx.send(MetroEvent::Error(format!("ERROR: SPECTRUM UPDATE SEND FAIL: {}", e)));
                                    return;
                                }
                            }
                        } else if msg.addr == "/monokit/cpu" {
                            if let Some(cpu_data) = parse_cpu_message(&msg.args) {
                                if let Err(e) = event_tx.send(MetroEvent::CpuUpdate(cpu_data)) {
                                    let _ = event_tx.send(MetroEvent::Error(format!("ERROR: CPU UPDATE SEND FAIL: {}", e)));
                                    return;
                                }
                            }
                        } else if msg.addr == "/monokit/scope" {
                            if let Some(scope_data) = parse_scope_message(&msg.args) {
                                if event_tx.send(MetroEvent::ScopeUpdate(scope_data)).is_err() {
                                    return;
                                }
                            }
                        }
                    }
                }
            }
            Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock || e.kind() == std::io::ErrorKind::TimedOut => {
                decay_peak_holds(&mut meter_data);
            }
            Err(e) => {
                let _ = event_tx.send(MetroEvent::Error(format!("ERROR: METER SOCKET RECV: {}", e)));
                return;
            }
        }
    }
}

#[derive(Debug)]
struct MeterUpdate {
    channel: i32,
    peak: f32,
    rms: f32,
}

fn parse_meter_message(args: &[OscType]) -> Option<MeterUpdate> {
    if args.len() != 3 {
        return None;
    }

    // Channel can come as Int or Float from SC
    let channel = match &args[0] {
        OscType::Int(i) => *i,
        OscType::Float(f) => *f as i32,
        _ => return None,
    };

    // Peak and RMS are floats
    let peak = match &args[1] {
        OscType::Float(f) => *f,
        OscType::Double(d) => *d as f32,
        _ => return None,
    };

    let rms = match &args[2] {
        OscType::Float(f) => *f,
        OscType::Double(d) => *d as f32,
        _ => return None,
    };

    Some(MeterUpdate { channel, peak, rms })
}

const CLIP_RESET_THRESHOLD: f32 = 0.90;

fn apply_meter_update(meter_data: &mut MeterData, update: MeterUpdate) {
    let peak = update.peak.max(0.0).min(1.0);
    let rms = update.rms.max(0.0).min(1.0);

    match update.channel {
        1 => {
            meter_data.peak_l = peak;
            meter_data.rms_l = rms;

            if peak > meter_data.peak_hold_l {
                meter_data.peak_hold_l = peak;
            }

            // Set clip on threshold, reset when level drops
            if peak >= CLIP_THRESHOLD {
                meter_data.clip_l = true;
            } else if peak < CLIP_RESET_THRESHOLD {
                meter_data.clip_l = false;
            }
        }
        2 => {
            meter_data.peak_r = peak;
            meter_data.rms_r = rms;

            if peak > meter_data.peak_hold_r {
                meter_data.peak_hold_r = peak;
            }

            if peak >= CLIP_THRESHOLD {
                meter_data.clip_r = true;
            } else if peak < CLIP_RESET_THRESHOLD {
                meter_data.clip_r = false;
            }
        }
        _ => {}
    }
}

fn decay_peak_holds(meter_data: &mut MeterData) {
    meter_data.peak_hold_l *= PEAK_HOLD_DECAY_RATE;
    meter_data.peak_hold_r *= PEAK_HOLD_DECAY_RATE;

    if meter_data.peak_hold_l < meter_data.peak_l {
        meter_data.peak_hold_l = meter_data.peak_l;
    }
    if meter_data.peak_hold_r < meter_data.peak_r {
        meter_data.peak_hold_r = meter_data.peak_r;
    }
}

struct SpectrumUpdate {
    bands: [f32; SPECTRUM_BANDS],
    clips: [bool; SPECTRUM_BANDS],
}

fn parse_spectrum_message(args: &[OscType]) -> Option<SpectrumUpdate> {
    // Backwards compatibility: accept just 15 bands (no clip data)
    if args.len() == SPECTRUM_BANDS {
        let mut bands = [0.0f32; SPECTRUM_BANDS];
        for (i, arg) in args.iter().enumerate() {
            let value = match arg {
                OscType::Float(f) => *f,
                OscType::Double(d) => *d as f32,
                _ => return None,
            };
            bands[i] = value;
        }
        return Some(SpectrumUpdate {
            bands,
            clips: [false; SPECTRUM_BANDS],
        });
    }

    // New format: 15 floats (bands) + 15 ints (clip flags)
    if args.len() != SPECTRUM_BANDS * 2 {
        return None;
    }

    let mut bands = [0.0f32; SPECTRUM_BANDS];
    for (i, arg) in args.iter().take(SPECTRUM_BANDS).enumerate() {
        let value = match arg {
            OscType::Float(f) => *f,
            OscType::Double(d) => *d as f32,
            _ => return None,
        };
        bands[i] = value;
    }

    let mut clips = [false; SPECTRUM_BANDS];
    for (i, arg) in args.iter().skip(SPECTRUM_BANDS).enumerate() {
        let value = match arg {
            OscType::Int(v) => *v != 0,
            OscType::Float(f) => *f != 0.0,
            _ => return None,
        };
        clips[i] = value;
    }

    Some(SpectrumUpdate { bands, clips })
}

// Decay rate for smooth falloff (higher = slower decay)
const SPECTRUM_DECAY_RATE: f32 = 0.85;
const SPECTRUM_PEAK_HOLD_DECAY_RATE: f32 = 0.92;

fn apply_spectrum_update(spectrum_data: &mut SpectrumData, update: SpectrumUpdate) {
    for i in 0..SPECTRUM_BANDS {
        let new_value = update.bands[i].max(0.0).min(1.0);

        // Smooth falloff: if new value is higher, jump to it; otherwise decay slowly
        if new_value > spectrum_data.bands[i] {
            spectrum_data.bands[i] = new_value;
        } else {
            spectrum_data.bands[i] *= SPECTRUM_DECAY_RATE;
            // But don't go below the new value
            if spectrum_data.bands[i] < new_value {
                spectrum_data.bands[i] = new_value;
            }
        }

        // Peak hold (for potential future use)
        if new_value > spectrum_data.peak_hold[i] {
            spectrum_data.peak_hold[i] = new_value;
        } else {
            spectrum_data.peak_hold[i] *= SPECTRUM_PEAK_HOLD_DECAY_RATE;
        }

        // Set clip flags from update
        spectrum_data.clip[i] = update.clips[i];
    }
}

fn parse_cpu_message(args: &[OscType]) -> Option<CpuData> {
    if args.len() < 2 {
        return None;
    }

    let avg_cpu = match &args[0] {
        OscType::Float(f) => *f,
        OscType::Double(d) => *d as f32,
        _ => return None,
    };

    let peak_cpu = match &args[1] {
        OscType::Float(f) => *f,
        OscType::Double(d) => *d as f32,
        _ => return None,
    };

    Some(CpuData { avg_cpu, peak_cpu })
}

fn parse_scope_message(args: &[OscType]) -> Option<ScopeData> {
    if args.len() != SCOPE_SAMPLES {
        return None;
    }

    let mut samples = [0.0f32; SCOPE_SAMPLES];
    for (i, arg) in args.iter().enumerate() {
        let value = match arg {
            OscType::Float(f) => *f,
            OscType::Double(d) => *d as f32,
            OscType::Int(n) => *n as f32,
            _ => return None,
        };
        samples[i] = value.clamp(-1.0, 1.0);
    }

    Some(ScopeData { samples })
}
