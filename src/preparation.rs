use std::collections::HashMap;

#[derive(Debug, Clone, Copy)]
enum Operator {
    Add,
    Sub,
    Mul,
    Div,
}

impl Operator {
    fn apply(&self, a: i64, b: i64) -> Option<i64> {
        match self {
            Operator::Add => Some(a + b),
            Operator::Sub => Some(a - b),
            Operator::Mul => Some(a * b),
            Operator::Div => {
                if b != 0 {
                    Some(a / b)
                } else {
                    None
                }
            }
        }
    }

    fn to_str(&self) -> &'static str {
        match self {
            Operator::Add => "+",
            Operator::Sub => "-",
            Operator::Mul => "*",
            Operator::Div => "/",
        }
    }

    fn from_str(s: &str) -> Option<Operator> {
        match s {
            "+" => Some(Operator::Add),
            "-" => Some(Operator::Sub),
            "*" => Some(Operator::Mul),
            "/" => Some(Operator::Div),
            _ => None,
        }
    }
}
// 部分列の先頭を切り出して数に変換して、残りを結合する。
// `comma` ＝残りに挿入するカンマの数
fn number_combinations(n: &[i64], comma: usize) -> Vec<Vec<i64>> {
    if comma == 0 {
        return vec![trans_list_to_number(n)];
    }

    let mut result: Vec<Vec<i64>> = Vec::new();

    // i = 切り出す先頭の長さ（1..= n.len()-comma）
    let max_i = n.len().saturating_sub(comma);
    for i in 1..=max_i {
        let left = trans_list_to_number(&n[..i]);
        for sublist in number_combinations(&n[i..], comma - 1) {
            let mut combined = left.clone();
            combined.extend(sublist);
            result.push(combined);
        }
    }
    result
}

// 桁列を整数に変換して1要素のベクタにする。
// 例: [3,3] -> vec![33]
fn trans_list_to_number(n: &[i64]) -> Vec<i64> {
    let mut result = 0i64;
    for &d in n {
        result = result * 10 + d;
    }
    vec![result]
}

// 与えられた組み合わせから、順序を保ったまま生成できる
// すべての式を生成して、結果値 -> RPN 式列 のマップを返す。
// 返り値: HashMap<計算結果, Vec<RPN文字列>>
fn calculate_all_expressions(nums: &[i64]) -> HashMap<i64, Vec<String>> {
    let mut map: HashMap<i64, Vec<String>> = HashMap::new();
    if nums.is_empty() {
        return map;
    }

    // 再帰で全ての式（RPNのみ）を生成する
    let rpn_list = generate_all_exprs(nums);

    for rpn in rpn_list {
        if let Some(val) = evaluate_rpn(&rpn) {
            let entry = map.entry(val).or_insert_with(Vec::new);
            entry.push(rpn.clone());
        }
    }

    // 重複除去と短い式優先（RPN文字列長でソート）
    for (_k, v) in map.iter_mut() {
        v.sort_by_key(|s| (s.len(), s.clone()));
        v.dedup();
    }

    map
}

// 入力数列に対して、順序を保ったまますべての
// 二分木構造（＝括弧の入れ方）と演算子割当を生成し、
// 各組合せの RPN 文字列を返す。
// RPN のトークンは空白で区切る。
fn generate_all_exprs(nums: &[i64]) -> Vec<String> {
    if nums.len() == 1 {
        return vec![nums[0].to_string()];
    }

    let mut results: Vec<String> = Vec::new();

    // split point: left uses first k operands, right uses the rest
    for k in 1..nums.len() {
        let left = &nums[..k];
        let right = &nums[k..];

        let left_exprs = generate_all_exprs(left);
        let right_exprs = generate_all_exprs(right);

        for lrpn in &left_exprs {
            for rrpn in &right_exprs {
                for op in &[Operator::Add, Operator::Sub, Operator::Mul, Operator::Div] {
                    // RPN: left right op
                    let rpn = format!("{} {} {}", lrpn, rrpn, op.to_str());
                    results.push(rpn);
                }
            }
        }
    }

    results
}

// RPN する（失敗時は None）。
// トークンは空白区切りで、演算は整数演算（割り算はゼロ除算で失敗）。
fn evaluate_rpn(rpn: &str) -> Option<i64> {
    let mut stack: Vec<i64> = Vec::new();
    for tok in rpn.split_whitespace() {
        if let Ok(n) = tok.parse::<i64>() {
            stack.push(n);
        } else if let Some(op) = Operator::from_str(tok) {
            // pop right then left
            if stack.len() < 2 {
                return None;
            }
            let b = stack.pop().unwrap();
            let a = stack.pop().unwrap();
            if let Some(res) = op.apply(a, b) {
                stack.push(res);
            } else {
                return None;
            }
        } else {
            return None;
        }
    }
    if stack.len() == 1 {
        Some(stack[0])
    } else {
        None
    }
}

// すべての組み合わせを返す。
// [3,3,4] -> [[3,3,4], [33,4], [3,34], [334]]
pub fn create_combination(n: &[i64]) -> Vec<Vec<i64>> {
    let mut result: Vec<Vec<i64>> = Vec::new();
    if n.is_empty() {
        return result;
    }
    for comma in 0..n.len() {
        result.extend(number_combinations(n, comma));
    }
    result
}

// 与えられた桁列に対するすべての結合パターンごとに式を生成・評価し、
// 結果値 -> RPN 式列 の総合マップを返す。
pub fn create_all_combinations(n: &[i64]) -> HashMap<i64, Vec<String>> {
    let mut result: HashMap<i64, Vec<String>> = HashMap::new();

    for comb in create_combination(n) {
        let map = calculate_all_expressions(&comb);
        for (k, mut v) in map {
            let entry = result.entry(k).or_insert_with(Vec::new);
            entry.append(&mut v);
        }
    }

    // 最終的な重複除去・ソート（短い式優先）
    for (_k, v) in result.iter_mut() {
        v.sort_by_key(|s| (s.len(), s.clone()));
        v.dedup();
    }

    result
}

