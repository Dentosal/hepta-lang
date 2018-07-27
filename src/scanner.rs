use std::iter::Peekable;
use std::str::Chars;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum Token {
    Identifier(String),
    AssignIdentifier(String),
    SetNamespace(String),
    FunctionStart,
    FunctionEnd,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TokenScanAction {
    Continue,
    DoneConsumeThis,
    DoneContinueHere,
}

#[derive(Debug, Clone)]
enum TokenScanState {
    /// Comment(nesting_level)
    Comment(u8),
    /// Identifier(so_far)
    Identifier(String),
    /// AssignIdentifier(so_far)
    AssignIdentifier(String),
    /// SetNamespace(so_far)
    SetNamespace(String),
    /// Start of function definition
    FunctionStart,
    /// End of function definition
    FunctionEnd,
}
impl TokenScanState {
    fn scan_first(c: char) -> Self {
        use self::TokenScanState::*;
        match c {
            '(' => Comment(0),
            '{' => FunctionStart,
            '}' => FunctionEnd,
            '/' => AssignIdentifier(String::new()),
            '#' => SetNamespace(String::new()),
            chr => Identifier(chr.to_string()),
        }
    }

    fn scan_join(self, c: char) -> Result<(Self, TokenScanAction), ()> {
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
            SetNamespace(ident) => if c.is_whitespace() || c == '{' || c == '}' {
                Ok((self, DoneContinueHere))
            } else {
                Ok((SetNamespace(format!("{}{}", ident, c)), Continue))
            },
            FunctionStart => Ok((FunctionStart, DoneContinueHere)),
            FunctionEnd => Ok((FunctionEnd, DoneContinueHere)),
        }
    }

    fn to_token(&self) -> Option<Token> {
        use self::TokenScanState::*;
        match self {
            Comment(_) => None,
            Identifier(ident) => Some(Token::Identifier(ident.clone())),
            AssignIdentifier(ident) => Some(Token::AssignIdentifier(ident.clone())),
            SetNamespace(ident) => Some(Token::SetNamespace(ident.clone())),
            FunctionStart => Some(Token::FunctionStart),
            FunctionEnd => Some(Token::FunctionEnd),
        }
    }
}

fn scan_one_token(input: &mut Peekable<Chars<'_>>) -> Result<TokenScanState, ()> {
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

pub(crate) fn scan_token(mut input: &mut Peekable<Chars<'_>>) -> Result<Token, ()> {
    loop {
        if let Some(token) = scan_one_token(&mut input)?.to_token() {
            return Ok(token);
        }
    }
}
