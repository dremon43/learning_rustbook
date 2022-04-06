//Преобразование температуры из градусов по Цельсию в градусы по Фаренгейту.

use std::io;

fn main() {
    println!("Введите температуру в градусах по Цельсию");
	
	let mut temperature_c = String::new();

    io::stdin()
        .read_line(&mut temperature_c)
        .expect("Failed to read line");

	let temperature_c: f64 = temperature_c.trim().parse().expect("Please type a number!");

	let temperature_f = temperature_c * 1.8 + 32.0;
	println!("Температура по Фаренгейту: {}", temperature_f);

}