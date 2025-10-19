use crate::enemy;

pub struct Player {
    pub health: isize,
    //pub attack: isize,
    pub defence: isize,
    pub maxHealth: isize,
}

impl Player {
    pub fn add_health(&mut self, amount: isize){
        self.health += amount;
        if self.health > self.maxHealth {
            self.health = self.maxHealth;
        }
    }

    pub fn take_damage(&mut self, damage: isize){
        if self.defence >= damage{
            self.defence -= damage;
        }

        else {
            self.defence = 0;
        }

        if self.defence == 0 {
            self.health -= damage;
        }
    }

    pub fn attack_enemy(&self, en: &mut enemy::Enemy) -> Vec<String>{
        let result: Vec<String> = en.fight();
        return result;
    }
}