use std::path::PathBuf;
use std::sync::mpsc::Sender;
use rayon::{ThreadPool, ThreadPoolBuilder};
use url::Url;

pub enum DownloadState {

}

pub struct DownloadManager {
    pool: ThreadPool
}

impl DownloadManager {
    pub fn download(url: Url, dest: PathBuf, sender: Sender<DownloadState>) {

    }

    pub fn new() -> Self {
        Self {
            pool: ThreadPoolBuilder::new()
                .num_threads(5)
                .build()
                .unwrap()
        }
    }
}