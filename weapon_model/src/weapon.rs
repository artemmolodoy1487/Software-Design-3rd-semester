use crate::stripe::Stripe;
use crate::barrel::Barrel;
use crate::scope::Scope;

#[derive(Debug)]
struct Weapon {
    stripe: Stripe,
    barrel: Barrel,
    scope: Scope,
    name: String,
}

impl Weapon {
    
    pub fn new(name: String, stripe: Stripe, barrel: Barrel, scope: Scope) -> Self {
        Weapon { name, stripe, barrel, scope }
    }

    pub fn fire(&mut self){
        if self.stripe.amount() == 0 {
            return
        }
        
    }
}
