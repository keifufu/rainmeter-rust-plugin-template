#![allow(unused)]
use std::os::raw::{c_int, c_void};

pub struct RainmeterAPI {
  rm: *mut c_void,
}

// If we call .to_ptr() here, it will sometimes get deallocated too early.
pub fn to_wchar(str: String) -> Vec<u16> {
  format!("{}\0", str).encode_utf16().collect::<Vec<u16>>()
}

pub fn from_wchar(ptr: *const u16) -> String {
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

impl RainmeterAPI {
  pub fn new(rm: *mut c_void) -> RainmeterAPI {
    RainmeterAPI { rm }
  }
  pub fn read_string(
    &self,
    option: &str,
    def_value: &str,
    replace_measures: Option<bool>,
  ) -> String {
    unsafe {
      from_wchar(RmReadString(
        self.rm,
        to_wchar(option.to_string()).as_ptr(),
        to_wchar(def_value.to_string()).as_ptr(),
        replace_measures.unwrap_or(true),
      ))
    }
  }
  pub fn read_path(&self, option: &str, def_value: &str) -> String {
    unsafe {
      from_wchar(RmPathToAbsolute(
        self.rm,
        to_wchar(self.read_string(option, def_value, None)).as_ptr(),
      ))
    }
  }
  pub fn read_double(&self, option: &str, def_value: f64) -> f64 {
    unsafe { RmReadFormula(self.rm, to_wchar(option.to_string()).as_ptr(), def_value) }
  }
  pub fn read_int(&self, option: &str, def_value: i32) -> i32 {
    unsafe {
      RmReadFormula(
        self.rm,
        to_wchar(option.to_string()).as_ptr(),
        def_value as f64,
      ) as i32
    }
  }
  pub fn replace_variables(&self, str: &str) -> String {
    unsafe {
      from_wchar(RmReplaceVariables(
        self.rm,
        to_wchar(str.to_string()).as_ptr(),
      ))
    }
  }
  pub fn get_measure_name(&self) -> String {
    unsafe { from_wchar(RmGet(self.rm, RmGetType::MeasureName) as *mut u16) }
  }
  pub fn get_skin(&self) -> *mut c_void {
    unsafe { RmGet(self.rm, RmGetType::Skin) }
  }
  pub fn get_settings_file(&self) -> String {
    unsafe { from_wchar(RmGet(self.rm, RmGetType::SettingsFile) as *mut u16) }
  }
  pub fn get_skin_name(&self) -> String {
    unsafe { from_wchar(RmGet(self.rm, RmGetType::SkinName) as *mut u16) }
  }
  pub fn execute(&self, skin: *mut c_void, command: &str) {
    unsafe { RmExecute(skin, to_wchar(command.to_string()).as_ptr()) }
  }
  pub fn execute_self(&self, command: &str) {
    unsafe { RmExecute(self.get_skin(), to_wchar(command.to_string()).as_ptr()) }
  }
  pub fn get_skin_window(&self) -> *mut c_void {
    unsafe { RmGet(self.rm, RmGetType::SkinWindowHandle) }
  }
  pub fn log(&self, log_type: LogType, message: String) -> i32 {
    unsafe { RmLog(self.rm, log_type, to_wchar(message).as_ptr()) }
  }
}

#[repr(C)]
pub enum LogType {
  Error = 1,
  Warning = 2,
  Notice = 3,
  Debug = 4,
}

#[repr(C)]
enum RmGetType {
  MeasureName = 0,
  Skin = 1,
  SettingsFile = 2,
  SkinName = 3,
  SkinWindowHandle = 4,
}

#[link(name = "api/rainmeter")]
extern "C" {
  fn RmReadString(
    rm: *mut c_void,
    option: *const u16,
    defValue: *const u16,
    replaceMeasures: bool,
  ) -> *mut u16;
  fn RmReadFormula(rm: *mut c_void, option: *const u16, defValue: f64) -> f64;
  fn RmReplaceVariables(rm: *mut c_void, string: *const u16) -> *mut u16;
  fn RmPathToAbsolute(rm: *mut c_void, relativePath: *const u16) -> *mut u16;
  fn RmExecute(skin: *mut c_void, command: *const u16);
  fn RmGet(rm: *mut c_void, rm_get_type: RmGetType) -> *mut c_void;
  fn LSLog(log_type: c_int, unused: *const u16, message: *const u16) -> c_int;
  fn RmLog(rm: *mut c_void, level: LogType, message: *const u16) -> c_int;
}
