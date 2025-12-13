use crate::*;
use clap::{ArgAction, Parser};

enum CheckResult {
    Ok,
    ExpectFileDir,
    ExpectMDL,
    ExpectMDX,
    ExpectMDLX,
}

#[derive(Debug, Parser, Clone)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[arg(hide = true)]
    pub input: String,
    #[arg(hide = true)]
    pub output: Option<String>,

    #[arg(long, short = '1', help = "Convert *.mdl to *.mdx")]
    pub mdl2x: bool,
    #[arg(long, short = '2', help = "Convert *.mdx to *.mdl")]
    pub mdx2l: bool,
    #[arg(long, short = 'B', help = "Swap color components when needed to make sure they are in RGB order in mdl files [default: as-is]")]
    pub mdl_rgb: bool,

    #[arg(long, short = 'F', help = "Put output files in one directory and ignore hierarchy")]
    pub flat: bool,
    #[arg(long, short = 'f', help = "Overwrite existing output files [default: skip]")]
    pub overwrite: bool,
    #[arg(long, short = 'e', help = "Stop walking the directory hierarchy when an error occurs")]
    pub stop_on_error: bool,
    #[arg(long, short = 'd', default_value_t = 255, value_name = "0..255", help = "Max depth of directory traversal")]
    pub max_depth: u8,

    #[arg(
        long,
        short = 'p',
        default_value_t = 4,
        value_parser = clap::value_parser!(i8).range(0..=10),
        value_name = "0..10",
        help = "Max precision of decimal numbers when converted to text",
    )]
    pub precision: i8,
    #[arg(
        long,
        short = 'n',
        value_name = "CR|LF|CRLF",
        value_parser = ["CR", "LF", "CRLF"],
        default_value = "CRLF",
        help = "Used when writing text files",
    )]
    pub line_ending: String,
    #[arg(
        long,
        short = 'i',
        value_name = "Ns|Nt",
        value_parser = validate_indent,
        default_value = "1t",
        help = "Used when writing text files (e.g. 1t: one tab, 4s: four spaces)",
    )]
    pub indent: String,
    #[arg(skip)]
    pub indents: HashMap<u8, String>,

    #[arg(long, short, help = "Do not print log messages")]
    pub quiet: bool,
    #[arg(long, short, action = ArgAction::Count, help = "Print verbose log messages (-vv very verbose)")]
    pub verbose: u8,
    #[arg(skip)]
    pub log_level: LogLevel,
}

fn validate_indent(s: &str) -> Result<String, String> {
    let re = Regex::new(r"^[0-9]+[st]$").unwrap();
    if re.is_match(s) { Ok(s.to_string()) } else { Err("must be digits followed by 's' or 't'".into()) }
}

