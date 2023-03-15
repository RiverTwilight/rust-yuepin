use yuepin::yuepin_entry::*;

let line = "你好嗎 你好吗 [ni3 hao3 ma5] {nei5 hou2 maa1} /how are you?/";
let entry = CedictEntry::new(line).unwrap();

assert_eq!(entry.traditional, "你好嗎");
assert_eq!(entry.simplified, "你好吗");
assert_eq!(entry.pinyin, Some(
    vec![
        Syllable::new("ni", "3"),
        Syllable::new("hao", "3"),
        Syllable::new("ma", "5"),
    ]
));
assert_eq!(entry.jyutping, Some(
    vec![
        Syllable::new("nei", "5"),
        Syllable::new("hou", "2"),
        Syllable::new("maa", "1"),
    ]
));
assert_eq!(entry.definitions, Some(vec!["how are you?".to_string()]));
