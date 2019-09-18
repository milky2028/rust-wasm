use serde::{Deserialize, Serialize};
use serde_json::Result;
use wasm_bindgen::prelude::*;

#[derive(Serialize, Deserialize)]
struct Payload {
  notionals: Vec<f64>,
  metrics: Vec<f64>,
}

fn init_panic_hook() {
  console_error_panic_hook::set_once();
}

#[wasm_bindgen]
pub fn weighted_average(json: &str) -> f64 {
  init_panic_hook();
  let parsed: Result<Payload> = serde_json::from_str(json);
  match parsed {
    Ok(v) => {
      let notionals = &v.notionals;
      let metrics = &v.metrics;

      if notionals.len() != metrics.len() {
        panic!("Invalid inputs")
      } else {
        let total_notional = &notionals.into_iter().fold(0_f64, |acc, curr| acc + curr);
        let total = &notionals
          .into_iter()
          .enumerate()
          .fold(0_f64, |acc, (i, value)| value * metrics[i] + acc);

        total / total_notional
      }
    }
    Err(_) => panic!("Couldn't parse json structure"),
  }
}

mod tests {

  #[test]
  fn test_weighted_avg() {
    const TEST_DATA: &str = r#"{"notionals":[2925816.124,4138782.768,1671373.879,2963345.887,907077.281,2169818.582,566923.3007,2244999.066,4535386.405,4538409.996,3127148.926,804892.4123,837133.8823,630715.0978,1814154.562,1992000.479,590284.2022,2061539.273,453538.6405,3716019.476,2483483.284,2267693.203,1671373.879,559247.1217,1435073.965,2488163.375,2361486.932,752118.2455,839886.3713,2484060.26,1659601.567,2555817.363,954700.7616,832180.9918,1954595.206,3312730.996,3454027.573,902541.8946,832180.9918,1131011.985,360063.9772,150423.6491,1160792.447,1125300.028,1131004.882,2197142.742,717826.6014,2954171.753,516621.9292,993745.9521,417458.1354,832180.9918,434999.8313,2978237.073,1928121.43,1347852.412,1525948.365,729211.2349,2993355.027,794268.5814,370389.8898,1163622.686,2188323.941,2896980.697,370404.1006,2440399.726,1133846.601,1133846.601,2111222.372,568619.6194,907077.281,3941080.663,2267693.203,294044.2186,832180.9918,1446691.644,3312080.347,394859.7613,2267693.203,669832.3583,2202681.858,1209436.375,3025003.057,1131004.882,3173739.345,453538.6405,872189.6958,1122733.348,1727766.252,3395642.486,482289.7508,2222509.416,1199657.561,3328578.907,2005309.461,2267693.203,755897.7342,452404.7939,4083177.389,1060422.107,317768.2461,1259054.514,1119673.519,983549.8553,731331.0578,1357197.292,1677878.234,3618957.81,423759.8376,241144.8754,748131.186,3176764.947,994296.2539,3520249.703,890921.899,1984231.552,2934114.398,1809538.391,771015.6889,990687.9782,732127.3659,3353630.794,1022913.447,890746.9238,1130179.088,866803.5537,98819.42996,542274.4539,852923.4158,1593385.431,2579314.942,1584521.989,1656040.174,746482.0453,690591.6924,2052902.146,1370432.287,2138600.635,2954792.738,2494462.523,2128512.538,1745960.991,111953.033,1656040.174,905683.9196,1451582.469,5410836.465,750228.5012,1128148.886,2539779.796,747635.3143,372590.3232,4354667.993,2963119.118,1662975.015,1643557.459,2477818.903,1132627.045,1239314.533,1481748.156,762479.1347,839886.3713,4400428.333,1667174.447,1450626.781,1935968.525,2713846.211,832180.9918,1468698.987,966769.0743,294063.116,679284.0037,1492935.82,975180.5072,212878.9133,989538.8603,1158663.089,128934.5564,4386872.471,906221.9849,2738436.302,1360615.922,2513359.966,3685862.664,2257707.031,702019.3465,1090770.45,1814154.562,462899.1741,967591.6148,1662975.015,1279605.524,1774064.985,497028.6512,3732245.063,1133846.601,905064.359,1639396.554,2484060.26,3595353.359,3465477.659,3985174.572,430187.3049,2882827.479,224504.4649,360398.5472,1061083.22,746449.0125,1091057.694,608518.217,2503060.618,1210531.489,2152258.629,821252.4914,566923.3007,1209436.375,368128.2605,1024983.189,904636.3724,1651879.277,300847.2982,2471452.086,2834449.072,1639948.481,3628309.124,4037019.007,1481559.559,1248515.425,2519659.114,2075980.015,2480726.584,750227.6999,4456249.718,300847.2982,950430.2415,7388900.352,3947994.822,1500475.9,1437623.789,3278730.07,1717855.194,1126297.073,2194588.443,3587743.54,2079593.595,1257048.105,2110279.986,763656.5792,2261863.653,841077.9636,3135178.824,1130976.101,2494462.523,2250685.504,1370324.703,750228.5012,603085.7355,2250644.711,314277.9812,1380340.017,856804.3822,2632900.985,2848503.206,2355809.495,680307.9608,1389538.459,733371.9817,2231732.523,1135820.311,4421717.573,1133846.601,715812.2445,2256653.815,3269127.866,1504236.491,835632.6538,2074183.383,722664.2756,2471388.882,1110274.684,474181.9389,2496542.975,832180.9918,2267693.203,266205.5633,1443077.486,2587514.505,703523.7633,1267967.206,832180.9918,2418872.752,740836.4718,565349.5636,2477818.903,1130990.569,3268660.928,907077.281,291413.6057,568615.0331,300733.9912],"metrics":[3490,3490,4770,2720,2220,610,1350,2720,6500,2720,2720,3490,2720,2720,2720,2220,2220,1766,2220,2720,2220,2720,2720,1766,3490,3490,2720,3490,3490,2720,3490,2720,2220,1766,3490,4770,940,3490,3490,2720,3490,3490,2720,3490,2220,2720,2220,3490,1350,2720,1766,1350,3490,3490,4770,3490,3490,3490,2720,940,1766,1350,2220,1350,2720,1350,940,2720,2720,3490,3490,2720,6500,2220,3490,3490,1766,3490,1766,1766,3490,3490,1766,3490,2720,3490,2220,3490,3490,940,3490,3490,2220,4770,2720,2720,2720,3490,2720,2720,3490,2220,3490,3490,3490,3490,940,3490,1766,3490,1766,3490,940,2720,3490,2220,2720,3490,8070,940,2720,3490,2720,2720,2220,3490,2720,2220,3490,2720,2220,2720,2220,940,1766,940,2220,2720,1766,4770,940,2220,3490,3490,3490,2720,3490,1350,3490,3490,1350,3490,3490,3490,1766,3490,3490,3490,3490,1766,3490,3490,3490,1350,940,2720,3490,3490,2720,1766,2720,1766,1766,1766,2720,3490,2720,1350,4770,3490,1766,2220,2220,3490,940,1350,3490,3490,6500,2220,2220,2220,2720,2720,2720,1766,1766,3490,3490,2220,940,3490,1766,2720,3490,3490,1766,2220,1766,3490,4770,2220,2720,3490,1766,3490,2220,2220,3490,3490,2720,3490,3490,3490,3490,2720,4770,940,2720,3490,2220,2720,2720,2720,2220,1766,1350,2720,2720,4770,2220,2720,1766,2720,3490,2220,3490,1766,4770,3490,3490,4770,1766,3490,2220,2220,3490,4770,1350,3490,2720,1766,2720,8070,2220,1766,2720,3490,3490,2720,3490,2220,2720,3490,3490,2220,1350,4770,3490,3490,3490,3490,3490,1766,1766,1766,1350,1350,2720,2720,3490,1766,2720,2720,2720,2220,2720,2720,2220,3490]}"#;

    use super::*;
    assert_eq!(2869.939546094432, weighted_average(TEST_DATA));
  }
}
