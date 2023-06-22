//! Logic for turning plain text source code into HTML spans for code highlighting.
//!
//! Each line of source code is processed and turned into a set of HTML spans with CSS classes.
//! Those classes define the coloring of each piece of code. The exact coloring is defined in a
//! separate CSS file, which can be selected during code generation by a theme name.

use std::fmt::{Display, Write};

use camino::Utf8Path;
use color_eyre::eyre::{eyre, Result, WrapErr};
use syntect::parsing::{ParseState, Scope, ScopeStack, ScopeStackOp, SyntaxSet, SCOPE_REPO};

/// The highlighter is the main component that performs transformation of plain source code into
/// highlighted HTML tags.
///
/// It should be shared whenever possible, instead of cloning instances. This is to prevent the
/// repeated (relatively slow) generation of required state.
pub struct Highlighter {
    /// Collection of syntaxes, that can parse source code into ASTs (**a**bstract **s**yntax
    /// **t**ree), that can then be further turned into scopes for highlighting.
    ps: SyntaxSet,
}

impl Highlighter {
    pub fn new() -> Self {
        Self {
            ps: SyntaxSet::load_defaults_nonewlines(),
        }
    }

    /// Read the file at the given path and turn each line into annotated HTML content.
    pub fn file_to_spans(&self, file: &Utf8Path, no_highlight: bool) -> Result<Vec<String>> {
        let content = std::fs::read_to_string(file)
            .wrap_err_with(|| format!("failed reading file contents from {file:?}"))?;
        let syntax = self
            .ps
            .find_syntax_by_extension("rs")
            .ok_or_else(|| eyre!("missing highlighting syntax for Rust"))?;

        let mut parse_state = ParseState::new(syntax);
        let mut scope_stack = ScopeStack::new();
        let mut lines = Vec::new();

        for line in content.lines() {
            lines.push(if no_highlight {
                escape(line).to_string()
            } else {
                let parsed_line = parse_state.parse_line(line, &self.ps)?;
                line_tokens_to_span(line, &parsed_line, &mut scope_stack)?
            });
        }

        Ok(lines)
    }
}

/// Convert a single source code line into a set of HTML spans.
fn line_tokens_to_span(
    line: &str,
    ops: &[(usize, ScopeStackOp)],
    stack: &mut ScopeStack,
) -> Result<String> {
    let mut buf = String::with_capacity(line.len() + ops.len() * 8);
    let mut pos = 0;

    for &(i, ref op) in ops {
        if i > pos {
            append_span(&mut buf, &stack.scopes, &line[pos..i])?;
            pos = i;
        }

        stack.apply(op)?;
    }

    if line.len() > pos {
        append_span(&mut buf, &stack.scopes, &line[pos..line.len()])?;
    }

    Ok(buf)
}

/// Append a span to the given buffer, wrapping it in a span with the corresponding CSS classes.
fn append_span(buf: &mut String, scopes: &[Scope], line: &str) -> Result<(), std::fmt::Error> {
    // No point in highlighting whitespace, so we can skip the overhead of a span around it.
    if line.chars().all(char::is_whitespace) {
        return write!(buf, "{}", escape(line));
    }

    if let Some(scope) = scopes.last() {
        buf.push_str("<span class=\"");
        scope_to_classes(buf, *scope);
        buf.push_str("\">");
    }

    write!(buf, "{}", escape(line))?;

    if !scopes.is_empty() {
        buf.push_str("</span>");
    }

    Ok(())
}

/// Turn the current code scope into a list of CSS classes and append them to the buffer.
fn scope_to_classes(s: &mut String, scope: Scope) {
    let repo = SCOPE_REPO.lock().unwrap();
    for i in (0..scope.len()).rev() {
        let atom = scope.atom_at(i as usize);
        let atom_s = repo.atom_str(atom);

        if i != scope.len() - 1 {
            s.push(' ');
        }

        s.push_str("syntect-");
        s.push_str(atom_s);
    }
}

/// Escape the content into HTML-safe text, so it can be combined in a template without causing
/// clashes with surrounding HTML tags.
fn escape(value: &str) -> impl Display + '_ {
    askama_escape::escape(value, askama_escape::Html)
}
