use clap::{builder::OsStr, ArgAction};
use semver::Version;
use serde::Deserialize;
use std::{collections::BTreeMap, path::Path};

use crate::lib::{error::ExtensionError, extension::Extension};

pub static MANIFEST_FILE_NAME: &str = "extension.json";
type DependencyName = String;
type SubcmdName = String;
type ArgName = String;

#[derive(Debug, Deserialize)]
pub struct ExtensionManifest {
    pub name: String,
    pub version: String,
    pub homepage: String,
    pub authors: Option<String>,
    pub summary: String,
    pub categories: Vec<String>,
    pub keywords: Option<Vec<String>>,
    pub description: Option<String>,
    pub dependencies: Option<BTreeMap<DependencyName, Version>>,
    pub subcommands: Option<BTreeMap<SubcmdName, ExtensionSubcommandOpts>>, // TODO: https://dfinity.atlassian.net/browse/SDK-599
}

impl ExtensionManifest {
    pub fn new(name: &str, extensions_root_dir: &Path) -> Result<Self, ExtensionError> {
        let manifest_path = extensions_root_dir.join(name).join(MANIFEST_FILE_NAME);
        if !manifest_path.exists() {
            return Err(ExtensionError::ExtensionManifestMissing(name.to_owned()));
        }
        dfx_core::json::load_json_file(&manifest_path)
            .map_err(ExtensionError::ExtensionManifestIsNotValid)
    }

    pub fn into_clap_commands(self) -> Option<Vec<clap::Command>> {
        if let Some(subcmds) = self.subcommands {
            return Some(
                subcmds
                    .into_iter()
                    .map(|(k, v)| v.into_clap_command(k))
                    .collect(),
            );
        }
        None
    }
}

#[derive(Debug, Deserialize)]
pub struct ExtensionSubcommandOpts {
    pub about: Option<String>,
    pub args: Option<BTreeMap<ArgName, ExtensionSubcommandArgOpts>>,
    pub subcommands: Option<BTreeMap<SubcmdName, ExtensionSubcommandOpts>>,
}

#[derive(Debug, Deserialize)]
pub struct ExtensionSubcommandArgOpts {
    pub about: Option<String>,
    pub long: Option<String>,
    pub short: Option<char>,
}

impl ExtensionSubcommandArgOpts {
    pub fn into_clap_arg(self, name: String) -> clap::Arg {
        let mut arg =
            clap::Arg::new(name).help(self.about.unwrap_or("Missing arg description.".to_string()));
        if let Some(l) = self.long {
            arg = arg.long(l);
        }
        if let Some(s) = self.short {
            arg = arg.short(s);
        }
        arg
            // let's not enforce any restrictions
            .allow_hyphen_values(false)
            .required(false)
            .action(ArgAction::Append)
    }
}

impl ExtensionSubcommandOpts {
    pub fn into_clap_command(self, name: String) -> clap::Command {
        let mut cmd = clap::Command::new(name);

        if let Some(about) = self.about {
            cmd = cmd.about(about);
        }

        if let Some(args) = self.args {
            for (name, opts) in args {
                cmd = cmd.arg(opts.into_clap_arg(name));
            }
        }

        if let Some(subcommands) = self.subcommands {
            for (name, subcommand) in subcommands {
                cmd = cmd.subcommand(subcommand.into_clap_command(name));
            }
        }

        cmd
    }
}

#[test]
fn parse_test_file() {
    let f = r#"
{
  "name": "sns",
  "version": "0.1.0",
  "homepage": "https://github.com/dfinity/dfx-extensions",
  "authors": "DFINITY",
  "summary": "Toolkit for simulating decentralizing a dapp via SNS.",
  "categories": [
    "sns",
    "nns"
  ],
  "keywords": [
    "sns",
    "nns",
    "deployment"
  ],
  "subcommands": {
    "config": {
      "about": "Subcommands for working with configuration.",
      "subcommands": {
        "create": {
          "about": "Command line options for creating an SNS configuration."
        },
        "validate": {
          "about": "Command line options for validating an SNS configuration."
        }
      }
    },
    "deploy": {
      "about": "Subcommand for creating an SNS."
    },
    "import": {
      "about": "Subcommand for importing sns API definitions and canister IDs.",
      "args": {
        "network_mapping": {
          "about": "Networks to import canisters ids for.\n  --network-mapping <network name in both places>\n  --network-mapping <network name here>=<network name in project being imported>\nExamples:\n  --network-mapping ic\n  --network-mapping ic=mainnet",
          "long": "network-mapping"
        }
      }
    },
    "download": {
      "about": "Subcommand for downloading SNS WASMs.",
      "args": {
        "ic_commit": {
          "about": "IC commit of SNS canister WASMs to download",
          "long": "ic-commit"
        },
        "wasms_dir": {
          "about": "Path to store downloaded SNS canister WASMs",
          "long": "wasms-dir"
        }
      }
    }
  }
}
"#;

    let m: Result<ExtensionManifest, serde_json::Error> = serde_json::from_str(f);
    // dbg!(&m);
    assert!(m.is_ok());

    let subcmds = m.unwrap().into_clap_commands().unwrap();
    dbg!(&subcmds);
    for s in &subcmds {
        if s.get_name() == "download" {
            let matches = s
                .clone()
                .get_matches_from(vec!["download", "--ic-commit", "value"]);
            assert_eq!(
                Some(&"value".to_string()),
                matches.get_one::<String>("ic_commit")
            );
        }
    }

    let mut cli = clap::Command::new("sns").subcommands(subcmds);
    cli.debug_assert();
}
