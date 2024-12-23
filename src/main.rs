use std::env;
use std::error::Error;
use std::fs;
use std::path::PathBuf;
use std::process;

type Result<T> = std::result::Result<T, Box<dyn Error>>;

#[derive(Debug)]
struct Config {
    error: String,
    changelog_path: PathBuf,
    commit_message: String,
    github_output_path: String,
}

impl Config {
    pub fn new() -> Result<Self> {
        let mut args = env::args().skip(1);

        // Take ownership of each argument directly
        let error = args.next().ok_or("Missing error argument")?;
        let changelog_path_str = args.next().ok_or("Missing changelog path")?;
        let commit_message = args.next().ok_or("Missing commit message")?;

        if args.next().is_some() {
            return Err("Too many arguments provided".into());
        }

        let github_output_path =
            env::var("GITHUB_OUTPUT").map_err(|_| "GITHUB_OUTPUT environment variable not set")?;

        if !error.is_empty() {
            Self::exit_with_error(&error, &github_output_path);
        }

        if changelog_path_str.is_empty() {
            Self::exit_with_error("No changelog path specified", &github_output_path);
        }

        let changelog_path = PathBuf::from(changelog_path_str);
        if !changelog_path.is_file() {
            Self::exit_with_error(
                "The specified changelog could not be found",
                &github_output_path,
            );
        }

        Ok(Self {
            error,
            changelog_path,
            commit_message,
            github_output_path,
        })
    }

    fn write_github_output(error_msg: &str, github_output_path: &str) -> Result<()> {
        fs::write(github_output_path, format!("error={error_msg}"))
            .map_err(|e| format!("Failed to write to GITHUB_OUTPUT: {e}"))?;
        Ok(())
    }

    fn exit_with_error(error_msg: &str, github_output_path: &str) -> ! {
        eprintln!("Error: {error_msg}");
        if let Err(e) = Self::write_github_output(error_msg, github_output_path) {
            eprintln!("Additional error when writing output: {e}");
        }
        process::exit(1);
    }
}

fn run() -> Result<()> {
    let config = Config::new()?;
    // Add your main logic here
    Ok(())
}

fn main() {
    if let Err(err) = run() {
        if let Ok(github_output_path) = env::var("GITHUB_OUTPUT") {
            Config::exit_with_error(&err.to_string(), &github_output_path);
        } else {
            eprintln!("Error: {} (Failed to access GITHUB_OUTPUT)", err);
            process::exit(1);
        }
    }
}
