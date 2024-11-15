use clap::ColorChoice;
use clap::{Parser, ValueEnum};
use std::fmt::Display;
use std::str::FromStr;

#[derive(Parser)]
#[command(name = "uaup-cli", version, author, color = ColorChoice::Auto)]
pub struct UaupCliArgs {
	#[clap(subcommand)]
	pub subcmd: SubCommand,

}

#[derive(Parser)]
pub enum SubCommand {
	#[command(name = "install")]
	Install(InstallArgs),
	#[command(name = "uninstall")]
	Uninstall(UninstallArgs),
	#[command(name = "update")]
	Update(UpdateArgs),
	#[command(name = "create", color = ColorChoice::Auto)]
	/// Provides options to create an update
	Create(CreateArgs),
}

#[derive(Parser)]
pub struct InstallArgs {
	#[arg(short, long)]
	pub force: bool,
}

#[derive(Parser)]
pub struct UninstallArgs {
	#[arg(short, long)]
	pub force: bool,
}

#[derive(Parser)]
pub struct UpdateArgs {
	#[arg(short, long)]
	pub force: bool,
}

#[derive(Parser)]
pub struct CreateArgs {
	#[arg(short, long, default_value = "./updates")]
	/// The directory where the updates will be stored
	pub output: Option<String>,
	#[arg(short, long, default_value = "./uaup.toml")]
	/// This will contain the configuration for the update
	/// If not provided, the default value is "./uaup.toml"
	pub config_file: Option<String>,
	#[arg(
		short='t',
		long="type",
		default_value_t  =  ProjectType::Other,
		default_value = "other",
		value_enum
	)]
	/// The type of project you are creating the update for,
	/// this will allow the fetching of information from project files
	/// Ex: NodeJS will fetch the version from the package.json file
	pub project_type: ProjectType,

	#[arg(long)]
	/// This will not push the archives to the server and will not increment the version number in the config file
	pub dry_run: bool,
}

#[derive(ValueEnum, Copy, Clone, Debug, PartialEq, Eq)]
pub enum ProjectType {
	NodeJS,
	Rust,
	Java,
	DotNet,
	Python,
	Other,
}

impl FromStr for ProjectType {
	type Err = String;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match s.to_lowercase().as_str() {
			"nodejs" => Ok(ProjectType::NodeJS),
			"rust" => Ok(ProjectType::Rust),
			"java" => Ok(ProjectType::Java),
			"dotnet" => Ok(ProjectType::DotNet),
			"python" => Ok(ProjectType::Python),
			"other" => Ok(ProjectType::Other),
			_ => Err("Invalid project type".to_string())
		}
	}
}

impl Display for ProjectType {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			ProjectType::NodeJS => write!(f, "NodeJS"),
			ProjectType::Rust => write!(f, "Rust"),
			ProjectType::Java => write!(f, "Java"),
			ProjectType::DotNet => write!(f, "DotNet"),
			ProjectType::Python => write!(f, "Python"),
			ProjectType::Other => write!(f, "Other"),
		}
	}
}