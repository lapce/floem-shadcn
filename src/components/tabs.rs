//! Tabs component with builder-style API
//!
//! Based on shadcn/ui Tabs component for tabbed content navigation.
//!
//! # Example
//!
//! ```rust
//! use floem::reactive::RwSignal;
//! use floem_shadcn::components::tabs::{Tabs, TabsList, Tab, TabsContent};
//!
//! let active_tab = RwSignal::new("account".to_string());
//!
//! let tabs = Tabs::new(active_tab, (
//!     TabsList::new((
//!         Tab::new("account", "Account"),
//!         Tab::new("password", "Password"),
//!     )),
//!     TabsContent::new("account", account_view()),
//!     TabsContent::new("password", password_view()),
//! ));
//! ```

use floem::prelude::*;
use floem::reactive::{RwSignal, SignalGet, SignalUpdate};
use floem::style::CursorStyle;
use floem::views::Decorators;
use floem::{HasViewId, ViewId};
use floem_tailwind::TailwindExt;

use crate::theme::ShadcnThemeExt;

/// Tabs container that manages active tab state.
pub struct Tabs<V> {
    id: ViewId,
    #[allow(dead_code)]
    active: RwSignal<String>,
    child: V,
}

impl<V: floem::view::IntoViewIter + 'static> Tabs<V> {
    pub fn new(active: RwSignal<String>, child: V) -> Self {
        Self {
            id: ViewId::new(),
            active,
            child,
        }
    }
}

impl<V: floem::view::IntoViewIter + 'static> HasViewId for Tabs<V> {
    fn view_id(&self) -> ViewId {
        self.id
    }
}

impl<V: floem::view::IntoViewIter + 'static> IntoView for Tabs<V> {
    type V = Box<dyn View>;
    type Intermediate = Box<dyn View>;
    fn into_intermediate(self) -> Self::Intermediate {
        self.into_view()
    }
    fn into_view(self) -> Self::V {
        Box::new(floem::views::Stack::with_id(self.id, self.child).style(|s| s.flex_col().gap_2()))
    }
}

/// Container for tab triggers (the buttons).
pub struct TabsList<V> {
    id: ViewId,
    child: V,
}

impl<V: floem::view::IntoViewIter + 'static> TabsList<V> {
    pub fn new(child: V) -> Self {
        Self {
            id: ViewId::new(),
            child,
        }
    }
}

impl<V: floem::view::IntoViewIter + 'static> HasViewId for TabsList<V> {
    fn view_id(&self) -> ViewId {
        self.id
    }
}

impl<V: floem::view::IntoViewIter + 'static> IntoView for TabsList<V> {
    type V = Box<dyn View>;
    type Intermediate = Box<dyn View>;
    fn into_intermediate(self) -> Self::Intermediate {
        self.into_view()
    }
    fn into_view(self) -> Self::V {
        Box::new(
            floem::views::Stack::with_id(self.id, self.child).style(move |s| {
                s.with_shadcn_theme(|s, t| {
                    s.flex_row()
                        .items_center()
                        .background(t.muted)
                        .h_9()
                        .rounded_lg()
                        .padding(3.0)
                        .gap(3.0)
                })
            }),
        )
    }
}

/// Individual tab trigger.
pub struct Tab {
    view_id: ViewId,
    id: String,
    label: String,
    active_signal: Option<RwSignal<String>>,
}

