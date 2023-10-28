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
FreeBSD (13.2) - following the installer and add a new user, enabling all of the security options (MAC) for that user to restrict it
OpenBSD (7.4) - default installer options
```




