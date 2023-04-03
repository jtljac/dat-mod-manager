use std::fmt;
use std::fmt::Formatter;
use std::path::PathBuf;
use url::Url;

pub trait Downloader {
    fn name(&self) -> &'static str;
    fn supported_protocols(&self) -> Vec<&'static str>;
    fn download(&self, src: &Url, dest: PathBuf, callback: &mut dyn FnMut(u64, u64, String)) -> Result<(), DownloadFailed>;
}

#[derive(Debug)]
pub struct DownloadFailed {
    pub msg: String
}