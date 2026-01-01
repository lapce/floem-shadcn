//! # floem-shadcn
//!
//! A shadcn/ui inspired component library for Floem.
//!
//! This library provides beautifully designed, accessible components
//! that leverage floem-tailwind for styling.
//!
//! ## Usage
//!
//! ```rust
//! use floem_shadcn::prelude::*;
//!
//! // Use components
//! let btn = Button::new("Click me").primary();
//!
//! // Use semantic styling
//! let style = Style::new()
//!     .bg_primary()
//!     .text_primary_foreground();
//! ```

pub mod components;
pub mod styled;
pub mod text;
pub mod theme;

pub mod prelude {
    pub use crate::components::accordion::{Accordion, AccordionItem};
    pub use crate::components::alert::{Alert, AlertVariant};
    pub use crate::components::alert_dialog::{
        AlertDialog, AlertDialogAction, AlertDialogCancel, AlertDialogContent,
        AlertDialogDescription, AlertDialogFooter, AlertDialogHeader, AlertDialogTitle,
        AlertDialogTrigger,
    };
    pub use crate::components::aspect_ratio::{AspectRatio, AspectRatioSimple};
    pub use crate::components::avatar::Avatar;
    pub use crate::components::badge::Badge;
    pub use crate::components::breadcrumb::{
        Breadcrumb, BreadcrumbEllipsis, BreadcrumbItem, BreadcrumbLink, BreadcrumbList,
        BreadcrumbPage, BreadcrumbSeparator,
    };
    pub use crate::components::button::Button;
    pub use crate::components::calendar::{Calendar, CalendarSimple, SimpleDate};
    pub use crate::components::card::{
        Card, CardContent, CardDescription, CardFooter, CardHeader, CardTitle,
    };
    pub use crate::components::carousel::{
        Carousel, CarouselContent, CarouselItem, CarouselNext, CarouselOrientation,
        CarouselPrevious,
    };
    pub use crate::components::checkbox::Checkbox;
    pub use crate::components::collapsible::{Collapsible, CollapsibleContent, CollapsibleTrigger};
    pub use crate::components::combobox::{
        Combobox, ComboboxContent, ComboboxContext, ComboboxEmpty, ComboboxGroup, ComboboxInput,
        ComboboxItem, ComboboxLabel, ComboboxList, ComboboxSeparator, ComboboxTrigger,
    };
    pub use crate::components::command::{
        Command, CommandEmpty, CommandGroup, CommandInput, CommandItem, CommandItemCustom,
        CommandList, CommandSeparator, CommandShortcut,
    };
    pub use crate::components::context_menu::{
        ContextMenu, ContextMenuContent, ContextMenuGroup, ContextMenuItem, ContextMenuLabel,
        ContextMenuSeparator,
    };
    pub use crate::components::date_picker::{DatePicker, DateRangePicker};
    pub use crate::components::dialog::{
        Dialog, DialogClose, DialogContent, DialogContext, DialogDescription, DialogFooter,
        DialogHeader, DialogTitle, DialogTrigger,
    };
    pub use crate::components::drawer::{
        Drawer, DrawerClose, DrawerContent, DrawerDescription, DrawerFooter, DrawerHeader,
        DrawerSide, DrawerTitle, DrawerTrigger,
    };
    pub use crate::components::dropdown_menu::{
        DropdownMenu, DropdownMenuContent, DropdownMenuGroup, DropdownMenuItem,
        DropdownMenuItemCustom, DropdownMenuLabel, DropdownMenuSeparator, DropdownMenuShortcut,
    };
    pub use crate::components::hover_card::{
        HoverCard, HoverCardAlign, HoverCardContent, HoverCardSide, HoverCardTrigger,
    };
    pub use crate::components::input::Input;
    pub use crate::components::input_otp::{
        InputOTP, InputOTPGroup, InputOTPSeparator, InputOTPSlot, PinInput,
    };
    pub use crate::components::label::{FormField, FormLabel, LabelWithIcon};
    pub use crate::components::menubar::{
        Menubar, MenubarCheckboxItem, MenubarContent, MenubarItem, MenubarMenu, MenubarSeparator,
        MenubarShortcut, MenubarTrigger,
    };
    pub use crate::components::navigation_menu::{
        NavigationMenu, NavigationMenuContent, NavigationMenuIndicator, NavigationMenuItem,
        NavigationMenuLink, NavigationMenuList, NavigationMenuTrigger, NavigationMenuViewport,
    };
    pub use crate::components::pagination::{
        Pagination, PaginationContent, PaginationEllipsis, PaginationItem, PaginationLink,
        PaginationNext, PaginationPrevious,
    };
    pub use crate::components::popover::{
        Popover, PopoverAlign, PopoverContent, PopoverSide, PopoverTrigger,
    };
    pub use crate::components::progress::Progress;
    pub use crate::components::radio_group::{RadioGroup, RadioGroupItem};
    pub use crate::components::resizable::{
        ResizableDirection, ResizableHandle, ResizablePanel, ResizablePanelGroup,
        resizable_horizontal, resizable_vertical,
    };
    pub use crate::components::scroll_area::{
        ScrollArea, ScrollAreaWithBar, ScrollOrientation, VirtualScrollArea,
    };
    pub use crate::components::select::{
        Select, SelectContent, SelectGroup, SelectItem, SelectItemData, SelectLabel,
        SelectSeparator, SelectTrigger,
    };
    pub use crate::components::separator::{Separator, SeparatorOrientation};
    pub use crate::components::sheet::{
        Sheet, SheetClose, SheetContent, SheetDescription, SheetFooter, SheetHeader, SheetSide,
        SheetTitle,
    };
    pub use crate::components::sidebar::{
        Sidebar, SidebarContent, SidebarFooter, SidebarGroup, SidebarGroupAction,
        SidebarGroupContent, SidebarGroupLabel, SidebarHeader, SidebarMenu, SidebarMenuButton,
        SidebarMenuItem, SidebarSeparator,
    };
    pub use crate::components::skeleton::Skeleton;
    pub use crate::components::slider::Slider;
    pub use crate::components::switch::Switch;
    pub use crate::components::table::{
        Table, TableBody, TableCaption, TableCell, TableCellCustom, TableFooter, TableHead,
        TableHeadCustom, TableHeader, TableRow,
    };
    pub use crate::components::tabs::{Tab, Tabs, TabsContent, TabsList};
    pub use crate::components::textarea::Textarea;
    pub use crate::components::toast::{
        Toast, ToastAction, ToastContainer, ToastData, ToastVariant, clear_toasts, push_toast,
        remove_toast,
    };
    pub use crate::components::toggle::{Toggle, ToggleCustom, ToggleSize, ToggleVariant};
    pub use crate::components::toggle_group::{
        ToggleGroup, ToggleGroupItem, ToggleGroupItemMultiple, ToggleGroupMultiple,
        ToggleGroupSize, ToggleGroupVariant,
    };
    pub use crate::components::tooltip::TooltipExt;
    pub use crate::styled::ShadcnStyleExt;
    pub use crate::theme::{ShadcnTheme, ShadcnThemeExt, ShadcnThemeProp, ThemeMode};

    // Re-export ParentView for .child() API
    pub use floem::view::ParentView;
}
