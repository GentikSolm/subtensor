use super::*;
use frame_support::pallet_prelude::{Decode, Encode};
use substrate_fixed::types::U64F64;
use codec::Compact;

/// Struct to hold information about a child neuron
#[derive(Decode, Encode, PartialEq, Eq, Clone, Debug)]
pub struct ChildInfo<T: Config> {
    /// The AccountId of the child neuron
    child_ss58: T::AccountId,
    /// The proportion of stake allocated to this child
    proportion: Compact<u64>,
    /// The total stake of the child (including its own children and parents)
    total_stake: Compact<u64>,
    /// The emissions per day for this child
    emissions_per_day: Compact<u64>,
    /// The return per 1000 TAO staked for this child
    return_per_1000: Compact<u64>,
    /// The take (commission) of the child
    take: Compact<u16>,
}

impl<T: Config> Pallet<T> {
    /// Get information about all children for a given network
    ///
    /// # Arguments
    ///
    /// * `netuid` - The network ID to query
    ///
    /// # Returns
    ///
    /// * `Vec<(T::AccountId, Vec<ChildInfo<T>>)>` - A vector of tuples containing the parent AccountId and a vector of ChildInfo for its children
    pub fn get_children_info(netuid: u16) -> Vec<(T::AccountId, Vec<ChildInfo<T>>)> {
        let mut children_info = Vec::new();

        // Iterate through all accounts that have children in this network
        for (parent, children) in ChildKeys::<T>::iter_prefix(netuid) {
            let mut parent_children_info = Vec::new();

            for (proportion, child) in children {
                let child_info = Self::get_child_info(netuid, &parent, &child, proportion);
                parent_children_info.push(child_info);
            }

            children_info.push((parent, parent_children_info));
        }

        children_info
    }

    /// Helper function to get information about a single child neuron
    ///
    /// This function calculates and returns detailed information about a child neuron,
    /// including its stake, emissions, returns, and other relevant data.
    ///
    /// # Arguments
    ///
    /// * `netuid` - The network ID
    /// * `parent` - The AccountId of the parent neuron
    /// * `child` - The AccountId of the child neuron
    /// * `proportion` - The proportion of stake allocated to this child
    ///
    /// # Returns
    ///
    /// * `ChildInfo<T>` - A struct containing detailed information about the child neuron
    pub fn get_child_info(
        netuid: u16,
        parent: &T::AccountId,
        child: &T::AccountId,
        proportion: u64,
    ) -> ChildInfo<T> {
        // Calculate the total stake for the child, including its own children and parents
        let total_stake: u64 = Self::get_stake_with_children_and_parents(child, netuid);
        
        // Get the UID for the child neuron, defaulting to 0 if not found
        let uid: u16 = Self::get_uid_for_net_and_hotkey(netuid, child).unwrap_or(0);
        
        // Calculate emissions per day
        let emission: U64F64 = Self::get_emission_for_uid(netuid, uid).into();
        let tempo: U64F64 = Self::get_tempo(netuid).into();
        let epochs_per_day: U64F64 = U64F64::from_num(7200) / tempo;
        let emissions_per_day: u64 = U64F64::to_num::<u64>(emission * epochs_per_day);

        // Calculate return per 1000 TAO staked
        let return_per_1000: u64 = if total_stake > 0 {
            let total_stake_f64: U64F64 = total_stake.into();
            U64F64::to_num::<u64>((U64F64::from_num(emissions_per_day) * U64F64::from_num(1000)) / total_stake_f64)
        } else {
            0
        };

        // Get the take (commission) for the child
        let take: u16 = <Delegates<T>>::get(child.clone());

        // Construct and return the ChildInfo struct
        ChildInfo {
            child_ss58: child.clone(),
            proportion: proportion.into(),
            total_stake: total_stake.into(),
            emissions_per_day: emissions_per_day.into(),
            return_per_1000: return_per_1000.into(),
            take: take.into(),
        }
    }
}