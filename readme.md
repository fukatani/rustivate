## rustivate

*Switch rust alternative CLI and traditional CLI in one command.*

Recently, modern rust CLI tools alternative to traditional CLI like `grep`, `ls` are emerging.

For example,
[ripgrep](https://github.com/BurntSushi/ripgrep) is faster than `grep` in many cases,
[exa](https://github.com/ogham/exa) is more features and better defaults than `ls`.

I want to use rust tools, but I unintentionally use traditional command often. 

`rustivate` command, you can set aliases.
```bash
alias grep='ripgrep'
alias ls='exa'
alias du='dust'
alias cat='bat'
alias find='fd'
```
These alias enable us to use alternative CLI without extra our memory.

These tools are not full compatible of traditional CLI, sometimes you need release aliases.

Only by `derustivate` command, you can release all aliases.

And If you want to use rust alternative CLI again, `rustivate` in shell.

### Supported tool

[exa](https://github.com/ogham/exa): Modern replacement for `ls`
[ripgrep](https://github.com/BurntSushi/ripgrep): Fast line-oriented search tool alternative to `grep`
[fd](https://github.com/sharkdp/fd): Simple, fast and user-friendly alternative to `find`
[bat]
[dust](https://github.com/bootandy/dust): Like `du` but more intuitive

### Environment
Currently, `rustivate` supports Linux and bash only.
`cargo` is also needed.

### Install
```
git clone https://github.com/fukatani/rustivate.git`
cd rustivate
cargo run
echo "source ~/.rustivate/aliases.sh" >> ~/.bashrc
```

If you want to use rust alternative as default,
```
echo "rustivate" >> ~/.bashrc
```
(but I don't recommend it.)

If you want to use rustivate immidiately,
`source ~/.bashrc`
is needed.

### Usage
In bash,

`rustivate`: set alternative tools enable.

`derustivate`: set alternative tools disable.

### Uninstall
1. Remove description about rustivate in `~/.bashrc`
2. Remove cloned git repository by `rm -rf rustivate`
