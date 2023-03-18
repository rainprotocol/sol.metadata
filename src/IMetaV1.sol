// SPDX-License-Identifier: CAL
pragma solidity ^0.8.16;

/// @dev Randomly generated magic number with first bytes oned out.
/// https://github.com/rainprotocol/specs/blob/main/metadata-v1.md
uint64 constant META_MAGIC_NUMBER_V1 = 0xff0a89c674ee7874;

/// @title IMetaV1
interface IMetaV1 {
    /// An onchain wrapper to carry arbitrary Rain metadata. Assigns the sender
    /// to the metadata so that tooling can easily drop/ignore data from unknown
    /// sources.
    /// @param sender The msg.sender.
    /// @param meta Rain metadata V1 compliant metadata bytes.
    /// https://github.com/rainprotocol/specs/blob/main/metadata-v1.md
    event MetaV1(address sender, bytes meta);
}
