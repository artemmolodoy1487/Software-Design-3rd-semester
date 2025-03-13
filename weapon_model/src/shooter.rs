use crate::ammunition::Ammunition;
use crate::weapon::Weapon;

pub struct Shooter {
    name: String,
    weapon: Weapon,
    ammo: Ammunition,
    aiming: bool, // Поле для состояния целится или нет
}

impl Shooter {
    pub fn new(name: &str, weapon: Weapon, ammo: Ammunition) -> Self {
        Shooter {
            name: name.to_string(),
            weapon,
            ammo,
            aiming: false, // Начальное состояние — не целится
        }
    }

    pub fn aim(&mut self) {
        self.aiming = true;
        println!("{} нацелился.", self.name);
    }

    pub fn stop_aiming(&mut self) {
        self.aiming = false;
        println!("{} больше не целится.", self.name);
    }

    pub fn shoot(&mut self) {
        if self.aiming {
            if self.weapon.fire() {
                println!("{} выстрелил!", self.name);
            } else {
                println!("{} не может выстрелить, патроны закончились!", self.name);
            }
        } else {
            println!("{} не может стрелять, так как не целится.", self.name);
        }
    }

    pub fn reload(&mut self) {
        // Предполагается, что у Weapon есть метод для перезарядки
        self.weapon.reload(&mut self.ammo);
        println!("{} перезарядил оружие.", self.name);
    }

    pub fn unload(&mut self) {
        // Предполагается, что у Weapon есть метод для разрядки
        self.weapon.unload();
        println!("{} разрядил оружие.", self.name);
    }
}