#!/bin/bash
mkdir -p bin
cp target/debug/discord_bot bin/debug_discord_bot
cp target/release/discord_bot bin/discord_bot

sudo setcap cap_net_raw=eip bin/discord_bot
sudo setcap cap_net_raw=eip bin/debug_discord_bot