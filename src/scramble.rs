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

pub struct ScrambleAnimation {
    pub target_text: String,
    pub current_display: String,
    start_time: Instant,
    ms_per_char: u64,
    char_states: Vec<CharState>,
    pub complete: bool,
    mode: ScrambleMode,
}

struct CharState {
    current_char: char,
    last_change: Instant,
    change_interval_ms: u64, // Each char has its own scramble speed
}

impl ScrambleAnimation {
    pub fn new(text: &str) -> Self {
        Self::new_with_mode(text, ScrambleMode::Rolling, 5)
    }

    pub fn new_with_mode(text: &str, mode: ScrambleMode, speed: u8) -> Self {
        let speed = speed.clamp(1, 10);
        let base_ms_per_char = 100;
        let base_change_min = 60;
        let base_change_max = 100;

        let ms_per_char = (base_ms_per_char * (11 - speed as u64)) / 5;
        let change_min = (base_change_min * (11 - speed as u64)) / 5;
        let change_max = (base_change_max * (11 - speed as u64)) / 5;

        let mut rng = rand::thread_rng();
        let char_states: Vec<CharState> = (0..text.len())
            .map(|_| CharState {
                current_char: random_char(),
                last_change: Instant::now(),
                change_interval_ms: rng.gen_range(change_min..change_max),
            })
            .collect();

        let initial_display: String = char_states.iter().map(|s| s.current_char).collect();

        Self {
            target_text: text.to_string(),
            current_display: initial_display,
            start_time: Instant::now(),
            ms_per_char,
            char_states,
            complete: false,
            mode,
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

    fn update_regular(&mut self) -> &str {
        let elapsed = self.start_time.elapsed();
        let char_pos = (elapsed.as_millis() / self.ms_per_char as u128) as usize;

        let target_len = self.target_text.len();
        let target_chars: Vec<char> = self.target_text.chars().collect();

        if char_pos >= target_len {
            self.current_display = self.target_text.clone();
            self.complete = true;
            return &self.current_display;
        }

        let mut display = String::with_capacity(target_len);

        for i in 0..char_pos.min(target_len) {
            display.push(target_chars[i]);
        }

        for _ in char_pos..target_len {
            display.push(random_char());
        }

        self.current_display = display;
        &self.current_display
    }

    fn update_smash(&mut self) -> &str {
        let elapsed = self.start_time.elapsed();
        let char_pos = (elapsed.as_millis() / self.ms_per_char as u128) as usize;

        let target_len = self.target_text.len();

        if char_pos >= target_len {
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
        let elapsed = self.start_time.elapsed();
        let char_pos = (elapsed.as_millis() / self.ms_per_char as u128) as usize;

        let scramble_buffer = 5;
        let target_len = self.target_text.len();
        let target_chars: Vec<char> = self.target_text.chars().collect();

        if char_pos >= target_len + scramble_buffer {
            self.current_display = self.target_text.clone();
            self.complete = true;
            return &self.current_display;
        }

        let revealed_count = if char_pos >= scramble_buffer {
            (char_pos - scramble_buffer + 1).min(target_len)
        } else {
            0
        };

        let mut display = String::with_capacity(target_len);
        let now = Instant::now();

        for (i, state) in self.char_states.iter_mut().enumerate() {
            if i < revealed_count {
                display.push(target_chars[i]);
            } else if i < char_pos + 1 && i < target_len {
                if now.duration_since(state.last_change).as_millis() >= state.change_interval_ms as u128 {
                    state.current_char = random_char();
                    state.last_change = now;
                }
                display.push(state.current_char);
            }
        }

        self.current_display = display;
        &self.current_display
    }

    fn update_overshoot(&mut self) -> &str {
        let elapsed = self.start_time.elapsed();
        let elapsed_ms = elapsed.as_millis() as f32;
        let target_len = self.target_text.len();
        let target_chars: Vec<char> = self.target_text.chars().collect();

        let total_duration = (target_len as f32 * self.ms_per_char as f32) + 200.0;

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

        for (i, state) in self.char_states.iter_mut().enumerate() {
            if i < revealed_count {
                display.push(target_chars[i]);
            } else if i < target_len {
                if now.duration_since(state.last_change).as_millis() >= state.change_interval_ms as u128 {
                    state.current_char = random_char();
                    state.last_change = now;
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

fn random_char() -> char {
    const SCRAMBLE_CHARS: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789?/\\(^)![]{}&^%$#";
    let mut rng = rand::thread_rng();
    let idx = rng.gen_range(0..SCRAMBLE_CHARS.len());
    SCRAMBLE_CHARS.chars().nth(idx).unwrap()
}
