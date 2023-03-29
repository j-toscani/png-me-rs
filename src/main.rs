mod args;
mod chunks;
mod chunk_type;
mod commads;
mod png;

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

fn main() {
    todo!();
}
