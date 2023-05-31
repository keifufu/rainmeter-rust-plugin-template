use std::ffi::c_void;

#[allow(non_camel_case_types)]
pub type wchar_t = u16;
pub type RmData = *mut c_void;
pub type RmRm = *mut c_void;
pub type RmString = *const wchar_t;
pub type RmArgv = *const RmString;

pub trait Wchar {
  fn to_wchar_vec(&self) -> Vec<wchar_t>;
  fn from_wchar_ptr(ptr: *const wchar_t) -> String;
}

impl Wchar for String {
  fn to_wchar_vec(&self) -> Vec<wchar_t> {
    to_wchar_vec(self)
  }
  fn from_wchar_ptr(ptr: *const wchar_t) -> String {
    from_wchar_ptr(ptr)
  }
}

impl Wchar for str {
  fn to_wchar_vec(&self) -> Vec<wchar_t> {
    to_wchar_vec(self)
  }
  fn from_wchar_ptr(ptr: *const wchar_t) -> String {
    from_wchar_ptr(ptr)
  }
}

// If we call .to_ptr() here, it will sometimes get deallocated too early.
fn to_wchar_vec<T: Into<String>>(str: T) -> Vec<wchar_t> {
  let str: String = str.into();
  str.encode_utf16().chain(Some(0)).collect::<Vec<wchar_t>>()
}

fn from_wchar_ptr(ptr: *const wchar_t) -> String {
  if ptr.is_null() {
    return String::new();
  }

  let mut len = 0;
  while unsafe { *ptr.offset(len) != 0 } {
    len += 1;
  }

  let slice = unsafe { std::slice::from_raw_parts(ptr, len as usize) };
  String::from_utf16_lossy(slice)
}
