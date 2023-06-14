/*
 *  Crate: Wyn
 *   Test: Reposition
 */

//! This test ensures window-resizing functionality works as intended.

mod utils;

// ================================================================================================================================ //

#[test]
pub fn reposition() {
    utils::timeout::test_deadline(5.0);
    test_main();
}

// ================================================================================================================================ //

fn test_main() {
    let app = TestApp::new();
    let events = EventLoop::new(&app).unwrap();
    events.run();
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

impl EventHandler for TestApp {
    fn start(&self, events: &EventLoop) {
        let window = Window::open(events).unwrap();

        // Change Content-Rectangle.
        {
            let origin = Point::new(5.0, 6.0);
            let size = Size::new(256.0, 240.0);
            let rect = Rect { origin, size };
            window.reposition_content(events, rect).unwrap();
            let res = window.content_rect(events).unwrap();
            assert_eq!(rect, res);
        }

        // Change Border-Rectangle.
        {
            let rect = Rect::new(65.0, 135.0, 640.0, 480.0);
            window.reposition_border(events, rect).unwrap();
            let res = window.border_rect(events).unwrap();
            assert_eq!(rect, res);
        }

        // Conversions
        {
            let pt1 = Point::new(1.0, 2.0);
            let nt_pt = NativePoint::from(pt1);
            let pt2 = Point::from(nt_pt);
            assert_eq!(pt1.clone(), pt2);

            let sz1 = Size::new(3.0, 4.0);
            let nt_sz = NativeSize::from(sz1);
            let sz2 = Size::from(nt_sz);
            assert_eq!(sz1.clone(), sz2);

            let rc1 = Rect::new(5.0, 6.0, 7.0, 8.0);
            let nt_rc = NativeRect::from(rc1);
            let rc2 = Rect::from(nt_rc);
            assert_eq!(rc1.clone(), rc2);

            let _ = format!("{pt2:?} {sz2:?} {rc2:?}");
        }

        window.close(events).unwrap();
        events.request_stop();
    }
}

// ================================================================================================================================ //
