use crate::osc_utils::{create_bundle, OSC_LATENCY_MS};
use crate::types::{DelayedCommand, MetroCommand, MetroEvent, MetroState, SyncMode, OSC_ADDR, MONOKIT_NODE_ID, route_param_to_node, route_param_to_nodes, NOISE_NODE_ID, MOD_NODE_ID, PRIMARY_NODE_ID, MAIN_NODE_ID, PLAITS_NODE_ID};
use rosc::{encoder, OscMessage, OscPacket, OscType};
use spin_sleep::SpinSleeper;
use audio_thread_priority::promote_current_thread_to_real_time;
use socket2::{Domain, Protocol, Socket, Type};
use std::fs::OpenOptions;
use std::io::Write;
use std::net::{SocketAddr, UdpSocket};
use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

const OSC_BUFFER_SIZE: usize = 4 * 1024 * 1024; // 4MB buffer to prevent packet loss

fn get_osc_log_path() -> std::path::PathBuf {
    std::env::temp_dir().join("monokit_osc.log")
}

fn format_osc_args(args: &[OscType]) -> String {
    args.iter()
        .map(|arg| match arg {
            OscType::Int(i) => i.to_string(),
            OscType::Float(f) => format!("{:.3}", f),
            OscType::String(s) => format!("\"{}\"", s),
            OscType::Long(l) => l.to_string(),
            OscType::Double(d) => format!("{:.3}", d),
            OscType::Bool(b) => b.to_string(),
            OscType::Char(c) => format!("'{}'", c),
            _ => format!("{:?}", arg),
        })
        .collect::<Vec<_>>()
        .join(" ")
}

fn log_osc_message(msg: &OscMessage, label: &str) {
    let timestamp = chrono::Local::now().format("%Y-%m-%d %H:%M:%S%.3f");
    let args_str = format_osc_args(&msg.args);
    let log_line = format!("[{}] {} → {} {}\n", timestamp, label, msg.addr, args_str);

    if let Ok(mut file) = OpenOptions::new()
        .create(true)
        .append(true)
        .open(get_osc_log_path())
    {
        let _ = file.write_all(log_line.as_bytes());
    }
}

#[cfg(feature = "scsynth-direct")]
fn create_param_message(param_name: &str, value: OscType) -> OscMessage {
    let node_id = route_param_to_node(param_name);
    let msg = OscMessage {
        addr: "/n_set".to_string(),
        args: vec![
            OscType::Int(node_id),
            OscType::String(param_name.to_string()),
            value.clone(),
        ],
    };
    log_osc_message(&msg, "CREATE_PARAM");
    msg
}

#[cfg(feature = "scsynth-direct")]
fn create_param_messages(param_name: &str, value: OscType) -> Vec<OscMessage> {
    let node_ids = route_param_to_nodes(param_name);
    node_ids.into_iter().map(|node_id| {
        let msg = OscMessage {
            addr: "/n_set".to_string(),
            args: vec![
                OscType::Int(node_id),
                OscType::String(param_name.to_string()),
                value.clone(),
            ],
        };
        log_osc_message(&msg, "CREATE_PARAM");
        msg
    }).collect()
}

#[cfg(not(feature = "scsynth-direct"))]
fn create_param_message(param_name: &str, value: OscType) -> OscMessage {
    let msg = OscMessage {
        addr: "/monokit/param".to_string(),
        args: vec![OscType::String(param_name.to_string()), value],
    };
    log_osc_message(&msg, "CREATE_PARAM");
    msg
}

