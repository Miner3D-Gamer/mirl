/// None
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Hash, PartialOrd, Ord)]
pub enum SpecialDirections {
    #[allow(missing_docs)]
    #[default]
    None,
}
