# wait-for-internet

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
