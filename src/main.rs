use std::vec;

mod code_guess;

fn main() {
    let code: Vec<usize> = vec![1, 2, 3, 4, 5];
    let guess:Vec<usize> = vec![5, 3, 6, 1, 10];    
    let x: Vec<String> = code_guess::guess_code(code, guess);

    println!("{:?}", x);
}