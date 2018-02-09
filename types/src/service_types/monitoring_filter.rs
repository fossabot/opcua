// This file was autogenerated from Opc.Ua.Types.bsd.xml by tools/schema/gen_types.js
// DO NOT EDIT THIS FILE

use std::io::{Read, Write};

use encoding::*;
#[allow(unused_imports)]
use basic_types::*;
use service_types::impls::MessageInfo;
use node_ids::ObjectId;

#[derive(Debug, Clone, PartialEq)]
pub struct MonitoringFilter {
}

impl MessageInfo for MonitoringFilter {
    fn object_id(&self) -> ObjectId {
        ObjectId::MonitoringFilter_Encoding_DefaultBinary
    }
}

impl BinaryEncoder<MonitoringFilter> for MonitoringFilter {
    fn byte_len(&self) -> usize {
        0
    }

    #[allow(unused_variables)]
    fn encode<S: Write>(&self, stream: &mut S) -> EncodingResult<usize> {
        Ok(0)
    }

    #[allow(unused_variables)]
    fn decode<S: Read>(stream: &mut S) -> EncodingResult<Self> {
        Ok(MonitoringFilter {
        })
    }
}