use std::collections::BTreeMap;

use anyhow::Result;
use vergen_gix::{
    AddCustomEntries, BuildBuilder, CargoBuilder, CargoRerunIfChanged, CargoWarning, DefaultConfig,
    DependencyKind, Emitter, GixBuilder, RustcBuilder, SysinfoBuilder,
};

pub fn main() -> Result<()> {
    let mut cargo = CargoBuilder::all_cargo()?;
    _ = cargo.set_dep_kind_filter(Some(DependencyKind::Normal));

    Emitter::default()
        .add_instructions(&BuildBuilder::all_build()?)?
        .add_instructions(&cargo)?
        .add_instructions(&GixBuilder::all_git()?)?
        .add_instructions(&RustcBuilder::all_rustc()?)?
        .add_instructions(&SysinfoBuilder::all_sysinfo()?)?
        .add_custom_instructions(&Custom::default())?
        .emit()
}

#[derive(Default)]
struct Custom {}

impl AddCustomEntries<&str, &str> for Custom {
    fn add_calculated_entries(
        &self,
        _idempotent: bool,
        cargo_rustc_env_map: &mut BTreeMap<&str, &str>,
        _cargo_rerun_if_changed: &mut CargoRerunIfChanged,
        cargo_warning: &mut CargoWarning,
    ) -> Result<()> {
        cargo_rustc_env_map.insert("vergen-cl", "custom_instruction");
        cargo_warning.push("custom instruction generated".to_string());
        Ok(())
    }

    fn add_default_entries(
        &self,
        _config: &DefaultConfig,
        _cargo_rustc_env_map: &mut BTreeMap<&str, &str>,
        _cargo_rerun_if_changed: &mut CargoRerunIfChanged,
        _cargo_warning: &mut CargoWarning,
    ) -> Result<()> {
        Ok(())
    }
}
