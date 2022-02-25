/**
 *  Cassette
 *  Content Addressable Storage Container
 */

use std::collections::BTreeMap;
use std::env;
use std::fs;
use std::fs::File;
use std::io;
use std::io::Seek;
use std::io::Write;
use std::os::linux::fs::MetadataExt;
use std::path::Path;
use std::time::Instant;

use jwalk::WalkDir;


fn print_size(sz: u64) -> String {
    if sz > 1073741824 {
       return format!(" {} GB ", sz / 1073741824);
    } else if sz > 1048576 {
       return format!(" {} MB ", sz / 1048576);
    } else if sz > 1024 {
       return format!(" {} KB ", sz / 1024);
    }

    return format!(" {} B ", sz);
}

fn add_file(
    p: &Path,
    encoder: &mut dyn Write,
    map: &mut BTreeMap<[u8; 32], String>,
) -> io::Result<u64> {
    let stat = fs::metadata(p)?;
    if stat.is_file() && p != Path::new("./foo.cassette") {
        let mut file = File::open(p)?;
        let mut hasher = blake3::Hasher::new();
        std::io::copy(&mut file, &mut hasher)?;
        file.rewind()?;
        let hash = hasher.finalize();
        std::io::copy(&mut file, encoder)?;
        let path_string = String::from(p.to_str().unwrap());
        map.insert(*hash.as_bytes(), path_string);
    }
    Ok(stat.st_size())
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let dir = args.get(1);
    let archive = File::create("foo.cassette")?;
    let mut frame_info = lz4_flex::frame::FrameInfo::new();
    frame_info.block_mode = lz4_flex::frame::BlockMode::Linked;
    let mut compressor = lz4_flex::frame::FrameEncoder::with_frame_info(frame_info, archive);
    let mut map = BTreeMap::new();

    let now = Instant::now();
    let mut n: u32 = 0;
    let mut sz: u64 = 0;
    for f in WalkDir::new(Path::new(dir.unwrap_or(&String::from(".")))).sort(true) {
        sz += add_file(f?.path().as_path(), &mut compressor, &mut map)?;
        n = n + 1;
    }

    for (hash_bytes, fname) in &map {
        println!("{} : {}", blake3::Hash::from(*hash_bytes), fname);
    }

    println!("hashed & bundled {} files and {} bytes in {} seconds ", n, print_size(sz), now.elapsed().as_secs_f32());
    let mut file = File::open("./foo.cassette")?;
    let mut hasher = blake3::Hasher::new();
    std::io::copy(&mut file, &mut hasher)?;
    println!("foo.cassette : {}", hasher.finalize());

    Ok(())
}
