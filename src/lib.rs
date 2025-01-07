//If rust plugin feature is enabled, the following code will be compiled
#[cfg(feature = "rust_bridge")]
pub mod rust;

#[cfg(feature = "c_bridge")]
pub mod c;
