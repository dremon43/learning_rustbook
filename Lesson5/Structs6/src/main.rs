// Листинг 5-7: Использование синтаксиса обновления структуры для установки нового email значения для User экземпляра, 
// но использование остальных значений из user1

fn main() {
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    let user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };
}

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}