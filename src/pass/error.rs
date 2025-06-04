use std::fmt::Display;

use crate::compile::registry::Key;


pub type TypResult<T> = Result<T, TypError>;

#[derive(Debug)]
pub struct TypError {
    pub slug: Key,
    errors: Vec<anyhow::Error>,
}

impl TypError {
    pub fn new(key: Key) -> Self {
        TypError {
            slug: key,
            errors: Vec::new(),
        }
    }

    pub fn has_error(&self) -> bool {
        !self.errors.is_empty()
    }

    pub fn result(&mut self, result: anyhow::Result<()>) {
        if let Err(err) = result { self.errors.push(err) }
    }
    pub fn add(&mut self, err: anyhow::Error) {
        self.errors.push(err)
    }
}

impl Display for TypError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Errors for article: '{}'", self.slug)?;
        writeln!(f, "------------------------------------")?; 
        if self.errors.is_empty() {
            writeln!(f, "No errors reported.")?;
        } else {
            for (index, error) in self.errors.iter().enumerate() {
                writeln!(f, "Error #{}:", index + 1)?;
                writeln!(f, "  {error}")?;
                for cause in error.chain().skip(1) {
                    writeln!(f, "  caused by: {cause}")?;
                }
                if index < self.errors.len() - 1 {
                    writeln!(f, "---")?;                }
            }
        }
        Ok(())
    }
}
