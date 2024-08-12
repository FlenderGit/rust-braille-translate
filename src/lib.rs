use std::{result, vec};

pub fn print_braille(i: u8) {
    print!("{}", get_braille(i));
}

pub fn get_braille(i: u8) -> char {
    return match std::char::from_u32( 0x2800 + i as u32 ) {
        Some(c) => c,
        None => '�',
    };
}

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn string_to_braille(string: &str) -> Vec<u8> {

    let mut index = 0;
    let mut last_index = 0;
    let length = string.len();
    let mut list_braille: Vec<u8> = Vec::new();
    let list_chars: Vec<char> = string.chars().collect();

    while index < length {

        while index < length && list_chars[index].is_alphabetic() { index += 1; }

        if last_index != index {
            let word = &string[last_index..index];
            let braille = word_to_braille(word);
            list_braille.extend(braille.iter());
        }

        if index >= length {
            break;
        }

        if let Some(result) = match_1(list_chars[index]) {
            list_braille.push(result);
            index += 1;
        }

        //println!("{} : {:?}", list_chars[index], list_braille);

        last_index = index;
    }

    return list_braille;
}

pub fn word_to_braille(word: &str) -> Vec<u8> {

    // Check if the word is a word sign
    if let Some(result) = match_wordsign(word) {
        return vec![result];
    }

    let length = word.len();
    let mut braille: Vec<u8> = Vec::new();
    let mut index: usize = 0;
    let ls_function = [match_3, match_4, match_5, match_6, match_7, match_8, match_9];
    let length_function: usize = ls_function.len();

    // Set the limit between the len of the word and the length of the function array to avoid overflow
    let limit_index =  min(length, length_function);

    // While the word is not fully parsed
    while index < length {

        if (length - index) >= 3 {
            let mut found: bool = false;

            // TD : min between m & (length - index) to avoid overflow
            let limit_word = min(limit_index, length - index - 2) as usize;

            for i in (0..limit_word).rev() {
                let string = &word[index..(index + i + 3)];
                let result = ls_function[i](string);
                //println!("{} : {:?} -- {}", string, result, i);
                if !result.is_empty() {
                    braille.extend(result.iter());
                    index += i + 3;
                    found = true;
                    break;
                }
            }

            if found {
                continue;
            }
        }
        

        

        // If a match was found, continue to the next iteration
        

        // Else, try 2 to 1 substring
        if let Some(result) = match_2(&word[index..min(index + 2, length) as usize]) {
            braille.push(result);
            index += 2;
            continue;
        }

        //println!("Adding : {}", match_1(word.chars().nth(index as usize).unwrap()));
        braille.push(match_1(word.chars().nth(index as usize).unwrap()).unwrap());
        index += 1;
    }

    return braille;

}


const NONE: u8 = 0;
const TL: u8 = 1 << 0;
const ML: u8 = 1 << 1;
const BL: u8 = 1 << 2;
const TR: u8 = 1 << 3;
const MR: u8 = 1 << 4;
const BR: u8 = 1 << 5;

const A: u8 = TL;
const B: u8 = TL | ML;
const C: u8 = TL | TR;
const D: u8 = TL | TR | MR;
const E: u8 = TL | MR;
const F: u8 = TL | ML | TR;
const G: u8 = TL | ML | TR | MR;
const H: u8 = TL | ML | MR;
const I: u8 = ML | TR;
const J: u8 = ML | TR | MR;
const K: u8 = TL | BL;
const L: u8 = TL | ML | BL;
const M: u8 = TL | TR | BL;
const N: u8 = TL | TR | MR | BL;
const O: u8 = TL | MR | BL;
const P: u8 = TL | ML | TR | BL;
const Q: u8 = TL | ML | TR | MR | BL;
const R: u8 = TL | ML | MR | BL;
const S: u8 = ML | TR | BL;
const T: u8 = ML | TR | MR | BL;
const U: u8 = TL | BL | BR;
const V: u8 = TL | ML | BL | BR;
const W: u8 = ML | TR | MR | BR;
const X: u8 = TL | TR | BL | BR;
const Y: u8 = TL | TR | MR | BL | BR;
const Z: u8 = TL | MR | BL | BR;


pub fn match_1(c: char) -> Option<u8> {
    let result = match c {
        'a' => A,
        'b' => B,
        'c' => C,
        'd' => D,
        'e' => E,
        'f' => F,
        'g' => G,
        'h' => H,
        'i' => I,
        'j' => J,
        'k' => K,
        'l' => L,
        'm' => M,
        'n' => N,
        'o' => O,
        'p' => P,
        'q' => Q,
        'r' => R,
        's' => S,
        't' => T,
        'u' => U,
        'v' => V,
        'w' => W,
        'x' => X,
        'y' => Y,
        'z' => Z,

        ' ' => NONE,
        '.' => ML | MR | BR,
        '!' => ML | MR | BL,
        ',' => ML,
        _ => NONE,
    };

    if result != NONE || c == ' ' {
        return Some(result);
    }

    return None;
}

pub fn match_2(string: &str) -> Option<u8> {
    let result = match string {
        "gg" => E | I,
        "ch" => TL | BR,
        "gh" => TL | ML | BR,
        "sh" => TL | TR | BR,
        "th" => TL | TR | MR | BR,
        "wh" => TL | MR | BR,
        "ed" => F | BR,
        "er" => G | BR,
        "ou" => H | BR,
        "ow" => I | BR,
        "en" => ML | BR,
        "ar" => TR | MR | BL,
        "in" => MR | BL,
        "of" => L | TR | MR,
        "ff" => ML | MR | BL,
        "st" => BL | TR,
        "ea" => ML,
        "bb" | "be" => ML | BL,
        
        "cc" => ML | MR,
        _ => NONE,
    } as u8;

    if result != NONE {
        return Some(result);
    }

    return None
}

