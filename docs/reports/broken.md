# Code Review: Broken and Stubbed Components in floem-shadcn

After a thorough deep dive into the codebase, I've identified numerous issues ranging from completely stubbed components to broken behaviors and missing functionality. Here's a comprehensive review:

## 🔴 Completely Stubbed (Non-Functional)

### 1. **NavigationMenuItem** (`src/components/navigation_menu.rs:98-112`)
```rust
fn into_view(self) -> Self::V {
    /* simplified; real impl would include dropdown */
    Box::new(floem::views::Empty::new())
}
```
**Issue**: Returns an empty view with a TODO comment. Navigation menu items render nothing.

### 2. **KbdGroup** (`src/components/kbd.rs:47-65`)
```rust
pub fn new(_: Vec<String>) -> Self {
    Self
}
// ...
fn into_view(self) -> Self::V {
    Box::new(floem::views::Empty::new())
}
```
**Issue**: Accepts keyboard shortcuts but completely ignores them and renders nothing.

### 3. **SelectItem/SelectLabel/SelectSeparator** (`src/components/select.rs:191-250`)
```rust
impl IntoView for SelectItem {
    fn into_view(self) -> Self::V {
        Box::new(floem::views::Empty::new())
    }
}
```
**Issue**: All three are exported in the prelude but render nothing when used.

### 4. **VirtualScrollArea** (`src/components/scroll_area.rs:203-222`)
```rust
impl VirtualScrollArea {
    pub fn usage_hint() -> &'static str {
        "Use floem::views::virtual_stack for virtualized scrolling of large lists"
    }
}
```
**Issue**: Not a component at all—just returns a string hint.

---

## 🟡 Partially Stubbed (Major Missing Functionality)

### 5. **Calendar::today()** (`src/components/calendar.rs:47`)
```rust
pub fn today() -> Self {
    Self::new(2025, 1, 1)  // HARDCODED!
}
```
**Issue**: Returns a hardcoded date instead of the actual system date. Should use `chrono` or similar.

### 6. **DatePicker** (`src/components/date_picker.rs:126-177`)
```rust
fn create_calendar_content(
    #[allow(unused_variables)] selected: RwSignal<Option<SimpleDate>>,
    view_year: RwSignal<i32>,
    view_month: RwSignal<u32>,
    _is_open: RwSignal<bool>,
) -> impl IntoView {
    let header = floem::views::Stack::horizontal((/* prev/next buttons */));
    floem::views::Stack::vertical((header,))  // MISSING: actual calendar grid!
}
```
**Issue**: Only renders the header navigation but NO day cells. Users cannot select dates.

### 7. **Carousel** (`src/components/carousel.rs:62-103`)
```rust
let content = floem::views::Container::new(self.items)
    .style(|s| s.flex_grow(1.0).flex().items_center().justify_center());
```
**Issue**: Displays all items at once regardless of `current` index. No sliding/cycling animation.

### 8. **Resizable** (`src/components/resizable.rs:134-209`)
```rust
impl IntoView for ResizableHandle {
    fn into_view(self) -> Self::V {
        Box::new(floem::views::Container::new(handle_indicator).style(move |s| {
            // Visual styling only - NO DRAG HANDLERS
        }))
    }
}
```
**Issue**: Renders handles but has zero drag event handling. Panels cannot be resized.

### 9. **InputOTP** (`src/components/input_otp.rs:18-45`)
```rust
fn create_otp_slot(value: RwSignal<String>, index: usize, mask: bool) -> impl IntoView {
    floem::views::Label::derived(move || {
        let val = value.get();
        // Display only - NO INPUT HANDLING
    })
}
```
**Issue**: Renders visual slots but cannot receive keyboard input. Completely non-functional as an input.

### 10. **ScrollArea Orientation** (`src/components/scroll_area.rs:135-172`)
```rust
ScrollOrientation::Vertical => floem::views::Scroll::new(self.child).style(...).into_any(),
ScrollOrientation::Horizontal => floem::views::Scroll::new(self.child).style(...).into_all(), // IDENTICAL
ScrollOrientation::Both => floem::views::Scroll::new(self.child).style(...).into_any(), // IDENTICAL
```
**Issue**: All three orientations use identical implementation. The `orientation` parameter is completely ignored.

---

## 🟠 Broken Behavior

### 11. **Accordion** (`src/components/accordion.rs:44-90`)
```rust
pub struct AccordionItem {
    expanded_signal: Option<RwSignal<Option<String>>>,  // Never set!
}

impl<V: IntoView + 'static> Accordion<V> {
    pub fn new(expanded: RwSignal<Option<String>>, child: V) -> Self {
        Self {
            expanded,  // Has signal but never passes it to children
            child,
        }
    }
}
```
**Issue**: The `Accordion` holds the expanded state but never connects it to `AccordionItem`. Clicking items does nothing.

