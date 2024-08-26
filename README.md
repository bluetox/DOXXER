# Doxxer

Doxxer is a Rust-based command-line tool that gathers information about a given IP address using an external API and performs a scan on known ports. This project is intended for educational purposes and network diagnostics.

## Features

- Retrieve detailed information about a given IP address from an external API.
- Perform a scan on known open ports.
- Simple and fast, with a focus on privacy and security.

### Prerequisites

- Rust (Ensure Rust is installed on your system)
- A terminal/command prompt

## Usage

After building the tool, you can run it from the command line.

## Configuration

You may need to configure the API endpoint for retrieving IP data. This can be done by modifying the source code or setting environment variables (if implemented).

## Supported APIs

Doxxer uses external APIs to retrieve information about IP addresses. Depending on the service, you may need an API key. Examples of supported APIs include:

- ip-api.com

## Port Scanning

After retrieving information about the IP address, Doxxer performs a scan on common ports to detect which ones are open. The tool uses Rust's networking libraries to check for open ports efficiently.

