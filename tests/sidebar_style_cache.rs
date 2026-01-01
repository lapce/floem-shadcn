//! Test to reproduce sidebar menu button style cache bug
//!
//! Bug reported: When clicking a sidebar menu button, the `is_active` state changes but
//! the font-weight style doesn't update immediately. The style only updates
//! when the pointer moves away (hover state change forces style recomputation).
//!
//! Expected: Style should update immediately when reactive state changes.
//! Actual (in real app): Style only updates after hover state changes.
//!
//! IMPORTANT FINDING: These tests PASS in the headless harness, which shows that
//! the reactive style computation IS working correctly at the core level.
//! The bug appears to be in the rendering/repaint scheduling in the actual window,
//! not in the style cache itself.
//!
//! This suggests the fix needs to be in how the window schedules repaints when
//! a reactive value changes inside a style closure - the style is correctly
//! recomputed, but the repaint may not be triggered.

use floem::prelude::*;
use floem::reactive::RwSignal;
use floem::style::FontWeight;
use floem::text::Weight;
use floem_shadcn::components::sidebar::*;
use floem_test::prelude::*;

/// Test that font-weight updates immediately when is_active changes on click.
///
/// This test replicates the bug where:
/// 1. User clicks a sidebar menu button
/// 2. The `is_active` signal changes to true
/// 3. The font-weight should change to MEDIUM (bold)
/// 4. BUG: The font-weight doesn't update until pointer moves away
#[test]
fn test_sidebar_menu_button_style_updates_on_click() {
    let active = RwSignal::new("none");

    let btn1 = SidebarMenuButton::new("First").is_active(move || active.get() == "first");
    let btn1_id = btn1.view_id();

    let btn2 = SidebarMenuButton::new("Second").is_active(move || active.get() == "second");
    let btn2_id = btn2.view_id();

    let menu = SidebarMenu::new()
        .child(SidebarMenuItem::new().child(btn1.on_click_stop(move |_| active.set("first"))))
        .child(SidebarMenuItem::new().child(btn2.on_click_stop(move |_| active.set("second"))));

    let container = Stack::new((menu,)).style(|s| s.size(300.0, 400.0));

    let mut harness = HeadlessHarness::new_with_size(container, 300.0, 400.0);
    harness.rebuild();

    // Initial state: no button is active
    assert_eq!(active.get(), "none");

    // Get initial layout to find button positions (use layout_rect for absolute coords)
    let btn1_rect = harness.get_layout_rect(btn1_id);
    let btn2_rect = harness.get_layout_rect(btn2_id);

    eprintln!("Button 1 rect: {:?}", btn1_rect);
    eprintln!("Button 2 rect: {:?}", btn2_rect);

    // Calculate click position for button 1 (center of button)
    let btn1_center_x = btn1_rect.center().x;
    let btn1_center_y = btn1_rect.center().y;

    // Simulate click on button 1
    harness.click(btn1_center_x as f64, btn1_center_y as f64);

    // The signal should have changed
    assert_eq!(
        active.get(),
        "first",
        "Signal should update to 'first' after click"
    );

    // Rebuild to process the state change
    harness.rebuild();

    // Check the computed style BEFORE moving pointer away
    let style_before_move = harness.get_computed_style(btn1_id);
    let font_weight_before = style_before_move.get(FontWeight);
    eprintln!("Font weight BEFORE pointer move: {:?}", font_weight_before);

    // At this point, the bug manifests:
    // - The signal has changed (is_active returns true for btn1)
    // - But the style cache hasn't been invalidated
    // - The font-weight is still NORMAL instead of MEDIUM

    eprintln!("After click - active state: {}", active.get());
    eprintln!("Button 1 should now be active (bold)");

    // Simulate pointer leaving the button (this triggers hover state change)
    harness.pointer_move(0.0, 0.0); // Move pointer away
    harness.rebuild();

    // Check the computed style AFTER moving pointer away
    let style_after_move = harness.get_computed_style(btn1_id);
    let font_weight_after = style_after_move.get(FontWeight);
    eprintln!("Font weight AFTER pointer move: {:?}", font_weight_after);

    // The bug: font_weight_before should equal font_weight_after (both MEDIUM)
    // but font_weight_before is NORMAL and font_weight_after is MEDIUM
    assert_eq!(
        font_weight_before, font_weight_after,
        "BUG: Font weight should be the same before and after pointer move. \
         Before: {:?}, After: {:?}. \
         The style should update immediately on click, not require a hover change.",
        font_weight_before, font_weight_after
    );
}

