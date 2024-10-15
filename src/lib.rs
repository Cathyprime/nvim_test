use nvim_oxi::{
    Result,
    api::{
        self,
        opts::EchoOpts,
    }
};

#[nvim_oxi::plugin]
fn nvim_test() {
    let echo_opts = EchoOpts::default();
    let chunks = [
        ("Hello, from nvim-oxi", Some("Normal"))
    ];
    let _ = api::echo(chunks, false, &echo_opts);
}
