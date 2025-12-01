use crate::osc_utils::{create_bundle, OSC_LATENCY_MS};
use crate::types::{DelayedCommand, MetroCommand, MetroEvent, MetroState, SyncMode, OSC_ADDR};
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
fn send_osc(socket: &UdpSocket, msg: OscMessage, use_timestamp: bool) {
    let packet = if use_timestamp {
        create_bundle(vec![msg], OSC_LATENCY_MS)
    } else {
        OscPacket::Message(msg)
    };
    if let Ok(buf) = encoder::encode(&packet) {
        let _ = socket.send(&buf);
    }
}

pub fn metro_thread(rx: mpsc::Receiver<MetroCommand>, state: Arc<Mutex<MetroState>>, event_tx: mpsc::Sender<MetroEvent>) {
    let _rt_handle = promote_current_thread_to_real_time(512, 48000).ok();

    let spinner = SpinSleeper::default();

    // Create socket with large buffers to prevent UDP packet loss
    let socket = match Socket::new(Domain::IPV4, Type::DGRAM, Some(Protocol::UDP)) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Metro thread: Failed to create socket: {}", e);
            return;
        }
    };

    // Set large send buffer to prevent packet loss
    if let Err(e) = socket.set_send_buffer_size(OSC_BUFFER_SIZE) {
        eprintln!("Warning: Failed to set send buffer size: {}", e);
    }

    // Bind to any available port
    let bind_addr: SocketAddr = "0.0.0.0:0".parse().unwrap();
    if let Err(e) = socket.bind(&bind_addr.into()) {
        eprintln!("Metro thread: Failed to bind UDP socket: {}", e);
        return;
    }

    // Connect to SuperCollider
    let osc_addr: SocketAddr = OSC_ADDR.parse().unwrap();
    if let Err(e) = socket.connect(&osc_addr.into()) {
        eprintln!("Metro thread: Failed to connect to OSC address: {}", e);
        return;
    }

    // Convert to std UdpSocket for compatibility
    let socket: UdpSocket = socket.into();

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
                    let msg = OscMessage {
                        addr: "/monokit/param".to_string(),
                        args: vec![OscType::String(name), value],
                    };
                    send_osc(&socket, msg, sync_mode == SyncMode::Internal);
                }
                MetroCommand::SendTrigger => {
                    let msg = OscMessage {
                        addr: "/monokit/trigger".to_string(),
                        args: vec![],
                    };
                    send_osc(&socket, msg, sync_mode == SyncMode::Internal);
                    metro_timing.trigger_count += 1;
                }
                MetroCommand::SendVolume(value) => {
                    let msg = OscMessage {
                        addr: "/monokit/volume".to_string(),
                        args: vec![OscType::Float(value)],
                    };
                    send_osc(&socket, msg, sync_mode == SyncMode::Internal);
                }
                MetroCommand::StartRecording(dir) => {
                    let msg = OscMessage {
                        addr: "/monokit/rec".to_string(),
                        args: vec![OscType::String(dir)],
                    };
                    send_osc(&socket, msg, sync_mode == SyncMode::Internal);
                }
                MetroCommand::StopRecording => {
                    let msg = OscMessage {
                        addr: "/monokit/rec/stop".to_string(),
                        args: vec![],
                    };
                    send_osc(&socket, msg, sync_mode == SyncMode::Internal);
                }
                MetroCommand::SetRecordingPath(path) => {
                    let msg = OscMessage {
                        addr: "/monokit/rec/path".to_string(),
                        args: vec![OscType::String(path)],
                    };
                    send_osc(&socket, msg, sync_mode == SyncMode::Internal);
                }
                MetroCommand::SetSlewTime(time_sec) => {
                    let msg = OscMessage {
                        addr: "/monokit/slew".to_string(),
                        args: vec![OscType::Float(time_sec)],
                    };
                    send_osc(&socket, msg, sync_mode == SyncMode::Internal);
                }
                MetroCommand::SetParamSlew(param, time_sec) => {
                    let msg = OscMessage {
                        addr: "/monokit/slew/param".to_string(),
                        args: vec![OscType::String(param), OscType::Float(time_sec)],
                    };
                    send_osc(&socket, msg, sync_mode == SyncMode::Internal);
                }
                MetroCommand::SetGate(time_sec) => {
                    let msg = OscMessage {
                        addr: "/monokit/gate".to_string(),
                        args: vec![OscType::Float(time_sec)],
                    };
                    send_osc(&socket, msg, sync_mode == SyncMode::Internal);
                }
                MetroCommand::SetEnvGate(env_name, time_sec) => {
                    let msg = OscMessage {
                        addr: "/monokit/gate/env".to_string(),
                        args: vec![OscType::String(env_name), OscType::Float(time_sec)],
                    };
                    send_osc(&socket, msg, sync_mode == SyncMode::Internal);
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
                            eprintln!("WARNING: MidiClockTick processing took {}us", delay_us);
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
                    let msg = OscMessage {
                        addr: "/monokit/diag".to_string(),
                        args: vec![OscType::Int(value)],
                    };
                    send_osc(&socket, msg, sync_mode == SyncMode::Internal);
                }
                MetroCommand::SendScDiagReport => {
                    let msg = OscMessage {
                        addr: "/monokit/diag/report".to_string(),
                        args: vec![],
                    };
                    send_osc(&socket, msg, sync_mode == SyncMode::Internal);
                }
                MetroCommand::GetTriggerCount => {
                    eprintln!("TRIGGERS SENT: {}", metro_timing.trigger_count);
                }
                MetroCommand::ResetTriggerCount => {
                    metro_timing.trigger_count = 0;
                    eprintln!("TRIGGER COUNTER RESET");
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
