# wait-for-internet

So that I don't mash Refresh on a web page while I'm waiting for my computer to have a remote connection.

```
wait-for-internet 0.1.1
Command line utility that waits till you have an internet connection.

USAGE:
    wait-for-internet [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -t, --timeout <timeout>    Exits if a successful connection is not made within timeout seconds.
    -w, --wait-time <wait>     Time to wait between failed requests [default: 0]
```

Exits successfully once it makes a successful request, see [online](https://github.com/jesusprubio/online) for which URLs/fullbacks.

### Install

Install [cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html).

```
git clone https://github.com/seanbreckenridge/wait-for-internet
cd wait-for-internet
cargo build --bins --release  # you can add the '-j <CPU_COUNT>' flag to specify CPU count
# copy to somewhere on your $PATH
cp ./target/wait-for-internet /usr/local/bin/
wait-for-internet  # test the script
```

#### Example Usage

On Mac, one can do something like:

`wait-for-internet && afplay /System/Library/Sounds/Funk.aiff`

... or:

`wait-for-internet && say "You have internet"`

I have an alias setup on my machine using that plays the default 'message' sound and sends a notification:

`alias wfi='wait-for-internet && notify-send "INTERNET" && mpv /usr/share/sounds/freedesktop/stereo/message.oga > /dev/null 2>&1'`
