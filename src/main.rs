extern crate protobuf;
extern crate condensor;

use condensor::steam::steammessages_remoteclient_discovery;
use condensor::steam::steammessages_remoteclient_discovery::ERemoteClientBroadcastMsg;
use std::io;
use std::io::net::udp;
use std::io::net::ip;

// Magic bytes that identify Steam IHS packets
static MAGIC: [u8; 8] = [ 0xFF, 0xFF, 0xFF, 0xFF, 0x21, 0x4c, 0x5f, 0xa0 ];

fn parse_message<T: protobuf::Message + protobuf::MessageStatic>(reader: &mut io::BufReader) -> T {
    let length = reader.read_le_i32().ok().expect("Failed to read payload length");
    let mut payload = reader.read_exact(length as uint).ok().expect("Failed to read payload");

    protobuf::parse_from_bytes::<T>(payload.as_slice()).ok().expect("Failed to parse body")
}

fn broadcast_msg_discovery(reader: &mut io::BufReader) {
    let message = parse_message::<steammessages_remoteclient_discovery::CMsgRemoteClientBroadcastDiscovery>(reader);
    println!("Message {}", message);
}

fn broadcast_msg_status(reader: &mut io::BufReader) {
    let message = parse_message::<steammessages_remoteclient_discovery::CMsgRemoteClientBroadcastStatus>(reader);
    println!("Message {}", message);
}

fn decode(buffer: &mut [u8]) {
    let mut reader = io::BufReader::new(buffer);

    let magic = reader.read_exact(8).ok().expect("Failed to read magic");

    let header = parse_message::<steammessages_remoteclient_discovery::CMsgRemoteClientBroadcastHeader>(&mut reader);

    println!("Header {}", header);

    match header.get_msg_type() {
        ERemoteClientBroadcastMsg::k_ERemoteClientBroadcastMsgDiscovery => broadcast_msg_discovery(&mut reader),
        ERemoteClientBroadcastMsg::k_ERemoteClientBroadcastMsgStatus => broadcast_msg_status(&mut reader),
        _ => ()
    };

}

fn main() {
    let addr = ip::SocketAddr { ip: ip::Ipv4Addr(0, 0, 0, 0), port: 27036 };

    let mut socket = match udp::UdpSocket::bind(addr) {
        Ok(s) => s,
        Err(e) => panic!("couldn't bind socket: {}", e),
    };

    loop {
        let mut buffer = [0u8; 1500];

        match socket.recv_from(buffer.as_mut_slice()) {
            Ok((amount, _))  => decode(buffer.slice_to_mut(amount)),
            Err(e) => panic!("couldn't receive a datagram: {}", e),
        };
    }

    drop(socket);
}

