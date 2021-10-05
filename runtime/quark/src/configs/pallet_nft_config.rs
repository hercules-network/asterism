use crate::*;
use pallet_nft::Config;

/// Used for the module nft in `./nft.rs`
impl Config for Runtime {
    type ModuleId = AsterismNftModuleId;
    type Currency = Lyr;
    type Event = Event;
    type WeightInfo = ();
}
