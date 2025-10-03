use crate::random_code_generator;

pub struct Enemy {
    pub id: isize,
    pub cat: String,
    pub health: isize,
    pub size: usize,
    pub lower: isize,
    pub upper: isize,
    pub code: Vec<isize>,
}

impl Enemy {
    pub fn generate_code(&mut self){
        self.code = random_code_generator::generate_random_code(self.size, self.lower, self.upper);
    }

    pub fn change_health(&mut self, amount: isize){
        self.health += amount;
    }
}

