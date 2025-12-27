//! Tests for Button component layout and sizing
//!
//! These tests verify that buttons have correct dimensions based on their size variant.
//! Expected sizing (from floem-tailwind):
//! - Sm: height 36px (h_9), padding-x 12px (px_3)
//! - Default: height 40px (h_10), padding-x 16px (px_4), padding-y 8px (py_2)
//! - Lg: height 44px (h_11), padding-x 32px (px_8)
//! - Icon: height 40px (h_10), width 40px (w_10)

use floem::prelude::*;
use floem_shadcn::components::button::Button;
use floem_test::prelude::*;

// =============================================================================
// Button Size Tests
// =============================================================================

#[test]
fn test_button_default_height() {
    let button = Button::new("Click");
    let id = button.view_id();

    let container = Stack::new((button,)).style(|s| s.size(400.0, 200.0));

    let mut harness = HeadlessHarness::new_with_size(container, 400.0, 200.0);
    harness.rebuild();

    let layout = id.get_layout().expect("Button layout should exist");

    eprintln!("Default button size: {}x{}", layout.size.width, layout.size.height);

    // Default button should have h_10 = 40px height
    assert!(
        (layout.size.height - 40.0).abs() < 0.1,
        "Default button height should be 40.0 (h_10), got {}",
        layout.size.height
    );
}

#[test]
fn test_button_sm_height() {
    let button = Button::new("Small").sm();
    let id = button.view_id();

    let container = Stack::new((button,)).style(|s| s.size(400.0, 200.0));

    let mut harness = HeadlessHarness::new_with_size(container, 400.0, 200.0);
    harness.rebuild();

    let layout = id.get_layout().expect("Button layout should exist");

    eprintln!("Small button size: {}x{}", layout.size.width, layout.size.height);

    // Small button should have h_9 = 36px height
    assert!(
        (layout.size.height - 36.0).abs() < 0.1,
        "Small button height should be 36.0 (h_9), got {}",
        layout.size.height
    );
}

#[test]
fn test_button_lg_height() {
    let button = Button::new("Large").lg();
    let id = button.view_id();

    let container = Stack::new((button,)).style(|s| s.size(400.0, 200.0));

    let mut harness = HeadlessHarness::new_with_size(container, 400.0, 200.0);
    harness.rebuild();

    let layout = id.get_layout().expect("Button layout should exist");

    eprintln!("Large button size: {}x{}", layout.size.width, layout.size.height);

    // Large button should have h_11 = 44px height
    assert!(
        (layout.size.height - 44.0).abs() < 0.1,
        "Large button height should be 44.0 (h_11), got {}",
        layout.size.height
    );
}

#[test]
fn test_button_icon_size() {
    let button = Button::new("X").icon();
    let id = button.view_id();

    let container = Stack::new((button,)).style(|s| s.size(400.0, 200.0));

    let mut harness = HeadlessHarness::new_with_size(container, 400.0, 200.0);
    harness.rebuild();

    let layout = id.get_layout().expect("Button layout should exist");

    eprintln!("Icon button size: {}x{}", layout.size.width, layout.size.height);

    // Icon button should be square: 40x40 (h_10, w_10)
    assert!(
        (layout.size.height - 40.0).abs() < 0.1,
        "Icon button height should be 40.0 (h_10), got {}",
        layout.size.height
    );
    assert!(
        (layout.size.width - 40.0).abs() < 0.1,
        "Icon button width should be 40.0 (w_10), got {}",
        layout.size.width
    );
}

// =============================================================================
// Button Width Tests (content + padding)
// =============================================================================

#[test]
fn test_button_sm_has_minimum_width_from_padding() {
    // Small button with minimal content should still have padding-x applied
    // px_3 = 12px on each side = 24px total horizontal padding
    let button = Button::new("X").sm();
    let id = button.view_id();

    let container = Stack::new((button,)).style(|s| s.size(400.0, 200.0));

    let mut harness = HeadlessHarness::new_with_size(container, 400.0, 200.0);
    harness.rebuild();

    let layout = id.get_layout().expect("Button layout should exist");

    eprintln!("Small button 'X' width: {}", layout.size.width);

    // Width should be at least 24px (just the padding)
    // Plus text width - even a single char should make it wider
    assert!(
        layout.size.width >= 24.0,
        "Small button width should be at least 24.0 (px_3 * 2 padding), got {}",
        layout.size.width
    );
}

