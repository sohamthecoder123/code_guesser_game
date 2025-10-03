use crate::random_code_generator;
use crate::player;
use crate::user_input;
use crate::text_parser;
use crate::code_guess;

fn guess_code(code: &Vec<isize>) -> Vec<String>{
    let input: String = user_input::get_user_input_trimmed("");
    let guess: Vec<isize> = text_parser::text_to_code(input);    
    
    let x: Vec<String> = code_guess::guess_code_check(&code, &guess);
    return x;
}

pub struct Enemy {
    pub id: isize,
    pub cat: String,
    pub health: isize,
    pub size: usize,
    pub lower: isize,
    pub upper: isize,
    pub code: Vec<isize>,
    pub damage: isize,
}

impl Enemy {
    pub fn generate_code(&mut self){
        self.code = random_code_generator::generate_random_code(self.size, self.lower, self.upper);
    }

    pub fn change_health(&mut self, amount: isize){
        self.health += amount;
    }

    pub fn attack_player(&mut self, mut pl: player::Player){
        pl.take_damage(self.damage);
    }

    pub fn fight(&mut self) -> Vec<String>{
        let result: Vec<String> = guess_code(&self.code);

        for i in &result{
            if *i == "in_place".to_string(){
                self.change_health(-2);
            }
            else if *i == "out_of_place".to_string(){
                self.change_health(-1);
            }
            else {
                continue;
            }
        }

        return result;
    }

}

