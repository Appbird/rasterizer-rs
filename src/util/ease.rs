pub fn out_cubic(t:f64) -> f64 {
    let x = 1. - t;
    return 1. - x*x*x;
}
pub fn in_cubic(t:f64) -> f64 {
    return t*t*t;
}
pub fn out_back(t:f64) -> f64 {
    let c1 = 1.70158;
    let c3 = c1 + 1.;
    let s = t - 1.;
    return 1. + c3 * s*s*s + c1 * s*s; 
}

pub fn in_quart(t:f64) -> f64 {
    return t*t*t*t;
}
pub fn out_quart(t:f64) -> f64 {
    let s = 1. - t;
    return 1. - s*s*s*s;
}
pub fn outin_quart(t:f64) -> f64 {
    let x = t - 0.5;
    return f64::signum(x)*f64::powi(x, 4) * 8. + 0.5;
}
