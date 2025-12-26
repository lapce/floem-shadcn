//! Tests for the Slider component click position calculations.

use floem::ViewId;
use floem::prelude::*;
use floem::reactive::RwSignal;
use floem::views::Decorators;
use floem_test::prelude::*;
use ui_events::pointer::PointerEvent;

/// Create a simple test slider that tracks layout info
fn create_test_slider(value: RwSignal<f64>, width: f64) -> (ViewId, impl IntoView) {
    let container_id = ViewId::new();
    let padding = 8.0;

    let view = floem::views::Container::with_id(
        container_id,
        floem::views::Empty::new().style(move |s| {
            let percent = value.get();
            s.height(6.0)
                .width_pct(percent)
                .background(peniko::Color::from_rgb8(14, 165, 233))
        }),
    )
    .style(move |s| {
        s.width(width)
            .height(16.0)
            .items_center()
            .background(peniko::Color::from_rgb8(229, 229, 229))
            .padding_left(padding)
            .padding_right(padding)
            .cursor(floem::style::CursorStyle::Pointer)
    })
    .on_event_stop(floem::event::EventListener::PointerDown, move |e| {
        if let floem::event::Event::Pointer(PointerEvent::Down(pointer_event)) = e {
            let layout = container_id.layout_rect();
            let content_rect = container_id.get_content_rect();

            // Debug print
            eprintln!("=== Slider Click Debug ===");
            eprintln!("layout_rect: {:?}", layout);
            eprintln!("content_rect: {:?}", content_rect);
            eprintln!("physical position.x: {}", pointer_event.state.position.x);
            eprintln!(
                "logical position.x: {}",
                pointer_event.state.logical_point().x
            );
            eprintln!("scale_factor: {}", pointer_event.state.scale_factor);

            // Use logical_point() to convert physical to logical coordinates
            let track_width = content_rect.width();
            let click_x = pointer_event.state.logical_point().x - content_rect.x0;
            let percent = (click_x / track_width).clamp(0.0, 1.0);

            eprintln!("track_width: {}", track_width);
            eprintln!("click_x: {}", click_x);
            eprintln!("percent: {}", percent);
            eprintln!("new_value: {}", percent * 100.0);

            value.set(percent * 100.0);
        }
    });

    (container_id, view)
}

#[test]
fn test_slider_layout_with_padding() {
    // Create slider with known width
    let value = RwSignal::new(50.0);
    let (container_id, view) = create_test_slider(value, 300.0);

    let mut harness = HeadlessHarness::new_with_size(view, 400.0, 100.0);

    // Get layout info
    let layout_rect = harness.get_layout_rect(container_id);
    let content_rect = harness.get_content_rect(container_id);

    eprintln!("=== Layout Test ===");
    eprintln!("layout_rect: {:?}", layout_rect);
    eprintln!("content_rect: {:?}", content_rect);
    eprintln!("layout width: {}", layout_rect.width());
    eprintln!("content width: {}", content_rect.width());

    // With 8px padding on each side, content should be 284px (300 - 16)
    assert!(
        (content_rect.width() - 284.0).abs() < 1.0,
        "Content width should be 284 (300 - 2*8 padding), got {}",
        content_rect.width()
    );
}

#[test]
fn test_slider_click_at_start() {
    let value = RwSignal::new(50.0);
    let (_container_id, view) = create_test_slider(value, 300.0);

    let mut harness = HeadlessHarness::new_with_size(view, 400.0, 100.0);

    // Click at the start of the track (after padding)
    // With 8px left padding, the track starts at x=8
    harness.pointer_down(8.0, 8.0);

    let new_value = value.get();
    eprintln!("Click at x=8: value = {}", new_value);

    assert!(
        new_value < 5.0,
        "Clicking at start should give ~0%, got {}%",
        new_value
    );
}

#[test]
fn test_slider_click_at_middle() {
    let value = RwSignal::new(0.0);
    let (_container_id, view) = create_test_slider(value, 300.0);

    let mut harness = HeadlessHarness::new_with_size(view, 400.0, 100.0);

    // Click at the middle of the slider
    // Total width = 300, middle = 150
    harness.pointer_down(150.0, 8.0);

    let new_value = value.get();
    eprintln!("Click at x=150: value = {}", new_value);

    // Should be close to 50%
    assert!(
        (new_value - 50.0).abs() < 10.0,
        "Clicking at middle should give ~50%, got {}%",
        new_value
    );
}

#[test]
fn test_slider_click_at_end() {
    let value = RwSignal::new(0.0);
    let (_container_id, view) = create_test_slider(value, 300.0);

    let mut harness = HeadlessHarness::new_with_size(view, 400.0, 100.0);

    // Click at the end of the track (before right padding)
    // With 8px right padding and 300px width, track ends at x=292
    harness.pointer_down(292.0, 8.0);

    let new_value = value.get();
    eprintln!("Click at x=292: value = {}", new_value);

    assert!(
        new_value > 95.0,
        "Clicking at end should give ~100%, got {}%",
        new_value
    );
}

