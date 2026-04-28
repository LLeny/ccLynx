use std::{fs::File, io::{Read, Write}, path::Path};
use cc6502::{compile::CompilerState, generate::GeneratorState};
use std::io::Error;

use crate::builder::consts::write_block_size;

#[repr(u8)]
#[derive(Default, Clone, Copy)]
#[allow(dead_code)]
pub(crate) enum CartRotation {
    #[default]
    None = 0,
    Left = 1,
    Right = 2,
}

#[repr(u8)]
#[derive(Default, Clone, Copy)]
pub(crate) enum CartEeprom {
    #[default]
    None = 0,
    _93C46_16 = 1,
    _93C56_16 = 2,
    _93C66_16 = 3,
    _93C76_16 = 4,
    _93C86_16 = 5,
    _93C46_8 = 0x81,
    _93C56_8 = 0x82,
    _93C66_8 = 0x83,
    _93C76_8 = 0x84,
    _93C86_8 = 0x85,
}

pub(crate) struct CartHeader<'a> {
    pub(crate) bank0_block_size: u16,
    pub(crate) bank1_block_size: u16,
    pub(crate) version: u16,
    pub(crate) name: &'a str,
    pub(crate) author: &'a str,
    pub(crate) rotation: CartRotation,
    pub(crate) aud: u8,
    pub(crate) eeprom: CartEeprom,
    pub(crate) spare: [u8; 3],
}

impl CartHeader<'_> {
    pub(crate) fn to_bytes(&self) -> Vec<u8> {
        let mut data = Vec::with_capacity(64);

        data.extend_from_slice("LYNX".as_bytes());
        data.extend_from_slice(&self.bank0_block_size.to_le_bytes());
        data.extend_from_slice(&self.bank1_block_size.to_le_bytes());
        data.extend_from_slice(&self.version.to_le_bytes());

        let mut buf = [0u8; 32];
        buf[..self.name.len()].copy_from_slice(self.name.as_bytes());
        data.extend_from_slice(&buf);
        let mut buf = [0u8; 16];
        buf[..self.author.len()].copy_from_slice(self.author.as_bytes());
        data.extend_from_slice(&buf);
        data.push(self.rotation as u8);
        data.push(self.aud);
        data.push(self.eeprom as u8);
        data.extend_from_slice(&self.spare);

        data
    }
}

pub(crate) fn create_cart_header(args: &'_ crate::CliArgs) -> Result<CartHeader<'_>, Error> {
    Ok(CartHeader {
        bank0_block_size: 0,
        bank1_block_size: 0,
        version: args.cart_version,
        name: &args.name,
        author: &args.author,
        rotation: args.rotation.into(),
        aud: args.aud,
        eeprom: args.eeprom.into(),
        spare: [0u8; 3],
    })
}

pub(crate) fn prepend_lnx_header(rom: &Path, header: &CartHeader) -> Result<(), Error> {
    let mut header_data = header.to_bytes();

    let mut cart =  File::open(rom)?;
    cart.read_to_end(&mut header_data)?;

    let mut cart = File::create(rom)?;
    cart.write_all(header_data.as_slice())?;
    
    Ok(())
}

pub(crate) fn write_code_header(gstate: &mut GeneratorState, compiler_state: &CompilerState) -> Result<(), Error> {
    let _ = write_block_size(gstate, compiler_state);
    Ok(())
}
