#![allow(dead_code)]

#[inline]
pub fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * std::f64::consts::PI / 180.0
}

#[inline]
pub fn clamp<T: std::cmp::PartialOrd>(value: T, min: T, max: T) -> T {
    assert!(min <= max);
    let mut x = value;
    if x < min {
        x = min;
    }
    if x > max {
        x = max;
    }
    x
}

#[inline]
pub fn schlick(cos: f64, ref_idx: f64) -> f64 {
    let mut r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
    r0 *= r0;
    r0 + (1.0 - r0) * (1.0 - cos).powf(5.0)
}
