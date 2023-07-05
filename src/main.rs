mod models {
    automod::dir!(pub "src/models");
}
mod controller {
    automod::dir!(pub "src/controller");
}
mod os_methods;
mod view;
mod tasks;

slint::include_modules!();

fn main() {
    view::run_main_window()
}
