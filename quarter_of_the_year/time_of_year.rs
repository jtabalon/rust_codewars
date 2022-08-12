fn quarter_of(month: u8) -> u8 {
    let result: u8 = match month {
        1 => 1, 
        2 => 1, 
        3 => 1, 
        4 => 2, 
        5 => 2, 
        6 => 2, 
        7 => 3, 
        8 => 3, 
        9 => 3, 
        10 => 4, 
        11 => 4, 
        12 => 4, 
        _ => 0
    };
    result
}


fn main() {
    let test: u8 = quarter_of(6);
    println!("{}", test);
}