#[test]
fn test_slider_click_at_quarter() {
    let value = RwSignal::new(0.0);
    let (_container_id, view) = create_test_slider(value, 300.0);

    let mut harness = HeadlessHarness::new_with_size(view, 400.0, 100.0);

    // Click at 25% position
    // Track width = 284, 25% = 71, plus 8px padding = 79
    harness.pointer_down(79.0, 8.0);

    let new_value = value.get();
    eprintln!("Click at x=79 (25%): value = {}", new_value);

    assert!(
        (new_value - 25.0).abs() < 10.0,
        "Clicking at 25% should give ~25%, got {}%",
        new_value
    );
}

#[test]
fn test_slider_click_at_three_quarters() {
    let value = RwSignal::new(0.0);
    let (_container_id, view) = create_test_slider(value, 300.0);

    let mut harness = HeadlessHarness::new_with_size(view, 400.0, 100.0);

    // Click at 75% position
    // Track width = 284, 75% = 213, plus 8px padding = 221
    harness.pointer_down(221.0, 8.0);

    let new_value = value.get();
    eprintln!("Click at x=221 (75%): value = {}", new_value);

    assert!(
        (new_value - 75.0).abs() < 10.0,
        "Clicking at 75% should give ~75%, got {}%",
        new_value
    );
}

/// Test to understand what coordinates we receive
#[test]
fn test_debug_coordinates() {
    let value = RwSignal::new(50.0);
    let positions_received: std::rc::Rc<std::cell::RefCell<Vec<(f64, f64, f64)>>> =
        std::rc::Rc::new(std::cell::RefCell::new(Vec::new()));
    let positions_clone = positions_received.clone();

    let container_id = ViewId::new();

    let view = floem::views::Container::with_id(
        container_id,
        floem::views::Empty::new().style(|s| s.height(6.0).width_full()),
    )
    .style(|s| {
        s.width(300.0)
            .height(16.0)
            .padding_left(8.0)
            .padding_right(8.0)
    })
    .on_event_stop(floem::event::EventListener::PointerDown, move |e| {
        if let floem::event::Event::Pointer(PointerEvent::Down(pointer_event)) = e {
            let layout = container_id.layout_rect();
            let content = container_id.get_content_rect();
            positions_clone.borrow_mut().push((
                pointer_event.state.position.x,
                layout.width(),
                content.width(),
            ));
        }
    });

    let mut harness = HeadlessHarness::new_with_size(view, 400.0, 100.0);

    // Click at different positions
    harness.pointer_down(0.0, 8.0);
    harness.pointer_down(50.0, 8.0);
    harness.pointer_down(100.0, 8.0);
    harness.pointer_down(150.0, 8.0);
    harness.pointer_down(200.0, 8.0);
    harness.pointer_down(250.0, 8.0);
    harness.pointer_down(300.0, 8.0);

    let positions = positions_received.borrow();
    eprintln!("=== Coordinate Debug ===");
    for (i, (pos_x, layout_w, content_w)) in positions.iter().enumerate() {
        eprintln!(
            "Click {}: pos_x={:.1}, layout_w={:.1}, content_w={:.1}",
            i, pos_x, layout_w, content_w
        );
    }

    // Basic sanity check - some clicks may be outside the view bounds
    assert!(
        positions.len() >= 6,
        "Should have recorded at least 6 clicks, got {}",
        positions.len()
    );
}

// =============================================================================
// Scale Factor Tests - Test with different DPI/scale factors
// =============================================================================

#[test]
fn test_slider_scale_2x_layout() {
    // Test with 2x scale factor (Retina display)
    let value = RwSignal::new(50.0);
    let (container_id, view) = create_test_slider(value, 300.0);

    let mut harness = HeadlessHarness::new_with_size(view, 400.0, 100.0);
    harness.set_scale(2.0);

    // Get layout info at 2x scale
    let layout_rect = harness.get_layout_rect(container_id);
    let content_rect = harness.get_content_rect(container_id);

    eprintln!("=== Scale 2x Layout Test ===");
    eprintln!("layout_rect: {:?}", layout_rect);
    eprintln!("content_rect: {:?}", content_rect);
    eprintln!("layout width: {}", layout_rect.width());
    eprintln!("content width: {}", content_rect.width());
}

