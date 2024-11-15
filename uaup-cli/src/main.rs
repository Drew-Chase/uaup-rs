use clap::Parser;

mod uaup_cli_args;

fn main() {
	std::env::set_var("RUST_LOG", "debug");
	env_logger::init();
	let v = uaup_cli_args::UaupCliArgs::parse();
}
