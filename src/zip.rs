use std::{
    io::{Cursor, Read},
    path::Path,
};

use anyhow::Result;
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
    let apk = std::fs::read(apk_path)?;
    let db = extract_by_name(apk, app_apk_db_path(app))?;
    std::fs::write(&db_path, db)?;
    Ok(())
}
