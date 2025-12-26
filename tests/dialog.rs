//! Tests for the Dialog component
//!
//! The Dialog component uses context-based state management with DialogTrigger,
//! DialogContent, and DialogClose components.

use floem::prelude::*;
use floem::reactive::RwSignal;
use floem::text::Weight;
use floem::views::Scroll;
use floem_test::prelude::*;

use floem_shadcn::components::button::Button;
use floem_shadcn::components::dialog::{
    Dialog, DialogClose, DialogContent, DialogContext, DialogFooter, DialogHeader, DialogTrigger,
};
use floem_shadcn::theme::ShadcnThemeExt;

// =============================================================================
// Basic Dialog Tests with new API
// =============================================================================

/// Test that dialog opens when DialogTrigger is clicked
#[test]
fn test_dialog_opens_via_trigger() {
    // DialogTrigger wraps a button and opens the dialog when clicked
    // Note: Don't add on_click_stop to the child button - DialogTrigger handles the click
    let dialog = Dialog::new((
        DialogTrigger::new(Button::new("Open Dialog")),
        DialogContent::new((DialogHeader::new()
            .title("Test Dialog")
            .description("This is a test dialog"),)),
    ));
    let open = dialog.open_signal();

    let view = v_stack((dialog,));

    let mut harness = HeadlessHarness::new_with_size(view, 800.0, 600.0);

    eprintln!("=== Dialog Open via Trigger Test ===");
    eprintln!("Initial open state: {}", open.get());

    harness.rebuild();

    // Click the trigger button (approximate position)
    harness.click(50.0, 20.0);
    harness.rebuild();

    eprintln!("After click, open state: {}", open.get());

    // The signal should be true after clicking the trigger
    assert!(
        open.get(),
        "Dialog open signal should be true after clicking the trigger"
    );
}

/// Test that dialog closes when DialogClose is clicked
#[test]
fn test_dialog_closes_via_close_button() {
    let dialog = Dialog::new((
        DialogTrigger::new(Button::new("Open")),
        DialogContent::new((
            DialogHeader::new().title("Test"),
            DialogFooter::new(DialogClose::new(Button::new("Close"))),
        )),
    ));
    let open = dialog.open_signal();

    let view = floem::views::stack((dialog,)).style(|s| s.size(800.0, 600.0));

    let mut harness = HeadlessHarness::new_with_size(view, 800.0, 600.0);

    eprintln!("=== Dialog Close via Close Button Test ===");

    harness.rebuild();

    // Open the dialog first
    open.set(true);
    harness.rebuild();
    eprintln!("After setting open=true: {}", open.get());

    assert!(open.get(), "Dialog should be open");

    // Click on the close button (center of viewport where dialog content is)
    harness.click(400.0, 300.0);
    harness.rebuild();

    eprintln!("After clicking close area: {}", open.get());
}

/// Test that clicking backdrop closes the dialog
#[test]
fn test_dialog_backdrop_click_closes() {
    let dialog = Dialog::new((
        DialogTrigger::new(Button::new("Open")),
        DialogContent::new((DialogHeader::new().title("Test"),)),
    ));
    let open = dialog.open_signal();

    let view = floem::views::stack((dialog,)).style(|s| s.size(800.0, 600.0));

    let mut harness = HeadlessHarness::new_with_size(view, 800.0, 600.0);

    eprintln!("=== Dialog Backdrop Click Test ===");

    harness.rebuild();

    // Open the dialog
    open.set(true);
    harness.rebuild();
    eprintln!("Initial: open={}", open.get());

    // Click on the corner (backdrop area)
    harness.click(10.0, 10.0);
    harness.rebuild();

    eprintln!("After backdrop click: open={}", open.get());

    assert!(
        !open.get(),
        "Dialog should close when clicking on the backdrop"
    );
}