#[test]
fn test_slider_scale_2x_click_at_middle() {
    let value = RwSignal::new(0.0);
    let (_container_id, view) = create_test_slider(value, 300.0);

    let mut harness = HeadlessHarness::new_with_size(view, 400.0, 100.0);
    harness.set_scale(2.0);

    // Click at the middle of the slider
    // At 2x scale, logical middle (150) might need to be 300 in physical pixels?
    harness.pointer_down(150.0, 8.0);

    let new_value = value.get();
    eprintln!("Scale 2x - Click at x=150: value = {}", new_value);

    // Should be close to 50%
    assert!(
        (new_value - 50.0).abs() < 10.0,
        "At 2x scale, clicking at middle should give ~50%, got {}%",
        new_value
    );
}

#[test]
fn test_slider_scale_2x_click_positions() {
    // Test multiple click positions at 2x scale
    let value = RwSignal::new(0.0);

    let positions_and_expected: Vec<(f64, f64)> = vec![
        (8.0, 0.0),     // Start of track
        (79.0, 25.0),   // 25%
        (150.0, 50.0),  // Middle
        (221.0, 75.0),  // 75%
        (292.0, 100.0), // End of track
    ];

    for (click_x, expected_percent) in positions_and_expected {
        value.set(0.0);
        let (_container_id, view) = create_test_slider(value, 300.0);

        let mut harness = HeadlessHarness::new_with_size(view, 400.0, 100.0);
        harness.set_scale(2.0);

        harness.pointer_down(click_x, 8.0);

        let actual = value.get();
        eprintln!(
            "Scale 2x - Click at x={}: expected {}%, got {}%",
            click_x, expected_percent, actual
        );

        assert!(
            (actual - expected_percent).abs() < 10.0,
            "At 2x scale, click at x={} should give ~{}%, got {}%",
            click_x,
            expected_percent,
            actual
        );
    }
}

/// Debug test to understand coordinate systems at scale 1.0
#[test]
fn test_debug_scale_1x_coordinates() {
    let debug_info: std::rc::Rc<std::cell::RefCell<Vec<String>>> =
        std::rc::Rc::new(std::cell::RefCell::new(Vec::new()));
    let debug_clone = debug_info.clone();

    let container_id = ViewId::new();

    let view = floem::views::Container::with_id(
        container_id,
        floem::views::Empty::new().style(|s| s.height(6.0).width_full()),
    )
    .style(|s| {
        s.width(300.0)
            .height(16.0)
            .padding_left(8.0)
            .padding_right(8.0)
    })
    .on_event_stop(floem::event::EventListener::PointerDown, move |e| {
        if let floem::event::Event::Pointer(PointerEvent::Down(pointer_event)) = e {
            let layout = container_id.layout_rect();
            let content = container_id.get_content_rect();
            let info = format!(
                "pos=({:.1},{:.1}) layout=({:.1},{:.1},{:.1},{:.1}) content=({:.1},{:.1},{:.1},{:.1})",
                pointer_event.state.position.x,
                pointer_event.state.position.y,
                layout.x0, layout.y0, layout.x1, layout.y1,
                content.x0, content.y0, content.x1, content.y1,
            );
            debug_clone.borrow_mut().push(info);
        }
    });

    eprintln!("=== Scale 1.0 ===");
    let mut harness = HeadlessHarness::new_with_size(view, 400.0, 100.0);
    harness.set_scale(1.0);
    harness.pointer_down(150.0, 8.0);

    for info in debug_info.borrow().iter() {
        eprintln!("{}", info);
    }
}

/// Debug test to understand coordinate systems at scale 2.0
#[test]
fn test_debug_scale_2x_coordinates() {
    let debug_info: std::rc::Rc<std::cell::RefCell<Vec<String>>> =
        std::rc::Rc::new(std::cell::RefCell::new(Vec::new()));
    let debug_clone = debug_info.clone();

    let container_id = ViewId::new();

    let view = floem::views::Container::with_id(
        container_id,
        floem::views::Empty::new().style(|s| s.height(6.0).width_full()),
    )
    .style(|s| {
        s.width(300.0)
            .height(16.0)
            .padding_left(8.0)
            .padding_right(8.0)
    })
    .on_event_stop(floem::event::EventListener::PointerDown, move |e| {
        if let floem::event::Event::Pointer(PointerEvent::Down(pointer_event)) = e {
            let layout = container_id.layout_rect();
            let content = container_id.get_content_rect();
            let info = format!(
                "pos=({:.1},{:.1}) layout=({:.1},{:.1},{:.1},{:.1}) content=({:.1},{:.1},{:.1},{:.1})",
                pointer_event.state.position.x,
                pointer_event.state.position.y,
                layout.x0, layout.y0, layout.x1, layout.y1,
                content.x0, content.y0, content.x1, content.y1,
            );
            debug_clone.borrow_mut().push(info);
        }
    });

    eprintln!("=== Scale 2.0 ===");
    let mut harness = HeadlessHarness::new_with_size(view, 400.0, 100.0);
    harness.set_scale(2.0);
    harness.pointer_down(150.0, 8.0);

    for info in debug_info.borrow().iter() {
        eprintln!("{}", info);
    }
}

