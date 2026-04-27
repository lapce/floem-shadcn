//! Full showcase for floem-shadcn components

use floem::prelude::*;
use floem::reactive::RwSignal;
use floem::text::FontWeight;
use floem::views::{Decorators, Label, Stack};
use floem_shadcn::components::accordion::{Accordion, AccordionItem};
use floem_shadcn::components::alert::Alert;
use floem_shadcn::components::alert_dialog::AlertDialog;
use floem_shadcn::components::aspect_ratio::AspectRatio;
use floem_shadcn::components::avatar::Avatar;
use floem_shadcn::components::badge::Badge;
use floem_shadcn::components::breadcrumb::{
    Breadcrumb, BreadcrumbItem, BreadcrumbPage, BreadcrumbSeparator,
};
use floem_shadcn::components::button::Button; // <-- explicit shadcn Button
use floem_shadcn::components::calendar::Calendar;
use floem_shadcn::components::card::{Card, CardContent, CardFooter, CardHeader};
use floem_shadcn::components::carousel::Carousel;
use floem_shadcn::components::checkbox::Checkbox; // <-- explicit shadcn Checkbox
use floem_shadcn::components::combobox::{
    Combobox, ComboboxContent, ComboboxEmpty, ComboboxInput, ComboboxItem, ComboboxList,
    ComboboxTrigger,
};
use floem_shadcn::components::command::{Command, CommandGroup, CommandItem, CommandList};
use floem_shadcn::components::context_menu::{ContextMenu, ContextMenuItem};
use floem_shadcn::components::date_picker::DatePicker;
use floem_shadcn::components::dialog::{
    Dialog, DialogClose, DialogContent, DialogFooter, DialogHeader, DialogTrigger,
};
use floem_shadcn::components::drawer::{Drawer, DrawerSide};
use floem_shadcn::components::dropdown_menu::{
    DropdownMenu, DropdownMenuItem, DropdownMenuSeparator,
};
use floem_shadcn::components::hover_card::HoverCard;
use floem_shadcn::components::input::Input;
use floem_shadcn::components::input_otp::InputOTP;
use floem_shadcn::components::label::FormLabel;
use floem_shadcn::components::menubar::{Menubar, MenubarItem, MenubarMenu};
use floem_shadcn::components::navigation_menu::{
    NavigationMenu, NavigationMenuItem, NavigationMenuLink,
};
use floem_shadcn::components::pagination::Pagination;
use floem_shadcn::components::popover::Popover;
use floem_shadcn::components::progress::Progress;
use floem_shadcn::components::radio_group::RadioGroupItem;
use floem_shadcn::components::resizable::{ResizableHandle, ResizablePanel, ResizablePanelGroup};
use floem_shadcn::components::scroll_area::ScrollArea;
use floem_shadcn::components::select::{Select, SelectItemData};
use floem_shadcn::components::separator::Separator;
use floem_shadcn::components::sheet::{Sheet, SheetContent, SheetSide};
use floem_shadcn::components::sidebar::{
    Sidebar, SidebarContent, SidebarFooter, SidebarGroup, SidebarGroupContent, SidebarGroupLabel,
    SidebarHeader, SidebarMenu, SidebarMenuButton, SidebarMenuItem, SidebarSeparator,
};
use floem_shadcn::components::skeleton::Skeleton;
use floem_shadcn::components::slider::Slider;
use floem_shadcn::components::switch::Switch;
use floem_shadcn::components::table::{
    Table, TableBody, TableCell, TableHead, TableHeader, TableRow,
};
use floem_shadcn::components::tabs::{Tab, Tabs, TabsContent, TabsList}; // <-- shadcn Tab
use floem_shadcn::components::textarea::Textarea;
use floem_shadcn::components::toast::{ToastContainer, ToastData, push_toast};
use floem_shadcn::components::toggle::Toggle;
use floem_shadcn::components::toggle_group::{ToggleGroup, ToggleGroupItem};
use floem_shadcn::components::tooltip::TooltipExt;
use floem_shadcn::styled::ShadcnStyleExt;
use floem_shadcn::theme::{ShadcnThemeExt, ThemeMode};
use floem_tailwind::TailwindExt; // <-- ADD THIS

