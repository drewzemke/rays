pub fn clamp(val: f32, min: f32, max: f32) -> f32 {
    match val {
        x if x < min => min,
        x if x > max => max,
        _ => val,
    }
}

#[cfg(test)]
mod clamp_tests {
    use super::clamp;

    #[test]
    fn clamp_val_in_range() {
        assert_eq!(clamp(0.2, 0.0, 1.0), 0.2)
    }

    #[test]
    fn clamp_val_below_min() {
        assert_eq!(clamp(-0.2, 0.0, 1.0), 0.0)
    }

    #[test]
    fn clamp_val_above_max() {
        assert_eq!(clamp(1.2, 0.0, 1.0), 1.0)
    }
}
