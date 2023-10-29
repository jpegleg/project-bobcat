#!/usr/bin/env bash

uname -a | grep FreeBSD || cp pf.conf__openbsd-lb /etc/pf.conf && 
  cp haproxy.cfg__tls-passthrough /etc/haproxy/haproxy.cfg && 
  rcctl restart haproxy
uname -a | grep OpenBSD || cp pf.conf__freebsd-compute /etc/pf.conf && 
  cat add_pf >> /etc/rc.conf && cp morpho /usr/local/sbin/morpho &&
  chmod +x /usr/local/sbin/morpho &&
  /usr/local/sbin/morpho start

pfctl -f /etc/pf.conf
