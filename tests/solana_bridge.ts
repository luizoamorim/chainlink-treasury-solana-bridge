import * as anchor from '@coral-xyz/anchor';
import { Program } from '@coral-xyz/anchor';
import { SolanaBridge } from '../target/types/solana_bridge';
import { createMint, getOrCreateAssociatedTokenAccount, getAccount } from '@solana/spl-token';
import { assert } from 'chai';

describe('🔗 Chainlink Treasury Bridge', () => {
	const provider = anchor.AnchorProvider.env();
	anchor.setProvider(provider);

	const program = anchor.workspace.SolanaBridge as Program<SolanaBridge>;

	let wlinkMint: anchor.web3.PublicKey;
	let usdcMint: anchor.web3.PublicKey;
	const admin = provider.wallet;
	const configAccount = anchor.web3.Keypair.generate();

	it('🚀 Setup: Cria os Tokens e Inicializa o Contrato', async () => {
		// 1. Criar wLINK
		wlinkMint = await createMint(provider.connection, (admin as any).payer, admin.publicKey, null, 6);
		console.log('   🟦 wLINK Mint criado:', wlinkMint.toBase58());

		// 2. Criar USDC
		usdcMint = await createMint(provider.connection, (admin as any).payer, admin.publicKey, null, 6);
		console.log('   💵 USDC Mint criado:', usdcMint.toBase58());

		// 3. Inicializar
		await program.methods
			.initialize(wlinkMint, usdcMint)
			.accounts({
				config: configAccount.publicKey,
				signer: admin.publicKey,
				// REMOVIDO: systemProgram (Anchor resolve sozinho)
			})
			.signers([configAccount])
			.rpc();

		console.log('   ✅ Contrato Inicializado com sucesso!');
	});

	it('🔗 Fluxo LINK: Lock & Mint', async () => {
		const userATA = await getOrCreateAssociatedTokenAccount(provider.connection, (admin as any).payer, wlinkMint, admin.publicKey);

		const amountToMint = new anchor.BN(100 * 1_000_000);

		await program.methods
			.mintWlink(amountToMint)
			.accounts({
				signer: admin.publicKey,
				config: configAccount.publicKey,
				wlinkMint: wlinkMint,
				userDestination: userATA.address,
				// REMOVIDO: tokenProgram (Anchor resolve sozinho)
			})
			.rpc();

		const balance = await getAccount(provider.connection, userATA.address);
		console.log('   💰 Saldo wLINK:', Number(balance.amount) / 1_000_000);
		assert.equal(Number(balance.amount), 100_000_000);
	});

	it('💵 Fluxo USDC: CCTP Simulation', async () => {
		const userATA = await getOrCreateAssociatedTokenAccount(provider.connection, (admin as any).payer, usdcMint, admin.publicKey);

		const amountToMint = new anchor.BN(500 * 1_000_000);
		const mockSignature = 'simulated_circle_signature_xyz';

		await program.methods
			.receiveCctpMessage(amountToMint, mockSignature)
			.accounts({
				signer: admin.publicKey,
				config: configAccount.publicKey,
				usdcMint: usdcMint,
				userDestination: userATA.address,
				// REMOVIDO: tokenProgram (Anchor resolve sozinho)
			})
			.rpc();

		const balance = await getAccount(provider.connection, userATA.address);
		console.log('   💰 Saldo USDC:', Number(balance.amount) / 1_000_000);
		assert.equal(Number(balance.amount), 500_000_000);
	});
});
