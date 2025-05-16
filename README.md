# Dynamic Cloudflare DNS Updater

A Rust tool that automatically updates Cloudflare DNS A records when your public IP address changes. Perfect for home servers with dynamic IP addresses.

## Features

- Automatically detects your current public IP address
- Updates multiple Cloudflare DNS records simultaneously
- Configurable via environment variables or .env file
- Detailed logging of all operations

## Installation

1. Clone this repository:

   ```
   git clone https://github.com/yourusername/dyncf.git
   cd dyncf
   ```

2. Create a `.env` file from the sample:

   ```
   cp .env_sample .env
   ```

3. Edit the `.env` file with your Cloudflare credentials and domain information

4. Build the project:
   ```
   cargo build --release
   ```

## Configuration

Copy `.env_sample` to `.env` and configure the following variables:

- `CF_API_TOKEN`: Your Cloudflare API token
- `ZONE_ID`: Your Cloudflare zone ID
- `DOMAIN`: Your domain name (e.g., example.com)
- `RECORDS_NAMES`: Comma-separated list of subdomains to update (e.g., "www, \*, api")

## Getting Your Cloudflare API Token

1. Log in to your Cloudflare account
2. Go to "My Profile" > "API Tokens"
3. Click "Create Token"
4. Select "Create Custom Token"
5. Provide a name (e.g., "DNS Updater")
6. Under "Permissions", add:
   - Zone - DNS - Edit
   - Zone - Zone - Read
7. Under "Zone Resources", select:
   - Include - Specific zone - Your domain
8. Click "Continue to summary", then "Create Token"
9. Copy the generated token to your `.env` file as `CF_API_TOKEN`

## Finding Your Zone ID

1. Log in to your Cloudflare account
2. Select your domain
3. On the overview page, look for "Zone ID" in the right sidebar
4. Copy this ID to your `.env` file as `ZONE_ID`

## Usage

Run the application to update your DNS records:

```
./target/release/dyncf
```

For automated updates, you can set up a cron job:

```
crontab -e
```

Then add a line like:

```
*/15 * * * * cd /path/to/dyncf && ./target/release/dyncf >> /var/log/dyncf.log 2>&1
```

This will run the updater every 15 minutes.

## License

MIT
