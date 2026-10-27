//! Installer packaging (): turn built artifacts plus a
//! config into a distributable installer layout - the payload, a `manifest.json`,
//! the license text and an NSIS install script - that the installer app and the
//! `osbtk package` CLI both produce.
//!
//! Producing the final signed `.exe` (NSIS/Inno/WiX compilation) is a build-time
//! concern; this module assembles everything that compilation consumes.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::process::Command;

use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::service::{directory_size, Artifact, ArtifactKind, Result, Service, ServiceError};

/// A full installer configuration.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InstallerConfig {
    pub product: ProductInfo,
    #[serde(default)]
    pub license: License,
    #[serde(default)]
    pub components: Vec<Component>,
    /// User-defined destination variables (`NAME` -> value), overriding defaults.
    #[serde(default)]
    pub variables: BTreeMap<String, String>,
    #[serde(default)]
    pub options: Options,
    /// Where to write the installer; defaults to the artifact root.
    #[serde(default)]
    pub output_dir: Option<String>,
}

/// The product being packaged.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProductInfo {
    pub name: String,
    #[serde(default)]
    pub vendor: String,
    #[serde(default)]
    pub version: String,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub homepage: Option<String>,
}

/// The license carried by the installer (embedded, written next to the plug-ins and
/// shown as the EULA). Defaults to BSD-3-Clause for OpenFX compatibility.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", tag = "kind", content = "text")]
pub enum License {
    #[default]
    #[serde(rename = "bsd-3-clause")]
    Bsd3Clause,
    #[serde(rename = "mit")]
    Mit,
    #[serde(rename = "apache-2.0")]
    Apache2,
    #[serde(rename = "gpl-3.0")]
    Gpl3,
    #[serde(rename = "proprietary")]
    Proprietary,
    #[serde(rename = "custom")]
    Custom(String),
}

impl License {
    /// A stable SPDX-style id recorded in the manifest.
    pub fn id(&self) -> &'static str {
        match self {
            License::Bsd3Clause => "BSD-3-Clause",
            License::Mit => "MIT",
            License::Apache2 => "Apache-2.0",
            License::Gpl3 => "GPL-3.0",
            License::Proprietary => "LicenseRef-Proprietary",
            License::Custom(_) => "LicenseRef-Custom",
        }
    }

    /// Whether the license is compatible with the bundled BSD-3-Clause OpenFX SDK.
    pub fn is_openfx_compatible(&self) -> bool {
        matches!(self, License::Bsd3Clause | License::Mit | License::Apache2)
    }

    /// The full license text written to `LICENSE.txt`.
    pub fn text(&self, product: &ProductInfo) -> String {
        let holder = if product.vendor.trim().is_empty() {
            product.name.clone()
        } else {
            product.vendor.clone()
        };
        match self {
            License::Bsd3Clause => BSD_3_CLAUSE.replace("<YEAR>", "2026").replace("<HOLDER>", &holder),
            License::Mit => MIT.replace("<YEAR>", "2026").replace("<HOLDER>", &holder),
            License::Apache2 => format!(
                "Apache License, Version 2.0\n\nCopyright 2026 {holder}\n\n\
                 Licensed under the Apache License, Version 2.0 (the \"License\"); you may not use\n\
                 this file except in compliance with the License. A copy of the License is available\n\
                 at https://www.apache.org/licenses/LICENSE-2.0\n"
            ),
            License::Gpl3 => format!(
                "GNU General Public License, Version 3\n\nCopyright 2026 {holder}\n\n\
                 This program is free software: you can redistribute it and/or modify it under the\n\
                 terms of the GNU General Public License as published by the Free Software\n\
                 Foundation, either version 3 of the License, or (at your option) any later version.\n\
                 See https://www.gnu.org/licenses/gpl-3.0.txt\n"
            ),
            License::Proprietary => format!(
                "All rights reserved.\n\nCopyright 2026 {holder}. This software is proprietary and\n\
                 confidential; use, copying and distribution are prohibited without prior written\n\
                 permission from {holder}.\n"
            ),
            License::Custom(text) => text.clone(),
        }
    }
}

/// One plug-in component installed by the installer.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Component {
    pub name: String,
    /// The host target (`after-effects` | `premiere-pro` | `davinci-resolve`).
    pub target: String,
    /// The built artifact (`.aex` or `.ofx.bundle`) on disk.
    pub artifact: String,
    /// The destination variable (e.g. `$AE_PLUGINS`); defaults per target.
    #[serde(default)]
    pub destination: Option<String>,
}

/// Installer behavior options.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Options {
    #[serde(default)]
    pub scope: Scope,
    #[serde(default)]
    pub shortcuts: bool,
    #[serde(default)]
    pub launch_at_end: bool,
    #[serde(default = "default_true")]
    pub uninstaller: bool,
}

