use clap::{Parser};

#[derive(Parser,Clone)]
pub struct DISpair {
    pub interpreter : String ,
    pub store : String ,
    pub deployer : String ,
}  

impl DISpair {
    pub fn new(i : String, s : String, d : String) -> DISpair {
        DISpair { interpreter: i, store: s, deployer: d }
    }
}
 
 
pub fn replace_dis_pair(
    tx_data : &String ,
    from_dis : &DISpair , 
    to_dis : &DISpair
) -> Option<String> { 

   let mut ret_str = tx_data.clone().to_lowercase() ;  

   if tx_data.contains(&from_dis.interpreter[2..].to_lowercase()){
       ret_str = ret_str.replace(&from_dis.interpreter[2..].to_lowercase(), &to_dis.interpreter[2..].to_lowercase()) ; 
   } 
   if tx_data.contains(&from_dis.store[2..].to_lowercase()){
       ret_str = ret_str.replace(&from_dis.store[2..].to_lowercase(), &to_dis.store[2..].to_lowercase()) ; 
   } 
   if tx_data.contains(&from_dis.deployer[2..].to_lowercase()){
       ret_str = ret_str.replace(&from_dis.deployer[2..].to_lowercase(), &to_dis.deployer[2..].to_lowercase()) ; 
   }

    Some(ret_str)
}

