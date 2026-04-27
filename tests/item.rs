use floem::views::Decorators;
use floem_test::TestRoot;
// Tests for Item component

use floem::prelude::*;
use floem::view::ParentView;
use floem_shadcn::components::item::*;
use floem_test::prelude::*;

#[test]
fn test_item_renders_title_and_description() {
    let item = Item::new().child(
        ItemContent::new()
            .child(ItemTitle::new("Account"))
            .child(ItemDescription::new("Manage your account")),
    );
    let id = item.view_id();

    let container = Stack::new((item,)).style(|s| s.size(400.0, 100.0));
    let mut harness = HeadlessHarness::new_with_size(TestRoot::new(), container, 400.0, 100.0);
    harness.rebuild();

    let layout = id.get_layout().expect("Item layout should exist");
    assert!(layout.size.width > 50.0, "Item should have width");
    assert!(
        layout.size.height > 40.0,
        "Item should be tall enough for title+description"
    );
}