fn main() {
    floem::launch(app_view);
}

fn app_view() -> impl IntoView {
    let active_section = RwSignal::new("buttons".to_string());
    let theme_mode = RwSignal::new(ThemeMode::Light);

    Stack::horizontal((
        Sidebar::new()
            .header(
                SidebarHeader::new().child(
                    Stack::vertical((
                        Label::derived(|| "floem-shadcn")
                            .style(|s| s.font_size(18.0).font_weight(FontWeight::BOLD)),
                        Button::new("Toggle Theme").outline().sm().on_event_stop(
                            floem::event::listener::Click,
                            move |_, _| {
                                theme_mode.update(|m| {
                                    *m = match m {
                                        ThemeMode::Light => ThemeMode::Dark,
                                        ThemeMode::Dark => ThemeMode::Light,
                                    }
                                });
                            },
                        ),
                    ))
                    .style(|s| s.gap(4.0)),
                ),
            )
            .content(
                SidebarContent::new()
                    .child(sidebar_group(
                        "Form Inputs",
                        active_section,
                        vec![
                            "Buttons",
                            "Badges",
                            "Cards",
                            "Inputs",
                            "Textarea",
                            "Checkbox",
                            "Switch",
                            "Radio Group",
                            "Slider",
                            "Select",
                            "Combobox",
                            "Input OTP",
                            "Date Picker",
                            "Label",
                        ],
                    ))
                    .child(SidebarSeparator::new())
                    .child(sidebar_group(
                        "Layout & Feedback",
                        active_section,
                        vec![
                            "Tabs",
                            "Accordion",
                            "Collapsible",
                            "Dialog",
                            "Alert Dialog",
                            "Drawer",
                            "Alert",
                            "Avatar",
                            "Progress",
                            "Separator",
                            "Skeleton",
                            "Tooltip",
                            "Aspect Ratio",
                            "Scroll Area",
                            "Resizable",
                        ],
                    ))
                    .child(SidebarSeparator::new())
                    .child(sidebar_group(
                        "Overlays & Navigation",
                        active_section,
                        vec![
                            "Popover",
                            "Sheet",
                            "Dropdown Menu",
                            "Menubar",
                            "Navigation Menu",
                            "Breadcrumb",
                            "Pagination",
                            "Command",
                        ],
                    ))
                    .child(SidebarSeparator::new())
                    .child(sidebar_group(
                        "Data & Misc",
                        active_section,
                        vec![
                            "Table",
                            "Calendar",
                            "Carousel",
                            "Toast",
                            "Toggle",
                            "Toggle Group",
                            "Hover Card",
                            "Context Menu",
                            "Sidebar",
                        ],
                    )),
            )
            .footer(
                SidebarFooter::new().child(Label::derived(|| "v0.1.0").style(|s| {
                    s.font_size(12.0)
                        .with_shadcn_theme(|s, t| s.color(t.muted_foreground))
                })),
            ),
        floem::views::Scroll::new(floem::views::dyn_container(
            move || active_section.get(),
            move |section| match section.as_str() {
                "buttons" => buttons_demo().into_any(),
                "badges" => badges_demo().into_any(),
                "cards" => cards_demo().into_any(),
                "inputs" => inputs_demo().into_any(),
                "textarea" => textarea_demo().into_any(),
                "checkbox" => checkbox_demo().into_any(),
                "switch" => switch_demo().into_any(),
                "tabs" => tabs_demo().into_any(),
                "dialog" => dialog_demo().into_any(),
                "alert" => alert_demo().into_any(),
                "avatar" => avatar_demo().into_any(),
                "progress" => progress_demo().into_any(),
                "separator" => separator_demo().into_any(),
                "skeleton" => skeleton_demo().into_any(),
                "tooltip" => tooltip_demo().into_any(),
                "accordion" => accordion_demo().into_any(),
                "slider" => slider_demo().into_any(),
                "radio_group" => radio_demo().into_any(),
                "popover" => popover_demo().into_any(),
                "sheet" => sheet_demo().into_any(),
                "dropdown_menu" => dropdown_demo().into_any(),
                "breadcrumb" => breadcrumb_demo().into_any(),
                "table" => table_demo().into_any(),
                "toast" => toast_demo().into_any(),
                "toggle" => toggle_demo().into_any(),
                "toggle_group" => toggle_group_demo().into_any(),
                "hover_card" => hover_card_demo().into_any(),
                "context_menu" => context_menu_demo().into_any(),
                "sidebar" => sidebar_demo().into_any(),
                "select" => select_demo().into_any(),
                "combobox" => combobox_demo().into_any(),
                "input_otp" => input_otp_demo().into_any(),
                "date_picker" => date_picker_demo().into_any(),
                "label" => label_demo().into_any(),
                "collapsible" => collapsible_demo().into_any(),
                "alert_dialog" => alert_dialog_demo().into_any(),
                "drawer" => drawer_demo().into_any(),
                "aspect_ratio" => aspect_ratio_demo().into_any(),
                "scroll_area" => scroll_area_demo().into_any(),
                "resizable" => resizable_demo().into_any(),
                "menubar" => menubar_demo().into_any(),
                "navigation_menu" => navigation_menu_demo().into_any(),
                "pagination" => pagination_demo().into_any(),
                "command" => command_demo().into_any(),
                "calendar" => calendar_demo().into_any(),
                "carousel" => carousel_demo().into_any(),
                _ => buttons_demo().into_any(),
            },
        ))
        .style(|s| s.flex_grow(1.0).h_full().p_8().bg_background()),
    ))
    .style(move |s| {
        let _ = theme_mode.get();
        s.with_shadcn_theme(|s, t| {
            s.background(t.background)
                .color(t.foreground)
                .w_full()
                .h_full()
        })
    })
}