#[test]
fn test_button_default_has_minimum_width_from_padding() {
    // Default button with minimal content
    // px_4 = 16px on each side = 32px total horizontal padding
    let button = Button::new("X");
    let id = button.view_id();

    let container = Stack::new((button,)).style(|s| s.size(400.0, 200.0));

    let mut harness = HeadlessHarness::new_with_size(container, 400.0, 200.0);
    harness.rebuild();

    let layout = id.get_layout().expect("Button layout should exist");

    eprintln!("Default button 'X' width: {}", layout.size.width);

    // Width should be at least 32px (just the padding)
    assert!(
        layout.size.width >= 32.0,
        "Default button width should be at least 32.0 (px_4 * 2 padding), got {}",
        layout.size.width
    );
}

#[test]
fn test_button_lg_has_minimum_width_from_padding() {
    // Large button with minimal content
    // px_8 = 32px on each side = 64px total horizontal padding
    let button = Button::new("X").lg();
    let id = button.view_id();

    let container = Stack::new((button,)).style(|s| s.size(400.0, 200.0));

    let mut harness = HeadlessHarness::new_with_size(container, 400.0, 200.0);
    harness.rebuild();

    let layout = id.get_layout().expect("Button layout should exist");

    eprintln!("Large button 'X' width: {}", layout.size.width);

    // Width should be at least 64px (just the padding)
    assert!(
        layout.size.width >= 64.0,
        "Large button width should be at least 64.0 (px_8 * 2 padding), got {}",
        layout.size.width
    );
}

// =============================================================================
// Button Layout in Horizontal Stack Tests
// =============================================================================

#[test]
fn test_buttons_in_horizontal_stack_with_gap() {
    let btn1 = Button::new("First");
    let btn1_id = btn1.view_id();
    let btn2 = Button::new("Second");
    let btn2_id = btn2.view_id();

    let container = Stack::horizontal((btn1, btn2))
        .style(|s| s.gap(8.0).size(400.0, 200.0)); // gap_2 = 8px

    let mut harness = HeadlessHarness::new_with_size(container, 400.0, 200.0);
    harness.rebuild();

    let layout1 = btn1_id.get_layout().expect("Button 1 layout should exist");
    let layout2 = btn2_id.get_layout().expect("Button 2 layout should exist");

    eprintln!("Button 1 position: ({}, {}), size: {}x{}",
        layout1.location.x, layout1.location.y, layout1.size.width, layout1.size.height);
    eprintln!("Button 2 position: ({}, {}), size: {}x{}",
        layout2.location.x, layout2.location.y, layout2.size.width, layout2.size.height);

    // Second button should be positioned after first button + gap
    let expected_x = layout1.size.width + 8.0;
    assert!(
        (layout2.location.x - expected_x).abs() < 0.1,
        "Second button x should be {} (first width {} + 8 gap), got {}",
        expected_x, layout1.size.width, layout2.location.x
    );

    // Both buttons should have same y position (aligned at top)
    assert!(
        (layout1.location.y - layout2.location.y).abs() < 0.1,
        "Buttons should have same y position, got {} and {}",
        layout1.location.y, layout2.location.y
    );
}

#[test]
fn test_buttons_centered_in_horizontal_stack() {
    let btn1 = Button::new("Small").sm();
    let btn1_id = btn1.view_id();
    let btn2 = Button::new("Large").lg();
    let btn2_id = btn2.view_id();

    let container = Stack::horizontal((btn1, btn2))
        .style(|s| s.gap(8.0).items_center().size(400.0, 200.0));

    let mut harness = HeadlessHarness::new_with_size(container, 400.0, 200.0);
    harness.rebuild();

    let layout1 = btn1_id.get_layout().expect("Small button layout should exist");
    let layout2 = btn2_id.get_layout().expect("Large button layout should exist");

    eprintln!("Small button: pos ({}, {}), size {}x{}",
        layout1.location.x, layout1.location.y, layout1.size.width, layout1.size.height);
    eprintln!("Large button: pos ({}, {}), size {}x{}",
        layout2.location.x, layout2.location.y, layout2.size.width, layout2.size.height);

    // With items_center, both buttons should be centered vertically
    // Small (36px) and Large (44px) should have different y positions to center them
    let center_y1 = layout1.location.y + layout1.size.height / 2.0;
    let center_y2 = layout2.location.y + layout2.size.height / 2.0;

    assert!(
        (center_y1 - center_y2).abs() < 0.1,
        "Buttons should be vertically centered at same point, got centers at {} and {}",
        center_y1, center_y2
    );
}