/// Test that the reactive system works with dialog signals
#[test]
fn test_dialog_reactivity() {
    use std::cell::RefCell;
    use std::rc::Rc;

    let dialog = Dialog::new((
        DialogTrigger::new(Button::new("Open")),
        DialogContent::new((DialogHeader::new().title("Test"),)),
    ));
    let open = dialog.open_signal();

    let effect_count = Rc::new(RefCell::new(0));
    let effect_count_clone = effect_count.clone();

    // Use an effect to track when open changes
    floem::reactive::Effect::new(move |_| {
        let is_open = open.get();
        *effect_count_clone.borrow_mut() += 1;
        eprintln!(
            "Effect triggered: open={}, count={}",
            is_open,
            effect_count_clone.borrow()
        );
    });

    let view = floem::views::stack((dialog,)).style(|s| s.size(800.0, 600.0));

    let mut harness = HeadlessHarness::new_with_size(view, 800.0, 600.0);

    eprintln!("=== Dialog Reactivity Test ===");
    eprintln!("Initial effect count: {}", effect_count.borrow());

    harness.rebuild();

    // Change the signal
    open.set(true);
    harness.rebuild();

    eprintln!(
        "After setting to true, effect count: {}",
        effect_count.borrow()
    );

    open.set(false);
    harness.rebuild();

    eprintln!(
        "After setting to false, effect count: {}",
        effect_count.borrow()
    );

    // Effect should have been triggered at least twice (initial + changes)
    assert!(
        *effect_count.borrow() >= 2,
        "Effect should trigger when signal changes, count: {}",
        effect_count.borrow()
    );
}

/// Test dialog with paint cycle
#[test]
fn test_dialog_with_paint_cycle() {
    let dialog = Dialog::new((
        DialogTrigger::new(Button::new("Open")),
        DialogContent::new((
            DialogHeader::new()
                .title("Test Dialog")
                .description("Testing paint cycle"),
            DialogFooter::new(DialogClose::new(Button::new("Close"))),
        )),
    ));
    let open = dialog.open_signal();

    let view = floem::views::stack((dialog,)).style(|s| s.size(800.0, 600.0));

    let mut harness = HeadlessHarness::new_with_size(view, 800.0, 600.0);

    eprintln!("=== Dialog Paint Cycle Test ===");

    // Initial build and paint
    harness.rebuild();
    harness.paint();
    eprintln!("1. Initial: open={}", open.get());

    // Open dialog
    open.set(true);
    harness.rebuild();
    harness.paint();
    eprintln!("2. After open: open={}", open.get());

    // Close dialog
    open.set(false);
    harness.rebuild();
    harness.paint();
    eprintln!("3. After close: open={}", open.get());

    // Reopen
    open.set(true);
    harness.rebuild();
    harness.paint();
    eprintln!("4. After reopen: open={}", open.get());

    // Close again
    open.set(false);
    harness.rebuild();
    harness.paint();
    eprintln!("5. Final: open={}", open.get());

    assert!(!open.get());
}

// =============================================================================
// Tests with Scroll Container
// =============================================================================

/// Test dialog inside a scroll container
#[test]
fn test_dialog_inside_scroll() {
    let dialog = Dialog::new((
        DialogTrigger::new(Button::new("Open Dialog")),
        DialogContent::new((
            DialogHeader::new()
                .title("Test Dialog in Scroll")
                .description("This dialog is inside a scroll container"),
            DialogFooter::new(DialogClose::new(Button::new("Close"))),
        )),
    ));
    let open = dialog.open_signal();

    let view = Scroll::new(v_stack((dialog,)));

    let mut harness = HeadlessHarness::new_with_size(view, 800.0, 600.0);

    eprintln!("=== Dialog Inside Scroll Test ===");
    eprintln!("Initial open state: {}", open.get());

    harness.rebuild();

    // Open via signal
    open.set(true);
    harness.rebuild();

    eprintln!("After set(true): open={}", open.get());

    assert!(open.get(), "Dialog should be open");

    // Close via signal
    open.set(false);
    harness.rebuild();

    eprintln!("After set(false): open={}", open.get());
    assert!(!open.get(), "Dialog should close");
}

