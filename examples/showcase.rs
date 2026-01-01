//! Showcase example for floem-shadcn components
//!
//! Run with: cargo run --example showcase

use floem::IntoView;
use floem::reactive::{RwSignal, SignalGet, SignalUpdate};
use floem::text::Weight;
use floem::views::{Decorators, Label, Stack};
use floem_shadcn::prelude::*;
use floem_shadcn::theme::{ShadcnTheme, ShadcnThemeExt, ThemeMode};
use floem_tailwind::TailwindExt;

fn main() {
    floem::launch(app_view);
}

fn app_view() -> impl IntoView {
    // Track which component section is active
    let active_section = RwSignal::new("buttons".to_string());
    // Track current theme mode for reactive theme switching
    let theme_mode = RwSignal::new(ThemeMode::Light);

    Stack::horizontal((
        // Sidebar navigation using full Sidebar APIs
        Sidebar::new()
            .header(
                SidebarHeader::new().child(
                    Stack::vertical((
                        Label::derived(|| "floem-shadcn")
                            .style(|s| s.font_size(18.0).font_weight(Weight::BOLD)),
                        Button::new("Toggle Theme")
                            .outline()
                            .sm()
                            .on_click_stop(move |_| {
                                theme_mode.update(|m| {
                                    *m = match m {
                                        ThemeMode::Light => ThemeMode::Dark,
                                        ThemeMode::Dark => ThemeMode::Light,
                                    }
                                });
                            }),
                    ))
                    .style(|s| s.gap_3()),
                ),
            )
            .content(
                SidebarContent::new()
                    // Group 1: Form Inputs
                    .child(
                        SidebarGroup::new()
                            .child(SidebarGroupLabel::new("Form Inputs"))
                            .child(
                                SidebarGroupContent::new().child(
                                    SidebarMenu::new()
                                        .child(sidebar_button("Buttons", active_section))
                                        .child(sidebar_button("Badges", active_section))
                                        .child(sidebar_button("Cards", active_section))
                                        .child(sidebar_button("Inputs", active_section))
                                        .child(sidebar_button("Textarea", active_section))
                                        .child(sidebar_button("Checkbox", active_section))
                                        .child(sidebar_button("Switch", active_section))
                                        .child(sidebar_button("Radio Group", active_section))
                                        .child(sidebar_button("Slider", active_section))
                                        .child(sidebar_button("Select", active_section))
                                        .child(sidebar_button("Combobox", active_section))
                                        .child(sidebar_button("Input OTP", active_section))
                                        .child(sidebar_button("Date Picker", active_section))
                                        .child(sidebar_button("Label", active_section)),
                                ),
                            ),
                    )
                    .child(SidebarSeparator::new())
                    // Group 2: Layout & Feedback
                    .child(
                        SidebarGroup::new()
                            .child(SidebarGroupLabel::new("Layout & Feedback"))
                            .child(
                                SidebarGroupContent::new().child(
                                    SidebarMenu::new()
                                        .child(sidebar_button("Tabs", active_section))
                                        .child(sidebar_button("Accordion", active_section))
                                        .child(sidebar_button("Collapsible", active_section))
                                        .child(sidebar_button("Dialog", active_section))
                                        .child(sidebar_button("Alert Dialog", active_section))
                                        .child(sidebar_button("Drawer", active_section))
                                        .child(sidebar_button("Alert", active_section))
                                        .child(sidebar_button("Avatar", active_section))
                                        .child(sidebar_button("Progress", active_section))
                                        .child(sidebar_button("Separator", active_section))
                                        .child(sidebar_button("Skeleton", active_section))
                                        .child(sidebar_button("Tooltip", active_section))
                                        .child(sidebar_button("Aspect Ratio", active_section))
                                        .child(sidebar_button("Scroll Area", active_section))
                                        .child(sidebar_button("Resizable", active_section)),
                                ),
                            ),
                    )
                    .child(SidebarSeparator::new())
                    // Group 3: Overlays & Navigation
                    .child(
                        SidebarGroup::new()
                            .child(SidebarGroupLabel::new("Overlays & Navigation"))
                            .child(
                                SidebarGroupContent::new().child(
                                    SidebarMenu::new()
                                        .child(sidebar_button("Popover", active_section))
                                        .child(sidebar_button("Sheet", active_section))
                                        .child(sidebar_button("Dropdown Menu", active_section))
                                        .child(sidebar_button("Menubar", active_section))
                                        .child(sidebar_button("Navigation Menu", active_section))
                                        .child(sidebar_button("Breadcrumb", active_section))
                                        .child(sidebar_button("Pagination", active_section))
                                        .child(sidebar_button("Command", active_section)),
                                ),
                            ),
                    )
                    .child(SidebarSeparator::new())
                    // Group 4: Data & Misc
                    .child(
                        SidebarGroup::new()
                            .child(SidebarGroupLabel::new("Data & Misc"))
                            .child(
                                SidebarGroupContent::new().child(
                                    SidebarMenu::new()
                                        .child(sidebar_button("Table", active_section))
                                        .child(sidebar_button("Calendar", active_section))
                                        .child(sidebar_button("Carousel", active_section))
                                        .child(sidebar_button("Toast", active_section))
                                        .child(sidebar_button("Toggle", active_section))
                                        .child(sidebar_button("Toggle Group", active_section))
                                        .child(sidebar_button("Hover Card", active_section))
                                        .child(sidebar_button("Context Menu", active_section))
                                        .child(sidebar_button("Sidebar", active_section)),
                                ),
                            ),
                    ),
            )
            .footer(
                SidebarFooter::new().child(Label::derived(|| "v0.1.0").style(|s| {
                    s.font_size(12.0)
                        .with_shadcn_theme(|s, t| s.color(t.muted_foreground))
                })),
            ),
        // Main content area
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
        let theme = match theme_mode.get() {
            ThemeMode::Light => ShadcnTheme::light(),
            ThemeMode::Dark => ShadcnTheme::dark(),
        };
        s.shadcn_theme(theme)
            .w_full()
            .h_full()
            .bg_background()
            .text_foreground()
    })
}

// ============================================================================
// Component Demos
// ============================================================================

fn buttons_demo() -> impl IntoView {
    demo_section(
        "Buttons",
        "A button component with multiple variants and sizes.",
        Stack::vertical((
            // Button variants
            subsection(
                "Variants",
                Stack::horizontal((
                    Button::new("Default"),
                    Button::new("Secondary").secondary(),
                    Button::new("Destructive").destructive(),
                    Button::new("Outline").outline(),
                    Button::new("Ghost").ghost(),
                    Button::new("Link").link(),
                ))
                .style(|s| s.gap_2().flex_wrap(floem::style::FlexWrap::Wrap)),
            ),
            // Button sizes
            subsection(
                "Sizes",
                Stack::horizontal((
                    Button::new("Small").sm(),
                    Button::new("Default"),
                    Button::new("Large").lg(),
                ))
                .style(|s| s.gap_2().items_center()),
            ),
        ))
        .style(|s| s.gap_8()),
    )
}

