# Nimtool
## A simple tool to grab Nimiq metrics from [api.nimiqx.com](https://api.nimiqx.com "NimiqX API")
[![Build Status](https://travis-ci.org/Germanicus1982/nimtool.svg?branch=master)](https://travis-ci.org/Germanicus1982/nimtool)

This tool is still in very early stages of development so it is not feature complete. I've decided to release a small working sample so I can get feedback from the community on which features are actually wanted. Implementing every endpoint from api.nimiqx.com in my spare time will take me months. So, if you'd like to see another currency added or another endpoint included submit a feature request so I can properly prioritize.

This tool is my first effort in learning Rust after reading the book so there's lot's of probably poor practices in the code. By developing this further I hope to develop my skills with Rust so please keep that in mind if examining the code. It's bad, I know.

If you find this tool useful feel free to send me some NIM at "NQ91 GVBG 4EQD 39FA HPTL VPES VDFM FSPG 8QNB"
  
# Usage:
```sh
Example: nimtool --price current --currency jpy
Output: GMT 2018-07-13 23:55:02: ¥0.3082 - 1h: %-2.25 - 24h: %14.29


    nimtool [FLAGS] [OPTIONS]

    FLAGS:
        -h, --help             Prints help information
        -n, --network-stats    Hashrate, last found block, height, difficulty, last reward and Nim per day per kH
        -s, --supply           Current existing supply of Nim
        -V, --version          Prints version information

    OPTIONS:
        -a, --addressbook <ADDRESS>     Find a name by address. Must use quotes e.g. "NQ91 GV..."
        -b, --blockinfo <IDENTIFIER>    Get block information by block number
        -c, --currency <CURRENCY>       Set the output currency [possible values: usd, eur, aud, brl, cad, cny, gbp, nzd, dkk, jpy, pln, krw, rub, mxn, sek, hkd, myr, sgd, chf,
                                        huf, nok, thb, clp, idr, try, ils, php, twd, czk, inr, pkr, zar, btc]
        -p, --price <SCOPE>             Price of Nim and percentage change [possible values: current, day, week]
        -t, --transaction <HASH>        Get information from a transaction hash
```

# Installation
There are no packages for any systems yet, that's pretty low on my todo list at this stage. Furthermore I have only tested this on x86_64 linux however, it builds alright on Travis CI on Mac as well. I just don't have one to test it properly. To use the tool you first need to install the rust toolchain. Fortunately rust makes this real easy with Rustup. This tool is currently using Rust nightly so be sure you have it installed.

```sh
curl https://sh.rustup.rs -sSf | sh
```
Once installed clone this repository

```sh
git clone https://github.com/Germanicus1982/nimtool.git
```
and then you can use Cargo to build it

```sh
cd nimtool
cargo build --release
```
This will output the binary to target/release/nimtool

# Thanks
### Thanks to [Miao](https://api.nimiqx.com/docs/about "Miao") at [NimiqX](https://api.nimiqx.com "NimiqX.com") for such a comprehensive API for me to experiment with.

# License
[MIT](LICENSE "MIT License")
