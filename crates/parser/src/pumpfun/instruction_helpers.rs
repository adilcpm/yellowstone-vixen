use std::fmt::Debug;

use borsh::{BorshDeserialize, BorshSerialize};
use yellowstone_vixen_core::Pubkey;

pub const CREATE_IX_DISC: [u8; 8] = [24, 30, 200, 40, 5, 28, 7, 119];

#[derive(Debug, Clone, Copy)]
pub struct CreateAccounts {
    pub mint: Pubkey,
    pub mint_authority: Pubkey,
    pub bonding_curve: Pubkey,
    pub associated_bonding_curve: Pubkey,
    pub global: Pubkey,
    pub mpl_token_metadata: Pubkey,
    pub metadata: Pubkey,
    pub user: Pubkey,
    pub system_program: Pubkey,
    pub token_program: Pubkey,
    pub associated_token_program: Pubkey,
    pub rent: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}

#[derive(Debug, BorshDeserialize, BorshSerialize, Clone)]
pub struct CreateIxData {
    pub name: String,
    pub symbol: String,
    pub uri: String,
}

#[derive(Debug)]
pub enum PumpFunProgramIx {
    Create(CreateAccounts, CreateIxData),
    Unknown,
}

// #[cfg(feature = "proto")]
// mod proto_parser {
//     use yellowstone_vixen_proto::parser::{
//         orca_program_ix_proto::IxOneof, OrcaProgramIxProto, OrcaSwapAccountsProto,
//         OrcaSwapInstructionProto, OrcaSwapIxDataProto, OrcaSwapV2AccountsProto,
//         OrcaSwapV2InstructionProto, OrcaSwapV2IxDataProto,
//     };

//     use super::{CreateAccounts, CreateIxData, PumpFunProgramIx};
//     use crate::helpers::IntoProto;

//     impl IntoProto<OrcaSwapAccountsProto> for CreateAccounts {
//         fn into_proto(self) -> OrcaSwapAccountsProto {
//             OrcaSwapAccountsProto {
//                 token_program: self.token_program.to_string(),
//                 token_authority: self.token_authority.to_string(),
//                 whirlpool: self.whirlpool.to_string(),
//                 token_owner_account_a: self.token_owner_account_a.to_string(),
//                 token_vault_a: self.token_vault_a.to_string(),
//                 token_owner_account_b: self.token_owner_account_b.to_string(),
//                 token_vault_b: self.token_vault_b.to_string(),
//                 tick_array0: self.tick_array0.to_string(),
//                 tick_array1: self.tick_array1.to_string(),
//                 tick_array2: self.tick_array2.to_string(),
//                 oracle: self.oracle.to_string(),
//             }
//         }
//     }

//     impl IntoProto<OrcaSwapIxDataProto> for CreateIxData {
//         fn into_proto(self) -> OrcaSwapIxDataProto {
//             OrcaSwapIxDataProto {
//                 amount: self.amount,
//                 other_amount_threshold: self.other_amount_threshold,
//                 sqrt_price_limit: self.sqrt_price_limit.to_string(),
//                 amount_specified_is_input: self.amount_specified_is_input,
//                 a_to_b: self.a_to_b,
//             }
//         }
//     }

//     impl IntoProto<OrcaProgramIxProto> for PumpFunProgramIx {
//         fn into_proto(self) -> OrcaProgramIxProto {
//             match self {
//                 PumpFunProgramIx::Create(acc, data) => OrcaProgramIxProto {
//                     ix_oneof: Some(IxOneof::Swap(OrcaSwapInstructionProto {
//                         accounts: Some(acc.into_proto()),
//                         data: Some(data.into_proto()),
//                     })),
//                 },
//             }
//         }
//     }
// }
