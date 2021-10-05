use crate::*;
use pallet_nft_multi::Config;

/// Used for the module nft-multi in `./nft-multi.rs`
impl Config for Runtime {
    type ModuleId = AsterismNftModuleId;
    type MultiCurrency = Currencies;
    type Event = Event;
    type WeightInfo = ();
}
