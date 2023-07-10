pub mod models {
    automod::dir!(pub "src/models");
}
pub mod controller {
    automod::dir!(pub "src/controller");
}
pub mod config;
pub mod os_methods;
pub mod tasks;
pub mod view;

slint::include_modules!();
