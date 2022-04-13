use super::process::*;
use super::normalize::*;
use crate::utils::io::{overwrite_file, delete_file};

#[test]
fn test_text_process() {
    let input: &str = "This is a test of the TEXT-PROCESSING MODULE text-processing test test of of of of tHIS";

    if let Some(map) = text_process(input) {
        for (term, freq) in map {
            println!("{}, {}", term, freq);
        }
    } else {
        println!("Not ASCII input");
    }
}

#[test]
fn test_text_process_file() {
    
    // create a file
    let path = String::from("./test.txt");
    let contents = "Hello,,,,,,, I am-the-greatest-ever the the a a run bike swim RUN BIKE SWIM. wow,punctuation,is,crazy!!";
    overwrite_file(&path, contents.as_bytes()).expect("Could not write file");

    if let Some(map) = text_process_file(&path) {
        for (term, freq) in map {
            println!("{}, {}", term, freq);
        }
    } else {
        println!("Not ASCII input");
    }

    delete_file(&path).expect("Could not delete file");
}

#[test]
fn test_replace_punctuation_whitespace() {
    let input = "Hello, my name-is the beast_money_37!!!";
    println!("{}", replace_punctuation_whitespace(input))
}

#[test]
fn test_is_stop_word() {
    assert_eq!(is_stop_word("i"), true);
    assert_eq!(is_stop_word("they"), true);
    assert_eq!(is_stop_word("doing"), true);
    assert_eq!(is_stop_word("r"), true);
}