impl Default for Options {
    fn default() -> Self {
        Options {
            scope: Scope::PerUser,
            shortcuts: false,
            launch_at_end: false,
            uninstaller: true,
        }
    }
}

fn default_true() -> bool {
    true
}

/// Installation scope.
#[derive(Clone, Copy, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Scope {
    #[default]
    PerUser,
    Machine,
}

/// The result of packaging: where the installer was written and what it contains.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Installer {
    pub id: String,
    pub product: String,
    pub output_dir: String,
    /// Files written, relative to `output_dir`.
    pub files: Vec<String>,
    pub manifest: serde_json::Value,
}

struct Resolved {
    name: String,
    artifact: String,
    destination: String,
}

impl Service {
    /// Assembles an installer from a config and built artifacts. Prefers the
    /// external `@tochii/installer-cli` when configured (`OSBTK_INSTALLER_CLI`),
    /// otherwise uses the built-in packager.
    pub fn package(&self, config: &InstallerConfig) -> Result<Installer> {
        match &self.config.installer_cli {
            Some(cli) => run_installer_cli(&self.config.node, cli, config),
            None => build(config, &self.config.artifact_root),
        }
    }

    /// Packages with an explicit default output root (used by the CLI).
    pub fn package_into(&self, config: &InstallerConfig, root: &Path) -> Result<Installer> {
        build(config, root)
    }

    /// Packages an installer and wraps its output directory as a downloadable
    /// artifact (a bundle), for the build/package job pipeline.
    pub fn package_artifact(&self, config: &InstallerConfig) -> Result<Artifact> {
        let installer = self.package(config)?;
        let path = PathBuf::from(&installer.output_dir);
        Ok(Artifact {
            id: String::new(),
            name: installer.product,
            target: "installer".into(),
            kind: ArtifactKind::Bundle,
            size: directory_size(&path)?,
            path: installer.output_dir,
        })
    }
}

/// Runs the external installer CLI (`tochires-installer build <config> --json`)
/// and parses its `Installer` JSON result.
fn run_installer_cli(node: &str, cli: &Path, config: &InstallerConfig) -> Result<Installer> {
    let work = osb_core::io::temp_dir().join(format!("osbtk-installer-cfg-{}", unique_id()));
    osb_core::io::create_dir_all(&work)?;
    let config_path = work.join("installer.json");
    let bytes = serde_json::to_vec_pretty(config)
        .map_err(|error| ServiceError::Package(error.to_string()))?;
    osb_core::io::write(&config_path, bytes)?;

    let mut command = if cli.extension().and_then(|extension| extension.to_str()) == Some("js") {
        let mut command = Command::new(node);
        command.arg(cli);
        command
    } else {
        Command::new(cli)
    };
    command.arg("build").arg(&config_path).arg("--json");

    let output = command.output()?;
    if !output.status.success() {
        return Err(ServiceError::Package(
            String::from_utf8_lossy(&output.stderr).trim().to_string(),
        ));
    }
    serde_json::from_slice(&output.stdout)
        .map_err(|error| ServiceError::Package(format!("installer CLI output: {error}")))
}

/// Assembles the installer layout under `default_root`.
pub fn build(config: &InstallerConfig, default_root: &Path) -> Result<Installer> {
    if config.product.name.trim().is_empty() {
        return Err(ServiceError::Package("product.name is required".into()));
    }

    let id = unique_id();
    let out_dir = config
        .output_dir
        .as_deref()
        .map(PathBuf::from)
        .unwrap_or_else(|| default_root.join(format!("installer-{id}")));
    let payload_dir = out_dir.join("payload");
    osb_core::io::create_dir_all(&payload_dir)?;

    let mut files = Vec::new();
    let mut resolved = Vec::new();
    for component in &config.components {
        let source = PathBuf::from(&component.artifact);
        if !source.exists() {
            return Err(ServiceError::Package(format!(
                "artifact not found: {}",
                source.display()
            )));
        }
        let file_name = source
            .file_name()
            .map(|name| name.to_string_lossy().into_owned())
            .ok_or_else(|| ServiceError::Package("artifact has no file name".into()))?;
        let relative = format!("payload/{file_name}");
        copy_entry(&source, &out_dir.join(&relative))?;
        resolved.push(Resolved {
            name: component.name.clone(),
            artifact: relative.clone(),
            destination: component
                .destination
                .clone()
                .unwrap_or_else(|| default_destination(&component.target).to_string()),
        });
        files.push(relative);
    }

    let license_text = config.license.text(&config.product);
    write(&out_dir, "LICENSE.txt", &license_text)?;
    files.push("LICENSE.txt".into());

    let manifest = json!({
        "product": config.product,
        "license": config.license.id(),
        "licenseCompatibleWithOpenFx": config.license.is_openfx_compatible(),
        "components": resolved
            .iter()
            .map(|entry| json!({
                "name": entry.name,
                "artifact": entry.artifact,
                "destination": entry.destination,
            }))
            .collect::<Vec<_>>(),
        "variables": config.variables,
        "options": config.options,
    });
    let manifest_text = serde_json::to_string_pretty(&manifest)
        .map_err(|error| ServiceError::Package(error.to_string()))?;
    write(&out_dir, "manifest.json", &manifest_text)?;
    files.push("manifest.json".into());

    write(&out_dir, "variables.nsh", &render_variables(config))?;
    files.push("variables.nsh".into());
    write(&out_dir, "installer.nsi", &render_nsis(config, &resolved))?;
    files.push("installer.nsi".into());

    Ok(Installer {
        id,
        product: config.product.name.clone(),
        output_dir: out_dir.to_string_lossy().into_owned(),
        files,
        manifest,
    })
}

