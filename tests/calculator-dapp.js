const assert = require('assert');
const anchor = require('@coral-xyz/anchor');

const { SystemProgram } = anchor.web3;

describe('calculator_dapp', () => {
  const provider = anchor.AnchorProvider.local();
  anchor.setProvider(provider);

  const calculator = anchor.web3.Keypair.generate();
  const program = anchor.workspace.calculator_dapp;

  it('Creates a Calculator', async () => {
    await program.methods
      .create("Welcome to Solana")
      .accounts({
        calculator: calculator.publicKey,
        user: provider.wallet.publicKey,
        systemProgram: SystemProgram.programId,
      })
      .signers([calculator])
      .rpc();

    const account = await program.account.calculator.fetch(calculator.publicKey);
    assert.strictEqual(account.greeting, "Welcome to Solana");
  });
});
