resource "vultr_vpc" "main_vpc" {
  description = "Web Zone"
  region = "atl"
  v4_subnet = "10.100.0.0"
  v4_subnet_mask = "16"
}

resource "vultr_firewall_group" "fw_network_a" {
  description = "Firewall Network A"
}

resource "vultr_firewall_group" "fw_network_b" {
  description = "Firewall Network B"
}

resource "vultr_firewall_rule" "network_a_inbound" {
  firewall_group_id = vultr_firewall_group.fw_network_a.id
  protocol = "tcp"
  ip_type = "v4"
  subnet = "0.0.0.0"
  subnet_size = 0
  port = "22:443"
}

resource "vultr_firewall_rule" "network_b_inbound" {
  firewall_group_id = vultr_firewall_group.fw_network_b.id
  protocol = "tcp"
  ip_type = "v4"
  subnet = "0.0.0.0"
  subnet_size = 0
  port = "22:443"
}

resource "vultr_firewall_rule" "network_a_to_b_outbound" {
  firewall_group_id = vultr_firewall_group.fw_network_a.id
  protocol = "tcp"
  ip_type = "v4"
  subnet = "10.100.20.0"
  subnet_size = 24
  port = "1:62000"
}

resource "vultr_firewall_rule" "network_b_from_a_inbound" {
  firewall_group_id = vultr_firewall_group.fw_network_b.id
  protocol = "tcp"
  ip_type = "v4"
  subnet = "10.100.10.0"
  subnet_size = 24
  port = "1:62000"
}
