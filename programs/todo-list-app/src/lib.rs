use anchor_lang::prelude::*;

declare_id!("H52i4cUPbh7CUoqafWm6bpTVFnENAJkrSYrrGB5CYifz");

#[program]
pub mod todo_list_app {
    use super::*;
    // these are functions in the main app and program
    pub fn adding_task(ctx: Context<AddingTask>, text: String) -> Result<()> {
        let task = &mut ctx.accounts.task;
        let author = &ctx.accounts.author; // The `author` account
        let clock = Clock::get().unwrap(); // Getting the current timestamp

        if text.chars().count() > 400 {
            return Err(ErrorCode::TextTooLong.into());
        }

        task.author = *author.key;
        task.is_done = false;
        task.created_at = clock.unix_timestamp;
        task.updated_at = clock.unix_timestamp;
        task.text = text;
        Ok(())
    }

    pub fn updating_task(ctx: Context<UpdatingTask>, is_done: bool) -> Result<()> {
        let task = &mut ctx.accounts.task;
        let author = &ctx.accounts.author; // The `author` account
        let clock = Clock::get().unwrap(); // Getting the current timestamp

        task.author = *author.key;
        task.is_done = is_done;
        task.updated_at = clock.unix_timestamp;
        Ok(())
    }

    pub fn deleting_task(ctx: Context<DeletingTask>) -> Result<()> {
        let task = &mut ctx.accounts.task;
        let author = &ctx.accounts.author; // The `author` account
        let clock = Clock::get().unwrap(); // Getting the current timestamp

        task.author = *author.key;
        task.is_done = true;
        task.updated_at = clock.unix_timestamp;
        Ok(())
    }
}

#[derive(Accounts)]
//this defines the the Adding  task function and the value it accepts and its type
pub struct AddingTask<'info> {
    #[account(init, payer = author, space = Task::LEN)]
    pub task: Account<'info, Task>,
    #[account(mut)]
    pub author: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
//this defines the the Updating  task function and the value it accepts and its type

pub struct UpdatingTask<'info> {
    #[account(mut, has_one = author)]
    pub task: Account<'info, Task>,
    pub author: Signer<'info>,
}

#[derive(Accounts)]
//this defines the the Deleting task function and the value it accepts and its type
pub struct DeletingTask<'info> {
    #[account(mut, has_one = author)]
    pub task: Account<'info, Task>,
    pub author: Signer<'info>,
}

#[account]
//this defines how a task would look like and its type
pub struct Task {
    pub author: Pubkey,  // The account that owns the task
    pub is_done: bool,   // Whether the task is done or not
    pub text: String,    // The text of the task
    pub created_at: i64, // The timestamp when the task was created
    pub updated_at: i64, // The timestamp when the task was last updated
}

// to declare the size of the program and other values
const DISCRIMINATOR: usize = 8;
const PUBLIC_KEY_LENGTH: usize = 32;
const BOOL_LENGTH: usize = 1;
const TEXT_LENGTH: usize = 4 + 400 * 4; // 400 chars
const TIMESTAMP_LENGTH: usize = 8;

//A task implementation
impl Task {
    const LEN: usize = DISCRIMINATOR + // discriminator
        PUBLIC_KEY_LENGTH + // author
        BOOL_LENGTH + // is_done
        TEXT_LENGTH +  // text
        TIMESTAMP_LENGTH + // created_at
        TIMESTAMP_LENGTH; // updated_at
}

#[error_code]
//this defines the error code
pub enum ErrorCode {
    #[msg("The text is too long")]
    TextTooLong,
}
