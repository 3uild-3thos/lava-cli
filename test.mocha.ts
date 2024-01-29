
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
const maker_ata_b = getAssociatedTokenAddressSync(token_b.publicKey, maker.publicKey);
const taker_ata_b = getAssociatedTokenAddressSync(token_b.publicKey, taker.publicKey);
const maker_ata_a = getAssociatedTokenAddressSync(token_a.publicKey, maker.publicKey);
const taker_ata_a = getAssociatedTokenAddressSync(token_a.publicKey, taker.publicKey);
const accounts = {
maker: maker.publicKey,
taker: taker.publicKey,
token_a: token_a.publicKey,
token_b: token_b.publicKey,
escrow,
maker_ata_b,
taker_ata_b,
maker_ata_a,
taker_ata_a
}

    it("setup", async() => {
        let instructions = [
            
        ];
})