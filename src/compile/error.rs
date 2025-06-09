use std::fmt::Display;

use anyhow::Error;

use crate::compile::registry::Key;

pub type TypResult<T> = Result<T, TypError>;

#[derive(Debug)]
pub struct TypError {
    pub slug: Key,
    schema: Option<String>,
    errors: Vec<anyhow::Error>,
}

impl TypError {
    pub fn new(slug: Key) -> Self {
        TypError {
            slug,
            schema: None,
            errors: Vec::new(),
        }
    }

    pub fn new_with(slug: Key, errors: Vec<anyhow::Error>) -> TypError {
        TypError {
            slug,
            schema: None,
            errors,
        }
    }
    pub fn new_schema(slug: Key, schema: &str) -> Self {
        TypError {
            slug,
            schema: Some(schema.to_string()),
            errors: Vec::new(),
        }
    }

    pub fn has_error(&self) -> bool {
        !self.errors.is_empty()
    }

    pub fn result(&mut self, result: anyhow::Result<()>) {
        if let Err(err) = result {
            self.errors.push(err)
        }
    }
    pub fn ok<T>(&mut self, result: anyhow::Result<T>) -> Option<T> {
        match result {
            Ok(result) => Some(result),
            Err(err) => {
                self.errors.push(err);
                None
            }
        }
    }
    pub fn ok_typ<T>(&mut self, result: TypResult<T>) -> Option<T> {
        match result {
            Ok(result) => Some(result),
            Err(err) => {
                self.errors.extend(err.errors);
                None
            }
        }
    }

    pub fn add(&mut self, err: anyhow::Error) {
        self.errors.push(err)
    }

    pub fn err_or<T>(self, ok: impl FnOnce() -> T) -> TypResult<T> {
        if self.has_error() {
            Err(self)
        } else {
            Ok(ok())
        }
    }
}

fn padding_error(error: &Error, padding:&str) -> String{
    error.to_string().lines().map(|it| format!("{padding}{it}")).collect::<Vec<String>>().join("\n")
}

impl Display for TypError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "       Errors for article '{}' :", self.slug)?;
        if let Some(schema) = &self.schema {
            writeln!(f, "        (while passing schema '{schema}')",)?;
        }
        writeln!(f, "        ------------------------------------")?;
        if self.errors.is_empty() {
            writeln!(f, "        No errors reported.")?;
        } else {
            for (index, error) in self.errors.iter().enumerate() {
                writeln!(f, "        Error #{}:", index + 1)?;
                writeln!(f, "{}",padding_error(error, "            "))?;
                for cause in error.chain().skip(1) {
                    writeln!(f, "            caused by: {cause}")?;
                }
                // if index < self.errors.len() - 1 {
                //     writeln!(f, "        ---")?;
                // }
            }
        }
        Ok(())
    }
}
