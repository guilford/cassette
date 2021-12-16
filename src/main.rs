/**
 *  Cassette
 *  Content Addressable Storage Container
 */

use std::env;
use std::fs::File;
use std::fs;
use std::io;
use std::io::BufReader;
use std::io::Seek;
use std::io::Write;
use std::path::Path;

use jwalk::{WalkDir};


fn add_file(p: &Path, encoder: &mut dyn Write) -> io::Result<()> {
    let stat = fs::metadata(p)?;
    if stat.is_file() && p != Path::new("./foo.cassette") {
        let file = File::open(p)?;
        let mut hasher = blake3::Hasher::new();
        let mut reader = BufReader::new(file);
        std::io::copy(&mut reader, &mut hasher)?;
        reader.rewind()?;
        std::io::copy(&mut reader, encoder)?;
        let hash = hasher.finalize();
        println!("{}: {}", hash, p.display()); 
    }
    Ok(())
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let dir = args.get(1);
    let archive = File::create("foo.cassette")?;
    let mut frame_info = lz4_flex::frame::FrameInfo::new();
    frame_info.block_mode = lz4_flex::frame::BlockMode::Linked;
    let mut compressor = lz4_flex::frame::FrameEncoder::with_frame_info(frame_info, archive);

    let mut n: u32 = 0;
    for f in WalkDir::new(Path::new(dir.unwrap_or(&String::from(".")))) {
	    add_file(f?.path().as_path(), &mut compressor)?;
        n = n + 1;
    }
    println!("hashed {} files",  n);
    Ok(())
}

