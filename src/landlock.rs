use landlock::{
    Access, AccessFs, AccessNet, RestrictionStatus, Ruleset, RulesetAttr, RulesetError, Scope, ABI,
};

pub struct LandlockConfiner {}

impl LandlockConfiner {
    pub fn new() -> LandlockConfiner {
        Self {}
    }

    pub fn confine(&self) -> Result<RestrictionStatus, RulesetError> {
        // TODO add prctl(PR_SET_NO_NEW_PRIVS)
        let abi = ABI::V6;
        let ruleset = Ruleset::default();
        // TODO let open /dev/input
        ruleset
            .handle_access(AccessFs::from_all(abi))?
            .handle_access(AccessNet::from_all(abi))?
            .scope(Scope::Signal)?
            .scope(Scope::AbstractUnixSocket)?
            .create()?
            .restrict_self()
    }
}
