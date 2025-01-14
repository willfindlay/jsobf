# jsobf - The JSON Obfuscator

`jsobf` is an obfuscator for JSON documents that completely randomizes keys and
values, but preserves **the original structure** of the JSON document and the
**original mapping** of keys and values.

Why build `jsobf`? I wanted to obfuscate some sensitive JSON data but there was
no simple CLI tool for it that I could trust.

Need a simple example? Let's obfuscate the following:

```json
{
    "foo": "bar",
    "baz": {
        "test1": ["a", "b", "long string"],
        "test2" : false,
        "test3": 12
    },
    "qux": 13.37
}
```

becomes 

```json
{
  "0lH": {
    "84y8R": 7316513358385113447,
    "a9MEx": false,
    "jcML3": ["k", "n", "gIT6GsAhKEj"]
  },
  "0zv": 0.8398630941676407,
  "9Rf": "7qK"
}
```

## Installation

`cargo install jsobf`

## Usage

`echo '{"foo": ["a", "b", "c"]}' | jsobf -`

or

`jsobf file.json`