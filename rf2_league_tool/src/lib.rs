use std::sync::{Arc, Mutex};
use std::thread::JoinHandle;

pub mod controller {
    automod::dir!(pub "src/controller");
}
mod os_methods;
pub mod view;

slint::include_modules!();

pub type ThreadHandle = Arc<Mutex<Option<JoinHandle<()>>>>;

pub trait EmptyThreadHandleCreation {
    fn empty() -> ThreadHandle;
}

impl EmptyThreadHandleCreation for ThreadHandle {
    fn empty() -> ThreadHandle {
        Arc::new(Mutex::new(None))
    }
}
