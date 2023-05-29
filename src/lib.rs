use std::ffi::c_void;

mod measure;
mod rainmeter;
mod utils;

use measure::Measure;
use rainmeter::Api;
use utils::Wchar;

#[no_mangle]
pub extern "C" fn Initialize(data: &mut *mut c_void, rm: *mut c_void) {
  let measure = Measure::new(Api::new(rm));
  let handle = Box::into_raw(Box::new(measure));

  unsafe {
    *data = std::mem::transmute(handle);
  }
}

#[no_mangle]
pub extern "C" fn Finalize(data: *mut c_void) {
  let handle: Box<Measure> = unsafe { Box::from_raw(data as *mut Measure) };
  handle.dispose();
  drop(handle);
}

#[no_mangle]
pub extern "C" fn Reload(data: *mut c_void, rm: *mut c_void, max_value: &mut f64) {
  let measure_ptr: *mut Measure = data as *mut Measure;
  let measure = unsafe { &mut *measure_ptr };
  measure.reload(Api::new(rm), max_value);
}

#[no_mangle]
pub extern "C" fn Update(data: *mut c_void) -> f64 {
  let measure_ptr: *mut Measure = data as *mut Measure;
  let measure = unsafe { &mut *measure_ptr };
  measure.update()
}

#[no_mangle]
pub extern "C" fn GetString(data: *mut c_void) -> *const u16 {
  let measure_ptr: *mut Measure = data as *mut Measure;
  let measure = unsafe { &mut *measure_ptr };

  if let Some(string_value) = measure.get_string() {
    return string_value.to_wchar_vec().as_ptr();
  }

  std::ptr::null()
}

#[no_mangle]
pub extern "C" fn ExecuteBang(data: *mut c_void, args: *const u16) {
  let measure_ptr: *mut Measure = data as *mut Measure;
  let measure = unsafe { &mut *measure_ptr };
  measure.execute_bang(String::from_wchar_ptr(args));
}
