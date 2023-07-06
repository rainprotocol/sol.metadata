use clap::{Args, Subcommand};
use crate::deploy::{registry::RainNetworks, deploy_contract, dis::DISpair};  

#[derive(Subcommand)]
pub enum CrossDeploy{
    /// Cross Deploy a Rain consumer contract 
    DeployConsumer(Consumer)
}

#[derive(Args, Debug)]
pub struct Consumer{

    /// origin Network to deploy contract from
    #[arg(short, long = "from-network")]
    origin_network: RainNetworks, 

    /// origin network interpreter
    #[arg(short ='i' , long = "from-interpreter")]
    from_interpreter: String,

    /// origin network store
    #[arg(short ='s' , long = "from-store")]
    from_store: String,

    /// origin network deployer
    #[arg(short ='d' , long = "from-deployer")]
    from_deployer: String, 

    /// origin network interpreter
    #[arg(short ='I' , long = "to-interpreter")]
    to_interpreter: String,

    /// origin network store
    #[arg(short ='S' , long = "to-store")]
    to_store: String,

    /// origin network deployer
    #[arg(short ='D' , long = "to-deployer")]
    to_deployer: String,

    /// origin network contract address
    #[arg(short ='c' , long = "contract-address")]
    contract_address: String
} 

pub async fn deploy(cross_deploy: CrossDeploy) -> anyhow::Result<()> {
     
    let data = match cross_deploy {
        CrossDeploy::DeployConsumer(consumer) => { 
            deploy_contract(
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
                ).await?              
        }
    } ;

    println!("{}",data) ;
    Ok(())

}
