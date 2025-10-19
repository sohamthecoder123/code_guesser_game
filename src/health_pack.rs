use crate::player;

pub struct HealthPack {
    pub amount: isize,
    pub canBeUsed: bool,
}

impl HealthPack {
    pub fn use_health_pack(&mut self, pl: &mut player::Player){
        if self.canBeUsed{
            pl.add_health(self.amount);
            self.canBeUsed = false;
        }
    }
}