mod ammunition;
mod stripe;
mod weapon;
mod barrel;
mod scope;
mod triger;
mod blocker;
mod shooter;

use crate::scope::Scope;
use crate::stripe::Stripe;
use crate::triger::Triger;
use crate::barrel::Barrel;
use crate::ammunition::Ammunition;
use crate::weapon::Weapon;
use crate::shooter::Shooter; // Предполагается, что Shooter находится в модуле shooter

fn main() {
    // Создаем экземпляры оружия и патронов
    let weapon = Weapon::new(
        "Rifle".to_string(),
        Stripe::new("9mm", 30, 30),
        Barrel::new(),
        Scope::new(1),
        Triger::new(),
    ); // Убедитесь, что все параметры корректны
    let mut ammo = Ammunition::new("7.62mm", 30); // Пример создания патронов

    // Создаем стрелка
    let mut shooter = Shooter::new("Стрелок", weapon, ammo);

    loop {
        println!("\nВыберите действие:");
        println!("1. Нацелиться");
        println!("2. Прекратить прицеливание");
        println!("3. Выстрелить");
        println!("4. Перезарядить");
        println!("5. Разрядить");
        println!("6. Выйти");

        let mut input = String::new();
        std::io::stdin().read_line(&mut input).expect("Не удалось прочитать строку");
        let choice = input.trim();

        match choice {
            "1" => shooter.aim(),
            "2" => shooter.stop_aiming(),
            "3" => shooter.shoot(),
            "4" => shooter.reload(),
            "5" => shooter.unload(),
            "6" => {
                println!("Выход из программы.");
                break;
            },
            _ => println!("Неверный выбор, попробуйте снова."),
        }
    }
}