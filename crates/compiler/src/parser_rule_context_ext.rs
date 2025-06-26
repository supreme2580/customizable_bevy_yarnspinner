//! Adapted from <https://github.com/YarnSpinnerTool/YarnSpinner/blob/da39c7195107d8211f21c263e4084f773b84eaff/YarnSpinner.Compiler/YarnSpinnerRuleContextExt.cs>

use crate::prelude::*;
use antlr_rust::parser_rule_context::ParserRuleContext;
use antlr_rust::token::Token;
use antlr_rust::token_stream::TokenStream;
use std::iter;

pub(crate) trait ParserRuleContextExt<'input>: ParserRuleContext<'input> {
    /// Returns the original text of this [`ParserRuleContext`], including all
    /// whitespace.
    ///
    /// ## Implementation Notes
    ///
    /// In contrast to the original, we need to pass a token stream here, because
    /// antlr4rust does not allow us to retrieve it from the context or a token.
    fn get_text_with_whitespace(&self, token_stream: &ActualTokenStream<'input>) -> String {
        // We can't use "expressionContext.GetText()" here, because
        // that just concatenates the text of all captured tokens,
        // and doesn't include text on hidden channels (e.g.
        // whitespace and comments).

        // some times it seems that vscode can request a negative interval
        // almost certainly something wrong we are doing
        // but as a non-crashing fallback we prevent this
        let start = self.start().get_token_index();
        let stop = self.stop().get_token_index();
        if start > stop {
            self.get_text()
        } else {
            // ## Implementation Notes
            // Uses `get_token_index()` instead of `get_start()` and `get_stop()`.
            // I suspect the `get_text_from_interval` implementation behaves differently
            // from the C# ANTLR runtime. Might even be bugged. Alas, the way this
            // function is written now behaves the same way the original did, even if it does not seem so.
            token_stream.get_text_from_interval(start, stop)
        }
    }

    fn get_lines_around(
        &self,
        token_stream: &ActualTokenStream<'input>,
        surrounding_lines: usize,
    ) -> LinesAroundResult {
        // This seems expensive, but it's only used for error reporting.
        let whole_file = token_stream.get_all_text();
        let char_start = self.start().get_start() as usize;
        let char_stop = self.stop().get_stop() as usize + 1;
        let byte_start = whole_file
            .char_indices()
            .map(|(byte_start, _)| byte_start)
            .nth(char_start)
            .unwrap();
        let byte_stop = whole_file
            .char_indices()
            .map(|(byte_start, _)| byte_start)
            .nth(char_stop)
            .unwrap();
        let first_line = self.start().get_line_as_usize().saturating_sub(1);

        let head = &whole_file[..byte_start];
        let body = &whole_file[byte_start..byte_stop];
        let tail = &whole_file[byte_stop..];

        let head_lines_to_take = if head.ends_with('\n') || body.starts_with('\n') {
            surrounding_lines
        } else {
            surrounding_lines + 1
        };

        let head_lines = head.lines().rev().take(head_lines_to_take);
        let head_lines: Vec<_> = if head.ends_with('\n') {
            iter::once("").chain(head_lines).collect()
        } else {
            head_lines.collect()
        };
        let first_line = first_line - head_lines.len().saturating_sub(1);
        let head = head_lines.into_iter().rev().collect::<Vec<_>>().join("\n");

        let tail_lines_to_take = if body.ends_with('\n') || tail.starts_with('\n') {
            surrounding_lines
        } else {
            surrounding_lines + 1
        };
        let tail = tail
            .lines()
            .take(tail_lines_to_take)
            .collect::<Vec<_>>()
            .join("\n");
        let lines = head + body + &tail;
        LinesAroundResult { lines, first_line }
    }
}

#[derive(Debug, Clone)]
pub(crate) struct LinesAroundResult {
    pub(crate) lines: String,
    pub(crate) first_line: usize,
}

impl<'input, T: ?Sized> ParserRuleContextExt<'input> for T where T: ParserRuleContext<'input> {}
