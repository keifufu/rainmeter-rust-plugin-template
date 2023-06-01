use rainmeter::api::RmApi;

// Example on how to return a string or a number based on if the measure provided `Type=String` or `Type=Number`
pub enum DataType {
  String,
  Number,
}

fn string_to_enum(s: &str) -> Option<DataType> {
  match s {
    "String" => Some(DataType::String),
    "Number" => Some(DataType::Number),
    _ => None,
  }
}

pub struct Measure {
  pub rm_api: RmApi,
  pub count: i32,
  pub data_type: DataType,
}

impl Measure {
  pub fn new(api: RmApi) -> Measure {
    Measure {
      rm_api: api,
      count: 0,
      data_type: DataType::String,
    }
  }
  pub fn dispose(&self) {}
  #[allow(unused)]
  pub fn reload(&mut self, rm_api: RmApi, max_value: &mut f64) {
    self.rm_api = rm_api;
    let data_type_string = self.rm_api.read_string("Type", "String", None);

    let data_type = string_to_enum(&data_type_string);

    if let Some(value) = data_type {
      self.data_type = value;
    }
  }
  pub fn execute_bang(&mut self, args: String) {
    let bang = args.to_lowercase();

    #[allow(clippy::single_match)]
    match bang.as_str() {
      "resetcounter" => self.count = 0,
      _ => {}
    }
  }
  pub fn update(&mut self) -> f64 {
    self.count += 1;
    match self.data_type {
      DataType::Number => self.count as f64,
      DataType::String => 0.0,
    }
  }
  pub fn get_string(&mut self) -> Option<String> {
    match self.data_type {
      DataType::Number => None,
      DataType::String => Some(format!("Count: {}", self.count)),
    }
  }
}
