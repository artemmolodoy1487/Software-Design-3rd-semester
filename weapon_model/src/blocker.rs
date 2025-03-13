use crate::triger::Triger;

#[derive(Debug)]
pub struct Blocker {
    is_unblocked: bool,
}

impl Blocker {
    pub fn new() -> Self {
        Blocker { is_unblocked: true }
    }

    pub fn press(&mut self, triger: &mut Triger) {
        self.is_unblocked = !self.is_unblocked;
        triger.change_activity();
    }
}
