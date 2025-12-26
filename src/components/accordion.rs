//! Accordion component with builder-style API
//!
//! Based on shadcn/ui Accordion - collapsible content sections.
//!
//! # Example
//!
//! ```rust
//! use floem::reactive::RwSignal;
//! use floem_shadcn::components::accordion::{Accordion, AccordionItem};
//!
//! let expanded = RwSignal::new(Some("item-1".to_string()));
//!
//! Accordion::new(expanded, (
//!     AccordionItem::new("item-1", "Is it accessible?", "Yes. It adheres to the WAI-ARIA design pattern."),
//!     AccordionItem::new("item-2", "Is it styled?", "Yes. It comes with default styles."),
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
// Accordion
// ============================================================================

/// Accordion container that manages which item is expanded
pub struct Accordion<V> {
    id: ViewId,
    expanded: RwSignal<Option<String>>,
    child: V,
}

impl<V: IntoView + 'static> Accordion<V> {
    /// Create a new accordion with the given expanded signal and items
    pub fn new(expanded: RwSignal<Option<String>>, child: V) -> Self {
        Self {
            id: ViewId::new(),
            expanded,
            child,
        }
    }
}

impl<V: IntoView + 'static> HasViewId for Accordion<V> {
    fn view_id(&self) -> ViewId {
        self.id
    }
}

impl<V: IntoView + 'static> IntoView for Accordion<V> {
    type V = Box<dyn View>;
    type Intermediate = Self;

    fn into_intermediate(self) -> Self::Intermediate {
        self
    }

    fn into_view(self) -> Self::V {
        Box::new(floem::views::Container::with_id(self.id, self.child).style(|s| {
            s.width_full()
                .flex_direction(floem::style::FlexDirection::Column)
        }))
    }
}

// ============================================================================
// AccordionItem
// ============================================================================

/// Individual accordion item with trigger and content
pub struct AccordionItem {
    view_id: ViewId,
    id: String,
    title: String,
    content: String,
    expanded_signal: Option<RwSignal<Option<String>>>,
}

impl AccordionItem {
    /// Create a new accordion item
    pub fn new(
        id: impl Into<String>,
        title: impl Into<String>,
        content: impl Into<String>,
    ) -> Self {
        Self {
            view_id: ViewId::new(),
            id: id.into(),
            title: title.into(),
            content: content.into(),
            expanded_signal: None,
        }
    }

    /// Set the expanded signal for this item
    pub fn expanded(mut self, signal: RwSignal<Option<String>>) -> Self {
        self.expanded_signal = Some(signal);
        self
    }

    /// Build the accordion item view
    pub fn build(self) -> impl IntoView {
        let id = self.id.clone();
        let title = self.title.clone();
        let content = self.content.clone();
        let expanded_signal = self.expanded_signal;
        let item_id = id.clone();
        let item_id_click = id.clone();
        let item_id_content = id.clone();

        let trigger = floem::views::Container::new(
            floem::views::Stack::horizontal((
                floem::views::Label::new(title).style(|s| {
                    s.with_shadcn_theme(|s, t| {
                        s.font_size(14.0)
                            .font_weight(Weight::MEDIUM)
                            .color(t.foreground)
                            .flex_grow(1.0)
                    })
                }),
                // Chevron icon
                floem::views::svg(move || {
                    let is_expanded = expanded_signal
                        .map(|sig| sig.get() == Some(item_id.clone()))
                        .unwrap_or(false);
                    if is_expanded {
                        CHEVRON_UP_SVG.to_string()
                    } else {
                        CHEVRON_DOWN_SVG.to_string()
                    }
                })
                .style(|s| {
                    s.with_shadcn_theme(move |s, t| {
                        s.width(16.0).height(16.0).color(t.muted_foreground)
                    })
                }),
            ))
            .style(|s| s.width_full().items_center()),
        )
        .style(|s| {
            s.with_shadcn_theme(|s, t| {
                s.width_full()
                    .padding(16.0)
                    .cursor(CursorStyle::Pointer)
                    .border_bottom(1.0)
                    .border_color(t.border)
                    .hover(|s| s.background(t.muted))
            })
        })
        .on_click_stop(move |_| {
            if let Some(signal) = expanded_signal {
                signal.update(|current| {
                    if *current == Some(item_id_click.clone()) {
                        *current = None;
                    } else {
                        *current = Some(item_id_click.clone());
                    }
                });
            }
        });

        let content_view =
            floem::views::Container::new(floem::views::Label::new(content).style(|s| {
                s.with_shadcn_theme(|s, t| {
                    s.font_size(14.0).color(t.muted_foreground).line_height(1.5)
                })
            }))
            .style(move |s| {
                let item_id = item_id_content.clone();
                s.with_shadcn_theme(move |s, t| {
                    let is_expanded = expanded_signal
                        .map(|sig| sig.get() == Some(item_id.clone()))
                        .unwrap_or(false);

                    s.width_full()
                        .padding(16.0)
                        .padding_top(0.0)
                        .border_bottom(1.0)
                        .border_color(t.border)
                        .apply_if(!is_expanded, |s| s.display(floem::style::Display::None))
                })
            });

        floem::views::Stack::vertical((trigger, content_view)).style(|s| s.width_full())
    }
}

impl HasViewId for AccordionItem {
    fn view_id(&self) -> ViewId {
        self.view_id
    }
}

impl IntoView for AccordionItem {
    type V = Box<dyn View>;
    type Intermediate = Self;

    fn into_intermediate(self) -> Self::Intermediate {
        self
    }

    fn into_view(self) -> Self::V {
        Box::new(self.build().into_view())
    }
}

// Chevron down SVG
const CHEVRON_DOWN_SVG: &str = r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><polyline points="6 9 12 15 18 9"></polyline></svg>"#;

// Chevron up SVG
const CHEVRON_UP_SVG: &str = r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><polyline points="18 15 12 9 6 15"></polyline></svg>"#;
