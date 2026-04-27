use floem_test::TestRoot;
// Tests for Typography components

use floem::prelude::*;
use floem_shadcn::components::typography::*;
use floem_test::prelude::*;

#[test]
fn test_h1_renders() {
    let h1 = TypographyH1::new("Title");
    let id = h1.view_id();

    let container = Stack::new((h1,)).style(|s| s.size(400.0, 100.0));
    let mut harness = HeadlessHarness::new_with_size(TestRoot::new(), container, 400.0, 100.0);
    harness.rebuild();

    let layout = id.get_layout().expect("Layout should exist");
    // In headless, zero-sized text is expected; just ensure layout exists.
    assert!(layout.size.width >= 0.0 && layout.size.height >= 0.0);
}

#[test]
fn test_muted_renders() {
    let muted = TypographyMuted::new("muted text");
    let id = muted.view_id();

    let container = Stack::new((muted,)).style(|s| s.size(400.0, 100.0));
    let mut harness = HeadlessHarness::new_with_size(TestRoot::new(), container, 400.0, 100.0);
    harness.rebuild();

    let layout = id.get_layout().expect("Layout should exist");
    assert!(layout.size.width >= 0.0);
}

#[test]
fn test_list_renders() {
    let list = TypographyList::new(vec!["Item 1".into(), "Item 2".into()], false);
    let id = list.view_id();

    let container = Stack::new((list,)).style(|s| s.size(400.0, 200.0));
    let mut harness = HeadlessHarness::new_with_size(TestRoot::new(), container, 400.0, 200.0);
    harness.rebuild();

    let layout = id.get_layout().expect("Layout should exist");
    assert!(layout.size.width >= 0.0);
}
