use anchor_lang::prelude::*;

declare_id!("7hspbAvDn3m8XhTFquf4RU3yChGRa2VspmT44PthykE4");

#[program]
pub mod delegated_voting_exercise {
    use super::*;
    pub fn initialize(ctx: Context<Initialize>) -> ProgramResult {
        let proposals_account = &mut ctx.accounts.proposals_account;
        proposals_account.proposal_1 = 0;
        proposals_account.proposal_2 = 0;
        Ok(())
    }

    pub fn add_voter(ctx: Context<AddVoter>, public_key: Pubkey) -> ProgramResult {
        let voter_account = &mut ctx.accounts.voter_account;
        voter_account.num_votes = 1;
        voter_account.public_key = public_key;
        Ok(())
    }

    pub fn delegate_vote(ctx: Context<DelegateVote>) -> ProgramResult {
        let vote_delegator_account = &mut ctx.accounts.vote_delegator_account;
        let vote_delegatee_account = &mut ctx.accounts.vote_delegatee_account;
        vote_delegatee_account.num_votes += vote_delegator_account.num_votes;
        vote_delegator_account.num_votes = 0;
        Ok(())
    }

    pub fn vote(ctx: Context<Vote>, proposal: u64) -> ProgramResult {
        let proposals_account = &mut ctx.accounts.proposals_account;
        let voter_account = &mut ctx.accounts.voter_account;
        let num_votes = voter_account.num_votes;
        match proposal {
            0 => proposals_account.proposal_1 += num_votes,
            1 => proposals_account.proposal_2 += num_votes,
            _ => return Err(ErrorCode::NoSuchProposal.into()),
        }
        voter_account.num_votes = 0;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = user, space = 16 + 16)]
    pub proposals_account: Account<'info, ProposalsAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct AddVoter<'info> {
    #[account(init, payer = user, space = 16 + 32)]
    pub voter_account: Account<'info, VoterAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct DelegateVote<'info> {
    #[account(mut)]
    pub vote_delegator_account: Account<'info, VoterAccount>,
    #[account(mut)]
    pub vote_delegatee_account: Account<'info, VoterAccount>,
    pub signer: Signer<'info>,
}

#[derive(Accounts)]
pub struct Vote<'info> {
    #[account(mut)]
    pub proposals_account: Account<'info, ProposalsAccount>,
    #[account(mut)]
    pub voter_account: Account<'info, VoterAccount>,
    pub signer: Signer<'info>,
}

#[account]
pub struct VoterAccount {
    pub num_votes: u64,
    pub public_key: Pubkey,
}

#[account]
pub struct ProposalsAccount {
    pub proposal_1: u64,
    pub proposal_2: u64,
}

#[error]
pub enum ErrorCode {
    #[msg("No such proposal: Please recast your vote")]
    NoSuchProposal,
}
