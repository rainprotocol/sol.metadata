// SPDX-License-Identifier: CAL
pragma solidity ^0.8.16;

import "./IMetaV1.sol";

/// Defines a general purpose contract that anon may call to emit ANY metadata.
/// Anons MAY send garbage and malicious metadata so it is up to tooling to
/// discard any suspect data before use, and generally treat it all as untrusted.
interface IMetaBoardV1 is IMetaV1 {
    /// Emit a single MetaV1 event. Typically this is sufficient for most use
    /// cases as a single MetaV1 event can contain many metas as a single
    /// cbor-seq. Metadata MUST match the metadata V1 specification for Rain
    /// metadata or tooling MAY drop it. `IMetaBoardV1` contracts MUST revert any
    /// metadata that does not start with the Rain metadata magic number.
    /// @param meta Bytes to be emitted as metadata.
    function emitMeta(bytes calldata meta) external;

    /// Emit many MetaV1 events. Identical to `emitMeta` but allowing a batch of
    /// meta events to be emitted. Consider emitting a single meta under a
    /// cbor-seq rather than many events as it will usually be more efficient.
    /// `IMetaBoardV1` contracts MUST revert any metadata that does not start
    /// with the Rain metadata magic number.
    /// @param metas Array of bytes to be emitted as metadata.
    function emitMetas(bytes[] calldata metas) external;
}