// =============================================================================
// Button Size Comparison Tests
// =============================================================================

#[test]
fn test_button_sizes_are_ordered_horizontal() {
    // Use horizontal stack to test intrinsic widths (no cross-axis stretching)
    let sm = Button::new("Click").sm();
    let sm_id = sm.view_id();
    let default = Button::new("Click");
    let default_id = default.view_id();
    let lg = Button::new("Click").lg();
    let lg_id = lg.view_id();

    let container = Stack::horizontal((sm, default, lg))
        .style(|s| s.gap(8.0).size(600.0, 100.0));

    let mut harness = HeadlessHarness::new_with_size(container, 600.0, 100.0);
    harness.rebuild();

    let sm_layout = sm_id.get_layout().expect("Small button layout should exist");
    let default_layout = default_id.get_layout().expect("Default button layout should exist");
    let lg_layout = lg_id.get_layout().expect("Large button layout should exist");

    eprintln!("Size comparison (horizontal stack):");
    eprintln!("  Small: {}x{}", sm_layout.size.width, sm_layout.size.height);
    eprintln!("  Default: {}x{}", default_layout.size.width, default_layout.size.height);
    eprintln!("  Large: {}x{}", lg_layout.size.width, lg_layout.size.height);

    // Heights should be ordered: sm < default < lg
    assert!(
        sm_layout.size.height < default_layout.size.height,
        "Small height ({}) should be less than default height ({})",
        sm_layout.size.height, default_layout.size.height
    );
    assert!(
        default_layout.size.height < lg_layout.size.height,
        "Default height ({}) should be less than large height ({})",
        default_layout.size.height, lg_layout.size.height
    );

    // Widths should also follow the pattern due to different padding
    // sm has px_3 (12), default has px_4 (16), lg has px_8 (32)
    assert!(
        sm_layout.size.width < default_layout.size.width,
        "Small width ({}) should be less than default width ({})",
        sm_layout.size.width, default_layout.size.width
    );
    assert!(
        default_layout.size.width < lg_layout.size.width,
        "Default width ({}) should be less than large width ({})",
        default_layout.size.width, lg_layout.size.width
    );
}

/// Verify that buttons maintain their intrinsic width in a vertical stack
/// and don't stretch to fill the container's cross-axis.
#[test]
fn test_button_does_not_stretch_in_vertical_stack() {
    let button = Button::new("Short");
    let id = button.view_id();

    // In a vertical stack with 400px width, button should NOT stretch
    let container = Stack::vertical((button,))
        .style(|s| s.size(400.0, 200.0));

    let mut harness = HeadlessHarness::new_with_size(container, 400.0, 200.0);
    harness.rebuild();

    let layout = id.get_layout().expect("Button layout should exist");

    eprintln!("Button in vertical stack: {}x{}", layout.size.width, layout.size.height);

    // Button should maintain intrinsic width based on content + padding (~100px for "Short")
    // NOT stretch to fill container width (400px)
    assert!(
        layout.size.width < 200.0,
        "Button should maintain intrinsic width (~100px), got {} (should NOT stretch to container width)",
        layout.size.width
    );

    // Height should still be correct
    assert!(
        (layout.size.height - 40.0).abs() < 0.1,
        "Button height should be 40 (default size), got {}",
        layout.size.height
    );
}

