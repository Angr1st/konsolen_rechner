use std::error;
use std::fmt;
use std::io::Write;
use std::io::{self, ErrorKind};
use std::num::ParseIntError;

#[derive(Debug)]
struct Operands {
    operand_one: usize,
    operand_two: usize,
}

#[derive(Debug)]
enum Errors {
    IoError(std::io::Error),
    ParseError(ParseIntError),
    OperatorParseError(std::io::Error),
}

#[derive(Debug)]
enum Operators {
    Plus(Operands),
    Minus(Operands),
    Multiply(Operands),
    Divide(Operands),
}

impl fmt::Display for Operators {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Operators::Plus(x) => write!(
                f,
                "{} + {} = {}",
                x.operand_one,
                x.operand_two,
                x.operand_one + x.operand_two
            ),
            Operators::Minus(x) => write!(
                f,
                "{} - {} = {}",
                x.operand_one,
                x.operand_two,
                x.operand_one - x.operand_two
            ),
            Operators::Multiply(x) => write!(
                f,
                "{} * {} = {}",
                x.operand_one,
                x.operand_two,
                x.operand_one * x.operand_two
            ),
            Operators::Divide(x) => write!(
                f,
                "{} / {} = {}",
                x.operand_one,
                x.operand_two,
                x.operand_one / x.operand_two
            ),
        }
    }
}

impl error::Error for Operators {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        None
    }
}

impl std::convert::TryFrom<(String, Operands)> for Operators {
    type Error = io::Error;

    fn try_from(item: (String, Operands)) -> Result<Self, Self::Error> {
        match item.0.as_ref() {
            "+" => Ok(Operators::Plus(item.1)),
            "-" => Ok(Operators::Minus(item.1)),
            "*" => Ok(Operators::Multiply(item.1)),
            "/" => Ok(Operators::Divide(item.1)),
            x => Err(Self::Error::new(
                ErrorKind::InvalidInput,
                format!("The value: {} is not a supported operator!", x),
            )),
        }
    }
}

impl fmt::Display for Errors {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Errors::IoError(erro) => write!(f, "{}", erro),
            Errors::ParseError(erro) => write!(f, "{}", erro),
            Errors::OperatorParseError(erro) => write!(f, "{}", erro),
        }
    }
}

impl error::Error for Errors {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            Errors::IoError(erro) => Some(erro),
            Errors::ParseError(erro) => Some(erro),
            Errors::OperatorParseError(erro) => Some(erro),
        }
    }
}

impl std::convert::From<Errors> for io::Error {
    fn from(item: Errors) -> Self {
        Self::new(ErrorKind::InvalidInput, item)
    }
}

fn main() -> io::Result<()> {
    let operand1 = get_user_input("Operand 1:")?;
    let operand2 = get_user_input("Operand 2:")?;

    let operands = Operands {
        operand_one: operand1,
        operand_two: operand2,
    };
    let operator = get_operator("Operator (+,-,*,/):", operands)?;

    println!("{}",operator);
    io::stdout().flush().unwrap(); 
    Ok(())
}

use std::convert::TryFrom;

fn get_operator(call: &str, operands: Operands) -> Result<Operators, Errors> {
    match get_input(call) {
        Err(x) => Err(x),
        Ok(x) => match Operators::try_from((x, operands)) {
            Ok(y) => Ok(y),
            Err(y) => Err(Errors::OperatorParseError(y)),
        },
    }
}

fn get_user_input(name: &str) -> Result<usize, Errors> {
    get_input(name).and_then(|x| to_usize(x))
}

fn get_input(name: &str) -> Result<String, Errors> {
    print_to_output(name);
    let mut buffer = String::new();
    match io::stdin().read_line(&mut buffer) {
        Err(error) => Err(Errors::IoError(error)),
        Ok(_) => Ok(buffer.trim_start().trim_end().to_owned()),
    }
}

fn to_usize(value: String) -> Result<usize, Errors> {
    match usize::from_str_radix(&value, 10) {
        Ok(num) => Ok(num),
        Err(error) => Err(Errors::ParseError(error)),
    }
}

fn print_to_output(value: &str) {
    print!("{}", value);
    io::stdout().flush().unwrap();
}
