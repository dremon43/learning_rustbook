// Листинг 5-5. build_user Функция, использующая сокращение для инициализации поля, 
// поскольку параметры email и username имеют то же имя, что и поля структуры.

fn main() {
	let my_mail = String::from("dremon43@gmail.com");
	let my_name = String::from("Lavga");
	build_user(my_mail, my_name);
}

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}