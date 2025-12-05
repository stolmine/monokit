use std::env;
use std::io::Write;
use std::path::PathBuf;
use std::process::{Child, ChildStdin, Command, Stdio};
use std::time::Duration;
use std::thread;

#[cfg(feature = "scsynth-direct")]
use crate::scsynth_direct::ScsynthDirect;

pub struct ScProcess {
    #[cfg(feature = "scsynth-direct")]
    scsynth_direct: Option<ScsynthDirect>,

    #[cfg(not(feature = "scsynth-direct"))]
    child: Option<Child>,
    #[cfg(not(feature = "scsynth-direct"))]
    stdin: Option<ChildStdin>,
    #[cfg(not(feature = "scsynth-direct"))]
    sclang_path: PathBuf,
    #[cfg(not(feature = "scsynth-direct"))]
    script_path: PathBuf,
}

impl ScProcess {
    #[cfg(feature = "scsynth-direct")]
    pub fn new() -> Result<Self, String> {
        let scsynth = ScsynthDirect::new()?;
        Ok(Self {
            scsynth_direct: Some(scsynth),
        })
    }

    #[cfg(not(feature = "scsynth-direct"))]
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

    #[cfg(feature = "scsynth-direct")]
    pub fn start(&mut self, audio_out_device: Option<&str>) -> Result<(), String> {
        if let Some(ref mut scsynth) = self.scsynth_direct {
            scsynth.start(audio_out_device)
        } else {
            Err("scsynth not initialized".to_string())
        }
    }

    #[cfg(not(feature = "scsynth-direct"))]
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

        self.stdin = child.stdin.take();
        self.child = Some(child);
        Ok(())
    }

    #[cfg(feature = "scsynth-direct")]
    pub fn stop(&mut self) {
        if let Some(ref mut scsynth) = self.scsynth_direct {
            scsynth.stop();
        }
    }

    #[cfg(not(feature = "scsynth-direct"))]
    pub fn stop(&mut self) {
        if let Some(mut stdin) = self.stdin.take() {
            let _ = stdin.write_all(b"Server.quitAll; 0.exit;\n");
            let _ = stdin.write_all(&[0x1b]);
            let _ = stdin.flush();

            thread::sleep(Duration::from_millis(500));
        }

        if let Some(mut child) = self.child.take() {
            match child.try_wait() {
                Ok(Some(_)) => {}
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
    }

    #[cfg(feature = "scsynth-direct")]
    pub fn restart_with_device(&mut self, device: &str) -> Result<(), String> {
        if let Some(ref mut scsynth) = self.scsynth_direct {
            scsynth.restart_with_device(device)
        } else {
            Err("scsynth not initialized".to_string())
        }
    }

    #[cfg(not(feature = "scsynth-direct"))]
    pub fn restart_with_device(&mut self, device: &str) -> Result<(), String> {
        self.start(Some(device))
    }

    #[cfg(feature = "scsynth-direct")]
    pub fn spawn_ready_sender(&self) -> Option<std::sync::mpsc::Receiver<()>> {
        if let Some(ref scsynth) = self.scsynth_direct {
            Some(scsynth.send_ready())
        } else {
            None
        }
    }

    #[cfg(not(feature = "scsynth-direct"))]
    pub fn spawn_ready_sender(&self) -> Option<std::sync::mpsc::Receiver<()>> {
        None
    }

    #[cfg(feature = "scsynth-direct")]
    pub fn is_running(&self) -> bool {
        self.scsynth_direct.as_ref().map_or(false, |s| s.is_running())
    }

    #[cfg(not(feature = "scsynth-direct"))]
    pub fn is_running(&self) -> bool {
        self.child.is_some()
    }

    /// Start recording (scsynth-direct mode only)
    /// In sclang mode, recording is handled via OSC messages to sclang
    #[cfg(feature = "scsynth-direct")]
    pub fn start_recording(&self, dir: &str, custom_path: Option<&str>) -> Result<(), String> {
        if let Some(ref scsynth) = self.scsynth_direct {
            scsynth.start_recording(dir, custom_path)
        } else {
            Err("scsynth not initialized".to_string())
        }
    }

    /// Stop recording (scsynth-direct mode only)
    /// In sclang mode, recording is handled via OSC messages to sclang
    #[cfg(feature = "scsynth-direct")]
    pub fn stop_recording(&self) -> Result<(), String> {
        if let Some(ref scsynth) = self.scsynth_direct {
            scsynth.stop_recording()
        } else {
            Err("scsynth not initialized".to_string())
        }
    }
}

impl Drop for ScProcess {
    fn drop(&mut self) {
        self.stop();
    }
}

#[cfg(not(feature = "scsynth-direct"))]
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

#[cfg(not(feature = "scsynth-direct"))]
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

    if let Some(config_dir) = dirs::config_dir() {
        let script = config_dir.join("monokit/monokit_server.scd");
        if script.exists() {
            return Ok(script);
        }
    }

    // Homebrew install location (macOS)
    let homebrew_paths = [
        "/opt/homebrew/share/monokit/sc/monokit_server.scd",  // Apple Silicon
        "/usr/local/share/monokit/sc/monokit_server.scd",     // Intel Mac
    ];
    for path in homebrew_paths {
        let script = PathBuf::from(path);
        if script.exists() {
            return Ok(script);
        }
    }

    Err("monokit_server.scd not found".to_string())
}
