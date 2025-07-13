use clap::Parser;
use regex::Regex;
use std::fs;
use std::io::Write;
use std::process::{Command, Output};

/// A silly widget to handle decryption of PGP data with sensitive values in text files, like YAML configs.
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Passphrase used with decryption
    #[clap(short = 'P', long)]
    passphrase: Option<String>,

    /// Input file path of text containing encrypted PGP messages to decrypt
    #[clap(short = 'i', long, value_parser)]
    input_text: String,

    /// Output file path to write decrypted text
    #[clap(short = 'o', long, value_parser)]
    output_text: Option<String>,
}

/// Decrypts a PGP message using the `gpg` command-line tool.
fn decrypt_pgp_message(pgp_message: &str, passphrase: &str) -> Result<String, String> {
    let child = Command::new("gpg")
        .arg("--decrypt")
        .arg("--batch") // Do not prompt for interaction
        .arg("--yes") // Answer yes to any questions
        .arg("--no-tty") // Do not assume a tty is available
        .arg("--passphrase")
        .arg(passphrase)
        .arg("--pinentry-mode") // expect input on standard input (loopback)
        .arg("loopback")
        .arg("--always-trust")
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .map_err(|e| format!("Failed to spawn gpg command: {e}"))?;

    let mut stdin = child.stdin.as_ref().unwrap();
    stdin
        .write_all(pgp_message.as_bytes())
        .map_err(|e| format!("Failed to write to gpg stdin: {e}"))?;
    let _ = stdin; // Close stdin to allow gpg to proceed

    let Output {
        stdout,
        stderr,
        status,
    } = child
        .wait_with_output()
        .map_err(|e| format!("Failed to wait for gpg output: {e}"))?;

    if status.success() {
        Ok(String::from_utf8_lossy(&stdout).into_owned())
    } else {
        Err(format!(
            "GPG decryption failed: {}\n{}",
            String::from_utf8_lossy(&stderr),
            String::from_utf8_lossy(&stdout)
        ))
    }
}

/// Decrypts PGP messages embedded within the input text.
fn decrypt(input_text: &str, passphrase: &str) -> Result<String, String> {
    let mut output_lines = Vec::new();
    let mut pgp_message_lines: Option<Vec<String>> = None;
    let mut pgp_message_indent = String::new();

    let begin_regex = Regex::new(r"^\s*-----BEGIN PGP MESSAGE-----$").unwrap();
    let end_regex = Regex::new(r"^\s*-----END PGP MESSAGE-----$").unwrap();
    let indent_regex = Regex::new(r"^(?P<indent>\s*)(?P<line>[^\s].*)$").unwrap();

    for line in input_text.lines() {
        if begin_regex.is_match(line) {
            pgp_message_lines = Some(vec![line.trim().to_string()]);
            if let Some(captures) = indent_regex.captures(line) {
                pgp_message_indent = captures
                    .name("indent")
                    .map_or("", |m| m.as_str())
                    .to_string();
            }
        } else if end_regex.is_match(line) {
            if let Some(mut pgp_lines) = pgp_message_lines.take() {
                pgp_lines.push(line.trim().to_string());
                let full_pgp_message = pgp_lines.join("\n");
                let decrypted_message = decrypt_pgp_message(&full_pgp_message, passphrase)?;
                for decrypted_line in decrypted_message.trim().lines() {
                    output_lines.push(format!("{pgp_message_indent}{decrypted_line}"));
                }
                pgp_message_indent.clear(); // Reset indent for next message
            } else {
                output_lines.push(line.to_string());
            }
        } else if let Some(ref mut pgp_lines) = pgp_message_lines {
            pgp_lines.push(line.trim().to_string());
        } else {
            output_lines.push(line.to_string());
        }
    }

    Ok(output_lines.join("\n"))
}

fn main() -> Result<(), String> {
    let args = Args::parse();

    let input_content = fs::read_to_string(&args.input_text)
        .map_err(|e| format!("Failed to read input file '{}': {}", args.input_text, e))?;

    let passphrase = if let Some(p) = args.passphrase {
        p
    } else if let Ok(env_pass) = std::env::var("PGP_KEY_PASSPHRASE") {
        env_pass
    } else {
        rpassword::prompt_password("Key passphrase: ")
            .map_err(|e| format!("Failed to read passphrase: {e}"))?
    };

    let output_content = decrypt(&input_content, &passphrase)?;

    if let Some(output_path) = args.output_text {
        fs::write(&output_path, output_content)
            .map_err(|e| format!("Failed to write output to '{output_path}': {e}"))?;
        println!("Decrypted content written to '{output_path}'");
    } else {
        println!("{output_content}");
    }

    Ok(())
}
