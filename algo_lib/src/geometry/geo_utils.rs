static mut EPSILON: f64 = 1e-9;

pub fn epsilon() -> f64 {
    unsafe { EPSILON }
}

pub fn set_epsilon(e: f64) {
    unsafe {
        EPSILON = e;
    }
}
