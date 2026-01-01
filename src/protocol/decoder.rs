#![allow(dead_code)]
use bytes::{Buf, Bytes};
use crate::protocol::types::{Packet, Publish};

// Very minimal, fake decoder for demo/testing only
pub fn decode_packet(mut bytes: Bytes) -> Option<Packet> {
    if bytes.remaining() < 2 { return None; }
    let _first = bytes.get_u8();
    let len = bytes.get_u8() as usize;
    if bytes.remaining() < len { return None; }
    let topic = "test/topic".to_string();
    let payload = bytes.copy_to_bytes(len).to_vec();
    Some(Packet::Publish(Publish { topic, payload }))
}
