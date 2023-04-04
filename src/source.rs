use std::fmt::Debug;
use std::sync::Arc;

use camino::Utf8Path;
use miette::{MietteSpanContents, SourceCode, SourceSpan};

use crate::{error::*, LocalAsset, RemoteAsset};

/// The inner contents of a [`SourceFile`][].
#[derive(Eq, PartialEq)]
struct SourceFileInner {
    /// "Name" of the file
    filename: String,
    /// Origin path of the file
    origin_path: String,
    /// Contents of the file
    contents: String,
}

/// A file's contents along with its display name
///
/// This is used for reporting rustc-style diagnostics where we show
/// where in the file we found a problem. It contains an Arc so that
/// it's ~free for everything to pass/copy these around and produce
/// better diagnostics.
#[derive(Clone, Eq, PartialEq)]
pub struct SourceFile {
    /// The actual impl
    inner: Arc<SourceFileInner>,
}

impl SourceFile {
    /// Create an empty SourceFile with the given name
    pub fn new_empty(origin_path: &str) -> Result<Self> {
        Self::new(origin_path, String::new())
    }

    /// Create a new source file with the given name and contents.
    pub fn new(origin_path: &str, contents: String) -> Result<Self> {
        Ok(SourceFile {
            inner: Arc::new(SourceFileInner {
                filename: LocalAsset::filename(origin_path)?,
                origin_path: origin_path.to_owned(),
                contents,
            }),
        })
    }

    #[cfg(feature = "remote")]
    /// SourceFile equivalent of [`crate::RemoteAsset::load`][]
    pub async fn load_remote(origin_path: &str) -> Result<SourceFile> {
        let contents = crate::RemoteAsset::load_string(origin_path).await?;
        Ok(SourceFile {
            inner: Arc::new(SourceFileInner {
                filename: RemoteAsset::load(origin_path).await?.filename,
                origin_path: origin_path.to_owned(),
                contents,
            }),
        })
    }

    /// SourceFile equivalent of [`LocalAsset::load`][]
    pub fn load_local<'a>(origin_path: impl Into<&'a Utf8Path>) -> Result<SourceFile> {
        let origin_path = origin_path.into();
        let contents = LocalAsset::load_string(origin_path.as_str())?;
        Ok(SourceFile {
            inner: Arc::new(SourceFileInner {
                filename: LocalAsset::filename(origin_path.as_str())?,
                origin_path: origin_path.to_string(),
                contents,
            }),
        })
    }

    /// Try to deserialize the contents of the SourceFile as json
    #[cfg(feature = "json-serde")]
    pub fn deserialize_json<'a, T: serde::Deserialize<'a>>(&'a self) -> Result<T> {
        let json = serde_json::from_str(self.source()).map_err(|details| {
            let span = self.span_for_line_col(details.line(), details.column());
            AxoassetError::Json {
                contents: self.clone(),
                span,
                details,
            }
        })?;
        Ok(json)
    }

    /// Try to deserialize the contents of the SourceFile as toml
    #[cfg(feature = "toml-serde")]
    pub fn deserialize_toml<'a, T: serde::Deserialize<'a>>(&'a self) -> Result<T> {
        let toml = toml::from_str(self.contents()).map_err(|details| {
            let span = details
                .line_col()
                .and_then(|(line, col)| self.span_for_line_col(line, col));
            AxoassetError::Toml {
                contents: self.clone(),
                span,
                details,
            }
        })?;
        Ok(toml)
    }

    /// Get the filename of a SourceFile
    pub fn filename(&self) -> &str {
        &self.inner.filename
    }

    /// Get the origin_path of a SourceFile
    pub fn origin_path(&self) -> &str {
        &self.inner.origin_path
    }

    /// Get the contents of a SourceFile
    pub fn contents(&self) -> &str {
        &self.inner.contents
    }

    /// Gets a proper [`SourceSpan`] from a line-and-column representation
    ///
    /// Both values are 1's based, so `(1, 1)` is the start of the file.
    /// If anything underflows/overflows or goes out of bounds then we'll
    /// just return `None`. `unwrap_or_default()` will give you the empty span from that.
    ///
    /// This is a pretty heavy-weight process, we have to basically linearly scan the source
    /// for this position!
    pub fn span_for_line_col(&self, line: usize, col: usize) -> Option<SourceSpan> {
        let src = self.contents();
        let src_line = src.lines().nth(line.checked_sub(1)?)?;
        if col > src_line.len() {
            return None;
        }
        let src_addr = src.as_ptr() as usize;
        let line_addr = src_line.as_ptr() as usize;
        let line_offset = line_addr.checked_sub(src_addr)?;
        let start = line_offset.checked_add(col)?.checked_sub(1)?;
        let end = start.checked_add(1)?;
        if start > end || end >= src.len() {
            return None;
        }
        Some(SourceSpan::from(start..end))
    }
}

impl SourceCode for SourceFile {
    fn read_span<'a>(
        &'a self,
        span: &SourceSpan,
        context_lines_before: usize,
        context_lines_after: usize,
    ) -> std::result::Result<Box<dyn miette::SpanContents<'a> + 'a>, miette::MietteError> {
        let contents =
            self.contents()
                .read_span(span, context_lines_before, context_lines_after)?;
        Ok(Box::new(MietteSpanContents::new_named(
            self.origin_path().to_owned(),
            contents.data(),
            *contents.span(),
            contents.line(),
            contents.column(),
            contents.line_count(),
        )))
    }
}

impl Debug for SourceFile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SourceFile")
            .field("origin_path", &self.origin_path())
            .field("contents", &self.contents())
            .finish()
    }
}
