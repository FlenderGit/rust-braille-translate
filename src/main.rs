use std::time::{SystemTime, UNIX_EPOCH};

pub mod lib;

fn main() {
    log("Starting program");
    //display_characters();
    test_compare_string_and_word();

    log("Ending program");
}

/*
fn test_decode_lowercase() {

    let string: &str = "everyone must learn braille ! it is a great knowledge to have for someone who is blind.";
    //let result: Vec<u8> = braille_utils::word_to_braille(string);
    let result = braille_utils::string_to_braille(string);

    // Log "String to Braille" + string
    log(&format!("Converting : {}", string));

    // Join the result vector into a string and log it using the get_braille function
    let braille_string: String = result.iter().map(|&x| get_braille(x)).collect();
    log(&format!("Result     : {}", braille_string));
}
*/

fn test_compare_string_and_word() {
    let string: &str = "everyone must learn braille ! it is a great knowledge to have for someone who is blind. however, even if it is much easier to learn braille when you are young, it is never too late to learn it. braille is a great way to read and write for people who are blind. it is a great way to communicate with others and to learn new things. thanks to braille, people who are blind can read books, write letters, and even use a computer...";
    let start1 = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    lib::string_to_braille(string);
    let end1 = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    
    let start2 = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    lib::word_to_braille(string);
    let end2 = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();

    log(&format!("String with {} characters", string.len()));
    log(&format!("String to Braille : {:?}", end1 - start1));
    log(&format!("Word to Braille   : {:?}", end2 - start2));
}

/*
fn display_characters() {
    for i in 0..128 {
        println!("{}: {}", i, std::char::from_u32(i).unwrap());
    }
}
*/

fn log(s: &str) {
    println!("[BRAILLE] {}", s);
}