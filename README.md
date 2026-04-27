# floem-shadcn

**shadcn/ui inspired components for [Floem](https://github.com/lapce/floem)**

Beautifully designed, accessible UI components for Rust GUI applications — built with Floem's reactive framework and styled with [floem-tailwind](https://github.com/elcoosp/floem-tailwind).

## Features

- **48+ components** — Accordion, Alert, AlertDialog, AspectRatio, Avatar, Badge, Breadcrumb, Button, Calendar, Card, Carousel, Checkbox, Collapsible, Combobox, Command, ContextMenu, DatePicker, Dialog, Drawer, DropdownMenu, Empty, HoverCard, Input, InputOTP, Kbd, Label, Menubar, NavigationMenu, Pagination, Popover, Progress, RadioGroup, Resizable, ScrollArea, Select, Separator, Sheet, Sidebar, Skeleton, Slider, Spinner, Switch, Table, Tabs, Textarea, Toast, Toggle, ToggleGroup, Tooltip, Typography
- **Light & Dark themes** — OKLCH-based semantic color tokens with runtime switching
- **Builder API** — Ergonomic, chainable component construction
- **Semantic styling** — `bg_primary()`, `text_muted_foreground()`, `border_input()`, and more
- **Tailwind utilities** — `px_4()`, `rounded_md()`, `gap_2()`, `text_sm()` via floem-tailwind
- **Fully reactive** — All state managed through Floem's `RwSignal`

## Quick Start

Add to your `Cargo.toml`:

```toml
[dependencies]
floem-shadcn = { git = "https://github.com/elcoosp/floem-shadcn" }
```

```rust
use floem::prelude::*;
use floem_shadcn::prelude::*;

fn app_view() -> impl IntoView {
    let checked = RwSignal::new(false);
    let progress = RwSignal::new(60.0);

    Stack::vertical((
        Button::new("Click me").secondary(),
        Button::new("Delete").destructive().sm(),
        Badge::new("New").outline(),
        Checkbox::new(checked).label("Accept terms"),
        Progress::new(progress),
        Input::new().placeholder("Email"),
    ))
    .style(|s| s.gap(12.0).padding(32.0))
}
```

## Theme

Set the theme at application startup:

```rust
use floem_shadcn::theme::{ShadcnTheme, ThemeMode};

// Light (default)
floem_shadcn::theme::set_theme(ShadcnTheme::light());

// Dark
floem_shadcn::theme::set_theme(ShadcnTheme::dark());
```

Apply theme-aware styles to any view:

```rust
use floem_shadcn::styled::ShadcnStyleExt;
use floem_shadcn::theme::ShadcnThemeExt;

// Semantic color helpers
some_view.style(|s| {
    s.bg_primary()
        .text_primary_foreground()
        .border_input()
        .rounded_md()
})

// Theme closure for arbitrary logic
some_view.style(|s| {
    s.with_shadcn_theme(|s, t| {
        s.background(t.card)
            .color(t.foreground)
            .border_color(t.border)
    })
})
```

### Color Tokens

| Token | Usage |
|---|---|
| `background` / `foreground` | Page background & text |
| `card` / `card_foreground` | Card surfaces |
| `popover` / `popover_foreground` | Floating panels |
| `primary` / `primary_foreground` | Primary actions |
| `secondary` / `secondary_foreground` | Secondary elements |
| `muted` / `muted_foreground` | Subdued content |
| `accent` / `accent_foreground` | Hover / active states |
| `destructive` / `destructive_foreground` | Error / danger |
| `border`, `input`, `ring` | Borders & focus rings |

## Components

### Button

Six variants, four sizes:

```rust
Button::new("Default");
Button::new("Secondary").secondary();
Button::new("Danger").destructive();
Button::new("Bordered").outline();
Button::new("Subtle").ghost();
Button::new("Link").link();

Button::new("Small").sm();
Button::new("Large").lg();
Button::new("≡").icon();
```

### Dialog

Context-based open/close state:

```rust
Dialog::new((
    DialogTrigger::new(Button::new("Open")),
    DialogContent::new((
        DialogHeader::new()
            .title("Confirm Action")
            .description("This cannot be undone."),
        DialogFooter::new((
            DialogClose::new(Button::new("Cancel").outline()),
            Button::new("Continue"),
        )),
    )),
));
```

### Select

```rust
let selected = RwSignal::new(Some("a".to_string()));

Select::new(selected)
    .placeholder("Choose...")
    .items(vec![
        SelectItemData::new("a", "Option A"),
        SelectItemData::new("b", "Option B"),
    ]);
```

### Sidebar

```rust
let active = RwSignal::new("home".to_string());

Sidebar::new()
    .header(SidebarHeader::new().child(label(|| "My App")))
    .content(
        SidebarContent::new().child(
            SidebarGroup::new()
                .child(SidebarGroupLabel::new("Navigation"))
                .child(
                    SidebarGroupContent::new().child(
                        SidebarMenu::new().child(
                            SidebarMenuItem::new().child(
                                SidebarMenuButton::new("Home")
                                    .is_active(move || active.get() == "home")
                                    .on_event_stop(Click, move |_, _| active.set("home"))
                            )
                        )
                    )
                )
        )
    )
    .footer(SidebarFooter::new().child(label(|| "v0.1.0")));
```

### Toast

```rust
let toasts = RwSignal::new(Vec::new());

Button::new("Notify").on_event_stop(Click, move |_, _| {
    push_toast(toasts, ToastData::new("Saved", "Changes saved successfully."));
});

ToastContainer::new(toasts);
```

## Running the Showcase

```bash
cargo run --example showcase
```

The showcase demonstrates all 48 components in a sidebar-navigated layout with live theme toggling.

## Running Tests

```bash
cargo nextest run
```

## Requirements

- Rust **1.87+** (edition 2024)
- Floem dependencies are pulled from GitHub (`lapce/floem`)

## License

MIT