/// Test that buttons maintain intrinsic width in showcase-style layout
#[test]
fn test_showcase_demo_section_layout() {
    // This simulates the structure from the showcase demo:
    // demo_section -> Stack::vertical -> subsection -> Stack::horizontal -> buttons

    let btn1 = Button::new("Default");
    let btn1_id = btn1.view_id();
    let btn2 = Button::new("Secondary").secondary();
    let btn2_id = btn2.view_id();

    // Inner horizontal stack (like in the showcase)
    let button_row = Stack::horizontal((btn1, btn2))
        .style(|s| s.gap(8.0).flex_wrap(floem::style::FlexWrap::Wrap));

    // Outer vertical stack (simulating demo_section)
    let container = Stack::vertical((button_row,))
        .style(|s| s.gap(16.0).size(600.0, 400.0));

    let mut harness = HeadlessHarness::new_with_size(container, 600.0, 400.0);
    harness.rebuild();

    let layout1 = btn1_id.get_layout().expect("Button 1 layout should exist");
    let layout2 = btn2_id.get_layout().expect("Button 2 layout should exist");

    eprintln!("Showcase-style demo section layout:");
    eprintln!("  Button 1: pos ({}, {}), size {}x{}",
        layout1.location.x, layout1.location.y, layout1.size.width, layout1.size.height);
    eprintln!("  Button 2: pos ({}, {}), size {}x{}",
        layout2.location.x, layout2.location.y, layout2.size.width, layout2.size.height);

    // Check if buttons maintain proper height
    assert!(
        (layout1.size.height - 40.0).abs() < 0.1,
        "Default button height should be 40, got {}",
        layout1.size.height
    );
    assert!(
        (layout2.size.height - 40.0).abs() < 0.1,
        "Secondary button height should be 40, got {}",
        layout2.size.height
    );

    // In a properly laid out showcase, buttons should have their intrinsic width
    // not stretch to fill the container
    let total_btn_width = layout1.size.width + layout2.size.width + 8.0; // plus gap
    eprintln!("Total button row width: {} (container: 600)", total_btn_width);

    // If buttons are stretching, their widths would be close to 600/2 = 300 each
    // If they're correct, they should be around 100-150px each
    assert!(
        layout1.size.width < 200.0,
        "Default button should not stretch - expected ~100-150px, got {}",
        layout1.size.width
    );
}

/// Test with exact showcase structure including w_full()
#[test]
fn test_exact_showcase_structure() {
    use floem::text::Weight;
    use floem_tailwind::TailwindExt;

    let btn1 = Button::new("Default");
    let btn1_id = btn1.view_id();
    let btn2 = Button::new("Secondary").secondary();
    let btn2_id = btn2.view_id();

    // Exact structure from showcase.rs:
    // subsection = Stack::vertical((title, content))
    let subsection = Stack::vertical((
        floem::views::Label::new("Variants")
            .style(|s| s.font_size(14.0).font_weight(Weight::MEDIUM).mb_3()),
        Stack::horizontal((btn1, btn2))
            .style(|s| s.gap_2().flex_wrap(floem::style::FlexWrap::Wrap)),
    ));

    // demo_section = Stack::vertical((title, desc, content)).style(|s| s.w_full())
    let demo_section = Stack::vertical((
        floem::views::Label::new("Buttons")
            .style(|s| s.font_size(24.0).font_weight(Weight::BOLD).mb_2()),
        floem::views::Label::new("Description")
            .style(|s| s.font_size(14.0).mb_6()),
        Stack::vertical((subsection,)).style(|s| s.gap_8()),
    ))
    .style(|s| s.w_full());

    // Container
    let container = Stack::new((demo_section,))
        .style(|s| s.size(800.0, 600.0));

    let mut harness = HeadlessHarness::new_with_size(container, 800.0, 600.0);
    harness.rebuild();

    let layout1 = btn1_id.get_layout().expect("Button 1 layout should exist");
    let layout2 = btn2_id.get_layout().expect("Button 2 layout should exist");

    eprintln!("Exact showcase structure:");
    eprintln!("  Button 1: pos ({}, {}), size {}x{}",
        layout1.location.x, layout1.location.y, layout1.size.width, layout1.size.height);
    eprintln!("  Button 2: pos ({}, {}), size {}x{}",
        layout2.location.x, layout2.location.y, layout2.size.width, layout2.size.height);

    // Buttons should NOT stretch to fill container
    assert!(
        layout1.size.width < 200.0,
        "Button 1 should not stretch - expected ~100-150px, got {}",
        layout1.size.width
    );
    assert!(
        layout2.size.width < 200.0,
        "Button 2 should not stretch - expected ~100-150px, got {}",
        layout2.size.width
    );
}