### 12. **Command Search** (`src/components/command.rs:30-89`)
```rust
pub struct Command {
    #[allow(dead_code)]
    search: RwSignal<String>,  // Passed in but UNUSED
}

impl IntoView for CommandInput {
    fn into_view(self) -> Self::V {
        Box::new(
            TextInput::new(RwSignal::new(String::new()))  // Creates NEW signal!
                .placeholder(self.placeholder),
        )
    }
}
```
**Issue**: Creates a new signal instead of using the provided one. Search input doesn't filter items.

### 13. **HoverCard** (`src/components/hover_card.rs:90-95`)
```rust
.on_event_stop(floem::event::listener::Click, move |_, _| {
    is_hovered.set(true)
})
.on_event_stop(floem::event::listener::Click, move |_, _| {
    is_hovered.set(false)
})
```
**Issue**: Uses `Click` events instead of actual hover/pointer enter/leave events. Named "HoverCard" but behaves like a click toggle.

---

## 🟤 Incomplete (Missing Expected Features)

### 14. **Toast** (`src/components/toast.rs`)
- ❌ No auto-dismiss timer
- ❌ No enter/exit animations
- ❌ No swipe-to-dismiss
- ❌ No toast stacking/queuing

### 15. **Context Menu / Dropdown Menu** (`src/components/context_menu.rs`, `dropdown_menu.rs`)
- ⚠️ Uses relative positioning instead of window coordinates
- ⚠️ No proper click-outside-to-close backdrop
- ⚠️ No keyboard navigation (arrow keys, escape)

### 16. **Dialog / Sheet / Drawer**
- ⚠️ No focus trapping
- ⚠️ No escape key handling
- ⚠️ No animation for open/close

---

## 📋 Test Issues

### `tests/dialog.rs`
```rust
#[test]
#[ignore]
fn test_dialog_backdrop_click_closes() {
    // Backdrop click closing is flaky in headless; ignored for now.
}
```
Multiple tests marked `#[ignore]` due to overlay/backdrop click issues in headless mode.

### `tests/sidebar_style_cache.rs`
Test name doesn't match behavior—it tests click handling, not style caching.

### `tests/slider.rs`
```rust
fn create_test_slider(value: RwSignal<f64>, width: f64) -> (ViewId, impl IntoView) {
    // Creates a custom view, NOT the actual Slider component!
}
```
Tests use a mock slider instead of the real component.

---

## 🔧 Code Quality Issues

### 1. **Theme System** (`src/theme.rs:28`)
```rust
thread_local! { 
    static THEME: std::cell::RefCell<ShadcnTheme> = std::cell::RefCell::new(ShadcnTheme::light()); 
}
```
**Issue**: Uses thread-local storage which can cause issues in async/wasm contexts. Should use reactive context.

### 2. **Pervasive `#[allow(dead_code)]`**
Multiple components have `#[allow(dead_code)]` on fields that should be used:
- `Command.search`
- `Accordion.expanded`
- `ResizablePanel.min_size`, `max_size`
- `InputGroup.disabled`

### 3. **Missing Feature Documentation**
No indication in docs/prelude that these components are stubbed:
- `NavigationMenuItem`
- `KbdGroup`
- `VirtualScrollArea`
- `SelectItem` family

---

## 📊 Summary Statistics

| Category | Count | Components |
|----------|-------|------------|
| Completely Stubbed | 4 | NavigationMenuItem, KbdGroup, SelectItem*, VirtualScrollArea |
| Partially Stubbed | 6 | Calendar, DatePicker, Carousel, Resizable, InputOTP, ScrollArea |
| Broken Behavior | 3 | Accordion, Command, HoverCard |
| Incomplete UX | 6 | Toast, Context Menu, Dropdown Menu, Dialog, Sheet, Drawer |
| **Total Issues** | **19** | |

---

## 🎯 Priority Fixes

### High Priority (Breaks User Expectations)
1. **DatePicker** - Add missing calendar grid
2. **Accordion** - Connect expanded signal to items
3. **Command** - Connect search signal to input
4. **InputOTP** - Add keyboard event handling

### Medium Priority (Feature Completeness)
1. **Carousel** - Implement sliding with current index
2. **Resizable** - Add drag handlers
3. **HoverCard** - Use actual hover events
4. **ScrollArea** - Respect orientation parameter
5. **Calendar::today()** - Use real system date

### Low Priority (Stubs Can Be Removed)
1. **NavigationMenuItem** - Either implement or remove from prelude
2. **KbdGroup** - Either implement or remove from prelude
3. **SelectItem family** - Either implement or remove from prelude
4. **VirtualScrollArea** - Remove or properly delegate to floem
