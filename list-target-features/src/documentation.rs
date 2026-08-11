use std::io::{self, Write};

use crate::platform::{Cpu, Feature};

fn write_markdown_escaped(writer: &mut impl Write, text: &str) -> io::Result<()> {
    let bytes = text.as_bytes();
    let mut start = 0;
    for (index, character) in text.char_indices() {
        if matches!(character, '\\' | '[' | ']' | '|') {
            writer.write_all(&bytes[start..index])?;
            writer.write_all(b"\\")?;
            start = index;
        }
    }
    writer.write_all(&bytes[start..])
}

fn write_features(docs: &mut impl Write, features: &[Feature]) -> io::Result<()> {
    writeln!(
        docs,
        "    /// | Feature | Description | Also Enables<sup>†</sup> |"
    )?;
    writeln!(
        docs,
        "    /// | ------- | ----------- | ------------------------ |"
    )?;
    for Feature {
        feature,
        description,
        implies,
        ..
    } in features
    {
        write!(docs, "    /// | `{feature}` | ")?;
        write_markdown_escaped(docs, description)?;
        write!(docs, " | ")?;
        for (index, feature) in implies.iter().enumerate() {
            if index != 0 {
                write!(docs, ", ")?;
            }
            write!(docs, "`{feature}`")?;
        }
        writeln!(docs, " |")?;
    }
    writeln!(docs, "    ///")?;
    writeln!(docs, "    /// <sup>†</sup> This is often empirical, rather than specified in any standard, i.e. all available CPUs with a particular feature also have another feature.")?;
    writeln!(docs, "    pub mod feature {{}}")
}

fn write_cpus(docs: &mut impl Write, cpus: &[Cpu]) -> io::Result<()> {
    writeln!(docs)?;
    writeln!(docs, "    /// | CPU | Enabled Features |")?;
    writeln!(docs, "    /// | --- | -------- |")?;
    for Cpu { cpu, features } in cpus {
        writeln!(
            docs,
            "    /// | `{cpu}` | {} |",
            features
                .iter()
                .map(|feature| format!("`{feature}`"))
                .collect::<Vec<_>>()
                .join(", ")
        )?;
    }
    writeln!(docs, "    pub mod cpus {{}}")
}

pub(crate) fn write(
    docs: &mut impl Write,
    architecture: &str,
    target_features: &[Feature],
    target_cpus: &[Cpu],
) -> io::Result<()> {
    writeln!(docs, "/// {} documentation", architecture.to_lowercase())?;
    writeln!(docs, "pub mod {} {{", architecture.to_lowercase())?;
    write_features(docs, target_features)?;
    write_cpus(docs, target_cpus)?;
    writeln!(docs, "}}")
}
