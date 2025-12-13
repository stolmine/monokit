use chrono::Local;
use rosc::{encoder, OscMessage, OscPacket, OscType};
use std::env;
use std::fs::{self, File, OpenOptions};
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
    recording_path_prefix: Option<String>,
    is_recording: bool,
}

impl ScsynthDirect {
    pub fn new() -> Result<Self, String> {
        Ok(Self {
            child: None,
            osc_socket: None,
            recording_path_prefix: None,
            is_recording: false,
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
                let mut server_ready = false;
                for line in reader.lines() {
                    if let Ok(line) = line {
                        // Only print to console during boot, suppress "late" warnings always
                        // Once server is ready, stop printing to avoid TUI corruption
                        let is_late_warning = line.contains("late ");
                        if !silent_stdout && !server_ready && !is_late_warning {
                            eprintln!("[scsynth] {}", line);
                        }

                        // Always log to file (including late warnings)
                        if let Ok(mut guard) = log_file_stdout.lock() {
                            if let Some(ref mut f) = *guard {
                                let _ = writeln!(f, "[stdout] {}", line);
                                let _ = f.flush();
                            }
                        }

                        // Signal when server is ready and stop console output
                        if line.contains("server ready") {
                            server_ready = true;
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
            synthdefs_dir.join("monokit_noise.scsyndef"),
            synthdefs_dir.join("monokit_mod.scsyndef"),
            synthdefs_dir.join("monokit_primary.scsyndef"),
            synthdefs_dir.join("monokit_plaits.scsyndef"),
            synthdefs_dir.join("monokit_main.scsyndef"),
            synthdefs_dir.join("monokit_spectrum.scsyndef"),
            synthdefs_dir.join("monokit_scope.scsyndef"),
            synthdefs_dir.join("monokit_recorder.scsyndef"),
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

        Self::spawn_voice_synths(socket, silent)?;

        Self::send_osc_message_static(
            socket,
            "/s_new",
            vec![
                OscType::String("monokit_spectrum".to_string()),
                OscType::Int(1005),
                OscType::Int(1),
                OscType::Int(0),
            ],
        )?;
        if !silent {
            eprintln!("[monokit]   Created monokit_spectrum (node 1005)");
        }

        Self::send_osc_message_static(
            socket,
            "/s_new",
            vec![
                OscType::String("monokit_scope".to_string()),
                OscType::Int(1006),
                OscType::Int(1),
                OscType::Int(0),
            ],
        )?;
        if !silent {
            eprintln!("[monokit]   Created monokit_scope (node 1006)");
        }

        thread::sleep(Duration::from_millis(500));

        // Start CPU monitoring routine (2Hz)
        if !silent {
            eprintln!("[monokit] Starting CPU monitor...");
        }
        Self::start_cpu_monitor(socket)?;

        if !silent {
            eprintln!("[monokit] SCSYNTH-DIRECT boot complete!");
        }
        Ok(())
    }

    fn start_cpu_monitor(_socket: &UdpSocket) -> Result<(), String> {
        // CPU monitoring is now handled by meter_thread via /status polling
        // This function is kept for API compatibility but does nothing
        Ok(())
    }

    fn spawn_voice_synths(socket: &UdpSocket, silent: bool) -> Result<(), String> {
        use crate::types::{VoiceSynths, PRIMARY_BUS, MOD_BUS, NOISE_BUS, PLAITS_MAIN_BUS, PLAITS_AUX_BUS};

        let synths = VoiceSynths::new();

        Self::send_osc_message_static(
            socket,
            "/s_new",
            vec![
                OscType::String("monokit_noise".to_string()),
                OscType::Int(synths.noise_node),
                OscType::Int(0),
                OscType::Int(0),
                OscType::String("noiseBus".to_string()),
                OscType::Int(NOISE_BUS),
            ],
        )?;
        if !silent {
            eprintln!("[monokit]   Created monokit_noise (node {})", synths.noise_node);
        }

        Self::send_osc_message_static(
            socket,
            "/s_new",
            vec![
                OscType::String("monokit_mod".to_string()),
                OscType::Int(synths.mod_node),
                OscType::Int(0),
                OscType::Int(0),
                OscType::String("modBus".to_string()),
                OscType::Int(MOD_BUS),
                OscType::String("noiseBus".to_string()),
                OscType::Int(NOISE_BUS),
            ],
        )?;
        if !silent {
            eprintln!("[monokit]   Created monokit_mod (node {})", synths.mod_node);
        }

        Self::send_osc_message_static(
            socket,
            "/s_new",
            vec![
                OscType::String("monokit_primary".to_string()),
                OscType::Int(synths.primary_node),
                OscType::Int(0),
                OscType::Int(0),
                OscType::String("primaryBus".to_string()),
                OscType::Int(PRIMARY_BUS),
                OscType::String("modBus".to_string()),
                OscType::Int(MOD_BUS),
                OscType::String("noiseBus".to_string()),
                OscType::Int(NOISE_BUS),
            ],
        )?;
        if !silent {
            eprintln!("[monokit]   Created monokit_primary (node {})", synths.primary_node);
        }

        Self::send_osc_message_static(
            socket,
            "/s_new",
            vec![
                OscType::String("monokit_plaits".to_string()),
                OscType::Int(synths.plaits_node),
                OscType::Int(0),
                OscType::Int(0),
                OscType::String("plaitsMainBus".to_string()),
                OscType::Int(PLAITS_MAIN_BUS),
                OscType::String("plaitsAuxBus".to_string()),
                OscType::Int(PLAITS_AUX_BUS),
            ],
        )?;
        if !silent {
            eprintln!("[monokit]   Created monokit_plaits (node {})", synths.plaits_node);
        }

        thread::sleep(Duration::from_millis(50));

        Self::send_osc_message_static(
            socket,
            "/s_new",
            vec![
                OscType::String("monokit_main".to_string()),
                OscType::Int(synths.main_node),
                OscType::Int(0),
                OscType::Int(0),
                OscType::String("primaryBus".to_string()),
                OscType::Int(PRIMARY_BUS),
                OscType::String("modBus".to_string()),
                OscType::Int(MOD_BUS),
                OscType::String("noiseBus".to_string()),
                OscType::Int(NOISE_BUS),
                OscType::String("plaitsMainBus".to_string()),
                OscType::Int(PLAITS_MAIN_BUS),
                OscType::String("plaitsAuxBus".to_string()),
                OscType::Int(PLAITS_AUX_BUS),
            ],
        )?;
        if !silent {
            eprintln!("[monokit]   Created monokit_main (node {})", synths.main_node);
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

    /// Start recording audio to a file using DiskOut UGen
    pub fn start_recording(&mut self, dir: &str, _custom_path: Option<&str>) -> Result<(), String> {
        if self.is_recording {
            return Err("Already recording".to_string());
        }

        // Generate timestamp in YYMMDD_HHMMSS format (matches SC Date.getDate.stamp)
        let timestamp = Local::now().format("%y%m%d_%H%M%S").to_string();

        // Generate file path
        let file_path = if let Some(ref prefix) = self.recording_path_prefix {
            format!("{}_{}.wav", prefix, timestamp)
        } else {
            format!("{}/monokit_{}.wav", dir, timestamp)
        };

        // Create socket for OSC communication
        let socket = UdpSocket::bind("127.0.0.1:0")
            .map_err(|e| format!("Failed to bind socket for recording: {}", e))?;

        socket.set_read_timeout(Some(Duration::from_secs(5)))
            .map_err(|e| format!("Failed to set socket timeout: {}", e))?;

        // 1. Allocate buffer (bufnum=99, numFrames=65536, numChannels=2)
        Self::send_osc_message_static(
            &socket,
            "/b_alloc",
            vec![
                OscType::Int(99),
                OscType::Int(65536),
                OscType::Int(2),
            ],
        )?;

        // Wait for buffer allocation to complete (increased from 100ms)
        thread::sleep(Duration::from_millis(200));

        // 2. Open file for streaming recording via DiskOut
        // /b_write with leaveOpen=1 opens file for continuous writes by DiskOut UGen
        Self::send_osc_message_static(
            &socket,
            "/b_write",
            vec![
                OscType::Int(99),                           // buffer number
                OscType::String(file_path),                 // path
                OscType::String("wav".to_string()),         // header format
                OscType::String("int24".to_string()),       // sample format
                OscType::Int(-1),                           // numFrames (-1 = all)
                OscType::Int(0),                            // startFrame
                OscType::Int(1),                            // leaveOpen = 1 for DiskOut
            ],
        )?;

        // Wait for file to be opened (increased from 100ms)
        thread::sleep(Duration::from_millis(200));

        // 3. Create recorder synth (/s_new "monokit_recorder", nodeID=2000, addAction=1, targetID=0)
        Self::send_osc_message_static(
            &socket,
            "/s_new",
            vec![
                OscType::String("monokit_recorder".to_string()),
                OscType::Int(2000),
                OscType::Int(1),  // addAction=1 (addToTail)
                OscType::Int(0),  // targetID=0 (default group)
                OscType::String("bufnum".to_string()),
                OscType::Int(99),
            ],
        )?;

        self.is_recording = true;

        Ok(())
    }

    /// Stop recording and close the file
    pub fn stop_recording(&mut self) -> Result<(), String> {
        if !self.is_recording {
            return Err("Not currently recording".to_string());
        }

        // Create socket for OSC communication
        let socket = UdpSocket::bind("127.0.0.1:0")
            .map_err(|e| format!("Failed to bind socket for recording: {}", e))?;

        // 1. Free recorder synth node
        Self::send_osc_message_static(
            &socket,
            "/n_free",
            vec![OscType::Int(2000)],
        )?;

        thread::sleep(Duration::from_millis(50));

        // 2. Close buffer file
        Self::send_osc_message_static(
            &socket,
            "/b_close",
            vec![OscType::Int(99)],
        )?;

        thread::sleep(Duration::from_millis(50));

        // 3. Free buffer
        Self::send_osc_message_static(
            &socket,
            "/b_free",
            vec![OscType::Int(99)],
        )?;

        self.is_recording = false;

        Ok(())
    }

    /// Set custom recording path prefix
    pub fn set_recording_path_prefix(&mut self, prefix: String) {
        self.recording_path_prefix = Some(prefix);
    }
}

impl Drop for ScsynthDirect {
    fn drop(&mut self) {
        self.stop();
    }
}

/// Get the directory containing the real executable (resolving symlinks)
fn get_exe_dir() -> Option<PathBuf> {
    if let Ok(exe) = env::current_exe() {
        // Resolve symlinks to find the real binary location
        let real_exe = fs::canonicalize(&exe).unwrap_or(exe);
        return real_exe.parent().map(|p| p.to_path_buf());
    }
    None
}

fn find_scsynth() -> Result<PathBuf, String> {
    // Check for bundled scsynth (in Resources/ subdirectory for framework path resolution)
    if let Some(dir) = get_exe_dir() {
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
    if let Some(dir) = get_exe_dir() {
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
    if let Some(dir) = get_exe_dir() {
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
