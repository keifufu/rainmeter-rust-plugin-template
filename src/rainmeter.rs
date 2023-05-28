#![allow(unused)]
use crate::utils::{wchar_t, Wchar};
use std::os::raw::{c_int, c_void};

pub struct Api {
  rm: *mut c_void,
}

impl Api {
  pub fn new(rm: *mut c_void) -> Api {
    Api { rm }
  }
  pub fn read_string(
    &self,
    option: &str,
    def_value: &str,
    replace_measures: Option<bool>,
  ) -> String {
    unsafe {
      String::from_wchar_ptr(RmReadString(
        self.rm,
        option.to_wchar_vec().as_ptr(),
        def_value.to_wchar_vec().as_ptr(),
        replace_measures.unwrap_or(true),
      ))
    }
  }
  pub fn read_path(&self, option: &str, def_value: &str) -> String {
    unsafe {
      String::from_wchar_ptr(RmPathToAbsolute(
        self.rm,
        self
          .read_string(option, def_value, None)
          .to_wchar_vec()
          .as_ptr(),
      ))
    }
  }
  pub fn read_double(&self, option: &str, def_value: f64) -> f64 {
    unsafe { RmReadFormula(self.rm, option.to_wchar_vec().as_ptr(), def_value) }
  }
  pub fn read_int(&self, option: &str, def_value: i32) -> i32 {
    unsafe { RmReadFormula(self.rm, option.to_wchar_vec().as_ptr(), def_value as f64) as i32 }
  }
  pub fn replace_variables(&self, str: &str) -> String {
    unsafe { String::from_wchar_ptr(RmReplaceVariables(self.rm, str.to_wchar_vec().as_ptr())) }
  }
  pub fn get_measure_name(&self) -> String {
    unsafe { String::from_wchar_ptr(RmGet(self.rm, RmGetType::MeasureName) as *mut wchar_t) }
  }
  pub fn get_skin(&self) -> *mut c_void {
    unsafe { RmGet(self.rm, RmGetType::Skin) }
  }
  pub fn get_settings_file(&self) -> String {
    unsafe { String::from_wchar_ptr(RmGet(self.rm, RmGetType::SettingsFile) as *mut wchar_t) }
  }
  pub fn get_skin_name(&self) -> String {
    unsafe { String::from_wchar_ptr(RmGet(self.rm, RmGetType::SkinName) as *mut wchar_t) }
  }
  pub fn execute(&self, skin: *mut c_void, command: &str) {
    unsafe { RmExecute(skin, command.to_wchar_vec().as_ptr()) }
  }
  pub fn execute_self(&self, command: &str) {
    unsafe { RmExecute(self.get_skin(), command.to_wchar_vec().as_ptr()) }
  }
  pub fn get_skin_window(&self) -> *mut c_void {
    unsafe { RmGet(self.rm, RmGetType::SkinWindowHandle) }
  }
  pub fn log(&self, log_type: LogType, message: String) -> i32 {
    unsafe { RmLog(self.rm, log_type, message.to_wchar_vec().as_ptr()) }
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
// TODO: actually expose lslog?

#[cfg(target_arch = "x86_64")]
#[link(name = "api/x64/rainmeter")]
extern "C" {
  fn RmReadString(
    rm: *mut c_void,
    option: *const wchar_t,
    defValue: *const wchar_t,
    replaceMeasures: bool,
  ) -> *mut wchar_t;
  fn RmReadFormula(rm: *mut c_void, option: *const wchar_t, defValue: f64) -> f64;
  fn RmReplaceVariables(rm: *mut c_void, string: *const wchar_t) -> *mut wchar_t;
  fn RmPathToAbsolute(rm: *mut c_void, relativePath: *const wchar_t) -> *mut wchar_t;
  fn RmExecute(skin: *mut c_void, command: *const wchar_t);
  fn RmGet(rm: *mut c_void, rm_get_type: RmGetType) -> *mut c_void;
  fn LSLog(log_type: c_int, unused: *const wchar_t, message: *const wchar_t) -> c_int;
  fn RmLog(rm: *mut c_void, level: LogType, message: *const wchar_t) -> c_int;
}

#[cfg(target_arch = "x86")]
#[link(name = "api/x86/rainmeter")]
extern "C" {
  #[cfg_attr(target_arch = "x86", link_name = "RmReadString@16")]
  fn RmReadString(
    rm: *mut c_void,
    option: *const wchar_t,
    defValue: *const wchar_t,
    replaceMeasures: bool,
  ) -> *mut wchar_t;
  fn RmReadFormula(rm: *mut c_void, option: *const wchar_t, defValue: f64) -> f64;
  fn RmReplaceVariables(rm: *mut c_void, string: *const wchar_t) -> *mut wchar_t;
  fn RmPathToAbsolute(rm: *mut c_void, relativePath: *const wchar_t) -> *mut wchar_t;
  fn RmExecute(skin: *mut c_void, command: *const wchar_t);
  fn RmGet(rm: *mut c_void, rm_get_type: RmGetType) -> *mut c_void;
  fn LSLog(log_type: c_int, unused: *const wchar_t, message: *const wchar_t) -> c_int;
  fn RmLog(rm: *mut c_void, level: LogType, message: *const wchar_t) -> c_int;
}
