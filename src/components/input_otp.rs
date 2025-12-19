//! Input OTP component with builder-style API
//!
//! Based on shadcn/ui Input OTP - one-time password input with individual slots.
//!
//! # Example
//!
//! ```rust
//! use floem::reactive::RwSignal;
//! use floem_shadcn::components::input_otp::*;
//!
//! let otp = RwSignal::new(String::new());
//!
//! InputOTP::new(otp, 6);  // 6-digit OTP
//! ```

use floem::prelude::*;
use floem::{HasViewId, ViewId};
use floem::reactive::{RwSignal, SignalGet, SignalUpdate};
use floem::views::Decorators;

use crate::theme::ShadcnThemeExt;

// ============================================================================
// InputOTP
// ============================================================================

/// One-time password input with individual digit slots
pub struct InputOTP {
    id: ViewId,
    value: RwSignal<String>,
    max_length: usize,
    mask: bool,
}

impl InputOTP {
    /// Create a new OTP input
    pub fn new(value: RwSignal<String>, max_length: usize) -> Self { Self { id: ViewId::new(),
            value,
            max_length,
            mask: false,
        }
    }

    /// Mask the input (show dots instead of digits)
    pub fn mask(mut self) -> Self { self.mask = true;
        self
    }
}


impl HasViewId for InputOTP {
    fn view_id(&self) -> ViewId {
        self.id
    }
}

impl IntoView for InputOTP {
    type V = Box<dyn View>;
    type Intermediate = Self;

    fn into_intermediate(self) -> Self::Intermediate {
        self
    }

    fn into_view(self) -> Self::V {
        let value = self.value;
        let max_length = self.max_length;
        let mask = self.mask;

        // Create slots based on max_length (up to 8)
        match max_length {
            4 => Box::new(create_otp_group_4(value, mask).into_view()),
            6 => Box::new(create_otp_group_6(value, mask).into_view()),
            _ => Box::new(create_otp_group_6(value, mask).into_view()), // Default to 6
        }
    }
}

fn create_otp_group_4(value: RwSignal<String>, mask: bool) -> impl IntoView {
    floem::views::h_stack((
        create_otp_slot(value, 0, mask),
        create_otp_slot(value, 1, mask),
        create_otp_slot(value, 2, mask),
        create_otp_slot(value, 3, mask),
    ))
    .style(|s| s.gap(8.0))
}

fn create_otp_group_6(value: RwSignal<String>, mask: bool) -> impl IntoView {
    floem::views::h_stack((
        create_otp_slot(value, 0, mask),
        create_otp_slot(value, 1, mask),
        create_otp_slot(value, 2, mask),
        InputOTPSeparator::new(),
        create_otp_slot(value, 3, mask),
        create_otp_slot(value, 4, mask),
        create_otp_slot(value, 5, mask),
    ))
    .style(|s| s.gap(8.0).items_center())
}

fn create_otp_slot(value: RwSignal<String>, index: usize, mask: bool) -> impl IntoView {
    floem::views::Label::derived(move || {
        let val = value.get();
        let chars: Vec<char> = val.chars().collect();

        if index < chars.len() {
            if mask {
                "●".to_string()
            } else {
                chars[index].to_string()
            }
        } else {
            String::new()
        }
    })
    .style(move |s| {
        s.with_shadcn_theme(move |s, t| {
            let val = value.get();
            let is_filled = index < val.len();
            let is_current = index == val.len();
            let base = s
                .width(40.0)
                .height(48.0)
                .font_size(20.0)
                .font_weight(floem::text::Weight::MEDIUM)
                .color(t.foreground)
                .background(t.background)
                .border(1.0)
                .border_color(t.input)
                .border_radius(t.radius)
                .display(floem::style::Display::Flex)
                .items_center()
                .justify_center();
            if is_current {
                base.border_color(t.primary)
            } else if is_filled {
                base
            } else {
                base
            }
        })
    })
}

// ============================================================================
// InputOTPGroup
// ============================================================================

/// Group of OTP slots
pub struct InputOTPGroup<V> {
    id: ViewId,
    child: V,
}

impl<V: IntoView + 'static> InputOTPGroup<V> {
    /// Create a new OTP group
    pub fn new(child: V) -> Self { Self { id: ViewId::new(), child }
    }
}


impl<V: IntoView + 'static> HasViewId for InputOTPGroup<V> {
    fn view_id(&self) -> ViewId {
        self.id
    }
}

