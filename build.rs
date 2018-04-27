//! build.rs file to obtain the host information.

// Allow dead code in triple.rs and targets.rs for our purposes here.
#![allow(dead_code)]

use std::env;
use std::fs::File;
use std::io::{self, Write};
use std::path::PathBuf;
use std::str::FromStr;

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

use triple::Triple;

fn main() {
    let out_dir = PathBuf::from(env::var("OUT_DIR").expect(
        "The OUT_DIR environment variable must be set",
    ));

    let target = env::var("TARGET").expect("The TARGET environment variable must be set");
    let triple = Triple::from_str(&target).expect("can't parse host target");
    assert_eq!(target, triple.to_string(), "host is unrecognized");

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
