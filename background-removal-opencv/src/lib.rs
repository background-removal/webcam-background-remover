use core_traits::BackgroundRemover;
use eyre::Result;
use opencv::core::{Mat, Vector};
use opencv::{core, highgui, imgcodecs, imgproc, prelude::*, video, videoio};

pub struct OpenCv {}

impl OpenCv {
    pub fn new() -> Result<Self> {
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

        let mut background_extractor =
            video::create_background_subtractor_knn(500, 400.0, true).unwrap();

        background_extractor.apply(&source_matrix, &mut output_matrix, 0.0);

        let mut output_buffer = Vector::new();
        let parameters = Vector::new();
        imgcodecs::imencode(".png", &output_matrix, &mut output_buffer, &parameters)?;

        Ok(output_buffer.to_vec())
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn it_works() {}
}
