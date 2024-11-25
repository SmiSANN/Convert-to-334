use std::collections::HashMap;

mod preparation;

fn convert_to_334(n: i32) -> String {
    // 使用する数字
    let conversion_criteria_number = "334";
    let conversion_criteria_number_len = conversion_criteria_number.len();

    let mut conversion_criteria_number_tmp_array: Vec<&str> = Vec::new();
    for i in (0..conversion_criteria_number_len) {
        for j in 0..conversion_criteria_number.len()-i {
            conversion_criteria_number_tmp_array.push(&conversion_criteria_number[j..j+i+1]);
        }
    }
    // 操作を不可能にするため、使う数字の配列をコピー
    let conversion_criteria_number_array: Vec<i32> = conversion_criteria_number_tmp_array.iter().map(|&x| x.parse::<i32>().unwrap()).collect();
    println!("Conversion Criteria Number Array: {:?}", conversion_criteria_number_array);

    // 与えられた数字の組み合わせで全ての計算式を計算
    let mut result1: HashMap<i32, Vec<String>> = HashMap::new();

    // println!("Result1: {:?}", result1);


    "waaa".to_string()
}


fn main() {
    println!("Converting natural number to 334 equation");
    let nums = vec![3,3,4];
    // let result = convert_to_334(10);
    let result = preparation::create_all_combinations(&nums);
    // println!("Result: {:?}", result);
    let mut sorted_result: Vec<_> = result.iter().collect();
    sorted_result.sort();
    println!("Sorted Result: {:?}", sorted_result);
    // println!("result: {:?}", create_combination(vec![334,33,34,3,3,4], 4));
}
