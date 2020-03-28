use std::io::Write;
use std::num::ParseIntError;
use std::io::{self, ErrorKind};
use std::error;
use std::fmt;

#[derive(Debug)]
enum Errors {
    IoError(std::io::Error),
    ParseError(ParseIntError)
}

impl fmt::Display for Errors {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Errors::IoError(erro) => write!(f, "{}", erro),
            Errors::ParseError(erro) => write!(f, "{}", erro)
        }
    }
}

impl error::Error for Errors {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            Errors::IoError(erro) => Some(erro),
            Errors::ParseError(erro) => Some(erro)
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