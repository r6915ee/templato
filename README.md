# Templato

Templato is an extremely simple, command line program for using template files.

Templato is great for general-purpose use, due to its speed and the simple
syntax of the actual command line arguments.

## Usage

### Basic

Templato relies on _search paths_ to search for templates. These function like
the paths in the `PATH` environment variable, but allow for searching for
text files instead of executable ones. The default search path is
`~/Templates`, assuming that `~` is the home directory.

A template file is a generic text file that contains the actual template data
to copy. Its contents are copied over to the output file.

Assuming you have a template named `lua/module.lua` in your search paths, you
can simply run the following command:

```sh
templato lua/module.lua
```

This will copy the template over to the current directory. Do note that there
is a scheme for what filename is used for the output file; if a custom filename
is specified using the optional, second argument, then that filename is used.
If this argument isn't specified, then Templato will attempt to use the base
filename of the template, or it will use a default `template` filename if the
filename could not be extracted for whatever reason.

### Advanced

To configure search paths, Templato uses command line flags and a dedicated
`TEMPLATO_SEARCH_PATHS` environment variable. The command line flags,
`--prepend` and `--append`, will prepend and append a search path,
respectively. These flags can be used multiple times.

The `TEMPLATO_SEARCH_PATHS` environment variable, if present, will override the
default home directory search path with the paths present in the environment
variable. The contents of the environment variable follow the same delimiter
rules as the current operating system, where each path is split through a
delimiter; on Windows, this delimiter will be a semicolon (`;`), while other
systems will typically use a colon (`:`). This environment variable can be
safely used with both flags above.

The order of search paths matters, because Templato will use the first template
it finds in the search paths. Prepending in this case can be useful in order to
force Templato to use that search path first.
