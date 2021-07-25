//! Consensus logic and parameters.

use std::cmp::{Ord, Ordering};
use std::convert::TryFrom;
use std::fmt;
use std::ops::{Add, Sub};

use crate::constants;

/// A wrapper type representing blockchain heights. Safe conversion from
/// various integer types, as well as addition and subtraction, are provided.
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct BlockHeight(u32);

pub const H0: BlockHeight = BlockHeight(0);

impl BlockHeight {
    pub const fn from_u32(v: u32) -> BlockHeight {
        BlockHeight(v)
    }
}

impl fmt::Display for BlockHeight {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(formatter)
    }
}

impl Ord for BlockHeight {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.cmp(&other.0)
    }
}

impl PartialOrd for BlockHeight {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl From<u32> for BlockHeight {
    fn from(value: u32) -> Self {
        BlockHeight(value)
    }
}

impl From<BlockHeight> for u32 {
    fn from(value: BlockHeight) -> u32 {
        value.0
    }
}

impl TryFrom<u64> for BlockHeight {
    type Error = std::num::TryFromIntError;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        u32::try_from(value).map(BlockHeight)
    }
}

impl From<BlockHeight> for u64 {
    fn from(value: BlockHeight) -> u64 {
        value.0 as u64
    }
}

impl TryFrom<i32> for BlockHeight {
    type Error = std::num::TryFromIntError;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        u32::try_from(value).map(BlockHeight)
    }
}

impl TryFrom<i64> for BlockHeight {
    type Error = std::num::TryFromIntError;

    fn try_from(value: i64) -> Result<Self, Self::Error> {
        u32::try_from(value).map(BlockHeight)
    }
}

impl From<BlockHeight> for i64 {
    fn from(value: BlockHeight) -> i64 {
        value.0 as i64
    }
}

impl Add<u32> for BlockHeight {
    type Output = Self;

    fn add(self, other: u32) -> Self {
        BlockHeight(self.0 + other)
    }
}

impl Add for BlockHeight {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        self + other.0
    }
}

impl Sub<u32> for BlockHeight {
    type Output = Self;

    fn sub(self, other: u32) -> Self {
        if other > self.0 {
            panic!("Subtraction resulted in negative block height.");
        }

        BlockHeight(self.0 - other)
    }
}

impl Sub for BlockHeight {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        self - other.0
    }
}

/// Zcash consensus parameters.
pub trait Parameters: Clone {
    /// Returns the activation height for a particular network upgrade,
    /// if an activation height has been set.
    fn activation_height(&self, nu: NetworkUpgrade) -> Option<BlockHeight>;

    /// Determines whether the specified network upgrade is active as of the
    /// provided block height on the network to which this Parameters value applies.
    fn is_nu_active(&self, nu: NetworkUpgrade, height: BlockHeight) -> bool {
        self.activation_height(nu).map_or(false, |h| h <= height)
    }

    /// The coin type for ZEC, as defined by [SLIP 44].
    ///
    /// [SLIP 44]: https://github.com/satoshilabs/slips/blob/master/slip-0044.md
    fn coin_type(&self) -> u32;

    /// Returns the human-readable prefix for Bech32-encoded Sapling extended spending keys
    /// the network to which this Parameters value applies.
    ///
    /// Defined in [ZIP 32].
    ///
    /// [`ExtendedSpendingKey`]: zcash_primitives::zip32::ExtendedSpendingKey
    /// [ZIP 32]: https://github.com/zcash/zips/blob/master/zip-0032.rst
    fn hrp_sapling_extended_spending_key(&self) -> &str;

    /// Returns the human-readable prefix for Bech32-encoded Sapling extended full
    /// viewing keys for the network to which this Parameters value applies.
    ///
    /// Defined in [ZIP 32].
    ///
    /// [`ExtendedFullViewingKey`]: zcash_primitives::zip32::ExtendedFullViewingKey
    /// [ZIP 32]: https://github.com/zcash/zips/blob/master/zip-0032.rst
    fn hrp_sapling_extended_full_viewing_key(&self) -> &str;

