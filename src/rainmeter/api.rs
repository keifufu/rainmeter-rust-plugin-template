#![allow(unused)]
use crate::rainmeter::types::*;

pub struct Api {
  rm: RmRm,
}

impl Api {
  pub fn new(rm: RmRm) -> Api {
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
  pub fn get_skin(&self) -> RmData {
    unsafe { RmGet(self.rm, RmGetType::Skin) }
  }
  pub fn get_settings_file(&self) -> String {
    unsafe { String::from_wchar_ptr(RmGet(self.rm, RmGetType::SettingsFile) as *mut wchar_t) }
  }
  pub fn get_skin_name(&self) -> String {
    unsafe { String::from_wchar_ptr(RmGet(self.rm, RmGetType::SkinName) as *mut wchar_t) }
  }
  pub fn execute(&self, skin: RmData, command: &str) {
    unsafe { RmExecute(skin, command.to_wchar_vec().as_ptr()) }
  }
  pub fn execute_self(&self, command: &str) {
    unsafe { RmExecute(self.get_skin(), command.to_wchar_vec().as_ptr()) }
  }
  pub fn get_skin_window(&self) -> RmData {
    unsafe { RmGet(self.rm, RmGetType::SkinWindowHandle) }
  }
  pub fn log<T: Into<String>>(&self, log_type: LogType, message: T) -> i32 {
    let message: String = message.into();
    unsafe { RmLog(self.rm, log_type, message.to_wchar_vec().as_ptr()) }
  }
  pub fn ls_log<T: Into<String>>(log_type: LogType, message: T) -> i32 {
    let message: String = message.into();
    unsafe { LSLog(log_type, std::ptr::null(), message.to_wchar_vec().as_ptr()) }
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

#[cfg(target_arch = "x86_64")]
#[link(name = "api/x64/rainmeter")]
extern "C" {
  fn RmReadString(
    rm: RmRm,
    option: *const wchar_t,
    defValue: *const wchar_t,
    replaceMeasures: bool,
  ) -> *mut wchar_t;
  fn RmReadFormula(rm: RmRm, option: *const wchar_t, defValue: f64) -> f64;
  fn RmReplaceVariables(rm: RmRm, string: *const wchar_t) -> *mut wchar_t;
  fn RmPathToAbsolute(rm: RmRm, relativePath: *const wchar_t) -> *mut wchar_t;
  fn RmExecute(skin: RmRm, command: *const wchar_t);
  fn RmGet(rm: RmRm, rm_get_type: RmGetType) -> RmData;
  fn LSLog(log_type: LogType, unused: *const wchar_t, message: *const wchar_t) -> i32;
  fn RmLog(rm: RmRm, level: LogType, message: *const wchar_t) -> i32;
}

#[cfg(target_arch = "x86")]
#[link(name = "api/x86/rainmeter")]
extern "C" {
  fn LSLog(log_type: LogType, unused: *const wchar_t, message: *const wchar_t) -> i32;
}

#[cfg(target_arch = "x86")]
#[link(name = "x86_api_wrapper")]
extern "C" {
  #[cfg_attr(target_arch = "x86", link_name = "RmReadStringWrapper")]
  fn RmReadString(
    rm: RmRm,
    option: *const wchar_t,
    defValue: *const wchar_t,
    replaceMeasures: bool,
  ) -> *mut wchar_t;
  #[cfg_attr(target_arch = "x86", link_name = "RmReadFormulaWrapper")]
  fn RmReadFormula(rm: RmRm, option: *const wchar_t, defValue: f64) -> f64;
  #[cfg_attr(target_arch = "x86", link_name = "RmReplaceVariablesWrapper")]
  fn RmReplaceVariables(rm: RmRm, string: *const wchar_t) -> *mut wchar_t;
  #[cfg_attr(target_arch = "x86", link_name = "RmPathToAbsoluteWrapper")]
  fn RmPathToAbsolute(rm: RmRm, relativePath: *const wchar_t) -> *mut wchar_t;
  #[cfg_attr(target_arch = "x86", link_name = "RmExecuteWrapper")]
  fn RmExecute(skin: RmRm, command: *const wchar_t);
  #[cfg_attr(target_arch = "x86", link_name = "RmGetWrapper")]
  fn RmGet(rm: RmRm, rm_get_type: RmGetType) -> RmData;
  #[cfg_attr(target_arch = "x86", link_name = "RmLogWrapper")]
  fn RmLog(rm: RmRm, level: LogType, message: *const wchar_t) -> i32;
}
