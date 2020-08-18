//! Partial implementation of the *Ink* markup language for game dialogue.
//!
//! Ink is a creation of [Inkle](https://www.inklestudios.com/). For more information
//! about the language, [see their website](https://www.inklestudios.com/ink/).
//!
//! # Why `inkling`?
//! *   Simple interface for walking through branching stories or dialog trees
//! *   Designed to slot into an external framework: like Inkle's implementation this
//!     is not a stand alone game engine, just a processor that will feed the story
//!     text and choices to the user
//! *   Rust native, no wrestling with Unity or C# integration
//! *   Support for non-latin alphabets in identifiers
//! *   Few dependencies: currently only `serde` as an optional dependency
//!     to de/serialize stories, probably `rand` as maybe-optional in the future.
//!
//! # Why not `inkling`?
//! *   Fewer features than Inkle's implementation of the language
//! *   Untested in serious work loads and large scripts
//! *   Not even alpha status, what is this???
//!
//! # Usage example
//!
//! ## A short story
//! This example shows the basics of using `inkling`.
//!
//! ### Parsing the story
//! ```
//! use inkling::read_story_from_string;
//!
//! // Imagine that this is a complete `Ink` story!
//!
//! let story_content = "\
//! Hello, World!
//! *   Hello[ back!] right back at you!
//! ";
//!
//! let mut story = read_story_from_string(story_content).unwrap();
//! ```
//!
//! ### Starting the story processor
//! ```
//! # use inkling::read_story_from_string;
//! # let story_content = "Hello, World!\n*Hello[ back!] right back at you!";
//! # let mut story = read_story_from_string(story_content).unwrap();
//! // We will supply a buffer for the story to read content into
//! let mut line_buffer = Vec::new();
//!
//! // Mark the story as being prepared by calling `start`
//! story.start().unwrap();
//!
//! // Begin the story processing: it will return once it encounters the branching choice
//! let result = story.resume(&mut line_buffer).unwrap();
//!
//! assert_eq!(&line_buffer[0].text, "Hello, World!\n");
//! ```
//!
//!
//! ### Accessing the content of a presented choice
//! ```
//! # use inkling::read_story_from_string;
//! # let story_content = "Hello, World!\n*Hello[ back!] right back at you!";
//! # let mut story = read_story_from_string(story_content).unwrap();
//! # let mut line_buffer = Vec::new();
//! # story.start().unwrap();
//! # let result = story.resume(&mut line_buffer).unwrap();
//! use inkling::Prompt;
//!
//! // The story provided us with the choice in the result
//! let choice = match result {
//!     Prompt::Choice(choices) => choices[0].clone(),
//!     _ => unreachable!(),
//! };
//!
//! assert_eq!(&choice.text, "Hello back!");
//! ```
//!
//! ### Resuming with a selected choice
//! ```
//! # use inkling::{read_story_from_string, Prompt};
//! # let story_content = "Hello, World!\n*Hello[ back!] right back at you!";
//! # let mut story = read_story_from_string(story_content).unwrap();
//! # let mut line_buffer = Vec::new();
//! # story.start().unwrap();
//! # let result = story.resume(&mut line_buffer).unwrap();
//! // Resume by supplying a selected choice index and calling `resume`
//! story.make_choice(0).unwrap();
//!
//! match story.resume(&mut line_buffer).unwrap() {
//!     Prompt::Done => (),
//!     _ => unreachable!(),
//! }
//!
//! // The choice text has now been added to the buffer
//! assert_eq!(&line_buffer[1].text, "Hello right back at you!\n");
//! ```
//!
//! ## Story loop
//! The idea is that story processing should be as simple as this loop:
//!
//! ```
//! # use inkling::{read_story_from_string, Prompt};
//! # let mut story = read_story_from_string("Line").unwrap();
//! # let mut line_buffer = Vec::new();
//! # story.start().unwrap();
//! while let Ok(Prompt::Choice(choices)) = story.resume(&mut line_buffer) {
//!     // Present story text to user, then have them select a choice
//!     # break;
//! }
//! ```
//!
//! An example of a simple command line story player is provided in the examples.
//! Run `cargo run --example player` to try it out and browse the source code to see the
//! implementation.
//!
//! # Features
//! A complete recreation of all features in the Ink language is beyond the scope
//! of this project (and my capabilities as a programmer).
//!
//! Currently the processor supports:
//!
//! *   Structure:  Knots, stitches, nested branching choices, gathers, diverts,
//!                 tags for knots and story
//! *   Choices:    Non-sticky, sticky, fallback, line variations, conditions
//! *   Lines:      Plain text, diverts, tags, conditions, alternative sequences (all
//!                 except shuffle)
//! *   Conditions: Nested, `and`/`or` linking, can check against variables
//! *   Reading:    Address validation for diverts and conditions. Conditions and expressions
//!                 are validated after parsing the story.
//! *   Variables:  Used as text in sentences and in conditions, can modify from calling program
//! *   Mathematics: In line text, using numbers, parenthesis and variables for all numerical
//!                  calculations. Strings can be concatenated using the `+` operator.
//!
//! Hopefully coming:
//!
//! *   Structure:  Multi-line blocks, labels
//! *   Variables:  Modify in the Ink script
//! *   Reading:    Include statements in files
//!
//! Unlikely features:
//!
//! *   Structure:  Threads (maybe?) and tunnels
//! *   Program:    Defining functions in the Ink story file, "advanced state tracking,"
//!                 calling Rust functions from the script to get variables
//!
//! # De/serializing stories
//! Enable the `serde_support` feature to derive `Deserialize` and `Serialize` for all
//! required objects. If you are unfamiliar with `serde`, this corresponds to reading
//! and writing finished story files in their current state. In game terms: saving
//! and loading.
//!
//! For more information about `serde` see their [website](https://serde.rs/).
//!
//! # Contributions
//! I am a complete novice at designing frameworks which will fit into larger schemes.
//! As such I have no real idea of best practices for interacting with an engine like this.
//! If you have a suggestion for how to make it easier for a user to run the processor
//! I would be very glad to hear it!
//!
//! Likewise, contributions are welcome. Please open an issue on
//! [Github](https://github.com/pjohansson/inkling) to discuss improvements or submit
//! a pull request.

mod consts;
pub mod error;
mod follow;
mod knot;
mod line;
mod node;
mod process;
mod story;
mod utils;

pub use error::InklingError;
pub use line::Variable;
pub use story::{
    copy_lines_into_string, read_story_from_string, Choice, Line, LineBuffer, Prompt, Story,
};