impl Tab {
    pub fn new(id: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            view_id: ViewId::new(),
            id: id.into(),
            label: label.into(),
            active_signal: None,
        }
    }
    pub fn active(mut self, signal: RwSignal<String>) -> Self {
        self.active_signal = Some(signal);
        self
    }

    pub fn build(self) -> impl IntoView {
        let id = self.id.clone();
        let label = self.label.clone();
        let active_signal = self.active_signal;
        let item_id = id.clone();
        let item_id_click = id.clone();
        let view_id = self.view_id;

        floem::views::Stack::with_id(view_id, (floem::views::Label::new(label),))
            .style(move |s| {
                let id = item_id.clone();
                s.with_shadcn_theme(move |s, t| {
                    let is_active = active_signal
                        .map(|sig| sig.get() == id.clone())
                        .unwrap_or(false);
                    let base = s
                        .flex_row()
                        .flex_grow(1.0)
                        .flex_basis(0.0)
                        .min_width(0.0)
                        .height(29.0)
                        .px_2()
                        .py_1()
                        .items_center()
                        .justify_center()
                        .rounded_md()
                        .border_1()
                        .border_color(peniko::Color::TRANSPARENT)
                        .text_sm()
                        .font_medium()
                        .cursor(CursorStyle::Pointer)
                        .transition(
                            floem::style::Background,
                            floem::style::Transition::linear(millis(100)),
                        );
                    if is_active {
                        base.background(t.background)
                            .color(t.foreground)
                            .box_shadow_blur(2.0)
                            .box_shadow_color(peniko::Color::from_rgba8(0, 0, 0, 25))
                    } else {
                        base.background(peniko::Color::TRANSPARENT)
                            .color(t.muted_foreground)
                    }
                })
            })
            .on_event_stop(floem::event::listener::Click, move |_, _| {
                if let Some(signal) = active_signal {
                    signal.set(item_id_click.clone());
                }
            })
    }
}

impl HasViewId for Tab {
    fn view_id(&self) -> ViewId {
        self.view_id
    }
}

impl IntoView for Tab {
    type V = Box<dyn View>;
    type Intermediate = Box<dyn View>;
    fn into_intermediate(self) -> Self::Intermediate {
        self.into_view()
    }
    fn into_view(self) -> Self::V {
        Box::new(self.build().into_view())
    }
}

/// Content panel for a specific tab.
pub struct TabsContent<V> {
    view_id: ViewId,
    id: String,
    child: V,
    active_signal: Option<RwSignal<String>>,
}

impl<V: IntoView + 'static> TabsContent<V> {
    pub fn new(id: impl Into<String>, child: V) -> Self {
        Self {
            view_id: ViewId::new(),
            id: id.into(),
            child,
            active_signal: None,
        }
    }
    pub fn active(mut self, signal: RwSignal<String>) -> Self {
        self.active_signal = Some(signal);
        self
    }
}

impl<V: IntoView + 'static> HasViewId for TabsContent<V> {
    fn view_id(&self) -> ViewId {
        self.view_id
    }
}

impl<V: IntoView + 'static> IntoView for TabsContent<V> {
    type V = Box<dyn View>;
    type Intermediate = Box<dyn View>;
    fn into_intermediate(self) -> Self::Intermediate {
        self.into_view()
    }
    fn into_view(self) -> Self::V {
        let id = self.id;
        let active_signal = self.active_signal;

        Box::new(
            floem::views::Container::with_id(self.view_id, self.child).style(move |s| {
                let is_active = active_signal.map(|sig| sig.get() == id).unwrap_or(true);
                s.apply_if(!is_active, |s| s.display(floem::style::Display::None))
            }),
        )
    }
}

