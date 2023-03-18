// SPDX-License-Identifier: CAL
pragma solidity ^0.8.16;

/// @dev Randomly generated magic number with first bytes oned out.
/// https://github.com/rainprotocol/specs/blob/main/metadata-v1.md
uint64 constant META_MAGIC_NUMBER_V1 = 0xff0a89c674ee7874;

/// @title IMetaV1
interface IMetaV1 {
    event MetaV1(address sender, bytes meta);
}
