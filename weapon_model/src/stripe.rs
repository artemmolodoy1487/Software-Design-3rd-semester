use crate::ammunition::Ammunition;

#[derive(Debug)]
pub struct Stripe {
    bullet_type: String,
    max_amount: u32,
    amount: u32,
}

impl Stripe {
    pub fn new(bullet_type: &str, max_amount: u32, amount: u32) -> Self {
        println!(
            "Создание новой ленты: тип пули - {}, максимальное количество - {}, текущее количество - {}",
            bullet_type, max_amount, amount
        );
        Stripe {
            bullet_type: bullet_type.to_string(),
            max_amount,
            amount,
        }
    }

    pub fn bullet_type(&self) -> &str {
        println!("Получение типа пули: {}", self.bullet_type);
        &self.bullet_type
    }

    pub fn max_amount(&self) -> u32 {
        println!("Получение максимального количества: {}", self.max_amount);
        self.max_amount
    }

    pub fn amount(&self) -> u32 {
        self.amount
    }

    pub fn set_amount(&mut self, amount: &u32) {
        println!("Количество патронов: {}", amount);
        self.amount = *amount;
    }

    pub fn load_from_amo(&mut self, ammo: &mut Ammunition) {
        if self.bullet_type() != ammo.bullet_type() {
            println!(
                "Типы пуль не совпадают: {} != {}",
                self.bullet_type(),
                ammo.bullet_type()
            );
            return;
        }

        let needed_amount = self.max_amount() - self.amount();
        if ammo.bullet_amount() >= needed_amount {
            ammo.set_bullet_amount(ammo.bullet_amount() - needed_amount);
            self.set_amount(&self.max_amount());
            println!(
                "Загрузка пуль: {} пуль загружено, осталось в магазине: {}",
                needed_amount,
                ammo.bullet_amount()
            );
        } else {
            println!(
                "Недостаточно пуль для загрузки. Необходимо: {}, доступно: {}",
                needed_amount,
                ammo.bullet_amount()
            );
        }
    }

    pub fn use_bullet(&mut self) -> bool {
        if self.amount() > 0 {
            self.set_amount(&(self.amount() - 1));
            println!("Использована пуля. Осталось: {}", self.amount);
            return true;
        }
        println!("Нет доступных пуль для использования.");
        return false;
    }
}