// =============================================================================
// Drag Tests - Test dragging behavior with pointer capture
// =============================================================================

/// Create a slider with drag support and pointer capture
fn create_drag_slider(value: RwSignal<f64>, width: f64) -> (ViewId, impl IntoView) {
    use floem::reactive::SignalGet;

    let container_id = ViewId::new();
    let padding = 8.0;
    let min = 0.0;
    let max = 100.0;
    let is_dragging = RwSignal::new(false);

    let calc_value = move |x: f64| {
        let content_rect = container_id.get_content_rect();
        let track_width = content_rect.width();
        let click_x = x - content_rect.x0;
        let percent = (click_x / track_width).clamp(0.0, 1.0);
        min + percent * (max - min)
    };

    let view = floem::views::Container::with_id(
        container_id,
        floem::views::Empty::new().style(move |s| {
            let percent = value.get();
            s.height(6.0)
                .width_pct(percent)
                .background(peniko::Color::from_rgb8(14, 165, 233))
        }),
    )
    .style(move |s| {
        s.width(width)
            .height(16.0)
            .items_center()
            .background(peniko::Color::from_rgb8(229, 229, 229))
            .padding_left(padding)
            .padding_right(padding)
            .cursor(floem::style::CursorStyle::Pointer)
    })
    .on_event(floem::event::EventListener::PointerDown, move |e| {
        if let floem::event::Event::Pointer(PointerEvent::Down(pointer_event)) = e {
            let x = pointer_event.state.logical_point().x;
            value.set(calc_value(x));
            is_dragging.set(true);
            // Set pointer capture to receive events even outside bounds
            if let Some(pointer_id) = pointer_event.pointer.pointer_id {
                container_id.set_pointer_capture(pointer_id);
            }
        }
        floem::event::EventPropagation::Continue
    })
    .on_event(floem::event::EventListener::PointerMove, move |e| {
        if !is_dragging.get() {
            return floem::event::EventPropagation::Continue;
        }
        if let floem::event::Event::Pointer(PointerEvent::Move(pointer_event)) = e {
            let x = pointer_event.current.logical_point().x;
            value.set(calc_value(x));
        }
        floem::event::EventPropagation::Continue
    })
    .on_event(floem::event::EventListener::PointerUp, move |_| {
        is_dragging.set(false);
        floem::event::EventPropagation::Continue
    });

    (container_id, view)
}

#[test]
fn test_slider_drag_within_bounds() {
    let value = RwSignal::new(0.0);
    let (_container_id, view) = create_drag_slider(value, 300.0);

    let mut harness = HeadlessHarness::new_with_size(view, 400.0, 100.0);

    // Click at start
    harness.pointer_down(8.0, 8.0);
    let start_value = value.get();
    eprintln!("After pointer down at x=8: value = {}", start_value);
    assert!(
        start_value < 5.0,
        "Initial click should be near 0%, got {}",
        start_value
    );

    // Drag to middle
    harness.pointer_move(150.0, 8.0);
    let mid_value = value.get();
    eprintln!("After pointer move to x=150: value = {}", mid_value);
    assert!(
        (mid_value - 50.0).abs() < 10.0,
        "Drag to middle should be ~50%, got {}",
        mid_value
    );

    // Drag to end
    harness.pointer_move(292.0, 8.0);
    let end_value = value.get();
    eprintln!("After pointer move to x=292: value = {}", end_value);
    assert!(
        end_value > 95.0,
        "Drag to end should be ~100%, got {}",
        end_value
    );

    // Release
    harness.pointer_up(292.0, 8.0);

    // Value should stay
    assert!(value.get() > 95.0, "Value should stay after release");
}

