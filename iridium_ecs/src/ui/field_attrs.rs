use std::str::FromStr;

/// The attributes a component field has.
///
/// Map is attribute name -> attribute value.
#[derive(Default)]
pub struct InspectorUiFieldAttributes {
    attrs: std::collections::HashMap<&'static str, &'static str>,
}

impl InspectorUiFieldAttributes {
    /// Create a new `InspectorUiFieldAttributes` from the already parsed attributes.
    #[must_use]
    pub const fn from_inner(attrs: std::collections::HashMap<&'static str, &'static str>) -> Self {
        Self { attrs }
    }

    /// Gets a value from the attributes of a given type.
    ///
    /// Returns `None` if the attribute is not present or the value fails to parse to T.
    #[must_use]
    pub fn get<T: FromStr>(&self, name: &'static str) -> Option<T> {
        let value = self.attrs.get(name)?;
        value.parse().ok()
    }
}
