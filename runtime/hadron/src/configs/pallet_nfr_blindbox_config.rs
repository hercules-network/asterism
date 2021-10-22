use crate::*;
use pallet_nfr_blindbox::Config;

impl Config for Runtime {
    type LockModuleId = AsterismBlindBoxModuleId;
    type NfrHandler = Nfr;
    type Event = Event;
    type Randomness = RandomnessCollectiveFlip;
    type WeightInfo = ();
}
