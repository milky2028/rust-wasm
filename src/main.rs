use serde::{Deserialize, Serialize};
use serde_json::Result;

const JSON: &str = r#"
  {
    "nums": [
      15,
      30,
      55,
      76,
      548
    ],
    "last_name": "Gross"
  }
"#;

#[derive(Serialize, Deserialize)]
struct JSON {
  nums: Vec<u32>,
  last_name: String,
}

fn parse_json(json: &str) -> JSON {
  let parsed: Result<JSON> = serde_json::from_str(json);
  match parsed {
    Ok(v) => v,
    Err(_) => panic!("Couldn't parse"),
  }
}

fn main() {
  let sum = parse_json(&JSON)
    .nums
    .into_iter()
    .fold(0, |acc, curr| acc + curr);
  println!("{}", sum);
}
