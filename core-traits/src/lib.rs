use eyre::Result;

pub trait BackgroundRemover<Ref> {
    fn remove_background(&mut self, source: &[Ref]) -> Result<Vec<u8>>
    where
        Ref: AsRef<[u8]>;
}