    /// Returns the Bech32-encoded human-readable prefix for Sapling payment addresses
    /// viewing keys for the network to which this Parameters value applies.
    ///
    /// Defined in section 5.6.4 of the [Zcash Protocol Specification].
    ///
    /// [`PaymentAddress`]: zcash_primitives::primitives::PaymentAddress
    /// [Zcash Protocol Specification]: https://github.com/zcash/zips/blob/master/protocol/protocol.pdf
    fn hrp_sapling_payment_address(&self) -> &str;

    /// Returns the human-readable prefix for Base58Check-encoded transparent
    /// pay-to-public-key-hash payment addresses for the network to which this Parameters value
    /// applies.
    ///
    /// [`TransparentAddress::PublicKey`]: zcash_primitives::legacy::TransparentAddress::PublicKey
    fn b58_pubkey_address_prefix(&self) -> [u8; 2];

    /// Returns the human-readable prefix for Base58Check-encoded transparent pay-to-script-hash
    /// payment addresses for the network to which this Parameters value applies.
    ///
    /// [`TransparentAddress::Script`]: zcash_primitives::legacy::TransparentAddress::Script
    fn b58_script_address_prefix(&self) -> [u8; 2];
}

/// Marker struct for the production network.
#[derive(PartialEq, Copy, Clone, Debug)]
pub struct MainNetwork;

pub const MAIN_NETWORK: MainNetwork = MainNetwork;

impl Parameters for MainNetwork {
    fn activation_height(&self, nu: NetworkUpgrade) -> Option<BlockHeight> {
        match nu {
            NetworkUpgrade::Overwinter => Some(BlockHeight(347_500)),
            NetworkUpgrade::Sapling => Some(BlockHeight(419_200)),
            NetworkUpgrade::Ycash => Some(BlockHeight(570_000)),
            NetworkUpgrade::Blossom => Some(BlockHeight(10_000_000)),
            NetworkUpgrade::Heartwood => Some(BlockHeight(20_000_000)),
            NetworkUpgrade::Canopy => Some(BlockHeight(30_000_000)),
            #[cfg(feature = "zfuture")]
            NetworkUpgrade::ZFuture => None,
        }
    }

    fn coin_type(&self) -> u32 {
        constants::mainnet::COIN_TYPE
    }

    fn hrp_sapling_extended_spending_key(&self) -> &str {
        constants::mainnet::HRP_SAPLING_EXTENDED_SPENDING_KEY
    }

    fn hrp_sapling_extended_full_viewing_key(&self) -> &str {
        constants::mainnet::HRP_SAPLING_EXTENDED_FULL_VIEWING_KEY
    }

    fn hrp_sapling_payment_address(&self) -> &str {
        constants::mainnet::HRP_SAPLING_PAYMENT_ADDRESS
    }

    fn b58_pubkey_address_prefix(&self) -> [u8; 2] {
        constants::mainnet::B58_PUBKEY_ADDRESS_PREFIX
    }

    fn b58_script_address_prefix(&self) -> [u8; 2] {
        constants::mainnet::B58_SCRIPT_ADDRESS_PREFIX
    }
}

/// Marker struct for the test network.
#[derive(PartialEq, Copy, Clone, Debug)]
pub struct TestNetwork;

pub const TEST_NETWORK: TestNetwork = TestNetwork;

impl Parameters for TestNetwork {
    fn activation_height(&self, nu: NetworkUpgrade) -> Option<BlockHeight> {
        match nu {
            NetworkUpgrade::Overwinter => Some(BlockHeight(207_500)),
            NetworkUpgrade::Sapling => Some(BlockHeight(280_000)),
            NetworkUpgrade::Ycash => Some(BlockHeight(510_248)),
            NetworkUpgrade::Blossom => Some(BlockHeight(10_000_000)),
            NetworkUpgrade::Heartwood => Some(BlockHeight(20_000_000)),
            NetworkUpgrade::Canopy => Some(BlockHeight(30_000_000)),
            #[cfg(feature = "zfuture")]
            NetworkUpgrade::ZFuture => None,
        }
    }

