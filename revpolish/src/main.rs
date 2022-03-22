use clap::Parser;
use anyhow::{Context, Result, ensure, bail};
use std::fs::File;
use std::io::{stdin, BufRead, BufReader};

#[derive(Parser, Debug)]
#[clap(
    name = "My RPN Program",
    version = "0.1.0",
    author = "Your name",
    about = "about"
)]
struct Opts {
    #[clap(short, long)]
    verbose: bool,

    #[clap(name = "FILE")]
    formula_file: Option<String>,
}

struct RPNCalculator(bool);

impl RPNCalculator {
    pub fn new(verbose: bool) -> Self {
        Self(verbose)
    }

    pub fn eval(&self, formula: &str) -> Result<i32> {
        let mut tokens = formula.split_whitespace().rev().collect::<Vec<_>>();
        self.eval_inner(&mut tokens)
    }

    fn eval_inner(&self, tokens: &mut Vec<&str>) -> Result<i32> {
        let mut stack = Vec::new();
        let mut pos = 0;

        while let Some(token) = tokens.pop() {
            pos += 1;

            if let Ok(x) = token.parse::<i32>() {
                stack.push(x)
            } else {
                let y = stack.pop().context(format!("invalid syntax at {}", pos))?;
                let x = stack.pop().context(format!("invalid syntax at {}", pos))?;

                let res = match token {
                    "+" => x + y,
                    "-" => x - y,
                    "*" => x * y,
                    "/" => x / y,
                    "%" => x % y,
                    _ => bail!("invalid tokens at {}", pos),
                };
                stack.push(res)
            }

            if self.0 {
                println!("{:?}, {:?}", tokens, stack);
            }
        }
        ensure!(stack.len() == 1, "invalid syntax");
        Ok(stack[0])
}

fn main() {
    let opts = Opts::parse();

    if let Some(path) = opts.formula_file {
        let f = File::open(path).unwrap();
        let reader = BufReader::new(f);
        run(reader, opts.verbose);
    } else {
        let stdin = stdin();
        let reader = stdin.lock();
        run(reader, opts.verbose);
    }
}

fn run<R: BufRead>(reader: R, verbose: bool) {
    let calc = RPNCalculator::new(verbose);

    for line in reader.lines() {
        let line = line.unwrap();
        let answer = calc.eval(&line);
        println!("{}", answer);
    }
}

fn get_int_from_file() -> Result<i32, String> {
    let path = "number.txt";
    let num_str = std::fs::read_to_string(path).with_context(|| format!("Failed to read string from {}", path))?;

    num_str
        .trim()
        .parse::<i32>()
        .map(|t| t * 2)
        .map_err(|e| e.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ok() {
        let calc = RPNCalculator::new(false);
        assert_eq!(calc.eval("-50"), -50);

        assert_eq!(calc.eval("2 3 +"), 5);
        assert_eq!(calc.eval("2 3 -"), -1);
        assert_eq!(calc.eval("2 3 *"), 6);
        assert_eq!(calc.eval("2 3 /"), 0);
        assert_eq!(calc.eval("2 3 %"), 2);

<<<<<<< HEAD
        #[test]
        #[should_panic]
        fn test_ng() {
            let calc = RPNCalculator::new(false);
            calc.eval("1 1 ^");
            // as long as RPNCalculator.eval returns panic for an inappropreate input
            unreachable!();
=======
    #[test]
    #[should_panic]
    fn test_ng() {
        let calc = RPNCalculator::new(false);
        calc.eval("1 1 ^");
        // as long as RPNCalculator.eval returns panic for an inappropreate input
        unreachable!();
>>>>>>> a0d366de6a72350efa0bbf24c556c3fcae18a20c
        }
    }
}
