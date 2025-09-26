use std::{collections::HashMap, env};

mod prime_factorization;
mod preparation;

fn convert_to_334(n: i64, exp: i64) -> String {
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
    let conversion_criteria_number_array: Vec<i64> = conversion_criteria_number_tmp_array.iter().map(|&x| x.parse::<i64>().unwrap()).collect();
    println!("Conversion Criteria Number Array: {:?}", conversion_criteria_number_array);

    // 与えられた数字の組み合わせで全ての計算式を計算
    let mut result1: HashMap<i64, Vec<String>> = HashMap::new();

    // println!("Result1: {:?}", result1);


    "waaa".to_string()
}


fn main() {
    println!("Converting natural number to 334 equation");
        // コマンドライン引数を取得
    let args: Vec<String> = env::args().collect();

    // 引数のバリデーション
    if args.len() < 2 {
        eprintln!("エラー: 数字を引数として指定してください。");
        eprintln!("使い方: {} <変換したい自然数>", args[0]);
        std::process::exit(1);
    }

    // 引数をu64型にパース
    let num_to_factor = match args[1].parse::<u64>() {
        Ok(n) => n,
        Err(_) => {
            eprintln!("エラー: 有効な正の整数を指定してください。");
            std::process::exit(1);
        }
    };

    let factors = prime_factorization::prime_factorize(num_to_factor);
        for (prime, exp) in factors {
        print!("{{{},{}}}", prime, exp);
        convert_to_334(prime as i64, exp as i64);
    }

    let nums = vec![3,3,4];
    let result = preparation::create_all_combinations(&nums);
    // println!("Result: {:?}", result);
    let mut sorted_result: Vec<_> = result.iter().collect();
    sorted_result.sort();
    println!("Sorted Result: {:?}", sorted_result);
    // println!("result: {:?}", create_combination(vec![334,33,34,3,3,4], 4));
}