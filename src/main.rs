mod code_guess;
mod random_code_generator;
mod user_input;
mod text_parser;

fn main() {
    let code:Vec<isize> =  random_code_generator::generate_random_code(5, 1, 5);
    //let guess:Vec<isize> = vec![1, 3, 2, 4, 5];    
    
    let input: String = user_input::get_user_input_trimmed("");
    let guess: Vec<isize> = text_parser::text_to_code(input);    
    
    let x: Vec<String> = code_guess::guess_code(&code, &guess);

    println!("{:?}", code);
    println!("{:?}", x);
}