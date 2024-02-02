
import * as anchor from "@coral-xyz/anchor";
import { Program, BN } from "@coral-xyz/anchor";
    describe("Escrow", () => {
        anchor.setProvider(anchor.AnchorProvider.env());

        const provider = anchor.getProvider();

        const connection = provider.connection;

        const program = anchor.workspace.AnchorEscrow as Program<AnchorEscrow>;

        const confirm = async (signature: string): Promise<string> => {
            const block = await connection.getLatestBlockhash();
            await connection.confirmTransaction({
                signature,
                ...block,
            });
        return signature;
    };

    const log = async (signature: string): Promise<string> => {
        console.log(
        `Your transaction signature: https://explorer.solana.com/transaction/${signature}?cluster=custom&customUrl=${connection.rpcEndpoint}`
        );
        return signature;
    };

    // Accounts
    
const accountsPublicKeys = {

}

    it("setup", async() => {
        let instructions = [
            
        ];
    })

    it("Make", async() => {
                const accounts = {associatedTokenProgram: accountsPublicKeys[associated_token_program], escrow: accountsPublicKeys[escrow], maker: accountsPublicKeys[maker], makerAtaA: accountsPublicKeys[maker_ata_a], mintA: accountsPublicKeys[token_a], mintB: accountsPublicKeys[token_b], systemProgram: accountsPublicKeys[system_program], tokenProgram: accountsPublicKeys[token_program], vault: accountsPublicKeys[vault]}
                await program.methods
                .make(new BN(1), new BN(1000000), new BN(1000000))
                .accounts({ ...accounts })
                .signers([])
                .rpc()
                .then(confirm)
                .then(log);
            });
})