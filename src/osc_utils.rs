use rosc::{OscBundle, OscMessage, OscPacket, OscTime};
use std::time::{Duration, SystemTime};

pub const OSC_LATENCY_MS: u64 = 50;

pub fn create_osc_timestamp(latency_ms: u64) -> OscTime {
    let now = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap_or(Duration::ZERO);

    let ntp_offset = Duration::from_secs(2208988800);
    let target_time = now + ntp_offset + Duration::from_millis(latency_ms);

    OscTime {
        seconds: target_time.as_secs() as u32,
        fractional: ((target_time.subsec_nanos() as f64 / 1e9) * (u32::MAX as f64)) as u32,
    }
}

pub fn create_bundle(messages: Vec<OscMessage>, latency_ms: u64) -> OscPacket {
    OscPacket::Bundle(OscBundle {
        timetag: create_osc_timestamp(latency_ms),
        content: messages.into_iter().map(OscPacket::Message).collect(),
    })
}
