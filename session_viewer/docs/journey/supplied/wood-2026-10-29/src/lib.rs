pub mod proto {
    include!("proto/wood_proto.rs");
}

pub mod beam;
pub mod beam_variable;
pub mod block;
pub mod column;
pub mod element;
pub mod geometry;
pub mod plate;
pub mod support;

pub use beam::Beam;
pub use beam_variable::BeamVariable;
pub use block::Block;
pub use column::Column;
pub use element::WoodElement;
pub use plate::Plate;
pub use support::Support;
