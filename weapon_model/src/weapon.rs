use crate::barrel::Barrel;
use crate::scope::Scope;
use crate::stripe::Stripe;
use crate::triger::Triger;

#[derive(Debug)]
pub struct Weapon {
    stripe: Stripe,
    barrel: Barrel,
    scope: Scope,
    triger: Triger,
    name: String,
}

impl Weapon {
    pub fn new(name: String, stripe: Stripe, barrel: Barrel, scope: Scope, triger: Triger) -> Self {
        Weapon {
            name,
            stripe,
            barrel,
            scope,
            triger,
        }
    }

    pub fn fire(&mut self) -> bool {
        if !self.triger.is_active() {
            return false;
        }
        if !self.stripe.use_bullet() {
            return false;
        }

        return true;
    }
}