#[cfg(feature = "scsynth-direct")]
fn create_trigger_messages() -> Vec<OscMessage> {
    // Send t_gate to all 4 synths
    let messages = vec![
        OscMessage {
            addr: "/n_set".to_string(),
            args: vec![OscType::Int(NOISE_NODE_ID), OscType::String("t_gate".to_string()), OscType::Int(1)],
        },
        OscMessage {
            addr: "/n_set".to_string(),
            args: vec![OscType::Int(MOD_NODE_ID), OscType::String("t_gate".to_string()), OscType::Int(1)],
        },
        OscMessage {
            addr: "/n_set".to_string(),
            args: vec![OscType::Int(PRIMARY_NODE_ID), OscType::String("t_gate".to_string()), OscType::Int(1)],
        },
        OscMessage {
            addr: "/n_set".to_string(),
            args: vec![OscType::Int(MAIN_NODE_ID), OscType::String("t_gate".to_string()), OscType::Int(1)],
        },
    ];
    for msg in &messages {
        log_osc_message(msg, "CREATE_TRIGGER");
    }
    messages
}

#[cfg(not(feature = "scsynth-direct"))]
fn create_trigger_message() -> OscMessage {
    let msg = OscMessage {
        addr: "/monokit/trigger".to_string(),
        args: vec![],
    };
    log_osc_message(&msg, "CREATE_TRIGGER");
    msg
}

#[cfg(feature = "scsynth-direct")]
fn create_volume_message(value: f32) -> OscMessage {
    OscMessage {
        addr: "/n_set".to_string(),
        args: vec![
            OscType::Int(MONOKIT_NODE_ID),
            OscType::String("volume".to_string()),
            OscType::Float(value),
        ],
    }
}

#[cfg(not(feature = "scsynth-direct"))]
fn create_volume_message(value: f32) -> OscMessage {
    OscMessage {
        addr: "/monokit/volume".to_string(),
        args: vec![OscType::Float(value)],
    }
}

#[cfg(feature = "scsynth-direct")]
fn create_slew_message(time_sec: f32) -> OscMessage {
    OscMessage {
        addr: "/n_set".to_string(),
        args: vec![
            OscType::Int(MONOKIT_NODE_ID),
            OscType::String("slew_time".to_string()),
            OscType::Float(time_sec),
        ],
    }
}

#[cfg(not(feature = "scsynth-direct"))]
fn create_slew_message(time_sec: f32) -> OscMessage {
    OscMessage {
        addr: "/monokit/slew".to_string(),
        args: vec![OscType::Float(time_sec)],
    }
}

#[cfg(feature = "scsynth-direct")]
fn create_param_slew_message(param: &str, time_sec: f32) -> OscMessage {
    let slew_param = format!("slew_{}", param);
    OscMessage {
        addr: "/n_set".to_string(),
        args: vec![
            OscType::Int(MONOKIT_NODE_ID),
            OscType::String(slew_param),
            OscType::Float(time_sec),
        ],
    }
}

#[cfg(not(feature = "scsynth-direct"))]
fn create_param_slew_message(param: &str, time_sec: f32) -> OscMessage {
    OscMessage {
        addr: "/monokit/slew/param".to_string(),
        args: vec![OscType::String(param.to_string()), OscType::Float(time_sec)],
    }
}

#[cfg(feature = "scsynth-direct")]
fn create_gate_message(time_sec: f32) -> OscMessage {
    OscMessage {
        addr: "/n_set".to_string(),
        args: vec![
            OscType::Int(MONOKIT_NODE_ID),
            OscType::String("env_atk".to_string()),
            OscType::Float(time_sec * 1000.0),
        ],
    }
}

#[cfg(not(feature = "scsynth-direct"))]
fn create_gate_message(time_sec: f32) -> OscMessage {
    OscMessage {
        addr: "/monokit/gate".to_string(),
        args: vec![OscType::Float(time_sec)],
    }
}

#[cfg(feature = "scsynth-direct")]
fn create_env_gate_message(env_name: &str, time_sec: f32) -> OscMessage {
    let param_name = format!("{}_atk", env_name);
    OscMessage {
        addr: "/n_set".to_string(),
        args: vec![
            OscType::Int(MONOKIT_NODE_ID),
            OscType::String(param_name),
            OscType::Float(time_sec * 1000.0),
        ],
    }
}

