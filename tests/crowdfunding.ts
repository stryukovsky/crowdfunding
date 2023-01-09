import * as anchor from "@coral-xyz/anchor";
import {Program} from "@coral-xyz/anchor";
import {Crowdfunding} from "../target/types/crowdfunding";
import {expect} from "chai";

describe("crowdfunding", () => {
    anchor.setProvider(anchor.AnchorProvider.env());

    const program = anchor.workspace.Crowdfunding as Program<Crowdfunding>;
    const account_campaign = anchor.web3.Keypair.generate();

    it("should be initialized", async () => {
        await program.methods.initialize("Coco's rolls royce", "A good car to a good cat")
            .accounts({
                campaign: account_campaign.publicKey,
            })
            .signers([account_campaign])
            .rpc();
    });

    it("should receive money", async () => {
        const balanceBefore = (await anchor.getProvider().connection.getAccountInfo(account_campaign.publicKey)).lamports;
        const amount = 1000;
        await program.methods.donate(new anchor.BN(amount))
            .accounts({
                campaign: account_campaign.publicKey,
                sender: anchor.getProvider().publicKey,
            })
            .rpc();
        const balanceAfter = (await anchor.getProvider().connection.getAccountInfo(account_campaign.publicKey)).lamports;
        const campaign = await program.account.campaign.fetch(account_campaign.publicKey);
        expect(campaign.amountDonated.toNumber()).to.be.eq(amount);
        expect(balanceAfter - balanceBefore).to.be.eq(amount);
    });

    it("should withdraw money", async () => {
        const balanceBefore = (await anchor.getProvider().connection.getAccountInfo(account_campaign.publicKey)).lamports;
        const amount = 500;
        await program.methods.withdraw(new anchor.BN(amount))
            .accounts({
                campaign: account_campaign.publicKey,
                signer: anchor.getProvider().publicKey,
            })
            .rpc();
        const balanceAfter = (await anchor.getProvider().connection.getAccountInfo(account_campaign.publicKey)).lamports;
        expect(balanceBefore - balanceAfter).to.be.eq(amount);
    });
});