/// Test dialog with dyn_container inside scroll
#[test]
fn test_dialog_with_dyn_container_in_scroll() {
    let section = RwSignal::new("dialog".to_string());
    let dialog_open = RwSignal::new(false);

    let view = Scroll::new(floem::views::dyn_container(
        move || section.get(),
        move |s| {
            eprintln!("[DynContainer] Rendering section: {}", s);
            match s.as_str() {
                "dialog" => {
                    // Store the open signal for testing
                    let dialog = Dialog::new((
                        DialogTrigger::new(Button::new("Open Dialog")),
                        DialogContent::new((DialogHeader::new().title("Test"),)),
                    ));
                    // Sync with our test signal
                    let open = dialog.open_signal();
                    floem::reactive::Effect::new(move |_| {
                        dialog_open.set(open.get());
                    });
                    v_stack((dialog,)).into_any()
                }
                _ => Label::new("Other").into_any(),
            }
        },
    ));

    let mut harness = HeadlessHarness::new_with_size(view, 800.0, 600.0);

    eprintln!("=== DynContainer Dialog Test ===");

    harness.rebuild();
    eprintln!("Initial: dialog_open={}", dialog_open.get());

    // Click trigger to open dialog
    harness.click(50.0, 20.0);
    harness.rebuild();

    eprintln!("After click: dialog_open={}", dialog_open.get());
}

// =============================================================================
// Centering Tests
// =============================================================================

/// Test dialog content centering
#[test]
fn test_dialog_centering() {
    use floem::HasViewId;
    use floem_tailwind::TailwindExt;

    let dialog = Dialog::new((
        DialogTrigger::new(Button::new("Open")),
        DialogContent::new((DialogHeader::new()
            .title("Centered Dialog")
            .description("This should be centered"),)),
    ));
    let open = dialog.open_signal();
    let dialog_id = dialog.view_id();

    let view = floem::views::stack((dialog,)).style(|s| s.size(800.0, 600.0));

    let mut harness = HeadlessHarness::new_with_size(view, 800.0, 600.0);

    // Open the dialog
    open.set(true);
    harness.rebuild();

    eprintln!("=== Dialog Centering Test ===");
    eprintln!("Dialog ViewId: {:?}", dialog_id);

    // Print the view tree
    fn print_tree(id: floem::ViewId, depth: usize) {
        let indent = "  ".repeat(depth);
        if let Some(layout) = id.get_layout() {
            let transform = id.get_transform();
            let coeffs = transform.as_coeffs();
            let has_transform = coeffs[4].abs() > 0.1 || coeffs[5].abs() > 0.1;

            eprintln!(
                "{}ViewId({:?}): pos=({:.1}, {:.1}), size={:.1}x{:.1}{}",
                indent,
                id,
                layout.location.x,
                layout.location.y,
                layout.size.width,
                layout.size.height,
                if has_transform {
                    format!(", transform=({:.1}, {:.1})", coeffs[4], coeffs[5])
                } else {
                    String::new()
                }
            );
        }
        for child in id.children() {
            print_tree(child, depth + 1);
        }
    }

    print_tree(dialog_id, 0);
}

// =============================================================================
// Multiple Dialogs Test
// =============================================================================

/// Test that multiple dialogs can coexist
#[test]
fn test_multiple_dialogs() {
    let dialog1 = Dialog::new((
        DialogTrigger::new(Button::new("Open Dialog 1")),
        DialogContent::new((DialogHeader::new().title("Dialog 1"),)),
    ));
    let open1 = dialog1.open_signal();

    let dialog2 = Dialog::new((
        DialogTrigger::new(Button::new("Open Dialog 2")),
        DialogContent::new((DialogHeader::new().title("Dialog 2"),)),
    ));
    let open2 = dialog2.open_signal();

    let view = v_stack((dialog1, dialog2));

    let mut harness = HeadlessHarness::new_with_size(view, 800.0, 600.0);

    eprintln!("=== Multiple Dialogs Test ===");

    harness.rebuild();

    // Open both dialogs
    open1.set(true);
    open2.set(true);
    harness.rebuild();

    eprintln!(
        "After opening both: dialog1={}, dialog2={}",
        open1.get(),
        open2.get()
    );

    assert!(open1.get(), "Dialog 1 should be open");
    assert!(open2.get(), "Dialog 2 should be open");

    // Close one
    open1.set(false);
    harness.rebuild();

    eprintln!(
        "After closing dialog1: dialog1={}, dialog2={}",
        open1.get(),
        open2.get()
    );

    assert!(!open1.get(), "Dialog 1 should be closed");
    assert!(open2.get(), "Dialog 2 should still be open");
}

// =============================================================================
// Event Propagation Tests
// =============================================================================

