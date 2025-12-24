# project-bobcat 🐱

Lightweight compute OS configurations using OpenBSD. 

This repository is for the operating system configurations for ultra low resources systems with high levels of security.

The web server and load balancer have OpenBSD's `unveil` and `pledge` integrated to restrict syscalls and files to only what is required, sandboxing the program.
The web server is made with Actix, and uses the openssl crypto provider from the host, which is libressl for OpenBSD.
The ciphers are set in the program at compile time, currently coded for "modern_v5" which is TLSv1.3 only. See https://wiki.mozilla.org/Security/Server_Side_TLS for more information regarding "modern v5".
The load balancer is made with Tokio, and uses extremely efficient async-io, an ordered "first up", and bidirectional streaming. The environment variabes set in the `kia` script determine the listener
and the backend targets. All traffic will be routed to the first web server unless it is down. If it is down, traffic will go to the second web server.


<b>Important note for running kiaproxy on OpenBSD:</b> The file limits (the important default to change is defined in /etc/login.conf) should be raised from the OpenBSD defaults for the load balancer nodes that run kiaproxy,
otherwise a DoS condition is possible. The other limit on OpenBSD is from (sysctl kern.maxfiles), which is the global limit that is usually more reasonable. I would ensure kiaproxy can get over 4,000 files, and wouldn't feel bad about going higher, especially if internet facing.

#### terraform

There is a directory named `terraform` that deploys 4 servers small shared servers to one Vultr datacenter. Default is Atlanta. Add a second group to another DC for DC failover.

There are two load balancers and two web servers, with some networking rules to limit access. Further network restriction is done with `pf`.

#### ansible

There is a directory named `ansible` that has an Ansible playbook for configuring the servers. Before running the playbook, compile `morphobsd` (morpho-server binary compiled for OpenBSD), compile `kiaproxybsd` (kiaproxy binary compiled for OpenBSD), and place the binaries in a directory named `file`s. Additionally, copy the file `morpho` into `files` and `kia.j2` into `files`. We also need `pf.conf` files as `pf.conf__lb` and `pf.conf__compute` in `files`.

The ansible is a WIP, more to come soon.

#### more documentation coming soon

