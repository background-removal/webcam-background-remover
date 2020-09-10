use color_eyre::eyre::{ensure, WrapErr};
use color_eyre::Result;
use opencv::core::{Vector, CV_8UC1};
use opencv::highgui::{imshow, wait_key};
use opencv::imgcodecs;
use opencv::prelude::*;
use opencv::types::VectorOfu8;
use opencv::videoio;

use background_removal::OpenCv;
use core_traits::BackgroundRemover;
use opencv::imgcodecs::imdecode;
use opencv::imgproc::{cvt_color, COLOR_BGR2BGRA, COLOR_RGB2GRAY};

fn main() -> Result<()> {
    color_eyre::install()?;
    let opencv = OpenCv::new()?;
    let mut cam = videoio::VideoCapture::new(0, videoio::CAP_ANY)?; // 0 is the default camera
    ensure!(
        videoio::VideoCapture::is_opened(&cam)?,
        "Unable to open default camera!"
    );

    loop {
        let mut raw_frame = Mat::default()?;
        if !cam.read(&mut raw_frame)? {
            println!("Skipped empty frame");
        }
        println!("{:?}", raw_frame.size());
        println!("{}", raw_frame.dims());
        println!("{:?}", raw_frame.channels());
        println!("{:#?}", raw_frame);

        let mut raw_frame_one_channel = Mat::default()?;
        cvt_color(&raw_frame, &mut raw_frame_one_channel, COLOR_RGB2GRAY, 0)?;
        println!("{:#?}", raw_frame_one_channel);

        // let decoded_raw_frame = imdecode(&raw_frame, 0)?;
        let decoded_raw_frame = raw_frame_one_channel
            .to_vec_2d()
            .wrap_err("failed to convert raw frame to generic matrix")?;
        let encoded_frame = opencv.remove_background(&decoded_raw_frame)?;

        // As the inner crates implement a generic interface, we need to decode again to a matrix
        let encoded_frame = VectorOfu8::from(encoded_frame);
        let decoded_frame = imgcodecs::imdecode(&encoded_frame, 0)?;
        imshow("Original", &raw_frame)?;
        imshow("Converted", &raw_frame_one_channel)?;
        if wait_key(10)? > 0 {
            break;
        }
    }
    Ok(())
}
//
// let opencv = OpenCv::new().unwrap();
// let mut cam = opencv::videoio::VideoCapture::new(0, opencv::videoio::CAP_ANY).unwrap();  // 0 is the default camera
// let opened = opencv::videoio::VideoCapture::is_opened(&cam).unwrap();
// if !opened {
// panic!("Unable to open default camera!");
// }
// let parmeters = Vector::new();
// let mut i = 0;
// let number_of_frames = 3;
// let mut background_extractor = opencv::video::create_background_subtractor_knn(500, 400.0, true).unwrap();
//
//
// let mut output_buffer = Mat::default().unwrap();
//
// while i < number_of_frames {
// let mut frame = Mat::default().unwrap();
// cam.read(&mut frame);
// let mut gray = Mat::default().unwrap();
//
// opencv::imgproc::cvt_color(
// &frame,
// &mut gray,
// opencv::imgproc::COLOR_BGR2GRAY,
// 0
// ).unwrap();
//
// background_extractor.apply(&frame, &mut output_buffer, 0.0);
//
// let filename = format!("/app/temp/no_background{}.png", i);
// opencv::imgcodecs::imwrite(&filename, &mut output_buffer, &parmeters);
//
// i += 1;
// }
