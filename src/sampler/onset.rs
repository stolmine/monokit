pub struct OnsetDetector {
    sample_rate: u32,
    hop_size: usize,
    window_size: usize,
    min_spacing_ms: f32,
    sensitivity: f32,
    median_window: usize,
}

impl OnsetDetector {
    pub fn new(sample_rate: u32) -> Self {
        Self {
            sample_rate,
            hop_size: 512,
            window_size: 1024,
            min_spacing_ms: 50.0,
            sensitivity: 2.0,
            median_window: 7,
        }
    }

    pub fn with_sensitivity(mut self, sens: u32) -> Self {
        // Map 0-10 to threshold multiplier:
        // 0 = 0.5 (very sensitive, catches weak transients)
        // 5 = 2.0 (balanced)
        // 10 = 8.0 (conservative, only strong transients)
        // Using exponential curve for more intuitive control
        let sens = sens.min(10) as f32;
        self.sensitivity = 0.5 * (2.0_f32).powf(sens * 0.4);
        self
    }

    pub fn with_min_spacing(mut self, ms: f32) -> Self {
        self.min_spacing_ms = ms.clamp(10.0, 500.0);
        self
    }

    pub fn detect(&self, audio: &[f32]) -> Vec<usize> {
        if audio.len() < self.window_size {
            return Vec::new();
        }

        let energy = self.compute_energy(audio);
        if energy.len() < 2 {
            return Vec::new();
        }

        let novelty = self.compute_novelty(&energy);
        let threshold = self.compute_threshold(&novelty);
        let peaks = self.pick_peaks(&novelty, &threshold);

        peaks
            .into_iter()
            .map(|hop_idx| hop_idx * self.hop_size)
            .collect()
    }

    fn compute_energy(&self, audio: &[f32]) -> Vec<f32> {
        let num_hops = (audio.len().saturating_sub(self.window_size)) / self.hop_size + 1;
        let mut energy = Vec::with_capacity(num_hops);
        let inv_window = 1.0 / self.window_size as f32;

        for i in 0..num_hops {
            let start = i * self.hop_size;
            let end = start + self.window_size;
            if end > audio.len() {
                break;
            }

            let sum_sq: f32 = audio[start..end].iter().map(|x| x * x).sum();
            energy.push((sum_sq * inv_window).sqrt());
        }

        energy
    }

    fn compute_novelty(&self, energy: &[f32]) -> Vec<f32> {
        let len = energy.len();
        let mut novelty = Vec::with_capacity(len);
        novelty.push(0.0);

        for i in 1..len {
            novelty.push((energy[i] - energy[i - 1]).max(0.0));
        }

        novelty
    }

    fn compute_threshold(&self, novelty: &[f32]) -> Vec<f32> {
        let mut threshold = Vec::with_capacity(novelty.len());
        let half_window = self.median_window / 2;
        // Reusable buffer for median calculation (max 7 elements)
        let mut buf = [0.0f32; 8];

        for i in 0..novelty.len() {
            let start = i.saturating_sub(half_window);
            let end = (i + half_window + 1).min(novelty.len());
            let window = &novelty[start..end];

            let median = Self::median_inplace(window, &mut buf);
            threshold.push(median * self.sensitivity);
        }

        threshold
    }

    fn median_inplace(values: &[f32], buf: &mut [f32; 8]) -> f32 {
        let len = values.len();
        if len == 0 {
            return 0.0;
        }

        // Copy to buffer and sort in place (no heap allocation)
        buf[..len].copy_from_slice(values);
        buf[..len].sort_unstable_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));

        let mid = len / 2;
        if len & 1 == 0 {
            (buf[mid - 1] + buf[mid]) * 0.5
        } else {
            buf[mid]
        }
    }

    fn pick_peaks(&self, novelty: &[f32], threshold: &[f32]) -> Vec<usize> {
        let min_spacing_samples = (self.min_spacing_ms / 1000.0 * self.sample_rate as f32) as usize;
        let min_spacing_hops = (min_spacing_samples / self.hop_size).max(1);

        let mut peaks = Vec::new();
        let mut last_peak_hop = 0usize;

        for i in 1..novelty.len().saturating_sub(1) {
            if novelty[i] > threshold[i]
                && novelty[i] > novelty[i - 1]
                && novelty[i] > novelty[i + 1]
            {
                if peaks.is_empty() || i >= last_peak_hop + min_spacing_hops {
                    peaks.push(i);
                    last_peak_hop = i;
                }
            }
        }

        peaks
    }
}

pub fn to_mono(samples: &[f32], channels: u16) -> Vec<f32> {
    let channels = channels as usize;
    if channels == 1 {
        return samples.to_vec();
    }

    let num_frames = samples.len() / channels;
    let mut mono = Vec::with_capacity(num_frames);
    let inv_channels = 1.0 / channels as f32;

    for frame_idx in 0..num_frames {
        let start = frame_idx * channels;
        let sum: f32 = samples[start..start + channels].iter().sum();
        mono.push(sum * inv_channels);
    }

    mono
}