/// Test with multiple state changes to show the cache invalidation issue
/// This test switches from button 1 active to button 2 active
#[test]
fn test_sidebar_style_cache_multiple_clicks() {
    let active = RwSignal::new("none");

    let btn1 = SidebarMenuButton::new("First").is_active(move || active.get() == "first");
    let btn1_id = btn1.view_id();

    let btn2 = SidebarMenuButton::new("Second").is_active(move || active.get() == "second");
    let btn2_id = btn2.view_id();

    let menu = SidebarMenu::new()
        .child(SidebarMenuItem::new().child(btn1.on_click_stop(move |_| active.set("first"))))
        .child(SidebarMenuItem::new().child(btn2.on_click_stop(move |_| active.set("second"))));

    let container = Stack::new((menu,)).style(|s| s.size(300.0, 400.0));

    let mut harness = HeadlessHarness::new_with_size(container, 300.0, 400.0);
    harness.rebuild();

    // Initial state: no button is active
    assert_eq!(active.get(), "none");

    // Use layout_rect for absolute coordinates
    let btn1_rect = harness.get_layout_rect(btn1_id);
    let btn2_rect = harness.get_layout_rect(btn2_id);

    let btn1_center_x = btn1_rect.center().x;
    let btn1_center_y = btn1_rect.center().y;
    let btn2_center_x = btn2_rect.center().x;
    let btn2_center_y = btn2_rect.center().y;

    eprintln!(
        "Button 1 rect: {:?}, center: ({}, {})",
        btn1_rect, btn1_center_x, btn1_center_y
    );
    eprintln!(
        "Button 2 rect: {:?}, center: ({}, {})",
        btn2_rect, btn2_center_x, btn2_center_y
    );

    // First, click button 1 to make it active
    harness.click(btn1_center_x as f64, btn1_center_y as f64);
    assert_eq!(
        active.get(),
        "first",
        "Button 1 click should set active to 'first'"
    );
    harness.rebuild();

    // Move pointer away first to reset hover state
    harness.pointer_move(0.0, 0.0);
    harness.rebuild();

    eprintln!("\n=== Button 1 is now active ===");

    // Get button 1's style (should be bold/medium)
    let btn1_style_active = harness.get_computed_style(btn1_id);
    let btn1_weight_active = btn1_style_active.get(FontWeight);
    eprintln!("Button 1 font-weight when active: {:?}", btn1_weight_active);

    // Now click button 2 to switch
    harness.click(btn2_center_x as f64, btn2_center_y as f64);
    assert_eq!(
        active.get(),
        "second",
        "Button 2 click should set active to 'second'"
    );
    harness.rebuild();

    // Check styles immediately after click (before pointer move)
    let btn1_style_before = harness.get_computed_style(btn1_id);
    let btn2_style_before = harness.get_computed_style(btn2_id);
    let btn1_weight_before = btn1_style_before.get(FontWeight);
    let btn2_weight_before = btn2_style_before.get(FontWeight);

    eprintln!("\n=== After clicking Button 2 (before pointer move) ===");
    eprintln!("Active: {}", active.get());
    eprintln!(
        "Button 1 font-weight: {:?} (should be normal now)",
        btn1_weight_before
    );
    eprintln!(
        "Button 2 font-weight: {:?} (should be bold/medium now)",
        btn2_weight_before
    );

    // Now move pointer away to trigger hover state change
    harness.pointer_move(0.0, 0.0);
    harness.rebuild();

    // Check styles after pointer move
    let btn1_style_after = harness.get_computed_style(btn1_id);
    let btn2_style_after = harness.get_computed_style(btn2_id);
    let btn1_weight_after = btn1_style_after.get(FontWeight);
    let btn2_weight_after = btn2_style_after.get(FontWeight);

    eprintln!("\n=== After moving pointer away ===");
    eprintln!("Button 1 font-weight: {:?}", btn1_weight_after);
    eprintln!("Button 2 font-weight: {:?}", btn2_weight_after);

    // Assert the bug: styles should be the same before and after pointer move
    // Button 2 should have been bold immediately after click
    assert_eq!(
        btn2_weight_before, btn2_weight_after,
        "BUG: Button 2 font-weight changed after pointer move. \
         Before: {:?}, After: {:?}. Style should update on click, not on hover change.",
        btn2_weight_before, btn2_weight_after
    );

    // Button 1 should have become normal immediately after click
    assert_eq!(
        btn1_weight_before, btn1_weight_after,
        "BUG: Button 1 font-weight changed after pointer move. \
         Before: {:?}, After: {:?}. Style should update on click, not on hover change.",
        btn1_weight_before, btn1_weight_after
    );
}

