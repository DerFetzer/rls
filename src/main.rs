use std::{env, fs, io, path};

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let path = if args.len() > 1 {
        args.last().unwrap()
    } else {
        "."
    };
    let path = path::Path::new(path);
    if path.is_dir() {
        let mut entries = fs::read_dir(path)?
            .map(|res| res.map(|e| e.file_name().into_string().unwrap()))
            .filter(|n| !matches!(n, Ok(s) if s.starts_with('.')))
            .collect::<Result<Vec<_>, io::Error>>()?;
        entries.sort();
        entries.iter().for_each(|e| println!("{e}"));
    } else if path.exists() {
        println!("{}", path.file_name().unwrap().to_str().unwrap());
    } else {
        return Err(io::Error::new(io::ErrorKind::NotFound, "Sorry, not found."));
    }

    Ok(())
}
