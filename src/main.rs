mod models {
    automod::dir!(pub "src/models");
}
mod controller {
    automod::dir!(pub "src/controller");
}
mod os_methods;
mod view;

slint::include_modules!();

fn main() {
    view::run_main_window()
}
