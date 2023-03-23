import { metaBoard, subgraph } from "./1_factory";
import {
  encodeMeta,
  getEventArgs,
  waitForSubgraphToBeSynced,
} from "./utils";
import { MetaV1Event } from "../typechain/MetaBoard";
import { FetchResult } from "apollo-fetch";
import assert from "assert";
import { ethers } from "hardhat";

describe("MetaBoard MetaV1 event tests", () => {
    let metaCount = 0;
  it("Should emit emitMeta event", async () => {
    let trx = await metaBoard.emitMeta(encodeMeta("Hello"));
    metaCount++;
    const { sender, meta } = (await getEventArgs(
      trx,
      "MetaV1",
      metaBoard
    )) as MetaV1Event["args"];

    await waitForSubgraphToBeSynced();

    const query = `{
      metaBoard(id: "${metaBoard.address.toLowerCase()}"){
        id
        metaCount
        metas{
          id
        }
      }
    }`;

    const response = (await subgraph({ query })) as FetchResult;
    const metaBoardData = response.data.metaBoard;

    assert.equal(metaBoard.address.toLowerCase(), metaBoardData.id);
    assert.equal(metaBoardData.metas.length, 1, "Wrong meta array length");
    assert.equal(metaBoardData.metaCount, 1, "Wrong meta array length");

    const metaQuery = `{
        metaV1(id: "${trx.hash.toLowerCase()}"){
            id
            meta
            sender
            metaBoard {
              id
            }
        }
      }`;

    const metaResponse = (await subgraph({ query: metaQuery })) as FetchResult;
    const metaData = metaResponse.data.metaV1;

    assert.equal(metaData.id, trx.hash, "wrong meta id");
    assert.equal(metaData.meta, meta);
    assert.equal(metaData.sender, sender.toLowerCase());
    assert.equal(metaData.metaBoard.id, metaBoard.address.toLowerCase());
  });

  it("Should emit emitMeta event with diff signers", async () => {
    const signers = await ethers.getSigners();

    const eventEmitter = signers[2];

    const encodedMeta = encodeMeta(eventEmitter.address);

    let trx = await metaBoard.connect(eventEmitter).emitMeta(encodedMeta);
    metaCount++;

    await waitForSubgraphToBeSynced();

    const query = `{
      metaBoard(id: "${metaBoard.address.toLowerCase()}"){
        id
        metaCount
        metas{
          id
        }
      }
    }`;

    const response = (await subgraph({ query })) as FetchResult;
    const metaBoardData = response.data.metaBoard;

    assert.equal(metaBoard.address.toLowerCase(), metaBoardData.id);
    assert.equal(metaBoardData.metas.length, 2, "Wrong meta array length");
    assert.equal(metaBoardData.metaCount, 2, "Wrong meta array length");

    const metaQuery = `{
        metaV1(id: "${trx.hash.toLowerCase()}"){
            id
            meta
            sender
            metaBoard {
              id
            }
        }
      }`;

    const metaResponse = (await subgraph({ query: metaQuery })) as FetchResult;
    const metaData = metaResponse.data.metaV1;

    assert.equal(metaData.id, trx.hash, "wrong meta id");
    assert.equal(metaData.meta, encodedMeta);
    assert.equal(metaData.sender, eventEmitter.address.toLowerCase());
    assert.equal(metaData.metaBoard.id, metaBoard.address.toLowerCase());
  });

  it("Should add 200 new notices",async () => {
    for(let i=0;i<50;i++){
        await metaBoard.emitMeta(encodeMeta(i.toString()));
        metaCount++;
    }

    await waitForSubgraphToBeSynced();

    const query = `{
        metaBoard(id: "${metaBoard.address.toLowerCase()}"){
          metaCount
        }
      }`;

      const response = (await subgraph({query})) as FetchResult;
      const _metaCount = response.data.metaBoard.metaCount;

      assert.equal(_metaCount, metaCount, `Incorrect metaCount ${metaCount} - ${metaCount}`);
  });
});
