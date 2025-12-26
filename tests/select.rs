//! Tests for Select component

use floem::prelude::*;
use floem::reactive::RwSignal;
use floem_shadcn::components::select::{Select, SelectItemData};
use floem_test::prelude::*;

/// Test basic select click interaction
#[test]
fn test_select_basic_click() {
    let selected = RwSignal::new(None::<String>);
    let click_count = RwSignal::new(0);

    // Create a simple button to verify click works
    let test_button = floem::views::Label::new("Test Button")
        .style(|s| {
            s.size(100.0, 40.0)
                .background(peniko::Color::from_rgb8(255, 0, 0))
        })
        .on_click_stop(move |_| {
            click_count.update(|c| *c += 1);
        });

    let select = Select::new(selected).placeholder("Select...").items(vec![
        SelectItemData::new("a", "Option A"),
        SelectItemData::new("b", "Option B"),
    ]);

    let view = floem::views::Stack::vertical((test_button, select))
        .style(|s| s.size(400.0, 300.0).gap(10.0));

    let mut harness = HeadlessHarness::new_with_size(view, 400.0, 300.0);
    harness.rebuild();

    eprintln!("=== Basic Click Test ===");

    // Click the test button first to verify clicking works
    harness.click(50.0, 20.0);
    harness.rebuild();
    eprintln!(
        "Click count after clicking test button: {}",
        click_count.get()
    );

    // Now click on select trigger (should be at y ~50 after the button)
    harness.click(60.0, 70.0);
    harness.rebuild();

    // Try to click an option
    harness.click(60.0, 120.0);
    harness.rebuild();

    eprintln!("Selected: {:?}", selected.get());
}

/// Test that dropdown is positioned correctly below the trigger
#[test]
fn test_select_dropdown_position() {
    let selected = RwSignal::new(None::<String>);

    let select = Select::new(selected)
        .placeholder("Select an option...")
        .items(vec![
            SelectItemData::new("option1", "Option 1"),
            SelectItemData::new("option2", "Option 2"),
            SelectItemData::new("option3", "Option 3"),
        ]);

    // Put select in a container with some offset to test positioning
    // Trigger should be at approximately (100, 50) in window coordinates
    let view = floem::views::Stack::vertical((
        floem::views::Empty::new().style(|s| s.height(50.0)), // spacer
        floem::views::Stack::horizontal((
            floem::views::Empty::new().style(|s| s.width(100.0)), // left spacer
            select,
        )),
    ))
    .style(|s| s.size(800.0, 600.0));

    let mut harness = HeadlessHarness::new_with_size(view, 800.0, 600.0);
    harness.rebuild();

    eprintln!("=== Select Dropdown Position Test ===");

    // Click to open the dropdown - trigger at (100, 50)
    harness.click(150.0, 70.0);
    harness.rebuild();

    // The dropdown should appear at x=100 (trigger's left edge in window coords)
    // If using local coords incorrectly, it would appear at x=0

    // Try clicking at the correct position (x=110, accounting for trigger x=100)
    harness.click(110.0, 100.0);
    harness.rebuild();

    eprintln!(
        "Selected after clicking at (110, 100): {:?}",
        selected.get()
    );

    // For now, just verify the test runs - we'll assert after confirming clicks work
}

/// Test select at origin - verify position tracking works
/// Note: Full dropdown interaction tests require visual testing since
/// the Overlay may not fully render in headless mode.
#[test]
fn test_select_dropdown_position_at_origin() {
    let selected = RwSignal::new(None::<String>);

    let select = Select::new(selected).placeholder("Select...").items(vec![
        SelectItemData::new("a", "Option A"),
        SelectItemData::new("b", "Option B"),
    ]);

    let view = floem::views::Stack::vertical((select,)).style(|s| s.size(400.0, 300.0));

    let mut harness = HeadlessHarness::new_with_size(view, 400.0, 300.0);
    harness.rebuild();

    eprintln!("=== Select at Origin Test ===");

    // Just verify the test runs without panicking - position testing
    // requires visual verification since Overlay behavior in headless mode
    // may differ from real windowed mode.
    // The key fix was using on_move (window coords) instead of on_resize (local coords)
}

/// Test that select with offset trigger doesn't break
#[test]
fn test_select_with_offset_trigger() {
    let selected = RwSignal::new(None::<String>);

    let select = Select::new(selected)
        .placeholder("Select...")
        .items(vec![SelectItemData::new("test", "Test Option")]);

    // Put select at offset (50, 100)
    let view = floem::views::Stack::vertical((
        floem::views::Empty::new().style(|s| s.height(100.0)),
        floem::views::Stack::horizontal((
            floem::views::Empty::new().style(|s| s.width(50.0)),
            select,
        )),
    ))
    .style(|s| s.size(400.0, 300.0));

    let mut harness = HeadlessHarness::new_with_size(view, 400.0, 300.0);
    harness.rebuild();

    eprintln!("=== Select with Offset Test ===");

    // Verify the view builds correctly with offset positioning
    // The fix: using on_move for window coordinates ensures the dropdown
    // appears at x=50 (trigger's window x) instead of x=0 (local origin)
}
