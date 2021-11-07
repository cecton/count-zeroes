count-zeroes
============

Count zeroes on a disk or a file.

Installation & Usage
--------------------

```
cargo install count-zeroes
count-zeroes /dev/sdXN
```

Purpose
-------

This is useful if you want to check if an SSD has been trimmed or a disk has
been wiped-clean. I found no obvious way to do that and it's useful when you
try to recover a filesystem to avoid chasing ghosts. In my case, an attempt to
recover a BTRFS partition showed me there was a superblock somewhere but it was
just some random unrelated data.

This program can be used as a library, it has no dependency. It is optimized
for speed.
