use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::{Path, PathBuf};

#[derive(Debug, Default)]
struct Config {
    // Position parameter
    pattern: String,
    paths: Vec<PathBuf>,
    case_insensitive: bool, // -i
    line_numbers: bool,     // -n
    invert: bool,           // -v
    recursive: bool,        // -r/-R
    print_filenames: bool,  // -f
    color: bool,            // -c
    help: bool,             // -h/--help
}

const HELP: &str = "Usage: grep [OPTIONS] <pattern> <files...>\n\
\n\
Options:\n\
  -i          Case-insensitive search\n\
  -n          Print line numbers\n\
  -v          Invert match (exclude lines that match the pattern)\n\
  -r          Recursive directory search\n\
  -f          Print filenames\n\
  -c          Enable colored output\n\
  -h, --help  Show help information\n";

fn parse_args<I, S>(iter: I) -> Option<Config>
where
    I: IntoIterator<Item = S>,
    S: Into<String>,
{
    let mut cfg = Config::default();
    let mut have_pattern = false;

    for arg in iter.into_iter().skip(1).map(Into::into) {
        if arg == "-h" || arg == "--help" {
            cfg.help = true;
            return Some(cfg);
        }
        match arg.as_str() {
            "-i" => cfg.case_insensitive = true,
            "-n" => cfg.line_numbers = true,
            "-v" => cfg.invert = true,
            "-r" | "-R" => cfg.recursive = true,
            "-f" => cfg.print_filenames = true,
            "-c" => cfg.color = true,
            a if a.starts_with('-') => {}
            _ => {
                if !have_pattern {
                    cfg.pattern = arg;
                    have_pattern = true;
                } else {
                    cfg.paths.push(PathBuf::from(arg));
                }
            }
        }
    }
    Some(cfg)
}

fn print_help() {
    print!("{HELP}");
}

fn run(cfg: Config) -> io::Result<()> {
    for p in &cfg.paths {
        if p.is_dir() {
            continue;
        }
        search_one_file(&cfg, p)?;
    }
    Ok(())
}

fn search_one_file(cfg: &Config, path: &Path) -> io::Result<()> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let pat_lc = if cfg.case_insensitive {
        Some(cfg.pattern.to_lowercase())
    } else {
        None
    };

    for (idx, line_res) in reader.lines().enumerate() {
        let line = line_res?;

        // determine whether this line matches
        let is_match = if let Some(ref pat) = pat_lc {
            line.to_lowercase().contains(pat)
        } else {
            line.contains(&cfg.pattern)
        };

        // -v negates the result
        let should_print = if cfg.invert { !is_match } else { is_match };

        if should_print {
            if cfg.line_numbers {
                println!("{}: {}", idx + 1, line);
            } else {
                println!("{}", line);
            }
        }
    }
    Ok(())
}

fn main() {
    let cfg = parse_args(env::args()).expect("failed to parse args");

    if cfg.help {
        print_help();
        return;
    }
    if cfg.pattern.is_empty() || cfg.paths.is_empty() {
        print_help();
        return;
    }

    if let Err(e) = run(cfg) {
        panic!("error: {e}");
    }
}
