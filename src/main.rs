mod preparation;

fn convert_to_334(n: i32) -> String {
    // 使用する数字
    let using_number = 334;
    // 上記の値を最大桁数-1桁までにそれぞれの桁数ごとに分解し、配列へ
    let mut using_number_tmp_array: Vec<i32> = using_number.to_string().chars().map(|c| c.to_digit(10).unwrap() as i32).collect();

    for i in 0..(using_number_tmp_array.len()-1) {
        using_number_tmp_array.push(using_number_tmp_array[i]*10+using_number_tmp_array[i+1]);
    }

    using_number_tmp_array.push(using_number);

    // 使う数字の配列を昇順にソート
    using_number_tmp_array.sort();

    // 操作を不可能にするため、使う数字の配列をコピー
    let using_number_array = using_number_tmp_array;

    println!("Using number: {:?}", using_number_array);  // TODO: 消す



    "waaa".to_string()
}


fn main() {
    let result1 = preparation::calculate_all_expressions(&[1, 1, 4, 5, 1, 4]);
    println!("Result1: {:?}", result1);
    println!("Converting natural number to 334 equation");
    let result = convert_to_334(10);
    println!("Result: {}", result);
}
