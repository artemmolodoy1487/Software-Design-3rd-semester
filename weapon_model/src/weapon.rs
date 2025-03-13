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

    
}
