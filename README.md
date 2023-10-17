# sc-donations-rs

## 📜 About
The `sc-donations-rs` contracts are designed to facilitate charitable donations via the MultiversX blockchain. Through these contracts, users can send funds directly to specific organizations or make use of the quadratic funding mechanism to ensure fairer distribution of donations.

These contracts are built using [**xSuite**](https://xsuite.dev), a full-suite toolkit for developing MultiversX smart contracts. 

For the client-side implementation and integration, please refer to the [client repository](https://github.com/GiveCup/client).

## 🏗 Structure

- [**simple**](https://github.com/givecup/sc-donations-rs/tree/main/simple): Allows users to directly send funds to specific organizations.
  
- [**quadratic-funding**](https://github.com/givecup/sc-donations-rs/tree/main/quadratic-funding) Implements a quadratic funding mechanism which enhances the fairness in fund distribution. Quadratic funding is an innovative method wherein the amount of funding a project receives is proportional to the square of the sum of the square roots of contributions it receives. This ensures that projects with broader public support receive more funding, as opposed to those supported by a few large donors.

## 🛠 Testing & Deploying

xSuite provides an intuitive Command Line Interface (CLI) for seamless contract operations.

1. Install Rust (if not already installed):
    ```bash
    xsuite install-rust
    ```

2. Build the contract:
    ```bash
    xsuite build
    ```

3. To test the Rust parts of the contract:
    ```bash
    xsuite test-rust
    ```

4. For scenario tests:
    ```bash
    xsuite test-scen
    ```

5. Deployment:
    ```bash
    xsuite deploy
    ```

For a comprehensive understanding of all available commands, run:
    ```
    xsuite --help
    ```

To get specific help for a particular command, append `--help` to the command, like:
    ```
    xsuite new --help
    ```

## 🚀 Deployments

### Regular Donations
- **On devnet**: coming soon ⏳
- **On mainnet**: coming soon ⏳

### Quadratic Funding
- **On devnet**: coming soon ⏳
- **On mainnet**: coming soon ⏳

