Chat bot that listens to chat in a single irc channel (mainly for twitch chat) and runs the messages through tts.

Currently, it requires you to download and unpack a binary build of marytts from 
<https://github.com/marytts/marytts/releases> and run the server.

This app lacks nice config at the moment, but the marytts server host/port should be made to match in the `say()` 
function.

When connecting to twitch, you'll need to follow the guide here:

<https://help.twitch.tv/customer/portal/articles/1302780-twitch-irc>

Get your oauth token here: <http://www.twitchapps.com/tmi>
