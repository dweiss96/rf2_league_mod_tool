use std::any::Any;

#[cfg(test)]
mod tests {
    use super::*;
    use serde::Deserialize;
    use std::fmt::{Debug, Display, Formatter};
    use std::io::ErrorKind;

    #[derive(Deserialize)]
    struct TestError {
        pub err_msg: String,
    }

    impl Debug for TestError {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.err_msg.as_str())
        }
    }

    impl Display for TestError {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.err_msg.as_str())
        }
    }

    impl std::error::Error for TestError {}

    #[test]
    fn io_error_is_catchable() {
        let err = std::io::Error::new(ErrorKind::NotFound, "error");
        assert_eq!(err.catch().msg, "error");
        assert_eq!(
            err.catch_with_msg("custom error msg".to_string()).msg,
            "custom error msg"
        );
    }

    #[test]
    fn from_utf8_error_is_catchable() {
        // some invalid bytes, in a vector
        let bytes = vec![0, 159];

        let err = String::from_utf8(bytes).err().unwrap();
        assert_eq!(
            err.catch().msg,
            "invalid utf-8 sequence of 1 bytes from index 1"
        );
        assert_eq!(
            err.catch_with_msg("custom error msg".to_string()).msg,
            "custom error msg"
        );
    }

    #[test]
    fn parse_int_error_is_catchable() {
        let err = "no int".parse::<i8>().err().unwrap();
        assert_eq!(err.catch().msg, "invalid digit found in string");
        assert_eq!(
            err.catch_with_msg("custom error msg".to_string()).msg,
            "custom error msg"
        );
    }

    #[test]
    fn serde_json_error_is_catchable() {
        let err = serde_json::from_str::<TestError>("no json").err().unwrap();
        assert_eq!(err.catch().msg, "expected ident at line 1 column 2");
        assert_eq!(
            err.catch_with_msg("custom error msg".to_string()).msg,
            "custom error msg"
        );
    }

    // #[test]
    // fn std_error_error_is_catchable() {
    //     let base_err =  TestError { err_msg: "error" };
    //     let err: dyn std::error::Error = &base_err;
    //     assert_eq!(err.catch().msg, "error");
    //     assert_eq!(err.catch_with_msg("custom error msg".to_string()).msg, "custom error msg");
    // }

    // #[test]
    // fn joinhandle_error_is_catchable() {
    //     // let err = std::io::Error::new(ErrorKind::NotFound, "error");
    //     assert_eq!(err.catch().msg, "error");
    //     assert_eq!(
    //         err.catch_with_msg("custom error msg".to_string()).msg,
    //         "custom error msg"
    //     );
    // }

    #[test]
    fn result_of_catchable_can_be_caught() {
        let err_res = "no int".parse::<i8>();
        assert_eq!(
            err_res.clone().catch_err().err().unwrap().msg,
            "invalid digit found in string"
        );
        assert_eq!(
            err_res
                .catch_err_with_msg("custom error msg".to_string())
                .err()
                .unwrap()
                .msg,
            "custom error msg"
        );
    }

    #[test]
    fn any_empty_option_can_be_caught() {
        let err_opt = "no int".parse::<i8>().ok();
        let custom_opt = None::<u8>;
        assert_eq!(
            err_opt
                .catch_none("custom error msg".to_string())
                .err()
                .unwrap()
                .msg,
            "custom error msg"
        );
        assert_eq!(
            custom_opt
                .catch_none("custom error msg".to_string())
                .err()
                .unwrap()
                .msg,
            "custom error msg"
        );
    }
}

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