fn badges_demo() -> impl IntoView {
    demo_section(
        "Badges",
        "A badge component for displaying status or labels.",
        Stack::vertical((subsection(
            "Variants",
            Stack::horizontal((
                Badge::new("Default"),
                Badge::new("Secondary").secondary(),
                Badge::new("Destructive").destructive(),
                Badge::new("Outline").outline(),
            ))
            .style(|s| s.gap_2()),
        ),)),
    )
}

fn cards_demo() -> impl IntoView {
    let project_name = RwSignal::new(String::new());

    demo_section(
        "Cards",
        "A card component for grouping related content.",
        Stack::vertical((Card::new((
            CardHeader::new()
                .title("Create project")
                .description("Deploy your new project in one-click."),
            CardContent::new(
                Stack::vertical((
                    Label::derived(|| "Name")
                        .style(|s| s.font_size(14.0).font_weight(Weight::MEDIUM).mb_2()),
                    Input::new()
                        .placeholder("Name of your project")
                        .value(move || project_name.get())
                        .on_update(move |s| project_name.set(s.to_string()))
                        .style(|s| s.w_full()),
                ))
                .style(|s| s.gap_2().w_full()),
            ),
            CardFooter::new(
                Stack::horizontal((Button::new("Cancel").outline(), Button::new("Deploy")))
                    .style(|s| s.gap_2()),
            ),
        ))
        .style(|s| s.max_w_md()),)),
    )
}

fn inputs_demo() -> impl IntoView {
    let email = RwSignal::new(String::new());
    let password = RwSignal::new(String::new());

    demo_section(
        "Inputs",
        "A text input component with placeholder support.",
        Stack::vertical((subsection(
            "Basic",
            Stack::vertical((
                form_field(
                    "Email",
                    Input::new()
                        .placeholder("Enter your email")
                        .value(move || email.get())
                        .on_update(move |s| email.set(s.to_string())),
                ),
                form_field(
                    "Password",
                    Input::new()
                        .placeholder("Enter your password")
                        .value(move || password.get())
                        .on_update(move |s| password.set(s.to_string())),
                ),
            ))
            .style(|s| s.gap_4().max_w_md()),
        ),)),
    )
}

fn sidebar_demo() -> impl IntoView {
    demo_section(
        "Sidebar",
        "A sidebar component for navigation.",
        Stack::vertical((
            Label::derived(
                || "The sidebar you see on the left is built using the Sidebar component.",
            )
            .style(|s| s.with_shadcn_theme(|s, t| s.color(t.muted_foreground))),
            Label::derived(|| "It supports sections, items with active state, and click handlers."),
        ))
        .style(|s| s.gap_2()),
    )
}

fn checkbox_demo() -> impl IntoView {
    let checked1 = RwSignal::new(false);
    let checked2 = RwSignal::new(true);
    let checked3 = RwSignal::new(false);

    demo_section(
        "Checkbox",
        "A checkbox component for toggling boolean values.",
        Stack::vertical((
            subsection(
                "Basic",
                Stack::vertical((
                    Checkbox::new(checked1).label("Accept terms and conditions"),
                    Checkbox::new(checked2).label("Send me updates"),
                    Checkbox::new(checked3).label("Enable notifications"),
                ))
                .style(|s| s.gap_3()),
            ),
            subsection(
                "Disabled",
                Stack::vertical((Checkbox::new(RwSignal::new(false))
                    .label("Disabled checkbox")
                    .disabled(true),))
                .style(|s| s.gap_3()),
            ),
        ))
        .style(|s| s.gap_8()),
    )
}

fn switch_demo() -> impl IntoView {
    let enabled1 = RwSignal::new(false);
    let enabled2 = RwSignal::new(true);

    demo_section(
        "Switch",
        "A toggle switch component like iOS.",
        Stack::vertical((
            subsection(
                "Basic",
                Stack::vertical((
                    Switch::new(enabled1).label("Airplane mode"),
                    Switch::new(enabled2).label("Dark mode"),
                ))
                .style(|s| s.gap_3()),
            ),
            subsection(
                "Disabled",
                Stack::vertical((Switch::new(RwSignal::new(false))
                    .label("Disabled switch")
                    .disabled(true),))
                .style(|s| s.gap_3()),
            ),
        ))
        .style(|s| s.gap_8()),
    )
}

fn tabs_demo() -> impl IntoView {
    let active_tab = RwSignal::new("account".to_string());

    demo_section(
        "Tabs",
        "A tabs component for switching between content panels.",
        Stack::vertical((Tabs::new(
            active_tab,
            (
                TabsList::new((
                    Tab::new("account", "Account").active(active_tab),
                    Tab::new("password", "Password").active(active_tab),
                    Tab::new("settings", "Settings").active(active_tab),
                )),
                TabsContent::new(
                    "account",
                    Card::new((CardHeader::new()
                        .title("Account")
                        .description("Manage your account settings."),))
                    .style(|s| s.w_full()),
                )
                .active(active_tab),
                TabsContent::new(
                    "password",
                    Card::new((CardHeader::new()
                        .title("Password")
                        .description("Change your password here."),)),
                )
                .active(active_tab),
                TabsContent::new(
                    "settings",
                    Card::new((CardHeader::new()
                        .title("Settings")
                        .description("Configure your preferences."),)),
                )
                .active(active_tab),
            ),
        )
        .style(|s| s.max_w_md()),))
        .style(|s| s.gap_8()),
    )
}

fn dialog_demo() -> impl IntoView {
    demo_section(
        "Dialog",
        "A modal dialog component for important interactions.",
        Stack::vertical((subsection(
            "Basic Dialog",
            Dialog::new((
                DialogTrigger::new(Button::new("Open Dialog")),
                DialogContent::new((
                    DialogHeader::new().title("Are you sure?").description(
                        "This action cannot be undone. This will permanently delete your account.",
                    ),
                    DialogFooter::new(
                        Stack::horizontal((
                            DialogClose::new(Button::new("Cancel").outline()),
                            DialogClose::new(Button::new("Continue").destructive()),
                        ))
                        .style(|s| s.gap_2()),
                    ),
                )),
            )),
        ),))
        .style(|s| s.gap_8()),
    )
}

fn alert_demo() -> impl IntoView {
    demo_section(
        "Alert",
        "An alert component for displaying feedback messages.",
        Stack::vertical((
            subsection(
                "Default",
                Alert::new()
                    .title("Heads up!")
                    .description("You can add components to your app using the CLI."),
            ),
            subsection(
                "Destructive",
                Alert::new()
                    .destructive()
                    .title("Error")
                    .description("Your session has expired. Please log in again."),
            ),
        ))
        .style(|s| s.gap_8().max_w_lg()),
    )
}

