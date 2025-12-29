//! Integration tests for the Tabs component using floem-test.
//!
//! These tests verify that:
//! 1. Tabs with flex-grow actually stretch to fill space equally
//! 2. The layout calculations match our expected behavior
//! 3. Tab heights match the shadcn/ui specifications

use floem::prelude::*;
use floem_shadcn::components::tabs::{Tab, Tabs, TabsContent, TabsList};
use floem_test::prelude::*;

#[test]
fn test_tabs_flex_grow_equal_distribution() {
    // Create tabs with flex_grow(1.0) - they should divide space equally
    let active = RwSignal::new("tab1".to_string());

    let tab1 = Tab::new("tab1", "Tab 1").active(active);
    let tab1_id = tab1.view_id();

    let tab2 = Tab::new("tab2", "Tab 2").active(active);
    let tab2_id = tab2.view_id();

    let tab3 = Tab::new("tab3", "Tab 3").active(active);
    let tab3_id = tab3.view_id();

    let tabs_list = TabsList::new((tab1, tab2, tab3));
    let tabs_list_id = tabs_list.view_id();

    // Apply width_full() directly to TabsList
    let tabs_list_styled = tabs_list.style(|s| s.width_full());

    // Wrap in a container to give it explicit width
    let container = Stack::new((tabs_list_styled,)).style(|s| s.width(300.0));

    // Container is 300px wide
    // TabsList has: 3px padding left + 3px padding right = 6px
    // Gaps: 3px between tab1-tab2, 3px between tab2-tab3 = 6px total
    // Available for tabs: 300 - 6 - 6 = 288px
    // Each tab should get: 288 / 3 = 96px

    let mut harness = HeadlessHarness::new_with_size(container, 400.0, 200.0);
    harness.rebuild();

    let tabs_list_layout = tabs_list_id
        .get_layout()
        .expect("TabsList layout should exist");
    let layout1 = tab1_id.get_layout().expect("Tab 1 layout should exist");
    let layout2 = tab2_id.get_layout().expect("Tab 2 layout should exist");
    let layout3 = tab3_id.get_layout().expect("Tab 3 layout should exist");

    // Debug output
    println!("TabsList width: {}", tabs_list_layout.size.width);
    println!("Tab 1 width: {}", layout1.size.width);
    println!("Tab 2 width: {}", layout2.size.width);
    println!("Tab 3 width: {}", layout3.size.width);

    // Verify TabsList actually takes the full container width
    assert!(
        (tabs_list_layout.size.width - 300.0).abs() < 1.0,
        "TabsList should fill container width (300px), got {}",
        tabs_list_layout.size.width
    );

    // Verify tabs fill the TabsList space:
    // TabsList width - padding (6px) - gaps (6px) = available for tabs
    let padding = 6.0; // 3px left + 3px right
    let gaps = 6.0; // 3px gap * 2 gaps
    let available_width = tabs_list_layout.size.width - padding - gaps;
    let total_tab_width = layout1.size.width + layout2.size.width + layout3.size.width;

    assert!(
        (total_tab_width - available_width).abs() < 1.0,
        "Tabs should fill available space. Available: {}, Total tab width: {}",
        available_width,
        total_tab_width
    );

    // Verify equal distribution: all three tabs should have EQUAL width due to flex_grow(1.0)
    assert!(
        (layout1.size.width - layout2.size.width).abs() < 0.1,
        "Tab 1 and Tab 2 should have equal width. Got {} and {}",
        layout1.size.width,
        layout2.size.width
    );

    assert!(
        (layout2.size.width - layout3.size.width).abs() < 0.1,
        "Tab 2 and Tab 3 should have equal width. Got {} and {}",
        layout2.size.width,
        layout3.size.width
    );
}

#[test]
fn test_tabs_height() {
    // Test that tabs have the correct height
    // TabsList: h-9 (36px)
    // Tab: h-[calc(100%-1px)] ≈ 29px (36 - 6 padding - 1)

    let active = RwSignal::new("tab1".to_string());

    let tab1 = Tab::new("tab1", "Tab 1").active(active);
    let tab1_id = tab1.view_id();

    let tabs_list = TabsList::new((tab1,));
    let tabs_list_id = tabs_list.view_id();

    let container = Stack::new((tabs_list,)).style(|s| s.width(300.0));

    let mut harness = HeadlessHarness::new_with_size(container, 400.0, 200.0);
    harness.rebuild();

    let list_layout = tabs_list_id
        .get_layout()
        .expect("TabsList layout should exist");
    let tab_layout = tab1_id.get_layout().expect("Tab layout should exist");

    println!("TabsList height: {}", list_layout.size.height);
    println!("Tab height: {}", tab_layout.size.height);

    // TabsList should be 36px (h-9)
    assert!(
        (list_layout.size.height - 36.0).abs() < 1.0,
        "TabsList height should be 36px, got {}",
        list_layout.size.height
    );

    // Tab should be 29px (h-[calc(100%-1px)])
    assert!(
        (tab_layout.size.height - 29.0).abs() < 1.0,
        "Tab height should be 29px, got {}",
        tab_layout.size.height
    );
}

