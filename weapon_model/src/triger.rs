#[derive(Debug)]
pub struct Triger {
    is_active: bool,
}

impl Triger {
    pub fn new() -> Self {
        Triger { is_active: true }
    }

    pub fn change_activity(&mut self) {
        self.is_active = !self.is_active;
        println!("Trigger is now {:?}", self.is_active);
    }

    pub fn is_active(&self) -> bool {
        self.is_active
    }
}
