// final
use colored::Colorize;
use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

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
const HELP: &str = include_str!("../help.txt");

fn print_help() {
    print!("{HELP}");
}

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

fn run(cfg: Config) -> io::Result<()> {
    //collect all files to be searched
    let mut files: Vec<PathBuf> = Vec::new();

    for p in &cfg.paths {
        if p.is_file() {
            files.push(p.to_path_buf());
        } else if cfg.recursive {
            for entry in WalkDir::new(p).into_iter().filter_map(Result::ok) {
                if entry.file_type().is_file() {
                    files.push(entry.path().to_path_buf());
                }
            }
        } else {
        }
    }

    files.sort_by(|a, b| a.to_string_lossy().cmp(&b.to_string_lossy()));

    //search each file individually
    for f in files {
        search_one_file(&cfg, &f)?;
    }

    Ok(())
}

fn search_one_file(cfg: &Config, path: &Path) -> io::Result<()> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    // -i
    let pat_lc = if cfg.case_insensitive {
        Some(cfg.pattern.to_lowercase())
    } else {
        None
    };

    let path_str = path.to_string_lossy().replace('\\', "/");

    for (idx, line_res) in reader.lines().enumerate() {
        let line = line_res?;

        // determine whether it matches
        let is_match = if let Some(ref pat) = pat_lc {
            line.to_lowercase().contains(pat)
        } else {
            line.contains(&cfg.pattern)
        };

        // -v：
        let should_print = if cfg.invert { !is_match } else { is_match };
        if !should_print {
            continue;
        }

        let display_line = if cfg.color && !cfg.invert && is_match {
            highlight_line(&line, &cfg.pattern, cfg.case_insensitive)
        } else {
            line.clone()
        };

        if cfg.print_filenames && cfg.line_numbers {
            println!("{}: {}: {}", path_str, idx + 1, display_line);
        } else if cfg.print_filenames {
            println!("{}: {}", path_str, display_line);
        } else if cfg.line_numbers {
            println!("{}: {}", idx + 1, display_line);
        } else {
            println!("{}", display_line);
        }
    }

    Ok(())
}

// highlight non-overlapping matches in red
fn highlight_line(line: &str, pattern: &str, case_insensitive: bool) -> String {
    if pattern.is_empty() {
        return line.to_string();
    }

    if !case_insensitive {
        let mut out = String::new();
        let mut i = 0;
        while let Some(pos) = line[i..].find(pattern) {
            let start = i + pos;
            let end = start + pattern.len();
            out.push_str(&line[i..start]);
            out.push_str(&line[start..end].red().to_string());
            i = end;
        }
        out.push_str(&line[i..]);
        return out;
    }

    let ll = line.to_lowercase();
    let pp = pattern.to_lowercase();
    let mut out = String::new();
    let mut i = 0;
    while let Some(pos) = ll[i..].find(&pp) {
        let start = i + pos;
        let end = start + pp.len();
        out.push_str(&line[i..start]);
        out.push_str(&line[start..end].red().to_string());
        i = end;
    }
    out.push_str(&line[i..]);
    out
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
