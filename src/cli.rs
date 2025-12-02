use crate::*;

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
    #[arg(long, help = "Convert *.mdl to *.mdx")]
    pub mdl2x: bool,
    #[arg(long, help = "Convert *.mdx to *.mdl")]
    pub mdx2l: bool,
    #[arg(long, short = 'F', help = "Put output files in one directory and ignore hierarchy")]
    pub flat: bool,
    #[arg(long, short = 'f', help = "Overwrite existing output files (default: skip)")]
    pub overwrite: bool,
    #[arg(long, short = 'E', help = "Stop walking the directory hierarchy when an error occurs")]
    pub stop_on_error: bool,
    #[arg(long, short, help = "Do not print log messages")]
    pub quiet: bool,
    #[arg(long, short, action = ArgAction::Count, help = "Print verbose log messages (-vv very verbose)")]
    pub verbose: u8,
    #[arg(long, short = 'd', default_value_t = 255, value_name = "0..255", help = "Max depth of directory traversal")]
    pub max_depth: u8,
}

impl Args {
    pub fn new() -> Self {
        let this = Self::parse();
        this.set_log_level();
        return this;
    }

    fn set_log_level(&self) {
        if self.quiet {
            set_log_level(LogLevel::Warn);
        } else {
            match self.verbose {
                1 => set_log_level(LogLevel::Verbose),
                2 => set_log_level(LogLevel::Verbose2),
                3.. => set_log_level(LogLevel::Verbose3),
                _ => (),
            }
        }
    }

    pub fn execute(&self) -> Result<(), MyError> {
        let input = PathBuf::from(&self.input);
        match self.check_input(&input) {
            CheckResult::ExpectFileDir => ERR!("Not a file or directory: {}", self.input),
            CheckResult::ExpectMDLX => ERR!("Invalid input path: {}, expect *.mdl or *.mdx", self.input),
            CheckResult::ExpectMDL => ERR!("Invalid input path: {}, expect *.mdl", self.input),
            CheckResult::ExpectMDX => ERR!("Invalid input path: {}, expect *.mdx", self.input),
            CheckResult::Ok => match input.ext_lower().as_str() {
                ext @ ("mdl" | "mdx") => self.handle_file(&input, ext),
                _ => self.handle_dir(&input),
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

    fn handle_file(&self, input: &Path, inext: &str) -> Result<(), MyError> {
        let mut ouput = match &self.output {
            Some(o) => PathBuf::from(o),
            None => input.with_extension(self.guess_output_ext(inext)),
        };

        let opath = ouput.display().to_string();
        match self.check_output(&ouput) {
            CheckResult::ExpectFileDir => EXIT!("Not a file or directory: {}", opath),
            CheckResult::ExpectMDLX => EXIT!("Invalid ouput path: {}, expect *.mdl or *.mdx", opath),
            CheckResult::ExpectMDL => EXIT!("Invalid ouput path: {}, expect *.mdl", opath),
            CheckResult::ExpectMDX => EXIT!("Invalid ouput path: {}, expect *.mdx", opath),
            _ok => ouput = if ouput.is_dir() { ouput.join(input.file_name().unwrap()) } else { ouput },
        };

        if ouput.exists() && !self.overwrite {
            log!("Skipped existing file: {:?}", ouput);
            EXIT!();
        }

        return self.process_file(&input, &ouput);
    }

    fn handle_dir(&self, input: &Path) -> Result<(), MyError> {
        let output = self.output.as_ref().and_then(|o| Some(PathBuf::from(o))).unwrap_or(input.to_path_buf());
        if !output.is_dir() {
            EXIT!("Output path should be also a directory: {:?}", self.output);
        }

        let (mut ok, mut error, mut skipped) = (0, 0, 0);
        for entry in WalkDir::new(&input).max_depth(self.max_depth as usize + 1).into_iter().filter_map(|e| e.ok()) {
            let ifile = entry.path();
            if !ifile.is_file() || !matches!(self.check_input(ifile), CheckResult::Ok) {
                continue;
            }

            let ofile = match self.flat {
                true => output.join(ifile.file_name().unwrap()),
                false => output.join(ifile.strip_prefix(&input).unwrap()),
            }
            .with_extension(self.guess_output_ext(ifile.ext_lower().as_str()));

            if ofile.exists() && !self.overwrite {
                log!("Skipped existing file: {:?}", ofile);
                skipped += 1;
                continue;
            }

            if let Err(err) = self.process_file(ifile, &ofile) {
                error += 1;
                elog!("{}", err);
                yes!(self.stop_on_error, break);
            } else {
                ok += 1
            };
        }

        println!(
            "Converted {ok} files{}{}.",
            yesno!(skipped > 0, format!(", {skipped} skipped"), "".into()),
            yesno!(error > 0, format!(", {error} errors"), "".into()),
        );
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

    fn process_file(&self, input: &Path, output: &Path) -> Result<(), MyError> {
        log!("Converting {:?} -> {:?} ...", input, output);
        return MdlxData::read(input)?.write(output);
    }
}
