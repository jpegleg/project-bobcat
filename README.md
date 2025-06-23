# project-bobcat 🐱

Lightweight compute OS configurations using OpenBSD. 

This repository is for the operating system configurations for ultra low resources systems with high levels of security.

The web server has OpenBSD's `unveil` and `pledge` integrated to restrict syscalls and files to only what is required, sandboxing the program.
The web server is made with Actix, and uses the openssl crypto provider from the host, which is libressl for OpenBSD.
The ciphers are set in the program at compile time, currently coded for "modern_v5" which is TLSv1.3 only. See https://wiki.mozilla.org/Security/Server_Side_TLS for more information regarding "modern v5".

#### more instruction detail coming to this document soon

