//! The SingleStepTests Z80 test vectors (<https://github.com/SingleStepTests/z80>),
//! for emu_lib's tests. Use it only as a dev-dependency: it brings 1.6 GB of data.

/// The directory holding one JSON file per opcode ("00.json", "ed 4d.json",
/// "dd cb __ 06.json", ...), each with 1000 test cases. Reading the files from
/// here at runtime keeps them out of the test binary; turn off the default
/// `embed` feature to skip embedding them too.
pub const Z80_V1_DIR: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/z80/v1");

#[cfg(feature = "embed")]
mod embedded {
    use include_directory::{include_directory, Dir};

    const Z80_TESTS: Dir<'_> = include_directory!("z80/v1");

    /// A test file's contents, compiled into the binary.
    pub fn get_z80_tests(name: &str) -> Option<&'static str> {
        Z80_TESTS.get_file(name).map(|f| f.contents_utf8().unwrap())
    }
}

#[cfg(feature = "embed")]
pub use embedded::get_z80_tests;
