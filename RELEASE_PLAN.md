# Monokit Release Plan

## Release & Homebrew Distribution Steps

### 1. Prepare Source Repo

```bash
# Ensure clean state
cargo test
cargo build --release
git status  # should be clean or commit pending changes
```

### 2. Tag a Release

```bash
git tag -a v0.1.0 -m "Initial release"
git push origin v0.1.0
```

### 3. Build Release Binaries

```bash
# For your Mac (Apple Silicon)
cargo build --release
mkdir -p dist
cp target/release/monokit dist/
cp -r sc dist/
cd dist && tar -czvf monokit-0.1.0-aarch64-apple-darwin.tar.gz monokit sc
shasum -a 256 monokit-0.1.0-aarch64-apple-darwin.tar.gz
# Save this hash for the formula
```

### 4. Create GitHub Release

1. Go to `github.com/yourusername/monokit/releases`
2. Click "Create a new release"
3. Select tag `v0.1.0`
4. Title: `v0.1.0`
5. Upload `monokit-0.1.0-aarch64-apple-darwin.tar.gz`
6. Publish

### 5. Create Homebrew Tap Repo

```bash
# Create new repo: github.com/yourusername/homebrew-monokit
mkdir homebrew-monokit && cd homebrew-monokit
git init
mkdir Formula
```

### 6. Write Formula

Create `Formula/monokit.rb`:
```ruby
class Monokit < Formula
  desc "Text-based scripting language for monophonic drum synthesis"
  homepage "https://github.com/yourusername/monokit"
  version "0.1.0"
  license "MIT"

  if Hardware::CPU.arm?
    url "https://github.com/yourusername/monokit/releases/download/v0.1.0/monokit-0.1.0-aarch64-apple-darwin.tar.gz"
    sha256 "HASH_FROM_STEP_3"
  else
    url "https://github.com/yourusername/monokit/releases/download/v0.1.0/monokit-0.1.0-x86_64-apple-darwin.tar.gz"
    sha256 "HASH_FOR_INTEL"
  end

  depends_on :macos
  depends_on "supercollider" => :recommended

  def install
    bin.install "monokit"
    pkgshare.install "sc/monokit_server.scd"
  end

  def caveats
    <<~EOS
      SC server file installed to:
        #{pkgshare}/monokit_server.scd

      Ensure SuperCollider is installed:
        brew install supercollider
    EOS
  end

  test do
    assert_match "monokit", shell_output("#{bin}/monokit --help 2>&1", 1)
  end
end
```

### 7. Push Tap

```bash
git add .
git commit -m "Add monokit formula v0.1.0"
git remote add origin git@github.com:yourusername/homebrew-monokit.git
git push -u origin main
```

### 8. Test Installation

```bash
brew tap yourusername/monokit
brew install monokit
monokit --help
```

### 9. Update SC Path (if needed)

You may need to update monokit to find the scd file at:
- `$(brew --prefix)/share/monokit/monokit_server.scd`

Or instruct users to symlink/copy to `~/.monokit/`.

---

### Future Releases

1. Make changes, commit
2. `git tag -a v0.1.1 -m "Description"`
3. `git push origin v0.1.1`
4. Build binary, create tarball, get sha256
5. Upload to GitHub release
6. Update formula in tap repo (url version + sha256)
7. Push tap, users run `brew upgrade monokit`

---

## Terminal Compatibility

### Known Issues with macOS Terminal.app

Terminal.app does NOT support true color (24-bit RGB). Limited to 256-color palette.

| Feature | Terminal.app | iTerm2 | Monokit Current |
|---------|--------------|--------|-----------------|
| True Color (24-bit) | ✗ No | ✓ Yes | Hard-coded RGB |
| 256-color | ✓ Yes | ✓ Yes | No fallback |
| Block chars (▁▂▃▄▅▆▇█) | ✓ Gaps | ✓ Gaps | Direct use |
| Braille (⠀-⠿) | ✓ Yes | ✓ Yes | Direct use |
| Cursor visibility | ⚠ Issues | ✓ Yes | Via crossterm |

**Specific Problems:**

1. **Colors:** RGB colors display unpredictably - may blink, show wrong colors, or fail entirely
2. **Block character gaps:** Font-dependent gaps in meters/spectrum (Monaco works best, Menlo has gaps)
3. **Cursor:** May not be visible or restore properly
4. **Themes:** Only light/dark approximate correctly; custom themes break

### Detection Strategy

Check at startup:
```rust
let colorterm = env::var("COLORTERM").ok();
let true_color = colorterm.as_deref() == Some("truecolor")
              || colorterm.as_deref() == Some("24bit");
// Terminal.app: COLORTERM not set, TERM=xterm-256color
// iTerm2: May set COLORTERM=truecolor
```

### Implementation Phases

**Phase 1 - Pre-release (Critical):** COMPLETE
- [x] Add terminal capability detection at startup
- [x] Display warning if Terminal.app detected (no truecolor)
- [x] Implement 256-color theme fallback (map RGB to nearest)
- [x] Document limitations in help system

**Additional Phase 1 features implemented:**
- COMPAT command: Display terminal capabilities
- METER.ASCII command: Toggle ASCII-only meter display
- COMPAT.MODE command: Force compatibility mode on/off
- High-contrast cursor fallback for 256-color mode
- Theme change 256-color conversion fix

**Phase 2 - v0.2.0:**
- [ ] Add ANSI 16-color fallback themes
- [ ] Character set fallback for scope (ASCII mode)
- [ ] Config option: `compatibility_mode = "auto"|"full"|"basic"`
- [ ] Terminal.app user guide

**Phase 3 - v0.3.0:**
- [ ] Font recommendations at startup
- [ ] Auto theme selection based on terminal
- [ ] VT100 alternate charset support

### Color Fallback Architecture

```rust
pub enum ColorMode {
    TrueColor,  // 24-bit RGB (iTerm2, modern terminals)
    Color256,   // 256-color palette (Terminal.app)
    Color16,    // ANSI 16 colors (universal fallback)
}

// Theme system needs variants for each mode
// RGB -> 256-color mapping function needed
```

### Character Fallback Options

For meters/scope when gaps detected:
```
SCOPE.MODE options:
  0 = BRAILLE (best quality, needs good font)
  1 = BLOCKS (▁▂▃▄▅▆▇█ - may have gaps)
  2 = LINES (╱╲│─)
  3 = DOTS (·•●)
  4 = QUADRANTS (▘▝▀▖▌▞▛▗▚▐▜▄▙▟█)

ASCII fallbacks for Terminal.app:
  BLOCKS_ASCII: ||||||||| (pure ASCII)
  LINES_ASCII: /\\|-
```

### Recommended Fonts for Terminal.app

Best to worst for block character rendering:
1. Monaco (best compatibility)
2. Courier New (good)
3. Menlo (visible gaps)
4. Fira Code (visible gaps)

### Config Options (Future)

```toml
[compatibility]
terminal_mode = "auto"  # "iterm2", "terminal", "minimal"
color_mode = "auto"     # "truecolor", "256", "16"
ascii_fallback = false  # Use ASCII-only characters
```

### Help Text Updates

Add to help system (46 char limit):
```
COMPAT INFO: Terminal.app has limits.
Use iTerm2 for best experience.
SCOPE.MODE 1 may help with gaps.
```
