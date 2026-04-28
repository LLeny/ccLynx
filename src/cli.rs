use clap::{Parser, ValueEnum};

use cc6502::Args;

#[derive(Parser, Debug)]
#[command(author, about = "ccLynx - a subset of C compiler for Atari Lynx cartridges")]
pub(crate) struct CliArgs {
    /// Input file name
    #[arg(default_value = "stdin")]
    pub input: String,

    /// Preprocessor definitions
    #[arg(short = 'D')]
    pub defines: Vec<String>,

    /// Optimization level
    #[arg(short = 'O', default_value = "1", value_parser = clap::value_parser!(u8).range(0..=3))]
    pub optimization_level: u8,

    /// Activate verbose output
    #[arg(short, long, default_value = "false")]
    pub verbose: bool,

    /// Set verbosity level
    #[arg(short = 'V', long, default_value = "0")]
    pub verbosity: u8,

    /// Include directories
    #[arg(short = 'I')]
    pub include_directories: Vec<String>,

    /// Warning directives
    #[arg(short = 'W')]
    pub warnings: Vec<String>,

    /// Output file name
    #[arg(short, long, default_value = "a.out")]
    pub output: String,

    /// Insert C code as comments
    #[arg(long, default_value = "false")]
    pub insert_code: bool,

    /// Set char signedness to signed
    #[arg(long("fsigned_char"), default_value = "false")]
    pub signed_chars: bool,

    /// Set char signedness to unsigned (default)
    #[arg(long("funsigned_char"), default_value = "true")]
    pub unsigned_chars: bool,

    /// Stop after the stage of compilation proper; do not assemble
    #[arg(short = 'S', default_value = "false")]
    pub assembler_output: bool,

    /// Generate debug information
    #[arg(short = 'g', long, default_value = "false")]
    pub debug: bool,

    /// Print compiler version
    #[arg(long, default_value = "false")]
    pub version: bool,

    /// Cart header version
    #[arg(long = "cart-version", default_value_t = 1)]
    pub cart_version: u16,

    /// Cart name/title
    #[arg(long, default_value = "none")]
    pub name: String,

    /// Cart author
    #[arg(long, default_value = "none")]
    pub author: String,

    /// Block size for the cartridge
    #[arg(long = "blocksize", default_value_t = 1024)]
    pub block_size: usize,

    /// Cart rotation mode
    #[arg(long, value_enum, default_value_t = CartRotationArg::None)]
    pub rotation: CartRotationArg,

    /// Cart aud
    #[arg(long, default_value_t = 0)]
    pub aud: u8,

    /// Cart EEPROM type
    #[arg(long, value_enum, default_value_t = CartEepromArg::None)]
    pub eeprom: CartEepromArg,
}

#[derive(Copy, Clone, Debug, ValueEnum)]
pub(crate) enum CartRotationArg {
    None,
    Left,
    Right,
}

#[derive(Copy, Clone, Debug, ValueEnum)]
pub(crate) enum CartEepromArg {
    None,
    #[value(name = "93C46_16")]
    C93C46_16,
    #[value(name = "93C56_16")]
    C93C56_16,
    #[value(name = "93C66_16")]
    C93C66_16,
    #[value(name = "93C76_16")]
    C93C76_16,
    #[value(name = "93C86_16")]
    C93C86_16,
    #[value(name = "93C46_8")]
    C93C46_8,
    #[value(name = "93C56_8")]
    C93C56_8,
    #[value(name = "93C66_8")]
    C93C66_8,
    #[value(name = "93C76_8")]
    C93C76_8,
    #[value(name = "93C86_8")]
    C93C86_8,
}

impl CliArgs {
    pub fn to_old_args(&self) -> Args {
        let mut argv = vec!["ccLynx".to_string(), self.input.clone()];

        for define in &self.defines {
            argv.push("-D".to_string());
            argv.push(define.clone());
        }

        argv.push("-D".to_string());
        argv.push(format!("DBLOCKSIZE={}", self.block_size));

        argv.push("-O".to_string());
        argv.push(self.optimization_level.to_string());

        if self.verbose {
            argv.push("--verbose".to_string());
        }

        argv.push("-V".to_string());
        argv.push(self.verbosity.to_string());

        for inc in &self.include_directories {
            argv.push("-I".to_string());
            argv.push(inc.clone());
        }

        for warning in &self.warnings {
            argv.push("-W".to_string());
            argv.push(warning.clone());
        }

        argv.push("-o".to_string());
        argv.push(self.output.clone());

        if self.insert_code {
            argv.push("--insert-code".to_string());
        }
        if self.signed_chars {
            argv.push("--fsigned_char".to_string());
        }
        if !self.unsigned_chars {
            argv.push("--funsigned_char=false".to_string());
        }
        if self.assembler_output {
            argv.push("-S".to_string());
        }
        if self.debug {
            argv.push("-g".to_string());
        }
        if self.version {
            argv.push("--version".to_string());
        }

        Args::parse_from(argv)
    }
}

impl From<CartRotationArg> for crate::builder::header::CartRotation {
    fn from(rotation: CartRotationArg) -> Self {
        match rotation {
            CartRotationArg::None => crate::builder::header::CartRotation::None,
            CartRotationArg::Left => crate::builder::header::CartRotation::Left,
            CartRotationArg::Right => crate::builder::header::CartRotation::Right,
        }
    }
}

impl From<CartEepromArg> for crate::builder::header::CartEeprom {
    fn from(eeprom: CartEepromArg) -> Self {
        match eeprom {
            CartEepromArg::None => crate::builder::header::CartEeprom::None,
            CartEepromArg::C93C46_16 => crate::builder::header::CartEeprom::_93C46_16,
            CartEepromArg::C93C56_16 => crate::builder::header::CartEeprom::_93C56_16,
            CartEepromArg::C93C66_16 => crate::builder::header::CartEeprom::_93C66_16,
            CartEepromArg::C93C76_16 => crate::builder::header::CartEeprom::_93C76_16,
            CartEepromArg::C93C86_16 => crate::builder::header::CartEeprom::_93C86_16,
            CartEepromArg::C93C46_8 => crate::builder::header::CartEeprom::_93C46_8,
            CartEepromArg::C93C56_8 => crate::builder::header::CartEeprom::_93C56_8,
            CartEepromArg::C93C66_8 => crate::builder::header::CartEeprom::_93C66_8,
            CartEepromArg::C93C76_8 => crate::builder::header::CartEeprom::_93C76_8,
            CartEepromArg::C93C86_8 => crate::builder::header::CartEeprom::_93C86_8,
        }
    }
}
