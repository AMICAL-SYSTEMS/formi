//! 6.1.1240 DO
//! do CORE
//!
//!     Interpretation: Interpretation semantics for this word are undefined.
//!
//!     Run-time: ( n1|u1 n2|u2 -- ) ( R: -- loop-sys )
//!
//! Setup loop control using start index n2|u2 and limit n1|u1 and execute the
//! loop body until the limit is reached. Loop termination is controlled by the
//! trailing `LOOP` or `+LOOP` token.
//!
//! [`https://www.taygeta.com/forth/dpans6.htm#6.1.1240`]
use crate::{error::RuntimeError, r#loop::LoopSys};
use alloc::vec;

crate::define_word!(Do, "DO", |it, tks| {
    let start = it.pop_last_stack()?;
    let limit = it.pop_last_stack()?;
    let loop_range = start..limit;

    let index = start;

    let loop_sys = LoopSys {
        range: loop_range,
        current_index: index,
    };

    let mut vec_token = vec![];
    let mut is_loop_plus = false;

    for token in tks {
        match token {
            "LOOP" => break,
            "+LOOP" => {
                is_loop_plus = true;
                break;
            }
            other => vec_token.push(other),
        }
    }

    loop_sys.to_stack(&mut it.return_stack);

    let looping_tokens = vec_token.join(" ");
    let mut intermediate_loop_sys = loop_sys;
    loop {
        if intermediate_loop_sys.current_index >= intermediate_loop_sys.range.end {
            break;
        } else {
            it.execute_tokens(looping_tokens.clone())?;
            if let Ok(stack_loop_sys) = LoopSys::pop_from_stack(&mut it.return_stack) {
                intermediate_loop_sys = stack_loop_sys;
                if is_loop_plus {
                    let n = it.stack.pop().ok_or(RuntimeError::EmptyStack)?;
                    intermediate_loop_sys.current_index += n;
                } else {
                    intermediate_loop_sys.current_index += 1;
                }
                intermediate_loop_sys.to_stack(&mut it.return_stack);
            } else {
                // No loop-sys, end loop.
                break;
            }
        }
    }

    // Remove loop-sys from stack, since we're done with it.
    let _ = LoopSys::pop_from_stack(&mut it.return_stack);

    Ok(())
});
