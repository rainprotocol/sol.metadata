# rain.metadata

## Specs

MetadataV1 spec - https://github.com/rainprotocol/specs/blob/main/metadata-v1.md 

### Cross Deploy 
- CLI utility to cross deploy Rain and "Non-Rain" Contract . A single utility for all contracts . 
Utility command and options : 
Run the following command in shell 
```sh
rain-meta cross-deploy deploy-consumer --help
``` 
- You'll get the following output :
```sh 
Usage: rain-meta cross-deploy deploy-consumer [OPTIONS] --from-network <ORIGIN_NETWORK> --contract-address <CONTRACT_ADDRESS>

Options:
  -o, --from-network <ORIGIN_NETWORK>
          origin network to deploy contract from [possible values: ethereum, polygon, mumbai, fuji]
  -t, --to-network <TO_NETWORK>
          optional target network to dpeloy to [possible values: ethereum, polygon, mumbai, fuji]
  -i, --from-interpreter <FROM_INTERPRETER>
          origin network interpreter
  -s, --from-store <FROM_STORE>
          origin network store
  -d, --from-deployer <FROM_DEPLOYER>
          origin network deployer
  -I, --to-interpreter <TO_INTERPRETER>
          target network interpreter
  -S, --to-store <TO_STORE>
          target network store
  -D, --to-deployer <TO_DEPLOYER>
          target network deployer
  -c, --contract-address <CONTRACT_ADDRESS>
          origin network contract address
  -H, --transaction-hash <TRANSACTION_HASH>
          origin network contract address
      --deploy
          Set to true to deploy contract to target network
  -k, --priavte-key <PRIVATE_KEY>
          private key (unprefixed)
  -h, --help 
  ```
#### Deploying Non-Rain Contracts
- To deploy Non-Rain contracts, note that currently only non-rain contracts with no constructor arguments are supported . 
Eg : Extrospection : https://mumbai.polygonscan.com/address/0x2c9f3204590765aefa7bee01bccb540a7d06e967
To cross deploy Non-Rain Contract run :

```
rain-meta cross-deploy deploy-consumer -o mumbai -t fuji \
-c 0x2c9f3204590765aefa7bee01bccb540a7d06e967 \
-H 0xea76ed73832498c4293aa06aeca2899f2b5adca15d703b03690185ed829f3e71 
```
- When you do not add the optional **--deploy** flag the cli gives you the transaction data needed to deploy the contract. You can pass this transaction data to any signer and submit the transaction.
- Eg: Passing the output of the above command to a frontend integrated with wallet. 
The output of the cli is the transaction data : 
```
0x608060405234801561001057600080fd5b50610562806100206000396000f3fe608060405234801561001057600080fd5b50600436106100725760003560e01c8063b8270b1e11610050578063b8270b1e14610104578063c7338a0214610127578063f5a...91505056
```

- ==Please note that even though the transaction hash option **-H, --transaction-hash** is optional, for Non-Rain Contracts and DISpair contracts it must be specified, until further updates== .
- To deploy the contract you can add the **--deploy** flag along with the **unprefixed** private key **-k, --priavte-key** : 
```
rain-meta cross-deploy deploy-consumer -o mumbai -t fuji \
-c 0x2c9f3204590765aefa7bee01bccb540a7d06e967 \
-H 0xea76ed73832498c4293aa06aeca2899f2b5adca15d703b03690185ed829f3e71 \
--deploy \
-k 123...abcd
```
Output : Deployed transaction hash and contract address on target network. 
```
Contract Deployed !!
#################################
✅ Hash : "0x1e6f2c7601adc4b098114145b786d591db965b06f585be90be87a2c98675b618"
Contract Address: "0x6d291256f4ddd8b3a8ca8d540ae8b31df2a7474b"
-----------------------------------
```
#### Deploying DIS contracts
- To deploy intreperter, store you can simply pass the contract address and transaction hash :
 - Eg : **-c, --contract-address** is the interpreter/store of the originating network .
```sh
rain meta cross-deploy deploy-consumer -o mumbai -t fuji \
-c 0x5f02c2f831d3e0d430aa58c973b8b751f3d81b38 \
-H 0xd8ff2d9381573294ce7d260d3f95e8d00a42d55a5ac29ff9ae22a401b53c2e19 \
--deploy \
-k  1234...abcd
```
 - To deploy expression deployer pass the contract address along with interpreter and store of origin and target network . 
 - Eg : where 
-i, -s : Intreperter and Store of origin network.
-I, -S : Intreperter and Store of target network.
-c : Deployer of origin network.
-H : Deployer contract deployment transaction.
```
rain meta cross-deploy deploy-consumer -o mumbai -t fuji \
> -i 0x5f02c2f831d3e0d430aa58c973b8b751f3d81b38 \
> -s 0xa5d9c16ddfd05d398fd0f302edd9e9e16d328796 \
> -I 0xfd1da7eee4a9391f6fcabb28617f41894ba84cdc \
> -S 0x9b8571bd2742ec628211111de3aa940f5984e82b \
> -c 0xd3870063bcf25d5110ab9df9672a0d5c79c8b2d5 \
> --deploy \
> -k 421c..9a1c8 \
> -H 0xebacdb3971924c9bbd2257334d436b4590d3d98f54969f6f942d6bd7a68da80b
```

#### Deploying Rain contracts
- To deploy Rain Contracts you have to mention origin and target network DIS contract address along with Rain Consumer contract address on origin network(Eg : Flow, Orderbook) .

Eg : 
```
rain meta cross-deploy deploy-consumer -o mumbai -t fuji \
-i 0x5f02c2f831d3e0d430aa58c973b8b751f3d81b38 \
-s 0xa5d9c16ddfd05d398fd0f302edd9e9e16d328796 \
-d 0xd3870063bcf25d5110ab9df9672a0d5c79c8b2d5 \
-I 0xfd1da7eee4a9391f6fcabb28617f41894ba84cdc \
-S 0x9b8571bd2742ec628211111de3aa940f5984e82b \
-D 0x3d7d894afc7dbfd45bf50867c9b051da8eee85e9 \
-c 0x3cc6c6e888b4ad891eea635041a269c4ba1c4a63 \
--deploy \
-k 12cbee..a1c8
```
- where : 
-d, -D : origin, target network deployer addresses. 
-c : Contract Address of Rain Consumer Contract like Orderbook, Flow on origin network.
-H : Transaction hash which is optional.