fn textarea_demo() -> impl IntoView {
    demo_section(
        "Textarea",
        "A multi-line text input component.",
        Stack::vertical((
            subsection(
                "Basic",
                Textarea::new("")
                    .placeholder("Type your message here...")
                    .rows(4),
            ),
            subsection(
                "Resizable",
                Textarea::new("")
                    .placeholder("Drag the bottom-right corner to resize...")
                    .rows(4)
                    .resizable(true),
            ),
        ))
        .style(|s| s.gap_8().max_w_md()),
    )
}

fn avatar_demo() -> impl IntoView {
    demo_section(
        "Avatar",
        "An avatar component for displaying user images with fallback.",
        Stack::vertical((
            subsection(
                "Sizes",
                Stack::horizontal((
                    Avatar::new().fallback("SM").size(32.0),
                    Avatar::new().fallback("MD").size(40.0),
                    Avatar::new().fallback("LG").size(48.0),
                    Avatar::new().fallback("XL").size(64.0),
                ))
                .style(|s| s.gap_4().items_center()),
            ),
            subsection(
                "Initials",
                Stack::horizontal((
                    Avatar::new().fallback("JD"),
                    Avatar::new().fallback("AB"),
                    Avatar::new().fallback("MK"),
                ))
                .style(|s| s.gap_4()),
            ),
        ))
        .style(|s| s.gap_8()),
    )
}

fn progress_demo() -> impl IntoView {
    let progress_value = RwSignal::new(60.0);

    demo_section(
        "Progress",
        "A progress bar component for showing completion status.",
        Stack::vertical((subsection(
            "Basic",
            Stack::vertical((
                Progress::new(progress_value),
                Stack::horizontal((
                    Button::new("-10").sm().outline().on_click_stop(move |_| {
                        progress_value.update(|v| *v = (*v - 10.0).max(0.0))
                    }),
                    Button::new("+10").sm().outline().on_click_stop(move |_| {
                        progress_value.update(|v| *v = (*v + 10.0).min(100.0))
                    }),
                ))
                .style(|s| s.gap_2()),
            ))
            .style(|s| s.gap_4()),
        ),))
        .style(|s| s.gap_8().max_w_md()),
    )
}

fn separator_demo() -> impl IntoView {
    demo_section(
        "Separator",
        "A visual divider between content sections.",
        Stack::vertical((
            subsection(
                "Horizontal",
                Stack::vertical((
                    Label::derived(|| "Content above"),
                    Separator::new(),
                    Label::derived(|| "Content below"),
                ))
                .style(|s| s.gap_4()),
            ),
            subsection(
                "Vertical",
                Stack::horizontal((
                    Label::derived(|| "Left"),
                    Separator::new().vertical(),
                    Label::derived(|| "Right"),
                ))
                .style(|s| s.gap_4().items_center().height(40.0)),
            ),
        ))
        .style(|s| s.gap_8()),
    )
}

fn skeleton_demo() -> impl IntoView {
    demo_section(
        "Skeleton",
        "A loading placeholder component.",
        Stack::vertical((
            subsection(
                "Basic Shapes",
                Stack::vertical((
                    Skeleton::text(),
                    Skeleton::new().width(200.0).height(20.0),
                    Skeleton::new().width(150.0).height(20.0),
                ))
                .style(|s| s.gap_2()),
            ),
            subsection(
                "Card Skeleton",
                Stack::horizontal((
                    Skeleton::new().circle(48.0),
                    Stack::vertical((
                        Skeleton::new().width(150.0).height(16.0),
                        Skeleton::new().width(100.0).height(14.0),
                    ))
                    .style(|s| s.gap_2()),
                ))
                .style(|s| s.gap_4().items_center()),
            ),
        ))
        .style(|s| s.gap_8()),
    )
}

fn tooltip_demo() -> impl IntoView {
    demo_section(
        "Tooltip",
        "A tooltip component for showing hints on hover.",
        Stack::vertical((subsection(
            "Basic",
            Stack::horizontal((
                Button::new("Hover me").tooltip_styled("This is a tooltip!"),
                Button::new("Another")
                    .outline()
                    .tooltip_styled("Another tooltip"),
            ))
            .style(|s| s.gap_4()),
        ),))
        .style(|s| s.gap_8()),
    )
}

fn accordion_demo() -> impl IntoView {
    let expanded = RwSignal::new(Some("item-1".to_string()));

    demo_section(
        "Accordion",
        "A collapsible content component.",
        Stack::vertical((subsection(
            "Basic",
            Stack::vertical((
                AccordionItem::new(
                    "item-1",
                    "Is it accessible?",
                    "Yes. It adheres to the WAI-ARIA design pattern.",
                )
                .expanded(expanded),
                AccordionItem::new(
                    "item-2",
                    "Is it styled?",
                    "Yes. It comes with default styles that match the other components.",
                )
                .expanded(expanded),
                AccordionItem::new(
                    "item-3",
                    "Is it animated?",
                    "Yes. Animations can be added with CSS transitions.",
                )
                .expanded(expanded),
            )),
        ),))
        .style(|s| s.gap_8().max_w_lg()),
    )
}

fn slider_demo() -> impl IntoView {
    let value = RwSignal::new(50.0);

    demo_section(
        "Slider",
        "A slider component for selecting values from a range.",
        Stack::vertical((subsection(
            "Basic",
            Stack::vertical((
                Slider::new(value),
                Label::derived(move || format!("Value: {:.0}", value.get()))
                    .style(|s| s.font_size(14.0)),
            ))
            .style(|s| s.gap_4()),
        ),))
        .style(|s| s.gap_8().max_w_md()),
    )
}

fn radio_demo() -> impl IntoView {
    let selected = RwSignal::new("option1".to_string());

    demo_section(
        "Radio Group",
        "A set of radio buttons for selecting one option.",
        Stack::vertical((
            subsection(
                "Basic",
                Stack::vertical((
                    RadioGroupItem::new("option1", "Default").selected(selected),
                    RadioGroupItem::new("option2", "Comfortable").selected(selected),
                    RadioGroupItem::new("option3", "Compact").selected(selected),
                ))
                .style(|s| s.gap_2()),
            ),
            subsection(
                "Current Selection",
                Label::derived(move || format!("Selected: {}", selected.get())).style(|s| {
                    s.font_size(14.0)
                        .with_shadcn_theme(|s, t| s.color(t.muted_foreground))
                }),
            ),
        ))
        .style(|s| s.gap_8()),
    )
}

