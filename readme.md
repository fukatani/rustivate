## rustivate

*Switch rust alternative CLI and traditional CLI in one command.*

Recently, modern rust CLI tools alternative to traditional CLI like `grep`, `ls` are emerging.

For example,
[ripgrep](https://github.com/BurntSushi/ripgrep) is faster than `grep` in many cases,
[exa](https://github.com/ogham/exa) is more features and better defaults than `ls`.

I want to use rust tools, but I unintentionally use traditional command often. 

`rustivate` in bash, you can set aliases.
```bash
alias grep='ripgrep'
alias ls='exa'
alias du='dust'
alias cat='bat'
alias find='fd'
```
These alias enable us to use alternative CLI without extra our memory.

Sometimes you need traditional tools, you can release all aliases by `derustivate`.

And If you want to use rust alternative CLI again, `rustivate` in shell.

### Supported tool

- [exa](https://github.com/ogham/exa): Modern replacement for `ls`
- [ripgrep](https://github.com/BurntSushi/ripgrep): Fast line-oriented search tool alternative to `grep`
- [fd](https://github.com/sharkdp/fd): Simple, fast and user-friendly alternative to `find`
- [bat](https://github.com/sharkdp/bat): `cat` clone with syntax highlighting and Git integration.
- [dust](https://github.com/bootandy/dust): Like `du` but more intuitive

### Environment
Currently, `rustivate` supports Linux and bash only.

[cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html) is needed.

### Install
```
git clone https://github.com/fukatani/rustivate.git
cd rustivate
cargo run  # Install rust CLIs
echo "source ~/.rustivate/aliases.sh" >> ~/.bashrc
source ~/.bashrc
```

If you want to use rust alternative as default,
```
echo "rustivate" >> ~/.bashrc
```
(but I don't recommend it.)

### Usage
In bash,

`rustivate`: set alternative tools enable.

`derustivate`: set alternative tools disable.

#### configure which tool to use

Edit `~/.rustivate/user_settings.json` please.

This is example for to use `ripgrep` with `grep` command.
```json
[ 
    {"name": "grep", "value": "ripgrep"}
]
```
Default settings is [here](https://github.com/fukatani/rustivate/blob/main/resources/default.json).

All alternative CLT listed in above is enabled in default.

### Uninstall
1. Remove description about rustivate in `~/.bashrc`
2. Remove cloned git repository by `rm -rf rustivate`
