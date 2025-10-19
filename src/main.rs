mod code_guess;
mod random_code_generator;
mod user_input;
mod text_parser;
mod enemy;
mod player;
mod fight_loop;
mod health_pack;
mod room;

fn main() {
    let mut player1: player::Player = player::Player{ health: 8, defence: 10, maxHealth: 10 };

    let mut enemy1: enemy::Enemy = enemy::Enemy{ name: "enemy1".to_string(), cat: "default".to_string(), health: 10, size: 5, lower: 1, upper: 5, code: vec!{0,}, damage: 0, isAlive: true};
    enemy1.generate_code();

    let mut enemy2: enemy::Enemy = enemy::Enemy{ name: "enemy2".to_string(), cat: "default".to_string(), health: 10, size: 5, lower: 1, upper: 5, code: vec!{0,}, damage: 0, isAlive: true};
    enemy2.generate_code();

    let mut enemy3: enemy::Enemy = enemy::Enemy{ name: "enemy3".to_string(), cat: "default".to_string(), health: 10, size: 5, lower: 1, upper: 5, code: vec!{0,}, damage: 0, isAlive: true};
    enemy3.generate_code();

    let mut hp1: health_pack::HealthPack = health_pack::HealthPack{ amount: 1, canBeUsed: true };

    let mut enemies: Vec<enemy::Enemy> = vec![enemy1, enemy2, enemy3];

    //fight_loop::fight_loop(&mut player1, &mut enemies);

    room::room(&mut player1, &mut enemies);

    //hp1.use_health_pack(&mut player1);

    //println!("{:?}", player1.health);
    /*
    while enemy1.isAlive && player1.health > 0 {
        enemy1.attack_player(&mut player1);
        let result: Vec<String> = player1.attack_enemy(&mut enemy1); 

        println!("Result: {:?}", result);
        println!("Your health: {}", &player1.health);
        println!("Your defence: {}", &player1.defence);
    }

    if player1.health == 0 {
        println! ("You Lost!");
    }
    else if !enemy1.isAlive {
        println!("You Won!")
    } 
    */   
}