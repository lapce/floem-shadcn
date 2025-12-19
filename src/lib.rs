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

pub mod theme;
pub mod styled;
pub mod components;
pub mod text;

pub mod prelude {
    pub use crate::theme::{ShadcnTheme, ThemeMode, ShadcnThemeProp, ShadcnThemeExt};
    pub use crate::styled::ShadcnStyleExt;
    pub use crate::components::button::Button;
    pub use crate::components::card::{Card, CardHeader, CardContent, CardFooter, CardTitle, CardDescription};
    pub use crate::components::input::Input;
    pub use crate::components::badge::Badge;
    pub use crate::components::sidebar::{
        Sidebar, SidebarHeader, SidebarContent, SidebarFooter,
        SidebarGroup, SidebarGroupLabel, SidebarGroupContent, SidebarGroupAction,
        SidebarMenu, SidebarMenuItem, SidebarMenuButton, SidebarSeparator,
    };
    pub use crate::components::checkbox::Checkbox;
    pub use crate::components::switch::Switch;
    pub use crate::components::select::{Select, SimpleSelect, SelectItem};
    pub use crate::components::tabs::{Tabs, TabsList, Tab, TabsContent};
    pub use crate::components::dialog::{Dialog, DialogContent, DialogHeader, DialogFooter, DialogTitle, DialogDescription};
    pub use crate::components::alert::{Alert, AlertVariant};
    pub use crate::components::tooltip::TooltipExt;
    pub use crate::components::separator::{Separator, SeparatorOrientation};
    pub use crate::components::progress::Progress;
    pub use crate::components::avatar::Avatar;
    pub use crate::components::textarea::Textarea;
    pub use crate::components::skeleton::Skeleton;
    pub use crate::components::accordion::{Accordion, AccordionItem};
    pub use crate::components::slider::Slider;
    pub use crate::components::radio_group::{RadioGroup, RadioGroupItem};
    pub use crate::components::popover::{Popover, PopoverTrigger, PopoverContent, PopoverSide, PopoverAlign};
    pub use crate::components::sheet::{Sheet, SheetContent, SheetHeader, SheetTitle, SheetDescription, SheetFooter, SheetClose, SheetSide};
    pub use crate::components::breadcrumb::{Breadcrumb, BreadcrumbList, BreadcrumbItem, BreadcrumbLink, BreadcrumbPage, BreadcrumbSeparator, BreadcrumbEllipsis};
    pub use crate::components::table::{Table, TableHeader, TableBody, TableFooter, TableRow, TableHead, TableHeadCustom, TableCell, TableCellCustom, TableCaption};
    pub use crate::components::dropdown_menu::{DropdownMenu, DropdownMenuContent, DropdownMenuItem, DropdownMenuItemCustom, DropdownMenuSeparator, DropdownMenuLabel, DropdownMenuGroup, DropdownMenuShortcut};
    pub use crate::components::toast::{Toast, ToastContainer, ToastData, ToastVariant, ToastAction, push_toast, remove_toast, clear_toasts};
    pub use crate::components::toggle::{Toggle, ToggleCustom, ToggleVariant, ToggleSize};
    pub use crate::components::toggle_group::{ToggleGroup, ToggleGroupMultiple, ToggleGroupItem, ToggleGroupItemMultiple, ToggleGroupVariant, ToggleGroupSize};
    pub use crate::components::hover_card::{HoverCard, HoverCardContent, HoverCardTrigger, HoverCardSide, HoverCardAlign};
    pub use crate::components::context_menu::{ContextMenu, ContextMenuContent, ContextMenuItem, ContextMenuSeparator, ContextMenuLabel, ContextMenuGroup};
    pub use crate::components::collapsible::{Collapsible, CollapsibleTrigger, CollapsibleContent};
    pub use crate::components::label::{FormLabel, LabelWithIcon, FormField};
    pub use crate::components::aspect_ratio::{AspectRatio, AspectRatioSimple};
    pub use crate::components::scroll_area::{ScrollArea, ScrollAreaWithBar, ScrollOrientation, VirtualScrollArea};
    pub use crate::components::command::{Command, CommandInput, CommandList, CommandEmpty, CommandGroup, CommandItem, CommandItemCustom, CommandSeparator, CommandShortcut};
    pub use crate::components::calendar::{Calendar, CalendarSimple, SimpleDate};
    pub use crate::components::carousel::{Carousel, CarouselItem, CarouselPrevious, CarouselNext, CarouselOrientation, CarouselContent};
    pub use crate::components::navigation_menu::{NavigationMenu, NavigationMenuList, NavigationMenuItem, NavigationMenuTrigger, NavigationMenuContent, NavigationMenuLink, NavigationMenuIndicator, NavigationMenuViewport};
    pub use crate::components::resizable::{ResizablePanelGroup, ResizablePanel, ResizableHandle, ResizableDirection, resizable_horizontal, resizable_vertical};
    pub use crate::components::menubar::{Menubar, MenubarMenu, MenubarTrigger, MenubarContent, MenubarItem, MenubarSeparator, MenubarCheckboxItem, MenubarShortcut};
    pub use crate::components::input_otp::{InputOTP, InputOTPGroup, InputOTPSlot, InputOTPSeparator, PinInput};
    pub use crate::components::date_picker::{DatePicker, DateRangePicker};
    pub use crate::components::alert_dialog::{AlertDialog, AlertDialogTrigger, AlertDialogContent, AlertDialogHeader, AlertDialogFooter, AlertDialogTitle, AlertDialogDescription, AlertDialogAction, AlertDialogCancel};
    pub use crate::components::drawer::{Drawer, DrawerTrigger, DrawerContent, DrawerHeader, DrawerTitle, DrawerDescription, DrawerFooter, DrawerClose, DrawerSide};
    pub use crate::components::pagination::{Pagination, PaginationContent, PaginationItem, PaginationLink, PaginationPrevious, PaginationNext, PaginationEllipsis};
    pub use crate::components::combobox::{Combobox, ComboboxTrigger, ComboboxContent, ComboboxInput, ComboboxEmpty, ComboboxGroup, ComboboxItem, ComboboxItemData, ComboboxSeparator};
}
