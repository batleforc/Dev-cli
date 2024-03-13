use jsonschema::JSONSchema;
use serde::Deserialize;
use serde_yaml::Value;
use tracing::event;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum DevFileVersion {
    V200,
    V210,
    V220,
    V221,
    V222,
}

impl DevFileVersion {
    #[tracing::instrument(level = "trace")]
    pub fn validate(yaml: String) -> Option<DevFileVersion> {
        let schema_version = DevFileVersion::extract_schema_version(yaml.clone());

        match schema_version {
            Some(version) => {
                let dev_file_value: serde_json::Value =
                    match serde_json::from_str(&version.get_def()) {
                        Ok(val) => val,
                        Err(err) => {
                            event!(tracing::Level::ERROR, "Couldn't load schema : {:?}", err);
                            return None;
                        }
                    };
                let compiled = match JSONSchema::compile(&dev_file_value) {
                    Ok(validator) => validator,
                    Err(err) => {
                        event!(
                            tracing::Level::ERROR,
                            "Couldn't load jsonschema : {:?}",
                            err
                        );
                        return None;
                    }
                };
                let json_value: serde_json::Value = match serde_yaml::from_str(&yaml) {
                    Ok(yaml_to_json) => yaml_to_json,
                    Err(err) => {
                        event!(tracing::Level::ERROR, "Couldn't parse to json : {:?}", err);
                        return None;
                    }
                };
                let result = compiled.validate(&json_value);
                match result {
                    Ok(_) => Some(version),
                    Err(_) => {
                        event!(tracing::Level::ERROR, "Couldn't validate yaml");
                        return None;
                    }
                }
            }
            None => None,
        }
    }

    #[tracing::instrument(level = "trace")]
    pub fn get_def(&self) -> String {
        match &self {
            DevFileVersion::V200 => include_str!("devfile.V200.json").to_string(),
            DevFileVersion::V210 => include_str!("devfile.V220.json").to_string(),
            DevFileVersion::V220 => include_str!("devfile.V220.json").to_string(),
            DevFileVersion::V221 => include_str!("devfile.V221.json").to_string(),
            DevFileVersion::V222 => include_str!("devfile.V222.json").to_string(),
        }
    }

    #[tracing::instrument(level = "trace")]
    pub fn extract_schema_version(yaml: String) -> Option<DevFileVersion> {
        let dev_file = serde_yaml::Deserializer::from_str(&yaml);
        let dev_file_value = match Value::deserialize(dev_file) {
            Ok(val) => val,
            Err(err) => {
                event!(tracing::Level::ERROR, "Couldn't parse : {:?}", err);
                return None;
            }
        };
        let schema_version = match dev_file_value.get("schemaVersion") {
            Some(version) => version.as_str(),
            None => {
                event!(
                    tracing::Level::ERROR,
                    "No schemaVersion found, invalid devfile"
                );
                return None;
            }
        };
        match schema_version {
            Some("2.0.0") => {
                event!(tracing::Level::TRACE, "Found : 2.0.0");
                Some(DevFileVersion::V200)
            }
            Some("2.1.0") => {
                event!(tracing::Level::TRACE, "Found : 2.1.0");
                Some(DevFileVersion::V210)
            }
            Some("2.2.0") => {
                event!(tracing::Level::TRACE, "Found : 2.2.0");
                Some(DevFileVersion::V220)
            }
            Some("2.2.1") => {
                event!(tracing::Level::TRACE, "Found : 2.2.1");
                Some(DevFileVersion::V221)
            }
            Some("2.2.2") => {
                event!(tracing::Level::TRACE, "Found : 2.2.2");
                Some(DevFileVersion::V222)
            }
            Some(ver) => {
                event!(tracing::Level::ERROR, "Unknown version : {}", ver);
                None
            }
            None => {
                event!(
                    tracing::Level::ERROR,
                    "Invalid version format, should be a string"
                );
                None
            }
        }
    }
}
