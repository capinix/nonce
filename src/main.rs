use std::fs;
use std::convert::TryInto;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
enum Command {
    /// Display nonce in the specified or default nonce file
    /// the default file is $HOME/.orga-wallet/nonce
    #[structopt(name = "read")]
    Read {
        /// Optional file from which to read the nonce
        file: Option<String>,
    },
    /// Write nonce to the specified or default nonce file
    #[structopt(name = "write")]
    Write {
        /// Integer value to write as nonce
        nonce: i64,
        /// optional file to write the nonce to
        file: Option<String>,
    },
}

fn main() -> Result<(), String> {
    let default_nonce_file = dirs::home_dir()
        .ok_or_else(|| "Failed to get home directory".to_string())?
        .join(".orga-wallet")
        .join("nonce");

    let command = Command::from_args();
    match command {
        Command::Read { file } => {
            let file_path = file
                .map(PathBuf::from)
                .unwrap_or_else(|| default_nonce_file.clone());
            let contents =
                std::fs::read(&file_path).map_err(|e| format!("Failed to read file: {}", e))?;
            let nonce = u64::from_be_bytes(
                contents
                    .try_into()
                    .map_err(|e| format!("Failed to convert file contents: {:?}", e))?,
            );
            println!("{}", nonce.to_string());
        }
        Command::Write { nonce, file } => {
            let file_path = file
                .map(PathBuf::from)
                .unwrap_or_else(|| default_nonce_file.clone());

            let bytes: [u8; 8] = nonce.to_be_bytes();

            if let Err(e) = fs::write(&file_path, &bytes) {
                eprintln!("Error writing to file: {}", e);
            } else {
                println!("{} written to {:?} successfully.", &nonce, &file_path);
            }
        }
    }
    Ok(())
}
