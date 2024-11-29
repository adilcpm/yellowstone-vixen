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

pub const BUY_IX_DISC: [u8; 8] = [102, 6, 61, 18, 1, 218, 235, 234];

#[derive(Debug, Clone, Copy)]
pub struct BuyAccounts {
    pub global: Pubkey,
    pub fee_recipient: Pubkey,
    pub mint: Pubkey,
    pub bonding_curve: Pubkey,
    pub associated_bonding_curve: Pubkey,
    pub associated_user: Pubkey,
    pub user: Pubkey,
    pub system_program: Pubkey,
    pub token_program: Pubkey,
    pub rent: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}

#[derive(Debug, BorshDeserialize, BorshSerialize, Clone, Copy)]
pub struct BuyIxData {
    pub amount: u64,
    pub max_sol_cost: u64,
}

#[derive(Debug)]
pub enum PumpFunProgramIx {
    Create(CreateAccounts, CreateIxData),
    Buy(BuyAccounts, BuyIxData),
    Unknown,
}

#[cfg(feature = "proto")]
mod proto_parser {
    use yellowstone_vixen_proto::parser::{
        pump_fun_program_ix_proto::IxOneof, PumpFunBuyAccountsProto, PumpFunBuyInstructionProto,
        PumpFunBuyIxDataProto, PumpFunCreateAccountsProto, PumpFunCreateInstructionProto,
        PumpFunCreateIxDataProto, PumpFunProgramIxProto,
    };

    use super::{BuyAccounts, BuyIxData, CreateAccounts, CreateIxData, PumpFunProgramIx};
    use crate::helpers::IntoProto;

    impl IntoProto<PumpFunCreateAccountsProto> for CreateAccounts {
        fn into_proto(self) -> PumpFunCreateAccountsProto {
            PumpFunCreateAccountsProto {
                mint: self.mint.to_string(),
                mint_authority: self.mint_authority.to_string(),
                bonding_curve: self.bonding_curve.to_string(),
                associated_bonding_curve: self.associated_bonding_curve.to_string(),
                global: self.global.to_string(),
                mpl_token_metadata: self.mpl_token_metadata.to_string(),
                metadata: self.metadata.to_string(),
                user: self.user.to_string(),
                system_program: self.system_program.to_string(),
                token_program: self.token_program.to_string(),
                associated_token_program: self.associated_token_program.to_string(),
                rent: self.rent.to_string(),
                event_authority: self.event_authority.to_string(),
                program: self.program.to_string(),
            }
        }
    }

    impl IntoProto<PumpFunCreateIxDataProto> for CreateIxData {
        fn into_proto(self) -> PumpFunCreateIxDataProto {
            PumpFunCreateIxDataProto {
                name: self.name,
                symbol: self.symbol,
                uri: self.uri,
            }
        }
    }

    impl IntoProto<PumpFunBuyAccountsProto> for BuyAccounts {
        fn into_proto(self) -> PumpFunBuyAccountsProto {
            PumpFunBuyAccountsProto {
                global: self.global.to_string(),
                fee_recipient: self.fee_recipient.to_string(),
                mint: self.mint.to_string(),
                bonding_curve: self.bonding_curve.to_string(),
                associated_bonding_curve: self.associated_bonding_curve.to_string(),
                associated_user: self.associated_user.to_string(),
                user: self.user.to_string(),
                system_program: self.system_program.to_string(),
                token_program: self.token_program.to_string(),
                rent: self.rent.to_string(),
                event_authority: self.event_authority.to_string(),
                program: self.program.to_string(),
            }
        }
    }

    impl IntoProto<PumpFunBuyIxDataProto> for BuyIxData {
        fn into_proto(self) -> PumpFunBuyIxDataProto {
            PumpFunBuyIxDataProto {
                amount: self.amount,
                max_sol_cost: self.max_sol_cost,
            }
        }
    }

    impl IntoProto<PumpFunProgramIxProto> for PumpFunProgramIx {
        fn into_proto(self) -> PumpFunProgramIxProto {
            match self {
                PumpFunProgramIx::Create(acc, data) => PumpFunProgramIxProto {
                    ix_oneof: Some(IxOneof::Create(PumpFunCreateInstructionProto {
                        accounts: Some(acc.into_proto()),
                        data: Some(data.into_proto()),
                    })),
                },
                PumpFunProgramIx::Buy(acc, data) => PumpFunProgramIxProto {
                    ix_oneof: Some(IxOneof::Buy(PumpFunBuyInstructionProto {
                        accounts: Some(acc.into_proto()),
                        data: Some(data.into_proto()),
                    })),
                },
                PumpFunProgramIx::Unknown => PumpFunProgramIxProto { ix_oneof: None },
            }
        }
    }
}
