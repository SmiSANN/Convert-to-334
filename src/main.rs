fn convert_to_334(n: i32) -> String {
    let mut dp_table = vec![vec![0; (n+1) as usize]];
}


fn main() {
    println!("Converting natural number to 334 equation");
    let result = convert_to_334(10);
    println!("Result: {}", result);
}
