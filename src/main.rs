mod runes;

use std::{io::{self, Write}, collections::HashMap};

fn main() {
    loop {
        let raw_runes = input("Runes: ");

        let elder_futhark = transliterate_runes(
            &raw_runes,
            &*runes::ELDER_FUTHARK
        );
        let elder_futhark_alt = transliterate_runes(
            &raw_runes,
            &*runes::ELDER_FUTHARK_ALT
        );
        let futhorc = transliterate_runes(
            &raw_runes,
            &*runes::FUTHORC
        );
        let futhorc_alt = transliterate_runes(
            &raw_runes,
            &*runes::FUTHORC_ALT
        );
        let younger_futhark = transliterate_runes(
            &raw_runes,
            &*runes::YOUNGER_FUTHARK
        );
        let younger_futhark_alt = transliterate_runes(
            &raw_runes,
            &*runes::YOUNGER_FUTHARK_ALT
        );
        let futhork = transliterate_runes(
            &raw_runes,
            &*runes::FUTHORK
        );
        let futhork_alt = transliterate_runes(
            &raw_runes,
            &*runes::FUTHORK_ALT
        );

        println!("[Elder Futhark / Proto-Germanic]\n— {}", elder_futhark);
        println!("[Elder Futhark (Alternative)]\n— {}", elder_futhark_alt);
        println!("[Futhorc / Anglo-Saxon]\n— {}", futhorc);
        println!("[Futhorc (Alternative)]\n— {}", futhorc_alt);
        println!("[Younger Futhark / Old Norse]\n— {}", younger_futhark);
        println!("[Younger Futhark (Alternative)]\n— {}", younger_futhark_alt);
        println!("[Futhork / Medieval]\n— {}", futhork);
        println!("[Futhork (Alternative)]\n— {}", futhork_alt);

        println!();
    }
}

fn transliterate_runes(
    message: &String,
    runes: &HashMap<&'static str, &'static str>
) -> String {
    let mut result = message.clone();

    for (rune, transliteration) in runes {
        result = result.replace(rune, transliteration);
    }

    result
}

fn input(message: &str) -> String {
    let mut input = String::new();

    print!("{}", &message);
    io::stdout().flush().unwrap();

    io::stdin().read_line(&mut input).expect("Failed to read line.");

    input.trim().to_owned()
}