//! 对齐: `cn.hutool.core.math.Calculator`
//! 来源: hutool-core/src/main/java/cn/hutool/core/math/Calculator.java
//!
//! 数学表达式求值（后缀栈），覆盖 CalculatorTest 向量。

use rust_decimal::Decimal;
use rust_decimal::prelude::ToPrimitive;

/// 表达式计算器 —— 对齐 Java `Calculator`。
#[derive(Debug, Default)]
pub struct Calculator {
    postfix_stack: Vec<String>,
    /// 运算符优先级表（索引 = ASCII - 40），与 Hutool 一致。
    operat_priority: [i32; 8],
}

impl Calculator {
    /// 对齐 Java: `new Calculator()`
    pub fn new() -> Self {
        Self {
            postfix_stack: Vec::new(),
            operat_priority: [0, 3, 2, 1, -1, 1, 0, 2],
        }
    }

    /// 对齐 Java: `Calculator.conversion(String)`
    pub fn conversion(expression: &str) -> f64 {
        Self::new().calculate(expression)
    }

    /// 对齐 Java: `calculate(String expression)`
    pub fn calculate(&mut self, expression: &str) -> f64 {
        self.postfix_stack.clear();
        let transformed = transform(expression);
        self.prepare(&transformed);

        let mut result_stack: Vec<String> = Vec::new();
        self.postfix_stack.reverse();
        while let Some(mut current_op) = self.postfix_stack.pop() {
            if current_op.is_empty() {
                continue;
            }
            let first_char = current_op.chars().next().unwrap();
            if !is_operator(first_char) {
                current_op = current_op.replace('~', "-");
                result_stack.push(current_op);
            } else {
                let second_value = result_stack.pop().unwrap_or_else(|| "0".into());
                let first_value = result_stack.pop().unwrap_or_else(|| "0".into());
                let first_value = first_value.replace('~', "-");
                let second_value = second_value.replace('~', "-");
                let temp = calc_op(&first_value, &second_value, first_char);
                result_stack.push(temp.to_string());
            }
        }
        // 省略乘法：(11+2)12 → 多操作数相乘
        mul_all(&result_stack)
    }

    fn prepare(&mut self, expression: &str) {
        let mut op_stack: Vec<char> = vec![','];
        let arr: Vec<char> = expression.chars().collect();
        let mut current_index = 0usize;
        let mut count = 0usize;
        for (i, &current_op) in arr.iter().enumerate() {
            if is_operator(current_op) {
                if count > 0 {
                    self.postfix_stack
                        .push(arr[current_index..current_index + count].iter().collect());
                }
                let mut peek_op = *op_stack.last().unwrap();
                if current_op == ')' {
                    while *op_stack.last().unwrap() != '(' {
                        self.postfix_stack.push(op_stack.pop().unwrap().to_string());
                    }
                    op_stack.pop();
                } else {
                    while current_op != '('
                        && peek_op != ','
                        && self.compare_ops(current_op, peek_op)
                    {
                        self.postfix_stack.push(op_stack.pop().unwrap().to_string());
                        peek_op = *op_stack.last().unwrap();
                    }
                    op_stack.push(current_op);
                }
                count = 0;
                current_index = i + 1;
            } else {
                count += 1;
            }
        }
        if count > 1
            || (count == 1 && current_index < arr.len() && !is_operator(arr[current_index]))
        {
            self.postfix_stack
                .push(arr[current_index..current_index + count].iter().collect());
        }
        while *op_stack.last().unwrap() != ',' {
            self.postfix_stack.push(op_stack.pop().unwrap().to_string());
        }
    }

    fn compare_ops(&self, cur: char, peek: char) -> bool {
        let offset = 40i32;
        let mut cur_c = cur;
        let mut peek_c = peek;
        if cur_c == '%' {
            cur_c = '/';
        }
        if peek_c == '%' {
            peek_c = '/';
        }
        let peek_idx = (peek_c as i32 - offset) as usize;
        let cur_idx = (cur_c as i32 - offset) as usize;
        self.operat_priority[peek_idx] >= self.operat_priority[cur_idx]
    }
}

fn is_operator(c: char) -> bool {
    matches!(c, '+' | '-' | '*' | '/' | '(' | ')' | '%')
}

fn is_prev_op_or_lparen(c: char) -> bool {
    matches!(c, '%' | '+' | '-' | '*' | '/' | '(')
}

fn transform(expression: &str) -> String {
    let expression = expression
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect::<String>();
    let expression = expression.strip_suffix('=').unwrap_or(&expression);
    let arr: Vec<char> = expression.chars().collect();
    let mut out = String::with_capacity(arr.len());
    let mut i = 0usize;
    while i < arr.len() {
        let c = arr[i];
        if c.eq_ignore_ascii_case(&'x') {
            out.push('*');
            i += 1;
            continue;
        }
        if c == '+' || c == '-' {
            if let Some(prev_out) = out.chars().last() {
                if prev_out == 'e' || prev_out == 'E' {
                    if c == '-' {
                        out.push('~');
                    }
                    i += 1;
                    continue;
                }
            }
            let mut j = i as i32 - 1;
            while j >= 0 && arr[j as usize].is_whitespace() {
                j -= 1;
            }
            let unary = j < 0 || is_prev_op_or_lparen(arr[j as usize]);
            if unary {
                let mut k = i;
                let mut minus_count = 0;
                while k < arr.len() && (arr[k] == '+' || arr[k] == '-') {
                    if arr[k] == '-' {
                        minus_count += 1;
                    }
                    k += 1;
                }
                if minus_count % 2 == 1 {
                    out.push('~');
                }
                i = k;
            } else {
                out.push(c);
                i += 1;
            }
            continue;
        }
        out.push(c);
        i += 1;
    }
    let mut res: Vec<char> = out.chars().collect();
    if res.len() >= 2 && res[0] == '~' && res[1] == '(' {
        res[0] = '-';
        return format!("0{}", res.iter().collect::<String>());
    }
    res.into_iter().collect()
}

fn parse_decimal(s: &str) -> Decimal {
    let s = s.replace('~', "-");
    s.parse::<Decimal>().unwrap_or(Decimal::ZERO)
}

fn calc_op(first: &str, second: &str, op: char) -> Decimal {
    let a = parse_decimal(first);
    let b = parse_decimal(second);
    if (op == '/' || op == '%') && b.is_zero() {
        panic!("Division by zero: cannot divide {first} by zero");
    }
    match op {
        '+' => a + b,
        '-' => a - b,
        '*' => a * b,
        '/' => a / b,
        '%' => a % b,
        _ => panic!("Unexpected value: {op}"),
    }
}

fn mul_all(values: &[String]) -> f64 {
    if values.is_empty() {
        return 0.0;
    }
    let mut acc = Decimal::ONE;
    for v in values {
        acc *= parse_decimal(v);
    }
    acc.to_f64().unwrap_or(0.0)
}
