use floem::headless::TestRoot;
use floem::prelude::*;
use floem::views::Stack;
use floem_shadcn::components::empty::Empty as ShadcnEmpty;
use floem_test::prelude::*;

#[test]
fn test_empty_component_creation() {
    let view = Stack::vertical((ShadcnEmpty::new(),)).style(|s| s.size(100.0, 100.0));
    let _harness = HeadlessHarness::new_with_size(TestRoot::new(), view, 200.0, 200.0);
}

#[test]
fn test_empty_component_with_content() {
    let view = Stack::vertical((Stack::horizontal((
        ShadcnEmpty::new().style(|s| s.width(50.0).height(50.0)),
    )),))
    .style(|s| s.size(200.0, 100.0));
    let _harness = HeadlessHarness::new_with_size(TestRoot::new(), view, 300.0, 200.0);
}
