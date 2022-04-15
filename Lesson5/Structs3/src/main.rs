// Листинг 5-4. build_user Функция, которая принимает адрес электронной почты и имя пользователя и возвращает User экземпляр.

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
        email: email,
        username: username,
        active: true,
        sign_in_count: 1,
    }
}