use std::str::FromStr;

use chunk::Chunk;
use chunk_type::ChunkType;
use png::Png;

mod args;
mod chunk;
mod chunk_type;
mod commands;
mod png;

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

fn main() -> Result<()> {

    Result::Ok(())
    // todo!()
}

// let chunk_data = vec![chunk_from_strings("FrSt", "I am the first chunk")
//     .unwrap()];
// let chunk_data = vec![chunk_from_strings("miDl", "I am another chunk")
//     .unwrap()];
//     let chunk_data = vec![chunk_from_strings("LASt", "I am the last chunk")
//         .unwrap(), chunk_from_strings("miDl", "I am another chunk").unwrap()];

//     let chunk_bytes: Vec<u8> = chunk_data
//         .into_iter()
//         .flat_map(|chunk| chunk.as_bytes())
//         .collect();

//     let bytes: Vec<u8> = Png::STANDARD_HEADER
//         .iter()
//         .chain(chunk_bytes.iter())
//         .copied()
//         .collect();

//     let chunk = Png::try_from(bytes.as_ref());

// fn chunk_from_strings(chunk_type: &str, data: &str) -> Result<Chunk> {
//     let chunk_type = ChunkType::from_str(chunk_type)?;
//     let data: Vec<u8> = data.bytes().collect();

//     Ok(Chunk::new(chunk_type, data))
// }
