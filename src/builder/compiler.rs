use std::{path::Path, process::Command};

const COMPILER: &str = "lyxass";

pub(crate) fn compile_source_file(
    file_src: &Path,
    file_destination: &Path,
) -> Result<(), std::io::Error> {
    if !file_src.exists() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            format!("{} doesn't exist.", file_src.to_string_lossy()),
        ));
    }

    let output = Command::new(COMPILER)
        .arg("-sh")
        .arg("-d")
        .arg("-o")
        .arg(file_destination)
        .arg(file_src)
        .output()?;

    if output.status.success() {
        Ok(())
    } else {
        Err(std::io::Error::other(format!(
            "stdout: {}\nstderr: {}\n",
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        )))
    }
}
