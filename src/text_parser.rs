/**
 * The aim of this module is to convert any text to any format we may wanna use in this game.
 * 
 * Current functions:
 * 
 * 1. text_to_code(text)
 *  Converts given text of the format: "1 2 3 4 5" to a code format similar to vec![1, 2, 3, 4, 5]
 */

pub fn text_to_code(text: String) -> Vec<isize> {
    let text_vec: Vec<&str> = text.split_whitespace().collect();

    let mut guess_vec: Vec<isize> = Vec::new();

    for j in text_vec{
        let code: isize = j.parse().unwrap();
        guess_vec.push(code);
    }

    return guess_vec;
}