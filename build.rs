use git2::{Repository, DescribeOptions};

use std::env;
use std::convert::AsRef;
use std::fs::{File, create_dir_all};
use std::io::{Write, Read, BufWriter, Error, ErrorKind};
use std::path::Path;
use std::string::ToString;

extern crate chrono;
use chrono::Utc;

fn main() {
    let desc = Repository::discover(".").unwrap().describe(&DescribeOptions::new().describe_tags().show_commit_oid_as_fallback(true)).unwrap().format(None).unwrap();
    write_value("version.rs", "VERSION", desc).expect("Writing commit hash");

    write_value("date.rs", "DATE", Utc::now().format("%Y-%m-%d")).expect("Writing date");
}

pub fn write_value<P: AsRef<Path>, S: ToString>(dir: P, var_name: &'static str, value: S) -> Result<(), Error> {
    let path = env::var_os("OUT_DIR").ok_or(Error::new(ErrorKind::Other, "Idk"))?;
    let path: &Path = path.as_ref();

    create_dir_all(path)?;

    let path = path.join(dir);

    let content = format!("static {}: &'static str = \"{}\";\n", var_name, value.to_string());

    let is_fresh = if path.exists() {
        let mut f = File::open(&path)?;
        let mut current = String::new();
        f.read_to_string(&mut current)?;

        current == content
    } else {
        false
    };

    if !is_fresh {
      let mut file = BufWriter::new(File::create(&path)?);

      write!(file, "{}", content)?;
    }
    Ok(())
}
