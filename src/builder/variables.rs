use cc6502::{
    compile::{CompilerState, Variable, VariableDefinition, VariableMemory, VariableType, VariableValue},
    error::Error,
    generate::GeneratorState,
};

pub(crate) fn write_non_zeropage(
    gstate: &mut GeneratorState,
    compiler_state: &CompilerState,
    bank: u32,
) -> Result<(), Error> {
    let vars = compiler_state.sorted_variables();

    for v in vars.iter().filter(|v| {
        matches!(v.1.memory, VariableMemory::Bank(b) if b == bank)
            && !compiler_state.functions.contains_key(v.0)
            && (!v.1.var_const || matches!(v.1.def, VariableDefinition::Array(_)))
    }) {
        write_variable_len(gstate, v.0, v.1)?;
    }

    Ok(())
}

pub(crate) fn write_zeropage(
    gstate: &mut GeneratorState,
    compiler_state: &CompilerState,
) -> Result<usize, Error> {
    gstate.write("ORG 0\n")?;
    let mut zp_len = 0;

    for v in compiler_state.sorted_variables().iter().filter(|v| {
        v.1.memory == VariableMemory::Zeropage && v.1.def == VariableDefinition::None && v.1.global && !v.1.var_const
    }) {
        zp_len += write_variable_len(gstate, v.0, v.1)?;
    }

    Ok(zp_len)
}

fn write_variable_len(
    gstate: &mut GeneratorState,
    name: &str,
    var: &Variable,
) -> Result<usize, Error> {
    let len = if var.size > 1 {
        let s = match var.var_type {
            VariableType::CharPtr => 1,
            VariableType::CharPtrPtr | VariableType::ShortPtr => 2,
            _ => unreachable!(),
        };
        var.size * s
    } else {
        match var.var_type {
            VariableType::Char => 1,
            VariableType::Short
            | VariableType::CharPtr
            | VariableType::CharPtrPtr
            | VariableType::ShortPtr => 2,
        }
    };

    if let VariableDefinition::Array(values) = &var.def && var.var_type == VariableType::CharPtr {
        let string_value: String = values
            .iter()
            .filter_map(|v| match v {
                VariableValue::Int(c) if c >= &32 => Some(*c as u8),
                _ => None,
            })
            .map(|b| b as char)
            .collect();

        gstate.write(&format!("{name:23}\tdb \"{string_value}\",0\n"))?;
    } else {
        gstate.write(&format!("{name:23}\tds {len}\n"))?;
    }

    Ok(len)
}