fn popover_demo() -> impl IntoView {
    let popover_open = RwSignal::new(false);

    demo_section(
        "Popover",
        "A floating panel that appears when triggered.",
        Stack::vertical((subsection(
            "Basic",
            Popover::new(popover_open)
                .trigger(|| Button::new("Open Popover"))
                .content(|| {
                    Stack::vertical((
                        Label::derived(|| "Dimensions")
                            .style(|s| s.font_size(14.0).font_weight(Weight::MEDIUM)),
                        Label::derived(|| "Set the dimensions for the layer.").style(|s| {
                            s.font_size(12.0)
                                .with_shadcn_theme(|s, t| s.color(t.muted_foreground))
                        }),
                    ))
                    .style(|s| s.gap_2().min_width(200.0))
                }),
        ),))
        .style(|s| s.gap_8()),
    )
}

fn sheet_demo() -> impl IntoView {
    let sheet_open = RwSignal::new(false);
    let sheet_left_open = RwSignal::new(false);

    demo_section(
        "Sheet",
        "A slide-out panel overlay for additional content.",
        Stack::vertical((
            subsection(
                "Right Sheet",
                Stack::vertical((
                    Button::new("Open Sheet").on_click_stop(move |_| sheet_open.set(true)),
                    Sheet::new(sheet_open, SheetContent::new(
                        Stack::vertical((
                            SheetHeader::new(Stack::vertical((
                                SheetTitle::new("Edit Profile"),
                                SheetDescription::new("Make changes to your profile here. Click save when you're done."),
                            ))),
                            // Some content
                            Stack::vertical((
                                Label::derived(|| "Name").style(|s| s.font_size(14.0).font_weight(Weight::MEDIUM)),
                                Label::derived(|| "Enter your name here...").style(|s| {
                                    s.font_size(14.0).with_shadcn_theme(|s, t| s.color(t.muted_foreground))
                                }),
                            )).style(|s| s.gap_2()),
                            SheetFooter::new(
                                Button::new("Save changes").on_click_stop(move |_| sheet_open.set(false))
                            ),
                        )).style(|s| s.gap_4())
                    ).side(SheetSide::Right)),
                ))
                .style(|s| s.gap_4()),
            ),
            subsection(
                "Left Sheet",
                Stack::vertical((
                    Button::new("Open Left Sheet").outline().on_click_stop(move |_| sheet_left_open.set(true)),
                    Sheet::new(sheet_left_open, SheetContent::new(
                        Stack::vertical((
                            SheetHeader::new(Stack::vertical((
                                SheetTitle::new("Navigation"),
                                SheetDescription::new("Browse menu options."),
                            ))),
                            Stack::vertical((
                                Label::new("Home"),
                                Label::new("Products"),
                                Label::new("About"),
                                Label::new("Contact"),
                            )).style(|s| s.gap_2()),
                        )).style(|s| s.gap_4())
                    ).side(SheetSide::Left)),
                ))
                .style(|s| s.gap_4()),
            ),
        ))
        .style(|s| s.gap_8()),
    )
}

fn breadcrumb_demo() -> impl IntoView {
    demo_section(
        "Breadcrumb",
        "A navigation trail showing the current location.",
        Stack::vertical((
            subsection(
                "Basic",
                Breadcrumb::new((
                    BreadcrumbItem::new("Home").href("/"),
                    BreadcrumbSeparator::new(),
                    BreadcrumbItem::new("Components").href("/components"),
                    BreadcrumbSeparator::new(),
                    BreadcrumbPage::new("Breadcrumb"),
                )),
            ),
            subsection(
                "With Chevron",
                Breadcrumb::new((
                    BreadcrumbItem::new("Dashboard").href("/"),
                    BreadcrumbSeparator::chevron(),
                    BreadcrumbItem::new("Settings").href("/settings"),
                    BreadcrumbSeparator::chevron(),
                    BreadcrumbPage::new("Profile"),
                )),
            ),
        ))
        .style(|s| s.gap_8()),
    )
}

fn table_demo() -> impl IntoView {
    use floem::view::ParentView;

    demo_section(
        "Table",
        "A responsive table component for displaying data.",
        Stack::vertical((subsection(
            "Basic Table",
            Table::new()
                .child(
                    TableHeader::new().child(
                        TableRow::new()
                            .child(TableHead::new("Name"))
                            .child(TableHead::new("Email"))
                            .child(TableHead::new("Role"))
                            .child(TableHead::new("Status")),
                    ),
                )
                .child(
                    TableBody::new()
                        .child(
                            TableRow::new()
                                .child(TableCell::new("John Doe"))
                                .child(TableCell::new("john@example.com"))
                                .child(TableCell::new("Admin"))
                                .child(TableCell::new("Active")),
                        )
                        .child(
                            TableRow::new()
                                .child(TableCell::new("Jane Smith"))
                                .child(TableCell::new("jane@example.com"))
                                .child(TableCell::new("Editor"))
                                .child(TableCell::new("Active")),
                        )
                        .child(
                            TableRow::new()
                                .child(TableCell::new("Bob Wilson"))
                                .child(TableCell::new("bob@example.com"))
                                .child(TableCell::new("Viewer"))
                                .child(TableCell::new("Pending")),
                        ),
                )
                .into_view(),
        ),))
        .style(|s| s.gap_8().max_w_2xl()),
    )
}

fn dropdown_demo() -> impl IntoView {
    let dropdown_open = RwSignal::new(false);

    demo_section(
        "Dropdown Menu",
        "A menu that appears when triggered.",
        Stack::vertical((subsection(
            "Basic",
            DropdownMenu::new(dropdown_open)
                .trigger(|| Button::new("Open Menu").outline())
                .content((
                    DropdownMenuLabel::new("My Account"),
                    DropdownMenuSeparator::new(),
                    DropdownMenuItem::new("Profile"),
                    DropdownMenuItem::new("Settings"),
                    DropdownMenuItem::new("Billing"),
                    DropdownMenuSeparator::new(),
                    DropdownMenuItem::new("Log out").destructive(),
                )),
        ),))
        .style(|s| s.gap_8()),
    )
}

fn toast_demo() -> impl IntoView {
    let toasts: RwSignal<Vec<ToastData>> = RwSignal::new(Vec::new());

    demo_section(
        "Toast",
        "A notification popup for feedback messages.",
        Stack::vertical((
            subsection(
                "Trigger Toasts",
                Stack::horizontal((
                    Button::new("Default Toast").on_click_stop(move |_| {
                        push_toast(
                            toasts,
                            ToastData::new("Scheduled", "Your meeting has been scheduled."),
                        );
                    }),
                    Button::new("Success").outline().on_click_stop(move |_| {
                        push_toast(
                            toasts,
                            ToastData::new("Success!", "Your changes have been saved.").success(),
                        );
                    }),
                    Button::new("Error").destructive().on_click_stop(move |_| {
                        push_toast(
                            toasts,
                            ToastData::new("Error", "Something went wrong.").destructive(),
                        );
                    }),
                ))
                .style(|s| s.gap_2()),
            ),
            // Toast container (renders in corner)
            ToastContainer::new(toasts),
        ))
        .style(|s| s.gap_8()),
    )
}

