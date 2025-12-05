use rosc::{encoder, OscMessage, OscPacket, OscType};
use std::env;
use std::fs::{File, OpenOptions};
use std::io::{BufRead, Write};
use std::net::UdpSocket;
use std::path::PathBuf;
use std::process::{Child, Command, Stdio};
use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::Duration;

#[cfg(target_os = "macos")]
use crate::audio_devices;

pub struct ScsynthDirect {
    child: Option<Child>,
    osc_socket: Option<UdpSocket>,
}

impl ScsynthDirect {
    pub fn new() -> Result<Self, String> {
        Ok(Self {
            child: None,
            osc_socket: None,
        })
    }

    pub fn start(&mut self, audio_out_device: Option<&str>) -> Result<(), String> {
        self.start_internal(audio_out_device, false)
    }

    fn start_internal(&mut self, audio_out_device: Option<&str>, silent: bool) -> Result<(), String> {
        self.stop();

        if !silent {
            eprintln!("[monokit] Starting in SCSYNTH-DIRECT mode");
        }

        let scsynth_path = find_scsynth()?;
        let synthdefs_dir = find_synthdefs_dir()?;
        let plugins_dir = find_plugins_dir()?;

        if !silent {
            eprintln!("[monokit] scsynth: {:?}", scsynth_path);
            eprintln!("[monokit] synthdefs: {:?}", synthdefs_dir);
            eprintln!("[monokit] plugins: {:?}", plugins_dir);
        }

        let mut cmd = Command::new(&scsynth_path);

        cmd.arg("-u").arg("57110");  // UDP port
        cmd.arg("-o").arg("2");       // Output channels (stereo)
        cmd.arg("-i").arg("0");       // No input channels (avoid sample rate mismatch)
        cmd.arg("-R").arg("0");       // Don't load default synthdefs
        cmd.arg("-l").arg("1");       // Max logins

        if plugins_dir.exists() {
            cmd.arg("-U").arg(&plugins_dir);
        }

        #[cfg(target_os = "macos")]
        if let Some(device) = audio_out_device {
            match audio_devices::find_device_by_name(device) {
                Ok(Some(audio_device)) => {
                    if !silent {
                        eprintln!("[monokit] Using audio device: {}", audio_device.name);
                    }
                    cmd.arg("-H").arg(&audio_device.name);
                }
                Ok(None) => {
                    if !silent {
                        eprintln!("[monokit] WARNING: Audio device '{}' not found, using default", device);
                    }
                }
                Err(e) => {
                    if !silent {
                        eprintln!("[monokit] WARNING: Failed to query audio devices: {}", e);
                        eprintln!("[monokit] Using default audio device");
                    }
                }
            }
        }

        #[cfg(not(target_os = "macos"))]
        if let Some(device) = audio_out_device {
            if !silent {
                eprintln!("[monokit] WARNING: Audio device selection not supported on this platform");
                eprintln!("[monokit] Requested device: {}", device);
                eprintln!("[monokit] Using default audio device");
            }
        }

        cmd.stdout(Stdio::piped());
        cmd.stderr(Stdio::piped());

        let mut child = cmd
            .spawn()
            .map_err(|e| format!("Failed to spawn scsynth: {}", e))?;

        let stdout = child.stdout.take();
        let stderr = child.stderr.take();

        // Create a log file for scsynth output (so we can debug after TUI takes over)
        let log_path = PathBuf::from("/tmp/scsynth.log");

        // Ensure directory exists
        if let Some(parent) = log_path.parent() {
            let _ = std::fs::create_dir_all(parent);
        }

        let log_file = Arc::new(Mutex::new(
            OpenOptions::new()
                .create(true)
                .write(true)
                .truncate(true)
                .open(&log_path)
                .ok()
        ));

        if !silent {
            eprintln!("[monokit] scsynth log: {:?}", log_path);
        }

        // Use channels to wait for "server ready" message
        let (ready_tx, ready_rx) = std::sync::mpsc::channel();

        let log_file_stdout = Arc::clone(&log_file);
        let silent_stdout = silent;
        thread::spawn(move || {
            if let Some(stdout) = stdout {
                let reader = std::io::BufReader::new(stdout);
                for line in reader.lines() {
                    if let Ok(line) = line {
                        if !silent_stdout {
                            eprintln!("[scsynth] {}", line);
                        }

                        // Also log to file
                        if let Ok(mut guard) = log_file_stdout.lock() {
                            if let Some(ref mut f) = *guard {
                                let _ = writeln!(f, "[stdout] {}", line);
                                let _ = f.flush();
                            }
                        }

                        // Signal when server is ready
                        if line.contains("server ready") {
                            let _ = ready_tx.send(());
                        }
                    }
                }

                // Log that stdout closed
                if let Ok(mut guard) = log_file_stdout.lock() {
                    if let Some(ref mut f) = *guard {
                        let _ = writeln!(f, "[stdout] --- STREAM CLOSED ---");
                        let _ = f.flush();
                    }
                }
            }
        });

        let log_file_stderr = Arc::clone(&log_file);
        let silent_stderr = silent;
        thread::spawn(move || {
            if let Some(stderr) = stderr {
                let reader = std::io::BufReader::new(stderr);
                for line in reader.lines() {
                    if let Ok(line) = line {
                        if !silent_stderr {
                            eprintln!("[scsynth stderr] {}", line);
                        }

                        // Also log to file
                        if let Ok(mut guard) = log_file_stderr.lock() {
                            if let Some(ref mut f) = *guard {
                                let _ = writeln!(f, "[stderr] {}", line);
                                let _ = f.flush();
                            }
                        }
                    }
                }

                // Log that stderr closed
                if let Ok(mut guard) = log_file_stderr.lock() {
                    if let Some(ref mut f) = *guard {
                        let _ = writeln!(f, "[stderr] --- STREAM CLOSED ---");
                        let _ = f.flush();
                    }
                }
            }
        });

        self.child = Some(child);

        // Wait for "server ready" message with timeout
        if !silent {
            eprintln!("[monokit] Waiting for scsynth to initialize...");
        }
        match ready_rx.recv_timeout(Duration::from_secs(10)) {
            Ok(_) => {
                if !silent {
                    eprintln!("[monokit] scsynth is ready");
                }
            }
            Err(_) => {
                if !silent {
                    eprintln!("[monokit] WARNING: Timeout waiting for scsynth ready signal");
                }
            }
        }

        // Small delay to ensure scsynth is fully ready for OSC
        thread::sleep(Duration::from_millis(200));

        // For initial boot: bind to 57121 so we hold the port until meter_thread takes over
        // For restart (silent): meter_thread already has 57121, use a random port
        let socket = if silent {
            // Restart mode - meter_thread already has 57121, use random port
            UdpSocket::bind("127.0.0.1:0")
                .map_err(|e| format!("Failed to bind OSC socket: {}", e))?
        } else {
            // Initial boot - bind to 57121 to hold port for meter_thread
            UdpSocket::bind("127.0.0.1:57121")
                .map_err(|e| format!("Failed to bind OSC socket to 57121: {}", e))?
        };

        socket.set_read_timeout(Some(Duration::from_secs(5)))
            .map_err(|e| format!("Failed to set socket timeout: {}", e))?;

        self.send_boot_sequence(&socket, &synthdefs_dir, silent)?;

        // Close the socket so meter_thread can bind to 57121 (initial boot only)
        drop(socket);
        self.osc_socket = None;

        if !silent {
            eprintln!("[monokit] Boot sequence complete");
            eprintln!("[monokit] Ready sender will start after meter thread initializes");
        }

        // For restart: send ready signal to meter_thread since it's already listening
        if silent {
            thread::sleep(Duration::from_millis(100));
            if let Ok(ready_socket) = UdpSocket::bind("127.0.0.1:0") {
                let msg = OscMessage {
                    addr: "/monokit/ready".to_string(),
                    args: vec![],
                };
                if let Ok(packet) = encoder::encode(&OscPacket::Message(msg)) {
                    let _ = ready_socket.send_to(&packet, "127.0.0.1:57121");
                }
            }
        }

        Ok(())
    }

