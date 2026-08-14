use std::{
    io::{Cursor, Read},
    path::Path,
};

use anyhow::{Context, Result};
use serde::Deserialize;
use zip::ZipArchive;

use crate::{info::App, query::app_apk_db_path};

pub fn extract_by_name<T: AsRef<[u8]>>(data: T, name: &str) -> Result<Vec<u8>> {
    let mut zip = ZipArchive::new(Cursor::new(data))?;
    let mut data = zip.by_name(name)?;

    let mut buf = Vec::with_capacity(data.size() as usize);
    data.read_to_end(&mut buf)?;

    Ok(buf)
}

pub fn extract_db(app: App, apk_path: impl AsRef<Path>, db_path: impl AsRef<Path>) -> Result<()> {
    let path = apk_path.as_ref();
    let apk = if path.extension().is_some_and(|ext| ext == "xapk") {
        extract_xapk(path)?
    } else {
        std::fs::read(path)?
    };

    let db = extract_by_name(apk, app_apk_db_path(app))?;
    std::fs::write(&db_path, db)?;
    Ok(())
}

/// Extract name of .apk file from ApkPure's .xapk
fn extract_xapk(path: impl AsRef<Path>) -> Result<Vec<u8>> {
    let xapk = std::fs::read(path).context("failed to read .xapk file")?;
    let manifest =
        extract_by_name(&xapk, "manifest.json").context("manifest.json not found in .xapk")?;
    let manifest =
        std::str::from_utf8(&manifest).context("expected manifest to be utf-8 encoded")?;
    let manifest: XapkManifest =
        serde_json::from_str(manifest).context("failed to parse manifest.json")?;

    extract_by_name(xapk, &format!("{}.apk", manifest.package_name))
}

#[derive(Debug, Deserialize)]
struct XapkManifest {
    package_name: String,
}
