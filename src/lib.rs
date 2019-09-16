use serde::{Deserialize, Serialize};
use serde_json::Result;
use wasm_bindgen::prelude::*;

#[derive(Serialize, Deserialize)]
struct Payload {
  notionals: Vec<f32>,
  metrics: Vec<f32>
}

#[wasm_bindgen]
pub fn weighted_average(json: &str) -> f32 {
  let parsed: Result<Payload> = serde_json::from_str(json);
  match parsed {
    Ok(v) => {
      let notionals = &v.notionals;
      let metrics = &v.metrics;

      if notionals.len() != metrics.len() {
        panic!("Invalid inputs")
      } else {
        let total_notional = &notionals.into_iter().fold(0_f32, |acc, curr| { acc + curr });
        let total = vec![&notionals, &metrics].into_iter().fold(0_f32, |acc, curr| { acc + (curr[0] * curr[1]) });
        total / total_notional
      }
    },
    Err(_) => panic!("Couldn't parse json structure"),
  }
}
