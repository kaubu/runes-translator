use std::collections::HashMap;

use lazy_static::lazy_static;

lazy_static! {
    pub static ref ELDER_FUTHARK: HashMap<
    &'static str,
    &'static str
    > = HashMap::from([
        ("ᚠ", "f"),
        ("ᚢ", "u"),
        ("ᚦ", "þ"),
        ("ᚨ", "a"),
        ("ᚱ", "r"),
        ("ᚲ", "k"),
        ("ᚷ", "g"),
        ("ᚹ", "w"),
        ("ᚺ", "h"),
        ("ᚻ", "h"),
        ("ᚾ", "n"),
        ("ᛁ", "i"),
        ("ᛃ", "j"),
        ("ᛇ", "ï"),
        ("ᛈ", "p"),
        ("ᛉ", "z"),
        ("ᛊ", "s"),
        ("ᛏ", "t"),
        ("ᛒ", "b"),
        ("ᛖ", "e"),
        ("ᛗ", "m"),
        ("ᛚ", "l"),
        ("ᛜ", "ŋ"),
        ("ᛞ", "d"),
        ("ᛟ", "o"),
    ]);

    static ref ELDER_FUTHARK_ALT: HashMap<String, String> = HashMap::new();
    static ref FUTHORC: HashMap<String, String> = HashMap::new();
    static ref FUTHORC_ALT: HashMap<String, String> = HashMap::new();
    static ref YOUNGER_FUTHARK: HashMap<String, String> = HashMap::new();
    static ref YOUNGER_FUTHARK_ALT: HashMap<String, String> = HashMap::new();
    static ref FUTHORK: HashMap<String, String> = HashMap::new();
    static ref FUTHORK_ALT: HashMap<String, String> = HashMap::new();
}

