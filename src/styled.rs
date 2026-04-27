use crate::theme::current_theme;
use floem::style::Style;

pub trait ShadcnStyleExt: Sized {
    fn bg_background(self) -> Self;
    fn bg_foreground(self) -> Self;
    fn bg_card(self) -> Self;
    fn bg_card_foreground(self) -> Self;
    fn bg_popover(self) -> Self;
    fn bg_popover_foreground(self) -> Self;
    fn bg_primary(self) -> Self;
    fn bg_primary_foreground(self) -> Self;
    fn bg_secondary(self) -> Self;
    fn bg_secondary_foreground(self) -> Self;
    fn bg_muted(self) -> Self;
    fn bg_muted_foreground(self) -> Self;
    fn bg_accent(self) -> Self;
    fn bg_accent_foreground(self) -> Self;
    fn bg_destructive(self) -> Self;
    fn bg_destructive_foreground(self) -> Self;
    fn text_background(self) -> Self;
    fn text_foreground(self) -> Self;
    fn text_card(self) -> Self;
    fn text_card_foreground(self) -> Self;
    fn text_popover(self) -> Self;
    fn text_popover_foreground(self) -> Self;
    fn text_primary(self) -> Self;
    fn text_primary_foreground(self) -> Self;
    fn text_secondary(self) -> Self;
    fn text_secondary_foreground(self) -> Self;
    fn text_muted(self) -> Self;
    fn text_muted_foreground(self) -> Self;
    fn text_accent(self) -> Self;
    fn text_accent_foreground(self) -> Self;
    fn text_destructive(self) -> Self;
    fn text_destructive_foreground(self) -> Self;
    fn border_border(self) -> Self;
    fn border_input(self) -> Self;
    fn border_ring(self) -> Self;
    fn border_primary(self) -> Self;
    fn border_secondary(self) -> Self;
    fn border_destructive(self) -> Self;
    fn border_muted(self) -> Self;
    fn border_accent(self) -> Self;
    fn outline_ring(self) -> Self;
    fn outline_primary(self) -> Self;
    fn outline_destructive(self) -> Self;
    fn rounded_radius(self) -> Self;
    fn rounded_radius_sm(self) -> Self;
    fn rounded_radius_md(self) -> Self;
    fn rounded_radius_lg(self) -> Self;
}
macro_rules! bg {
    ($n:ident,$f:ident) => {
        fn $n(self) -> Self {
            let t = current_theme();
            self.background(t.$f)
        }
    };
}
macro_rules! txt {
    ($n:ident,$f:ident) => {
        fn $n(self) -> Self {
            let t = current_theme();
            self.color(t.$f)
        }
    };
}
macro_rules! brd {
    ($n:ident,$f:ident) => {
        fn $n(self) -> Self {
            let t = current_theme();
            self.border_color(t.$f)
        }
    };
}
macro_rules! out {
    ($n:ident,$f:ident) => {
        fn $n(self) -> Self {
            let t = current_theme();
            self.outline_color(t.$f)
        }
    };
}
macro_rules! rad {
    ($n:ident,$f:ident) => {
        fn $n(self) -> Self {
            let t = current_theme();
            self.border_radius(t.$f)
        }
    };
}
impl ShadcnStyleExt for Style {
    bg!(bg_background, background);
    bg!(bg_foreground, foreground);
    bg!(bg_card, card);
    bg!(bg_card_foreground, card_foreground);
    bg!(bg_popover, popover);
    bg!(bg_popover_foreground, popover_foreground);
    bg!(bg_primary, primary);
    bg!(bg_primary_foreground, primary_foreground);
    bg!(bg_secondary, secondary);
    bg!(bg_secondary_foreground, secondary_foreground);
    bg!(bg_muted, muted);
    bg!(bg_muted_foreground, muted_foreground);
    bg!(bg_accent, accent);
    bg!(bg_accent_foreground, accent_foreground);
    bg!(bg_destructive, destructive);
    bg!(bg_destructive_foreground, destructive_foreground);
    txt!(text_background, background);
    txt!(text_foreground, foreground);
    txt!(text_card, card);
    txt!(text_card_foreground, card_foreground);
    txt!(text_popover, popover);
    txt!(text_popover_foreground, popover_foreground);
    txt!(text_primary, primary);
    txt!(text_primary_foreground, primary_foreground);
    txt!(text_secondary, secondary);
    txt!(text_secondary_foreground, secondary_foreground);
    txt!(text_muted, muted);
    txt!(text_muted_foreground, muted_foreground);
    txt!(text_accent, accent);
    txt!(text_accent_foreground, accent_foreground);
    txt!(text_destructive, destructive);
    txt!(text_destructive_foreground, destructive_foreground);
    brd!(border_border, border);
    brd!(border_input, input);
    brd!(border_ring, ring);
    brd!(border_primary, primary);
    brd!(border_secondary, secondary);
    brd!(border_destructive, destructive);
    brd!(border_muted, muted);
    brd!(border_accent, accent);
    out!(outline_ring, ring);
    out!(outline_primary, primary);
    out!(outline_destructive, destructive);
    rad!(rounded_radius, radius);
    rad!(rounded_radius_sm, radius_sm);
    rad!(rounded_radius_md, radius_md);
    rad!(rounded_radius_lg, radius_lg);
}