#[cfg(not(feature = "scsynth-direct"))]
fn create_env_gate_message(env_name: &str, time_sec: f32) -> OscMessage {
    OscMessage {
        addr: "/monokit/gate/env".to_string(),
        args: vec![OscType::String(env_name.to_string()), OscType::Float(time_sec)],
    }
}

#[cfg(feature = "scsynth-direct")]
fn create_scope_rate_message(time_ms: f32) -> OscMessage {
    let rate = 128.0 / (time_ms * 44.1);
    OscMessage {
        addr: "/n_set".to_string(),
        args: vec![
            OscType::Int(1002),
            OscType::String("scopeRate".to_string()),
            OscType::Float(rate),
        ],
    }
}

#[cfg(not(feature = "scsynth-direct"))]
fn create_scope_rate_message(time_ms: f32) -> OscMessage {
    OscMessage {
        addr: "/monokit/scope/rate".to_string(),
        args: vec![OscType::Float(time_ms)],
    }
}

// Metro thread timing diagnostics
struct MetroTimingStats {
    last_process_time: Option<Instant>,
    process_intervals: Vec<f64>,
    recv_to_send_delays: Vec<f64>,
    enabled: bool,
    trigger_count: u64,
}

impl MetroTimingStats {
    fn new() -> Self {
        Self {
            last_process_time: None,
            process_intervals: Vec::new(),
            recv_to_send_delays: Vec::new(),
            enabled: false,
            trigger_count: 0,
        }
    }

    fn enable(&mut self) {
        self.enabled = true;
        self.last_process_time = None;
        self.process_intervals.clear();
        self.recv_to_send_delays.clear();
        self.trigger_count = 0;
    }

    fn disable(&mut self) {
        self.enabled = false;
    }

    fn record_process(&mut self, recv_to_send_us: u64) {
        if !self.enabled {
            return;
        }

        let now = Instant::now();

        if let Some(last) = self.last_process_time {
            let interval_ms = now.duration_since(last).as_micros() as f64 / 1000.0;
            self.process_intervals.push(interval_ms);
            if self.process_intervals.len() > 100 {
                self.process_intervals.remove(0);
            }
        }

        self.last_process_time = Some(now);

        // Record recv-to-send delay in microseconds
        self.recv_to_send_delays.push(recv_to_send_us as f64);
        if self.recv_to_send_delays.len() > 100 {
            self.recv_to_send_delays.remove(0);
        }
    }

