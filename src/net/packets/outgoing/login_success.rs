use ferrumc_codec::network_types::varint::VarInt;

use ferrumc_macros::NetEncode;

/// Sent by the server to the client to start the play state.
#[derive(NetEncode)]
pub struct LoginSuccess {
    #[encode(default=VarInt::from(0x02))]
    pub packet_id: VarInt,
    pub uuid: Vec<u8>,
    pub username: String,
    // Just set this to 0
    pub property_count: VarInt,
    // TODO: Figure out how what in the everloving fuck this is
    pub properties: Vec<Property>,
}

#[derive(NetEncode)]
pub struct Property {
    pub name: String,
    pub value: String,
    pub is_signed: bool,
    // Only if is_signed is true
    pub signature: String,
}
