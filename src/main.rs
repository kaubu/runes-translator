mod runes;

use std::{io::{self, Write}, collections::HashMap, os::raw};

fn main() {
    println!("Hello, world!");

    loop {
        let mut raw_runes = input("Runes: ");

        let elder_futhark = transliterate_runes(
            raw_runes,
            &*runes::ELDER_FUTHARK
        );

        println!("[Elder Futhark]\n{}\n", elder_futhark);
    }
}

fn transliterate_runes(
    message: String,
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