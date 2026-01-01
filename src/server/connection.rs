use crate::errors::Result;
use bytes::{Buf, BytesMut};
use tokio::{io::{AsyncReadExt, AsyncWriteExt}, net::TcpStream};

// Very naive MQTT v3.1.1 handling sufficient for mosquitto_pub QoS0:
// - Respond CONNACK to CONNECT
// - Accept PUBLISH (QoS0 assumed) without response
// - Close on DISCONNECT
pub async fn handle(mut stream: TcpStream) -> Result<()> {
    let mut buf = BytesMut::with_capacity(2048);
    loop {
        let mut temp = [0u8; 512];
        let n = stream.read(&mut temp).await?;
        if n == 0 { break; }
        buf.extend_from_slice(&temp[..n]);

        // Process messages in a very simplified way (assumes single-byte Remaining Length)
        while buf.len() >= 2 {
            let header = buf[0];
            let remaining_len = buf[1] as usize;
            if buf.len() < 2 + remaining_len { break; }

            let packet_type = header & 0xF0; // high nibble
            let _flags = header & 0x0F; // not validated

            // Slice out the current packet payload (skip fixed header + remaining length)
            // Extract the current frame and then payload view
            let mut frame = buf.split_to(2 + remaining_len);
            // Drop fixed header
            frame.advance(2);
            let _payload = &frame[..];

            match packet_type {
                0x10 => {
                    // CONNECT -> reply CONNACK (Connection Accepted)
                    // CONNACK: 0x20, remaining length 0x02, flags 0x00, return code 0x00
                    let resp = [0x20u8, 0x02u8, 0x00u8, 0x00u8];
                    stream.write_all(&resp).await?;
                }
                0x30 => {
                    // PUBLISH (QoS0) -> no response required
                    // In future, route message to subscribers
                }
                0xE0 => {
                    // DISCONNECT -> close connection
                    return Ok(());
                }
                _ => {
                    // Ignore other packets for now
                }
            }
        }
    }
    Ok(())
}