fn sidebar_group(label: &str, active_section: RwSignal<String>, items: Vec<&str>) -> SidebarGroup {
    let mut content = SidebarGroupContent::new().child(SidebarMenu::new());
    for item in items {
        let id = item.to_lowercase().replace(' ', "_");
        let id_clone = id.clone();
        content = content.child(
            SidebarMenuItem::new().child(
                SidebarMenuButton::new(item.to_string())
                    .is_active(move || active_section.get() == id)
                    .on_event_stop(floem::event::listener::Click, move |_, _| {
                        active_section.set(id_clone.clone())
                    }),
            ),
        );
    }
    SidebarGroup::new()
        .child(SidebarGroupLabel::new(label.to_string()))
        .child(content)
}

// ---------------------------------------------------------------
// Demo functions (each uses explicit shadcn components now)
// ---------------------------------------------------------------

fn buttons_demo() -> impl IntoView {
    demo_section(
        "Buttons",
        "A button component with multiple variants and sizes.",
        Stack::vertical((Stack::horizontal((
            Button::new("Default"),
            Button::new("Secondary").secondary(),
            Button::new("Destructive").destructive(),
            Button::new("Outline").outline(),
            Button::new("Ghost").ghost(),
            Button::new("Link").link(),
        ))
        .style(|s| s.gap(8.0).flex_wrap(floem::style::FlexWrap::Wrap)),))
        .style(|s| s.gap(8.0)),
    )
}

fn badges_demo() -> impl IntoView {
    demo_section(
        "Badges",
        "A badge component for displaying status or labels.",
        Stack::horizontal((
            Badge::new("Default"),
            Badge::new("Secondary").secondary(),
            Badge::new("Destructive").destructive(),
            Badge::new("Outline").outline(),
        ))
        .style(|s| s.gap(8.0)),
    )
}

