Here's a comprehensive setup plan for monokit on Linux (Ubuntu/Raspberry Pi CM4):

  Build Environment Setup Steps

  1. Install System Dependencies

  sudo apt update
  sudo apt install -y build-essential git pkg-config libasound2-dev

  2. Install Rust

  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  source $HOME/.cargo/env
  rustc --version  # verify

  3. Install SuperCollider

  sudo apt install -y supercollider supercollider-server
  scsynth -v  # verify scsynth
  sclang -v   # verify sclang

  4. Clone monokit Repository

  cd ~
  git clone https://github.com/[your-username]/monokit.git  # or your fork
  cd monokit

  5. Install sc3-plugins

  sudo apt install -y sc3-plugins
  # Verify location
  find /usr -name "sc3-plugins" 2>/dev/null

  6. Install mi-UGens (for Plaits)

  # Download mi-UGens v0.0.8
  wget https://github.com/v7b1/mi-UGens/releases/download/v0.0.8/mi-UGens_v0.0.8_linux.zip
  unzip mi-UGens_v0.0.8_linux.zip

  # Find SC extensions directory
  sclang -e "Platform.userExtensionDir.postln; 0.exit"

  # Copy to SC extensions (adjust path as needed)
  mkdir -p ~/.local/share/SuperCollider/Extensions
  cp -r mi-UGens/* ~/.local/share/SuperCollider/Extensions/

  7. Compile SuperCollider SynthDefs

  cd ~/monokit
  sclang sc/compile_synthdefs.scd
  # Check for .scsyndef files in sc/synthdefs/
  ls -l sc/synthdefs/

  8. Build monokit

  cargo build --release
  # Binary will be at: target/release/monokit

  9. Test Basic Functionality

  # Dry run (no audio)
  ./target/release/monokit --dry-run

  # If that works, try with audio
  ./target/release/monokit

  10. Platform-Specific Considerations

  - Audio Backend: Linux uses ALSA/JACK - may need JACK if ALSA has issues
  - ARM64: Verify all dependencies compile for aarch64
  - Permissions: May need to add user to audio group: sudo usermod -aG audio $USER

  Potential Issues to Watch For

  1. sc3-plugins architecture - May need to compile from source if ARM64 binaries unavailable
  2. mi-UGens availability - Check if Linux ARM64 build exists
  3. Audio device access - Permission issues with ALSA
  4. Path differences - SuperCollider extension paths differ on Linux