fn default_destination(target: &str) -> &'static str {
    match target {
        "after-effects" => "$AE_PLUGINS",
        "premiere-pro" => "$PPRO_PLUGINS",
        "davinci-resolve" => "$RESOLVE_OFX",
        _ => "$PLUGINS",
    }
}

/// Copies a file or directory tree, so an `.ofx.bundle` directory is preserved.
fn copy_entry(source: &Path, destination: &Path) -> Result<()> {
    if source.is_dir() {
        osb_core::io::create_dir_all(destination)?;
        for entry in osb_core::io::read_dir(source)? {
            copy_entry(&entry.path, &destination.join(&entry.name))?;
        }
    } else {
        if let Some(parent) = destination.parent() {
            osb_core::io::create_dir_all(parent)?;
        }
        osb_core::io::copy(source, destination)?;
    }
    Ok(())
}

fn write(root: &Path, relative: &str, contents: &str) -> Result<()> {
    osb_core::io::create_dir_all(root)?;
    osb_core::io::write(root.join(relative), contents)?;
    Ok(())
}

fn render_variables(config: &InstallerConfig) -> String {
    let mut out = String::from("; Destination variables for the osbTK installer.\n");
    out.push_str("!define AE_PLUGINS \"$PROGRAMFILES64\\Adobe\\Adobe After Effects\\Support Files\\Plug-ins\"\n");
    out.push_str(
        "!define PPRO_PLUGINS \"$PROGRAMFILES64\\Adobe\\Common\\Plug-ins\\7.0\\MediaCore\"\n",
    );
    out.push_str(
        "!define RESOLVE_OFX \"$PROGRAMFILES\\Blackmagic Design\\DaVinci Resolve\\OFX\"\n",
    );
    for (name, value) in &config.variables {
        out.push_str(&format!("!define {name} \"{value}\"\n"));
    }
    out
}

fn render_nsis(config: &InstallerConfig, components: &[Resolved]) -> String {
    let scope = match config.options.scope {
        Scope::Machine => "admin",
        Scope::PerUser => "user",
    };
    let mut out = String::new();
    out.push_str("!include \"MUI2.nsh\"\n!include \"variables.nsh\"\n\n");
    out.push_str(&format!("Name \"{}\"\n", config.product.name));
    out.push_str(&format!("OutFile \"{}-Setup.exe\"\n", config.product.name));
    out.push_str("InstallDir \"$PROGRAMFILES64\\${VENDOR}\\${PRODUCT}\"\n");
    out.push_str(&format!("RequestExecutionLevel {scope}\n\n"));
    out.push_str("!insertmacro MUI_PAGE_DIRECTORY\n!insertmacro MUI_PAGE_INSTFILES\n");
    out.push_str("!insertmacro MUI_LANGUAGE \"English\"\n\n");
    out.push_str("Section \"Install\"\n");
    for component in components {
        out.push_str(&format!(
            "  SetOutPath \"{}\"\n  File \"{}\"\n",
            to_nsis_var(&component.destination),
            component.artifact.replace('/', "\\")
        ));
    }
    out.push_str("SectionEnd\n");
    out
}

/// Rewrites a `$NAME` destination variable into NSIS's `${NAME}` form.
fn to_nsis_var(destination: &str) -> String {
    match destination.strip_prefix('$') {
        Some(name) => format!("${{{}}}", name.trim_matches(['{', '}'])),
        None => destination.to_string(),
    }
}

fn unique_id() -> String {
    format!(
        "{}-{:x}",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|duration| duration.as_nanos())
            .unwrap_or(0)
    )
}