fn cards_demo() -> impl IntoView {
    demo_section(
        "Cards",
        "A card component for grouping related content.",
        Card::new((
            CardHeader::new()
                .title("Create project")
                .description("Deploy your new project in one-click."),
            CardContent::new(
                Stack::vertical((
                    Label::derived(|| "Name")
                        .style(|s| s.font_size(14.0).font_weight(FontWeight::MEDIUM)),
                    Input::new().placeholder("Name of your project"),
                ))
                .style(|s| s.gap(8.0)),
            ),
            CardFooter::new(
                Stack::horizontal((Button::new("Cancel").outline(), Button::new("Deploy")))
                    .style(|s| s.gap(8.0)),
            ),
        ))
        .style(|s| s.max_width(512.0)),
    )
}

fn inputs_demo() -> impl IntoView {
    demo_section(
        "Inputs",
        "A text input component.",
        Stack::vertical((
            Input::new().placeholder("Email"),
            Input::new().placeholder("Password"),
        ))
        .style(|s| s.gap(8.0).max_width(512.0)),
    )
}

fn textarea_demo() -> impl IntoView {
    demo_section(
        "Textarea",
        "A multi-line text input.",
        Textarea::new("")
            .placeholder("Type your message...")
            .rows(4)
            .style(|s| s.max_width(512.0)),
    )
}

fn checkbox_demo() -> impl IntoView {
    // Use shadcn Checkbox which takes RwSignal<bool>
    let c1 = RwSignal::new(false);
    demo_section(
        "Checkbox",
        "A checkbox component.",
        Stack::vertical((Checkbox::new(c1.clone()).label("Accept terms"),)).style(|s| s.gap(8.0)),
    )
}

fn switch_demo() -> impl IntoView {
    let s1 = RwSignal::new(false);
    demo_section(
        "Switch",
        "A toggle switch component.",
        Switch::new(s1).label("Airplane mode"),
    )
}

fn tabs_demo() -> impl IntoView {
    // Use shadcn Tab
    let active = RwSignal::new("account".to_string());
    demo_section(
        "Tabs",
        "A tabs component.",
        Tabs::new(
            active,
            (
                TabsList::new((
                    Tab::new("account", "Account").active(active),
                    Tab::new("password", "Password").active(active),
                )),
                TabsContent::new("account", Label::derived(|| "Account content")).active(active),
                TabsContent::new("password", Label::derived(|| "Password content")).active(active),
            ),
        ),
    )
}

fn dialog_demo() -> impl IntoView {
    demo_section(
        "Dialog",
        "A modal dialog component.",
        Dialog::new((
            DialogTrigger::new(Button::new("Open Dialog")),
            DialogContent::new((
                DialogHeader::new()
                    .title("Dialog Title")
                    .description("Dialog description."),
                DialogFooter::new(DialogClose::new(Button::new("Close"))),
            )),
        )),
    )
}

fn alert_demo() -> impl IntoView {
    demo_section(
        "Alert",
        "An alert component.",
        Stack::vertical((
            Alert::new()
                .title("Heads up!")
                .description("You can add components to your app using the CLI."),
            Alert::new()
                .destructive()
                .title("Error")
                .description("Your session has expired."),
        ))
        .style(|s| s.gap(8.0)),
    )
}

fn avatar_demo() -> impl IntoView {
    demo_section(
        "Avatar",
        "An avatar component.",
        Stack::horizontal((Avatar::new().fallback("JD"), Avatar::new().fallback("AB")))
            .style(|s| s.gap(8.0)),
    )
}

fn progress_demo() -> impl IntoView {
    let p = RwSignal::new(60.0);
    demo_section(
        "Progress",
        "A progress bar component.",
        Stack::vertical((
            Progress::new(p),
            Label::derived(move || format!("{:.0}%", p.get())),
        ))
        .style(|s| s.gap(8.0)),
    )
}

fn separator_demo() -> impl IntoView {
    demo_section(
        "Separator",
        "A visual divider.",
        Stack::vertical((
            Label::derived(|| "Above"),
            Separator::new(),
            Label::derived(|| "Below"),
        ))
        .style(|s| s.gap(8.0)),
    )
}

