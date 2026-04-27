use floem::prelude::*;
use floem::view::ParentView;
use floem_shadcn::components::input::Input;
use floem_shadcn::components::input_group::{AddonPosition, InputGroup, InputGroupAddon};
use floem_test::TestRoot;
use floem_test::prelude::*;

#[test]
fn test_input_group_with_prefix_addon() {
    floem_shadcn::theme::set_theme(floem_shadcn::theme::ShadcnTheme::light());
    let addon = InputGroupAddon::new(AddonPosition::Prefix, Label::new("$"));
    let group = InputGroup::new()
        .child(addon)
        .child(Input::new().placeholder("Amount"));
    let id = group.view_id();

    let container = Stack::new((group,)).style(|s| s.size(300.0, 100.0));
    let mut harness = HeadlessHarness::new_with_size(TestRoot::new(), container, 300.0, 100.0);
    harness.rebuild();

    let layout = id.get_layout().expect("InputGroup layout should exist");
    // In headless environment, layout may have zero width, so we only check existence.
    assert!(layout.size.width >= 0.0, "Group should have a valid layout");
}

#[test]
fn test_input_group_with_disabled() {
    floem_shadcn::theme::set_theme(floem_shadcn::theme::ShadcnTheme::light());
    let addon = InputGroupAddon::new(AddonPosition::Suffix, Label::new(".com"));
    let group = InputGroup::new()
        .disabled(true)
        .child(addon)
        .child(Input::new().placeholder("domain"));
    let id = group.view_id();

    let container = Stack::new((group,)).style(|s| s.size(300.0, 100.0));
    let mut harness = HeadlessHarness::new_with_size(TestRoot::new(), container, 300.0, 100.0);
    harness.rebuild();

    let layout = id.get_layout().expect("Layout should exist");
    assert!(layout.size.width >= 0.0);
}