#[test]
fn test_two_tabs_equal_distribution() {
    // Test with 2 tabs - should divide space equally
    let active = RwSignal::new("tab1".to_string());

    let tab1 = Tab::new("tab1", "Short").active(active);
    let tab1_id = tab1.view_id();

    let tab2 = Tab::new("tab2", "Very Long Label").active(active);
    let tab2_id = tab2.view_id();

    let tabs_list = TabsList::new((tab1, tab2));
    let tabs_list_id = tabs_list.view_id();

    // Apply width_full() directly to TabsList
    let tabs_list_styled = tabs_list.style(|s| s.width_full());

    let container = Stack::new((tabs_list_styled,)).style(|s| s.width(300.0));

    // Container: 300px
    // Padding: 6px (3px left + 3px right)
    // Gap: 3px (one gap between two tabs)
    // Available: 300 - 6 - 3 = 291px
    // Each tab: 291 / 2 = 145.5px

    let mut harness = HeadlessHarness::new_with_size(container, 400.0, 200.0);
    harness.rebuild();

    let tabs_list_layout = tabs_list_id.get_layout().expect("TabsList should exist");
    let layout1 = tab1_id.get_layout().expect("Tab 1 layout should exist");
    let layout2 = tab2_id.get_layout().expect("Tab 2 layout should exist");

    // Debug output
    println!("TabsList width: {}", tabs_list_layout.size.width);
    println!("Short tab width: {}", layout1.size.width);
    println!("Long tab width: {}", layout2.size.width);

    // Verify TabsList actually takes the full container width
    assert!(
        (tabs_list_layout.size.width - 300.0).abs() < 1.0,
        "TabsList should fill container width (300px), got {}",
        tabs_list_layout.size.width
    );

    // Verify tabs fill the TabsList space:
    // TabsList width - padding (6px) - gaps (3px for 1 gap) = available for tabs
    let padding = 6.0; // 3px left + 3px right
    let gaps = 3.0; // 3px gap * 1 gap (between 2 tabs)
    let available_width = tabs_list_layout.size.width - padding - gaps;
    let total_tab_width = layout1.size.width + layout2.size.width;

    assert!(
        (total_tab_width - available_width).abs() < 1.0,
        "Tabs should fill available space. Available: {}, Total tab width: {}",
        available_width,
        total_tab_width
    );

    // Verify equal distribution: both tabs should have EQUAL width despite different text lengths
    // This is the KEY test that verifies flex_grow(1.0) is working correctly
    assert!(
        (layout1.size.width - layout2.size.width).abs() < 0.1,
        "Both tabs should have equal width regardless of text length. Got {} and {}",
        layout1.size.width,
        layout2.size.width
    );
}

#[test]
fn test_four_tabs_equal_distribution() {
    // Test with 4 tabs
    let active = RwSignal::new("tab1".to_string());

    let tab1 = Tab::new("tab1", "1").active(active);
    let tab1_id = tab1.view_id();

    let tab2 = Tab::new("tab2", "2").active(active);
    let tab2_id = tab2.view_id();

    let tab3 = Tab::new("tab3", "3").active(active);
    let tab3_id = tab3.view_id();

    let tab4 = Tab::new("tab4", "4").active(active);
    let tab4_id = tab4.view_id();

    let tabs_list = TabsList::new((tab1, tab2, tab3, tab4));
    let tabs_list_id = tabs_list.view_id();

    // Apply width_full() directly to TabsList
    let tabs_list_styled = tabs_list.style(|s| s.width_full());

    let container = Stack::new((tabs_list_styled,)).style(|s| s.width(400.0));

    // Container: 400px
    // Padding: 6px (3 + 3)
    // Gaps: 9px (3 gaps of 3px each)
    // Available: 400 - 6 - 9 = 385px
    // Each tab: 385 / 4 = 96.25px

    let mut harness = HeadlessHarness::new_with_size(container, 500.0, 200.0);
    harness.rebuild();

    let tabs_list_layout = tabs_list_id
        .get_layout()
        .expect("TabsList layout should exist");
    let layout1 = tab1_id.get_layout().expect("Tab 1 layout should exist");
    let layout2 = tab2_id.get_layout().expect("Tab 2 layout should exist");
    let layout3 = tab3_id.get_layout().expect("Tab 3 layout should exist");
    let layout4 = tab4_id.get_layout().expect("Tab 4 layout should exist");

    // Debug output
    println!("TabsList width: {}", tabs_list_layout.size.width);
    println!(
        "Tab widths: {}, {}, {}, {}",
        layout1.size.width, layout2.size.width, layout3.size.width, layout4.size.width
    );

    // Verify TabsList actually takes the full container width
    assert!(
        (tabs_list_layout.size.width - 400.0).abs() < 1.0,
        "TabsList should fill container width (400px), got {}",
        tabs_list_layout.size.width
    );

    // Verify tabs fill the TabsList space:
    // TabsList width - padding (6px) - gaps (9px for 3 gaps) = available for tabs
    let padding = 6.0; // 3px left + 3px right
    let gaps = 9.0; // 3px gap * 3 gaps (between 4 tabs)
    let available_width = tabs_list_layout.size.width - padding - gaps;
    let total_tab_width =
        layout1.size.width + layout2.size.width + layout3.size.width + layout4.size.width;

    assert!(
        (total_tab_width - available_width).abs() < 1.0,
        "Tabs should fill available space. Available: {}, Total tab width: {}",
        available_width,
        total_tab_width
    );

    // Verify equal distribution: all tabs should have EQUAL width
    assert!(
        (layout1.size.width - layout2.size.width).abs() < 0.1,
        "Tab 1 and Tab 2 should have equal width. Got {} and {}",
        layout1.size.width,
        layout2.size.width
    );

    assert!(
        (layout2.size.width - layout3.size.width).abs() < 0.1,
        "Tab 2 and Tab 3 should have equal width. Got {} and {}",
        layout2.size.width,
        layout3.size.width
    );

    assert!(
        (layout3.size.width - layout4.size.width).abs() < 0.1,
        "Tab 3 and Tab 4 should have equal width. Got {} and {}",
        layout3.size.width,
        layout4.size.width
    );
}
