# SuperCollider sclang Integration Research

## Overview
This document provides comprehensive research on spawning and managing the SuperCollider language interpreter (sclang) as a child process from Rust on macOS, with cross-platform considerations.

---

## 1. sclang Binary Location on macOS

### Standard Installation Paths

**macOS Application Bundle (most common):**
```
/Applications/SuperCollider.app/Contents/MacOS/sclang
/Applications/SuperCollider.app/Contents/Resources/scsynth
```

**Homebrew Installation:**
```bash
# Intel Macs
/usr/local/bin/sclang

# Apple Silicon Macs (M1/M2/M3)
/opt/homebrew/bin/sclang
```

### Detection Strategy

```rust
use std::path::PathBuf;
use std::process::Command;

/// Attempts to find sclang binary on macOS
fn find_sclang_macos() -> Option<PathBuf> {
    let candidates = vec![
        PathBuf::from("/Applications/SuperCollider.app/Contents/MacOS/sclang"),
        PathBuf::from("/usr/local/bin/sclang"),
        PathBuf::from("/opt/homebrew/bin/sclang"),
    ];

    for path in candidates {
        if path.exists() {
            return Some(path);
        }
    }

    // Try to find via which command
    if let Ok(output) = Command::new("which").arg("sclang").output() {
        if output.status.success() {
            let path_str = String::from_utf8_lossy(&output.stdout).trim().to_string();
            if !path_str.is_empty() {
                return Some(PathBuf::from(path_str));
            }
        }
    }

    None
}

/// Cross-platform sclang finder
fn find_sclang() -> Option<PathBuf> {
    #[cfg(target_os = "macos")]
    return find_sclang_macos();

    #[cfg(target_os = "linux")]
    return find_sclang_linux();

    #[cfg(target_os = "windows")]
    return find_sclang_windows();
}

#[cfg(target_os = "linux")]
fn find_sclang_linux() -> Option<PathBuf> {
    let candidates = vec![
        PathBuf::from("/usr/bin/sclang"),
        PathBuf::from("/usr/local/bin/sclang"),
    ];

    candidates.into_iter().find(|p| p.exists())
}

#[cfg(target_os = "windows")]
fn find_sclang_windows() -> Option<PathBuf> {
    // Windows typically installs to Program Files
    let candidates = vec![
        PathBuf::from("C:\\Program Files\\SuperCollider\\sclang.exe"),
        PathBuf::from("C:\\Program Files (x86)\\SuperCollider\\sclang.exe"),
    ];

    candidates.into_iter().find(|p| p.exists())
}
```

### Environment Variables

SuperCollider respects these environment variables:

- **`SC_SYNTHDEF_PATH`** - Additional directories for synth definitions
- **`SC_PLUGIN_PATH`** - Additional directories for UGen plugins
- **`SC_JACK_DEFAULT_INPUTS`** (Linux) - Default JACK input connections
- **`SC_JACK_DEFAULT_OUTPUTS`** (Linux) - Default JACK output connections

---

## 2. Spawning sclang with a Script

### Command Line Syntax

```bash
# Execute a script file (will hang at end without 0.exit)
sclang script.scd

# Execute with explicit path
/Applications/SuperCollider.app/Contents/MacOS/sclang /path/to/script.scd

# Pass command line arguments (accessible via thisProcess.argv)
sclang script.scd arg1 arg2 arg3
```

### Behavior Characteristics

**Critical Issue:** When you run `sclang script.scd`, it:
1. Compiles the class library
2. Executes your script
3. **HANGS** waiting for interactive input (does NOT exit)

**Workarounds:**
1. Add `0.exit;` to end of script (only works if no compilation errors)
2. Use timeout wrapper
3. Monitor for specific output and kill process

### Example Script Structure

```supercollider
// my_server.scd
// This script boots the SC server and keeps it running

// Boot the server
s.boot;

// Wait for boot, then set up ready notification
s.waitForBoot({
    // Send a message that Rust can detect
    "SC_SERVER_READY".postln;

    // Set up your synths, routes, etc.
    SynthDef(\monokit_voice, {
        // Your synth definition
    }).add;

    // Keep running - do NOT call 0.exit if you want server to stay alive
});

// The script will hang here, keeping sclang running
// This is DESIRED behavior for a server process
```

### Rust Implementation

