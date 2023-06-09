use std::ops::{Add, Mul};
pub fn clamp(val: f32, min: f32, max: f32) -> f32 {
    match val {
        x if x < min => min,
        x if x > max => max,
        _ => val,
    }
}

#[cfg(test)]
mod clamp_tests {
    use super::*;

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

pub fn lerp<T>(t: f32, start: &T, end: &T) -> T
where
    for<'a> f32: Mul<&'a T, Output = T>,
    for<'a> &'a T: Add<&'a T, Output = T>,
{
    &((1.0 - t) * start) + &(t * end)
}

#[cfg(test)]
mod lerp_tests {
    use crate::math::vec3::Vec3;

    use super::*;

    #[test]
    fn lerp_with_float() {
        assert_eq!(lerp(0.5, &2.0, &4.0), 3.0)
    }

    #[test]
    fn lerp_with_vec3() {
        let u = Vec3::new(1.0, 2.0, 3.0);
        let v = Vec3::new(0.0, 4.0, 8.0);
        assert_eq!(lerp(0.5, &u, &v), Vec3::new(0.5, 3.0, 5.5))
    }
}
