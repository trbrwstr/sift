use std::{
    fs::File,
    io::{BufRead, BufReader, Seek, SeekFrom},
    thread,
    time::Duration,
};

pub fn run(path: &str) -> std::io::Result<()> {
    let canonical = std::fs::canonicalize(path)?;
    let mut file = File::open(canonical)?;
    let mut pos = file.seek(SeekFrom::End(0))?;

    loop {
        let mut reader = BufReader::new(&file);
        reader.seek(SeekFrom::Start(pos))?;

        let mut line = String::new();
        while reader.read_line(&mut line)? > 0 {
            print!("{}", line);
            line.clear();
        }

        pos = reader.stream_position()?;
        thread::sleep(Duration::from_millis(500));
    }
}
