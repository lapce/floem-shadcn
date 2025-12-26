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
use floem::text::Weight;
use floem::views::Decorators;
use floem::{HasViewId, ViewId};

use crate::theme::ShadcnThemeExt;

// ============================================================================
// Tabs Container
// ============================================================================

/// Tabs container that manages active tab state
pub struct Tabs<V> {
    id: ViewId,
    active: RwSignal<String>,
    child: V,
}

impl<V: IntoView + 'static> Tabs<V> {
    /// Create a new tabs container with the given active signal and content
    pub fn new(active: RwSignal<String>, child: V) -> Self {
        Self {
            id: ViewId::new(),
            active,
            child,
        }
    }
}

impl<V: IntoView + 'static> HasViewId for Tabs<V> {
    fn view_id(&self) -> ViewId {
        self.id
    }
}

impl<V: IntoView + 'static> IntoView for Tabs<V> {
    type V = Box<dyn View>;
    type Intermediate = Self;

    fn into_intermediate(self) -> Self::Intermediate {
        self
    }

    fn into_view(self) -> Self::V {
        Box::new(
            floem::views::Container::with_id(self.id, self.child).style(|s| {
                s.width_full()
                    .flex_direction(floem::style::FlexDirection::Column)
                    .gap(8.0)
            }),
        )
    }
}

// ============================================================================
// TabsList
// ============================================================================

/// Container for tab triggers
pub struct TabsList<V> {
    id: ViewId,
    child: V,
}

impl<V: IntoView + 'static> TabsList<V> {
    /// Create a new tabs list with the given tabs
    pub fn new(child: V) -> Self {
        Self {
            id: ViewId::new(),
            child,
        }
    }
}

impl<V: IntoView + 'static> HasViewId for TabsList<V> {
    fn view_id(&self) -> ViewId {
        self.id
    }
}

impl<V: IntoView + 'static> IntoView for TabsList<V> {
    type V = Box<dyn View>;
    type Intermediate = Self;

    fn into_intermediate(self) -> Self::Intermediate {
        self
    }

    fn into_view(self) -> Self::V {
        Box::new(
            floem::views::Container::with_id(self.id, self.child).style(move |s| {
                s.with_shadcn_theme(|s, t| {
                    s.display(floem::style::Display::Flex)
                        .flex_direction(floem::style::FlexDirection::Row)
                        .background(t.muted)
                        .border_radius(6.0)
                        .padding(4.0)
                        .gap(4.0)
                })
            }),
        )
    }
}

// ============================================================================
// Tab (trigger)
// ============================================================================

/// Individual tab trigger
pub struct Tab {
    view_id: ViewId,
    id: String,
    label: String,
    active_signal: Option<RwSignal<String>>,
}

impl Tab {
    /// Create a new tab with the given id and label
    pub fn new(id: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            view_id: ViewId::new(),
            id: id.into(),
            label: label.into(),
            active_signal: None,
        }
    }

    /// Set the active signal for this tab
    pub fn active(mut self, signal: RwSignal<String>) -> Self {
        self.active_signal = Some(signal);
        self
    }

    /// Build the tab view
    pub fn build(self) -> impl IntoView {
        let id = self.id.clone();
        let label = self.label.clone();
        let active_signal = self.active_signal;
        let item_id = id.clone();
        let item_id_click = id.clone();

        floem::views::Container::new(floem::views::Label::new(label))
            .style(move |s| {
                let id = item_id.clone();
                s.with_shadcn_theme(move |s, t| {
                    let is_active = active_signal
                        .map(|sig| sig.get() == id.clone())
                        .unwrap_or(false);
                    let base = s
                        .padding_left(12.0)
                        .padding_right(12.0)
                        .padding_top(6.0)
                        .padding_bottom(6.0)
                        .border_radius(4.0)
                        .font_size(14.0)
                        .font_weight(Weight::MEDIUM)
                        .cursor(CursorStyle::Pointer)
                        .transition(
                            floem::style::Background,
                            floem::style::Transition::linear(millis(100)),
                        );
                    if is_active {
                        base.background(t.background).color(t.foreground)
                    } else {
                        base.background(peniko::Color::TRANSPARENT)
                            .color(t.muted_foreground)
                    }
                })
            })
            .on_click_stop(move |_| {
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
    type Intermediate = Self;

    fn into_intermediate(self) -> Self::Intermediate {
        self
    }

    fn into_view(self) -> Self::V {
        Box::new(self.build().into_view())
    }
}

// ============================================================================
// TabsContent
// ============================================================================

/// Content panel for a specific tab
pub struct TabsContent<V> {
    view_id: ViewId,
    id: String,
    child: V,
    active_signal: Option<RwSignal<String>>,
}

impl<V: IntoView + 'static> TabsContent<V> {
    /// Create new tab content for the given tab id
    pub fn new(id: impl Into<String>, child: V) -> Self {
        Self {
            view_id: ViewId::new(),
            id: id.into(),
            child,
            active_signal: None,
        }
    }

    /// Set the active signal for this content
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
    type Intermediate = Self;

    fn into_intermediate(self) -> Self::Intermediate {
        self
    }

    fn into_view(self) -> Self::V {
        let id = self.id;
        let active_signal = self.active_signal;

        Box::new(
            floem::views::Container::with_id(self.view_id, self.child).style(move |s| {
                let is_active = active_signal.map(|sig| sig.get() == id).unwrap_or(true); // Show by default if no signal

                s.width_full()
                    .apply_if(!is_active, |s| s.display(floem::style::Display::None))
            }),
        )
    }
}

fn millis(ms: u64) -> std::time::Duration {
    std::time::Duration::from_millis(ms)
}
