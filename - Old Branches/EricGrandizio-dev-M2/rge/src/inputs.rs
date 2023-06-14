/*
 *  Crate: RGE
 * Module: Inputs
 */

//! Functionality for handling Inputs.

// ================================================================================================================================ //

#[allow(unused_imports)]
use super::*;

pub use wyn::inputs::*;
pub use wyn::types::{Coord, Extent, Point, Rect, Size};

// ================================================================================================================================ //

/// A variant of all possible input-types.
#[derive(Clone, Copy, PartialEq)]
pub enum Input {
    /// The Window's focus has changed.
    WindowFocus {
        /// Focused or Unfocused flag.
        focused: bool,
    },
    /// A Mouse-Cursor was moved in the Window.
    CursorMove {
        /// The position of the cursor, in pixel-coordinates.
        point: Point,
    },
    /// A Scroll-Wheel was scrolled in the Window.
    ScrollWheel {
        /// The horizontal scroll units.
        delta_x: f64,
        /// The vertical scroll units.
        delta_y: f64,
    },
    /// A Mouse-Button was pressed/released in the Window.
    ButtonPress {
        /// The button-code.
        button: MouseButton,
        /// Pressed or Released flag.
        pressed: bool,
    },
    /// A Keyboard-Key was pressed/released in the Window.
    KeyPress {
        /// The key-code.
        keycode: KeyCode,
        /// Pressed or Release flag.
        pressed: bool,
    },
    /// A Character was input in the Window.
    CharacterInput {
        /// The character-code.
        character: char,
    },
}

/// A collection of Inputs.
#[derive(Clone, Default)]
pub struct Inputs {
    /// The Input Events, in order of occurrence.
    pub events: Vec<Input>,

    /// The XInput Controllers.
    pub xinput: XInputControllers,

    /// Information about the connected Screens.
    pub screens: Vec<ScreenInfo>,
}

impl Inputs {
    /// Returns whether or not the Input is in the list.
    pub fn contains(&self, input: Input) -> bool {
        self.events.contains(&input)
    }

    /// Counts how many times the Input occurred in the list.
    pub fn count(&self, input: Input) -> usize {
        self.events.iter().filter(|event| **event == input).count()
    }
}

// ================================================================================================================================ //
