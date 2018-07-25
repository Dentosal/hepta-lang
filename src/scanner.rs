use std::iter::Peekable;
use std::str::Chars;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TokenScanAction {
    Continue,
    DoneConsumeThis,
    DoneContinueHere,
}

#[derive(Debug, Clone)]
pub(crate) enum TokenScanState {
    /// Comment(nesting_level)
    Comment(u8),
    /// Identifier(so_far)
    Identifier(String),
    /// AssignIdentifier(so_far)
    AssignIdentifier(String),
    /// Start of function definition
    FunctionStart,
    /// End of function definition
    FunctionEnd,
}
impl TokenScanState {
    pub(crate) fn scan_first(c: char) -> TokenScanState {
        use self::TokenScanState::*;
        match c {
            '(' => Comment(0),
            '/' => AssignIdentifier(String::new()),
            '{' => FunctionStart,
            '}' => FunctionEnd,
            chr => Identifier(chr.to_string()),
        }
    }

    fn scan_join(self, c: char) -> Result<(TokenScanState, TokenScanAction), ()> {
        use self::TokenScanAction::*;
        use self::TokenScanState::*;

        match self.clone() {
            Comment(cmt) => match c {
                ')' => if cmt == 0 {
                    Ok((Comment(0), DoneConsumeThis))
                } else {
                    Ok((Comment(cmt - 1), Continue))
                },
                '(' => Ok((Comment(cmt + 1), Continue)),
                _ => Ok((Comment(cmt), Continue)),
            },
            Identifier(ident) => if c.is_whitespace() || c == '{' || c == '}' {
                Ok((self, DoneContinueHere))
            } else {
                Ok((Identifier(format!("{}{}", ident, c)), Continue))
            },
            AssignIdentifier(ident) => if c.is_whitespace() || c == '{' || c == '}' {
                Ok((self, DoneContinueHere))
            } else {
                Ok((AssignIdentifier(format!("{}{}", ident, c)), Continue))
            },
            FunctionStart => Ok((FunctionStart, DoneContinueHere)),
            FunctionEnd => Ok((FunctionEnd, DoneContinueHere)),
        }
    }
}

pub(crate) fn scan_token(input: &mut Peekable<Chars<'_>>) -> Result<TokenScanState, ()> {
    use self::TokenScanAction::*;

    let mut state: Option<TokenScanState> = None;

    while input.peek().ok_or(())?.is_whitespace() {
        input.next();
    }

    while let Some(&c) = input.peek() {
        // println!("> {:?} : {:?}", c, state.clone());
        state = Some(if let Some(s) = state.clone() {
            let (new_state, action) = s.scan_join(c)?;

            if action != DoneContinueHere {
                input.next();
            }

            if action != Continue {
                return Ok(new_state);
            }

            new_state
        } else {
            input.next();
            TokenScanState::scan_first(c)
        });
    }

    state.ok_or(())
}