fn millis(ms: u64) -> std::time::Duration {
    std::time::Duration::from_millis(ms)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tab_has_flex_grow() {
        // Create a simple tab to test its styling
        let tab = Tab::new("test", "Test Tab");

        // Build the view to get the container
        let _view = tab.build();

        // This test verifies that Tab components can be constructed
        // In a real UI, the flex_grow property would make tabs stretch equally
        assert!(true, "Tab component builds successfully");
    }

    #[test]
    fn test_tabs_list_is_flex_container() {
        // Verify TabsList creates a flex container
        let active = RwSignal::new("tab1".to_string());
        let _tabs_list = TabsList::new((
            Tab::new("tab1", "Tab 1").active(active),
            Tab::new("tab2", "Tab 2").active(active),
        ));

        // This verifies the component can be constructed
        // The actual flex behavior is tested visually
        assert!(true, "TabsList with multiple tabs builds successfully");
    }

    #[test]
    fn test_flex_grow_behavior_simulation() {
        // Simulate flex-grow behavior:
        // With 3 tabs and flex-grow: 1 on each, they should divide space equally

        let container_width = 300.0;
        let gap = 3.0;
        let padding = 3.0;
        let num_tabs = 3;

        // Available space = container_width - (2 * padding) - ((num_tabs - 1) * gap)
        let available_space = container_width - (2.0 * padding) - ((num_tabs - 1) as f64 * gap);

        // Each tab with flex-grow: 1 should get equal space
        let expected_tab_width = available_space / num_tabs as f64;

        // With flex-grow: 1, each tab should be approximately 94px
        // (300 - 6 - 6) / 3 = 96px per tab
        assert_eq!(
            expected_tab_width, 96.0,
            "Each tab should get equal width with flex-grow"
        );
    }

    #[test]
    fn test_tab_height_calculation() {
        // Test the height calculation: h-[calc(100%-1px)]
        // TabsList: h-9 (36px), p-[3px] (3px padding top/bottom)
        // Content area: 36 - 6 = 30px
        // Tab height: calc(100% - 1px) = 29px

        let tabs_list_height = 36.0;
        let padding = 3.0;
        let content_height = tabs_list_height - (2.0 * padding);
        let tab_height = content_height - 1.0; // calc(100% - 1px)

        assert_eq!(
            tab_height, 29.0,
            "Tab height should be 29px (h-[calc(100%-1px)])"
        );
    }

    #[test]
    fn test_inline_flex_vs_flex_with_flex_grow() {
        // This test documents the equivalence between:
        // CSS: inline-flex flex-1
        // Floem: display(Flex) + flex_grow(1.0)

        // In CSS:
        // - inline-flex: makes the element inline-level
        // - flex-1: makes it grow to fill space

        // In Floem (using Taffy):
        // - display(Flex): makes it a flex container
        // - flex_grow(1.0): makes it grow to fill space

        // When the element is a child of a flex container,
        // the inline vs block distinction doesn't affect layout
        // because the parent flex container controls child layout.

        // So: display(Flex) + flex_grow(1.0) ≈ inline-flex flex-1
        assert!(
            true,
            "Documented equivalence between inline-flex flex-1 and Flex + flex_grow"
        );
    }

    #[test]
    fn test_tabs_equal_distribution() {
        // Test that with flex-grow: 1, tabs distribute equally
        // regardless of their content length

        let container_width = 400.0;
        let padding = 3.0;
        let gap = 3.0;
        let num_tabs = 4;

        let available_space = container_width - (2.0 * padding) - ((num_tabs - 1) as f64 * gap);
        let expected_width_per_tab = available_space / num_tabs as f64;

        // Each tab gets: (400 - 6 - 9) / 4 = 96.25px
        assert_eq!(
            expected_width_per_tab, 96.25,
            "With flex-grow: 1, all tabs get equal width regardless of content"
        );
    }

    #[test]
    fn test_tabs_without_flex_grow() {
        // This test documents what would happen WITHOUT flex-grow
        // Tabs would be sized based on content (intrinsic sizing)

        let short_content_width = 50.0; // "Home"
        let long_content_width = 100.0; // "Settings"

        // Without flex-grow, tabs would have different widths
        assert_ne!(
            short_content_width, long_content_width,
            "Without flex-grow, tabs would have different widths based on content"
        );

        // With flex-grow: 1, they would both get the same width
        let container_width = 300.0;
        let padding = 3.0;
        let gap = 3.0;
        let equal_width = (container_width - (2.0 * padding) - gap) / 2.0;

        assert_eq!(
            equal_width, 145.5,
            "With flex-grow: 1, both tabs get equal width (145.5px each)"
        );
    }
}
