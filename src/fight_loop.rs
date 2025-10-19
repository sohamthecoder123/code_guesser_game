use crate::player;
use crate::enemy;
use crate::user_input;
use crate::text_parser;


pub fn fight_loop(pl: &mut player::Player, enemies: &mut Vec<enemy::Enemy>){
    println!("Enemies face you!");
    for en in enemies.iter(){
        println!("{}", en.cat);
    }
    print!("\n");

    //let mut en: enemy::Enemy = enemy::Enemy{ id: 0, cat: "default".to_string(), health: 10, size: 5, lower: 1, upper: 5, code: vec!{0,}, damage: 1, isAlive: true};
    let mut result: Vec<String> = Vec::new();

    loop{
        println!("Choose enemy to fight: ");
        let input: String = user_input::get_user_input_trimmed("");

        let signed_choice: isize = text_parser::text_to_int(&input) - 1;

        if signed_choice < 0 {
            println!("Invalid Choice");
            continue;
        }

        let choice: usize = text_parser::text_to_u_int(&input) - 1;

        println!("{:?}", choice);
        
        //println!("{:?}", choice);
        if let Some(en) = enemies.get_mut(choice){
            result = pl.attack_enemy(en);
            break;
        }

        else {
            println!("Invalid Choice!");
        }
 
    }

    enemies.retain_mut(|en: &mut enemy::Enemy|{
        en.attack_player(pl);
        en.isAlive
    });

    println!("Result: {:?}", result);
    println!("Your health: {}", &pl.health);
    println!("Your defence: {}", &pl.defence);
}
