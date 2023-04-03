use std::fs::OpenOptions;
use std::io::{Read, Write};
use std::path::Path;
use std::time::Duration;
use error_chain::bail;
use reqwest::blocking::Client;
use url::Url;
use crate::plugin::plugin_errors;

pub fn get_http_file_size(url: Url) -> plugin_errors::Result<u64> {
    let client = Client::builder()
        .timeout(Duration::from_secs(5))
        .build()
        .unwrap();

    let mut res = client.head(url.clone())
        .send()
        .or(Err(plugin_errors::ErrorKind::DownloadFailedToConnect))?;

    res.content_length().ok_or(Err(plugin_errors::ErrorKind::DownloadNoFileSize)?)
}

pub fn download_http_file(url: Url, dest: &Path, progress_callback: &mut dyn FnMut(u64)) -> plugin_errors::Result<()> {
    let client = Client::builder()
        .timeout(Duration::from_secs(5))
        .build()
        .unwrap();

    let mut res = client.get(url.clone())
        .send()
        .or(Err(plugin_errors::ErrorKind::DownloadFailedToConnect))?;

    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .open(dest)
        .or(Err(plugin_errors::ErrorKind::DownloadFailedToCreateDestFile))?;

    let mut buffer: [u8; 1024] = [0; 1024];
    let mut current_size: u64 = 0;
    loop {
        match res.read(&mut buffer) {
            Ok(size) => {
                if size > 0 {
                    file.write_all(&buffer[0..size]).unwrap();
                    current_size += size as u64;
                    progress_callback(current_size);
                } else {
                    break;
                }
            }
            Err(err) => {
                bail!(plugin_errors::Error::with_chain(err, plugin_errors::ErrorKind::DownloadErrorDuringDownload));
            }
        };
    }

    Ok(())
}