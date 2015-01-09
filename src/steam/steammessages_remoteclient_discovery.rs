// This file is generated. Do not edit

#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(unused_imports)]

use protobuf::Message as Message_imported_for_functions;
use protobuf::ProtobufEnum as ProtobufEnum_imported_for_functions;

#[derive(Clone,Default)]
pub struct CMsgRemoteClientBroadcastHeader {
    client_id: ::std::option::Option<u64>,
    msg_type: ::std::option::Option<ERemoteClientBroadcastMsg>,
    instance_id: ::std::option::Option<u64>,
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl CMsgRemoteClientBroadcastHeader {
    pub fn new() -> CMsgRemoteClientBroadcastHeader {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgRemoteClientBroadcastHeader {
        static mut instance: ::protobuf::lazy::Lazy<CMsgRemoteClientBroadcastHeader> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgRemoteClientBroadcastHeader,
        };
        unsafe {
            instance.get(|| {
                CMsgRemoteClientBroadcastHeader {
                    client_id: ::std::option::Option::None,
                    msg_type: ::std::option::Option::None,
                    instance_id: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional uint64 client_id = 1;

    pub fn clear_client_id(&mut self) {
        self.client_id = ::std::option::Option::None;
    }

    pub fn has_client_id(&self) -> bool {
        self.client_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_client_id(&mut self, v: u64) {
        self.client_id = ::std::option::Option::Some(v);
    }

    pub fn get_client_id<'a>(&self) -> u64 {
        self.client_id.unwrap_or(0)
    }

    // optional .ERemoteClientBroadcastMsg msg_type = 2;

    pub fn clear_msg_type(&mut self) {
        self.msg_type = ::std::option::Option::None;
    }

    pub fn has_msg_type(&self) -> bool {
        self.msg_type.is_some()
    }

    // Param is passed by value, moved
    pub fn set_msg_type(&mut self, v: ERemoteClientBroadcastMsg) {
        self.msg_type = ::std::option::Option::Some(v);
    }

    pub fn get_msg_type<'a>(&self) -> ERemoteClientBroadcastMsg {
        self.msg_type.unwrap_or(ERemoteClientBroadcastMsg::k_ERemoteClientBroadcastMsgDiscovery)
    }

    // optional uint64 instance_id = 3;

    pub fn clear_instance_id(&mut self) {
        self.instance_id = ::std::option::Option::None;
    }

    pub fn has_instance_id(&self) -> bool {
        self.instance_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_instance_id(&mut self, v: u64) {
        self.instance_id = ::std::option::Option::Some(v);
    }

    pub fn get_instance_id<'a>(&self) -> u64 {
        self.instance_id.unwrap_or(0)
    }
}

impl ::protobuf::Message for CMsgRemoteClientBroadcastHeader {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_uint64());
                    self.client_id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_enum());
                    self.msg_type = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_uint64());
                    self.instance_id = ::std::option::Option::Some(tmp);
                },
                _ => {
                    let unknown = try!(is.read_unknown(wire_type));
                    self.mut_unknown_fields().add_value(field_number, unknown);
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.client_id.iter() {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.msg_type.iter() {
            my_size += ::protobuf::rt::enum_size(2, *value);
        };
        for value in self.instance_id.iter() {
            my_size += ::protobuf::rt::value_size(3, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.client_id {
            try!(os.write_uint64(1, v));
        };
        if let Some(v) = self.msg_type {
            try!(os.write_enum(2, v as i32));
        };
        if let Some(v) = self.instance_id {
            try!(os.write_uint64(3, v));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields<'s>(&'s self) -> &'s ::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields<'s>(&'s mut self) -> &'s mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::intrinsics::TypeId {
        ::std::intrinsics::TypeId::of::<CMsgRemoteClientBroadcastHeader>()
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CMsgRemoteClientBroadcastHeader {
    fn new() -> CMsgRemoteClientBroadcastHeader {
        CMsgRemoteClientBroadcastHeader::new()
    }

    #[allow(unused_unsafe,unused_mut)]
    fn descriptor_static(_: ::std::option::Option<CMsgRemoteClientBroadcastHeader>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "client_id",
                    CMsgRemoteClientBroadcastHeader::has_client_id,
                    CMsgRemoteClientBroadcastHeader::get_client_id,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_enum_accessor(
                    "msg_type",
                    CMsgRemoteClientBroadcastHeader::has_msg_type,
                    CMsgRemoteClientBroadcastHeader::get_msg_type,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "instance_id",
                    CMsgRemoteClientBroadcastHeader::has_instance_id,
                    CMsgRemoteClientBroadcastHeader::get_instance_id,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgRemoteClientBroadcastHeader>(
                    "CMsgRemoteClientBroadcastHeader",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgRemoteClientBroadcastHeader {
    fn clear(&mut self) {
        self.clear_client_id();
        self.clear_msg_type();
        self.clear_instance_id();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for CMsgRemoteClientBroadcastHeader {
    fn eq(&self, other: &CMsgRemoteClientBroadcastHeader) -> bool {
        self.client_id == other.client_id &&
        self.msg_type == other.msg_type &&
        self.instance_id == other.instance_id &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Show for CMsgRemoteClientBroadcastHeader {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct CMsgRemoteClientBroadcastStatus {
    version: ::std::option::Option<i32>,
    min_version: ::std::option::Option<i32>,
    connect_port: ::std::option::Option<u32>,
    hostname: ::protobuf::SingularField<::std::string::String>,
    enabled_services: ::std::option::Option<u32>,
    ostype: ::std::option::Option<i32>,
    is64bit: ::std::option::Option<bool>,
    users: ::protobuf::RepeatedField<CMsgRemoteClientBroadcastStatus_User>,
    remote_control_port: ::std::option::Option<u32>,
    euniverse: ::std::option::Option<i32>,
    timestamp: ::std::option::Option<u32>,
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl CMsgRemoteClientBroadcastStatus {
    pub fn new() -> CMsgRemoteClientBroadcastStatus {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgRemoteClientBroadcastStatus {
        static mut instance: ::protobuf::lazy::Lazy<CMsgRemoteClientBroadcastStatus> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgRemoteClientBroadcastStatus,
        };
        unsafe {
            instance.get(|| {
                CMsgRemoteClientBroadcastStatus {
                    version: ::std::option::Option::None,
                    min_version: ::std::option::Option::None,
                    connect_port: ::std::option::Option::None,
                    hostname: ::protobuf::SingularField::none(),
                    enabled_services: ::std::option::Option::None,
                    ostype: ::std::option::Option::None,
                    is64bit: ::std::option::Option::None,
                    users: ::protobuf::RepeatedField::new(),
                    remote_control_port: ::std::option::Option::None,
                    euniverse: ::std::option::Option::None,
                    timestamp: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional int32 version = 1;

    pub fn clear_version(&mut self) {
        self.version = ::std::option::Option::None;
    }

    pub fn has_version(&self) -> bool {
        self.version.is_some()
    }

    // Param is passed by value, moved
    pub fn set_version(&mut self, v: i32) {
        self.version = ::std::option::Option::Some(v);
    }

    pub fn get_version<'a>(&self) -> i32 {
        self.version.unwrap_or(0)
    }

    // optional int32 min_version = 2;

    pub fn clear_min_version(&mut self) {
        self.min_version = ::std::option::Option::None;
    }

    pub fn has_min_version(&self) -> bool {
        self.min_version.is_some()
    }

    // Param is passed by value, moved
    pub fn set_min_version(&mut self, v: i32) {
        self.min_version = ::std::option::Option::Some(v);
    }

    pub fn get_min_version<'a>(&self) -> i32 {
        self.min_version.unwrap_or(0)
    }

    // optional uint32 connect_port = 3;

    pub fn clear_connect_port(&mut self) {
        self.connect_port = ::std::option::Option::None;
    }

    pub fn has_connect_port(&self) -> bool {
        self.connect_port.is_some()
    }

    // Param is passed by value, moved
    pub fn set_connect_port(&mut self, v: u32) {
        self.connect_port = ::std::option::Option::Some(v);
    }

    pub fn get_connect_port<'a>(&self) -> u32 {
        self.connect_port.unwrap_or(0)
    }

    // optional string hostname = 4;

    pub fn clear_hostname(&mut self) {
        self.hostname.clear();
    }

    pub fn has_hostname(&self) -> bool {
        self.hostname.is_some()
    }

    // Param is passed by value, moved
    pub fn set_hostname(&mut self, v: ::std::string::String) {
        self.hostname = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_hostname<'a>(&'a mut self) -> &'a mut ::std::string::String {
        if self.hostname.is_none() {
            self.hostname.set_default();
        };
        self.hostname.as_mut().unwrap()
    }

    // Take field
    pub fn take_hostname(&mut self) -> ::std::string::String {
        self.hostname.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_hostname<'a>(&'a self) -> &'a str {
        match self.hostname.as_ref() {
            Some(v) => v.as_slice(),
            None => "",
        }
    }

    // optional uint32 enabled_services = 6;

    pub fn clear_enabled_services(&mut self) {
        self.enabled_services = ::std::option::Option::None;
    }

    pub fn has_enabled_services(&self) -> bool {
        self.enabled_services.is_some()
    }

    // Param is passed by value, moved
    pub fn set_enabled_services(&mut self, v: u32) {
        self.enabled_services = ::std::option::Option::Some(v);
    }

    pub fn get_enabled_services<'a>(&self) -> u32 {
        self.enabled_services.unwrap_or(0)
    }

    // optional int32 ostype = 7;

    pub fn clear_ostype(&mut self) {
        self.ostype = ::std::option::Option::None;
    }

    pub fn has_ostype(&self) -> bool {
        self.ostype.is_some()
    }

    // Param is passed by value, moved
    pub fn set_ostype(&mut self, v: i32) {
        self.ostype = ::std::option::Option::Some(v);
    }

    pub fn get_ostype<'a>(&self) -> i32 {
        self.ostype.unwrap_or(0i32)
    }

    // optional bool is64bit = 8;

    pub fn clear_is64bit(&mut self) {
        self.is64bit = ::std::option::Option::None;
    }

    pub fn has_is64bit(&self) -> bool {
        self.is64bit.is_some()
    }

    // Param is passed by value, moved
    pub fn set_is64bit(&mut self, v: bool) {
        self.is64bit = ::std::option::Option::Some(v);
    }

    pub fn get_is64bit<'a>(&self) -> bool {
        self.is64bit.unwrap_or(false)
    }

    // repeated .CMsgRemoteClientBroadcastStatus.User users = 9;

    pub fn clear_users(&mut self) {
        self.users.clear();
    }

    // Param is passed by value, moved
    pub fn set_users(&mut self, v: ::protobuf::RepeatedField<CMsgRemoteClientBroadcastStatus_User>) {
        self.users = v;
    }

    // Mutable pointer to the field.
    pub fn mut_users<'a>(&'a mut self) -> &'a mut ::protobuf::RepeatedField<CMsgRemoteClientBroadcastStatus_User> {
        &mut self.users
    }

    // Take field
    pub fn take_users(&mut self) -> ::protobuf::RepeatedField<CMsgRemoteClientBroadcastStatus_User> {
        ::std::mem::replace(&mut self.users, ::protobuf::RepeatedField::new())
    }

    pub fn get_users<'a>(&'a self) -> &'a [CMsgRemoteClientBroadcastStatus_User] {
        self.users.as_slice()
    }

    // optional uint32 remote_control_port = 10;

    pub fn clear_remote_control_port(&mut self) {
        self.remote_control_port = ::std::option::Option::None;
    }

    pub fn has_remote_control_port(&self) -> bool {
        self.remote_control_port.is_some()
    }

    // Param is passed by value, moved
    pub fn set_remote_control_port(&mut self, v: u32) {
        self.remote_control_port = ::std::option::Option::Some(v);
    }

    pub fn get_remote_control_port<'a>(&self) -> u32 {
        self.remote_control_port.unwrap_or(0)
    }

    // optional int32 euniverse = 11;

    pub fn clear_euniverse(&mut self) {
        self.euniverse = ::std::option::Option::None;
    }

    pub fn has_euniverse(&self) -> bool {
        self.euniverse.is_some()
    }

    // Param is passed by value, moved
    pub fn set_euniverse(&mut self, v: i32) {
        self.euniverse = ::std::option::Option::Some(v);
    }

    pub fn get_euniverse<'a>(&self) -> i32 {
        self.euniverse.unwrap_or(0)
    }

    // optional uint32 timestamp = 12;

    pub fn clear_timestamp(&mut self) {
        self.timestamp = ::std::option::Option::None;
    }

    pub fn has_timestamp(&self) -> bool {
        self.timestamp.is_some()
    }

    // Param is passed by value, moved
    pub fn set_timestamp(&mut self, v: u32) {
        self.timestamp = ::std::option::Option::Some(v);
    }

    pub fn get_timestamp<'a>(&self) -> u32 {
        self.timestamp.unwrap_or(0)
    }
}

impl ::protobuf::Message for CMsgRemoteClientBroadcastStatus {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_int32());
                    self.version = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_int32());
                    self.min_version = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_uint32());
                    self.connect_port = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.hostname.set_default();
                    try!(is.read_string_into(tmp))
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_uint32());
                    self.enabled_services = ::std::option::Option::Some(tmp);
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_int32());
                    self.ostype = ::std::option::Option::Some(tmp);
                },
                8 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_bool());
                    self.is64bit = ::std::option::Option::Some(tmp);
                },
                9 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.users));
                },
                10 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_uint32());
                    self.remote_control_port = ::std::option::Option::Some(tmp);
                },
                11 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_int32());
                    self.euniverse = ::std::option::Option::Some(tmp);
                },
                12 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_uint32());
                    self.timestamp = ::std::option::Option::Some(tmp);
                },
                _ => {
                    let unknown = try!(is.read_unknown(wire_type));
                    self.mut_unknown_fields().add_value(field_number, unknown);
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.version.iter() {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.min_version.iter() {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.connect_port.iter() {
            my_size += ::protobuf::rt::value_size(3, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.hostname.iter() {
            my_size += ::protobuf::rt::string_size(4, value.as_slice());
        };
        for value in self.enabled_services.iter() {
            my_size += ::protobuf::rt::value_size(6, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.ostype.iter() {
            my_size += ::protobuf::rt::value_size(7, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        if self.is64bit.is_some() {
            my_size += 2;
        };
        for value in self.users.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.remote_control_port.iter() {
            my_size += ::protobuf::rt::value_size(10, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.euniverse.iter() {
            my_size += ::protobuf::rt::value_size(11, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.timestamp.iter() {
            my_size += ::protobuf::rt::value_size(12, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.version {
            try!(os.write_int32(1, v));
        };
        if let Some(v) = self.min_version {
            try!(os.write_int32(2, v));
        };
        if let Some(v) = self.connect_port {
            try!(os.write_uint32(3, v));
        };
        if let Some(v) = self.hostname.as_ref() {
            try!(os.write_string(4, v.as_slice()));
        };
        if let Some(v) = self.enabled_services {
            try!(os.write_uint32(6, v));
        };
        if let Some(v) = self.ostype {
            try!(os.write_int32(7, v));
        };
        if let Some(v) = self.is64bit {
            try!(os.write_bool(8, v));
        };
        for v in self.users.iter() {
            try!(os.write_tag(9, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.remote_control_port {
            try!(os.write_uint32(10, v));
        };
        if let Some(v) = self.euniverse {
            try!(os.write_int32(11, v));
        };
        if let Some(v) = self.timestamp {
            try!(os.write_uint32(12, v));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields<'s>(&'s self) -> &'s ::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields<'s>(&'s mut self) -> &'s mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::intrinsics::TypeId {
        ::std::intrinsics::TypeId::of::<CMsgRemoteClientBroadcastStatus>()
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CMsgRemoteClientBroadcastStatus {
    fn new() -> CMsgRemoteClientBroadcastStatus {
        CMsgRemoteClientBroadcastStatus::new()
    }

    #[allow(unused_unsafe,unused_mut)]
    fn descriptor_static(_: ::std::option::Option<CMsgRemoteClientBroadcastStatus>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "version",
                    CMsgRemoteClientBroadcastStatus::has_version,
                    CMsgRemoteClientBroadcastStatus::get_version,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "min_version",
                    CMsgRemoteClientBroadcastStatus::has_min_version,
                    CMsgRemoteClientBroadcastStatus::get_min_version,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u32_accessor(
                    "connect_port",
                    CMsgRemoteClientBroadcastStatus::has_connect_port,
                    CMsgRemoteClientBroadcastStatus::get_connect_port,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "hostname",
                    CMsgRemoteClientBroadcastStatus::has_hostname,
                    CMsgRemoteClientBroadcastStatus::get_hostname,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u32_accessor(
                    "enabled_services",
                    CMsgRemoteClientBroadcastStatus::has_enabled_services,
                    CMsgRemoteClientBroadcastStatus::get_enabled_services,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "ostype",
                    CMsgRemoteClientBroadcastStatus::has_ostype,
                    CMsgRemoteClientBroadcastStatus::get_ostype,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor(
                    "is64bit",
                    CMsgRemoteClientBroadcastStatus::has_is64bit,
                    CMsgRemoteClientBroadcastStatus::get_is64bit,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "users",
                    CMsgRemoteClientBroadcastStatus::get_users,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u32_accessor(
                    "remote_control_port",
                    CMsgRemoteClientBroadcastStatus::has_remote_control_port,
                    CMsgRemoteClientBroadcastStatus::get_remote_control_port,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "euniverse",
                    CMsgRemoteClientBroadcastStatus::has_euniverse,
                    CMsgRemoteClientBroadcastStatus::get_euniverse,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u32_accessor(
                    "timestamp",
                    CMsgRemoteClientBroadcastStatus::has_timestamp,
                    CMsgRemoteClientBroadcastStatus::get_timestamp,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgRemoteClientBroadcastStatus>(
                    "CMsgRemoteClientBroadcastStatus",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgRemoteClientBroadcastStatus {
    fn clear(&mut self) {
        self.clear_version();
        self.clear_min_version();
        self.clear_connect_port();
        self.clear_hostname();
        self.clear_enabled_services();
        self.clear_ostype();
        self.clear_is64bit();
        self.clear_users();
        self.clear_remote_control_port();
        self.clear_euniverse();
        self.clear_timestamp();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for CMsgRemoteClientBroadcastStatus {
    fn eq(&self, other: &CMsgRemoteClientBroadcastStatus) -> bool {
        self.version == other.version &&
        self.min_version == other.min_version &&
        self.connect_port == other.connect_port &&
        self.hostname == other.hostname &&
        self.enabled_services == other.enabled_services &&
        self.ostype == other.ostype &&
        self.is64bit == other.is64bit &&
        self.users == other.users &&
        self.remote_control_port == other.remote_control_port &&
        self.euniverse == other.euniverse &&
        self.timestamp == other.timestamp &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Show for CMsgRemoteClientBroadcastStatus {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct CMsgRemoteClientBroadcastStatus_User {
    steamid: ::std::option::Option<u64>,
    auth_key_id: ::std::option::Option<u32>,
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl CMsgRemoteClientBroadcastStatus_User {
    pub fn new() -> CMsgRemoteClientBroadcastStatus_User {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgRemoteClientBroadcastStatus_User {
        static mut instance: ::protobuf::lazy::Lazy<CMsgRemoteClientBroadcastStatus_User> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgRemoteClientBroadcastStatus_User,
        };
        unsafe {
            instance.get(|| {
                CMsgRemoteClientBroadcastStatus_User {
                    steamid: ::std::option::Option::None,
                    auth_key_id: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional fixed64 steamid = 1;

    pub fn clear_steamid(&mut self) {
        self.steamid = ::std::option::Option::None;
    }

    pub fn has_steamid(&self) -> bool {
        self.steamid.is_some()
    }

    // Param is passed by value, moved
    pub fn set_steamid(&mut self, v: u64) {
        self.steamid = ::std::option::Option::Some(v);
    }

    pub fn get_steamid<'a>(&self) -> u64 {
        self.steamid.unwrap_or(0)
    }

    // optional uint32 auth_key_id = 2;

    pub fn clear_auth_key_id(&mut self) {
        self.auth_key_id = ::std::option::Option::None;
    }

    pub fn has_auth_key_id(&self) -> bool {
        self.auth_key_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_auth_key_id(&mut self, v: u32) {
        self.auth_key_id = ::std::option::Option::Some(v);
    }

    pub fn get_auth_key_id<'a>(&self) -> u32 {
        self.auth_key_id.unwrap_or(0)
    }
}

impl ::protobuf::Message for CMsgRemoteClientBroadcastStatus_User {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_fixed64());
                    self.steamid = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_uint32());
                    self.auth_key_id = ::std::option::Option::Some(tmp);
                },
                _ => {
                    let unknown = try!(is.read_unknown(wire_type));
                    self.mut_unknown_fields().add_value(field_number, unknown);
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if self.steamid.is_some() {
            my_size += 9;
        };
        for value in self.auth_key_id.iter() {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.steamid {
            try!(os.write_fixed64(1, v));
        };
        if let Some(v) = self.auth_key_id {
            try!(os.write_uint32(2, v));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields<'s>(&'s self) -> &'s ::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields<'s>(&'s mut self) -> &'s mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::intrinsics::TypeId {
        ::std::intrinsics::TypeId::of::<CMsgRemoteClientBroadcastStatus_User>()
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CMsgRemoteClientBroadcastStatus_User {
    fn new() -> CMsgRemoteClientBroadcastStatus_User {
        CMsgRemoteClientBroadcastStatus_User::new()
    }

    #[allow(unused_unsafe,unused_mut)]
    fn descriptor_static(_: ::std::option::Option<CMsgRemoteClientBroadcastStatus_User>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "steamid",
                    CMsgRemoteClientBroadcastStatus_User::has_steamid,
                    CMsgRemoteClientBroadcastStatus_User::get_steamid,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u32_accessor(
                    "auth_key_id",
                    CMsgRemoteClientBroadcastStatus_User::has_auth_key_id,
                    CMsgRemoteClientBroadcastStatus_User::get_auth_key_id,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgRemoteClientBroadcastStatus_User>(
                    "CMsgRemoteClientBroadcastStatus_User",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgRemoteClientBroadcastStatus_User {
    fn clear(&mut self) {
        self.clear_steamid();
        self.clear_auth_key_id();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for CMsgRemoteClientBroadcastStatus_User {
    fn eq(&self, other: &CMsgRemoteClientBroadcastStatus_User) -> bool {
        self.steamid == other.steamid &&
        self.auth_key_id == other.auth_key_id &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Show for CMsgRemoteClientBroadcastStatus_User {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct CMsgRemoteClientBroadcastDiscovery {
    seq_num: ::std::option::Option<u32>,
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl CMsgRemoteClientBroadcastDiscovery {
    pub fn new() -> CMsgRemoteClientBroadcastDiscovery {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgRemoteClientBroadcastDiscovery {
        static mut instance: ::protobuf::lazy::Lazy<CMsgRemoteClientBroadcastDiscovery> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgRemoteClientBroadcastDiscovery,
        };
        unsafe {
            instance.get(|| {
                CMsgRemoteClientBroadcastDiscovery {
                    seq_num: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional uint32 seq_num = 1;

    pub fn clear_seq_num(&mut self) {
        self.seq_num = ::std::option::Option::None;
    }

    pub fn has_seq_num(&self) -> bool {
        self.seq_num.is_some()
    }

    // Param is passed by value, moved
    pub fn set_seq_num(&mut self, v: u32) {
        self.seq_num = ::std::option::Option::Some(v);
    }

    pub fn get_seq_num<'a>(&self) -> u32 {
        self.seq_num.unwrap_or(0)
    }
}

impl ::protobuf::Message for CMsgRemoteClientBroadcastDiscovery {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_uint32());
                    self.seq_num = ::std::option::Option::Some(tmp);
                },
                _ => {
                    let unknown = try!(is.read_unknown(wire_type));
                    self.mut_unknown_fields().add_value(field_number, unknown);
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.seq_num.iter() {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.seq_num {
            try!(os.write_uint32(1, v));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields<'s>(&'s self) -> &'s ::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields<'s>(&'s mut self) -> &'s mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::intrinsics::TypeId {
        ::std::intrinsics::TypeId::of::<CMsgRemoteClientBroadcastDiscovery>()
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CMsgRemoteClientBroadcastDiscovery {
    fn new() -> CMsgRemoteClientBroadcastDiscovery {
        CMsgRemoteClientBroadcastDiscovery::new()
    }

    #[allow(unused_unsafe,unused_mut)]
    fn descriptor_static(_: ::std::option::Option<CMsgRemoteClientBroadcastDiscovery>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_u32_accessor(
                    "seq_num",
                    CMsgRemoteClientBroadcastDiscovery::has_seq_num,
                    CMsgRemoteClientBroadcastDiscovery::get_seq_num,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgRemoteClientBroadcastDiscovery>(
                    "CMsgRemoteClientBroadcastDiscovery",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgRemoteClientBroadcastDiscovery {
    fn clear(&mut self) {
        self.clear_seq_num();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for CMsgRemoteClientBroadcastDiscovery {
    fn eq(&self, other: &CMsgRemoteClientBroadcastDiscovery) -> bool {
        self.seq_num == other.seq_num &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Show for CMsgRemoteClientBroadcastDiscovery {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct CMsgRemoteDeviceAuthorizationRequest {
    device_token: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    device_name: ::protobuf::SingularField<::std::string::String>,
    encrypted_request: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl CMsgRemoteDeviceAuthorizationRequest {
    pub fn new() -> CMsgRemoteDeviceAuthorizationRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgRemoteDeviceAuthorizationRequest {
        static mut instance: ::protobuf::lazy::Lazy<CMsgRemoteDeviceAuthorizationRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgRemoteDeviceAuthorizationRequest,
        };
        unsafe {
            instance.get(|| {
                CMsgRemoteDeviceAuthorizationRequest {
                    device_token: ::protobuf::SingularField::none(),
                    device_name: ::protobuf::SingularField::none(),
                    encrypted_request: ::protobuf::SingularField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required bytes device_token = 1;

    pub fn clear_device_token(&mut self) {
        self.device_token.clear();
    }

    pub fn has_device_token(&self) -> bool {
        self.device_token.is_some()
    }

    // Param is passed by value, moved
    pub fn set_device_token(&mut self, v: ::std::vec::Vec<u8>) {
        self.device_token = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_device_token<'a>(&'a mut self) -> &'a mut ::std::vec::Vec<u8> {
        if self.device_token.is_none() {
            self.device_token.set_default();
        };
        self.device_token.as_mut().unwrap()
    }

    // Take field
    pub fn take_device_token(&mut self) -> ::std::vec::Vec<u8> {
        self.device_token.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_device_token<'a>(&'a self) -> &'a [u8] {
        match self.device_token.as_ref() {
            Some(v) => v.as_slice(),
            None => [].as_slice(),
        }
    }

    // optional string device_name = 2;

    pub fn clear_device_name(&mut self) {
        self.device_name.clear();
    }

    pub fn has_device_name(&self) -> bool {
        self.device_name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_device_name(&mut self, v: ::std::string::String) {
        self.device_name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_device_name<'a>(&'a mut self) -> &'a mut ::std::string::String {
        if self.device_name.is_none() {
            self.device_name.set_default();
        };
        self.device_name.as_mut().unwrap()
    }

    // Take field
    pub fn take_device_name(&mut self) -> ::std::string::String {
        self.device_name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_device_name<'a>(&'a self) -> &'a str {
        match self.device_name.as_ref() {
            Some(v) => v.as_slice(),
            None => "",
        }
    }

    // required bytes encrypted_request = 3;

    pub fn clear_encrypted_request(&mut self) {
        self.encrypted_request.clear();
    }

    pub fn has_encrypted_request(&self) -> bool {
        self.encrypted_request.is_some()
    }

    // Param is passed by value, moved
    pub fn set_encrypted_request(&mut self, v: ::std::vec::Vec<u8>) {
        self.encrypted_request = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_encrypted_request<'a>(&'a mut self) -> &'a mut ::std::vec::Vec<u8> {
        if self.encrypted_request.is_none() {
            self.encrypted_request.set_default();
        };
        self.encrypted_request.as_mut().unwrap()
    }

    // Take field
    pub fn take_encrypted_request(&mut self) -> ::std::vec::Vec<u8> {
        self.encrypted_request.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_encrypted_request<'a>(&'a self) -> &'a [u8] {
        match self.encrypted_request.as_ref() {
            Some(v) => v.as_slice(),
            None => [].as_slice(),
        }
    }
}

impl ::protobuf::Message for CMsgRemoteDeviceAuthorizationRequest {
    fn is_initialized(&self) -> bool {
        if self.device_token.is_none() {
            return false;
        };
        if self.encrypted_request.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.device_token.set_default();
                    try!(is.read_bytes_into(tmp))
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.device_name.set_default();
                    try!(is.read_string_into(tmp))
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.encrypted_request.set_default();
                    try!(is.read_bytes_into(tmp))
                },
                _ => {
                    let unknown = try!(is.read_unknown(wire_type));
                    self.mut_unknown_fields().add_value(field_number, unknown);
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.device_token.iter() {
            my_size += ::protobuf::rt::bytes_size(1, value.as_slice());
        };
        for value in self.device_name.iter() {
            my_size += ::protobuf::rt::string_size(2, value.as_slice());
        };
        for value in self.encrypted_request.iter() {
            my_size += ::protobuf::rt::bytes_size(3, value.as_slice());
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.device_token.as_ref() {
            try!(os.write_bytes(1, v.as_slice()));
        };
        if let Some(v) = self.device_name.as_ref() {
            try!(os.write_string(2, v.as_slice()));
        };
        if let Some(v) = self.encrypted_request.as_ref() {
            try!(os.write_bytes(3, v.as_slice()));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields<'s>(&'s self) -> &'s ::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields<'s>(&'s mut self) -> &'s mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::intrinsics::TypeId {
        ::std::intrinsics::TypeId::of::<CMsgRemoteDeviceAuthorizationRequest>()
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CMsgRemoteDeviceAuthorizationRequest {
    fn new() -> CMsgRemoteDeviceAuthorizationRequest {
        CMsgRemoteDeviceAuthorizationRequest::new()
    }

    #[allow(unused_unsafe,unused_mut)]
    fn descriptor_static(_: ::std::option::Option<CMsgRemoteDeviceAuthorizationRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "device_token",
                    CMsgRemoteDeviceAuthorizationRequest::has_device_token,
                    CMsgRemoteDeviceAuthorizationRequest::get_device_token,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "device_name",
                    CMsgRemoteDeviceAuthorizationRequest::has_device_name,
                    CMsgRemoteDeviceAuthorizationRequest::get_device_name,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "encrypted_request",
                    CMsgRemoteDeviceAuthorizationRequest::has_encrypted_request,
                    CMsgRemoteDeviceAuthorizationRequest::get_encrypted_request,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgRemoteDeviceAuthorizationRequest>(
                    "CMsgRemoteDeviceAuthorizationRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgRemoteDeviceAuthorizationRequest {
    fn clear(&mut self) {
        self.clear_device_token();
        self.clear_device_name();
        self.clear_encrypted_request();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for CMsgRemoteDeviceAuthorizationRequest {
    fn eq(&self, other: &CMsgRemoteDeviceAuthorizationRequest) -> bool {
        self.device_token == other.device_token &&
        self.device_name == other.device_name &&
        self.encrypted_request == other.encrypted_request &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Show for CMsgRemoteDeviceAuthorizationRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct CMsgRemoteDeviceAuthorizationRequest_CKeyEscrow_Ticket {
    password: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    identifier: ::std::option::Option<u64>,
    payload: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    timestamp: ::std::option::Option<u32>,
    usage: ::std::option::Option<CMsgRemoteDeviceAuthorizationRequest_EKeyEscrowUsage>,
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl CMsgRemoteDeviceAuthorizationRequest_CKeyEscrow_Ticket {
    pub fn new() -> CMsgRemoteDeviceAuthorizationRequest_CKeyEscrow_Ticket {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgRemoteDeviceAuthorizationRequest_CKeyEscrow_Ticket {
        static mut instance: ::protobuf::lazy::Lazy<CMsgRemoteDeviceAuthorizationRequest_CKeyEscrow_Ticket> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgRemoteDeviceAuthorizationRequest_CKeyEscrow_Ticket,
        };
        unsafe {
            instance.get(|| {
                CMsgRemoteDeviceAuthorizationRequest_CKeyEscrow_Ticket {
                    password: ::protobuf::SingularField::none(),
                    identifier: ::std::option::Option::None,
                    payload: ::protobuf::SingularField::none(),
                    timestamp: ::std::option::Option::None,
                    usage: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional bytes password = 1;

    pub fn clear_password(&mut self) {
        self.password.clear();
    }

    pub fn has_password(&self) -> bool {
        self.password.is_some()
    }

    // Param is passed by value, moved
    pub fn set_password(&mut self, v: ::std::vec::Vec<u8>) {
        self.password = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_password<'a>(&'a mut self) -> &'a mut ::std::vec::Vec<u8> {
        if self.password.is_none() {
            self.password.set_default();
        };
        self.password.as_mut().unwrap()
    }

    // Take field
    pub fn take_password(&mut self) -> ::std::vec::Vec<u8> {
        self.password.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_password<'a>(&'a self) -> &'a [u8] {
        match self.password.as_ref() {
            Some(v) => v.as_slice(),
            None => [].as_slice(),
        }
    }

    // optional uint64 identifier = 2;

    pub fn clear_identifier(&mut self) {
        self.identifier = ::std::option::Option::None;
    }

    pub fn has_identifier(&self) -> bool {
        self.identifier.is_some()
    }

    // Param is passed by value, moved
    pub fn set_identifier(&mut self, v: u64) {
        self.identifier = ::std::option::Option::Some(v);
    }

    pub fn get_identifier<'a>(&self) -> u64 {
        self.identifier.unwrap_or(0)
    }

    // optional bytes payload = 3;

    pub fn clear_payload(&mut self) {
        self.payload.clear();
    }

    pub fn has_payload(&self) -> bool {
        self.payload.is_some()
    }

    // Param is passed by value, moved
    pub fn set_payload(&mut self, v: ::std::vec::Vec<u8>) {
        self.payload = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_payload<'a>(&'a mut self) -> &'a mut ::std::vec::Vec<u8> {
        if self.payload.is_none() {
            self.payload.set_default();
        };
        self.payload.as_mut().unwrap()
    }

    // Take field
    pub fn take_payload(&mut self) -> ::std::vec::Vec<u8> {
        self.payload.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_payload<'a>(&'a self) -> &'a [u8] {
        match self.payload.as_ref() {
            Some(v) => v.as_slice(),
            None => [].as_slice(),
        }
    }

    // optional uint32 timestamp = 4;

    pub fn clear_timestamp(&mut self) {
        self.timestamp = ::std::option::Option::None;
    }

    pub fn has_timestamp(&self) -> bool {
        self.timestamp.is_some()
    }

    // Param is passed by value, moved
    pub fn set_timestamp(&mut self, v: u32) {
        self.timestamp = ::std::option::Option::Some(v);
    }

    pub fn get_timestamp<'a>(&self) -> u32 {
        self.timestamp.unwrap_or(0)
    }

    // optional .CMsgRemoteDeviceAuthorizationRequest.EKeyEscrowUsage usage = 5;

    pub fn clear_usage(&mut self) {
        self.usage = ::std::option::Option::None;
    }

    pub fn has_usage(&self) -> bool {
        self.usage.is_some()
    }

    // Param is passed by value, moved
    pub fn set_usage(&mut self, v: CMsgRemoteDeviceAuthorizationRequest_EKeyEscrowUsage) {
        self.usage = ::std::option::Option::Some(v);
    }

    pub fn get_usage<'a>(&self) -> CMsgRemoteDeviceAuthorizationRequest_EKeyEscrowUsage {
        self.usage.unwrap_or(CMsgRemoteDeviceAuthorizationRequest_EKeyEscrowUsage::k_EKeyEscrowUsageStreamingDevice)
    }
}

impl ::protobuf::Message for CMsgRemoteDeviceAuthorizationRequest_CKeyEscrow_Ticket {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.password.set_default();
                    try!(is.read_bytes_into(tmp))
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_uint64());
                    self.identifier = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.payload.set_default();
                    try!(is.read_bytes_into(tmp))
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_uint32());
                    self.timestamp = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_enum());
                    self.usage = ::std::option::Option::Some(tmp);
                },
                _ => {
                    let unknown = try!(is.read_unknown(wire_type));
                    self.mut_unknown_fields().add_value(field_number, unknown);
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.password.iter() {
            my_size += ::protobuf::rt::bytes_size(1, value.as_slice());
        };
        for value in self.identifier.iter() {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.payload.iter() {
            my_size += ::protobuf::rt::bytes_size(3, value.as_slice());
        };
        for value in self.timestamp.iter() {
            my_size += ::protobuf::rt::value_size(4, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.usage.iter() {
            my_size += ::protobuf::rt::enum_size(5, *value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.password.as_ref() {
            try!(os.write_bytes(1, v.as_slice()));
        };
        if let Some(v) = self.identifier {
            try!(os.write_uint64(2, v));
        };
        if let Some(v) = self.payload.as_ref() {
            try!(os.write_bytes(3, v.as_slice()));
        };
        if let Some(v) = self.timestamp {
            try!(os.write_uint32(4, v));
        };
        if let Some(v) = self.usage {
            try!(os.write_enum(5, v as i32));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields<'s>(&'s self) -> &'s ::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields<'s>(&'s mut self) -> &'s mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::intrinsics::TypeId {
        ::std::intrinsics::TypeId::of::<CMsgRemoteDeviceAuthorizationRequest_CKeyEscrow_Ticket>()
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CMsgRemoteDeviceAuthorizationRequest_CKeyEscrow_Ticket {
    fn new() -> CMsgRemoteDeviceAuthorizationRequest_CKeyEscrow_Ticket {
        CMsgRemoteDeviceAuthorizationRequest_CKeyEscrow_Ticket::new()
    }

    #[allow(unused_unsafe,unused_mut)]
    fn descriptor_static(_: ::std::option::Option<CMsgRemoteDeviceAuthorizationRequest_CKeyEscrow_Ticket>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "password",
                    CMsgRemoteDeviceAuthorizationRequest_CKeyEscrow_Ticket::has_password,
                    CMsgRemoteDeviceAuthorizationRequest_CKeyEscrow_Ticket::get_password,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "identifier",
                    CMsgRemoteDeviceAuthorizationRequest_CKeyEscrow_Ticket::has_identifier,
                    CMsgRemoteDeviceAuthorizationRequest_CKeyEscrow_Ticket::get_identifier,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "payload",
                    CMsgRemoteDeviceAuthorizationRequest_CKeyEscrow_Ticket::has_payload,
                    CMsgRemoteDeviceAuthorizationRequest_CKeyEscrow_Ticket::get_payload,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u32_accessor(
                    "timestamp",
                    CMsgRemoteDeviceAuthorizationRequest_CKeyEscrow_Ticket::has_timestamp,
                    CMsgRemoteDeviceAuthorizationRequest_CKeyEscrow_Ticket::get_timestamp,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_enum_accessor(
                    "usage",
                    CMsgRemoteDeviceAuthorizationRequest_CKeyEscrow_Ticket::has_usage,
                    CMsgRemoteDeviceAuthorizationRequest_CKeyEscrow_Ticket::get_usage,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgRemoteDeviceAuthorizationRequest_CKeyEscrow_Ticket>(
                    "CMsgRemoteDeviceAuthorizationRequest_CKeyEscrow_Ticket",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgRemoteDeviceAuthorizationRequest_CKeyEscrow_Ticket {
    fn clear(&mut self) {
        self.clear_password();
        self.clear_identifier();
        self.clear_payload();
        self.clear_timestamp();
        self.clear_usage();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for CMsgRemoteDeviceAuthorizationRequest_CKeyEscrow_Ticket {
    fn eq(&self, other: &CMsgRemoteDeviceAuthorizationRequest_CKeyEscrow_Ticket) -> bool {
        self.password == other.password &&
        self.identifier == other.identifier &&
        self.payload == other.payload &&
        self.timestamp == other.timestamp &&
        self.usage == other.usage &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Show for CMsgRemoteDeviceAuthorizationRequest_CKeyEscrow_Ticket {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,PartialEq,Eq,Show)]
pub enum CMsgRemoteDeviceAuthorizationRequest_EKeyEscrowUsage {
    k_EKeyEscrowUsageStreamingDevice = 0,
}

impl ::protobuf::ProtobufEnum for CMsgRemoteDeviceAuthorizationRequest_EKeyEscrowUsage {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<CMsgRemoteDeviceAuthorizationRequest_EKeyEscrowUsage> {
        match value {
            0 => ::std::option::Option::Some(CMsgRemoteDeviceAuthorizationRequest_EKeyEscrowUsage::k_EKeyEscrowUsageStreamingDevice),
            _ => ::std::option::Option::None
        }
    }

    fn enum_descriptor_static(_: Option<CMsgRemoteDeviceAuthorizationRequest_EKeyEscrowUsage>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("CMsgRemoteDeviceAuthorizationRequest_EKeyEscrowUsage", file_descriptor_proto())
            })
        }
    }
}

impl ::std::kinds::Copy for CMsgRemoteDeviceAuthorizationRequest_EKeyEscrowUsage {
}

#[derive(Clone,Default)]
pub struct CMsgRemoteDeviceAuthorizationResponse {
    result: ::std::option::Option<ERemoteDeviceAuthorizationResult>,
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl CMsgRemoteDeviceAuthorizationResponse {
    pub fn new() -> CMsgRemoteDeviceAuthorizationResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgRemoteDeviceAuthorizationResponse {
        static mut instance: ::protobuf::lazy::Lazy<CMsgRemoteDeviceAuthorizationResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgRemoteDeviceAuthorizationResponse,
        };
        unsafe {
            instance.get(|| {
                CMsgRemoteDeviceAuthorizationResponse {
                    result: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required .ERemoteDeviceAuthorizationResult result = 1;

    pub fn clear_result(&mut self) {
        self.result = ::std::option::Option::None;
    }

    pub fn has_result(&self) -> bool {
        self.result.is_some()
    }

    // Param is passed by value, moved
    pub fn set_result(&mut self, v: ERemoteDeviceAuthorizationResult) {
        self.result = ::std::option::Option::Some(v);
    }

    pub fn get_result<'a>(&self) -> ERemoteDeviceAuthorizationResult {
        self.result.unwrap_or(ERemoteDeviceAuthorizationResult::k_ERemoteDeviceAuthorizationSuccess)
    }
}

impl ::protobuf::Message for CMsgRemoteDeviceAuthorizationResponse {
    fn is_initialized(&self) -> bool {
        if self.result.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_enum());
                    self.result = ::std::option::Option::Some(tmp);
                },
                _ => {
                    let unknown = try!(is.read_unknown(wire_type));
                    self.mut_unknown_fields().add_value(field_number, unknown);
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.result.iter() {
            my_size += ::protobuf::rt::enum_size(1, *value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.result {
            try!(os.write_enum(1, v as i32));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields<'s>(&'s self) -> &'s ::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields<'s>(&'s mut self) -> &'s mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::intrinsics::TypeId {
        ::std::intrinsics::TypeId::of::<CMsgRemoteDeviceAuthorizationResponse>()
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CMsgRemoteDeviceAuthorizationResponse {
    fn new() -> CMsgRemoteDeviceAuthorizationResponse {
        CMsgRemoteDeviceAuthorizationResponse::new()
    }

    #[allow(unused_unsafe,unused_mut)]
    fn descriptor_static(_: ::std::option::Option<CMsgRemoteDeviceAuthorizationResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_enum_accessor(
                    "result",
                    CMsgRemoteDeviceAuthorizationResponse::has_result,
                    CMsgRemoteDeviceAuthorizationResponse::get_result,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgRemoteDeviceAuthorizationResponse>(
                    "CMsgRemoteDeviceAuthorizationResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgRemoteDeviceAuthorizationResponse {
    fn clear(&mut self) {
        self.clear_result();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for CMsgRemoteDeviceAuthorizationResponse {
    fn eq(&self, other: &CMsgRemoteDeviceAuthorizationResponse) -> bool {
        self.result == other.result &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Show for CMsgRemoteDeviceAuthorizationResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct CMsgRemoteDeviceStreamingRequest {
    request_id: ::std::option::Option<u32>,
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl CMsgRemoteDeviceStreamingRequest {
    pub fn new() -> CMsgRemoteDeviceStreamingRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgRemoteDeviceStreamingRequest {
        static mut instance: ::protobuf::lazy::Lazy<CMsgRemoteDeviceStreamingRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgRemoteDeviceStreamingRequest,
        };
        unsafe {
            instance.get(|| {
                CMsgRemoteDeviceStreamingRequest {
                    request_id: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required uint32 request_id = 1;

    pub fn clear_request_id(&mut self) {
        self.request_id = ::std::option::Option::None;
    }

    pub fn has_request_id(&self) -> bool {
        self.request_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_request_id(&mut self, v: u32) {
        self.request_id = ::std::option::Option::Some(v);
    }

    pub fn get_request_id<'a>(&self) -> u32 {
        self.request_id.unwrap_or(0)
    }
}

impl ::protobuf::Message for CMsgRemoteDeviceStreamingRequest {
    fn is_initialized(&self) -> bool {
        if self.request_id.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_uint32());
                    self.request_id = ::std::option::Option::Some(tmp);
                },
                _ => {
                    let unknown = try!(is.read_unknown(wire_type));
                    self.mut_unknown_fields().add_value(field_number, unknown);
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.request_id.iter() {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.request_id {
            try!(os.write_uint32(1, v));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields<'s>(&'s self) -> &'s ::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields<'s>(&'s mut self) -> &'s mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::intrinsics::TypeId {
        ::std::intrinsics::TypeId::of::<CMsgRemoteDeviceStreamingRequest>()
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CMsgRemoteDeviceStreamingRequest {
    fn new() -> CMsgRemoteDeviceStreamingRequest {
        CMsgRemoteDeviceStreamingRequest::new()
    }

    #[allow(unused_unsafe,unused_mut)]
    fn descriptor_static(_: ::std::option::Option<CMsgRemoteDeviceStreamingRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_u32_accessor(
                    "request_id",
                    CMsgRemoteDeviceStreamingRequest::has_request_id,
                    CMsgRemoteDeviceStreamingRequest::get_request_id,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgRemoteDeviceStreamingRequest>(
                    "CMsgRemoteDeviceStreamingRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgRemoteDeviceStreamingRequest {
    fn clear(&mut self) {
        self.clear_request_id();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for CMsgRemoteDeviceStreamingRequest {
    fn eq(&self, other: &CMsgRemoteDeviceStreamingRequest) -> bool {
        self.request_id == other.request_id &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Show for CMsgRemoteDeviceStreamingRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct CMsgRemoteDeviceStreamingResponse {
    request_id: ::std::option::Option<u32>,
    result: ::std::option::Option<ERemoteDeviceStreamingResult>,
    port: ::std::option::Option<u32>,
    encrypted_session_key: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl CMsgRemoteDeviceStreamingResponse {
    pub fn new() -> CMsgRemoteDeviceStreamingResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgRemoteDeviceStreamingResponse {
        static mut instance: ::protobuf::lazy::Lazy<CMsgRemoteDeviceStreamingResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgRemoteDeviceStreamingResponse,
        };
        unsafe {
            instance.get(|| {
                CMsgRemoteDeviceStreamingResponse {
                    request_id: ::std::option::Option::None,
                    result: ::std::option::Option::None,
                    port: ::std::option::Option::None,
                    encrypted_session_key: ::protobuf::SingularField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required uint32 request_id = 1;

    pub fn clear_request_id(&mut self) {
        self.request_id = ::std::option::Option::None;
    }

    pub fn has_request_id(&self) -> bool {
        self.request_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_request_id(&mut self, v: u32) {
        self.request_id = ::std::option::Option::Some(v);
    }

    pub fn get_request_id<'a>(&self) -> u32 {
        self.request_id.unwrap_or(0)
    }

    // required .ERemoteDeviceStreamingResult result = 2;

    pub fn clear_result(&mut self) {
        self.result = ::std::option::Option::None;
    }

    pub fn has_result(&self) -> bool {
        self.result.is_some()
    }

    // Param is passed by value, moved
    pub fn set_result(&mut self, v: ERemoteDeviceStreamingResult) {
        self.result = ::std::option::Option::Some(v);
    }

    pub fn get_result<'a>(&self) -> ERemoteDeviceStreamingResult {
        self.result.unwrap_or(ERemoteDeviceStreamingResult::k_ERemoteDeviceStreamingSuccess)
    }

    // optional uint32 port = 3;

    pub fn clear_port(&mut self) {
        self.port = ::std::option::Option::None;
    }

    pub fn has_port(&self) -> bool {
        self.port.is_some()
    }

    // Param is passed by value, moved
    pub fn set_port(&mut self, v: u32) {
        self.port = ::std::option::Option::Some(v);
    }

    pub fn get_port<'a>(&self) -> u32 {
        self.port.unwrap_or(0)
    }

    // optional bytes encrypted_session_key = 4;

    pub fn clear_encrypted_session_key(&mut self) {
        self.encrypted_session_key.clear();
    }

    pub fn has_encrypted_session_key(&self) -> bool {
        self.encrypted_session_key.is_some()
    }

    // Param is passed by value, moved
    pub fn set_encrypted_session_key(&mut self, v: ::std::vec::Vec<u8>) {
        self.encrypted_session_key = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_encrypted_session_key<'a>(&'a mut self) -> &'a mut ::std::vec::Vec<u8> {
        if self.encrypted_session_key.is_none() {
            self.encrypted_session_key.set_default();
        };
        self.encrypted_session_key.as_mut().unwrap()
    }

    // Take field
    pub fn take_encrypted_session_key(&mut self) -> ::std::vec::Vec<u8> {
        self.encrypted_session_key.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_encrypted_session_key<'a>(&'a self) -> &'a [u8] {
        match self.encrypted_session_key.as_ref() {
            Some(v) => v.as_slice(),
            None => [].as_slice(),
        }
    }
}

impl ::protobuf::Message for CMsgRemoteDeviceStreamingResponse {
    fn is_initialized(&self) -> bool {
        if self.request_id.is_none() {
            return false;
        };
        if self.result.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_uint32());
                    self.request_id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_enum());
                    self.result = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_uint32());
                    self.port = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.encrypted_session_key.set_default();
                    try!(is.read_bytes_into(tmp))
                },
                _ => {
                    let unknown = try!(is.read_unknown(wire_type));
                    self.mut_unknown_fields().add_value(field_number, unknown);
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.request_id.iter() {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.result.iter() {
            my_size += ::protobuf::rt::enum_size(2, *value);
        };
        for value in self.port.iter() {
            my_size += ::protobuf::rt::value_size(3, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.encrypted_session_key.iter() {
            my_size += ::protobuf::rt::bytes_size(4, value.as_slice());
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.request_id {
            try!(os.write_uint32(1, v));
        };
        if let Some(v) = self.result {
            try!(os.write_enum(2, v as i32));
        };
        if let Some(v) = self.port {
            try!(os.write_uint32(3, v));
        };
        if let Some(v) = self.encrypted_session_key.as_ref() {
            try!(os.write_bytes(4, v.as_slice()));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields<'s>(&'s self) -> &'s ::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields<'s>(&'s mut self) -> &'s mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::intrinsics::TypeId {
        ::std::intrinsics::TypeId::of::<CMsgRemoteDeviceStreamingResponse>()
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CMsgRemoteDeviceStreamingResponse {
    fn new() -> CMsgRemoteDeviceStreamingResponse {
        CMsgRemoteDeviceStreamingResponse::new()
    }

    #[allow(unused_unsafe,unused_mut)]
    fn descriptor_static(_: ::std::option::Option<CMsgRemoteDeviceStreamingResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_u32_accessor(
                    "request_id",
                    CMsgRemoteDeviceStreamingResponse::has_request_id,
                    CMsgRemoteDeviceStreamingResponse::get_request_id,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_enum_accessor(
                    "result",
                    CMsgRemoteDeviceStreamingResponse::has_result,
                    CMsgRemoteDeviceStreamingResponse::get_result,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u32_accessor(
                    "port",
                    CMsgRemoteDeviceStreamingResponse::has_port,
                    CMsgRemoteDeviceStreamingResponse::get_port,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "encrypted_session_key",
                    CMsgRemoteDeviceStreamingResponse::has_encrypted_session_key,
                    CMsgRemoteDeviceStreamingResponse::get_encrypted_session_key,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgRemoteDeviceStreamingResponse>(
                    "CMsgRemoteDeviceStreamingResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgRemoteDeviceStreamingResponse {
    fn clear(&mut self) {
        self.clear_request_id();
        self.clear_result();
        self.clear_port();
        self.clear_encrypted_session_key();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for CMsgRemoteDeviceStreamingResponse {
    fn eq(&self, other: &CMsgRemoteDeviceStreamingResponse) -> bool {
        self.request_id == other.request_id &&
        self.result == other.result &&
        self.port == other.port &&
        self.encrypted_session_key == other.encrypted_session_key &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Show for CMsgRemoteDeviceStreamingResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct CMsgRemoteDeviceProofRequest {
    challenge: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl CMsgRemoteDeviceProofRequest {
    pub fn new() -> CMsgRemoteDeviceProofRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgRemoteDeviceProofRequest {
        static mut instance: ::protobuf::lazy::Lazy<CMsgRemoteDeviceProofRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgRemoteDeviceProofRequest,
        };
        unsafe {
            instance.get(|| {
                CMsgRemoteDeviceProofRequest {
                    challenge: ::protobuf::SingularField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required bytes challenge = 1;

    pub fn clear_challenge(&mut self) {
        self.challenge.clear();
    }

    pub fn has_challenge(&self) -> bool {
        self.challenge.is_some()
    }

    // Param is passed by value, moved
    pub fn set_challenge(&mut self, v: ::std::vec::Vec<u8>) {
        self.challenge = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_challenge<'a>(&'a mut self) -> &'a mut ::std::vec::Vec<u8> {
        if self.challenge.is_none() {
            self.challenge.set_default();
        };
        self.challenge.as_mut().unwrap()
    }

    // Take field
    pub fn take_challenge(&mut self) -> ::std::vec::Vec<u8> {
        self.challenge.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_challenge<'a>(&'a self) -> &'a [u8] {
        match self.challenge.as_ref() {
            Some(v) => v.as_slice(),
            None => [].as_slice(),
        }
    }
}

impl ::protobuf::Message for CMsgRemoteDeviceProofRequest {
    fn is_initialized(&self) -> bool {
        if self.challenge.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.challenge.set_default();
                    try!(is.read_bytes_into(tmp))
                },
                _ => {
                    let unknown = try!(is.read_unknown(wire_type));
                    self.mut_unknown_fields().add_value(field_number, unknown);
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.challenge.iter() {
            my_size += ::protobuf::rt::bytes_size(1, value.as_slice());
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.challenge.as_ref() {
            try!(os.write_bytes(1, v.as_slice()));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields<'s>(&'s self) -> &'s ::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields<'s>(&'s mut self) -> &'s mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::intrinsics::TypeId {
        ::std::intrinsics::TypeId::of::<CMsgRemoteDeviceProofRequest>()
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CMsgRemoteDeviceProofRequest {
    fn new() -> CMsgRemoteDeviceProofRequest {
        CMsgRemoteDeviceProofRequest::new()
    }

    #[allow(unused_unsafe,unused_mut)]
    fn descriptor_static(_: ::std::option::Option<CMsgRemoteDeviceProofRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "challenge",
                    CMsgRemoteDeviceProofRequest::has_challenge,
                    CMsgRemoteDeviceProofRequest::get_challenge,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgRemoteDeviceProofRequest>(
                    "CMsgRemoteDeviceProofRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgRemoteDeviceProofRequest {
    fn clear(&mut self) {
        self.clear_challenge();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for CMsgRemoteDeviceProofRequest {
    fn eq(&self, other: &CMsgRemoteDeviceProofRequest) -> bool {
        self.challenge == other.challenge &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Show for CMsgRemoteDeviceProofRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct CMsgRemoteDeviceProofResponse {
    response: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl CMsgRemoteDeviceProofResponse {
    pub fn new() -> CMsgRemoteDeviceProofResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgRemoteDeviceProofResponse {
        static mut instance: ::protobuf::lazy::Lazy<CMsgRemoteDeviceProofResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgRemoteDeviceProofResponse,
        };
        unsafe {
            instance.get(|| {
                CMsgRemoteDeviceProofResponse {
                    response: ::protobuf::SingularField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required bytes response = 1;

    pub fn clear_response(&mut self) {
        self.response.clear();
    }

    pub fn has_response(&self) -> bool {
        self.response.is_some()
    }

    // Param is passed by value, moved
    pub fn set_response(&mut self, v: ::std::vec::Vec<u8>) {
        self.response = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_response<'a>(&'a mut self) -> &'a mut ::std::vec::Vec<u8> {
        if self.response.is_none() {
            self.response.set_default();
        };
        self.response.as_mut().unwrap()
    }

    // Take field
    pub fn take_response(&mut self) -> ::std::vec::Vec<u8> {
        self.response.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_response<'a>(&'a self) -> &'a [u8] {
        match self.response.as_ref() {
            Some(v) => v.as_slice(),
            None => [].as_slice(),
        }
    }
}

impl ::protobuf::Message for CMsgRemoteDeviceProofResponse {
    fn is_initialized(&self) -> bool {
        if self.response.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.response.set_default();
                    try!(is.read_bytes_into(tmp))
                },
                _ => {
                    let unknown = try!(is.read_unknown(wire_type));
                    self.mut_unknown_fields().add_value(field_number, unknown);
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.response.iter() {
            my_size += ::protobuf::rt::bytes_size(1, value.as_slice());
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.response.as_ref() {
            try!(os.write_bytes(1, v.as_slice()));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields<'s>(&'s self) -> &'s ::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields<'s>(&'s mut self) -> &'s mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::intrinsics::TypeId {
        ::std::intrinsics::TypeId::of::<CMsgRemoteDeviceProofResponse>()
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CMsgRemoteDeviceProofResponse {
    fn new() -> CMsgRemoteDeviceProofResponse {
        CMsgRemoteDeviceProofResponse::new()
    }

    #[allow(unused_unsafe,unused_mut)]
    fn descriptor_static(_: ::std::option::Option<CMsgRemoteDeviceProofResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "response",
                    CMsgRemoteDeviceProofResponse::has_response,
                    CMsgRemoteDeviceProofResponse::get_response,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgRemoteDeviceProofResponse>(
                    "CMsgRemoteDeviceProofResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgRemoteDeviceProofResponse {
    fn clear(&mut self) {
        self.clear_response();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for CMsgRemoteDeviceProofResponse {
    fn eq(&self, other: &CMsgRemoteDeviceProofResponse) -> bool {
        self.response == other.response &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Show for CMsgRemoteDeviceProofResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,PartialEq,Eq,Show)]
pub enum ERemoteClientBroadcastMsg {
    k_ERemoteClientBroadcastMsgDiscovery = 0,
    k_ERemoteClientBroadcastMsgStatus = 1,
    k_ERemoteClientBroadcastMsgOffline = 2,
    k_ERemoteDeviceAuthorizationRequest = 3,
    k_ERemoteDeviceAuthorizationResponse = 4,
    k_ERemoteDeviceStreamingRequest = 5,
    k_ERemoteDeviceStreamingResponse = 6,
    k_ERemoteDeviceProofRequest = 7,
    k_ERemoteDeviceProofResponse = 8,
}

impl ::protobuf::ProtobufEnum for ERemoteClientBroadcastMsg {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<ERemoteClientBroadcastMsg> {
        match value {
            0 => ::std::option::Option::Some(ERemoteClientBroadcastMsg::k_ERemoteClientBroadcastMsgDiscovery),
            1 => ::std::option::Option::Some(ERemoteClientBroadcastMsg::k_ERemoteClientBroadcastMsgStatus),
            2 => ::std::option::Option::Some(ERemoteClientBroadcastMsg::k_ERemoteClientBroadcastMsgOffline),
            3 => ::std::option::Option::Some(ERemoteClientBroadcastMsg::k_ERemoteDeviceAuthorizationRequest),
            4 => ::std::option::Option::Some(ERemoteClientBroadcastMsg::k_ERemoteDeviceAuthorizationResponse),
            5 => ::std::option::Option::Some(ERemoteClientBroadcastMsg::k_ERemoteDeviceStreamingRequest),
            6 => ::std::option::Option::Some(ERemoteClientBroadcastMsg::k_ERemoteDeviceStreamingResponse),
            7 => ::std::option::Option::Some(ERemoteClientBroadcastMsg::k_ERemoteDeviceProofRequest),
            8 => ::std::option::Option::Some(ERemoteClientBroadcastMsg::k_ERemoteDeviceProofResponse),
            _ => ::std::option::Option::None
        }
    }

    fn enum_descriptor_static(_: Option<ERemoteClientBroadcastMsg>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("ERemoteClientBroadcastMsg", file_descriptor_proto())
            })
        }
    }
}

impl ::std::kinds::Copy for ERemoteClientBroadcastMsg {
}

#[derive(Clone,PartialEq,Eq,Show)]
pub enum ERemoteClientService {
    k_ERemoteClientServiceNone = 0,
    k_ERemoteClientServiceRemoteControl = 1,
    k_ERemoteClientServiceGameStreaming = 2,
}

impl ::protobuf::ProtobufEnum for ERemoteClientService {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<ERemoteClientService> {
        match value {
            0 => ::std::option::Option::Some(ERemoteClientService::k_ERemoteClientServiceNone),
            1 => ::std::option::Option::Some(ERemoteClientService::k_ERemoteClientServiceRemoteControl),
            2 => ::std::option::Option::Some(ERemoteClientService::k_ERemoteClientServiceGameStreaming),
            _ => ::std::option::Option::None
        }
    }

    fn enum_descriptor_static(_: Option<ERemoteClientService>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("ERemoteClientService", file_descriptor_proto())
            })
        }
    }
}

impl ::std::kinds::Copy for ERemoteClientService {
}

#[derive(Clone,PartialEq,Eq,Show)]
pub enum ERemoteDeviceAuthorizationResult {
    k_ERemoteDeviceAuthorizationSuccess = 0,
    k_ERemoteDeviceAuthorizationDenied = 1,
    k_ERemoteDeviceAuthorizationNotLoggedIn = 2,
    k_ERemoteDeviceAuthorizationOffline = 3,
    k_ERemoteDeviceAuthorizationBusy = 4,
    k_ERemoteDeviceAuthorizationInProgress = 5,
    k_ERemoteDeviceAuthorizationTimedOut = 6,
    k_ERemoteDeviceAuthorizationFailed = 7,
}

impl ::protobuf::ProtobufEnum for ERemoteDeviceAuthorizationResult {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<ERemoteDeviceAuthorizationResult> {
        match value {
            0 => ::std::option::Option::Some(ERemoteDeviceAuthorizationResult::k_ERemoteDeviceAuthorizationSuccess),
            1 => ::std::option::Option::Some(ERemoteDeviceAuthorizationResult::k_ERemoteDeviceAuthorizationDenied),
            2 => ::std::option::Option::Some(ERemoteDeviceAuthorizationResult::k_ERemoteDeviceAuthorizationNotLoggedIn),
            3 => ::std::option::Option::Some(ERemoteDeviceAuthorizationResult::k_ERemoteDeviceAuthorizationOffline),
            4 => ::std::option::Option::Some(ERemoteDeviceAuthorizationResult::k_ERemoteDeviceAuthorizationBusy),
            5 => ::std::option::Option::Some(ERemoteDeviceAuthorizationResult::k_ERemoteDeviceAuthorizationInProgress),
            6 => ::std::option::Option::Some(ERemoteDeviceAuthorizationResult::k_ERemoteDeviceAuthorizationTimedOut),
            7 => ::std::option::Option::Some(ERemoteDeviceAuthorizationResult::k_ERemoteDeviceAuthorizationFailed),
            _ => ::std::option::Option::None
        }
    }

    fn enum_descriptor_static(_: Option<ERemoteDeviceAuthorizationResult>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("ERemoteDeviceAuthorizationResult", file_descriptor_proto())
            })
        }
    }
}

impl ::std::kinds::Copy for ERemoteDeviceAuthorizationResult {
}

#[derive(Clone,PartialEq,Eq,Show)]
pub enum ERemoteDeviceStreamingResult {
    k_ERemoteDeviceStreamingSuccess = 0,
    k_ERemoteDeviceStreamingUnauthorized = 1,
    k_ERemoteDeviceStreamingScreenLocked = 2,
    k_ERemoteDeviceStreamingFailed = 3,
    k_ERemoteDeviceStreamingBusy = 4,
    k_ERemoteDeviceStreamingInProgress = 5,
}

impl ::protobuf::ProtobufEnum for ERemoteDeviceStreamingResult {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<ERemoteDeviceStreamingResult> {
        match value {
            0 => ::std::option::Option::Some(ERemoteDeviceStreamingResult::k_ERemoteDeviceStreamingSuccess),
            1 => ::std::option::Option::Some(ERemoteDeviceStreamingResult::k_ERemoteDeviceStreamingUnauthorized),
            2 => ::std::option::Option::Some(ERemoteDeviceStreamingResult::k_ERemoteDeviceStreamingScreenLocked),
            3 => ::std::option::Option::Some(ERemoteDeviceStreamingResult::k_ERemoteDeviceStreamingFailed),
            4 => ::std::option::Option::Some(ERemoteDeviceStreamingResult::k_ERemoteDeviceStreamingBusy),
            5 => ::std::option::Option::Some(ERemoteDeviceStreamingResult::k_ERemoteDeviceStreamingInProgress),
            _ => ::std::option::Option::None
        }
    }

    fn enum_descriptor_static(_: Option<ERemoteDeviceStreamingResult>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("ERemoteDeviceStreamingResult", file_descriptor_proto())
            })
        }
    }
}

impl ::std::kinds::Copy for ERemoteDeviceStreamingResult {
}

static file_descriptor_proto_data: &'static [u8] = &[
    0x0a, 0x2a, 0x73, 0x74, 0x65, 0x61, 0x6d, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x73, 0x5f,
    0x72, 0x65, 0x6d, 0x6f, 0x74, 0x65, 0x63, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x5f, 0x64, 0x69, 0x73,
    0x63, 0x6f, 0x76, 0x65, 0x72, 0x79, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x22, 0x9d, 0x01, 0x0a,
    0x1f, 0x43, 0x4d, 0x73, 0x67, 0x52, 0x65, 0x6d, 0x6f, 0x74, 0x65, 0x43, 0x6c, 0x69, 0x65, 0x6e,
    0x74, 0x42, 0x72, 0x6f, 0x61, 0x64, 0x63, 0x61, 0x73, 0x74, 0x48, 0x65, 0x61, 0x64, 0x65, 0x72,
    0x12, 0x11, 0x0a, 0x09, 0x63, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x5f, 0x69, 0x64, 0x18, 0x01, 0x20,
    0x01, 0x28, 0x04, 0x12, 0x52, 0x0a, 0x08, 0x6d, 0x73, 0x67, 0x5f, 0x74, 0x79, 0x70, 0x65, 0x18,
    0x02, 0x20, 0x01, 0x28, 0x0e, 0x32, 0x1a, 0x2e, 0x45, 0x52, 0x65, 0x6d, 0x6f, 0x74, 0x65, 0x43,
    0x6c, 0x69, 0x65, 0x6e, 0x74, 0x42, 0x72, 0x6f, 0x61, 0x64, 0x63, 0x61, 0x73, 0x74, 0x4d, 0x73,
    0x67, 0x3a, 0x24, 0x6b, 0x5f, 0x45, 0x52, 0x65, 0x6d, 0x6f, 0x74, 0x65, 0x43, 0x6c, 0x69, 0x65,
    0x6e, 0x74, 0x42, 0x72, 0x6f, 0x61, 0x64, 0x63, 0x61, 0x73, 0x74, 0x4d, 0x73, 0x67, 0x44, 0x69,
    0x73, 0x63, 0x6f, 0x76, 0x65, 0x72, 0x79, 0x12, 0x13, 0x0a, 0x0b, 0x69, 0x6e, 0x73, 0x74, 0x61,
    0x6e, 0x63, 0x65, 0x5f, 0x69, 0x64, 0x18, 0x03, 0x20, 0x01, 0x28, 0x04, 0x22, 0xdb, 0x02, 0x0a,
    0x1f, 0x43, 0x4d, 0x73, 0x67, 0x52, 0x65, 0x6d, 0x6f, 0x74, 0x65, 0x43, 0x6c, 0x69, 0x65, 0x6e,
    0x74, 0x42, 0x72, 0x6f, 0x61, 0x64, 0x63, 0x61, 0x73, 0x74, 0x53, 0x74, 0x61, 0x74, 0x75, 0x73,
    0x12, 0x0f, 0x0a, 0x07, 0x76, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x18, 0x01, 0x20, 0x01, 0x28,
    0x05, 0x12, 0x13, 0x0a, 0x0b, 0x6d, 0x69, 0x6e, 0x5f, 0x76, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e,
    0x18, 0x02, 0x20, 0x01, 0x28, 0x05, 0x12, 0x14, 0x0a, 0x0c, 0x63, 0x6f, 0x6e, 0x6e, 0x65, 0x63,
    0x74, 0x5f, 0x70, 0x6f, 0x72, 0x74, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0d, 0x12, 0x10, 0x0a, 0x08,
    0x68, 0x6f, 0x73, 0x74, 0x6e, 0x61, 0x6d, 0x65, 0x18, 0x04, 0x20, 0x01, 0x28, 0x09, 0x12, 0x18,
    0x0a, 0x10, 0x65, 0x6e, 0x61, 0x62, 0x6c, 0x65, 0x64, 0x5f, 0x73, 0x65, 0x72, 0x76, 0x69, 0x63,
    0x65, 0x73, 0x18, 0x06, 0x20, 0x01, 0x28, 0x0d, 0x12, 0x11, 0x0a, 0x06, 0x6f, 0x73, 0x74, 0x79,
    0x70, 0x65, 0x18, 0x07, 0x20, 0x01, 0x28, 0x05, 0x3a, 0x01, 0x30, 0x12, 0x16, 0x0a, 0x07, 0x69,
    0x73, 0x36, 0x34, 0x62, 0x69, 0x74, 0x18, 0x08, 0x20, 0x01, 0x28, 0x08, 0x3a, 0x05, 0x66, 0x61,
    0x6c, 0x73, 0x65, 0x12, 0x34, 0x0a, 0x05, 0x75, 0x73, 0x65, 0x72, 0x73, 0x18, 0x09, 0x20, 0x03,
    0x28, 0x0b, 0x32, 0x25, 0x2e, 0x43, 0x4d, 0x73, 0x67, 0x52, 0x65, 0x6d, 0x6f, 0x74, 0x65, 0x43,
    0x6c, 0x69, 0x65, 0x6e, 0x74, 0x42, 0x72, 0x6f, 0x61, 0x64, 0x63, 0x61, 0x73, 0x74, 0x53, 0x74,
    0x61, 0x74, 0x75, 0x73, 0x2e, 0x55, 0x73, 0x65, 0x72, 0x12, 0x1b, 0x0a, 0x13, 0x72, 0x65, 0x6d,
    0x6f, 0x74, 0x65, 0x5f, 0x63, 0x6f, 0x6e, 0x74, 0x72, 0x6f, 0x6c, 0x5f, 0x70, 0x6f, 0x72, 0x74,
    0x18, 0x0a, 0x20, 0x01, 0x28, 0x0d, 0x12, 0x11, 0x0a, 0x09, 0x65, 0x75, 0x6e, 0x69, 0x76, 0x65,
    0x72, 0x73, 0x65, 0x18, 0x0b, 0x20, 0x01, 0x28, 0x05, 0x12, 0x11, 0x0a, 0x09, 0x74, 0x69, 0x6d,
    0x65, 0x73, 0x74, 0x61, 0x6d, 0x70, 0x18, 0x0c, 0x20, 0x01, 0x28, 0x0d, 0x1a, 0x2c, 0x0a, 0x04,
    0x55, 0x73, 0x65, 0x72, 0x12, 0x0f, 0x0a, 0x07, 0x73, 0x74, 0x65, 0x61, 0x6d, 0x69, 0x64, 0x18,
    0x01, 0x20, 0x01, 0x28, 0x06, 0x12, 0x13, 0x0a, 0x0b, 0x61, 0x75, 0x74, 0x68, 0x5f, 0x6b, 0x65,
    0x79, 0x5f, 0x69, 0x64, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0d, 0x22, 0x35, 0x0a, 0x22, 0x43, 0x4d,
    0x73, 0x67, 0x52, 0x65, 0x6d, 0x6f, 0x74, 0x65, 0x43, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x42, 0x72,
    0x6f, 0x61, 0x64, 0x63, 0x61, 0x73, 0x74, 0x44, 0x69, 0x73, 0x63, 0x6f, 0x76, 0x65, 0x72, 0x79,
    0x12, 0x0f, 0x0a, 0x07, 0x73, 0x65, 0x71, 0x5f, 0x6e, 0x75, 0x6d, 0x18, 0x01, 0x20, 0x01, 0x28,
    0x0d, 0x22, 0xed, 0x02, 0x0a, 0x24, 0x43, 0x4d, 0x73, 0x67, 0x52, 0x65, 0x6d, 0x6f, 0x74, 0x65,
    0x44, 0x65, 0x76, 0x69, 0x63, 0x65, 0x41, 0x75, 0x74, 0x68, 0x6f, 0x72, 0x69, 0x7a, 0x61, 0x74,
    0x69, 0x6f, 0x6e, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x12, 0x14, 0x0a, 0x0c, 0x64, 0x65,
    0x76, 0x69, 0x63, 0x65, 0x5f, 0x74, 0x6f, 0x6b, 0x65, 0x6e, 0x18, 0x01, 0x20, 0x02, 0x28, 0x0c,
    0x12, 0x13, 0x0a, 0x0b, 0x64, 0x65, 0x76, 0x69, 0x63, 0x65, 0x5f, 0x6e, 0x61, 0x6d, 0x65, 0x18,
    0x02, 0x20, 0x01, 0x28, 0x09, 0x12, 0x19, 0x0a, 0x11, 0x65, 0x6e, 0x63, 0x72, 0x79, 0x70, 0x74,
    0x65, 0x64, 0x5f, 0x72, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x18, 0x03, 0x20, 0x02, 0x28, 0x0c,
    0x1a, 0xc5, 0x01, 0x0a, 0x11, 0x43, 0x4b, 0x65, 0x79, 0x45, 0x73, 0x63, 0x72, 0x6f, 0x77, 0x5f,
    0x54, 0x69, 0x63, 0x6b, 0x65, 0x74, 0x12, 0x10, 0x0a, 0x08, 0x70, 0x61, 0x73, 0x73, 0x77, 0x6f,
    0x72, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0c, 0x12, 0x12, 0x0a, 0x0a, 0x69, 0x64, 0x65, 0x6e,
    0x74, 0x69, 0x66, 0x69, 0x65, 0x72, 0x18, 0x02, 0x20, 0x01, 0x28, 0x04, 0x12, 0x0f, 0x0a, 0x07,
    0x70, 0x61, 0x79, 0x6c, 0x6f, 0x61, 0x64, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0c, 0x12, 0x11, 0x0a,
    0x09, 0x74, 0x69, 0x6d, 0x65, 0x73, 0x74, 0x61, 0x6d, 0x70, 0x18, 0x04, 0x20, 0x01, 0x28, 0x0d,
    0x12, 0x66, 0x0a, 0x05, 0x75, 0x73, 0x61, 0x67, 0x65, 0x18, 0x05, 0x20, 0x01, 0x28, 0x0e, 0x32,
    0x35, 0x2e, 0x43, 0x4d, 0x73, 0x67, 0x52, 0x65, 0x6d, 0x6f, 0x74, 0x65, 0x44, 0x65, 0x76, 0x69,
    0x63, 0x65, 0x41, 0x75, 0x74, 0x68, 0x6f, 0x72, 0x69, 0x7a, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x52,
    0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x2e, 0x45, 0x4b, 0x65, 0x79, 0x45, 0x73, 0x63, 0x72, 0x6f,
    0x77, 0x55, 0x73, 0x61, 0x67, 0x65, 0x3a, 0x20, 0x6b, 0x5f, 0x45, 0x4b, 0x65, 0x79, 0x45, 0x73,
    0x63, 0x72, 0x6f, 0x77, 0x55, 0x73, 0x61, 0x67, 0x65, 0x53, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x69,
    0x6e, 0x67, 0x44, 0x65, 0x76, 0x69, 0x63, 0x65, 0x22, 0x37, 0x0a, 0x0f, 0x45, 0x4b, 0x65, 0x79,
    0x45, 0x73, 0x63, 0x72, 0x6f, 0x77, 0x55, 0x73, 0x61, 0x67, 0x65, 0x12, 0x24, 0x0a, 0x20, 0x6b,
    0x5f, 0x45, 0x4b, 0x65, 0x79, 0x45, 0x73, 0x63, 0x72, 0x6f, 0x77, 0x55, 0x73, 0x61, 0x67, 0x65,
    0x53, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x69, 0x6e, 0x67, 0x44, 0x65, 0x76, 0x69, 0x63, 0x65, 0x10,
    0x00, 0x22, 0x7f, 0x0a, 0x25, 0x43, 0x4d, 0x73, 0x67, 0x52, 0x65, 0x6d, 0x6f, 0x74, 0x65, 0x44,
    0x65, 0x76, 0x69, 0x63, 0x65, 0x41, 0x75, 0x74, 0x68, 0x6f, 0x72, 0x69, 0x7a, 0x61, 0x74, 0x69,
    0x6f, 0x6e, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x12, 0x56, 0x0a, 0x06, 0x72, 0x65,
    0x73, 0x75, 0x6c, 0x74, 0x18, 0x01, 0x20, 0x02, 0x28, 0x0e, 0x32, 0x21, 0x2e, 0x45, 0x52, 0x65,
    0x6d, 0x6f, 0x74, 0x65, 0x44, 0x65, 0x76, 0x69, 0x63, 0x65, 0x41, 0x75, 0x74, 0x68, 0x6f, 0x72,
    0x69, 0x7a, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x52, 0x65, 0x73, 0x75, 0x6c, 0x74, 0x3a, 0x23, 0x6b,
    0x5f, 0x45, 0x52, 0x65, 0x6d, 0x6f, 0x74, 0x65, 0x44, 0x65, 0x76, 0x69, 0x63, 0x65, 0x41, 0x75,
    0x74, 0x68, 0x6f, 0x72, 0x69, 0x7a, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x53, 0x75, 0x63, 0x63, 0x65,
    0x73, 0x73, 0x22, 0x36, 0x0a, 0x20, 0x43, 0x4d, 0x73, 0x67, 0x52, 0x65, 0x6d, 0x6f, 0x74, 0x65,
    0x44, 0x65, 0x76, 0x69, 0x63, 0x65, 0x53, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x69, 0x6e, 0x67, 0x52,
    0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x12, 0x12, 0x0a, 0x0a, 0x72, 0x65, 0x71, 0x75, 0x65, 0x73,
    0x74, 0x5f, 0x69, 0x64, 0x18, 0x01, 0x20, 0x02, 0x28, 0x0d, 0x22, 0xb4, 0x01, 0x0a, 0x21, 0x43,
    0x4d, 0x73, 0x67, 0x52, 0x65, 0x6d, 0x6f, 0x74, 0x65, 0x44, 0x65, 0x76, 0x69, 0x63, 0x65, 0x53,
    0x74, 0x72, 0x65, 0x61, 0x6d, 0x69, 0x6e, 0x67, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65,
    0x12, 0x12, 0x0a, 0x0a, 0x72, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x5f, 0x69, 0x64, 0x18, 0x01,
    0x20, 0x02, 0x28, 0x0d, 0x12, 0x4e, 0x0a, 0x06, 0x72, 0x65, 0x73, 0x75, 0x6c, 0x74, 0x18, 0x02,
    0x20, 0x02, 0x28, 0x0e, 0x32, 0x1d, 0x2e, 0x45, 0x52, 0x65, 0x6d, 0x6f, 0x74, 0x65, 0x44, 0x65,
    0x76, 0x69, 0x63, 0x65, 0x53, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x69, 0x6e, 0x67, 0x52, 0x65, 0x73,
    0x75, 0x6c, 0x74, 0x3a, 0x1f, 0x6b, 0x5f, 0x45, 0x52, 0x65, 0x6d, 0x6f, 0x74, 0x65, 0x44, 0x65,
    0x76, 0x69, 0x63, 0x65, 0x53, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x69, 0x6e, 0x67, 0x53, 0x75, 0x63,
    0x63, 0x65, 0x73, 0x73, 0x12, 0x0c, 0x0a, 0x04, 0x70, 0x6f, 0x72, 0x74, 0x18, 0x03, 0x20, 0x01,
    0x28, 0x0d, 0x12, 0x1d, 0x0a, 0x15, 0x65, 0x6e, 0x63, 0x72, 0x79, 0x70, 0x74, 0x65, 0x64, 0x5f,
    0x73, 0x65, 0x73, 0x73, 0x69, 0x6f, 0x6e, 0x5f, 0x6b, 0x65, 0x79, 0x18, 0x04, 0x20, 0x01, 0x28,
    0x0c, 0x22, 0x31, 0x0a, 0x1c, 0x43, 0x4d, 0x73, 0x67, 0x52, 0x65, 0x6d, 0x6f, 0x74, 0x65, 0x44,
    0x65, 0x76, 0x69, 0x63, 0x65, 0x50, 0x72, 0x6f, 0x6f, 0x66, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73,
    0x74, 0x12, 0x11, 0x0a, 0x09, 0x63, 0x68, 0x61, 0x6c, 0x6c, 0x65, 0x6e, 0x67, 0x65, 0x18, 0x01,
    0x20, 0x02, 0x28, 0x0c, 0x22, 0x31, 0x0a, 0x1d, 0x43, 0x4d, 0x73, 0x67, 0x52, 0x65, 0x6d, 0x6f,
    0x74, 0x65, 0x44, 0x65, 0x76, 0x69, 0x63, 0x65, 0x50, 0x72, 0x6f, 0x6f, 0x66, 0x52, 0x65, 0x73,
    0x70, 0x6f, 0x6e, 0x73, 0x65, 0x12, 0x10, 0x0a, 0x08, 0x72, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73,
    0x65, 0x18, 0x01, 0x20, 0x02, 0x28, 0x0c, 0x2a, 0xf5, 0x02, 0x0a, 0x19, 0x45, 0x52, 0x65, 0x6d,
    0x6f, 0x74, 0x65, 0x43, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x42, 0x72, 0x6f, 0x61, 0x64, 0x63, 0x61,
    0x73, 0x74, 0x4d, 0x73, 0x67, 0x12, 0x28, 0x0a, 0x24, 0x6b, 0x5f, 0x45, 0x52, 0x65, 0x6d, 0x6f,
    0x74, 0x65, 0x43, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x42, 0x72, 0x6f, 0x61, 0x64, 0x63, 0x61, 0x73,
    0x74, 0x4d, 0x73, 0x67, 0x44, 0x69, 0x73, 0x63, 0x6f, 0x76, 0x65, 0x72, 0x79, 0x10, 0x00, 0x12,
    0x25, 0x0a, 0x21, 0x6b, 0x5f, 0x45, 0x52, 0x65, 0x6d, 0x6f, 0x74, 0x65, 0x43, 0x6c, 0x69, 0x65,
    0x6e, 0x74, 0x42, 0x72, 0x6f, 0x61, 0x64, 0x63, 0x61, 0x73, 0x74, 0x4d, 0x73, 0x67, 0x53, 0x74,
    0x61, 0x74, 0x75, 0x73, 0x10, 0x01, 0x12, 0x26, 0x0a, 0x22, 0x6b, 0x5f, 0x45, 0x52, 0x65, 0x6d,
    0x6f, 0x74, 0x65, 0x43, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x42, 0x72, 0x6f, 0x61, 0x64, 0x63, 0x61,
    0x73, 0x74, 0x4d, 0x73, 0x67, 0x4f, 0x66, 0x66, 0x6c, 0x69, 0x6e, 0x65, 0x10, 0x02, 0x12, 0x27,
    0x0a, 0x23, 0x6b, 0x5f, 0x45, 0x52, 0x65, 0x6d, 0x6f, 0x74, 0x65, 0x44, 0x65, 0x76, 0x69, 0x63,
    0x65, 0x41, 0x75, 0x74, 0x68, 0x6f, 0x72, 0x69, 0x7a, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x52, 0x65,
    0x71, 0x75, 0x65, 0x73, 0x74, 0x10, 0x03, 0x12, 0x28, 0x0a, 0x24, 0x6b, 0x5f, 0x45, 0x52, 0x65,
    0x6d, 0x6f, 0x74, 0x65, 0x44, 0x65, 0x76, 0x69, 0x63, 0x65, 0x41, 0x75, 0x74, 0x68, 0x6f, 0x72,
    0x69, 0x7a, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x10,
    0x04, 0x12, 0x23, 0x0a, 0x1f, 0x6b, 0x5f, 0x45, 0x52, 0x65, 0x6d, 0x6f, 0x74, 0x65, 0x44, 0x65,
    0x76, 0x69, 0x63, 0x65, 0x53, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x69, 0x6e, 0x67, 0x52, 0x65, 0x71,
    0x75, 0x65, 0x73, 0x74, 0x10, 0x05, 0x12, 0x24, 0x0a, 0x20, 0x6b, 0x5f, 0x45, 0x52, 0x65, 0x6d,
    0x6f, 0x74, 0x65, 0x44, 0x65, 0x76, 0x69, 0x63, 0x65, 0x53, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x69,
    0x6e, 0x67, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x10, 0x06, 0x12, 0x1f, 0x0a, 0x1b,
    0x6b, 0x5f, 0x45, 0x52, 0x65, 0x6d, 0x6f, 0x74, 0x65, 0x44, 0x65, 0x76, 0x69, 0x63, 0x65, 0x50,
    0x72, 0x6f, 0x6f, 0x66, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x10, 0x07, 0x12, 0x20, 0x0a,
    0x1c, 0x6b, 0x5f, 0x45, 0x52, 0x65, 0x6d, 0x6f, 0x74, 0x65, 0x44, 0x65, 0x76, 0x69, 0x63, 0x65,
    0x50, 0x72, 0x6f, 0x6f, 0x66, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x10, 0x08, 0x2a,
    0x88, 0x01, 0x0a, 0x14, 0x45, 0x52, 0x65, 0x6d, 0x6f, 0x74, 0x65, 0x43, 0x6c, 0x69, 0x65, 0x6e,
    0x74, 0x53, 0x65, 0x72, 0x76, 0x69, 0x63, 0x65, 0x12, 0x1e, 0x0a, 0x1a, 0x6b, 0x5f, 0x45, 0x52,
    0x65, 0x6d, 0x6f, 0x74, 0x65, 0x43, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x53, 0x65, 0x72, 0x76, 0x69,
    0x63, 0x65, 0x4e, 0x6f, 0x6e, 0x65, 0x10, 0x00, 0x12, 0x27, 0x0a, 0x23, 0x6b, 0x5f, 0x45, 0x52,
    0x65, 0x6d, 0x6f, 0x74, 0x65, 0x43, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x53, 0x65, 0x72, 0x76, 0x69,
    0x63, 0x65, 0x52, 0x65, 0x6d, 0x6f, 0x74, 0x65, 0x43, 0x6f, 0x6e, 0x74, 0x72, 0x6f, 0x6c, 0x10,
    0x01, 0x12, 0x27, 0x0a, 0x23, 0x6b, 0x5f, 0x45, 0x52, 0x65, 0x6d, 0x6f, 0x74, 0x65, 0x43, 0x6c,
    0x69, 0x65, 0x6e, 0x74, 0x53, 0x65, 0x72, 0x76, 0x69, 0x63, 0x65, 0x47, 0x61, 0x6d, 0x65, 0x53,
    0x74, 0x72, 0x65, 0x61, 0x6d, 0x69, 0x6e, 0x67, 0x10, 0x02, 0x2a, 0xed, 0x02, 0x0a, 0x20, 0x45,
    0x52, 0x65, 0x6d, 0x6f, 0x74, 0x65, 0x44, 0x65, 0x76, 0x69, 0x63, 0x65, 0x41, 0x75, 0x74, 0x68,
    0x6f, 0x72, 0x69, 0x7a, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x52, 0x65, 0x73, 0x75, 0x6c, 0x74, 0x12,
    0x27, 0x0a, 0x23, 0x6b, 0x5f, 0x45, 0x52, 0x65, 0x6d, 0x6f, 0x74, 0x65, 0x44, 0x65, 0x76, 0x69,
    0x63, 0x65, 0x41, 0x75, 0x74, 0x68, 0x6f, 0x72, 0x69, 0x7a, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x53,
    0x75, 0x63, 0x63, 0x65, 0x73, 0x73, 0x10, 0x00, 0x12, 0x26, 0x0a, 0x22, 0x6b, 0x5f, 0x45, 0x52,
    0x65, 0x6d, 0x6f, 0x74, 0x65, 0x44, 0x65, 0x76, 0x69, 0x63, 0x65, 0x41, 0x75, 0x74, 0x68, 0x6f,
    0x72, 0x69, 0x7a, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x44, 0x65, 0x6e, 0x69, 0x65, 0x64, 0x10, 0x01,
    0x12, 0x2b, 0x0a, 0x27, 0x6b, 0x5f, 0x45, 0x52, 0x65, 0x6d, 0x6f, 0x74, 0x65, 0x44, 0x65, 0x76,
    0x69, 0x63, 0x65, 0x41, 0x75, 0x74, 0x68, 0x6f, 0x72, 0x69, 0x7a, 0x61, 0x74, 0x69, 0x6f, 0x6e,
    0x4e, 0x6f, 0x74, 0x4c, 0x6f, 0x67, 0x67, 0x65, 0x64, 0x49, 0x6e, 0x10, 0x02, 0x12, 0x27, 0x0a,
    0x23, 0x6b, 0x5f, 0x45, 0x52, 0x65, 0x6d, 0x6f, 0x74, 0x65, 0x44, 0x65, 0x76, 0x69, 0x63, 0x65,
    0x41, 0x75, 0x74, 0x68, 0x6f, 0x72, 0x69, 0x7a, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x4f, 0x66, 0x66,
    0x6c, 0x69, 0x6e, 0x65, 0x10, 0x03, 0x12, 0x24, 0x0a, 0x20, 0x6b, 0x5f, 0x45, 0x52, 0x65, 0x6d,
    0x6f, 0x74, 0x65, 0x44, 0x65, 0x76, 0x69, 0x63, 0x65, 0x41, 0x75, 0x74, 0x68, 0x6f, 0x72, 0x69,
    0x7a, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x42, 0x75, 0x73, 0x79, 0x10, 0x04, 0x12, 0x2a, 0x0a, 0x26,
    0x6b, 0x5f, 0x45, 0x52, 0x65, 0x6d, 0x6f, 0x74, 0x65, 0x44, 0x65, 0x76, 0x69, 0x63, 0x65, 0x41,
    0x75, 0x74, 0x68, 0x6f, 0x72, 0x69, 0x7a, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x49, 0x6e, 0x50, 0x72,
    0x6f, 0x67, 0x72, 0x65, 0x73, 0x73, 0x10, 0x05, 0x12, 0x28, 0x0a, 0x24, 0x6b, 0x5f, 0x45, 0x52,
    0x65, 0x6d, 0x6f, 0x74, 0x65, 0x44, 0x65, 0x76, 0x69, 0x63, 0x65, 0x41, 0x75, 0x74, 0x68, 0x6f,
    0x72, 0x69, 0x7a, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x54, 0x69, 0x6d, 0x65, 0x64, 0x4f, 0x75, 0x74,
    0x10, 0x06, 0x12, 0x26, 0x0a, 0x22, 0x6b, 0x5f, 0x45, 0x52, 0x65, 0x6d, 0x6f, 0x74, 0x65, 0x44,
    0x65, 0x76, 0x69, 0x63, 0x65, 0x41, 0x75, 0x74, 0x68, 0x6f, 0x72, 0x69, 0x7a, 0x61, 0x74, 0x69,
    0x6f, 0x6e, 0x46, 0x61, 0x69, 0x6c, 0x65, 0x64, 0x10, 0x07, 0x2a, 0x85, 0x02, 0x0a, 0x1c, 0x45,
    0x52, 0x65, 0x6d, 0x6f, 0x74, 0x65, 0x44, 0x65, 0x76, 0x69, 0x63, 0x65, 0x53, 0x74, 0x72, 0x65,
    0x61, 0x6d, 0x69, 0x6e, 0x67, 0x52, 0x65, 0x73, 0x75, 0x6c, 0x74, 0x12, 0x23, 0x0a, 0x1f, 0x6b,
    0x5f, 0x45, 0x52, 0x65, 0x6d, 0x6f, 0x74, 0x65, 0x44, 0x65, 0x76, 0x69, 0x63, 0x65, 0x53, 0x74,
    0x72, 0x65, 0x61, 0x6d, 0x69, 0x6e, 0x67, 0x53, 0x75, 0x63, 0x63, 0x65, 0x73, 0x73, 0x10, 0x00,
    0x12, 0x28, 0x0a, 0x24, 0x6b, 0x5f, 0x45, 0x52, 0x65, 0x6d, 0x6f, 0x74, 0x65, 0x44, 0x65, 0x76,
    0x69, 0x63, 0x65, 0x53, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x69, 0x6e, 0x67, 0x55, 0x6e, 0x61, 0x75,
    0x74, 0x68, 0x6f, 0x72, 0x69, 0x7a, 0x65, 0x64, 0x10, 0x01, 0x12, 0x28, 0x0a, 0x24, 0x6b, 0x5f,
    0x45, 0x52, 0x65, 0x6d, 0x6f, 0x74, 0x65, 0x44, 0x65, 0x76, 0x69, 0x63, 0x65, 0x53, 0x74, 0x72,
    0x65, 0x61, 0x6d, 0x69, 0x6e, 0x67, 0x53, 0x63, 0x72, 0x65, 0x65, 0x6e, 0x4c, 0x6f, 0x63, 0x6b,
    0x65, 0x64, 0x10, 0x02, 0x12, 0x22, 0x0a, 0x1e, 0x6b, 0x5f, 0x45, 0x52, 0x65, 0x6d, 0x6f, 0x74,
    0x65, 0x44, 0x65, 0x76, 0x69, 0x63, 0x65, 0x53, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x69, 0x6e, 0x67,
    0x46, 0x61, 0x69, 0x6c, 0x65, 0x64, 0x10, 0x03, 0x12, 0x20, 0x0a, 0x1c, 0x6b, 0x5f, 0x45, 0x52,
    0x65, 0x6d, 0x6f, 0x74, 0x65, 0x44, 0x65, 0x76, 0x69, 0x63, 0x65, 0x53, 0x74, 0x72, 0x65, 0x61,
    0x6d, 0x69, 0x6e, 0x67, 0x42, 0x75, 0x73, 0x79, 0x10, 0x04, 0x12, 0x26, 0x0a, 0x22, 0x6b, 0x5f,
    0x45, 0x52, 0x65, 0x6d, 0x6f, 0x74, 0x65, 0x44, 0x65, 0x76, 0x69, 0x63, 0x65, 0x53, 0x74, 0x72,
    0x65, 0x61, 0x6d, 0x69, 0x6e, 0x67, 0x49, 0x6e, 0x50, 0x72, 0x6f, 0x67, 0x72, 0x65, 0x73, 0x73,
    0x10, 0x05, 0x42, 0x02, 0x48, 0x01, 0x4a, 0x99, 0x20, 0x0a, 0x06, 0x12, 0x04, 0x00, 0x00, 0x6c,
    0x01, 0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12, 0x03, 0x00, 0x00, 0x1c, 0x0a, 0x0b, 0x0a, 0x04, 0x08,
    0xe7, 0x07, 0x00, 0x12, 0x03, 0x00, 0x00, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x08, 0xe7, 0x07, 0x00,
    0x02, 0x12, 0x03, 0x00, 0x07, 0x13, 0x0a, 0x0d, 0x0a, 0x06, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00,
    0x12, 0x03, 0x00, 0x07, 0x13, 0x0a, 0x0e, 0x0a, 0x07, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x01,
    0x12, 0x03, 0x00, 0x07, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x08, 0xe7, 0x07, 0x00, 0x03, 0x12, 0x03,
    0x00, 0x16, 0x1b, 0x0a, 0x0a, 0x0a, 0x02, 0x05, 0x00, 0x12, 0x04, 0x02, 0x00, 0x0c, 0x01, 0x0a,
    0x0a, 0x0a, 0x03, 0x05, 0x00, 0x01, 0x12, 0x03, 0x02, 0x05, 0x1e, 0x0a, 0x0b, 0x0a, 0x04, 0x05,
    0x00, 0x02, 0x00, 0x12, 0x03, 0x03, 0x08, 0x31, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x00,
    0x01, 0x12, 0x03, 0x03, 0x08, 0x2c, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x00, 0x02, 0x12,
    0x03, 0x03, 0x2f, 0x30, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x01, 0x12, 0x03, 0x04, 0x08,
    0x2e, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x04, 0x08, 0x29, 0x0a,
    0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x01, 0x02, 0x12, 0x03, 0x04, 0x2c, 0x2d, 0x0a, 0x0b, 0x0a,
    0x04, 0x05, 0x00, 0x02, 0x02, 0x12, 0x03, 0x05, 0x08, 0x2f, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00,
    0x02, 0x02, 0x01, 0x12, 0x03, 0x05, 0x08, 0x2a, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x02,
    0x02, 0x12, 0x03, 0x05, 0x2d, 0x2e, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x03, 0x12, 0x03,
    0x06, 0x08, 0x30, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x03, 0x01, 0x12, 0x03, 0x06, 0x08,
    0x2b, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x03, 0x02, 0x12, 0x03, 0x06, 0x2e, 0x2f, 0x0a,
    0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x04, 0x12, 0x03, 0x07, 0x08, 0x31, 0x0a, 0x0c, 0x0a, 0x05,
    0x05, 0x00, 0x02, 0x04, 0x01, 0x12, 0x03, 0x07, 0x08, 0x2c, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00,
    0x02, 0x04, 0x02, 0x12, 0x03, 0x07, 0x2f, 0x30, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x05,
    0x12, 0x03, 0x08, 0x08, 0x2c, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x05, 0x01, 0x12, 0x03,
    0x08, 0x08, 0x27, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x05, 0x02, 0x12, 0x03, 0x08, 0x2a,
    0x2b, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x06, 0x12, 0x03, 0x09, 0x08, 0x2d, 0x0a, 0x0c,
    0x0a, 0x05, 0x05, 0x00, 0x02, 0x06, 0x01, 0x12, 0x03, 0x09, 0x08, 0x28, 0x0a, 0x0c, 0x0a, 0x05,
    0x05, 0x00, 0x02, 0x06, 0x02, 0x12, 0x03, 0x09, 0x2b, 0x2c, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00,
    0x02, 0x07, 0x12, 0x03, 0x0a, 0x08, 0x28, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x07, 0x01,
    0x12, 0x03, 0x0a, 0x08, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x07, 0x02, 0x12, 0x03,
    0x0a, 0x26, 0x27, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x08, 0x12, 0x03, 0x0b, 0x08, 0x29,
    0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x08, 0x01, 0x12, 0x03, 0x0b, 0x08, 0x24, 0x0a, 0x0c,
    0x0a, 0x05, 0x05, 0x00, 0x02, 0x08, 0x02, 0x12, 0x03, 0x0b, 0x27, 0x28, 0x0a, 0x0a, 0x0a, 0x02,
    0x05, 0x01, 0x12, 0x04, 0x0e, 0x00, 0x12, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x05, 0x01, 0x01, 0x12,
    0x03, 0x0e, 0x05, 0x19, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x01, 0x02, 0x00, 0x12, 0x03, 0x0f, 0x08,
    0x27, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x01, 0x02, 0x00, 0x01, 0x12, 0x03, 0x0f, 0x08, 0x22, 0x0a,
    0x0c, 0x0a, 0x05, 0x05, 0x01, 0x02, 0x00, 0x02, 0x12, 0x03, 0x0f, 0x25, 0x26, 0x0a, 0x0b, 0x0a,
    0x04, 0x05, 0x01, 0x02, 0x01, 0x12, 0x03, 0x10, 0x08, 0x30, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x01,
    0x02, 0x01, 0x01, 0x12, 0x03, 0x10, 0x08, 0x2b, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x01, 0x02, 0x01,
    0x02, 0x12, 0x03, 0x10, 0x2e, 0x2f, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x01, 0x02, 0x02, 0x12, 0x03,
    0x11, 0x08, 0x30, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x01, 0x02, 0x02, 0x01, 0x12, 0x03, 0x11, 0x08,
    0x2b, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x01, 0x02, 0x02, 0x02, 0x12, 0x03, 0x11, 0x2e, 0x2f, 0x0a,
    0x0a, 0x0a, 0x02, 0x05, 0x02, 0x12, 0x04, 0x14, 0x00, 0x1d, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x05,
    0x02, 0x01, 0x12, 0x03, 0x14, 0x05, 0x25, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x02, 0x02, 0x00, 0x12,
    0x03, 0x15, 0x08, 0x30, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x02, 0x02, 0x00, 0x01, 0x12, 0x03, 0x15,
    0x08, 0x2b, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x02, 0x02, 0x00, 0x02, 0x12, 0x03, 0x15, 0x2e, 0x2f,
    0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x02, 0x02, 0x01, 0x12, 0x03, 0x16, 0x08, 0x2f, 0x0a, 0x0c, 0x0a,
    0x05, 0x05, 0x02, 0x02, 0x01, 0x01, 0x12, 0x03, 0x16, 0x08, 0x2a, 0x0a, 0x0c, 0x0a, 0x05, 0x05,
    0x02, 0x02, 0x01, 0x02, 0x12, 0x03, 0x16, 0x2d, 0x2e, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x02, 0x02,
    0x02, 0x12, 0x03, 0x17, 0x08, 0x34, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x02, 0x02, 0x02, 0x01, 0x12,
    0x03, 0x17, 0x08, 0x2f, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x02, 0x02, 0x02, 0x02, 0x12, 0x03, 0x17,
    0x32, 0x33, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x02, 0x02, 0x03, 0x12, 0x03, 0x18, 0x08, 0x30, 0x0a,
    0x0c, 0x0a, 0x05, 0x05, 0x02, 0x02, 0x03, 0x01, 0x12, 0x03, 0x18, 0x08, 0x2b, 0x0a, 0x0c, 0x0a,
    0x05, 0x05, 0x02, 0x02, 0x03, 0x02, 0x12, 0x03, 0x18, 0x2e, 0x2f, 0x0a, 0x0b, 0x0a, 0x04, 0x05,
    0x02, 0x02, 0x04, 0x12, 0x03, 0x19, 0x08, 0x2d, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x02, 0x02, 0x04,
    0x01, 0x12, 0x03, 0x19, 0x08, 0x28, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x02, 0x02, 0x04, 0x02, 0x12,
    0x03, 0x19, 0x2b, 0x2c, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x02, 0x02, 0x05, 0x12, 0x03, 0x1a, 0x08,
    0x33, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x02, 0x02, 0x05, 0x01, 0x12, 0x03, 0x1a, 0x08, 0x2e, 0x0a,
    0x0c, 0x0a, 0x05, 0x05, 0x02, 0x02, 0x05, 0x02, 0x12, 0x03, 0x1a, 0x31, 0x32, 0x0a, 0x0b, 0x0a,
    0x04, 0x05, 0x02, 0x02, 0x06, 0x12, 0x03, 0x1b, 0x08, 0x31, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x02,
    0x02, 0x06, 0x01, 0x12, 0x03, 0x1b, 0x08, 0x2c, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x02, 0x02, 0x06,
    0x02, 0x12, 0x03, 0x1b, 0x2f, 0x30, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x02, 0x02, 0x07, 0x12, 0x03,
    0x1c, 0x08, 0x2f, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x02, 0x02, 0x07, 0x01, 0x12, 0x03, 0x1c, 0x08,
    0x2a, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x02, 0x02, 0x07, 0x02, 0x12, 0x03, 0x1c, 0x2d, 0x2e, 0x0a,
    0x0a, 0x0a, 0x02, 0x05, 0x03, 0x12, 0x04, 0x1f, 0x00, 0x26, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x05,
    0x03, 0x01, 0x12, 0x03, 0x1f, 0x05, 0x21, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x03, 0x02, 0x00, 0x12,
    0x03, 0x20, 0x08, 0x2c, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x03, 0x02, 0x00, 0x01, 0x12, 0x03, 0x20,
    0x08, 0x27, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x03, 0x02, 0x00, 0x02, 0x12, 0x03, 0x20, 0x2a, 0x2b,
    0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x03, 0x02, 0x01, 0x12, 0x03, 0x21, 0x08, 0x31, 0x0a, 0x0c, 0x0a,
    0x05, 0x05, 0x03, 0x02, 0x01, 0x01, 0x12, 0x03, 0x21, 0x08, 0x2c, 0x0a, 0x0c, 0x0a, 0x05, 0x05,
    0x03, 0x02, 0x01, 0x02, 0x12, 0x03, 0x21, 0x2f, 0x30, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x03, 0x02,
    0x02, 0x12, 0x03, 0x22, 0x08, 0x31, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x03, 0x02, 0x02, 0x01, 0x12,
    0x03, 0x22, 0x08, 0x2c, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x03, 0x02, 0x02, 0x02, 0x12, 0x03, 0x22,
    0x2f, 0x30, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x03, 0x02, 0x03, 0x12, 0x03, 0x23, 0x08, 0x2b, 0x0a,
    0x0c, 0x0a, 0x05, 0x05, 0x03, 0x02, 0x03, 0x01, 0x12, 0x03, 0x23, 0x08, 0x26, 0x0a, 0x0c, 0x0a,
    0x05, 0x05, 0x03, 0x02, 0x03, 0x02, 0x12, 0x03, 0x23, 0x29, 0x2a, 0x0a, 0x0b, 0x0a, 0x04, 0x05,
    0x03, 0x02, 0x04, 0x12, 0x03, 0x24, 0x08, 0x29, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x03, 0x02, 0x04,
    0x01, 0x12, 0x03, 0x24, 0x08, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x03, 0x02, 0x04, 0x02, 0x12,
    0x03, 0x24, 0x27, 0x28, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x03, 0x02, 0x05, 0x12, 0x03, 0x25, 0x08,
    0x2f, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x03, 0x02, 0x05, 0x01, 0x12, 0x03, 0x25, 0x08, 0x2a, 0x0a,
    0x0c, 0x0a, 0x05, 0x05, 0x03, 0x02, 0x05, 0x02, 0x12, 0x03, 0x25, 0x2d, 0x2e, 0x0a, 0x0a, 0x0a,
    0x02, 0x04, 0x00, 0x12, 0x04, 0x28, 0x00, 0x2c, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00, 0x01,
    0x12, 0x03, 0x28, 0x08, 0x27, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x29,
    0x08, 0x26, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x04, 0x12, 0x03, 0x29, 0x08, 0x10,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x05, 0x12, 0x03, 0x29, 0x11, 0x17, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x29, 0x18, 0x21, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x29, 0x24, 0x25, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00,
    0x02, 0x01, 0x12, 0x03, 0x2a, 0x08, 0x6a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x04,
    0x12, 0x03, 0x2a, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x06, 0x12, 0x03,
    0x2a, 0x11, 0x2b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x2a, 0x2c,
    0x34, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x03, 0x12, 0x03, 0x2a, 0x37, 0x38, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x08, 0x12, 0x03, 0x2a, 0x39, 0x69, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x01, 0x07, 0x12, 0x03, 0x2a, 0x44, 0x68, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x00, 0x02, 0x02, 0x12, 0x03, 0x2b, 0x08, 0x28, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02,
    0x04, 0x12, 0x03, 0x2b, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x05, 0x12,
    0x03, 0x2b, 0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x01, 0x12, 0x03, 0x2b,
    0x18, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x03, 0x12, 0x03, 0x2b, 0x26, 0x27,
    0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x01, 0x12, 0x04, 0x2e, 0x00, 0x3f, 0x01, 0x0a, 0x0a, 0x0a, 0x03,
    0x04, 0x01, 0x01, 0x12, 0x03, 0x2e, 0x08, 0x27, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x01, 0x03, 0x00,
    0x12, 0x04, 0x2f, 0x08, 0x32, 0x09, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x03, 0x00, 0x01, 0x12,
    0x03, 0x2f, 0x10, 0x14, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x01, 0x03, 0x00, 0x02, 0x00, 0x12, 0x03,
    0x30, 0x10, 0x2d, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x01, 0x03, 0x00, 0x02, 0x00, 0x04, 0x12, 0x03,
    0x30, 0x10, 0x18, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x01, 0x03, 0x00, 0x02, 0x00, 0x05, 0x12, 0x03,
    0x30, 0x19, 0x20, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x01, 0x03, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03,
    0x30, 0x21, 0x28, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x01, 0x03, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03,
    0x30, 0x2b, 0x2c, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x01, 0x03, 0x00, 0x02, 0x01, 0x12, 0x03, 0x31,
    0x10, 0x30, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x01, 0x03, 0x00, 0x02, 0x01, 0x04, 0x12, 0x03, 0x31,
    0x10, 0x18, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x01, 0x03, 0x00, 0x02, 0x01, 0x05, 0x12, 0x03, 0x31,
    0x19, 0x1f, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x01, 0x03, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x31,
    0x20, 0x2b, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x01, 0x03, 0x00, 0x02, 0x01, 0x03, 0x12, 0x03, 0x31,
    0x2e, 0x2f, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x00, 0x12, 0x03, 0x34, 0x08, 0x23, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x04, 0x12, 0x03, 0x34, 0x08, 0x10, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x01, 0x02, 0x00, 0x05, 0x12, 0x03, 0x34, 0x11, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x01, 0x02, 0x00, 0x01, 0x12, 0x03, 0x34, 0x17, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02,
    0x00, 0x03, 0x12, 0x03, 0x34, 0x21, 0x22, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x01, 0x12,
    0x03, 0x35, 0x08, 0x27, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x04, 0x12, 0x03, 0x35,
    0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x05, 0x12, 0x03, 0x35, 0x11, 0x16,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x01, 0x12, 0x03, 0x35, 0x17, 0x22, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x03, 0x12, 0x03, 0x35, 0x25, 0x26, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x01, 0x02, 0x02, 0x12, 0x03, 0x36, 0x08, 0x29, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02,
    0x02, 0x04, 0x12, 0x03, 0x36, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x02, 0x05,
    0x12, 0x03, 0x36, 0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x02, 0x01, 0x12, 0x03,
    0x36, 0x18, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x02, 0x03, 0x12, 0x03, 0x36, 0x27,
    0x28, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x03, 0x12, 0x03, 0x37, 0x08, 0x25, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x01, 0x02, 0x03, 0x04, 0x12, 0x03, 0x37, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x01, 0x02, 0x03, 0x05, 0x12, 0x03, 0x37, 0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01,
    0x02, 0x03, 0x01, 0x12, 0x03, 0x37, 0x18, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x03,
    0x03, 0x12, 0x03, 0x37, 0x23, 0x24, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x04, 0x12, 0x03,
    0x38, 0x08, 0x2d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x04, 0x04, 0x12, 0x03, 0x38, 0x08,
    0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x04, 0x05, 0x12, 0x03, 0x38, 0x11, 0x17, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x04, 0x01, 0x12, 0x03, 0x38, 0x18, 0x28, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x01, 0x02, 0x04, 0x03, 0x12, 0x03, 0x38, 0x2b, 0x2c, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x01, 0x02, 0x05, 0x12, 0x03, 0x39, 0x08, 0x30, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x05,
    0x04, 0x12, 0x03, 0x39, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x05, 0x05, 0x12,
    0x03, 0x39, 0x11, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x05, 0x01, 0x12, 0x03, 0x39,
    0x17, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x05, 0x03, 0x12, 0x03, 0x39, 0x20, 0x21,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x05, 0x08, 0x12, 0x03, 0x39, 0x22, 0x2f, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x01, 0x02, 0x05, 0x07, 0x12, 0x03, 0x39, 0x2d, 0x2e, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x01, 0x02, 0x06, 0x12, 0x03, 0x3a, 0x08, 0x34, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02,
    0x06, 0x04, 0x12, 0x03, 0x3a, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x06, 0x05,
    0x12, 0x03, 0x3a, 0x11, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x06, 0x01, 0x12, 0x03,
    0x3a, 0x16, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x06, 0x03, 0x12, 0x03, 0x3a, 0x20,
    0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x06, 0x08, 0x12, 0x03, 0x3a, 0x22, 0x33, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x06, 0x07, 0x12, 0x03, 0x3a, 0x2d, 0x32, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x01, 0x02, 0x07, 0x12, 0x03, 0x3b, 0x08, 0x41, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01,
    0x02, 0x07, 0x04, 0x12, 0x03, 0x3b, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x07,
    0x06, 0x12, 0x03, 0x3b, 0x11, 0x36, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x07, 0x01, 0x12,
    0x03, 0x3b, 0x37, 0x3c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x07, 0x03, 0x12, 0x03, 0x3b,
    0x3f, 0x40, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x08, 0x12, 0x03, 0x3c, 0x08, 0x31, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x08, 0x04, 0x12, 0x03, 0x3c, 0x08, 0x10, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x01, 0x02, 0x08, 0x05, 0x12, 0x03, 0x3c, 0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x01, 0x02, 0x08, 0x01, 0x12, 0x03, 0x3c, 0x18, 0x2b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02,
    0x08, 0x03, 0x12, 0x03, 0x3c, 0x2e, 0x30, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x09, 0x12,
    0x03, 0x3d, 0x08, 0x26, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x09, 0x04, 0x12, 0x03, 0x3d,
    0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x09, 0x05, 0x12, 0x03, 0x3d, 0x11, 0x16,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x09, 0x01, 0x12, 0x03, 0x3d, 0x17, 0x20, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x01, 0x02, 0x09, 0x03, 0x12, 0x03, 0x3d, 0x23, 0x25, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x01, 0x02, 0x0a, 0x12, 0x03, 0x3e, 0x08, 0x27, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02,
    0x0a, 0x04, 0x12, 0x03, 0x3e, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x0a, 0x05,
    0x12, 0x03, 0x3e, 0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x0a, 0x01, 0x12, 0x03,
    0x3e, 0x18, 0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x0a, 0x03, 0x12, 0x03, 0x3e, 0x24,
    0x26, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x02, 0x12, 0x04, 0x41, 0x00, 0x43, 0x01, 0x0a, 0x0a, 0x0a,
    0x03, 0x04, 0x02, 0x01, 0x12, 0x03, 0x41, 0x08, 0x2a, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02,
    0x00, 0x12, 0x03, 0x42, 0x08, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x04, 0x12,
    0x03, 0x42, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x05, 0x12, 0x03, 0x42,
    0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x01, 0x12, 0x03, 0x42, 0x18, 0x1f,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x03, 0x12, 0x03, 0x42, 0x22, 0x23, 0x0a, 0x0a,
    0x0a, 0x02, 0x04, 0x03, 0x12, 0x04, 0x45, 0x00, 0x55, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x03,
    0x01, 0x12, 0x03, 0x45, 0x08, 0x2c, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x03, 0x03, 0x00, 0x12, 0x04,
    0x46, 0x08, 0x4c, 0x09, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x03, 0x00, 0x01, 0x12, 0x03, 0x46,
    0x10, 0x21, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x03, 0x03, 0x00, 0x02, 0x00, 0x12, 0x03, 0x47, 0x10,
    0x2c, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x03, 0x03, 0x00, 0x02, 0x00, 0x04, 0x12, 0x03, 0x47, 0x10,
    0x18, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x03, 0x03, 0x00, 0x02, 0x00, 0x05, 0x12, 0x03, 0x47, 0x19,
    0x1e, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x03, 0x03, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x47, 0x1f,
    0x27, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x03, 0x03, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x47, 0x2a,
    0x2b, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x03, 0x03, 0x00, 0x02, 0x01, 0x12, 0x03, 0x48, 0x10, 0x2f,
    0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x03, 0x03, 0x00, 0x02, 0x01, 0x04, 0x12, 0x03, 0x48, 0x10, 0x18,
    0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x03, 0x03, 0x00, 0x02, 0x01, 0x05, 0x12, 0x03, 0x48, 0x19, 0x1f,
    0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x03, 0x03, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x48, 0x20, 0x2a,
    0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x03, 0x03, 0x00, 0x02, 0x01, 0x03, 0x12, 0x03, 0x48, 0x2d, 0x2e,
    0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x03, 0x03, 0x00, 0x02, 0x02, 0x12, 0x03, 0x49, 0x10, 0x2b, 0x0a,
    0x0e, 0x0a, 0x07, 0x04, 0x03, 0x03, 0x00, 0x02, 0x02, 0x04, 0x12, 0x03, 0x49, 0x10, 0x18, 0x0a,
    0x0e, 0x0a, 0x07, 0x04, 0x03, 0x03, 0x00, 0x02, 0x02, 0x05, 0x12, 0x03, 0x49, 0x19, 0x1e, 0x0a,
    0x0e, 0x0a, 0x07, 0x04, 0x03, 0x03, 0x00, 0x02, 0x02, 0x01, 0x12, 0x03, 0x49, 0x1f, 0x26, 0x0a,
    0x0e, 0x0a, 0x07, 0x04, 0x03, 0x03, 0x00, 0x02, 0x02, 0x03, 0x12, 0x03, 0x49, 0x29, 0x2a, 0x0a,
    0x0d, 0x0a, 0x06, 0x04, 0x03, 0x03, 0x00, 0x02, 0x03, 0x12, 0x03, 0x4a, 0x10, 0x2e, 0x0a, 0x0e,
    0x0a, 0x07, 0x04, 0x03, 0x03, 0x00, 0x02, 0x03, 0x04, 0x12, 0x03, 0x4a, 0x10, 0x18, 0x0a, 0x0e,
    0x0a, 0x07, 0x04, 0x03, 0x03, 0x00, 0x02, 0x03, 0x05, 0x12, 0x03, 0x4a, 0x19, 0x1f, 0x0a, 0x0e,
    0x0a, 0x07, 0x04, 0x03, 0x03, 0x00, 0x02, 0x03, 0x01, 0x12, 0x03, 0x4a, 0x20, 0x29, 0x0a, 0x0e,
    0x0a, 0x07, 0x04, 0x03, 0x03, 0x00, 0x02, 0x03, 0x03, 0x12, 0x03, 0x4a, 0x2c, 0x2d, 0x0a, 0x0e,
    0x0a, 0x06, 0x04, 0x03, 0x03, 0x00, 0x02, 0x04, 0x12, 0x04, 0x4b, 0x10, 0x86, 0x01, 0x0a, 0x0e,
    0x0a, 0x07, 0x04, 0x03, 0x03, 0x00, 0x02, 0x04, 0x04, 0x12, 0x03, 0x4b, 0x10, 0x18, 0x0a, 0x0e,
    0x0a, 0x07, 0x04, 0x03, 0x03, 0x00, 0x02, 0x04, 0x06, 0x12, 0x03, 0x4b, 0x19, 0x4e, 0x0a, 0x0e,
    0x0a, 0x07, 0x04, 0x03, 0x03, 0x00, 0x02, 0x04, 0x01, 0x12, 0x03, 0x4b, 0x4f, 0x54, 0x0a, 0x0e,
    0x0a, 0x07, 0x04, 0x03, 0x03, 0x00, 0x02, 0x04, 0x03, 0x12, 0x03, 0x4b, 0x57, 0x58, 0x0a, 0x0f,
    0x0a, 0x07, 0x04, 0x03, 0x03, 0x00, 0x02, 0x04, 0x08, 0x12, 0x04, 0x4b, 0x59, 0x85, 0x01, 0x0a,
    0x0f, 0x0a, 0x07, 0x04, 0x03, 0x03, 0x00, 0x02, 0x04, 0x07, 0x12, 0x04, 0x4b, 0x64, 0x84, 0x01,
    0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x03, 0x04, 0x00, 0x12, 0x04, 0x4e, 0x08, 0x50, 0x09, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x03, 0x04, 0x00, 0x01, 0x12, 0x03, 0x4e, 0x0d, 0x1c, 0x0a, 0x0d, 0x0a, 0x06,
    0x04, 0x03, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x4f, 0x10, 0x35, 0x0a, 0x0e, 0x0a, 0x07, 0x04,
    0x03, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x4f, 0x10, 0x30, 0x0a, 0x0e, 0x0a, 0x07, 0x04,
    0x03, 0x04, 0x00, 0x02, 0x00, 0x02, 0x12, 0x03, 0x4f, 0x33, 0x34, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x03, 0x02, 0x00, 0x12, 0x03, 0x52, 0x08, 0x28, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00,
    0x04, 0x12, 0x03, 0x52, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x05, 0x12,
    0x03, 0x52, 0x11, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x01, 0x12, 0x03, 0x52,
    0x17, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x03, 0x12, 0x03, 0x52, 0x26, 0x27,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x01, 0x12, 0x03, 0x53, 0x08, 0x28, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x03, 0x02, 0x01, 0x04, 0x12, 0x03, 0x53, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x03, 0x02, 0x01, 0x05, 0x12, 0x03, 0x53, 0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02,
    0x01, 0x01, 0x12, 0x03, 0x53, 0x18, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x01, 0x03,
    0x12, 0x03, 0x53, 0x26, 0x27, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x02, 0x12, 0x03, 0x54,
    0x08, 0x2d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x02, 0x04, 0x12, 0x03, 0x54, 0x08, 0x10,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x02, 0x05, 0x12, 0x03, 0x54, 0x11, 0x16, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x03, 0x02, 0x02, 0x01, 0x12, 0x03, 0x54, 0x17, 0x28, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x03, 0x02, 0x02, 0x03, 0x12, 0x03, 0x54, 0x2b, 0x2c, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x04,
    0x12, 0x04, 0x57, 0x00, 0x59, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x04, 0x01, 0x12, 0x03, 0x57,
    0x08, 0x2d, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x04, 0x02, 0x00, 0x12, 0x03, 0x58, 0x08, 0x6e, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x04, 0x12, 0x03, 0x58, 0x08, 0x10, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x04, 0x02, 0x00, 0x06, 0x12, 0x03, 0x58, 0x11, 0x32, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x04, 0x02, 0x00, 0x01, 0x12, 0x03, 0x58, 0x33, 0x39, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02,
    0x00, 0x03, 0x12, 0x03, 0x58, 0x3c, 0x3d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x08,
    0x12, 0x03, 0x58, 0x3e, 0x6d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x07, 0x12, 0x03,
    0x58, 0x49, 0x6c, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x05, 0x12, 0x04, 0x5b, 0x00, 0x5d, 0x01, 0x0a,
    0x0a, 0x0a, 0x03, 0x04, 0x05, 0x01, 0x12, 0x03, 0x5b, 0x08, 0x28, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x05, 0x02, 0x00, 0x12, 0x03, 0x5c, 0x08, 0x27, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00,
    0x04, 0x12, 0x03, 0x5c, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x05, 0x12,
    0x03, 0x5c, 0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x01, 0x12, 0x03, 0x5c,
    0x18, 0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x03, 0x12, 0x03, 0x5c, 0x25, 0x26,
    0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x06, 0x12, 0x04, 0x5f, 0x00, 0x64, 0x01, 0x0a, 0x0a, 0x0a, 0x03,
    0x04, 0x06, 0x01, 0x12, 0x03, 0x5f, 0x08, 0x29, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x06, 0x02, 0x00,
    0x12, 0x03, 0x60, 0x08, 0x27, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x00, 0x04, 0x12, 0x03,
    0x60, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x00, 0x05, 0x12, 0x03, 0x60, 0x11,
    0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x00, 0x01, 0x12, 0x03, 0x60, 0x18, 0x22, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x00, 0x03, 0x12, 0x03, 0x60, 0x25, 0x26, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x06, 0x02, 0x01, 0x12, 0x03, 0x61, 0x08, 0x66, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06,
    0x02, 0x01, 0x04, 0x12, 0x03, 0x61, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x01,
    0x06, 0x12, 0x03, 0x61, 0x11, 0x2e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x01, 0x01, 0x12,
    0x03, 0x61, 0x2f, 0x35, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x01, 0x03, 0x12, 0x03, 0x61,
    0x38, 0x39, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x01, 0x08, 0x12, 0x03, 0x61, 0x3a, 0x65,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x01, 0x07, 0x12, 0x03, 0x61, 0x45, 0x64, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x06, 0x02, 0x02, 0x12, 0x03, 0x62, 0x08, 0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x06, 0x02, 0x02, 0x04, 0x12, 0x03, 0x62, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02,
    0x02, 0x05, 0x12, 0x03, 0x62, 0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x02, 0x01,
    0x12, 0x03, 0x62, 0x18, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x02, 0x03, 0x12, 0x03,
    0x62, 0x1f, 0x20, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x06, 0x02, 0x03, 0x12, 0x03, 0x63, 0x08, 0x31,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x03, 0x04, 0x12, 0x03, 0x63, 0x08, 0x10, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x06, 0x02, 0x03, 0x05, 0x12, 0x03, 0x63, 0x11, 0x16, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x06, 0x02, 0x03, 0x01, 0x12, 0x03, 0x63, 0x17, 0x2c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06,
    0x02, 0x03, 0x03, 0x12, 0x03, 0x63, 0x2f, 0x30, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x07, 0x12, 0x04,
    0x66, 0x00, 0x68, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x07, 0x01, 0x12, 0x03, 0x66, 0x08, 0x24,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x07, 0x02, 0x00, 0x12, 0x03, 0x67, 0x08, 0x25, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x07, 0x02, 0x00, 0x04, 0x12, 0x03, 0x67, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x07, 0x02, 0x00, 0x05, 0x12, 0x03, 0x67, 0x11, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02,
    0x00, 0x01, 0x12, 0x03, 0x67, 0x17, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x00, 0x03,
    0x12, 0x03, 0x67, 0x23, 0x24, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x08, 0x12, 0x04, 0x6a, 0x00, 0x6c,
    0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x08, 0x01, 0x12, 0x03, 0x6a, 0x08, 0x25, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x08, 0x02, 0x00, 0x12, 0x03, 0x6b, 0x08, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08,
    0x02, 0x00, 0x04, 0x12, 0x03, 0x6b, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x00,
    0x05, 0x12, 0x03, 0x6b, 0x11, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x00, 0x01, 0x12,
    0x03, 0x6b, 0x17, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x00, 0x03, 0x12, 0x03, 0x6b,
    0x22, 0x23,
];

static mut file_descriptor_proto_lazy: ::protobuf::lazy::Lazy<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::lazy::Lazy {
    lock: ::protobuf::lazy::ONCE_INIT,
    ptr: 0 as *const ::protobuf::descriptor::FileDescriptorProto,
};

fn parse_descriptor_proto() -> ::protobuf::descriptor::FileDescriptorProto {
    ::protobuf::parse_from_bytes(file_descriptor_proto_data).unwrap()
}

pub fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    unsafe {
        file_descriptor_proto_lazy.get(|| {
            parse_descriptor_proto()
        })
    }
}
