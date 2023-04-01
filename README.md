Haptic Feedback for Factorio
============================

How to use
----------

- Take the files in the `factorio` folder and put them inside a folder called `haptics` in your `%APPDATA%/Factorio/mods`.
- While you're there, make sure that the folder `%APPDATA%/Factorio/script-output` exists. Create it if it doesn't.
- Launch the game with the mod enabled and start a new game, or load a saved one.
- Make sure Intiface Central is running, including the server.
- Launch factorio-haptics.exe
- You should now receive Haptic Feedback when you're researching, and the strength depends on your science pack consumption.
- factorio-haptics.exe will automatically close when the connection to Intiface Central is lost.

Caveats
-------

The mod will only really work with Vanilla and Vanilla-like Factorio. It is programmed to consider all science packs, which will not work for something like Space Exploration, where there are a lot of packs, and not all of them are used at the same time.

There are no settings, except for what's in the mod script file itself. There, you may change the IGNORE list for packs to ignore, and the TARGET number for what amount of Science Per Minute should be 100% haptics.

It will only work on Windows because APPDATA is hardcoded in the Rust code.

It will Feedback all motors on all connected devices.
