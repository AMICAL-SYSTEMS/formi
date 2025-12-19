//! 6.1.1700 IF
//! CORE
//!
//!     Interpretation: Interpretation semantics for this word are undefined.
//!
//!     Compilation: ( C: -- orig )
//!
//! Put the location of a new unresolved forward reference orig onto the
//! control flow stack. Append the run-time semantics given below to the
//! current definition. The semantics are incomplete until orig is resolved,
//! e.g., by THEN or ELSE.
//!
//!     Run-time: ( x -- )
//!
//! If all bits of x are zero, continue execution at the location specified by the resolution of orig.
//!
//! [`https://www.taygeta.com/forth/dpans6.htm#6.1.1700`]

use crate::types::{Flag, TRUE_FLAG};
use alloc::vec;

crate::define_word!(If, "IF", |it, tks| {
    let if_condition: Flag = it.pop_last_stack()? as Flag;

    if if_condition == TRUE_FLAG {
        // Do IF branch
        let mut if_branch_tokens = vec![];
        for next_token in tks.by_ref() {
            if_branch_tokens.push(next_token);
            if next_token == "ELSE" || next_token == "THEN" {
                break;
            }
        }

        // Skip until THEN (might include ELSE)
        for next_token in tks.by_ref() {
            if next_token == "THEN" {
                break;
            }
        }

        it.execute_tokens(if_branch_tokens.join(" "))
    } else {
        // Do ELSE branch, ignore IF branch
        let mut else_branch_tokens = vec![];
        for next_token in tks.by_ref() {
            if next_token == "ELSE" {
                break;
            }
        }
        for next_token in tks.by_ref() {
            else_branch_tokens.push(next_token);
            if next_token == "THEN" {
                break;
            }
        }

        it.execute_tokens(else_branch_tokens.join(" "))
    }
});
