use cc6502::{
    assemble::AssemblyCode, compile::CompilerState, error::Error, generate::GeneratorState,
};
use log::debug;
use std::collections::{HashMap, HashSet};

const MAIN_FUNCTION: &str = "main";

pub(crate) fn optimize_functions<'a>(
    gstate: &mut GeneratorState<'a>,
    compiler_state: &'a CompilerState<'a>,
    optimization_level: u8,
) -> Result<Option<String>, Error> {
    let mut res: Option<&str> = None;

    for f in &compiler_state.sorted_functions() {
        if let Some(code) = f.1.code.as_ref() {
            gstate.local_label_counter_for = 0;
            gstate.local_label_counter_if = 0;
            gstate
                .functions_code
                .insert(f.0.clone(), AssemblyCode::new());
            gstate.current_function = Some(f.0.clone());
            gstate.current_bank = f.1.bank;
            gstate.generate_statement(code)?;
            gstate.current_function = None;
            if optimization_level > 0 {
                gstate.optimize_function(f.0);
            }
            gstate.check_branches(f.0);
        }

        if f.1.interrupt {
            res = Some(f.0.as_str());
        }
    }
    Ok(res.map(String::from))
}

pub(crate) fn compute_functions_call_tree(
    gstate: &GeneratorState,
    compiler_state: &CompilerState,
    nmi_interrupt: &str,
) -> Vec<Vec<String>> {
    // Compute in the call tree the level of each function
    let mut function_levels: Vec<Vec<String>> = Vec::new();

    for f in &compiler_state.sorted_functions() {
        let lev = if f.0 == MAIN_FUNCTION {
            Some(0)
        } else {
            let mut already_seen = HashSet::new();
            compute_function_level(
                f.0,
                MAIN_FUNCTION,
                1,
                &gstate.functions_call_tree,
                &mut already_seen,
            )
        };
        if let Some(level) = lev {
            let l = function_levels.get_mut(level);
            if let Some(a) = l {
                a.push(f.0.clone());
            } else {
                function_levels.resize(level + 1, Vec::new());
                function_levels[level].push(f.0.clone());
            }
        }
    }

    // Look at the interrupt call tree
    let base_level = function_levels.len();

    for f in &compiler_state.sorted_functions() {
        let lev = if f.1.interrupt {
            Some(0)
        } else {
            let mut already_seen = HashSet::new();
            compute_function_level(
                f.0,
                nmi_interrupt,
                1,
                &gstate.functions_call_tree,
                &mut already_seen,
            )
        };
        if let Some(level) = lev {
            let level = level + base_level;
            let l = function_levels.get_mut(level);
            if let Some(a) = l {
                a.push(f.0.clone());
            } else {
                function_levels.resize(level + 1, Vec::new());
                function_levels[level].push(f.0.clone());
            }
        }
    }

    function_levels
}

fn compute_function_level(
    function_name: &str,
    node: &str,
    current_level: usize,
    tree: &HashMap<String, Vec<String>>,
    already_seen: &mut HashSet<String>,
) -> Option<usize> {
    let mut ret = None;
    if let Some(calls) = tree.get(node) {
        if calls.iter().any(|call| call == function_name) {
            ret = Some(current_level);
        }

        for nodex in calls {
            debug!("Function name: {function_name}, {nodex:?}");
            if already_seen.contains(nodex) {
                debug!("Function {nodex} has already been seen");
                return None;
            }
            already_seen.insert(nodex.clone());
            if let Some(lx) =
                compute_function_level(function_name, nodex, current_level + 1, tree, already_seen)
            {
                ret = ret.map_or(Some(lx), |current| Some(current.max(lx)));
            }
            already_seen.remove(nodex);
        }
    }
    ret
}

pub(crate) fn write_functions(
    gstate: &mut GeneratorState,
    compiler_state: &CompilerState,
    args: &cc6502::Args,
    bank: u32,
) -> Result<(), Error> {
    let mut processed = HashSet::new();

    if let Some(main) = compiler_state.functions.iter().find(|f| {
        f.1.code.is_some() && !f.1.inline && f.1.bank == bank && f.0.as_str() == MAIN_FUNCTION
    }) {
        processed.insert(main.0.clone());
        gstate.write(&format!("\n{}:\n", main.0.as_str()))?;
        gstate.write_function(main.0)?;
    }

    for f in compiler_state
        .sorted_functions()
        .iter()
        .filter(|f| f.1.code.is_some() && !f.1.inline && f.1.bank == bank)
    {
        if processed.contains(f.0) {
            continue;
        }

        if !gstate.functions_actually_in_use.contains(f.0) && !f.1.interrupt {
            debug!("Skipped function {}", f.0.as_str());
            processed.insert(f.0.clone());
            continue;
        }

        gstate.write(&format!("\n{}:\n", f.0.as_str()))?;
        gstate.write_function(f.0)?;
        gstate.write("\tRTS\n")?;

        processed.insert(f.0.clone());

        if args.verbose && args.verbosity >= 2 {
            println!(" - {} function processed", f.0.as_str());
        }
    }

    Ok(())
}
