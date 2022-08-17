use std::error::Error;

type ResultX<T> = Result<T, Box<dyn Error>>;


mod template;


pub use template::flate2_tar;


