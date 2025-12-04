use rand::Rng;
use std::time::Instant;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScrambleMode {
    Regular = 0,
    Smash = 1,
    Rolling = 2,
    Overshoot = 3,
}

impl ScrambleMode {
    pub fn from_u8(value: u8) -> Self {
        match value {
            0 => ScrambleMode::Regular,
            1 => ScrambleMode::Smash,
            3 => ScrambleMode::Overshoot,
            _ => ScrambleMode::Rolling,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ScrambleCurve {
    #[default]
    Linear = 0,
    Expo = 1,
}

impl ScrambleCurve {
    pub fn from_u8(value: u8) -> Self {
        match value {
            1 => ScrambleCurve::Expo,
            _ => ScrambleCurve::Linear,
        }
    }
}

pub struct ScrambleAnimation {
    pub target_text: String,
    pub current_display: String,
    start_time: Instant,
    total_duration_ms: u64,
    char_states: Vec<CharState>,
    pub complete: bool,
    mode: ScrambleMode,
    curve: ScrambleCurve,
    is_icon_mode: bool, // Single-char icons get special treatment
}

struct CharState {
    current_char: char,
    target_char: char,       // The final character this will become
    last_change: Instant,
    change_interval_ms: u64, // Each char has its own scramble speed
    scramble_start: Option<Instant>, // When this char started scrambling
    locked_early: bool,      // Has this char settled on its target early?
    icon_offset: i32,        // For icon mode: current offset from target
    icon_start_offset: i32,  // For icon mode: initial offset (to calculate progress)
}

impl ScrambleAnimation {
    pub fn new(text: &str) -> Self {
        Self::new_with_options(text, ScrambleMode::Rolling, 5, ScrambleCurve::Linear)
    }

    pub fn new_with_mode(text: &str, mode: ScrambleMode, speed: u8) -> Self {
        Self::new_with_options(text, mode, speed, ScrambleCurve::Linear)
    }

    pub fn new_with_options(text: &str, mode: ScrambleMode, speed: u8, curve: ScrambleCurve) -> Self {
        let speed = speed.clamp(1, 10);
        let base_ms_per_char = 100;

        let mut rng = rand::thread_rng();
        let target_chars: Vec<char> = text.chars().collect();
        let text_len = target_chars.len();

        // Single-char texts are icons - use Unicode traversal instead of random chars
        let is_icon_mode = text_len == 1;

        // Settle curve uses slower base scramble rate for more visible deceleration
        let (base_change_min, base_change_max) = if curve == ScrambleCurve::Expo {
            (600, 1000)
        } else {
            (60, 100)
        };

        let ms_per_char = (base_ms_per_char * (11 - speed as u64)) / 5;
        let change_min = (base_change_min * (11 - speed as u64)) / 5;
        let change_max = (base_change_max * (11 - speed as u64)) / 5;

        let scramble_buffer = if mode == ScrambleMode::Rolling { 5 } else { 0 };

        // For icons, generate offset first to calculate proportional duration
        let icon_offset: i32 = if is_icon_mode {
            if rng.gen_bool(0.5) {
                rng.gen_range(100..=500)
            } else {
                rng.gen_range(-500..=-100)
            }
        } else {
            0
        };

        // Duration: for text, based on char count; for icons, use same timing as ~7 char title
        let total_duration_ms = if is_icon_mode {
            const ICON_REFERENCE_LEN: u64 = 7; // Same as "MONOKIT"
            ms_per_char * (ICON_REFERENCE_LEN + scramble_buffer)
        } else {
            ms_per_char * (text_len as u64 + scramble_buffer)
        };

        let char_states: Vec<CharState> = (0..text_len)
            .map(|i| {
                let initial_char = if is_icon_mode {
                    char_at_offset(target_chars[i], icon_offset)
                } else {
                    random_char()
                };
                CharState {
                    current_char: initial_char,
                    target_char: target_chars[i],
                    last_change: Instant::now(),
                    change_interval_ms: rng.gen_range(change_min..change_max),
                    scramble_start: None,
                    locked_early: false,
                    icon_offset,
                    icon_start_offset: icon_offset,
                }
            })
            .collect();

        let initial_display: String = char_states.iter().map(|s| s.current_char).collect();

        Self {
            target_text: text.to_string(),
            current_display: initial_display,
            start_time: Instant::now(),
            total_duration_ms,
            char_states,
            complete: false,
            mode,
            curve,
            is_icon_mode,
        }
    }

    pub fn update(&mut self) -> &str {
        if self.complete {
            return &self.current_display;
        }

        match self.mode {
            ScrambleMode::Regular => self.update_regular(),
            ScrambleMode::Smash => self.update_smash(),
            ScrambleMode::Rolling => self.update_rolling(),
            ScrambleMode::Overshoot => self.update_overshoot(),
        }
    }

    /// Calculate eased progress (0.0 to 1.0) based on elapsed time and curve
    fn eased_progress(&self) -> f32 {
        let elapsed_ms = self.start_time.elapsed().as_millis() as f32;
        let t = (elapsed_ms / self.total_duration_ms as f32).min(1.0);

        match self.curve {
            ScrambleCurve::Linear => t,
            ScrambleCurve::Expo => ease_out_expo(t),
        }
    }

    fn update_regular(&mut self) -> &str {
        let target_len = self.target_text.chars().count();
        let target_chars: Vec<char> = self.target_text.chars().collect();

        let progress = self.eased_progress();
        let char_pos = (progress * target_len as f32) as usize;

        if progress >= 1.0 {
            self.current_display = self.target_text.clone();
            self.complete = true;
            return &self.current_display;
        }

        let mut display = String::with_capacity(target_len);
        let now = Instant::now();

        for i in 0..char_pos.min(target_len) {
            display.push(target_chars[i]);
        }

        // Scramble unrevealed characters with curve-based slowdown
        let remaining = target_len - char_pos;
        for (idx, i) in (char_pos..target_len).enumerate() {
            if i < self.char_states.len() {
                let state = &mut self.char_states[i];

                if self.is_icon_mode {
                    // Icon mode: time-based animation using same duration/curve as text
                    if state.scramble_start.is_none() {
                        state.scramble_start = Some(now);
                    }
                    let elapsed = now.duration_since(state.scramble_start.unwrap()).as_millis() as f32;
                    let progress = (elapsed / self.total_duration_ms as f32).min(1.0);
                    let eased = if self.curve == ScrambleCurve::Expo {
                        1.0 - (1.0 - progress).powi(3) // Cubic ease-out for settle
                    } else {
                        progress // Linear
                    };
                    let new_offset = (state.icon_start_offset as f32 * (1.0 - eased)) as i32;
                    if new_offset != state.icon_offset {
                        state.icon_offset = new_offset;
                        state.current_char = char_at_offset(state.target_char, state.icon_offset);
                    }
                } else {
                    let effective_interval = if self.curve == ScrambleCurve::Expo {
                        let proximity = 1.0 - (idx as f32 / remaining.max(1) as f32).min(1.0);
                        let slowdown = 1.0 + proximity.powi(4) * 49.0;
                        (state.change_interval_ms as f32 * slowdown) as u128
                    } else {
                        state.change_interval_ms as u128
                    };

                    if now.duration_since(state.last_change).as_millis() >= effective_interval {
                        state.current_char = random_char();
                        state.last_change = now;
                    }
                }
                display.push(state.current_char);
            } else {
                display.push(random_char());
            }
        }

        self.current_display = display;
        &self.current_display
    }

    fn update_smash(&mut self) -> &str {
        let target_len = self.target_text.chars().count();

        let progress = self.eased_progress();
        let char_pos = (progress * target_len as f32) as usize;

        if progress >= 1.0 {
            self.current_display = self.target_text.clone();
            self.complete = true;
            return &self.current_display;
        }

        let mut display = String::with_capacity(char_pos + 1);

        for _ in 0..=char_pos.min(target_len - 1) {
            display.push(random_char());
        }

        self.current_display = display;
        &self.current_display
    }

    fn update_rolling(&mut self) -> &str {
        let scramble_buffer = 5;
        let target_len = self.target_text.chars().count();
        let target_chars: Vec<char> = self.target_text.chars().collect();

        let progress = self.eased_progress();
        let total_steps = target_len + scramble_buffer;
        let char_pos = (progress * total_steps as f32) as usize;

        if progress >= 1.0 {
            self.current_display = self.target_text.clone();
            self.complete = true;
            return &self.current_display;
        }

        let revealed_count = if char_pos >= scramble_buffer {
            (char_pos - scramble_buffer + 1).min(target_len)
        } else {
            0
        };

        // How long does each char have to scramble before reveal?
        let ms_per_step = self.total_duration_ms as f32 / total_steps as f32;
        let scramble_duration_ms = ms_per_step * scramble_buffer as f32;

        let mut display = String::with_capacity(target_len);
        let now = Instant::now();

        for (i, state) in self.char_states.iter_mut().enumerate() {
            if i < revealed_count {
                // Character is revealed
                state.scramble_start = None;
                display.push(target_chars[i]);
            } else if i < char_pos + 1 && i < target_len {
                // Character is scrambling - track when it started
                if state.scramble_start.is_none() {
                    state.scramble_start = Some(now);
                }

                if self.is_icon_mode {
                    // Icon mode: time-based animation using same duration/curve as text
                    let elapsed = now.duration_since(state.scramble_start.unwrap()).as_millis() as f32;
                    let progress = (elapsed / self.total_duration_ms as f32).min(1.0);
                    let eased = if self.curve == ScrambleCurve::Expo {
                        1.0 - (1.0 - progress).powi(3) // Cubic ease-out for settle
                    } else {
                        progress // Linear
                    };
                    let new_offset = (state.icon_start_offset as f32 * (1.0 - eased)) as i32;
                    if new_offset != state.icon_offset {
                        state.icon_offset = new_offset;
                        state.current_char = char_at_offset(state.target_char, state.icon_offset);
                    }
                } else if self.curve == ScrambleCurve::Expo {
                    // How far through this char's scramble period are we? (0.0 = just started, 1.0 = about to lock)
                    let time_scrambling = now.duration_since(state.scramble_start.unwrap()).as_millis() as f32;
                    let proximity = (time_scrambling / scramble_duration_ms).min(1.0);

                    // At 70% through, lock onto target and stay there
                    if proximity >= 0.7 {
                        state.current_char = state.target_char;
                        state.locked_early = true;
                    } else if !state.locked_early {
                        // Still scrambling - use base interval
                        if now.duration_since(state.last_change).as_millis() >= state.change_interval_ms as u128 {
                            state.current_char = random_char();
                            state.last_change = now;
                        }
                    }
                } else {
                    // Linear mode - just scramble at constant rate
                    if now.duration_since(state.last_change).as_millis() >= state.change_interval_ms as u128 {
                        state.current_char = random_char();
                        state.last_change = now;
                    }
                };

                display.push(state.current_char);
            }
        }

        self.current_display = display;
        &self.current_display
    }

    fn update_overshoot(&mut self) -> &str {
        let elapsed = self.start_time.elapsed();
        let elapsed_ms = elapsed.as_millis() as f32;
        let target_len = self.target_text.chars().count();
        let target_chars: Vec<char> = self.target_text.chars().collect();

        let total_duration = self.total_duration_ms as f32 + 200.0;

        if elapsed_ms >= total_duration {
            self.current_display = self.target_text.clone();
            self.complete = true;
            return &self.current_display;
        }

        let t = (elapsed_ms / total_duration).min(1.0);
        let eased = ease_out_back(t);
        let char_pos = (eased * target_len as f32) as usize;

        let revealed_count = char_pos.min(target_len);

        let mut display = String::with_capacity(target_len);
        let now = Instant::now();

        let remaining = target_len - revealed_count;
        for (i, state) in self.char_states.iter_mut().enumerate() {
            if i < revealed_count {
                display.push(target_chars[i]);
            } else if i < target_len {
                if self.is_icon_mode {
                    // Icon mode: time-based animation using same duration/curve as text
                    if state.scramble_start.is_none() {
                        state.scramble_start = Some(now);
                    }
                    let elapsed = now.duration_since(state.scramble_start.unwrap()).as_millis() as f32;
                    let progress = (elapsed / self.total_duration_ms as f32).min(1.0);
                    let eased = if self.curve == ScrambleCurve::Expo {
                        1.0 - (1.0 - progress).powi(3) // Cubic ease-out for settle
                    } else {
                        progress // Linear
                    };
                    let new_offset = (state.icon_start_offset as f32 * (1.0 - eased)) as i32;
                    if new_offset != state.icon_offset {
                        state.icon_offset = new_offset;
                        state.current_char = char_at_offset(state.target_char, state.icon_offset);
                    }
                } else {
                    let effective_interval = if self.curve == ScrambleCurve::Expo {
                        let distance_from_reveal = i - revealed_count;
                        let proximity = 1.0 - (distance_from_reveal as f32 / remaining.max(1) as f32).min(1.0);
                        let slowdown = 1.0 + proximity.powi(4) * 49.0;
                        (state.change_interval_ms as f32 * slowdown) as u128
                    } else {
                        state.change_interval_ms as u128
                    };

                    if now.duration_since(state.last_change).as_millis() >= effective_interval {
                        state.current_char = random_char();
                        state.last_change = now;
                    }
                }
                display.push(state.current_char);
            }
        }

        self.current_display = display;
        &self.current_display
    }
}

fn ease_out_back(t: f32) -> f32 {
    const C1: f32 = 1.70158;
    const C3: f32 = C1 + 1.0;
    1.0 + C3 * (t - 1.0).powi(3) + C1 * (t - 1.0).powi(2)
}

fn ease_out_expo(t: f32) -> f32 {
    if t >= 1.0 {
        1.0
    } else {
        1.0 - 2.0_f32.powf(-10.0 * t)
    }
}

fn random_char() -> char {
    const SCRAMBLE_CHARS: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789?/\\(^)![]{}&^%$#";
    let mut rng = rand::thread_rng();
    let idx = rng.gen_range(0..SCRAMBLE_CHARS.len());
    SCRAMBLE_CHARS.chars().nth(idx).unwrap()
}

/// Get a character at a given offset from the target character in Unicode
fn char_at_offset(target: char, offset: i32) -> char {
    let target_code = target as u32;
    let new_code = (target_code as i64 + offset as i64) as u32;
    char::from_u32(new_code).unwrap_or(target)
}
