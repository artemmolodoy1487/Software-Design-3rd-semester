#[derive(Debug)]
pub struct Trigger {
    is_active: bool,
}

impl Trigger {
    pub fn new() -> Self {
        Trigger {
            is_active: true,
        }
    }

    pub fn change_activity(&mut self) {
        self.is_active = !self.is_active; 
        println!("Trigger is now {:?}", self.is_active); 
    }
}