fn main() {
    let mut s = String::from("lo");
    let mut s1 = String::from("foo");
    let s2 = "bar"; 
    s1.push_str(s2);
    s.push('l');

    let s5 = String::from("Hello, ");
    let s3 = String::from("World!");

    let s4 = s5 + &s3;

    let tic = String::from("tic");
    let tac = String::from("tac");
    let toc = String::from("toc");


    let x = format!("{tic}-{tac}-{toc}"); 



    println!("x is : {x}");




    println!("s2 is : {s2}");
    println!("s4 is : {s4}");
    println!("s is : {s}");
}
