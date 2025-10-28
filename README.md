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

```bash
# Clone the repository
git clone https://github.com/neoarz/Longshot.git
cd Longshot

# Install dependencies
cargo build --release

# Run the bot
./target/release/longshot
```


## Configuration:

```json
{
  "main_token": "YOUR_TOKEN_HERE", // Your main Discord token
  "snipe_on_main_token": true,
  "sub_tokens": ["YOUR_TOKEN_HERE", "YOUR_TOKEN_HERE"], // Your sub tokens
  "webhook": "YOUR_WEBHOOK_HERE", // To send notifications 
  "guild_blacklist": [1234567890, 1234567890] // Servers to ignore
}
```
