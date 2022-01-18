// Parse system-deps metadata from Cargo.toml

use std::{fs, io::Read, path::Path};

use anyhow::{anyhow, bail, Error};
use toml::{map::Map, Value};

#[derive(Debug, PartialEq)]
pub(crate) struct MetaData {
    pub(crate) deps: Vec<Dependency>,
}

#[derive(Debug, PartialEq)]
pub(crate) struct Dependency {
    pub(crate) key: String,
    pub(crate) version: Option<String>,
    pub(crate) name: Option<String>,
    pub(crate) feature: Option<String>,
    pub(crate) optional: bool,
    pub(crate) cfg: Option<cfg_expr::Expression>,
    pub(crate) version_overrides: Vec<VersionOverride>,
}

impl Dependency {
    fn new(name: &str) -> Self {
        Self {
            key: name.to_string(),
            ..Default::default()
        }
    }

    pub(crate) fn lib_name(&self) -> String {
        self.name.as_ref().unwrap_or(&self.key).to_string()
    }
}

impl Default for Dependency {
    fn default() -> Self {
        Self {
            key: "".to_string(),
            version: None,
            name: None,
            feature: None,
            optional: false,
            cfg: None,
            version_overrides: Vec::new(),
        }
    }
}

#[derive(Debug, PartialEq)]
pub(crate) struct VersionOverride {
    pub(crate) key: String,
    pub(crate) version: String,
    pub(crate) name: Option<String>,
    pub(crate) optional: Option<bool>,
}

struct VersionOverrideBuilder {
    version_id: String,
    version: Option<String>,
    full_name: Option<String>,
    optional: Option<bool>,
}

impl VersionOverrideBuilder {
    fn new(version_id: &str) -> Self {
        Self {
            version_id: version_id.to_string(),
            version: None,
            full_name: None,
            optional: None,
        }
    }

    fn build(self) -> Result<VersionOverride, Error> {
        let version = self
            .version
            .ok_or_else(|| anyhow!("missing version field"))?;

        Ok(VersionOverride {
            key: self.version_id,
            version,
            name: self.full_name,
            optional: self.optional,
        })
    }
}

impl MetaData {
    pub(crate) fn from_file(path: &Path) -> Result<Self, crate::Error> {
        let mut manifest = fs::File::open(&path).map_err(|e| {
            crate::Error::FailToRead(format!("error opening {}", path.display()), e)
        })?;

        let mut manifest_str = String::new();
        manifest.read_to_string(&mut manifest_str).map_err(|e| {
            crate::Error::FailToRead(format!("error reading {}", path.display()), e)
        })?;

        Self::from_str(manifest_str)
            .map_err(|e| crate::Error::InvalidMetadata(format!("{}: {}", path.display(), e)))
    }

    fn from_str(manifest_str: String) -> Result<Self, Error> {
        let toml = manifest_str
            .parse::<toml::Value>()
            .map_err(|e| anyhow!("error parsing TOML: {:?}", e))?;

        let key = "package.metadata.system-deps";
        let meta = toml
            .get("package")
            .and_then(|v| v.get("metadata"))
            .and_then(|v| v.get("system-deps"))
            .ok_or_else(|| anyhow!("no {}", key))?;

        let deps = Self::parse_deps_table(meta, key, true)?;

        Ok(MetaData { deps })
    }

    fn parse_deps_table(
        table: &Value,
        key: &str,
        allow_cfg: bool,
    ) -> Result<Vec<Dependency>, Error> {
        let table = table
            .as_table()
            .ok_or_else(|| anyhow!("{} not a table", key))?;

        let mut deps = Vec::new();

        for (name, value) in table {
            if name.starts_with("cfg(") {
                if allow_cfg {
                    let cfg_exp = cfg_expr::Expression::parse(name)?;

                    for mut dep in
                        Self::parse_deps_table(value, &format!("{}.{}", key, name), false)?
                    {
                        dep.cfg = Some(cfg_exp.clone());
                        deps.push(dep);
                    }
                } else {
                    bail!("{}.{}: cfg() cannot be nested", key, name);
                }
            } else {
                let dep =
                    Self::parse_dep(name, value).map_err(|e| anyhow!("{}.{}: {}", key, name, e))?;
                deps.push(dep);
            }
        }

        Ok(deps)
    }

    fn parse_dep(name: &str, value: &Value) -> Result<Dependency, Error> {
        let mut dep = Dependency::new(name);

        match value {
            // somelib = "1.0"
            toml::Value::String(ref s) => {
                dep.version = Some(s.clone());
            }
            toml::Value::Table(ref t) => {
                Self::parse_dep_table(&mut dep, t)?;
            }
            _ => {
                bail!("not a string or table");
            }
        }

        Ok(dep)
    }

