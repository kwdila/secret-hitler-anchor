use anyhow::Result as AnyhowResult;
use secret_hitler::ToAccountMetas;
use solana_sdk::instruction::Instruction;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::keypair_from_seed;
use solana_sdk::signer::Signer;
use solana_sdk::transaction::Transaction;
use std::str::FromStr;

pub fn get_start_game_ix(host_key_str: &str) -> AnyhowResult<Transaction> {
    let program_id = Pubkey::try_from("FxKWd5DzXvHPVfrGo3DvXgTeG25TGUGUAFvjQ1zSMp1B").unwrap();

    let client = solana_client::rpc_client::RpcClient::new("http://localhost:8899".to_string());

    let host_key = Pubkey::from_str(host_key_str)?;
    let server_key = keypair_from_seed(&[
        12, 190, 63, 66, 74, 74, 209, 14, 215, 119, 162, 254, 47, 168, 67, 135, 138, 159, 54, 160,
        93, 148, 236, 126, 255, 49, 253, 27, 53, 143, 123, 196, 245, 12, 104, 216, 120, 213, 151,
        231, 244, 215, 25, 33, 163, 144, 100, 10, 144, 156, 169, 89, 184, 100, 80, 94, 36, 86, 241,
        254, 102, 208, 133, 105,
    ])
    .unwrap();

    let (game_data, _) = Pubkey::find_program_address(
        &[b"secret_hitler", host_key.to_bytes().as_ref()][..],
        &program_id,
    );

    let ix = init_game(host_key, game_data, server_key.pubkey());

    let mut tx = Transaction::new_with_payer(&[ix], None);

    let hash = client.get_latest_blockhash().unwrap();
    tx.partial_sign(&[&server_key], hash);

    Ok(tx)
}

pub fn init_game(host: Pubkey, game_data: Pubkey, server: Pubkey) -> Instruction {
    Instruction {
        program_id: secret_hitler::id(),
        accounts: ToAccountMetas::to_account_metas(
            &secret_hitler::accounts::StartGame {
                server,
                host,
                game_data,
            },
            None,
        ),
        data: anchor_lang::InstructionData::data(&secret_hitler::instruction::StartGame {}),
    }
}
