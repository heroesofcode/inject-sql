# injeqtor

[![CI](https://github.com/heroesofcode/injeqtor/actions/workflows/CI.yml/badge.svg)](https://github.com/heroesofcode/injeqtor/actions/workflows/CI.yml)
[![Crates.io](https://img.shields.io/crates/v/injeqtor)](https://crates.io/crates/injeqtor)
![Msrv](https://img.shields.io/badge/msrv-1.56.1-blue.svg?logo=rust&logoColor=orange)
[![Downloads](https://img.shields.io/crates/d/injeqtor.svg?logo=rust&logoColor=orange)](https://crates.io/crates/injeqtor)
[![License](https://img.shields.io/github/license/heroesofcode/injeqtor.svg)](https://github.com/heroesofcode/injeqtor/blob/main/LICENSE)

Command Line Tools to check for SQL Injection vulnerability. This tool is to help pentest in their daily lives quickly.

## Installing

### Cargo
Installing from [crates.io](https://crates.io/) (requires Rust/Cargo):

```shell
cargo install injeqtor
```

### Homebrew
You can install with [Homebrew](https://brew.sh/):

```shell
brew tap heroesofcode/taps
brew install heroesofcode/taps/injeqtor
```

## How to use

#### How to test?

If you want to test or are studying pentest I recommend you use these tools below:

- [Acunetix](http://testphp.vulnweb.com/) 🇬🇧
- [HackTheBox](https://www.hackthebox.com/) 🇬🇧
- [Solyd](https://solyd.com.br/ead/) 🇧🇷

> [!IMPORTANT] 
> injeqtor does not perform an in-depth SQL injection attack, it only checks whether SQL Injection is vulnerable and tries to find the name of the database. If you want to carry out an attack to obtain more information, I recommend using [sqlmap](https://github.com/sqlmapproject/sqlmap).

> [!WARNING]
> Usage of injeqtor for attacking targets without prior mutual consent is illegal. It is the end user's responsibility to obey all applicable local, state and federal laws. Developers assume no liability and are not responsible for any misuse or damage caused by this program.

## Contributing

To contribute, just fork this project and then open a pull request, feel free to contribute, bring ideas and raise any problem in the issue tab.

## License

injeqtor is released under the MIT license. See [LICENSE](https://github.com/heroesofcode/xrun/blob/main/LICENSE) for details.