    fn coin_type(&self) -> u32 {
        constants::testnet::COIN_TYPE
    }

    fn hrp_sapling_extended_spending_key(&self) -> &str {
        constants::testnet::HRP_SAPLING_EXTENDED_SPENDING_KEY
    }

    fn hrp_sapling_extended_full_viewing_key(&self) -> &str {
        constants::testnet::HRP_SAPLING_EXTENDED_FULL_VIEWING_KEY
    }

    fn hrp_sapling_payment_address(&self) -> &str {
        constants::testnet::HRP_SAPLING_PAYMENT_ADDRESS
    }

    fn b58_pubkey_address_prefix(&self) -> [u8; 2] {
        constants::testnet::B58_PUBKEY_ADDRESS_PREFIX
    }

    fn b58_script_address_prefix(&self) -> [u8; 2] {
        constants::testnet::B58_SCRIPT_ADDRESS_PREFIX
    }
}

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum Network {
    MainNetwork,
    TestNetwork,
}

impl Parameters for Network {
    fn activation_height(&self, nu: NetworkUpgrade) -> Option<BlockHeight> {
        match self {
            Network::MainNetwork => MAIN_NETWORK.activation_height(nu),
            Network::TestNetwork => TEST_NETWORK.activation_height(nu),
        }
    }

    fn coin_type(&self) -> u32 {
        match self {
            Network::MainNetwork => MAIN_NETWORK.coin_type(),
            Network::TestNetwork => TEST_NETWORK.coin_type(),
        }
    }

    fn hrp_sapling_extended_spending_key(&self) -> &str {
        match self {
            Network::MainNetwork => MAIN_NETWORK.hrp_sapling_extended_spending_key(),
            Network::TestNetwork => TEST_NETWORK.hrp_sapling_extended_spending_key(),
        }
    }

    fn hrp_sapling_extended_full_viewing_key(&self) -> &str {
        match self {
            Network::MainNetwork => MAIN_NETWORK.hrp_sapling_extended_full_viewing_key(),
            Network::TestNetwork => TEST_NETWORK.hrp_sapling_extended_full_viewing_key(),
        }
    }

    fn hrp_sapling_payment_address(&self) -> &str {
        match self {
            Network::MainNetwork => MAIN_NETWORK.hrp_sapling_payment_address(),
            Network::TestNetwork => TEST_NETWORK.hrp_sapling_payment_address(),
        }
    }

    fn b58_pubkey_address_prefix(&self) -> [u8; 2] {
        match self {
            Network::MainNetwork => MAIN_NETWORK.b58_pubkey_address_prefix(),
            Network::TestNetwork => TEST_NETWORK.b58_pubkey_address_prefix(),
        }
    }

    fn b58_script_address_prefix(&self) -> [u8; 2] {
        match self {
            Network::MainNetwork => MAIN_NETWORK.b58_script_address_prefix(),
            Network::TestNetwork => TEST_NETWORK.b58_script_address_prefix(),
        }
    }
}

/// An event that occurs at a specified height on the Zcash chain, at which point the
/// consensus rules enforced by the network are altered.
///
/// See [ZIP 200](https://zips.z.cash/zip-0200) for more details.
#[derive(Clone, Copy, Debug)]
pub enum NetworkUpgrade {
    /// The [Overwinter] network upgrade.
    ///
    /// [Overwinter]: https://z.cash/upgrade/overwinter/
    Overwinter,
    /// The [Sapling] network upgrade.
    ///
    /// [Sapling]: https://z.cash/upgrade/sapling/
    Sapling,
    /// The [Ycash] network upgrade.
    ///
    /// [Ycash]: https://y.cash
    Ycash,
    /// The [Blossom] network upgrade.
    ///
    /// [Blossom]: https://z.cash/upgrade/blossom/
    Blossom,
    /// The [Heartwood] network upgrade.
    ///
    /// [Heartwood]: https://z.cash/upgrade/heartwood/
    Heartwood,
    /// The [Canopy] network upgrade.
    ///
    /// [Canopy]: https://z.cash/upgrade/canopy/
    Canopy,
    /// The ZFUTURE network upgrade.
    ///
    /// This upgrade is expected never to activate on mainnet;
    /// it is intended for use in integration testing of functionality
    /// that is a candidate for integration in a future network upgrade.
    #[cfg(feature = "zfuture")]
    ZFuture,
}

