use crate::types::{MetroCommand, MetroEvent, MetroState, OSC_ADDR};
use rosc::{encoder, OscMessage, OscPacket, OscType};
use std::net::UdpSocket;
use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

pub fn precise_sleep(duration: Duration) {
    let start = Instant::now();
    let spin_threshold = Duration::from_micros(100);

    if duration > spin_threshold {
        thread::sleep(duration - spin_threshold);
    }

    while start.elapsed() < duration {
        std::hint::spin_loop();
    }
}

pub fn metro_thread(rx: mpsc::Receiver<MetroCommand>, state: Arc<Mutex<MetroState>>, event_tx: mpsc::Sender<MetroEvent>) {
    let socket = match UdpSocket::bind("0.0.0.0:0") {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Metro thread: Failed to bind UDP socket: {}", e);
            return;
        }
    };

    if let Err(e) = socket.connect(OSC_ADDR) {
        eprintln!("Metro thread: Failed to connect to OSC address: {}", e);
        return;
    }

    let mut interval_ms: u64 = 500;
    let mut active = false;
    let mut next_tick = Instant::now();

    loop {
        let mut interval_changed = false;
        while let Ok(cmd) = rx.try_recv() {
            match cmd {
                MetroCommand::SetInterval(ms) => {
                    interval_ms = ms;
                    interval_changed = true;
                }
                MetroCommand::SetActive(act) => {
                    active = act;
                }
                MetroCommand::SetScriptIndex(idx) => {
                    let mut state = state.lock().unwrap();
                    state.script_index = idx;
                }
                MetroCommand::SendParam(name, value) => {
                    let msg = OscMessage {
                        addr: "/monokit/param".to_string(),
                        args: vec![OscType::String(name), value],
                    };
                    let packet = OscPacket::Message(msg);
                    if let Ok(buf) = encoder::encode(&packet) {
                        let _ = socket.send(&buf);
                    }
                }
                MetroCommand::SendTrigger => {
                    let msg = OscMessage {
                        addr: "/monokit/trigger".to_string(),
                        args: vec![],
                    };
                    let packet = OscPacket::Message(msg);
                    if let Ok(buf) = encoder::encode(&packet) {
                        let _ = socket.send(&buf);
                    }
                }
                MetroCommand::SendVolume(value) => {
                    let msg = OscMessage {
                        addr: "/monokit/volume".to_string(),
                        args: vec![OscType::Float(value)],
                    };
                    let packet = OscPacket::Message(msg);
                    if let Ok(buf) = encoder::encode(&packet) {
                        let _ = socket.send(&buf);
                    }
                }
                MetroCommand::StartRecording(dir) => {
                    let msg = OscMessage {
                        addr: "/monokit/rec".to_string(),
                        args: vec![OscType::String(dir)],
                    };
                    let packet = OscPacket::Message(msg);
                    if let Ok(buf) = encoder::encode(&packet) {
                        let _ = socket.send(&buf);
                    }
                }
                MetroCommand::StopRecording => {
                    let msg = OscMessage {
                        addr: "/monokit/rec/stop".to_string(),
                        args: vec![],
                    };
                    let packet = OscPacket::Message(msg);
                    if let Ok(buf) = encoder::encode(&packet) {
                        let _ = socket.send(&buf);
                    }
                }
                MetroCommand::SetRecordingPath(path) => {
                    let msg = OscMessage {
                        addr: "/monokit/rec/path".to_string(),
                        args: vec![OscType::String(path)],
                    };
                    let packet = OscPacket::Message(msg);
                    if let Ok(buf) = encoder::encode(&packet) {
                        let _ = socket.send(&buf);
                    }
                }
            }
        }

        if interval_changed {
            next_tick = Instant::now();
        }

        if active {
            let script_index = {
                let st = state.lock().unwrap();
                st.script_index
            };

            let _ = event_tx.send(MetroEvent::ExecuteScript(script_index));

            next_tick += Duration::from_millis(interval_ms);

            let now = Instant::now();
            if next_tick > now {
                let sleep_duration = next_tick - now;
                precise_sleep(sleep_duration);
            } else {
                next_tick = now;
            }
        } else {
            thread::sleep(Duration::from_millis(10));
        }
    }
}
