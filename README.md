# DeepL CLI

## Install

```bash
$ git clone https://github.com/ksk001100/deepl-cli
$ cd deepl-cli
$ cargo install --path .
```
or

Download [here](https://github.com/ksk001100/deepl-cli/releases)

## Usage

```bash
$ export DEEPL_KEY=xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx
$ deepl --help # => Show help
$ deepl こんにちは世界 # => Default target language (EN)
$ deepl Hello world --lang JA
```