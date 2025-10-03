use crate::enemy::Enemy;

pub struct Player {
    pub health: isize,
    pub attack: isize,
    pub defence: isize,
}

impl Player {
    pub fn add_health(&mut self, amount: isize){
        self.health += amount;
    }

    pub fn take_damage(&mut self, damage: isize){
        self.health -= (damage) * (1 - self.defence/10);
        
        if self.defence >= damage{
            self.defence -= damage;
        }

        else {
            self.defence = 0;
        }
    }

    pub fn attack_enemy(&self, en: &mut crate::enemy::Enemy) -> Vec<String>{
        let result: Vec<String> = en.fight();
        return result;
    }
}