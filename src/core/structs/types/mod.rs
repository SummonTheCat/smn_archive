/*
    Allowing unused code to prevent warnings for functions that may not be used directly 
    in this module but are available when the crate is used as a library in other programs.
*/

#[allow(unused)]
pub mod types_id;
pub use types_id::*;

#[allow(unused)]
pub mod types_str;
pub use types_str::*;

#[allow(unused)]
pub mod types_misc;
pub use types_misc::*;

#[allow(unused)]
pub mod types_formtype;
pub use types_formtype::*;

#[allow(unused)]
pub mod types_vectors;
pub use types_vectors::*;

#[allow(unused)]
pub mod types_entinstance;
pub use types_entinstance::*;

#[allow(unused)]
pub mod types_color;
pub use types_color::*;