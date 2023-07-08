
use anyhow::{Result};
use crate::{subgraph::get_transaction_hash, cli::deploy::Consumer, deploy::registry::Fuji};
use self::{registry::{RainNetworks, Ethereum, Mumbai, Polygon}, transaction::get_transaction_data, dis::{DISpair, replace_dis_pair}}; 
use ethers::{providers::{Provider, Middleware, Http}} ; 
use ethers::{signers::LocalWallet, types::{Eip1559TransactionRequest, U64}, prelude::SignerMiddleware};
use std::str::FromStr;
pub mod registry; 
use anyhow::anyhow; 

pub mod transaction; 
pub mod dis; 

pub async fn deploy_data(
    from_network : RainNetworks ,
    contract_address : String ,
    from_dis : DISpair , 
    to_dis : DISpair
) -> Result<String> {    
    // Get tx hash
    let tx_data = get_transaction_hash(&from_network, &contract_address).await? ;  
    // Get tx data
    let tx_data = get_transaction_data(&from_network, &tx_data).await? ;  
    // Replace DIS instances 
    let tx_data = replace_dis_pair(&tx_data,&from_dis,&to_dis).unwrap() ;  

    Ok(tx_data)
}  


pub async fn deploy_contract(consumer : Consumer)  -> Result<()> {  

    if consumer.deploy {
        let key = match consumer.private_key {
            Some(key) => key,
            None => return Err(anyhow!("Private Key Not Provided")),
        };   
        
        let data = deploy_data(
            consumer.origin_network ,
            consumer.contract_address, 
            DISpair::new(
                consumer.from_interpreter,
                consumer.from_store,
                consumer.from_deployer
            ) ,
            DISpair::new(
                consumer.to_interpreter,
                consumer.to_store,
                consumer.to_deployer
            ) 
        ).await? ; 

        let (url,chain_id,scan_base_uri) = match consumer.to_network.unwrap() {
            RainNetworks::Ethereum => {
                (Ethereum::default().provider,Ethereum::default().chain_id,Ethereum::default().scan_base_uri)
            } ,
            RainNetworks::Polygon => {
                (Polygon::default().provider,Polygon::default().chain_id,Polygon::default().scan_base_uri)
            },
            RainNetworks::Mumbai => {
                (Mumbai::default().provider,Mumbai::default().chain_id,Mumbai::default().scan_base_uri)
            },
            RainNetworks::Fuji => {
                (Fuji::default().provider,Fuji::default().chain_id,Fuji::default().scan_base_uri)
            }
        } ; 
            
        let provider = Provider::<Http>::try_from(url)
        .expect("could not instantiate HTTP Provider"); 

        let wallet: LocalWallet = key.parse()?; 
        let client = SignerMiddleware::new_with_provider_chain(provider, wallet).await?;  

        let bytes_data = ethers::core::types::Bytes::from_str(&data).unwrap() ; 
        let chain_id = U64::from_dec_str(&chain_id).unwrap() ; 
        let tx = Eip1559TransactionRequest::new().data(bytes_data).chain_id(chain_id) ; 

        let tx = client.send_transaction(tx, None).await?;   

        let receipt = tx.confirmations(6).await?.unwrap(); 

        println!(
            "\nContract Deployed !!\n#################################\n✅ Hash : {}\nContract : {}/{}\n-----------------------------------\n" ,
            serde_json::to_string(&receipt.transaction_hash).unwrap().to_string(),
            &scan_base_uri,
            serde_json::to_string(&receipt.contract_address.unwrap()).unwrap().to_string(),
        ) ;

        Ok(())

    }else{ 
        
        let tx_data = deploy_data(
                        consumer.origin_network ,
                        consumer.contract_address, 
                        DISpair::new(
                            consumer.from_interpreter,
                            consumer.from_store,
                            consumer.from_deployer
                        ) ,
                        DISpair::new(
                            consumer.to_interpreter,
                            consumer.to_store,
                            consumer.to_deployer
                        ) 
        ).await? ;

        println!("{}",tx_data) ;
        Ok(())

    }
     
}