fn skeleton_demo() -> impl IntoView {
    demo_section(
        "Skeleton",
        "A loading placeholder.",
        Skeleton::new().width(200.0).height(20.0),
    )
}

fn tooltip_demo() -> impl IntoView {
    demo_section(
        "Tooltip",
        "A tooltip component.",
        Button::new("Hover me").tooltip_styled("This is a tooltip!"),
    )
}

fn accordion_demo() -> impl IntoView {
    let expanded = RwSignal::new(Some("item-1".to_string()));
    demo_section(
        "Accordion",
        "A collapsible content component.",
        Accordion::new(
            expanded,
            (
                AccordionItem::new("item-1", "Section 1", "Content 1"),
                AccordionItem::new("item-2", "Section 2", "Content 2"),
            ),
        ),
    )
}

fn slider_demo() -> impl IntoView {
    let v = RwSignal::new(50.0);
    demo_section(
        "Slider",
        "A slider component.",
        Stack::vertical((
            Slider::new(v),
            Label::derived(move || format!("Value: {:.0}", v.get())),
        ))
        .style(|s| s.gap(8.0)),
    )
}

fn radio_demo() -> impl IntoView {
    let sel = RwSignal::new("opt1".to_string());
    demo_section(
        "Radio Group",
        "A set of radio buttons.",
        Stack::vertical((
            RadioGroupItem::new("opt1", "Option 1").selected(sel),
            RadioGroupItem::new("opt2", "Option 2").selected(sel),
        ))
        .style(|s| s.gap(4.0)),
    )
}

fn popover_demo() -> impl IntoView {
    let open = RwSignal::new(false);
    demo_section(
        "Popover",
        "A floating panel.",
        Popover::new(open)
            .trigger(|| Button::new("Open Popover"))
            .content(|| Label::derived(|| "Popover content")),
    )
}

fn sheet_demo() -> impl IntoView {
    let open = RwSignal::new(false);
    demo_section(
        "Sheet",
        "A slide-out panel.",
        Stack::vertical((
            Button::new("Open Sheet")
                .on_event_stop(floem::event::listener::Click, move |_, _| open.set(true)),
            Sheet::new(
                open,
                SheetContent::new(Label::derived(|| "Sheet content")).side(SheetSide::Right),
            ),
        )),
    )
}

fn dropdown_demo() -> impl IntoView {
    let open = RwSignal::new(false);
    demo_section(
        "Dropdown Menu",
        "A dropdown menu.",
        DropdownMenu::new(open)
            .trigger(|| Button::new("Open Menu").outline())
            .content((
                DropdownMenuItem::new("Profile"),
                DropdownMenuItem::new("Settings"),
                DropdownMenuSeparator::new(),
                DropdownMenuItem::new("Log out").destructive(),
            )),
    )
}

fn breadcrumb_demo() -> impl IntoView {
    demo_section(
        "Breadcrumb",
        "A navigation trail.",
        Breadcrumb::new((
            BreadcrumbItem::new("Home").on_click(|| {}),
            BreadcrumbSeparator::new(),
            BreadcrumbPage::new("Current"),
        )),
    )
}

fn table_demo() -> impl IntoView {
    demo_section(
        "Table",
        "A responsive table component.",
        Table::new()
            .child(
                TableHeader::new().child(
                    TableRow::new()
                        .child(TableHead::new("Name"))
                        .child(TableHead::new("Email")),
                ),
            )
            .child(
                TableBody::new()
                    .child(
                        TableRow::new()
                            .child(TableCell::new("John Doe"))
                            .child(TableCell::new("john@example.com")),
                    )
                    .child(
                        TableRow::new()
                            .child(TableCell::new("Jane Smith"))
                            .child(TableCell::new("jane@example.com")),
                    ),
            )
            .into_view(),
    )
}

fn toast_demo() -> impl IntoView {
    let toasts = RwSignal::new(Vec::new());
    demo_section(
        "Toast",
        "A notification system.",
        Stack::vertical((
            Button::new("Show Toast").on_event_stop(floem::event::listener::Click, move |_, _| {
                push_toast(toasts, ToastData::new("Title", "Description"));
            }),
            ToastContainer::new(toasts),
        )),
    )
}

