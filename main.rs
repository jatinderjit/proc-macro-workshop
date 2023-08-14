// Write code here.
//
// To see what the code looks like after macro expansion:
//     $ cargo expand
//
// To run the code:
//     $ cargo run

use std::error::Error;

use derive_builder::Builder;

#[derive(Builder)]
pub struct Command {
    executable: String,
    args: Vec<String>,
    env: Vec<String>,
    current_dir: Option<String>,
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut cmd = Command::builder();
    cmd.executable("ls".to_owned());
    cmd.args(vec!["abc".to_owned(), "def".to_owned()]);
    cmd.env(vec!["abc".to_owned()]);
    cmd.build()?;
    Ok(())
}
