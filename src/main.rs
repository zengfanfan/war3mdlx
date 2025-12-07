use bitflags::bitflags;
use byteorder::{BigEndian, LittleEndian, ReadBytesExt};
use clap::{ArgAction, Parser};
use derive_debug::Dbg;
use glam::{Vec2, Vec3, Vec4};
use paste::paste;
use pretty_hex::*;
use regex::Regex;
use std::collections::HashMap;
use std::fmt::{Debug as stdDebug, Display, Formatter as stdFormatter, Result as stdResult};
use std::fs;
use std::io::{Cursor, Error as ioError, Read};
use std::path::{Path, PathBuf};
use std::process::ExitCode;

use walkdir::WalkDir;
mod cli;
mod extends;
mod fields;
mod parser;
mod utils;
mod worker;

use cli::*;
use extends::*;
use fields::*;
use parser::*;
use utils::*;
use worker::*;

#[macro_export]
macro_rules! EXIT {
    () => {{ return Ok(()); }};
    ($($arg:tt)*) => {{ log!($($arg)*); EXIT!(); }};
}
#[macro_export]
macro_rules! EXIT1 {
    () => {{ return Ok(()); }};
    ($($arg:tt)*) => {{ return ERR!($($arg)*); }};
}

fn _main() -> Result<(), MyError> {
    let args = Args::init();
    let mut worker = Worker::init();
    args.execute(&mut worker)?;
    return worker.join();
}

fn main() -> ExitCode {
    if let Err(e) = _main() {
        elog!("{}", e);
        ExitCode::FAILURE
    } else {
        ExitCode::SUCCESS
    }
}