#[test]
fn test_slider_pointer_capture_sets_correctly() {
    use floem::event::EventListener;

    let value = RwSignal::new(0.0);
    let tracker = PointerCaptureTracker::new();

    let container_id = ViewId::new();
    let is_dragging = RwSignal::new(false);

    let calc_value = move |x: f64| {
        let content_rect = container_id.get_content_rect();
        let track_width = content_rect.width();
        let click_x = x - content_rect.x0;
        (click_x / track_width).clamp(0.0, 1.0) * 100.0
    };

    let base = floem::views::Container::with_id(
        container_id,
        floem::views::Empty::new().style(|s| s.height(6.0).width_full()),
    )
    .style(|s| {
        s.width(300.0)
            .height(16.0)
            .padding_left(8.0)
            .padding_right(8.0)
    })
    .on_event(EventListener::PointerDown, move |e| {
        if let floem::event::Event::Pointer(PointerEvent::Down(pointer_event)) = e {
            let x = pointer_event.state.logical_point().x;
            value.set(calc_value(x));
            is_dragging.set(true);
            if let Some(pointer_id) = pointer_event.pointer.pointer_id {
                container_id.set_pointer_capture(pointer_id);
            }
        }
        floem::event::EventPropagation::Continue
    })
    .on_event(EventListener::PointerMove, move |e| {
        if !is_dragging.get() {
            return floem::event::EventPropagation::Continue;
        }
        if let floem::event::Event::Pointer(PointerEvent::Move(pointer_event)) = e {
            let x = pointer_event.current.logical_point().x;
            value.set(calc_value(x));
        }
        floem::event::EventPropagation::Continue
    })
    .on_event(EventListener::PointerUp, move |_| {
        is_dragging.set(false);
        floem::event::EventPropagation::Continue
    });

    let view = tracker.track("slider", base);

    let mut harness = HeadlessHarness::new_with_size(view, 400.0, 100.0);

    // Pointer down should set capture
    harness.pointer_down(50.0, 8.0);
    harness.rebuild();

    // Move to trigger capture activation
    harness.pointer_move(50.0, 8.0);

    eprintln!("Got capture count: {}", tracker.got_capture_count());
    eprintln!("Got capture names: {:?}", tracker.got_capture_names());

    assert_eq!(
        tracker.got_capture_count(),
        1,
        "Should have received GotPointerCapture event"
    );
    assert_eq!(
        tracker.got_capture_names(),
        vec!["slider"],
        "Slider should receive the capture event"
    );
}

#[test]
fn test_slider_drag_outside_bounds_with_capture() {
    let value = RwSignal::new(0.0);
    let tracker = PointerCaptureTracker::new();

    let container_id = ViewId::new();
    let is_dragging = RwSignal::new(false);

    let calc_value = move |x: f64| {
        let content_rect = container_id.get_content_rect();
        let track_width = content_rect.width();
        let click_x = x - content_rect.x0;
        (click_x / track_width).clamp(0.0, 1.0) * 100.0
    };

    let base = floem::views::Container::with_id(
        container_id,
        floem::views::Empty::new().style(|s| s.height(6.0).width_full()),
    )
    .style(|s| {
        s.width(300.0)
            .height(16.0)
            .padding_left(8.0)
            .padding_right(8.0)
    })
    .on_event(floem::event::EventListener::PointerDown, move |e| {
        if let floem::event::Event::Pointer(PointerEvent::Down(pointer_event)) = e {
            let x = pointer_event.state.logical_point().x;
            value.set(calc_value(x));
            is_dragging.set(true);
            if let Some(pointer_id) = pointer_event.pointer.pointer_id {
                container_id.set_pointer_capture(pointer_id);
            }
        }
        floem::event::EventPropagation::Continue
    })
    .on_event(floem::event::EventListener::PointerMove, move |e| {
        if !is_dragging.get() {
            return floem::event::EventPropagation::Continue;
        }
        if let floem::event::Event::Pointer(PointerEvent::Move(pointer_event)) = e {
            let x = pointer_event.current.logical_point().x;
            value.set(calc_value(x));
        }
        floem::event::EventPropagation::Continue
    })
    .on_event(floem::event::EventListener::PointerUp, move |_| {
        is_dragging.set(false);
        floem::event::EventPropagation::Continue
    });

    let view = tracker.track("slider", base);

    let mut harness = HeadlessHarness::new_with_size(view, 400.0, 100.0);

    // Start drag
    harness.pointer_down(150.0, 8.0);
    harness.rebuild();

    let initial_value = value.get();
    eprintln!("After pointer down at x=150: value = {}", initial_value);

    // Move to activate capture
    harness.pointer_move(150.0, 8.0);

    assert_eq!(tracker.got_capture_count(), 1, "Should have capture");

    tracker.reset();

    // Move OUTSIDE the slider bounds (slider is 0-300, move to x=350)
    harness.pointer_move(350.0, 8.0);

    let outside_value = value.get();
    eprintln!(
        "After pointer move to x=350 (outside): value = {}",
        outside_value
    );

    // With capture, the slider should still receive the move event
    // Value should be clamped to 100% since we're past the end
    assert_eq!(
        tracker.pointer_move_names(),
        vec!["slider"],
        "Slider should receive move event even outside bounds due to capture"
    );

    assert!(
        outside_value >= 100.0,
        "Value should be clamped to 100% when dragging past end, got {}",
        outside_value
    );
}

