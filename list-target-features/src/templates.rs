use std::{collections::BTreeMap, error::Error, io};

use askama::Template;

use crate::{platform, ArchitectureData};

pub(crate) struct Outputs {
    pub(crate) database: String,
}

struct Feature {
    id: String,
    name_literal: String,
    description_literal: String,
    closure: Vec<String>,
}

struct Architecture {
    module_name: String,
    feature_words: usize,
    features: Vec<Feature>,
}

struct Data {
    architectures: Vec<Architecture>,
}

#[derive(Template)]
#[template(path = "database.rs", escape = "none")]
struct DatabaseTemplate<'a> {
    architectures: &'a [Architecture],
}

fn invalid_data(message: impl Into<String>) -> io::Error {
    io::Error::new(io::ErrorKind::InvalidData, message.into())
}

fn markdown_escape(text: &str) -> String {
    let mut escaped = String::with_capacity(text.len());
    for character in text.chars() {
        if matches!(character, '\\' | '[' | ']' | '|') {
            escaped.push('\\');
        }
        escaped.push(character);
    }
    escaped
}

fn feature_id(name: &str) -> io::Result<String> {
    if name.is_empty() || !name.is_ascii() {
        return Err(invalid_data(format!("invalid feature name {name:?}")));
    }

    let mut id = String::new();
    if name.as_bytes()[0].is_ascii_digit() {
        id.push_str("F_");
    }
    for byte in name.bytes() {
        if byte.is_ascii_alphanumeric() {
            id.push(byte.to_ascii_uppercase() as char);
        } else {
            id.push('_');
        }
    }
    if id == "_" {
        return Err(invalid_data(format!("invalid feature name {name:?}")));
    }
    Ok(id)
}

fn feature_ids(features: &[platform::Feature]) -> io::Result<Vec<String>> {
    let mut names = BTreeMap::new();
    let mut ids = Vec::with_capacity(features.len());
    for feature in features {
        let id = feature_id(&feature.feature)?;
        if let Some(existing) = names.insert(id.clone(), feature.feature.as_str()) {
            return Err(invalid_data(format!(
                "feature names {existing:?} and {:?} both normalize to {id}",
                feature.feature
            )));
        }
        ids.push(id);
    }
    Ok(ids)
}

impl Data {
    fn new(architectures: &[ArchitectureData]) -> io::Result<Self> {
        let mut rendered_architectures = Vec::with_capacity(architectures.len());
        for architecture in architectures {
            if architecture.features.len() > u16::MAX as usize {
                return Err(invalid_data("an architecture has too many features"));
            }

            let feature_ids = feature_ids(&architecture.features)?;
            let mut features = Vec::with_capacity(architecture.features.len());
            for (feature, id) in architecture.features.iter().zip(&feature_ids) {
                let closure = architecture
                    .features
                    .iter()
                    .zip(&feature_ids)
                    .filter(|(candidate, _)| {
                        candidate.feature == feature.feature
                            || feature.implies.contains(&candidate.feature)
                    })
                    .map(|(_, id)| id.clone())
                    .collect();

                features.push(Feature {
                    id: id.clone(),
                    name_literal: format!("{:?}", feature.feature),
                    description_literal: format!("{:?}", markdown_escape(&feature.description)),
                    closure,
                });
            }

            rendered_architectures.push(Architecture {
                module_name: architecture.spec.name.to_lowercase(),
                feature_words: architecture.features.len().div_ceil(64),
                features,
            });
        }

        Ok(Self {
            architectures: rendered_architectures,
        })
    }
}

pub(crate) fn render(architectures: &[ArchitectureData]) -> Result<Outputs, Box<dyn Error>> {
    let data = Data::new(architectures)?;
    let mut database = DatabaseTemplate {
        architectures: &data.architectures,
    }
    .render()?;
    database.push('\n');

    Ok(Outputs { database })
}

#[cfg(test)]
mod tests {
    use super::{feature_ids, markdown_escape};
    use crate::platform::Feature;

    fn feature(name: &str) -> Feature {
        Feature {
            feature: name.to_owned(),
            description: String::new(),
            implies: Vec::new(),
        }
    }

    #[test]
    fn rejects_feature_id_collisions() {
        let error = feature_ids(&[feature("example-one"), feature("example.one")]).unwrap_err();
        assert!(error.to_string().contains("both normalize to EXAMPLE_ONE"));
    }

    #[test]
    fn escapes_feature_descriptions_for_markdown() {
        assert_eq!(markdown_escape(r"FRInt[32|64]"), r"FRInt\[32\|64\]");
    }
}
