//! Showcase example for floem-shadcn components
//!
//! Run with: cargo run --example showcase

use floem::IntoView;
use floem::reactive::{RwSignal, SignalGet, SignalUpdate};
use floem::text::Weight;
use floem::views::{Decorators, Label, h_stack, v_stack};
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

    h_stack((
        // Sidebar navigation using full Sidebar APIs
        Sidebar::new()
            .header(SidebarHeader::new(
                v_stack((
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
            ))
            .content(SidebarContent::new(v_stack((
                // Group 1: Form Inputs
                SidebarGroup::new(v_stack((
                    SidebarGroupLabel::new("Form Inputs"),
                    SidebarGroupContent::new(SidebarMenu::new(v_stack((
                        sidebar_button("Buttons", active_section),
                        sidebar_button("Badges", active_section),
                        sidebar_button("Cards", active_section),
                        sidebar_button("Inputs", active_section),
                        sidebar_button("Textarea", active_section),
                        sidebar_button("Checkbox", active_section),
                        sidebar_button("Switch", active_section),
                        sidebar_button("Radio Group", active_section),
                        sidebar_button("Slider", active_section),
                        sidebar_button("Select", active_section),
                        sidebar_button("Combobox", active_section),
                        sidebar_button("Input OTP", active_section),
                        sidebar_button("Date Picker", active_section),
                        sidebar_button("Label", active_section),
                    )))),
                ))),
                SidebarSeparator::new(),
                // Group 2: Layout & Feedback
                SidebarGroup::new(v_stack((
                    SidebarGroupLabel::new("Layout & Feedback"),
                    SidebarGroupContent::new(SidebarMenu::new(v_stack((
                        sidebar_button("Tabs", active_section),
                        sidebar_button("Accordion", active_section),
                        sidebar_button("Collapsible", active_section),
                        sidebar_button("Dialog", active_section),
                        sidebar_button("Alert Dialog", active_section),
                        sidebar_button("Drawer", active_section),
                        sidebar_button("Alert", active_section),
                        sidebar_button("Avatar", active_section),
                        sidebar_button("Progress", active_section),
                        sidebar_button("Separator", active_section),
                        sidebar_button("Skeleton", active_section),
                        sidebar_button("Tooltip", active_section),
                        sidebar_button("Aspect Ratio", active_section),
                        sidebar_button("Scroll Area", active_section),
                        sidebar_button("Resizable", active_section),
                    )))),
                ))),
                SidebarSeparator::new(),
                // Group 3: Overlays & Navigation
                SidebarGroup::new(v_stack((
                    SidebarGroupLabel::new("Overlays & Navigation"),
                    SidebarGroupContent::new(SidebarMenu::new(v_stack((
                        sidebar_button("Popover", active_section),
                        sidebar_button("Sheet", active_section),
                        sidebar_button("Dropdown Menu", active_section),
                        sidebar_button("Menubar", active_section),
                        sidebar_button("Navigation Menu", active_section),
                        sidebar_button("Breadcrumb", active_section),
                        sidebar_button("Pagination", active_section),
                        sidebar_button("Command", active_section),
                    )))),
                ))),
                SidebarSeparator::new(),
                // Group 4: Data & Misc
                SidebarGroup::new(v_stack((
                    SidebarGroupLabel::new("Data & Misc"),
                    SidebarGroupContent::new(SidebarMenu::new(v_stack((
                        sidebar_button("Table", active_section),
                        sidebar_button("Calendar", active_section),
                        sidebar_button("Carousel", active_section),
                        sidebar_button("Toast", active_section),
                        sidebar_button("Toggle", active_section),
                        sidebar_button("Toggle Group", active_section),
                        sidebar_button("Hover Card", active_section),
                        sidebar_button("Context Menu", active_section),
                        sidebar_button("Sidebar", active_section),
                    )))),
                ))),
            ))))
            .footer(SidebarFooter::new(Label::derived(|| "v0.1.0").style(|s| {
                s.font_size(12.0)
                    .with_shadcn_theme(|s, t| s.color(t.muted_foreground))
            }))),
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
        v_stack((
            // Button variants
            subsection(
                "Variants",
                h_stack((
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
                h_stack((
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
        v_stack((subsection(
            "Variants",
            h_stack((
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
        v_stack((Card::new((
            CardHeader::new()
                .title("Create project")
                .description("Deploy your new project in one-click."),
            CardContent::new(
                v_stack((
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
                h_stack((Button::new("Cancel").outline(), Button::new("Deploy")))
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
        v_stack((subsection(
            "Basic",
            v_stack((
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
        v_stack((
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
        v_stack((
            subsection(
                "Basic",
                v_stack((
                    Checkbox::new(checked1).label("Accept terms and conditions"),
                    Checkbox::new(checked2).label("Send me updates"),
                    Checkbox::new(checked3).label("Enable notifications"),
                ))
                .style(|s| s.gap_3()),
            ),
            subsection(
                "Disabled",
                v_stack((Checkbox::new(RwSignal::new(false))
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
        v_stack((
            subsection(
                "Basic",
                v_stack((
                    Switch::new(enabled1).label("Airplane mode"),
                    Switch::new(enabled2).label("Dark mode"),
                ))
                .style(|s| s.gap_3()),
            ),
            subsection(
                "Disabled",
                v_stack((Switch::new(RwSignal::new(false))
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
        v_stack((Tabs::new(
            active_tab,
            v_stack((
                TabsList::new(h_stack((
                    Tab::new("account", "Account").active(active_tab),
                    Tab::new("password", "Password").active(active_tab),
                    Tab::new("settings", "Settings").active(active_tab),
                ))),
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
                        .description("Change your password here."),))
                    .style(|s| s.w_full()),
                )
                .active(active_tab),
                TabsContent::new(
                    "settings",
                    Card::new((CardHeader::new()
                        .title("Settings")
                        .description("Configure your preferences."),))
                    .style(|s| s.w_full()),
                )
                .active(active_tab),
            )),
        )
        .style(|s| s.max_w_md()),))
        .style(|s| s.gap_8()),
    )
}

fn dialog_demo() -> impl IntoView {
    let dialog_open = RwSignal::new(false);

    demo_section(
        "Dialog",
        "A modal dialog component for important interactions.",
        v_stack((
            subsection(
                "Basic Dialog",
                v_stack((
                    Button::new("Open Dialog").on_click_stop(move |_| dialog_open.set(true)),
                    Dialog::new(dialog_open, move || {
                        DialogContent::new(v_stack((
                            DialogHeader::new()
                                .title("Are you sure?")
                                .description("This action cannot be undone. This will permanently delete your account."),
                            DialogFooter::new(h_stack((
                                Button::new("Cancel").outline().on_click_stop(move |_| dialog_open.set(false)),
                                Button::new("Continue").destructive().on_click_stop(move |_| dialog_open.set(false)),
                            )).style(|s| s.gap_2())),
                        )))
                    }),
                ))
                .style(|s| s.gap_4()),
            ),
        ))
        .style(|s| s.gap_8()),
    )
}

fn alert_demo() -> impl IntoView {
    demo_section(
        "Alert",
        "An alert component for displaying feedback messages.",
        v_stack((
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
        v_stack((
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
        v_stack((
            subsection(
                "Sizes",
                h_stack((
                    Avatar::new().fallback("SM").size(32.0),
                    Avatar::new().fallback("MD").size(40.0),
                    Avatar::new().fallback("LG").size(48.0),
                    Avatar::new().fallback("XL").size(64.0),
                ))
                .style(|s| s.gap_4().items_center()),
            ),
            subsection(
                "Initials",
                h_stack((
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
        v_stack((subsection(
            "Basic",
            v_stack((
                Progress::new(progress_value),
                h_stack((
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
        v_stack((
            subsection(
                "Horizontal",
                v_stack((
                    Label::derived(|| "Content above"),
                    Separator::new(),
                    Label::derived(|| "Content below"),
                ))
                .style(|s| s.gap_4()),
            ),
            subsection(
                "Vertical",
                h_stack((
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
        v_stack((
            subsection(
                "Basic Shapes",
                v_stack((
                    Skeleton::text(),
                    Skeleton::new().width(200.0).height(20.0),
                    Skeleton::new().width(150.0).height(20.0),
                ))
                .style(|s| s.gap_2()),
            ),
            subsection(
                "Card Skeleton",
                h_stack((
                    Skeleton::new().circle(48.0),
                    v_stack((
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
        v_stack((subsection(
            "Basic",
            h_stack((
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
        v_stack((subsection(
            "Basic",
            v_stack((
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
        v_stack((subsection(
            "Basic",
            v_stack((
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
        v_stack((
            subsection(
                "Basic",
                v_stack((
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
        v_stack((subsection(
            "Basic",
            Popover::new(popover_open)
                .trigger(|| Button::new("Open Popover"))
                .content(|| {
                    v_stack((
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
        v_stack((
            subsection(
                "Right Sheet",
                v_stack((
                    Button::new("Open Sheet").on_click_stop(move |_| sheet_open.set(true)),
                    Sheet::new(sheet_open, SheetContent::new(
                        v_stack((
                            SheetHeader::new(v_stack((
                                SheetTitle::new("Edit Profile"),
                                SheetDescription::new("Make changes to your profile here. Click save when you're done."),
                            ))),
                            // Some content
                            v_stack((
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
                v_stack((
                    Button::new("Open Left Sheet").outline().on_click_stop(move |_| sheet_left_open.set(true)),
                    Sheet::new(sheet_left_open, SheetContent::new(
                        v_stack((
                            SheetHeader::new(v_stack((
                                SheetTitle::new("Navigation"),
                                SheetDescription::new("Browse menu options."),
                            ))),
                            v_stack((
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
        v_stack((
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
    demo_section(
        "Table",
        "A responsive table component for displaying data.",
        v_stack((subsection(
            "Basic Table",
            Table::new((
                TableHeader::new(TableRow::new((
                    TableHead::new("Name"),
                    TableHead::new("Email"),
                    TableHead::new("Role"),
                    TableHead::new("Status"),
                ))),
                TableBody::new((
                    TableRow::new((
                        TableCell::new("John Doe"),
                        TableCell::new("john@example.com"),
                        TableCell::new("Admin"),
                        TableCell::new("Active"),
                    )),
                    TableRow::new((
                        TableCell::new("Jane Smith"),
                        TableCell::new("jane@example.com"),
                        TableCell::new("Editor"),
                        TableCell::new("Active"),
                    )),
                    TableRow::new((
                        TableCell::new("Bob Wilson"),
                        TableCell::new("bob@example.com"),
                        TableCell::new("Viewer"),
                        TableCell::new("Pending"),
                    )),
                )),
            )),
        ),))
        .style(|s| s.gap_8().max_w_2xl()),
    )
}

fn dropdown_demo() -> impl IntoView {
    let dropdown_open = RwSignal::new(false);

    demo_section(
        "Dropdown Menu",
        "A menu that appears when triggered.",
        v_stack((subsection(
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
        v_stack((
            subsection(
                "Trigger Toasts",
                h_stack((
                    Button::new("Default Toast").on_click_stop({
                        let toasts = toasts;
                        move |_| {
                            push_toast(
                                toasts,
                                ToastData::new("Scheduled", "Your meeting has been scheduled."),
                            );
                        }
                    }),
                    Button::new("Success").outline().on_click_stop({
                        let toasts = toasts;
                        move |_| {
                            push_toast(
                                toasts,
                                ToastData::new("Success!", "Your changes have been saved.")
                                    .success(),
                            );
                        }
                    }),
                    Button::new("Error").destructive().on_click_stop({
                        let toasts = toasts;
                        move |_| {
                            push_toast(
                                toasts,
                                ToastData::new("Error", "Something went wrong.").destructive(),
                            );
                        }
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
        v_stack((
            subsection(
                "Basic",
                h_stack((
                    Toggle::new(bold, "B"),
                    Toggle::new(italic, "I"),
                    Toggle::new(underline, "U"),
                ))
                .style(|s| s.gap_1()),
            ),
            subsection(
                "Outline Variant",
                h_stack((
                    Toggle::new(RwSignal::new(true), "Bold").outline(),
                    Toggle::new(RwSignal::new(false), "Italic").outline(),
                ))
                .style(|s| s.gap_1()),
            ),
            subsection(
                "Sizes",
                h_stack((
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
        v_stack((
            subsection(
                "Single Selection",
                v_stack((
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
                v_stack((
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
        v_stack((subsection(
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
                    v_stack((
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
        v_stack((subsection(
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
        v_stack((subsection(
            "Basic",
            v_stack((
                Select::new(selected)
                    .placeholder("Select a fruit...")
                    .items(vec![
                        SelectItemData::new("apple", "Apple"),
                        SelectItemData::new("banana", "Banana"),
                        SelectItemData::new("cherry", "Cherry"),
                        SelectItemData::new("date", "Date"),
                    ]),
                Label::derived(move || {
                    format!("Selected: {}", selected.get().unwrap_or_default())
                })
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

    demo_section(
        "Combobox",
        "A searchable select component with filtering.",
        v_stack((subsection(
            "Basic",
            v_stack((
                Combobox::new(selected, search)
                    .placeholder("Select a framework...")
                    .items(vec![
                        ComboboxItemData::new("next", "Next.js"),
                        ComboboxItemData::new("sveltekit", "SvelteKit"),
                        ComboboxItemData::new("nuxt", "Nuxt.js"),
                        ComboboxItemData::new("remix", "Remix"),
                        ComboboxItemData::new("astro", "Astro"),
                    ]),
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
        v_stack((
            subsection(
                "6-Digit OTP",
                v_stack((
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
                v_stack((
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
        v_stack((
            subsection(
                "Single Date",
                v_stack((
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
                v_stack((
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
        v_stack((
            subsection(
                "Form Label",
                v_stack((
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
        v_stack((subsection(
            "Basic",
            Collapsible::new(is_open)
                .trigger(move || {
                    h_stack((
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
                    v_stack((
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
        v_stack((
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
        v_stack((
            subsection(
                "Bottom Drawer",
                v_stack((
                    Button::new("Open Bottom Drawer").on_click_stop(move |_| drawer_open.set(true)),
                    Drawer::new(drawer_open)
                        .side(DrawerSide::Bottom)
                        .content(v_stack((
                            DrawerHeader::new(v_stack((
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
                v_stack((
                    Button::new("Open Right Drawer")
                        .outline()
                        .on_click_stop(move |_| drawer_right.set(true)),
                    Drawer::new(drawer_right)
                        .side(DrawerSide::Right)
                        .content(v_stack((
                            DrawerHeader::new(v_stack((
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
        v_stack((
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
        v_stack((subsection(
            "Vertical Scroll",
            ScrollArea::new(
                v_stack((
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
        v_stack((subsection(
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
        v_stack((subsection(
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
        v_stack((subsection(
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
        v_stack((subsection(
            "Basic",
            v_stack((
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
        v_stack((subsection(
            "Basic",
            Command::new(search)
                .placeholder("Type a command or search...")
                .content(v_stack((
                    CommandGroup::new("Suggestions").items(v_stack((
                        CommandItem::new("calendar", "Calendar"),
                        CommandItem::new("search", "Search Emoji"),
                        CommandItem::new("calculator", "Calculator"),
                    ))),
                    CommandGroup::new("Settings").items(v_stack((
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
        v_stack((subsection(
            "Basic",
            v_stack((
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
        v_stack((subsection(
            "Basic",
            v_stack((
                Carousel::new(current_index, 3).items(v_stack((
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
    v_stack((
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
    v_stack((
        Label::derived(move || title)
            .style(|s| s.font_size(14.0).font_weight(Weight::MEDIUM).mb_3()),
        content,
    ))
}

fn form_field<V: IntoView + 'static>(label_text: &'static str, input: V) -> impl IntoView {
    h_stack((
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
    SidebarMenuItem::new(
        SidebarMenuButton::new(label)
            .is_active(move || active_section.get() == id_for_active)
            .on_click_stop(move |_| active_section.set(id_for_click.clone())),
    )
}