    pub fn send_ready(&self) -> mpsc::Receiver<()> {
        let (tx, rx) = mpsc::channel();
        thread::spawn(move || {
            for _i in 0..20 {
                thread::sleep(Duration::from_millis(100));

                if let Ok(socket) = UdpSocket::bind("127.0.0.1:0") {
                    let msg = OscMessage {
                        addr: "/monokit/ready".to_string(),
                        args: vec![],
                    };

                    if let Ok(packet) = encoder::encode(&OscPacket::Message(msg)) {
                        if socket.send_to(&packet, "127.0.0.1:57121").is_ok() {
                            let _ = tx.send(());
                            break;
                        }
                    }
                }
            }
        });
        rx
    }

    fn send_boot_sequence(&self, socket: &UdpSocket, synthdefs_dir: &PathBuf, silent: bool) -> Result<(), String> {
        // Only send /notify on initial boot (not silent)
        // On restart, meter_thread sends /notify from port 57121
        if !silent {
            eprintln!("[monokit] Sending /notify...");
            Self::send_osc_message_static(socket, "/notify", vec![OscType::Int(1)])?;
        }

        thread::sleep(Duration::from_millis(500));

        let synthdef_files = vec![
            synthdefs_dir.join("monokit.scsyndef"),
            synthdefs_dir.join("monokit_spectrum.scsyndef"),
            synthdefs_dir.join("monokit_scope.scsyndef"),
        ];

        if !silent {
            eprintln!("[monokit] Loading SynthDefs...");
        }
        for file in &synthdef_files {
            if !file.exists() {
                return Err(format!("SynthDef not found: {}", file.display()));
            }

            let path_str = file.to_str()
                .ok_or(format!("Invalid path: {}", file.display()))?;

            if !silent {
                eprintln!("[monokit]   Loading: {}", file.file_name().unwrap_or_default().to_string_lossy());
            }
            Self::send_osc_message_static(socket, "/d_load", vec![OscType::String(path_str.to_string())])?;
            thread::sleep(Duration::from_millis(200));
        }

        thread::sleep(Duration::from_millis(500));

        if !silent {
            eprintln!("[monokit] Creating synth instances...");
        }
        Self::send_osc_message_static(
            socket,
            "/s_new",
            vec![
                OscType::String("monokit".to_string()),
                OscType::Int(1000),
                OscType::Int(0),
                OscType::Int(0),
            ],
        )?;
        if !silent {
            eprintln!("[monokit]   Created monokit (node 1000)");
        }

        Self::send_osc_message_static(
            socket,
            "/s_new",
            vec![
                OscType::String("monokit_spectrum".to_string()),
                OscType::Int(1001),
                OscType::Int(1),
                OscType::Int(0),
            ],
        )?;
        if !silent {
            eprintln!("[monokit]   Created monokit_spectrum (node 1001)");
        }

        Self::send_osc_message_static(
            socket,
            "/s_new",
            vec![
                OscType::String("monokit_scope".to_string()),
                OscType::Int(1002),
                OscType::Int(1),
                OscType::Int(0),
            ],
        )?;
        if !silent {
            eprintln!("[monokit]   Created monokit_scope (node 1002)");
        }

        thread::sleep(Duration::from_millis(500));

        if !silent {
            eprintln!("[monokit] SCSYNTH-DIRECT boot complete!");
        }
        Ok(())
    }