fn toggle_demo() -> impl IntoView {
    let b = RwSignal::new(false);
    demo_section("Toggle", "A two-state button.", Toggle::new(b, "Toggle me"))
}

fn toggle_group_demo() -> impl IntoView {
    let sel = RwSignal::new(Some("left".to_string()));
    demo_section(
        "Toggle Group",
        "A group of toggle buttons.",
        ToggleGroup::single(
            sel,
            (
                ToggleGroupItem::new("left", "Left").selected(sel),
                ToggleGroupItem::new("right", "Right").selected(sel),
            ),
        ),
    )
}

fn hover_card_demo() -> impl IntoView {
    demo_section(
        "Hover Card",
        "A card that appears on hover.",
        HoverCard::new()
            .trigger(|| {
                Label::derived(|| "Hover me").style(|s| {
                    s.font_weight(FontWeight::MEDIUM)
                        .cursor(floem::style::CursorStyle::Pointer)
                })
            })
            .content(|| Label::derived(|| "Card content")),
    )
}

fn context_menu_demo() -> impl IntoView {
    let open = RwSignal::new(false);
    demo_section(
        "Context Menu",
        "A right-click menu.",
        ContextMenu::new(open)
            .trigger(|| {
                Label::derived(|| "Right-click here").style(|s| s.padding(20.0).border(1.0))
            })
            .content((ContextMenuItem::new("Cut"), ContextMenuItem::new("Copy"))),
    )
}

fn sidebar_demo() -> impl IntoView {
    demo_section(
        "Sidebar",
        "A sidebar navigation component.",
        Label::derived(|| "The sidebar on the left is built with the Sidebar component.")
            .style(|s| s.with_shadcn_theme(|s, t| s.color(t.muted_foreground))),
    )
}

fn select_demo() -> impl IntoView {
    let sel = RwSignal::new(Some("a".to_string()));
    demo_section(
        "Select",
        "A dropdown select component.",
        Select::new(sel).placeholder("Select...").items(vec![
            SelectItemData::new("a", "Option A"),
            SelectItemData::new("b", "Option B"),
        ]),
    )
}

fn combobox_demo() -> impl IntoView {
    let sel = RwSignal::new(None::<String>);
    let search = RwSignal::new(String::new());
    demo_section(
        "Combobox",
        "A searchable select component.",
        Combobox::new(sel, search)
            .child(ComboboxTrigger::new("Select...").items([("a", "A"), ("b", "B")]))
            .child(
                ComboboxContent::new()
                    .child(ComboboxInput::new())
                    .child(
                        ComboboxList::new()
                            .child(ComboboxItem::new("a", "A"))
                            .child(ComboboxItem::new("b", "B")),
                    )
                    .child(ComboboxEmpty::new("No results")),
            ),
    )
}

fn input_otp_demo() -> impl IntoView {
    let otp = RwSignal::new(String::new());
    demo_section(
        "Input OTP",
        "A one-time password input.",
        InputOTP::new(otp, 6),
    )
}

fn date_picker_demo() -> impl IntoView {
    let date = RwSignal::new(None);
    demo_section(
        "Date Picker",
        "A date selection component.",
        DatePicker::new(date),
    )
}

fn label_demo() -> impl IntoView {
    demo_section("Label", "A form label.", FormLabel::new("Email"))
}

fn collapsible_demo() -> impl IntoView {
    demo_section(
        "Collapsible",
        "An expandable/collapsible component.",
        Label::derived(|| "Collapsible component (needs migration)"),
    )
}

fn alert_dialog_demo() -> impl IntoView {
    let open = RwSignal::new(false);
    demo_section(
        "Alert Dialog",
        "A modal confirmation dialog.",
        AlertDialog::new(open)
            .trigger("Delete")
            .title("Confirm")
            .action("Delete", || {}),
    )
}

