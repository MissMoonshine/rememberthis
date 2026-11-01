# rememberthis (rt)
-------------------
rememberthis is a command line utility to aid in memorizing snippets of information. Use it to take short notes that you want to commit to memory.

## Installation
The binary name for rememberthis is `rt`.

```
$ cargo install rememberthis
```

## Usage
rememberthis has three modes of operation:

* create new notes
* edit notes
* list (print) notes

### Create
`rt` appends notes you write to stdin. Entries can be separated with a line containing `====!`. When you are done writing notes write a line containing `====#` to exit.

### Edit
`rt edit` opens the notes file in an editor.

### List
`rt list` writes notes to stdout.
