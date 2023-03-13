fn p_5_1_structs() {
    let mut user1 = User {
        active: true,
        username: String::from("shin"),
        email: String::from("shin@email.com"),
        sign_in_count: 1,
    };
    user1.email = String::from("seokjin@email.net");

    // normal
    let user2 = User {
        active: user1.active,
        username: user1.username,
        email: String::from("aaaabbbbb@ccc.ddd"),
        sign_in_count: user1.sign_in_count,
    };

    // struct
    let user3 = User {
        email: String::from("ccc@ccc.ccc"),
        ..user2
    };


    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);


    let subject = AlwaysEqual;
}

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username: username,
        email: email,
        sign_in_count: 1,
    }
}

fn build_user_2(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}


struct Color(i32, i32, i32);
struct Point(i32, i32, i32);


struct AlwaysEqual;