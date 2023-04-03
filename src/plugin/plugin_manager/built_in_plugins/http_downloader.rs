use std::fs::OpenOptions;
use std::io::{Read, Write};
use std::path::PathBuf;
use std::thread;
use std::thread::Thread;
use std::time::Duration;
use reqwest::blocking::Client;
use url::Url;
use crate::plugin::downloader::{Downloader, DownloadFailed};
use crate::plugin::plugin_errors;


pub struct HttpDownloader{}

impl Downloader for HttpDownloader {
    fn name(&self) -> &'static str {
        "Http Downloader"
    }

    fn supported_protocols(&self) -> Vec<&'static str> {
        vec!["http", "https"]
    }

    fn download(&self, src: &Url, dest: PathBuf, callback: &mut dyn FnMut(u64)) -> plugin_errors::Result<()> {
        src.path_segments().unwrap().last().unwrap()

        Ok(())
    }
}