fn drawer_demo() -> impl IntoView {
    let open = RwSignal::new(false);
    demo_section(
        "Drawer",
        "A slide-out panel.",
        Stack::vertical((
            Button::new("Open Drawer")
                .on_event_stop(floem::event::listener::Click, move |_, _| open.set(true)),
            Drawer::new(open)
                .side(DrawerSide::Bottom)
                .content(Label::derived(|| "Drawer content")),
        )),
    )
}

fn aspect_ratio_demo() -> impl IntoView {
    demo_section(
        "Aspect Ratio",
        "A container with fixed aspect ratio.",
        AspectRatio::video().child(|| Label::derived(|| "16:9")),
    )
}

fn scroll_area_demo() -> impl IntoView {
    demo_section(
        "Scroll Area",
        "A scrollable container.",
        ScrollArea::new(Stack::vertical((
            Label::derived(|| "Item 1"),
            Label::derived(|| "Item 2"),
            Label::derived(|| "Item 3"),
        )))
        .height(100.0),
    )
}

fn resizable_demo() -> impl IntoView {
    demo_section(
        "Resizable",
        "Resizable panels.",
        ResizablePanelGroup::horizontal((
            ResizablePanel::new(Label::derived(|| "Left")).default_size(50.0),
            ResizableHandle::new(),
            ResizablePanel::new(Label::derived(|| "Right")).default_size(50.0),
        )),
    )
}

fn menubar_demo() -> impl IntoView {
    demo_section(
        "Menubar",
        "A horizontal menu bar.",
        Menubar::new((
            MenubarMenu::new("File").content((MenubarItem::new("New"), MenubarItem::new("Open"))),
            MenubarMenu::new("Edit").content((MenubarItem::new("Cut"), MenubarItem::new("Copy"))),
        )),
    )
}

fn navigation_menu_demo() -> impl IntoView {
    demo_section(
        "Navigation Menu",
        "A site navigation menu.",
        NavigationMenu::new((
            NavigationMenuItem::new("Getting Started")
                .content((NavigationMenuLink::new("Introduction", "/"),)),
            NavigationMenuItem::new("Components")
                .content((NavigationMenuLink::new("Button", "/button"),)),
        )),
    )
}

fn pagination_demo() -> impl IntoView {
    let page = RwSignal::new(1usize);
    demo_section(
        "Pagination",
        "Page navigation.",
        Stack::vertical((
            Pagination::new(page, 5),
            Label::derived(move || format!("Page {}", page.get())),
        )),
    )
}

fn command_demo() -> impl IntoView {
    let search = RwSignal::new(String::new());
    demo_section(
        "Command",
        "A command palette.",
        Command::new(search)
            .placeholder("Type a command...")
            .child(CommandList::new().child(
                CommandGroup::new("Suggestions").child(CommandItem::new("calendar", "Calendar")),
            )),
    )
}

fn calendar_demo() -> impl IntoView {
    let sel = RwSignal::new(None);
    demo_section("Calendar", "A date selection calendar.", Calendar::new(sel))
}

fn carousel_demo() -> impl IntoView {
    let idx = RwSignal::new(0usize);
    demo_section(
        "Carousel",
        "A slideshow component.",
        Stack::vertical((
            Carousel::new(idx, 3).items(Stack::vertical((
                Label::derived(|| "Slide 1"),
                Label::derived(|| "Slide 2"),
                Label::derived(|| "Slide 3"),
            ))),
            Label::derived(move || format!("Slide {}", idx.get() + 1)),
        ))
        .style(|s| s.gap(8.0)),
    )
}

fn demo_section<V: IntoView + 'static>(
    title: &'static str,
    desc: &'static str,
    content: V,
) -> impl IntoView {
    Stack::vertical((
        Label::derived(move || title).style(|s| s.font_size(24.0).font_weight(FontWeight::BOLD)),
        Label::derived(move || desc).style(|s| {
            s.font_size(14.0)
                .with_shadcn_theme(|s, t| s.color(t.muted_foreground))
        }),
        content,
    ))
    .style(|s| s.w_full().mb_4())
}
