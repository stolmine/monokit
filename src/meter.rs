use crate::types::{CpuData, MeterData, MetroEvent, ScopeData, SpectrumData, SCOPE_SAMPLES, SPECTRUM_BANDS};
use rosc::{decoder, encoder, OscMessage, OscPacket, OscType};
use socket2::{Domain, Protocol, Socket, Type};
use std::net::{SocketAddr, UdpSocket};
use std::sync::mpsc;
use std::time::Duration;

const SCSYNTH_PORT: u16 = 57110;

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

    #[cfg(feature = "scsynth-direct")]
    eprintln!("[meter] Successfully bound to port {}", METER_PORT);

    let socket: UdpSocket = socket.into();

    if let Err(e) = socket.set_read_timeout(Some(Duration::from_millis(RECV_TIMEOUT_MS))) {
        let _ = event_tx.send(MetroEvent::Error(format!("ERROR: METER SOCKET TIMEOUT FAIL: {}", e)));
        return;
    }

    let mut meter_data = MeterData::default();
    let mut spectrum_data = SpectrumData::default();
    let mut buf = [0u8; 1024];

    // Track if we've completed initial boot - used to detect restart vs initial boot
    // We only send /notify on RESTART (when we receive /monokit/ready after initial boot)
    #[cfg(feature = "scsynth-direct")]
    let mut initial_boot_complete = false;

    // CPU monitoring: poll /status every 500ms (2Hz)
    #[cfg(feature = "scsynth-direct")]
    let mut last_status_poll = std::time::Instant::now();
    #[cfg(feature = "scsynth-direct")]
    let status_poll_interval = Duration::from_millis(500);

    loop {
        // Poll scsynth /status for CPU data (scsynth-direct mode only)
        #[cfg(feature = "scsynth-direct")]
        if last_status_poll.elapsed() >= status_poll_interval {
            let status_msg = OscMessage {
                addr: "/status".to_string(),
                args: vec![],
            };
            if let Ok(packet) = encoder::encode(&OscPacket::Message(status_msg)) {
                let _ = socket.send_to(&packet, format!("127.0.0.1:{}", SCSYNTH_PORT));
            }
            last_status_poll = std::time::Instant::now();
        }
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
                        } else if msg.addr == "/status.reply" {
                            // Parse CPU data from scsynth /status.reply
                            // Format: [unused, numUGens, numSynths, numGroups, numSynthDefs,
                            //          avgCPU, peakCPU, sampleRate, actualSampleRate]
                            #[cfg(feature = "scsynth-direct")]
                            if msg.args.len() >= 7 {
                                if let Some(cpu_data) = parse_status_reply_cpu(&msg.args) {
                                    if let Err(e) = event_tx.send(MetroEvent::CpuUpdate(cpu_data)) {
                                        let _ = event_tx.send(MetroEvent::Error(format!("ERROR: CPU UPDATE SEND FAIL: {}", e)));
                                        return;
                                    }
                                }
                            }
                        } else if msg.addr == "/monokit/scope" {
                            if let Some(scope_data) = parse_scope_message(&msg.args) {
                                if event_tx.send(MetroEvent::ScopeUpdate(scope_data)).is_err() {
                                    return;
                                }
                            }
                        } else if msg.addr == "/monokit/ready" {
                            // On RESTART only: send /notify from THIS socket (57121)
                            // so scsynth knows to send meter data here.
                            // On initial boot, the boot sequence already sent /notify.
                            #[cfg(feature = "scsynth-direct")]
                            if initial_boot_complete {
                                // This is a restart - we need to re-register with the new scsynth
                                let notify_msg = OscMessage {
                                    addr: "/notify".to_string(),
                                    args: vec![OscType::Int(1)],
                                };
                                if let Ok(packet) = encoder::encode(&OscPacket::Message(notify_msg)) {
                                    let _ = socket.send_to(&packet, format!("127.0.0.1:{}", SCSYNTH_PORT));
                                }
                            } else {
                                // First /monokit/ready - initial boot is now complete
                                initial_boot_complete = true;
                            }
                            let _ = event_tx.send(MetroEvent::ScReady);
                        } else if msg.addr == "/monokit/audio/out/list" {
                            if let Some(args) = &msg.args.get(0..) {
                                let mut devices = Vec::new();
                                let mut current = String::from("default");

                                for (i, arg) in args.iter().enumerate() {
                                    if let rosc::OscType::String(s) = arg {
                                        if i == 0 {
                                            current = s.clone();
                                        } else {
                                            devices.push(s.clone());
                                        }
                                    }
                                }

                                let _ = event_tx.send(MetroEvent::AudioDeviceList { current, devices });
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
    // Handle both formats:
    // - sclang forwarded: [channel, peak, rms] (3 args)
    // - scsynth direct (SendPeakRMS): [nodeID, channel, peak, rms] (4 args)
    let (channel_idx, peak_idx, rms_idx) = match args.len() {
        3 => (0, 1, 2),  // sclang mode
        4 => (1, 2, 3),  // scsynth-direct mode (skip nodeID at index 0)
        _ => return None,
    };

    // Channel can come as Int or Float from SC
    let channel = match &args[channel_idx] {
        OscType::Int(i) => *i,
        OscType::Float(f) => *f as i32,
        _ => return None,
    };

    // Peak and RMS are floats
    let peak = match &args[peak_idx] {
        OscType::Float(f) => *f,
        OscType::Double(d) => *d as f32,
        _ => return None,
    };

    let rms = match &args[rms_idx] {
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
    // Handle both formats:
    // - sclang forwarded: [band0...band14, clip0...clip14] (30 args) or just [band0...band14] (15 args)
    // - scsynth direct (SendReply): [nodeID, replyID, band0...band14, clip0...clip14] (32 args) or [nodeID, replyID, band0...band14] (17 args)

    let data_start = match args.len() {
        15 | 30 => 0,  // sclang mode
        17 | 32 => 2,  // scsynth-direct mode (skip nodeID and replyID)
        _ => return None,
    };

    let data_len = args.len() - data_start;

    // Backwards compatibility: accept just 15 bands (no clip data)
    if data_len == SPECTRUM_BANDS {
        let mut bands = [0.0f32; SPECTRUM_BANDS];
        for (i, arg) in args.iter().skip(data_start).enumerate() {
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
    if data_len != SPECTRUM_BANDS * 2 {
        return None;
    }

    let mut bands = [0.0f32; SPECTRUM_BANDS];
    for (i, arg) in args.iter().skip(data_start).take(SPECTRUM_BANDS).enumerate() {
        let value = match arg {
            OscType::Float(f) => *f,
            OscType::Double(d) => *d as f32,
            _ => return None,
        };
        bands[i] = value;
    }

    let mut clips = [false; SPECTRUM_BANDS];
    for (i, arg) in args.iter().skip(data_start + SPECTRUM_BANDS).enumerate() {
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

#[cfg(feature = "scsynth-direct")]
fn parse_status_reply_cpu(args: &[OscType]) -> Option<CpuData> {
    // /status.reply format: [unused, numUGens, numSynths, numGroups, numSynthDefs,
    //                        avgCPU, peakCPU, sampleRate, actualSampleRate]
    // avgCPU is at index 5, peakCPU at index 6
    if args.len() < 7 {
        return None;
    }

    let avg_cpu = match &args[5] {
        OscType::Float(f) => *f,
        OscType::Double(d) => *d as f32,
        _ => return None,
    };

    let peak_cpu = match &args[6] {
        OscType::Float(f) => *f,
        OscType::Double(d) => *d as f32,
        _ => return None,
    };

    Some(CpuData { avg_cpu, peak_cpu })
}

fn parse_scope_message(args: &[OscType]) -> Option<ScopeData> {
    // Handle both formats:
    // - sclang forwarded: [sample0...sample127] (128 args)
    // - scsynth direct (SendReply): [nodeID, replyID, sample0...sample127] (130 args)

    let data_start = match args.len() {
        128 => 0,   // sclang mode (SCOPE_SAMPLES)
        130 => 2,   // scsynth-direct mode (skip nodeID and replyID)
        _ => return None,
    };

    let mut samples = [0.0f32; SCOPE_SAMPLES];
    for (i, arg) in args.iter().skip(data_start).enumerate() {
        if i >= SCOPE_SAMPLES {
            break;
        }
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