    fn send_osc_message_static(socket: &UdpSocket, addr: &str, args: Vec<OscType>) -> Result<(), String> {
        let msg = OscMessage {
            addr: addr.to_string(),
            args,
        };

        let packet = OscPacket::Message(msg);
        let buf = encoder::encode(&packet)
            .map_err(|e| format!("Failed to encode OSC message: {}", e))?;

        socket.send_to(&buf, "127.0.0.1:57110")
            .map_err(|e| format!("Failed to send OSC message: {}", e))?;

        Ok(())
    }

    pub fn stop(&mut self) {
        // Create temporary socket to send /quit
        if let Ok(socket) = UdpSocket::bind("127.0.0.1:0") {
            let msg = OscMessage {
                addr: "/quit".to_string(),
                args: vec![],
            };
            let packet = OscPacket::Message(msg);
            if let Ok(buf) = encoder::encode(&packet) {
                let _ = socket.send_to(&buf, "127.0.0.1:57110");
            }
        }

        thread::sleep(Duration::from_millis(500));

        if let Some(mut child) = self.child.take() {
            match child.try_wait() {
                Ok(Some(_)) => {},
                Ok(None) => {
                    let _ = child.kill();
                    let _ = child.wait();
                }
                Err(_) => {
                    let _ = child.kill();
                    let _ = child.wait();
                }
            }

            thread::sleep(Duration::from_millis(300));
        }

        let _ = Command::new("pkill").arg("-f").arg("scsynth").output();
        thread::sleep(Duration::from_millis(200));

        self.osc_socket = None;
    }

    pub fn restart_with_device(&mut self, device: &str) -> Result<(), String> {
        self.start_internal(Some(device), true)
    }

    pub fn is_running(&self) -> bool {
        self.child.is_some()
    }

    /// Send OSC message to scsynth
    /// Used for recording control and other commands that need to reach scsynth directly
    pub fn send_osc(&self, addr: &str, args: Vec<OscType>) -> Result<(), String> {
        let socket = UdpSocket::bind("127.0.0.1:0")
            .map_err(|e| format!("Failed to bind socket for OSC: {}", e))?;

        Self::send_osc_message_static(&socket, addr, args)
    }

