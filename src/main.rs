use gtk::*;
use gtk_layer_shell_rs::*;
use gdk::*;

const GTK_STYLE_PROVIDER_PRIORITY_USER: i32 = 800;


fn main() {
    const COL_PADDING: u32 = 17;

    let config_wal = 0;
    let config_columns = 4;
    let config_prompt = "run:";

    open_config();
    let desktop_entries = find_desktop_entries();

    gtk::init().expect("Could not init GTK!");

    let window = gtk::Window::new(gtk::WindowType::Toplevel);
    init_for_window(&window);
    set_layer(&window, Layer::Top);
    set_anchor(&window, Edge::Top, true);
    set_anchor(&window, Edge::Left, true);
    set_anchor(&window, Edge::Right, true);
    set_anchor(&window, Edge::Bottom, true);

    window.set_resizable(false);
    window.set_decorated(false);

    let layout = Box::new(Orientation::Vertical, 0);
    window.add(&layout);

    let search_box = Box::new(Orientation::Horizontal, 0);
    layout.pack_start(&search_box, false, false, 0);

    let grid_width = 270 * config_columns + (config_columns + 2) * COL_PADDING;
    let spacer = Box::new(Orientation::Horizontal, 0);
    let width = get_monitor_width() - (grid_width as i32) / 2;
    spacer.set_size_request(width, 1);

    search_box.pack_start(&spacer, false, false, 0);

    let search_label = Label::new(None);
    search_label.set_markup(config_prompt);
    search_box.pack_start(&search_label, false, false, 0);

    add_class(&search_label, "textview-label");

    let search_input = TextView::new();
//    search_input.set_name("search");
    search_box.pack_start(&search_input, true, true, 0);

    let app_grid = Grid::new();
    layout.pack_start(&app_grid, true, true, 0);

    for i in 0..config_columns {
        app_grid.insert_column(i as i32);
    }

    app_grid.set_column_spacing(COL_PADDING);
    app_grid.set_row_spacing(COL_PADDING);
//    app_grid.set_name("apps");

    app_grid.set_halign(Align::Center);

    update_apps(&desktop_entries);

    let css_provider = CssProvider::new();

    if config_wal == 1 {
        add_class(&window, "pywal");
    }

    let css = get_css();

    if let Ok(_) = css_provider.load_from_data(css.as_ref()) {
        StyleContext::add_provider_for_screen(
            &gdk::Screen::get_default().expect("Error initializing css provider"),
            &css_provider,
            STYLE_PROVIDER_PRIORITY_USER,
        );
    }

    if let Some(display) = window.get_display() {
        let arrow = Cursor::new_from_name(&display, "default")
            .expect("Could not create 'default' cursor!");

        let pointer = Cursor::new_from_name(&display, "pointer")
            .expect("Could not create 'pointer' cursor!");
    }

    window.set_title("Waffy");
    window.show_all();

    gtk::main();
}

struct DesktopEntry {

}

fn get_css() -> String {
    return String::from("window { background: alpha(#000, .7) }");
}

fn update_apps(apps: &Vec<DesktopEntry>) {

}

fn get_monitor_width() -> i32 {
    return 1920;
}

fn find_desktop_entries() -> Vec<DesktopEntry> {
    return Vec::<DesktopEntry>::new();
}

fn open_config() {

}

fn add_class<W: gtk::prelude::WidgetExt>(widget: &W, class_name: &str) {
    let ctx = widget.get_style_context();
    ctx.add_class(class_name);
}