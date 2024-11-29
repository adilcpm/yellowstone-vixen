use std::borrow::Cow;

use borsh::BorshDeserialize;
use yellowstone_vixen_core::{
    instruction::InstructionUpdate, ParseError, ParseResult, Parser, ProgramParser,
};

use super::instruction_helpers::{
    BuyAccounts, BuyIxData, CreateAccounts, CreateIxData, PumpFunProgramIx, BUY_IX_DISC,
    CREATE_IX_DISC,
};
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

        let sell_ix_discriminator: [u8; 8] = [51, 230, 133, 164, 1, 127, 131, 173];
        let unknown_ix_discriminator: [u8; 8] = [228, 69, 165, 46, 81, 203, 154, 29];

        if ix_discriminator == unknown_ix_discriminator || ix_discriminator == sell_ix_discriminator
        {
            return Ok(PumpFunProgramIx::Unknown);
        }

        match ix_discriminator {
            CREATE_IX_DISC => {
                check_min_accounts_req(accounts_len, 11)?;
                let create_ix_data: CreateIxData =
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
                        name: create_ix_data.name,
                        symbol: create_ix_data.symbol,
                        uri: create_ix_data.uri,
                    },
                ))
            },
            BUY_IX_DISC => {
                check_min_accounts_req(accounts_len, 11)?;
                let buy_ix_data: BuyIxData = BorshDeserialize::deserialize(&mut ix_data).unwrap();
                Ok(PumpFunProgramIx::Buy(
                    BuyAccounts {
                        global: ix.accounts[0],
                        fee_recipient: ix.accounts[1],
                        mint: ix.accounts[2],
                        bonding_curve: ix.accounts[3],
                        associated_bonding_curve: ix.accounts[4],
                        associated_user: ix.accounts[5],
                        user: ix.accounts[6],
                        system_program: ix.accounts[7],
                        token_program: ix.accounts[8],
                        rent: ix.accounts[9],
                        event_authority: ix.accounts[10],
                        program: ix.accounts[11],
                    },
                    BuyIxData {
                        amount: buy_ix_data.amount,
                        max_sol_cost: buy_ix_data.max_sol_cost,
                    },
                ))
            },
            _ => Err(ParseError::from("Unknown instruction")),
        }
    }
}

#[cfg(feature = "proto")]
mod proto_parser {
    use yellowstone_vixen_core::proto::ParseProto;
    use yellowstone_vixen_proto::parser::PumpFunProgramIxProto;

    use super::InstructionParser;
    use crate::helpers::IntoProto;

    impl ParseProto for InstructionParser {
        type Message = PumpFunProgramIxProto;

        fn output_into_message(value: Self::Output) -> Self::Message {
            value.into_proto()
        }
    }
}

#[cfg(test)]
mod tests {
    use yellowstone_vixen_mock::tx_fixture;

    use super::*;

    #[tokio::test]
    async fn test_pumpfun_ix_parsing_create() {
        let parser = InstructionParser;

        let ixs = tx_fixture!("4VFPXFczu56zZuZHGPToQnd76CPDcSyaWCvhu7s4UPzxm5U1atjbAUg3n8RXPDXerGCVmNW9R6sh2p27qD7nRsy8",&parser);

        let ix = &ixs[0];

        if let PumpFunProgramIx::Create(accounts, data) = ix {
            assert_eq!(
                accounts.bonding_curve.to_string(),
                "w5LWYUqui5Vwb6Tp3y5NiaWk4e2ies58KRE26XoAo9E".to_string()
            );
            assert_eq!(data.symbol, "girlpump");
        } else {
            panic!("Invalid Instruction");
        }
    }

    #[tokio::test]
    async fn test_pumpfun_ix_parsing_buy() {
        let parser = InstructionParser;

        let ixs = tx_fixture!("3A8c3N4Q6HmCBufjMdQUoRcoBPEuRRRswfmmCVVujC6ayEFBiqF8UAPa585ib3K5GJt6X8rw1f6XPXx1eN3yegYx",&parser);

        let ix = &ixs[1];

        if let PumpFunProgramIx::Buy(accounts, data) = ix {
            println!("{:?}", accounts);
            println!("{:?}", data);
            assert_eq!(
                accounts.mint.to_string(),
                "4svHchJwpb18beJC1DFrymAu4vsyvR5XwhVmiTkZpump".to_string()
            );
            assert_eq!(data.amount, 2497838377120);
        } else {
            panic!("Invalid Instruction");
        }
    }
}
