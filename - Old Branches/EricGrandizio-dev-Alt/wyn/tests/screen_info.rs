/*
 *  Crate: Wyn
 *   Test: Screen-Info
 */

//! This test collects a list of Displays/Monitors/Screens.
//! It then prints their names, coordinates, and sizes to the console.
//!
//! The user should verify this information is correct, and if it is, then the Test has PASSED.

mod utils;

// ================================================================================================================================ //

#[test]
#[ignore = "User Acceptance Test (Milestone 1, Story 8)"]
pub fn screen_info() {
    utils::timeout::test_deadline(5.0);
    test_main()
}

// ================================================================================================================================ //

fn test_main() {
    let app = TestApp::new();
    let events = EventLoop::new(&app).unwrap();
    let screens = Screen::collect(&events);

    println!("{} Screens connected.\n", screens.len());

    for (i, screen) in screens.iter().enumerate() {
        let name = screen.name(&events);
        let rect = screen.rect(&events);

        println!(
            "[SCREEN {i}]\n* Name: \"{}\"\n* Rect: [ Origin: ({}, {}), Size: ({}, {}) ]\n",
            name, rect.origin.x, rect.origin.y, rect.size.w, rect.size.h,
        );
    }

    {
        let screen = Screen::primary(&events);
        let name = screen.name(&events);
        let rect = screen.rect(&events);

        println!(
            "[PRIMARY SCREEN]\n* Name: \"{}\"\n* Rect: [ Origin: ({}, {}), Size: ({}, {}) ]\n",
            name, rect.origin.x, rect.origin.y, rect.size.w, rect.size.h,
        );
    }
}

// ================================================================================================================================ //

#[allow(unused_imports)]
use wyn::{errors::*, event_loop::*, events::*, inputs::*, screen::*, types::*, window::*, *};

// -------------------------------------------------------------------------------------------------------------------------------- //

struct TestApp {}

// -------------------------------------------------------------------------------------------------------------------------------- //

impl TestApp {
    pub fn new() -> Self {
        Self {}
    }
}

// -------------------------------------------------------------------------------------------------------------------------------- //

impl EventHandler for TestApp {}

// ================================================================================================================================ //
