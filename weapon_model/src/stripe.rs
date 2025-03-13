use crate::ammunition::Ammunition;

pub struct Stripe{
    bullet_type: String,
    max_amount: u32,
    amount: u32,
}

impl Stripe{
    pub fn new(bullet_type: &str, max_amount: u32, amount: u32) -> Self {
        Stripe {
            bullet_type: bullet_type.to_string(),
            max_amount,
            amount,
        }
    }

    pub fn bullet_type(&self) -> &str {
        &self.bullet_type
    }

    pub fn max_amount(&self) -> u32 {
        self.max_amount
    }

    pub fn amount(&self) -> u32 {
        self.amount
    }
    
    pub fn set_amount(&mut self, amount: &u32){
        self.amount = *amount
    }


    pub fn load_from_amo(&mut self, ammo: &mut Ammunition) {
        if self.bullet_type() != ammo.bullet_type() {
            return
        }

        if ammo.bullet_amount() >= (self.max_amount() -self.amount()) {
            ammo.set_bullet_amount(ammo.bullet_amount() - self.max_amount());
            self.set_amount(&self.max_amount());
        }
    }

    pub fn use_bullet(&mut self) {
        self.set_amount(&(self.amount()-1));
    }
}