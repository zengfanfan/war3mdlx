use byteorder::{BigEndian, LittleEndian, ReadBytesExt};
use clap::{ArgAction, Parser};
use derive_debug::Dbg;
use glam::{Vec2, Vec3, Vec4};
use paste::paste;
use pretty_hex::*;
use std::fmt::Debug as stdDebug;
use std::fmt::Display;
use std::io::{Cursor, Error as ioError, Read};
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

mod cli;
mod extends;
mod fields;
mod parser;
mod utils;

use extends::*;
use fields::*;
use parser::*;
use utils::*;

fn main() {
    if let Err(e) = cli::Args::new().execute() {
        elog!("{}", e);
        std::process::exit(1);
    }
}
