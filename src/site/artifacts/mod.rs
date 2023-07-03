use std::collections::HashMap;

use crate::config::Config;
use crate::data::artifacts::{DisplayPreference, InstallMethod};
use crate::data::{Context, Release};
use crate::errors::*;

mod installers;
mod table;

use axohtml::elements::div;
use axohtml::{html, text};

pub fn page(context: &Context, config: &Config) -> Result<String> {
    let Some(release) = context.latest() else {
        return Ok(String::new());
    };

    let installer_scripts = scripts(release, config)?;
    let artifact_table = table::build(release, config)?;

    Ok(html!(
    <div>
        <div class="package-managers-downloads">
            {installer_scripts}
        </div>
        <div>
            {artifact_table}
        </div>
    </div>
    )
    .to_string())
}

pub fn scripts(release: &Release, config: &Config) -> Result<Vec<Box<div<String>>>> {
    // We only display runnable scripts here
    let mut scripts = HashMap::new();
    for (_, installer) in release.artifacts.installers() {
        // Hide installers that should be hidden
        if installer.display == DisplayPreference::Hidden {
            continue;
        }
        let InstallMethod::Run { .. } = &installer.method else {
            continue;
        };
        scripts.insert(installer.label.clone(), installer);
    }

    // Sort by label name for now
    let mut scripts: Vec<_> = scripts.into_iter().collect();
    scripts.sort_by(|(label1, _), (label2, _)| label1.cmp(label2));

    let mut output = vec![];
    for (label, installer) in scripts {
        let InstallMethod::Run { file, run_hint } = &installer.method else {
            continue;
        };
        let script = installers::run_html(*file, run_hint, release, config);
        output.push(html!(
        <div>
            <h3>{text!(label)}</h3>
            {script}
        </div>
        ));
    }
    Ok(output)
}

/// Build the install-widget for the latest release
pub fn header(context: &Context, config: &Config) -> Result<Box<div<String>>> {
    let Some(release) = context.latest() else {
        return Ok(html!(<div></div>));
    };

    header_for_release(release, config)
}

/// Build the install-widget for a given release
pub fn header_for_release(release: &Release, config: &Config) -> Result<Box<div<String>>> {
    installers::build_header(release, config)
}
