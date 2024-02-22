


struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_content: u64,


    
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

struct AlwaysEqual;


fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_content: 1,

    }



}


fn main() {
    println!("Hello, world!");
    let mut user1 = User{
        active: true,
        username: String::from("d.lulakov"),
        email: String::from("d.lulakov@gmail.com"),
        sign_in_content: 1,
    };
    let user2 = User{
        email: String::from("user2@gmail.com"),
        ..user1
        
    };

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    let subject = AlwaysEqual; 
    user1.email = String::from("anotheremail@gmail.com");
}
