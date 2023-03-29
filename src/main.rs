mod args;
mod chunk_type;
mod chunk;
mod commads;
mod png;

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

fn main() {
    todo!();
}