impl<V: IntoView + 'static> IntoView for InputOTPGroup<V> {
    type V = Box<dyn View>;
    type Intermediate = Self;

    fn into_intermediate(self) -> Self::Intermediate {
        self
    }

    fn into_view(self) -> Self::V {
        Box::new(floem::views::Container::with_id(self.id, self.child).style(|s| {
            s.display(floem::style::Display::Flex)
                .flex_direction(floem::style::FlexDirection::Row)
                .gap(8.0)
        }))
    }
}

// ============================================================================
// InputOTPSlot
// ============================================================================

/// Individual OTP slot/digit
pub struct InputOTPSlot {
    id: ViewId,
    value: RwSignal<String>,
    index: usize,
    mask: bool,
}

impl InputOTPSlot {
    /// Create a new OTP slot
    pub fn new(value: RwSignal<String>, index: usize) -> Self { Self { id: ViewId::new(),
            value,
            index,
            mask: false,
        }
    }

    /// Mask the digit
    pub fn mask(mut self) -> Self { self.mask = true;
        self
    }
}


impl HasViewId for InputOTPSlot {
    fn view_id(&self) -> ViewId {
        self.id
    }
}

impl IntoView for InputOTPSlot {
    type V = Box<dyn View>;
    type Intermediate = Self;

    fn into_intermediate(self) -> Self::Intermediate {
        self
    }

    fn into_view(self) -> Self::V {
        let value = self.value;
        let index = self.index;
        let mask = self.mask;

        Box::new(
            floem::views::Label::derived(move || {
                let val = value.get();
                let chars: Vec<char> = val.chars().collect();

                if index < chars.len() {
                    if mask {
                        "●".to_string()
                    } else {
                        chars[index].to_string()
                    }
                } else {
                    String::new()
                }
            })
            .style(move |s| {
                s.with_shadcn_theme(move |s, t| {
                    let val = value.get();
                    let is_filled = index < val.len();
                    let is_current = index == val.len();
                    let base = s
                        .width(40.0)
                        .height(48.0)
                        .font_size(20.0)
                        .font_weight(floem::text::Weight::MEDIUM)
                        .color(t.foreground)
                        .background(t.background)
                        .border(1.0)
                        .border_color(t.input)
                        .border_radius(t.radius)
                        .display(floem::style::Display::Flex)
                        .items_center()
                        .justify_center();
                    if is_current {
                        base.border_color(t.primary)
                    } else if is_filled {
                        base
                    } else {
                        base
                    }
                })
            }),
        )
    }
}

// ============================================================================
// InputOTPSeparator
// ============================================================================

/// Separator between OTP groups (e.g., XXX-XXX)
pub struct InputOTPSeparator;

impl InputOTPSeparator {
    /// Create a new separator
    pub fn new() -> Self {
        Self
    }
}

impl Default for InputOTPSeparator {
    fn default() -> Self {
        Self::new()
    }
}


impl HasViewId for InputOTPSeparator {
    fn view_id(&self) -> ViewId {
        ViewId::new()
    }
}

impl IntoView for InputOTPSeparator {
    type V = Box<dyn View>;
    type Intermediate = Self;

    fn into_intermediate(self) -> Self::Intermediate {
        self
    }

    fn into_view(self) -> Self::V {
        Box::new(floem::views::Label::new("-").style(|s| {
            s.with_shadcn_theme(move |s, t| {
                s.font_size(20.0)
                    .color(t.muted_foreground)
                    .padding_left(4.0)
                    .padding_right(4.0)
            })
        }))
    }
}

// ============================================================================
// PinInput - Alternative simpler API
// ============================================================================

/// Simple PIN input (alias for InputOTP)
pub struct PinInput {
    id: ViewId,
    value: RwSignal<String>,
    length: usize,
    mask: bool,
}

impl PinInput {
    /// Create a new PIN input
    pub fn new(value: RwSignal<String>, length: usize) -> Self { Self { id: ViewId::new(),
            value,
            length,
            mask: true, // PINs are usually masked
        }
    }

    /// Show digits instead of dots
    pub fn show_digits(mut self) -> Self { self.mask = false;
        self
    }
}


impl HasViewId for PinInput {
    fn view_id(&self) -> ViewId {
        self.id
    }
}

impl IntoView for PinInput {
    type V = Box<dyn View>;
    type Intermediate = Self;

    fn into_intermediate(self) -> Self::Intermediate {
        self
    }

    fn into_view(self) -> Self::V {
        let value = self.value;
        let length = self.length;
        let mask = self.mask;

        match length {
            4 => Box::new(create_otp_group_4(value, mask).into_view()),
            6 => Box::new(create_otp_group_6(value, mask).into_view()),
            _ => Box::new(create_otp_group_4(value, mask).into_view()),
        }
    }
}
