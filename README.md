## NoMake
NoMake is a simple C project manage tool for people who don't like Makefile.
It uses a group of default rules to generate binary targets, either binary, shared lib or static lib.

Not tend to be fast or fully functional.

usage:
```bash
cd project-root
nomake -o project-name
```

The program will create bin folder to place object files.   
though it won't reuse them in current version:(

parameters:
```bash
-l <lib>           compile with library rules, value could be "static" or "shared"
-o <output>        specify output file
```
TODO: Add debug/release mode choice

TODO: Add ignore file function(something like .gitignore)