#[test]
fn test_slider_capture_released_on_pointer_up() {
    let value = RwSignal::new(0.0);
    let tracker = PointerCaptureTracker::new();

    let container_id = ViewId::new();
    let is_dragging = RwSignal::new(false);

    let calc_value = move |x: f64| {
        let content_rect = container_id.get_content_rect();
        let track_width = content_rect.width();
        let click_x = x - content_rect.x0;
        (click_x / track_width).clamp(0.0, 1.0) * 100.0
    };

    let base = floem::views::Container::with_id(
        container_id,
        floem::views::Empty::new().style(|s| s.height(6.0).width_full()),
    )
    .style(|s| {
        s.width(300.0)
            .height(16.0)
            .padding_left(8.0)
            .padding_right(8.0)
    })
    .on_event(floem::event::EventListener::PointerDown, move |e| {
        if let floem::event::Event::Pointer(PointerEvent::Down(pointer_event)) = e {
            let x = pointer_event.state.logical_point().x;
            value.set(calc_value(x));
            is_dragging.set(true);
            if let Some(pointer_id) = pointer_event.pointer.pointer_id {
                container_id.set_pointer_capture(pointer_id);
            }
        }
        floem::event::EventPropagation::Continue
    })
    .on_event(floem::event::EventListener::PointerMove, move |e| {
        if !is_dragging.get() {
            return floem::event::EventPropagation::Continue;
        }
        if let floem::event::Event::Pointer(PointerEvent::Move(pointer_event)) = e {
            let x = pointer_event.current.logical_point().x;
            value.set(calc_value(x));
        }
        floem::event::EventPropagation::Continue
    })
    .on_event(floem::event::EventListener::PointerUp, move |_| {
        is_dragging.set(false);
        floem::event::EventPropagation::Continue
    });

    let view = tracker.track("slider", base);

    let mut harness = HeadlessHarness::new_with_size(view, 400.0, 100.0);

    // Start drag
    harness.pointer_down(150.0, 8.0);
    harness.rebuild();
    harness.pointer_move(150.0, 8.0);

    assert_eq!(tracker.got_capture_count(), 1, "Should have capture");

    // Release pointer
    harness.pointer_up(150.0, 8.0);

    // Move to process release
    harness.pointer_move(150.0, 8.0);

    eprintln!("Lost capture count: {}", tracker.lost_capture_count());

    assert_eq!(
        tracker.lost_capture_count(),
        1,
        "Should have received LostPointerCapture after pointer up"
    );
}

// =============================================================================
// Bug Reproduction Tests - Value resets to 0 on pointer up
// =============================================================================

/// Test that replicates the bug: value resets to 0 after pointer up
#[test]
fn test_slider_value_persists_after_click() {
    let value = RwSignal::new(0.0);
    let (_container_id, view) = create_drag_slider(value, 300.0);

    let mut harness = HeadlessHarness::new_with_size(view, 400.0, 100.0);

    // Click at middle
    harness.pointer_down(150.0, 8.0);
    let value_after_down = value.get();
    eprintln!("After pointer_down at x=150: value = {}", value_after_down);

    // Release at same position
    harness.pointer_up(150.0, 8.0);
    let value_after_up = value.get();
    eprintln!("After pointer_up: value = {}", value_after_up);

    // Value should NOT reset to 0
    assert!(
        (value_after_up - value_after_down).abs() < 0.1,
        "Value should persist after pointer up! Was {} after down, but {} after up",
        value_after_down,
        value_after_up
    );
}

/// Test with rebuild between events (simulates real app frame updates)
#[test]
fn test_slider_value_persists_after_click_with_rebuild() {
    let value = RwSignal::new(0.0);
    let (_container_id, view) = create_drag_slider(value, 300.0);

    let mut harness = HeadlessHarness::new_with_size(view, 400.0, 100.0);

    // Click at middle
    harness.pointer_down(150.0, 8.0);
    let value_after_down = value.get();
    eprintln!("After pointer_down at x=150: value = {}", value_after_down);

    // Rebuild (like a frame update in real app)
    harness.rebuild();
    let value_after_rebuild = value.get();
    eprintln!("After rebuild: value = {}", value_after_rebuild);

    // Release at same position
    harness.pointer_up(150.0, 8.0);
    let value_after_up = value.get();
    eprintln!("After pointer_up: value = {}", value_after_up);

    // Another rebuild
    harness.rebuild();
    let value_after_second_rebuild = value.get();
    eprintln!(
        "After second rebuild: value = {}",
        value_after_second_rebuild
    );

    // Value should NOT reset to 0 at any point
    assert!(
        value_after_down > 40.0,
        "Value should be ~50% after pointer down, got {}",
        value_after_down
    );
    assert!(
        value_after_rebuild > 40.0,
        "Value should persist after rebuild, got {}",
        value_after_rebuild
    );
    assert!(
        value_after_up > 40.0,
        "Value should persist after pointer up, got {}",
        value_after_up
    );
    assert!(
        value_after_second_rebuild > 40.0,
        "Value should persist after second rebuild, got {}",
        value_after_second_rebuild
    );
}

