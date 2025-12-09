/// 32 prime rhythms from Noise Engineering Numeric Repetitor
const NR_PRIMES: [u16; 32] = [
    0x8888, 0x888A, 0x8892, 0x8894, 0x88A2, 0x88A4,
    0x8912, 0x8914, 0x8922, 0x8924, 0x8A8A, 0x8AAA,
    0x9292, 0x92AA, 0x94AA, 0x952A, 0x8282, 0x828A,
    0x8292, 0x82A2, 0x8484, 0x848A, 0x8492, 0x8494,
    0x84A2, 0x84A4, 0x850A, 0x8512, 0x8514, 0x8522,
    0x8524, 0x8544
];

/// Euclidean rhythm - returns 1 if step should trigger
pub fn euclidean(fill: i16, length: i16, step: i16) -> i16 {
    if length < 1 || length > 32 || fill < 1 || fill > length {
        return 0;
    }
    let length = length as i32;
    let fill = fill as i32;
    // Handle negative steps with proper modulo
    let step = ((step as i32 % length) + length) % length;
    // Bjorklund's algorithm: distribute fill events across length steps
    // A step triggers if bucket accumulation crosses threshold
    let bucket = (step + 1) * fill;
    let prev_bucket = step * fill;
    if bucket / length > prev_bucket / length { 1 } else { 0 }
}

/// Numeric Repeater - returns 1 if step triggers in pattern
pub fn numeric_repeater(prime: i16, mask: i16, factor: i16, step: i16) -> i16 {
    let prime_idx = ((prime as i32 % 32) + 32) as usize % 32;
    let pattern = NR_PRIMES[prime_idx];

    let mask_val = ((mask as i32 % 4) + 4) as u16 % 4;
    let masked = match mask_val {
        0 => pattern,
        1 => pattern & 0x0F0F,
        2 => pattern & 0xF003,
        3 => pattern & 0x01F0,
        _ => pattern,
    };

    // Apply factor by multiplying and wrapping
    let factor_u = (factor.clamp(0, 16) as u32) + 1;
    let varied = ((masked as u32 * factor_u) & 0xFFFF) as u16;

    let step_idx = ((step as i32 % 16) + 16) as usize % 16;
    if (varied >> (15 - step_idx)) & 1 == 1 { 1 } else { 0 }
}