```rust
use std::process::{Command, Stdio, Child};
use std::io::{BufRead, BufReader};
use std::thread;
use std::sync::mpsc::{channel, Sender, Receiver};
use anyhow::{Result, Context};

pub struct SclangProcess {
    child: Child,
    output_rx: Receiver<String>,
}

impl SclangProcess {
    /// Spawn sclang with a script file
    pub fn spawn(sclang_path: &str, script_path: &str, args: Vec<String>) -> Result<Self> {
        let mut child = Command::new(sclang_path)
            .arg(script_path)
            .args(&args)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .context("Failed to spawn sclang process")?;

        // Take stdout for monitoring
        let stdout = child.stdout.take()
            .context("Failed to capture sclang stdout")?;

        // Create channel for output
        let (tx, rx) = channel();

        // Spawn thread to read output
        thread::spawn(move || {
            let reader = BufReader::new(stdout);
            for line in reader.lines() {
                if let Ok(line) = line {
                    // Send to channel (ignore errors if receiver dropped)
                    let _ = tx.send(line);
                }
            }
        });

        Ok(SclangProcess {
            child,
            output_rx: rx,
        })
    }

    /// Wait for specific output string (with timeout)
    pub fn wait_for_output(&self, target: &str, timeout_ms: u64) -> Result<()> {
        use std::time::{Duration, Instant};

        let deadline = Instant::now() + Duration::from_millis(timeout_ms);

        while Instant::now() < deadline {
            // Non-blocking receive
            if let Ok(line) = self.output_rx.try_recv() {
                println!("sclang: {}", line);
                if line.contains(target) {
                    return Ok(());
                }
            }
            thread::sleep(Duration::from_millis(10));
        }

        anyhow::bail!("Timeout waiting for sclang output: {}", target)
    }

    /// Send code to sclang via stdin
    pub fn send_code(&mut self, code: &str) -> Result<()> {
        use std::io::Write;

        if let Some(stdin) = self.child.stdin.as_mut() {
            // SuperCollider terminal mode expects 0x1b after code to execute
            stdin.write_all(code.as_bytes())?;
            stdin.write_all(&[0x1b])?; // Execute token
            stdin.flush()?;
            Ok(())
        } else {
            anyhow::bail!("stdin not available")
        }
    }

    /// Kill the process
    pub fn kill(&mut self) -> Result<()> {
        self.child.kill().context("Failed to kill sclang process")
    }

    /// Check if process is still running
    pub fn is_running(&mut self) -> bool {
        self.child.try_wait().ok().flatten().is_none()
    }
}

impl Drop for SclangProcess {
    fn drop(&mut self) {
        // Ensure child is killed when SclangProcess is dropped
        let _ = self.child.kill();
    }
}
```

---

## 3. Passing Configuration to sclang

### Environment Variables

SuperCollider can read environment variables using `String.getenv`:

```supercollider
// In your .scd script
var portNum = "MONOKIT_OSC_PORT".getenv;
if (portNum.isNil, {
    portNum = "57120"; // Default
});
portNum = portNum.asInteger;
```

**Rust side:**

```rust
use std::process::Command;
use std::env;

let mut cmd = Command::new(sclang_path);
cmd.arg(script_path)
    .env("MONOKIT_OSC_PORT", "57120")
    .env("MONOKIT_BUFFER_SIZE", "512")
    .env("MONOKIT_SAMPLE_RATE", "48000");
```

### Command Line Arguments

Arguments are accessible via `thisProcess.argv`:

```supercollider
// In script.scd
var args = thisProcess.argv;
if (args.size > 0, {
    ("Port: " ++ args[0]).postln;
});
```

**Rust side:**

```rust
SclangProcess::spawn(
    sclang_path,
    script_path,
    vec!["57120".to_string(), "512".to_string()]
)?;
```

### Configuration Files

SuperCollider has startup files:

- **macOS:** `~/Library/Application Support/SuperCollider/startup.scd`
- **Linux:** `~/.config/SuperCollider/startup.scd`
- **Windows:** `%LOCALAPPDATA%\SuperCollider\startup.scd`

However, for your use case, **environment variables** are recommended because:
- They don't interfere with user's global SC setup
- They're process-specific
- They're easy to change per-instance

---

## 4. Detecting When SC Server is Ready

### Method 1: stdout Detection (Simplest)

Have your SuperCollider script print a specific marker:

```supercollider
// In your .scd file
s.waitForBoot({
    "MONOKIT_SC_READY".postln;
    // Now set up synths, etc.
});
```

**Rust side:**

