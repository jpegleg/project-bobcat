output "web_ip1" {
  value       = vultr_instance.wb1.main_ip
  description = "Web server 1 IP."
}

output "web_ip2" {
  value       = vultr_instance.wb2.main_ip
  description = "Web server 2 IP."
}

output "lb_ip1" {
  value       = vultr_instance.lb1.main_ip
  description = "Load balancer firewall server 1 IP."
}

output "lb_ip2" {
  value       = vultr_instance.lb2.main_ip
  description = "Load balancer firewall server 2 IP."
}
