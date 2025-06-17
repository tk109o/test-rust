use std::collections::HashMap;

trait Divisible {
    fn is_divisible_by(&self, divisor: u32) -> bool;
}

impl Divisible for u32 {
    fn is_divisible_by(&self, divisor: u32) -> bool {
        self % divisor == 0
    }
}

struct FizzBuzzRule {
    divisor: u32,
    word: String,
}

impl FizzBuzzRule {
    fn new(divisor: u32, word: &str) -> Self {
        FizzBuzzRule {
            divisor,
            word: word.to_string(),
        }
    }

    fn applies_to(&self, number: u32) -> bool {
        number.is_divisible_by(self.divisor)
    }
}

struct FizzBuzzEngine {
    rules: Vec<FizzBuzzRule>,
    cache: HashMap<u32, String>,
}

impl FizzBuzzEngine {
    fn new() -> Self {
        FizzBuzzEngine {
            rules: vec![
                FizzBuzzRule::new(3, "Fizz"),
                FizzBuzzRule::new(5, "Buzz"),
            ],
            cache: HashMap::new(),
        }
    }

    fn add_rule(&mut self, divisor: u32, word: &str) {
        self.rules.push(FizzBuzzRule::new(divisor, word));
        self.cache.clear();
    }

    fn evaluate(&mut self, number: u32) -> String {
        if let Some(cached_result) = self.cache.get(&number) {
            return cached_result.clone();
        }

        let result = self.rules
            .iter()
            .filter(|rule| rule.applies_to(number))
            .map(|rule| rule.word.as_str())
            .collect::<Vec<_>>()
            .join("");

        let final_result = if result.is_empty() {
            number.to_string()
        } else {
            result
        };

        self.cache.insert(number, final_result.clone());
        final_result
    }

    fn run_sequence(&mut self, start: u32, end: u32) -> Vec<String> {
        (start..=end)
            .map(|i| self.evaluate(i))
            .collect()
    }
}

fn main() {
    let mut engine = FizzBuzzEngine::new();
    
    let results = engine.run_sequence(1, 100);
    
    for (index, result) in results.iter().enumerate() {
        println!("{}: {}", index + 1, result);
    }
    
    println!("\nCache size: {}", engine.cache.len());
    
    engine.add_rule(7, "Boom");
    println!("\nWith additional rule (divisible by 7 = 'Boom'):");
    let extended_results = engine.run_sequence(1, 21);
    for (index, result) in extended_results.iter().enumerate() {
        println!("{}: {}", index + 1, result);
    }
}