/// Test with Scroll wrapper like actual showcase
#[test]
fn test_showcase_with_scroll() {
    use floem::text::Weight;
    use floem_tailwind::TailwindExt;

    let btn1 = Button::new("Default");
    let btn1_id = btn1.view_id();
    let btn2 = Button::new("Secondary").secondary();
    let btn2_id = btn2.view_id();

    // subsection
    let subsection = Stack::vertical((
        floem::views::Label::new("Variants")
            .style(|s| s.font_size(14.0).font_weight(Weight::MEDIUM).mb_3()),
        Stack::horizontal((btn1, btn2))
            .style(|s| s.gap_2().flex_wrap(floem::style::FlexWrap::Wrap)),
    ));

    // demo_section
    let demo_section = Stack::vertical((
        floem::views::Label::new("Buttons")
            .style(|s| s.font_size(24.0).font_weight(Weight::BOLD).mb_2()),
        floem::views::Label::new("Description")
            .style(|s| s.font_size(14.0).mb_6()),
        Stack::vertical((subsection,)).style(|s| s.gap_8()),
    ))
    .style(|s| s.w_full());

    // Wrap in Scroll like the actual showcase
    let scroll_content = floem::views::Scroll::new(demo_section)
        .style(|s| s.flex_grow(1.0).h_full().p_8());

    // Container (simulating the horizontal layout with sidebar)
    let container = Stack::horizontal((scroll_content,))
        .style(|s| s.size(800.0, 600.0));

    let mut harness = HeadlessHarness::new_with_size(container, 800.0, 600.0);
    harness.rebuild();

    let layout1 = btn1_id.get_layout().expect("Button 1 layout should exist");
    let layout2 = btn2_id.get_layout().expect("Button 2 layout should exist");

    eprintln!("Showcase with Scroll:");
    eprintln!("  Button 1: pos ({}, {}), size {}x{}",
        layout1.location.x, layout1.location.y, layout1.size.width, layout1.size.height);
    eprintln!("  Button 2: pos ({}, {}), size {}x{}",
        layout2.location.x, layout2.location.y, layout2.size.width, layout2.size.height);

    // Buttons should NOT stretch to fill container
    assert!(
        layout1.size.width < 200.0,
        "Button 1 should not stretch - expected ~100-150px, got {}",
        layout1.size.width
    );
    assert!(
        layout2.size.width < 200.0,
        "Button 2 should not stretch - expected ~100-150px, got {}",
        layout2.size.width
    );
}

/// Test button heights in showcase-style nested layout
#[test]
fn test_button_height_in_nested_layout() {
    let sm = Button::new("Small").sm();
    let sm_id = sm.view_id();
    let default = Button::new("Default");
    let default_id = default.view_id();
    let lg = Button::new("Large").lg();
    let lg_id = lg.view_id();

    // Nested layout similar to showcase
    let inner = Stack::horizontal((sm, default, lg))
        .style(|s| s.gap(8.0).items_center());

    let outer = Stack::vertical((inner,))
        .style(|s| s.gap(32.0).size(600.0, 400.0));

    let mut harness = HeadlessHarness::new_with_size(outer, 600.0, 400.0);
    harness.rebuild();

    let sm_layout = sm_id.get_layout().expect("Small layout");
    let default_layout = default_id.get_layout().expect("Default layout");
    let lg_layout = lg_id.get_layout().expect("Large layout");

    eprintln!("Nested layout heights:");
    eprintln!("  Small: {}", sm_layout.size.height);
    eprintln!("  Default: {}", default_layout.size.height);
    eprintln!("  Large: {}", lg_layout.size.height);

    // Verify each button maintains its correct height
    assert!(
        (sm_layout.size.height - 36.0).abs() < 0.1,
        "Small button height should be 36 (h_9), got {}",
        sm_layout.size.height
    );
    assert!(
        (default_layout.size.height - 40.0).abs() < 0.1,
        "Default button height should be 40 (h_10), got {}",
        default_layout.size.height
    );
    assert!(
        (lg_layout.size.height - 44.0).abs() < 0.1,
        "Large button height should be 44 (h_11), got {}",
        lg_layout.size.height
    );
}

// =============================================================================
// Variant Tests (all variants should have same dimensions for same size)
// =============================================================================