pub fn match_3(string: &str) -> Vec<u8> {
    return match string {
        "the" => vec![TR | ML | BL | BR],
        "ing" => vec![TR | BL | BR],
        "and" => vec![L | TR | BR],
        "for" => vec![L | TR | MR | BR],
        "dis" => vec![ML | MR | BR],
        "con" => vec![ML | MR],

        "had" => vec![TR | MR | BR, H],
        "day" => vec![MR, D],
        "one" => vec![MR, O],

        "ong" => vec![MR | BR, G],
        "ful" => vec![MR | BR, F],
        "ity" => vec![MR | BR, Y],

        _ => vec![]
    };
}

pub fn match_4(string: &str) -> Vec<u8> {
    return match string {
        "with" => vec![I | MR | BL | BR],

        "were" => vec![ML | MR | BL | BR],
        "this" => vec![D | BR],

        "upon" => vec![TR | MR, U],
        "word" => vec![TR | MR, W],
        "many" => vec![TR | MR | BR, M],
        "ever" => vec![MR, E],
        "here" => vec![MR, H],
        "know" => vec![MR, K],
        "lord" => vec![MR, L],
        "name" => vec![MR, N],
        "part" => vec![MR, P],
        "some" => vec![MR, S],
        "time" => vec![MR, T],
        "work" => vec![MR, W],

        "ound" => vec![TR | BR, D],
        "ance" => vec![TR | BR, E],
        "sion" => vec![TR | BR, D | BL],
        "less" => vec![TR | BR, I | BL],
        "ount" => vec![TR | BR, T],
        "ence" => vec![MR | BR, E],
        "tion" => vec![MR | BR, N],
        "ness" => vec![MR | BR, S],
        "ment" => vec![MR | BR, T],

        _ => vec![],
    };
}

pub fn match_5(string: &str) -> Vec<u8> {
    return match string {

        "these" => vec![TR | MR, I | BL | BR],
        "those" => vec![TR | MR, E | TR | BR],
        "whose" => vec![TR | MR, E | BR],
        "their" => vec![TR | MR | BR, I | BL | BR],
        "world" => vec![TR | MR | BR, W],
        "right" => vec![MR, R],
        "under" => vec![MR, U],
        "young" => vec![MR, Y],
        "there" => vec![MR, I | BL | BR],
        "where" => vec![MR, E | BR],
        "ought" => vec![MR, H | BR],

        _ => vec![],
    };
}

pub fn match_6(string: &str) -> Vec<u8> {
    return match string {
        "cannot" => vec![TR | MR | BR, C],
        "spirit" => vec![TR | MR | BR, S],
        "father" => vec![MR, F],
        "mother" => vec![MR, M],

        _ => vec![],
    };
}

pub fn match_7(string: &str) -> Vec<u8> {
    return match string {

        "through" => vec![MR, I | TR | BR],

        _ => vec![],
    };
}

pub fn match_8(string: &str) -> Vec<u8> {
    return match string {
        "question" => vec![MR, Q],
        _ => vec![],
    };
}

pub fn match_9(string: &str) -> Vec<u8> {
    return match string {
        "character" => vec![MR | TL | BR],
        _ => vec![],
    };
}

pub fn match_wordsign(string: &str) -> Option<u8> {
    let result = match string {
        "do" => D,
        "go" => G,
        "so" => S,
        "us" => U,
        "it" => X,
        "as" => A,
        "but" => B,
        "can" => C,
        "not" => N,
        "you" => Y,
        "out" => H | BR,
        "his" => ML | BL | BR,
        "was" => MR | BL | BR,
        "from" => F,
        "have" => H,
        "just" => J,
        "like" => L,
        "more" => M,
        "that" => T,
        "very" => V,
        "will" => W,
        "every" => E,
        "quite" => Q,
        "still" => TR | BL,
        "child" => TL | BR,
        "shall" => C | BR,
        "which" => E | BR,

        "people" => P,
        "enough" => ML | BR,

        "reather" => R,

        "knowledge" => K,

        _ => NONE
    };

    if result != NONE {
        return Some(result);
    }

    return None;
}

pub fn match_wordsign_complexe(string: &str) -> Option<Vec<u8>> {

    let result = match string {
        
        "about" => vec![A, B],
        "above" => vec![A, B, V],
        "according" => vec![A, B],
        "accross" => vec![A, C, R],
        "after" => vec![A, F],
        "afternoon" => vec![A, F, N],
        "afterward" => vec![A, F, W],
        "again" => vec![A, G],
        "against" => vec![A, G, TR | BL],
        "also" => vec![A, L],
        "almost" => vec![A, L, M],
        "already" => vec![A, L, R],
        "altogather" => vec![A, L, T],
        "although" => vec![A, L, D | BR],
        "always" => vec![A, L, W],
        "blind" => vec![B, L],
        "braille" => vec![ B, R, L ],
        "could" => vec![ C, D ],
        "declare" => vec![ D, C, L ],
        "declaring" => vec![ D, C, L, G ],
        
        _ => vec![],
    };

    if !result.is_empty() {
        return Some(result);
    }

    return None;
}


fn min(a: usize, b: usize) -> usize {
    return if a < b { a } else { b };
}