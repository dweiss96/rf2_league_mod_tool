use crate::controller::*;
use crate::slint_generatedMain::*;
use slint::{ComponentHandle, Image, Rgba8Pixel, SharedPixelBuffer};

use crate::tasks::ProcessHandle;
use image;

pub fn run_main_window() {
    let main_window = Main::new().unwrap();

    let mut generator_thread: Option<ProcessHandle> = None;
    let mut output_thread: Option<std::thread::JoinHandle<()>> = None;

    // initialize controller
    workshop_path_controller::initialize(main_window.clone_strong());
    modmgr_path_controller::initialize(main_window.clone_strong());
    generator_controller::initialize(
        main_window.clone_strong(),
        &mut generator_thread,
        &mut output_thread,
    );

    load_images(main_window.clone_strong());
    main_window.run().unwrap();

    println!("finish all threads");
    match (generator_thread, output_thread) {
        (Some(gt), Some(ot)) => {
            gt.kill();
            ot.join().unwrap_or_default();
        }
        (None, Some(ot)) => ot.join().unwrap_or_default(),
        (Some(gt), None) => gt.kill(),
        _ => {}
    }
}

fn load_images(window: Main) {
    window.global::<Images>().set_steam(decode_image_bytes(
        include_bytes!("steam.png").as_slice(),
        (128, 128),
    ));
    window.global::<Images>().set_rfactor2(decode_image_bytes(
        include_bytes!("rfactor2.png").as_slice(),
        (128, 128),
    ));
}

fn decode_image_bytes(bytes: &[u8], dimensions: (u32, u32)) -> Image {
    let di = image::load_from_memory_with_format(bytes, image::ImageFormat::Png).unwrap();
    let rgba8 = di.as_rgba8().unwrap();
    let pixel_buffer =
        SharedPixelBuffer::<Rgba8Pixel>::clone_from_slice(rgba8, dimensions.0, dimensions.1);
    Image::from_rgba8(pixel_buffer)
}
