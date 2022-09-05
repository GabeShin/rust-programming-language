#[derive(Debug)]
struct User {
    name: String,
    email: String,
    active: bool,
    sign_in_count: u64,
}

impl User {
    fn create_admin(name: String) -> Self {
        Self {
            name: name,
            email: String::from("admin@visual.camp"),
            active: true,
            sign_in_count: 1,
        }
    }

    fn add_sign_in_count(&mut self) {
        self.sign_in_count = self.sign_in_count + 1;
    }
}

struct Color(i32, i32, i32);

fn main() {
    let user = User {
        name: String::from("Gabe"),
        email: String::from("gabe@visual.camp"),
        active: true,
        sign_in_count: 1,
    };

    println!("{:?}", user);

    let user_2 = build_user(String::from("Paul"), String::from("paul@whatever.com"));

    let mut user = User {
        sign_in_count: dbg!(user.sign_in_count + 1),
        ..user
    };

    println!("{:?}", user);

    user.add_sign_in_count();

    println!("{:?}", user);

    let admin = User::create_admin(String::from("admin"));

    println!("{:?}", admin);

    let black = Color(0, 0, 0);
}

fn build_user(name: String, email: String) {
    let user = User {
        name,
        email,
        active: true,
        sign_in_count: 1,
    };
}
