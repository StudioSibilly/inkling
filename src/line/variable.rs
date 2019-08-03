//! Types of variables used in a story.

use crate::{
    error::{InklingError, InvalidAddressError},
    follow::FollowData,
    knot::{get_num_visited, Address, KnotSet, ValidateAddresses},
};

#[derive(Clone, Debug, PartialEq)]
/// Variables in a story.
///
/// Not all of these will evaluate to a string when used as a variable. Numbers and strings
/// make perfect sense to print: a divert to another location, not as much.
///
/// Variables which cannot be printed will raise errors when used as such.
pub enum Variable {
    /// Address to stitch, evaluates to the number of times it has been visited.
    Address(Address),
    /// True or false, evaluates to 1 for true and 0 for false.
    Bool(bool),
    /// Divert to another address, *cannot be printed*.
    Divert(Address),
    /// Decimal number.
    Float(f32),
    /// Integer number.
    Int(i32),
    /// Text string.
    String(String),
}

impl Variable {
    /// Return a string representation of the variable.
    pub fn to_string(&self, data: &FollowData) -> Result<String, InklingError> {
        match &self {
            Variable::Address(address) => {
                let num_visited = get_num_visited(address, data)?;
                Ok(format!("{}", num_visited))
            }
            Variable::Bool(value) => Ok(format!("{}", *value as u8)),
            Variable::Divert(..) => Err(InklingError::PrintInvalidVariable {
                name: String::new(),
                value: self.clone(),
            }),
            Variable::Float(value) => Ok(format!("{}", value)),
            Variable::Int(value) => Ok(format!("{}", value)),
            Variable::String(content) => Ok(content.clone()),
        }
    }
}

impl ValidateAddresses for Variable {
    fn validate(
        &mut self,
        current_address: &Address,
        knots: &KnotSet,
    ) -> Result<(), InvalidAddressError> {
        match self {
            Variable::Address(address) | Variable::Divert(address) => {
                address.validate(current_address, knots)
            }
            Variable::Bool(..) | Variable::Float(..) | Variable::Int(..) | Variable::String(..) => {
                Ok(())
            }
        }
    }

    #[cfg(test)]
    fn all_addresses_are_valid(&self) -> bool {
        match self {
            Variable::Address(address) | Variable::Divert(address) => {
                address.all_addresses_are_valid()
            }
            Variable::Bool(..) | Variable::Float(..) | Variable::Int(..) | Variable::String(..) => {
                true
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::collections::HashMap;

    fn mock_follow_data(knots: &[(&str, &str, u32)]) -> FollowData {
        let mut knot_visit_counts = HashMap::new();

        for (knot, stitch, num_visited) in knots {
            let mut stitch_count = HashMap::new();
            stitch_count.insert(stitch.to_string(), *num_visited);

            knot_visit_counts.insert(knot.to_string(), stitch_count);
        }

        FollowData {
            knot_visit_counts,
            variables: HashMap::new(),
        }
    }

    #[test]
    fn booleans_are_printed_as_numbers() {
        let data = mock_follow_data(&[]);

        assert_eq!(&Variable::Bool(true).to_string(&data).unwrap(), "1");
        assert_eq!(&Variable::Bool(false).to_string(&data).unwrap(), "0");
    }

    #[test]
    fn numbers_can_be_printed() {
        let data = mock_follow_data(&[]);

        assert_eq!(&Variable::Int(5).to_string(&data).unwrap(), "5");
        assert_eq!(&Variable::Float(1.0).to_string(&data).unwrap(), "1");
        assert_eq!(&Variable::Float(1.35).to_string(&data).unwrap(), "1.35");
        assert_eq!(
            &Variable::Float(1.0000000003).to_string(&data).unwrap(),
            "1"
        );
    }

    #[test]
    fn strings_are_just_cloned() {
        let data = mock_follow_data(&[]);

        assert_eq!(
            &Variable::String("two words".to_string())
                .to_string(&data)
                .unwrap(),
            "two words"
        );
    }

    #[test]
    fn addresses_are_printed_as_their_number_of_visits() {
        let data = mock_follow_data(&[("tripoli", "cinema", 0), ("addis_ababa", "with_family", 3)]);

        let tripoli = Address::from_parts_unchecked("tripoli", Some("cinema"));
        let addis_ababa = Address::from_parts_unchecked("addis_ababa", Some("with_family"));

        assert_eq!(&Variable::Address(tripoli).to_string(&data).unwrap(), "0");
        assert_eq!(
            &Variable::Address(addis_ababa).to_string(&data).unwrap(),
            "3"
        );
    }

    #[test]
    fn diverts_cannot_be_printed_but_yield_error() {
        let data = mock_follow_data(&[]);
        let address = Address::from_parts_unchecked("tripoli", Some("cinema"));

        assert!(Variable::Divert(address).to_string(&data).is_err());
    }
}