```rust
let mut sclang = SclangProcess::spawn(sclang_path, script_path, vec![])?;

// Wait up to 10 seconds for ready signal
sclang.wait_for_output("MONOKIT_SC_READY", 10000)?;

println!("SuperCollider server is ready!");
```

### Method 2: OSC Notification (Most Robust)

Send an OSC message back to your Rust application:

```supercollider
// In your .scd file
s.waitForBoot({
    var monokit_addr = NetAddr("127.0.0.1", "MONOKIT_OSC_PORT".getenv.asInteger);

    // Send ready notification
    monokit_addr.sendMsg("/sc/ready", 1);

    "Server booted, notification sent".postln;
});
```

**Rust side (using rosc, which you already have):**

```rust
use rosc::{OscPacket, OscMessage};
use std::net::UdpSocket;
use std::time::Duration;

fn wait_for_sc_ready(socket: &UdpSocket, timeout_secs: u64) -> Result<()> {
    use std::time::Instant;

    socket.set_read_timeout(Some(Duration::from_secs(1)))?;
    let deadline = Instant::now() + Duration::from_secs(timeout_secs);

    let mut buf = [0u8; rosc::decoder::MTU];

    while Instant::now() < deadline {
        match socket.recv_from(&mut buf) {
            Ok((size, _addr)) => {
                if let Ok(packet) = rosc::decoder::decode_udp(&buf[..size]) {
                    if let OscPacket::Message(msg) = packet.1 {
                        if msg.addr == "/sc/ready" {
                            println!("Received SC ready notification");
                            return Ok(());
                        }
                    }
                }
            }
            Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                // Timeout on this receive, continue loop
                continue;
            }
            Err(e) => return Err(e.into()),
        }
    }

    anyhow::bail!("Timeout waiting for SC ready notification")
}
```

### Method 3: OSC Status Polling

Query the server directly:

```supercollider
// SuperCollider automatically responds to /status with /status.reply
```

**Rust side:**

```rust
use rosc::{OscPacket, OscMessage, OscType};

fn poll_sc_status(sc_addr: &str, listen_port: u16) -> Result<()> {
    let socket = UdpSocket::bind(format!("127.0.0.1:{}", listen_port))?;
    socket.set_read_timeout(Some(Duration::from_secs(1)))?;

    let status_msg = rosc::encoder::encode(&OscPacket::Message(OscMessage {
        addr: "/status".to_string(),
        args: vec![],
    }))?;

    for _ in 0..10 {
        socket.send_to(&status_msg, sc_addr)?;

        let mut buf = [0u8; rosc::decoder::MTU];
        if let Ok((size, _)) = socket.recv_from(&mut buf) {
            if let Ok(packet) = rosc::decoder::decode_udp(&buf[..size]) {
                if let OscPacket::Message(msg) = packet.1 {
                    if msg.addr == "/status.reply" {
                        println!("SC server is responding");
                        return Ok(());
                    }
                }
            }
        }

        thread::sleep(Duration::from_millis(500));
    }

    anyhow::bail!("SC server not responding to /status")
}
```

### Typical Boot Times

- **Fast machines (SSD, modern CPU):** 2-5 seconds
- **Slower machines:** 5-10 seconds
- **With many plugins/extensions:** 10-15 seconds

**Recommendation:** Use 15-20 second timeout to be safe.

---

## 5. Process Management in Rust

### Complete Example

