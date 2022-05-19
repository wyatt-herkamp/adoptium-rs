use crate::{InstallerError, ADOPTIUM_USER_AGENT};
use adoptium_api::Adoptium;
use async_compression::tokio::bufread::{GzipDecoder, GzipEncoder};
use bytes::Bytes;
use futures_util::StreamExt;
use indicatif::{ProgressBar, ProgressStyle};
use reqwest::ClientBuilder;
use std::io;
use std::io::Read;
use std::path::PathBuf;
use std::str::FromStr;
use tokio::fs::{create_dir_all, remove_dir_all, remove_file, OpenOptions};
use tokio::io::{duplex, AsyncReadExt, AsyncWriteExt, BufReader};
use tokio::sync::mpsc::channel;
use tokio_tar::Archive;
use url::Url;

pub async fn download(url: Url, total_size: u64, location: PathBuf) -> Result<(), InstallerError> {
    let client = ClientBuilder::new()
        .user_agent(ADOPTIUM_USER_AGENT)
        .build()?;
    if location.exists() {
        remove_dir_all(&location).await?
    }
    create_dir_all(&location).await?;
    let pb = ProgressBar::new(total_size);
    pb.set_style(ProgressStyle::default_bar()
        .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {bytes}/{total_bytes} ({eta})")
        .progress_chars("#>-"));
    let source = client.get(url).send().await?;
    if !source.status().is_success() {
        return Err(InstallerError::Custom(format!(
            "Bad Response {}",
            source.status()
        )));
    }
    let (mut send, read) = duplex(1024);
    tokio::spawn(async move {
        let decoder = GzipDecoder::new(BufReader::new(read));
        let mut archive = Archive::new(decoder);
        archive.unpack(&location).await.unwrap();
    });
    let mut stream = source.bytes_stream();
    while let Some(item) = stream.next().await {
        let chunk: Bytes = item.unwrap();
        pb.inc(chunk.len() as u64);
        send.write_all(chunk.as_ref()).await?;
    }

    Ok(())
}
