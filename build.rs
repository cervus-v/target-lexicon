//! build.rs file to obtain the host information.

// Allow dead code in triple.rs and targets.rs for our purposes here.
#![allow(dead_code)]

use serde_json::Value;
use std::env;
use std::fs::File;
use std::io::{self, prelude::*, Write};
use std::path::{Path, PathBuf};
use std::str::FromStr;

extern crate serde_json;

// Include triple.rs and targets.rs so we can parse the TARGET environment variable.
mod triple {
    include!("src/triple.rs");
}
mod targets {
    include!("src/targets.rs");
}

// Stub out `ParseError` to minimally support triple.rs and targets.rs.
mod parse_error {
    #[derive(Debug)]
    pub enum ParseError {
        UnrecognizedArchitecture(String),
        UnrecognizedVendor(String),
        UnrecognizedOperatingSystem(String),
        UnrecognizedEnvironment(String),
        UnrecognizedBinaryFormat(String),
        UnrecognizedField(String),
        NoneWithoutBinaryFormat,
    }
}

use triple::{Endianness, PointerWidth, Triple};

fn main() {
    let out_dir =
        PathBuf::from(env::var("OUT_DIR").expect("The OUT_DIR environment variable must be set"));

    let target = env::var("TARGET").expect("The TARGET environment variable must be set");

    let pwd = env::var("PWD").expect("The PWD environment variable must be set");

    let triple = match Triple::from_str(&target) {
        Ok(triple) => {
            assert_eq!(target, triple.to_string(), "host is unrecognized");
            triple
        }
        Err(_) => {
            let mut file = File::open(Path::new(&pwd).join(&format!("{}.json", target))).expect("error opening target file");
            let mut json = String::new();
            file.read_to_string(&mut json)
                .expect("error reading target file");
            let v: Value = serde_json::from_str(&json).expect("error parsing target file as json");
            let target = v["llvm-target"]
                .as_str()
                .expect("error parsing \"llvm-target\" as a string");
            let triple = Triple::from_str(target).expect("error parsing host target");
            assert_eq!(
                target,
                triple.to_string(),
                "host is unrecognized (as defined in target json file)"
            );

            // Check that the JSON describes a known target configuration.
            //
            // Unfortunately, none of Rust's "arch", "os", "env", nor "vendor"
            // fields directly correspond to triple fields, so we can't easily
            // check them.
            if let Some(endian) = v["target-endian"].as_str() {
                assert_eq!(
                    endian,
                    match triple.endianness().unwrap() {
                        Endianness::Little => "little",
                        Endianness::Big => "big",
                    },
                    "\"target-endian\" field disagrees with the target triple"
                );
            }
            if let Some(pointer_width) = v["target-pointer-width"].as_str() {
                assert_eq!(
                    pointer_width,
                    match triple.pointer_width().unwrap() {
                        PointerWidth::U16 => "16",
                        PointerWidth::U32 => "32",
                        PointerWidth::U64 => "64",
                    },
                    "\"target-pointer-width\" field disagrees with the target triple"
                );
            }

            triple
        }
    };

    let out = File::create(out_dir.join("host.rs")).expect("error creating host.rs");
    write_host_rs(out, triple).expect("error writing host.rs");
}

fn write_host_rs(mut out: File, triple: Triple) -> io::Result<()> {
    writeln!(out, "/// The `Triple` of the current host.")?;
    writeln!(out, "pub static HOST: Triple = Triple {{")?;
    writeln!(
        out,
        "    architecture: Architecture::{:?},",
        triple.architecture
    )?;
    writeln!(out, "    vendor: Vendor::{:?},", triple.vendor)?;
    writeln!(
        out,
        "    operating_system: OperatingSystem::{:?},",
        triple.operating_system
    )?;
    writeln!(
        out,
        "    environment: Environment::{:?},",
        triple.environment
    )?;
    writeln!(
        out,
        "    binary_format: BinaryFormat::{:?},",
        triple.binary_format
    )?;
    writeln!(out, "}};")?;
    writeln!(out)?;

    writeln!(out, "impl Architecture {{")?;
    writeln!(out, "    /// Return the architecture for the current host.")?;
    writeln!(out, "    pub fn host() -> Self {{")?;
    writeln!(out, "        Architecture::{:?}", triple.architecture)?;
    writeln!(out, "    }}")?;
    writeln!(out, "}}")?;
    writeln!(out)?;

    writeln!(out, "impl Vendor {{")?;
    writeln!(out, "    /// Return the vendor for the current host.")?;
    writeln!(out, "    pub fn host() -> Self {{")?;
    writeln!(out, "        Vendor::{:?}", triple.vendor)?;
    writeln!(out, "    }}")?;
    writeln!(out, "}}")?;
    writeln!(out)?;

    writeln!(out, "impl OperatingSystem {{")?;
    writeln!(
        out,
        "    /// Return the operating system for the current host."
    )?;
    writeln!(out, "    pub fn host() -> Self {{")?;
    writeln!(
        out,
        "        OperatingSystem::{:?}",
        triple.operating_system
    )?;
    writeln!(out, "    }}")?;
    writeln!(out, "}}")?;
    writeln!(out)?;

    writeln!(out, "impl Environment {{")?;
    writeln!(out, "    /// Return the environment for the current host.")?;
    writeln!(out, "    pub fn host() -> Self {{")?;
    writeln!(out, "        Environment::{:?}", triple.environment)?;
    writeln!(out, "    }}")?;
    writeln!(out, "}}")?;
    writeln!(out)?;

    writeln!(out, "impl BinaryFormat {{")?;
    writeln!(
        out,
        "    /// Return the binary format for the current host."
    )?;
    writeln!(out, "    pub fn host() -> Self {{")?;
    writeln!(out, "        BinaryFormat::{:?}", triple.binary_format)?;
    writeln!(out, "    }}")?;
    writeln!(out, "}}")?;
    writeln!(out)?;

    writeln!(out, "impl Triple {{")?;
    writeln!(out, "    /// Return the triple for the current host.")?;
    writeln!(out, "    pub fn host() -> Self {{")?;
    writeln!(out, "        Self {{")?;
    writeln!(
        out,
        "            architecture: Architecture::{:?},",
        triple.architecture
    )?;
    writeln!(out, "            vendor: Vendor::{:?},", triple.vendor)?;
    writeln!(
        out,
        "            operating_system: OperatingSystem::{:?},",
        triple.operating_system
    )?;
    writeln!(
        out,
        "            environment: Environment::{:?},",
        triple.environment
    )?;
    writeln!(
        out,
        "            binary_format: BinaryFormat::{:?},",
        triple.binary_format
    )?;
    writeln!(out, "        }}")?;
    writeln!(out, "    }}")?;
    writeln!(out, "}}")?;

    Ok(())
}
