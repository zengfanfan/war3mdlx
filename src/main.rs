use clap::Parser;
use std::{
    io,
    path::{Path, PathBuf},
};
use walkdir::WalkDir;

mod logging;

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
    #[arg(long, short, help = "Print verbose log messages")]
    verbose: bool,
    #[arg(
        long,
        short = 'd',
        default_value_t = 255,
        value_name = "0..255",
        help = "Max depth of directory traversal"
    )]
    max_depth: u8,
}

macro_rules! EXIT {
    () => {{
        return Ok(());
    }};
    ($($arg:tt)*) => {{
        eprintln!($($arg)*);
        return Ok(());
    }};
}

fn main() -> io::Result<()> {
    let mut args: Args = Parser::parse();
    let input = PathBuf::from(&args.input);

    if args.quiet {
        logging::set_level(logging::LogLevel::Error);
    } else if args.verbose {
        logging::set_level(logging::LogLevel::Verbose);
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

        process_file(&input, &ouput, &args)?;
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

        for entry in WalkDir::new(&input)
            .max_depth(args.max_depth as usize + 1)
            .into_iter()
            .filter_map(|e| e.ok())
        {
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

            process_file(p, &ofile, &_args)?;
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

fn process_file(input: &Path, output: &Path, args: &Args) -> io::Result<()> {
    dbg!(&args);
    log!("Converting {:?} -> {:?} ...", input, output);
    return Ok(());
}