#[test]
fn test_all_variants_have_same_dimensions() {
    let default_btn = Button::new("Click");
    let default_id = default_btn.view_id();
    let secondary = Button::new("Click").secondary();
    let secondary_id = secondary.view_id();
    let destructive = Button::new("Click").destructive();
    let destructive_id = destructive.view_id();
    let outline = Button::new("Click").outline();
    let outline_id = outline.view_id();
    let ghost = Button::new("Click").ghost();
    let ghost_id = ghost.view_id();
    let link = Button::new("Click").link();
    let link_id = link.view_id();

    let container = Stack::vertical((
        default_btn,
        secondary,
        destructive,
        outline,
        ghost,
        link,
    ))
    .style(|s| s.gap(8.0).size(400.0, 600.0));

    let mut harness = HeadlessHarness::new_with_size(container, 400.0, 600.0);
    harness.rebuild();

    let default_layout = default_id.get_layout().expect("Layout should exist");
    let secondary_layout = secondary_id.get_layout().expect("Layout should exist");
    let destructive_layout = destructive_id.get_layout().expect("Layout should exist");
    let outline_layout = outline_id.get_layout().expect("Layout should exist");
    let ghost_layout = ghost_id.get_layout().expect("Layout should exist");
    let link_layout = link_id.get_layout().expect("Layout should exist");

    eprintln!("Variant sizes:");
    eprintln!("  Default: {}x{}", default_layout.size.width, default_layout.size.height);
    eprintln!("  Secondary: {}x{}", secondary_layout.size.width, secondary_layout.size.height);
    eprintln!("  Destructive: {}x{}", destructive_layout.size.width, destructive_layout.size.height);
    eprintln!("  Outline: {}x{}", outline_layout.size.width, outline_layout.size.height);
    eprintln!("  Ghost: {}x{}", ghost_layout.size.width, ghost_layout.size.height);
    eprintln!("  Link: {}x{}", link_layout.size.width, link_layout.size.height);

    // All variants should have the same height
    let expected_height = 40.0; // h_10
    for (name, layout) in [
        ("Default", &default_layout),
        ("Secondary", &secondary_layout),
        ("Destructive", &destructive_layout),
        ("Outline", &outline_layout),
        ("Ghost", &ghost_layout),
        ("Link", &link_layout),
    ] {
        assert!(
            (layout.size.height - expected_height).abs() < 0.1,
            "{} variant height should be {}, got {}",
            name, expected_height, layout.size.height
        );
    }

    // All variants should have the same width (same text, same padding)
    let expected_width = default_layout.size.width;
    for (name, layout) in [
        ("Secondary", &secondary_layout),
        ("Destructive", &destructive_layout),
        ("Outline", &outline_layout),
        ("Ghost", &ghost_layout),
        ("Link", &link_layout),
    ] {
        assert!(
            (layout.size.width - expected_width).abs() < 0.1,
            "{} variant width should be {}, got {}",
            name, expected_width, layout.size.width
        );
    }
}

// =============================================================================
// Showcase-like Layout Tests
// =============================================================================

