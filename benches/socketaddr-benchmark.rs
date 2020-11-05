use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::net::{Ipv4Addr, Ipv6Addr, SocketAddr, SocketAddrV4, SocketAddrV6, UdpSocket};

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("Ipv4Addr::new", |b| {
        b.iter(|| Ipv4Addr::new(black_box(20), black_box(30), black_box(40), black_box(50)))
    });
    c.bench_function("Ipv6Addr::new", |b| {
        b.iter(|| {
            Ipv6Addr::new(
                black_box(20),
                black_box(30),
                black_box(40),
                black_box(50),
                black_box(60),
                black_box(70),
                black_box(80),
                black_box(90),
            )
        })
    });

    c.bench_function("SocketAddrV4::new", |b| {
        let ip = Ipv4Addr::new(20, 30, 40, 50);
        b.iter(|| SocketAddrV4::new(black_box(ip), black_box(9876)))
    });
    c.bench_function("SocketAddrV6::new", |b| {
        let ip = Ipv6Addr::new(1, 2, 3, 4, 5, 6, 7, 8);
        b.iter(|| SocketAddrV6::new(black_box(ip), black_box(9876), 0, 0))
    });

    c.bench_function("Ipv4Addr::fmt", |b| {
        let ip = Ipv4Addr::new(20, 30, 40, 50);
        b.iter(|| format!("{}", ip))
    });
    c.bench_function("Ipv6Addr::fmt", |b| {
        let ip = Ipv6Addr::new(0x2000, 2, 3, 4, 5, 0, 0, 1);
        b.iter(|| format!("{}", ip))
    });

    c.bench_function("SocketAddrV4::fmt", |b| {
        let ip = Ipv4Addr::new(20, 30, 40, 50);
        let addr = SocketAddrV4::new(ip, 9876);
        b.iter(|| format!("{}", addr))
    });
    c.bench_function("SocketAddrV6::fmt", |b| {
        let ip = Ipv6Addr::new(1, 2, 3, 4, 5, 6, 7, 8);
        let addr = SocketAddrV6::new(ip, 9876, 0, 0);
        b.iter(|| format!("{}", addr))
    });

    c.bench_function("UdpSocket::send_to (v4)", |b| {
        let local_addr = SocketAddr::new(Ipv4Addr::UNSPECIFIED.into(), 0);
        let remote_addr = SocketAddr::new(Ipv4Addr::new(198, 18, 0, 1).into(), 1234);
        let socket = UdpSocket::bind(local_addr).unwrap();
        let buf = [0u8; 1];
        b.iter(|| socket.send_to(&buf, black_box(remote_addr)).unwrap())
    });
    c.bench_function("UdpSocket::send_to (v6)", |b| {
        let local_addr = SocketAddr::new(Ipv6Addr::UNSPECIFIED.into(), 0);
        let remote_addr = SocketAddr::new(Ipv6Addr::new(0xfe80, 0, 0, 0,0, 0, 0, 0x1).into(), 1234);
        let socket = UdpSocket::bind(local_addr).unwrap();
        let buf = [0u8; 1];
        b.iter(|| socket.send_to(&buf, black_box(remote_addr)).unwrap())
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
