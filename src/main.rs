mod code_guess;
mod random_code_generator;
mod user_input;
mod text_parser;
mod enemy;
mod player;

fn main() {

    let player1: player::Player = player::Player{ health: 10, attack: 2, defence: 10 };

    let mut enemy1: enemy::Enemy = enemy::Enemy{ id: 0, cat: "default".to_string(), health: 10, size: 5, lower: 1, upper: 5, code: vec!{0,}, damage: 1};
    enemy1.generate_code();
    
    while enemy1.health > 0 {
        let result: Vec<String> = player1.attack_enemy(&mut enemy1); 

        println!("Result: {:?}", result);
        println!("Enemy health: {}", &enemy1.health);
    }

    println!("Congratulations! You have defeated the Enemy!");
    
}