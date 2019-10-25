# chunk-json-lite
A little tool to split a json into multiple valid json array of a given max size

## Example usage

```bash
$ cargo install chunk-json-lite
$ cat misc/movies-10.json | chunk-json-lite '10 MB'
$ cat chunk.0.json
```

You can also combine it with `csv2json-lite` and transform a csv into chunked json:

```bash
$ cargo install csv2json-lite chunk-json-lite
$ cat misc/movies-10.csv | csv2json-lite | chunk-json-lite '10 MB'
$ cat chunk.0.json
```
