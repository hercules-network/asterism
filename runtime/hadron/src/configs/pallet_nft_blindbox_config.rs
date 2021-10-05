use crate::*;
use pallet_nft_blindbox::Config;

impl Config for Runtime {
    type LockModuleId = AsterismBlindBoxModuleId;
    type NftHandler = Nft;
    type Event = Event;
    type Randomness = RandomnessCollectiveFlip;
    type WeightInfo = ();
}
