pub use lpf_impl::LowPassFilter;

/// create a lpf instance with a sample rate and cutoff frequency
/// The client is responsible for freeing the instance's memory when it's no longer required,
/// see `destroy()`.
/// 

#[no_mangle]
pub extern "C" fn create(fs: f32, f0: f32) -> *mut LowPassFilter {
    Box::into_raw(Box::new(LowPassFilter::new(fs, f0)))
}

/// Destroy a lpf instance
///
/// # Safety
///
/// The instance must have been previously created using `create()`.
#[no_mangle]
pub unsafe extern "C" fn destroy(lpf: *mut LowPassFilter) {
    if !lpf.is_null() {
        unsafe {
            let _ = Box::from_raw(lpf);
        }
    } else {
        panic!("")
    }
}

/// filter the input with a given buffer
#[no_mangle]
pub unsafe extern "C" fn process(
    lpf: &mut LowPassFilter,
    input_l: *const f32,
    input_r: *const f32,
    output_l: *mut f32,
    output_r: *mut f32,
    sample_count: usize,
) {
    for i in 0..sample_count as isize {
        let out = lpf.process((*input_l.offset(i) as f32, *input_r.offset(i) as f32));
        *output_l.offset(i) = out.0;
        *output_r.offset(i) = out.1;
    }
}

#[no_mangle]
pub extern "C" fn set_f0(lpf: &mut LowPassFilter, value: f32) {
    lpf.set_f0(value);
}

#[no_mangle]
pub extern "C" fn set_fs(lpf: &mut LowPassFilter, value: f32) {
    lpf.set_fs(value);
}