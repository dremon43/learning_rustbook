// Создание экземпляра user1 из User структуры.

fn main() {
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
}

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}