use std::collections::BTreeMap;

use anyhow::Result;
use vergen_gitcl::{
    AddCustomEntries, Build, Cargo, CargoRerunIfChanged, CargoWarning, DefaultConfig,
    DependencyKind, Emitter, Gitcl, Rustc, Sysinfo,
};

pub fn main() -> Result<()> {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rustc-check-cfg=cfg(coverage_nightly)");
    nightly();
    beta();
    stable();

    let mut cargo = Cargo::all_cargo();
    _ = cargo.set_dep_kind_filter(Some(DependencyKind::Normal));

    let gitcl = Gitcl::all()
        .remote_url("https://github.com/rustyhorde/vergen-cl.git")
        .describe(true, true, None)
        .build();

    Emitter::default()
        .add_instructions(&Build::all_build())?
        .add_instructions(&cargo)?
        .add_instructions(&gitcl)?
        .add_instructions(&Rustc::all_rustc())?
        .add_instructions(&Sysinfo::all_sysinfo())?
        .add_custom_instructions(&Custom::default())?
        .emit()
}

#[rustversion::nightly]
fn nightly() {
    println!("cargo:rustc-check-cfg=cfg(nightly)");
    println!("cargo:rustc-cfg=nightly");
}

#[rustversion::not(nightly)]
fn nightly() {
    println!("cargo:rustc-check-cfg=cfg(nightly)");
}

#[rustversion::beta]
fn beta() {
    println!("cargo:rustc-check-cfg=cfg(beta)");
    println!("cargo:rustc-cfg=beta");
}

#[rustversion::not(beta)]
fn beta() {
    println!("cargo:rustc-check-cfg=cfg(beta)");
}

#[rustversion::stable]
fn stable() {
    println!("cargo:rustc-check-cfg=cfg(stable)");
    println!("cargo:rustc-cfg=stable");
}

#[rustversion::not(stable)]
fn stable() {
    println!("cargo:rustc-check-cfg=cfg(stable)");
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
