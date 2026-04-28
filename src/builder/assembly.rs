use cc6502::{compile::CompilerState, error::Error, generate::GeneratorState};

pub(crate) fn write_assembly(
    gstate: &mut GeneratorState,
    compiler_state: &CompilerState,
    bank: u32,
) -> Result<(), Error> {
    for asm in compiler_state
        .included_assembler
        .iter()
        .filter(|a| a.3.unwrap_or(0) == bank)
    {
        gstate.write(&format!("{}\n", asm.0))?;
    }
    Ok(())
}
