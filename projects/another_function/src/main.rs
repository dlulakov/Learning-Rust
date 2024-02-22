fn print_labeled_mesurment(value: i32, unit_label: char){
    println!("The mesurment is : {value}{unit_label}");
}
fn five() -> i32{
    5
}
fn plus_five(x: i32) -> i32 {
   x + 1 
}
fn main() {
    println!("Hello, world!");
    print_labeled_mesurment(5, 'h');
    let y = {

        let x = 3;
        x + 1
    };
    //let x = five();
    let x = plus_five(5);

//    println!("The value of y is: {y}");
    println!("The value of x is: {x}");
}
