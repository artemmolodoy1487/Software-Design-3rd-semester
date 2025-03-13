#[derive(Debug)]
pub struct Ammunition {
    bullet_type: String,
    bullet_amount: u32,
}

impl Ammunition {
    pub fn new(bullet_type: &str, bullet_amount: u32) -> Self {
        println!(
            "Создание новой амуниции: тип - {}, количество - {}",
            bullet_type, bullet_amount
        );
        Ammunition {
            bullet_type: bullet_type.to_string(),
            bullet_amount,
        }
    }

    pub fn bullet_type(&self) -> &str {
        println!("Получение типа пули: {}", self.bullet_type);
        &self.bullet_type
    }

    pub fn set_bullet_type(&mut self, bullet_type: &str) {
        println!("Установка нового типа пули: {}", bullet_type);
        self.bullet_type = bullet_type.to_string();
    }

    pub fn bullet_amount(&self) -> u32 {
        println!("Получение количества пуль: {}", self.bullet_amount);
        self.bullet_amount
    }

    pub fn set_bullet_amount(&mut self, bullet_amount: u32) {
        println!("Установка нового количества пуль: {}", bullet_amount);
        self.bullet_amount = bullet_amount;
    }
}
