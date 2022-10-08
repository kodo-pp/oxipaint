use gtk::prelude::*;
use gtk::{
    Application, ApplicationWindow, Box as GtkBox, DrawingArea, HeaderBar, Label, MenuBar,
    MenuItem, Orientation, Paned, Widget,
};

const MENU_BAR_PADDING: i32 = 0;
const HEADER_BAR_PADDING: i32 = 0;
const NO_EXTRA_PADDING: u32 = 0;
const BAR_EXPAND: bool = false;
const BAR_FILL: bool = false;

fn build_tool_window(name: &str) -> GtkBox {
    let vbox = GtkBox::new(Orientation::Vertical, HEADER_BAR_PADDING);

    const HEADER_BAR_EXPAND: bool = false;
    const HEADER_BAR_FILL: bool = false;
    const CONTENT_EXPAND: bool = true;
    const CONTENT_FILL: bool = true;
    const CONTENT_MARGIN: i32 = 5;

    let header_bar = HeaderBar::builder().title(name).build();
    let content = Label::new(Some(&format!("Item {:?}", name)));
    content.set_margin(CONTENT_MARGIN);

    vbox.pack_start(
        &header_bar,
        HEADER_BAR_EXPAND,
        HEADER_BAR_FILL,
        NO_EXTRA_PADDING,
    );
    vbox.pack_start(&content, CONTENT_EXPAND, CONTENT_FILL, NO_EXTRA_PADDING);
    vbox
}

fn build_toolbar() -> GtkBox {
    const TOOLBAR_ITEM_PADDING: i32 = 5;
    let bar = GtkBox::new(Orientation::Horizontal, TOOLBAR_ITEM_PADDING);
    let label = Label::new(Some("Toolbar (TODO)"));
    bar.add(&label);
    bar
}

fn build_status_bar() -> GtkBox {
    const STATUS_BAR_ITEM_PADDING: i32 = 5;
    let bar = GtkBox::new(Orientation::Horizontal, STATUS_BAR_ITEM_PADDING);
    let label = Label::new(Some("Status bar (TODO)"));
    bar.add(&label);
    bar
}

fn build_menu_bar() -> MenuBar {
    let menu_bar = MenuBar::new();
    menu_bar.add(&MenuItem::with_label("File"));
    menu_bar.add(&MenuItem::with_label("Edit"));
    menu_bar.add(&MenuItem::with_label("Help"));
    menu_bar
}

fn build_toolspace(top: &impl IsA<Widget>, bottom: &impl IsA<Widget>) -> Paned {
    let toolspace = Paned::new(Orientation::Vertical);

    const RESIZE: bool = true;
    const SHRINK: bool = false;
    toolspace.pack1(top, RESIZE, SHRINK);
    toolspace.pack2(bottom, RESIZE, SHRINK);
    toolspace
}

fn build_workspace() -> Paned {
    let paned_left_rest = Paned::new(Orientation::Horizontal);
    let paned_mid_right = Paned::new(Orientation::Horizontal);

    let drawing_tool_window = build_tool_window("Tools");
    let color_selection_window = build_tool_window("Color");
    let layers_window = build_tool_window("Layers");
    let history_window = build_tool_window("History");

    let left_toolspace = build_toolspace(&drawing_tool_window, &color_selection_window);
    let right_toolspace = build_toolspace(&layers_window, &history_window);
    let content = DrawingArea::new();

    const TOOLSPACE_RESIZE: bool = false;
    const TOOLSPACE_SHRINK: bool = false;
    const CONTENT_RESIZE: bool = true;
    const CONTENT_SHRINK: bool = false;

    const TOOL_WINDOW_RESIZE: bool = true;
    const TOOL_WINDOW_SHRINK: bool = false;

    left_toolspace.pack1(
        &build_tool_window("Top Left"),
        TOOL_WINDOW_RESIZE,
        TOOL_WINDOW_SHRINK,
    );
    left_toolspace.pack2(
        &build_tool_window("Bottom Left"),
        TOOL_WINDOW_RESIZE,
        TOOL_WINDOW_SHRINK,
    );
    right_toolspace.pack1(
        &build_tool_window("Top Right"),
        TOOL_WINDOW_RESIZE,
        TOOL_WINDOW_SHRINK,
    );
    right_toolspace.pack2(
        &build_tool_window("Bottom Right"),
        TOOL_WINDOW_RESIZE,
        TOOL_WINDOW_SHRINK,
    );

    paned_mid_right.pack1(&content, CONTENT_RESIZE, CONTENT_SHRINK);
    paned_mid_right.pack2(&right_toolspace, TOOLSPACE_RESIZE, TOOLSPACE_SHRINK);
    paned_left_rest.pack1(&left_toolspace, TOOLSPACE_RESIZE, TOOLSPACE_SHRINK);
    paned_left_rest.pack2(&paned_mid_right, CONTENT_RESIZE, CONTENT_SHRINK);

    paned_left_rest
}

pub fn build_ui(app: &Application) {
    let main_window = ApplicationWindow::builder()
        .application(app)
        .default_width(800)
        .default_height(600)
        .title("Oxipaint")
        .build();

    let main_vbox = GtkBox::new(Orientation::Vertical, MENU_BAR_PADDING);

    let menu_bar = build_menu_bar();
    main_vbox.pack_start(&menu_bar, BAR_EXPAND, BAR_FILL, NO_EXTRA_PADDING);

    let toolbar = build_toolbar();
    main_vbox.pack_start(&toolbar, BAR_EXPAND, BAR_FILL, NO_EXTRA_PADDING);

    let workspace = build_workspace();

    const TOOLBAR_PADDING: u32 = 0;
    const CONTENT_EXPAND: bool = true;
    const CONTENT_FILL: bool = true;
    main_vbox.pack_start(&workspace, CONTENT_EXPAND, CONTENT_FILL, TOOLBAR_PADDING);

    const STATUS_BAR_PADDING: u32 = 0;
    let status_bar = build_status_bar();
    main_vbox.pack_start(&status_bar, BAR_EXPAND, BAR_FILL, STATUS_BAR_PADDING);

    main_window.add(&main_vbox);

    main_window.show_all();
}
