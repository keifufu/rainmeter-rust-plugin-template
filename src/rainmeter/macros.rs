#[macro_export]
macro_rules! rm_borrow_data {
  ($data:expr, $ty:ty) => {
    unsafe { &mut *($data as *mut $ty) }
  };
}

#[macro_export]
macro_rules! rm_set_data {
  ($data:expr, $val:expr) => {
    let handle = Box::into_raw(Box::new($val));
    unsafe { *$data = std::mem::transmute(handle) }
  };
}

#[macro_export]
macro_rules! rm_drop_data {
  ($data: expr, $ty:ty) => {
    let data_to_drop = unsafe { Box::from_raw($data as *mut $ty) };
    drop(data_to_drop);
  };
}

#[macro_export]
macro_rules! rm_parse_string {
  ($data: expr) => {
    String::from_wchar_ptr($data)
  };
}

#[macro_export]
macro_rules! rm_to_string {
  ($data: expr) => {
    $data.to_wchar_vec().as_ptr()
  };
}

#[macro_export]
macro_rules! rm_null_string {
  () => {
    std::ptr::null()
  };
}

#[macro_export]
macro_rules! rm_parse_args {
  ($argv: expr, $argc: expr) => {
    unsafe {
      let argv_slice: &[RmString] = std::slice::from_raw_parts(*$argv, $argc as usize);
      let argv_vec: Vec<String> = argv_slice
        .iter()
        .map(|&arg| {
          let wide_string = {
            let mut len = 0;
            while *arg.add(len) != 0 {
              len += 1;
            }
            std::slice::from_raw_parts(arg, len)
          };

          String::from_utf16_lossy(wide_string)
        })
        .collect();
      argv_vec
    }
  };
}
