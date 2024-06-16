use anyhow::Result;
use nodb::{DumpPolicy, NoDb, SerializationMethod};
use std::path::Path;

use crate::DB_PATH;

fn is_db_init() -> bool {
    Path::new(DB_PATH).exists()
}

pub fn start_db() -> Result<NoDb> {
    let db = if is_db_init() {
        NoDb::load(DB_PATH, DumpPolicy::Auto, SerializationMethod::Json)?
    } else {
        NoDb::new(DB_PATH, DumpPolicy::Auto, SerializationMethod::Json)
    };
    Ok(db)
}
