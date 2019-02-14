use crate::lex::errors::LexError;
use crate::lex::errors::LexErrorType;
use crate::lex::errors::LexSuccess;
use crate::lex::errors::Location;
use crate::lex::types::LexItem;
use crate::lex::Lexer;
use std::iter::FromIterator;
use std::str::Chars;

#[derive(Copy, Clone, Debug)]
pub(super) struct SourceChar {
    pub(super) ch: char,
    location: Location,
}

#[derive(Debug)]
pub(super) struct SourceString(Vec<(String, Location)>);

impl ToString for SourceString {
    fn to_string(&self) -> String {
        self.0.iter().map(|(s, _)| s.as_str()).collect()
    }
}

struct ExactChars<'a>(Chars<'a>, usize);

impl<'a> Iterator for ExactChars<'a> {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        self.1 -= 1;
        self.0.next()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.len();
        (len, Some(len))
    }
}

impl<'a> ExactSizeIterator for ExactChars<'a> {
    fn len(&self) -> usize {
        self.1
    }
}

impl<'a> DoubleEndedIterator for ExactChars<'a> {
    fn next_back(&mut self) -> Option<Self::Item> {
        let val = self.0.next_back();
        if val.is_some() {
            self.1 -= 1;
        }
        val
    }
}

impl<'a> ExactChars<'a> {
    fn new(ch: Chars<'a>) -> ExactChars<'a> {
        ExactChars(ch.clone(), ch.count())
    }
}

impl SourceString {
    pub(super) fn char_iter<'a>(
        &'a self,
    ) -> impl Iterator<Item = SourceChar> + DoubleEndedIterator + 'a {
        self.0.iter().flat_map(|(s, l)| {
            ExactChars::new(s.chars())
                .enumerate()
                .map(move |(index, ch)| SourceChar {
                    ch,
                    location: Location {
                        character: l.character + index,
                    },
                })
        })
    }
    pub(super) fn new() -> SourceString {
        SourceString(Vec::new())
    }
    pub(super) fn push(&mut self, ch: SourceChar) {
        if let Some(last) = self.0.last_mut() {
            if ch.location.character == last.1.character + last.0.chars().count() {
                last.0.push(ch.ch);
                return;
            }
        }
        self.0.push((ch.ch.to_string(), ch.location));
    }
    pub(super) fn len(&self) -> usize {
        self.0.iter().map(|s| s.0.chars().count()).sum()
    }
}

impl FromIterator<SourceChar> for SourceString {
    fn from_iter<T: IntoIterator<Item = SourceChar>>(iter: T) -> Self {
        let mut s = SourceString::new();
        iter.into_iter().for_each(|ch| s.push(ch));
        s
    }
}

impl<It: Iterator<Item = char>> Lexer<It> {
    pub(super) fn set_start_pos(&mut self) -> Option<()> {
        let ch = self.next_after_whitespace()?;
        self.start_char = ch.location;
        self.nextnt(ch);
        Some(())
    }

    pub(super) fn next_char(&mut self) -> Option<SourceChar> {
        self.lookahead.pop().or_else(|| {
            let (location, ch) = self.source.next()?;
            self.character.character += 1;
            Some(SourceChar {
                ch,
                location: Location {
                    character: location,
                },
            })
        })
    }

    pub(super) fn next_chars(&mut self, n: usize) -> Option<SourceString> {
        let next: SourceString = (0..n).filter_map(|_| self.next_char()).collect();
        if next.len() < n {
            self.nextnt_string(next);
            None
        } else {
            Some(next)
        }
    }

    pub(super) fn skip_chars(&mut self, chars: &str) -> Option<SourceChar> {
        loop {
            let ch = self.next_char()?;
            if !chars.chars().any(|c| c == ch.ch) {
                break Some(ch);
            }
        }
    }
    pub(super) fn next_after_whitespace(&mut self) -> Option<SourceChar> {
        let ch = self.skip_chars(" \n\t\r")?;
        self.nextnt(ch);

        let next = self.next_chars(2);

        if next
            .as_ref()
            .map_or(false, |s| s.to_string().as_str() == "//")
        {
            while self.next_char()?.ch != '\n' {}
            self.next_after_whitespace()
        } else {
            if let Some(next) = next {
                self.nextnt_string(next);
            }
            self.next_char()
        }
    }
    pub(super) fn nextnt(&mut self, character: SourceChar) {
        self.lookahead.push(character);
    }

    pub(super) fn nextnt_string(&mut self, s: SourceString) {
        s.char_iter().rev().for_each(|ch| self.nextnt(ch));
    }

    pub(super) fn ok_token(&self, token: LexItem) -> LexSuccess {
        LexSuccess {
            item: token,
            location: self.start_char,
        }
    }

    pub(super) fn error_token(&self, token: LexErrorType) -> LexError {
        LexError {
            error_type: token,
            location: self.start_char,
        }
    }
}
