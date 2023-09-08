use clap::Parser;
use console::Style;
use std::path::PathBuf;
use std::{fs, io};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(value_name = "FILE")]
    path: Option<PathBuf>,
    #[arg(short, long)]
    all: bool,
}

fn main() -> io::Result<()> {
    let cli = Cli::parse();
    let path = cli.path.unwrap_or(PathBuf::from("."));

    let file_style = Style::new().color256(7);
    let dir_style = Style::new().color256(12);
    let link_style = Style::new().color256(14);
    let unknown_style = Style::new().magenta();

    if cli.all {
        println!("{}", dir_style.apply_to("."));
        println!("{}", dir_style.apply_to(".."));
    }
    if path.is_dir() {
        let mut entries = fs::read_dir(path)?
            .filter(|n| {
                cli.all
                    || !matches!(n.as_ref().unwrap().file_name().into_string(), Ok(s) if s
                    .starts_with('.'))
            })
            .collect::<Result<Vec<_>, io::Error>>()?;
        entries.sort_by_key(|e| e.file_name().into_string().unwrap());
        entries.iter().for_each(|e| {
            let style = if e.file_type().unwrap().is_dir() {
                &dir_style
            } else if e.file_type().unwrap().is_symlink() {
                &link_style
            } else if e.file_type().unwrap().is_file() {
                &file_style
            } else {
                &unknown_style
            };
            println!("{}", style.apply_to(e.file_name().to_str().unwrap()));
        });
    } else if path.exists() {
        println!(
            "{}",
            file_style.apply_to(path.file_name().unwrap().to_str().unwrap())
        );
    } else {
        return Err(io::Error::new(io::ErrorKind::NotFound, "Sorry, not found."));
    }

    Ok(())
}
