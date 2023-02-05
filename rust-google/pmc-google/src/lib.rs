pub mod drive;
pub mod sheets;

pub fn hello(name: String) -> String {
    String::from(format!("Hello, {}!", name))
}
