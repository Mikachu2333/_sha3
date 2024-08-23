## What's this
This program is used to calc value(s) of SHA3_256 for given file path(s).

## USAGE
```sh
_sha3 <FILE(s)> <DIR(s)>

_sha3 .gitignore
_sha3 D:\\desktop\\
_sha3 "C:\Program Files\"
_sha3 ./main.pdf
_sha3 .\dir\

_sha3 # SHOW HELP INFO
```

STYLE: `<{FILENAME}> SHA3_256={VALUE}`

## Known Bugs
1. file with no content would be ignore
2. some extremely special filename would make it crash, e.g. `"` (This is a file name)