<div align="center">
  <picture>
    <img alt="Longshot Discord Nitro Sniper Banner" src="assets/banner.png" 
  </picture>
</div>


<h3 align="center">A high-performance Discord Nitro Sniper written in Rust </h3>

> [!CAUTION]
> **Breaching Discord's TOS**
> 
> Using your discord account token to snipe nitro is against Discord's TOS. Use at your own risk. We (longshot) are not responsible for any bans or other consequences, that may arise from using this tool. (e.g. your account getting banned, etc.)


## Installation:

> [!WARNING]
> **Windows Users**
> 
> Due to me using a mac, ~~windows is ass~~ I don't want to test the bot on Windows. If you are a Windows user, please test and lmk if it works. I can guarantee that the bot will work on Linux and macOS. It also works on [WSL](https://learn.microsoft.com/en-us/windows/wsl/install) from my testing.

### Installing Rust

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source "$HOME/.cargo/env"

# Verify installation
rustc --version
cargo --version
```
### Build the bot

```bash
# Clone the repository
git clone https://github.com/neoarz/Longshot.git
cd Longshot

# Build the program
cargo build --release

# Run the program
./target/release/longshot
```


## Configuration:

#### After running the program for the first time, it will create a `config.json` file in the root directory.

```json
{
  "main_token": "YOUR_TOKEN_HERE", // Your main Discord token
  "snipe_on_main_token": true,
  "sub_tokens": ["YOUR_TOKEN_HERE", "YOUR_TOKEN_HERE"], // Your sub tokens
  "webhook": "YOUR_WEBHOOK_HERE", // To send notifications 
  "guild_blacklist": [1234567890, 1234567890] // Servers to ignore
}
```


## Support:

If you need help, feel free to make an issue on the repository. Alternatively, you can DM [neoarz](https://discord.com/users/1015372540937502851) on Discord.