impl fmt::Display for NetworkUpgrade {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            NetworkUpgrade::Overwinter => write!(f, "Overwinter"),
            NetworkUpgrade::Sapling => write!(f, "Sapling"),
            NetworkUpgrade::Ycash => write!(f, "Ycash"),
            NetworkUpgrade::Blossom => write!(f, "Blossom"),
            NetworkUpgrade::Heartwood => write!(f, "Heartwood"),
            NetworkUpgrade::Canopy => write!(f, "Canopy"),
            #[cfg(feature = "zfuture")]
            NetworkUpgrade::ZFuture => write!(f, "ZFUTURE"),
        }
    }
}

impl NetworkUpgrade {
    fn branch_id(self) -> BranchId {
        match self {
            NetworkUpgrade::Overwinter => BranchId::Overwinter,
            NetworkUpgrade::Sapling => BranchId::Sapling,
            NetworkUpgrade::Ycash => BranchId::Ycash,
            NetworkUpgrade::Blossom => BranchId::Blossom,
            NetworkUpgrade::Heartwood => BranchId::Heartwood,
            NetworkUpgrade::Canopy => BranchId::Canopy,
            #[cfg(feature = "zfuture")]
            NetworkUpgrade::ZFuture => BranchId::ZFuture,
        }
    }
}

/// The network upgrades on the Zcash chain in order of activation.
///
/// This order corresponds to the activation heights, but because Rust enums are
/// full-fledged algebraic data types, we need to define it manually.
const UPGRADES_IN_ORDER: &[NetworkUpgrade] = &[
    NetworkUpgrade::Overwinter,
    NetworkUpgrade::Sapling,
    NetworkUpgrade::Ycash,
    NetworkUpgrade::Blossom,
    NetworkUpgrade::Heartwood,
    NetworkUpgrade::Canopy,
];

pub const ZIP212_GRACE_PERIOD: u32 = 32256;

/// A globally-unique identifier for a set of consensus rules within the Zcash chain.
///
/// Each branch ID in this enum corresponds to one of the epochs between a pair of Zcash
/// network upgrades. For example, `BranchId::Overwinter` corresponds to the blocks
/// starting at Overwinter activation, and ending the block before Sapling activation.
///
/// The main use of the branch ID is in signature generation: transactions commit to a
/// specific branch ID by including it as part of [`signature_hash`]. This ensures
/// two-way replay protection for transactions across network upgrades.
///
/// See [ZIP 200](https://zips.z.cash/zip-0200) for more details.
///
/// [`signature_hash`]: crate::transaction::signature_hash
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BranchId {
    /// The consensus rules at the launch of Zcash.
    Sprout,
    /// The consensus rules deployed by [`NetworkUpgrade::Overwinter`].
    Overwinter,
    /// The consensus rules deployed by [`NetworkUpgrade::Sapling`].
    Sapling,
    /// The consensus rules deployed by [`NetworkUpgrade::Ycash`].
    Ycash,
    /// The consensus rules deployed by [`NetworkUpgrade::Blossom`].
    Blossom,
    /// The consensus rules deployed by [`NetworkUpgrade::Heartwood`].
    Heartwood,
    /// The consensus rules deployed by [`NetworkUpgrade::Canopy`].
    Canopy,
    /// Candidates for future consensus rules; this branch will never
    /// activate on mainnet.
    #[cfg(feature = "zfuture")]
    ZFuture,
}

impl TryFrom<u32> for BranchId {
    type Error = &'static str;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(BranchId::Sprout),
            0x5ba8_1b19 => Ok(BranchId::Overwinter),
            0x76b8_09bb => Ok(BranchId::Sapling),
            0x374d_694f => Ok(BranchId::Ycash),
            0x2bb4_0e60 => Ok(BranchId::Blossom),
            0xf5b9_230b => Ok(BranchId::Heartwood),
            0xe9ff_75a6 => Ok(BranchId::Canopy),
            #[cfg(feature = "zfuture")]
            0xffff_ffff => Ok(BranchId::ZFuture),
            _ => Err("Unknown consensus branch ID"),
        }
    }
}

