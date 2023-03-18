// SPDX-License-Identifier: CAL
pragma solidity =0.8.19;

import "./IMetaBoardV1.sol";
import "./LibMeta.sol";

contract MetaBoard is IMetaBoardV1 {
    /// @inheritdoc IMetaBoardV1
    function emitMeta(bytes calldata meta_) public {
        LibMeta.checkMetaUnhashed(meta_);
        emit MetaV1(msg.sender, meta_);
    }

    /// @inheritdoc IMetaBoardV1
    function emitMetas(bytes[] calldata metas_) external {
        unchecked {
            for (uint256 i_ = 0; i_ < metas_.length; i_++) {
                emitMeta(metas_[i_]);
            }
        }
    }
}
