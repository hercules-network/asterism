use crate::*;
use pallet_nfr_multi::Config;

/// Used for the module nfr-multi in `./nfr-multi.rs`
impl Config for Runtime {
    type ModuleId = AsterismNfrModuleId;
    type MultiCurrency = Currencies;
    type Event = Event;
    type WeightInfo = ();
}
