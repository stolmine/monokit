use crate::types::EqState;

/// Calculate EQ frequency response curve for visualization
/// Returns samples in range -1.0 to 1.0 (normalized from ±24dB)
pub fn calculate_eq_response(eq: &EqState, num_points: usize) -> Vec<f32> {
    if num_points == 0 {
        return Vec::new();
    }

    let mut response = Vec::with_capacity(num_points);

    for i in 0..num_points {
        let freq = frequency_at_point(i, num_points);
        let db = calculate_total_response_db(eq, freq);
        let normalized = (db / 24.0).clamp(-1.0, 1.0);
        response.push(normalized);
    }

    response
}

fn frequency_at_point(index: usize, num_points: usize) -> f32 {
    let min_freq = 20.0_f32;
    let max_freq = 20000.0_f32;
    let log_min = min_freq.ln();
    let log_max = max_freq.ln();
    let t = index as f32 / (num_points - 1).max(1) as f32;
    (log_min + t * (log_max - log_min)).exp()
}

fn calculate_total_response_db(eq: &EqState, freq: f32) -> f32 {
    let low_shelf = low_shelf_response(freq, eq.low_freq, eq.low_db);
    let high_shelf = high_shelf_response(freq, eq.high_freq, eq.high_db);
    let parametric = parametric_response(freq, eq.mid_freq, eq.mid_db, eq.mid_q);

    low_shelf + high_shelf + parametric
}

fn low_shelf_response(freq: f32, center_freq: f32, gain_db: f32) -> f32 {
    gain_db * (1.0 / (1.0 + (freq / center_freq).powi(2)))
}

fn high_shelf_response(freq: f32, center_freq: f32, gain_db: f32) -> f32 {
    gain_db * (1.0 / (1.0 + (center_freq / freq).powi(2)))
}

fn parametric_response(freq: f32, center_freq: f32, gain_db: f32, q: f32) -> f32 {
    let bandwidth = 1.0 / q;
    let log_distance = freq.ln() - center_freq.ln();
    gain_db * (-log_distance.powi(2) / (2.0 * bandwidth.powi(2))).exp()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_points() {
        let eq = EqState::default();
        let result = calculate_eq_response(&eq, 0);
        assert_eq!(result.len(), 0);
    }

    #[test]
    fn test_flat_response() {
        let eq = EqState::default();
        let result = calculate_eq_response(&eq, 100);
        assert_eq!(result.len(), 100);
        for sample in result {
            assert!(sample.abs() < 0.1);
        }
    }

    #[test]
    fn test_frequency_range() {
        let freq_0 = frequency_at_point(0, 100);
        let freq_99 = frequency_at_point(99, 100);
        assert!((freq_0 - 20.0).abs() < 1.0);
        assert!((freq_99 - 20000.0).abs() < 100.0);
    }

    #[test]
    fn test_normalization() {
        let mut eq = EqState::default();
        eq.low_db = 24.0;
        eq.mid_db = 24.0;
        eq.high_db = 24.0;
        let result = calculate_eq_response(&eq, 100);
        for sample in result {
            assert!(sample >= -1.0 && sample <= 1.0);
        }
    }

    #[test]
    fn test_low_shelf_boost() {
        let mut eq = EqState::default();
        eq.low_db = 12.0;
        eq.low_freq = 200.0;
        let low_freq_response = calculate_total_response_db(&eq, 50.0);
        let high_freq_response = calculate_total_response_db(&eq, 5000.0);
        assert!(low_freq_response > high_freq_response);
    }

    #[test]
    fn test_high_shelf_boost() {
        let mut eq = EqState::default();
        eq.high_db = 12.0;
        eq.high_freq = 2500.0;
        let low_freq_response = calculate_total_response_db(&eq, 100.0);
        let high_freq_response = calculate_total_response_db(&eq, 10000.0);
        assert!(high_freq_response > low_freq_response);
    }

    #[test]
    fn test_parametric_peak() {
        let mut eq = EqState::default();
        eq.mid_db = 12.0;
        eq.mid_freq = 1000.0;
        eq.mid_q = 2.0;
        let center_response = calculate_total_response_db(&eq, 1000.0);
        let side_response = calculate_total_response_db(&eq, 500.0);
        assert!(center_response > side_response);
    }
}
