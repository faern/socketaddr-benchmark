//! Prints the memory size and alignment of the network types in question

use std::net::{Ipv4Addr, Ipv6Addr, SocketAddr, SocketAddrV4, SocketAddrV6};

fn main() {
    macro_rules! print_mem {
        ($t:ty) => {
            println!(
                "{}: size={}, align={}",
                stringify!($t),
                std::mem::size_of::<$t>(),
                std::mem::align_of::<$t>()
            );
        };
    }

    print_mem!(Ipv4Addr);
    print_mem!(Ipv6Addr);
    println!();
    print_mem!(SocketAddrV4);
    print_mem!(SocketAddrV6);
    print_mem!(SocketAddr);
}
