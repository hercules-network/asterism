use crate::*;
use pallet_nfr::Config;

/// Used for the module nfr in `./nfr.rs`
impl Config for Runtime {
    type ModuleId = AsterismNfrModuleId;
    type Currency = Lyr;
    type Event = Event;
    type WeightInfo = ();
}
