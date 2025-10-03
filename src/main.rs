mod code_guess;
mod random_code_generator;
mod user_input;
mod text_parser;
mod enemy;

fn guess_code(code: &Vec<isize>) -> Vec<String>{
    let input: String = user_input::get_user_input_trimmed("");
    let guess: Vec<isize> = text_parser::text_to_code(input);    
    
    let x: Vec<String> = code_guess::guess_code_check(&code, &guess);
    return x;
}

fn main() {

    let mut enemy1: enemy::Enemy = enemy::Enemy{ id: 0, cat: "default".to_string(), health: 10, size: 5, lower: 1, upper: 5, code: vec!{0,} };

    enemy1.generate_code();
    
    while enemy1.health > 0{
        let code: &Vec<isize> =  &enemy1.code;   

        let result: Vec<String> = guess_code(code);

        for i in &result{
            if *i == "in_place".to_string(){
                enemy1.change_health(-2);
            }
            else if *i == "out_of_place".to_string(){
                enemy1.change_health(-1);
            }
            else {
                continue;
            }
        }

        println!("Result: {:?}", result);
        println!("Enemy health: {}", &enemy1.health);
    }

    println!("Congratulations! You have defeated the Enemy!");
    
}