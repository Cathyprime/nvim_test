use nvim_oxi::{
    Dictionary,
    Function,
    api::{
        self,
        opts::EchoOpts,
    }
};

#[nvim_oxi::plugin]
fn nvim_test() -> nvim_oxi::Result<Dictionary> {
    let echo_opts = EchoOpts::default();

    let meow = Function::from(move |(msg, hl): (String, String)| {
        let chunks = [
            (msg.as_str(), Some(hl.as_str()))
        ];
        let _ = api::echo(chunks, false, &echo_opts);
    });

    let api = Dictionary::from_iter([
        ("meow", meow)
    ]);

    Ok(api)
}
