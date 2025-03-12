#[derive(Debug)]
pub struct Ammunition {
    bullet_type: String,
    bullet_amount: u32,
}

impl Ammunition {
    pub fn new(bullet_type: &str, bullet_amount: u32) -> Self {
        Ammunition {
            bullet_type: bullet_type.to_string(),
            bullet_amount,
        }
    }

    pub fn bullet_type(&self) -> &str {
        &self.bullet_type
    }

    pub fn set_bullet_type(&mut self, bullet_type: &str) {
        self.bullet_type = bullet_type.to_string();
    }

    pub fn bullet_amount(&self) -> u32 {
        self.bullet_amount
    }

    pub fn set_bullet_amount(&mut self, bullet_amount: u32) {
        self.bullet_amount = bullet_amount;
    }
}