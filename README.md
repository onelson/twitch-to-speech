Chat bot that listens to chat in a single irc channel (mainly for twitch chat) and runs the messages through tts.

Currently, it requires you to download and unpack a binary build of marytts from 
<https://github.com/marytts/marytts/releases> and run the server.

This app lacks nice config at the moment, but the marytts server host/port should be made to match in the `say()` 
function.