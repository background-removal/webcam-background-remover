use core_traits::BackgroundRemover;
use eyre::Result;
use opencv::core::{Mat, Ptr, Vector};
use opencv::{imgcodecs, prelude::*, video};

pub struct OpenCv {
    background_subtractor: Ptr<dyn BackgroundSubtractorKNN>,
}

impl OpenCv {
    pub fn new() -> Result<Self> {
        let background_subtractor = video::create_background_subtractor_knn(500, 400.0, true)?;
        Ok(Self {
            background_subtractor,
        })
    }
}

impl<Ref> BackgroundRemover<Ref> for OpenCv {
    fn remove_background(&mut self, source: &[Ref]) -> Result<Vec<u8>>
    where
        Ref: AsRef<[u8]>,
    {
        let source_matrix = Mat::from_slice_2d(source)?;
        let mut output_matrix = Mat::default()?;

        self.background_subtractor
            .apply(&source_matrix, &mut output_matrix, 0.0)?;

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
