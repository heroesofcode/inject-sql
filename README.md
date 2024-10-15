# inject-sql

[![CI](https://github.com/heroesofcode/inject-sql/actions/workflows/CI.yml/badge.svg)](https://github.com/heroesofcode/inject-sql/actions/workflows/CI.yml)
[![Crates.io](https://img.shields.io/crates/v/injectsql)](https://crates.io/crates/injectsql)
[![Rustc](https://img.shields.io/badge/rustc-1.74.1-blue.svg?logo=rust&logoColor=orange)](https://img.shields.io/badge/rustc-1.70.0-blue.svg?logo=rust&logoColor=orange)
[![Downloads](https://img.shields.io/crates/d/injectsql.svg?logo=rust&logoColor=orange)](https://crates.io/crates/injectsql)
[![License](https://img.shields.io/github/license/heroesofcode/inject-sql.svg)](https://github.com/heroesofcode/inject-sql/blob/main/LICENSE)

🛢️🖥️ Command Line Tools to check for SQL Injection vulnerability. This tool is to help pentest in their daily lives quickly.

## Installing

### Cargo
Installing from [crates.io](https://crates.io/) (requires Rust/Cargo):

```shell
cargo install injectsql
```

### Homebrew
You can install with [Homebrew](https://brew.sh/):

```shell
brew tap heroesofcode/taps
brew install heroesofcode/taps/injectsql
```

## Usage

<img src="https://github.com/heroesofcode/inject-sql/blob/main/assets/example.png?raw=true">

If you want to test or are studying pentest I recommend you use these tools below:

- [Acunetix](http://testphp.vulnweb.com/) 🇬🇧
- [HackTheBox](https://www.hackthebox.com/) 🇬🇧
- [Solyd](https://solyd.com.br/) 🇧🇷

> [!IMPORTANT] 
> injeqtor does not perform an in-depth SQL injection attack, it only checks whether SQL Injection is vulnerable and tries to find the name of the database. If you want to carry out an attack to obtain more information, I recommend using [sqlmap](https://github.com/sqlmapproject/sqlmap).

> [!WARNING]
> The use of the `injectsql` tool for offensive activities without express permission from the parties involved is illegal. The user is solely responsible for ensuring compliance with applicable local, state and federal laws. The creators of this software are not responsible for any misuse or damages resulting from the use of this program.

## Contributing

To contribute, just fork this project and then open a pull request, feel free to contribute, bring ideas and raise any problem in the issue tab.

## License

injeqtor is released under the MIT license. See [LICENSE](https://github.com/heroesofcode/injector/blob/main/LICENSE) for details.
