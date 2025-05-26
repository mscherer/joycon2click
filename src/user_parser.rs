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
            match User::from_name(user) {
                Err(e) => {
                    return Err(err);
                }
                Ok(None) => {
                    return Err(err);
                }
                Ok(Some(u)) => {
                    return Ok(ParsedUser(u));
                }
            }
        }
        return Err(err);
    }
}
