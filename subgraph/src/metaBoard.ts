import { BigInt } from "@graphprotocol/graph-ts";
import { MetaV1 as MetaV1Event} from "../generated/MetaBoard/MetaBoard";
import { MetaBoard, MetaV1 } from "../generated/schema"
export function handleMetaV1(event: MetaV1Event): void {
    let metaBoard = MetaBoard.load(event.address);
    if(!metaBoard){
        metaBoard = new MetaBoard(event.address);
        metaBoard.address = event.address;
        metaBoard.metaCount = BigInt.fromI32(0);
        metaBoard.save();
    }

    let metaV1 = new MetaV1(event.transaction.hash.toHex());
    metaV1.sender = event.params.sender;
    metaV1.meta = event.params.meta;
    metaV1.metaBoard = event.address;
    metaV1.save();

    metaBoard.metaCount = metaBoard.metaCount.plus(BigInt.fromI32(1));
    metaBoard.save();
}