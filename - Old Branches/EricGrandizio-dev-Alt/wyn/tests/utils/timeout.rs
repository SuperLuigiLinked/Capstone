/*
 *  Crate: Wyn
 * Module: Tests - Utils - Timeout
 */

//! This module provides a helper-function to fail tests after a given deadline.
//!
//! Rust Tests will run until completion.
//! If they run for over a certain time threshold, they will print a warning, but not terminate.
//!
//! Given the design of the Wyn Library, if something goes wrong, it likely results in a deadlock.
//! The library needs access to the Main Thread, so this module spawns a separate watchdog thread to manage timeouts.

// ================================================================================================================================ //

/// Spawns a thread that sleeps for the given duration.
/// Once the deadline is reached, it will kill the running process.
#[allow(unused)]
pub fn test_deadline(secs: f64) {
    // Spawn a new thread to watch the time.
    std::thread::spawn(move || {
        // Convert from seconds to a Duration.
        let dur = std::time::Duration::from_secs_f64(secs);

        // Sleep for the provided deadline period.
        std::thread::sleep(dur);

        // If the test is still running, then abort the process.
        std::process::exit(1);
    });
}

// ================================================================================================================================ //
