//! Braille character rendering for oscilloscope waveform display
//!
//! Each braille character represents a 2x4 dot matrix, allowing high-resolution
//! waveform display in terminal environments.

/// Unicode braille base character (empty pattern)
pub const BRAILLE_BASE: u32 = 0x2800;

/// Dot bit positions for left column (dots 1,2,3,7 in braille numbering)
const LEFT_DOTS: [u8; 4] = [0x01, 0x02, 0x04, 0x40];

/// Dot bit positions for right column (dots 4,5,6,8 in braille numbering)
const RIGHT_DOTS: [u8; 4] = [0x08, 0x10, 0x20, 0x80];

/// Convert audio samples to a braille character grid with connected lines
///
/// # Arguments
/// * `samples` - Audio samples in range -1.0 to 1.0
/// * `width_chars` - Width of output grid in characters
/// * `height_chars` - Height of output grid in characters
///
/// # Returns
/// A 2D vector of characters, row-major order (grid[row][col])
pub fn samples_to_braille(
    samples: &[f32],
    width_chars: usize,
    height_chars: usize,
) -> Vec<Vec<char>> {
    if samples.is_empty() || width_chars == 0 || height_chars == 0 {
        return vec![vec![char::from_u32(BRAILLE_BASE).unwrap(); width_chars]; height_chars];
    }

    let height_dots = height_chars * 4;
    let width_dots = width_chars * 2;

    // Initialize bit grid
    let mut grid = vec![vec![0u8; width_chars]; height_chars];

    let mut prev_row: Option<usize> = None;

    for dot_col in 0..width_dots {
        // Map dot column to sample index
        let sample_idx = (dot_col * samples.len()) / width_dots;
        let sample = samples.get(sample_idx).copied().unwrap_or(0.0).clamp(-1.0, 1.0);

        // Map sample (-1 to 1) to dot row (0 to height_dots-1)
        // sample 1.0 = top (row 0), sample -1.0 = bottom (row height_dots-1)
        let normalized = (1.0 - sample) / 2.0;
        let dot_row = ((normalized * (height_dots - 1) as f32).round() as usize)
            .min(height_dots - 1);

        // Draw connected line from previous row to current row
        let rows_to_draw: Vec<usize> = match prev_row {
            Some(prev) if prev != dot_row => {
                let (start, end) = if prev < dot_row {
                    (prev, dot_row)
                } else {
                    (dot_row, prev)
                };
                (start..=end).collect()
            }
            _ => vec![dot_row],
        };

        for row in rows_to_draw {
            let char_col = dot_col / 2;
            let char_row = row / 4;
            let dot_in_char = row % 4;
            let is_right = dot_col % 2 == 1;

            if char_col < width_chars && char_row < height_chars {
                let bits = if is_right {
                    RIGHT_DOTS[dot_in_char]
                } else {
                    LEFT_DOTS[dot_in_char]
                };
                grid[char_row][char_col] |= bits;
            }
        }

        prev_row = Some(dot_row);
    }

    // Convert bit patterns to braille characters
    grid.into_iter()
        .map(|row| {
            row.into_iter()
                .map(|bits| {
                    char::from_u32(BRAILLE_BASE | bits as u32)
                        .unwrap_or(char::from_u32(BRAILLE_BASE).unwrap())
                })
                .collect()
        })
        .collect()
}

/// Block character rendering (▁▂▃▄▅▆▇█)
const BLOCK_CHARS: [char; 9] = [' ', '▁', '▂', '▃', '▄', '▅', '▆', '▇', '█'];

pub fn samples_to_blocks(
    samples: &[f32],
    width_chars: usize,
    height_chars: usize,
) -> Vec<Vec<char>> {
    if samples.is_empty() || width_chars == 0 || height_chars == 0 {
        return vec![vec![' '; width_chars]; height_chars];
    }

    let height_levels = height_chars * 8; // 8 levels per character
    let mut grid = vec![vec![' '; width_chars]; height_chars];

    for col in 0..width_chars {
        let sample_idx = (col * samples.len()) / width_chars;
        let sample = samples.get(sample_idx).copied().unwrap_or(0.0).clamp(-1.0, 1.0);

        // Map sample to row and sub-level
        let normalized = (1.0 - sample) / 2.0;
        let level = (normalized * (height_levels - 1) as f32).round() as usize;
        let level = level.min(height_levels - 1);

        let char_row = level / 8;
        let sub_level = 8 - (level % 8); // Invert for bottom-up fill

        if char_row < height_chars {
            grid[char_row][col] = BLOCK_CHARS[sub_level.min(8)];
        }
    }
    grid
}

/// Line drawing rendering (╱╲│─)
pub fn samples_to_lines(
    samples: &[f32],
    width_chars: usize,
    height_chars: usize,
) -> Vec<Vec<char>> {
    if samples.is_empty() || width_chars == 0 || height_chars == 0 {
        return vec![vec![' '; width_chars]; height_chars];
    }

    let mut grid = vec![vec![' '; width_chars]; height_chars];
    let mut prev_row: Option<usize> = None;

    for col in 0..width_chars {
        let sample_idx = (col * samples.len()) / width_chars;
        let sample = samples.get(sample_idx).copied().unwrap_or(0.0).clamp(-1.0, 1.0);

        let normalized = (1.0 - sample) / 2.0;
        let row = ((normalized * (height_chars - 1) as f32).round() as usize).min(height_chars - 1);

        let ch = match prev_row {
            Some(prev) if prev < row => '╲',  // Going down
            Some(prev) if prev > row => '╱',  // Going up
            _ => '─',                          // Level
        };

        grid[row][col] = ch;
        prev_row = Some(row);
    }
    grid
}

