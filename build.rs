use anyhow::Result;
use vergen::{DependencyKind, EmitBuilder};

pub fn main() -> Result<()> {
    EmitBuilder::builder()
        .all_build()
        .all_cargo()
        .cargo_dependencies_dep_kind_filter(Some(DependencyKind::Normal))
        .all_git()
        .git_describe(true, true, None)
        .all_rustc()
        .all_sysinfo()
        .emit()
}
