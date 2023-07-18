use std::any::Any;

#[derive(Debug)]
pub struct CaughtError {
    pub msg: String,
}

pub trait Catchable {
    fn catch(&self) -> CaughtError;

    fn catch_with_msg(&self, msg: String) -> CaughtError {
        CaughtError { msg }
    }
}

impl Catchable for std::io::Error {
    fn catch(&self) -> CaughtError {
        CaughtError {
            msg: self.to_string(),
        }
    }
}

impl Catchable for dyn std::error::Error {
    fn catch(&self) -> CaughtError {
        CaughtError {
            msg: self.to_string(),
        }
    }
}

impl Catchable for Box<dyn Any + Send> {
    fn catch(&self) -> CaughtError {
        CaughtError {
            msg: "caught some `Box<dyn Any + Send>`".to_string(),
        }
    }
}

impl Catchable for std::string::FromUtf8Error {
    fn catch(&self) -> CaughtError {
        CaughtError {
            msg: self.to_string(),
        }
    }
}

impl Catchable for serde_json::Error {
    fn catch(&self) -> CaughtError {
        CaughtError {
            msg: self.to_string(),
        }
    }
}

impl Catchable for std::num::ParseIntError {
    fn catch(&self) -> CaughtError {
        CaughtError {
            msg: self.to_string(),
        }
    }
}

pub trait CatchableRes<T> {
    fn catch_err(self) -> Result<T, CaughtError>;
    fn catch_err_with_msg(self, msg: String) -> Result<T, CaughtError>;
}
// impl<T> CatchableRes<T> for Result<T, dyn std::error::Error> {
//     fn catch_err(self) -> Result<T, CaughtError> {
//         self.map_err(|e| { e.catch() })
//     }
//     fn catch_err_with_msg(self, msg: String) -> Result<T, CaughtError> {
//         self.map_err(|e| { e.catch_with_msg(msg) })
//     }
// }

impl<T, E> CatchableRes<T> for Result<T, E>
where
    E: Catchable,
{
    fn catch_err(self) -> Result<T, CaughtError> {
        self.map_err(|e| e.catch())
    }

    fn catch_err_with_msg(self, msg: String) -> Result<T, CaughtError> {
        self.map_err(|e| e.catch_with_msg(msg))
    }
}

pub trait CatchableOpt<T> {
    fn catch_none(self, msg: String) -> Result<T, CaughtError>;
}
impl<T> CatchableOpt<T> for Option<T> {
    fn catch_none(self, msg: String) -> Result<T, CaughtError> {
        self.ok_or(CaughtError { msg })
    }
}
