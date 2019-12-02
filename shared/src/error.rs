use std::error::Error as StdError;
use std::fmt;

pub struct Error {
    inner: Box<Inner>,
}

pub(crate) type Source = Box<dyn StdError + Send + Sync>;

struct Inner {
    kind: Kind,
    source: Option<Source>,
}

#[derive(Debug)]
pub(crate) enum Kind {
    Bincode,
}

impl Error {
    pub(crate) fn new(kind: Kind) -> Error {
        Error {
            inner: Box::new(Inner { kind, source: None }),
        }
    }

    pub(crate) fn with<S: Into<Source>>(mut self, source: S) -> Error {
        self.inner.source = Some(source.into());
        self
    }

    #[allow(unused)]
    pub(crate) fn kind(&self) -> &Kind {
        &self.inner.kind
    }
}

impl fmt::Debug for Error {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut builder = fmt.debug_struct("Error");
        builder.field("kind", &self.inner.kind);
        if let Some(ref source) = self.inner.source {
            builder.field("source", source);
        }
        builder.finish()
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some(ref source) = self.inner.source {
            write!(f, "{}: {}", self.description(), source)
        } else {
            f.write_str(&self.description())
        }
    }
}

impl StdError for Error {
    fn description(&self) -> &str {
        match self.inner.kind {
            Kind::Bincode => "bincode error",
        }
    }

    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        self.inner.source.as_ref().map(|e| &**e as _)
    }
}

// Helpers
#[allow(unused)]
pub(crate) fn bincode<E: Into<Source>>(e: E) -> Error {
    Error::new(Kind::Bincode).with(e)
}
