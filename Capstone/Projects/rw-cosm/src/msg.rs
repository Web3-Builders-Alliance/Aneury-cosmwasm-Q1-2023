use cosmwasm_schema::{cw_serde, QueryResponses};

#[cw_serde]
pub struct InstantiateMsg {
    pub admin_address: String
    /// todo: decide which kind of contract to add for example if add proxy contract in order to have multiples admin.
}

#[cw_serde]
pub enum ExecuteMsg {
   CreateCampaign{
       creator_addr: Addr,
       title : String,
       description: String, 
   },
   GiveReward{
      customer_address: Addr,
      reward_point: i32,
      creator_addr: Addr,
      references: String,
   },
   TradeReward{
      customer_address: Addr,
   }
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    // GetCount returns the current count as a json-encoded number
    //#[returns(GetCountResponse)]
    //GetCount {},

   #[returns(GetReportOfRewardOffered)]
   GetReportOfRewards{},


}

// We define a custom struct for each query response
//#[cw_serde]
//pub struct GetCountResponse {
//    pub count: i32,
//}


#[cw_serde]
pub struct GetReportOfRewardOffered{
  // todo: Add metadata to share in reports?
}