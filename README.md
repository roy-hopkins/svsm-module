# Example SVSM loadable module
This project contains a very simple example of a loadable module that can be
added to the coconut-SVSM ramfs initial filesystem. On startup, coconut-SVSM
will enumerate modules and call the entry point at CPL-3. See the coconut-SVSM
project here: https://github.com/coconut-svsm/svsm

This project has been created to support the PR titled "Draft: Add support for
loadable modules at CPL-3" as an example module.

## Building
Build the example module with:

```bash
cargo build
```

## Packaging for coconut-SVSM
The module is built into the file `target/debug/svsm-module`. This needs to be
added to the filesystem image that is included in your SVSM build.

To build the image you need to use the coconut `packit` tool. Clone and build it from here:
https://github.com/coconut-svsm/packit

Once you have `packit` installed, create the directory structure expected by the
loadable module support on coconut-SVSM:

```bash
mkdir -p image/modules/
cp target/debug/svsm-module image/modules
strip image/modules/svsm-module
packit --output fs.bin --dir image/
```

Once you have built the image you can bundle it in the coconut-SVSM initial
ramfs image:

```
cd coco-svsm/
FS_FILE = /path/to/fs.bin make
```


