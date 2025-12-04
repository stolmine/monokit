use rand::Rng;
use std::time::Instant;

pub struct ScrambleAnimation {
    pub target_text: String,
    pub current_display: String,
    start_time: Instant,
    ms_per_char: u64,
    // Each position has its own scramble state that updates independently
    char_states: Vec<CharState>,
    pub complete: bool,
}

struct CharState {
    current_char: char,
    last_change: Instant,
    change_interval_ms: u64, // Each char has its own scramble speed
}

impl ScrambleAnimation {
    pub fn new(text: &str) -> Self {
        let mut rng = rand::thread_rng();
        let char_states: Vec<CharState> = (0..text.len())
            .map(|_| CharState {
                current_char: random_char(),
                last_change: Instant::now(),
                // Each character has slightly different scramble timing (40-80ms)
                change_interval_ms: rng.gen_range(40..80),
            })
            .collect();

        let initial_display: String = char_states.iter().map(|s| s.current_char).collect();

        Self {
            target_text: text.to_string(),
            current_display: initial_display,
            start_time: Instant::now(),
            ms_per_char: 66,
            char_states,
            complete: false,
        }
    }

    pub fn update(&mut self) -> &str {
        if self.complete {
            return &self.current_display;
        }

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

        // Calculate how many characters are revealed
        let revealed_count = if char_pos >= scramble_buffer {
            (char_pos - scramble_buffer + 1).min(target_len)
        } else {
            0
        };

        // Build the display string
        let mut display = String::with_capacity(target_len);
        let now = Instant::now();

        for (i, state) in self.char_states.iter_mut().enumerate() {
            if i < revealed_count {
                // This position is revealed - show final character
                display.push(target_chars[i]);
            } else if i < char_pos + 1 && i < target_len {
                // This position is in the scramble zone
                // Update its scramble state based on its own timing
                if now.duration_since(state.last_change).as_millis() >= state.change_interval_ms as u128 {
                    state.current_char = random_char();
                    state.last_change = now;
                }
                display.push(state.current_char);
            }
            // Positions beyond char_pos are not shown yet
        }

        self.current_display = display;
        &self.current_display
    }
}

fn random_char() -> char {
    const SCRAMBLE_CHARS: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789?/\\(^)![]{}&^%$#";
    let mut rng = rand::thread_rng();
    let idx = rng.gen_range(0..SCRAMBLE_CHARS.len());
    SCRAMBLE_CHARS.chars().nth(idx).unwrap()
}
