mod measure;
mod rainmeter;

use measure::Measure;
use rainmeter::api::Api;
use rainmeter::types::*;

#[no_mangle]
pub extern "C" fn Initialize(data: &mut RmData, rm: RmRm) {
  let measure = Measure::new(Api::new(rm));
  set_data!(data, measure);
}

#[no_mangle]
pub extern "C" fn Finalize(data: RmData) {
  let measure = take_data!(data, Measure);
  measure.dispose();
  drop(measure);
}

#[no_mangle]
pub extern "C" fn Reload(data: RmData, rm: RmRm, max_value: &mut f64) {
  let measure = borrow_data!(data, Measure);
  measure.reload(Api::new(rm), max_value);
}

#[no_mangle]
pub extern "C" fn Update(data: RmData) -> f64 {
  let measure = borrow_data!(data, Measure);
  measure.update()
}

#[no_mangle]
pub extern "C" fn GetString(data: RmData) -> RmString {
  let measure = borrow_data!(data, Measure);

  if let Some(string) = measure.get_string() {
    return to_rm_string!(string);
  }

  null_rm_string!()
}

#[no_mangle]
pub extern "C" fn ExecuteBang(data: RmData, args: RmString) {
  let measure = borrow_data!(data, Measure);
  measure.execute_bang(parse_rm_string!(args));
}

// Example of a custom function
#[no_mangle]
pub extern "C" fn ToRandomCase(data: RmData, argc: i32, argv: RmArgv) -> RmString {
  let _ = borrow_data!(data, Measure);
  let args = parse_rm_args!(&argv, argc);

  if args.len() == 1 {
    // This isn't actually random to not require the rand crate,
    // so I just alternative upper-lower for this example.
    let mut result = String::new();
    let mut uppercase = false;
    for c in args[0].chars() {
      if uppercase {
        result.push(c.to_ascii_uppercase());
      } else {
        result.push(c.to_ascii_lowercase());
      }
      uppercase = !uppercase;
    }
    return to_rm_string!(result);
  }

  null_rm_string!()
}