#[test]
fn test_showcase_variants_layout() {
    // Mimics the showcase demo: horizontal stack with flex_wrap
    // Using wider container (800px) to fit all buttons on one row
    let btn1 = Button::new("Default");
    let btn1_id = btn1.view_id();
    let btn2 = Button::new("Secondary").secondary();
    let btn2_id = btn2.view_id();
    let btn3 = Button::new("Destructive").destructive();
    let btn3_id = btn3.view_id();
    let btn4 = Button::new("Outline").outline();
    let btn4_id = btn4.view_id();
    let btn5 = Button::new("Ghost").ghost();
    let btn5_id = btn5.view_id();
    let btn6 = Button::new("Link").link();
    let btn6_id = btn6.view_id();

    // 800px wide to ensure all buttons fit on one row
    let container = Stack::horizontal((btn1, btn2, btn3, btn4, btn5, btn6))
        .style(|s| s.gap(8.0).flex_wrap(floem::style::FlexWrap::Wrap).size(800.0, 200.0));

    let mut harness = HeadlessHarness::new_with_size(container, 800.0, 200.0);
    harness.rebuild();

    let layouts = [
        ("Default", btn1_id.get_layout().expect("Layout should exist")),
        ("Secondary", btn2_id.get_layout().expect("Layout should exist")),
        ("Destructive", btn3_id.get_layout().expect("Layout should exist")),
        ("Outline", btn4_id.get_layout().expect("Layout should exist")),
        ("Ghost", btn5_id.get_layout().expect("Layout should exist")),
        ("Link", btn6_id.get_layout().expect("Layout should exist")),
    ];

    eprintln!("Showcase variants layout:");
    let mut total_width = 0.0;
    for (name, layout) in &layouts {
        eprintln!("  {}: pos ({}, {}), size {}x{}",
            name, layout.location.x, layout.location.y, layout.size.width, layout.size.height);
        total_width += layout.size.width;
    }
    total_width += 5.0 * 8.0; // 5 gaps between 6 buttons
    eprintln!("Total width needed: {}px", total_width);

    // All buttons on same row should have y = 0 (assuming they fit on one row)
    let first_y = layouts[0].1.location.y;
    for (name, layout) in &layouts {
        assert!(
            (layout.location.y - first_y).abs() < 0.1,
            "{} should be on same row as first button (y={}), got y={}",
            name, first_y, layout.location.y
        );
    }

    // Buttons should be laid out sequentially with gaps
    let mut expected_x = 0.0;
    for (name, layout) in layouts.iter() {
        assert!(
            (layout.location.x - expected_x).abs() < 1.0, // Allow small tolerance
            "{} should be at x={}, got x={}",
            name, expected_x, layout.location.x
        );
        expected_x += layout.size.width + 8.0; // width + gap

        // Verify each button has correct height
        assert!(
            (layout.size.height - 40.0).abs() < 0.1,
            "{} should have height 40, got {}",
            name, layout.size.height
        );
    }
}

#[test]
fn test_showcase_sizes_layout() {
    // Mimics the showcase demo: horizontal stack with items_center
    let sm = Button::new("Small").sm();
    let sm_id = sm.view_id();
    let default = Button::new("Default");
    let default_id = default.view_id();
    let lg = Button::new("Large").lg();
    let lg_id = lg.view_id();

    let container = Stack::horizontal((sm, default, lg))
        .style(|s| s.gap(8.0).items_center().size(600.0, 200.0));

    let mut harness = HeadlessHarness::new_with_size(container, 600.0, 200.0);
    harness.rebuild();

    let sm_layout = sm_id.get_layout().expect("Small layout should exist");
    let default_layout = default_id.get_layout().expect("Default layout should exist");
    let lg_layout = lg_id.get_layout().expect("Large layout should exist");

    eprintln!("Showcase sizes layout:");
    eprintln!("  Small: pos ({}, {}), size {}x{}",
        sm_layout.location.x, sm_layout.location.y, sm_layout.size.width, sm_layout.size.height);
    eprintln!("  Default: pos ({}, {}), size {}x{}",
        default_layout.location.x, default_layout.location.y, default_layout.size.width, default_layout.size.height);
    eprintln!("  Large: pos ({}, {}), size {}x{}",
        lg_layout.location.x, lg_layout.location.y, lg_layout.size.width, lg_layout.size.height);

    // Verify heights
    assert!(
        (sm_layout.size.height - 36.0).abs() < 0.1,
        "Small should have height 36, got {}",
        sm_layout.size.height
    );
    assert!(
        (default_layout.size.height - 40.0).abs() < 0.1,
        "Default should have height 40, got {}",
        default_layout.size.height
    );
    assert!(
        (lg_layout.size.height - 44.0).abs() < 0.1,
        "Large should have height 44, got {}",
        lg_layout.size.height
    );

    // With items_center, all buttons should be vertically centered
    let center_sm = sm_layout.location.y + sm_layout.size.height / 2.0;
    let center_default = default_layout.location.y + default_layout.size.height / 2.0;
    let center_lg = lg_layout.location.y + lg_layout.size.height / 2.0;

    assert!(
        (center_sm - center_default).abs() < 0.1,
        "Small and Default should be centered at same y, got {} and {}",
        center_sm, center_default
    );
    assert!(
        (center_default - center_lg).abs() < 0.1,
        "Default and Large should be centered at same y, got {} and {}",
        center_default, center_lg
    );
}