    fn parse_dep_table(dep: &mut Dependency, t: &Map<String, Value>) -> Result<(), Error> {
        for (key, value) in t {
            match (key.as_str(), value) {
                ("feature", &toml::Value::String(ref s)) => {
                    dep.feature = Some(s.clone());
                }
                ("version", &toml::Value::String(ref s)) => {
                    dep.version = Some(s.clone());
                }
                ("name", &toml::Value::String(ref s)) => {
                    dep.name = Some(s.clone());
                }
                ("optional", &toml::Value::Boolean(optional)) => {
                    dep.optional = optional;
                }
                (version_feature, &toml::Value::Table(ref version_settings))
                    if version_feature.starts_with('v') =>
                {
                    let mut builder = VersionOverrideBuilder::new(version_feature);

                    for (k, v) in version_settings {
                        match (k.as_str(), v) {
                            ("version", &toml::Value::String(ref feat_vers)) => {
                                builder.version = Some(feat_vers.into());
                            }
                            ("name", &toml::Value::String(ref feat_name)) => {
                                builder.full_name = Some(feat_name.into());
                            }
                            ("optional", &toml::Value::Boolean(optional)) => {
                                builder.optional = Some(optional);
                            }
                            _ => {
                                bail!(
                                    "unexpected version settings key: {} type: {}",
                                    k,
                                    v.type_str()
                                )
                            }
                        }
                    }

                    dep.version_overrides.push(builder.build()?);
                }
                _ => {
                    bail!("unexpected key {} type {}", key, value.type_str());
                }
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert_matches::assert_matches;
    use cfg_expr::Expression;
    use std::{env, path::PathBuf};

    fn parse_file(dir: &str) -> Result<MetaData, crate::Error> {
        let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
        let mut p: PathBuf = manifest_dir.into();
        p.push("src");
        p.push("tests");
        p.push(dir);
        p.push("Cargo.toml");
        assert!(p.exists());

        MetaData::from_file(&p)
    }

    #[test]
    fn parse_good() {
        let m = parse_file("toml-good").unwrap();

        assert_eq!(
            m,
            MetaData {
                deps: vec![
                    Dependency {
                        key: "testdata".into(),
                        version: Some("4".into()),
                        ..Default::default()
                    },
                    Dependency {
                        key: "testlib".into(),
                        version: Some("1".into()),
                        feature: Some("test-feature".into()),
                        ..Default::default()
                    },
                    Dependency {
                        key: "testmore".into(),
                        version: Some("2".into()),
                        feature: Some("another-test-feature".into()),
                        ..Default::default()
                    }
                ]
            }
        )
    }

    #[test]
    fn parse_feature_not_string() {
        assert_matches!(
            parse_file("toml-feature-not-string"),
            Err(crate::Error::InvalidMetadata(_))
        );
    }

    #[test]
    fn parse_override_name() {
        let m = parse_file("toml-override-name").unwrap();

        assert_eq!(
            m,
            MetaData {
                deps: vec![Dependency {
                    key: "test_lib".into(),
                    version: Some("1.0".into()),
                    name: Some("testlib".into()),
                    version_overrides: vec![VersionOverride {
                        key: "v1_2".into(),
                        version: "1.2".into(),
                        name: None,
                        optional: None,
                    }],
                    ..Default::default()
                },]
            }
        )
    }

    #[test]
    fn parse_feature_versions() {
        let m = parse_file("toml-feature-versions").unwrap();

        assert_eq!(
            m,
            MetaData {
                deps: vec![Dependency {
                    key: "testdata".into(),
                    version: Some("4".into()),
                    version_overrides: vec![
                        VersionOverride {
                            key: "v5".into(),
                            version: "5".into(),
                            name: None,
                            optional: None,
                        },
                        VersionOverride {
                            key: "v6".into(),
                            version: "6".into(),
                            name: None,
                            optional: None,
                        },
                    ],
                    ..Default::default()
                },]
            }
        )
    }

    #[test]
    fn parse_optional() {
        let m = parse_file("toml-optional").unwrap();

        assert_eq!(
            m,
            MetaData {
                deps: vec![
                    Dependency {
                        key: "testbadger".into(),
                        version: Some("1".into()),
                        optional: true,
                        ..Default::default()
                    },
                    Dependency {
                        key: "testlib".into(),
                        version: Some("1.0".into()),
                        optional: true,
                        version_overrides: vec![VersionOverride {
                            key: "v5".into(),
                            version: "5.0".into(),
                            name: Some("testlib-5.0".into()),
                            optional: Some(false),
                        },],
                        ..Default::default()
                    },
                    Dependency {
                        key: "testmore".into(),
                        version: Some("2".into()),
                        version_overrides: vec![VersionOverride {
                            key: "v3".into(),
                            version: "3.0".into(),
                            name: None,
                            optional: Some(true),
                        },],
                        ..Default::default()
                    },
                ]
            }
        )
    }

    #[test]
    fn parse_os_specific() {
        let m = parse_file("toml-os-specific").unwrap();

        assert_eq!(
            m,
            MetaData {
                deps: vec![
                    Dependency {
                        key: "testlib".into(),
                        version: Some("1".into()),
                        cfg: Some(Expression::parse("not(target_os = \"macos\")").unwrap()),
                        ..Default::default()
                    },
                    Dependency {
                        key: "testdata".into(),
                        version: Some("1".into()),
                        cfg: Some(Expression::parse("target_os = \"linux\"").unwrap()),
                        ..Default::default()
                    },
                    Dependency {
                        key: "testanotherlib".into(),
                        version: Some("1".into()),
                        cfg: Some(Expression::parse("unix").unwrap()),
                        optional: true,
                        ..Default::default()
                    },
                ]
            }
        )
    }
}
