use solana_sdk::signer::Signer;

use crate::{
    send_and_confirm::ComputeBudget,
    utils::{get_relayer, get_relayer_proof, relayer_proof_pubkey},
    Miner,
};

impl Miner {
    pub async fn open_escrow(&self) {
        let rpc_client = self.rpc_client.clone();
        // Return early if miner is already registered
        let signer = self.signer();
        let proof_address = relayer_proof_pubkey(signer.pubkey());
        if rpc_client.get_account(&proof_address).await.is_ok() {
            println!("proof ok");
            return;
        }

        // Sign and send transaction
        println!("Generating challenge...");
        let relayer = get_relayer(&rpc_client).await;
        println!("relayer: {:?}", relayer);
        let ix = ore_relay_api::instruction::open_escrow(signer.pubkey(), relayer, signer.pubkey());
        let res = self
            .send_and_confirm(&[ix], ComputeBudget::Dynamic, false)
            .await;
        match res {
            Ok(_) => {
                println!("open escrow ix");
            }
            Err(err) => {
                println!("err on open escrow ix: {}", err);
            }
        }
    }
}
