# Nimtool
## A simple tool to grab Nimiq metrics from [api.nimiqx.com](https://api.nimiqx.com "NimiqX API")
[![Build Status](https://travis-ci.org/Germanicus1982/nimtool.svg?branch=dev)](https://travis-ci.org/Germanicus1982/nimtool)


This tool is my first effort in learning Rust after reading the book so there's lot's of probably poor practices in the code. By developing this further I hope to develop my skills with Rust so please keep that in mind if examining the code. It's bad, I know.

If you find this tool useful feel free to send me some NIM at "NQ63 RHCJ XVXE 3TX5 V318 SGUD 75NQ SP4G E9YQ"
  
# Usage:
```sh
Example: nimtool --price current --currency btc
Output: GMT 2018-07-18 22:20:03: ₿0.00000044 - 1h: %-10.20 - 24h: %12.82

    nimtool [FLAGS] [OPTIONS]

    FLAGS:
            --help             Prints help information
        -n, --network-stats    Hashrate, last found block, height, difficulty, last reward and Nim per day per kH
        -s, --supply           Current existing supply of Nim
        -V, --version          Prints version information

    OPTIONS:
        -a, --addressbook <ADDRESS>     Find a name by address. Must use quotes e.g. "NQ91 GV..."
            --apikey <API_KEY>          Set the application key
        -b, --blockinfo <IDENTIFIER>    Get block information by block number
        -c, --currency <CURRENCY>       Set the output currency [possible values: usd, eur, aud, brl, cad, cny, gbp, nzd, dkk, jpy, pln, krw, rub, mxn, sek, hkd, myr, sgd, chf,
                                        huf, nok, thb, clp, idr, try, ils, php, twd, czk, inr, pkr, zar, btc]
        -h, --hashrate <SCOPE>           [possible values: current, hour, day, week, month, year]
        -l, --label <LABEL>             Find an address by name
        -p, --price <SCOPE>             Price of Nim and percentage change [possible values: current, day, week, month, year]
        -t, --transaction <HASH>        Get information from a transaction hash

```

# Installation
To download the latest binaries for Linux and Windows visit https://nimtool.com/downloads

I have only tested this on x86_64 Linux and Windows 10 however, it builds alright on Travis CI on Mac as well. To build this tool from source you first need to install the rust toolchain. Fortunately rust makes this real easy with [Rustup](https://rustup.rs). You will also need a free api key from [NimiqX](https://api.nimiqx.com/docs/about "NimiqX API Key"). You must set the --apikey first or all other commands will fail.

Once Rust is installed clone this repository

```sh
git clone https://github.com/Germanicus1982/nimtool.git
```
and then you can use Cargo to build it

```sh
cd nimtool
cargo build --release
```
This will output the binary to target/release/nimtool. From there you can copy it to a folder in your path or just call it from that directory.

# Thanks
### Thanks to [Miao](https://api.nimiqx.com/docs/about "Miao") at [NimiqX](https://api.nimiqx.com "NimiqX.com") for such a comprehensive API for me to experiment with.

# License
[MIT](LICENSE "MIT License")
