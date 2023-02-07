# Devzat Forth

[Devzat](https://github.com/quackduck/devzat) is a chatroom imagined for developers. It makes sense to let them code directly on the chatroom. This plugin brings a Forth interpreter to the chatroom.

The Forth interpreter is [ASC miniForth](https://github.com/Arkaeriit/ASCminiForth).

## In-chat usage

For the people in the chat-room, this plugin exposes the command, `forth`. This command execute any Forth code given as argument. The state of the Forth interpreter is kept between calls to the function.

Here is an example of in-chat use:

```
Arkaeriit: forth 1 1 + .
2
Arkaeriit: forth : function 3 0 do ." Hello Devzat!" CR loop ;
Arkaeriit: forth function
Hello Devzat!
Hello Devzat!
Hello Devzat!
```

The plugin can search for errors such as timeouts and stack overflows. In those case, the state will be reset completely or partially.

## Admin usage

The plugin is configured with the following environment variable.

|Variable name |Description                                                   |Default                                                                     |
|--------------|--------------------------------------------------------------|----------------------------------------------------------------------------|
|`PLUGIN_HOST` |URL of the chat-room interface                                |`https://devzat.hackclub.com:5556`                                          |
|`PLUGIN_TOKEN`|Authentication token                                          |Does not defaults to anything. The program panics if the token is not given.|
|`LOGIN_ROOM`  |Name of the room where the bot will tell when it is connected.|`#bots`                                                                     |
|`DEV_NICK`    |Nickname of the user the bot will tell when it is connected   |`Arkaeriit`                                                                 |
|`BOT_NAME`    |Name used by the bot to introduce itself.                     |`Forth`                                                                     |

The plugin can catch stack overflows and timeouts on its own but it cannot catch segmentation faults. This is unfortunate as segmentation faults are very easy to cause in Forth (`0 @`). To minimize that issue, `launch_script.sh` can be run to restart the plugin when needed.

The plugin can execute code that allocate memory or take a lot of CPU time. When hosting it, you should limit the memory and the process priority of the user that runs it.

## Notes to developer

As running arbitrary code that is as low level as Forth in inherently unsafe, no effort have been made to write safe Rust. Thus, you might find in `src/am_forth.rs` some of the most horrendous Rust you ever saw.

## Acknowledgment

Special thanks to [Tommy](https://github.com/TommyPujol06) for the [library](https://github.com/TommyPujol06/devzat-rs) he made the library to make Devzat plugins in Rust.