fn toggle_demo() -> impl IntoView {
    let bold = RwSignal::new(false);
    let italic = RwSignal::new(true);
    let underline = RwSignal::new(false);

    demo_section(
        "Toggle",
        "A two-state button that can be on or off.",
        Stack::vertical((
            subsection(
                "Basic",
                Stack::horizontal((
                    Toggle::new(bold, "B"),
                    Toggle::new(italic, "I"),
                    Toggle::new(underline, "U"),
                ))
                .style(|s| s.gap_1()),
            ),
            subsection(
                "Outline Variant",
                Stack::horizontal((
                    Toggle::new(RwSignal::new(true), "Bold").outline(),
                    Toggle::new(RwSignal::new(false), "Italic").outline(),
                ))
                .style(|s| s.gap_1()),
            ),
            subsection(
                "Sizes",
                Stack::horizontal((
                    Toggle::new(RwSignal::new(false), "Sm").sm(),
                    Toggle::new(RwSignal::new(true), "Default"),
                    Toggle::new(RwSignal::new(false), "Lg").lg(),
                ))
                .style(|s| s.gap_2().items_center()),
            ),
        ))
        .style(|s| s.gap_8()),
    )
}

fn toggle_group_demo() -> impl IntoView {
    let alignment = RwSignal::new(Some("left".to_string()));
    let formatting: RwSignal<Vec<String>> = RwSignal::new(vec!["bold".to_string()]);

    demo_section(
        "Toggle Group",
        "A group of toggle buttons with single or multiple selection.",
        Stack::vertical((
            subsection(
                "Single Selection",
                Stack::vertical((
                    ToggleGroup::single(
                        alignment,
                        (
                            ToggleGroupItem::new("left", "Left").selected(alignment),
                            ToggleGroupItem::new("center", "Center").selected(alignment),
                            ToggleGroupItem::new("right", "Right").selected(alignment),
                        ),
                    ),
                    Label::derived(move || format!("Selected: {:?}", alignment.get())).style(|s| {
                        s.font_size(12.0)
                            .with_shadcn_theme(|s, t| s.color(t.muted_foreground))
                    }),
                ))
                .style(|s| s.gap_2()),
            ),
            subsection(
                "Multiple Selection",
                Stack::vertical((
                    ToggleGroupMultiple::new(
                        formatting,
                        (
                            ToggleGroupItemMultiple::new("bold", "B").selected(formatting),
                            ToggleGroupItemMultiple::new("italic", "I").selected(formatting),
                            ToggleGroupItemMultiple::new("underline", "U").selected(formatting),
                        ),
                    ),
                    Label::derived(move || format!("Selected: {:?}", formatting.get())).style(
                        |s| {
                            s.font_size(12.0)
                                .with_shadcn_theme(|s, t| s.color(t.muted_foreground))
                        },
                    ),
                ))
                .style(|s| s.gap_2()),
            ),
        ))
        .style(|s| s.gap_8()),
    )
}

fn hover_card_demo() -> impl IntoView {
    demo_section(
        "Hover Card",
        "A card that appears when hovering over a trigger.",
        Stack::vertical((subsection(
            "Basic",
            HoverCard::new()
                .trigger(|| {
                    floem::views::Label::new("@floem").style(move |s| {
                        s.font_weight(Weight::MEDIUM)
                            .cursor(floem::style::CursorStyle::Pointer)
                            .with_shadcn_theme(|s, t| s.color(t.primary))
                    })
                })
                .content(|| {
                    Stack::vertical((
                        Label::derived(|| "Floem")
                            .style(|s| s.font_size(16.0).font_weight(Weight::SEMIBOLD)),
                        Label::derived(|| "A native Rust UI library with fine-grained reactivity.")
                            .style(|s| {
                                s.font_size(14.0)
                                    .with_shadcn_theme(|s, t| s.color(t.muted_foreground))
                            }),
                        Label::derived(|| "Joined December 2023").style(|s| {
                            s.font_size(12.0)
                                .margin_top(8.0)
                                .with_shadcn_theme(|s, t| s.color(t.muted_foreground))
                        }),
                    ))
                    .style(|s| s.gap_2())
                }),
        ),))
        .style(|s| s.gap_8()),
    )
}

fn context_menu_demo() -> impl IntoView {
    let menu_open = RwSignal::new(false);

    demo_section(
        "Context Menu",
        "A menu triggered by right-click.",
        Stack::vertical((subsection(
            "Right Click Area",
            ContextMenu::new(menu_open)
                .trigger(|| {
                    floem::views::Container::new(Label::derived(|| "Right click here")).style(
                        move |s| {
                            s.padding(40.0)
                                .border(2.0)
                                .items_center()
                                .justify_center()
                                .with_shadcn_theme(|s, t| {
                                    s.border_color(t.border)
                                        .border_radius(t.radius)
                                        .color(t.muted_foreground)
                                })
                        },
                    )
                })
                .content((
                    ContextMenuLabel::new("Edit"),
                    ContextMenuItem::new("Cut").shortcut("⌘X"),
                    ContextMenuItem::new("Copy").shortcut("⌘C"),
                    ContextMenuItem::new("Paste").shortcut("⌘V"),
                    ContextMenuSeparator::new(),
                    ContextMenuItem::new("Delete").destructive(),
                )),
        ),))
        .style(|s| s.gap_8()),
    )
}

fn select_demo() -> impl IntoView {
    let selected = RwSignal::new(Some("apple".to_string()));

    demo_section(
        "Select",
        "A dropdown select component for choosing from options.",
        Stack::vertical((subsection(
            "Basic",
            Stack::vertical((
                Select::new(selected)
                    .placeholder("Select a fruit...")
                    .items(vec![
                        SelectItemData::new("apple", "Apple"),
                        SelectItemData::new("banana", "Banana"),
                        SelectItemData::new("cherry", "Cherry"),
                        SelectItemData::new("date", "Date"),
                    ]),
                Label::derived(move || format!("Selected: {}", selected.get().unwrap_or_default()))
                    .style(|s| {
                        s.font_size(12.0)
                            .margin_top(8.0)
                            .with_shadcn_theme(|s, t| s.color(t.muted_foreground))
                    }),
            ))
            .style(|s| s.gap_2()),
        ),))
        .style(|s| s.gap_8().max_w_md()),
    )
}

