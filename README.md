<p align="center">
	<img src="https://raw.githubusercontent.com/heroesofcode/inject-sql/main/assets/logo.png" width="530" height="530" alt="Logo">
</p>

<p align="center">
		<a href="https://github.com/heroesofcode/inject-sql/actions/workflows/CI.yml"><img src="https://github.com/heroesofcode/inject-sql/actions/workflows/CI.yml/badge.svg"></a>
		<a href="https://github.com/heroesofcode/inject-sql/actions/workflows/Publish.yml"><img src="https://github.com/heroesofcode/inject-sql/actions/workflows/Publish.yml/badge.svg"></a>
    <a href="https://crates.io/crates/injectsql"><img src="https://img.shields.io/crates/v/injectsql"></a>
		<a href="https://brew.sh/pt-br/"><img src="https://img.shields.io/badge/Homebrew-f1c072.svg?logo=homebrew&logoColor=7c6a50"></a>
    <a href="https://img.shields.io/badge/rustc-1.74.1-blue.svg?logo=rust&logoColor=orange"><img src="https://img.shields.io/badge/rustc-1.74.1-blue.svg?logo=rust&logoColor=orange"></a>
    <a href="https://crates.io/crates/injectsql"><img src="https://img.shields.io/crates/d/injectsql.svg?logo=rust&logoColor=orange"></a>
    <a href="https://github.com/heroesofcode/inject-sql/blob/main/LICENSE"><img src="https://img.shields.io/github/license/heroesofcode/inject-sql.svg"></a>
</p>


🛢️ 🖥️ Command Line Tools to check for SQL Injection vulnerability. This tool is to help pentest in their daily lives quickly.

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

```sh
injectsql
```

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
