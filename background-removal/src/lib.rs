use eyre::Result;
extern crate opencv;
use opencv::core::{Vector, Mat};
use opencv::{
    core,
    highgui,
    prelude::*,
    video,
    videoio,
    imgcodecs
};

trait BackgroundRemover<Ref> {
    fn remove_background(&self, source: &[Ref]) -> Result<Vec<u8>>
    where
        Ref: AsRef<[u8]>;
}

struct OpenCv {}

impl OpenCv {
    fn new() -> Result<Self> {
        Ok(Self {})
    }
}

impl<Ref> BackgroundRemover<Ref> for OpenCv {
    fn remove_background(&self, source: &[Ref]) -> Result<Vec<u8>>
    where
        Ref: AsRef<[u8]>,
    {
        let mut source_matrix = Mat::from_slice_2d(source)?;
        let mut output_matrix = Mat::default()?;

        let mut output_buffer = Vector::new();

        // TODO: set things here
        // No idea how to convert Vec to Mat for filesave

        Ok(output_buffer.to_vec())
    }
}

#[cfg(test)]
mod tests {
    use crate::{BackgroundRemover, OpenCv};
    extern crate opencv;
    use opencv::core::{Vector};
    use opencv::prelude::*;

    #[test]
    fn it_works() {
        let opencv = OpenCv::new().unwrap();
        let mut cam = opencv::videoio::VideoCapture::new(0, opencv::videoio::CAP_ANY).unwrap();  // 0 is the default camera
        let opened = opencv::videoio::VideoCapture::is_opened(&cam).unwrap();
        if !opened {
            panic!("Unable to open default camera!");
        }
        let parmeters = Vector::new();
        let mut i = 0;
        let number_of_frames = 3;
        let mut background_extractor = opencv::video::create_background_subtractor_knn(500, 400.0, true).unwrap();


        let mut output_buffer = Mat::default().unwrap();

        while i < number_of_frames {
            let mut frame = Mat::default().unwrap();
            cam.read(&mut frame);
            let mut gray = Mat::default().unwrap();

            opencv::imgproc::cvt_color(
                &frame,
                &mut gray,
                opencv::imgproc::COLOR_BGR2GRAY,
                0
            ).unwrap();

            background_extractor.apply(&frame, &mut output_buffer, 0.0);

            let filename = format!("/app/temp/no_background{}.png", i);
            opencv::imgcodecs::imwrite(&filename, &mut output_buffer, &parmeters);

            i += 1;
        }
    }
}
