use std::fmt;
use std::fs::File;
use std::io;

struct AppError {
    kind: String,
    message: String,
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let err_msg = match self.code {
            404 => "Sorry, Can not find the Page!",
            _ => "Sorry, something is wrong! Please Try Again!",
        };

        write!(f, "{}", err_msg)
    }
}

// for debugging output
impl fmt::Debug for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "AppError {{ code: {}, message: {} }}",
            self.code, self.message
        )
    }
}
impl From<io::Error> for AppError {
    fn from(error: io::Error) -> Self {
        AppError {
            kind: String::from("io"),
            message: error.to_string(),
        }
    }
}

// Implement std::convert::From for AppError; from num::ParseIntError
impl From<num::ParseIntError> for AppError {
    fn from(error: num::ParseIntError) -> Self {
        AppError {
            kind: String::from("parse"),
            message: error.to_string(),
        }
    }
}


// example use to
fn main() -> Result<(), AppError> {
    let mut file = File::open("no.txt")?; // generates an io::Error, if can not open the file and converts to an AppError

    let mut content = String::new();
    file.read_to_string(&mut content)?; // generates an io::Error, if can not read file content and converts to an AppError

    let _number: usize;
    _number = content.parse()?; // generates num::ParseIntError, if can not convert file content to usize and converts to an AppError

    Ok(())
}