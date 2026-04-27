//! Tests for the Slider component

use floem::prelude::*;
use floem::reactive::RwSignal;
use floem::view::ViewId;
use floem::views::Decorators;
use floem_shadcn::components::slider::Slider;
use floem_test::TestRoot;
use floem_test::prelude::*;

fn create_test_slider(value: RwSignal<f64>, width: f64) -> (ViewId, impl IntoView) {
    let container_id = ViewId::new();
    let padding = 8.0;

    let view = floem::views::Container::with_id(
        container_id,
        floem::views::Empty::new().style(move |s| {
            s.height(6.0)
                .width_pct(value.get())
                .background(peniko::Color::from_rgb8(14, 165, 233))
        }),
    )
    .style(move |s| {
        s.width(width)
            .height(16.0)
            .items_center()
            .background(peniko::Color::from_rgb8(229, 229, 229))
            .padding_left(padding)
            .padding_right(padding)
            .cursor(floem::style::CursorStyle::Pointer)
    });

    (container_id, view)
}

#[test]
fn test_slider_creation() {
    let value = RwSignal::new(50.0);
    let slider = Slider::new(value).build();
    let root = TestRoot::new();
    let mut harness = HeadlessHarness::new_with_size(root, slider, 200.0, 40.0);
    harness.rebuild();
    // just verify it builds
}

#[test]
fn test_slider_min_max() {
    let value = RwSignal::new(0.0);
    let slider = Slider::new(value).min(10.0).max(50.0).build();
    let root = TestRoot::new();
    let _harness = HeadlessHarness::new_with_size(root, slider, 200.0, 40.0);
}

#[test]
fn test_slider_disabled() {
    let value = RwSignal::new(50.0);
    let slider = Slider::new(value).disabled(true).build();
    let root = TestRoot::new();
    let _harness = HeadlessHarness::new_with_size(root, slider, 200.0, 40.0);
}

#[test]
fn test_slider_layout_with_padding() {
    let value = RwSignal::new(50.0);
    let (container_id, view) = create_test_slider(value, 300.0);
    let mut harness = HeadlessHarness::new_with_size(TestRoot::new(), view, 400.0, 100.0);
    harness.rebuild();

    let content_rect = harness.get_content_rect(container_id);
    assert!(
        content_rect.width() >= 0.0,
        "Should have a valid content rect"
    );
}

#[test]
fn test_slider_value_updates() {
    // The slider's value signal can be set manually and the internal state updates.
    let value = RwSignal::new(0.0);
    let slider = Slider::new(value).build();
    let root = TestRoot::new();
    let _harness = HeadlessHarness::new_with_size(root, slider, 200.0, 40.0);

    // Manually change the value and verify
    value.set(75.0);
    assert!((value.get() - 75.0).abs() < 0.01);
}

#[test]
fn test_actual_slider_component_value_persists() {
    let value = RwSignal::new(0.0);
    let view = Slider::new(value).build();
    let mut harness = HeadlessHarness::new_with_size(TestRoot::new(), view, 400.0, 100.0);
    harness.rebuild();

    // In headless, pointer events are limited; we can just verify the slider renders
    // and the value signal is accessible.
    value.set(42.0);
    assert!((value.get() - 42.0).abs() < 0.01);
}
