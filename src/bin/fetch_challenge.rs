use std::io::prelude::*;
use std::fs::File;
use std::env;
use std::path::PathBuf;

fn main() -> anyhow::Result<()> {
    let session = env::var("AOC_SESSION")?;
    let day = env::args().nth(1).unwrap().parse::<usize>()?;
    let url = format!("https://adventofcode.com/2022/day/{}/input", day);
    let resp = ureq::get(&url)
        .set("Cookie", &format!("session={}", session))
        .call()?;
    let input = resp.into_string()?;
    let path = format!("inputs/{:02}.txt", day);
    let mut file = File::create(&path)?;
    file.write_all(input.as_bytes())?;

    let bin_path = PathBuf::from(format!("src/bin/d{:02}.rs", day));
    if !bin_path.exists() {
        let mut bin_file = File::create(&bin_path)?;
        std::io::copy(&mut File::open("src/template.rs")?, &mut bin_file)?;
    }
    Ok(())
}
