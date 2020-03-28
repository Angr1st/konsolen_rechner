use std::io::Write;
use std::num::ParseIntError;
use std::io::{self, ErrorKind};
use std::error;
use std::fmt;

#[derive(Debug)]
enum Errors {
    IoError(std::io::Error),
    ParseError(ParseIntError),
    OperatorParseError(std::io::Error)
}

#[derive(Debug)]
enum Operators {
    Plus(),
    Minus(),
    Multiply(),
    Divide()
}

impl fmt::Display for Operators {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Operators::Plus() => write!(f, "+"),
            Operators::Minus() => write!(f, "-"),
            Operators::Multiply() => write!(f, "*"),
            Operators::Divide() => write!(f, "/"),
        }
    }
}

impl error::Error for Operators {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        None
    }
}


impl std::convert::TryFrom<String> for Operators {
    type Error = io::Error;

    fn try_from(item:String) -> Result<Self,Self::Error> {
        match item.as_ref() {
            "+" => Ok(Operators::Plus()),
            "-" => Ok(Operators::Minus()),
            "*" => Ok(Operators::Multiply()),
            "/" => Ok(Operators::Divide()),
            x => Err(Self::Error::new(ErrorKind::InvalidInput,format!("The value: {} is not a supported operator!", x)))
        }
    }
}

impl fmt::Display for Errors {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Errors::IoError(erro) => write!(f, "{}", erro),
            Errors::ParseError(erro) => write!(f, "{}", erro),
            Errors::OperatorParseError(erro) => write!(f, "{}", erro)
        }
    }
}

impl error::Error for Errors {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            Errors::IoError(erro) => Some(erro),
            Errors::ParseError(erro) => Some(erro),
            Errors::OperatorParseError(erro) => Some(erro)
        }
    }
}

impl std::convert::From<Errors> for io::Error {
    fn from(item:Errors) -> Self {
        Self::new(ErrorKind::InvalidInput, item)
    }
}

fn main() -> io::Result<()> {
    let operand1 = get_user_input("Operand 1:")?;
    let operand2 = get_user_input("Operand 2:")?;

    let operator = 

    println!("Result: {} + {} = {}", operand1, operand2, operand1 + operand2 );
    Ok(())
}

fn get_user_input(name:&str) -> Result<usize, Errors> {
    get_input(name).and_then(|x| { to_usize(x)})
}

fn get_input(name:&str) -> Result<String,Errors> {
    print!("{}", name);
    io::stdout().flush().unwrap();
    let mut buffer = String::new();
    match io::stdin().read_line(&mut buffer) {
        Err(error) => Err(Errors::IoError(error)),
        Ok(_) => Ok(buffer.trim_start().trim_end().to_owned())
    }
}

fn to_usize(value:String) -> Result<usize, Errors> {
    match usize::from_str_radix(&value, 10) {
        Ok(num) => Ok(num),
        Err(error) => Err(Errors::ParseError(error))
    }
}