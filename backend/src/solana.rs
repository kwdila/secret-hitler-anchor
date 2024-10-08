use anyhow::{anyhow, Ok, Result as AnyhowResult};
use rand::distributions::Uniform;
use rand::{thread_rng, Rng};
use secret_hitler::state::GameData;
use secret_hitler::{PlayerCount, ToAccountMetas};
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::instruction::Instruction;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::keypair_from_seed;
use solana_sdk::signer::Signer;
use solana_sdk::transaction::Transaction;
use std::str::FromStr;

use crate::db::Player;
use crate::role::Role;

pub async fn get_start_game_ix(
    host_key_str: &str,
    client: &RpcClient,
) -> AnyhowResult<Transaction> {
    let program_id = Pubkey::try_from("6VYMcKKWTWucWArbJ8soisLHaNqA4PEHPUS5jDo1SyBn").unwrap();

    let host_key = Pubkey::from_str(host_key_str)?;
    let server_key = keypair_from_seed(&[
        12, 190, 63, 66, 74, 74, 209, 14, 215, 119, 162, 254, 47, 168, 67, 135, 138, 159, 54, 160,
        93, 148, 236, 126, 255, 49, 253, 27, 53, 143, 123, 196, 245, 12, 104, 216, 120, 213, 151,
        231, 244, 215, 25, 33, 163, 144, 100, 10, 144, 156, 169, 89, 184, 100, 80, 94, 36, 86, 241,
        254, 102, 208, 133, 105,
    ])
    .unwrap();

    let (game_key, _) = Pubkey::find_program_address(
        &[b"secret_hitler", host_key.to_bytes().as_ref()][..],
        &program_id,
    );

    let ix = init_game(host_key, game_key, server_key.pubkey());

    let mut tx = Transaction::new_with_payer(&[ix], None);

    let hash = client.get_latest_blockhash().await?;
    tx.partial_sign(&[&server_key], hash);

    Ok(tx)
}

fn init_game(host: Pubkey, game_key: Pubkey, server: Pubkey) -> Instruction {
    Instruction {
        program_id: secret_hitler::id(),
        accounts: ToAccountMetas::to_account_metas(
            &secret_hitler::accounts::StartGame {
                server,
                host,
                game_data: game_key,
            },
            None,
        ),
        data: anchor_lang::InstructionData::data(&secret_hitler::instruction::StartGame {}),
    }
}

pub async fn set_player_roles(game_key: &str, client: &RpcClient) -> AnyhowResult<Vec<Player>> {
    let game_key = Pubkey::from_str(game_key).expect("failed to get pubkey from game_key str");

    let game_account = client.get_account(&game_key).await.unwrap();
    let game: GameData =
        anchor_lang::AccountDeserialize::try_deserialize(&mut game_account.data.as_slice())
            .expect("failed to deserialise game data account");

    let players = &game.active_players;
    let player_count = &game.start_player_count.unwrap();
    let max_liberals: u8 = PlayerCount::liberal_count(player_count);
    let max_fascists: u8 = PlayerCount::fascist_count(player_count);

    let mut is_hitler = false;
    let mut liberal_num: u8 = 0;
    let mut fascist_num: u8 = 0;
    let mut rng = thread_rng();
    let mut side = Uniform::new(0, 3);

    let mut players_to_db: Vec<Player> = vec![];

    for player_key in players {
        let mut rnd_num = rng.sample(side);
        if !(rnd_num == 0 && is_hitler) {
            side = Uniform::new(1, 3);
            rnd_num = rng.sample(side);
        };

        let mut role = Role::from(rnd_num).expect("Failed to convert number to role");

        match (
            &mut role,
            max_fascists > fascist_num,
            max_liberals > liberal_num,
        ) {
            (Role::Hitler, _, _) => {
                is_hitler = true;
            }
            (Role::Fascist, false, true) => {
                role = Role::Liberal;
                liberal_num += 1;
            }
            (Role::Liberal, true, false) => {
                role = Role::Fascist;
                fascist_num += 1;
            }
            (Role::Fascist, true, _) => fascist_num += 1,
            (Role::Liberal, _, true) => liberal_num += 1,
            _ => {
                return Err(anyhow!(
                    "Unexpectred number of players depending on number of roles"
                ))
            }
        }
        // add a new player to game in DB with the rnd_num and Pubkey;
        players_to_db.push(Player {
            id: None,
            pubkey: player_key.to_string(),
            role: role.to_num(),
            game_key: game_key.to_string(),
            game_id: None,
        });
    }
    Ok(players_to_db)
}
