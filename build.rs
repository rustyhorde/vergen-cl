use anyhow::Result;
use vergen::EmitBuilder;

pub fn main() -> Result<()> {
    EmitBuilder::builder()
        .all_build()
        .all_cargo()
        .all_git()
        .git_describe(true, true, None)
        .all_rustc()
        .all_sysinfo()
        .emit()
}