```rust
use std::process::{Command, Stdio, Child};
use std::io::{BufRead, BufReader, Write};
use std::thread;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use anyhow::{Result, Context};

pub struct SuperColliderManager {
    process: Option<Child>,
    stdin_handle: Option<Arc<Mutex<std::process::ChildStdin>>>,
    output_lines: Arc<Mutex<Vec<String>>>,
    is_ready: Arc<Mutex<bool>>,
}

impl SuperColliderManager {
    pub fn new() -> Self {
        SuperColliderManager {
            process: None,
            stdin_handle: None,
            output_lines: Arc::new(Mutex::new(Vec::new())),
            is_ready: Arc::new(Mutex::new(false)),
        }
    }

    /// Start SuperCollider with the given script
    pub fn start(&mut self, sclang_path: &str, script_path: &str) -> Result<()> {
        let mut child = Command::new(sclang_path)
            .arg(script_path)
            .env("MONOKIT_OSC_PORT", "57120")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .context("Failed to spawn sclang")?;

        // Capture stdin
        let stdin = child.stdin.take()
            .context("Failed to open stdin")?;
        self.stdin_handle = Some(Arc::new(Mutex::new(stdin)));

        // Capture stdout
        let stdout = child.stdout.take()
            .context("Failed to open stdout")?;

        // Capture stderr
        let stderr = child.stderr.take()
            .context("Failed to open stderr")?;

        // Spawn stdout reader thread
        let output_lines = self.output_lines.clone();
        let is_ready = self.is_ready.clone();
        thread::spawn(move || {
            let reader = BufReader::new(stdout);
            for line in reader.lines().flatten() {
                println!("[sclang stdout] {}", line);

                // Check for ready marker
                if line.contains("MONOKIT_SC_READY") {
                    *is_ready.lock().unwrap() = true;
                }

                // Store line
                output_lines.lock().unwrap().push(line);
            }
        });

        // Spawn stderr reader thread
        thread::spawn(move || {
            let reader = BufReader::new(stderr);
            for line in reader.lines().flatten() {
                eprintln!("[sclang stderr] {}", line);
            }
        });

        self.process = Some(child);
        Ok(())
    }

    /// Wait for SC to be ready (with timeout)
    pub fn wait_for_ready(&self, timeout_secs: u64) -> Result<()> {
        let deadline = Instant::now() + Duration::from_secs(timeout_secs);

        while Instant::now() < deadline {
            if *self.is_ready.lock().unwrap() {
                return Ok(());
            }
            thread::sleep(Duration::from_millis(100));
        }

        anyhow::bail!("Timeout waiting for SuperCollider to be ready")
    }

    /// Send code to sclang (for interactive commands)
    pub fn execute_code(&self, code: &str) -> Result<()> {
        if let Some(stdin_arc) = &self.stdin_handle {
            let mut stdin = stdin_arc.lock().unwrap();
            stdin.write_all(code.as_bytes())?;
            stdin.write_all(b"\n")?;
            stdin.write_all(&[0x1b])?; // Execute token
            stdin.flush()?;
            Ok(())
        } else {
            anyhow::bail!("sclang not started")
        }
    }

    /// Check if process is still alive
    pub fn is_running(&mut self) -> bool {
        if let Some(child) = &mut self.process {
            child.try_wait().ok().flatten().is_none()
        } else {
            false
        }
    }

    /// Get recent output lines
    pub fn get_output(&self, last_n: usize) -> Vec<String> {
        let lines = self.output_lines.lock().unwrap();
        let start = lines.len().saturating_sub(last_n);
        lines[start..].to_vec()
    }

    /// Shutdown sclang gracefully
    pub fn shutdown(&mut self) -> Result<()> {
        // Try graceful shutdown first
        if let Some(stdin_arc) = &self.stdin_handle {
            let mut stdin = stdin_arc.lock().unwrap();
            let _ = stdin.write_all(b"0.exit;\n");
            let _ = stdin.write_all(&[0x1b]);
            let _ = stdin.flush();
        }

        // Wait briefly for graceful exit
        thread::sleep(Duration::from_millis(500));

        // Force kill if still running
        if let Some(child) = &mut self.process {
            if child.try_wait()?.is_none() {
                child.kill()?;
            }
        }

        self.process = None;
        Ok(())
    }
}

impl Drop for SuperColliderManager {
    fn drop(&mut self) {
        // Ensure cleanup on drop
        let _ = self.shutdown();
    }
}
```

### Usage Example

```rust
fn main() -> Result<()> {
    // Find sclang
    let sclang_path = find_sclang()
        .context("Could not find sclang binary")?;

    println!("Found sclang at: {}", sclang_path.display());

    // Create manager
    let mut sc_manager = SuperColliderManager::new();

    // Start with script
    sc_manager.start(
        sclang_path.to_str().unwrap(),
        "/path/to/monokit_server.scd"
    )?;

    println!("Waiting for SuperCollider to boot...");
    sc_manager.wait_for_ready(15)?;
    println!("SuperCollider is ready!");

    // Now you can send OSC messages to scsynth
    // or execute code via sc_manager.execute_code()

    // ... your application logic ...

    // Cleanup
    sc_manager.shutdown()?;

    Ok(())
}
```

---

## 6. Cross-Platform Considerations

### Binary Locations

| Platform | Typical Locations |
|----------|-------------------|
| **macOS** | `/Applications/SuperCollider.app/Contents/MacOS/sclang`<br>`/usr/local/bin/sclang` (Homebrew Intel)<br>`/opt/homebrew/bin/sclang` (Homebrew ARM) |
| **Linux** | `/usr/bin/sclang`<br>`/usr/local/bin/sclang`<br>`~/bin/sclang` |
| **Windows** | `C:\Program Files\SuperCollider\sclang.exe`<br>`C:\Program Files (x86)\SuperCollider\sclang.exe` |

