use nvim_oxi::{
    Result,
    api::{
        self,
        opts::EchoOpts,
    }
};

#[nvim_oxi::plugin]
fn echo() -> Result<()> {
    let echo_opts = EchoOpts::default();
    let chunks = [
        ("Hello, from nvim-oxi", Some("Normal"))
    ];
    Ok(api::echo(chunks, false, &echo_opts)?)
}
