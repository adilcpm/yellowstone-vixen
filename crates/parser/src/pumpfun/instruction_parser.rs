use std::borrow::Cow;

use borsh::BorshDeserialize;
use yellowstone_vixen_core::{
    instruction::InstructionUpdate, ParseError, ParseResult, Parser, ProgramParser,
};

use super::instruction_helpers::{CreateAccounts, CreateIxData, PumpFunProgramIx, CREATE_IX_DISC};
use crate::helpers::{check_min_accounts_req, IX_DISCRIMINATOR_SIZE};

use solana_program::{pubkey, pubkey::Pubkey};

/// Public key for the Pump.fun program
pub const PUMPFUN: Pubkey = pubkey!("6EF8rrecthR5Dkzon8Nwu78hRvfCKubJ14M5uBEwF6P");

#[derive(Debug, Clone, Copy)]
pub struct InstructionParser;

impl Parser for InstructionParser {
    type Input = InstructionUpdate;
    type Output = PumpFunProgramIx;

    fn id(&self) -> Cow<str> {
        "yellowstone_vixen_parser::pumpfun::InstructionParser".into()
    }

    fn prefilter(&self) -> yellowstone_vixen_core::Prefilter {
        yellowstone_vixen_core::Prefilter::builder()
            .transaction_accounts([PUMPFUN])
            .build()
            .unwrap()
    }

    async fn parse(&self, ix_update: &InstructionUpdate) -> ParseResult<Self::Output> {
        if ix_update.program.equals_ref(PUMPFUN) {
            InstructionParser::parse_impl(ix_update)
        } else {
            Err(ParseError::Filtered)
        }
    }
}

impl ProgramParser for InstructionParser {
    #[inline]
    fn program_id(&self) -> yellowstone_vixen_core::Pubkey {
        PUMPFUN.to_bytes().into()
    }
}

impl InstructionParser {
    pub(crate) fn parse_impl(ix: &InstructionUpdate) -> Result<PumpFunProgramIx, ParseError> {
        let accounts_len = ix.accounts.len();
        let ix_discriminator: [u8; 8] = ix.data[0..IX_DISCRIMINATOR_SIZE].try_into()?;
        let mut ix_data = &ix.data[IX_DISCRIMINATOR_SIZE..];

        let buy_ix_discriminator: [u8; 8] = [102, 6, 61, 18, 1, 218, 235, 234];
        let unknown_ix_discriminator: [u8; 8] = [228, 69, 165, 46, 81, 203, 154, 29];

        if ix_discriminator == buy_ix_discriminator || ix_discriminator == unknown_ix_discriminator
        {
            return Ok(PumpFunProgramIx::Unknown);
        }

        match ix_discriminator {
            CREATE_IX_DISC => {
                check_min_accounts_req(accounts_len, 11)?;
                let swap_ix_data: CreateIxData =
                    BorshDeserialize::deserialize(&mut ix_data).unwrap();
                Ok(PumpFunProgramIx::Create(
                    CreateAccounts {
                        mint: ix.accounts[0],
                        mint_authority: ix.accounts[1],
                        bonding_curve: ix.accounts[2],
                        associated_bonding_curve: ix.accounts[3],
                        global: ix.accounts[4],
                        mpl_token_metadata: ix.accounts[5],
                        metadata: ix.accounts[6],
                        user: ix.accounts[7],
                        system_program: ix.accounts[8],
                        token_program: ix.accounts[9],
                        associated_token_program: ix.accounts[10],
                        rent: ix.accounts[11],
                        event_authority: ix.accounts[12],
                        program: ix.accounts[13],
                    },
                    CreateIxData {
                        name: swap_ix_data.name,
                        symbol: swap_ix_data.symbol,
                        uri: swap_ix_data.uri,
                    },
                ))
            },
            // SWAP_V2_IX_DISC => {
            //     check_min_accounts_req(accounts_len, 15)?;
            //     let swap_ix_v2_data: SwapV2IxData =
            //         BorshDeserialize::deserialize(&mut ix_data).unwrap();
            //     Ok(PumpFunProgramIx::SwapV2(
            //         SwapV2Accounts {
            //             token_program_a: ix.accounts[0],
            //             token_program_b: ix.accounts[1],
            //             memo_program: ix.accounts[2],
            //             token_authority: ix.accounts[3],
            //             whirlpool: ix.accounts[4],
            //             token_mint_a: ix.accounts[5],
            //             token_mint_b: ix.accounts[6],
            //             token_owner_account_a: ix.accounts[7],
            //             token_vault_a: ix.accounts[8],
            //             token_owner_account_b: ix.accounts[9],
            //             token_vault_b: ix.accounts[10],
            //             tick_array0: ix.accounts[11],
            //             tick_array1: ix.accounts[12],
            //             tick_array2: ix.accounts[13],
            //             oracle: ix.accounts[14],
            //         },
            //         SwapV2IxData {
            //             a_to_b: swap_ix_v2_data.a_to_b,
            //             amount: swap_ix_v2_data.amount,
            //             other_amount_threshold: swap_ix_v2_data.other_amount_threshold,
            //             sqrt_price_limit: swap_ix_v2_data.sqrt_price_limit,
            //             amount_specified_is_input: swap_ix_v2_data.amount_specified_is_input,
            //         },
            //     ))
            // },
            _ => Err(ParseError::from("Unknown instruction")),
        }
    }
}

// #[cfg(feature = "proto")]
// mod proto_parser {
//     use yellowstone_vixen_core::proto::ParseProto;
//     use yellowstone_vixen_proto::parser::OrcaProgramIxProto;

//     use super::InstructionParser;
//     use crate::helpers::IntoProto;

//     impl ParseProto for InstructionParser {
//         type Message = OrcaProgramIxProto;

//         fn output_into_message(value: Self::Output) -> Self::Message {
//             value.into_proto()
//         }
//     }
// }

#[cfg(test)]
mod tests {
    use yellowstone_vixen_mock::tx_fixture;

    use super::*;

    #[tokio::test]
    async fn test_swap_ix_parsing_pump() {
        let parser = InstructionParser;

        let ixs = tx_fixture!("3A8c3N4Q6HmCBufjMdQUoRcoBPEuRRRswfmmCVVujC6ayEFBiqF8UAPa585ib3K5GJt6X8rw1f6XPXx1eN3yegYx",&parser);

        let ix = &ixs[0];

        if let PumpFunProgramIx::Create(accounts, data) = ix {
            assert_eq!(
                accounts.bonding_curve.to_string(),
                "GTeshboq42dhmQwDSjNENqrBkhPRbA1FCBqJuYKFY1f9".to_string()
            );
            assert_eq!(data.name, "Patek");
        } else {
            panic!("Invalid Instruction");
        }
    }
}
