# beepr
beepr is a very simple program to play a single notification sound when run. This
project was just a way for me to learn how to package rust applications in linux.

Uses the [https://crates.io/crates/ears](ears) package to play sound. (It's also a copy of the example code).

Notification sound is taken from [https://www.freesound.org/people/morrisjm/sounds/268756/](freesound user morrisjm).

# usage
```
beepr
```

What I use it for is long running scripts to know when they've terminated.
```
download_all_of_the_internetz && beepr
```


# install
Arch Linux/Pacman users can download the `arch_pkg/PKGBUILD` file and install it
using `makepkg` and then `pacman -U <pkgname>.tar.xz`.

Other linux users can run `make && make install` which will install the files
into `/usr/bin` and `/usr/share/beepr`.
