
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
    const taker = Keypair.generate();
const maker = Keypair.generate();
const token_a = new Keypair();
const token_b = new Keypair();
const escrow = PublicKey.findProgramAddressSync([], anchor_escrow_2024)[0]
const vault = getAssociatedTokenAddressSync(token_a.publicKey, escrow);
const taker_ata_a = getAssociatedTokenAddressSync(token_a.publicKey, taker.publicKey);
const make_ata_b = getAssociatedTokenAddressSync(token_b.publicKey, maker.publicKey);
const taker_ata_b = getAssociatedTokenAddressSync(token_b.publicKey, taker.publicKey);
const maker_ata_a = getAssociatedTokenAddressSync(token_a.publicKey, maker.publicKey);

    it("setup", async() => {
        let instructions = [
            
        ];
})