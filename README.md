# sorrygle-rs

This is a [Sorrygle](https://github.com/JJoriping/Sorrygle.git) compiler written in Rust

## How to use (Not yet implemented)

You can run command to simple text to play:
```sh
sorrygle-rs "(q=8)cdefgfedc~~~"
```

### Options (Not yet implemented)

If you want to convert script to midi, you must put the `-c`(or `--convert`) option.
```sh
sorrygle-rs -c script.srg output.mid
```

and put `--strict` option, the script will be checked more strict
```sh
sorrygle-rs -c script.srg output.mid --strict
```

When you use extended commands, you must to insert `-e` (or `--extend`)
```sh
sorrygle-rs -c script.srg output.mid -e
```


## Todo

See this [todo](docs/todo.md) page
