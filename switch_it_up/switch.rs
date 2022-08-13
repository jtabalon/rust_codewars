fn switch_it_up(n: usize) -> &'static str {
    let result: &'static str = match n {
        0 => "Zero",
        1 => "One",
        2 => "Two",
        3 => "Three",
        4 => "Four",
        5 => "Five",
        6 => "Six",
        7 => "Seven",
        8 => "Eight",
        9 => "Nine",
        _ => "Invalid",
    };
    result
}

fn main() {
    let three: &'static str = switch_it_up(0);
    println!("{}", three);
}
