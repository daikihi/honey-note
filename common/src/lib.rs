pub mod errors;

#[cfg(not(target_arch = "wasm32"))]
pub mod infrastructure;

#[cfg(not(target_arch = "wasm32"))]
pub mod repository;

pub mod libs;
