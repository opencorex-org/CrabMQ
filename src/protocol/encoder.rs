#![allow(dead_code)]
use bytes::{BufMut, BytesMut};
use crate::protocol::types::{Packet, Publish};

// Very minimal, fake encoder for demo/testing only
pub fn encode_packet(packet: &Packet) -> BytesMut {
    let mut out = BytesMut::new();
    match packet {
        Packet::Publish(Publish { topic: _, payload }) => {
            out.put_u8(0x30); // PUBLISH fixed header
            out.put_u8(payload.len() as u8); // tiny len
            out.extend_from_slice(payload);
        }
    }
    out
}
