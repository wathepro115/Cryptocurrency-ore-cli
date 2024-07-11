use solana_sdk::{instruction::Instruction, signer::Signer};

use crate::{utils::get_relayer_escrow, Miner};

impl Miner {
    pub async fn build_collect_ix(&self) -> Instruction {
        let rpc_client = self.rpc_client.clone();
        let signer = self.signer();
        let escrow = get_relayer_escrow(&rpc_client, signer.pubkey()).await;
        let beneficiary = self.initialize_ata().await;
        ore_relay_api::instruction::collect(signer.pubkey(), escrow, beneficiary)
    }
}