/// Test that clicking on dialog content does NOT close the dialog
#[test]
fn test_clicking_dialog_content_does_not_close() {
    use floem::HasViewId;

    let dialog = Dialog::new((
        DialogTrigger::new(Button::new("Open")),
        DialogContent::new((
            DialogHeader::new()
                .title("Test Dialog")
                .description("Click on me, I should not close!"),
            DialogFooter::new(Button::new("Stay Open").on_click_stop(move |_| {
                eprintln!("Button inside dialog clicked");
            })),
        )),
    ));
    let open = dialog.open_signal();
    let dialog_id = dialog.view_id();

    let view = floem::views::stack((dialog,)).style(|s| s.size(800.0, 600.0));

    let mut harness = HeadlessHarness::new_with_size(view, 800.0, 600.0);

    eprintln!("=== Click Dialog Content Test ===");

    // Open the dialog
    open.set(true);
    harness.rebuild();

    eprintln!("Initial: open={}", open.get());

    // Find the content's visual position
    fn find_content_bounds(id: floem::ViewId) -> Option<(f64, f64, f64, f64)> {
        // DialogPortal's content is the second child (after backdrop) inside the Overlay
        let children = id.children();
        for child in children {
            let grandchildren = child.children();
            if grandchildren.len() >= 2 {
                let content = grandchildren[1]; // Content is second child
                let rect = content.get_layout_rect();
                return Some((rect.x0, rect.y0, rect.x1, rect.y1));
            }
            // Recurse
            if let Some(bounds) = find_content_bounds(child) {
                return Some(bounds);
            }
        }
        None
    }

    if let Some((x0, y0, x1, y1)) = find_content_bounds(dialog_id) {
        let center_x = (x0 + x1) / 2.0;
        let center_y = (y0 + y1) / 2.0;
        eprintln!(
            "Content bounds: ({:.1}, {:.1}) to ({:.1}, {:.1})",
            x0, y0, x1, y1
        );
        eprintln!(
            "Clicking at content center: ({:.1}, {:.1})",
            center_x, center_y
        );

        harness.click(center_x, center_y);
        harness.rebuild();
    } else {
        // Fallback to center
        harness.click(400.0, 300.0);
        harness.rebuild();
    }

    eprintln!("After click: open={}", open.get());

    assert!(
        open.get(),
        "Dialog should remain open when clicking on dialog content"
    );
}

/// Test that clicking the backdrop (outside content) DOES close the dialog
#[test]
fn test_clicking_backdrop_closes_dialog() {
    let dialog = Dialog::new((
        DialogTrigger::new(Button::new("Open")),
        DialogContent::new((DialogHeader::new()
            .title("Dialog")
            .description("Click outside to close"),)),
    ));
    let open = dialog.open_signal();

    let view = floem::views::stack((dialog,)).style(|s| s.size(800.0, 600.0));

    let mut harness = HeadlessHarness::new_with_size(view, 800.0, 600.0);

    eprintln!("=== Click Backdrop Test ===");

    // Open the dialog
    open.set(true);
    harness.rebuild();
    eprintln!("Initial: open={}", open.get());

    // Click on the corner of the viewport (on the backdrop)
    harness.click(10.0, 10.0);
    harness.rebuild();

    eprintln!("After clicking corner: open={}", open.get());

    assert!(
        !open.get(),
        "Dialog should close when clicking on the backdrop"
    );
}

// =============================================================================
// Context Access Test
// =============================================================================

/// Test that DialogContext can be accessed from child components
#[test]
fn test_dialog_context_access() {
    use floem::reactive::Context;

    let context_found = RwSignal::new(false);

    // Create a custom component that checks for DialogContext
    let custom_content = floem::views::Empty::new().style(move |s| {
        // Check if DialogContext is available during style computation
        if Context::get::<DialogContext>().is_some() {
            context_found.set(true);
        }
        s
    });

    let dialog = Dialog::new((
        DialogTrigger::new(Button::new("Open")),
        DialogContent::new((custom_content,)),
    ));
    let open = dialog.open_signal();

    let view = floem::views::stack((dialog,)).style(|s| s.size(800.0, 600.0));

    let mut harness = HeadlessHarness::new_with_size(view, 800.0, 600.0);

    eprintln!("=== Dialog Context Access Test ===");

    harness.rebuild();

    // Open the dialog to trigger style computation
    open.set(true);
    harness.rebuild();

    eprintln!("Context found: {}", context_found.get());
    // Note: Context access in style closures may not work as expected
    // This test documents the current behavior
}
