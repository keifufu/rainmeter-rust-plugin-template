mod measure;
mod rainmeter;

use measure::Measure;
use rainmeter::api::RmApi;
use rainmeter::types::*;

#[export_name = "Initialize"]
fn initialize(data: &mut RmData, rm: RmRm) {
  let measure = Measure::new(RmApi::new(rm));
  set_data!(data, measure);
}

#[export_name = "Finalize"]
fn finalize(data: RmData) {
  let measure = rm_take_data!(data, Measure);
  measure.dispose();
  drop(measure);
}

#[export_name = "Reload"]
fn reload(data: RmData, rm: RmRm, max_value: &mut f64) {
  let measure = rm_borrow_data!(data, Measure);
  measure.reload(RmApi::new(rm), max_value);
}

#[export_name = "Update"]
fn update(data: RmData) -> f64 {
  let measure = rm_borrow_data!(data, Measure);
  measure.update()
}

#[export_name = "GetString"]
fn get_string(data: RmData) -> RmString {
  let measure = rm_borrow_data!(data, Measure);

  if let Some(string) = measure.get_string() {
    return rm_to_string!(string);
  }

  rm_null_string!()
}

#[export_name = "ExecuteBang"]
fn execute_bang(data: RmData, args: RmString) {
  let measure = rm_borrow_data!(data, Measure);
  measure.execute_bang(rm_parse_string!(args));
}

// Example of a custom function
#[export_name = "ToRandomCase"]
fn to_random_case(data: RmData, argc: i32, argv: RmArgv) -> RmString {
  let _ = rm_borrow_data!(data, Measure);
  let args = rm_parse_args!(&argv, argc);

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
    return rm_to_string!(result);
  }

  rm_null_string!()
}
