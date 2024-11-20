use std::collections::HashMap;

#[derive(Debug, Clone, Copy)]
enum Operator {
    Add,
    Sub,
    Mul,
    Div,
}

impl Operator {
    fn apply(&self, a: i32, b: i32) -> Option<i32> {
        match self {
            Operator::Add => Some(a + b),
            Operator::Sub => Some(a - b),
            Operator::Mul => Some(a * b),
            Operator::Div => if b != 0 { Some(a / b) } else { None },
        }
    }
    
    fn to_string(&self) -> &str {
        match self {
            Operator::Add => "+",
            Operator::Sub => "-",
            Operator::Mul => "*",
            Operator::Div => "/",
        }
    }
    
    fn precedence(&self) -> i32 {
        match self {
            Operator::Mul | Operator::Div => 2,
            Operator::Add | Operator::Sub => 1,
        }
    }
}

// 与えられた数字の組み合わせで全ての計算式を計算
// 計算結果と計算式のリストのマップを返す
pub fn calculate_all_expressions(numbers: &[i32]) -> HashMap<i32, Vec<String>> {
    let mut results: HashMap<i32, Vec<String>> = HashMap::new();
    
    if numbers.len() == 1 {
        results.insert(numbers[0], vec![numbers[0].to_string()]);
        return results;
    }
    
    let operators = [Operator::Add, Operator::Sub, Operator::Mul, Operator::Div];
    
    for i in 1..numbers.len() {
        let left_map = calculate_all_expressions(&numbers[..i]);
        let right_map = calculate_all_expressions(&numbers[i..]);
        
        for (&left_val, left_formulas) in left_map.iter() {
            for (&right_val, right_formulas) in right_map.iter() {
                for &op in &operators {
                    // 除算の場合は割り切れるかチェック
                    let should_calculate = match op {
                        Operator::Div => right_val != 0 && left_val % right_val == 0,
                        _ => true
                    };
                    if should_calculate {
                        if let Some(value) = op.apply(left_val, right_val) {
                            // 値がある場合、式を作成
                            for left_formula in left_formulas {
                                for right_formula in right_formulas {
                                    // 括弧が必要か判定
                                    let formula = if needs_parentheses(left_formula, op) && needs_parentheses(right_formula, op) {
                                        format!("({}){}({})", left_formula, op.to_string(), right_formula)
                                    } else if needs_parentheses(left_formula, op) {
                                        format!("({}){}{}", left_formula, op.to_string(), right_formula)
                                    } else if needs_parentheses(right_formula, op) {
                                        format!("{}{}({})", left_formula, op.to_string(), right_formula)
                                    } else {
                                        format!("{}{}{}", left_formula, op.to_string(), right_formula)
                                    };
                                    
                                    results.entry(value)
                                        .or_insert_with(Vec::new)
                                        .push(formula);
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    
    // 各値の式リストから重複を除去し、最短の式を優先
    for formulas in results.values_mut() {
        formulas.sort_by_key(|formula| formula.len());
        formulas.dedup();
    }
    
    results
}

// 与えられた計算式に括弧が必要か判定
fn needs_parentheses(formula: &str, parent_op: Operator) -> bool {
    if !formula.contains('+') && !formula.contains('-') && 
       !formula.contains('*') && !formula.contains('/') {
        return false;
    }
    
    let last_op = formula.chars()
        .find(|&c| c == '+' || c == '-' || c == '*' || c == '/')
        .and_then(|c| match c {
            '+' => Some(Operator::Add),
            '-' => Some(Operator::Sub),
            '*' => Some(Operator::Mul),
            '/' => Some(Operator::Div),
            _ => None,
        });
    
    if let Some(last_op) = last_op {
        // 親の演算子よりも優先度が低い(加算と引算)場合は括弧が必要
        last_op.precedence() < parent_op.precedence()
    } else {
        false
    }
}