    /// Start recording audio to a file
    ///
    /// NOTE: Recording in scsynth-direct mode requires DiskOut UGen implementation.
    /// This is a stub that will be fully implemented in Phase 4.
    ///
    /// The proper implementation requires:
    /// 1. Adding DiskOut UGen to the monokit SynthDef
    /// 2. Using /b_alloc to allocate recording buffer
    /// 3. Using /b_record to start recording to buffer
    /// 4. Using /b_write to write buffer to disk
    /// 5. Using /b_free to clean up
    pub fn start_recording(&self, _dir: &str, _custom_path: Option<&str>) -> Result<(), String> {
        eprintln!("[monokit] Recording not yet fully implemented in scsynth-direct mode");
        eprintln!("[monokit] TODO: Implement DiskOut-based recording");
        // Return Ok for now so the command doesn't error out
        // In production, this should either work or clearly fail
        Ok(())
    }

    /// Stop recording
    ///
    /// NOTE: Stub for DiskOut-based recording (Phase 4 implementation pending)
    pub fn stop_recording(&self) -> Result<(), String> {
        eprintln!("[monokit] Recording stop (not yet implemented in scsynth-direct mode)");
        Ok(())
    }
}

impl Drop for ScsynthDirect {
    fn drop(&mut self) {
        self.stop();
    }
}

fn find_scsynth() -> Result<PathBuf, String> {
    // Check for bundled scsynth (in Resources/ subdirectory for framework path resolution)
    if let Ok(exe) = env::current_exe() {
        if let Some(dir) = exe.parent() {
            // New bundle structure: monokit is at root, scsynth in Resources/
            let bundled = dir.join("Resources/scsynth");
            if bundled.exists() {
                return Ok(bundled);
            }
            // Legacy: scsynth at same level as monokit
            let bundled = dir.join("scsynth");
            if bundled.exists() {
                return Ok(bundled);
            }
        }
    }

    let candidates = [
        "/Applications/SuperCollider.app/Contents/Resources/scsynth",
        "/opt/homebrew/bin/scsynth",
        "/usr/local/bin/scsynth",
    ];

    for path in candidates {
        let p = PathBuf::from(path);
        if p.exists() {
            return Ok(p);
        }
    }

    if let Ok(output) = Command::new("which").arg("scsynth").output() {
        if output.status.success() {
            let path = String::from_utf8_lossy(&output.stdout).trim().to_string();
            if !path.is_empty() {
                return Ok(PathBuf::from(path));
            }
        }
    }

    Err("scsynth not found. Install SuperCollider.".to_string())
}

fn find_synthdefs_dir() -> Result<PathBuf, String> {
    if let Ok(exe) = env::current_exe() {
        if let Some(dir) = exe.parent() {
            // New bundle structure: Resources/synthdefs/
            let synthdefs = dir.join("Resources/synthdefs");
            if synthdefs.exists() {
                return Ok(synthdefs);
            }
            // Legacy: synthdefs at same level
            let synthdefs = dir.join("synthdefs");
            if synthdefs.exists() {
                return Ok(synthdefs);
            }
            let synthdefs = dir.join("sc/synthdefs");
            if synthdefs.exists() {
                return Ok(synthdefs);
            }
            if let Some(parent) = dir.parent() {
                let synthdefs = parent.join("sc/synthdefs");
                if synthdefs.exists() {
                    return Ok(synthdefs);
                }
            }
        }
    }

    let cwd = PathBuf::from("sc/synthdefs");
    if cwd.exists() {
        return Ok(cwd);
    }

    if let Some(config_dir) = dirs::config_dir() {
        let synthdefs = config_dir.join("monokit/sc/synthdefs");
        if synthdefs.exists() {
            return Ok(synthdefs);
        }
    }

    let homebrew_paths = [
        "/opt/homebrew/share/monokit/sc/synthdefs",
        "/usr/local/share/monokit/sc/synthdefs",
    ];
    for path in homebrew_paths {
        let synthdefs = PathBuf::from(path);
        if synthdefs.exists() {
            return Ok(synthdefs);
        }
    }

    Err("SynthDefs directory not found".to_string())
}

fn find_plugins_dir() -> Result<PathBuf, String> {
    if let Ok(exe) = env::current_exe() {
        if let Some(dir) = exe.parent() {
            // New bundle structure: Resources/plugins/
            let plugins = dir.join("Resources/plugins");
            if plugins.exists() {
                return Ok(plugins);
            }
            // Legacy: plugins at same level
            let plugins = dir.join("plugins");
            if plugins.exists() {
                return Ok(plugins);
            }
        }
    }

    let candidates = [
        "/Applications/SuperCollider.app/Contents/Resources/plugins",
        "/opt/homebrew/share/SuperCollider/plugins",
        "/usr/local/share/SuperCollider/plugins",
    ];

    for path in candidates {
        let p = PathBuf::from(path);
        if p.exists() {
            return Ok(p);
        }
    }

    Ok(PathBuf::from(""))
}
