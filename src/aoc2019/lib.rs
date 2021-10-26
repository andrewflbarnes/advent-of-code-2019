pub mod utils;
mod d1;
mod d2;

pub mod solutions {
    pub use crate::{
        d1::solve as d1,
        d2::solve as d2,
    };
}