    /// Append metro thread timing report to midi_timing_report.txt
    fn write_report(&self) {
        if self.process_intervals.is_empty() {
            return;
        }

        let mut report = String::new();
        report.push_str("\n=== Metro Thread Timing Report ===\n");
        report.push_str(&format!("Triggers sent: {}\n", self.trigger_count));

        // Process intervals
        let sum: f64 = self.process_intervals.iter().sum();
        let mean = sum / self.process_intervals.len() as f64;
        let variance = self.process_intervals.iter()
            .map(|&x| (x - mean).powi(2))
            .sum::<f64>() / self.process_intervals.len() as f64;
        let std_dev = variance.sqrt();
        let min = self.process_intervals.iter().copied().fold(f64::INFINITY, f64::min);
        let max = self.process_intervals.iter().copied().fold(f64::NEG_INFINITY, f64::max);

        report.push_str(&format!("Samples:     {}\n", self.process_intervals.len()));
        report.push_str(&format!("Mean:        {:.3}ms\n", mean));
        report.push_str(&format!("Std dev:     {:.3}ms\n", std_dev));
        report.push_str(&format!("Min:         {:.3}ms\n", min));
        report.push_str(&format!("Max:         {:.3}ms\n", max));
        report.push_str(&format!("Jitter:      {:.3}ms\n", max - min));

        // Show last 16 intervals
        report.push_str("\nLast 16 intervals (ms):\n");
        let start = if self.process_intervals.len() > 16 { self.process_intervals.len() - 16 } else { 0 };
        for (i, interval) in self.process_intervals[start..].iter().enumerate() {
            let deviation = interval - mean;
            let sign = if deviation >= 0.0 { "+" } else { "" };
            report.push_str(&format!("{:6.2}({}{:.1}) ", interval, sign, deviation));
            if (i + 1) % 4 == 0 {
                report.push('\n');
            }
        }

        // Swing analysis
        if self.process_intervals.len() >= 8 {
            let mut odd_sum = 0.0;
            let mut even_sum = 0.0;
            let mut odd_count = 0;
            let mut even_count = 0;

            for (i, &interval) in self.process_intervals.iter().enumerate() {
                if i % 2 == 0 {
                    even_sum += interval;
                    even_count += 1;
                } else {
                    odd_sum += interval;
                    odd_count += 1;
                }
            }

            let even_avg = even_sum / even_count as f64;
            let odd_avg = odd_sum / odd_count as f64;
            let swing_ratio = odd_avg / even_avg;

            report.push_str("\nSwing analysis:\n");
            report.push_str(&format!("  Even intervals avg: {:.3}ms\n", even_avg));
            report.push_str(&format!("  Odd intervals avg:  {:.3}ms\n", odd_avg));
            report.push_str(&format!("  Ratio (odd/even):   {:.3}\n", swing_ratio));

            if (swing_ratio - 1.0).abs() > 0.05 {
                report.push_str("  >>> SWING DETECTED at metro thread level! <<<\n");
            }
        }

        // Recv-to-send delays
        let delay_sum: f64 = self.recv_to_send_delays.iter().sum();
        let delay_mean = delay_sum / self.recv_to_send_delays.len() as f64;
        let delay_min = self.recv_to_send_delays.iter().copied().fold(f64::INFINITY, f64::min);
        let delay_max = self.recv_to_send_delays.iter().copied().fold(f64::NEG_INFINITY, f64::max);

        report.push_str("\nProcessing latency (recv -> send):\n");
        report.push_str(&format!("  Mean: {:.1}us, Min: {:.1}us, Max: {:.1}us\n", delay_mean, delay_min, delay_max));
        report.push_str("===================================\n");

        // Append to file
        if let Ok(mut file) = OpenOptions::new()
            .create(true)
            .append(true)
            .open("midi_timing_report.txt")
        {
            let _ = file.write_all(report.as_bytes());
        }
    }
}

/// Send OSC - uses timestamps for internal timing, immediate for MIDI sync
fn send_osc(socket: Option<&UdpSocket>, msg: OscMessage, use_timestamp: bool) {
    log_osc_message(&msg, "SEND");

    if let Some(socket) = socket {
        let packet = if use_timestamp {
            create_bundle(vec![msg], OSC_LATENCY_MS)
        } else {
            OscPacket::Message(msg)
        };
        if let Ok(buf) = encoder::encode(&packet) {
            let _ = socket.send(&buf);
        }
    }
}

