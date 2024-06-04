#[cfg(feature = "derive")]
extern crate typeon_derive;

#[cfg(feature = "derive")]
pub use typeon_derive::*;

pub trait TypeInfo: 'static + Sync + Send {
    const TYPE_NAME: &'static str;
    fn type_name(&self) -> &'static str {
        Self::TYPE_NAME
    }
}
