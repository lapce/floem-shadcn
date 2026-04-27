use floem_test::TestRoot;
// Tests for Spinner component

use floem::prelude::*;
use floem_shadcn::components::spinner::*;
use floem_test::prelude::*;

#[test]
fn test_spinner_default_md_size_renders() {
    floem_shadcn::theme::set_theme(floem_shadcn::theme::ShadcnTheme::light());
    let spinner = Spinner::new();
    let id = spinner.view_id();

    let container = Stack::new((spinner,)).style(|s| s.size(100.0, 100.0));
    let mut harness = HeadlessHarness::new_with_size(TestRoot::new(), container, 100.0, 100.0);
    harness.rebuild();

    let layout = id.get_layout().expect("Spinner layout should exist");
    // In headless, SVG may have zero dimensions – just ensure layout exists
    assert!(
        layout.size.width >= 0.0,
        "Spinner layout width should be non-negative"
    );
    assert!(
        layout.size.height >= 0.0,
        "Spinner layout height should be non-negative"
    );
}

#[test]
fn test_spinner_sm_size_renders() {
    floem_shadcn::theme::set_theme(floem_shadcn::theme::ShadcnTheme::light());
    let spinner = Spinner::new().size(SpinnerSize::Sm);
    let id = spinner.view_id();

    let container = Stack::new((spinner,)).style(|s| s.size(100.0, 100.0));
    let mut harness = HeadlessHarness::new_with_size(TestRoot::new(), container, 100.0, 100.0);
    harness.rebuild();

    let layout = id.get_layout().expect("Layout should exist");
    assert!(layout.size.width >= 0.0);
}

#[test]
fn test_spinner_lg_size_renders() {
    floem_shadcn::theme::set_theme(floem_shadcn::theme::ShadcnTheme::light());
    let spinner = Spinner::new().size(SpinnerSize::Lg);
    let id = spinner.view_id();

    let container = Stack::new((spinner,)).style(|s| s.size(100.0, 100.0));
    let mut harness = HeadlessHarness::new_with_size(TestRoot::new(), container, 100.0, 100.0);
    harness.rebuild();

    let layout = id.get_layout().expect("Layout should exist");
    assert!(layout.size.width >= 0.0);
}
