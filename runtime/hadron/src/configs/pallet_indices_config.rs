use crate::{weights::pallet_indices::WeightInfo, *};
use lyra_primitives::*;
use pallet_indices::Config;

frame_support::parameter_types! {
    pub const IndexDeposit: Balance = 1 * LYR;
}
impl Config for Runtime {
    type AccountIndex = AccountIndex;
    type Currency = Lyr;
    type Deposit = IndexDeposit;
    type Event = Event;
    type WeightInfo = WeightInfo<Runtime>;
}