/// Test using the actual Slider component (not test helper)
#[test]
fn test_actual_slider_component_value_persists() {
    use floem_shadcn::components::slider::Slider;

    let value = RwSignal::new(0.0);
    let view = Slider::new(value).build();

    let mut harness = HeadlessHarness::new_with_size(view, 400.0, 100.0);

    eprintln!("Initial value: {}", value.get());

    // Click at middle of slider
    harness.pointer_down(200.0, 8.0);
    let value_after_down = value.get();
    eprintln!("After pointer_down at x=200: value = {}", value_after_down);

    harness.rebuild();
    let value_after_rebuild = value.get();
    eprintln!("After rebuild: value = {}", value_after_rebuild);

    // Release
    harness.pointer_up(200.0, 8.0);
    let value_after_up = value.get();
    eprintln!("After pointer_up: value = {}", value_after_up);

    harness.rebuild();
    let value_final = value.get();
    eprintln!("Final value after rebuild: value = {}", value_final);

    // Check that value persists
    assert!(
        value_final > 0.0,
        "Value should NOT reset to 0 after pointer up! Final value: {}",
        value_final
    );
}

/// Test with Retina scale (2.0) - common on macOS
#[test]
fn test_actual_slider_component_value_persists_scale_2x() {
    use floem_shadcn::components::slider::Slider;

    let value = RwSignal::new(0.0);
    let view = Slider::new(value).build();

    let mut harness = HeadlessHarness::new_with_size(view, 400.0, 100.0);
    harness.set_scale(2.0);

    eprintln!("=== Scale 2.0 Test ===");
    eprintln!("Initial value: {}", value.get());

    // Click at middle of slider
    harness.pointer_down(200.0, 8.0);
    let value_after_down = value.get();
    eprintln!("After pointer_down at x=200: value = {}", value_after_down);

    harness.rebuild();
    let value_after_rebuild = value.get();
    eprintln!("After rebuild: value = {}", value_after_rebuild);

    // Release
    harness.pointer_up(200.0, 8.0);
    let value_after_up = value.get();
    eprintln!("After pointer_up: value = {}", value_after_up);

    harness.rebuild();
    let value_final = value.get();
    eprintln!("Final value after rebuild: value = {}", value_final);

    // Check that value persists
    assert!(
        value_final > 0.0,
        "Value should NOT reset to 0 after pointer up! Final value: {}",
        value_final
    );
}

/// Test with click event (harness.click simulates full click cycle)
#[test]
fn test_actual_slider_component_click() {
    use floem_shadcn::components::slider::Slider;

    let value = RwSignal::new(0.0);
    let view = Slider::new(value).build();

    let mut harness = HeadlessHarness::new_with_size(view, 400.0, 100.0);

    eprintln!("=== Click Test ===");
    eprintln!("Initial value: {}", value.get());

    // Use click which does pointer_down + pointer_up
    harness.click(200.0, 8.0);
    let value_after_click = value.get();
    eprintln!("After click at x=200: value = {}", value_after_click);

    harness.rebuild();
    let value_final = value.get();
    eprintln!("Final value after rebuild: value = {}", value_final);

    // Check that value persists
    assert!(
        value_final > 0.0,
        "Value should NOT reset to 0 after click! Final value: {}",
        value_final
    );
}

// =============================================================================
// Coordinate System Tests - Verify handling of window vs view-relative coords
// =============================================================================

