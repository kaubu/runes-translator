use lazy_static::lazy_static;

struct Letter {
    rune: &'static str,
    transliteration: &'static str,
    ipa: Vec<&'static str>,
}

impl Letter {
    fn new(
        rune: &'static str,
        transliteration: &'static str,
        ipa: Vec<&'static str>
    ) -> Letter {
        Letter {
            rune,
            transliteration,
            ipa,
        }
    }
}

struct Alphabet {
    letters: Vec<Letter>,
}

impl Alphabet {
    fn new(letters: Vec<Letter>) -> Alphabet {
        Alphabet {
            letters
        }
    }
}

// lazy_static! {
//     static ref ELDER_FUTHARK: Alphabet = Alphabet::new(
//         vec!(
//             Letter::new("ᚠ", "f", vec!("ɸ", "f")),
//             Letter::new("ᚢ", "u", vec!("u(ː)")),
//             Letter::new("ᚦ", "þ", vec!("θ", "ð")),
//             Letter::new("ᚨ", "a", vec!("a(ː)")),
//             Letter::new("ᚱ", "r", vec!("r")),
//             Letter::new("ᚲ", "k", vec!("k")),
//             Letter::new("ᚷ", "g", vec!("g")),
//             Letter::new("ᚹ", "w", vec!("w")),
//             Letter::new("ᚺ", "h", vec!("h")),
//             Letter::new("ᚾ", "n", vec!("n")),
//             Letter::new("ᛁ", "i", vec!("i(ː)")),
//             Letter::new("ᛃ", "j", vec!("j")),
//             Letter::new("ᛇ", "ï", vec!("æː")),
//             Letter::new("ᛈ", "p", vec!("p")),
//             Letter::new("ᛉ", "z", vec!("z")),
//             Letter::new("ᛊ", "s", vec!("s")),
//             Letter::new("ᛏ", "t", vec!("t")),
//             Letter::new("ᛒ", "b", vec!("b")),
//             Letter::new("ᛖ", "e", vec!("e(ː)")),
//             Letter::new("ᛗ", "m", vec!("m")),
//             Letter::new("ᛚ", "l", vec!("l")),
//             Letter::new("ᛜ", "ŋ", vec!("ŋ")),
//             Letter::new("ᛞ", "d", vec!("d")),
//             Letter::new("ᛟ", "o", vec!("o(ː)")),
//         )
//     );
// }