const BSD_3_CLAUSE: &str = "BSD 3-Clause License\n\nCopyright (c) <YEAR>, <HOLDER>\n\n\
Redistribution and use in source and binary forms, with or without modification, are\n\
permitted provided that the following conditions are met:\n\n\
1. Redistributions of source code must retain the above copyright notice, this list of\n\
   conditions and the following disclaimer.\n\n\
2. Redistributions in binary form must reproduce the above copyright notice, this list\n\
   of conditions and the following disclaimer in the documentation and/or other\n\
   materials provided with the distribution.\n\n\
3. Neither the name of the copyright holder nor the names of its contributors may be\n\
   used to endorse or promote products derived from this software without specific\n\
   prior written permission.\n\n\
THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS \"AS IS\" AND ANY\n\
EXPRESS OR IMPLIED WARRANTIES ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR\n\
CONTRIBUTORS BE LIABLE FOR ANY DAMAGES ARISING IN ANY WAY OUT OF THE USE OF THIS SOFTWARE.\n";

const MIT: &str = "MIT License\n\nCopyright (c) <YEAR> <HOLDER>\n\n\
Permission is hereby granted, free of charge, to any person obtaining a copy of this\n\
software and associated documentation files (the \"Software\"), to deal in the Software\n\
without restriction, including without limitation the rights to use, copy, modify,\n\
merge, publish, distribute, sublicense, and/or sell copies of the Software, and to\n\
permit persons to whom the Software is furnished to do so, subject to the following\n\
conditions:\n\n\
The above copyright notice and this permission notice shall be included in all copies\n\
or substantial portions of the Software.\n\n\
THE SOFTWARE IS PROVIDED \"AS IS\", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED.\n";

#[cfg(test)]
mod tests {
    use super::*;
    use crate::service::ServiceConfig;

    fn sample_config(root: &Path) -> InstallerConfig {
        osb_core::io::create_dir_all(root).unwrap();
        let artifact = root.join("Thing.aex");
        osb_core::io::write(&artifact, b"binary").unwrap();
        InstallerConfig {
            product: ProductInfo {
                name: "Thing".into(),
                vendor: "Example".into(),
                version: "1.0.0".into(),
                ..ProductInfo::default()
            },
            components: vec![Component {
                name: "Thing".into(),
                target: "after-effects".into(),
                artifact: artifact.to_string_lossy().into_owned(),
                destination: None,
            }],
            ..InstallerConfig::default()
        }
    }

    fn temp_root() -> PathBuf {
        osb_core::io::temp_dir().join(format!("osbtk-installer-{}", unique_id()))
    }

    #[test]
    fn writes_manifest_license_and_nsis() {
        let root = temp_root();
        let installer = build(&sample_config(&root), &root).unwrap();
        let out = PathBuf::from(&installer.output_dir);

        assert!(out.join("manifest.json").is_file());
        assert!(out.join("LICENSE.txt").is_file());
        assert!(out.join("installer.nsi").is_file());
        assert!(out.join("payload/Thing.aex").is_file());
        assert_eq!(installer.manifest["license"], "BSD-3-Clause");
        assert_eq!(
            installer.manifest["components"][0]["destination"],
            "$AE_PLUGINS"
        );

        let nsi = osb_core::io::read_string(out.join("installer.nsi")).unwrap();
        assert!(nsi.contains("SetOutPath \"${AE_PLUGINS}\""));
        assert!(nsi.contains("File \"payload\\Thing.aex\""));

        osb_core::io::remove_dir_all(&root).ok();
    }

    #[test]
    fn rejects_a_missing_artifact_and_an_empty_product() {
        let root = temp_root();
        let mut config = sample_config(&root);
        config.components[0].artifact = root.join("missing.aex").to_string_lossy().into_owned();
        assert!(build(&config, &root).is_err());

        let mut empty = sample_config(&root);
        empty.product.name = "  ".into();
        assert!(build(&empty, &root).is_err());
        osb_core::io::remove_dir_all(&root).ok();
    }

    #[test]
    fn custom_license_text_and_openfx_compatibility() {
        let root = temp_root();
        let mut config = sample_config(&root);
        config.license = License::Custom("Custom EULA".into());
        let installer = build(&config, &root).unwrap();
        assert_eq!(installer.manifest["license"], "LicenseRef-Custom");
        assert!(!installer.manifest["licenseCompatibleWithOpenFx"]
            .as_bool()
            .unwrap());
        assert_eq!(
            osb_core::io::read_string(PathBuf::from(&installer.output_dir).join("LICENSE.txt"))
                .unwrap(),
            "Custom EULA"
        );
        osb_core::io::remove_dir_all(&root).ok();
    }

    #[test]
    fn service_packages_into_the_artifact_root() {
        let root = temp_root();
        let service = Service::with_config(ServiceConfig {
            artifact_root: root.clone(),
            ..ServiceConfig::in_process()
        });
        let installer = service.package(&sample_config(&root)).unwrap();
        assert!(PathBuf::from(&installer.output_dir).starts_with(&root));
        osb_core::io::remove_dir_all(&root).ok();
    }
}