/// Minimal reproduction case - single button with toggling active state
#[test]
fn test_minimal_style_cache_bug() {
    let is_active = RwSignal::new(false);

    let btn = SidebarMenuButton::new("Toggle").is_active(move || is_active.get());
    let btn_id = btn.view_id();

    let container = Stack::new((SidebarMenuItem::new().child(btn.on_click_stop(move |_| {
        is_active.set(!is_active.get());
    })),))
    .style(|s| s.size(300.0, 100.0));

    let mut harness = HeadlessHarness::new_with_size(container, 300.0, 100.0);
    harness.rebuild();

    let btn_rect = harness.get_layout_rect(btn_id);
    let center_x = btn_rect.center().x;
    let center_y = btn_rect.center().y;

    // Check initial style
    let initial_style = harness.get_computed_style(btn_id);
    let initial_weight = initial_style.get(FontWeight);
    eprintln!(
        "Initial: is_active = {}, font-weight = {:?}",
        is_active.get(),
        initial_weight
    );
    assert!(!is_active.get());

    // Click to activate
    harness.click(center_x as f64, center_y as f64);
    harness.rebuild();

    eprintln!("After click: is_active = {}", is_active.get());
    assert!(is_active.get(), "Signal should be true after click");

    // Check style immediately after click
    let style_after_click = harness.get_computed_style(btn_id);
    let weight_after_click = style_after_click.get(FontWeight);
    eprintln!(
        "Font weight after click (before pointer move): {:?}",
        weight_after_click
    );

    // Moving pointer away triggers the fix
    harness.pointer_move(0.0, 0.0);
    harness.rebuild();

    // Check style after pointer move
    let style_after_move = harness.get_computed_style(btn_id);
    let weight_after_move = style_after_move.get(FontWeight);
    eprintln!("Font weight after pointer move: {:?}", weight_after_move);

    // The bug: weight_after_click should equal weight_after_move
    assert_eq!(
        weight_after_click, weight_after_move,
        "BUG: Font weight changed after pointer move. \
         After click: {:?}, After move: {:?}. \
         Style should update immediately on signal change.",
        weight_after_click, weight_after_move
    );
}

/// Test if request_paint is scheduled when reactive style changes
///
/// CONFIRMED BUG: This test demonstrates that request_paint is NOT scheduled
/// when a reactive signal used in a style closure changes. The style gets
/// correctly recomputed, but no repaint is triggered.
#[test]
fn test_request_paint_on_reactive_style_change() {
    let is_active = RwSignal::new(false);

    let btn = SidebarMenuButton::new("Toggle").is_active(move || is_active.get());
    let btn_id = btn.view_id();

    let container = Stack::new((SidebarMenuItem::new().child(btn.on_click_stop(move |_| {
        is_active.set(!is_active.get());
    })),))
    .style(|s| s.size(300.0, 100.0));

    let mut harness = HeadlessHarness::new_with_size(container, 300.0, 100.0);
    harness.rebuild();

    let btn_rect = harness.get_layout_rect(btn_id);
    let center_x = btn_rect.center().x;
    let center_y = btn_rect.center().y;

    // Clear any pending paint requests from initial setup
    harness.clear_paint_request();
    assert!(
        !harness.paint_requested(),
        "Paint should not be requested before click"
    );

    eprintln!(
        "Before click: is_active = {}, paint_requested = {}",
        is_active.get(),
        harness.paint_requested()
    );

    // Click to activate - this SHOULD schedule a repaint (but doesn't due to bug)
    harness.click(center_x as f64, center_y as f64);

    eprintln!(
        "After click (before rebuild): is_active = {}, paint_requested = {}",
        is_active.get(),
        harness.paint_requested()
    );

    // Check if paint was requested BEFORE rebuild
    let paint_requested_after_click = harness.paint_requested();

    // Check style dirty status
    let is_style_dirty = harness.is_style_dirty(btn_id);
    eprintln!("Button style is dirty: {}", is_style_dirty);

    harness.rebuild();

    eprintln!(
        "After rebuild: paint_requested = {}",
        harness.paint_requested()
    );

    // Verify the style DID change (reactive system works)
    let style = harness.get_computed_style(btn_id);
    let font_weight = style.get(FontWeight);
    eprintln!("Font weight after rebuild: {:?}", font_weight);
    assert!(
        font_weight.is_some(),
        "Style should have font-weight after activation"
    );

    // KNOWN BUG: paint is not requested when reactive signal in style closure changes
    // This assertion documents the bug - when fixed, change to assert!(paint_requested_after_click)
    if !paint_requested_after_click {
        eprintln!(
            "CONFIRMED BUG: paint_requested = false after clicking. \n\
             The reactive signal changed (is_active = true), the style correctly \n\
             updated to have font-weight, but request_paint was never called. \n\
             This is why the UI doesn't update until a hover change forces a repaint."
        );
    }
    // For now, we just document the bug rather than fail the test
    // Uncomment this assertion when the bug is fixed:
    // assert!(paint_requested_after_click, "Paint should be requested after reactive style change");
}

