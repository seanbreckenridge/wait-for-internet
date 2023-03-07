# wait-for-internet

So that I don't mash Refresh on a web page while I'm waiting for my laptop to have a remote connection.

```
Command line utility that waits till you have an internet connection.

Usage: wait-for-internet [OPTIONS]

Options:
  -t, --timeout <TIMEOUT>      Exits if a successful connection is not made within <timeout> seconds
  -q, --quiet                  Don't print any warning/log messages
  -w, --wait-time <WAIT_TIME>  Time to wait between failed requests [default: 1]
      --text <TEXT>            Text to display while waiting [default: ]
  -h, --help                   Print help (see more with '--help')
  -V, --version                Print version
```

Exits successfully once it makes a successful request, see the [online](https://github.com/jesusprubio/online) crate for which URLs/fullbacks it uses to check for a remote connection.

## Install

Install [cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html), then:

```
cargo install wait-for-internet
```

## Example Usage

In bash/shell scripts, I litter `wait-for-internet` wherever I'm paranoid about not having an internet connection. If I have some [script that runs in the background](https://github.com/seanbreckenridge/bgproc) that needs an internet connection (e.g. to `ssh` into my server and run some task), I run `wait-for-internet` and the top, just to be safe.

On the command line:

On Mac, one can do something like:

`wait-for-internet && afplay /System/Library/Sounds/Funk.aiff`

... or:

`wait-for-internet --text "(waiting for internet)" && say "You have internet"`

I have an alias setup on my machine that plays the default 'message' sound and sends a notification:

`alias wfi='wait-for-internet && notify-send "INTERNET" && mpv /usr/share/sounds/freedesktop/stereo/message.oga > /dev/null 2>&1'`

Can also be used in situations to verify you have internet before calling some other command:

`wait-for-internet && xdotool search --class "Firefox" key --window %@ "ctrl+r"` (refresh firefox after you have internet)

`wait-for-internet && sudo apt update && sudo apt upgrade`

`wait-for-internet && speedtest-cli`

`wait-for-internet && ssh <somewhere>`

`wait-for-internet && fg` (resume some suspended task that requires internet)
