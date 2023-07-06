use clap::ValueEnum;

#[derive(Debug)]
pub struct Ethereum {
    pub url : String ,
    pub provider : String
}  

impl Default for Ethereum {
    fn default() -> Ethereum {
        Ethereum { 
            url: String::from("https://api.thegraph.com/subgraphs/name/rainprotocol/interpreter-registry-ethereum") ,
            provider : String::from("https://eth-mainnet.g.alchemy.com/v2/gqp-i6HKrlY8gShHDXkJw-iqudcviIyx")
        }
    }
}  

#[derive(Debug)]
pub struct Polygon {
    pub url : String ,
    pub provider : String
    
}  

impl Default for Polygon {
    fn default() -> Polygon {
        Polygon { 
            url: String::from("https://api.thegraph.com/subgraphs/name/rainprotocol/interpreter-registry-polygon") ,
            provider : String::from("https://polygon-mainnet.g.alchemy.com/v2/WLWVvo6m4MXAZ3GkzmMI8ZnLIg_bBNaO")

        }
    }
}  

#[derive(Debug)]
pub struct Mumbai {
    pub url : String ,
    pub provider : String

}  

impl Default for Mumbai {
    fn default() -> Mumbai {
        Mumbai { 
            url: String::from("https://api.thegraph.com/subgraphs/name/rainprotocol/interpreter-registry") ,
            provider : String::from("https://polygon-mumbai.g.alchemy.com/v2/yAwbpk-0UDo-G398kyr6iKrUVWL5fyVj")
        }
    }
}  

 #[derive(Debug)]
 #[derive(Clone,ValueEnum)]
pub enum RainNetworks{
    Ethereum,
    Polygon,
    Mumbai,
}  