fn combobox_demo() -> impl IntoView {
    let selected = RwSignal::new(None::<String>);
    let search = RwSignal::new(String::new());

    let items = vec![
        ("next", "Next.js"),
        ("sveltekit", "SvelteKit"),
        ("nuxt", "Nuxt.js"),
        ("remix", "Remix"),
        ("astro", "Astro"),
    ];

    demo_section(
        "Combobox",
        "A searchable select component with filtering.",
        Stack::vertical((subsection(
            "Basic",
            Stack::vertical((
                Combobox::new(selected, search)
                    .child(
                        ComboboxTrigger::new("Select a framework...")
                            .items(items.clone()),
                    )
                    .child(
                        ComboboxContent::new()
                            .child(ComboboxInput::new())
                            .child(
                                ComboboxList::new()
                                    .child(ComboboxItem::new("next", "Next.js"))
                                    .child(ComboboxItem::new("sveltekit", "SvelteKit"))
                                    .child(ComboboxItem::new("nuxt", "Nuxt.js"))
                                    .child(ComboboxItem::new("remix", "Remix"))
                                    .child(ComboboxItem::new("astro", "Astro")),
                            )
                            .child(ComboboxEmpty::new("No framework found.")),
                    ),
                Label::derived(move || format!("Selected: {:?}", selected.get())).style(|s| {
                    s.font_size(12.0)
                        .margin_top(8.0)
                        .with_shadcn_theme(|s, t| s.color(t.muted_foreground))
                }),
            ))
            .style(|s| s.gap_2()),
        ),))
        .style(|s| s.gap_8().max_w_md()),
    )
}

fn input_otp_demo() -> impl IntoView {
    let otp_value = RwSignal::new(String::new());
    let pin_value = RwSignal::new(String::new());

    demo_section(
        "Input OTP",
        "One-time password input with individual character slots.",
        Stack::vertical((
            subsection(
                "6-Digit OTP",
                Stack::vertical((
                    InputOTP::new(otp_value, 6),
                    Label::derived(move || format!("Value: {}", otp_value.get())).style(|s| {
                        s.font_size(12.0)
                            .with_shadcn_theme(|s, t| s.color(t.muted_foreground))
                    }),
                ))
                .style(|s| s.gap_4()),
            ),
            subsection(
                "4-Digit PIN",
                Stack::vertical((
                    PinInput::new(pin_value, 4),
                    Label::derived(move || format!("PIN: {}", pin_value.get())).style(|s| {
                        s.font_size(12.0)
                            .with_shadcn_theme(|s, t| s.color(t.muted_foreground))
                    }),
                ))
                .style(|s| s.gap_4()),
            ),
        ))
        .style(|s| s.gap_8()),
    )
}

fn date_picker_demo() -> impl IntoView {
    let selected_date = RwSignal::new(None);
    let start_date = RwSignal::new(None);
    let end_date = RwSignal::new(None);

    demo_section(
        "Date Picker",
        "A date selection component with calendar popup.",
        Stack::vertical((
            subsection(
                "Single Date",
                Stack::vertical((
                    DatePicker::new(selected_date),
                    Label::derived(move || {
                        if let Some(date) = selected_date.get() {
                            format!("Selected: {}-{:02}-{:02}", date.year, date.month, date.day)
                        } else {
                            "No date selected".to_string()
                        }
                    })
                    .style(|s| {
                        s.font_size(12.0)
                            .with_shadcn_theme(|s, t| s.color(t.muted_foreground))
                    }),
                ))
                .style(|s| s.gap_4()),
            ),
            subsection(
                "Date Range",
                Stack::vertical((
                    DateRangePicker::new(start_date, end_date),
                    Label::derived(move || {
                        let start = start_date.get();
                        let end = end_date.get();
                        match (start, end) {
                            (Some(s), Some(e)) => format!(
                                "Range: {}-{:02}-{:02} to {}-{:02}-{:02}",
                                s.year, s.month, s.day, e.year, e.month, e.day
                            ),
                            (Some(s), None) => {
                                format!("Start: {}-{:02}-{:02}", s.year, s.month, s.day)
                            }
                            _ => "No range selected".to_string(),
                        }
                    })
                    .style(|s| {
                        s.font_size(12.0)
                            .with_shadcn_theme(|s, t| s.color(t.muted_foreground))
                    }),
                ))
                .style(|s| s.gap_4()),
            ),
        ))
        .style(|s| s.gap_8()),
    )
}

fn label_demo() -> impl IntoView {
    let email = RwSignal::new(String::new());

    demo_section(
        "Label",
        "Accessible form labels and field components.",
        Stack::vertical((
            subsection(
                "Form Label",
                Stack::vertical((
                    FormLabel::new("Email"),
                    Input::new()
                        .placeholder("Enter your email")
                        .value(move || email.get())
                        .on_update(move |s| email.set(s.to_string())),
                ))
                .style(|s| s.gap_2()),
            ),
            subsection("Form Field", {
                let username = RwSignal::new(String::new());
                FormField::new(
                    "Username",
                    Input::new()
                        .placeholder("Enter username")
                        .value(move || username.get())
                        .on_update(move |s| username.set(s.to_string())),
                )
            }),
        ))
        .style(|s| s.gap_8().max_w_md()),
    )
}

fn collapsible_demo() -> impl IntoView {
    let is_open = RwSignal::new(false);

    demo_section(
        "Collapsible",
        "A component that can be expanded or collapsed.",
        Stack::vertical((subsection(
            "Basic",
            Collapsible::new(is_open)
                .trigger(move || {
                    Stack::horizontal((
                        Label::derived(|| "Can I use this in my project?"),
                        Label::derived(move || if is_open.get() { "▼" } else { "▶" })
                            .style(|s| s.font_size(12.0)),
                    ))
                    .style(|s| {
                        s.gap_8()
                            .padding(12.0)
                            .border(1.0)
                            .with_shadcn_theme(|s, t| {
                                s.border_color(t.border).border_radius(t.radius)
                            })
                    })
                })
                .content(|| {
                    Stack::vertical((
                        Label::derived(|| "Yes. Free to use for personal and commercial projects."),
                        Label::derived(|| "No attribution required."),
                    ))
                    .style(|s| {
                        s.gap_2()
                            .padding(12.0)
                            .margin_top(8.0)
                            .font_size(14.0)
                            .with_shadcn_theme(|s, t| {
                                s.background(t.muted)
                                    .border_radius(t.radius)
                                    .color(t.muted_foreground)
                            })
                    })
                }),
        ),))
        .style(|s| s.gap_8().max_w_lg()),
    )
}