### Config File Locations

| Platform | Startup File Location |
|----------|----------------------|
| **macOS** | `~/Library/Application Support/SuperCollider/startup.scd` |
| **Linux** | `~/.config/SuperCollider/startup.scd` |
| **Windows** | `%LOCALAPPDATA%\SuperCollider\startup.scd` |

### Platform-Specific Code

```rust
fn get_config_dir() -> PathBuf {
    #[cfg(target_os = "macos")]
    {
        let home = env::var("HOME").unwrap();
        PathBuf::from(format!("{}/Library/Application Support/SuperCollider", home))
    }

    #[cfg(target_os = "linux")]
    {
        let home = env::var("HOME").unwrap();
        PathBuf::from(format!("{}/.config/SuperCollider", home))
    }

    #[cfg(target_os = "windows")]
    {
        let local_app_data = env::var("LOCALAPPDATA").unwrap();
        PathBuf::from(format!("{}\\SuperCollider", local_app_data))
    }
}
```

### Process Management Differences

**macOS/Linux:**
- Use SIGTERM for graceful shutdown: `child.kill()` sends SIGKILL
- For graceful: use `nix` crate to send SIGTERM first

**Windows:**
- No signal system
- Rely on stdin commands (`0.exit;`) for graceful shutdown

---

## 7. Recommended Architecture for Monokit

Based on your existing codebase structure, here's the recommended integration:

### File Structure

```
src/
├── commands/
│   └── system/
│       ├── sc.rs           (existing - expand for process management)
│       └── mod.rs
├── sc_process/             (new module)
│   ├── mod.rs              (SuperColliderManager)
│   ├── finder.rs           (find_sclang functions)
│   └── ready_detector.rs   (readiness detection strategies)
└── main.rs
```

### Integration Pattern

```rust
// In your main application struct
pub struct MonokitApp {
    // ... existing fields ...
    sc_manager: Option<SuperColliderManager>,
    sc_ready: bool,
}

impl MonokitApp {
    pub fn init_supercollider(&mut self) -> Result<()> {
        let sclang_path = find_sclang()
            .context("SuperCollider not found")?;

        let script_path = /* path to your monokit.scd */;

        let mut sc_manager = SuperColliderManager::new();
        sc_manager.start(
            sclang_path.to_str().unwrap(),
            script_path
        )?;

        // Wait in background thread to not block UI
        let is_ready = Arc::new(AtomicBool::new(false));
        let is_ready_clone = is_ready.clone();

        thread::spawn(move || {
            if sc_manager.wait_for_ready(15).is_ok() {
                is_ready_clone.store(true, Ordering::SeqCst);
            }
        });

        self.sc_manager = Some(sc_manager);
        Ok(())
    }
}
```

---

## 8. Key Takeaways

1. **sclang hangs by design** - This is expected behavior; add `0.exit;` only if you want it to terminate
2. **Use environment variables** - Cleanest way to pass config without affecting user's SC setup
3. **stdout detection is simplest** - Watch for a custom marker string like "MONOKIT_SC_READY"
4. **OSC is most robust** - Have SC send `/sc/ready` message back to your app
5. **Always handle cleanup** - Implement Drop to kill process on panic/exit
6. **Boot time varies** - Allow 15-20 second timeout
7. **Cross-platform paths differ** - Use conditional compilation or runtime detection

---

## Sources

- [Calling Supercollider script from Windows batch file](https://scsynth.org/t/calling-supercollider-script-from-windows-batch-file/2964)
- [How do I manually start up scsynth and sclang? - Stack Overflow](https://stackoverflow.com/questions/31605159/how-do-i-manually-start-up-scsynth-and-sclang)
- [Running "sclang foo.scd" hangs by default · Issue #3393](https://github.com/supercollider/supercollider/issues/3393)
- [Main | SuperCollider 3.14.0-dev Help](https://doc.sccode.org/Classes/Main.html)
- [Getenv | SuperCollider 3.14.0 Help](https://doc.sccode.org/Classes/Getenv.html)
- [Server Guide | SuperCollider 3.14.0 Help](https://doc.sccode.org/Guides/Server-Guide.html)
- [Scripting sclang via external process](https://scsynth.org/t/scripting-sclang-via-external-process/2430)
