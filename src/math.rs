pub fn fabs(x: f64) -> f64 {
    unsafe { core::intrinsics::fabsf64(x) }
}

pub fn cos(x: f64) -> f64 {
    unsafe { core::intrinsics::cosf64(x) }
}

pub fn powi(x: f64, y: i32) -> f64 {
    unsafe { core::intrinsics::powif64(x, y) }
}

pub fn round(x: f64) -> f64 {
    unsafe { core::intrinsics::roundf64(x) }
}

pub fn sin(x: f64) -> f64 {
    unsafe { core::intrinsics::sinf64(x) }
}

pub fn sqrt(x: f64) -> f64 {
    unsafe { core::intrinsics::sqrtf64(x) }
}
