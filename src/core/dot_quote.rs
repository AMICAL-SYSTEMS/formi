use alloc::{format, vec};

crate::define_word!(DotQuote, ".\"", |it, tks| {
    let mut vec_token = vec![];

    for token in tks {
        match token {
            "\"" => break,
            other => vec_token.push(other),
        }
    }

    it.stdout_write(format!("{:?}", vec_token).as_bytes())?;

    Ok(())
});
