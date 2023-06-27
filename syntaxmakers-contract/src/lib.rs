mod msg;
mod state {
    pub mod admins;
}
mod error;
mod contract;

use cosmwasm_std::{
    entry_point, DepsMut, Env, MessageInfo, Response, StdResult, Deps, Binary
};
use error::ContractError;
use msg::{InstantiateMsg, QueryMsg, ExecuteMsg};

#[entry_point]
pub fn instantiate(deps: DepsMut, env: Env, info: MessageInfo, msg: InstantiateMsg)
  -> StdResult<Response>
{
    contract::instantiate(deps, env, info, msg)
}

#[entry_point]
pub fn query(deps: Deps, env: Env, msg: QueryMsg)
  -> StdResult<Binary>
{
    contract::query(deps, env, msg)
}

#[entry_point]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    contract::execute(deps, env, info, msg)
}