/// Dot/scatter rendering (·•●)
pub fn samples_to_dots(
    samples: &[f32],
    width_chars: usize,
    height_chars: usize,
) -> Vec<Vec<char>> {
    if samples.is_empty() || width_chars == 0 || height_chars == 0 {
        return vec![vec![' '; width_chars]; height_chars];
    }

    let mut grid = vec![vec![' '; width_chars]; height_chars];

    for col in 0..width_chars {
        let sample_idx = (col * samples.len()) / width_chars;
        let sample = samples.get(sample_idx).copied().unwrap_or(0.0).clamp(-1.0, 1.0);

        let normalized = (1.0 - sample) / 2.0;
        let row = ((normalized * (height_chars - 1) as f32).round() as usize).min(height_chars - 1);

        grid[row][col] = '●';
    }
    grid
}

/// Quadrant block rendering (2×2 grid per character)
/// Characters: ▖▗▘▙▚▛▜▝▞▟ and space/full block
/// Bit positions: top-left=1, top-right=2, bottom-left=4, bottom-right=8
pub fn samples_to_quadrants(
    samples: &[f32],
    width_chars: usize,
    height_chars: usize,
) -> Vec<Vec<char>> {
    if samples.is_empty() || width_chars == 0 || height_chars == 0 {
        return vec![vec![' '; width_chars]; height_chars];
    }

    let height_dots = height_chars * 2;
    let width_dots = width_chars * 2;

    // Bit patterns for quadrants: TL=1, TR=2, BL=4, BR=8
    // But Unicode quadrants start at U+2596 with specific mappings
    let mut grid = vec![vec![0u8; width_chars]; height_chars];

    let mut prev_row: Option<usize> = None;

    for dot_col in 0..width_dots {
        let sample_idx = (dot_col * samples.len()) / width_dots;
        let sample = samples.get(sample_idx).copied().unwrap_or(0.0).clamp(-1.0, 1.0);

        let normalized = (1.0 - sample) / 2.0;
        let dot_row = ((normalized * (height_dots - 1) as f32).round() as usize).min(height_dots - 1);

        // Draw connected line from prev to current
        let rows_to_draw: Vec<usize> = match prev_row {
            Some(prev) if prev != dot_row => {
                let (start, end) = if prev < dot_row { (prev, dot_row) } else { (dot_row, prev) };
                (start..=end).collect()
            }
            _ => vec![dot_row],
        };

        for row in rows_to_draw {
            let char_col = dot_col / 2;
            let char_row = row / 2;
            let is_bottom = row % 2 == 1;
            let is_right = dot_col % 2 == 1;

            if char_col < width_chars && char_row < height_chars {
                // Quadrant bit: TL=0, TR=1, BL=2, BR=3 (for indexing)
                let bit = match (is_bottom, is_right) {
                    (false, false) => 0, // top-left
                    (false, true) => 1,  // top-right
                    (true, false) => 2,  // bottom-left
                    (true, true) => 3,   // bottom-right
                };
                grid[char_row][char_col] |= 1 << bit;
            }
        }
        prev_row = Some(dot_row);
    }

    // Convert bit patterns to quadrant characters
    grid.into_iter()
        .map(|row| {
            row.into_iter()
                .map(|bits| bits_to_quadrant(bits))
                .collect()
        })
        .collect()
}

fn bits_to_quadrant(bits: u8) -> char {
    // Quadrant mapping: bits = TL(1) + TR(2) + BL(4) + BR(8)
    // Unicode quadrants U+2596-259F don't follow simple bit pattern
    // Map manually:
    match bits {
        0 => ' ',
        1 => '▘',   // top-left
        2 => '▝',   // top-right
        3 => '▀',   // top half
        4 => '▖',   // bottom-left
        5 => '▌',   // left half
        6 => '▞',   // diagonal
        7 => '▛',   // all but bottom-right
        8 => '▗',   // bottom-right
        9 => '▚',   // diagonal
        10 => '▐', // right half
        11 => '▜', // all but bottom-left
        12 => '▄', // bottom half
        13 => '▙', // all but top-right
        14 => '▟', // all but top-left
        15 => '█', // full
        _ => ' ',
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_samples() {
        let result = samples_to_braille(&[], 10, 4);
        assert_eq!(result.len(), 4);
        assert_eq!(result[0].len(), 10);
        // All should be empty braille
        assert!(result.iter().all(|row| row.iter().all(|&c| c == '⠀')));
    }

    #[test]
    fn test_zero_samples() {
        // Zero samples should produce dots in the middle row
        let samples = vec![0.0; 20];
        let result = samples_to_braille(&samples, 10, 4);
        assert_eq!(result.len(), 4);
        // Middle rows should have dots
        assert!(result[1].iter().any(|&c| c != '⠀') || result[2].iter().any(|&c| c != '⠀'));
    }

    #[test]
    fn test_max_samples() {
        // All 1.0 should produce dots at top
        let samples = vec![1.0; 20];
        let result = samples_to_braille(&samples, 10, 4);
        // Top row should have dots
        assert!(result[0].iter().any(|&c| c != '⠀'));
    }

    #[test]
    fn test_min_samples() {
        // All -1.0 should produce dots at bottom
        let samples = vec![-1.0; 20];
        let result = samples_to_braille(&samples, 10, 4);
        // Bottom row should have dots
        assert!(result[3].iter().any(|&c| c != '⠀'));
    }
}
