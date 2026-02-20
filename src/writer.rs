use std::{fs::write, path::Path};

use miette::{IntoDiagnostic, Result};

pub struct TemplateWriter {
    template: String,
}

impl TemplateWriter {
    pub fn new(template: String) -> TemplateWriter {
        TemplateWriter { template }
    }

    pub fn write(self, path: impl AsRef<Path>) -> Result<()> {
        write(path.as_ref(), self.template).into_diagnostic()?;
        Ok(())
    }
}
