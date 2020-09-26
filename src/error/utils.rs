//! Utilities for printing and handling errors.

use std::fmt;

#[cfg(feature = "serde_support")]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde_support", derive(Deserialize, Serialize))]
/// Information about the origin of an item.
///
/// To be used to present errors when during parsing or runtime, allowing access to where
/// the error originated from.
pub struct MetaData {
    /// Which line in the original story the item originated from.
    pub(crate) line_index: u32,
}

impl fmt::Display for MetaData {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "line {}", self.line())
    }
}

impl MetaData {
    /// Get the line number of the corresponding data in a file.
    ///
    /// # Indexing
    /// Line numbers start from 1.
    pub fn line(&self) -> u32 {
        self.line_index + 1
    }
}

/// Write meta data information for a line or piece of content in a story.
pub(crate) fn write_line_information<W: fmt::Write>(
    buffer: &mut W,
    meta_data: &MetaData,
) -> fmt::Result {
    write!(buffer, "({}) ", meta_data)
}

/// Wrapper to implement From for variants when the variant is simply encapsulated
/// in the enum.
///
/// # Example
/// Running
/// ```
/// impl_from_error[
///     MyError,
///     [Variant, ErrorData]
/// ];
/// ```
/// is identical to running
/// ```
/// impl From<ErrorData> for MyError {
///     from(err: ErrorData) -> Self {
///         Self::Variant(err)
///     }
/// }
/// ```
/// The macro can also implement several variants at once:
/// ```
/// impl_from_error[
///     MyError,
///     [Variant1, ErrorData1],
///     [Variant2, ErrorData2]
/// ];
/// ```
macro_rules! impl_from_error {
    ($for_type:ident; $([$variant:ident, $from_type:ident]),+) => {
        $(
            impl From<$from_type> for $for_type {
                fn from(err: $from_type) -> Self {
                    $for_type::$variant(err)
                }
            }
        )*
    }
}

impl From<usize> for MetaData {
    fn from(line_index: usize) -> Self {
        MetaData {
            line_index: line_index as u32,
        }
    }
}

#[cfg(test)]
impl From<()> for MetaData {
    fn from(_: ()) -> Self {
        MetaData { line_index: 0 }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn meta_data_from_index_sets_index() {
        assert_eq!(MetaData::from(6), MetaData { line_index: 6 });
    }

    #[test]
    fn meta_data_line_number_starts_from_one() {
        assert_eq!(MetaData::from(6).line(), 7);
    }
}
