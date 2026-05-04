use crate::builder::{
    assembly::write_assembly, consts::write_consts, functions::write_functions,
    variables::write_non_zeropage,
};
use cc6502::{compile::CompilerState, error::Error, generate::GeneratorState};

pub(crate) fn write_bank_directory(
    gstate: &mut GeneratorState,
    bank_dests: &[u16],
) -> Result<(), Error> {
    // Directory: block(1), block offset (2), destination(2), length(2)
    gstate.write("DIRECTORY EQU ROM_ADDR\n")?;
    gstate.write("run 0\n")?;
    gstate.write("DIRECTORY_START:\n")?;
    for (i, bank_dest) in bank_dests.iter().enumerate() {
        gstate.write(&format!("dc.b BANK{i}_ADDR/BLOCKSIZE\n"))?;
        gstate.write(&format!("dc.b <(BANK{i}_ADDR&(BLOCKSIZE-1))\n"))?;
        gstate.write(&format!("dc.b >(BANK{i}_ADDR&(BLOCKSIZE-1))\n"))?;
        gstate.write(&format!("dc.b <{bank_dest}\n"))?;
        gstate.write(&format!("dc.b >{bank_dest}\n"))?;
        gstate.write(&format!("dc.b <BANK{i}_LEN\n"))?;
        gstate.write(&format!("dc.b >BANK{i}_LEN\n"))?;
        gstate.write("dc.b 0\n")?;
    }
    gstate.write("DIRECTORY_END:\n")?;
    gstate.write("ROM_ADDR set ROM_ADDR + (DIRECTORY_END-DIRECTORY_START)\n")?;
    Ok(())
}

pub(crate) fn write_bank(
    gstate: &mut GeneratorState,
    compiler_state: &CompilerState,
    args: &cc6502::Args,
    bank: u32,
    destination: u16,
) -> Result<(), Error> {
    if bank == 0 {
        write_consts(gstate, compiler_state)?;
    }

    gstate.write(&format!("BANK{bank}_ADDR equ ROM_ADDR\n"))?;
    gstate.write(&format!("run ${destination:X}\n\n"))?;
    gstate.write(&format!("BANK{bank}_START:\n"))?;

    write_functions(gstate, compiler_state, args, bank)?;
    write_assembly(gstate, compiler_state, bank)?;
    write_non_zeropage(gstate, compiler_state, bank)?;

    gstate.write(&format!("BANK{bank}_END:\n"))?;
    gstate.write(&format!(
        "BANK{bank}_LEN: EQU BANK{bank}_END-BANK{bank}_START\n"
    ))?;

    gstate.write(&format!("ROM_ADDR set ROM_ADDR + BANK{bank}_LEN\n"))?;

    gstate.write(&format!(
        "echo \"Bank {bank} start:%HBANK{bank}_ADDR len:%HBANK{bank}_LEN\"\n"
    ))?;

    Ok(())
}
