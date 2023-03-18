// SPDX-License-Identifier: CAL
pragma solidity =0.8.16;

import "./IMetaV1.sol";
import "./LibMeta.sol";

contract MetaBoard is IMetaV1 {
    function emit(MetaV1[] calldata metas_) {
        unchecked {
            for (i_ = 0; i_ < metas_.length; i_++) {
                checkMetaUnhashed(meta_);
                emit MetaV1(msg.sender, meta_);
            }
        }
    }
}