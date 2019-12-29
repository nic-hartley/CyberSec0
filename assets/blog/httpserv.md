> title: mesher Overview
> author: nic
> tags: mesher,intro
> publish: 2019-10-29
---

httpserv is a [published] HTTP fileserver. It's meant to be a tiny,
easy-to-install, easy-to-use fileserver for local development of static sites.
In that regard, it succeeds wonderfully:

-   Installation is simple. All you need is [Rustup], which is easy to install,
    and then it takes one command and under a minute to install httpserv.
-   Usage is equally easy. To serve the current directory, all you need is
    `httpserv` (or `httpserv.exe`, on WSL) and you're serving those files over
    localhost. If you want to serve another directory, `httpserv /path/to/dir`
    will do it.
-   Because of its simplicity, httpserv starts up almost instantly, and serves
    requests within a millisecond or two. That makes it extremely responsive,
    even when you need to serve dozens of files.

httpserv is already fairly mature. It serves files as intended, and as quickly
as I want it to. It only has a couple more bugfixes before the 1.0 release.
Once that happens, I'll start releasing precompiled executables, to simplify
installation even further -- just drag-and-drop the executable onto your PATH.

For more information, see the [readme].

 [published]: https://crates.io/crates/httpserv
 [Rustup]: https://rustup.rs/
 [readme]: https://github.com/nic-hartley/httpserv/blob/master/README.md
