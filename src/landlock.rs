use landlock::{
    Access, AccessFs, AccessNet, RestrictionStatus, Ruleset, RulesetAttr, RulesetCreatedAttr,
    RulesetError, RulesetStatus, Scope, ABI,
};

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
