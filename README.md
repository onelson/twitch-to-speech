## Overview

Chat bot that listens to chat in a single irc channel (mainly for twitch chat) and runs the messages through tts.

This app requires you to install flite headers, gcc toolchain stuffs, etc since we link against them for the tts feature.

When connecting to twitch, you'll need to follow the guide here:

<https://help.twitch.tv/customer/portal/articles/1302780-twitch-irc>

Get your oauth token here: <http://www.twitchapps.com/tmi>

## Config

The binary looks for a single argument (a path to a json config file). This file contains config data for the irc connection, as per the irc crate (see their docs for details).

Here's a quick example, however:

```json
{
  "nickname": "omn_bot123",
  "alt_nicks": ["omn_bot456", "omn_bot789"],
  "use_ssl": false,
  "port": 6667,
  "server": "irc.freenode.net",
  "channels": ["#bot_test_channel"],
  "password": "something secret"
}

```
