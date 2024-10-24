use std::io;

#[derive(Debug)]
enum CalcError {
    InvalidInput,
    DivisionByZero,
}

struct Calculator {
    memory: f64,
}

impl Calculator {
    fn new() -> Self {
        Calculator { memory: 0.0 }
    }

    fn operations(&mut self, input: &str) -> Result<f64, CalcError> {
        let parts: Vec<&str> = input.trim().split_whitespace().collect();
        if parts.len() != 2 {
            return Err(CalcError::InvalidInput);
        }
        let operation = parts[0];
        let value = parts[1].parse::<f64>().map_err(|_| CalcError::InvalidInput)?;

        match operation {
            "/" => {
                if value == 0.0 {
                    Err(CalcError::DivisionByZero)
                } else {
                    self.memory /= value;
                    Ok(self.memory)
                }
            }
            "*" => {
                self.memory *= value;
                Ok(self.memory)
            }
            "+" => {
                self.memory += value;
                Ok(self.memory)
            }
            "-" => {
                self.memory -= value;
                Ok(self.memory)
            }
            _ => Err(CalcError::InvalidInput),
        }
    }
}

fn main() {
    let mut calculator = Calculator::new();
    println!("Введіть операцію з числом або 'q' для виходу:");

    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        if input.trim() == "q" {
            break;
        }

        match calculator.operations(&input) {
            Ok(result) => println!("Результат: {}", result),
            Err(CalcError::InvalidInput) => println!("Введіть коректну операцію"),
            Err(CalcError::DivisionByZero) => println!("Ділення на нуль не можливе"),
        }
    }
}
