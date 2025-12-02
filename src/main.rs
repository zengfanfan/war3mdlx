use byteorder::{BigEndian, LittleEndian, ReadBytesExt};
use clap::{ArgAction, Parser};
use derive_debug::Dbg;
use glam::{Vec2, Vec3, Vec4};
use paste::paste;
use pretty_hex::*;
use std::fmt::Debug as stdDebug;
use std::io::{self, Cursor, Read};
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

mod error;
mod extends;
mod fields;
mod logging;
mod parser;
mod string;
mod types;
mod util;

use error::*;
use extends::*;
use fields::*;
use logging::*;
use parser::*;
use string::*;
use types::*;
use util::*;

#[derive(Debug, Parser, Clone)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(hide = true)]
    input: String,
    #[arg(hide = true)]
    output: Option<String>,
    #[arg(long, help = "Convert *.mdl to *.mdx")]
    mdl2x: bool,
    #[arg(long, help = "Convert *.mdx to *.mdl")]
    mdx2l: bool,
    #[arg(long, short = 'F', help = "Put output files in one directory and ignore hierarchy")]
    flat: bool,
    #[arg(long, short = 'f', help = "Overwrite existing output files (default: skip)")]
    overwrite: bool,
    #[arg(long, short = 'E', help = "Stop walking the directory hierarchy when an error occurs")]
    stop_on_error: bool,
    #[arg(long, short, help = "Do not print log messages")]
    quiet: bool,
    #[arg(long, short, action = ArgAction::Count, help = "Print verbose log messages (-vv very verbose)")]
    verbose: u8,
    #[arg(long, short = 'd', default_value_t = 255, value_name = "0..255", help = "Max depth of directory traversal")]
    max_depth: u8,
}

macro_rules! EXIT {
    () => {{
        return Ok(());
    }};
    ($s:expr) => {{
        elog!("{}", $s);
        return Ok(());
    }};
    ($($arg:tt)*) => {{
        elog!($($arg)*);
        return Ok(());
    }};
}

fn main() -> io::Result<()> {
    let mut args: Args = Parser::parse();
    let input = PathBuf::from(&args.input);

    if args.quiet {
        set_log_level(LogLevel::Warn);
    } else {
        match args.verbose {
            1 => set_log_level(LogLevel::Verbose),
            2 => set_log_level(LogLevel::Verbose2),
            3.. => set_log_level(LogLevel::Verbose3),
            _ => (),
        }
    }

    if input.is_file() {
        if let Err(e) = check_input_file(&input, &mut args) {
            EXIT!("{}", e);
        }
        let ouput = match &args.output {
            Some(o) => PathBuf::from(o),
            None => input.with_extension(match args.mdl2x {
                true => "mdx",
                false => "mdl",
            }),
        };
        if let Err(e) = check_output_file(&ouput, &mut args) {
            EXIT!("{}", e);
        }
        if ouput.exists() && !args.overwrite {
            log!("Skipped existing file: {:?}", ouput);
            EXIT!();
        }

        if let Err(e) = process_file(&input, &ouput) {
            EXIT!(match e {
                MyError::String(s) => s,
                _ => format!("{:?}", e),
            });
        }
    } else if input.is_dir() {
        let mut ouput: Option<PathBuf> = None;
        if let Some(o) = &args.output {
            let opath = PathBuf::from(o);
            if opath.is_dir() {
                ouput = Some(opath);
            } else {
                EXIT!("Output path should be also a directory: {:?}", o);
            }
        }

        for entry in WalkDir::new(&input).max_depth(args.max_depth as usize + 1).into_iter().filter_map(|e| e.ok()) {
            let p = entry.path();
            if !p.is_file() {
                continue;
            }

            let mut _args = args.clone();
            if let Err(_) = check_input_file(&p, &mut _args) {
                continue;
            }

            let opath = match &ouput {
                Some(o) => o,
                None => &input,
            };
            let ofile = if args.flat {
                opath.join(p.file_name().unwrap())
            } else {
                opath.join(p.strip_prefix(&input).unwrap())
            };
            let ofile = ofile.with_extension(match _args.mdl2x {
                true => "mdx",
                false => "mdl",
            });

            if ofile.exists() && !args.overwrite {
                log!("Skipped existing file: {:?}", ofile);
                continue;
            }

            if let Err(e) = process_file(p, &ofile) {
                let s = match e {
                    MyError::String(s) => s,
                    _ => format!("{:?}", e),
                };
                if args.stop_on_error { EXIT!(s) } else { elog!("{}", s) }
            }
        }
    } else {
        EXIT!("Not a file or directory: {:?}", input);
    }

    EXIT!();
}

fn check_input_file(input: &Path, args: &mut Args) -> Result<(), String> {
    let iext = input.extension().and_then(|s| s.to_str()).unwrap_or("");
    if !args.mdl2x && !args.mdx2l {
        (args.mdl2x, args.mdx2l) = (iext == "mdl", iext == "mdx");
    }
    if args.mdl2x && iext != "mdl" {
        return Err(format!("Invalid input path: {:?}, expected *.mdl", input));
    } else if args.mdx2l && iext != "mdx" {
        return Err(format!("Invalid input path: {:?}, expected *.mdx", input));
    } else if iext != "mdl" && iext != "mdx" {
        return Err(format!("Invalid input path: {:?}, expected *.mdl or *.mdx", input));
    }
    return Ok(());
}

fn check_output_file(output: &Path, args: &Args) -> Result<(), String> {
    let ext = output.extension().and_then(|s| s.to_str()).unwrap_or("");
    if args.mdl2x && ext != "mdx" {
        return Err(format!("Invalid output path: {:?}, expected *.mdx", output));
    } else if args.mdx2l && ext != "mdl" {
        return Err(format!("Invalid output path: {:?}, expected *.mdl", output));
    } else if ext != "mdl" && ext != "mdx" {
        return Err(format!("Invalid output path: {:?}, expected *.mdl or *.mdx", output));
    }
    return Ok(());
}

fn process_file(input: &Path, output: &Path) -> Result<(), MyError> {
    log!("Converting {:?} -> {:?} ...", input, output);
    let data = MdlxData::read(input)?;
    return data.write(output);
}
