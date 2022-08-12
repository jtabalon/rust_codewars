fn make_upper_case(s: &str) -> String {
    // code here
    
    s.to_ascii_uppercase()

    
}   

fn main() {
    let test_string: &str = "hello world";
    println!("{}", make_upper_case(test_string));
}