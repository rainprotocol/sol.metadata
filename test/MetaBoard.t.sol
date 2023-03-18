// SPDX-License-Identifier: CAL
pragma solidity =0.8.16;

import "forge-std/Test.sol";
import "../src/IMetaV1.sol";
import "../src/LibMeta.sol";
import "../src/MetaBoard.sol";

contract MetaBoardTest is Test, IMetaV1 {
    function testEmitMeta(bytes memory data_) public {
        MetaBoard metaBoard_ = new MetaBoard();

        bytes memory meta_ = abi.encodePacked(META_MAGIC_NUMBER_V1, data_);
        vm.expectEmit(false, false, false, true);
        //slither-disable-next-line reentrancy-events
        emit MetaV1(address(this), meta_);
        metaBoard_.emitMeta(meta_);

        vm.expectRevert(abi.encodeWithSelector(NotRainMetaV1.selector, data_));
        metaBoard_.emitMeta(data_);
    }
}