impl From<BranchId> for u32 {
    fn from(consensus_branch_id: BranchId) -> u32 {
        match consensus_branch_id {
            BranchId::Sprout => 0,
            BranchId::Overwinter => 0x5ba8_1b19,
            BranchId::Sapling => 0x76b8_09bb,
            BranchId::Ycash => 0x374d_694f,
            BranchId::Blossom => 0x2bb4_0e60,
            BranchId::Heartwood => 0xf5b9_230b,
            BranchId::Canopy => 0xe9ff_75a6,
            #[cfg(feature = "zfuture")]
            BranchId::ZFuture => 0xffff_ffff,
        }
    }
}

impl BranchId {
    /// Returns the branch ID corresponding to the consensus rule set that is active at
    /// the given height.
    ///
    /// This is the branch ID that should be used when creating transactions.
    pub fn for_height<P: Parameters>(parameters: &P, height: BlockHeight) -> Self {
        for nu in UPGRADES_IN_ORDER.iter().rev() {
            if parameters.is_nu_active(*nu, height) {
                return nu.branch_id();
            }
        }

        // Sprout rules apply before any network upgrade
        BranchId::Sprout
    }
}

#[cfg(test)]
mod tests {
    use std::convert::TryFrom;

    use super::{
        BlockHeight, BranchId, NetworkUpgrade, Parameters, MAIN_NETWORK, UPGRADES_IN_ORDER,
    };

    #[test]
    fn nu_ordering() {
        for i in 1..UPGRADES_IN_ORDER.len() {
            let nu_a = UPGRADES_IN_ORDER[i - 1];
            let nu_b = UPGRADES_IN_ORDER[i];
            match (
                MAIN_NETWORK.activation_height(nu_a),
                MAIN_NETWORK.activation_height(nu_b),
            ) {
                (a, b) if a < b => (),
                _ => panic!(
                    "{} should not be before {} in UPGRADES_IN_ORDER",
                    nu_a, nu_b
                ),
            }
        }
    }

    #[test]
    fn nu_is_active() {
        assert!(!MAIN_NETWORK.is_nu_active(NetworkUpgrade::Overwinter, BlockHeight(0)));
        assert!(!MAIN_NETWORK.is_nu_active(NetworkUpgrade::Overwinter, BlockHeight(347_499)));
        assert!(MAIN_NETWORK.is_nu_active(NetworkUpgrade::Overwinter, BlockHeight(347_500)));
    }

    #[test]
    fn branch_id_from_u32() {
        assert_eq!(BranchId::try_from(0), Ok(BranchId::Sprout));
        assert!(BranchId::try_from(1).is_err());
    }

    #[test]
    fn branch_id_for_height() {
        assert_eq!(
            BranchId::for_height(&MAIN_NETWORK, BlockHeight(0)),
            BranchId::Sprout,
        );
        assert_eq!(
            BranchId::for_height(&MAIN_NETWORK, BlockHeight(419_199)),
            BranchId::Overwinter,
        );
        assert_eq!(
            BranchId::for_height(&MAIN_NETWORK, BlockHeight(419_200)),
            BranchId::Sapling,
        );
        assert_eq!(
            BranchId::for_height(&MAIN_NETWORK, BlockHeight(570_000)),
            BranchId::Ycash,
        );        /*
        assert_eq!(
            BranchId::for_height(&MAIN_NETWORK, BlockHeight(903_000)),
            BranchId::Heartwood,
        );
        assert_eq!(
            BranchId::for_height(&MAIN_NETWORK, BlockHeight(1_046_400)),
            BranchId::Canopy,
        );
        assert_eq!(
            BranchId::for_height(&MAIN_NETWORK, BlockHeight(5_000_000)),
            BranchId::Canopy,
        );
        */
    }
}
