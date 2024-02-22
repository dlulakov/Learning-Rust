use std::io;

fn fibonacci (x:u32) -> u32 {
    // 0 1 1 2 3 5 8 12 20
    if x == 1{
        return 0;
    }else if x == 2{
        return 1 ;
    }
    let mut first = 0;
    let mut second = 1;
    for _number in 0..x-2{
        let temp = second;
        second = first + second;
        first = temp;
    }
    return second;
}

fn main() {
    println!("Input the what number in the Fibbonacci you want");
    let mut temp  = String::new();
    io::stdin()
        .read_line(&mut temp)
        .expect("Failed to read line!");
    let temp: u32 = temp.trim().parse().expect("Please type a number");
    println!("The n-th Fibbonacci number is: {}",fibonacci(temp));
}
