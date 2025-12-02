use crate::types::{MeterData, MetroEvent};
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
            eprintln!("Meter thread: Failed to create socket: {}", e);
            return;
        }
    };

    let bind_addr: SocketAddr = format!("0.0.0.0:{}", METER_PORT).parse().unwrap();
    if let Err(e) = socket.bind(&bind_addr.into()) {
        eprintln!("Meter thread: Failed to bind UDP socket on port {}: {}", METER_PORT, e);
        return;
    }

    let socket: UdpSocket = socket.into();

    if let Err(e) = socket.set_read_timeout(Some(Duration::from_millis(RECV_TIMEOUT_MS))) {
        eprintln!("Meter thread: Failed to set socket timeout: {}", e);
        return;
    }

    let mut meter_data = MeterData::default();
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
                                    eprintln!("Meter thread: Failed to send MeterUpdate: {}", e);
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
                eprintln!("Meter thread: Socket recv error: {}", e);
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

            if peak >= CLIP_THRESHOLD {
                meter_data.clip_l = true;
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
