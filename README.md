# Rust PGP Decryptor Widget

A silly widget to handle decryption of PGP data with sensitive values in text files, like YAML configs.

## Install Rust (as needed)
https://www.rust-lang.org/tools/install
```shell
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

# Build and Use

Ensure the latest code is available
```shell
cd ./rust_pgp_decryptor_widget
git pull
```
Build the binary
```shell
make update
make build
```
Use an alias for convenience
```shell
alias pgp_decryptor='$HOME/repos/rust/rust_pgp_decryptor_widget/target/release/rust_pgp_decryptor_widget $*'
```
Example usage
```shell
pgp_decryptor --help
```
```shell
pgp_decryptor -i secrets.yaml -P '<PASSWORD>'
PGP_KEY_PASSPHRASE='<PASSWORD>' pgp_decryptor -i secrets.yaml
```
```shell
% export PGP_KEY_PASSPHRASE='<PASSWORD>'
% sdiff secrets.yaml <(pgp_decryptor -i secrets.yaml)
---                                                             ---
# secrets.yaml                                                  # secrets.yaml
apiVersion: v1                                                  apiVersion: v1
kind: Secret                                                    kind: Secret
metadata:                                                       metadata:
  name: secrets-test                                              name: secrets-test
  namespace: test                                                 namespace: test
type: Opaque                                                    type: Opaque
stringData:                                                     stringData:
  # gpg --encrypt --armor --recipient foo@bar.net <<EOF           # gpg --encrypt --armor --recipient foo@bar.net <<EOF
  # key: <PLACEHOLDER>                                            # key: <PLACEHOLDER>
  # EOF                                                           # EOF
  # gpg --decrypt                                                 # gpg --decrypt
  -----BEGIN PGP MESSAGE-----                                 |   key: test
                                                              <
  HF4D6u0zIETweqYSAQdAME+o82H/CW3AZklCR3phwgMm1EGqVt3xV20xyc1 <
  hKYV3NocIe6BoLJxkyiSD5oGwooSBdb0c3eTI/BaNlvbRVK7eY7OUGFFIoi <
  1E8BCQIQ6+O6kPmtMvvv8//IByX6WM5s4ycK4iGh+Po7+v9Vte4ReCMhBz/ <
  sIuMiPlUWwAZx0FTg6Dm6Nnq3AU8+Rt8aSuxrlAWdr/0                <
  =oC07                                                       <
  -----END PGP MESSAGE-----                                   <
```

# Develop and Test

Ensure the latest code is available
```shell
cd ./rust_pgp_decryptor_widget
git pull
```
Test with the latest version of rust and added dependencies 
```shell
rm Cargo.lock
make update
make format
make lint
make test
```
Check in any updates
```shell
git status
git add Cargo.toml
git commit -m 'Update dependencies to latest'
git push
```
Kill off any active gpg session cache
```shell
gpgconf --kill gpg-agent
echo RELOADAGENT | gpg-connect-agent
gpg-connect-agent killagent /bye
```
TEST, Test, and test again
```shell
export PGP_KEY_PASSPHRASE='<PASSWORD>'
cargo run -- -i secrets.yaml
```
```shell
make build
./target/release/rust_pgp_decryptor_widget -i secrets.yaml
```