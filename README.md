# project-bobcat 🐱

Lightweight compute OS configurations using OpenBSD. 

This repository is for the operating system configurations for ultra low resources systems with high levels of security.

The web server has OpenBSD's `unveil` and `pledge` integrated to restrict syscalls and files to only what is required, sandboxing the program.
The web server is made with Actix, and uses the openssl crypto provider from the host, which is libressl for OpenBSD.
The ciphers are set in the program at compile time, currently coded for "modern_v5" which is TLSv1.3 only. See https://wiki.mozilla.org/Security/Server_Side_TLS for more information regarding "modern v5".

#### terraform

There is a directory named `terraform` that deploys 4 servers small shared servers to one Vultr datacenter. Default is Atlanta. Add a second group to another DC for DC failover.

There are two load balancers and two web servers, with some networking rules to limit access. Further network restriction is done with `pf`.

#### ansible

There is a directory named `ansible` that has an Ansible playbook for configuring the serveres. Before running the playbook, compile `morphobsd` (morpho-server binary compiled for OpenBSD) and place that binary in a directory named files. Additionally, copy the file `morpho` into `files`. Further, create a `haproxy.cfg_prod` in `files`, using `haproxy.cfg__tls-passthrough` and as a reference. We also need `pf.conf` files as `pf.conf__lb` and `pf.conf__compute` in `files`.


#### more documentation coming soon

