extern crate opencv;

use std::path::Path;
use opencv::videoio::{VideoCapture, VideoCaptureTrait, CAP_ANY};
use opencv::{highgui, core};
use opencv::imgcodecs::imread;
use opencv::prelude::MatTrait;
use opencv::highgui::{imshow, wait_key, destroy_all_windows};
use opencv::core::MatTraitManual;

fn main() {
    let img = "test.jpg";

    let mat = imread(&img, 1).unwrap();

    if mat.empty().unwrap() {
        println!("img read error");
    } else {
        imshow("show img", &mat).unwrap();
        wait_key(1000).unwrap();

        let mut cam = VideoCapture::new(0, CAP_ANY).unwrap();
        let opened = cam.is_opened().unwrap();

        if !opened {
            wait_key(5000).unwrap();
            panic!("cam not opened");
        }

        loop {
            let mut frame = core::Mat::default();
            cam.read(&mut frame).unwrap();

            let  size = frame.size().unwrap();
            if size.width > 0 {
                highgui::imshow("show img", &mut frame).unwrap();
            }

            let key = wait_key(10).unwrap();
            if key > 0 && key != 255 {
                break;
            }
        }
    }
}
