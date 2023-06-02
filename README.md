# rename64

A simple CLI tool to rename files and directories. For personal use.


## Usage

A simple example:

```bash
rename64 "./*.txt" -r "file"
```

This will rename all files in the current directory with the extension `.txt` to `file-1.txt`, `file-2.txt`, etc.

Not passing the flag `-r` will keep the original file name and append the index to it.