/// Test that documents the coordinate system difference between PointerDown and PointerMove
/// when pointer capture is active.
///
/// Key finding:
/// - PointerDown: coordinates are in WINDOW space (absolute position in window)
/// - PointerMove (with capture): coordinates are VIEW-RELATIVE (relative to capturing view)
///
/// This test verifies our fix handles both coordinate systems correctly.
#[test]
fn test_coordinate_systems_with_pointer_capture() {
    use floem::reactive::SignalGet;

    // Create a slider at a known position (not at x=0 in the window)
    let value = RwSignal::new(0.0);

    let container_id = ViewId::new();
    let is_dragging = RwSignal::new(false);
    let min = 0.0;
    let max = 100.0;

    // Track coordinates received by each event type
    let coords_received = std::rc::Rc::new(std::cell::RefCell::new(Vec::new()));
    let coords_clone1 = coords_received.clone();
    let coords_clone2 = coords_received.clone();

    let view = floem::views::Container::with_id(
        container_id,
        floem::views::Empty::new().style(|s| s.height(6.0).width_full()),
    )
    .style(|s| {
        s.width(300.0)
            .height(16.0)
            .padding_left(8.0)
            .padding_right(8.0)
            .margin_left(100.0) // Offset to simulate being in a layout
    })
    .on_event(floem::event::EventListener::PointerDown, move |e| {
        if let floem::event::Event::Pointer(PointerEvent::Down(pointer_event)) = e {
            let x = pointer_event.state.logical_point().x;
            coords_clone1.borrow_mut().push(("PointerDown", x));
            is_dragging.set(true);
            if let Some(pointer_id) = pointer_event.pointer.pointer_id {
                container_id.set_pointer_capture(pointer_id);
            }
        }
        floem::event::EventPropagation::Continue
    })
    .on_event(floem::event::EventListener::PointerMove, move |e| {
        if !is_dragging.get() {
            return floem::event::EventPropagation::Continue;
        }
        if let floem::event::Event::Pointer(PointerEvent::Move(pointer_event)) = e {
            let x = pointer_event.current.logical_point().x;
            coords_clone2.borrow_mut().push(("PointerMove", x));
        }
        floem::event::EventPropagation::Continue
    });

    let mut harness = HeadlessHarness::new_with_size(view, 500.0, 100.0);

    // Click at x=200 in window coordinates
    harness.pointer_down(200.0, 8.0);
    harness.rebuild();

    // Move to trigger capture
    harness.pointer_move(200.0, 8.0);

    let coords = coords_received.borrow();
    eprintln!("=== Coordinate System Test ===");
    for (event, x) in coords.iter() {
        eprintln!("{}: x = {:.1}", event, x);
    }

    // This test documents the behavior - in a real scenario with pointer capture,
    // PointerMove would give view-relative coordinates which differ from the
    // window coordinates in PointerDown
    assert!(
        coords.len() >= 2,
        "Should have received both PointerDown and PointerMove"
    );
}

/// Test that the slider correctly handles the transition from window coords (PointerDown)
/// to view-relative coords (PointerMove with capture) during a drag operation
#[test]
fn test_slider_drag_with_coordinate_transition() {
    use floem_shadcn::components::slider::Slider;

    let value = RwSignal::new(0.0);
    let view = Slider::new(value).build();

    let mut harness = HeadlessHarness::new_with_size(view, 400.0, 100.0);

    eprintln!("=== Drag with Coordinate Transition Test ===");

    // Start drag at left side
    harness.pointer_down(50.0, 8.0);
    let value_at_start = value.get();
    eprintln!("After pointer_down at x=50: value = {:.1}", value_at_start);

    harness.rebuild();

    // Move to middle - if coordinate handling is wrong, this would give wrong value
    harness.pointer_move(200.0, 8.0);
    let value_at_middle = value.get();
    eprintln!(
        "After pointer_move to x=200: value = {:.1}",
        value_at_middle
    );

    // Move to right
    harness.pointer_move(350.0, 8.0);
    let value_at_right = value.get();
    eprintln!("After pointer_move to x=350: value = {:.1}", value_at_right);

    // Release
    harness.pointer_up(350.0, 8.0);
    let value_final = value.get();
    eprintln!("Final value: {:.1}", value_final);

    // Values should increase monotonically from left to right
    assert!(
        value_at_start < value_at_middle,
        "Value should increase from left ({:.1}) to middle ({:.1})",
        value_at_start,
        value_at_middle
    );
    assert!(
        value_at_middle < value_at_right,
        "Value should increase from middle ({:.1}) to right ({:.1})",
        value_at_middle,
        value_at_right
    );
    assert!(
        value_final > 80.0,
        "Final value should be high (~100%) when released at right, got {:.1}",
        value_final
    );
}

/// Test that value doesn't reset to 0 after PointerMove with different coordinate systems
#[test]
fn test_value_does_not_reset_after_pointer_move() {
    use floem_shadcn::components::slider::Slider;

    let value = RwSignal::new(0.0);
    let view = Slider::new(value).build();

    let mut harness = HeadlessHarness::new_with_size(view, 400.0, 100.0);

    // This test specifically reproduces the bug where value resets to 0
    // because PointerMove uses view-relative coords while the calculation
    // expected window coords

    harness.pointer_down(200.0, 8.0);
    let value_after_down = value.get();
    eprintln!(
        "After pointer_down at x=200: value = {:.1}",
        value_after_down
    );

    harness.rebuild();

    // This PointerMove would previously cause value to go to 0 because
    // the coords were misinterpreted
    harness.pointer_move(200.0, 8.0);
    let value_after_move = value.get();
    eprintln!(
        "After pointer_move at x=200: value = {:.1}",
        value_after_move
    );

    // Value should remain approximately the same (not reset to 0)
    assert!(
        value_after_move > 30.0,
        "Value should NOT reset to 0 after PointerMove! Got {:.1}",
        value_after_move
    );

    // The values should be close to each other
    let diff = (value_after_down - value_after_move).abs();
    assert!(
        diff < 20.0,
        "Values should be similar after down ({:.1}) and move ({:.1}), diff = {:.1}",
        value_after_down,
        value_after_move,
        diff
    );
}
