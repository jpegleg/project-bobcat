resource "vultr_ssh_key" "enre" {
  name = "enre"
  ssh_key = "YOUR PUBLIC KEY GOES HERE"
}

resource "vultr_instance" "lb1" {
    hostname = "lb1"
    plan = "vc2-1c-1gb"
    region = "atl"
    os_id = 2573
    ssh_key_ids = ["${vultr_ssh_key.enre.id}"]
    label = "lb1"
    vpc_ids = [vultr_vpc.main_vpc.id]
    firewall_group_id = vultr_firewall_group.fw_network_a.id
}

resource "vultr_instance" "lb2" {
    hostname = "lb2"
    plan = "vc2-1c-1gb"
    region = "atl"
    os_id = 2573
    ssh_key_ids = ["${vultr_ssh_key.enre.id}"]
    label = "lb1"
    vpc_ids = [vultr_vpc.main_vpc.id]
    firewall_group_id = vultr_firewall_group.fw_network_a.id
}

resource "vultr_instance" "wb1" {
    hostname = "wb1"
    plan = "vc2-1c-1gb"
    region = "atl"
    os_id = 2573
    ssh_key_ids = ["${vultr_ssh_key.enre.id}"]
    label = "wb1"
    vpc_ids = [vultr_vpc.main_vpc.id]
    firewall_group_id = vultr_firewall_group.fw_network_b.id
}

resource "vultr_instance" "wb2" {
    hostname = "wb2"
    plan = "vc2-1c-1gb"
    region = "atl"
    os_id = 2573
    ssh_key_ids = ["${vultr_ssh_key.enre.id}"]
    label = "wb2"
    vpc_ids = [vultr_vpc.main_vpc.id]
    firewall_group_id = vultr_firewall_group.fw_network_b.id
}
