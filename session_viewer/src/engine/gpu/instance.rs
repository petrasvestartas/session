mod layout;
#[cfg(test)]
mod validation;
pub use layout::Instance;
#[cfg(test)]
pub(crate) use validation::wgsl_fields;
