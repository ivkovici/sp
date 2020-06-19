## sp (**S**ingle **P**ath)

This cli was made because at work we use some shared drives. The OS usage in the company is split
between Windows and Linux in favor of Windows. Because most people use Windows we share links to
files like this:
```
 t:\igor\path\sccpre.cat-kim-jung-un-png-913514.png
 ```
If you are using linux opening these paths and linking them is a headache because we have routes like this:
```
/mnt/temp/igor/path/sccpre.cat-kim-jung-un-png-913514.png
```

This cli opens files from Windows routes and translates Linux routes to Windows routes.

## Dependencies

This cli was made using the Rust programming language, so you will need Rust to use it.

[INSTALL RUST](https://rustup.rs)

## Installation
```sh
$ cargo install sp
```

## Usage
You can check the usage with these commands:
```sh
$ sp -h
$ sp --help
```

First you need to set some replace pairs to translate Windows to Linux.
```sh
$ sp -f 'p:\' -r '/mnt/public/'
$ sp --find 't:\' --replace '/mnt/temp/'
```

You can list the replace pairs with these commands:
```sh
$ sp -l
$ sp --list
```

To clear the replace pairs you run one of these:
```sh
$ sp -e
$ sp --empty
```

For opening files we use these commands:
```sh
$ sp -o 't:\igor\path\sccpre.cat-kim-jung-un-png-913514.png'
$ sp --open 't:\igor\path\sccpre.cat-kim-jung-un-png-913514.png'
```

To translate a path to Windows and copy it to the clipboard you can do this:
```sh
$ sp -t /mnt/temp/igor/path/sccpre.cat-kim-jung-un-png-913514.png
$ sp --translate /mnt/temp/igor/path/sccpre.cat-kim-jung-un-png-913514.png
```
