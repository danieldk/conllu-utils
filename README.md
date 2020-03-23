# CoNLL-U Utilities

## Introduction

This is a set of utilities to process files in the CoNLL-U format. The
`conllu` command provides the following subcommands:

* `cleanup`: normalize unicode and replace unicode punctuation
* `from-text`: convert tokenized text files to CoNLL-U.
* `merge`: merge CoNLL-U files
* `partition`: partition a CoNLL-U file in N files.
* `shuffle`: shuffle the sentences in a CoNLL-U file.
* `to-text`: convert CoNLL-U to tokenized plain text.

## Usage

Executing a subcommand gives usage information when `--help` is given
as an argument.
