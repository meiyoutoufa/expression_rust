use std::collections::HashMap;
use crate::stack::Stack;

pub fn precedence_map() -> HashMap<String, usize> {
    let mut map: HashMap<String, usize> = HashMap::new();
    map.insert("||".to_string(), 1);
    map.insert("&&".to_string(), 2);
    map.insert("==".to_string(), 3);
    map.insert("!=".to_string(), 3);
    map.insert("<".to_string(), 3);
    map.insert(">".to_string(), 3);
    map.insert("<=".to_string(), 3);
    map.insert(">=".to_string(), 3);
    map.insert("+".to_string(), 4);
    map.insert("-".to_string(), 4);
    map.insert("*".to_string(), 5);
    map.insert("/".to_string(), 5);
    map.insert("!".to_string(), 6);
    map.insert("(".to_string(), 0);
    map.insert(")".to_string(), 0);
    map
}

pub struct Expression {
    operator_stack: Stack<String>,
    operand_stack: Stack<f64>,
}

impl Expression {
    pub fn new() -> Expression {
        Expression { operator_stack: Stack::new(), operand_stack: Stack::new() }
    }
    // 判断字符串是否是数字
    fn is_number(token: &str) -> bool {
        token.parse::<f64>().is_ok()
    }
    // 布尔值转换为浮点数
    fn bool_to_float(value: bool) -> f64 {
        if value { 1.0 } else { 0.0 }
    }

    // 计算二元运算
    fn calculate_operation(&self, a: f64, b: f64, operator: &str) -> f64 {
        match operator {
            "+" => a + b,
            "-" => a - b,
            "*" => a * b,
            "/" => a / b,
            "&&" => Self::bool_to_float(a != 0.0 && b != 0.0),
            "||" => Self::bool_to_float(a != 0.0 || b != 0.0),
            ">" => Self::bool_to_float(a > b),
            "<" => Self::bool_to_float(a < b),
            "==" => Self::bool_to_float(a == b),
            "!=" => Self::bool_to_float(a != b),
            ">=" => Self::bool_to_float(a >= b),
            "<=" => Self::bool_to_float(a <= b),
            _ => 0.0, // 默认返回 0.0
        }
    }

    pub fn infix_to_postfix(&mut self, expr: &str) -> Vec<String> {
        let tokens: Vec<&str> = expr.split_whitespace().collect();
        let mut output = Vec::new();
        for token in tokens {
            match token {
                // 如果是数字，直接添加到输出
                _ if Self::is_number(token) => output.push(token.to_string()),
                // 如果是左括号，压入栈
                "(" => self.operator_stack.push(token.to_string()),
                // 如果是右括号，弹出栈直到遇到左括号
                ")" => {
                    while let Some(top) = self.operator_stack.last() {
                        if top == "(" {
                            break;
                        }
                        if let Some(op) = self.operator_stack.pop() {
                            output.push(op);
                        }
                    }
                    // 弹出左括号
                    self.operator_stack.pop();
                }
                _ => {
                    while let Some(top) = self.operator_stack.last() {
                        if top != "(" && precedence_map()[top] >= precedence_map()[&token.to_string()] {
                            if let Some(op) = self.operator_stack.pop() {
                                output.push(op);
                            }
                        } else {
                            break;
                        }
                    }
                    self.operator_stack.push(token.to_string());
                }
            }
        }
        // 处理栈中剩余的运算符
        while let Some(op) = self.operator_stack.pop() {
            output.push(op);
        }
        output
    }

    pub fn evaluate_postfix(&mut self, tokens: Vec<String>) -> bool {
        self.operand_stack = Stack::new();
        for token_srting in tokens {
            let token = token_srting.as_str();
            match token.to_string().as_str() {
                _ if Self::is_number(&token) => {
                    let num: f64 = token.parse().unwrap();
                    self.operand_stack.push(num);
                }
                "!" => {
                    // 处理逻辑运算符（如"!"，表示非运算）
                    if let Some(operand) = self.operand_stack.pop() {
                        let result = Self::bool_to_float(operand == 0.0);
                        self.operand_stack.push(result);
                    }
                }
                _ => {
                    // 处理二元运算符
                    if let Some(b) = self.operand_stack.pop() {
                        if let Some(a) = self.operand_stack.pop() {
                            let result = self.calculate_operation(a, b, &token);
                            self.operand_stack.push(result);
                        }
                    }
                }
            }
        }
        // 返回栈顶的操作数，若不为0则返回true
        if let Some(final_result) = self.operand_stack.pop() {
            final_result != 0.0
        } else {
            false
        }
    }

    pub fn evaluate_expression(&mut self, expr: &str) -> bool {
        let mut posfix = Self::infix_to_postfix(self, expr);
        println!("posfix: {:?}", posfix);
        Self::evaluate_postfix(self, posfix)
    }
}
