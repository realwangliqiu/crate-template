use std::error::Error;

type ResultX<T> = Result<T, Box<dyn Error>>;


mod template;


pub use template::*;


/// diff from dbg! macro : only for Debug build, omit stringify!(expression)
#[macro_export]
#[cfg(debug_assertions)]
macro_rules! debug {
    ($val:expr $(,)?) => {
        match $val {
            tmp => {
                println!("[{}:{}]\n{:#?}", file!(), line!(), &tmp);
            }
        }
    };
}
