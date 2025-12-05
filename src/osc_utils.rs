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

/// Create a bundle with two messages at different times
/// msg1 executes at latency_ms, msg2 executes at latency_ms + offset_ms
pub fn create_timed_pair_bundle(msg1: OscMessage, msg2: OscMessage, latency_ms: u64, offset_ms: u64) -> OscPacket {
    let bundle1 = OscBundle {
        timetag: create_osc_timestamp(latency_ms),
        content: vec![OscPacket::Message(msg1)],
    };
    let bundle2 = OscBundle {
        timetag: create_osc_timestamp(latency_ms + offset_ms),
        content: vec![OscPacket::Message(msg2)],
    };
    // Outer bundle with "immediately" timetag containing inner timed bundles
    OscPacket::Bundle(OscBundle {
        timetag: OscTime { seconds: 1, fractional: 0 }, // timetag 1 = immediately
        content: vec![OscPacket::Bundle(bundle1), OscPacket::Bundle(bundle2)],
    })
}
