# project-bobcat 🐱

Lightweight compute OS configurations for morpho-web-lt featuring FreeBSD and OpenBSD nodes.

See the application source code for morpho-web-lt here: https://github.com/jpegleg/morpho-web-lt

This repository is for the operating system configurations for ultra low power systems. 
The morpho-web-lt application is a web server, which we'll run in project-bobcat on two
FreeBSD nodes. The ingress to the two FreeBSD nodes is two OpenBSD nodes which run HAProxy and pf firewall.


We'll apply maximum FreeBSD default MAC on the user we run the morpho-web-lt process under.

As of current (10/28/23) FreeBSD can leverage rustup, while OpenBSD cannot. While I have
been able to get some rust compiling on OpenBSD, it is currently not sufficient for morpho services.

OpenBSD is the gateway loadblancer with TCP passthrough roundrobin with TCP healthchecks to the morpho-web-lt compute nodes.

FreeBSD is the compute node running the morpho-web-lt application as a low privileged user.

The configurations are very simple.

The OS installation is described here:
```
FreeBSD (13.2) - following the installer (-games, no gui) and add a new user, enabling all of the security options (MAC) for that user to restrict it
OpenBSD (7.4) - default installer options (-games, no gui)
```

The scripts ending with "packages.sh" are adding vim and a few packages without adding a compiler.
The OpenBSD build only runs HAProxy from the package manager, so no compiler is needed at all.
The FreeBSD build will need binaries deployed to it. We can use cargo cross to build binaries
for the ARM or x86 compute nodes on a separate system.

The OpenBSD server is a single root process, that is mostly it. Round robin TCP.

The FreeBSD server is a single locked down process with high performance async IO with tokio actix and ZFS file systems.

The morpho wrapper can be used with init, cron, or manual.

```
/usr/local/sbin/morpho start
```

Additionally, the required files must
be in place:

```
/home/crab/cert.pem
/home/crab/privkey.pem
/home/crab/static/index.html
```

Within `/home/crab/static` is the web root. Place all the exposed
web content for morpho to serve up there, or in another location
as from new functions added to morpho.

The `/etc/pf.conf` is overwritten on both lb and compute nodes with
a custom config and reloaded. This is performed by `fire.sh` or
can be done normally with `pfctl -f /etc/pf.conf`.






