
import * as anchor from "@coral-xyz/anchor";
import { Program, BN } from "@coral-xyz/anchor";
import {
    Keypair,
    LAMPORTS_PER_SOL,
    PublicKey,
    SystemProgram,
    Transaction,
  } from "@solana/web3.js";
  import {
    ASSOCIATED_TOKEN_PROGRAM_ID,
    MINT_SIZE,
    TOKEN_PROGRAM_ID,
    createAssociatedTokenAccountIdempotentInstruction,
    createInitializeMint2Instruction,
    createMintToInstruction,
    getAssociatedTokenAddressSync,
    getMinimumBalanceForRentExemptMint,
  } from "@solana/spl-token";

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
const token_b = new Keypair();
const token_a = new Keypair();
const escrow = PublicKey.findProgramAddressSync([Buffer.from("escrow", "utf-8"), maker.toBuffer(), new BN(1).toBuffer("le", 8)], anchor_escrow_2024)[0]
const maker_ata_a = getAssociatedTokenAddressSync(token_a.publicKey, maker.publicKey);
const taker_ata_a = getAssociatedTokenAddressSync(token_a.publicKey, taker.publicKey);
const taker_ata_b = getAssociatedTokenAddressSync(token_b.publicKey, taker.publicKey);
const maker_ata_b = getAssociatedTokenAddressSync(token_b.publicKey, maker.publicKey);
const accountsPublicKeys = {
maker: maker.publicKey,
taker: taker.publicKey,
token_b: token_b.publicKey,
token_a: token_a.publicKey,
escrow,
maker_ata_a,
taker_ata_a,
taker_ata_b,
maker_ata_b
}

    it("setup", async() => {
        let lamports = await getMinimumBalanceForRentExemptMint(connection);
        let tx = new Transaction();
        let instructions = [
            SystemProgram.transfer({
            fromPubkey: provider.publicKey,
            toPubkey: maker.publicKey,
            lamports: 10,
          }),
SystemProgram.transfer({
            fromPubkey: provider.publicKey,
            toPubkey: taker.publicKey,
            lamports: 10,
          }),
SystemProgram.createAccount({
            fromPubkey: provider.publicKey,
            newAccountPubkey: token_b.publicKey,
            lamports,
            space: MINT_SIZE,
            programId: TOKEN_PROGRAM_ID,
          }),
SystemProgram.createAccount({
            fromPubkey: provider.publicKey,
            newAccountPubkey: token_a.publicKey,
            lamports,
            space: MINT_SIZE,
            programId: TOKEN_PROGRAM_ID,
          }),
createInitializeMint2Instruction(
            token_a.publicKey,
            6,
            maker.publicKey,
            null
          ),
createAssociatedTokenAccountIdempotentInstruction(provider.publicKey, maker_ata_a, maker.publicKey, token_a.publicKey),
createMintToInstruction(token_a.publicKey, maker_ata_a, maker.publicKey, 1000000000),
createInitializeMint2Instruction(
            token_b.publicKey,
            6,
            taker.publicKey,
            null
          ),
createAssociatedTokenAccountIdempotentInstruction(provider.publicKey, taker_ata_b, taker.publicKey, token_b.publicKey),
createMintToInstruction(token_b.publicKey, taker_ata_b, taker.publicKey, 1000000000)
        ];
        await provider.sendAndConfirm(tx, [token_b, token_a, maker, taker]).then(log);
    })

    it("Make", async() => {
                const accounts = {associatedTokenProgram: accountsPublicKeys[associated_token_program], escrow: accountsPublicKeys[escrow], maker: accountsPublicKeys[maker], makerAtaA: accountsPublicKeys[maker_ata_a], mintA: accountsPublicKeys[token_a], mintB: accountsPublicKeys[token_b], systemProgram: accountsPublicKeys[system_program], tokenProgram: accountsPublicKeys[token_program], vault: accountsPublicKeys[vault]}
                await program.methods
                .make(new BN(1), new BN(1000000), new BN(1000000))
                .accounts({ ...accounts })
.signers([maker])
                .rpc()
                .then(confirm)
                .then(log);
            });
})