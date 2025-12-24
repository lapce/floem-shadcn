//! Tests for the Slider component click position calculations.

use floem::prelude::*;
use floem::reactive::RwSignal;
use floem::views::Decorators;
use floem::ViewId;
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
            eprintln!("logical position.x: {}", pointer_event.state.logical_point().x);
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
        (8.0, 0.0),    // Start of track
        (79.0, 25.0),  // 25%
        (150.0, 50.0), // Middle
        (221.0, 75.0), // 75%
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
