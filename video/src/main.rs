extern crate opencv;

use std::path::Path;
use opencv::videoio::{VideoCapture, VideoCaptureTrait, CAP_ANY};
use opencv::{highgui, core};
use opencv::imgcodecs::imread;
use opencv::prelude::MatTrait;
use opencv::highgui::{imshow, wait_key, destroy_all_windows};
use opencv::core::{MatTraitManual, Point, BORDER_DEFAULT, BORDER_ISOLATED, Size, BORDER_CONSTANT};
use opencv::imgproc::{blur, hough_lines_p, canny, erode};
use opencv::core::BorderTypes::BORDER_REFLECT;

fn main() {
    let img = "test.jpg";

    let mat = imread(&img, 1).unwrap();

    if mat.empty().unwrap() {
        println!("img read error");
    } else {
        imshow("show img", &mat).unwrap();
        wait_key(100).unwrap();

        let mut cam = VideoCapture::new(0, CAP_ANY).unwrap();
        let opened = cam.is_opened().unwrap();

        if !opened {
            wait_key(5000).unwrap();
            panic!("cam not opened");
        }

        loop {
            let mut frame = core::Mat::default();
            cam.read(&mut frame).unwrap();

            let size = frame.size().unwrap();
            if size.width > 0 {
                let mut handled = core::Mat::default();
                // canny(&mut frame, &mut handled, 127.0, 255.0, 100, true);
                blur(&frame, &mut handled, Size::new(50, 50), Point::new(0, 0), BORDER_DEFAULT);

                highgui::imshow("show img", &mut handled).unwrap();
            }

            let key = wait_key(10).unwrap();
            if key > 0 && key != 255 {
                break;
            }
        }
    }
}
