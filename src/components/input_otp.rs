//! Input OTP component with builder-style API
//!
//! Based on shadcn/ui Input OTP - one-time password input with individual slots.
//!
//! # Example
//!
//! ```
//! use floem::reactive::RwSignal;
//! use floem_shadcn::components::input_otp::*;
//!
//! let otp = RwSignal::new(String::new());
//!
//! InputOTP::new(otp, 6);  // 6-digit OTP
//! ```

use crate::theme::ShadcnThemeExt;
use floem::prelude::*;
use floem::reactive::{RwSignal, SignalGet};
use floem::views::Decorators;
use floem::{HasViewId, ViewId};
use floem_tailwind::TailwindExt;

pub struct InputOTP {
    id: ViewId,
    value: RwSignal<String>,
    max_length: usize,
    mask: bool,
}
impl InputOTP {
    pub fn new(value: RwSignal<String>, max_length: usize) -> Self {
        Self {
            id: ViewId::new(),
            value,
            max_length,
            mask: false,
        }
    }
    pub fn mask(mut self) -> Self {
        self.mask = true;
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
    type Intermediate = Box<dyn View>;
    fn into_intermediate(self) -> Self::Intermediate {
        self.into_view()
    }
    fn into_view(self) -> Self::V {
        let value = self.value;
        let max_length = self.max_length;
        let mask = self.mask;
        match max_length {
            4 => Box::new(create_otp_group_4(value, mask).into_view()),
            6 => Box::new(create_otp_group_6(value, mask).into_view()),
            _ => Box::new(create_otp_group_6(value, mask).into_view()),
        }
    }
}

fn create_otp_group_4(value: RwSignal<String>, mask: bool) -> impl IntoView {
    floem::views::Stack::horizontal((
        create_otp_slot(value, 0, mask),
        create_otp_slot(value, 1, mask),
        create_otp_slot(value, 2, mask),
        create_otp_slot(value, 3, mask),
    ))
    .style(|s| s.gap_2())
}

fn create_otp_group_6(value: RwSignal<String>, mask: bool) -> impl IntoView {
    floem::views::Stack::horizontal((
        create_otp_slot(value, 0, mask),
        create_otp_slot(value, 1, mask),
        create_otp_slot(value, 2, mask),
        InputOTPSeparator::new(),
        create_otp_slot(value, 3, mask),
        create_otp_slot(value, 4, mask),
        create_otp_slot(value, 5, mask),
    ))
    .style(|s| s.gap_2().items_center())
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
            let is_current = index == val.len();
            let base = s
                .w_10()
                .h_12()
                .text_xl()
                .font_medium()
                .color(t.foreground)
                .background(t.background)
                .border_1()
                .border_color(t.input)
                .rounded_md()
                .flex()
                .items_center()
                .justify_center();
            if is_current {
                base.border_color(t.primary)
            } else {
                base
            }
        })
    })
}

pub struct InputOTPGroup<V> {
    id: ViewId,
    child: V,
}
impl<V: IntoView + 'static> InputOTPGroup<V> {
    pub fn new(child: V) -> Self {
        Self {
            id: ViewId::new(),
            child,
        }
    }
}
impl<V: IntoView + 'static> HasViewId for InputOTPGroup<V> {
    fn view_id(&self) -> ViewId {
        self.id
    }
}
impl<V: IntoView + 'static> IntoView for InputOTPGroup<V> {
    type V = Box<dyn View>;
    type Intermediate = Box<dyn View>;
    fn into_intermediate(self) -> Self::Intermediate {
        self.into_view()
    }
    fn into_view(self) -> Self::V {
        Box::new(
            floem::views::Container::with_id(self.id, self.child).style(|s| s.flex_row().gap_2()),
        )
    }
}

pub struct InputOTPSlot {
    id: ViewId,
    value: RwSignal<String>,
    index: usize,
    mask: bool,
}
impl InputOTPSlot {
    pub fn new(value: RwSignal<String>, index: usize) -> Self {
        Self {
            id: ViewId::new(),
            value,
            index,
            mask: false,
        }
    }
    pub fn mask(mut self) -> Self {
        self.mask = true;
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
    type Intermediate = Box<dyn View>;
    fn into_intermediate(self) -> Self::Intermediate {
        self.into_view()
    }
    fn into_view(self) -> Self::V {
        Box::new(create_otp_slot(self.value, self.index, self.mask).into_view())
    }
}

pub struct InputOTPSeparator;
impl InputOTPSeparator {
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
    type Intermediate = Box<dyn View>;
    fn into_intermediate(self) -> Self::Intermediate {
        self.into_view()
    }
    fn into_view(self) -> Self::V {
        Box::new(floem::views::Label::new("-").style(|s| {
            s.with_shadcn_theme(move |s, t| s.text_xl().color(t.muted_foreground).px_1())
        }))
    }
}

pub struct PinInput {
    id: ViewId,
    value: RwSignal<String>,
    length: usize,
    mask: bool,
}
impl PinInput {
    pub fn new(value: RwSignal<String>, length: usize) -> Self {
        Self {
            id: ViewId::new(),
            value,
            length,
            mask: true,
        }
    }
    pub fn show_digits(mut self) -> Self {
        self.mask = false;
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
    type Intermediate = Box<dyn View>;
    fn into_intermediate(self) -> Self::Intermediate {
        self.into_view()
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
