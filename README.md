# 工作空间生成工具 nvim_workspace_initializer

---

## About

! 该程序已被我弃用。目前使用[这个工具](https://github.com/lI15SO0/WorkspaceIniter)替代 !

这个程序是为了方便生成 neovim 的工作空间配置文件而制作的。其配合[这个配置文件](https://github.com/lI15SO0/nvim-config)来使用。

For init a neovim workspace, I write this program. It can create workspace dirs and exrc files.

This program was use with [ this neovim config ](https://github.com/lI15SO0/nvim-config).

## Usage:

编译安装，然后就能用 ```wsinit``` 来使用了。其 PATH 的默认值为 "./" 

Compile it and install it. Then you can use ```wsinit``` command to use it.

```
Init nvim workspace dir.

Usage: wsinit [OPTIONS] [PATH]

Arguments:
  [PATH]  

Options:
  -f, --force    
  -h, --help     Print help
  -V, --version  Print version
```

if PATH arg not provided. this program will use "./" as PATH.
