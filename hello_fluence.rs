use marine_rs_sdk::marine;


pub fn main() {}

#[marine]
pub fn hello() -> String {
    format!("Hello, test deployment")
}