fn alert_dialog_demo() -> impl IntoView {
    let dialog_open = RwSignal::new(false);

    demo_section(
        "Alert Dialog",
        "A modal dialog for important confirmations.",
        Stack::vertical((
            subsection(
                "Destructive Action",
                AlertDialog::new(dialog_open)
                    .trigger("Delete Account")
                    .title("Are you absolutely sure?")
                    .description("This action cannot be undone. This will permanently delete your account and remove your data from our servers.")
                    .cancel("Cancel")
                    .action("Delete", move || {
                        // Handle delete action
                    })
                    .destructive(),
            ),
        ))
        .style(|s| s.gap_8()),
    )
}

fn drawer_demo() -> impl IntoView {
    let drawer_open = RwSignal::new(false);
    let drawer_right = RwSignal::new(false);

    demo_section(
        "Drawer",
        "A slide-out panel from the edge of the screen.",
        Stack::vertical((
            subsection(
                "Bottom Drawer",
                Stack::vertical((
                    Button::new("Open Bottom Drawer").on_click_stop(move |_| drawer_open.set(true)),
                    Drawer::new(drawer_open)
                        .side(DrawerSide::Bottom)
                        .content(Stack::vertical((
                            DrawerHeader::new(Stack::vertical((
                                DrawerTitle::new("Edit profile"),
                                DrawerDescription::new("Make changes to your profile here."),
                            ))),
                            DrawerFooter::new(
                                Button::new("Save changes")
                                    .on_click_stop(move |_| drawer_open.set(false)),
                            ),
                        ))),
                ))
                .style(|s| s.gap_4()),
            ),
            subsection(
                "Right Drawer",
                Stack::vertical((
                    Button::new("Open Right Drawer")
                        .outline()
                        .on_click_stop(move |_| drawer_right.set(true)),
                    Drawer::new(drawer_right)
                        .side(DrawerSide::Right)
                        .content(Stack::vertical((
                            DrawerHeader::new(Stack::vertical((
                                DrawerTitle::new("Settings"),
                                DrawerDescription::new("Configure your preferences."),
                            ))),
                            DrawerFooter::new(
                                Button::new("Close")
                                    .outline()
                                    .on_click_stop(move |_| drawer_right.set(false)),
                            ),
                        ))),
                ))
                .style(|s| s.gap_4()),
            ),
        ))
        .style(|s| s.gap_8()),
    )
}

fn aspect_ratio_demo() -> impl IntoView {
    demo_section(
        "Aspect Ratio",
        "A component that maintains a specific aspect ratio.",
        Stack::vertical((
            subsection(
                "16:9 Ratio",
                AspectRatio::video()
                    .child(|| {
                        floem::views::Container::new(Label::derived(|| "16:9")).style(|s| {
                            s.width_full()
                                .height_full()
                                .items_center()
                                .justify_center()
                                .with_shadcn_theme(|s, t| {
                                    s.background(t.muted).border_radius(t.radius)
                                })
                        })
                    })
                    .style(|s| s.max_width(400.0)),
            ),
            subsection(
                "1:1 Square",
                AspectRatio::square()
                    .child(|| {
                        floem::views::Container::new(Label::derived(|| "1:1")).style(|s| {
                            s.width_full()
                                .height_full()
                                .items_center()
                                .justify_center()
                                .with_shadcn_theme(|s, t| {
                                    s.background(t.muted).border_radius(t.radius)
                                })
                        })
                    })
                    .style(|s| s.max_width(200.0)),
            ),
        ))
        .style(|s| s.gap_8()),
    )
}

fn scroll_area_demo() -> impl IntoView {
    demo_section(
        "Scroll Area",
        "A scrollable container for overflow content.",
        Stack::vertical((subsection(
            "Vertical Scroll",
            ScrollArea::new(
                Stack::vertical((
                    Label::derived(|| "Item 1"),
                    Label::derived(|| "Item 2"),
                    Label::derived(|| "Item 3"),
                    Label::derived(|| "Item 4"),
                    Label::derived(|| "Item 5"),
                    Label::derived(|| "Item 6"),
                    Label::derived(|| "Item 7"),
                    Label::derived(|| "Item 8"),
                    Label::derived(|| "Item 9"),
                    Label::derived(|| "Item 10"),
                ))
                .style(|s| s.gap_4().padding(8.0)),
            )
            .style(|s| {
                s.height(150.0)
                    .width(200.0)
                    .border(1.0)
                    .with_shadcn_theme(|s, t| s.border_color(t.border).border_radius(t.radius))
            }),
        ),))
        .style(|s| s.gap_8()),
    )
}

fn resizable_demo() -> impl IntoView {
    demo_section(
        "Resizable",
        "Resizable panels with draggable handles.",
        Stack::vertical((subsection(
            "Horizontal Layout",
            ResizablePanelGroup::horizontal((
                ResizablePanel::new(
                    floem::views::Container::new(Label::derived(|| "Left Panel")).style(|s| {
                        s.width_full()
                            .height_full()
                            .padding(16.0)
                            .items_center()
                            .justify_center()
                            .with_shadcn_theme(|s, t| s.background(t.muted))
                    }),
                )
                .default_size(50.0),
                ResizableHandle::new(),
                ResizablePanel::new(
                    floem::views::Container::new(Label::derived(|| "Right Panel")).style(|s| {
                        s.width_full()
                            .height_full()
                            .padding(16.0)
                            .items_center()
                            .justify_center()
                            .with_shadcn_theme(|s, t| s.background(t.muted))
                    }),
                )
                .default_size(50.0),
            ))
            .style(|s| {
                s.height(200.0)
                    .width(400.0)
                    .border(1.0)
                    .with_shadcn_theme(|s, t| s.border_color(t.border).border_radius(t.radius))
            }),
        ),))
        .style(|s| s.gap_8()),
    )
}

fn menubar_demo() -> impl IntoView {
    demo_section(
        "Menubar",
        "A horizontal menu bar for application commands.",
        Stack::vertical((subsection(
            "Basic",
            Menubar::new((
                MenubarMenu::new("File").content((
                    MenubarItem::new("New File").shortcut("⌘N"),
                    MenubarItem::new("Open").shortcut("⌘O"),
                    MenubarSeparator::new(),
                    MenubarItem::new("Save").shortcut("⌘S"),
                    MenubarItem::new("Save As...").shortcut("⇧⌘S"),
                )),
                MenubarMenu::new("Edit").content((
                    MenubarItem::new("Undo").shortcut("⌘Z"),
                    MenubarItem::new("Redo").shortcut("⇧⌘Z"),
                    MenubarSeparator::new(),
                    MenubarItem::new("Cut").shortcut("⌘X"),
                    MenubarItem::new("Copy").shortcut("⌘C"),
                    MenubarItem::new("Paste").shortcut("⌘V"),
                )),
                MenubarMenu::new("View").content((
                    MenubarItem::new("Zoom In").shortcut("⌘+"),
                    MenubarItem::new("Zoom Out").shortcut("⌘-"),
                    MenubarSeparator::new(),
                    MenubarItem::new("Full Screen").shortcut("⌃⌘F"),
                )),
            )),
        ),))
        .style(|s| s.gap_8()),
    )
}