pub fn metro_thread(rx: mpsc::Receiver<MetroCommand>, state: Arc<Mutex<MetroState>>, event_tx: mpsc::Sender<MetroEvent>, dry_run: bool) {
    let _rt_handle = promote_current_thread_to_real_time(512, 48000).ok();

    if dry_run {
        eprintln!("[monokit] Metro thread: DRY-RUN mode (no OSC)");
    } else {
        #[cfg(feature = "scsynth-direct")]
        eprintln!("[monokit] Metro thread: SCSYNTH-DIRECT mode (port 57110, /n_set format)");

        #[cfg(not(feature = "scsynth-direct"))]
        eprintln!("[monokit] Metro thread: SCLANG mode (port 57120, /monokit/* format)");
    }

    let spinner = SpinSleeper::default();

    let socket: Option<UdpSocket> = if dry_run {
        None
    } else {
        // Create socket with large buffers to prevent UDP packet loss
        let socket = match Socket::new(Domain::IPV4, Type::DGRAM, Some(Protocol::UDP)) {
            Ok(s) => s,
            Err(e) => {
                let _ = event_tx.send(MetroEvent::Error(format!("ERROR: OSC SOCKET CREATE FAIL: {}", e)));
                return;
            }
        };

        // Set large send buffer to prevent packet loss
        if let Err(e) = socket.set_send_buffer_size(OSC_BUFFER_SIZE) {
            let _ = event_tx.send(MetroEvent::Error(format!("WARN: OSC SEND BUFFER SIZE FAIL: {}", e)));
        }

        // Bind to any available port
        let bind_addr: SocketAddr = "0.0.0.0:0".parse().unwrap();
        if let Err(e) = socket.bind(&bind_addr.into()) {
            let _ = event_tx.send(MetroEvent::Error(format!("ERROR: OSC SOCKET BIND FAIL: {}", e)));
            return;
        }

        // Connect to SuperCollider
        let osc_addr: SocketAddr = OSC_ADDR.parse().unwrap();
        if let Err(e) = socket.connect(&osc_addr.into()) {
            let _ = event_tx.send(MetroEvent::Error(format!("ERROR: OSC CONNECT FAIL: {}", e)));
            return;
        }

        // Convert to std UdpSocket for compatibility
        Some(socket.into())
    };

    let mut interval_ms: u64 = 500;
    let mut active = false;
    let mut sync_mode = SyncMode::Internal;
    let mut next_tick = Instant::now();
    let mut delayed_commands: Vec<DelayedCommand> = Vec::new();
    let start_time = Instant::now();
    let mut metro_timing = MetroTimingStats::new();

    loop {
        let mut interval_changed = false;

        // In MIDI mode, use blocking recv to minimize latency
        // In Internal mode, use try_recv so we can maintain our own timing
        let commands: Vec<MetroCommand> = if sync_mode == SyncMode::MidiClock {
            // Block waiting for commands - MIDI ticks drive timing
            match rx.recv_timeout(Duration::from_millis(10)) {
                Ok(cmd) => {
                    // Got one command, drain any others that arrived
                    let mut cmds = vec![cmd];
                    while let Ok(c) = rx.try_recv() {
                        cmds.push(c);
                    }
                    cmds
                }
                Err(_) => vec![], // Timeout - check delayed commands
            }
        } else {
            // Non-blocking for internal timing mode
            let mut cmds = vec![];
            while let Ok(cmd) = rx.try_recv() {
                cmds.push(cmd);
            }
            cmds
        };

        for cmd in commands {
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
                    #[cfg(feature = "scsynth-direct")]
                    {
                        // Send to all target nodes (some parameters need multiple destinations)
                        for msg in create_param_messages(&name, value.clone()) {
                            send_osc(socket.as_ref(), msg, sync_mode == SyncMode::Internal);
                        }
                    }
                    #[cfg(not(feature = "scsynth-direct"))]
                    {
                        let msg = create_param_message(&name, value);
                        send_osc(socket.as_ref(), msg, sync_mode == SyncMode::Internal);
                    }
                }
                MetroCommand::SendTrigger => {
                    // Send t_gate to all 4 synths in multi-synth architecture
                    #[cfg(feature = "scsynth-direct")]
                    {
                        for msg in create_trigger_messages() {
                            send_osc(socket.as_ref(), msg, sync_mode == SyncMode::Internal);
                        }
                    }
                    #[cfg(not(feature = "scsynth-direct"))]
                    {
                        let msg = create_trigger_message();
                        send_osc(socket.as_ref(), msg, sync_mode == SyncMode::Internal);
                    }
                    metro_timing.trigger_count += 1;
                }
                MetroCommand::SendPlaitsTrigger => {
                    #[cfg(feature = "scsynth-direct")]
                    {
                        let msg = OscMessage {
                            addr: "/n_set".to_string(),
                            args: vec![OscType::Int(PLAITS_NODE_ID), OscType::String("t_gate".to_string()), OscType::Int(1)],
                        };
                        log_osc_message(&msg, "CREATE_PLAITS_TRIGGER");
                        send_osc(socket.as_ref(), msg, sync_mode == SyncMode::Internal);
                    }
                    #[cfg(not(feature = "scsynth-direct"))]
                    {
                        // For sclang mode, send via /monokit/param with plaits-specific parameter
                        let msg = OscMessage {
                            addr: "/monokit/param".to_string(),
                            args: vec![OscType::String("t_gate_plaits".to_string()), OscType::Int(1)],
                        };
                        log_osc_message(&msg, "CREATE_PLAITS_TRIGGER");
                        send_osc(socket.as_ref(), msg, sync_mode == SyncMode::Internal);
                    }
                }
                MetroCommand::SendVolume(value) => {
                    let msg = create_volume_message(value);
                    send_osc(socket.as_ref(), msg, sync_mode == SyncMode::Internal);
                }
                MetroCommand::StartRecording(dir) => {
                    #[cfg(not(feature = "scsynth-direct"))]
                    {
                        let msg = OscMessage {
                            addr: "/monokit/rec".to_string(),
                            args: vec![OscType::String(dir)],
                        };
                        send_osc(socket.as_ref(), msg, sync_mode == SyncMode::Internal);
                    }
                    #[cfg(feature = "scsynth-direct")]
                    {
                        let _ = event_tx.send(MetroEvent::StartRecordingDirect(dir));
                    }
                }
                MetroCommand::StopRecording => {
                    #[cfg(not(feature = "scsynth-direct"))]
                    {
                        let msg = OscMessage {
                            addr: "/monokit/rec/stop".to_string(),
                            args: vec![],
                        };
                        send_osc(socket.as_ref(), msg, sync_mode == SyncMode::Internal);
                    }
                    #[cfg(feature = "scsynth-direct")]
                    {
                        let _ = event_tx.send(MetroEvent::StopRecordingDirect);
                    }
                }
                MetroCommand::SetRecordingPath(path) => {
                    #[cfg(not(feature = "scsynth-direct"))]
                    {
                        let msg = OscMessage {
                            addr: "/monokit/rec/path".to_string(),
                            args: vec![OscType::String(path)],
                        };
                        send_osc(socket.as_ref(), msg, sync_mode == SyncMode::Internal);
                    }
                    #[cfg(feature = "scsynth-direct")]
                    {
                        let _ = event_tx.send(MetroEvent::SetRecordingPathDirect(path));
                    }
                }
                MetroCommand::SetSlewTime(time_sec) => {
                    let msg = create_slew_message(time_sec);
                    send_osc(socket.as_ref(), msg, sync_mode == SyncMode::Internal);
                }
                MetroCommand::SetParamSlew(param, time_sec) => {
                    let msg = create_param_slew_message(&param, time_sec);
                    send_osc(socket.as_ref(), msg, sync_mode == SyncMode::Internal);
                }
                MetroCommand::SetGate(time_sec) => {
                    let msg = create_gate_message(time_sec);
                    send_osc(socket.as_ref(), msg, sync_mode == SyncMode::Internal);
                }
                MetroCommand::SetEnvGate(env_name, time_sec) => {
                    let msg = create_env_gate_message(&env_name, time_sec);
                    send_osc(socket.as_ref(), msg, sync_mode == SyncMode::Internal);
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
                        let recv_time = Instant::now();

                        let script_index = {
                            let st = state.lock().unwrap();
                            st.script_index
                        };
                        let _ = event_tx.send(MetroEvent::ExecuteScript(script_index));

                        let send_time = Instant::now();
                        let delay_us = send_time.duration_since(recv_time).as_micros() as u64;

                        metro_timing.record_process(delay_us);

                        // Warn if processing takes too long
                        if metro_timing.enabled && delay_us > 100 {
                            let _ = event_tx.send(MetroEvent::Error(format!("WARN: MIDI TICK PROC {}US", delay_us)));
                        }
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
                MetroCommand::EnableMidiTimingDiag => {
                    metro_timing.enable();
                }
                MetroCommand::DisableMidiTimingDiag => {
                    metro_timing.disable();
                }
                MetroCommand::PrintMidiTimingReport => {
                    metro_timing.write_report();
                }
                MetroCommand::SendScDiag(value) => {
                    #[cfg(not(feature = "scsynth-direct"))]
                    {
                        let msg = OscMessage {
                            addr: "/monokit/diag".to_string(),
                            args: vec![OscType::Int(value)],
                        };
                        send_osc(socket.as_ref(), msg, sync_mode == SyncMode::Internal);
                    }
                    #[cfg(feature = "scsynth-direct")]
                    {
                        let _ = value;
                    }
                }
                MetroCommand::SendScDiagReport => {
                    #[cfg(not(feature = "scsynth-direct"))]
                    {
                        let msg = OscMessage {
                            addr: "/monokit/diag/report".to_string(),
                            args: vec![],
                        };
                        send_osc(socket.as_ref(), msg, sync_mode == SyncMode::Internal);
                    }
                }
                MetroCommand::GetTriggerCount => {
                    let _ = event_tx.send(MetroEvent::Error(format!("TRIGGERS SENT: {}", metro_timing.trigger_count)));
                }
                MetroCommand::ResetTriggerCount => {
                    metro_timing.trigger_count = 0;
                    let _ = event_tx.send(MetroEvent::Error("TRIGGER COUNTER RESET".to_string()));
                }
                MetroCommand::SendScopeRate(time_ms) => {
                    let msg = create_scope_rate_message(time_ms);
                    send_osc(socket.as_ref(), msg, sync_mode == SyncMode::Internal);
                }
                MetroCommand::QueryAudioOutDevices => {
                    #[cfg(not(feature = "scsynth-direct"))]
                    {
                        let msg = OscMessage {
                            addr: "/monokit/audio/out/query".to_string(),
                            args: vec![],
                        };
                        send_osc(socket.as_ref(), msg, sync_mode == SyncMode::Internal);
                    }
                    #[cfg(feature = "scsynth-direct")]
                    {
                        #[cfg(target_os = "macos")]
                        {
                            match crate::audio_devices::list_audio_devices() {
                                Ok(devices) => {
                                    let device_names: Vec<String> = devices.iter().map(|d| d.name.clone()).collect();

                                    let current = if let Ok(config) = crate::config::load_config() {
                                        config.display.audio_out_device.unwrap_or_else(|| "default".to_string())
                                    } else {
                                        "default".to_string()
                                    };

                                    let _ = event_tx.send(MetroEvent::AudioDeviceList {
                                        current,
                                        devices: device_names,
                                    });
                                }
                                Err(e) => {
                                    let _ = event_tx.send(MetroEvent::Error(
                                        format!("FAILED TO QUERY DEVICES: {}", e)
                                    ));
                                }
                            }
                        }
                        #[cfg(not(target_os = "macos"))]
                        {
                            let _ = event_tx.send(MetroEvent::Error(
                                "AUDIO DEVICE QUERY NOT SUPPORTED ON THIS PLATFORM".to_string()
                            ));
                        }
                    }
                }
                MetroCommand::SetAudioOutDevice(device) => {
                    let _ = event_tx.send(MetroEvent::RestartScWithDevice(device));
                }
                MetroCommand::Error(msg) => {
                    let _ = event_tx.send(MetroEvent::Error(msg));
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
                // In MIDI mode, timing is handled by recv_timeout() at top of loop.
                // MidiClockTick commands drive script execution directly.
                // No additional sleep needed - recv_timeout provides the wait.
            }
        }
    }
}
