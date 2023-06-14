/*
 *  Crate: RGE
 * Module: Timer
 */

//! Timer that allows syncing to a particular Framerate by sleeping.

// ================================================================================================================================ //

#[allow(unused_imports)]
use super::*;

use std::time::{Duration, Instant};

// ================================================================================================================================ //

/// A waitable timer that syncs to a desired Frames-Per-Second.
pub struct FrameTimer {
    /// Whether or not the Timer should VSYNC.
    vsync: bool,

    /// The desired Frames-Per-Second to target.
    fps: f64,

    /// The Instant the Timer was started.
    epoch: Instant,

    /// The last recorded Instant.
    tick: Instant,
}

// ================================================================================================================================ //

impl FrameTimer {
    // ---------------------------------------------------------------- //

    /// Returns whether or not the FPS value is Valid.
    fn valid_fps(fps: f64) -> bool {
        fps.is_finite() && fps.is_sign_positive()
    }

    /// Constructs a new Frame-Timer using the given FPS and VSYNC settings.
    pub(crate) fn new(fps: f64, vsync: bool) -> Self {
        assert!(Self::valid_fps(fps));

        self::init_os_timer();

        let epoch = Instant::now();
        let tick = epoch;

        Self {
            fps,
            epoch,
            tick,
            vsync,
        }
    }

    /// Resets the timer, using the new epoch.
    #[allow(unused)]
    pub fn reset_epoch(&mut self, instant: Instant) {
        self.epoch = instant;
        self.tick = instant;
    }

    /// Resets the timer, using the new FPS.
    #[allow(unused)]
    pub fn reset_fps(&mut self, fps: f64) {
        assert!(Self::valid_fps(fps));

        self.fps = fps;
    }

    /// Enables/Disables VSYNC.
    #[allow(unused)]
    pub fn toggle_vsync(&mut self, vsync: bool) {
        self.vsync = vsync;
    }

    // ---------------------------------------------------------------- //

    /// Returns the targeted Frames-Per-Second.
    pub fn fps(&self) -> f64 {
        self.fps
    }

    /// Returns the Instant the Frame-Timer was started.
    pub fn epoch(&self) -> Instant {
        self.epoch
    }

    /// Returns whether or not VSYNC is enabled.
    pub fn vsync(&self) -> bool {
        self.vsync
    }

    /// Returns `true` if the framerate is uncapped.
    pub fn unbounded(&self) -> bool {
        self.fps == 0.0
    }

    /// Returns `true` if the framerate is limited by VSYNC.
    pub fn vsync_bounded(&self) -> bool {
        self.unbounded() && self.vsync()
    }

    // ---------------------------------------------------------------- //

    /// Returns the Elapsed time since the Frame-Timer was started.
    pub fn elapsed(&self) -> Duration {
        Instant::now().saturating_duration_since(self.epoch)
    }

    /// Returns the Elapsed seconds since the Frame-Timer was started.
    pub fn elapsed_seconds(&self) -> f64 {
        self.elapsed().as_secs_f64()
    }

    /// Returns the Elapsed frames since the Frame-Timer was started.
    pub fn elapsed_frames(&self) -> f64 {
        if self.unbounded() {
            f64::INFINITY
        } else {
            self.elapsed_seconds() * self.fps
        }
    }

    // ---------------------------------------------------------------- //

    /// Returns the last recorded Instant.
    pub fn last_tick(&self) -> Instant {
        self.tick
    }

    /// Returns the elapsed Duration from the Last Tick since the Epoch.
    pub fn last_elapsed(&self) -> Duration {
        self.last_tick().saturating_duration_since(self.epoch)
    }

    /// Returns the elapsed Seconds from the Last Tick since the Epoch.
    pub fn last_seconds(&self) -> f64 {
        self.last_elapsed().as_secs_f64()
    }

    /// Returns the elapsed Frames from the Last Tick since the Epoch.
    pub fn last_frames(&self) -> f64 {
        if self.unbounded() {
            f64::INFINITY
        } else {
            self.last_seconds() * self.fps
        }
    }

    // ---------------------------------------------------------------- //

    /// Returns the elapsed Frames from the start of the Next Frame since the Epoch.
    pub fn next_frames(&self) -> f64 {
        if self.unbounded() {
            self.elapsed_frames()
        } else {
            self.last_frames().floor() + 1.0
        }
    }

    /// Returns the elapsed Seconds from the start of the Next Frame since the Epoch.
    pub fn next_seconds(&self) -> f64 {
        if self.unbounded() {
            self.elapsed_seconds()
        } else {
            self.next_frames() / self.fps()
        }
    }

    /// Returns the elapsed Duration from the start of the Next Frame since the Epoch.
    pub fn next_elapsed(&self) -> Duration {
        if self.unbounded() {
            self.elapsed()
        } else {
            Duration::from_secs_f64(self.next_seconds())
        }
    }

    /// Returns the Instant of the start of the Next Frame.
    pub fn next_tick(&self) -> Instant {
        if self.unbounded() {
            Instant::now()
        } else {
            self.epoch + self.next_elapsed()
        }
    }

    // ---------------------------------------------------------------- //

    /// Records the Last-Tick.
    pub(crate) fn update(&mut self) {
        self.tick = Instant::now();
    }

    /// Sleeps the current thread until the after the Next Frame has started.
    pub(crate) fn sync(tick: Instant) {
        let rem = tick.checked_duration_since(Instant::now());

        if let Some(dur) = rem {
            std::thread::sleep(dur);
        } else {
            std::thread::yield_now();
        }
    }

    // ---------------------------------------------------------------- //
}

impl Drop for FrameTimer {
    fn drop(&mut self) {
        self::cleanup_os_timer();
    }
}

// ================================================================================================================================ //

/// Initializes any OS-related state for proper Timer functionality.
fn init_os_timer() {
    #[cfg(target_os = "windows")]
    {
        let mut caps = unsafe { core::mem::zeroed() };
        let caps_size = size_of::<sys::TIMECAPS>() as u32;

        let res = unsafe { sys::timeGetDevCaps(addr_of_mut!(caps), caps_size) };
        assert_eq!(res, sys::MMSYSERR_NOERROR);

        let res = unsafe { sys::timeBeginPeriod(caps.wPeriodMin) };
        assert_eq!(res, sys::TIMERR_NOERROR);
    }
}

/// Cleans up any OS-related state for proper Timer functionality.
fn cleanup_os_timer() {
    #[cfg(target_os = "windows")]
    {
        let mut caps = unsafe { core::mem::zeroed() };
        let caps_size = size_of::<sys::TIMECAPS>() as u32;

        let res = unsafe { sys::timeGetDevCaps(addr_of_mut!(caps), caps_size) };
        assert_eq!(res, sys::MMSYSERR_NOERROR);

        let res = unsafe { sys::timeEndPeriod(caps.wPeriodMin) };
        assert_eq!(res, sys::TIMERR_NOERROR);
    }
}

// ================================================================================================================================ //
