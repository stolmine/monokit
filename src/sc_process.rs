use std::env;
use std::io::Write;
use std::path::PathBuf;
use std::process::{Child, ChildStdin, Command, Stdio};
use std::time::Duration;
use std::thread;

pub struct ScProcess {
    child: Option<Child>,
    stdin: Option<ChildStdin>,
    sclang_path: PathBuf,
    script_path: PathBuf,
}

impl ScProcess {
    pub fn new() -> Result<Self, String> {
        let sclang_path = find_sclang()?;
        let script_path = find_script()?;

        Ok(Self {
            child: None,
            stdin: None,
            sclang_path,
            script_path,
        })
    }

    pub fn start(&mut self, audio_out_device: Option<&str>) -> Result<(), String> {
        self.stop();

        let mut cmd = Command::new(&self.sclang_path);
        cmd.arg(&self.script_path);
        cmd.stdin(Stdio::piped());
        cmd.stdout(Stdio::piped());
        cmd.stderr(Stdio::piped());

        if let Some(device) = audio_out_device {
            cmd.env("MONOKIT_AUDIO_OUT", device);
        }

        let mut child = cmd
            .spawn()
            .map_err(|e| format!("Failed to spawn sclang: {}", e))?;

        // Capture stdin for graceful shutdown
        self.stdin = child.stdin.take();
        self.child = Some(child);
        Ok(())
    }

    pub fn stop(&mut self) {
        // First, try graceful shutdown via stdin
        if let Some(mut stdin) = self.stdin.take() {
            // Send quit command to sclang - this shuts down scsynth properly
            let _ = stdin.write_all(b"Server.quitAll; 0.exit;\n");
            let _ = stdin.write_all(&[0x1b]); // Execute token
            let _ = stdin.flush();

            // Give it a moment to shut down gracefully
            thread::sleep(Duration::from_millis(500));
        }

        if let Some(mut child) = self.child.take() {
            // Check if still running, kill if necessary
            match child.try_wait() {
                Ok(Some(_)) => {
                    // Already exited
                }
                Ok(None) => {
                    // Still running, force kill
                    let _ = child.kill();
                    let _ = child.wait();
                }
                Err(_) => {
                    let _ = child.kill();
                    let _ = child.wait();
                }
            }

            // Extra delay to ensure audio device is released
            thread::sleep(Duration::from_millis(300));
        }

        // Also kill any orphaned scsynth processes (belt and suspenders)
        let _ = Command::new("pkill").arg("-f").arg("scsynth").output();
        thread::sleep(Duration::from_millis(200));
    }

    pub fn restart_with_device(&mut self, device: &str) -> Result<(), String> {
        self.start(Some(device))
    }

    pub fn is_running(&self) -> bool {
        self.child.is_some()
    }
}

impl Drop for ScProcess {
    fn drop(&mut self) {
        self.stop();
    }
}

fn find_sclang() -> Result<PathBuf, String> {
    let candidates = [
        "/Applications/SuperCollider.app/Contents/MacOS/sclang",
        "/opt/homebrew/bin/sclang",
        "/usr/local/bin/sclang",
    ];

    for path in candidates {
        let p = PathBuf::from(path);
        if p.exists() {
            return Ok(p);
        }
    }

    if let Ok(output) = Command::new("which").arg("sclang").output() {
        if output.status.success() {
            let path = String::from_utf8_lossy(&output.stdout).trim().to_string();
            if !path.is_empty() {
                return Ok(PathBuf::from(path));
            }
        }
    }

    Err("sclang not found. Install SuperCollider.".to_string())
}

fn find_script() -> Result<PathBuf, String> {
    if let Ok(exe) = env::current_exe() {
        if let Some(dir) = exe.parent() {
            let script = dir.join("sc/monokit_server.scd");
            if script.exists() {
                return Ok(script);
            }
            if let Some(parent) = dir.parent() {
                let script = parent.join("sc/monokit_server.scd");
                if script.exists() {
                    return Ok(script);
                }
            }
        }
    }

    let cwd = PathBuf::from("sc/monokit_server.scd");
    if cwd.exists() {
        return Ok(cwd);
    }

    if let Some(home) = dirs::home_dir() {
        let script = home.join(".monokit/monokit_server.scd");
        if script.exists() {
            return Ok(script);
        }
    }

    Err("monokit_server.scd not found".to_string())
}
