fn main() {
    enum SpreedSheetCell {
        Int(i32),
        Float(f64),
        Text(String), 
    }
    let row = vec![
        SpreedSheetCell::Int(3),
        SpreedSheetCell::Text(String::from("blue")),
        SpreedSheetCell::Float(10.13),
    ];
}
