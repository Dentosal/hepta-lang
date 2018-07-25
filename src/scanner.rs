use std::iter::Peekable;
use std::str::Chars;

#[derive(Debug, Clone)]
pub(crate) enum TokenScanState {
    /// Comment(nesting_level)
    Comment(u8),
    /// DecInteger(positive, so_far)
    DecInteger(bool, u64),
    /// HexInteger(so_far)
    HexInteger(u64),
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
            '-' => DecInteger(false, 0),
            '0' => DecInteger(true, 0),
            '/' => AssignIdentifier(String::new()),
            '{' => FunctionStart,
            '}' => FunctionEnd,
            chr => Identifier(chr.to_string()),
        }
    }

    /// Ok(state) means continue scanning, Err(Some(state)) means done, Err(None) means error
    pub(crate) fn scan_join(self, c: char) -> Result<TokenScanState, Option<TokenScanState>> {
        use self::TokenScanState::*;
        match self {
            Comment(cmt) => match c {
                ')' => if cmt == 0 {
                    Err(Some(Comment(0)))
                } else {
                    Ok(Comment(cmt - 1))
                },
                '(' => Ok(Comment(cmt + 1)),
                _ => Ok(Comment(cmt)),
            },
            DecInteger(true, 0) => match c {
                'x' => Ok(HexInteger(0)),
                '1'..='9' => Ok(DecInteger(
                    true,
                    u64::from_str_radix(&c.to_string(), 10).unwrap(),
                )),
                _ => Err(None),
            },
            DecInteger(false, 0) => match c {
                '1'..='9' => Ok(DecInteger(
                    false,
                    u64::from_str_radix(&c.to_string(), 10).unwrap(),
                )),
                _ => Err(None),
            },
            DecInteger(neg, int) => match c {
                '0'..='9' => Ok(DecInteger(
                    neg,
                    int * 10 + u64::from_str_radix(&c.to_string(), 10).unwrap(),
                )),
                _ => Err(None),
            },
            HexInteger(int) => match c {
                '0'..='9' => Ok(HexInteger(
                    int * 0x10 + u64::from_str_radix(&c.to_string(), 10).unwrap(),
                )),
                _ => Err(None),
            },
            Identifier(ident) => if c.is_whitespace() {
                Err(Some(Identifier(ident)))
            } else {
                Ok(Identifier(format!("{}{}", ident, c)))
            },
            AssignIdentifier(ident) => if c.is_whitespace() {
                Err(Some(AssignIdentifier(ident)))
            } else {
                Ok(AssignIdentifier(format!("{}{}", ident, c)))
            },
            FunctionStart => Err(Some(FunctionStart)),
            FunctionEnd => Err(Some(FunctionEnd)),
        }
    }
}

pub(crate) fn scan_token(input: &mut Peekable<Chars>) -> Option<TokenScanState> {
    let mut state: Option<TokenScanState> = None;

    while input.peek()?.is_whitespace() {
        input.next();
    }

    for c in input {
        state = if let Some(s) = state.clone() {
            match s.scan_join(c) {
                Ok(new_state) => Some(new_state),
                Err(opt) => match opt {
                    Some(new_state) => {
                        break;
                    }
                    None => {
                        // TODO: Error
                        return None;
                    }
                },
            }
        } else {
            Some(TokenScanState::scan_first(c))
        };
    }

    state
}
