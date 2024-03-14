# Converter

## Overview
This currency converter is a Rust-based command-line tool that allows users to convert amounts between different currencies. It utilizes the ExchangeRate-API for fetching accurate conversion rates and supports an interactive mode for ease of use. Additionally, it offers functionality to list all conversion rates for a specified currency.

## Features
- Converting amounts between two specified currencies.
- Interactive mode for easier usage.
- Listing all conversion rates for a specified currency.
- Caching of API responses to optimize repeated requests.

## Getting Started
### Prerequisites
- A [Rust Installation](https://www.rust-lang.org/learn/get-started) to build this project.
- An ExchangeRate-API API key, which you can get for free on their [website](https://www.exchangerate-api.com/).

### Building
1. Clone the repository to your local machine:
   ```bash
   git clone https://github.com/bberni/converter
2. Build your project with cargo:
    ```bash
    cd converter
    cargo build --release
    ```
    Compiled binary can be found at converter/target/release
### Configuration
Before using the tool, you need to set up the environment variable for the API key:

Windows (PowerShell):
```powershell
$Env:EXCHANGERATE_API_KEY = "<your API key>"
```
Bash: 
```bash
export EXCHANGERATE_API_KEY="<your API key>"
```
### Usage
You can run the converter in interactive mode, where you will be prompted for all information needed for the conversion, or you can provide arguments directly. Alternatively, you can list all conversion rates for specified currency.

**Basic usage:**
```bash
./converter [from-currency] [to-currency] [amount]
```
Example:
```bash
./converter USD EUR 10
```
**Interactive mode:**
```bash 
./converter --interactive
```
**List conversion rates for given currency:**
```bash 
./converter --list <currency-code>
```
