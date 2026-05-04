use crate::builder::bank::write_bank_directory;
use crate::builder::boot_loader::write_boot_loader;
use crate::builder::functions::{compute_functions_call_tree, optimize_functions};
use crate::builder::header::write_code_header;
use crate::builder::variables::write_zeropage;
use cc6502::{
    Args,
    compile::{CompilerState, VariableMemory},
    generate::GeneratorState,
};
use std::io::Write;
use std::path::Path;

mod assembly;
mod bank;
mod boot_loader;
mod compiler;
mod consts;
mod functions;
pub(crate) mod header;
mod variables;

pub fn build_cartridge(
    compiler_state: &CompilerState,
    writer: &mut dyn Write,
    args: &Args,
) -> Result<(), cc6502::error::Error> {
    let mut gstate = GeneratorState::new(
        compiler_state,
        writer,
        args.insert_code,
        args.warnings.clone(),
        "",
    );

    let mut nmi_interrupt = "NMI".to_string();
    let num_banks = get_num_banks(compiler_state);

    if let Some(interrupt) =
        optimize_functions(&mut gstate, compiler_state, args.optimization_level)?
    {
        nmi_interrupt = interrupt;
    }
    gstate.compute_functions_actually_in_use()?;
    compute_functions_call_tree(&gstate, compiler_state, &nmi_interrupt);

    let mut banks_dest = vec![200u16; num_banks as usize + 1];
    for (bank, org) in &compiler_state.bank_orgs {
        banks_dest[*bank as usize] = *org;
    }

    create_source_file(compiler_state, args, gstate, &banks_dest)?;

    let src = Path::new(crate::TEMP_SOURCE);
    let dst = Path::new(&args.output);
    compile_source_file(src, dst)?;
    let _ = std::fs::remove_file(src);

    Ok(())
}

fn compile_source_file(source: &Path, destination: &Path) -> Result<(), std::io::Error> {
    compiler::compile_source_file(source, destination)?;
    Ok(())
}

fn create_source_file(
    compiler_state: &CompilerState<'_>,
    args: &Args,
    mut gstate: GeneratorState,
    banks_dest: &[u16],
) -> Result<(), cc6502::error::Error> {
    let zp_len = write_zeropage(&mut gstate, compiler_state)?;
    assert!(zp_len <= 0xFF, "Zeropage variables total size is {zp_len} bytes, which exceeds the 255 bytes limit");
    write_code_header(&mut gstate, compiler_state);
    write_boot_loader(&mut gstate)?;
    write_bank_directory(&mut gstate, banks_dest)?;
    for (i, dest) in banks_dest.iter().enumerate() {
        bank::write_bank(&mut gstate, compiler_state, args, i as u32, *dest)?;
    }
    Ok(())
}

fn get_num_banks(compiler_state: &CompilerState) -> u32 {
    std::cmp::max(
        compiler_state
            .sorted_functions()
            .iter()
            .map(|f| f.1.bank)
            .max()
            .unwrap_or(0),
        compiler_state
            .sorted_variables()
            .iter()
            .filter_map(|v| match v.1.memory {
                VariableMemory::Bank(b) => Some(b),
                VariableMemory::Zeropage => None,
            })
            .max()
            .unwrap_or(0),
    )
}
