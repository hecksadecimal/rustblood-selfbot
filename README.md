## Automatic troll quirking for Discord RPers

### Usage

WARNING: Self-bots are against discord's TOS. If you decide to run this you are doing so at your own risk, and understand that there's a chance your account will be banned.

To use this program you must first obtain your discord token. This is a sensitive piece of information that you should not share with anybody, or they will be able to use it to use your account regardless of if you have 2FA enabled or not.

These instructions are applicable to Firefox, but should be mostly the same between many modern browsers.

1. Go to [Discord](https://discord.com/channels/@me) and open the dev tools with F12. Select the 'Network' tab.
2. Reload the page.
3. In the 'Filter URLs' section type science.
4. Select any of the available entries.
5. To the right, click on the 'Headers' tab.
6. In the 'Request Headers' section, copy the contents of 'Authorization'. Everything after 'Authorization:' and before 'Connection' is your token.
7. See below for setting an environment variable.

When the program is started and it has successfully logged in it will listen to messages that you post and search at the beginning of each line of that message for any string of characters followed by a Colon and a space. If there's a matching file within the quirks folder, and the file itself is valid, it will apply those quirks to that line. 

See the included quirks folder for example usage. The program does not need to be restarted when adding new quirk files, or editing or deleting exisitng ones.

#### Setting Environment Variable (Windows)

Follow [these instructions](https://docs.oracle.com/en/database/oracle/machine-learning/oml4r/1.5.1/oread/creating-and-modifying-environment-variables-on-windows.html) to create a User Environment Variable. 

The Variable should be DISCORD_TOKEN, and the Value should be the key you've obtained earlier.

#### Setting Environment Variable (Mac & Linux)

Follow [these instructions](https://phoenixnap.com/kb/set-environment-variable-mac) to create an environment variable.

It should be DISCORD_TOKEN= followed by the token obtained earlier.





