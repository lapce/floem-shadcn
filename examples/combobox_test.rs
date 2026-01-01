//! Minimal test for Combobox component
//!
//! Run with: cargo run --example combobox_test

use floem::prelude::*;
use floem::view::ParentView;
use floem_shadcn::prelude::*;

fn main() {
    floem::launch(app_view);
}

fn app_view() -> impl IntoView {
    let selected = RwSignal::new(None::<String>);
    let search = RwSignal::new(String::new());

    Stack::vertical((
        // Status label
        Label::derived(move || {
            format!(
                "Selected: {:?}",
                selected.get()
            )
        })
        .style(|s| s.padding(8.0).margin_bottom(16.0)),
        // The combobox
        Combobox::new(selected.clone(), search.clone())
            .child(ComboboxTrigger::new("Select a framework...").items([
                ("next", "Next.js"),
                ("svelte", "SvelteKit"),
                ("nuxt", "Nuxt.js"),
            ]))
            .child(
                ComboboxContent::new()
                    .child(ComboboxInput::new())
                    .child(
                        ComboboxList::new()
                            .child(ComboboxItem::new("next", "Next.js"))
                            .child(ComboboxItem::new("svelte", "SvelteKit"))
                            .child(ComboboxItem::new("nuxt", "Nuxt.js")),
                    )
                    .child(ComboboxEmpty::new("No results found.")),
            ),
    ))
    .style(|s| s.padding(32.0).gap(8.0))
}
