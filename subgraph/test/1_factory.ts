import assert from "assert";
import { ethers } from "hardhat";
import { MetaBoard } from "../typechain/MetaBoard";
import { exec, fetchFile, fetchSubgraph, waitForSubgraphToBeSynced, writeFile } from "./utils";
import * as path from "path";
import { ApolloFetch, FetchResult } from "apollo-fetch";

export let metaBoard: MetaBoard;
export let subgraph: ApolloFetch
describe("MetaBoard test", function() {
  before(async () => {
    let MetaBoard = await ethers.getContractFactory("MetaBoard");
    metaBoard = (await MetaBoard.deploy()) as MetaBoard;
    await metaBoard.deployed();
  });

  it("Should deploy metaBoard contract", async () => {
    let configPath = path.resolve(__dirname, "../config/localhost.json");
    const config = JSON.parse(fetchFile(configPath));
    
    config.network = "localhost";
    config.MetaBoard = metaBoard.address;
    config.MetaBoardBlock = metaBoard.deployTransaction.blockNumber;
    
    writeFile(configPath, JSON.stringify(config, null, 2));
    
    // create subgraph instance
    exec("graph create --node http://localhost:8020/ test/test");
    
    // prepare subgraph manifest
    exec("npx mustache config/localhost.json subgraph.template.yaml subgraph.yaml");
    
    // deploy subgraph
    exec("graph deploy --node http://localhost:8020/ --ipfs http://localhost:5001 test/test  --version-label 1");

    subgraph = fetchSubgraph("test/test")

    await waitForSubgraphToBeSynced();

    // const query = `{
    //   metaBoard(id: "${metaBoard.address.toLowerCase()}"){
    //     id
    //     metas{
    //       id
    //     }
    //   }
    // }`;

    // const response = (await subgraph({query})) as FetchResult;
    // const metaBoardData = response.data.metaBoard;

    // assert.equal(metaBoardData, null, "Unrecognized entity");
  });
});
