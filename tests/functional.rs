use {
    borsh::BorshSerialize,
    nebula_vault::{
        instruction::{initialize, deposit, withdraw, set_timelock},
        processor::process_instruction,
        state::VaultState,
    },
    solana_program::{
        instruction::{AccountMeta, Instruction},
        pubkey::Pubkey,
    },
    solana_program_test::*,
    solana_sdk::{
        account::Account,
        signature::{Keypair, Signer},
        transaction::Transaction,
    },
    spl_token::{
        instruction as token_instruction,
        state::{Account as TokenAccount, Mint},
    },
};

pub fn program_test() -> ProgramTest {
    ProgramTest::new(
        "nebula_vault",
        nebula_vault::id(),
        processor!(nebula_vault::processor::process_instruction),
    )
}

async fn create_mint(
    banks_client: &mut BanksClient,
    payer: &Keypair,
    recent_blockhash: &Hash,
) -> Pubkey {
    let mint = Keypair::new();
    let rent = banks_client.get_rent().await.unwrap();
    let mint_rent = rent.minimum_balance(Mint::LEN);

    let transaction = Transaction::new_signed_with_payer(
        &[
            system_instruction::create_account(
                &payer.pubkey(),
                &mint.pubkey(),
                mint_rent,
                Mint::LEN as u64,
                &spl_token::id(),
            ),
            token_instruction::initialize_mint(
                &spl_token::id(),
                &mint.pubkey(),
                &payer.pubkey(),
                None,
                0,
            )
            .unwrap(),
        ],
        Some(&payer.pubkey()),
        &[payer, &mint],
        *recent_blockhash,
    );

    banks_client.process_transaction(transaction).await.unwrap();
    mint.pubkey()
}

async fn create_token_account(
    banks_client: &mut BanksClient,
    payer: &Keypair,
    recent_blockhash: &Hash,
    mint: &Pubkey,
    owner: &Pubkey,
) -> Pubkey {
    let token_account = Keypair::new();
    let rent = banks_client.get_rent().await.unwrap();
    let token_account_rent = rent.minimum_balance(TokenAccount::LEN);

    let transaction = Transaction::new_signed_with_payer(
        &[
            system_instruction::create_account(
                &payer.pubkey(),
                &token_account.pubkey(),
                token_account_rent,
                TokenAccount::LEN as u64,
                &spl_token::id(),
            ),
            token_instruction::initialize_account(
                &spl_token::id(),
                &token_account.pubkey(),
                mint,
                owner,
            )
            .unwrap(),
        ],
        Some(&payer.pubkey()),
        &[payer, &token_account],
        *recent_blockhash,
    );

    banks_client.process_transaction(transaction).await.unwrap();
    token_account.pubkey()
}

#[tokio::test]
async fn test_vault_initialize() {
    let mut program_test = program_test();
    let (mut banks_client, payer, recent_blockhash) = program_test.start().await;

    let vault = Keypair::new();
    let owner = Keypair::new();
    let mint = create_mint(&mut banks_client, &payer, &recent_blockhash).await;

    let rent = banks_client.get_rent().await.unwrap();
    let vault_rent = rent.minimum_balance(VaultState::LEN);

    let transaction = Transaction::new_signed_with_payer(
        &[
            system_instruction::create_account(
                &payer.pubkey(),
                &vault.pubkey(),
                vault_rent,
                VaultState::LEN as u64,
                &nebula_vault::id(),
            ),
            initialize(&nebula_vault::id(), &vault.pubkey(), &owner.pubkey(), &mint),
        ],
        Some(&payer.pubkey()),
        &[&payer, &vault],
        recent_blockhash,
    );

    banks_client.process_transaction(transaction).await.unwrap();

    let vault_account = banks_client.get_account(vault.pubkey()).await.unwrap().unwrap();
    let vault_state = VaultState::try_from_slice(&vault_account.data).unwrap();

    assert_eq!(vault_state.owner, owner.pubkey());
    assert_eq!(vault_state.token_mint, mint);
    assert_eq!(vault_state.balance, 0);
}

#[tokio::test]
async fn test_vault_deposit() {
    let mut program_test = program_test();
    let (mut banks_client, payer, recent_blockhash) = program_test.start().await;

    let vault = Keypair::new();
    let owner = Keypair::new();
    let mint = create_mint(&mut banks_client, &payer, &recent_blockhash).await;
    let token_account = create_token_account(&mut banks_client, &payer, &recent_blockhash, &mint, &owner.pubkey()).await;

    // init vault
    let rent = banks_client.get_rent().await.unwrap();
    let vault_rent = rent.minimum_balance(VaultState::LEN);

    let transaction = Transaction::new_signed_with_payer(
        &[
            system_instruction::create_account(
                &payer.pubkey(),
                &vault.pubkey(),
                vault_rent,
                VaultState::LEN as u64,
                &nebula_vault::id(),
            ),
            initialize(&nebula_vault::id(), &vault.pubkey(), &owner.pubkey(), &mint),
        ],
        Some(&payer.pubkey()),
        &[&payer, &vault],
        recent_blockhash,
    );

    banks_client.process_transaction(transaction).await.unwrap();

    // deposit tokens
    let amount = 100;
    let transaction = Transaction::new_signed_with_payer(
        &[deposit(
            &nebula_vault::id(),
            &vault.pubkey(),
            &owner.pubkey(),
            &token_account,
            amount,
        )],
        Some(&payer.pubkey()),
        &[&payer, &owner],
        recent_blockhash,
    );

    banks_client.process_transaction(transaction).await.unwrap();

    let vault_account = banks_client.get_account(vault.pubkey()).await.unwrap().unwrap();
    let vault_state = VaultState::try_from_slice(&vault_account.data).unwrap();

    assert_eq!(vault_state.balance, amount);
}
