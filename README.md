# project-bobcat 🐱

Lightweight compute OS configurations for morpho-web-lt featuring FreeBSD and OpenBSD nodes.

See the application source code for morpho-web-lt here: https://github.com/jpegleg/morpho-web-lt

This repository is for the operating system configurations for ultra low power systems. 
The morpho-web-lt application is a web server, which we'll run in project-bobcat on two
FreeBSD nodes. The ingress to the two FreeBSD nodes is two OpenBSD nodes which run HAProxy and pf firewall.


We'll apply maximum FreeBSD default MAC on the user we run the morpho-web-lt process under.

As of current (10/28/23) FreeBSD can leverage rustup, while OpenBSD cannot. While I have
been able to get some rust compiling on OpenBSD, it is currently not sufficient for morpho services.

The OpenBSD node HAProxy is not as high performance as Actix, but still performs great and is sufficient for project-bobcat.
The two main benefits of the HAProxy usage is solid mTLS configuration and load balancing between
the two morpho-web-lt FreeBSD nodes. While we could replace HAProxy with a rust load balancer, the
HAProxy mTLS configuration is currently supperior to stable rust mTLS designs.

