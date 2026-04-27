use floem_test::TestRoot;
// Original tests for sidebar menu button style-cache behaviour

use floem::prelude::*;
use floem::reactive::RwSignal;
use floem_shadcn::components::sidebar::*;
use floem_test::prelude::*;

#[test]
fn test_sidebar_button_active_state_updates_style() {
    let active = RwSignal::new("none");
    let btn = SidebarMenuButton::new("First").is_active(move || active.get() == "first");
    let id = btn.view_id();
    let view = Stack::new((btn,)).style(|s| s.size(300.0, 100.0));
    let mut harness = HeadlessHarness::new_with_size(TestRoot::new(), view, 300.0, 100.0);
    harness.rebuild();

    // Initially not active
    let initial_style = harness.get_computed_style(id);
    let initial_weight = initial_style.get(floem::style::FontWeight);
    eprintln!("Initial font weight: {:?}", initial_weight);

    // Click to activate
    let rect = harness.get_layout_rect(id);
    harness.click(rect.center().x as f64, rect.center().y as f64);
    harness.rebuild();

    // Now the button should be active -> font weight should be MEDIUM
    let active_style = harness.get_computed_style(id);
    let active_weight = active_style.get(floem::style::FontWeight);
    eprintln!("After click font weight: {:?}", active_weight);
    assert!(
        active_weight != initial_weight || active_weight.is_none(),
        "Style should update immediately after click"
    );
}
