use crate::osc_utils::{create_bundle, OSC_LATENCY_MS};
use crate::types::{DelayedCommand, MetroCommand, MetroEvent, MetroState, SyncMode, OSC_ADDR};
use rosc::{encoder, OscMessage, OscType};
use spin_sleep::SpinSleeper;
use audio_thread_priority::promote_current_thread_to_real_time;
use std::net::UdpSocket;
use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

fn send_bundled(socket: &UdpSocket, messages: Vec<OscMessage>) {
    let packet = create_bundle(messages, OSC_LATENCY_MS);
    if let Ok(buf) = encoder::encode(&packet) {
        let _ = socket.send(&buf);
    }
}

pub fn metro_thread(rx: mpsc::Receiver<MetroCommand>, state: Arc<Mutex<MetroState>>, event_tx: mpsc::Sender<MetroEvent>) {
    let _rt_handle = promote_current_thread_to_real_time(512, 48000).ok();

    let spinner = SpinSleeper::default();

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
    let mut sync_mode = SyncMode::Internal;
    let mut next_tick = Instant::now();
    let mut delayed_commands: Vec<DelayedCommand> = Vec::new();
    let start_time = Instant::now();

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
                    send_bundled(&socket, vec![msg]);
                }
                MetroCommand::SendTrigger => {
                    let msg = OscMessage {
                        addr: "/monokit/trigger".to_string(),
                        args: vec![],
                    };
                    send_bundled(&socket, vec![msg]);
                }
                MetroCommand::SendVolume(value) => {
                    let msg = OscMessage {
                        addr: "/monokit/volume".to_string(),
                        args: vec![OscType::Float(value)],
                    };
                    send_bundled(&socket, vec![msg]);
                }
                MetroCommand::StartRecording(dir) => {
                    let msg = OscMessage {
                        addr: "/monokit/rec".to_string(),
                        args: vec![OscType::String(dir)],
                    };
                    send_bundled(&socket, vec![msg]);
                }
                MetroCommand::StopRecording => {
                    let msg = OscMessage {
                        addr: "/monokit/rec/stop".to_string(),
                        args: vec![],
                    };
                    send_bundled(&socket, vec![msg]);
                }
                MetroCommand::SetRecordingPath(path) => {
                    let msg = OscMessage {
                        addr: "/monokit/rec/path".to_string(),
                        args: vec![OscType::String(path)],
                    };
                    send_bundled(&socket, vec![msg]);
                }
                MetroCommand::SetSlewTime(time_sec) => {
                    let msg = OscMessage {
                        addr: "/monokit/slew".to_string(),
                        args: vec![OscType::Float(time_sec)],
                    };
                    send_bundled(&socket, vec![msg]);
                }
                MetroCommand::SetParamSlew(param, time_sec) => {
                    let msg = OscMessage {
                        addr: "/monokit/slew/param".to_string(),
                        args: vec![OscType::String(param), OscType::Float(time_sec)],
                    };
                    send_bundled(&socket, vec![msg]);
                }
                MetroCommand::SetGate(time_sec) => {
                    let msg = OscMessage {
                        addr: "/monokit/gate".to_string(),
                        args: vec![OscType::Float(time_sec)],
                    };
                    send_bundled(&socket, vec![msg]);
                }
                MetroCommand::SetEnvGate(env_name, time_sec) => {
                    let msg = OscMessage {
                        addr: "/monokit/gate/env".to_string(),
                        args: vec![OscType::String(env_name), OscType::Float(time_sec)],
                    };
                    send_bundled(&socket, vec![msg]);
                }
                MetroCommand::ScheduleDelayed(cmd, delay_ms, script_idx) => {
                    let elapsed_ms = start_time.elapsed().as_millis() as u64;
                    let due_at_ms = elapsed_ms + delay_ms;
                    delayed_commands.push(DelayedCommand {
                        due_at_ms,
                        command: cmd,
                        script_index: script_idx,
                    });
                    delayed_commands.sort_by_key(|dc| dc.due_at_ms);
                }
                MetroCommand::ScheduleRepeated(cmd, count, interval_ms, script_idx) => {
                    let elapsed_ms = start_time.elapsed().as_millis() as u64;
                    for i in 0..count {
                        let due_at_ms = elapsed_ms + (i as u64 * interval_ms);
                        delayed_commands.push(DelayedCommand {
                            due_at_ms,
                            command: cmd.clone(),
                            script_index: script_idx,
                        });
                    }
                    delayed_commands.sort_by_key(|dc| dc.due_at_ms);
                }
                MetroCommand::ClearDelayed => {
                    delayed_commands.clear();
                }
                MetroCommand::SetSyncMode(mode) => {
                    sync_mode = mode;
                }
                MetroCommand::MidiClockTick => {
                    if sync_mode == SyncMode::MidiClock && active {
                        let script_index = {
                            let st = state.lock().unwrap();
                            st.script_index
                        };
                        let _ = event_tx.send(MetroEvent::ExecuteScript(script_index));
                    }
                }
                MetroCommand::MidiTransportStart => {
                    if sync_mode == SyncMode::MidiClock {
                        active = true;
                    }
                }
                MetroCommand::MidiTransportStop => {
                    if sync_mode == SyncMode::MidiClock {
                        active = false;
                    }
                }
                MetroCommand::Shutdown => {
                    return; // Exit the metro thread
                }
            }
        }

        if interval_changed {
            next_tick = Instant::now();
        }

        let elapsed_ms = start_time.elapsed().as_millis() as u64;
        while !delayed_commands.is_empty() && delayed_commands[0].due_at_ms <= elapsed_ms {
            let delayed_cmd = delayed_commands.remove(0);
            let _ = event_tx.send(MetroEvent::ExecuteDelayed(
                delayed_cmd.command,
                delayed_cmd.script_index,
            ));
        }

        match sync_mode {
            SyncMode::Internal => {
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
                        spinner.sleep(sleep_duration);
                    } else {
                        next_tick = now;
                    }
                } else {
                    thread::sleep(Duration::from_millis(10));
                }
            }
            SyncMode::MidiClock => {
                thread::sleep(Duration::from_millis(1));
            }
        }
    }
}
