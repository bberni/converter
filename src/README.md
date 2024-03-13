# Currency Converter

## Overview
This Currency Converter is a Rust-based command-line tool that allows users to convert amounts between different currencies. It utilizes the ExchangeRate-API for fetching real-time conversion rates and supports an interactive mode for ease of use. Additionally, it offers functionality to list all conversion rates for a specified currency.

## Features
- Convert amounts between two specified currencies.
- List all conversion rates for a specified currency.
- Interactive mode for easier user input handling.
- Caching of API responses to optimize repeated requests.

## Getting Started
### Prerequisites
- A [Rust Installation](https://www.rust-lang.org/learn/get-started) to build this project.
- An ExchangeRate-API API key, which you can get for free on their [website](https://www.exchangerate-api.com/).

### Installation
1. Clone the repository to your local machine:
   ```bash
   git clone https://github.com/bberni/converter
2. Build your project with cargo:
    ```bash
    cd converter
    cargo build --release
    ```
### Configuration
Before using the tool, you need to set up the environment variable for the API key:

Windows (PowerShell):
```powershell
$Env:EXCHANGERATE_API_KEY = "<your API key>"
```
Linux: 
```bash
export EXCHANGERATE_API_KEY="<your API key>"
```
### Usage
You can run the converter in interactive mode, where you will be prompted for all information needed for the conversion, or you can provide arguments directly.

**Basic usage:**
```bash
cargo run -- --from-currency USD --to-currency EUR --amount 100.00
```
**Interactive mode:**
```bash 
cargo run -- --interactive
```
**List conversion rates for given currency:**
```bash 
cargo run -- --list USD
```
