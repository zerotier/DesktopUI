# gir-files

This repository contains the [GIR](https://developer.gnome.org/programming-guidelines/stable/introspection.html.en)
files used to generate all [`gtk-rs`](https://github.com/gtk-rs/gtk-rs) crates.

## Updating all files

You can update all the files by doing:

```bash
./dl.sh
```

This command will fetch the gir files for the latest release of each library.
When updating all files, make sure you do not inadvertedly overwrite gir files
which were manually updated and which have not yet been included in a release
of the corresponding library.

## Updating a gir file manually

In general we prefer to use gir files that have been included in a release,
however sometimes it is required to update a specific gir files to incorporate
a bug fix or a missing API.

Manually copy the updated gir file and then run

```bash
./reformat.sh
./fix.sh
```

## Validating an update

After updating the gir files, please don't forget to check that [`gir`](https://github.com/gtk-rs/gir)
can still run with the new files and that the generated files have no breaking changes.

Refer to the [`gtk-rs`](https://github.com/gtk-rs/gtk-rs) README file for further information about
regenerating the `gtk-rs` crates.
