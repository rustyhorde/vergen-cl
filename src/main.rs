use anyhow::Result;
use vergen_pretty::{vergen_pretty_env, PrettyBuilder};

fn main() -> Result<()> {
    let mut stdout = vec![];
    PrettyBuilder::default()
        .env(vergen_pretty_env!())
        .build()?
        .display(&mut stdout)?;
    println!("{}", String::from_utf8_lossy(&stdout));
    Ok(())
}
