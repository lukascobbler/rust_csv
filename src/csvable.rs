pub mod user;
pub mod flight;

pub trait ToCsv {
    fn to_csv(&self) -> String;
    fn from_csv(line: &str) -> Option<Self> where Self: Sized;
    fn main_key(&self) -> String;
}