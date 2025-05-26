use landlock::{AccessFs, RestrictionStatus, Ruleset, RulesetAttr, RulesetError, ABI};

pub struct LandlockConfiner {}

impl LandlockConfiner {
    pub fn new() -> LandlockConfiner {
        Self {}
    }

    pub fn confine(&self) -> Result<RestrictionStatus, RulesetError> {
        let abi = ABI::V6;
        let ruleset = Ruleset::default();
        // TODO do a better job at confining
        ruleset
            .handle_access(AccessFs::from_read(abi))?
            .create()?
            .restrict_self()
    }
}
