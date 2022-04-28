#![doc = include_str!("../README.md")]

pub use parity_wasm::elements::CustomSection;
use parity_wasm::{elements::Serialize, *};
use thiserror::Error;

///Deserialization/serialization error
#[derive(Error, Debug)]
#[error(transparent)]
pub struct Error(#[from] elements::Error);

/// Represents WebAssembly module. Use new to build from buffer.
pub struct Module(elements::Module);

impl Module {
    /// Creates a Module from buffer.
    pub fn new(buf: &[u8]) -> Result<Self, Error> {
        Ok(Module(elements::Module::from_bytes(buf)?))
    }

    /// Returns an iterator over the module’s custom sections.
    pub fn custom_sections(&self) -> impl Iterator<Item = &elements::CustomSection> {
        self.0.custom_sections()
    }

    /// Sets the payload associated with the given custom section, or adds a new custom section, as appropriate.
    pub fn add_custom_section(&mut self, name: impl Into<String>, payload: Vec<u8>) {
        self.0.set_custom_section(name, payload)
    }

    /// Removes the given custom section, if it exists. Returns the removed section if it existed, or None otherwise.
    pub fn clear_custom_section(&mut self, name: impl AsRef<str>) -> Option<CustomSection> {
        self.0.clear_custom_section(name)
    }

    pub fn into_buffer(self) -> Result<Vec<u8>, Error> {
        let mut v = Vec::new();
        self.0.serialize(&mut v)?;
        Ok(v)
    }
}
