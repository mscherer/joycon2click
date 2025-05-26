use clap::error::{ContextKind, ContextValue, ErrorKind};
use nix::unistd::setuid;
use nix::unistd::User;
use std::fmt;

#[derive(Clone, Debug)]
pub struct ParsedUser(User);

impl ParsedUser {
    pub fn setuid(&self) -> Result<(), nix::errno::Errno> {
        setuid(self.0.uid)
    }
}

impl clap::builder::ValueParserFactory for ParsedUser {
    type Parser = ParsedUserParser;
    fn value_parser() -> Self::Parser {
        ParsedUserParser
    }
}

impl fmt::Display for ParsedUser {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0.name)
    }
}

#[derive(Clone, Debug)]
pub struct ParsedUserParser;

impl clap::builder::TypedValueParser for ParsedUserParser {
    type Value = ParsedUser;

    fn parse_ref(
        &self,
        cmd: &clap::Command,
        arg: Option<&clap::Arg>,
        value: &std::ffi::OsStr,
    ) -> Result<Self::Value, clap::Error> {
        let mut err = clap::Error::new(ErrorKind::ValueValidation).with_cmd(cmd);
        if let Some(arg) = arg {
            err.insert(
                ContextKind::InvalidArg,
                ContextValue::String(arg.to_string()),
            );
        }
        if let Some(user) = value.to_str() {
            err.insert(
                ContextKind::InvalidValue,
                ContextValue::String(user.to_string()),
            );
            // TODO seems ContextKind::Custom is unused
            // https://github.com/clap-rs/clap/discussions/5318#discussioncomment-8185351
            match User::from_name(user) {
                Err(e) => {
                    err.insert(
                        ContextKind::Custom,
                        ContextValue::String(format!("{:?}", e)),
                    );
                    return Err(err);
                }
                Ok(None) => {
                    err.insert(
                        ContextKind::Custom,
                        ContextValue::String(format!("User {user} not found")),
                    );
                    return Err(err);
                }
                Ok(Some(u)) => {
                    return Ok(ParsedUser(u));
                }
            }
        }
        Err(err)
    }
}
