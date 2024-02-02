
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
    const maker = new Keypair();
const taker = new Keypair();
const token_a = new Keypair();
const token_b = new Keypair();
const escrow = PublicKey.findProgramAddressSync([], anchor_escrow_2024)[0]
const taker_ata_b = getAssociatedTokenAddressSync(token_b.publicKey, taker.publicKey);
const taker_ata_a = getAssociatedTokenAddressSync(token_a.publicKey, taker.publicKey);
const maker_ata_b = getAssociatedTokenAddressSync(token_b.publicKey, maker.publicKey);
const maker_ata_a = getAssociatedTokenAddressSync(token_a.publicKey, maker.publicKey);
const accountsPublicKeys = {
maker: maker.publicKey,
taker: taker.publicKey,
token_a: token_a.publicKey,
token_b: token_b.publicKey,
escrow,
taker_ata_b,
taker_ata_a,
maker_ata_b,
maker_ata_a
}

    it("setup", async() => {
        let instructions = [
            
        ];
    })

    it("Make", async() => {
                const accounts = {associatedTokenProgram: accountsPublicKeys[associated_token_program], escrow: accountsPublicKeys[escrow], maker: accountsPublicKeys[maker], makerAtaA: accountsPublicKeys[maker_ata_a], mintA: accountsPublicKeys[token_a], mintB: accountsPublicKeys[token_b], systemProgram: accountsPublicKeys[system_program], tokenProgram: accountsPublicKeys[token_program], vault: accountsPublicKeys[vault}]
                await program.methods
                .make(new BN(1), new BN(1000000), new BN(1000000))
                .accounts({ ...accounts })
                .signers([])
                .rpc()
                .then(confirm)
                .then(log);
            });
})