fn navigation_menu_demo() -> impl IntoView {
    demo_section(
        "Navigation Menu",
        "A collection of links for site navigation.",
        Stack::vertical((subsection(
            "Basic",
            NavigationMenu::new((
                NavigationMenuItem::new("Getting Started").content((
                    NavigationMenuLink::new("Introduction", "/docs"),
                    NavigationMenuLink::new("Installation", "/docs/installation"),
                    NavigationMenuLink::new("Typography", "/docs/typography"),
                )),
                NavigationMenuItem::new("Components").content((
                    NavigationMenuLink::new("Button", "/docs/button"),
                    NavigationMenuLink::new("Card", "/docs/card"),
                    NavigationMenuLink::new("Dialog", "/docs/dialog"),
                )),
            )),
        ),))
        .style(|s| s.gap_8()),
    )
}

fn pagination_demo() -> impl IntoView {
    let current_page = RwSignal::new(1usize);

    demo_section(
        "Pagination",
        "Page navigation controls for paged content.",
        Stack::vertical((subsection(
            "Basic",
            Stack::vertical((
                Pagination::new(current_page, 10),
                Label::derived(move || format!("Current page: {}", current_page.get())).style(
                    |s| {
                        s.font_size(12.0)
                            .margin_top(8.0)
                            .with_shadcn_theme(|s, t| s.color(t.muted_foreground))
                    },
                ),
            ))
            .style(|s| s.gap_4()),
        ),))
        .style(|s| s.gap_8()),
    )
}

fn command_demo() -> impl IntoView {
    let search = RwSignal::new(String::new());

    demo_section(
        "Command",
        "A command palette for quick actions and search.",
        Stack::vertical((subsection(
            "Basic",
            Command::new(search)
                .placeholder("Type a command or search...")
                .content(Stack::vertical((
                    CommandGroup::new("Suggestions").items(Stack::vertical((
                        CommandItem::new("calendar", "Calendar"),
                        CommandItem::new("search", "Search Emoji"),
                        CommandItem::new("calculator", "Calculator"),
                    ))),
                    CommandGroup::new("Settings").items(Stack::vertical((
                        CommandItem::new("profile", "Profile"),
                        CommandItem::new("billing", "Billing"),
                        CommandItem::new("settings", "Settings"),
                    ))),
                )))
                .style(|s| s.max_width(400.0)),
        ),))
        .style(|s| s.gap_8()),
    )
}

fn calendar_demo() -> impl IntoView {
    let selected_date = RwSignal::new(None);

    demo_section(
        "Calendar",
        "A date selection calendar component.",
        Stack::vertical((subsection(
            "Basic",
            Stack::vertical((
                Calendar::new(selected_date),
                Label::derived(move || {
                    if let Some(date) = selected_date.get() {
                        format!("Selected: {}-{:02}-{:02}", date.year, date.month, date.day)
                    } else {
                        "No date selected".to_string()
                    }
                })
                .style(|s| {
                    s.font_size(12.0)
                        .margin_top(8.0)
                        .with_shadcn_theme(|s, t| s.color(t.muted_foreground))
                }),
            ))
            .style(|s| s.gap_4()),
        ),))
        .style(|s| s.gap_8()),
    )
}

fn carousel_demo() -> impl IntoView {
    let current_index = RwSignal::new(0usize);

    demo_section(
        "Carousel",
        "A slideshow component for cycling through content.",
        Stack::vertical((subsection(
            "Basic",
            Stack::vertical((
                Carousel::new(current_index, 3).items(Stack::vertical((
                    floem::views::Container::new(Label::derived(|| "Slide 1")).style(|s| {
                        s.width(300.0)
                            .height(200.0)
                            .items_center()
                            .justify_center()
                            .with_shadcn_theme(|s, t| s.background(t.muted).border_radius(t.radius))
                    }),
                    floem::views::Container::new(Label::derived(|| "Slide 2")).style(|s| {
                        s.width(300.0)
                            .height(200.0)
                            .items_center()
                            .justify_center()
                            .with_shadcn_theme(|s, t| {
                                s.background(t.accent).border_radius(t.radius)
                            })
                    }),
                    floem::views::Container::new(Label::derived(|| "Slide 3")).style(|s| {
                        s.width(300.0)
                            .height(200.0)
                            .items_center()
                            .justify_center()
                            .with_shadcn_theme(|s, t| s.background(t.muted).border_radius(t.radius))
                    }),
                ))),
                Label::derived(move || format!("Slide: {}", current_index.get() + 1)).style(|s| {
                    s.font_size(12.0)
                        .margin_top(8.0)
                        .with_shadcn_theme(|s, t| s.color(t.muted_foreground))
                }),
            ))
            .style(|s| s.gap_4()),
        ),))
        .style(|s| s.gap_8()),
    )
}

// ============================================================================
// Helper Components
// ============================================================================

fn demo_section<V: IntoView + 'static>(
    title: &'static str,
    description: &'static str,
    content: V,
) -> impl IntoView {
    Stack::vertical((
        // Title
        Label::derived(move || title).style(|s| s.font_size(24.0).font_weight(Weight::BOLD).mb_2()),
        // Description
        Label::derived(move || description).style(|s| {
            s.font_size(14.0)
                .mb_6()
                .with_shadcn_theme(|s, t| s.color(t.muted_foreground))
        }),
        // Content
        content,
    ))
    .style(|s| s.w_full())
}

fn subsection<V: IntoView + 'static>(title: &'static str, content: V) -> impl IntoView {
    Stack::vertical((
        Label::derived(move || title)
            .style(|s| s.font_size(14.0).font_weight(Weight::MEDIUM).mb_3()),
        content,
    ))
}

fn form_field<V: IntoView + 'static>(label_text: &'static str, input: V) -> impl IntoView {
    Stack::horizontal((
        Label::derived(move || label_text)
            .style(|s| s.font_size(14.0).font_weight(Weight::MEDIUM).w_24()),
        input,
    ))
    .style(|s| s.gap_4().items_center())
}

/// Helper to create a sidebar menu button with active state and click handler
fn sidebar_button(label: &'static str, active_section: RwSignal<String>) -> impl IntoView {
    let id = label.to_lowercase().replace(' ', "_");
    let id_for_active = id.clone();
    let id_for_click = id.clone();
    SidebarMenuItem::new().child(
        SidebarMenuButton::new(label)
            .is_active(move || active_section.get() == id_for_active)
            .on_click_stop(move |_| active_section.set(id_for_click.clone())),
    )
}
