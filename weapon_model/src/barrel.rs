#[derive(Debug)]
pub struct Barrel {
    is_clean: bool,
}

impl Barrel {
    pub fn new() -> Self {
        println!("Создание нового ствола: чистый");
        Barrel {
            is_clean: true,
        }
    }

    pub fn is_clean(&self) -> &bool {
        println!("Проверка состояния ствола: {}", self.is_clean);
        &self.is_clean
    }

    pub fn set_dirty(&mut self) {
        println!("Ствол стал грязным");
        self.is_clean = false;
    }

    pub fn work(&mut self) {
        if !*self.is_clean() {
            println!("Ствол грязный, работа не будет выполнена");
            return;
        }
        println!("Работа со стволом выполнена, он стал грязным");
        self.set_dirty();
    }
}