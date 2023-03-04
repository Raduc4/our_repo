// #![deny(clippy::all)]

use std::path::Path;

use vinyana::NeuralNetwork;

#[macro_use]
extern crate napi_derive;

#[napi]
pub fn predict(data: Vec<i32>) -> f32 {
  let nn = NeuralNetwork::load(Path::new("./bin/churn_model.bin")).unwrap();
  let normalized_record = normalize_dataset(&data);

  let result = nn.predict(&normalized_record);

  *result.first().unwrap()
}

fn normalize_dataset(train_data: &Vec<i32>) -> Vec<f32> {
  let debt = train_data[0] as f32;
  let contract = train_data[1] as f32;
  let incidents = train_data[2] as f32;
  let abs = train_data[3] as f32;
  let qty_calls = train_data[4] as f32;
  let qty_port_rez = train_data[5] as f32;

  let record = vec![
    normalize(debt, 0.0, 6.0),
    normalize(contract, 0.0, 3.0),
    normalize(incidents, 0.0, 7.0),
    normalize(abs, -325.0, 2550.0),
    normalize(qty_calls, 0.0, 90.0),
    normalize(qty_port_rez, 0.0, 7.0),
  ];
  record
}

fn normalize(x: f32, min_x: f32, max_x: f32) -> f32 {
  (x - min_x) / (max_x - min_x)
}
