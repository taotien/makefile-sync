use std::{
    collections::VecDeque,
    fs::{read_dir, read_to_string, write},
};

use anyhow::Context;

const MF_PATH: &str = "Makefile";
const USER_PATH: &str = "user";

fn main() -> anyhow::Result<()> {
    let mf = read_to_string(MF_PATH).context("could not find Makefile in current directory")?;

    let (head, uprogs) = mf.split_at(
        mf.find("UPROGS")
            .context("could not find UPROGS in makefile")?,
    );
    let (_uprogs, end) = uprogs.split_at(
        uprogs
            .find("fs.img")
            .context("could not find fs.img (end of block magic phrase) in Makefile")?,
    );

    let mut c_files: Vec<_> = read_dir(USER_PATH)
        .context("could not find user dir")?
        .filter_map(|e| {
            let e = e.ok()?.path();
            if e.extension()? == "c"
                && !["printf", "ulib", "umalloc"].contains(&e.file_stem()?.to_str()?)
            {
                let base = e.file_stem()?.to_string_lossy().to_string();
                format!("\t$U/_{}\\", base).into()
            } else {
                None
            }
        })
        .collect();

    c_files.sort_unstable();

    let mut c_files = VecDeque::from(c_files);
    c_files.push_front("UPROGS=\\".into());
    c_files.push_back("\n".into());

    let uprogs = Vec::from(c_files);
    let uprogs = uprogs.join("\n");

    let mf = format!("{head}{uprogs}{end}");

    write(MF_PATH, mf)?;

    Ok(())
}
