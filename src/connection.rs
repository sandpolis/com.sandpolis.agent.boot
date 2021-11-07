use smoltcp::iface::{EthernetInterfaceBuilder, NeighborCache, Routes};
use smoltcp::phy::{wait as phy_wait, Device, Medium};
use smoltcp::socket::{TcpSocket, TcpSocketBuffer};

pub struct Connection {

    socket: TcpSocket,
    interface: EthernetInterface,
}

impl Connection {

    pub fn new(snp: Snp) -> Result<Connection> {

        snp.mode().current_address()

        let iface = EthernetInterfaceBuilder::new(device)
            .ethernet_addr(hw_addr)
            .neighbor_cache(neighbor_cache)
            .ip_addrs(ip_addrs)
            .finalize();
    }
}
