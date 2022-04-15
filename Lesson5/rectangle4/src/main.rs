// Листинг 5-12. Добавление атрибута для получения Debug типажа и печать Rectangle экземпляра с использованием форматирования отладки.
#[derive(Debug)]

struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("rect1 is {:?}", rect1);
}