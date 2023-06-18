use std::{fmt, fmt::Write};

use anyhow::Result;
use camino::Utf8Path;
use syntect::parsing::{ParseState, Scope, ScopeStack, ScopeStackOp, SyntaxSet, SCOPE_REPO};

pub struct Highlighter {
    ps: SyntaxSet,
}

impl Highlighter {
    pub fn new() -> Self {
        Self {
            ps: SyntaxSet::load_defaults_nonewlines(),
        }
    }

    pub fn file_to_spans(&self, file: &Utf8Path, no_highlight: bool) -> Result<Vec<String>> {
        let content = std::fs::read_to_string(file)?;
        let syntax = self.ps.find_syntax_by_extension("rs").unwrap();

        let mut parse_state = ParseState::new(syntax);
        let mut scope_stack = ScopeStack::new();
        let mut lines = Vec::new();

        for line in content.lines() {
            lines.push(if no_highlight {
                Escape(line).to_string()
            } else {
                let parsed_line = parse_state.parse_line(line, &self.ps)?;
                line_tokens_to_span(line, &parsed_line, &mut scope_stack)?
            });
        }

        Ok(lines)
    }
}

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

fn append_span(buf: &mut String, scopes: &[Scope], line: &str) -> Result<(), std::fmt::Error> {
    if line.chars().all(char::is_whitespace) {
        return write!(buf, "{}", Escape(line));
    }

    if let Some(scope) = scopes.last() {
        buf.push_str("<span class=\"");
        scope_to_classes(buf, *scope);
        buf.push_str("\">");
    }

    write!(buf, "{}", Escape(line))?;

    if !scopes.is_empty() {
        buf.push_str("</span>");
    }

    Ok(())
}

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

/// Wrapper struct which will emit the HTML-escaped version of the contained
/// string when passed to a format string.
struct Escape<'a>(pub &'a str);

impl<'a> fmt::Display for Escape<'a> {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Escape(s) = *self;
        let pile_o_bits = s;
        let mut last = 0;
        for (i, ch) in s.bytes().enumerate() {
            match ch as char {
                '<' | '>' | '&' | '\'' | '"' => {
                    fmt.write_str(&pile_o_bits[last..i])?;
                    let s = match ch as char {
                        '>' => "&gt;",
                        '<' => "&lt;",
                        '&' => "&amp;",
                        '\'' => "&#39;",
                        '"' => "&quot;",
                        _ => unreachable!(),
                    };
                    fmt.write_str(s)?;
                    last = i + 1;
                }
                _ => {}
            }
        }

        if last < s.len() {
            fmt.write_str(&pile_o_bits[last..])?;
        }
        Ok(())
    }
}
