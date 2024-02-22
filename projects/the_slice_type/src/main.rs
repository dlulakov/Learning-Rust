fn main() {
    let mut s = String::from("hello world");
   // println!("{}",s.as_bytes());
   let word = first_world(&s); 
   s.clear();
}


fn first_world(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' '{
            return &s[..i];
        }
    }

    &s[..]
}

