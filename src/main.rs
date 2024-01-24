use std::str::FromStr;

use chunk::Chunk;
use chunk_type::ChunkType;

mod args;
mod chunk;
mod chunk_type;
mod commands;
mod png;

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

fn main() -> Result<()> {
  
    // let res = std::str::from_utf8(&[0, 0, 0, 50, 82, 117, 83, 116, 84, 104, 105, 115, 32, 105, 115, 32, 119, 104, 101, 114, 101, 32, 121, 111, 117, 114, 32, 115, 101, 99, 114, 101, 116, 32, 109, 101, 115, 115, 97, 103, 101, 32, 119, 105, 108, 108, 32, 98, 101, 33, 0, 0, 0, 4]);
    // println!("{:?}", res);
    Result::Ok(())
    // todo!()
}
