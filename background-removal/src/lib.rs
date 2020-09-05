use eyre::Result;
use opencv::core::{Size, Vector, BORDER_DEFAULT};
use opencv::prelude::Mat;
use opencv::{imgcodecs, imgproc};
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
        imgproc::gaussian_blur(
            &mut source_matrix,
            &mut output_matrix,
            Size {
                width: 31,
                height: 31,
            },
            -1.,
            -1.,
            BORDER_DEFAULT,
        )?;

        let mut output_buffer = Vector::new();
        let parmeters = Vector::new();

        imgcodecs::imencode(".jpg", &output_matrix, &mut output_buffer, &parmeters)?;
        Ok(output_buffer.to_vec())
    }
}

#[cfg(test)]
mod tests {
    use crate::{BackgroundRemover, OpenCv};
    use opencv::core::MatTraitManual;
    use std::{fs::File, io::Write};

    #[test]
    fn it_works() {
        let opencv = OpenCv::new().unwrap();
        let input = opencv::imgcodecs::imread("input.jpg", 0).unwrap();
        let input: Vec<Vec<u8>> = input.to_vec_2d().unwrap();
        let output = opencv.remove_background(&input).unwrap();
        let mut output_file = File::create("test.jpg").unwrap();
        output_file.write_all(&output).unwrap();
    }
}
