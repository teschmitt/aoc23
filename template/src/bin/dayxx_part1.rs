use anyhow::Result;
use utils::get_input_file;

fn main() -> Result<()> {
    let data = get_input_file()?;

    // all_inputs(&data).map_err(|err| err.to_owned())?;

    Ok(())
}
