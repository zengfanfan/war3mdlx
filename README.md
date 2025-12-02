# war3mdlx

A command line tool to convert warcraft 3 model file: *.mdl <-> *.mdx

## Usage

```bash
# convert mdl to mdx (omit output path)
$ war3mdlx input.mdl
# convert mdx to mdl
$ war3mdlx input.mdx output.mdl
# convert mdx to mdl (walk through directory hierarchy)
$ war3mdlx --mdx2l input/path output/path
# all available options
$ war3mdlx -h
Usage: war3mdlx [OPTIONS] <INPUT>

Options:
      --mdl2x               Convert *.mdl to *.mdx
      --mdx2l               Convert *.mdx to *.mdl
  -F, --flat                Put output files in one directory and ignore hierarchy
  -f, --overwrite           Overwrite existing output files (default: skip)
  -E, --stop-on-error       Stop walking the directory hierarchy when an error occurs
  -q, --quiet               Do not print log messages
  -v, --verbose...          Print verbose log messages (-vv very verbose)
  -d, --max-depth <0..255>  Max depth of directory traversal [default: 255]
  -h, --help                Print help
  -V, --version             Print version
```

## Build

When you are in the root directory of the project, run the following command to build the binary:
```bash
$ cargo build --release
```

> This is a Rust project, so you need to install [rust and cargo](https://rust-lang.org/tools/install/) to build it.