impl Args {
    pub fn init() -> &'static Self {
        Self::set_instance(Self::parse())
    }

    pub fn execute(&self, worker: &mut Worker) -> Result<(), MyError> {
        let input = PathBuf::from(&self.input);
        match self.check_input(&input) {
            CheckResult::ExpectFileDir => EXIT1!("Not a file or directory: {:?}", input),
            CheckResult::ExpectMDLX => EXIT1!("Invalid input: {:?}, expect *.mdl or *.mdx", input),
            CheckResult::ExpectMDL => EXIT1!("Invalid input: {:?}, expect *.mdl", input),
            CheckResult::ExpectMDX => EXIT1!("Invalid input: {:?}, expect *.mdx", input),
            CheckResult::Ok => match input.ext_lower().as_str() {
                ext @ ("mdl" | "mdx") => self.handle_file(worker, input, ext),
                _ => self.handle_dir(worker, input),
            },
        }
    }

    fn guess_output_ext(&self, inext: &str) -> &str {
        if self.mdl2x {
            "mdx"
        } else if self.mdx2l {
            "mdl"
        } else if inext == "mdl" {
            "mdx"
        } else {
            "mdl"
        }
    }

    fn handle_file(&self, worker: &mut Worker, input: PathBuf, inext: &str) -> Result<(), MyError> {
        let mut output = match &self.output {
            Some(o) => PathBuf::from(o),
            None => input.with_extension(self.guess_output_ext(inext)),
        };

        let opath = output.display().to_string();
        match self.check_output(&output) {
            CheckResult::ExpectFileDir => EXIT1!("Not a file or directory: {}", opath),
            CheckResult::ExpectMDLX => EXIT1!("Invalid path: {}, expect *.mdl or *.mdx", opath),
            CheckResult::ExpectMDL => EXIT1!("Invalid path: {}, expect *.mdl", opath),
            CheckResult::ExpectMDX => EXIT1!("Invalid path: {}, expect *.mdx", opath),
            _ok => output = if output.is_dir() { output.join(input.file_name().unwrap()) } else { output },
        };

        yes!(input.same_as(&output), EXIT!("Input and output are the same, do nothing."));

        self.process_file(worker, input, output);
        EXIT!();
    }

    fn handle_dir(&self, worker: &mut Worker, input: PathBuf) -> Result<(), MyError> {
        let output = self.output.as_ref().and_then(|o| Some(PathBuf::from(o))).unwrap_or(input.to_path_buf());
        if !output.is_dir() {
            EXIT1!("Output should be also a directory: {}", output.fmtx());
        }

        for entry in WalkDir::new(&input).max_depth(self.max_depth as usize + 1).into_iter().filter_map(|e| e.ok()) {
            let ifile = entry.into_path();
            if !ifile.is_file() || !matches!(self.check_input(&ifile), CheckResult::Ok) {
                continue;
            }

            let ofile = match self.flat {
                true => output.join(ifile.file_name().unwrap()),
                false => output.join(ifile.strip_prefix(&input).unwrap()),
            }
            .with_extension(self.guess_output_ext(ifile.ext_lower().as_str()));

            self.process_file(worker, ifile, ofile);
        }

        EXIT!();
    }

    fn check_input(&self, path: &Path) -> CheckResult {
        let ext = path.ext_lower();
        if path.is_dir() {
            CheckResult::Ok
        } else if path.is_file() {
            if self.mdl2x && ext != "mdl" {
                CheckResult::ExpectMDL
            } else if self.mdx2l && ext != "mdx" {
                CheckResult::ExpectMDX
            } else if ext != "mdl" && ext != "mdx" {
                CheckResult::ExpectMDLX
            } else {
                CheckResult::Ok
            }
        } else {
            CheckResult::ExpectFileDir
        }
    }

    fn check_output(&self, path: &Path) -> CheckResult {
        let ext = path.ext_lower();
        if path.is_dir() {
            CheckResult::Ok
        } else {
            if self.mdl2x && ext != "mdx" {
                CheckResult::ExpectMDX
            } else if self.mdx2l && ext != "mdl" {
                CheckResult::ExpectMDL
            } else if ext != "mdl" && ext != "mdx" {
                CheckResult::ExpectMDLX
            } else {
                CheckResult::Ok
            }
        }
    }

    fn process_file(&self, worker: &mut Worker, input: PathBuf, output: PathBuf) {
        if output.exists() && !self.overwrite {
            log!("Skipped existing output: {}", output.fmtx());
            worker.skip_job();
        } else {
            log!("Converting {} -> {} ...", input.fmtx(), output.fmtx());
            worker.add_job(input, output);
        }
    }
}

//#region [global] args

macro_rules! getter {
    ($($field:ident),+) => {
        $(
            #[macro_export]
            macro_rules! $field {
                () => {
                    &crate::cli::Args::instance().$field
                };
            }
        )+
    };
}
#[macro_export]
macro_rules! indent {
    () => {
        &crate::cli::Args::instance().indent
    };
    ($depth:expr) => {
        crate::cli::Args::instance().indent(&$depth)
    };
}

getter!(log_level, line_ending, precision, stop_on_error, overwrite, start_time, mdl_rgb);

static mut G_ARGS: Option<&'static Args> = None;

impl Args {
    pub fn instance() -> &'static Self {
        unsafe { G_ARGS.expect("Args not initialized! [impossible]") }
    }
    fn set_instance(mut args: Self) -> &'static Self {
        args.set_log_level();
        args.set_line_ending();
        args.set_indent();
        /* not thread-safe: make sure it is in main thread and before creating any other thread */
        no!(Worker::is_main(), panic!("{} must be initialized in main thread!", TNAMEL!()));
        let boxed: Box<Self> = Box::new(args);
        let static_ref: &'static Self = Box::leak(boxed);
        unsafe {
            G_ARGS = Some(static_ref);
            return static_ref;
        }
    }

    fn set_log_level(&mut self) {
        self.log_level = yesno!(
            self.quiet,
            LogLevel::Warn,
            match self.verbose {
                1 => LogLevel::Verbose,
                2 => LogLevel::Verbose2,
                3.. => LogLevel::Verbose3,
                _ => LogLevel::default(),
            }
        );
    }

    fn set_indent(&mut self) {
        let len = self.indent.len();
        let n = &self.indent[..len - 1];
        let t = self.indent.as_bytes()[len - 1] as char;
        self.indent = yesno!(t == 't', '\t', ' ').to_string().repeat(n.parse().unwrap());
        for i in 0..10 {
            self.indents.insert(i, self.indent.repeat(i as usize));
        }
    }
    pub fn indent(&self, depth: &u8) -> &str {
        match depth {
            0 => "",
            1 => &self.indent,
            _ => self.indents.get(depth).expect(&F!("Invalid indent depth: {depth}")),
        }
    }

    fn set_line_ending(&mut self) {
        self.line_ending = match self.line_ending.as_str() {
            "CR" => "\r",
            "LF" => "\n",
            "CRLF" => "\r\n",
            _ => "",
        }
        .to_string();
    }
}

//#endregion
