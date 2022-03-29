use std::io::{self, Error, ErrorKind};

use crate::{codec, ids};

/// ref. https://pkg.go.dev/github.com/ava-labs/avalanchego/vms/avm#FxCredential
/// ref. https://pkg.go.dev/github.com/ava-labs/avalanchego/vms/components/verify#Verifiable
/// ref. https://pkg.go.dev/github.com/ava-labs/avalanchego/vms/secp256k1fx#Credential
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Credential {
    /// Signatures, each must be length of 65.
    /// ref. https://pkg.go.dev/github.com/ava-labs/avalanchego/utils/crypto#SECP256K1RSigLen
    pub sigs: Vec<Vec<u8>>,
}

impl Default for Credential {
    fn default() -> Self {
        Self::default()
    }
}

impl Credential {
    pub fn default() -> Self {
        Self { sigs: Vec::new() }
    }

    pub fn new(sigs: Vec<Vec<u8>>) -> Self {
        Self { sigs }
    }

    pub fn type_name() -> String {
        "secp256k1fx.Credential".to_string()
    }

    pub fn type_id() -> io::Result<u32> {
        if let Some(type_id) = codec::WALLET_X_TYPES.get("secp256k1fx.Credential") {
            Ok((*type_id) as u32)
        } else {
            return Err(Error::new(
                ErrorKind::InvalidInput,
                format!("type_id not found for {}", Self::type_name()),
            ));
        }
    }
}

/// ref. https://pkg.go.dev/github.com/ava-labs/avalanchego/vms/secp256k1fx#OutputOwners
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct OutputOwners {
    pub locktime: u64,
    pub threshold: u32,
    pub addrs: Vec<ids::ShortId>,
}

impl Default for OutputOwners {
    fn default() -> Self {
        Self::default()
    }
}

impl OutputOwners {
    pub fn default() -> Self {
        Self {
            locktime: 0,
            threshold: 0,
            addrs: Vec::new(),
        }
    }

    pub fn new(locktime: u64, threshold: u32, addrs: &[ids::ShortId]) -> Self {
        Self {
            locktime,
            threshold,
            addrs: Vec::from(addrs),
        }
    }

    pub fn type_name() -> String {
        "secp256k1fx.OutputOwners".to_string()
    }

    pub fn type_id() -> io::Result<u32> {
        if let Some(type_id) = codec::WALLET_P_TYPES.get("secp256k1fx.OutputOwners") {
            Ok((*type_id) as u32)
        } else {
            return Err(Error::new(
                ErrorKind::InvalidInput,
                format!("type_id not found for {}", Self::type_name()),
            ));
        }
    }
}

/// ref. https://pkg.go.dev/github.com/ava-labs/avalanchego/vms/components/avax#TransferableOutput
/// ref. https://pkg.go.dev/github.com/ava-labs/avalanchego/vms/components/avax#TransferableOut
/// ref. https://pkg.go.dev/github.com/ava-labs/avalanchego/vms/secp256k1fx#TransferOutput
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct TransferOutput {
    pub amount: u64,
    pub output_owners: OutputOwners,
}

impl Default for TransferOutput {
    fn default() -> Self {
        Self::default()
    }
}

impl TransferOutput {
    pub fn default() -> Self {
        Self {
            amount: 0,
            output_owners: OutputOwners::default(),
        }
    }

    pub fn new(amount: u64, output_owners: OutputOwners) -> Self {
        Self {
            amount,
            output_owners,
        }
    }

    pub fn type_name() -> String {
        "secp256k1fx.TransferOutput".to_string()
    }

    pub fn type_id() -> io::Result<u32> {
        if let Some(type_id) = codec::WALLET_X_TYPES.get("secp256k1fx.TransferOutput") {
            Ok((*type_id) as u32)
        } else {
            return Err(Error::new(
                ErrorKind::InvalidInput,
                format!("type_id not found for {}", Self::type_name()),
            ));
        }
    }
}

/// ref. https://pkg.go.dev/github.com/ava-labs/avalanchego/vms/components/avax#TransferableInput
/// ref. https://pkg.go.dev/github.com/ava-labs/avalanchego/vms/components/avax#TransferableIn
/// ref. https://pkg.go.dev/github.com/ava-labs/avalanchego/vms/secp256k1fx#TransferInput
/// ref. https://pkg.go.dev/github.com/ava-labs/avalanchego/vms/secp256k1fx#Input
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct TransferInput {
    pub amount: u64,
    pub sig_indices: Vec<u32>,
}

impl Default for TransferInput {
    fn default() -> Self {
        Self::default()
    }
}

impl TransferInput {
    pub fn default() -> Self {
        Self {
            amount: 0,
            sig_indices: Vec::new(),
        }
    }

    pub fn new(amount: u64, sig_indices: Vec<u32>) -> Self {
        Self {
            amount,
            sig_indices,
        }
    }

    pub fn type_name() -> String {
        "secp256k1fx.TransferInput".to_string()
    }

    pub fn type_id() -> io::Result<u32> {
        if let Some(type_id) = codec::WALLET_X_TYPES.get("secp256k1fx.TransferInput") {
            Ok((*type_id) as u32)
        } else {
            return Err(Error::new(
                ErrorKind::InvalidInput,
                format!("type_id not found for {}", Self::type_name()),
            ));
        }
    }

    pub fn verify(&self) -> io::Result<()> {
        if self.amount == 0 {
            return Err(Error::new(
                ErrorKind::InvalidInput,
                "input has no value", // ref. "errNoValueInput"
            ));
        }
        if !is_sorted_and_unique(&self.sig_indices) {
            return Err(Error::new(
                ErrorKind::InvalidInput,
                "signatures not sorted and unique", // ref. "errNotSortedUnique"
            ));
        }
        Ok(())
    }

    /// ref. "vms/secp256k1fx.Input.Cost"
    pub fn sig_costs(&self) -> u64 {
        let sigs = self.sig_indices.len();
        (sigs as u64) * 1000
    }
}

fn is_sorted_and_unique<T>(data: &[T]) -> bool
where
    T: Ord,
{
    data.windows(2).all(|w| w[0] < w[1])
}

/// ref. https://pkg.go.dev/github.com/ava-labs/avalanchego/vms/secp256k1fx#Input
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Input {
    pub sig_indices: Vec<u32>,
}

impl Default for Input {
    fn default() -> Self {
        Self::default()
    }
}

impl Input {
    pub fn default() -> Self {
        Self {
            sig_indices: Vec::new(),
        }
    }

    pub fn new(sig_indices: Vec<u32>) -> Self {
        Self { sig_indices }
    }

    pub fn type_name() -> String {
        "secp256k1fx.Input".to_string()
    }

    pub fn type_id() -> io::Result<u32> {
        if let Some(type_id) = codec::WALLET_P_TYPES.get("secp256k1fx.Input") {
            Ok((*type_id) as u32)
        } else {
            return Err(Error::new(
                ErrorKind::InvalidInput,
                format!("type_id not found for {}", Self::type_name()),
            ));
        }
    }
}