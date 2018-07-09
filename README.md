# nimtool  
## A simple tool to grab Nimiq metrics from api.nimiqx.com
This tool is still in very early stages of development so it is not feature complete. I've decided to release a small working sample so I can get feedback from the community on which features are actually wanted. Implementing every endpoint from api.nimiqx.com in my spare time will take me months. So, if you'd like to see another currency added or another endpoint included submit a feature request so I can properly prioritize.

This tool is my first effort in learning Rust after reading the book so there's lot's of probably poor practices in the code. By developing this further I hope to develop my skills with Rust so please keep that in mind if examining the code. It's bad, I know.

If you find this tool useful feel free to send me some NIM at "NQ91 GVBG 4EQD 39FA HPTL VPES VDFM FSPG 8QNB"
  
# Usage:
```sh
**Example:** nimtool --price current --currency usd
    **Output:** 2018-07-08 05:25:02: 0.0032 - Percent change 1 hour: 0.00 - Percent change 24 hour: -3.03

    nimtool [OPTIONS]
    FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

    OPTIONS:
    -c, --currency <CURRENCY>    Set the output currency [possible values: usd, eur, cny, btc]
    -p, --price <SCOPE>          Price of Nim and percentage change [possible values: current]
```

# Installation
There are no packages for any systems yet, that's pretty low on my todo list at this stage. Furthermore I have only tested this on x86_64 linux. To use the tool you first need to install the rust toolchain. Fortunately rust makes this real easy with Rustup. This tool is currently using Rust nightly so be sure that's the toolchain you build against.

```sh
curl https://sh.rustup.rs -sSf | sh
```
Once installed clone this repository

```sh
git clone https://github.com/Germanicus1982/nimtool.git
```
and then you can use Cargo to build it

```sh
cargo build --release
```
This will build the output to target/release

# Thanks
### Thanks to Miao at NimiqX.com for such a comprehensive API for me to experiment with.

Copyright 2018 David Wagner

Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
