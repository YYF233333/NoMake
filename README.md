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
    --release            Activate release mode
    --lib <lib>          Compile with library rules, value could be "static" or "shared" [default: binary]
-o, --output <output>    Specify output file [default: a.out]
```

TODO: Add ignore file function(something like .gitignore)