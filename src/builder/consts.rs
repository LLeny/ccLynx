use cc6502::{
    compile::{CompilerState, VariableDefinition, VariableType, VariableValue},
    error::Error,
    generate::GeneratorState,
};

const EQU: &str = "EQU";

pub(crate) fn write_block_size(
    gstate: &mut GeneratorState,
    compiler_state: &CompilerState,
) -> Result<(), Error> {
    if let Some((name, var)) = compiler_state
        .sorted_variables()
        .iter()
        .find(|v| v.1.var_const && v.0 == "BLOCKSIZE")
    {
        if let VariableDefinition::Value(value) = &var.def {
            let line = const_line(name, value, var.var_type);
            gstate.write(&format!("{line}\n"))?;
        }
    }
    Ok(())
}

pub(crate) fn write_consts(
    gstate: &mut GeneratorState,
    compiler_state: &CompilerState,
) -> Result<(), Error> {
    for (name, var) in compiler_state
        .sorted_variables()
        .iter()
        .filter(|v| v.1.var_const && v.0 != "BLOCKSIZE")
    {
        if let VariableDefinition::Value(value) = &var.def {
            let line = const_line(name, value, var.var_type);
            gstate.write(&format!("{line}\n"))?;
        }
    }
    Ok(())
}

fn const_line(name: &str, value: &VariableValue, var_type: VariableType) -> String {
    match value {
        VariableValue::Int(val) => format!("{name:23}\t{EQU} ${val:x}"),
        VariableValue::LowPtr((s, offset)) => {
            if var_type == VariableType::CharPtr || var_type == VariableType::ShortPtr {
                if *offset != 0 {
                    format!("{name:23}\t{EQU} {s} + {offset}")
                } else {
                    format!("{name:23}\t{EQU} {s}")
                }
            } else if *offset != 0 {
                format!("{name:23}\t{EQU} <({s} + {offset})")
            } else {
                format!("{name:23}\t{EQU} <{s}")
            }
        }
        VariableValue::HiPtr((s, offset)) => {
            if *offset != 0 {
                format!("{name:23}\t{EQU} >({s} + {offset})")
            } else {
                format!("{name:23}\t{EQU} >{s}")
            }
        }
    }
}
