use std::io;

fn main() {
    loop {
        println!("Введіть вираз для обчислення (наприклад, 5 + 3 * 2 - 4 / 2):");

        let mut expression = String::new();
        io::stdin().read_line(&mut expression).expect("Помилка зчитування");

        match evaluate_expression(&expression) {
            Ok(result) => println!("Результат: {}", result),
            Err(e) => println!("Помилка: {}", e),
        }

        println!("Хочете виконати ще одну операцію? (y/n)");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Помилка зчитування");

        if choice.trim().to_lowercase() != "y" {
            break;
        }
    }
}

fn evaluate_expression(expression: &str) -> Result<f64, &'static str> {
    let mut tokens = tokenize(expression)?;
    let mut result = parse_term(&mut tokens)?;

    while let Some(op) = tokens.pop() {
        if op == '+' || op == '-' {
            let next_term = parse_term(&mut tokens)?;
            if op == '+' {
                result += next_term;
            } else {
                result -= next_term;
            }
        } else {
            return Err("Невідомий оператор.");
        }
    }

    Ok(result)
}

fn parse_term(tokens: &mut Vec<char>) -> Result<f64, &'static str> {
    let mut result = parse_factor(tokens)?;

    while let Some(op) = tokens.last().cloned() {
        if op == '*' || op == '/' {
            tokens.pop();
            let next_factor = parse_factor(tokens)?;
            if op == '*' {
                result *= next_factor;
            } else {
                if next_factor == 0.0 {
                    return Err("Ділення на нуль.");
                }
                result /= next_factor;
            }
        } else {
            break;
        }
    }

    Ok(result)
}

fn parse_factor(tokens: &mut Vec<char>) -> Result<f64, &'static str> {
    let mut number = String::new();

    while let Some(ch) = tokens.pop() {
        if ch.is_numeric() || ch == '.' {
            number.push(ch);
        } else {
            tokens.push(ch);
            break;
        }
    }

    number.parse::<f64>().map_err(|_| "Невірне число")
}

fn tokenize(expression: &str) -> Result<Vec<char>, &'static str> {
    let mut tokens = Vec::new();
    for ch in expression.chars().rev() {
        if ch.is_whitespace() {
            continue;
        } else if ch.is_numeric() || "+-*/".contains(ch) {
            tokens.push(ch);
        } else {
            return Err("Невірний символ у виразі.");
        }
    }
    Ok(tokens)
}
