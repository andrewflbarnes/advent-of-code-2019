mod d1;
mod d2;
mod d3;
mod d4;
mod d5;

pub mod utils;
pub mod vm;

pub mod solutions {
    pub use crate::{d1::solve as d1, d2::solve as d2, d3::solve as d3, d4::solve as d4, d5::solve as d5};
}
