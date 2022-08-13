fn litres(time: f64) -> i32 {
    (time / 2.0) as i32
}

fn main() {
    let result: i32 = litres(6.2);
    println!("{}", result);
}