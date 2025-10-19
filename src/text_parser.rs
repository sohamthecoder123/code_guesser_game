/**
 * The aim of this module is to convert any text to any format we may wanna use in this game.
 * 
 * Current functions:
 * 
 * 1. text_to_code(text)
 *  Converts given text of the format: "1 2 3 4 5" to a code format similar to vec![1, 2, 3, 4, 5]
 */


 pub fn text_to_int(text: &String) -> isize {
    let num: isize = match text.parse::<isize>() {
        Ok(n) => n,
        Err(e) => {
            println!("Error: {}", e);
            return -1;
        }
    };
    return num;
}

pub fn text_to_u_int(text: &String) -> usize {
    let num: usize = match text.parse::<usize>() {
        Ok(n) => n,
        Err(e) => {
            println!("Error: {}", e);
            return 0;
        }
    };
    //let num: usize = text.parse().expect("Error");
    return num;
}


pub fn text_to_code(text: &String) -> Vec<isize> {
    let text_vec: Vec<&str> = text.split_whitespace().collect();

    let mut guess_vec: Vec<isize> = Vec::new();

    for j in text_vec{
        let code: isize = text_to_int(&j.to_string());
        guess_vec.push(code);
    }

    return guess_vec;
}