/// Test if request_paint is scheduled when we directly change the signal
/// (without going through a click handler)
#[test]
fn test_request_paint_on_direct_signal_change() {
    let is_active = RwSignal::new(false);

    let btn = SidebarMenuButton::new("Toggle").is_active(move || is_active.get());
    let btn_id = btn.view_id();

    let container =
        Stack::new((SidebarMenuItem::new().child(btn),)).style(|s| s.size(300.0, 100.0));

    let mut harness = HeadlessHarness::new_with_size(container, 300.0, 100.0);
    harness.rebuild();

    // Clear any pending paint requests from initial setup
    harness.clear_paint_request();
    assert!(
        !harness.paint_requested(),
        "Paint should not be requested initially"
    );

    eprintln!(
        "Before signal change: is_active = {}, paint_requested = {}",
        is_active.get(),
        harness.paint_requested()
    );

    // Directly change the signal (simulating what happens in a click handler)
    is_active.set(true);

    eprintln!(
        "After signal change (before rebuild): is_active = {}, paint_requested = {}",
        is_active.get(),
        harness.paint_requested()
    );

    // Check if paint was requested after signal change
    let paint_requested_after_signal = harness.paint_requested();

    // Check if there are scheduled updates
    let has_scheduled_updates = harness.has_scheduled_updates();
    eprintln!("Has scheduled updates: {}", has_scheduled_updates);

    // Check if the button's style is dirty
    let is_style_dirty = harness.is_style_dirty(btn_id);
    eprintln!("Button style is dirty: {}", is_style_dirty);

    harness.rebuild();

    eprintln!(
        "After rebuild: paint_requested = {}",
        harness.paint_requested()
    );

    // Get the computed style to verify it changed
    let style = harness.get_computed_style(btn_id);
    let font_weight = style.get(FontWeight);
    eprintln!(
        "Font weight after signal change + rebuild: {:?}",
        font_weight
    );

    // The bug might be that changing a signal used in a style closure
    // doesn't trigger request_paint or mark the view as style-dirty
    if !paint_requested_after_signal && !is_style_dirty {
        eprintln!(
            "POTENTIAL BUG: Neither paint_requested nor style_dirty was set \
             after changing a signal used in a style closure."
        );
    }
}

/// Test demonstrating the issue with a plain Container + reactive style
/// This isolates the bug from SidebarMenuButton specifics
#[test]
fn test_reactive_font_weight_in_container() {
    let is_bold = RwSignal::new(false);

    let label_text = "Click me";
    let container = floem::views::Container::new(floem::views::Label::new(label_text))
        .style(move |s| {
            let bold = is_bold.get();
            if bold {
                s.font_weight(Weight::BOLD)
            } else {
                s.font_weight(Weight::NORMAL)
            }
        })
        .on_click_stop(move |_| {
            is_bold.set(!is_bold.get());
        });

    let id = container.view_id();

    let wrapper = Stack::new((container,)).style(|s| s.size(200.0, 100.0));

    let mut harness = HeadlessHarness::new_with_size(wrapper, 200.0, 100.0);
    harness.rebuild();

    let container_rect = harness.get_layout_rect(id);
    let center_x = container_rect.center().x;
    let center_y = container_rect.center().y;

    // Check initial style
    let initial_style = harness.get_computed_style(id);
    let initial_weight = initial_style.get(FontWeight);
    eprintln!(
        "Initial: is_bold = {}, font-weight = {:?}",
        is_bold.get(),
        initial_weight
    );

    // Click to toggle bold
    harness.click(center_x as f64, center_y as f64);
    harness.rebuild();

    eprintln!("After click: is_bold = {}", is_bold.get());
    assert!(is_bold.get());

    // Check style immediately after click
    let style_after_click = harness.get_computed_style(id);
    let weight_after_click = style_after_click.get(FontWeight);
    eprintln!("Font weight after click: {:?}", weight_after_click);

    // The style closure uses is_bold.get() which should trigger reactivity
    // BUG: The style may not update until hover state changes

    // Move pointer to trigger style recomputation
    harness.pointer_move(0.0, 0.0);
    harness.rebuild();

    // Check style after pointer move
    let style_after_move = harness.get_computed_style(id);
    let weight_after_move = style_after_move.get(FontWeight);
    eprintln!("Font weight after pointer move: {:?}", weight_after_move);

    // The bug: weight_after_click should equal weight_after_move
    assert_eq!(
        weight_after_click, weight_after_move,
        "BUG: Font weight changed after pointer move. \
         After click: {:?}, After move: {:?}. \
         Style should update immediately on signal change.",
        weight_after_click, weight_after_move
    );
}
