use crate::enemy;
use crate::player;
use crate::fight_loop;

pub fn room(pl: &mut player::Player, enemies: &mut Vec<enemy::Enemy>){
    while enemies.len() > 0 && pl.health > 0 {
        enemies.retain(|enemy: &enemy::Enemy| enemy.isAlive);
        println!("{:?}", enemies.len());
        fight_loop::fight_loop(pl, enemies);
    }

    if enemies.len() > 0 {
        println!("Room Cleared!")
    }

    if pl.health > 0 {
        println!("You Died!");
    }
}