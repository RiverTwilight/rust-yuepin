use serde_json::Value;
use std::collections::HashMap;

// fn to_yuepin(simplified: &str, dic_trad: &HashMap<String, Value>, dic_simp: &HashMap<String, Value>) -> String {
//     let mut yuepin = String::new();
//     for char in simplified.chars() {
//         let pinyin = match dic_simp.get(&char.to_string()) {
//             Some(val) => val["pinyin"].as_str().unwrap(),
//             None => match dic_trad.get(&char.to_string()) {
//                 Some(val) => val["pinyin"].as_str().unwrap(),
//                 None => char.to_string(),
//             },
//         };
//         yuepin.push_str(pinyin);
//         yuepin.push(' ');
//     }
//     yuepin.trim().to_owned()
// }

fn main() {
    let dic_trad: HashMap<String, Value> = serde_json::from_str(include_str!("dic_trad.json")).unwrap();
    let dic_simp: HashMap<String, Value> = serde_json::from_str(include_str!("dic_simp.json")).unwrap();
    // let simplified = "你好";
    // let yuepin = to_yuepin(simplified, &dic_trad, &dic_simp);
    let hekko = include_str!("dic_trad.json");
    println!("{}", hekko);
}
