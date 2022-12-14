use std::{
    fs::{self, File},
    io::Read,
    path::PathBuf,
    sync::Arc,
};

use anyhow::Result;
use numerals::roman::Roman;
use reqwest::{blocking::Client, cookie::Jar};

const INP_PATH: &str = "/home/dnk/Projects/advent-of-code/2022/rust/inputs/";

pub fn read_or_fetch(day: u32) -> Result<String> {
    let fname = PathBuf::from(format!("{INP_PATH}{:x}", Roman::from(day as i16)));
    if let Ok(input) = read_file(&fname) {
        Ok(input)
    } else {
        let input = fetch(&format!("https://adventofcode.com/2022/day/{day}/input"))?;
        fs::write(&fname, input.clone())?;
        Ok(input)
    }
}

pub fn read_file(fname: &PathBuf) -> Result<String> {
    let mut f = File::open(fname)?;
    let mut input = String::new();
    f.read_to_string(&mut input)?;
    Ok(input)
}

fn fetch(url: &str) -> Result<String> {
    const COOKIE: &str = include_str!("../.cookie");
    let jar = Jar::default();
    let url_parsed = url.parse()?;
    jar.add_cookie_str(COOKIE, &url_parsed);
    let client = Client::builder()
        .cookie_store(true)
        .cookie_provider(Arc::new(jar))
        .user_agent("duni-k")
        .build()?;
    let request = client.get(url_parsed).build()?;

    Ok(client.execute(request)?.text()?.to_string())
}
