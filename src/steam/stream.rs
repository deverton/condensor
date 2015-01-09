// This file is generated. Do not edit

#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(unused_imports)]

use protobuf::Message as Message_imported_for_functions;
use protobuf::ProtobufEnum as ProtobufEnum_imported_for_functions;

#[derive(Clone,Default)]
pub struct CDiscoveryRequest {
    token: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl CDiscoveryRequest {
    pub fn new() -> CDiscoveryRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CDiscoveryRequest {
        static mut instance: ::protobuf::lazy::Lazy<CDiscoveryRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CDiscoveryRequest,
        };
        unsafe {
            instance.get(|| {
                CDiscoveryRequest {
                    token: ::protobuf::SingularField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional bytes token = 1;

    pub fn clear_token(&mut self) {
        self.token.clear();
    }

    pub fn has_token(&self) -> bool {
        self.token.is_some()
    }

    // Param is passed by value, moved
    pub fn set_token(&mut self, v: ::std::vec::Vec<u8>) {
        self.token = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_token<'a>(&'a mut self) -> &'a mut ::std::vec::Vec<u8> {
        if self.token.is_none() {
            self.token.set_default();
        };
        self.token.as_mut().unwrap()
    }

    // Take field
    pub fn take_token(&mut self) -> ::std::vec::Vec<u8> {
        self.token.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_token<'a>(&'a self) -> &'a [u8] {
        match self.token.as_ref() {
            Some(v) => v.as_slice(),
            None => [].as_slice(),
        }
    }
}

impl ::protobuf::Message for CDiscoveryRequest {
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
                    let tmp = self.token.set_default();
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
        for value in self.token.iter() {
            my_size += ::protobuf::rt::bytes_size(1, value.as_slice());
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.token.as_ref() {
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
        ::std::intrinsics::TypeId::of::<CDiscoveryRequest>()
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CDiscoveryRequest {
    fn new() -> CDiscoveryRequest {
        CDiscoveryRequest::new()
    }

    #[allow(unused_unsafe,unused_mut)]
    fn descriptor_static(_: ::std::option::Option<CDiscoveryRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "token",
                    CDiscoveryRequest::has_token,
                    CDiscoveryRequest::get_token,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CDiscoveryRequest>(
                    "CDiscoveryRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CDiscoveryRequest {
    fn clear(&mut self) {
        self.clear_token();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for CDiscoveryRequest {
    fn eq(&self, other: &CDiscoveryRequest) -> bool {
        self.token == other.token &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Show for CDiscoveryRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct CDiscoveryResponse {
    name: ::protobuf::SingularField<::std::string::String>,
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl CDiscoveryResponse {
    pub fn new() -> CDiscoveryResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CDiscoveryResponse {
        static mut instance: ::protobuf::lazy::Lazy<CDiscoveryResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CDiscoveryResponse,
        };
        unsafe {
            instance.get(|| {
                CDiscoveryResponse {
                    name: ::protobuf::SingularField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional string name = 1;

    pub fn clear_name(&mut self) {
        self.name.clear();
    }

    pub fn has_name(&self) -> bool {
        self.name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_name(&mut self, v: ::std::string::String) {
        self.name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_name<'a>(&'a mut self) -> &'a mut ::std::string::String {
        if self.name.is_none() {
            self.name.set_default();
        };
        self.name.as_mut().unwrap()
    }

    // Take field
    pub fn take_name(&mut self) -> ::std::string::String {
        self.name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_name<'a>(&'a self) -> &'a str {
        match self.name.as_ref() {
            Some(v) => v.as_slice(),
            None => "",
        }
    }
}

impl ::protobuf::Message for CDiscoveryResponse {
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
                    let tmp = self.name.set_default();
                    try!(is.read_string_into(tmp))
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
        for value in self.name.iter() {
            my_size += ::protobuf::rt::string_size(1, value.as_slice());
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.name.as_ref() {
            try!(os.write_string(1, v.as_slice()));
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
        ::std::intrinsics::TypeId::of::<CDiscoveryResponse>()
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CDiscoveryResponse {
    fn new() -> CDiscoveryResponse {
        CDiscoveryResponse::new()
    }

    #[allow(unused_unsafe,unused_mut)]
    fn descriptor_static(_: ::std::option::Option<CDiscoveryResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "name",
                    CDiscoveryResponse::has_name,
                    CDiscoveryResponse::get_name,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CDiscoveryResponse>(
                    "CDiscoveryResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CDiscoveryResponse {
    fn clear(&mut self) {
        self.clear_name();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for CDiscoveryResponse {
    fn eq(&self, other: &CDiscoveryResponse) -> bool {
        self.name == other.name &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Show for CDiscoveryResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct CAuthenticationRequestMsg {
    token: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    version: ::std::option::Option<EStreamVersion>,
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl CAuthenticationRequestMsg {
    pub fn new() -> CAuthenticationRequestMsg {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CAuthenticationRequestMsg {
        static mut instance: ::protobuf::lazy::Lazy<CAuthenticationRequestMsg> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CAuthenticationRequestMsg,
        };
        unsafe {
            instance.get(|| {
                CAuthenticationRequestMsg {
                    token: ::protobuf::SingularField::none(),
                    version: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional bytes token = 1;

    pub fn clear_token(&mut self) {
        self.token.clear();
    }

    pub fn has_token(&self) -> bool {
        self.token.is_some()
    }

    // Param is passed by value, moved
    pub fn set_token(&mut self, v: ::std::vec::Vec<u8>) {
        self.token = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_token<'a>(&'a mut self) -> &'a mut ::std::vec::Vec<u8> {
        if self.token.is_none() {
            self.token.set_default();
        };
        self.token.as_mut().unwrap()
    }

    // Take field
    pub fn take_token(&mut self) -> ::std::vec::Vec<u8> {
        self.token.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_token<'a>(&'a self) -> &'a [u8] {
        match self.token.as_ref() {
            Some(v) => v.as_slice(),
            None => [].as_slice(),
        }
    }

    // optional .EStreamVersion version = 2;

    pub fn clear_version(&mut self) {
        self.version = ::std::option::Option::None;
    }

    pub fn has_version(&self) -> bool {
        self.version.is_some()
    }

    // Param is passed by value, moved
    pub fn set_version(&mut self, v: EStreamVersion) {
        self.version = ::std::option::Option::Some(v);
    }

    pub fn get_version<'a>(&self) -> EStreamVersion {
        self.version.unwrap_or(EStreamVersion::k_EStreamVersionNone)
    }
}

impl ::protobuf::Message for CAuthenticationRequestMsg {
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
                    let tmp = self.token.set_default();
                    try!(is.read_bytes_into(tmp))
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_enum());
                    self.version = ::std::option::Option::Some(tmp);
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
        for value in self.token.iter() {
            my_size += ::protobuf::rt::bytes_size(1, value.as_slice());
        };
        for value in self.version.iter() {
            my_size += ::protobuf::rt::enum_size(2, *value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.token.as_ref() {
            try!(os.write_bytes(1, v.as_slice()));
        };
        if let Some(v) = self.version {
            try!(os.write_enum(2, v as i32));
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
        ::std::intrinsics::TypeId::of::<CAuthenticationRequestMsg>()
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CAuthenticationRequestMsg {
    fn new() -> CAuthenticationRequestMsg {
        CAuthenticationRequestMsg::new()
    }

    #[allow(unused_unsafe,unused_mut)]
    fn descriptor_static(_: ::std::option::Option<CAuthenticationRequestMsg>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "token",
                    CAuthenticationRequestMsg::has_token,
                    CAuthenticationRequestMsg::get_token,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_enum_accessor(
                    "version",
                    CAuthenticationRequestMsg::has_version,
                    CAuthenticationRequestMsg::get_version,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CAuthenticationRequestMsg>(
                    "CAuthenticationRequestMsg",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CAuthenticationRequestMsg {
    fn clear(&mut self) {
        self.clear_token();
        self.clear_version();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for CAuthenticationRequestMsg {
    fn eq(&self, other: &CAuthenticationRequestMsg) -> bool {
        self.token == other.token &&
        self.version == other.version &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Show for CAuthenticationRequestMsg {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct CAuthenticationResponseMsg {
    result: ::std::option::Option<CAuthenticationResponseMsg_AuthenticationResult>,
    version: ::std::option::Option<EStreamVersion>,
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl CAuthenticationResponseMsg {
    pub fn new() -> CAuthenticationResponseMsg {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CAuthenticationResponseMsg {
        static mut instance: ::protobuf::lazy::Lazy<CAuthenticationResponseMsg> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CAuthenticationResponseMsg,
        };
        unsafe {
            instance.get(|| {
                CAuthenticationResponseMsg {
                    result: ::std::option::Option::None,
                    version: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional .CAuthenticationResponseMsg.AuthenticationResult result = 1;

    pub fn clear_result(&mut self) {
        self.result = ::std::option::Option::None;
    }

    pub fn has_result(&self) -> bool {
        self.result.is_some()
    }

    // Param is passed by value, moved
    pub fn set_result(&mut self, v: CAuthenticationResponseMsg_AuthenticationResult) {
        self.result = ::std::option::Option::Some(v);
    }

    pub fn get_result<'a>(&self) -> CAuthenticationResponseMsg_AuthenticationResult {
        self.result.unwrap_or(CAuthenticationResponseMsg_AuthenticationResult::SUCCEEDED)
    }

    // optional .EStreamVersion version = 2;

    pub fn clear_version(&mut self) {
        self.version = ::std::option::Option::None;
    }

    pub fn has_version(&self) -> bool {
        self.version.is_some()
    }

    // Param is passed by value, moved
    pub fn set_version(&mut self, v: EStreamVersion) {
        self.version = ::std::option::Option::Some(v);
    }

    pub fn get_version<'a>(&self) -> EStreamVersion {
        self.version.unwrap_or(EStreamVersion::k_EStreamVersionNone)
    }
}

impl ::protobuf::Message for CAuthenticationResponseMsg {
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
                    let tmp = try!(is.read_enum());
                    self.result = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_enum());
                    self.version = ::std::option::Option::Some(tmp);
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
        for value in self.version.iter() {
            my_size += ::protobuf::rt::enum_size(2, *value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.result {
            try!(os.write_enum(1, v as i32));
        };
        if let Some(v) = self.version {
            try!(os.write_enum(2, v as i32));
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
        ::std::intrinsics::TypeId::of::<CAuthenticationResponseMsg>()
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CAuthenticationResponseMsg {
    fn new() -> CAuthenticationResponseMsg {
        CAuthenticationResponseMsg::new()
    }

    #[allow(unused_unsafe,unused_mut)]
    fn descriptor_static(_: ::std::option::Option<CAuthenticationResponseMsg>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_enum_accessor(
                    "result",
                    CAuthenticationResponseMsg::has_result,
                    CAuthenticationResponseMsg::get_result,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_enum_accessor(
                    "version",
                    CAuthenticationResponseMsg::has_version,
                    CAuthenticationResponseMsg::get_version,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CAuthenticationResponseMsg>(
                    "CAuthenticationResponseMsg",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CAuthenticationResponseMsg {
    fn clear(&mut self) {
        self.clear_result();
        self.clear_version();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for CAuthenticationResponseMsg {
    fn eq(&self, other: &CAuthenticationResponseMsg) -> bool {
        self.result == other.result &&
        self.version == other.version &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Show for CAuthenticationResponseMsg {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,PartialEq,Eq,Show)]
pub enum CAuthenticationResponseMsg_AuthenticationResult {
    SUCCEEDED = 0,
    FAILED = 1,
}

impl ::protobuf::ProtobufEnum for CAuthenticationResponseMsg_AuthenticationResult {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<CAuthenticationResponseMsg_AuthenticationResult> {
        match value {
            0 => ::std::option::Option::Some(CAuthenticationResponseMsg_AuthenticationResult::SUCCEEDED),
            1 => ::std::option::Option::Some(CAuthenticationResponseMsg_AuthenticationResult::FAILED),
            _ => ::std::option::Option::None
        }
    }

    fn enum_descriptor_static(_: Option<CAuthenticationResponseMsg_AuthenticationResult>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("CAuthenticationResponseMsg_AuthenticationResult", file_descriptor_proto())
            })
        }
    }
}

impl ::std::kinds::Copy for CAuthenticationResponseMsg_AuthenticationResult {
}

#[derive(Clone,Default)]
pub struct CStreamVideoMode {
    width: ::std::option::Option<u32>,
    height: ::std::option::Option<u32>,
    refresh_rate: ::std::option::Option<u32>,
    refresh_rate_numerator: ::std::option::Option<u32>,
    refresh_rate_denominator: ::std::option::Option<u32>,
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl CStreamVideoMode {
    pub fn new() -> CStreamVideoMode {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CStreamVideoMode {
        static mut instance: ::protobuf::lazy::Lazy<CStreamVideoMode> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CStreamVideoMode,
        };
        unsafe {
            instance.get(|| {
                CStreamVideoMode {
                    width: ::std::option::Option::None,
                    height: ::std::option::Option::None,
                    refresh_rate: ::std::option::Option::None,
                    refresh_rate_numerator: ::std::option::Option::None,
                    refresh_rate_denominator: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required uint32 width = 1;

    pub fn clear_width(&mut self) {
        self.width = ::std::option::Option::None;
    }

    pub fn has_width(&self) -> bool {
        self.width.is_some()
    }

    // Param is passed by value, moved
    pub fn set_width(&mut self, v: u32) {
        self.width = ::std::option::Option::Some(v);
    }

    pub fn get_width<'a>(&self) -> u32 {
        self.width.unwrap_or(0)
    }

    // required uint32 height = 2;

    pub fn clear_height(&mut self) {
        self.height = ::std::option::Option::None;
    }

    pub fn has_height(&self) -> bool {
        self.height.is_some()
    }

    // Param is passed by value, moved
    pub fn set_height(&mut self, v: u32) {
        self.height = ::std::option::Option::Some(v);
    }

    pub fn get_height<'a>(&self) -> u32 {
        self.height.unwrap_or(0)
    }

    // optional uint32 refresh_rate = 3;

    pub fn clear_refresh_rate(&mut self) {
        self.refresh_rate = ::std::option::Option::None;
    }

    pub fn has_refresh_rate(&self) -> bool {
        self.refresh_rate.is_some()
    }

    // Param is passed by value, moved
    pub fn set_refresh_rate(&mut self, v: u32) {
        self.refresh_rate = ::std::option::Option::Some(v);
    }

    pub fn get_refresh_rate<'a>(&self) -> u32 {
        self.refresh_rate.unwrap_or(0)
    }

    // optional uint32 refresh_rate_numerator = 4;

    pub fn clear_refresh_rate_numerator(&mut self) {
        self.refresh_rate_numerator = ::std::option::Option::None;
    }

    pub fn has_refresh_rate_numerator(&self) -> bool {
        self.refresh_rate_numerator.is_some()
    }

    // Param is passed by value, moved
    pub fn set_refresh_rate_numerator(&mut self, v: u32) {
        self.refresh_rate_numerator = ::std::option::Option::Some(v);
    }

    pub fn get_refresh_rate_numerator<'a>(&self) -> u32 {
        self.refresh_rate_numerator.unwrap_or(0)
    }

    // optional uint32 refresh_rate_denominator = 5;

    pub fn clear_refresh_rate_denominator(&mut self) {
        self.refresh_rate_denominator = ::std::option::Option::None;
    }

    pub fn has_refresh_rate_denominator(&self) -> bool {
        self.refresh_rate_denominator.is_some()
    }

    // Param is passed by value, moved
    pub fn set_refresh_rate_denominator(&mut self, v: u32) {
        self.refresh_rate_denominator = ::std::option::Option::Some(v);
    }

    pub fn get_refresh_rate_denominator<'a>(&self) -> u32 {
        self.refresh_rate_denominator.unwrap_or(0)
    }
}

impl ::protobuf::Message for CStreamVideoMode {
    fn is_initialized(&self) -> bool {
        if self.width.is_none() {
            return false;
        };
        if self.height.is_none() {
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
                    self.width = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_uint32());
                    self.height = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_uint32());
                    self.refresh_rate = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_uint32());
                    self.refresh_rate_numerator = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_uint32());
                    self.refresh_rate_denominator = ::std::option::Option::Some(tmp);
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
        for value in self.width.iter() {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.height.iter() {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.refresh_rate.iter() {
            my_size += ::protobuf::rt::value_size(3, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.refresh_rate_numerator.iter() {
            my_size += ::protobuf::rt::value_size(4, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.refresh_rate_denominator.iter() {
            my_size += ::protobuf::rt::value_size(5, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.width {
            try!(os.write_uint32(1, v));
        };
        if let Some(v) = self.height {
            try!(os.write_uint32(2, v));
        };
        if let Some(v) = self.refresh_rate {
            try!(os.write_uint32(3, v));
        };
        if let Some(v) = self.refresh_rate_numerator {
            try!(os.write_uint32(4, v));
        };
        if let Some(v) = self.refresh_rate_denominator {
            try!(os.write_uint32(5, v));
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
        ::std::intrinsics::TypeId::of::<CStreamVideoMode>()
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CStreamVideoMode {
    fn new() -> CStreamVideoMode {
        CStreamVideoMode::new()
    }

    #[allow(unused_unsafe,unused_mut)]
    fn descriptor_static(_: ::std::option::Option<CStreamVideoMode>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_u32_accessor(
                    "width",
                    CStreamVideoMode::has_width,
                    CStreamVideoMode::get_width,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u32_accessor(
                    "height",
                    CStreamVideoMode::has_height,
                    CStreamVideoMode::get_height,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u32_accessor(
                    "refresh_rate",
                    CStreamVideoMode::has_refresh_rate,
                    CStreamVideoMode::get_refresh_rate,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u32_accessor(
                    "refresh_rate_numerator",
                    CStreamVideoMode::has_refresh_rate_numerator,
                    CStreamVideoMode::get_refresh_rate_numerator,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u32_accessor(
                    "refresh_rate_denominator",
                    CStreamVideoMode::has_refresh_rate_denominator,
                    CStreamVideoMode::get_refresh_rate_denominator,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CStreamVideoMode>(
                    "CStreamVideoMode",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CStreamVideoMode {
    fn clear(&mut self) {
        self.clear_width();
        self.clear_height();
        self.clear_refresh_rate();
        self.clear_refresh_rate_numerator();
        self.clear_refresh_rate_denominator();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for CStreamVideoMode {
    fn eq(&self, other: &CStreamVideoMode) -> bool {
        self.width == other.width &&
        self.height == other.height &&
        self.refresh_rate == other.refresh_rate &&
        self.refresh_rate_numerator == other.refresh_rate_numerator &&
        self.refresh_rate_denominator == other.refresh_rate_denominator &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Show for CStreamVideoMode {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct CNegotiatedConfig {
    reliable_data: ::std::option::Option<bool>,
    selected_audio_codec: ::std::option::Option<EStreamAudioCodec>,
    selected_video_codec: ::std::option::Option<EStreamVideoCodec>,
    available_video_modes: ::protobuf::RepeatedField<CStreamVideoMode>,
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl CNegotiatedConfig {
    pub fn new() -> CNegotiatedConfig {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CNegotiatedConfig {
        static mut instance: ::protobuf::lazy::Lazy<CNegotiatedConfig> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CNegotiatedConfig,
        };
        unsafe {
            instance.get(|| {
                CNegotiatedConfig {
                    reliable_data: ::std::option::Option::None,
                    selected_audio_codec: ::std::option::Option::None,
                    selected_video_codec: ::std::option::Option::None,
                    available_video_modes: ::protobuf::RepeatedField::new(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional bool reliable_data = 1;

    pub fn clear_reliable_data(&mut self) {
        self.reliable_data = ::std::option::Option::None;
    }

    pub fn has_reliable_data(&self) -> bool {
        self.reliable_data.is_some()
    }

    // Param is passed by value, moved
    pub fn set_reliable_data(&mut self, v: bool) {
        self.reliable_data = ::std::option::Option::Some(v);
    }

    pub fn get_reliable_data<'a>(&self) -> bool {
        self.reliable_data.unwrap_or(false)
    }

    // optional .EStreamAudioCodec selected_audio_codec = 2;

    pub fn clear_selected_audio_codec(&mut self) {
        self.selected_audio_codec = ::std::option::Option::None;
    }

    pub fn has_selected_audio_codec(&self) -> bool {
        self.selected_audio_codec.is_some()
    }

    // Param is passed by value, moved
    pub fn set_selected_audio_codec(&mut self, v: EStreamAudioCodec) {
        self.selected_audio_codec = ::std::option::Option::Some(v);
    }

    pub fn get_selected_audio_codec<'a>(&self) -> EStreamAudioCodec {
        self.selected_audio_codec.unwrap_or(EStreamAudioCodec::k_EStreamAudioCodecNone)
    }

    // optional .EStreamVideoCodec selected_video_codec = 3;

    pub fn clear_selected_video_codec(&mut self) {
        self.selected_video_codec = ::std::option::Option::None;
    }

    pub fn has_selected_video_codec(&self) -> bool {
        self.selected_video_codec.is_some()
    }

    // Param is passed by value, moved
    pub fn set_selected_video_codec(&mut self, v: EStreamVideoCodec) {
        self.selected_video_codec = ::std::option::Option::Some(v);
    }

    pub fn get_selected_video_codec<'a>(&self) -> EStreamVideoCodec {
        self.selected_video_codec.unwrap_or(EStreamVideoCodec::k_EStreamVideoCodecNone)
    }

    // repeated .CStreamVideoMode available_video_modes = 4;

    pub fn clear_available_video_modes(&mut self) {
        self.available_video_modes.clear();
    }

    // Param is passed by value, moved
    pub fn set_available_video_modes(&mut self, v: ::protobuf::RepeatedField<CStreamVideoMode>) {
        self.available_video_modes = v;
    }

    // Mutable pointer to the field.
    pub fn mut_available_video_modes<'a>(&'a mut self) -> &'a mut ::protobuf::RepeatedField<CStreamVideoMode> {
        &mut self.available_video_modes
    }

    // Take field
    pub fn take_available_video_modes(&mut self) -> ::protobuf::RepeatedField<CStreamVideoMode> {
        ::std::mem::replace(&mut self.available_video_modes, ::protobuf::RepeatedField::new())
    }

    pub fn get_available_video_modes<'a>(&'a self) -> &'a [CStreamVideoMode] {
        self.available_video_modes.as_slice()
    }
}

impl ::protobuf::Message for CNegotiatedConfig {
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
                    let tmp = try!(is.read_bool());
                    self.reliable_data = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_enum());
                    self.selected_audio_codec = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_enum());
                    self.selected_video_codec = ::std::option::Option::Some(tmp);
                },
                4 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.available_video_modes));
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
        if self.reliable_data.is_some() {
            my_size += 2;
        };
        for value in self.selected_audio_codec.iter() {
            my_size += ::protobuf::rt::enum_size(2, *value);
        };
        for value in self.selected_video_codec.iter() {
            my_size += ::protobuf::rt::enum_size(3, *value);
        };
        for value in self.available_video_modes.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.reliable_data {
            try!(os.write_bool(1, v));
        };
        if let Some(v) = self.selected_audio_codec {
            try!(os.write_enum(2, v as i32));
        };
        if let Some(v) = self.selected_video_codec {
            try!(os.write_enum(3, v as i32));
        };
        for v in self.available_video_modes.iter() {
            try!(os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
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
        ::std::intrinsics::TypeId::of::<CNegotiatedConfig>()
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CNegotiatedConfig {
    fn new() -> CNegotiatedConfig {
        CNegotiatedConfig::new()
    }

    #[allow(unused_unsafe,unused_mut)]
    fn descriptor_static(_: ::std::option::Option<CNegotiatedConfig>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor(
                    "reliable_data",
                    CNegotiatedConfig::has_reliable_data,
                    CNegotiatedConfig::get_reliable_data,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_enum_accessor(
                    "selected_audio_codec",
                    CNegotiatedConfig::has_selected_audio_codec,
                    CNegotiatedConfig::get_selected_audio_codec,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_enum_accessor(
                    "selected_video_codec",
                    CNegotiatedConfig::has_selected_video_codec,
                    CNegotiatedConfig::get_selected_video_codec,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "available_video_modes",
                    CNegotiatedConfig::get_available_video_modes,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CNegotiatedConfig>(
                    "CNegotiatedConfig",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CNegotiatedConfig {
    fn clear(&mut self) {
        self.clear_reliable_data();
        self.clear_selected_audio_codec();
        self.clear_selected_video_codec();
        self.clear_available_video_modes();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for CNegotiatedConfig {
    fn eq(&self, other: &CNegotiatedConfig) -> bool {
        self.reliable_data == other.reliable_data &&
        self.selected_audio_codec == other.selected_audio_codec &&
        self.selected_video_codec == other.selected_video_codec &&
        self.available_video_modes == other.available_video_modes &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Show for CNegotiatedConfig {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct CNegotiationInitMsg {
    reliable_data: ::std::option::Option<bool>,
    supported_audio_codecs: ::std::vec::Vec<EStreamAudioCodec>,
    supported_video_codecs: ::std::vec::Vec<EStreamVideoCodec>,
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl CNegotiationInitMsg {
    pub fn new() -> CNegotiationInitMsg {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CNegotiationInitMsg {
        static mut instance: ::protobuf::lazy::Lazy<CNegotiationInitMsg> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CNegotiationInitMsg,
        };
        unsafe {
            instance.get(|| {
                CNegotiationInitMsg {
                    reliable_data: ::std::option::Option::None,
                    supported_audio_codecs: ::std::vec::Vec::new(),
                    supported_video_codecs: ::std::vec::Vec::new(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional bool reliable_data = 1;

    pub fn clear_reliable_data(&mut self) {
        self.reliable_data = ::std::option::Option::None;
    }

    pub fn has_reliable_data(&self) -> bool {
        self.reliable_data.is_some()
    }

    // Param is passed by value, moved
    pub fn set_reliable_data(&mut self, v: bool) {
        self.reliable_data = ::std::option::Option::Some(v);
    }

    pub fn get_reliable_data<'a>(&self) -> bool {
        self.reliable_data.unwrap_or(false)
    }

    // repeated .EStreamAudioCodec supported_audio_codecs = 2;

    pub fn clear_supported_audio_codecs(&mut self) {
        self.supported_audio_codecs.clear();
    }

    // Param is passed by value, moved
    pub fn set_supported_audio_codecs(&mut self, v: ::std::vec::Vec<EStreamAudioCodec>) {
        self.supported_audio_codecs = v;
    }

    // Mutable pointer to the field.
    pub fn mut_supported_audio_codecs<'a>(&'a mut self) -> &'a mut ::std::vec::Vec<EStreamAudioCodec> {
        &mut self.supported_audio_codecs
    }

    // Take field
    pub fn take_supported_audio_codecs(&mut self) -> ::std::vec::Vec<EStreamAudioCodec> {
        ::std::mem::replace(&mut self.supported_audio_codecs, ::std::vec::Vec::new())
    }

    pub fn get_supported_audio_codecs<'a>(&'a self) -> &'a [EStreamAudioCodec] {
        self.supported_audio_codecs.as_slice()
    }

    // repeated .EStreamVideoCodec supported_video_codecs = 3;

    pub fn clear_supported_video_codecs(&mut self) {
        self.supported_video_codecs.clear();
    }

    // Param is passed by value, moved
    pub fn set_supported_video_codecs(&mut self, v: ::std::vec::Vec<EStreamVideoCodec>) {
        self.supported_video_codecs = v;
    }

    // Mutable pointer to the field.
    pub fn mut_supported_video_codecs<'a>(&'a mut self) -> &'a mut ::std::vec::Vec<EStreamVideoCodec> {
        &mut self.supported_video_codecs
    }

    // Take field
    pub fn take_supported_video_codecs(&mut self) -> ::std::vec::Vec<EStreamVideoCodec> {
        ::std::mem::replace(&mut self.supported_video_codecs, ::std::vec::Vec::new())
    }

    pub fn get_supported_video_codecs<'a>(&'a self) -> &'a [EStreamVideoCodec] {
        self.supported_video_codecs.as_slice()
    }
}

impl ::protobuf::Message for CNegotiationInitMsg {
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
                    let tmp = try!(is.read_bool());
                    self.reliable_data = ::std::option::Option::Some(tmp);
                },
                2 => {
                    try!(::protobuf::rt::read_repeated_enum_into(wire_type, is, &mut self.supported_audio_codecs));
                },
                3 => {
                    try!(::protobuf::rt::read_repeated_enum_into(wire_type, is, &mut self.supported_video_codecs));
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
        if self.reliable_data.is_some() {
            my_size += 2;
        };
        for value in self.supported_audio_codecs.iter() {
            my_size += ::protobuf::rt::enum_size(2, *value);
        };
        for value in self.supported_video_codecs.iter() {
            my_size += ::protobuf::rt::enum_size(3, *value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.reliable_data {
            try!(os.write_bool(1, v));
        };
        for v in self.supported_audio_codecs.iter() {
            try!(os.write_enum(2, *v as i32));
        };
        for v in self.supported_video_codecs.iter() {
            try!(os.write_enum(3, *v as i32));
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
        ::std::intrinsics::TypeId::of::<CNegotiationInitMsg>()
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CNegotiationInitMsg {
    fn new() -> CNegotiationInitMsg {
        CNegotiationInitMsg::new()
    }

    #[allow(unused_unsafe,unused_mut)]
    fn descriptor_static(_: ::std::option::Option<CNegotiationInitMsg>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor(
                    "reliable_data",
                    CNegotiationInitMsg::has_reliable_data,
                    CNegotiationInitMsg::get_reliable_data,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_enum_accessor(
                    "supported_audio_codecs",
                    CNegotiationInitMsg::get_supported_audio_codecs,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_enum_accessor(
                    "supported_video_codecs",
                    CNegotiationInitMsg::get_supported_video_codecs,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CNegotiationInitMsg>(
                    "CNegotiationInitMsg",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CNegotiationInitMsg {
    fn clear(&mut self) {
        self.clear_reliable_data();
        self.clear_supported_audio_codecs();
        self.clear_supported_video_codecs();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for CNegotiationInitMsg {
    fn eq(&self, other: &CNegotiationInitMsg) -> bool {
        self.reliable_data == other.reliable_data &&
        self.supported_audio_codecs == other.supported_audio_codecs &&
        self.supported_video_codecs == other.supported_video_codecs &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Show for CNegotiationInitMsg {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct CNegotiationSetConfigMsg {
    config: ::protobuf::SingularPtrField<CNegotiatedConfig>,
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl CNegotiationSetConfigMsg {
    pub fn new() -> CNegotiationSetConfigMsg {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CNegotiationSetConfigMsg {
        static mut instance: ::protobuf::lazy::Lazy<CNegotiationSetConfigMsg> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CNegotiationSetConfigMsg,
        };
        unsafe {
            instance.get(|| {
                CNegotiationSetConfigMsg {
                    config: ::protobuf::SingularPtrField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required .CNegotiatedConfig config = 1;

    pub fn clear_config(&mut self) {
        self.config.clear();
    }

    pub fn has_config(&self) -> bool {
        self.config.is_some()
    }

    // Param is passed by value, moved
    pub fn set_config(&mut self, v: CNegotiatedConfig) {
        self.config = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_config<'a>(&'a mut self) -> &'a mut CNegotiatedConfig {
        if self.config.is_none() {
            self.config.set_default();
        };
        self.config.as_mut().unwrap()
    }

    // Take field
    pub fn take_config(&mut self) -> CNegotiatedConfig {
        self.config.take().unwrap_or_else(|| CNegotiatedConfig::new())
    }

    pub fn get_config<'a>(&'a self) -> &'a CNegotiatedConfig {
        self.config.as_ref().unwrap_or_else(|| CNegotiatedConfig::default_instance())
    }
}

impl ::protobuf::Message for CNegotiationSetConfigMsg {
    fn is_initialized(&self) -> bool {
        if self.config.is_none() {
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
                    let tmp = self.config.set_default();
                    try!(is.merge_message(tmp))
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
        for value in self.config.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.config.as_ref() {
            try!(os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
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
        ::std::intrinsics::TypeId::of::<CNegotiationSetConfigMsg>()
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CNegotiationSetConfigMsg {
    fn new() -> CNegotiationSetConfigMsg {
        CNegotiationSetConfigMsg::new()
    }

    #[allow(unused_unsafe,unused_mut)]
    fn descriptor_static(_: ::std::option::Option<CNegotiationSetConfigMsg>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "config",
                    CNegotiationSetConfigMsg::has_config,
                    CNegotiationSetConfigMsg::get_config,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CNegotiationSetConfigMsg>(
                    "CNegotiationSetConfigMsg",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CNegotiationSetConfigMsg {
    fn clear(&mut self) {
        self.clear_config();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for CNegotiationSetConfigMsg {
    fn eq(&self, other: &CNegotiationSetConfigMsg) -> bool {
        self.config == other.config &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Show for CNegotiationSetConfigMsg {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct CNegotiationCompleteMsg {
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl CNegotiationCompleteMsg {
    pub fn new() -> CNegotiationCompleteMsg {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CNegotiationCompleteMsg {
        static mut instance: ::protobuf::lazy::Lazy<CNegotiationCompleteMsg> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CNegotiationCompleteMsg,
        };
        unsafe {
            instance.get(|| {
                CNegotiationCompleteMsg {
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }
}

impl ::protobuf::Message for CNegotiationCompleteMsg {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
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
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
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
        ::std::intrinsics::TypeId::of::<CNegotiationCompleteMsg>()
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CNegotiationCompleteMsg {
    fn new() -> CNegotiationCompleteMsg {
        CNegotiationCompleteMsg::new()
    }

    #[allow(unused_unsafe,unused_mut)]
    fn descriptor_static(_: ::std::option::Option<CNegotiationCompleteMsg>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<CNegotiationCompleteMsg>(
                    "CNegotiationCompleteMsg",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CNegotiationCompleteMsg {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for CNegotiationCompleteMsg {
    fn eq(&self, other: &CNegotiationCompleteMsg) -> bool {
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Show for CNegotiationCompleteMsg {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct CStartAudioDataMsg {
    channel: ::std::option::Option<u32>,
    codec: ::std::option::Option<EStreamAudioCodec>,
    codec_data: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    frequency: ::std::option::Option<u32>,
    channels: ::std::option::Option<u32>,
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl CStartAudioDataMsg {
    pub fn new() -> CStartAudioDataMsg {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CStartAudioDataMsg {
        static mut instance: ::protobuf::lazy::Lazy<CStartAudioDataMsg> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CStartAudioDataMsg,
        };
        unsafe {
            instance.get(|| {
                CStartAudioDataMsg {
                    channel: ::std::option::Option::None,
                    codec: ::std::option::Option::None,
                    codec_data: ::protobuf::SingularField::none(),
                    frequency: ::std::option::Option::None,
                    channels: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required uint32 channel = 2;

    pub fn clear_channel(&mut self) {
        self.channel = ::std::option::Option::None;
    }

    pub fn has_channel(&self) -> bool {
        self.channel.is_some()
    }

    // Param is passed by value, moved
    pub fn set_channel(&mut self, v: u32) {
        self.channel = ::std::option::Option::Some(v);
    }

    pub fn get_channel<'a>(&self) -> u32 {
        self.channel.unwrap_or(0)
    }

    // optional .EStreamAudioCodec codec = 3;

    pub fn clear_codec(&mut self) {
        self.codec = ::std::option::Option::None;
    }

    pub fn has_codec(&self) -> bool {
        self.codec.is_some()
    }

    // Param is passed by value, moved
    pub fn set_codec(&mut self, v: EStreamAudioCodec) {
        self.codec = ::std::option::Option::Some(v);
    }

    pub fn get_codec<'a>(&self) -> EStreamAudioCodec {
        self.codec.unwrap_or(EStreamAudioCodec::k_EStreamAudioCodecNone)
    }

    // optional bytes codec_data = 4;

    pub fn clear_codec_data(&mut self) {
        self.codec_data.clear();
    }

    pub fn has_codec_data(&self) -> bool {
        self.codec_data.is_some()
    }

    // Param is passed by value, moved
    pub fn set_codec_data(&mut self, v: ::std::vec::Vec<u8>) {
        self.codec_data = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_codec_data<'a>(&'a mut self) -> &'a mut ::std::vec::Vec<u8> {
        if self.codec_data.is_none() {
            self.codec_data.set_default();
        };
        self.codec_data.as_mut().unwrap()
    }

    // Take field
    pub fn take_codec_data(&mut self) -> ::std::vec::Vec<u8> {
        self.codec_data.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_codec_data<'a>(&'a self) -> &'a [u8] {
        match self.codec_data.as_ref() {
            Some(v) => v.as_slice(),
            None => [].as_slice(),
        }
    }

    // optional uint32 frequency = 5;

    pub fn clear_frequency(&mut self) {
        self.frequency = ::std::option::Option::None;
    }

    pub fn has_frequency(&self) -> bool {
        self.frequency.is_some()
    }

    // Param is passed by value, moved
    pub fn set_frequency(&mut self, v: u32) {
        self.frequency = ::std::option::Option::Some(v);
    }

    pub fn get_frequency<'a>(&self) -> u32 {
        self.frequency.unwrap_or(0)
    }

    // optional uint32 channels = 6;

    pub fn clear_channels(&mut self) {
        self.channels = ::std::option::Option::None;
    }

    pub fn has_channels(&self) -> bool {
        self.channels.is_some()
    }

    // Param is passed by value, moved
    pub fn set_channels(&mut self, v: u32) {
        self.channels = ::std::option::Option::Some(v);
    }

    pub fn get_channels<'a>(&self) -> u32 {
        self.channels.unwrap_or(0)
    }
}

impl ::protobuf::Message for CStartAudioDataMsg {
    fn is_initialized(&self) -> bool {
        if self.channel.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_uint32());
                    self.channel = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_enum());
                    self.codec = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.codec_data.set_default();
                    try!(is.read_bytes_into(tmp))
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_uint32());
                    self.frequency = ::std::option::Option::Some(tmp);
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_uint32());
                    self.channels = ::std::option::Option::Some(tmp);
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
        for value in self.channel.iter() {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.codec.iter() {
            my_size += ::protobuf::rt::enum_size(3, *value);
        };
        for value in self.codec_data.iter() {
            my_size += ::protobuf::rt::bytes_size(4, value.as_slice());
        };
        for value in self.frequency.iter() {
            my_size += ::protobuf::rt::value_size(5, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.channels.iter() {
            my_size += ::protobuf::rt::value_size(6, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.channel {
            try!(os.write_uint32(2, v));
        };
        if let Some(v) = self.codec {
            try!(os.write_enum(3, v as i32));
        };
        if let Some(v) = self.codec_data.as_ref() {
            try!(os.write_bytes(4, v.as_slice()));
        };
        if let Some(v) = self.frequency {
            try!(os.write_uint32(5, v));
        };
        if let Some(v) = self.channels {
            try!(os.write_uint32(6, v));
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
        ::std::intrinsics::TypeId::of::<CStartAudioDataMsg>()
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CStartAudioDataMsg {
    fn new() -> CStartAudioDataMsg {
        CStartAudioDataMsg::new()
    }

    #[allow(unused_unsafe,unused_mut)]
    fn descriptor_static(_: ::std::option::Option<CStartAudioDataMsg>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_u32_accessor(
                    "channel",
                    CStartAudioDataMsg::has_channel,
                    CStartAudioDataMsg::get_channel,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_enum_accessor(
                    "codec",
                    CStartAudioDataMsg::has_codec,
                    CStartAudioDataMsg::get_codec,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "codec_data",
                    CStartAudioDataMsg::has_codec_data,
                    CStartAudioDataMsg::get_codec_data,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u32_accessor(
                    "frequency",
                    CStartAudioDataMsg::has_frequency,
                    CStartAudioDataMsg::get_frequency,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u32_accessor(
                    "channels",
                    CStartAudioDataMsg::has_channels,
                    CStartAudioDataMsg::get_channels,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CStartAudioDataMsg>(
                    "CStartAudioDataMsg",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CStartAudioDataMsg {
    fn clear(&mut self) {
        self.clear_channel();
        self.clear_codec();
        self.clear_codec_data();
        self.clear_frequency();
        self.clear_channels();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for CStartAudioDataMsg {
    fn eq(&self, other: &CStartAudioDataMsg) -> bool {
        self.channel == other.channel &&
        self.codec == other.codec &&
        self.codec_data == other.codec_data &&
        self.frequency == other.frequency &&
        self.channels == other.channels &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Show for CStartAudioDataMsg {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct CStopAudioDataMsg {
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl CStopAudioDataMsg {
    pub fn new() -> CStopAudioDataMsg {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CStopAudioDataMsg {
        static mut instance: ::protobuf::lazy::Lazy<CStopAudioDataMsg> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CStopAudioDataMsg,
        };
        unsafe {
            instance.get(|| {
                CStopAudioDataMsg {
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }
}

impl ::protobuf::Message for CStopAudioDataMsg {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
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
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
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
        ::std::intrinsics::TypeId::of::<CStopAudioDataMsg>()
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CStopAudioDataMsg {
    fn new() -> CStopAudioDataMsg {
        CStopAudioDataMsg::new()
    }

    #[allow(unused_unsafe,unused_mut)]
    fn descriptor_static(_: ::std::option::Option<CStopAudioDataMsg>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<CStopAudioDataMsg>(
                    "CStopAudioDataMsg",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CStopAudioDataMsg {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for CStopAudioDataMsg {
    fn eq(&self, other: &CStopAudioDataMsg) -> bool {
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Show for CStopAudioDataMsg {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct CStartVideoDataMsg {
    channel: ::std::option::Option<u32>,
    codec: ::std::option::Option<EStreamVideoCodec>,
    codec_data: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    width: ::std::option::Option<u32>,
    height: ::std::option::Option<u32>,
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl CStartVideoDataMsg {
    pub fn new() -> CStartVideoDataMsg {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CStartVideoDataMsg {
        static mut instance: ::protobuf::lazy::Lazy<CStartVideoDataMsg> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CStartVideoDataMsg,
        };
        unsafe {
            instance.get(|| {
                CStartVideoDataMsg {
                    channel: ::std::option::Option::None,
                    codec: ::std::option::Option::None,
                    codec_data: ::protobuf::SingularField::none(),
                    width: ::std::option::Option::None,
                    height: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required uint32 channel = 1;

    pub fn clear_channel(&mut self) {
        self.channel = ::std::option::Option::None;
    }

    pub fn has_channel(&self) -> bool {
        self.channel.is_some()
    }

    // Param is passed by value, moved
    pub fn set_channel(&mut self, v: u32) {
        self.channel = ::std::option::Option::Some(v);
    }

    pub fn get_channel<'a>(&self) -> u32 {
        self.channel.unwrap_or(0)
    }

    // optional .EStreamVideoCodec codec = 2;

    pub fn clear_codec(&mut self) {
        self.codec = ::std::option::Option::None;
    }

    pub fn has_codec(&self) -> bool {
        self.codec.is_some()
    }

    // Param is passed by value, moved
    pub fn set_codec(&mut self, v: EStreamVideoCodec) {
        self.codec = ::std::option::Option::Some(v);
    }

    pub fn get_codec<'a>(&self) -> EStreamVideoCodec {
        self.codec.unwrap_or(EStreamVideoCodec::k_EStreamVideoCodecNone)
    }

    // optional bytes codec_data = 3;

    pub fn clear_codec_data(&mut self) {
        self.codec_data.clear();
    }

    pub fn has_codec_data(&self) -> bool {
        self.codec_data.is_some()
    }

    // Param is passed by value, moved
    pub fn set_codec_data(&mut self, v: ::std::vec::Vec<u8>) {
        self.codec_data = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_codec_data<'a>(&'a mut self) -> &'a mut ::std::vec::Vec<u8> {
        if self.codec_data.is_none() {
            self.codec_data.set_default();
        };
        self.codec_data.as_mut().unwrap()
    }

    // Take field
    pub fn take_codec_data(&mut self) -> ::std::vec::Vec<u8> {
        self.codec_data.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_codec_data<'a>(&'a self) -> &'a [u8] {
        match self.codec_data.as_ref() {
            Some(v) => v.as_slice(),
            None => [].as_slice(),
        }
    }

    // optional uint32 width = 4;

    pub fn clear_width(&mut self) {
        self.width = ::std::option::Option::None;
    }

    pub fn has_width(&self) -> bool {
        self.width.is_some()
    }

    // Param is passed by value, moved
    pub fn set_width(&mut self, v: u32) {
        self.width = ::std::option::Option::Some(v);
    }

    pub fn get_width<'a>(&self) -> u32 {
        self.width.unwrap_or(0)
    }

    // optional uint32 height = 5;

    pub fn clear_height(&mut self) {
        self.height = ::std::option::Option::None;
    }

    pub fn has_height(&self) -> bool {
        self.height.is_some()
    }

    // Param is passed by value, moved
    pub fn set_height(&mut self, v: u32) {
        self.height = ::std::option::Option::Some(v);
    }

    pub fn get_height<'a>(&self) -> u32 {
        self.height.unwrap_or(0)
    }
}

impl ::protobuf::Message for CStartVideoDataMsg {
    fn is_initialized(&self) -> bool {
        if self.channel.is_none() {
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
                    self.channel = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_enum());
                    self.codec = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.codec_data.set_default();
                    try!(is.read_bytes_into(tmp))
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_uint32());
                    self.width = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_uint32());
                    self.height = ::std::option::Option::Some(tmp);
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
        for value in self.channel.iter() {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.codec.iter() {
            my_size += ::protobuf::rt::enum_size(2, *value);
        };
        for value in self.codec_data.iter() {
            my_size += ::protobuf::rt::bytes_size(3, value.as_slice());
        };
        for value in self.width.iter() {
            my_size += ::protobuf::rt::value_size(4, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.height.iter() {
            my_size += ::protobuf::rt::value_size(5, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.channel {
            try!(os.write_uint32(1, v));
        };
        if let Some(v) = self.codec {
            try!(os.write_enum(2, v as i32));
        };
        if let Some(v) = self.codec_data.as_ref() {
            try!(os.write_bytes(3, v.as_slice()));
        };
        if let Some(v) = self.width {
            try!(os.write_uint32(4, v));
        };
        if let Some(v) = self.height {
            try!(os.write_uint32(5, v));
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
        ::std::intrinsics::TypeId::of::<CStartVideoDataMsg>()
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CStartVideoDataMsg {
    fn new() -> CStartVideoDataMsg {
        CStartVideoDataMsg::new()
    }

    #[allow(unused_unsafe,unused_mut)]
    fn descriptor_static(_: ::std::option::Option<CStartVideoDataMsg>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_u32_accessor(
                    "channel",
                    CStartVideoDataMsg::has_channel,
                    CStartVideoDataMsg::get_channel,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_enum_accessor(
                    "codec",
                    CStartVideoDataMsg::has_codec,
                    CStartVideoDataMsg::get_codec,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "codec_data",
                    CStartVideoDataMsg::has_codec_data,
                    CStartVideoDataMsg::get_codec_data,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u32_accessor(
                    "width",
                    CStartVideoDataMsg::has_width,
                    CStartVideoDataMsg::get_width,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u32_accessor(
                    "height",
                    CStartVideoDataMsg::has_height,
                    CStartVideoDataMsg::get_height,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CStartVideoDataMsg>(
                    "CStartVideoDataMsg",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CStartVideoDataMsg {
    fn clear(&mut self) {
        self.clear_channel();
        self.clear_codec();
        self.clear_codec_data();
        self.clear_width();
        self.clear_height();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for CStartVideoDataMsg {
    fn eq(&self, other: &CStartVideoDataMsg) -> bool {
        self.channel == other.channel &&
        self.codec == other.codec &&
        self.codec_data == other.codec_data &&
        self.width == other.width &&
        self.height == other.height &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Show for CStartVideoDataMsg {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct CStopVideoDataMsg {
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl CStopVideoDataMsg {
    pub fn new() -> CStopVideoDataMsg {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CStopVideoDataMsg {
        static mut instance: ::protobuf::lazy::Lazy<CStopVideoDataMsg> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CStopVideoDataMsg,
        };
        unsafe {
            instance.get(|| {
                CStopVideoDataMsg {
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }
}

impl ::protobuf::Message for CStopVideoDataMsg {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
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
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
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
        ::std::intrinsics::TypeId::of::<CStopVideoDataMsg>()
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CStopVideoDataMsg {
    fn new() -> CStopVideoDataMsg {
        CStopVideoDataMsg::new()
    }

    #[allow(unused_unsafe,unused_mut)]
    fn descriptor_static(_: ::std::option::Option<CStopVideoDataMsg>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<CStopVideoDataMsg>(
                    "CStopVideoDataMsg",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CStopVideoDataMsg {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for CStopVideoDataMsg {
    fn eq(&self, other: &CStopVideoDataMsg) -> bool {
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Show for CStopVideoDataMsg {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct CInputLatencyTestMsg {
    input_mark: ::std::option::Option<u32>,
    color: ::std::option::Option<u32>,
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl CInputLatencyTestMsg {
    pub fn new() -> CInputLatencyTestMsg {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CInputLatencyTestMsg {
        static mut instance: ::protobuf::lazy::Lazy<CInputLatencyTestMsg> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CInputLatencyTestMsg,
        };
        unsafe {
            instance.get(|| {
                CInputLatencyTestMsg {
                    input_mark: ::std::option::Option::None,
                    color: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required uint32 input_mark = 1;

    pub fn clear_input_mark(&mut self) {
        self.input_mark = ::std::option::Option::None;
    }

    pub fn has_input_mark(&self) -> bool {
        self.input_mark.is_some()
    }

    // Param is passed by value, moved
    pub fn set_input_mark(&mut self, v: u32) {
        self.input_mark = ::std::option::Option::Some(v);
    }

    pub fn get_input_mark<'a>(&self) -> u32 {
        self.input_mark.unwrap_or(0)
    }

    // optional uint32 color = 2;

    pub fn clear_color(&mut self) {
        self.color = ::std::option::Option::None;
    }

    pub fn has_color(&self) -> bool {
        self.color.is_some()
    }

    // Param is passed by value, moved
    pub fn set_color(&mut self, v: u32) {
        self.color = ::std::option::Option::Some(v);
    }

    pub fn get_color<'a>(&self) -> u32 {
        self.color.unwrap_or(0)
    }
}

impl ::protobuf::Message for CInputLatencyTestMsg {
    fn is_initialized(&self) -> bool {
        if self.input_mark.is_none() {
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
                    self.input_mark = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_uint32());
                    self.color = ::std::option::Option::Some(tmp);
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
        for value in self.input_mark.iter() {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.color.iter() {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.input_mark {
            try!(os.write_uint32(1, v));
        };
        if let Some(v) = self.color {
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
        ::std::intrinsics::TypeId::of::<CInputLatencyTestMsg>()
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CInputLatencyTestMsg {
    fn new() -> CInputLatencyTestMsg {
        CInputLatencyTestMsg::new()
    }

    #[allow(unused_unsafe,unused_mut)]
    fn descriptor_static(_: ::std::option::Option<CInputLatencyTestMsg>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_u32_accessor(
                    "input_mark",
                    CInputLatencyTestMsg::has_input_mark,
                    CInputLatencyTestMsg::get_input_mark,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u32_accessor(
                    "color",
                    CInputLatencyTestMsg::has_color,
                    CInputLatencyTestMsg::get_color,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CInputLatencyTestMsg>(
                    "CInputLatencyTestMsg",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CInputLatencyTestMsg {
    fn clear(&mut self) {
        self.clear_input_mark();
        self.clear_color();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for CInputLatencyTestMsg {
    fn eq(&self, other: &CInputLatencyTestMsg) -> bool {
        self.input_mark == other.input_mark &&
        self.color == other.color &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Show for CInputLatencyTestMsg {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct CInputMouseMotionMsg {
    input_mark: ::std::option::Option<u32>,
    x_normalized: ::std::option::Option<f32>,
    y_normalized: ::std::option::Option<f32>,
    dx: ::std::option::Option<i32>,
    dy: ::std::option::Option<i32>,
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl CInputMouseMotionMsg {
    pub fn new() -> CInputMouseMotionMsg {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CInputMouseMotionMsg {
        static mut instance: ::protobuf::lazy::Lazy<CInputMouseMotionMsg> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CInputMouseMotionMsg,
        };
        unsafe {
            instance.get(|| {
                CInputMouseMotionMsg {
                    input_mark: ::std::option::Option::None,
                    x_normalized: ::std::option::Option::None,
                    y_normalized: ::std::option::Option::None,
                    dx: ::std::option::Option::None,
                    dy: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional uint32 input_mark = 1;

    pub fn clear_input_mark(&mut self) {
        self.input_mark = ::std::option::Option::None;
    }

    pub fn has_input_mark(&self) -> bool {
        self.input_mark.is_some()
    }

    // Param is passed by value, moved
    pub fn set_input_mark(&mut self, v: u32) {
        self.input_mark = ::std::option::Option::Some(v);
    }

    pub fn get_input_mark<'a>(&self) -> u32 {
        self.input_mark.unwrap_or(0)
    }

    // optional float x_normalized = 2;

    pub fn clear_x_normalized(&mut self) {
        self.x_normalized = ::std::option::Option::None;
    }

    pub fn has_x_normalized(&self) -> bool {
        self.x_normalized.is_some()
    }

    // Param is passed by value, moved
    pub fn set_x_normalized(&mut self, v: f32) {
        self.x_normalized = ::std::option::Option::Some(v);
    }

    pub fn get_x_normalized<'a>(&self) -> f32 {
        self.x_normalized.unwrap_or(0.)
    }

    // optional float y_normalized = 3;

    pub fn clear_y_normalized(&mut self) {
        self.y_normalized = ::std::option::Option::None;
    }

    pub fn has_y_normalized(&self) -> bool {
        self.y_normalized.is_some()
    }

    // Param is passed by value, moved
    pub fn set_y_normalized(&mut self, v: f32) {
        self.y_normalized = ::std::option::Option::Some(v);
    }

    pub fn get_y_normalized<'a>(&self) -> f32 {
        self.y_normalized.unwrap_or(0.)
    }

    // optional int32 dx = 4;

    pub fn clear_dx(&mut self) {
        self.dx = ::std::option::Option::None;
    }

    pub fn has_dx(&self) -> bool {
        self.dx.is_some()
    }

    // Param is passed by value, moved
    pub fn set_dx(&mut self, v: i32) {
        self.dx = ::std::option::Option::Some(v);
    }

    pub fn get_dx<'a>(&self) -> i32 {
        self.dx.unwrap_or(0)
    }

    // optional int32 dy = 5;

    pub fn clear_dy(&mut self) {
        self.dy = ::std::option::Option::None;
    }

    pub fn has_dy(&self) -> bool {
        self.dy.is_some()
    }

    // Param is passed by value, moved
    pub fn set_dy(&mut self, v: i32) {
        self.dy = ::std::option::Option::Some(v);
    }

    pub fn get_dy<'a>(&self) -> i32 {
        self.dy.unwrap_or(0)
    }
}

impl ::protobuf::Message for CInputMouseMotionMsg {
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
                    self.input_mark = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_float());
                    self.x_normalized = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_float());
                    self.y_normalized = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_int32());
                    self.dx = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_int32());
                    self.dy = ::std::option::Option::Some(tmp);
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
        for value in self.input_mark.iter() {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        if self.x_normalized.is_some() {
            my_size += 5;
        };
        if self.y_normalized.is_some() {
            my_size += 5;
        };
        for value in self.dx.iter() {
            my_size += ::protobuf::rt::value_size(4, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.dy.iter() {
            my_size += ::protobuf::rt::value_size(5, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.input_mark {
            try!(os.write_uint32(1, v));
        };
        if let Some(v) = self.x_normalized {
            try!(os.write_float(2, v));
        };
        if let Some(v) = self.y_normalized {
            try!(os.write_float(3, v));
        };
        if let Some(v) = self.dx {
            try!(os.write_int32(4, v));
        };
        if let Some(v) = self.dy {
            try!(os.write_int32(5, v));
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
        ::std::intrinsics::TypeId::of::<CInputMouseMotionMsg>()
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CInputMouseMotionMsg {
    fn new() -> CInputMouseMotionMsg {
        CInputMouseMotionMsg::new()
    }

    #[allow(unused_unsafe,unused_mut)]
    fn descriptor_static(_: ::std::option::Option<CInputMouseMotionMsg>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_u32_accessor(
                    "input_mark",
                    CInputMouseMotionMsg::has_input_mark,
                    CInputMouseMotionMsg::get_input_mark,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f32_accessor(
                    "x_normalized",
                    CInputMouseMotionMsg::has_x_normalized,
                    CInputMouseMotionMsg::get_x_normalized,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f32_accessor(
                    "y_normalized",
                    CInputMouseMotionMsg::has_y_normalized,
                    CInputMouseMotionMsg::get_y_normalized,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "dx",
                    CInputMouseMotionMsg::has_dx,
                    CInputMouseMotionMsg::get_dx,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "dy",
                    CInputMouseMotionMsg::has_dy,
                    CInputMouseMotionMsg::get_dy,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CInputMouseMotionMsg>(
                    "CInputMouseMotionMsg",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CInputMouseMotionMsg {
    fn clear(&mut self) {
        self.clear_input_mark();
        self.clear_x_normalized();
        self.clear_y_normalized();
        self.clear_dx();
        self.clear_dy();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for CInputMouseMotionMsg {
    fn eq(&self, other: &CInputMouseMotionMsg) -> bool {
        self.input_mark == other.input_mark &&
        self.x_normalized == other.x_normalized &&
        self.y_normalized == other.y_normalized &&
        self.dx == other.dx &&
        self.dy == other.dy &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Show for CInputMouseMotionMsg {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct CInputMouseWheelMsg {
    input_mark: ::std::option::Option<u32>,
    direction: ::std::option::Option<EStreamMouseWheelDirection>,
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl CInputMouseWheelMsg {
    pub fn new() -> CInputMouseWheelMsg {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CInputMouseWheelMsg {
        static mut instance: ::protobuf::lazy::Lazy<CInputMouseWheelMsg> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CInputMouseWheelMsg,
        };
        unsafe {
            instance.get(|| {
                CInputMouseWheelMsg {
                    input_mark: ::std::option::Option::None,
                    direction: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional uint32 input_mark = 1;

    pub fn clear_input_mark(&mut self) {
        self.input_mark = ::std::option::Option::None;
    }

    pub fn has_input_mark(&self) -> bool {
        self.input_mark.is_some()
    }

    // Param is passed by value, moved
    pub fn set_input_mark(&mut self, v: u32) {
        self.input_mark = ::std::option::Option::Some(v);
    }

    pub fn get_input_mark<'a>(&self) -> u32 {
        self.input_mark.unwrap_or(0)
    }

    // required .EStreamMouseWheelDirection direction = 2;

    pub fn clear_direction(&mut self) {
        self.direction = ::std::option::Option::None;
    }

    pub fn has_direction(&self) -> bool {
        self.direction.is_some()
    }

    // Param is passed by value, moved
    pub fn set_direction(&mut self, v: EStreamMouseWheelDirection) {
        self.direction = ::std::option::Option::Some(v);
    }

    pub fn get_direction<'a>(&self) -> EStreamMouseWheelDirection {
        self.direction.unwrap_or(EStreamMouseWheelDirection::k_EStreamMouseWheelUp)
    }
}

impl ::protobuf::Message for CInputMouseWheelMsg {
    fn is_initialized(&self) -> bool {
        if self.direction.is_none() {
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
                    self.input_mark = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_enum());
                    self.direction = ::std::option::Option::Some(tmp);
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
        for value in self.input_mark.iter() {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.direction.iter() {
            my_size += ::protobuf::rt::enum_size(2, *value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.input_mark {
            try!(os.write_uint32(1, v));
        };
        if let Some(v) = self.direction {
            try!(os.write_enum(2, v as i32));
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
        ::std::intrinsics::TypeId::of::<CInputMouseWheelMsg>()
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CInputMouseWheelMsg {
    fn new() -> CInputMouseWheelMsg {
        CInputMouseWheelMsg::new()
    }

    #[allow(unused_unsafe,unused_mut)]
    fn descriptor_static(_: ::std::option::Option<CInputMouseWheelMsg>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_u32_accessor(
                    "input_mark",
                    CInputMouseWheelMsg::has_input_mark,
                    CInputMouseWheelMsg::get_input_mark,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_enum_accessor(
                    "direction",
                    CInputMouseWheelMsg::has_direction,
                    CInputMouseWheelMsg::get_direction,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CInputMouseWheelMsg>(
                    "CInputMouseWheelMsg",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CInputMouseWheelMsg {
    fn clear(&mut self) {
        self.clear_input_mark();
        self.clear_direction();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for CInputMouseWheelMsg {
    fn eq(&self, other: &CInputMouseWheelMsg) -> bool {
        self.input_mark == other.input_mark &&
        self.direction == other.direction &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Show for CInputMouseWheelMsg {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct CInputMouseDownMsg {
    input_mark: ::std::option::Option<u32>,
    button: ::std::option::Option<EStreamMouseButton>,
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl CInputMouseDownMsg {
    pub fn new() -> CInputMouseDownMsg {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CInputMouseDownMsg {
        static mut instance: ::protobuf::lazy::Lazy<CInputMouseDownMsg> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CInputMouseDownMsg,
        };
        unsafe {
            instance.get(|| {
                CInputMouseDownMsg {
                    input_mark: ::std::option::Option::None,
                    button: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional uint32 input_mark = 1;

    pub fn clear_input_mark(&mut self) {
        self.input_mark = ::std::option::Option::None;
    }

    pub fn has_input_mark(&self) -> bool {
        self.input_mark.is_some()
    }

    // Param is passed by value, moved
    pub fn set_input_mark(&mut self, v: u32) {
        self.input_mark = ::std::option::Option::Some(v);
    }

    pub fn get_input_mark<'a>(&self) -> u32 {
        self.input_mark.unwrap_or(0)
    }

    // required .EStreamMouseButton button = 2;

    pub fn clear_button(&mut self) {
        self.button = ::std::option::Option::None;
    }

    pub fn has_button(&self) -> bool {
        self.button.is_some()
    }

    // Param is passed by value, moved
    pub fn set_button(&mut self, v: EStreamMouseButton) {
        self.button = ::std::option::Option::Some(v);
    }

    pub fn get_button<'a>(&self) -> EStreamMouseButton {
        self.button.unwrap_or(EStreamMouseButton::k_EStreamMouseButtonLeft)
    }
}

impl ::protobuf::Message for CInputMouseDownMsg {
    fn is_initialized(&self) -> bool {
        if self.button.is_none() {
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
                    self.input_mark = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_enum());
                    self.button = ::std::option::Option::Some(tmp);
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
        for value in self.input_mark.iter() {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.button.iter() {
            my_size += ::protobuf::rt::enum_size(2, *value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.input_mark {
            try!(os.write_uint32(1, v));
        };
        if let Some(v) = self.button {
            try!(os.write_enum(2, v as i32));
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
        ::std::intrinsics::TypeId::of::<CInputMouseDownMsg>()
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CInputMouseDownMsg {
    fn new() -> CInputMouseDownMsg {
        CInputMouseDownMsg::new()
    }

    #[allow(unused_unsafe,unused_mut)]
    fn descriptor_static(_: ::std::option::Option<CInputMouseDownMsg>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_u32_accessor(
                    "input_mark",
                    CInputMouseDownMsg::has_input_mark,
                    CInputMouseDownMsg::get_input_mark,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_enum_accessor(
                    "button",
                    CInputMouseDownMsg::has_button,
                    CInputMouseDownMsg::get_button,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CInputMouseDownMsg>(
                    "CInputMouseDownMsg",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CInputMouseDownMsg {
    fn clear(&mut self) {
        self.clear_input_mark();
        self.clear_button();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for CInputMouseDownMsg {
    fn eq(&self, other: &CInputMouseDownMsg) -> bool {
        self.input_mark == other.input_mark &&
        self.button == other.button &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Show for CInputMouseDownMsg {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct CInputMouseUpMsg {
    input_mark: ::std::option::Option<u32>,
    button: ::std::option::Option<EStreamMouseButton>,
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl CInputMouseUpMsg {
    pub fn new() -> CInputMouseUpMsg {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CInputMouseUpMsg {
        static mut instance: ::protobuf::lazy::Lazy<CInputMouseUpMsg> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CInputMouseUpMsg,
        };
        unsafe {
            instance.get(|| {
                CInputMouseUpMsg {
                    input_mark: ::std::option::Option::None,
                    button: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional uint32 input_mark = 1;

    pub fn clear_input_mark(&mut self) {
        self.input_mark = ::std::option::Option::None;
    }

    pub fn has_input_mark(&self) -> bool {
        self.input_mark.is_some()
    }

    // Param is passed by value, moved
    pub fn set_input_mark(&mut self, v: u32) {
        self.input_mark = ::std::option::Option::Some(v);
    }

    pub fn get_input_mark<'a>(&self) -> u32 {
        self.input_mark.unwrap_or(0)
    }

    // required .EStreamMouseButton button = 2;

    pub fn clear_button(&mut self) {
        self.button = ::std::option::Option::None;
    }

    pub fn has_button(&self) -> bool {
        self.button.is_some()
    }

    // Param is passed by value, moved
    pub fn set_button(&mut self, v: EStreamMouseButton) {
        self.button = ::std::option::Option::Some(v);
    }

    pub fn get_button<'a>(&self) -> EStreamMouseButton {
        self.button.unwrap_or(EStreamMouseButton::k_EStreamMouseButtonLeft)
    }
}

impl ::protobuf::Message for CInputMouseUpMsg {
    fn is_initialized(&self) -> bool {
        if self.button.is_none() {
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
                    self.input_mark = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_enum());
                    self.button = ::std::option::Option::Some(tmp);
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
        for value in self.input_mark.iter() {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.button.iter() {
            my_size += ::protobuf::rt::enum_size(2, *value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.input_mark {
            try!(os.write_uint32(1, v));
        };
        if let Some(v) = self.button {
            try!(os.write_enum(2, v as i32));
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
        ::std::intrinsics::TypeId::of::<CInputMouseUpMsg>()
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CInputMouseUpMsg {
    fn new() -> CInputMouseUpMsg {
        CInputMouseUpMsg::new()
    }

    #[allow(unused_unsafe,unused_mut)]
    fn descriptor_static(_: ::std::option::Option<CInputMouseUpMsg>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_u32_accessor(
                    "input_mark",
                    CInputMouseUpMsg::has_input_mark,
                    CInputMouseUpMsg::get_input_mark,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_enum_accessor(
                    "button",
                    CInputMouseUpMsg::has_button,
                    CInputMouseUpMsg::get_button,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CInputMouseUpMsg>(
                    "CInputMouseUpMsg",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CInputMouseUpMsg {
    fn clear(&mut self) {
        self.clear_input_mark();
        self.clear_button();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for CInputMouseUpMsg {
    fn eq(&self, other: &CInputMouseUpMsg) -> bool {
        self.input_mark == other.input_mark &&
        self.button == other.button &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Show for CInputMouseUpMsg {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct CInputKeyDownMsg {
    input_mark: ::std::option::Option<u32>,
    scancode: ::std::option::Option<u32>,
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl CInputKeyDownMsg {
    pub fn new() -> CInputKeyDownMsg {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CInputKeyDownMsg {
        static mut instance: ::protobuf::lazy::Lazy<CInputKeyDownMsg> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CInputKeyDownMsg,
        };
        unsafe {
            instance.get(|| {
                CInputKeyDownMsg {
                    input_mark: ::std::option::Option::None,
                    scancode: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional uint32 input_mark = 1;

    pub fn clear_input_mark(&mut self) {
        self.input_mark = ::std::option::Option::None;
    }

    pub fn has_input_mark(&self) -> bool {
        self.input_mark.is_some()
    }

    // Param is passed by value, moved
    pub fn set_input_mark(&mut self, v: u32) {
        self.input_mark = ::std::option::Option::Some(v);
    }

    pub fn get_input_mark<'a>(&self) -> u32 {
        self.input_mark.unwrap_or(0)
    }

    // required uint32 scancode = 2;

    pub fn clear_scancode(&mut self) {
        self.scancode = ::std::option::Option::None;
    }

    pub fn has_scancode(&self) -> bool {
        self.scancode.is_some()
    }

    // Param is passed by value, moved
    pub fn set_scancode(&mut self, v: u32) {
        self.scancode = ::std::option::Option::Some(v);
    }

    pub fn get_scancode<'a>(&self) -> u32 {
        self.scancode.unwrap_or(0)
    }
}

impl ::protobuf::Message for CInputKeyDownMsg {
    fn is_initialized(&self) -> bool {
        if self.scancode.is_none() {
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
                    self.input_mark = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_uint32());
                    self.scancode = ::std::option::Option::Some(tmp);
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
        for value in self.input_mark.iter() {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.scancode.iter() {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.input_mark {
            try!(os.write_uint32(1, v));
        };
        if let Some(v) = self.scancode {
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
        ::std::intrinsics::TypeId::of::<CInputKeyDownMsg>()
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CInputKeyDownMsg {
    fn new() -> CInputKeyDownMsg {
        CInputKeyDownMsg::new()
    }

    #[allow(unused_unsafe,unused_mut)]
    fn descriptor_static(_: ::std::option::Option<CInputKeyDownMsg>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_u32_accessor(
                    "input_mark",
                    CInputKeyDownMsg::has_input_mark,
                    CInputKeyDownMsg::get_input_mark,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u32_accessor(
                    "scancode",
                    CInputKeyDownMsg::has_scancode,
                    CInputKeyDownMsg::get_scancode,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CInputKeyDownMsg>(
                    "CInputKeyDownMsg",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CInputKeyDownMsg {
    fn clear(&mut self) {
        self.clear_input_mark();
        self.clear_scancode();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for CInputKeyDownMsg {
    fn eq(&self, other: &CInputKeyDownMsg) -> bool {
        self.input_mark == other.input_mark &&
        self.scancode == other.scancode &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Show for CInputKeyDownMsg {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct CInputKeyUpMsg {
    input_mark: ::std::option::Option<u32>,
    scancode: ::std::option::Option<u32>,
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl CInputKeyUpMsg {
    pub fn new() -> CInputKeyUpMsg {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CInputKeyUpMsg {
        static mut instance: ::protobuf::lazy::Lazy<CInputKeyUpMsg> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CInputKeyUpMsg,
        };
        unsafe {
            instance.get(|| {
                CInputKeyUpMsg {
                    input_mark: ::std::option::Option::None,
                    scancode: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional uint32 input_mark = 1;

    pub fn clear_input_mark(&mut self) {
        self.input_mark = ::std::option::Option::None;
    }

    pub fn has_input_mark(&self) -> bool {
        self.input_mark.is_some()
    }

    // Param is passed by value, moved
    pub fn set_input_mark(&mut self, v: u32) {
        self.input_mark = ::std::option::Option::Some(v);
    }

    pub fn get_input_mark<'a>(&self) -> u32 {
        self.input_mark.unwrap_or(0)
    }

    // required uint32 scancode = 2;

    pub fn clear_scancode(&mut self) {
        self.scancode = ::std::option::Option::None;
    }

    pub fn has_scancode(&self) -> bool {
        self.scancode.is_some()
    }

    // Param is passed by value, moved
    pub fn set_scancode(&mut self, v: u32) {
        self.scancode = ::std::option::Option::Some(v);
    }

    pub fn get_scancode<'a>(&self) -> u32 {
        self.scancode.unwrap_or(0)
    }
}

impl ::protobuf::Message for CInputKeyUpMsg {
    fn is_initialized(&self) -> bool {
        if self.scancode.is_none() {
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
                    self.input_mark = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_uint32());
                    self.scancode = ::std::option::Option::Some(tmp);
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
        for value in self.input_mark.iter() {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.scancode.iter() {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.input_mark {
            try!(os.write_uint32(1, v));
        };
        if let Some(v) = self.scancode {
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
        ::std::intrinsics::TypeId::of::<CInputKeyUpMsg>()
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CInputKeyUpMsg {
    fn new() -> CInputKeyUpMsg {
        CInputKeyUpMsg::new()
    }

    #[allow(unused_unsafe,unused_mut)]
    fn descriptor_static(_: ::std::option::Option<CInputKeyUpMsg>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_u32_accessor(
                    "input_mark",
                    CInputKeyUpMsg::has_input_mark,
                    CInputKeyUpMsg::get_input_mark,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u32_accessor(
                    "scancode",
                    CInputKeyUpMsg::has_scancode,
                    CInputKeyUpMsg::get_scancode,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CInputKeyUpMsg>(
                    "CInputKeyUpMsg",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CInputKeyUpMsg {
    fn clear(&mut self) {
        self.clear_input_mark();
        self.clear_scancode();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for CInputKeyUpMsg {
    fn eq(&self, other: &CInputKeyUpMsg) -> bool {
        self.input_mark == other.input_mark &&
        self.scancode == other.scancode &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Show for CInputKeyUpMsg {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct CInputGamepadAttachedMsg {
    controller_id: ::std::option::Option<i32>,
    controller_type: ::std::option::Option<u32>,
    controller_subtype: ::std::option::Option<u32>,
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl CInputGamepadAttachedMsg {
    pub fn new() -> CInputGamepadAttachedMsg {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CInputGamepadAttachedMsg {
        static mut instance: ::protobuf::lazy::Lazy<CInputGamepadAttachedMsg> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CInputGamepadAttachedMsg,
        };
        unsafe {
            instance.get(|| {
                CInputGamepadAttachedMsg {
                    controller_id: ::std::option::Option::None,
                    controller_type: ::std::option::Option::None,
                    controller_subtype: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required int32 controller_id = 1;

    pub fn clear_controller_id(&mut self) {
        self.controller_id = ::std::option::Option::None;
    }

    pub fn has_controller_id(&self) -> bool {
        self.controller_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_controller_id(&mut self, v: i32) {
        self.controller_id = ::std::option::Option::Some(v);
    }

    pub fn get_controller_id<'a>(&self) -> i32 {
        self.controller_id.unwrap_or(0)
    }

    // optional uint32 controller_type = 2;

    pub fn clear_controller_type(&mut self) {
        self.controller_type = ::std::option::Option::None;
    }

    pub fn has_controller_type(&self) -> bool {
        self.controller_type.is_some()
    }

    // Param is passed by value, moved
    pub fn set_controller_type(&mut self, v: u32) {
        self.controller_type = ::std::option::Option::Some(v);
    }

    pub fn get_controller_type<'a>(&self) -> u32 {
        self.controller_type.unwrap_or(0)
    }

    // optional uint32 controller_subtype = 3;

    pub fn clear_controller_subtype(&mut self) {
        self.controller_subtype = ::std::option::Option::None;
    }

    pub fn has_controller_subtype(&self) -> bool {
        self.controller_subtype.is_some()
    }

    // Param is passed by value, moved
    pub fn set_controller_subtype(&mut self, v: u32) {
        self.controller_subtype = ::std::option::Option::Some(v);
    }

    pub fn get_controller_subtype<'a>(&self) -> u32 {
        self.controller_subtype.unwrap_or(0)
    }
}

impl ::protobuf::Message for CInputGamepadAttachedMsg {
    fn is_initialized(&self) -> bool {
        if self.controller_id.is_none() {
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
                    let tmp = try!(is.read_int32());
                    self.controller_id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_uint32());
                    self.controller_type = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_uint32());
                    self.controller_subtype = ::std::option::Option::Some(tmp);
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
        for value in self.controller_id.iter() {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.controller_type.iter() {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.controller_subtype.iter() {
            my_size += ::protobuf::rt::value_size(3, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.controller_id {
            try!(os.write_int32(1, v));
        };
        if let Some(v) = self.controller_type {
            try!(os.write_uint32(2, v));
        };
        if let Some(v) = self.controller_subtype {
            try!(os.write_uint32(3, v));
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
        ::std::intrinsics::TypeId::of::<CInputGamepadAttachedMsg>()
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CInputGamepadAttachedMsg {
    fn new() -> CInputGamepadAttachedMsg {
        CInputGamepadAttachedMsg::new()
    }

    #[allow(unused_unsafe,unused_mut)]
    fn descriptor_static(_: ::std::option::Option<CInputGamepadAttachedMsg>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "controller_id",
                    CInputGamepadAttachedMsg::has_controller_id,
                    CInputGamepadAttachedMsg::get_controller_id,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u32_accessor(
                    "controller_type",
                    CInputGamepadAttachedMsg::has_controller_type,
                    CInputGamepadAttachedMsg::get_controller_type,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u32_accessor(
                    "controller_subtype",
                    CInputGamepadAttachedMsg::has_controller_subtype,
                    CInputGamepadAttachedMsg::get_controller_subtype,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CInputGamepadAttachedMsg>(
                    "CInputGamepadAttachedMsg",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CInputGamepadAttachedMsg {
    fn clear(&mut self) {
        self.clear_controller_id();
        self.clear_controller_type();
        self.clear_controller_subtype();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for CInputGamepadAttachedMsg {
    fn eq(&self, other: &CInputGamepadAttachedMsg) -> bool {
        self.controller_id == other.controller_id &&
        self.controller_type == other.controller_type &&
        self.controller_subtype == other.controller_subtype &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Show for CInputGamepadAttachedMsg {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct CInputGamepadEventMsg {
    input_mark: ::std::option::Option<u32>,
    controller_id: ::std::option::Option<i32>,
    input: ::std::option::Option<EStreamGamepadInputType>,
    value: ::std::option::Option<f32>,
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl CInputGamepadEventMsg {
    pub fn new() -> CInputGamepadEventMsg {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CInputGamepadEventMsg {
        static mut instance: ::protobuf::lazy::Lazy<CInputGamepadEventMsg> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CInputGamepadEventMsg,
        };
        unsafe {
            instance.get(|| {
                CInputGamepadEventMsg {
                    input_mark: ::std::option::Option::None,
                    controller_id: ::std::option::Option::None,
                    input: ::std::option::Option::None,
                    value: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional uint32 input_mark = 1;

    pub fn clear_input_mark(&mut self) {
        self.input_mark = ::std::option::Option::None;
    }

    pub fn has_input_mark(&self) -> bool {
        self.input_mark.is_some()
    }

    // Param is passed by value, moved
    pub fn set_input_mark(&mut self, v: u32) {
        self.input_mark = ::std::option::Option::Some(v);
    }

    pub fn get_input_mark<'a>(&self) -> u32 {
        self.input_mark.unwrap_or(0)
    }

    // required int32 controller_id = 2;

    pub fn clear_controller_id(&mut self) {
        self.controller_id = ::std::option::Option::None;
    }

    pub fn has_controller_id(&self) -> bool {
        self.controller_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_controller_id(&mut self, v: i32) {
        self.controller_id = ::std::option::Option::Some(v);
    }

    pub fn get_controller_id<'a>(&self) -> i32 {
        self.controller_id.unwrap_or(0)
    }

    // required .EStreamGamepadInputType input = 3;

    pub fn clear_input(&mut self) {
        self.input = ::std::option::Option::None;
    }

    pub fn has_input(&self) -> bool {
        self.input.is_some()
    }

    // Param is passed by value, moved
    pub fn set_input(&mut self, v: EStreamGamepadInputType) {
        self.input = ::std::option::Option::Some(v);
    }

    pub fn get_input<'a>(&self) -> EStreamGamepadInputType {
        self.input.unwrap_or(EStreamGamepadInputType::k_EStreamGamepadInputInvalid)
    }

    // required float value = 4;

    pub fn clear_value(&mut self) {
        self.value = ::std::option::Option::None;
    }

    pub fn has_value(&self) -> bool {
        self.value.is_some()
    }

    // Param is passed by value, moved
    pub fn set_value(&mut self, v: f32) {
        self.value = ::std::option::Option::Some(v);
    }

    pub fn get_value<'a>(&self) -> f32 {
        self.value.unwrap_or(0.)
    }
}

impl ::protobuf::Message for CInputGamepadEventMsg {
    fn is_initialized(&self) -> bool {
        if self.controller_id.is_none() {
            return false;
        };
        if self.input.is_none() {
            return false;
        };
        if self.value.is_none() {
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
                    self.input_mark = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_int32());
                    self.controller_id = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_enum());
                    self.input = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_float());
                    self.value = ::std::option::Option::Some(tmp);
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
        for value in self.input_mark.iter() {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.controller_id.iter() {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.input.iter() {
            my_size += ::protobuf::rt::enum_size(3, *value);
        };
        if self.value.is_some() {
            my_size += 5;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.input_mark {
            try!(os.write_uint32(1, v));
        };
        if let Some(v) = self.controller_id {
            try!(os.write_int32(2, v));
        };
        if let Some(v) = self.input {
            try!(os.write_enum(3, v as i32));
        };
        if let Some(v) = self.value {
            try!(os.write_float(4, v));
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
        ::std::intrinsics::TypeId::of::<CInputGamepadEventMsg>()
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CInputGamepadEventMsg {
    fn new() -> CInputGamepadEventMsg {
        CInputGamepadEventMsg::new()
    }

    #[allow(unused_unsafe,unused_mut)]
    fn descriptor_static(_: ::std::option::Option<CInputGamepadEventMsg>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_u32_accessor(
                    "input_mark",
                    CInputGamepadEventMsg::has_input_mark,
                    CInputGamepadEventMsg::get_input_mark,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "controller_id",
                    CInputGamepadEventMsg::has_controller_id,
                    CInputGamepadEventMsg::get_controller_id,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_enum_accessor(
                    "input",
                    CInputGamepadEventMsg::has_input,
                    CInputGamepadEventMsg::get_input,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f32_accessor(
                    "value",
                    CInputGamepadEventMsg::has_value,
                    CInputGamepadEventMsg::get_value,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CInputGamepadEventMsg>(
                    "CInputGamepadEventMsg",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CInputGamepadEventMsg {
    fn clear(&mut self) {
        self.clear_input_mark();
        self.clear_controller_id();
        self.clear_input();
        self.clear_value();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for CInputGamepadEventMsg {
    fn eq(&self, other: &CInputGamepadEventMsg) -> bool {
        self.input_mark == other.input_mark &&
        self.controller_id == other.controller_id &&
        self.input == other.input &&
        self.value == other.value &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Show for CInputGamepadEventMsg {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct CInputGamepadDetachedMsg {
    controller_id: ::std::option::Option<i32>,
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl CInputGamepadDetachedMsg {
    pub fn new() -> CInputGamepadDetachedMsg {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CInputGamepadDetachedMsg {
        static mut instance: ::protobuf::lazy::Lazy<CInputGamepadDetachedMsg> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CInputGamepadDetachedMsg,
        };
        unsafe {
            instance.get(|| {
                CInputGamepadDetachedMsg {
                    controller_id: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required int32 controller_id = 1;

    pub fn clear_controller_id(&mut self) {
        self.controller_id = ::std::option::Option::None;
    }

    pub fn has_controller_id(&self) -> bool {
        self.controller_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_controller_id(&mut self, v: i32) {
        self.controller_id = ::std::option::Option::Some(v);
    }

    pub fn get_controller_id<'a>(&self) -> i32 {
        self.controller_id.unwrap_or(0)
    }
}

impl ::protobuf::Message for CInputGamepadDetachedMsg {
    fn is_initialized(&self) -> bool {
        if self.controller_id.is_none() {
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
                    let tmp = try!(is.read_int32());
                    self.controller_id = ::std::option::Option::Some(tmp);
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
        for value in self.controller_id.iter() {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.controller_id {
            try!(os.write_int32(1, v));
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
        ::std::intrinsics::TypeId::of::<CInputGamepadDetachedMsg>()
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CInputGamepadDetachedMsg {
    fn new() -> CInputGamepadDetachedMsg {
        CInputGamepadDetachedMsg::new()
    }

    #[allow(unused_unsafe,unused_mut)]
    fn descriptor_static(_: ::std::option::Option<CInputGamepadDetachedMsg>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "controller_id",
                    CInputGamepadDetachedMsg::has_controller_id,
                    CInputGamepadDetachedMsg::get_controller_id,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CInputGamepadDetachedMsg>(
                    "CInputGamepadDetachedMsg",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CInputGamepadDetachedMsg {
    fn clear(&mut self) {
        self.clear_controller_id();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for CInputGamepadDetachedMsg {
    fn eq(&self, other: &CInputGamepadDetachedMsg) -> bool {
        self.controller_id == other.controller_id &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Show for CInputGamepadDetachedMsg {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct CGamepadRumbleMsg {
    controller_id: ::std::option::Option<i32>,
    left_motor_speed: ::std::option::Option<i32>,
    right_motor_speed: ::std::option::Option<i32>,
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl CGamepadRumbleMsg {
    pub fn new() -> CGamepadRumbleMsg {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CGamepadRumbleMsg {
        static mut instance: ::protobuf::lazy::Lazy<CGamepadRumbleMsg> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CGamepadRumbleMsg,
        };
        unsafe {
            instance.get(|| {
                CGamepadRumbleMsg {
                    controller_id: ::std::option::Option::None,
                    left_motor_speed: ::std::option::Option::None,
                    right_motor_speed: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required int32 controller_id = 1;

    pub fn clear_controller_id(&mut self) {
        self.controller_id = ::std::option::Option::None;
    }

    pub fn has_controller_id(&self) -> bool {
        self.controller_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_controller_id(&mut self, v: i32) {
        self.controller_id = ::std::option::Option::Some(v);
    }

    pub fn get_controller_id<'a>(&self) -> i32 {
        self.controller_id.unwrap_or(0)
    }

    // optional int32 left_motor_speed = 2;

    pub fn clear_left_motor_speed(&mut self) {
        self.left_motor_speed = ::std::option::Option::None;
    }

    pub fn has_left_motor_speed(&self) -> bool {
        self.left_motor_speed.is_some()
    }

    // Param is passed by value, moved
    pub fn set_left_motor_speed(&mut self, v: i32) {
        self.left_motor_speed = ::std::option::Option::Some(v);
    }

    pub fn get_left_motor_speed<'a>(&self) -> i32 {
        self.left_motor_speed.unwrap_or(0)
    }

    // optional int32 right_motor_speed = 3;

    pub fn clear_right_motor_speed(&mut self) {
        self.right_motor_speed = ::std::option::Option::None;
    }

    pub fn has_right_motor_speed(&self) -> bool {
        self.right_motor_speed.is_some()
    }

    // Param is passed by value, moved
    pub fn set_right_motor_speed(&mut self, v: i32) {
        self.right_motor_speed = ::std::option::Option::Some(v);
    }

    pub fn get_right_motor_speed<'a>(&self) -> i32 {
        self.right_motor_speed.unwrap_or(0)
    }
}

impl ::protobuf::Message for CGamepadRumbleMsg {
    fn is_initialized(&self) -> bool {
        if self.controller_id.is_none() {
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
                    let tmp = try!(is.read_int32());
                    self.controller_id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_int32());
                    self.left_motor_speed = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_int32());
                    self.right_motor_speed = ::std::option::Option::Some(tmp);
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
        for value in self.controller_id.iter() {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.left_motor_speed.iter() {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.right_motor_speed.iter() {
            my_size += ::protobuf::rt::value_size(3, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.controller_id {
            try!(os.write_int32(1, v));
        };
        if let Some(v) = self.left_motor_speed {
            try!(os.write_int32(2, v));
        };
        if let Some(v) = self.right_motor_speed {
            try!(os.write_int32(3, v));
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
        ::std::intrinsics::TypeId::of::<CGamepadRumbleMsg>()
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CGamepadRumbleMsg {
    fn new() -> CGamepadRumbleMsg {
        CGamepadRumbleMsg::new()
    }

    #[allow(unused_unsafe,unused_mut)]
    fn descriptor_static(_: ::std::option::Option<CGamepadRumbleMsg>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "controller_id",
                    CGamepadRumbleMsg::has_controller_id,
                    CGamepadRumbleMsg::get_controller_id,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "left_motor_speed",
                    CGamepadRumbleMsg::has_left_motor_speed,
                    CGamepadRumbleMsg::get_left_motor_speed,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "right_motor_speed",
                    CGamepadRumbleMsg::has_right_motor_speed,
                    CGamepadRumbleMsg::get_right_motor_speed,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CGamepadRumbleMsg>(
                    "CGamepadRumbleMsg",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CGamepadRumbleMsg {
    fn clear(&mut self) {
        self.clear_controller_id();
        self.clear_left_motor_speed();
        self.clear_right_motor_speed();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for CGamepadRumbleMsg {
    fn eq(&self, other: &CGamepadRumbleMsg) -> bool {
        self.controller_id == other.controller_id &&
        self.left_motor_speed == other.left_motor_speed &&
        self.right_motor_speed == other.right_motor_speed &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Show for CGamepadRumbleMsg {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct CSetTitleMsg {
    text: ::protobuf::SingularField<::std::string::String>,
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl CSetTitleMsg {
    pub fn new() -> CSetTitleMsg {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CSetTitleMsg {
        static mut instance: ::protobuf::lazy::Lazy<CSetTitleMsg> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CSetTitleMsg,
        };
        unsafe {
            instance.get(|| {
                CSetTitleMsg {
                    text: ::protobuf::SingularField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional string text = 1;

    pub fn clear_text(&mut self) {
        self.text.clear();
    }

    pub fn has_text(&self) -> bool {
        self.text.is_some()
    }

    // Param is passed by value, moved
    pub fn set_text(&mut self, v: ::std::string::String) {
        self.text = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_text<'a>(&'a mut self) -> &'a mut ::std::string::String {
        if self.text.is_none() {
            self.text.set_default();
        };
        self.text.as_mut().unwrap()
    }

    // Take field
    pub fn take_text(&mut self) -> ::std::string::String {
        self.text.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_text<'a>(&'a self) -> &'a str {
        match self.text.as_ref() {
            Some(v) => v.as_slice(),
            None => "",
        }
    }
}

impl ::protobuf::Message for CSetTitleMsg {
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
                    let tmp = self.text.set_default();
                    try!(is.read_string_into(tmp))
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
        for value in self.text.iter() {
            my_size += ::protobuf::rt::string_size(1, value.as_slice());
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.text.as_ref() {
            try!(os.write_string(1, v.as_slice()));
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
        ::std::intrinsics::TypeId::of::<CSetTitleMsg>()
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CSetTitleMsg {
    fn new() -> CSetTitleMsg {
        CSetTitleMsg::new()
    }

    #[allow(unused_unsafe,unused_mut)]
    fn descriptor_static(_: ::std::option::Option<CSetTitleMsg>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "text",
                    CSetTitleMsg::has_text,
                    CSetTitleMsg::get_text,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CSetTitleMsg>(
                    "CSetTitleMsg",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CSetTitleMsg {
    fn clear(&mut self) {
        self.clear_text();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for CSetTitleMsg {
    fn eq(&self, other: &CSetTitleMsg) -> bool {
        self.text == other.text &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Show for CSetTitleMsg {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct CSetIconMsg {
    width: ::std::option::Option<i32>,
    height: ::std::option::Option<i32>,
    image: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl CSetIconMsg {
    pub fn new() -> CSetIconMsg {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CSetIconMsg {
        static mut instance: ::protobuf::lazy::Lazy<CSetIconMsg> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CSetIconMsg,
        };
        unsafe {
            instance.get(|| {
                CSetIconMsg {
                    width: ::std::option::Option::None,
                    height: ::std::option::Option::None,
                    image: ::protobuf::SingularField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional int32 width = 1;

    pub fn clear_width(&mut self) {
        self.width = ::std::option::Option::None;
    }

    pub fn has_width(&self) -> bool {
        self.width.is_some()
    }

    // Param is passed by value, moved
    pub fn set_width(&mut self, v: i32) {
        self.width = ::std::option::Option::Some(v);
    }

    pub fn get_width<'a>(&self) -> i32 {
        self.width.unwrap_or(0)
    }

    // optional int32 height = 2;

    pub fn clear_height(&mut self) {
        self.height = ::std::option::Option::None;
    }

    pub fn has_height(&self) -> bool {
        self.height.is_some()
    }

    // Param is passed by value, moved
    pub fn set_height(&mut self, v: i32) {
        self.height = ::std::option::Option::Some(v);
    }

    pub fn get_height<'a>(&self) -> i32 {
        self.height.unwrap_or(0)
    }

    // optional bytes image = 3;

    pub fn clear_image(&mut self) {
        self.image.clear();
    }

    pub fn has_image(&self) -> bool {
        self.image.is_some()
    }

    // Param is passed by value, moved
    pub fn set_image(&mut self, v: ::std::vec::Vec<u8>) {
        self.image = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_image<'a>(&'a mut self) -> &'a mut ::std::vec::Vec<u8> {
        if self.image.is_none() {
            self.image.set_default();
        };
        self.image.as_mut().unwrap()
    }

    // Take field
    pub fn take_image(&mut self) -> ::std::vec::Vec<u8> {
        self.image.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_image<'a>(&'a self) -> &'a [u8] {
        match self.image.as_ref() {
            Some(v) => v.as_slice(),
            None => [].as_slice(),
        }
    }
}

impl ::protobuf::Message for CSetIconMsg {
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
                    self.width = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_int32());
                    self.height = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.image.set_default();
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
        for value in self.width.iter() {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.height.iter() {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.image.iter() {
            my_size += ::protobuf::rt::bytes_size(3, value.as_slice());
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.width {
            try!(os.write_int32(1, v));
        };
        if let Some(v) = self.height {
            try!(os.write_int32(2, v));
        };
        if let Some(v) = self.image.as_ref() {
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
        ::std::intrinsics::TypeId::of::<CSetIconMsg>()
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CSetIconMsg {
    fn new() -> CSetIconMsg {
        CSetIconMsg::new()
    }

    #[allow(unused_unsafe,unused_mut)]
    fn descriptor_static(_: ::std::option::Option<CSetIconMsg>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "width",
                    CSetIconMsg::has_width,
                    CSetIconMsg::get_width,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "height",
                    CSetIconMsg::has_height,
                    CSetIconMsg::get_height,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "image",
                    CSetIconMsg::has_image,
                    CSetIconMsg::get_image,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CSetIconMsg>(
                    "CSetIconMsg",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CSetIconMsg {
    fn clear(&mut self) {
        self.clear_width();
        self.clear_height();
        self.clear_image();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for CSetIconMsg {
    fn eq(&self, other: &CSetIconMsg) -> bool {
        self.width == other.width &&
        self.height == other.height &&
        self.image == other.image &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Show for CSetIconMsg {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct CShowCursorMsg {
    x_normalized: ::std::option::Option<f32>,
    y_normalized: ::std::option::Option<f32>,
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl CShowCursorMsg {
    pub fn new() -> CShowCursorMsg {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CShowCursorMsg {
        static mut instance: ::protobuf::lazy::Lazy<CShowCursorMsg> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CShowCursorMsg,
        };
        unsafe {
            instance.get(|| {
                CShowCursorMsg {
                    x_normalized: ::std::option::Option::None,
                    y_normalized: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional float x_normalized = 1;

    pub fn clear_x_normalized(&mut self) {
        self.x_normalized = ::std::option::Option::None;
    }

    pub fn has_x_normalized(&self) -> bool {
        self.x_normalized.is_some()
    }

    // Param is passed by value, moved
    pub fn set_x_normalized(&mut self, v: f32) {
        self.x_normalized = ::std::option::Option::Some(v);
    }

    pub fn get_x_normalized<'a>(&self) -> f32 {
        self.x_normalized.unwrap_or(0.)
    }

    // optional float y_normalized = 2;

    pub fn clear_y_normalized(&mut self) {
        self.y_normalized = ::std::option::Option::None;
    }

    pub fn has_y_normalized(&self) -> bool {
        self.y_normalized.is_some()
    }

    // Param is passed by value, moved
    pub fn set_y_normalized(&mut self, v: f32) {
        self.y_normalized = ::std::option::Option::Some(v);
    }

    pub fn get_y_normalized<'a>(&self) -> f32 {
        self.y_normalized.unwrap_or(0.)
    }
}

impl ::protobuf::Message for CShowCursorMsg {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_float());
                    self.x_normalized = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_float());
                    self.y_normalized = ::std::option::Option::Some(tmp);
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
        if self.x_normalized.is_some() {
            my_size += 5;
        };
        if self.y_normalized.is_some() {
            my_size += 5;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.x_normalized {
            try!(os.write_float(1, v));
        };
        if let Some(v) = self.y_normalized {
            try!(os.write_float(2, v));
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
        ::std::intrinsics::TypeId::of::<CShowCursorMsg>()
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CShowCursorMsg {
    fn new() -> CShowCursorMsg {
        CShowCursorMsg::new()
    }

    #[allow(unused_unsafe,unused_mut)]
    fn descriptor_static(_: ::std::option::Option<CShowCursorMsg>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_f32_accessor(
                    "x_normalized",
                    CShowCursorMsg::has_x_normalized,
                    CShowCursorMsg::get_x_normalized,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f32_accessor(
                    "y_normalized",
                    CShowCursorMsg::has_y_normalized,
                    CShowCursorMsg::get_y_normalized,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CShowCursorMsg>(
                    "CShowCursorMsg",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CShowCursorMsg {
    fn clear(&mut self) {
        self.clear_x_normalized();
        self.clear_y_normalized();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for CShowCursorMsg {
    fn eq(&self, other: &CShowCursorMsg) -> bool {
        self.x_normalized == other.x_normalized &&
        self.y_normalized == other.y_normalized &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Show for CShowCursorMsg {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct CHideCursorMsg {
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl CHideCursorMsg {
    pub fn new() -> CHideCursorMsg {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CHideCursorMsg {
        static mut instance: ::protobuf::lazy::Lazy<CHideCursorMsg> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CHideCursorMsg,
        };
        unsafe {
            instance.get(|| {
                CHideCursorMsg {
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }
}

impl ::protobuf::Message for CHideCursorMsg {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
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
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
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
        ::std::intrinsics::TypeId::of::<CHideCursorMsg>()
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CHideCursorMsg {
    fn new() -> CHideCursorMsg {
        CHideCursorMsg::new()
    }

    #[allow(unused_unsafe,unused_mut)]
    fn descriptor_static(_: ::std::option::Option<CHideCursorMsg>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<CHideCursorMsg>(
                    "CHideCursorMsg",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CHideCursorMsg {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for CHideCursorMsg {
    fn eq(&self, other: &CHideCursorMsg) -> bool {
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Show for CHideCursorMsg {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct CSetCursorMsg {
    cursor_id: ::std::option::Option<u64>,
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl CSetCursorMsg {
    pub fn new() -> CSetCursorMsg {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CSetCursorMsg {
        static mut instance: ::protobuf::lazy::Lazy<CSetCursorMsg> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CSetCursorMsg,
        };
        unsafe {
            instance.get(|| {
                CSetCursorMsg {
                    cursor_id: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required uint64 cursor_id = 1;

    pub fn clear_cursor_id(&mut self) {
        self.cursor_id = ::std::option::Option::None;
    }

    pub fn has_cursor_id(&self) -> bool {
        self.cursor_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_cursor_id(&mut self, v: u64) {
        self.cursor_id = ::std::option::Option::Some(v);
    }

    pub fn get_cursor_id<'a>(&self) -> u64 {
        self.cursor_id.unwrap_or(0)
    }
}

impl ::protobuf::Message for CSetCursorMsg {
    fn is_initialized(&self) -> bool {
        if self.cursor_id.is_none() {
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
                    let tmp = try!(is.read_uint64());
                    self.cursor_id = ::std::option::Option::Some(tmp);
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
        for value in self.cursor_id.iter() {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.cursor_id {
            try!(os.write_uint64(1, v));
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
        ::std::intrinsics::TypeId::of::<CSetCursorMsg>()
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CSetCursorMsg {
    fn new() -> CSetCursorMsg {
        CSetCursorMsg::new()
    }

    #[allow(unused_unsafe,unused_mut)]
    fn descriptor_static(_: ::std::option::Option<CSetCursorMsg>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "cursor_id",
                    CSetCursorMsg::has_cursor_id,
                    CSetCursorMsg::get_cursor_id,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CSetCursorMsg>(
                    "CSetCursorMsg",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CSetCursorMsg {
    fn clear(&mut self) {
        self.clear_cursor_id();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for CSetCursorMsg {
    fn eq(&self, other: &CSetCursorMsg) -> bool {
        self.cursor_id == other.cursor_id &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Show for CSetCursorMsg {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct CGetCursorImageMsg {
    cursor_id: ::std::option::Option<u64>,
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl CGetCursorImageMsg {
    pub fn new() -> CGetCursorImageMsg {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CGetCursorImageMsg {
        static mut instance: ::protobuf::lazy::Lazy<CGetCursorImageMsg> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CGetCursorImageMsg,
        };
        unsafe {
            instance.get(|| {
                CGetCursorImageMsg {
                    cursor_id: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required uint64 cursor_id = 1;

    pub fn clear_cursor_id(&mut self) {
        self.cursor_id = ::std::option::Option::None;
    }

    pub fn has_cursor_id(&self) -> bool {
        self.cursor_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_cursor_id(&mut self, v: u64) {
        self.cursor_id = ::std::option::Option::Some(v);
    }

    pub fn get_cursor_id<'a>(&self) -> u64 {
        self.cursor_id.unwrap_or(0)
    }
}

impl ::protobuf::Message for CGetCursorImageMsg {
    fn is_initialized(&self) -> bool {
        if self.cursor_id.is_none() {
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
                    let tmp = try!(is.read_uint64());
                    self.cursor_id = ::std::option::Option::Some(tmp);
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
        for value in self.cursor_id.iter() {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.cursor_id {
            try!(os.write_uint64(1, v));
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
        ::std::intrinsics::TypeId::of::<CGetCursorImageMsg>()
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CGetCursorImageMsg {
    fn new() -> CGetCursorImageMsg {
        CGetCursorImageMsg::new()
    }

    #[allow(unused_unsafe,unused_mut)]
    fn descriptor_static(_: ::std::option::Option<CGetCursorImageMsg>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "cursor_id",
                    CGetCursorImageMsg::has_cursor_id,
                    CGetCursorImageMsg::get_cursor_id,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CGetCursorImageMsg>(
                    "CGetCursorImageMsg",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CGetCursorImageMsg {
    fn clear(&mut self) {
        self.clear_cursor_id();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for CGetCursorImageMsg {
    fn eq(&self, other: &CGetCursorImageMsg) -> bool {
        self.cursor_id == other.cursor_id &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Show for CGetCursorImageMsg {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct CSetCursorImageMsg {
    cursor_id: ::std::option::Option<u64>,
    width: ::std::option::Option<i32>,
    height: ::std::option::Option<i32>,
    hot_x: ::std::option::Option<i32>,
    hot_y: ::std::option::Option<i32>,
    image: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl CSetCursorImageMsg {
    pub fn new() -> CSetCursorImageMsg {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CSetCursorImageMsg {
        static mut instance: ::protobuf::lazy::Lazy<CSetCursorImageMsg> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CSetCursorImageMsg,
        };
        unsafe {
            instance.get(|| {
                CSetCursorImageMsg {
                    cursor_id: ::std::option::Option::None,
                    width: ::std::option::Option::None,
                    height: ::std::option::Option::None,
                    hot_x: ::std::option::Option::None,
                    hot_y: ::std::option::Option::None,
                    image: ::protobuf::SingularField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required uint64 cursor_id = 1;

    pub fn clear_cursor_id(&mut self) {
        self.cursor_id = ::std::option::Option::None;
    }

    pub fn has_cursor_id(&self) -> bool {
        self.cursor_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_cursor_id(&mut self, v: u64) {
        self.cursor_id = ::std::option::Option::Some(v);
    }

    pub fn get_cursor_id<'a>(&self) -> u64 {
        self.cursor_id.unwrap_or(0)
    }

    // optional int32 width = 2;

    pub fn clear_width(&mut self) {
        self.width = ::std::option::Option::None;
    }

    pub fn has_width(&self) -> bool {
        self.width.is_some()
    }

    // Param is passed by value, moved
    pub fn set_width(&mut self, v: i32) {
        self.width = ::std::option::Option::Some(v);
    }

    pub fn get_width<'a>(&self) -> i32 {
        self.width.unwrap_or(0)
    }

    // optional int32 height = 3;

    pub fn clear_height(&mut self) {
        self.height = ::std::option::Option::None;
    }

    pub fn has_height(&self) -> bool {
        self.height.is_some()
    }

    // Param is passed by value, moved
    pub fn set_height(&mut self, v: i32) {
        self.height = ::std::option::Option::Some(v);
    }

    pub fn get_height<'a>(&self) -> i32 {
        self.height.unwrap_or(0)
    }

    // optional int32 hot_x = 4;

    pub fn clear_hot_x(&mut self) {
        self.hot_x = ::std::option::Option::None;
    }

    pub fn has_hot_x(&self) -> bool {
        self.hot_x.is_some()
    }

    // Param is passed by value, moved
    pub fn set_hot_x(&mut self, v: i32) {
        self.hot_x = ::std::option::Option::Some(v);
    }

    pub fn get_hot_x<'a>(&self) -> i32 {
        self.hot_x.unwrap_or(0)
    }

    // optional int32 hot_y = 5;

    pub fn clear_hot_y(&mut self) {
        self.hot_y = ::std::option::Option::None;
    }

    pub fn has_hot_y(&self) -> bool {
        self.hot_y.is_some()
    }

    // Param is passed by value, moved
    pub fn set_hot_y(&mut self, v: i32) {
        self.hot_y = ::std::option::Option::Some(v);
    }

    pub fn get_hot_y<'a>(&self) -> i32 {
        self.hot_y.unwrap_or(0)
    }

    // optional bytes image = 6;

    pub fn clear_image(&mut self) {
        self.image.clear();
    }

    pub fn has_image(&self) -> bool {
        self.image.is_some()
    }

    // Param is passed by value, moved
    pub fn set_image(&mut self, v: ::std::vec::Vec<u8>) {
        self.image = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_image<'a>(&'a mut self) -> &'a mut ::std::vec::Vec<u8> {
        if self.image.is_none() {
            self.image.set_default();
        };
        self.image.as_mut().unwrap()
    }

    // Take field
    pub fn take_image(&mut self) -> ::std::vec::Vec<u8> {
        self.image.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_image<'a>(&'a self) -> &'a [u8] {
        match self.image.as_ref() {
            Some(v) => v.as_slice(),
            None => [].as_slice(),
        }
    }
}

impl ::protobuf::Message for CSetCursorImageMsg {
    fn is_initialized(&self) -> bool {
        if self.cursor_id.is_none() {
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
                    let tmp = try!(is.read_uint64());
                    self.cursor_id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_int32());
                    self.width = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_int32());
                    self.height = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_int32());
                    self.hot_x = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_int32());
                    self.hot_y = ::std::option::Option::Some(tmp);
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.image.set_default();
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
        for value in self.cursor_id.iter() {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.width.iter() {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.height.iter() {
            my_size += ::protobuf::rt::value_size(3, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.hot_x.iter() {
            my_size += ::protobuf::rt::value_size(4, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.hot_y.iter() {
            my_size += ::protobuf::rt::value_size(5, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.image.iter() {
            my_size += ::protobuf::rt::bytes_size(6, value.as_slice());
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.cursor_id {
            try!(os.write_uint64(1, v));
        };
        if let Some(v) = self.width {
            try!(os.write_int32(2, v));
        };
        if let Some(v) = self.height {
            try!(os.write_int32(3, v));
        };
        if let Some(v) = self.hot_x {
            try!(os.write_int32(4, v));
        };
        if let Some(v) = self.hot_y {
            try!(os.write_int32(5, v));
        };
        if let Some(v) = self.image.as_ref() {
            try!(os.write_bytes(6, v.as_slice()));
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
        ::std::intrinsics::TypeId::of::<CSetCursorImageMsg>()
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CSetCursorImageMsg {
    fn new() -> CSetCursorImageMsg {
        CSetCursorImageMsg::new()
    }

    #[allow(unused_unsafe,unused_mut)]
    fn descriptor_static(_: ::std::option::Option<CSetCursorImageMsg>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "cursor_id",
                    CSetCursorImageMsg::has_cursor_id,
                    CSetCursorImageMsg::get_cursor_id,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "width",
                    CSetCursorImageMsg::has_width,
                    CSetCursorImageMsg::get_width,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "height",
                    CSetCursorImageMsg::has_height,
                    CSetCursorImageMsg::get_height,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "hot_x",
                    CSetCursorImageMsg::has_hot_x,
                    CSetCursorImageMsg::get_hot_x,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "hot_y",
                    CSetCursorImageMsg::has_hot_y,
                    CSetCursorImageMsg::get_hot_y,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "image",
                    CSetCursorImageMsg::has_image,
                    CSetCursorImageMsg::get_image,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CSetCursorImageMsg>(
                    "CSetCursorImageMsg",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CSetCursorImageMsg {
    fn clear(&mut self) {
        self.clear_cursor_id();
        self.clear_width();
        self.clear_height();
        self.clear_hot_x();
        self.clear_hot_y();
        self.clear_image();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for CSetCursorImageMsg {
    fn eq(&self, other: &CSetCursorImageMsg) -> bool {
        self.cursor_id == other.cursor_id &&
        self.width == other.width &&
        self.height == other.height &&
        self.hot_x == other.hot_x &&
        self.hot_y == other.hot_y &&
        self.image == other.image &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Show for CSetCursorImageMsg {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct CSystemInfoMsg {
    info: ::protobuf::SingularField<::std::string::String>,
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl CSystemInfoMsg {
    pub fn new() -> CSystemInfoMsg {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CSystemInfoMsg {
        static mut instance: ::protobuf::lazy::Lazy<CSystemInfoMsg> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CSystemInfoMsg,
        };
        unsafe {
            instance.get(|| {
                CSystemInfoMsg {
                    info: ::protobuf::SingularField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional string info = 1;

    pub fn clear_info(&mut self) {
        self.info.clear();
    }

    pub fn has_info(&self) -> bool {
        self.info.is_some()
    }

    // Param is passed by value, moved
    pub fn set_info(&mut self, v: ::std::string::String) {
        self.info = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_info<'a>(&'a mut self) -> &'a mut ::std::string::String {
        if self.info.is_none() {
            self.info.set_default();
        };
        self.info.as_mut().unwrap()
    }

    // Take field
    pub fn take_info(&mut self) -> ::std::string::String {
        self.info.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_info<'a>(&'a self) -> &'a str {
        match self.info.as_ref() {
            Some(v) => v.as_slice(),
            None => "",
        }
    }
}

impl ::protobuf::Message for CSystemInfoMsg {
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
                    let tmp = self.info.set_default();
                    try!(is.read_string_into(tmp))
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
        for value in self.info.iter() {
            my_size += ::protobuf::rt::string_size(1, value.as_slice());
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.info.as_ref() {
            try!(os.write_string(1, v.as_slice()));
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
        ::std::intrinsics::TypeId::of::<CSystemInfoMsg>()
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CSystemInfoMsg {
    fn new() -> CSystemInfoMsg {
        CSystemInfoMsg::new()
    }

    #[allow(unused_unsafe,unused_mut)]
    fn descriptor_static(_: ::std::option::Option<CSystemInfoMsg>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "info",
                    CSystemInfoMsg::has_info,
                    CSystemInfoMsg::get_info,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CSystemInfoMsg>(
                    "CSystemInfoMsg",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CSystemInfoMsg {
    fn clear(&mut self) {
        self.clear_info();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for CSystemInfoMsg {
    fn eq(&self, other: &CSystemInfoMsg) -> bool {
        self.info == other.info &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Show for CSystemInfoMsg {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct CVideoDecoderInfoMsg {
    info: ::protobuf::SingularField<::std::string::String>,
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl CVideoDecoderInfoMsg {
    pub fn new() -> CVideoDecoderInfoMsg {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CVideoDecoderInfoMsg {
        static mut instance: ::protobuf::lazy::Lazy<CVideoDecoderInfoMsg> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CVideoDecoderInfoMsg,
        };
        unsafe {
            instance.get(|| {
                CVideoDecoderInfoMsg {
                    info: ::protobuf::SingularField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional string info = 1;

    pub fn clear_info(&mut self) {
        self.info.clear();
    }

    pub fn has_info(&self) -> bool {
        self.info.is_some()
    }

    // Param is passed by value, moved
    pub fn set_info(&mut self, v: ::std::string::String) {
        self.info = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_info<'a>(&'a mut self) -> &'a mut ::std::string::String {
        if self.info.is_none() {
            self.info.set_default();
        };
        self.info.as_mut().unwrap()
    }

    // Take field
    pub fn take_info(&mut self) -> ::std::string::String {
        self.info.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_info<'a>(&'a self) -> &'a str {
        match self.info.as_ref() {
            Some(v) => v.as_slice(),
            None => "",
        }
    }
}

impl ::protobuf::Message for CVideoDecoderInfoMsg {
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
                    let tmp = self.info.set_default();
                    try!(is.read_string_into(tmp))
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
        for value in self.info.iter() {
            my_size += ::protobuf::rt::string_size(1, value.as_slice());
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.info.as_ref() {
            try!(os.write_string(1, v.as_slice()));
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
        ::std::intrinsics::TypeId::of::<CVideoDecoderInfoMsg>()
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CVideoDecoderInfoMsg {
    fn new() -> CVideoDecoderInfoMsg {
        CVideoDecoderInfoMsg::new()
    }

    #[allow(unused_unsafe,unused_mut)]
    fn descriptor_static(_: ::std::option::Option<CVideoDecoderInfoMsg>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "info",
                    CVideoDecoderInfoMsg::has_info,
                    CVideoDecoderInfoMsg::get_info,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CVideoDecoderInfoMsg>(
                    "CVideoDecoderInfoMsg",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CVideoDecoderInfoMsg {
    fn clear(&mut self) {
        self.clear_info();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for CVideoDecoderInfoMsg {
    fn eq(&self, other: &CVideoDecoderInfoMsg) -> bool {
        self.info == other.info &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Show for CVideoDecoderInfoMsg {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct CVideoEncoderInfoMsg {
    info: ::protobuf::SingularField<::std::string::String>,
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl CVideoEncoderInfoMsg {
    pub fn new() -> CVideoEncoderInfoMsg {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CVideoEncoderInfoMsg {
        static mut instance: ::protobuf::lazy::Lazy<CVideoEncoderInfoMsg> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CVideoEncoderInfoMsg,
        };
        unsafe {
            instance.get(|| {
                CVideoEncoderInfoMsg {
                    info: ::protobuf::SingularField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional string info = 1;

    pub fn clear_info(&mut self) {
        self.info.clear();
    }

    pub fn has_info(&self) -> bool {
        self.info.is_some()
    }

    // Param is passed by value, moved
    pub fn set_info(&mut self, v: ::std::string::String) {
        self.info = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_info<'a>(&'a mut self) -> &'a mut ::std::string::String {
        if self.info.is_none() {
            self.info.set_default();
        };
        self.info.as_mut().unwrap()
    }

    // Take field
    pub fn take_info(&mut self) -> ::std::string::String {
        self.info.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_info<'a>(&'a self) -> &'a str {
        match self.info.as_ref() {
            Some(v) => v.as_slice(),
            None => "",
        }
    }
}

impl ::protobuf::Message for CVideoEncoderInfoMsg {
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
                    let tmp = self.info.set_default();
                    try!(is.read_string_into(tmp))
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
        for value in self.info.iter() {
            my_size += ::protobuf::rt::string_size(1, value.as_slice());
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.info.as_ref() {
            try!(os.write_string(1, v.as_slice()));
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
        ::std::intrinsics::TypeId::of::<CVideoEncoderInfoMsg>()
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CVideoEncoderInfoMsg {
    fn new() -> CVideoEncoderInfoMsg {
        CVideoEncoderInfoMsg::new()
    }

    #[allow(unused_unsafe,unused_mut)]
    fn descriptor_static(_: ::std::option::Option<CVideoEncoderInfoMsg>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "info",
                    CVideoEncoderInfoMsg::has_info,
                    CVideoEncoderInfoMsg::get_info,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CVideoEncoderInfoMsg>(
                    "CVideoEncoderInfoMsg",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CVideoEncoderInfoMsg {
    fn clear(&mut self) {
        self.clear_info();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for CVideoEncoderInfoMsg {
    fn eq(&self, other: &CVideoEncoderInfoMsg) -> bool {
        self.info == other.info &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Show for CVideoEncoderInfoMsg {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct CQuitRequest {
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl CQuitRequest {
    pub fn new() -> CQuitRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CQuitRequest {
        static mut instance: ::protobuf::lazy::Lazy<CQuitRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CQuitRequest,
        };
        unsafe {
            instance.get(|| {
                CQuitRequest {
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }
}

impl ::protobuf::Message for CQuitRequest {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
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
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
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
        ::std::intrinsics::TypeId::of::<CQuitRequest>()
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CQuitRequest {
    fn new() -> CQuitRequest {
        CQuitRequest::new()
    }

    #[allow(unused_unsafe,unused_mut)]
    fn descriptor_static(_: ::std::option::Option<CQuitRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<CQuitRequest>(
                    "CQuitRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CQuitRequest {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for CQuitRequest {
    fn eq(&self, other: &CQuitRequest) -> bool {
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Show for CQuitRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct CDeleteCursorMsg {
    cursor_id: ::std::option::Option<u64>,
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl CDeleteCursorMsg {
    pub fn new() -> CDeleteCursorMsg {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CDeleteCursorMsg {
        static mut instance: ::protobuf::lazy::Lazy<CDeleteCursorMsg> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CDeleteCursorMsg,
        };
        unsafe {
            instance.get(|| {
                CDeleteCursorMsg {
                    cursor_id: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required uint64 cursor_id = 1;

    pub fn clear_cursor_id(&mut self) {
        self.cursor_id = ::std::option::Option::None;
    }

    pub fn has_cursor_id(&self) -> bool {
        self.cursor_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_cursor_id(&mut self, v: u64) {
        self.cursor_id = ::std::option::Option::Some(v);
    }

    pub fn get_cursor_id<'a>(&self) -> u64 {
        self.cursor_id.unwrap_or(0)
    }
}

impl ::protobuf::Message for CDeleteCursorMsg {
    fn is_initialized(&self) -> bool {
        if self.cursor_id.is_none() {
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
                    let tmp = try!(is.read_uint64());
                    self.cursor_id = ::std::option::Option::Some(tmp);
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
        for value in self.cursor_id.iter() {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.cursor_id {
            try!(os.write_uint64(1, v));
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
        ::std::intrinsics::TypeId::of::<CDeleteCursorMsg>()
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CDeleteCursorMsg {
    fn new() -> CDeleteCursorMsg {
        CDeleteCursorMsg::new()
    }

    #[allow(unused_unsafe,unused_mut)]
    fn descriptor_static(_: ::std::option::Option<CDeleteCursorMsg>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "cursor_id",
                    CDeleteCursorMsg::has_cursor_id,
                    CDeleteCursorMsg::get_cursor_id,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CDeleteCursorMsg>(
                    "CDeleteCursorMsg",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CDeleteCursorMsg {
    fn clear(&mut self) {
        self.clear_cursor_id();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for CDeleteCursorMsg {
    fn eq(&self, other: &CDeleteCursorMsg) -> bool {
        self.cursor_id == other.cursor_id &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Show for CDeleteCursorMsg {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct CSetQualityPreferenceMsg {
    quality: ::std::option::Option<EStreamQualityPreference>,
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl CSetQualityPreferenceMsg {
    pub fn new() -> CSetQualityPreferenceMsg {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CSetQualityPreferenceMsg {
        static mut instance: ::protobuf::lazy::Lazy<CSetQualityPreferenceMsg> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CSetQualityPreferenceMsg,
        };
        unsafe {
            instance.get(|| {
                CSetQualityPreferenceMsg {
                    quality: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required .EStreamQualityPreference quality = 1;

    pub fn clear_quality(&mut self) {
        self.quality = ::std::option::Option::None;
    }

    pub fn has_quality(&self) -> bool {
        self.quality.is_some()
    }

    // Param is passed by value, moved
    pub fn set_quality(&mut self, v: EStreamQualityPreference) {
        self.quality = ::std::option::Option::Some(v);
    }

    pub fn get_quality<'a>(&self) -> EStreamQualityPreference {
        self.quality.unwrap_or(EStreamQualityPreference::k_EStreamQualityFast)
    }
}

impl ::protobuf::Message for CSetQualityPreferenceMsg {
    fn is_initialized(&self) -> bool {
        if self.quality.is_none() {
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
                    self.quality = ::std::option::Option::Some(tmp);
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
        for value in self.quality.iter() {
            my_size += ::protobuf::rt::enum_size(1, *value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.quality {
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
        ::std::intrinsics::TypeId::of::<CSetQualityPreferenceMsg>()
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CSetQualityPreferenceMsg {
    fn new() -> CSetQualityPreferenceMsg {
        CSetQualityPreferenceMsg::new()
    }

    #[allow(unused_unsafe,unused_mut)]
    fn descriptor_static(_: ::std::option::Option<CSetQualityPreferenceMsg>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_enum_accessor(
                    "quality",
                    CSetQualityPreferenceMsg::has_quality,
                    CSetQualityPreferenceMsg::get_quality,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CSetQualityPreferenceMsg>(
                    "CSetQualityPreferenceMsg",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CSetQualityPreferenceMsg {
    fn clear(&mut self) {
        self.clear_quality();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for CSetQualityPreferenceMsg {
    fn eq(&self, other: &CSetQualityPreferenceMsg) -> bool {
        self.quality == other.quality &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Show for CSetQualityPreferenceMsg {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct CSetMaximumResolutionMsg {
    x: ::std::option::Option<u32>,
    y: ::std::option::Option<u32>,
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl CSetMaximumResolutionMsg {
    pub fn new() -> CSetMaximumResolutionMsg {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CSetMaximumResolutionMsg {
        static mut instance: ::protobuf::lazy::Lazy<CSetMaximumResolutionMsg> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CSetMaximumResolutionMsg,
        };
        unsafe {
            instance.get(|| {
                CSetMaximumResolutionMsg {
                    x: ::std::option::Option::None,
                    y: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required uint32 x = 1;

    pub fn clear_x(&mut self) {
        self.x = ::std::option::Option::None;
    }

    pub fn has_x(&self) -> bool {
        self.x.is_some()
    }

    // Param is passed by value, moved
    pub fn set_x(&mut self, v: u32) {
        self.x = ::std::option::Option::Some(v);
    }

    pub fn get_x<'a>(&self) -> u32 {
        self.x.unwrap_or(0)
    }

    // required uint32 y = 2;

    pub fn clear_y(&mut self) {
        self.y = ::std::option::Option::None;
    }

    pub fn has_y(&self) -> bool {
        self.y.is_some()
    }

    // Param is passed by value, moved
    pub fn set_y(&mut self, v: u32) {
        self.y = ::std::option::Option::Some(v);
    }

    pub fn get_y<'a>(&self) -> u32 {
        self.y.unwrap_or(0)
    }
}

impl ::protobuf::Message for CSetMaximumResolutionMsg {
    fn is_initialized(&self) -> bool {
        if self.x.is_none() {
            return false;
        };
        if self.y.is_none() {
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
                    self.x = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_uint32());
                    self.y = ::std::option::Option::Some(tmp);
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
        for value in self.x.iter() {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.y.iter() {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.x {
            try!(os.write_uint32(1, v));
        };
        if let Some(v) = self.y {
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
        ::std::intrinsics::TypeId::of::<CSetMaximumResolutionMsg>()
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CSetMaximumResolutionMsg {
    fn new() -> CSetMaximumResolutionMsg {
        CSetMaximumResolutionMsg::new()
    }

    #[allow(unused_unsafe,unused_mut)]
    fn descriptor_static(_: ::std::option::Option<CSetMaximumResolutionMsg>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_u32_accessor(
                    "x",
                    CSetMaximumResolutionMsg::has_x,
                    CSetMaximumResolutionMsg::get_x,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u32_accessor(
                    "y",
                    CSetMaximumResolutionMsg::has_y,
                    CSetMaximumResolutionMsg::get_y,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CSetMaximumResolutionMsg>(
                    "CSetMaximumResolutionMsg",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CSetMaximumResolutionMsg {
    fn clear(&mut self) {
        self.clear_x();
        self.clear_y();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for CSetMaximumResolutionMsg {
    fn eq(&self, other: &CSetMaximumResolutionMsg) -> bool {
        self.x == other.x &&
        self.y == other.y &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Show for CSetMaximumResolutionMsg {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct CSetMaximumFramerateMsg {
    framerate: ::std::option::Option<u32>,
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl CSetMaximumFramerateMsg {
    pub fn new() -> CSetMaximumFramerateMsg {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CSetMaximumFramerateMsg {
        static mut instance: ::protobuf::lazy::Lazy<CSetMaximumFramerateMsg> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CSetMaximumFramerateMsg,
        };
        unsafe {
            instance.get(|| {
                CSetMaximumFramerateMsg {
                    framerate: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required uint32 framerate = 1;

    pub fn clear_framerate(&mut self) {
        self.framerate = ::std::option::Option::None;
    }

    pub fn has_framerate(&self) -> bool {
        self.framerate.is_some()
    }

    // Param is passed by value, moved
    pub fn set_framerate(&mut self, v: u32) {
        self.framerate = ::std::option::Option::Some(v);
    }

    pub fn get_framerate<'a>(&self) -> u32 {
        self.framerate.unwrap_or(0)
    }
}

impl ::protobuf::Message for CSetMaximumFramerateMsg {
    fn is_initialized(&self) -> bool {
        if self.framerate.is_none() {
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
                    self.framerate = ::std::option::Option::Some(tmp);
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
        for value in self.framerate.iter() {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.framerate {
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
        ::std::intrinsics::TypeId::of::<CSetMaximumFramerateMsg>()
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CSetMaximumFramerateMsg {
    fn new() -> CSetMaximumFramerateMsg {
        CSetMaximumFramerateMsg::new()
    }

    #[allow(unused_unsafe,unused_mut)]
    fn descriptor_static(_: ::std::option::Option<CSetMaximumFramerateMsg>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_u32_accessor(
                    "framerate",
                    CSetMaximumFramerateMsg::has_framerate,
                    CSetMaximumFramerateMsg::get_framerate,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CSetMaximumFramerateMsg>(
                    "CSetMaximumFramerateMsg",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CSetMaximumFramerateMsg {
    fn clear(&mut self) {
        self.clear_framerate();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for CSetMaximumFramerateMsg {
    fn eq(&self, other: &CSetMaximumFramerateMsg) -> bool {
        self.framerate == other.framerate &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Show for CSetMaximumFramerateMsg {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct CSetMaximumBitrateMsg {
    bitrate: ::std::option::Option<i32>,
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl CSetMaximumBitrateMsg {
    pub fn new() -> CSetMaximumBitrateMsg {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CSetMaximumBitrateMsg {
        static mut instance: ::protobuf::lazy::Lazy<CSetMaximumBitrateMsg> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CSetMaximumBitrateMsg,
        };
        unsafe {
            instance.get(|| {
                CSetMaximumBitrateMsg {
                    bitrate: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required int32 bitrate = 1;

    pub fn clear_bitrate(&mut self) {
        self.bitrate = ::std::option::Option::None;
    }

    pub fn has_bitrate(&self) -> bool {
        self.bitrate.is_some()
    }

    // Param is passed by value, moved
    pub fn set_bitrate(&mut self, v: i32) {
        self.bitrate = ::std::option::Option::Some(v);
    }

    pub fn get_bitrate<'a>(&self) -> i32 {
        self.bitrate.unwrap_or(0)
    }
}

impl ::protobuf::Message for CSetMaximumBitrateMsg {
    fn is_initialized(&self) -> bool {
        if self.bitrate.is_none() {
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
                    let tmp = try!(is.read_int32());
                    self.bitrate = ::std::option::Option::Some(tmp);
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
        for value in self.bitrate.iter() {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.bitrate {
            try!(os.write_int32(1, v));
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
        ::std::intrinsics::TypeId::of::<CSetMaximumBitrateMsg>()
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CSetMaximumBitrateMsg {
    fn new() -> CSetMaximumBitrateMsg {
        CSetMaximumBitrateMsg::new()
    }

    #[allow(unused_unsafe,unused_mut)]
    fn descriptor_static(_: ::std::option::Option<CSetMaximumBitrateMsg>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "bitrate",
                    CSetMaximumBitrateMsg::has_bitrate,
                    CSetMaximumBitrateMsg::get_bitrate,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CSetMaximumBitrateMsg>(
                    "CSetMaximumBitrateMsg",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CSetMaximumBitrateMsg {
    fn clear(&mut self) {
        self.clear_bitrate();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for CSetMaximumBitrateMsg {
    fn eq(&self, other: &CSetMaximumBitrateMsg) -> bool {
        self.bitrate == other.bitrate &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Show for CSetMaximumBitrateMsg {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct CSetQoSMsg {
    use_qos: ::std::option::Option<bool>,
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl CSetQoSMsg {
    pub fn new() -> CSetQoSMsg {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CSetQoSMsg {
        static mut instance: ::protobuf::lazy::Lazy<CSetQoSMsg> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CSetQoSMsg,
        };
        unsafe {
            instance.get(|| {
                CSetQoSMsg {
                    use_qos: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required bool use_qos = 1;

    pub fn clear_use_qos(&mut self) {
        self.use_qos = ::std::option::Option::None;
    }

    pub fn has_use_qos(&self) -> bool {
        self.use_qos.is_some()
    }

    // Param is passed by value, moved
    pub fn set_use_qos(&mut self, v: bool) {
        self.use_qos = ::std::option::Option::Some(v);
    }

    pub fn get_use_qos<'a>(&self) -> bool {
        self.use_qos.unwrap_or(false)
    }
}

impl ::protobuf::Message for CSetQoSMsg {
    fn is_initialized(&self) -> bool {
        if self.use_qos.is_none() {
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
                    let tmp = try!(is.read_bool());
                    self.use_qos = ::std::option::Option::Some(tmp);
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
        if self.use_qos.is_some() {
            my_size += 2;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.use_qos {
            try!(os.write_bool(1, v));
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
        ::std::intrinsics::TypeId::of::<CSetQoSMsg>()
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CSetQoSMsg {
    fn new() -> CSetQoSMsg {
        CSetQoSMsg::new()
    }

    #[allow(unused_unsafe,unused_mut)]
    fn descriptor_static(_: ::std::option::Option<CSetQoSMsg>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor(
                    "use_qos",
                    CSetQoSMsg::has_use_qos,
                    CSetQoSMsg::get_use_qos,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CSetQoSMsg>(
                    "CSetQoSMsg",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CSetQoSMsg {
    fn clear(&mut self) {
        self.clear_use_qos();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for CSetQoSMsg {
    fn eq(&self, other: &CSetQoSMsg) -> bool {
        self.use_qos == other.use_qos &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Show for CSetQoSMsg {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct CSetTargetFramerateMsg {
    framerate: ::std::option::Option<u32>,
    reasons: ::std::option::Option<u32>,
    framerate_numerator: ::std::option::Option<u32>,
    framerate_denominator: ::std::option::Option<u32>,
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl CSetTargetFramerateMsg {
    pub fn new() -> CSetTargetFramerateMsg {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CSetTargetFramerateMsg {
        static mut instance: ::protobuf::lazy::Lazy<CSetTargetFramerateMsg> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CSetTargetFramerateMsg,
        };
        unsafe {
            instance.get(|| {
                CSetTargetFramerateMsg {
                    framerate: ::std::option::Option::None,
                    reasons: ::std::option::Option::None,
                    framerate_numerator: ::std::option::Option::None,
                    framerate_denominator: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required uint32 framerate = 1;

    pub fn clear_framerate(&mut self) {
        self.framerate = ::std::option::Option::None;
    }

    pub fn has_framerate(&self) -> bool {
        self.framerate.is_some()
    }

    // Param is passed by value, moved
    pub fn set_framerate(&mut self, v: u32) {
        self.framerate = ::std::option::Option::Some(v);
    }

    pub fn get_framerate<'a>(&self) -> u32 {
        self.framerate.unwrap_or(0)
    }

    // optional uint32 reasons = 2;

    pub fn clear_reasons(&mut self) {
        self.reasons = ::std::option::Option::None;
    }

    pub fn has_reasons(&self) -> bool {
        self.reasons.is_some()
    }

    // Param is passed by value, moved
    pub fn set_reasons(&mut self, v: u32) {
        self.reasons = ::std::option::Option::Some(v);
    }

    pub fn get_reasons<'a>(&self) -> u32 {
        self.reasons.unwrap_or(0)
    }

    // optional uint32 framerate_numerator = 3;

    pub fn clear_framerate_numerator(&mut self) {
        self.framerate_numerator = ::std::option::Option::None;
    }

    pub fn has_framerate_numerator(&self) -> bool {
        self.framerate_numerator.is_some()
    }

    // Param is passed by value, moved
    pub fn set_framerate_numerator(&mut self, v: u32) {
        self.framerate_numerator = ::std::option::Option::Some(v);
    }

    pub fn get_framerate_numerator<'a>(&self) -> u32 {
        self.framerate_numerator.unwrap_or(0)
    }

    // optional uint32 framerate_denominator = 4;

    pub fn clear_framerate_denominator(&mut self) {
        self.framerate_denominator = ::std::option::Option::None;
    }

    pub fn has_framerate_denominator(&self) -> bool {
        self.framerate_denominator.is_some()
    }

    // Param is passed by value, moved
    pub fn set_framerate_denominator(&mut self, v: u32) {
        self.framerate_denominator = ::std::option::Option::Some(v);
    }

    pub fn get_framerate_denominator<'a>(&self) -> u32 {
        self.framerate_denominator.unwrap_or(0)
    }
}

impl ::protobuf::Message for CSetTargetFramerateMsg {
    fn is_initialized(&self) -> bool {
        if self.framerate.is_none() {
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
                    self.framerate = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_uint32());
                    self.reasons = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_uint32());
                    self.framerate_numerator = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_uint32());
                    self.framerate_denominator = ::std::option::Option::Some(tmp);
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
        for value in self.framerate.iter() {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.reasons.iter() {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.framerate_numerator.iter() {
            my_size += ::protobuf::rt::value_size(3, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.framerate_denominator.iter() {
            my_size += ::protobuf::rt::value_size(4, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.framerate {
            try!(os.write_uint32(1, v));
        };
        if let Some(v) = self.reasons {
            try!(os.write_uint32(2, v));
        };
        if let Some(v) = self.framerate_numerator {
            try!(os.write_uint32(3, v));
        };
        if let Some(v) = self.framerate_denominator {
            try!(os.write_uint32(4, v));
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
        ::std::intrinsics::TypeId::of::<CSetTargetFramerateMsg>()
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CSetTargetFramerateMsg {
    fn new() -> CSetTargetFramerateMsg {
        CSetTargetFramerateMsg::new()
    }

    #[allow(unused_unsafe,unused_mut)]
    fn descriptor_static(_: ::std::option::Option<CSetTargetFramerateMsg>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_u32_accessor(
                    "framerate",
                    CSetTargetFramerateMsg::has_framerate,
                    CSetTargetFramerateMsg::get_framerate,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u32_accessor(
                    "reasons",
                    CSetTargetFramerateMsg::has_reasons,
                    CSetTargetFramerateMsg::get_reasons,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u32_accessor(
                    "framerate_numerator",
                    CSetTargetFramerateMsg::has_framerate_numerator,
                    CSetTargetFramerateMsg::get_framerate_numerator,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u32_accessor(
                    "framerate_denominator",
                    CSetTargetFramerateMsg::has_framerate_denominator,
                    CSetTargetFramerateMsg::get_framerate_denominator,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CSetTargetFramerateMsg>(
                    "CSetTargetFramerateMsg",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CSetTargetFramerateMsg {
    fn clear(&mut self) {
        self.clear_framerate();
        self.clear_reasons();
        self.clear_framerate_numerator();
        self.clear_framerate_denominator();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for CSetTargetFramerateMsg {
    fn eq(&self, other: &CSetTargetFramerateMsg) -> bool {
        self.framerate == other.framerate &&
        self.reasons == other.reasons &&
        self.framerate_numerator == other.framerate_numerator &&
        self.framerate_denominator == other.framerate_denominator &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Show for CSetTargetFramerateMsg {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct COverlayEnabledMsg {
    enabled: ::std::option::Option<bool>,
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl COverlayEnabledMsg {
    pub fn new() -> COverlayEnabledMsg {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static COverlayEnabledMsg {
        static mut instance: ::protobuf::lazy::Lazy<COverlayEnabledMsg> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const COverlayEnabledMsg,
        };
        unsafe {
            instance.get(|| {
                COverlayEnabledMsg {
                    enabled: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required bool enabled = 1;

    pub fn clear_enabled(&mut self) {
        self.enabled = ::std::option::Option::None;
    }

    pub fn has_enabled(&self) -> bool {
        self.enabled.is_some()
    }

    // Param is passed by value, moved
    pub fn set_enabled(&mut self, v: bool) {
        self.enabled = ::std::option::Option::Some(v);
    }

    pub fn get_enabled<'a>(&self) -> bool {
        self.enabled.unwrap_or(false)
    }
}

impl ::protobuf::Message for COverlayEnabledMsg {
    fn is_initialized(&self) -> bool {
        if self.enabled.is_none() {
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
                    let tmp = try!(is.read_bool());
                    self.enabled = ::std::option::Option::Some(tmp);
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
        if self.enabled.is_some() {
            my_size += 2;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.enabled {
            try!(os.write_bool(1, v));
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
        ::std::intrinsics::TypeId::of::<COverlayEnabledMsg>()
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for COverlayEnabledMsg {
    fn new() -> COverlayEnabledMsg {
        COverlayEnabledMsg::new()
    }

    #[allow(unused_unsafe,unused_mut)]
    fn descriptor_static(_: ::std::option::Option<COverlayEnabledMsg>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor(
                    "enabled",
                    COverlayEnabledMsg::has_enabled,
                    COverlayEnabledMsg::get_enabled,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<COverlayEnabledMsg>(
                    "COverlayEnabledMsg",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for COverlayEnabledMsg {
    fn clear(&mut self) {
        self.clear_enabled();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for COverlayEnabledMsg {
    fn eq(&self, other: &COverlayEnabledMsg) -> bool {
        self.enabled == other.enabled &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Show for COverlayEnabledMsg {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct CInputControllerAttachedMsg {
    controller_index: ::std::option::Option<i32>,
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl CInputControllerAttachedMsg {
    pub fn new() -> CInputControllerAttachedMsg {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CInputControllerAttachedMsg {
        static mut instance: ::protobuf::lazy::Lazy<CInputControllerAttachedMsg> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CInputControllerAttachedMsg,
        };
        unsafe {
            instance.get(|| {
                CInputControllerAttachedMsg {
                    controller_index: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required int32 controller_index = 1;

    pub fn clear_controller_index(&mut self) {
        self.controller_index = ::std::option::Option::None;
    }

    pub fn has_controller_index(&self) -> bool {
        self.controller_index.is_some()
    }

    // Param is passed by value, moved
    pub fn set_controller_index(&mut self, v: i32) {
        self.controller_index = ::std::option::Option::Some(v);
    }

    pub fn get_controller_index<'a>(&self) -> i32 {
        self.controller_index.unwrap_or(0)
    }
}

impl ::protobuf::Message for CInputControllerAttachedMsg {
    fn is_initialized(&self) -> bool {
        if self.controller_index.is_none() {
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
                    let tmp = try!(is.read_int32());
                    self.controller_index = ::std::option::Option::Some(tmp);
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
        for value in self.controller_index.iter() {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.controller_index {
            try!(os.write_int32(1, v));
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
        ::std::intrinsics::TypeId::of::<CInputControllerAttachedMsg>()
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CInputControllerAttachedMsg {
    fn new() -> CInputControllerAttachedMsg {
        CInputControllerAttachedMsg::new()
    }

    #[allow(unused_unsafe,unused_mut)]
    fn descriptor_static(_: ::std::option::Option<CInputControllerAttachedMsg>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "controller_index",
                    CInputControllerAttachedMsg::has_controller_index,
                    CInputControllerAttachedMsg::get_controller_index,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CInputControllerAttachedMsg>(
                    "CInputControllerAttachedMsg",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CInputControllerAttachedMsg {
    fn clear(&mut self) {
        self.clear_controller_index();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for CInputControllerAttachedMsg {
    fn eq(&self, other: &CInputControllerAttachedMsg) -> bool {
        self.controller_index == other.controller_index &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Show for CInputControllerAttachedMsg {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct CInputControllerStateMsg {
    input_mark: ::std::option::Option<u32>,
    controller_index: ::std::option::Option<i32>,
    buttons: ::std::option::Option<u64>,
    left_pad_x: ::std::option::Option<i32>,
    left_pad_y: ::std::option::Option<i32>,
    right_pad_x: ::std::option::Option<i32>,
    right_pad_y: ::std::option::Option<i32>,
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl CInputControllerStateMsg {
    pub fn new() -> CInputControllerStateMsg {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CInputControllerStateMsg {
        static mut instance: ::protobuf::lazy::Lazy<CInputControllerStateMsg> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CInputControllerStateMsg,
        };
        unsafe {
            instance.get(|| {
                CInputControllerStateMsg {
                    input_mark: ::std::option::Option::None,
                    controller_index: ::std::option::Option::None,
                    buttons: ::std::option::Option::None,
                    left_pad_x: ::std::option::Option::None,
                    left_pad_y: ::std::option::Option::None,
                    right_pad_x: ::std::option::Option::None,
                    right_pad_y: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional uint32 input_mark = 1;

    pub fn clear_input_mark(&mut self) {
        self.input_mark = ::std::option::Option::None;
    }

    pub fn has_input_mark(&self) -> bool {
        self.input_mark.is_some()
    }

    // Param is passed by value, moved
    pub fn set_input_mark(&mut self, v: u32) {
        self.input_mark = ::std::option::Option::Some(v);
    }

    pub fn get_input_mark<'a>(&self) -> u32 {
        self.input_mark.unwrap_or(0)
    }

    // required int32 controller_index = 2;

    pub fn clear_controller_index(&mut self) {
        self.controller_index = ::std::option::Option::None;
    }

    pub fn has_controller_index(&self) -> bool {
        self.controller_index.is_some()
    }

    // Param is passed by value, moved
    pub fn set_controller_index(&mut self, v: i32) {
        self.controller_index = ::std::option::Option::Some(v);
    }

    pub fn get_controller_index<'a>(&self) -> i32 {
        self.controller_index.unwrap_or(0)
    }

    // optional uint64 buttons = 3;

    pub fn clear_buttons(&mut self) {
        self.buttons = ::std::option::Option::None;
    }

    pub fn has_buttons(&self) -> bool {
        self.buttons.is_some()
    }

    // Param is passed by value, moved
    pub fn set_buttons(&mut self, v: u64) {
        self.buttons = ::std::option::Option::Some(v);
    }

    pub fn get_buttons<'a>(&self) -> u64 {
        self.buttons.unwrap_or(0)
    }

    // optional sint32 left_pad_x = 4;

    pub fn clear_left_pad_x(&mut self) {
        self.left_pad_x = ::std::option::Option::None;
    }

    pub fn has_left_pad_x(&self) -> bool {
        self.left_pad_x.is_some()
    }

    // Param is passed by value, moved
    pub fn set_left_pad_x(&mut self, v: i32) {
        self.left_pad_x = ::std::option::Option::Some(v);
    }

    pub fn get_left_pad_x<'a>(&self) -> i32 {
        self.left_pad_x.unwrap_or(0)
    }

    // optional sint32 left_pad_y = 5;

    pub fn clear_left_pad_y(&mut self) {
        self.left_pad_y = ::std::option::Option::None;
    }

    pub fn has_left_pad_y(&self) -> bool {
        self.left_pad_y.is_some()
    }

    // Param is passed by value, moved
    pub fn set_left_pad_y(&mut self, v: i32) {
        self.left_pad_y = ::std::option::Option::Some(v);
    }

    pub fn get_left_pad_y<'a>(&self) -> i32 {
        self.left_pad_y.unwrap_or(0)
    }

    // optional sint32 right_pad_x = 6;

    pub fn clear_right_pad_x(&mut self) {
        self.right_pad_x = ::std::option::Option::None;
    }

    pub fn has_right_pad_x(&self) -> bool {
        self.right_pad_x.is_some()
    }

    // Param is passed by value, moved
    pub fn set_right_pad_x(&mut self, v: i32) {
        self.right_pad_x = ::std::option::Option::Some(v);
    }

    pub fn get_right_pad_x<'a>(&self) -> i32 {
        self.right_pad_x.unwrap_or(0)
    }

    // optional sint32 right_pad_y = 7;

    pub fn clear_right_pad_y(&mut self) {
        self.right_pad_y = ::std::option::Option::None;
    }

    pub fn has_right_pad_y(&self) -> bool {
        self.right_pad_y.is_some()
    }

    // Param is passed by value, moved
    pub fn set_right_pad_y(&mut self, v: i32) {
        self.right_pad_y = ::std::option::Option::Some(v);
    }

    pub fn get_right_pad_y<'a>(&self) -> i32 {
        self.right_pad_y.unwrap_or(0)
    }
}

impl ::protobuf::Message for CInputControllerStateMsg {
    fn is_initialized(&self) -> bool {
        if self.controller_index.is_none() {
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
                    self.input_mark = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_int32());
                    self.controller_index = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_uint64());
                    self.buttons = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_sint32());
                    self.left_pad_x = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_sint32());
                    self.left_pad_y = ::std::option::Option::Some(tmp);
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_sint32());
                    self.right_pad_x = ::std::option::Option::Some(tmp);
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_sint32());
                    self.right_pad_y = ::std::option::Option::Some(tmp);
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
        for value in self.input_mark.iter() {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.controller_index.iter() {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.buttons.iter() {
            my_size += ::protobuf::rt::value_size(3, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.left_pad_x.iter() {
            my_size += ::protobuf::rt::value_size(4, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.left_pad_y.iter() {
            my_size += ::protobuf::rt::value_size(5, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.right_pad_x.iter() {
            my_size += ::protobuf::rt::value_size(6, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.right_pad_y.iter() {
            my_size += ::protobuf::rt::value_size(7, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.input_mark {
            try!(os.write_uint32(1, v));
        };
        if let Some(v) = self.controller_index {
            try!(os.write_int32(2, v));
        };
        if let Some(v) = self.buttons {
            try!(os.write_uint64(3, v));
        };
        if let Some(v) = self.left_pad_x {
            try!(os.write_sint32(4, v));
        };
        if let Some(v) = self.left_pad_y {
            try!(os.write_sint32(5, v));
        };
        if let Some(v) = self.right_pad_x {
            try!(os.write_sint32(6, v));
        };
        if let Some(v) = self.right_pad_y {
            try!(os.write_sint32(7, v));
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
        ::std::intrinsics::TypeId::of::<CInputControllerStateMsg>()
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CInputControllerStateMsg {
    fn new() -> CInputControllerStateMsg {
        CInputControllerStateMsg::new()
    }

    #[allow(unused_unsafe,unused_mut)]
    fn descriptor_static(_: ::std::option::Option<CInputControllerStateMsg>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_u32_accessor(
                    "input_mark",
                    CInputControllerStateMsg::has_input_mark,
                    CInputControllerStateMsg::get_input_mark,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "controller_index",
                    CInputControllerStateMsg::has_controller_index,
                    CInputControllerStateMsg::get_controller_index,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "buttons",
                    CInputControllerStateMsg::has_buttons,
                    CInputControllerStateMsg::get_buttons,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "left_pad_x",
                    CInputControllerStateMsg::has_left_pad_x,
                    CInputControllerStateMsg::get_left_pad_x,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "left_pad_y",
                    CInputControllerStateMsg::has_left_pad_y,
                    CInputControllerStateMsg::get_left_pad_y,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "right_pad_x",
                    CInputControllerStateMsg::has_right_pad_x,
                    CInputControllerStateMsg::get_right_pad_x,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "right_pad_y",
                    CInputControllerStateMsg::has_right_pad_y,
                    CInputControllerStateMsg::get_right_pad_y,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CInputControllerStateMsg>(
                    "CInputControllerStateMsg",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CInputControllerStateMsg {
    fn clear(&mut self) {
        self.clear_input_mark();
        self.clear_controller_index();
        self.clear_buttons();
        self.clear_left_pad_x();
        self.clear_left_pad_y();
        self.clear_right_pad_x();
        self.clear_right_pad_y();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for CInputControllerStateMsg {
    fn eq(&self, other: &CInputControllerStateMsg) -> bool {
        self.input_mark == other.input_mark &&
        self.controller_index == other.controller_index &&
        self.buttons == other.buttons &&
        self.left_pad_x == other.left_pad_x &&
        self.left_pad_y == other.left_pad_y &&
        self.right_pad_x == other.right_pad_x &&
        self.right_pad_y == other.right_pad_y &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Show for CInputControllerStateMsg {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct CInputControllerWirelessPresenceMsg {
    wireless_present: ::std::option::Option<bool>,
    wireless_available: ::std::option::Option<bool>,
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl CInputControllerWirelessPresenceMsg {
    pub fn new() -> CInputControllerWirelessPresenceMsg {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CInputControllerWirelessPresenceMsg {
        static mut instance: ::protobuf::lazy::Lazy<CInputControllerWirelessPresenceMsg> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CInputControllerWirelessPresenceMsg,
        };
        unsafe {
            instance.get(|| {
                CInputControllerWirelessPresenceMsg {
                    wireless_present: ::std::option::Option::None,
                    wireless_available: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional bool wireless_present = 1;

    pub fn clear_wireless_present(&mut self) {
        self.wireless_present = ::std::option::Option::None;
    }

    pub fn has_wireless_present(&self) -> bool {
        self.wireless_present.is_some()
    }

    // Param is passed by value, moved
    pub fn set_wireless_present(&mut self, v: bool) {
        self.wireless_present = ::std::option::Option::Some(v);
    }

    pub fn get_wireless_present<'a>(&self) -> bool {
        self.wireless_present.unwrap_or(false)
    }

    // optional bool wireless_available = 2;

    pub fn clear_wireless_available(&mut self) {
        self.wireless_available = ::std::option::Option::None;
    }

    pub fn has_wireless_available(&self) -> bool {
        self.wireless_available.is_some()
    }

    // Param is passed by value, moved
    pub fn set_wireless_available(&mut self, v: bool) {
        self.wireless_available = ::std::option::Option::Some(v);
    }

    pub fn get_wireless_available<'a>(&self) -> bool {
        self.wireless_available.unwrap_or(false)
    }
}

impl ::protobuf::Message for CInputControllerWirelessPresenceMsg {
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
                    let tmp = try!(is.read_bool());
                    self.wireless_present = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_bool());
                    self.wireless_available = ::std::option::Option::Some(tmp);
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
        if self.wireless_present.is_some() {
            my_size += 2;
        };
        if self.wireless_available.is_some() {
            my_size += 2;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.wireless_present {
            try!(os.write_bool(1, v));
        };
        if let Some(v) = self.wireless_available {
            try!(os.write_bool(2, v));
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
        ::std::intrinsics::TypeId::of::<CInputControllerWirelessPresenceMsg>()
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CInputControllerWirelessPresenceMsg {
    fn new() -> CInputControllerWirelessPresenceMsg {
        CInputControllerWirelessPresenceMsg::new()
    }

    #[allow(unused_unsafe,unused_mut)]
    fn descriptor_static(_: ::std::option::Option<CInputControllerWirelessPresenceMsg>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor(
                    "wireless_present",
                    CInputControllerWirelessPresenceMsg::has_wireless_present,
                    CInputControllerWirelessPresenceMsg::get_wireless_present,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor(
                    "wireless_available",
                    CInputControllerWirelessPresenceMsg::has_wireless_available,
                    CInputControllerWirelessPresenceMsg::get_wireless_available,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CInputControllerWirelessPresenceMsg>(
                    "CInputControllerWirelessPresenceMsg",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CInputControllerWirelessPresenceMsg {
    fn clear(&mut self) {
        self.clear_wireless_present();
        self.clear_wireless_available();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for CInputControllerWirelessPresenceMsg {
    fn eq(&self, other: &CInputControllerWirelessPresenceMsg) -> bool {
        self.wireless_present == other.wireless_present &&
        self.wireless_available == other.wireless_available &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Show for CInputControllerWirelessPresenceMsg {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct CTriggerHapticPulseMsg {
    controller_index: ::std::option::Option<i32>,
    pad_index: ::std::option::Option<i32>,
    duration_microsec: ::std::option::Option<u32>,
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl CTriggerHapticPulseMsg {
    pub fn new() -> CTriggerHapticPulseMsg {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CTriggerHapticPulseMsg {
        static mut instance: ::protobuf::lazy::Lazy<CTriggerHapticPulseMsg> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CTriggerHapticPulseMsg,
        };
        unsafe {
            instance.get(|| {
                CTriggerHapticPulseMsg {
                    controller_index: ::std::option::Option::None,
                    pad_index: ::std::option::Option::None,
                    duration_microsec: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required int32 controller_index = 1;

    pub fn clear_controller_index(&mut self) {
        self.controller_index = ::std::option::Option::None;
    }

    pub fn has_controller_index(&self) -> bool {
        self.controller_index.is_some()
    }

    // Param is passed by value, moved
    pub fn set_controller_index(&mut self, v: i32) {
        self.controller_index = ::std::option::Option::Some(v);
    }

    pub fn get_controller_index<'a>(&self) -> i32 {
        self.controller_index.unwrap_or(0)
    }

    // required int32 pad_index = 2;

    pub fn clear_pad_index(&mut self) {
        self.pad_index = ::std::option::Option::None;
    }

    pub fn has_pad_index(&self) -> bool {
        self.pad_index.is_some()
    }

    // Param is passed by value, moved
    pub fn set_pad_index(&mut self, v: i32) {
        self.pad_index = ::std::option::Option::Some(v);
    }

    pub fn get_pad_index<'a>(&self) -> i32 {
        self.pad_index.unwrap_or(0)
    }

    // required uint32 duration_microsec = 3;

    pub fn clear_duration_microsec(&mut self) {
        self.duration_microsec = ::std::option::Option::None;
    }

    pub fn has_duration_microsec(&self) -> bool {
        self.duration_microsec.is_some()
    }

    // Param is passed by value, moved
    pub fn set_duration_microsec(&mut self, v: u32) {
        self.duration_microsec = ::std::option::Option::Some(v);
    }

    pub fn get_duration_microsec<'a>(&self) -> u32 {
        self.duration_microsec.unwrap_or(0)
    }
}

impl ::protobuf::Message for CTriggerHapticPulseMsg {
    fn is_initialized(&self) -> bool {
        if self.controller_index.is_none() {
            return false;
        };
        if self.pad_index.is_none() {
            return false;
        };
        if self.duration_microsec.is_none() {
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
                    let tmp = try!(is.read_int32());
                    self.controller_index = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_int32());
                    self.pad_index = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_uint32());
                    self.duration_microsec = ::std::option::Option::Some(tmp);
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
        for value in self.controller_index.iter() {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.pad_index.iter() {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.duration_microsec.iter() {
            my_size += ::protobuf::rt::value_size(3, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.controller_index {
            try!(os.write_int32(1, v));
        };
        if let Some(v) = self.pad_index {
            try!(os.write_int32(2, v));
        };
        if let Some(v) = self.duration_microsec {
            try!(os.write_uint32(3, v));
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
        ::std::intrinsics::TypeId::of::<CTriggerHapticPulseMsg>()
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CTriggerHapticPulseMsg {
    fn new() -> CTriggerHapticPulseMsg {
        CTriggerHapticPulseMsg::new()
    }

    #[allow(unused_unsafe,unused_mut)]
    fn descriptor_static(_: ::std::option::Option<CTriggerHapticPulseMsg>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "controller_index",
                    CTriggerHapticPulseMsg::has_controller_index,
                    CTriggerHapticPulseMsg::get_controller_index,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "pad_index",
                    CTriggerHapticPulseMsg::has_pad_index,
                    CTriggerHapticPulseMsg::get_pad_index,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u32_accessor(
                    "duration_microsec",
                    CTriggerHapticPulseMsg::has_duration_microsec,
                    CTriggerHapticPulseMsg::get_duration_microsec,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CTriggerHapticPulseMsg>(
                    "CTriggerHapticPulseMsg",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CTriggerHapticPulseMsg {
    fn clear(&mut self) {
        self.clear_controller_index();
        self.clear_pad_index();
        self.clear_duration_microsec();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for CTriggerHapticPulseMsg {
    fn eq(&self, other: &CTriggerHapticPulseMsg) -> bool {
        self.controller_index == other.controller_index &&
        self.pad_index == other.pad_index &&
        self.duration_microsec == other.duration_microsec &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Show for CTriggerHapticPulseMsg {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct CSetOverrideModeMsg {
    window_type: ::std::option::Option<i32>,
    override_mode: ::protobuf::SingularField<::std::string::String>,
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl CSetOverrideModeMsg {
    pub fn new() -> CSetOverrideModeMsg {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CSetOverrideModeMsg {
        static mut instance: ::protobuf::lazy::Lazy<CSetOverrideModeMsg> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CSetOverrideModeMsg,
        };
        unsafe {
            instance.get(|| {
                CSetOverrideModeMsg {
                    window_type: ::std::option::Option::None,
                    override_mode: ::protobuf::SingularField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional int32 window_type = 1;

    pub fn clear_window_type(&mut self) {
        self.window_type = ::std::option::Option::None;
    }

    pub fn has_window_type(&self) -> bool {
        self.window_type.is_some()
    }

    // Param is passed by value, moved
    pub fn set_window_type(&mut self, v: i32) {
        self.window_type = ::std::option::Option::Some(v);
    }

    pub fn get_window_type<'a>(&self) -> i32 {
        self.window_type.unwrap_or(0)
    }

    // optional string override_mode = 2;

    pub fn clear_override_mode(&mut self) {
        self.override_mode.clear();
    }

    pub fn has_override_mode(&self) -> bool {
        self.override_mode.is_some()
    }

    // Param is passed by value, moved
    pub fn set_override_mode(&mut self, v: ::std::string::String) {
        self.override_mode = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_override_mode<'a>(&'a mut self) -> &'a mut ::std::string::String {
        if self.override_mode.is_none() {
            self.override_mode.set_default();
        };
        self.override_mode.as_mut().unwrap()
    }

    // Take field
    pub fn take_override_mode(&mut self) -> ::std::string::String {
        self.override_mode.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_override_mode<'a>(&'a self) -> &'a str {
        match self.override_mode.as_ref() {
            Some(v) => v.as_slice(),
            None => "",
        }
    }
}

impl ::protobuf::Message for CSetOverrideModeMsg {
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
                    self.window_type = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.override_mode.set_default();
                    try!(is.read_string_into(tmp))
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
        for value in self.window_type.iter() {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.override_mode.iter() {
            my_size += ::protobuf::rt::string_size(2, value.as_slice());
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.window_type {
            try!(os.write_int32(1, v));
        };
        if let Some(v) = self.override_mode.as_ref() {
            try!(os.write_string(2, v.as_slice()));
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
        ::std::intrinsics::TypeId::of::<CSetOverrideModeMsg>()
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CSetOverrideModeMsg {
    fn new() -> CSetOverrideModeMsg {
        CSetOverrideModeMsg::new()
    }

    #[allow(unused_unsafe,unused_mut)]
    fn descriptor_static(_: ::std::option::Option<CSetOverrideModeMsg>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "window_type",
                    CSetOverrideModeMsg::has_window_type,
                    CSetOverrideModeMsg::get_window_type,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "override_mode",
                    CSetOverrideModeMsg::has_override_mode,
                    CSetOverrideModeMsg::get_override_mode,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CSetOverrideModeMsg>(
                    "CSetOverrideModeMsg",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CSetOverrideModeMsg {
    fn clear(&mut self) {
        self.clear_window_type();
        self.clear_override_mode();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for CSetOverrideModeMsg {
    fn eq(&self, other: &CSetOverrideModeMsg) -> bool {
        self.window_type == other.window_type &&
        self.override_mode == other.override_mode &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Show for CSetOverrideModeMsg {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct CSetGammaRampMsg {
    gamma_ramp: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl CSetGammaRampMsg {
    pub fn new() -> CSetGammaRampMsg {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CSetGammaRampMsg {
        static mut instance: ::protobuf::lazy::Lazy<CSetGammaRampMsg> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CSetGammaRampMsg,
        };
        unsafe {
            instance.get(|| {
                CSetGammaRampMsg {
                    gamma_ramp: ::protobuf::SingularField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional bytes gamma_ramp = 1;

    pub fn clear_gamma_ramp(&mut self) {
        self.gamma_ramp.clear();
    }

    pub fn has_gamma_ramp(&self) -> bool {
        self.gamma_ramp.is_some()
    }

    // Param is passed by value, moved
    pub fn set_gamma_ramp(&mut self, v: ::std::vec::Vec<u8>) {
        self.gamma_ramp = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_gamma_ramp<'a>(&'a mut self) -> &'a mut ::std::vec::Vec<u8> {
        if self.gamma_ramp.is_none() {
            self.gamma_ramp.set_default();
        };
        self.gamma_ramp.as_mut().unwrap()
    }

    // Take field
    pub fn take_gamma_ramp(&mut self) -> ::std::vec::Vec<u8> {
        self.gamma_ramp.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_gamma_ramp<'a>(&'a self) -> &'a [u8] {
        match self.gamma_ramp.as_ref() {
            Some(v) => v.as_slice(),
            None => [].as_slice(),
        }
    }
}

impl ::protobuf::Message for CSetGammaRampMsg {
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
                    let tmp = self.gamma_ramp.set_default();
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
        for value in self.gamma_ramp.iter() {
            my_size += ::protobuf::rt::bytes_size(1, value.as_slice());
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.gamma_ramp.as_ref() {
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
        ::std::intrinsics::TypeId::of::<CSetGammaRampMsg>()
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CSetGammaRampMsg {
    fn new() -> CSetGammaRampMsg {
        CSetGammaRampMsg::new()
    }

    #[allow(unused_unsafe,unused_mut)]
    fn descriptor_static(_: ::std::option::Option<CSetGammaRampMsg>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "gamma_ramp",
                    CSetGammaRampMsg::has_gamma_ramp,
                    CSetGammaRampMsg::get_gamma_ramp,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CSetGammaRampMsg>(
                    "CSetGammaRampMsg",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CSetGammaRampMsg {
    fn clear(&mut self) {
        self.clear_gamma_ramp();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for CSetGammaRampMsg {
    fn eq(&self, other: &CSetGammaRampMsg) -> bool {
        self.gamma_ramp == other.gamma_ramp &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Show for CSetGammaRampMsg {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct CInputControllerDetachedMsg {
    controller_index: ::std::option::Option<i32>,
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl CInputControllerDetachedMsg {
    pub fn new() -> CInputControllerDetachedMsg {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CInputControllerDetachedMsg {
        static mut instance: ::protobuf::lazy::Lazy<CInputControllerDetachedMsg> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CInputControllerDetachedMsg,
        };
        unsafe {
            instance.get(|| {
                CInputControllerDetachedMsg {
                    controller_index: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required int32 controller_index = 1;

    pub fn clear_controller_index(&mut self) {
        self.controller_index = ::std::option::Option::None;
    }

    pub fn has_controller_index(&self) -> bool {
        self.controller_index.is_some()
    }

    // Param is passed by value, moved
    pub fn set_controller_index(&mut self, v: i32) {
        self.controller_index = ::std::option::Option::Some(v);
    }

    pub fn get_controller_index<'a>(&self) -> i32 {
        self.controller_index.unwrap_or(0)
    }
}

impl ::protobuf::Message for CInputControllerDetachedMsg {
    fn is_initialized(&self) -> bool {
        if self.controller_index.is_none() {
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
                    let tmp = try!(is.read_int32());
                    self.controller_index = ::std::option::Option::Some(tmp);
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
        for value in self.controller_index.iter() {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.controller_index {
            try!(os.write_int32(1, v));
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
        ::std::intrinsics::TypeId::of::<CInputControllerDetachedMsg>()
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CInputControllerDetachedMsg {
    fn new() -> CInputControllerDetachedMsg {
        CInputControllerDetachedMsg::new()
    }

    #[allow(unused_unsafe,unused_mut)]
    fn descriptor_static(_: ::std::option::Option<CInputControllerDetachedMsg>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "controller_index",
                    CInputControllerDetachedMsg::has_controller_index,
                    CInputControllerDetachedMsg::get_controller_index,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CInputControllerDetachedMsg>(
                    "CInputControllerDetachedMsg",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CInputControllerDetachedMsg {
    fn clear(&mut self) {
        self.clear_controller_index();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for CInputControllerDetachedMsg {
    fn eq(&self, other: &CInputControllerDetachedMsg) -> bool {
        self.controller_index == other.controller_index &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Show for CInputControllerDetachedMsg {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct CStreamDataLostMsg {
    packets: ::std::vec::Vec<u32>,
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl CStreamDataLostMsg {
    pub fn new() -> CStreamDataLostMsg {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CStreamDataLostMsg {
        static mut instance: ::protobuf::lazy::Lazy<CStreamDataLostMsg> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CStreamDataLostMsg,
        };
        unsafe {
            instance.get(|| {
                CStreamDataLostMsg {
                    packets: ::std::vec::Vec::new(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // repeated uint32 packets = 1;

    pub fn clear_packets(&mut self) {
        self.packets.clear();
    }

    // Param is passed by value, moved
    pub fn set_packets(&mut self, v: ::std::vec::Vec<u32>) {
        self.packets = v;
    }

    // Mutable pointer to the field.
    pub fn mut_packets<'a>(&'a mut self) -> &'a mut ::std::vec::Vec<u32> {
        &mut self.packets
    }

    // Take field
    pub fn take_packets(&mut self) -> ::std::vec::Vec<u32> {
        ::std::mem::replace(&mut self.packets, ::std::vec::Vec::new())
    }

    pub fn get_packets<'a>(&'a self) -> &'a [u32] {
        self.packets.as_slice()
    }
}

impl ::protobuf::Message for CStreamDataLostMsg {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_repeated_uint32_into(wire_type, is, &mut self.packets));
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
        for value in self.packets.iter() {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in self.packets.iter() {
            try!(os.write_uint32(1, *v));
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
        ::std::intrinsics::TypeId::of::<CStreamDataLostMsg>()
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CStreamDataLostMsg {
    fn new() -> CStreamDataLostMsg {
        CStreamDataLostMsg::new()
    }

    #[allow(unused_unsafe,unused_mut)]
    fn descriptor_static(_: ::std::option::Option<CStreamDataLostMsg>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_u32_accessor(
                    "packets",
                    CStreamDataLostMsg::get_packets,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CStreamDataLostMsg>(
                    "CStreamDataLostMsg",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CStreamDataLostMsg {
    fn clear(&mut self) {
        self.clear_packets();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for CStreamDataLostMsg {
    fn eq(&self, other: &CStreamDataLostMsg) -> bool {
        self.packets == other.packets &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Show for CStreamDataLostMsg {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct CAudioFormat {
    format: ::std::option::Option<EAudioFormat>,
    frequency: ::std::option::Option<u32>,
    channels: ::std::option::Option<u32>,
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl CAudioFormat {
    pub fn new() -> CAudioFormat {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CAudioFormat {
        static mut instance: ::protobuf::lazy::Lazy<CAudioFormat> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CAudioFormat,
        };
        unsafe {
            instance.get(|| {
                CAudioFormat {
                    format: ::std::option::Option::None,
                    frequency: ::std::option::Option::None,
                    channels: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required .EAudioFormat format = 1;

    pub fn clear_format(&mut self) {
        self.format = ::std::option::Option::None;
    }

    pub fn has_format(&self) -> bool {
        self.format.is_some()
    }

    // Param is passed by value, moved
    pub fn set_format(&mut self, v: EAudioFormat) {
        self.format = ::std::option::Option::Some(v);
    }

    pub fn get_format<'a>(&self) -> EAudioFormat {
        self.format.unwrap_or(EAudioFormat::k_EAudioFormatNone)
    }

    // optional uint32 frequency = 2;

    pub fn clear_frequency(&mut self) {
        self.frequency = ::std::option::Option::None;
    }

    pub fn has_frequency(&self) -> bool {
        self.frequency.is_some()
    }

    // Param is passed by value, moved
    pub fn set_frequency(&mut self, v: u32) {
        self.frequency = ::std::option::Option::Some(v);
    }

    pub fn get_frequency<'a>(&self) -> u32 {
        self.frequency.unwrap_or(0)
    }

    // optional uint32 channels = 3;

    pub fn clear_channels(&mut self) {
        self.channels = ::std::option::Option::None;
    }

    pub fn has_channels(&self) -> bool {
        self.channels.is_some()
    }

    // Param is passed by value, moved
    pub fn set_channels(&mut self, v: u32) {
        self.channels = ::std::option::Option::Some(v);
    }

    pub fn get_channels<'a>(&self) -> u32 {
        self.channels.unwrap_or(0)
    }
}

impl ::protobuf::Message for CAudioFormat {
    fn is_initialized(&self) -> bool {
        if self.format.is_none() {
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
                    self.format = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_uint32());
                    self.frequency = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_uint32());
                    self.channels = ::std::option::Option::Some(tmp);
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
        for value in self.format.iter() {
            my_size += ::protobuf::rt::enum_size(1, *value);
        };
        for value in self.frequency.iter() {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.channels.iter() {
            my_size += ::protobuf::rt::value_size(3, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.format {
            try!(os.write_enum(1, v as i32));
        };
        if let Some(v) = self.frequency {
            try!(os.write_uint32(2, v));
        };
        if let Some(v) = self.channels {
            try!(os.write_uint32(3, v));
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
        ::std::intrinsics::TypeId::of::<CAudioFormat>()
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CAudioFormat {
    fn new() -> CAudioFormat {
        CAudioFormat::new()
    }

    #[allow(unused_unsafe,unused_mut)]
    fn descriptor_static(_: ::std::option::Option<CAudioFormat>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_enum_accessor(
                    "format",
                    CAudioFormat::has_format,
                    CAudioFormat::get_format,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u32_accessor(
                    "frequency",
                    CAudioFormat::has_frequency,
                    CAudioFormat::get_frequency,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u32_accessor(
                    "channels",
                    CAudioFormat::has_channels,
                    CAudioFormat::get_channels,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CAudioFormat>(
                    "CAudioFormat",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CAudioFormat {
    fn clear(&mut self) {
        self.clear_format();
        self.clear_frequency();
        self.clear_channels();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for CAudioFormat {
    fn eq(&self, other: &CAudioFormat) -> bool {
        self.format == other.format &&
        self.frequency == other.frequency &&
        self.channels == other.channels &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Show for CAudioFormat {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct CVideoFormat {
    format: ::std::option::Option<EVideoFormat>,
    width: ::std::option::Option<u32>,
    height: ::std::option::Option<u32>,
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl CVideoFormat {
    pub fn new() -> CVideoFormat {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CVideoFormat {
        static mut instance: ::protobuf::lazy::Lazy<CVideoFormat> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CVideoFormat,
        };
        unsafe {
            instance.get(|| {
                CVideoFormat {
                    format: ::std::option::Option::None,
                    width: ::std::option::Option::None,
                    height: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required .EVideoFormat format = 1;

    pub fn clear_format(&mut self) {
        self.format = ::std::option::Option::None;
    }

    pub fn has_format(&self) -> bool {
        self.format.is_some()
    }

    // Param is passed by value, moved
    pub fn set_format(&mut self, v: EVideoFormat) {
        self.format = ::std::option::Option::Some(v);
    }

    pub fn get_format<'a>(&self) -> EVideoFormat {
        self.format.unwrap_or(EVideoFormat::k_EVideoFormatNone)
    }

    // optional uint32 width = 2;

    pub fn clear_width(&mut self) {
        self.width = ::std::option::Option::None;
    }

    pub fn has_width(&self) -> bool {
        self.width.is_some()
    }

    // Param is passed by value, moved
    pub fn set_width(&mut self, v: u32) {
        self.width = ::std::option::Option::Some(v);
    }

    pub fn get_width<'a>(&self) -> u32 {
        self.width.unwrap_or(0)
    }

    // optional uint32 height = 3;

    pub fn clear_height(&mut self) {
        self.height = ::std::option::Option::None;
    }

    pub fn has_height(&self) -> bool {
        self.height.is_some()
    }

    // Param is passed by value, moved
    pub fn set_height(&mut self, v: u32) {
        self.height = ::std::option::Option::Some(v);
    }

    pub fn get_height<'a>(&self) -> u32 {
        self.height.unwrap_or(0)
    }
}

impl ::protobuf::Message for CVideoFormat {
    fn is_initialized(&self) -> bool {
        if self.format.is_none() {
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
                    self.format = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_uint32());
                    self.width = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_uint32());
                    self.height = ::std::option::Option::Some(tmp);
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
        for value in self.format.iter() {
            my_size += ::protobuf::rt::enum_size(1, *value);
        };
        for value in self.width.iter() {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.height.iter() {
            my_size += ::protobuf::rt::value_size(3, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.format {
            try!(os.write_enum(1, v as i32));
        };
        if let Some(v) = self.width {
            try!(os.write_uint32(2, v));
        };
        if let Some(v) = self.height {
            try!(os.write_uint32(3, v));
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
        ::std::intrinsics::TypeId::of::<CVideoFormat>()
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CVideoFormat {
    fn new() -> CVideoFormat {
        CVideoFormat::new()
    }

    #[allow(unused_unsafe,unused_mut)]
    fn descriptor_static(_: ::std::option::Option<CVideoFormat>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_enum_accessor(
                    "format",
                    CVideoFormat::has_format,
                    CVideoFormat::get_format,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u32_accessor(
                    "width",
                    CVideoFormat::has_width,
                    CVideoFormat::get_width,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u32_accessor(
                    "height",
                    CVideoFormat::has_height,
                    CVideoFormat::get_height,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CVideoFormat>(
                    "CVideoFormat",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CVideoFormat {
    fn clear(&mut self) {
        self.clear_format();
        self.clear_width();
        self.clear_height();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for CVideoFormat {
    fn eq(&self, other: &CVideoFormat) -> bool {
        self.format == other.format &&
        self.width == other.width &&
        self.height == other.height &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Show for CVideoFormat {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct CFrameEvent {
    event_id: ::std::option::Option<EStreamFrameEvent>,
    timestamp: ::std::option::Option<u32>,
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl CFrameEvent {
    pub fn new() -> CFrameEvent {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CFrameEvent {
        static mut instance: ::protobuf::lazy::Lazy<CFrameEvent> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CFrameEvent,
        };
        unsafe {
            instance.get(|| {
                CFrameEvent {
                    event_id: ::std::option::Option::None,
                    timestamp: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required .EStreamFrameEvent event_id = 1;

    pub fn clear_event_id(&mut self) {
        self.event_id = ::std::option::Option::None;
    }

    pub fn has_event_id(&self) -> bool {
        self.event_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_event_id(&mut self, v: EStreamFrameEvent) {
        self.event_id = ::std::option::Option::Some(v);
    }

    pub fn get_event_id<'a>(&self) -> EStreamFrameEvent {
        self.event_id.unwrap_or(EStreamFrameEvent::k_EStreamInputEventStart)
    }

    // required uint32 timestamp = 2;

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

impl ::protobuf::Message for CFrameEvent {
    fn is_initialized(&self) -> bool {
        if self.event_id.is_none() {
            return false;
        };
        if self.timestamp.is_none() {
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
                    self.event_id = ::std::option::Option::Some(tmp);
                },
                2 => {
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
        for value in self.event_id.iter() {
            my_size += ::protobuf::rt::enum_size(1, *value);
        };
        for value in self.timestamp.iter() {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.event_id {
            try!(os.write_enum(1, v as i32));
        };
        if let Some(v) = self.timestamp {
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
        ::std::intrinsics::TypeId::of::<CFrameEvent>()
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CFrameEvent {
    fn new() -> CFrameEvent {
        CFrameEvent::new()
    }

    #[allow(unused_unsafe,unused_mut)]
    fn descriptor_static(_: ::std::option::Option<CFrameEvent>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_enum_accessor(
                    "event_id",
                    CFrameEvent::has_event_id,
                    CFrameEvent::get_event_id,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u32_accessor(
                    "timestamp",
                    CFrameEvent::has_timestamp,
                    CFrameEvent::get_timestamp,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CFrameEvent>(
                    "CFrameEvent",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CFrameEvent {
    fn clear(&mut self) {
        self.clear_event_id();
        self.clear_timestamp();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for CFrameEvent {
    fn eq(&self, other: &CFrameEvent) -> bool {
        self.event_id == other.event_id &&
        self.timestamp == other.timestamp &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Show for CFrameEvent {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct CFrameStats {
    frame_id: ::std::option::Option<u32>,
    input_mark: ::std::option::Option<u32>,
    events: ::protobuf::RepeatedField<CFrameEvent>,
    result: ::std::option::Option<EStreamFrameResult>,
    frame_start_delta: ::std::option::Option<f32>,
    frame_display_delta: ::std::option::Option<f32>,
    ping_time: ::std::option::Option<f32>,
    server_bitrate: ::std::option::Option<f32>,
    client_bitrate: ::std::option::Option<f32>,
    link_bandwidth: ::std::option::Option<f32>,
    packet_loss: ::std::option::Option<f32>,
    frame_size: ::std::option::Option<u32>,
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl CFrameStats {
    pub fn new() -> CFrameStats {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CFrameStats {
        static mut instance: ::protobuf::lazy::Lazy<CFrameStats> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CFrameStats,
        };
        unsafe {
            instance.get(|| {
                CFrameStats {
                    frame_id: ::std::option::Option::None,
                    input_mark: ::std::option::Option::None,
                    events: ::protobuf::RepeatedField::new(),
                    result: ::std::option::Option::None,
                    frame_start_delta: ::std::option::Option::None,
                    frame_display_delta: ::std::option::Option::None,
                    ping_time: ::std::option::Option::None,
                    server_bitrate: ::std::option::Option::None,
                    client_bitrate: ::std::option::Option::None,
                    link_bandwidth: ::std::option::Option::None,
                    packet_loss: ::std::option::Option::None,
                    frame_size: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required uint32 frame_id = 1;

    pub fn clear_frame_id(&mut self) {
        self.frame_id = ::std::option::Option::None;
    }

    pub fn has_frame_id(&self) -> bool {
        self.frame_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_frame_id(&mut self, v: u32) {
        self.frame_id = ::std::option::Option::Some(v);
    }

    pub fn get_frame_id<'a>(&self) -> u32 {
        self.frame_id.unwrap_or(0)
    }

    // optional uint32 input_mark = 2;

    pub fn clear_input_mark(&mut self) {
        self.input_mark = ::std::option::Option::None;
    }

    pub fn has_input_mark(&self) -> bool {
        self.input_mark.is_some()
    }

    // Param is passed by value, moved
    pub fn set_input_mark(&mut self, v: u32) {
        self.input_mark = ::std::option::Option::Some(v);
    }

    pub fn get_input_mark<'a>(&self) -> u32 {
        self.input_mark.unwrap_or(0)
    }

    // repeated .CFrameEvent events = 3;

    pub fn clear_events(&mut self) {
        self.events.clear();
    }

    // Param is passed by value, moved
    pub fn set_events(&mut self, v: ::protobuf::RepeatedField<CFrameEvent>) {
        self.events = v;
    }

    // Mutable pointer to the field.
    pub fn mut_events<'a>(&'a mut self) -> &'a mut ::protobuf::RepeatedField<CFrameEvent> {
        &mut self.events
    }

    // Take field
    pub fn take_events(&mut self) -> ::protobuf::RepeatedField<CFrameEvent> {
        ::std::mem::replace(&mut self.events, ::protobuf::RepeatedField::new())
    }

    pub fn get_events<'a>(&'a self) -> &'a [CFrameEvent] {
        self.events.as_slice()
    }

    // required .EStreamFrameResult result = 4;

    pub fn clear_result(&mut self) {
        self.result = ::std::option::Option::None;
    }

    pub fn has_result(&self) -> bool {
        self.result.is_some()
    }

    // Param is passed by value, moved
    pub fn set_result(&mut self, v: EStreamFrameResult) {
        self.result = ::std::option::Option::Some(v);
    }

    pub fn get_result<'a>(&self) -> EStreamFrameResult {
        self.result.unwrap_or(EStreamFrameResult::k_EStreamFrameResultPending)
    }

    // optional float frame_start_delta = 5;

    pub fn clear_frame_start_delta(&mut self) {
        self.frame_start_delta = ::std::option::Option::None;
    }

    pub fn has_frame_start_delta(&self) -> bool {
        self.frame_start_delta.is_some()
    }

    // Param is passed by value, moved
    pub fn set_frame_start_delta(&mut self, v: f32) {
        self.frame_start_delta = ::std::option::Option::Some(v);
    }

    pub fn get_frame_start_delta<'a>(&self) -> f32 {
        self.frame_start_delta.unwrap_or(0.)
    }

    // optional float frame_display_delta = 6;

    pub fn clear_frame_display_delta(&mut self) {
        self.frame_display_delta = ::std::option::Option::None;
    }

    pub fn has_frame_display_delta(&self) -> bool {
        self.frame_display_delta.is_some()
    }

    // Param is passed by value, moved
    pub fn set_frame_display_delta(&mut self, v: f32) {
        self.frame_display_delta = ::std::option::Option::Some(v);
    }

    pub fn get_frame_display_delta<'a>(&self) -> f32 {
        self.frame_display_delta.unwrap_or(0.)
    }

    // optional float ping_time = 7;

    pub fn clear_ping_time(&mut self) {
        self.ping_time = ::std::option::Option::None;
    }

    pub fn has_ping_time(&self) -> bool {
        self.ping_time.is_some()
    }

    // Param is passed by value, moved
    pub fn set_ping_time(&mut self, v: f32) {
        self.ping_time = ::std::option::Option::Some(v);
    }

    pub fn get_ping_time<'a>(&self) -> f32 {
        self.ping_time.unwrap_or(0.)
    }

    // optional float server_bitrate = 8;

    pub fn clear_server_bitrate(&mut self) {
        self.server_bitrate = ::std::option::Option::None;
    }

    pub fn has_server_bitrate(&self) -> bool {
        self.server_bitrate.is_some()
    }

    // Param is passed by value, moved
    pub fn set_server_bitrate(&mut self, v: f32) {
        self.server_bitrate = ::std::option::Option::Some(v);
    }

    pub fn get_server_bitrate<'a>(&self) -> f32 {
        self.server_bitrate.unwrap_or(0.)
    }

    // optional float client_bitrate = 9;

    pub fn clear_client_bitrate(&mut self) {
        self.client_bitrate = ::std::option::Option::None;
    }

    pub fn has_client_bitrate(&self) -> bool {
        self.client_bitrate.is_some()
    }

    // Param is passed by value, moved
    pub fn set_client_bitrate(&mut self, v: f32) {
        self.client_bitrate = ::std::option::Option::Some(v);
    }

    pub fn get_client_bitrate<'a>(&self) -> f32 {
        self.client_bitrate.unwrap_or(0.)
    }

    // optional float link_bandwidth = 10;

    pub fn clear_link_bandwidth(&mut self) {
        self.link_bandwidth = ::std::option::Option::None;
    }

    pub fn has_link_bandwidth(&self) -> bool {
        self.link_bandwidth.is_some()
    }

    // Param is passed by value, moved
    pub fn set_link_bandwidth(&mut self, v: f32) {
        self.link_bandwidth = ::std::option::Option::Some(v);
    }

    pub fn get_link_bandwidth<'a>(&self) -> f32 {
        self.link_bandwidth.unwrap_or(0.)
    }

    // optional float packet_loss = 11;

    pub fn clear_packet_loss(&mut self) {
        self.packet_loss = ::std::option::Option::None;
    }

    pub fn has_packet_loss(&self) -> bool {
        self.packet_loss.is_some()
    }

    // Param is passed by value, moved
    pub fn set_packet_loss(&mut self, v: f32) {
        self.packet_loss = ::std::option::Option::Some(v);
    }

    pub fn get_packet_loss<'a>(&self) -> f32 {
        self.packet_loss.unwrap_or(0.)
    }

    // optional uint32 frame_size = 12;

    pub fn clear_frame_size(&mut self) {
        self.frame_size = ::std::option::Option::None;
    }

    pub fn has_frame_size(&self) -> bool {
        self.frame_size.is_some()
    }

    // Param is passed by value, moved
    pub fn set_frame_size(&mut self, v: u32) {
        self.frame_size = ::std::option::Option::Some(v);
    }

    pub fn get_frame_size<'a>(&self) -> u32 {
        self.frame_size.unwrap_or(0)
    }
}

impl ::protobuf::Message for CFrameStats {
    fn is_initialized(&self) -> bool {
        if self.frame_id.is_none() {
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
                    self.frame_id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_uint32());
                    self.input_mark = ::std::option::Option::Some(tmp);
                },
                3 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.events));
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_enum());
                    self.result = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_float());
                    self.frame_start_delta = ::std::option::Option::Some(tmp);
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_float());
                    self.frame_display_delta = ::std::option::Option::Some(tmp);
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_float());
                    self.ping_time = ::std::option::Option::Some(tmp);
                },
                8 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_float());
                    self.server_bitrate = ::std::option::Option::Some(tmp);
                },
                9 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_float());
                    self.client_bitrate = ::std::option::Option::Some(tmp);
                },
                10 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_float());
                    self.link_bandwidth = ::std::option::Option::Some(tmp);
                },
                11 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_float());
                    self.packet_loss = ::std::option::Option::Some(tmp);
                },
                12 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_uint32());
                    self.frame_size = ::std::option::Option::Some(tmp);
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
        for value in self.frame_id.iter() {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.input_mark.iter() {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.events.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.result.iter() {
            my_size += ::protobuf::rt::enum_size(4, *value);
        };
        if self.frame_start_delta.is_some() {
            my_size += 5;
        };
        if self.frame_display_delta.is_some() {
            my_size += 5;
        };
        if self.ping_time.is_some() {
            my_size += 5;
        };
        if self.server_bitrate.is_some() {
            my_size += 5;
        };
        if self.client_bitrate.is_some() {
            my_size += 5;
        };
        if self.link_bandwidth.is_some() {
            my_size += 5;
        };
        if self.packet_loss.is_some() {
            my_size += 5;
        };
        for value in self.frame_size.iter() {
            my_size += ::protobuf::rt::value_size(12, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.frame_id {
            try!(os.write_uint32(1, v));
        };
        if let Some(v) = self.input_mark {
            try!(os.write_uint32(2, v));
        };
        for v in self.events.iter() {
            try!(os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.result {
            try!(os.write_enum(4, v as i32));
        };
        if let Some(v) = self.frame_start_delta {
            try!(os.write_float(5, v));
        };
        if let Some(v) = self.frame_display_delta {
            try!(os.write_float(6, v));
        };
        if let Some(v) = self.ping_time {
            try!(os.write_float(7, v));
        };
        if let Some(v) = self.server_bitrate {
            try!(os.write_float(8, v));
        };
        if let Some(v) = self.client_bitrate {
            try!(os.write_float(9, v));
        };
        if let Some(v) = self.link_bandwidth {
            try!(os.write_float(10, v));
        };
        if let Some(v) = self.packet_loss {
            try!(os.write_float(11, v));
        };
        if let Some(v) = self.frame_size {
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
        ::std::intrinsics::TypeId::of::<CFrameStats>()
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CFrameStats {
    fn new() -> CFrameStats {
        CFrameStats::new()
    }

    #[allow(unused_unsafe,unused_mut)]
    fn descriptor_static(_: ::std::option::Option<CFrameStats>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_u32_accessor(
                    "frame_id",
                    CFrameStats::has_frame_id,
                    CFrameStats::get_frame_id,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u32_accessor(
                    "input_mark",
                    CFrameStats::has_input_mark,
                    CFrameStats::get_input_mark,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "events",
                    CFrameStats::get_events,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_enum_accessor(
                    "result",
                    CFrameStats::has_result,
                    CFrameStats::get_result,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f32_accessor(
                    "frame_start_delta",
                    CFrameStats::has_frame_start_delta,
                    CFrameStats::get_frame_start_delta,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f32_accessor(
                    "frame_display_delta",
                    CFrameStats::has_frame_display_delta,
                    CFrameStats::get_frame_display_delta,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f32_accessor(
                    "ping_time",
                    CFrameStats::has_ping_time,
                    CFrameStats::get_ping_time,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f32_accessor(
                    "server_bitrate",
                    CFrameStats::has_server_bitrate,
                    CFrameStats::get_server_bitrate,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f32_accessor(
                    "client_bitrate",
                    CFrameStats::has_client_bitrate,
                    CFrameStats::get_client_bitrate,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f32_accessor(
                    "link_bandwidth",
                    CFrameStats::has_link_bandwidth,
                    CFrameStats::get_link_bandwidth,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f32_accessor(
                    "packet_loss",
                    CFrameStats::has_packet_loss,
                    CFrameStats::get_packet_loss,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u32_accessor(
                    "frame_size",
                    CFrameStats::has_frame_size,
                    CFrameStats::get_frame_size,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CFrameStats>(
                    "CFrameStats",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CFrameStats {
    fn clear(&mut self) {
        self.clear_frame_id();
        self.clear_input_mark();
        self.clear_events();
        self.clear_result();
        self.clear_frame_start_delta();
        self.clear_frame_display_delta();
        self.clear_ping_time();
        self.clear_server_bitrate();
        self.clear_client_bitrate();
        self.clear_link_bandwidth();
        self.clear_packet_loss();
        self.clear_frame_size();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for CFrameStats {
    fn eq(&self, other: &CFrameStats) -> bool {
        self.frame_id == other.frame_id &&
        self.input_mark == other.input_mark &&
        self.events == other.events &&
        self.result == other.result &&
        self.frame_start_delta == other.frame_start_delta &&
        self.frame_display_delta == other.frame_display_delta &&
        self.ping_time == other.ping_time &&
        self.server_bitrate == other.server_bitrate &&
        self.client_bitrate == other.client_bitrate &&
        self.link_bandwidth == other.link_bandwidth &&
        self.packet_loss == other.packet_loss &&
        self.frame_size == other.frame_size &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Show for CFrameStats {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct CFrameStatAccumulatedValue {
    stat_type: ::std::option::Option<EFrameAccumulatedStat>,
    count: ::std::option::Option<i32>,
    average: ::std::option::Option<f32>,
    stddev: ::std::option::Option<f32>,
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl CFrameStatAccumulatedValue {
    pub fn new() -> CFrameStatAccumulatedValue {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CFrameStatAccumulatedValue {
        static mut instance: ::protobuf::lazy::Lazy<CFrameStatAccumulatedValue> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CFrameStatAccumulatedValue,
        };
        unsafe {
            instance.get(|| {
                CFrameStatAccumulatedValue {
                    stat_type: ::std::option::Option::None,
                    count: ::std::option::Option::None,
                    average: ::std::option::Option::None,
                    stddev: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required .EFrameAccumulatedStat stat_type = 1;

    pub fn clear_stat_type(&mut self) {
        self.stat_type = ::std::option::Option::None;
    }

    pub fn has_stat_type(&self) -> bool {
        self.stat_type.is_some()
    }

    // Param is passed by value, moved
    pub fn set_stat_type(&mut self, v: EFrameAccumulatedStat) {
        self.stat_type = ::std::option::Option::Some(v);
    }

    pub fn get_stat_type<'a>(&self) -> EFrameAccumulatedStat {
        self.stat_type.unwrap_or(EFrameAccumulatedStat::k_EFrameStatFPS)
    }

    // required int32 count = 2;

    pub fn clear_count(&mut self) {
        self.count = ::std::option::Option::None;
    }

    pub fn has_count(&self) -> bool {
        self.count.is_some()
    }

    // Param is passed by value, moved
    pub fn set_count(&mut self, v: i32) {
        self.count = ::std::option::Option::Some(v);
    }

    pub fn get_count<'a>(&self) -> i32 {
        self.count.unwrap_or(0)
    }

    // required float average = 3;

    pub fn clear_average(&mut self) {
        self.average = ::std::option::Option::None;
    }

    pub fn has_average(&self) -> bool {
        self.average.is_some()
    }

    // Param is passed by value, moved
    pub fn set_average(&mut self, v: f32) {
        self.average = ::std::option::Option::Some(v);
    }

    pub fn get_average<'a>(&self) -> f32 {
        self.average.unwrap_or(0.)
    }

    // optional float stddev = 4;

    pub fn clear_stddev(&mut self) {
        self.stddev = ::std::option::Option::None;
    }

    pub fn has_stddev(&self) -> bool {
        self.stddev.is_some()
    }

    // Param is passed by value, moved
    pub fn set_stddev(&mut self, v: f32) {
        self.stddev = ::std::option::Option::Some(v);
    }

    pub fn get_stddev<'a>(&self) -> f32 {
        self.stddev.unwrap_or(0.)
    }
}

impl ::protobuf::Message for CFrameStatAccumulatedValue {
    fn is_initialized(&self) -> bool {
        if self.stat_type.is_none() {
            return false;
        };
        if self.count.is_none() {
            return false;
        };
        if self.average.is_none() {
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
                    self.stat_type = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_int32());
                    self.count = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_float());
                    self.average = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_float());
                    self.stddev = ::std::option::Option::Some(tmp);
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
        for value in self.stat_type.iter() {
            my_size += ::protobuf::rt::enum_size(1, *value);
        };
        for value in self.count.iter() {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        if self.average.is_some() {
            my_size += 5;
        };
        if self.stddev.is_some() {
            my_size += 5;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.stat_type {
            try!(os.write_enum(1, v as i32));
        };
        if let Some(v) = self.count {
            try!(os.write_int32(2, v));
        };
        if let Some(v) = self.average {
            try!(os.write_float(3, v));
        };
        if let Some(v) = self.stddev {
            try!(os.write_float(4, v));
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
        ::std::intrinsics::TypeId::of::<CFrameStatAccumulatedValue>()
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CFrameStatAccumulatedValue {
    fn new() -> CFrameStatAccumulatedValue {
        CFrameStatAccumulatedValue::new()
    }

    #[allow(unused_unsafe,unused_mut)]
    fn descriptor_static(_: ::std::option::Option<CFrameStatAccumulatedValue>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_enum_accessor(
                    "stat_type",
                    CFrameStatAccumulatedValue::has_stat_type,
                    CFrameStatAccumulatedValue::get_stat_type,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "count",
                    CFrameStatAccumulatedValue::has_count,
                    CFrameStatAccumulatedValue::get_count,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f32_accessor(
                    "average",
                    CFrameStatAccumulatedValue::has_average,
                    CFrameStatAccumulatedValue::get_average,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f32_accessor(
                    "stddev",
                    CFrameStatAccumulatedValue::has_stddev,
                    CFrameStatAccumulatedValue::get_stddev,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CFrameStatAccumulatedValue>(
                    "CFrameStatAccumulatedValue",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CFrameStatAccumulatedValue {
    fn clear(&mut self) {
        self.clear_stat_type();
        self.clear_count();
        self.clear_average();
        self.clear_stddev();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for CFrameStatAccumulatedValue {
    fn eq(&self, other: &CFrameStatAccumulatedValue) -> bool {
        self.stat_type == other.stat_type &&
        self.count == other.count &&
        self.average == other.average &&
        self.stddev == other.stddev &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Show for CFrameStatAccumulatedValue {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct CFrameStatsListMsg {
    data_type: ::std::option::Option<EStreamingDataType>,
    stats: ::protobuf::RepeatedField<CFrameStats>,
    accumulated_stats: ::protobuf::RepeatedField<CFrameStatAccumulatedValue>,
    latest_frame_id: ::std::option::Option<i32>,
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl CFrameStatsListMsg {
    pub fn new() -> CFrameStatsListMsg {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CFrameStatsListMsg {
        static mut instance: ::protobuf::lazy::Lazy<CFrameStatsListMsg> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CFrameStatsListMsg,
        };
        unsafe {
            instance.get(|| {
                CFrameStatsListMsg {
                    data_type: ::std::option::Option::None,
                    stats: ::protobuf::RepeatedField::new(),
                    accumulated_stats: ::protobuf::RepeatedField::new(),
                    latest_frame_id: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required .EStreamingDataType data_type = 1;

    pub fn clear_data_type(&mut self) {
        self.data_type = ::std::option::Option::None;
    }

    pub fn has_data_type(&self) -> bool {
        self.data_type.is_some()
    }

    // Param is passed by value, moved
    pub fn set_data_type(&mut self, v: EStreamingDataType) {
        self.data_type = ::std::option::Option::Some(v);
    }

    pub fn get_data_type<'a>(&self) -> EStreamingDataType {
        self.data_type.unwrap_or(EStreamingDataType::k_EStreamingAudioData)
    }

    // repeated .CFrameStats stats = 2;

    pub fn clear_stats(&mut self) {
        self.stats.clear();
    }

    // Param is passed by value, moved
    pub fn set_stats(&mut self, v: ::protobuf::RepeatedField<CFrameStats>) {
        self.stats = v;
    }

    // Mutable pointer to the field.
    pub fn mut_stats<'a>(&'a mut self) -> &'a mut ::protobuf::RepeatedField<CFrameStats> {
        &mut self.stats
    }

    // Take field
    pub fn take_stats(&mut self) -> ::protobuf::RepeatedField<CFrameStats> {
        ::std::mem::replace(&mut self.stats, ::protobuf::RepeatedField::new())
    }

    pub fn get_stats<'a>(&'a self) -> &'a [CFrameStats] {
        self.stats.as_slice()
    }

    // repeated .CFrameStatAccumulatedValue accumulated_stats = 3;

    pub fn clear_accumulated_stats(&mut self) {
        self.accumulated_stats.clear();
    }

    // Param is passed by value, moved
    pub fn set_accumulated_stats(&mut self, v: ::protobuf::RepeatedField<CFrameStatAccumulatedValue>) {
        self.accumulated_stats = v;
    }

    // Mutable pointer to the field.
    pub fn mut_accumulated_stats<'a>(&'a mut self) -> &'a mut ::protobuf::RepeatedField<CFrameStatAccumulatedValue> {
        &mut self.accumulated_stats
    }

    // Take field
    pub fn take_accumulated_stats(&mut self) -> ::protobuf::RepeatedField<CFrameStatAccumulatedValue> {
        ::std::mem::replace(&mut self.accumulated_stats, ::protobuf::RepeatedField::new())
    }

    pub fn get_accumulated_stats<'a>(&'a self) -> &'a [CFrameStatAccumulatedValue] {
        self.accumulated_stats.as_slice()
    }

    // required int32 latest_frame_id = 4;

    pub fn clear_latest_frame_id(&mut self) {
        self.latest_frame_id = ::std::option::Option::None;
    }

    pub fn has_latest_frame_id(&self) -> bool {
        self.latest_frame_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_latest_frame_id(&mut self, v: i32) {
        self.latest_frame_id = ::std::option::Option::Some(v);
    }

    pub fn get_latest_frame_id<'a>(&self) -> i32 {
        self.latest_frame_id.unwrap_or(0)
    }
}

impl ::protobuf::Message for CFrameStatsListMsg {
    fn is_initialized(&self) -> bool {
        if self.data_type.is_none() {
            return false;
        };
        if self.latest_frame_id.is_none() {
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
                    self.data_type = ::std::option::Option::Some(tmp);
                },
                2 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.stats));
                },
                3 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.accumulated_stats));
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_int32());
                    self.latest_frame_id = ::std::option::Option::Some(tmp);
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
        for value in self.data_type.iter() {
            my_size += ::protobuf::rt::enum_size(1, *value);
        };
        for value in self.stats.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.accumulated_stats.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.latest_frame_id.iter() {
            my_size += ::protobuf::rt::value_size(4, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.data_type {
            try!(os.write_enum(1, v as i32));
        };
        for v in self.stats.iter() {
            try!(os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        for v in self.accumulated_stats.iter() {
            try!(os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.latest_frame_id {
            try!(os.write_int32(4, v));
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
        ::std::intrinsics::TypeId::of::<CFrameStatsListMsg>()
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CFrameStatsListMsg {
    fn new() -> CFrameStatsListMsg {
        CFrameStatsListMsg::new()
    }

    #[allow(unused_unsafe,unused_mut)]
    fn descriptor_static(_: ::std::option::Option<CFrameStatsListMsg>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_enum_accessor(
                    "data_type",
                    CFrameStatsListMsg::has_data_type,
                    CFrameStatsListMsg::get_data_type,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "stats",
                    CFrameStatsListMsg::get_stats,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "accumulated_stats",
                    CFrameStatsListMsg::get_accumulated_stats,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "latest_frame_id",
                    CFrameStatsListMsg::has_latest_frame_id,
                    CFrameStatsListMsg::get_latest_frame_id,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CFrameStatsListMsg>(
                    "CFrameStatsListMsg",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CFrameStatsListMsg {
    fn clear(&mut self) {
        self.clear_data_type();
        self.clear_stats();
        self.clear_accumulated_stats();
        self.clear_latest_frame_id();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for CFrameStatsListMsg {
    fn eq(&self, other: &CFrameStatsListMsg) -> bool {
        self.data_type == other.data_type &&
        self.stats == other.stats &&
        self.accumulated_stats == other.accumulated_stats &&
        self.latest_frame_id == other.latest_frame_id &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Show for CFrameStatsListMsg {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct CDebugDumpMsg {
    screenshot: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl CDebugDumpMsg {
    pub fn new() -> CDebugDumpMsg {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CDebugDumpMsg {
        static mut instance: ::protobuf::lazy::Lazy<CDebugDumpMsg> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CDebugDumpMsg,
        };
        unsafe {
            instance.get(|| {
                CDebugDumpMsg {
                    screenshot: ::protobuf::SingularField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional bytes screenshot = 1;

    pub fn clear_screenshot(&mut self) {
        self.screenshot.clear();
    }

    pub fn has_screenshot(&self) -> bool {
        self.screenshot.is_some()
    }

    // Param is passed by value, moved
    pub fn set_screenshot(&mut self, v: ::std::vec::Vec<u8>) {
        self.screenshot = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_screenshot<'a>(&'a mut self) -> &'a mut ::std::vec::Vec<u8> {
        if self.screenshot.is_none() {
            self.screenshot.set_default();
        };
        self.screenshot.as_mut().unwrap()
    }

    // Take field
    pub fn take_screenshot(&mut self) -> ::std::vec::Vec<u8> {
        self.screenshot.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_screenshot<'a>(&'a self) -> &'a [u8] {
        match self.screenshot.as_ref() {
            Some(v) => v.as_slice(),
            None => [].as_slice(),
        }
    }
}

impl ::protobuf::Message for CDebugDumpMsg {
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
                    let tmp = self.screenshot.set_default();
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
        for value in self.screenshot.iter() {
            my_size += ::protobuf::rt::bytes_size(1, value.as_slice());
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.screenshot.as_ref() {
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
        ::std::intrinsics::TypeId::of::<CDebugDumpMsg>()
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CDebugDumpMsg {
    fn new() -> CDebugDumpMsg {
        CDebugDumpMsg::new()
    }

    #[allow(unused_unsafe,unused_mut)]
    fn descriptor_static(_: ::std::option::Option<CDebugDumpMsg>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "screenshot",
                    CDebugDumpMsg::has_screenshot,
                    CDebugDumpMsg::get_screenshot,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CDebugDumpMsg>(
                    "CDebugDumpMsg",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CDebugDumpMsg {
    fn clear(&mut self) {
        self.clear_screenshot();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for CDebugDumpMsg {
    fn eq(&self, other: &CDebugDumpMsg) -> bool {
        self.screenshot == other.screenshot &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Show for CDebugDumpMsg {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,PartialEq,Eq,Show)]
pub enum EStreamChannel {
    k_EStreamChannelInvalid = -1,
    k_EStreamChannelDiscovery = 0,
    k_EStreamChannelControl = 1,
    k_EStreamChannelStats = 2,
    k_EStreamChannelDataChannelStart = 3,
}

impl ::protobuf::ProtobufEnum for EStreamChannel {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<EStreamChannel> {
        match value {
            -1 => ::std::option::Option::Some(EStreamChannel::k_EStreamChannelInvalid),
            0 => ::std::option::Option::Some(EStreamChannel::k_EStreamChannelDiscovery),
            1 => ::std::option::Option::Some(EStreamChannel::k_EStreamChannelControl),
            2 => ::std::option::Option::Some(EStreamChannel::k_EStreamChannelStats),
            3 => ::std::option::Option::Some(EStreamChannel::k_EStreamChannelDataChannelStart),
            _ => ::std::option::Option::None
        }
    }

    fn enum_descriptor_static(_: Option<EStreamChannel>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("EStreamChannel", file_descriptor_proto())
            })
        }
    }
}

impl ::std::kinds::Copy for EStreamChannel {
}

#[derive(Clone,PartialEq,Eq,Show)]
pub enum EStreamDiscoveryMessage {
    k_EStreamDiscoveryRequest = 1,
    k_EStreamDiscoveryResponse = 2,
}

impl ::protobuf::ProtobufEnum for EStreamDiscoveryMessage {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<EStreamDiscoveryMessage> {
        match value {
            1 => ::std::option::Option::Some(EStreamDiscoveryMessage::k_EStreamDiscoveryRequest),
            2 => ::std::option::Option::Some(EStreamDiscoveryMessage::k_EStreamDiscoveryResponse),
            _ => ::std::option::Option::None
        }
    }

    fn enum_descriptor_static(_: Option<EStreamDiscoveryMessage>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("EStreamDiscoveryMessage", file_descriptor_proto())
            })
        }
    }
}

impl ::std::kinds::Copy for EStreamDiscoveryMessage {
}

#[derive(Clone,PartialEq,Eq,Show)]
pub enum EStreamControlMessage {
    k_EStreamControlAuthenticationRequest = 1,
    k_EStreamControlAuthenticationResponse = 2,
    k_EStreamControlNegotiationInit = 3,
    k_EStreamControlNegotiationSetConfig = 4,
    k_EStreamControlNegotiationComplete = 5,
    k_EStreamControl_LAST_SETUP_MESSAGE = 15,
    k_EStreamControlStartAudioData = 50,
    k_EStreamControlStopAudioData = 51,
    k_EStreamControlStartVideoData = 52,
    k_EStreamControlStopVideoData = 53,
    k_EStreamControlInputMouseMotion = 54,
    k_EStreamControlInputMouseWheel = 55,
    k_EStreamControlInputMouseDown = 56,
    k_EStreamControlInputMouseUp = 57,
    k_EStreamControlInputKeyDown = 58,
    k_EStreamControlInputKeyUp = 59,
    k_EStreamControlInputGamepadAttached = 60,
    k_EStreamControlInputGamepadEvent = 61,
    k_EStreamControlInputGamepadDetached = 62,
    k_EStreamControlShowCursor = 63,
    k_EStreamControlHideCursor = 64,
    k_EStreamControlSetCursor = 65,
    k_EStreamControlGetCursorImage = 66,
    k_EStreamControlSetCursorImage = 67,
    k_EStreamControlDeleteCursor = 68,
    k_EStreamControlSetTargetFramerate = 69,
    k_EStreamControlInputLatencyTest = 70,
    k_EStreamControlGamepadRumble = 71,
    k_EStreamControlSetMaximumFramerate = 72,
    k_EStreamControlSetMaximumBitrate = 73,
    k_EStreamControlOverlayEnabled = 74,
    k_EStreamControlInputControllerAttached = 75,
    k_EStreamControlInputControllerState = 76,
    k_EStreamControlTriggerHapticPulse = 77,
    k_EStreamControlInputControllerDetached = 78,
    k_EStreamControlSystemInfo = 79,
    k_EStreamControlVideoDecoderInfo = 80,
    k_EStreamControlSetTitle = 81,
    k_EStreamControlSetIcon = 82,
    k_EStreamControlQuitRequest = 83,
    k_EStreamControlSetOverrideMode = 84,
    k_EStreamControlSetMaximumResolution = 85,
    k_EStreamControlSetQualityPreference = 86,
    k_EStreamControlSetQoS = 87,
    k_EStreamControlInputControllerWirelessPresence = 88,
    k_EStreamControlSetGammaRamp = 89,
    k_EStreamControlVideoEncoderInfo = 90,
}

impl ::protobuf::ProtobufEnum for EStreamControlMessage {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<EStreamControlMessage> {
        match value {
            1 => ::std::option::Option::Some(EStreamControlMessage::k_EStreamControlAuthenticationRequest),
            2 => ::std::option::Option::Some(EStreamControlMessage::k_EStreamControlAuthenticationResponse),
            3 => ::std::option::Option::Some(EStreamControlMessage::k_EStreamControlNegotiationInit),
            4 => ::std::option::Option::Some(EStreamControlMessage::k_EStreamControlNegotiationSetConfig),
            5 => ::std::option::Option::Some(EStreamControlMessage::k_EStreamControlNegotiationComplete),
            15 => ::std::option::Option::Some(EStreamControlMessage::k_EStreamControl_LAST_SETUP_MESSAGE),
            50 => ::std::option::Option::Some(EStreamControlMessage::k_EStreamControlStartAudioData),
            51 => ::std::option::Option::Some(EStreamControlMessage::k_EStreamControlStopAudioData),
            52 => ::std::option::Option::Some(EStreamControlMessage::k_EStreamControlStartVideoData),
            53 => ::std::option::Option::Some(EStreamControlMessage::k_EStreamControlStopVideoData),
            54 => ::std::option::Option::Some(EStreamControlMessage::k_EStreamControlInputMouseMotion),
            55 => ::std::option::Option::Some(EStreamControlMessage::k_EStreamControlInputMouseWheel),
            56 => ::std::option::Option::Some(EStreamControlMessage::k_EStreamControlInputMouseDown),
            57 => ::std::option::Option::Some(EStreamControlMessage::k_EStreamControlInputMouseUp),
            58 => ::std::option::Option::Some(EStreamControlMessage::k_EStreamControlInputKeyDown),
            59 => ::std::option::Option::Some(EStreamControlMessage::k_EStreamControlInputKeyUp),
            60 => ::std::option::Option::Some(EStreamControlMessage::k_EStreamControlInputGamepadAttached),
            61 => ::std::option::Option::Some(EStreamControlMessage::k_EStreamControlInputGamepadEvent),
            62 => ::std::option::Option::Some(EStreamControlMessage::k_EStreamControlInputGamepadDetached),
            63 => ::std::option::Option::Some(EStreamControlMessage::k_EStreamControlShowCursor),
            64 => ::std::option::Option::Some(EStreamControlMessage::k_EStreamControlHideCursor),
            65 => ::std::option::Option::Some(EStreamControlMessage::k_EStreamControlSetCursor),
            66 => ::std::option::Option::Some(EStreamControlMessage::k_EStreamControlGetCursorImage),
            67 => ::std::option::Option::Some(EStreamControlMessage::k_EStreamControlSetCursorImage),
            68 => ::std::option::Option::Some(EStreamControlMessage::k_EStreamControlDeleteCursor),
            69 => ::std::option::Option::Some(EStreamControlMessage::k_EStreamControlSetTargetFramerate),
            70 => ::std::option::Option::Some(EStreamControlMessage::k_EStreamControlInputLatencyTest),
            71 => ::std::option::Option::Some(EStreamControlMessage::k_EStreamControlGamepadRumble),
            72 => ::std::option::Option::Some(EStreamControlMessage::k_EStreamControlSetMaximumFramerate),
            73 => ::std::option::Option::Some(EStreamControlMessage::k_EStreamControlSetMaximumBitrate),
            74 => ::std::option::Option::Some(EStreamControlMessage::k_EStreamControlOverlayEnabled),
            75 => ::std::option::Option::Some(EStreamControlMessage::k_EStreamControlInputControllerAttached),
            76 => ::std::option::Option::Some(EStreamControlMessage::k_EStreamControlInputControllerState),
            77 => ::std::option::Option::Some(EStreamControlMessage::k_EStreamControlTriggerHapticPulse),
            78 => ::std::option::Option::Some(EStreamControlMessage::k_EStreamControlInputControllerDetached),
            79 => ::std::option::Option::Some(EStreamControlMessage::k_EStreamControlSystemInfo),
            80 => ::std::option::Option::Some(EStreamControlMessage::k_EStreamControlVideoDecoderInfo),
            81 => ::std::option::Option::Some(EStreamControlMessage::k_EStreamControlSetTitle),
            82 => ::std::option::Option::Some(EStreamControlMessage::k_EStreamControlSetIcon),
            83 => ::std::option::Option::Some(EStreamControlMessage::k_EStreamControlQuitRequest),
            84 => ::std::option::Option::Some(EStreamControlMessage::k_EStreamControlSetOverrideMode),
            85 => ::std::option::Option::Some(EStreamControlMessage::k_EStreamControlSetMaximumResolution),
            86 => ::std::option::Option::Some(EStreamControlMessage::k_EStreamControlSetQualityPreference),
            87 => ::std::option::Option::Some(EStreamControlMessage::k_EStreamControlSetQoS),
            88 => ::std::option::Option::Some(EStreamControlMessage::k_EStreamControlInputControllerWirelessPresence),
            89 => ::std::option::Option::Some(EStreamControlMessage::k_EStreamControlSetGammaRamp),
            90 => ::std::option::Option::Some(EStreamControlMessage::k_EStreamControlVideoEncoderInfo),
            _ => ::std::option::Option::None
        }
    }

    fn enum_descriptor_static(_: Option<EStreamControlMessage>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("EStreamControlMessage", file_descriptor_proto())
            })
        }
    }
}

impl ::std::kinds::Copy for EStreamControlMessage {
}

#[derive(Clone,PartialEq,Eq,Show)]
pub enum EStreamVersion {
    k_EStreamVersionNone = 0,
    k_EStreamVersionCurrent = 1,
}

impl ::protobuf::ProtobufEnum for EStreamVersion {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<EStreamVersion> {
        match value {
            0 => ::std::option::Option::Some(EStreamVersion::k_EStreamVersionNone),
            1 => ::std::option::Option::Some(EStreamVersion::k_EStreamVersionCurrent),
            _ => ::std::option::Option::None
        }
    }

    fn enum_descriptor_static(_: Option<EStreamVersion>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("EStreamVersion", file_descriptor_proto())
            })
        }
    }
}

impl ::std::kinds::Copy for EStreamVersion {
}

#[derive(Clone,PartialEq,Eq,Show)]
pub enum EStreamAudioCodec {
    k_EStreamAudioCodecNone = 0,
    k_EStreamAudioCodecRaw = 1,
    k_EStreamAudioCodecVorbis = 2,
    k_EStreamAudioCodecOpus = 3,
    k_EStreamAudioCodecMP3 = 4,
    k_EStreamAudioCodecAAC = 5,
}

impl ::protobuf::ProtobufEnum for EStreamAudioCodec {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<EStreamAudioCodec> {
        match value {
            0 => ::std::option::Option::Some(EStreamAudioCodec::k_EStreamAudioCodecNone),
            1 => ::std::option::Option::Some(EStreamAudioCodec::k_EStreamAudioCodecRaw),
            2 => ::std::option::Option::Some(EStreamAudioCodec::k_EStreamAudioCodecVorbis),
            3 => ::std::option::Option::Some(EStreamAudioCodec::k_EStreamAudioCodecOpus),
            4 => ::std::option::Option::Some(EStreamAudioCodec::k_EStreamAudioCodecMP3),
            5 => ::std::option::Option::Some(EStreamAudioCodec::k_EStreamAudioCodecAAC),
            _ => ::std::option::Option::None
        }
    }

    fn enum_descriptor_static(_: Option<EStreamAudioCodec>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("EStreamAudioCodec", file_descriptor_proto())
            })
        }
    }
}

impl ::std::kinds::Copy for EStreamAudioCodec {
}

#[derive(Clone,PartialEq,Eq,Show)]
pub enum EStreamVideoCodec {
    k_EStreamVideoCodecNone = 0,
    k_EStreamVideoCodecRaw = 1,
    k_EStreamVideoCodecVP8 = 2,
    k_EStreamVideoCodecVP9 = 3,
    k_EStreamVideoCodecH264 = 4,
    k_EStreamVideoCodecH265 = 5,
    k_EStreamVideoCodecORBX1 = 6,
    k_EStreamVideoCodecORBX2 = 7,
}

impl ::protobuf::ProtobufEnum for EStreamVideoCodec {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<EStreamVideoCodec> {
        match value {
            0 => ::std::option::Option::Some(EStreamVideoCodec::k_EStreamVideoCodecNone),
            1 => ::std::option::Option::Some(EStreamVideoCodec::k_EStreamVideoCodecRaw),
            2 => ::std::option::Option::Some(EStreamVideoCodec::k_EStreamVideoCodecVP8),
            3 => ::std::option::Option::Some(EStreamVideoCodec::k_EStreamVideoCodecVP9),
            4 => ::std::option::Option::Some(EStreamVideoCodec::k_EStreamVideoCodecH264),
            5 => ::std::option::Option::Some(EStreamVideoCodec::k_EStreamVideoCodecH265),
            6 => ::std::option::Option::Some(EStreamVideoCodec::k_EStreamVideoCodecORBX1),
            7 => ::std::option::Option::Some(EStreamVideoCodec::k_EStreamVideoCodecORBX2),
            _ => ::std::option::Option::None
        }
    }

    fn enum_descriptor_static(_: Option<EStreamVideoCodec>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("EStreamVideoCodec", file_descriptor_proto())
            })
        }
    }
}

impl ::std::kinds::Copy for EStreamVideoCodec {
}

#[derive(Clone,PartialEq,Eq,Show)]
pub enum EStreamingDataType {
    k_EStreamingAudioData = 0,
    k_EStreamingVideoData = 1,
}

impl ::protobuf::ProtobufEnum for EStreamingDataType {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<EStreamingDataType> {
        match value {
            0 => ::std::option::Option::Some(EStreamingDataType::k_EStreamingAudioData),
            1 => ::std::option::Option::Some(EStreamingDataType::k_EStreamingVideoData),
            _ => ::std::option::Option::None
        }
    }

    fn enum_descriptor_static(_: Option<EStreamingDataType>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("EStreamingDataType", file_descriptor_proto())
            })
        }
    }
}

impl ::std::kinds::Copy for EStreamingDataType {
}

#[derive(Clone,PartialEq,Eq,Show)]
pub enum EStreamMouseButton {
    k_EStreamMouseButtonLeft = 1,
    k_EStreamMouseButtonRight = 2,
    k_EStreamMouseButtonMiddle = 16,
    k_EStreamMouseButtonX1 = 32,
    k_EStreamMouseButtonX2 = 64,
    k_EStreamMouseButtonUnknown = 4096,
}

impl ::protobuf::ProtobufEnum for EStreamMouseButton {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<EStreamMouseButton> {
        match value {
            1 => ::std::option::Option::Some(EStreamMouseButton::k_EStreamMouseButtonLeft),
            2 => ::std::option::Option::Some(EStreamMouseButton::k_EStreamMouseButtonRight),
            16 => ::std::option::Option::Some(EStreamMouseButton::k_EStreamMouseButtonMiddle),
            32 => ::std::option::Option::Some(EStreamMouseButton::k_EStreamMouseButtonX1),
            64 => ::std::option::Option::Some(EStreamMouseButton::k_EStreamMouseButtonX2),
            4096 => ::std::option::Option::Some(EStreamMouseButton::k_EStreamMouseButtonUnknown),
            _ => ::std::option::Option::None
        }
    }

    fn enum_descriptor_static(_: Option<EStreamMouseButton>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("EStreamMouseButton", file_descriptor_proto())
            })
        }
    }
}

impl ::std::kinds::Copy for EStreamMouseButton {
}

#[derive(Clone,PartialEq,Eq,Show)]
pub enum EStreamMouseWheelDirection {
    k_EStreamMouseWheelUp = 120,
    k_EStreamMouseWheelDown = -120,
    k_EStreamMouseWheelLeft = 3,
    k_EStreamMouseWheelRight = 4,
}

impl ::protobuf::ProtobufEnum for EStreamMouseWheelDirection {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<EStreamMouseWheelDirection> {
        match value {
            120 => ::std::option::Option::Some(EStreamMouseWheelDirection::k_EStreamMouseWheelUp),
            -120 => ::std::option::Option::Some(EStreamMouseWheelDirection::k_EStreamMouseWheelDown),
            3 => ::std::option::Option::Some(EStreamMouseWheelDirection::k_EStreamMouseWheelLeft),
            4 => ::std::option::Option::Some(EStreamMouseWheelDirection::k_EStreamMouseWheelRight),
            _ => ::std::option::Option::None
        }
    }

    fn enum_descriptor_static(_: Option<EStreamMouseWheelDirection>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("EStreamMouseWheelDirection", file_descriptor_proto())
            })
        }
    }
}

impl ::std::kinds::Copy for EStreamMouseWheelDirection {
}

#[derive(Clone,PartialEq,Eq,Show)]
pub enum EStreamGamepadInputType {
    k_EStreamGamepadInputInvalid = 0,
    k_EStreamGamepadInputDPadUp = 1,
    k_EStreamGamepadInputDPadDown = 2,
    k_EStreamGamepadInputDPadLeft = 4,
    k_EStreamGamepadInputDPadRight = 8,
    k_EStreamGamepadInputStart = 16,
    k_EStreamGamepadInputBack = 32,
    k_EStreamGamepadInputLeftThumb = 64,
    k_EStreamGamepadInputRightThumb = 128,
    k_EStreamGamepadInputLeftShoulder = 256,
    k_EStreamGamepadInputRightShoulder = 512,
    k_EStreamGamepadInputGuide = 1024,
    k_EStreamGamepadInputA = 4096,
    k_EStreamGamepadInputB = 8192,
    k_EStreamGamepadInputX = 16384,
    k_EStreamGamepadInputY = 32768,
    k_EStreamGamepadInputLeftThumbX = 65536,
    k_EStreamGamepadInputLeftThumbY = 131072,
    k_EStreamGamepadInputRightThumbX = 262144,
    k_EStreamGamepadInputRightThumbY = 524288,
    k_EStreamGamepadInputLeftTrigger = 1048576,
    k_EStreamGamepadInputRightTrigger = 2097152,
}

impl ::protobuf::ProtobufEnum for EStreamGamepadInputType {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<EStreamGamepadInputType> {
        match value {
            0 => ::std::option::Option::Some(EStreamGamepadInputType::k_EStreamGamepadInputInvalid),
            1 => ::std::option::Option::Some(EStreamGamepadInputType::k_EStreamGamepadInputDPadUp),
            2 => ::std::option::Option::Some(EStreamGamepadInputType::k_EStreamGamepadInputDPadDown),
            4 => ::std::option::Option::Some(EStreamGamepadInputType::k_EStreamGamepadInputDPadLeft),
            8 => ::std::option::Option::Some(EStreamGamepadInputType::k_EStreamGamepadInputDPadRight),
            16 => ::std::option::Option::Some(EStreamGamepadInputType::k_EStreamGamepadInputStart),
            32 => ::std::option::Option::Some(EStreamGamepadInputType::k_EStreamGamepadInputBack),
            64 => ::std::option::Option::Some(EStreamGamepadInputType::k_EStreamGamepadInputLeftThumb),
            128 => ::std::option::Option::Some(EStreamGamepadInputType::k_EStreamGamepadInputRightThumb),
            256 => ::std::option::Option::Some(EStreamGamepadInputType::k_EStreamGamepadInputLeftShoulder),
            512 => ::std::option::Option::Some(EStreamGamepadInputType::k_EStreamGamepadInputRightShoulder),
            1024 => ::std::option::Option::Some(EStreamGamepadInputType::k_EStreamGamepadInputGuide),
            4096 => ::std::option::Option::Some(EStreamGamepadInputType::k_EStreamGamepadInputA),
            8192 => ::std::option::Option::Some(EStreamGamepadInputType::k_EStreamGamepadInputB),
            16384 => ::std::option::Option::Some(EStreamGamepadInputType::k_EStreamGamepadInputX),
            32768 => ::std::option::Option::Some(EStreamGamepadInputType::k_EStreamGamepadInputY),
            65536 => ::std::option::Option::Some(EStreamGamepadInputType::k_EStreamGamepadInputLeftThumbX),
            131072 => ::std::option::Option::Some(EStreamGamepadInputType::k_EStreamGamepadInputLeftThumbY),
            262144 => ::std::option::Option::Some(EStreamGamepadInputType::k_EStreamGamepadInputRightThumbX),
            524288 => ::std::option::Option::Some(EStreamGamepadInputType::k_EStreamGamepadInputRightThumbY),
            1048576 => ::std::option::Option::Some(EStreamGamepadInputType::k_EStreamGamepadInputLeftTrigger),
            2097152 => ::std::option::Option::Some(EStreamGamepadInputType::k_EStreamGamepadInputRightTrigger),
            _ => ::std::option::Option::None
        }
    }

    fn enum_descriptor_static(_: Option<EStreamGamepadInputType>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("EStreamGamepadInputType", file_descriptor_proto())
            })
        }
    }
}

impl ::std::kinds::Copy for EStreamGamepadInputType {
}

#[derive(Clone,PartialEq,Eq,Show)]
pub enum EStreamQualityPreference {
    k_EStreamQualityFast = 1,
    k_EStreamQualityBalanced = 2,
    k_EStreamQualityBeautiful = 3,
}

impl ::protobuf::ProtobufEnum for EStreamQualityPreference {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<EStreamQualityPreference> {
        match value {
            1 => ::std::option::Option::Some(EStreamQualityPreference::k_EStreamQualityFast),
            2 => ::std::option::Option::Some(EStreamQualityPreference::k_EStreamQualityBalanced),
            3 => ::std::option::Option::Some(EStreamQualityPreference::k_EStreamQualityBeautiful),
            _ => ::std::option::Option::None
        }
    }

    fn enum_descriptor_static(_: Option<EStreamQualityPreference>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("EStreamQualityPreference", file_descriptor_proto())
            })
        }
    }
}

impl ::std::kinds::Copy for EStreamQualityPreference {
}

#[derive(Clone,PartialEq,Eq,Show)]
pub enum EStreamBitrate {
    k_EStreamBitrateAutodetect = -1,
    k_EStreamBitrateUnlimited = 0,
}

impl ::protobuf::ProtobufEnum for EStreamBitrate {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<EStreamBitrate> {
        match value {
            -1 => ::std::option::Option::Some(EStreamBitrate::k_EStreamBitrateAutodetect),
            0 => ::std::option::Option::Some(EStreamBitrate::k_EStreamBitrateUnlimited),
            _ => ::std::option::Option::None
        }
    }

    fn enum_descriptor_static(_: Option<EStreamBitrate>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("EStreamBitrate", file_descriptor_proto())
            })
        }
    }
}

impl ::std::kinds::Copy for EStreamBitrate {
}

#[derive(Clone,PartialEq,Eq,Show)]
pub enum EStreamFramerateLimiter {
    k_EStreamFramerateSlowCapture = 1,
    k_EStreamFramerateSlowConvert = 2,
    k_EStreamFramerateSlowEncode = 4,
    k_EStreamFramerateSlowNetwork = 8,
    k_EStreamFramerateSlowDecode = 16,
    k_EStreamFramerateSlowGame = 32,
    k_EStreamFramerateSlowDisplay = 64,
}

impl ::protobuf::ProtobufEnum for EStreamFramerateLimiter {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<EStreamFramerateLimiter> {
        match value {
            1 => ::std::option::Option::Some(EStreamFramerateLimiter::k_EStreamFramerateSlowCapture),
            2 => ::std::option::Option::Some(EStreamFramerateLimiter::k_EStreamFramerateSlowConvert),
            4 => ::std::option::Option::Some(EStreamFramerateLimiter::k_EStreamFramerateSlowEncode),
            8 => ::std::option::Option::Some(EStreamFramerateLimiter::k_EStreamFramerateSlowNetwork),
            16 => ::std::option::Option::Some(EStreamFramerateLimiter::k_EStreamFramerateSlowDecode),
            32 => ::std::option::Option::Some(EStreamFramerateLimiter::k_EStreamFramerateSlowGame),
            64 => ::std::option::Option::Some(EStreamFramerateLimiter::k_EStreamFramerateSlowDisplay),
            _ => ::std::option::Option::None
        }
    }

    fn enum_descriptor_static(_: Option<EStreamFramerateLimiter>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("EStreamFramerateLimiter", file_descriptor_proto())
            })
        }
    }
}

impl ::std::kinds::Copy for EStreamFramerateLimiter {
}

#[derive(Clone,PartialEq,Eq,Show)]
pub enum EStreamDataMessage {
    k_EStreamDataPacket = 1,
    k_EStreamDataLost = 2,
}

impl ::protobuf::ProtobufEnum for EStreamDataMessage {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<EStreamDataMessage> {
        match value {
            1 => ::std::option::Option::Some(EStreamDataMessage::k_EStreamDataPacket),
            2 => ::std::option::Option::Some(EStreamDataMessage::k_EStreamDataLost),
            _ => ::std::option::Option::None
        }
    }

    fn enum_descriptor_static(_: Option<EStreamDataMessage>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("EStreamDataMessage", file_descriptor_proto())
            })
        }
    }
}

impl ::std::kinds::Copy for EStreamDataMessage {
}

#[derive(Clone,PartialEq,Eq,Show)]
pub enum EAudioFormat {
    k_EAudioFormatNone = 0,
    k_EAudioFormat16BitLittleEndian = 1,
    k_EAudioFormatFloat = 2,
}

impl ::protobuf::ProtobufEnum for EAudioFormat {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<EAudioFormat> {
        match value {
            0 => ::std::option::Option::Some(EAudioFormat::k_EAudioFormatNone),
            1 => ::std::option::Option::Some(EAudioFormat::k_EAudioFormat16BitLittleEndian),
            2 => ::std::option::Option::Some(EAudioFormat::k_EAudioFormatFloat),
            _ => ::std::option::Option::None
        }
    }

    fn enum_descriptor_static(_: Option<EAudioFormat>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("EAudioFormat", file_descriptor_proto())
            })
        }
    }
}

impl ::std::kinds::Copy for EAudioFormat {
}

#[derive(Clone,PartialEq,Eq,Show)]
pub enum EVideoFormat {
    k_EVideoFormatNone = 0,
    k_EVideoFormatYV12 = 1,
    k_EVideoFormatAccel = 2,
}

impl ::protobuf::ProtobufEnum for EVideoFormat {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<EVideoFormat> {
        match value {
            0 => ::std::option::Option::Some(EVideoFormat::k_EVideoFormatNone),
            1 => ::std::option::Option::Some(EVideoFormat::k_EVideoFormatYV12),
            2 => ::std::option::Option::Some(EVideoFormat::k_EVideoFormatAccel),
            _ => ::std::option::Option::None
        }
    }

    fn enum_descriptor_static(_: Option<EVideoFormat>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("EVideoFormat", file_descriptor_proto())
            })
        }
    }
}

impl ::std::kinds::Copy for EVideoFormat {
}

#[derive(Clone,PartialEq,Eq,Show)]
pub enum EStreamStatsMessage {
    k_EStreamStatsFrameEvents = 1,
    k_EStreamStatsDebugDump = 2,
}

impl ::protobuf::ProtobufEnum for EStreamStatsMessage {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<EStreamStatsMessage> {
        match value {
            1 => ::std::option::Option::Some(EStreamStatsMessage::k_EStreamStatsFrameEvents),
            2 => ::std::option::Option::Some(EStreamStatsMessage::k_EStreamStatsDebugDump),
            _ => ::std::option::Option::None
        }
    }

    fn enum_descriptor_static(_: Option<EStreamStatsMessage>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("EStreamStatsMessage", file_descriptor_proto())
            })
        }
    }
}

impl ::std::kinds::Copy for EStreamStatsMessage {
}

#[derive(Clone,PartialEq,Eq,Show)]
pub enum EStreamFrameEvent {
    k_EStreamInputEventStart = 0,
    k_EStreamInputEventSend = 1,
    k_EStreamInputEventRecv = 2,
    k_EStreamInputEventQueued = 3,
    k_EStreamInputEventHandled = 4,
    k_EStreamFrameEventStart = 5,
    k_EStreamFrameEventCaptureBegin = 6,
    k_EStreamFrameEventCaptureEnd = 7,
    k_EStreamFrameEventConvertBegin = 8,
    k_EStreamFrameEventConvertEnd = 9,
    k_EStreamFrameEventEncodeBegin = 10,
    k_EStreamFrameEventEncodeEnd = 11,
    k_EStreamFrameEventSend = 12,
    k_EStreamFrameEventRecv = 13,
    k_EStreamFrameEventDecodeBegin = 14,
    k_EStreamFrameEventDecodeEnd = 15,
    k_EStreamFrameEventUploadBegin = 16,
    k_EStreamFrameEventUploadEnd = 17,
    k_EStreamFrameEventComplete = 18,
}

impl ::protobuf::ProtobufEnum for EStreamFrameEvent {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<EStreamFrameEvent> {
        match value {
            0 => ::std::option::Option::Some(EStreamFrameEvent::k_EStreamInputEventStart),
            1 => ::std::option::Option::Some(EStreamFrameEvent::k_EStreamInputEventSend),
            2 => ::std::option::Option::Some(EStreamFrameEvent::k_EStreamInputEventRecv),
            3 => ::std::option::Option::Some(EStreamFrameEvent::k_EStreamInputEventQueued),
            4 => ::std::option::Option::Some(EStreamFrameEvent::k_EStreamInputEventHandled),
            5 => ::std::option::Option::Some(EStreamFrameEvent::k_EStreamFrameEventStart),
            6 => ::std::option::Option::Some(EStreamFrameEvent::k_EStreamFrameEventCaptureBegin),
            7 => ::std::option::Option::Some(EStreamFrameEvent::k_EStreamFrameEventCaptureEnd),
            8 => ::std::option::Option::Some(EStreamFrameEvent::k_EStreamFrameEventConvertBegin),
            9 => ::std::option::Option::Some(EStreamFrameEvent::k_EStreamFrameEventConvertEnd),
            10 => ::std::option::Option::Some(EStreamFrameEvent::k_EStreamFrameEventEncodeBegin),
            11 => ::std::option::Option::Some(EStreamFrameEvent::k_EStreamFrameEventEncodeEnd),
            12 => ::std::option::Option::Some(EStreamFrameEvent::k_EStreamFrameEventSend),
            13 => ::std::option::Option::Some(EStreamFrameEvent::k_EStreamFrameEventRecv),
            14 => ::std::option::Option::Some(EStreamFrameEvent::k_EStreamFrameEventDecodeBegin),
            15 => ::std::option::Option::Some(EStreamFrameEvent::k_EStreamFrameEventDecodeEnd),
            16 => ::std::option::Option::Some(EStreamFrameEvent::k_EStreamFrameEventUploadBegin),
            17 => ::std::option::Option::Some(EStreamFrameEvent::k_EStreamFrameEventUploadEnd),
            18 => ::std::option::Option::Some(EStreamFrameEvent::k_EStreamFrameEventComplete),
            _ => ::std::option::Option::None
        }
    }

    fn enum_descriptor_static(_: Option<EStreamFrameEvent>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("EStreamFrameEvent", file_descriptor_proto())
            })
        }
    }
}

impl ::std::kinds::Copy for EStreamFrameEvent {
}

#[derive(Clone,PartialEq,Eq,Show)]
pub enum EStreamFrameResult {
    k_EStreamFrameResultPending = 0,
    k_EStreamFrameResultDisplayed = 1,
    k_EStreamFrameResultDroppedNetworkSlow = 2,
    k_EStreamFrameResultDroppedNetworkLost = 3,
    k_EStreamFrameResultDroppedDecodeSlow = 4,
    k_EStreamFrameResultDroppedDecodeCorrupt = 5,
    k_EStreamFrameResultDroppedLate = 6,
    k_EStreamFrameResultDroppedReset = 7,
}

impl ::protobuf::ProtobufEnum for EStreamFrameResult {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<EStreamFrameResult> {
        match value {
            0 => ::std::option::Option::Some(EStreamFrameResult::k_EStreamFrameResultPending),
            1 => ::std::option::Option::Some(EStreamFrameResult::k_EStreamFrameResultDisplayed),
            2 => ::std::option::Option::Some(EStreamFrameResult::k_EStreamFrameResultDroppedNetworkSlow),
            3 => ::std::option::Option::Some(EStreamFrameResult::k_EStreamFrameResultDroppedNetworkLost),
            4 => ::std::option::Option::Some(EStreamFrameResult::k_EStreamFrameResultDroppedDecodeSlow),
            5 => ::std::option::Option::Some(EStreamFrameResult::k_EStreamFrameResultDroppedDecodeCorrupt),
            6 => ::std::option::Option::Some(EStreamFrameResult::k_EStreamFrameResultDroppedLate),
            7 => ::std::option::Option::Some(EStreamFrameResult::k_EStreamFrameResultDroppedReset),
            _ => ::std::option::Option::None
        }
    }

    fn enum_descriptor_static(_: Option<EStreamFrameResult>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("EStreamFrameResult", file_descriptor_proto())
            })
        }
    }
}

impl ::std::kinds::Copy for EStreamFrameResult {
}

#[derive(Clone,PartialEq,Eq,Show)]
pub enum EFrameAccumulatedStat {
    k_EFrameStatFPS = 0,
    k_EFrameStatCaptureDurationMS = 1,
    k_EFrameStatConvertDurationMS = 2,
    k_EFrameStatEncodeDurationMS = 3,
    k_EFrameStatSteamDurationMS = 4,
    k_EFrameStatServerDurationMS = 5,
    k_EFrameStatNetworkDurationMS = 6,
    k_EFrameStatDecodeDurationMS = 7,
    k_EFrameStatDisplayDurationMS = 8,
    k_EFrameStatClientDurationMS = 9,
    k_EFrameStatFrameDurationMS = 10,
    k_EFrameStatInputLatencyMS = 11,
    k_EFrameStatGameLatencyMS = 12,
    k_EFrameStatRoundTripLatencyMS = 13,
    k_EFrameStatPingTimeMS = 14,
    k_EFrameStatServerBitrateKbitPerSec = 15,
    k_EFrameStatClientBitrateKbitPerSec = 16,
    k_EFrameStatLinkBandwidthKbitPerSec = 17,
    k_EFrameStatPacketLossPercentage = 18,
}

impl ::protobuf::ProtobufEnum for EFrameAccumulatedStat {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<EFrameAccumulatedStat> {
        match value {
            0 => ::std::option::Option::Some(EFrameAccumulatedStat::k_EFrameStatFPS),
            1 => ::std::option::Option::Some(EFrameAccumulatedStat::k_EFrameStatCaptureDurationMS),
            2 => ::std::option::Option::Some(EFrameAccumulatedStat::k_EFrameStatConvertDurationMS),
            3 => ::std::option::Option::Some(EFrameAccumulatedStat::k_EFrameStatEncodeDurationMS),
            4 => ::std::option::Option::Some(EFrameAccumulatedStat::k_EFrameStatSteamDurationMS),
            5 => ::std::option::Option::Some(EFrameAccumulatedStat::k_EFrameStatServerDurationMS),
            6 => ::std::option::Option::Some(EFrameAccumulatedStat::k_EFrameStatNetworkDurationMS),
            7 => ::std::option::Option::Some(EFrameAccumulatedStat::k_EFrameStatDecodeDurationMS),
            8 => ::std::option::Option::Some(EFrameAccumulatedStat::k_EFrameStatDisplayDurationMS),
            9 => ::std::option::Option::Some(EFrameAccumulatedStat::k_EFrameStatClientDurationMS),
            10 => ::std::option::Option::Some(EFrameAccumulatedStat::k_EFrameStatFrameDurationMS),
            11 => ::std::option::Option::Some(EFrameAccumulatedStat::k_EFrameStatInputLatencyMS),
            12 => ::std::option::Option::Some(EFrameAccumulatedStat::k_EFrameStatGameLatencyMS),
            13 => ::std::option::Option::Some(EFrameAccumulatedStat::k_EFrameStatRoundTripLatencyMS),
            14 => ::std::option::Option::Some(EFrameAccumulatedStat::k_EFrameStatPingTimeMS),
            15 => ::std::option::Option::Some(EFrameAccumulatedStat::k_EFrameStatServerBitrateKbitPerSec),
            16 => ::std::option::Option::Some(EFrameAccumulatedStat::k_EFrameStatClientBitrateKbitPerSec),
            17 => ::std::option::Option::Some(EFrameAccumulatedStat::k_EFrameStatLinkBandwidthKbitPerSec),
            18 => ::std::option::Option::Some(EFrameAccumulatedStat::k_EFrameStatPacketLossPercentage),
            _ => ::std::option::Option::None
        }
    }

    fn enum_descriptor_static(_: Option<EFrameAccumulatedStat>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("EFrameAccumulatedStat", file_descriptor_proto())
            })
        }
    }
}

impl ::std::kinds::Copy for EFrameAccumulatedStat {
}

static file_descriptor_proto_data: &'static [u8] = &[
    0x0a, 0x0c, 0x73, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x22, 0x22,
    0x0a, 0x11, 0x43, 0x44, 0x69, 0x73, 0x63, 0x6f, 0x76, 0x65, 0x72, 0x79, 0x52, 0x65, 0x71, 0x75,
    0x65, 0x73, 0x74, 0x12, 0x0d, 0x0a, 0x05, 0x74, 0x6f, 0x6b, 0x65, 0x6e, 0x18, 0x01, 0x20, 0x01,
    0x28, 0x0c, 0x22, 0x22, 0x0a, 0x12, 0x43, 0x44, 0x69, 0x73, 0x63, 0x6f, 0x76, 0x65, 0x72, 0x79,
    0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x12, 0x0c, 0x0a, 0x04, 0x6e, 0x61, 0x6d, 0x65,
    0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x22, 0x62, 0x0a, 0x19, 0x43, 0x41, 0x75, 0x74, 0x68, 0x65,
    0x6e, 0x74, 0x69, 0x63, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74,
    0x4d, 0x73, 0x67, 0x12, 0x0d, 0x0a, 0x05, 0x74, 0x6f, 0x6b, 0x65, 0x6e, 0x18, 0x01, 0x20, 0x01,
    0x28, 0x0c, 0x12, 0x36, 0x0a, 0x07, 0x76, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x18, 0x02, 0x20,
    0x01, 0x28, 0x0e, 0x32, 0x0f, 0x2e, 0x45, 0x53, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x56, 0x65, 0x72,
    0x73, 0x69, 0x6f, 0x6e, 0x3a, 0x14, 0x6b, 0x5f, 0x45, 0x53, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x56,
    0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x4e, 0x6f, 0x6e, 0x65, 0x22, 0xd4, 0x01, 0x0a, 0x1a, 0x43,
    0x41, 0x75, 0x74, 0x68, 0x65, 0x6e, 0x74, 0x69, 0x63, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x52, 0x65,
    0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x4d, 0x73, 0x67, 0x12, 0x4b, 0x0a, 0x06, 0x72, 0x65, 0x73,
    0x75, 0x6c, 0x74, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0e, 0x32, 0x30, 0x2e, 0x43, 0x41, 0x75, 0x74,
    0x68, 0x65, 0x6e, 0x74, 0x69, 0x63, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x52, 0x65, 0x73, 0x70, 0x6f,
    0x6e, 0x73, 0x65, 0x4d, 0x73, 0x67, 0x2e, 0x41, 0x75, 0x74, 0x68, 0x65, 0x6e, 0x74, 0x69, 0x63,
    0x61, 0x74, 0x69, 0x6f, 0x6e, 0x52, 0x65, 0x73, 0x75, 0x6c, 0x74, 0x3a, 0x09, 0x53, 0x55, 0x43,
    0x43, 0x45, 0x45, 0x44, 0x45, 0x44, 0x12, 0x36, 0x0a, 0x07, 0x76, 0x65, 0x72, 0x73, 0x69, 0x6f,
    0x6e, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0e, 0x32, 0x0f, 0x2e, 0x45, 0x53, 0x74, 0x72, 0x65, 0x61,
    0x6d, 0x56, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x3a, 0x14, 0x6b, 0x5f, 0x45, 0x53, 0x74, 0x72,
    0x65, 0x61, 0x6d, 0x56, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x4e, 0x6f, 0x6e, 0x65, 0x22, 0x31,
    0x0a, 0x14, 0x41, 0x75, 0x74, 0x68, 0x65, 0x6e, 0x74, 0x69, 0x63, 0x61, 0x74, 0x69, 0x6f, 0x6e,
    0x52, 0x65, 0x73, 0x75, 0x6c, 0x74, 0x12, 0x0d, 0x0a, 0x09, 0x53, 0x55, 0x43, 0x43, 0x45, 0x45,
    0x44, 0x45, 0x44, 0x10, 0x00, 0x12, 0x0a, 0x0a, 0x06, 0x46, 0x41, 0x49, 0x4c, 0x45, 0x44, 0x10,
    0x01, 0x22, 0x89, 0x01, 0x0a, 0x10, 0x43, 0x53, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x56, 0x69, 0x64,
    0x65, 0x6f, 0x4d, 0x6f, 0x64, 0x65, 0x12, 0x0d, 0x0a, 0x05, 0x77, 0x69, 0x64, 0x74, 0x68, 0x18,
    0x01, 0x20, 0x02, 0x28, 0x0d, 0x12, 0x0e, 0x0a, 0x06, 0x68, 0x65, 0x69, 0x67, 0x68, 0x74, 0x18,
    0x02, 0x20, 0x02, 0x28, 0x0d, 0x12, 0x14, 0x0a, 0x0c, 0x72, 0x65, 0x66, 0x72, 0x65, 0x73, 0x68,
    0x5f, 0x72, 0x61, 0x74, 0x65, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0d, 0x12, 0x1e, 0x0a, 0x16, 0x72,
    0x65, 0x66, 0x72, 0x65, 0x73, 0x68, 0x5f, 0x72, 0x61, 0x74, 0x65, 0x5f, 0x6e, 0x75, 0x6d, 0x65,
    0x72, 0x61, 0x74, 0x6f, 0x72, 0x18, 0x04, 0x20, 0x01, 0x28, 0x0d, 0x12, 0x20, 0x0a, 0x18, 0x72,
    0x65, 0x66, 0x72, 0x65, 0x73, 0x68, 0x5f, 0x72, 0x61, 0x74, 0x65, 0x5f, 0x64, 0x65, 0x6e, 0x6f,
    0x6d, 0x69, 0x6e, 0x61, 0x74, 0x6f, 0x72, 0x18, 0x05, 0x20, 0x01, 0x28, 0x0d, 0x22, 0xf2, 0x01,
    0x0a, 0x11, 0x43, 0x4e, 0x65, 0x67, 0x6f, 0x74, 0x69, 0x61, 0x74, 0x65, 0x64, 0x43, 0x6f, 0x6e,
    0x66, 0x69, 0x67, 0x12, 0x15, 0x0a, 0x0d, 0x72, 0x65, 0x6c, 0x69, 0x61, 0x62, 0x6c, 0x65, 0x5f,
    0x64, 0x61, 0x74, 0x61, 0x18, 0x01, 0x20, 0x01, 0x28, 0x08, 0x12, 0x49, 0x0a, 0x14, 0x73, 0x65,
    0x6c, 0x65, 0x63, 0x74, 0x65, 0x64, 0x5f, 0x61, 0x75, 0x64, 0x69, 0x6f, 0x5f, 0x63, 0x6f, 0x64,
    0x65, 0x63, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0e, 0x32, 0x12, 0x2e, 0x45, 0x53, 0x74, 0x72, 0x65,
    0x61, 0x6d, 0x41, 0x75, 0x64, 0x69, 0x6f, 0x43, 0x6f, 0x64, 0x65, 0x63, 0x3a, 0x17, 0x6b, 0x5f,
    0x45, 0x53, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x41, 0x75, 0x64, 0x69, 0x6f, 0x43, 0x6f, 0x64, 0x65,
    0x63, 0x4e, 0x6f, 0x6e, 0x65, 0x12, 0x49, 0x0a, 0x14, 0x73, 0x65, 0x6c, 0x65, 0x63, 0x74, 0x65,
    0x64, 0x5f, 0x76, 0x69, 0x64, 0x65, 0x6f, 0x5f, 0x63, 0x6f, 0x64, 0x65, 0x63, 0x18, 0x03, 0x20,
    0x01, 0x28, 0x0e, 0x32, 0x12, 0x2e, 0x45, 0x53, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x56, 0x69, 0x64,
    0x65, 0x6f, 0x43, 0x6f, 0x64, 0x65, 0x63, 0x3a, 0x17, 0x6b, 0x5f, 0x45, 0x53, 0x74, 0x72, 0x65,
    0x61, 0x6d, 0x56, 0x69, 0x64, 0x65, 0x6f, 0x43, 0x6f, 0x64, 0x65, 0x63, 0x4e, 0x6f, 0x6e, 0x65,
    0x12, 0x30, 0x0a, 0x15, 0x61, 0x76, 0x61, 0x69, 0x6c, 0x61, 0x62, 0x6c, 0x65, 0x5f, 0x76, 0x69,
    0x64, 0x65, 0x6f, 0x5f, 0x6d, 0x6f, 0x64, 0x65, 0x73, 0x18, 0x04, 0x20, 0x03, 0x28, 0x0b, 0x32,
    0x11, 0x2e, 0x43, 0x53, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x56, 0x69, 0x64, 0x65, 0x6f, 0x4d, 0x6f,
    0x64, 0x65, 0x22, 0x94, 0x01, 0x0a, 0x13, 0x43, 0x4e, 0x65, 0x67, 0x6f, 0x74, 0x69, 0x61, 0x74,
    0x69, 0x6f, 0x6e, 0x49, 0x6e, 0x69, 0x74, 0x4d, 0x73, 0x67, 0x12, 0x15, 0x0a, 0x0d, 0x72, 0x65,
    0x6c, 0x69, 0x61, 0x62, 0x6c, 0x65, 0x5f, 0x64, 0x61, 0x74, 0x61, 0x18, 0x01, 0x20, 0x01, 0x28,
    0x08, 0x12, 0x32, 0x0a, 0x16, 0x73, 0x75, 0x70, 0x70, 0x6f, 0x72, 0x74, 0x65, 0x64, 0x5f, 0x61,
    0x75, 0x64, 0x69, 0x6f, 0x5f, 0x63, 0x6f, 0x64, 0x65, 0x63, 0x73, 0x18, 0x02, 0x20, 0x03, 0x28,
    0x0e, 0x32, 0x12, 0x2e, 0x45, 0x53, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x41, 0x75, 0x64, 0x69, 0x6f,
    0x43, 0x6f, 0x64, 0x65, 0x63, 0x12, 0x32, 0x0a, 0x16, 0x73, 0x75, 0x70, 0x70, 0x6f, 0x72, 0x74,
    0x65, 0x64, 0x5f, 0x76, 0x69, 0x64, 0x65, 0x6f, 0x5f, 0x63, 0x6f, 0x64, 0x65, 0x63, 0x73, 0x18,
    0x03, 0x20, 0x03, 0x28, 0x0e, 0x32, 0x12, 0x2e, 0x45, 0x53, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x56,
    0x69, 0x64, 0x65, 0x6f, 0x43, 0x6f, 0x64, 0x65, 0x63, 0x22, 0x3e, 0x0a, 0x18, 0x43, 0x4e, 0x65,
    0x67, 0x6f, 0x74, 0x69, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x53, 0x65, 0x74, 0x43, 0x6f, 0x6e, 0x66,
    0x69, 0x67, 0x4d, 0x73, 0x67, 0x12, 0x22, 0x0a, 0x06, 0x63, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x18,
    0x01, 0x20, 0x02, 0x28, 0x0b, 0x32, 0x12, 0x2e, 0x43, 0x4e, 0x65, 0x67, 0x6f, 0x74, 0x69, 0x61,
    0x74, 0x65, 0x64, 0x43, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x22, 0x19, 0x0a, 0x17, 0x43, 0x4e, 0x65,
    0x67, 0x6f, 0x74, 0x69, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x43, 0x6f, 0x6d, 0x70, 0x6c, 0x65, 0x74,
    0x65, 0x4d, 0x73, 0x67, 0x22, 0x9a, 0x01, 0x0a, 0x12, 0x43, 0x53, 0x74, 0x61, 0x72, 0x74, 0x41,
    0x75, 0x64, 0x69, 0x6f, 0x44, 0x61, 0x74, 0x61, 0x4d, 0x73, 0x67, 0x12, 0x0f, 0x0a, 0x07, 0x63,
    0x68, 0x61, 0x6e, 0x6e, 0x65, 0x6c, 0x18, 0x02, 0x20, 0x02, 0x28, 0x0d, 0x12, 0x3a, 0x0a, 0x05,
    0x63, 0x6f, 0x64, 0x65, 0x63, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0e, 0x32, 0x12, 0x2e, 0x45, 0x53,
    0x74, 0x72, 0x65, 0x61, 0x6d, 0x41, 0x75, 0x64, 0x69, 0x6f, 0x43, 0x6f, 0x64, 0x65, 0x63, 0x3a,
    0x17, 0x6b, 0x5f, 0x45, 0x53, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x41, 0x75, 0x64, 0x69, 0x6f, 0x43,
    0x6f, 0x64, 0x65, 0x63, 0x4e, 0x6f, 0x6e, 0x65, 0x12, 0x12, 0x0a, 0x0a, 0x63, 0x6f, 0x64, 0x65,
    0x63, 0x5f, 0x64, 0x61, 0x74, 0x61, 0x18, 0x04, 0x20, 0x01, 0x28, 0x0c, 0x12, 0x11, 0x0a, 0x09,
    0x66, 0x72, 0x65, 0x71, 0x75, 0x65, 0x6e, 0x63, 0x79, 0x18, 0x05, 0x20, 0x01, 0x28, 0x0d, 0x12,
    0x10, 0x0a, 0x08, 0x63, 0x68, 0x61, 0x6e, 0x6e, 0x65, 0x6c, 0x73, 0x18, 0x06, 0x20, 0x01, 0x28,
    0x0d, 0x22, 0x13, 0x0a, 0x11, 0x43, 0x53, 0x74, 0x6f, 0x70, 0x41, 0x75, 0x64, 0x69, 0x6f, 0x44,
    0x61, 0x74, 0x61, 0x4d, 0x73, 0x67, 0x22, 0x94, 0x01, 0x0a, 0x12, 0x43, 0x53, 0x74, 0x61, 0x72,
    0x74, 0x56, 0x69, 0x64, 0x65, 0x6f, 0x44, 0x61, 0x74, 0x61, 0x4d, 0x73, 0x67, 0x12, 0x0f, 0x0a,
    0x07, 0x63, 0x68, 0x61, 0x6e, 0x6e, 0x65, 0x6c, 0x18, 0x01, 0x20, 0x02, 0x28, 0x0d, 0x12, 0x3a,
    0x0a, 0x05, 0x63, 0x6f, 0x64, 0x65, 0x63, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0e, 0x32, 0x12, 0x2e,
    0x45, 0x53, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x56, 0x69, 0x64, 0x65, 0x6f, 0x43, 0x6f, 0x64, 0x65,
    0x63, 0x3a, 0x17, 0x6b, 0x5f, 0x45, 0x53, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x56, 0x69, 0x64, 0x65,
    0x6f, 0x43, 0x6f, 0x64, 0x65, 0x63, 0x4e, 0x6f, 0x6e, 0x65, 0x12, 0x12, 0x0a, 0x0a, 0x63, 0x6f,
    0x64, 0x65, 0x63, 0x5f, 0x64, 0x61, 0x74, 0x61, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0c, 0x12, 0x0d,
    0x0a, 0x05, 0x77, 0x69, 0x64, 0x74, 0x68, 0x18, 0x04, 0x20, 0x01, 0x28, 0x0d, 0x12, 0x0e, 0x0a,
    0x06, 0x68, 0x65, 0x69, 0x67, 0x68, 0x74, 0x18, 0x05, 0x20, 0x01, 0x28, 0x0d, 0x22, 0x13, 0x0a,
    0x11, 0x43, 0x53, 0x74, 0x6f, 0x70, 0x56, 0x69, 0x64, 0x65, 0x6f, 0x44, 0x61, 0x74, 0x61, 0x4d,
    0x73, 0x67, 0x22, 0x39, 0x0a, 0x14, 0x43, 0x49, 0x6e, 0x70, 0x75, 0x74, 0x4c, 0x61, 0x74, 0x65,
    0x6e, 0x63, 0x79, 0x54, 0x65, 0x73, 0x74, 0x4d, 0x73, 0x67, 0x12, 0x12, 0x0a, 0x0a, 0x69, 0x6e,
    0x70, 0x75, 0x74, 0x5f, 0x6d, 0x61, 0x72, 0x6b, 0x18, 0x01, 0x20, 0x02, 0x28, 0x0d, 0x12, 0x0d,
    0x0a, 0x05, 0x63, 0x6f, 0x6c, 0x6f, 0x72, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0d, 0x22, 0x6e, 0x0a,
    0x14, 0x43, 0x49, 0x6e, 0x70, 0x75, 0x74, 0x4d, 0x6f, 0x75, 0x73, 0x65, 0x4d, 0x6f, 0x74, 0x69,
    0x6f, 0x6e, 0x4d, 0x73, 0x67, 0x12, 0x12, 0x0a, 0x0a, 0x69, 0x6e, 0x70, 0x75, 0x74, 0x5f, 0x6d,
    0x61, 0x72, 0x6b, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0d, 0x12, 0x14, 0x0a, 0x0c, 0x78, 0x5f, 0x6e,
    0x6f, 0x72, 0x6d, 0x61, 0x6c, 0x69, 0x7a, 0x65, 0x64, 0x18, 0x02, 0x20, 0x01, 0x28, 0x02, 0x12,
    0x14, 0x0a, 0x0c, 0x79, 0x5f, 0x6e, 0x6f, 0x72, 0x6d, 0x61, 0x6c, 0x69, 0x7a, 0x65, 0x64, 0x18,
    0x03, 0x20, 0x01, 0x28, 0x02, 0x12, 0x0a, 0x0a, 0x02, 0x64, 0x78, 0x18, 0x04, 0x20, 0x01, 0x28,
    0x05, 0x12, 0x0a, 0x0a, 0x02, 0x64, 0x79, 0x18, 0x05, 0x20, 0x01, 0x28, 0x05, 0x22, 0x70, 0x0a,
    0x13, 0x43, 0x49, 0x6e, 0x70, 0x75, 0x74, 0x4d, 0x6f, 0x75, 0x73, 0x65, 0x57, 0x68, 0x65, 0x65,
    0x6c, 0x4d, 0x73, 0x67, 0x12, 0x12, 0x0a, 0x0a, 0x69, 0x6e, 0x70, 0x75, 0x74, 0x5f, 0x6d, 0x61,
    0x72, 0x6b, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0d, 0x12, 0x45, 0x0a, 0x09, 0x64, 0x69, 0x72, 0x65,
    0x63, 0x74, 0x69, 0x6f, 0x6e, 0x18, 0x02, 0x20, 0x02, 0x28, 0x0e, 0x32, 0x1b, 0x2e, 0x45, 0x53,
    0x74, 0x72, 0x65, 0x61, 0x6d, 0x4d, 0x6f, 0x75, 0x73, 0x65, 0x57, 0x68, 0x65, 0x65, 0x6c, 0x44,
    0x69, 0x72, 0x65, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x3a, 0x15, 0x6b, 0x5f, 0x45, 0x53, 0x74, 0x72,
    0x65, 0x61, 0x6d, 0x4d, 0x6f, 0x75, 0x73, 0x65, 0x57, 0x68, 0x65, 0x65, 0x6c, 0x55, 0x70, 0x22,
    0x67, 0x0a, 0x12, 0x43, 0x49, 0x6e, 0x70, 0x75, 0x74, 0x4d, 0x6f, 0x75, 0x73, 0x65, 0x44, 0x6f,
    0x77, 0x6e, 0x4d, 0x73, 0x67, 0x12, 0x12, 0x0a, 0x0a, 0x69, 0x6e, 0x70, 0x75, 0x74, 0x5f, 0x6d,
    0x61, 0x72, 0x6b, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0d, 0x12, 0x3d, 0x0a, 0x06, 0x62, 0x75, 0x74,
    0x74, 0x6f, 0x6e, 0x18, 0x02, 0x20, 0x02, 0x28, 0x0e, 0x32, 0x13, 0x2e, 0x45, 0x53, 0x74, 0x72,
    0x65, 0x61, 0x6d, 0x4d, 0x6f, 0x75, 0x73, 0x65, 0x42, 0x75, 0x74, 0x74, 0x6f, 0x6e, 0x3a, 0x18,
    0x6b, 0x5f, 0x45, 0x53, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x4d, 0x6f, 0x75, 0x73, 0x65, 0x42, 0x75,
    0x74, 0x74, 0x6f, 0x6e, 0x4c, 0x65, 0x66, 0x74, 0x22, 0x65, 0x0a, 0x10, 0x43, 0x49, 0x6e, 0x70,
    0x75, 0x74, 0x4d, 0x6f, 0x75, 0x73, 0x65, 0x55, 0x70, 0x4d, 0x73, 0x67, 0x12, 0x12, 0x0a, 0x0a,
    0x69, 0x6e, 0x70, 0x75, 0x74, 0x5f, 0x6d, 0x61, 0x72, 0x6b, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0d,
    0x12, 0x3d, 0x0a, 0x06, 0x62, 0x75, 0x74, 0x74, 0x6f, 0x6e, 0x18, 0x02, 0x20, 0x02, 0x28, 0x0e,
    0x32, 0x13, 0x2e, 0x45, 0x53, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x4d, 0x6f, 0x75, 0x73, 0x65, 0x42,
    0x75, 0x74, 0x74, 0x6f, 0x6e, 0x3a, 0x18, 0x6b, 0x5f, 0x45, 0x53, 0x74, 0x72, 0x65, 0x61, 0x6d,
    0x4d, 0x6f, 0x75, 0x73, 0x65, 0x42, 0x75, 0x74, 0x74, 0x6f, 0x6e, 0x4c, 0x65, 0x66, 0x74, 0x22,
    0x38, 0x0a, 0x10, 0x43, 0x49, 0x6e, 0x70, 0x75, 0x74, 0x4b, 0x65, 0x79, 0x44, 0x6f, 0x77, 0x6e,
    0x4d, 0x73, 0x67, 0x12, 0x12, 0x0a, 0x0a, 0x69, 0x6e, 0x70, 0x75, 0x74, 0x5f, 0x6d, 0x61, 0x72,
    0x6b, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0d, 0x12, 0x10, 0x0a, 0x08, 0x73, 0x63, 0x61, 0x6e, 0x63,
    0x6f, 0x64, 0x65, 0x18, 0x02, 0x20, 0x02, 0x28, 0x0d, 0x22, 0x36, 0x0a, 0x0e, 0x43, 0x49, 0x6e,
    0x70, 0x75, 0x74, 0x4b, 0x65, 0x79, 0x55, 0x70, 0x4d, 0x73, 0x67, 0x12, 0x12, 0x0a, 0x0a, 0x69,
    0x6e, 0x70, 0x75, 0x74, 0x5f, 0x6d, 0x61, 0x72, 0x6b, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0d, 0x12,
    0x10, 0x0a, 0x08, 0x73, 0x63, 0x61, 0x6e, 0x63, 0x6f, 0x64, 0x65, 0x18, 0x02, 0x20, 0x02, 0x28,
    0x0d, 0x22, 0x66, 0x0a, 0x18, 0x43, 0x49, 0x6e, 0x70, 0x75, 0x74, 0x47, 0x61, 0x6d, 0x65, 0x70,
    0x61, 0x64, 0x41, 0x74, 0x74, 0x61, 0x63, 0x68, 0x65, 0x64, 0x4d, 0x73, 0x67, 0x12, 0x15, 0x0a,
    0x0d, 0x63, 0x6f, 0x6e, 0x74, 0x72, 0x6f, 0x6c, 0x6c, 0x65, 0x72, 0x5f, 0x69, 0x64, 0x18, 0x01,
    0x20, 0x02, 0x28, 0x05, 0x12, 0x17, 0x0a, 0x0f, 0x63, 0x6f, 0x6e, 0x74, 0x72, 0x6f, 0x6c, 0x6c,
    0x65, 0x72, 0x5f, 0x74, 0x79, 0x70, 0x65, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0d, 0x12, 0x1a, 0x0a,
    0x12, 0x63, 0x6f, 0x6e, 0x74, 0x72, 0x6f, 0x6c, 0x6c, 0x65, 0x72, 0x5f, 0x73, 0x75, 0x62, 0x74,
    0x79, 0x70, 0x65, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0d, 0x22, 0x98, 0x01, 0x0a, 0x15, 0x43, 0x49,
    0x6e, 0x70, 0x75, 0x74, 0x47, 0x61, 0x6d, 0x65, 0x70, 0x61, 0x64, 0x45, 0x76, 0x65, 0x6e, 0x74,
    0x4d, 0x73, 0x67, 0x12, 0x12, 0x0a, 0x0a, 0x69, 0x6e, 0x70, 0x75, 0x74, 0x5f, 0x6d, 0x61, 0x72,
    0x6b, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0d, 0x12, 0x15, 0x0a, 0x0d, 0x63, 0x6f, 0x6e, 0x74, 0x72,
    0x6f, 0x6c, 0x6c, 0x65, 0x72, 0x5f, 0x69, 0x64, 0x18, 0x02, 0x20, 0x02, 0x28, 0x05, 0x12, 0x45,
    0x0a, 0x05, 0x69, 0x6e, 0x70, 0x75, 0x74, 0x18, 0x03, 0x20, 0x02, 0x28, 0x0e, 0x32, 0x18, 0x2e,
    0x45, 0x53, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x47, 0x61, 0x6d, 0x65, 0x70, 0x61, 0x64, 0x49, 0x6e,
    0x70, 0x75, 0x74, 0x54, 0x79, 0x70, 0x65, 0x3a, 0x1c, 0x6b, 0x5f, 0x45, 0x53, 0x74, 0x72, 0x65,
    0x61, 0x6d, 0x47, 0x61, 0x6d, 0x65, 0x70, 0x61, 0x64, 0x49, 0x6e, 0x70, 0x75, 0x74, 0x49, 0x6e,
    0x76, 0x61, 0x6c, 0x69, 0x64, 0x12, 0x0d, 0x0a, 0x05, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x18, 0x04,
    0x20, 0x02, 0x28, 0x02, 0x22, 0x31, 0x0a, 0x18, 0x43, 0x49, 0x6e, 0x70, 0x75, 0x74, 0x47, 0x61,
    0x6d, 0x65, 0x70, 0x61, 0x64, 0x44, 0x65, 0x74, 0x61, 0x63, 0x68, 0x65, 0x64, 0x4d, 0x73, 0x67,
    0x12, 0x15, 0x0a, 0x0d, 0x63, 0x6f, 0x6e, 0x74, 0x72, 0x6f, 0x6c, 0x6c, 0x65, 0x72, 0x5f, 0x69,
    0x64, 0x18, 0x01, 0x20, 0x02, 0x28, 0x05, 0x22, 0x5f, 0x0a, 0x11, 0x43, 0x47, 0x61, 0x6d, 0x65,
    0x70, 0x61, 0x64, 0x52, 0x75, 0x6d, 0x62, 0x6c, 0x65, 0x4d, 0x73, 0x67, 0x12, 0x15, 0x0a, 0x0d,
    0x63, 0x6f, 0x6e, 0x74, 0x72, 0x6f, 0x6c, 0x6c, 0x65, 0x72, 0x5f, 0x69, 0x64, 0x18, 0x01, 0x20,
    0x02, 0x28, 0x05, 0x12, 0x18, 0x0a, 0x10, 0x6c, 0x65, 0x66, 0x74, 0x5f, 0x6d, 0x6f, 0x74, 0x6f,
    0x72, 0x5f, 0x73, 0x70, 0x65, 0x65, 0x64, 0x18, 0x02, 0x20, 0x01, 0x28, 0x05, 0x12, 0x19, 0x0a,
    0x11, 0x72, 0x69, 0x67, 0x68, 0x74, 0x5f, 0x6d, 0x6f, 0x74, 0x6f, 0x72, 0x5f, 0x73, 0x70, 0x65,
    0x65, 0x64, 0x18, 0x03, 0x20, 0x01, 0x28, 0x05, 0x22, 0x1c, 0x0a, 0x0c, 0x43, 0x53, 0x65, 0x74,
    0x54, 0x69, 0x74, 0x6c, 0x65, 0x4d, 0x73, 0x67, 0x12, 0x0c, 0x0a, 0x04, 0x74, 0x65, 0x78, 0x74,
    0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x22, 0x3b, 0x0a, 0x0b, 0x43, 0x53, 0x65, 0x74, 0x49, 0x63,
    0x6f, 0x6e, 0x4d, 0x73, 0x67, 0x12, 0x0d, 0x0a, 0x05, 0x77, 0x69, 0x64, 0x74, 0x68, 0x18, 0x01,
    0x20, 0x01, 0x28, 0x05, 0x12, 0x0e, 0x0a, 0x06, 0x68, 0x65, 0x69, 0x67, 0x68, 0x74, 0x18, 0x02,
    0x20, 0x01, 0x28, 0x05, 0x12, 0x0d, 0x0a, 0x05, 0x69, 0x6d, 0x61, 0x67, 0x65, 0x18, 0x03, 0x20,
    0x01, 0x28, 0x0c, 0x22, 0x3c, 0x0a, 0x0e, 0x43, 0x53, 0x68, 0x6f, 0x77, 0x43, 0x75, 0x72, 0x73,
    0x6f, 0x72, 0x4d, 0x73, 0x67, 0x12, 0x14, 0x0a, 0x0c, 0x78, 0x5f, 0x6e, 0x6f, 0x72, 0x6d, 0x61,
    0x6c, 0x69, 0x7a, 0x65, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x02, 0x12, 0x14, 0x0a, 0x0c, 0x79,
    0x5f, 0x6e, 0x6f, 0x72, 0x6d, 0x61, 0x6c, 0x69, 0x7a, 0x65, 0x64, 0x18, 0x02, 0x20, 0x01, 0x28,
    0x02, 0x22, 0x10, 0x0a, 0x0e, 0x43, 0x48, 0x69, 0x64, 0x65, 0x43, 0x75, 0x72, 0x73, 0x6f, 0x72,
    0x4d, 0x73, 0x67, 0x22, 0x22, 0x0a, 0x0d, 0x43, 0x53, 0x65, 0x74, 0x43, 0x75, 0x72, 0x73, 0x6f,
    0x72, 0x4d, 0x73, 0x67, 0x12, 0x11, 0x0a, 0x09, 0x63, 0x75, 0x72, 0x73, 0x6f, 0x72, 0x5f, 0x69,
    0x64, 0x18, 0x01, 0x20, 0x02, 0x28, 0x04, 0x22, 0x27, 0x0a, 0x12, 0x43, 0x47, 0x65, 0x74, 0x43,
    0x75, 0x72, 0x73, 0x6f, 0x72, 0x49, 0x6d, 0x61, 0x67, 0x65, 0x4d, 0x73, 0x67, 0x12, 0x11, 0x0a,
    0x09, 0x63, 0x75, 0x72, 0x73, 0x6f, 0x72, 0x5f, 0x69, 0x64, 0x18, 0x01, 0x20, 0x02, 0x28, 0x04,
    0x22, 0x73, 0x0a, 0x12, 0x43, 0x53, 0x65, 0x74, 0x43, 0x75, 0x72, 0x73, 0x6f, 0x72, 0x49, 0x6d,
    0x61, 0x67, 0x65, 0x4d, 0x73, 0x67, 0x12, 0x11, 0x0a, 0x09, 0x63, 0x75, 0x72, 0x73, 0x6f, 0x72,
    0x5f, 0x69, 0x64, 0x18, 0x01, 0x20, 0x02, 0x28, 0x04, 0x12, 0x0d, 0x0a, 0x05, 0x77, 0x69, 0x64,
    0x74, 0x68, 0x18, 0x02, 0x20, 0x01, 0x28, 0x05, 0x12, 0x0e, 0x0a, 0x06, 0x68, 0x65, 0x69, 0x67,
    0x68, 0x74, 0x18, 0x03, 0x20, 0x01, 0x28, 0x05, 0x12, 0x0d, 0x0a, 0x05, 0x68, 0x6f, 0x74, 0x5f,
    0x78, 0x18, 0x04, 0x20, 0x01, 0x28, 0x05, 0x12, 0x0d, 0x0a, 0x05, 0x68, 0x6f, 0x74, 0x5f, 0x79,
    0x18, 0x05, 0x20, 0x01, 0x28, 0x05, 0x12, 0x0d, 0x0a, 0x05, 0x69, 0x6d, 0x61, 0x67, 0x65, 0x18,
    0x06, 0x20, 0x01, 0x28, 0x0c, 0x22, 0x1e, 0x0a, 0x0e, 0x43, 0x53, 0x79, 0x73, 0x74, 0x65, 0x6d,
    0x49, 0x6e, 0x66, 0x6f, 0x4d, 0x73, 0x67, 0x12, 0x0c, 0x0a, 0x04, 0x69, 0x6e, 0x66, 0x6f, 0x18,
    0x01, 0x20, 0x01, 0x28, 0x09, 0x22, 0x24, 0x0a, 0x14, 0x43, 0x56, 0x69, 0x64, 0x65, 0x6f, 0x44,
    0x65, 0x63, 0x6f, 0x64, 0x65, 0x72, 0x49, 0x6e, 0x66, 0x6f, 0x4d, 0x73, 0x67, 0x12, 0x0c, 0x0a,
    0x04, 0x69, 0x6e, 0x66, 0x6f, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x22, 0x24, 0x0a, 0x14, 0x43,
    0x56, 0x69, 0x64, 0x65, 0x6f, 0x45, 0x6e, 0x63, 0x6f, 0x64, 0x65, 0x72, 0x49, 0x6e, 0x66, 0x6f,
    0x4d, 0x73, 0x67, 0x12, 0x0c, 0x0a, 0x04, 0x69, 0x6e, 0x66, 0x6f, 0x18, 0x01, 0x20, 0x01, 0x28,
    0x09, 0x22, 0x0e, 0x0a, 0x0c, 0x43, 0x51, 0x75, 0x69, 0x74, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73,
    0x74, 0x22, 0x25, 0x0a, 0x10, 0x43, 0x44, 0x65, 0x6c, 0x65, 0x74, 0x65, 0x43, 0x75, 0x72, 0x73,
    0x6f, 0x72, 0x4d, 0x73, 0x67, 0x12, 0x11, 0x0a, 0x09, 0x63, 0x75, 0x72, 0x73, 0x6f, 0x72, 0x5f,
    0x69, 0x64, 0x18, 0x01, 0x20, 0x02, 0x28, 0x04, 0x22, 0x5c, 0x0a, 0x18, 0x43, 0x53, 0x65, 0x74,
    0x51, 0x75, 0x61, 0x6c, 0x69, 0x74, 0x79, 0x50, 0x72, 0x65, 0x66, 0x65, 0x72, 0x65, 0x6e, 0x63,
    0x65, 0x4d, 0x73, 0x67, 0x12, 0x40, 0x0a, 0x07, 0x71, 0x75, 0x61, 0x6c, 0x69, 0x74, 0x79, 0x18,
    0x01, 0x20, 0x02, 0x28, 0x0e, 0x32, 0x19, 0x2e, 0x45, 0x53, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x51,
    0x75, 0x61, 0x6c, 0x69, 0x74, 0x79, 0x50, 0x72, 0x65, 0x66, 0x65, 0x72, 0x65, 0x6e, 0x63, 0x65,
    0x3a, 0x14, 0x6b, 0x5f, 0x45, 0x53, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x51, 0x75, 0x61, 0x6c, 0x69,
    0x74, 0x79, 0x46, 0x61, 0x73, 0x74, 0x22, 0x30, 0x0a, 0x18, 0x43, 0x53, 0x65, 0x74, 0x4d, 0x61,
    0x78, 0x69, 0x6d, 0x75, 0x6d, 0x52, 0x65, 0x73, 0x6f, 0x6c, 0x75, 0x74, 0x69, 0x6f, 0x6e, 0x4d,
    0x73, 0x67, 0x12, 0x09, 0x0a, 0x01, 0x78, 0x18, 0x01, 0x20, 0x02, 0x28, 0x0d, 0x12, 0x09, 0x0a,
    0x01, 0x79, 0x18, 0x02, 0x20, 0x02, 0x28, 0x0d, 0x22, 0x2c, 0x0a, 0x17, 0x43, 0x53, 0x65, 0x74,
    0x4d, 0x61, 0x78, 0x69, 0x6d, 0x75, 0x6d, 0x46, 0x72, 0x61, 0x6d, 0x65, 0x72, 0x61, 0x74, 0x65,
    0x4d, 0x73, 0x67, 0x12, 0x11, 0x0a, 0x09, 0x66, 0x72, 0x61, 0x6d, 0x65, 0x72, 0x61, 0x74, 0x65,
    0x18, 0x01, 0x20, 0x02, 0x28, 0x0d, 0x22, 0x28, 0x0a, 0x15, 0x43, 0x53, 0x65, 0x74, 0x4d, 0x61,
    0x78, 0x69, 0x6d, 0x75, 0x6d, 0x42, 0x69, 0x74, 0x72, 0x61, 0x74, 0x65, 0x4d, 0x73, 0x67, 0x12,
    0x0f, 0x0a, 0x07, 0x62, 0x69, 0x74, 0x72, 0x61, 0x74, 0x65, 0x18, 0x01, 0x20, 0x02, 0x28, 0x05,
    0x22, 0x1d, 0x0a, 0x0a, 0x43, 0x53, 0x65, 0x74, 0x51, 0x6f, 0x53, 0x4d, 0x73, 0x67, 0x12, 0x0f,
    0x0a, 0x07, 0x75, 0x73, 0x65, 0x5f, 0x71, 0x6f, 0x73, 0x18, 0x01, 0x20, 0x02, 0x28, 0x08, 0x22,
    0x78, 0x0a, 0x16, 0x43, 0x53, 0x65, 0x74, 0x54, 0x61, 0x72, 0x67, 0x65, 0x74, 0x46, 0x72, 0x61,
    0x6d, 0x65, 0x72, 0x61, 0x74, 0x65, 0x4d, 0x73, 0x67, 0x12, 0x11, 0x0a, 0x09, 0x66, 0x72, 0x61,
    0x6d, 0x65, 0x72, 0x61, 0x74, 0x65, 0x18, 0x01, 0x20, 0x02, 0x28, 0x0d, 0x12, 0x0f, 0x0a, 0x07,
    0x72, 0x65, 0x61, 0x73, 0x6f, 0x6e, 0x73, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0d, 0x12, 0x1b, 0x0a,
    0x13, 0x66, 0x72, 0x61, 0x6d, 0x65, 0x72, 0x61, 0x74, 0x65, 0x5f, 0x6e, 0x75, 0x6d, 0x65, 0x72,
    0x61, 0x74, 0x6f, 0x72, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0d, 0x12, 0x1d, 0x0a, 0x15, 0x66, 0x72,
    0x61, 0x6d, 0x65, 0x72, 0x61, 0x74, 0x65, 0x5f, 0x64, 0x65, 0x6e, 0x6f, 0x6d, 0x69, 0x6e, 0x61,
    0x74, 0x6f, 0x72, 0x18, 0x04, 0x20, 0x01, 0x28, 0x0d, 0x22, 0x25, 0x0a, 0x12, 0x43, 0x4f, 0x76,
    0x65, 0x72, 0x6c, 0x61, 0x79, 0x45, 0x6e, 0x61, 0x62, 0x6c, 0x65, 0x64, 0x4d, 0x73, 0x67, 0x12,
    0x0f, 0x0a, 0x07, 0x65, 0x6e, 0x61, 0x62, 0x6c, 0x65, 0x64, 0x18, 0x01, 0x20, 0x02, 0x28, 0x08,
    0x22, 0x37, 0x0a, 0x1b, 0x43, 0x49, 0x6e, 0x70, 0x75, 0x74, 0x43, 0x6f, 0x6e, 0x74, 0x72, 0x6f,
    0x6c, 0x6c, 0x65, 0x72, 0x41, 0x74, 0x74, 0x61, 0x63, 0x68, 0x65, 0x64, 0x4d, 0x73, 0x67, 0x12,
    0x18, 0x0a, 0x10, 0x63, 0x6f, 0x6e, 0x74, 0x72, 0x6f, 0x6c, 0x6c, 0x65, 0x72, 0x5f, 0x69, 0x6e,
    0x64, 0x65, 0x78, 0x18, 0x01, 0x20, 0x02, 0x28, 0x05, 0x22, 0xab, 0x01, 0x0a, 0x18, 0x43, 0x49,
    0x6e, 0x70, 0x75, 0x74, 0x43, 0x6f, 0x6e, 0x74, 0x72, 0x6f, 0x6c, 0x6c, 0x65, 0x72, 0x53, 0x74,
    0x61, 0x74, 0x65, 0x4d, 0x73, 0x67, 0x12, 0x12, 0x0a, 0x0a, 0x69, 0x6e, 0x70, 0x75, 0x74, 0x5f,
    0x6d, 0x61, 0x72, 0x6b, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0d, 0x12, 0x18, 0x0a, 0x10, 0x63, 0x6f,
    0x6e, 0x74, 0x72, 0x6f, 0x6c, 0x6c, 0x65, 0x72, 0x5f, 0x69, 0x6e, 0x64, 0x65, 0x78, 0x18, 0x02,
    0x20, 0x02, 0x28, 0x05, 0x12, 0x0f, 0x0a, 0x07, 0x62, 0x75, 0x74, 0x74, 0x6f, 0x6e, 0x73, 0x18,
    0x03, 0x20, 0x01, 0x28, 0x04, 0x12, 0x12, 0x0a, 0x0a, 0x6c, 0x65, 0x66, 0x74, 0x5f, 0x70, 0x61,
    0x64, 0x5f, 0x78, 0x18, 0x04, 0x20, 0x01, 0x28, 0x11, 0x12, 0x12, 0x0a, 0x0a, 0x6c, 0x65, 0x66,
    0x74, 0x5f, 0x70, 0x61, 0x64, 0x5f, 0x79, 0x18, 0x05, 0x20, 0x01, 0x28, 0x11, 0x12, 0x13, 0x0a,
    0x0b, 0x72, 0x69, 0x67, 0x68, 0x74, 0x5f, 0x70, 0x61, 0x64, 0x5f, 0x78, 0x18, 0x06, 0x20, 0x01,
    0x28, 0x11, 0x12, 0x13, 0x0a, 0x0b, 0x72, 0x69, 0x67, 0x68, 0x74, 0x5f, 0x70, 0x61, 0x64, 0x5f,
    0x79, 0x18, 0x07, 0x20, 0x01, 0x28, 0x11, 0x22, 0x5b, 0x0a, 0x23, 0x43, 0x49, 0x6e, 0x70, 0x75,
    0x74, 0x43, 0x6f, 0x6e, 0x74, 0x72, 0x6f, 0x6c, 0x6c, 0x65, 0x72, 0x57, 0x69, 0x72, 0x65, 0x6c,
    0x65, 0x73, 0x73, 0x50, 0x72, 0x65, 0x73, 0x65, 0x6e, 0x63, 0x65, 0x4d, 0x73, 0x67, 0x12, 0x18,
    0x0a, 0x10, 0x77, 0x69, 0x72, 0x65, 0x6c, 0x65, 0x73, 0x73, 0x5f, 0x70, 0x72, 0x65, 0x73, 0x65,
    0x6e, 0x74, 0x18, 0x01, 0x20, 0x01, 0x28, 0x08, 0x12, 0x1a, 0x0a, 0x12, 0x77, 0x69, 0x72, 0x65,
    0x6c, 0x65, 0x73, 0x73, 0x5f, 0x61, 0x76, 0x61, 0x69, 0x6c, 0x61, 0x62, 0x6c, 0x65, 0x18, 0x02,
    0x20, 0x01, 0x28, 0x08, 0x22, 0x60, 0x0a, 0x16, 0x43, 0x54, 0x72, 0x69, 0x67, 0x67, 0x65, 0x72,
    0x48, 0x61, 0x70, 0x74, 0x69, 0x63, 0x50, 0x75, 0x6c, 0x73, 0x65, 0x4d, 0x73, 0x67, 0x12, 0x18,
    0x0a, 0x10, 0x63, 0x6f, 0x6e, 0x74, 0x72, 0x6f, 0x6c, 0x6c, 0x65, 0x72, 0x5f, 0x69, 0x6e, 0x64,
    0x65, 0x78, 0x18, 0x01, 0x20, 0x02, 0x28, 0x05, 0x12, 0x11, 0x0a, 0x09, 0x70, 0x61, 0x64, 0x5f,
    0x69, 0x6e, 0x64, 0x65, 0x78, 0x18, 0x02, 0x20, 0x02, 0x28, 0x05, 0x12, 0x19, 0x0a, 0x11, 0x64,
    0x75, 0x72, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x5f, 0x6d, 0x69, 0x63, 0x72, 0x6f, 0x73, 0x65, 0x63,
    0x18, 0x03, 0x20, 0x02, 0x28, 0x0d, 0x22, 0x41, 0x0a, 0x13, 0x43, 0x53, 0x65, 0x74, 0x4f, 0x76,
    0x65, 0x72, 0x72, 0x69, 0x64, 0x65, 0x4d, 0x6f, 0x64, 0x65, 0x4d, 0x73, 0x67, 0x12, 0x13, 0x0a,
    0x0b, 0x77, 0x69, 0x6e, 0x64, 0x6f, 0x77, 0x5f, 0x74, 0x79, 0x70, 0x65, 0x18, 0x01, 0x20, 0x01,
    0x28, 0x05, 0x12, 0x15, 0x0a, 0x0d, 0x6f, 0x76, 0x65, 0x72, 0x72, 0x69, 0x64, 0x65, 0x5f, 0x6d,
    0x6f, 0x64, 0x65, 0x18, 0x02, 0x20, 0x01, 0x28, 0x09, 0x22, 0x26, 0x0a, 0x10, 0x43, 0x53, 0x65,
    0x74, 0x47, 0x61, 0x6d, 0x6d, 0x61, 0x52, 0x61, 0x6d, 0x70, 0x4d, 0x73, 0x67, 0x12, 0x12, 0x0a,
    0x0a, 0x67, 0x61, 0x6d, 0x6d, 0x61, 0x5f, 0x72, 0x61, 0x6d, 0x70, 0x18, 0x01, 0x20, 0x01, 0x28,
    0x0c, 0x22, 0x37, 0x0a, 0x1b, 0x43, 0x49, 0x6e, 0x70, 0x75, 0x74, 0x43, 0x6f, 0x6e, 0x74, 0x72,
    0x6f, 0x6c, 0x6c, 0x65, 0x72, 0x44, 0x65, 0x74, 0x61, 0x63, 0x68, 0x65, 0x64, 0x4d, 0x73, 0x67,
    0x12, 0x18, 0x0a, 0x10, 0x63, 0x6f, 0x6e, 0x74, 0x72, 0x6f, 0x6c, 0x6c, 0x65, 0x72, 0x5f, 0x69,
    0x6e, 0x64, 0x65, 0x78, 0x18, 0x01, 0x20, 0x02, 0x28, 0x05, 0x22, 0x25, 0x0a, 0x12, 0x43, 0x53,
    0x74, 0x72, 0x65, 0x61, 0x6d, 0x44, 0x61, 0x74, 0x61, 0x4c, 0x6f, 0x73, 0x74, 0x4d, 0x73, 0x67,
    0x12, 0x0f, 0x0a, 0x07, 0x70, 0x61, 0x63, 0x6b, 0x65, 0x74, 0x73, 0x18, 0x01, 0x20, 0x03, 0x28,
    0x0d, 0x22, 0x66, 0x0a, 0x0c, 0x43, 0x41, 0x75, 0x64, 0x69, 0x6f, 0x46, 0x6f, 0x72, 0x6d, 0x61,
    0x74, 0x12, 0x31, 0x0a, 0x06, 0x66, 0x6f, 0x72, 0x6d, 0x61, 0x74, 0x18, 0x01, 0x20, 0x02, 0x28,
    0x0e, 0x32, 0x0d, 0x2e, 0x45, 0x41, 0x75, 0x64, 0x69, 0x6f, 0x46, 0x6f, 0x72, 0x6d, 0x61, 0x74,
    0x3a, 0x12, 0x6b, 0x5f, 0x45, 0x41, 0x75, 0x64, 0x69, 0x6f, 0x46, 0x6f, 0x72, 0x6d, 0x61, 0x74,
    0x4e, 0x6f, 0x6e, 0x65, 0x12, 0x11, 0x0a, 0x09, 0x66, 0x72, 0x65, 0x71, 0x75, 0x65, 0x6e, 0x63,
    0x79, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0d, 0x12, 0x10, 0x0a, 0x08, 0x63, 0x68, 0x61, 0x6e, 0x6e,
    0x65, 0x6c, 0x73, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0d, 0x22, 0x60, 0x0a, 0x0c, 0x43, 0x56, 0x69,
    0x64, 0x65, 0x6f, 0x46, 0x6f, 0x72, 0x6d, 0x61, 0x74, 0x12, 0x31, 0x0a, 0x06, 0x66, 0x6f, 0x72,
    0x6d, 0x61, 0x74, 0x18, 0x01, 0x20, 0x02, 0x28, 0x0e, 0x32, 0x0d, 0x2e, 0x45, 0x56, 0x69, 0x64,
    0x65, 0x6f, 0x46, 0x6f, 0x72, 0x6d, 0x61, 0x74, 0x3a, 0x12, 0x6b, 0x5f, 0x45, 0x56, 0x69, 0x64,
    0x65, 0x6f, 0x46, 0x6f, 0x72, 0x6d, 0x61, 0x74, 0x4e, 0x6f, 0x6e, 0x65, 0x12, 0x0d, 0x0a, 0x05,
    0x77, 0x69, 0x64, 0x74, 0x68, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0d, 0x12, 0x0e, 0x0a, 0x06, 0x68,
    0x65, 0x69, 0x67, 0x68, 0x74, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0d, 0x22, 0x60, 0x0a, 0x0b, 0x43,
    0x46, 0x72, 0x61, 0x6d, 0x65, 0x45, 0x76, 0x65, 0x6e, 0x74, 0x12, 0x3e, 0x0a, 0x08, 0x65, 0x76,
    0x65, 0x6e, 0x74, 0x5f, 0x69, 0x64, 0x18, 0x01, 0x20, 0x02, 0x28, 0x0e, 0x32, 0x12, 0x2e, 0x45,
    0x53, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x46, 0x72, 0x61, 0x6d, 0x65, 0x45, 0x76, 0x65, 0x6e, 0x74,
    0x3a, 0x18, 0x6b, 0x5f, 0x45, 0x53, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x49, 0x6e, 0x70, 0x75, 0x74,
    0x45, 0x76, 0x65, 0x6e, 0x74, 0x53, 0x74, 0x61, 0x72, 0x74, 0x12, 0x11, 0x0a, 0x09, 0x74, 0x69,
    0x6d, 0x65, 0x73, 0x74, 0x61, 0x6d, 0x70, 0x18, 0x02, 0x20, 0x02, 0x28, 0x0d, 0x22, 0xcf, 0x02,
    0x0a, 0x0b, 0x43, 0x46, 0x72, 0x61, 0x6d, 0x65, 0x53, 0x74, 0x61, 0x74, 0x73, 0x12, 0x10, 0x0a,
    0x08, 0x66, 0x72, 0x61, 0x6d, 0x65, 0x5f, 0x69, 0x64, 0x18, 0x01, 0x20, 0x02, 0x28, 0x0d, 0x12,
    0x12, 0x0a, 0x0a, 0x69, 0x6e, 0x70, 0x75, 0x74, 0x5f, 0x6d, 0x61, 0x72, 0x6b, 0x18, 0x02, 0x20,
    0x01, 0x28, 0x0d, 0x12, 0x1c, 0x0a, 0x06, 0x65, 0x76, 0x65, 0x6e, 0x74, 0x73, 0x18, 0x03, 0x20,
    0x03, 0x28, 0x0b, 0x32, 0x0c, 0x2e, 0x43, 0x46, 0x72, 0x61, 0x6d, 0x65, 0x45, 0x76, 0x65, 0x6e,
    0x74, 0x12, 0x40, 0x0a, 0x06, 0x72, 0x65, 0x73, 0x75, 0x6c, 0x74, 0x18, 0x04, 0x20, 0x02, 0x28,
    0x0e, 0x32, 0x13, 0x2e, 0x45, 0x53, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x46, 0x72, 0x61, 0x6d, 0x65,
    0x52, 0x65, 0x73, 0x75, 0x6c, 0x74, 0x3a, 0x1b, 0x6b, 0x5f, 0x45, 0x53, 0x74, 0x72, 0x65, 0x61,
    0x6d, 0x46, 0x72, 0x61, 0x6d, 0x65, 0x52, 0x65, 0x73, 0x75, 0x6c, 0x74, 0x50, 0x65, 0x6e, 0x64,
    0x69, 0x6e, 0x67, 0x12, 0x19, 0x0a, 0x11, 0x66, 0x72, 0x61, 0x6d, 0x65, 0x5f, 0x73, 0x74, 0x61,
    0x72, 0x74, 0x5f, 0x64, 0x65, 0x6c, 0x74, 0x61, 0x18, 0x05, 0x20, 0x01, 0x28, 0x02, 0x12, 0x1b,
    0x0a, 0x13, 0x66, 0x72, 0x61, 0x6d, 0x65, 0x5f, 0x64, 0x69, 0x73, 0x70, 0x6c, 0x61, 0x79, 0x5f,
    0x64, 0x65, 0x6c, 0x74, 0x61, 0x18, 0x06, 0x20, 0x01, 0x28, 0x02, 0x12, 0x11, 0x0a, 0x09, 0x70,
    0x69, 0x6e, 0x67, 0x5f, 0x74, 0x69, 0x6d, 0x65, 0x18, 0x07, 0x20, 0x01, 0x28, 0x02, 0x12, 0x16,
    0x0a, 0x0e, 0x73, 0x65, 0x72, 0x76, 0x65, 0x72, 0x5f, 0x62, 0x69, 0x74, 0x72, 0x61, 0x74, 0x65,
    0x18, 0x08, 0x20, 0x01, 0x28, 0x02, 0x12, 0x16, 0x0a, 0x0e, 0x63, 0x6c, 0x69, 0x65, 0x6e, 0x74,
    0x5f, 0x62, 0x69, 0x74, 0x72, 0x61, 0x74, 0x65, 0x18, 0x09, 0x20, 0x01, 0x28, 0x02, 0x12, 0x16,
    0x0a, 0x0e, 0x6c, 0x69, 0x6e, 0x6b, 0x5f, 0x62, 0x61, 0x6e, 0x64, 0x77, 0x69, 0x64, 0x74, 0x68,
    0x18, 0x0a, 0x20, 0x01, 0x28, 0x02, 0x12, 0x13, 0x0a, 0x0b, 0x70, 0x61, 0x63, 0x6b, 0x65, 0x74,
    0x5f, 0x6c, 0x6f, 0x73, 0x73, 0x18, 0x0b, 0x20, 0x01, 0x28, 0x02, 0x12, 0x12, 0x0a, 0x0a, 0x66,
    0x72, 0x61, 0x6d, 0x65, 0x5f, 0x73, 0x69, 0x7a, 0x65, 0x18, 0x0c, 0x20, 0x01, 0x28, 0x0d, 0x22,
    0x88, 0x01, 0x0a, 0x1a, 0x43, 0x46, 0x72, 0x61, 0x6d, 0x65, 0x53, 0x74, 0x61, 0x74, 0x41, 0x63,
    0x63, 0x75, 0x6d, 0x75, 0x6c, 0x61, 0x74, 0x65, 0x64, 0x56, 0x61, 0x6c, 0x75, 0x65, 0x12, 0x3a,
    0x0a, 0x09, 0x73, 0x74, 0x61, 0x74, 0x5f, 0x74, 0x79, 0x70, 0x65, 0x18, 0x01, 0x20, 0x02, 0x28,
    0x0e, 0x32, 0x16, 0x2e, 0x45, 0x46, 0x72, 0x61, 0x6d, 0x65, 0x41, 0x63, 0x63, 0x75, 0x6d, 0x75,
    0x6c, 0x61, 0x74, 0x65, 0x64, 0x53, 0x74, 0x61, 0x74, 0x3a, 0x0f, 0x6b, 0x5f, 0x45, 0x46, 0x72,
    0x61, 0x6d, 0x65, 0x53, 0x74, 0x61, 0x74, 0x46, 0x50, 0x53, 0x12, 0x0d, 0x0a, 0x05, 0x63, 0x6f,
    0x75, 0x6e, 0x74, 0x18, 0x02, 0x20, 0x02, 0x28, 0x05, 0x12, 0x0f, 0x0a, 0x07, 0x61, 0x76, 0x65,
    0x72, 0x61, 0x67, 0x65, 0x18, 0x03, 0x20, 0x02, 0x28, 0x02, 0x12, 0x0e, 0x0a, 0x06, 0x73, 0x74,
    0x64, 0x64, 0x65, 0x76, 0x18, 0x04, 0x20, 0x01, 0x28, 0x02, 0x22, 0xc1, 0x01, 0x0a, 0x12, 0x43,
    0x46, 0x72, 0x61, 0x6d, 0x65, 0x53, 0x74, 0x61, 0x74, 0x73, 0x4c, 0x69, 0x73, 0x74, 0x4d, 0x73,
    0x67, 0x12, 0x3d, 0x0a, 0x09, 0x64, 0x61, 0x74, 0x61, 0x5f, 0x74, 0x79, 0x70, 0x65, 0x18, 0x01,
    0x20, 0x02, 0x28, 0x0e, 0x32, 0x13, 0x2e, 0x45, 0x53, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x69, 0x6e,
    0x67, 0x44, 0x61, 0x74, 0x61, 0x54, 0x79, 0x70, 0x65, 0x3a, 0x15, 0x6b, 0x5f, 0x45, 0x53, 0x74,
    0x72, 0x65, 0x61, 0x6d, 0x69, 0x6e, 0x67, 0x41, 0x75, 0x64, 0x69, 0x6f, 0x44, 0x61, 0x74, 0x61,
    0x12, 0x1b, 0x0a, 0x05, 0x73, 0x74, 0x61, 0x74, 0x73, 0x18, 0x02, 0x20, 0x03, 0x28, 0x0b, 0x32,
    0x0c, 0x2e, 0x43, 0x46, 0x72, 0x61, 0x6d, 0x65, 0x53, 0x74, 0x61, 0x74, 0x73, 0x12, 0x36, 0x0a,
    0x11, 0x61, 0x63, 0x63, 0x75, 0x6d, 0x75, 0x6c, 0x61, 0x74, 0x65, 0x64, 0x5f, 0x73, 0x74, 0x61,
    0x74, 0x73, 0x18, 0x03, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x1b, 0x2e, 0x43, 0x46, 0x72, 0x61, 0x6d,
    0x65, 0x53, 0x74, 0x61, 0x74, 0x41, 0x63, 0x63, 0x75, 0x6d, 0x75, 0x6c, 0x61, 0x74, 0x65, 0x64,
    0x56, 0x61, 0x6c, 0x75, 0x65, 0x12, 0x17, 0x0a, 0x0f, 0x6c, 0x61, 0x74, 0x65, 0x73, 0x74, 0x5f,
    0x66, 0x72, 0x61, 0x6d, 0x65, 0x5f, 0x69, 0x64, 0x18, 0x04, 0x20, 0x02, 0x28, 0x05, 0x22, 0x23,
    0x0a, 0x0d, 0x43, 0x44, 0x65, 0x62, 0x75, 0x67, 0x44, 0x75, 0x6d, 0x70, 0x4d, 0x73, 0x67, 0x12,
    0x12, 0x0a, 0x0a, 0x73, 0x63, 0x72, 0x65, 0x65, 0x6e, 0x73, 0x68, 0x6f, 0x74, 0x18, 0x01, 0x20,
    0x01, 0x28, 0x0c, 0x2a, 0xb3, 0x01, 0x0a, 0x0e, 0x45, 0x53, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x43,
    0x68, 0x61, 0x6e, 0x6e, 0x65, 0x6c, 0x12, 0x24, 0x0a, 0x17, 0x6b, 0x5f, 0x45, 0x53, 0x74, 0x72,
    0x65, 0x61, 0x6d, 0x43, 0x68, 0x61, 0x6e, 0x6e, 0x65, 0x6c, 0x49, 0x6e, 0x76, 0x61, 0x6c, 0x69,
    0x64, 0x10, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x01, 0x12, 0x1d, 0x0a, 0x19,
    0x6b, 0x5f, 0x45, 0x53, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x43, 0x68, 0x61, 0x6e, 0x6e, 0x65, 0x6c,
    0x44, 0x69, 0x73, 0x63, 0x6f, 0x76, 0x65, 0x72, 0x79, 0x10, 0x00, 0x12, 0x1b, 0x0a, 0x17, 0x6b,
    0x5f, 0x45, 0x53, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x43, 0x68, 0x61, 0x6e, 0x6e, 0x65, 0x6c, 0x43,
    0x6f, 0x6e, 0x74, 0x72, 0x6f, 0x6c, 0x10, 0x01, 0x12, 0x19, 0x0a, 0x15, 0x6b, 0x5f, 0x45, 0x53,
    0x74, 0x72, 0x65, 0x61, 0x6d, 0x43, 0x68, 0x61, 0x6e, 0x6e, 0x65, 0x6c, 0x53, 0x74, 0x61, 0x74,
    0x73, 0x10, 0x02, 0x12, 0x24, 0x0a, 0x20, 0x6b, 0x5f, 0x45, 0x53, 0x74, 0x72, 0x65, 0x61, 0x6d,
    0x43, 0x68, 0x61, 0x6e, 0x6e, 0x65, 0x6c, 0x44, 0x61, 0x74, 0x61, 0x43, 0x68, 0x61, 0x6e, 0x6e,
    0x65, 0x6c, 0x53, 0x74, 0x61, 0x72, 0x74, 0x10, 0x03, 0x2a, 0x58, 0x0a, 0x17, 0x45, 0x53, 0x74,
    0x72, 0x65, 0x61, 0x6d, 0x44, 0x69, 0x73, 0x63, 0x6f, 0x76, 0x65, 0x72, 0x79, 0x4d, 0x65, 0x73,
    0x73, 0x61, 0x67, 0x65, 0x12, 0x1d, 0x0a, 0x19, 0x6b, 0x5f, 0x45, 0x53, 0x74, 0x72, 0x65, 0x61,
    0x6d, 0x44, 0x69, 0x73, 0x63, 0x6f, 0x76, 0x65, 0x72, 0x79, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73,
    0x74, 0x10, 0x01, 0x12, 0x1e, 0x0a, 0x1a, 0x6b, 0x5f, 0x45, 0x53, 0x74, 0x72, 0x65, 0x61, 0x6d,
    0x44, 0x69, 0x73, 0x63, 0x6f, 0x76, 0x65, 0x72, 0x79, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73,
    0x65, 0x10, 0x02, 0x2a, 0xf9, 0x0d, 0x0a, 0x15, 0x45, 0x53, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x43,
    0x6f, 0x6e, 0x74, 0x72, 0x6f, 0x6c, 0x4d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x12, 0x29, 0x0a,
    0x25, 0x6b, 0x5f, 0x45, 0x53, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x43, 0x6f, 0x6e, 0x74, 0x72, 0x6f,
    0x6c, 0x41, 0x75, 0x74, 0x68, 0x65, 0x6e, 0x74, 0x69, 0x63, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x52,
    0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x10, 0x01, 0x12, 0x2a, 0x0a, 0x26, 0x6b, 0x5f, 0x45, 0x53,
    0x74, 0x72, 0x65, 0x61, 0x6d, 0x43, 0x6f, 0x6e, 0x74, 0x72, 0x6f, 0x6c, 0x41, 0x75, 0x74, 0x68,
    0x65, 0x6e, 0x74, 0x69, 0x63, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e,
    0x73, 0x65, 0x10, 0x02, 0x12, 0x23, 0x0a, 0x1f, 0x6b, 0x5f, 0x45, 0x53, 0x74, 0x72, 0x65, 0x61,
    0x6d, 0x43, 0x6f, 0x6e, 0x74, 0x72, 0x6f, 0x6c, 0x4e, 0x65, 0x67, 0x6f, 0x74, 0x69, 0x61, 0x74,
    0x69, 0x6f, 0x6e, 0x49, 0x6e, 0x69, 0x74, 0x10, 0x03, 0x12, 0x28, 0x0a, 0x24, 0x6b, 0x5f, 0x45,
    0x53, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x43, 0x6f, 0x6e, 0x74, 0x72, 0x6f, 0x6c, 0x4e, 0x65, 0x67,
    0x6f, 0x74, 0x69, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x53, 0x65, 0x74, 0x43, 0x6f, 0x6e, 0x66, 0x69,
    0x67, 0x10, 0x04, 0x12, 0x27, 0x0a, 0x23, 0x6b, 0x5f, 0x45, 0x53, 0x74, 0x72, 0x65, 0x61, 0x6d,
    0x43, 0x6f, 0x6e, 0x74, 0x72, 0x6f, 0x6c, 0x4e, 0x65, 0x67, 0x6f, 0x74, 0x69, 0x61, 0x74, 0x69,
    0x6f, 0x6e, 0x43, 0x6f, 0x6d, 0x70, 0x6c, 0x65, 0x74, 0x65, 0x10, 0x05, 0x12, 0x27, 0x0a, 0x23,
    0x6b, 0x5f, 0x45, 0x53, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x43, 0x6f, 0x6e, 0x74, 0x72, 0x6f, 0x6c,
    0x5f, 0x4c, 0x41, 0x53, 0x54, 0x5f, 0x53, 0x45, 0x54, 0x55, 0x50, 0x5f, 0x4d, 0x45, 0x53, 0x53,
    0x41, 0x47, 0x45, 0x10, 0x0f, 0x12, 0x22, 0x0a, 0x1e, 0x6b, 0x5f, 0x45, 0x53, 0x74, 0x72, 0x65,
    0x61, 0x6d, 0x43, 0x6f, 0x6e, 0x74, 0x72, 0x6f, 0x6c, 0x53, 0x74, 0x61, 0x72, 0x74, 0x41, 0x75,
    0x64, 0x69, 0x6f, 0x44, 0x61, 0x74, 0x61, 0x10, 0x32, 0x12, 0x21, 0x0a, 0x1d, 0x6b, 0x5f, 0x45,
    0x53, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x43, 0x6f, 0x6e, 0x74, 0x72, 0x6f, 0x6c, 0x53, 0x74, 0x6f,
    0x70, 0x41, 0x75, 0x64, 0x69, 0x6f, 0x44, 0x61, 0x74, 0x61, 0x10, 0x33, 0x12, 0x22, 0x0a, 0x1e,
    0x6b, 0x5f, 0x45, 0x53, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x43, 0x6f, 0x6e, 0x74, 0x72, 0x6f, 0x6c,
    0x53, 0x74, 0x61, 0x72, 0x74, 0x56, 0x69, 0x64, 0x65, 0x6f, 0x44, 0x61, 0x74, 0x61, 0x10, 0x34,
    0x12, 0x21, 0x0a, 0x1d, 0x6b, 0x5f, 0x45, 0x53, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x43, 0x6f, 0x6e,
    0x74, 0x72, 0x6f, 0x6c, 0x53, 0x74, 0x6f, 0x70, 0x56, 0x69, 0x64, 0x65, 0x6f, 0x44, 0x61, 0x74,
    0x61, 0x10, 0x35, 0x12, 0x24, 0x0a, 0x20, 0x6b, 0x5f, 0x45, 0x53, 0x74, 0x72, 0x65, 0x61, 0x6d,
    0x43, 0x6f, 0x6e, 0x74, 0x72, 0x6f, 0x6c, 0x49, 0x6e, 0x70, 0x75, 0x74, 0x4d, 0x6f, 0x75, 0x73,
    0x65, 0x4d, 0x6f, 0x74, 0x69, 0x6f, 0x6e, 0x10, 0x36, 0x12, 0x23, 0x0a, 0x1f, 0x6b, 0x5f, 0x45,
    0x53, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x43, 0x6f, 0x6e, 0x74, 0x72, 0x6f, 0x6c, 0x49, 0x6e, 0x70,
    0x75, 0x74, 0x4d, 0x6f, 0x75, 0x73, 0x65, 0x57, 0x68, 0x65, 0x65, 0x6c, 0x10, 0x37, 0x12, 0x22,
    0x0a, 0x1e, 0x6b, 0x5f, 0x45, 0x53, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x43, 0x6f, 0x6e, 0x74, 0x72,
    0x6f, 0x6c, 0x49, 0x6e, 0x70, 0x75, 0x74, 0x4d, 0x6f, 0x75, 0x73, 0x65, 0x44, 0x6f, 0x77, 0x6e,
    0x10, 0x38, 0x12, 0x20, 0x0a, 0x1c, 0x6b, 0x5f, 0x45, 0x53, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x43,
    0x6f, 0x6e, 0x74, 0x72, 0x6f, 0x6c, 0x49, 0x6e, 0x70, 0x75, 0x74, 0x4d, 0x6f, 0x75, 0x73, 0x65,
    0x55, 0x70, 0x10, 0x39, 0x12, 0x20, 0x0a, 0x1c, 0x6b, 0x5f, 0x45, 0x53, 0x74, 0x72, 0x65, 0x61,
    0x6d, 0x43, 0x6f, 0x6e, 0x74, 0x72, 0x6f, 0x6c, 0x49, 0x6e, 0x70, 0x75, 0x74, 0x4b, 0x65, 0x79,
    0x44, 0x6f, 0x77, 0x6e, 0x10, 0x3a, 0x12, 0x1e, 0x0a, 0x1a, 0x6b, 0x5f, 0x45, 0x53, 0x74, 0x72,
    0x65, 0x61, 0x6d, 0x43, 0x6f, 0x6e, 0x74, 0x72, 0x6f, 0x6c, 0x49, 0x6e, 0x70, 0x75, 0x74, 0x4b,
    0x65, 0x79, 0x55, 0x70, 0x10, 0x3b, 0x12, 0x28, 0x0a, 0x24, 0x6b, 0x5f, 0x45, 0x53, 0x74, 0x72,
    0x65, 0x61, 0x6d, 0x43, 0x6f, 0x6e, 0x74, 0x72, 0x6f, 0x6c, 0x49, 0x6e, 0x70, 0x75, 0x74, 0x47,
    0x61, 0x6d, 0x65, 0x70, 0x61, 0x64, 0x41, 0x74, 0x74, 0x61, 0x63, 0x68, 0x65, 0x64, 0x10, 0x3c,
    0x12, 0x25, 0x0a, 0x21, 0x6b, 0x5f, 0x45, 0x53, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x43, 0x6f, 0x6e,
    0x74, 0x72, 0x6f, 0x6c, 0x49, 0x6e, 0x70, 0x75, 0x74, 0x47, 0x61, 0x6d, 0x65, 0x70, 0x61, 0x64,
    0x45, 0x76, 0x65, 0x6e, 0x74, 0x10, 0x3d, 0x12, 0x28, 0x0a, 0x24, 0x6b, 0x5f, 0x45, 0x53, 0x74,
    0x72, 0x65, 0x61, 0x6d, 0x43, 0x6f, 0x6e, 0x74, 0x72, 0x6f, 0x6c, 0x49, 0x6e, 0x70, 0x75, 0x74,
    0x47, 0x61, 0x6d, 0x65, 0x70, 0x61, 0x64, 0x44, 0x65, 0x74, 0x61, 0x63, 0x68, 0x65, 0x64, 0x10,
    0x3e, 0x12, 0x1e, 0x0a, 0x1a, 0x6b, 0x5f, 0x45, 0x53, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x43, 0x6f,
    0x6e, 0x74, 0x72, 0x6f, 0x6c, 0x53, 0x68, 0x6f, 0x77, 0x43, 0x75, 0x72, 0x73, 0x6f, 0x72, 0x10,
    0x3f, 0x12, 0x1e, 0x0a, 0x1a, 0x6b, 0x5f, 0x45, 0x53, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x43, 0x6f,
    0x6e, 0x74, 0x72, 0x6f, 0x6c, 0x48, 0x69, 0x64, 0x65, 0x43, 0x75, 0x72, 0x73, 0x6f, 0x72, 0x10,
    0x40, 0x12, 0x1d, 0x0a, 0x19, 0x6b, 0x5f, 0x45, 0x53, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x43, 0x6f,
    0x6e, 0x74, 0x72, 0x6f, 0x6c, 0x53, 0x65, 0x74, 0x43, 0x75, 0x72, 0x73, 0x6f, 0x72, 0x10, 0x41,
    0x12, 0x22, 0x0a, 0x1e, 0x6b, 0x5f, 0x45, 0x53, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x43, 0x6f, 0x6e,
    0x74, 0x72, 0x6f, 0x6c, 0x47, 0x65, 0x74, 0x43, 0x75, 0x72, 0x73, 0x6f, 0x72, 0x49, 0x6d, 0x61,
    0x67, 0x65, 0x10, 0x42, 0x12, 0x22, 0x0a, 0x1e, 0x6b, 0x5f, 0x45, 0x53, 0x74, 0x72, 0x65, 0x61,
    0x6d, 0x43, 0x6f, 0x6e, 0x74, 0x72, 0x6f, 0x6c, 0x53, 0x65, 0x74, 0x43, 0x75, 0x72, 0x73, 0x6f,
    0x72, 0x49, 0x6d, 0x61, 0x67, 0x65, 0x10, 0x43, 0x12, 0x20, 0x0a, 0x1c, 0x6b, 0x5f, 0x45, 0x53,
    0x74, 0x72, 0x65, 0x61, 0x6d, 0x43, 0x6f, 0x6e, 0x74, 0x72, 0x6f, 0x6c, 0x44, 0x65, 0x6c, 0x65,
    0x74, 0x65, 0x43, 0x75, 0x72, 0x73, 0x6f, 0x72, 0x10, 0x44, 0x12, 0x26, 0x0a, 0x22, 0x6b, 0x5f,
    0x45, 0x53, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x43, 0x6f, 0x6e, 0x74, 0x72, 0x6f, 0x6c, 0x53, 0x65,
    0x74, 0x54, 0x61, 0x72, 0x67, 0x65, 0x74, 0x46, 0x72, 0x61, 0x6d, 0x65, 0x72, 0x61, 0x74, 0x65,
    0x10, 0x45, 0x12, 0x24, 0x0a, 0x20, 0x6b, 0x5f, 0x45, 0x53, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x43,
    0x6f, 0x6e, 0x74, 0x72, 0x6f, 0x6c, 0x49, 0x6e, 0x70, 0x75, 0x74, 0x4c, 0x61, 0x74, 0x65, 0x6e,
    0x63, 0x79, 0x54, 0x65, 0x73, 0x74, 0x10, 0x46, 0x12, 0x21, 0x0a, 0x1d, 0x6b, 0x5f, 0x45, 0x53,
    0x74, 0x72, 0x65, 0x61, 0x6d, 0x43, 0x6f, 0x6e, 0x74, 0x72, 0x6f, 0x6c, 0x47, 0x61, 0x6d, 0x65,
    0x70, 0x61, 0x64, 0x52, 0x75, 0x6d, 0x62, 0x6c, 0x65, 0x10, 0x47, 0x12, 0x27, 0x0a, 0x23, 0x6b,
    0x5f, 0x45, 0x53, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x43, 0x6f, 0x6e, 0x74, 0x72, 0x6f, 0x6c, 0x53,
    0x65, 0x74, 0x4d, 0x61, 0x78, 0x69, 0x6d, 0x75, 0x6d, 0x46, 0x72, 0x61, 0x6d, 0x65, 0x72, 0x61,
    0x74, 0x65, 0x10, 0x48, 0x12, 0x25, 0x0a, 0x21, 0x6b, 0x5f, 0x45, 0x53, 0x74, 0x72, 0x65, 0x61,
    0x6d, 0x43, 0x6f, 0x6e, 0x74, 0x72, 0x6f, 0x6c, 0x53, 0x65, 0x74, 0x4d, 0x61, 0x78, 0x69, 0x6d,
    0x75, 0x6d, 0x42, 0x69, 0x74, 0x72, 0x61, 0x74, 0x65, 0x10, 0x49, 0x12, 0x22, 0x0a, 0x1e, 0x6b,
    0x5f, 0x45, 0x53, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x43, 0x6f, 0x6e, 0x74, 0x72, 0x6f, 0x6c, 0x4f,
    0x76, 0x65, 0x72, 0x6c, 0x61, 0x79, 0x45, 0x6e, 0x61, 0x62, 0x6c, 0x65, 0x64, 0x10, 0x4a, 0x12,
    0x2b, 0x0a, 0x27, 0x6b, 0x5f, 0x45, 0x53, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x43, 0x6f, 0x6e, 0x74,
    0x72, 0x6f, 0x6c, 0x49, 0x6e, 0x70, 0x75, 0x74, 0x43, 0x6f, 0x6e, 0x74, 0x72, 0x6f, 0x6c, 0x6c,
    0x65, 0x72, 0x41, 0x74, 0x74, 0x61, 0x63, 0x68, 0x65, 0x64, 0x10, 0x4b, 0x12, 0x28, 0x0a, 0x24,
    0x6b, 0x5f, 0x45, 0x53, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x43, 0x6f, 0x6e, 0x74, 0x72, 0x6f, 0x6c,
    0x49, 0x6e, 0x70, 0x75, 0x74, 0x43, 0x6f, 0x6e, 0x74, 0x72, 0x6f, 0x6c, 0x6c, 0x65, 0x72, 0x53,
    0x74, 0x61, 0x74, 0x65, 0x10, 0x4c, 0x12, 0x26, 0x0a, 0x22, 0x6b, 0x5f, 0x45, 0x53, 0x74, 0x72,
    0x65, 0x61, 0x6d, 0x43, 0x6f, 0x6e, 0x74, 0x72, 0x6f, 0x6c, 0x54, 0x72, 0x69, 0x67, 0x67, 0x65,
    0x72, 0x48, 0x61, 0x70, 0x74, 0x69, 0x63, 0x50, 0x75, 0x6c, 0x73, 0x65, 0x10, 0x4d, 0x12, 0x2b,
    0x0a, 0x27, 0x6b, 0x5f, 0x45, 0x53, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x43, 0x6f, 0x6e, 0x74, 0x72,
    0x6f, 0x6c, 0x49, 0x6e, 0x70, 0x75, 0x74, 0x43, 0x6f, 0x6e, 0x74, 0x72, 0x6f, 0x6c, 0x6c, 0x65,
    0x72, 0x44, 0x65, 0x74, 0x61, 0x63, 0x68, 0x65, 0x64, 0x10, 0x4e, 0x12, 0x1e, 0x0a, 0x1a, 0x6b,
    0x5f, 0x45, 0x53, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x43, 0x6f, 0x6e, 0x74, 0x72, 0x6f, 0x6c, 0x53,
    0x79, 0x73, 0x74, 0x65, 0x6d, 0x49, 0x6e, 0x66, 0x6f, 0x10, 0x4f, 0x12, 0x24, 0x0a, 0x20, 0x6b,
    0x5f, 0x45, 0x53, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x43, 0x6f, 0x6e, 0x74, 0x72, 0x6f, 0x6c, 0x56,
    0x69, 0x64, 0x65, 0x6f, 0x44, 0x65, 0x63, 0x6f, 0x64, 0x65, 0x72, 0x49, 0x6e, 0x66, 0x6f, 0x10,
    0x50, 0x12, 0x1c, 0x0a, 0x18, 0x6b, 0x5f, 0x45, 0x53, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x43, 0x6f,
    0x6e, 0x74, 0x72, 0x6f, 0x6c, 0x53, 0x65, 0x74, 0x54, 0x69, 0x74, 0x6c, 0x65, 0x10, 0x51, 0x12,
    0x1b, 0x0a, 0x17, 0x6b, 0x5f, 0x45, 0x53, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x43, 0x6f, 0x6e, 0x74,
    0x72, 0x6f, 0x6c, 0x53, 0x65, 0x74, 0x49, 0x63, 0x6f, 0x6e, 0x10, 0x52, 0x12, 0x1f, 0x0a, 0x1b,
    0x6b, 0x5f, 0x45, 0x53, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x43, 0x6f, 0x6e, 0x74, 0x72, 0x6f, 0x6c,
    0x51, 0x75, 0x69, 0x74, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x10, 0x53, 0x12, 0x23, 0x0a,
    0x1f, 0x6b, 0x5f, 0x45, 0x53, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x43, 0x6f, 0x6e, 0x74, 0x72, 0x6f,
    0x6c, 0x53, 0x65, 0x74, 0x4f, 0x76, 0x65, 0x72, 0x72, 0x69, 0x64, 0x65, 0x4d, 0x6f, 0x64, 0x65,
    0x10, 0x54, 0x12, 0x28, 0x0a, 0x24, 0x6b, 0x5f, 0x45, 0x53, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x43,
    0x6f, 0x6e, 0x74, 0x72, 0x6f, 0x6c, 0x53, 0x65, 0x74, 0x4d, 0x61, 0x78, 0x69, 0x6d, 0x75, 0x6d,
    0x52, 0x65, 0x73, 0x6f, 0x6c, 0x75, 0x74, 0x69, 0x6f, 0x6e, 0x10, 0x55, 0x12, 0x28, 0x0a, 0x24,
    0x6b, 0x5f, 0x45, 0x53, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x43, 0x6f, 0x6e, 0x74, 0x72, 0x6f, 0x6c,
    0x53, 0x65, 0x74, 0x51, 0x75, 0x61, 0x6c, 0x69, 0x74, 0x79, 0x50, 0x72, 0x65, 0x66, 0x65, 0x72,
    0x65, 0x6e, 0x63, 0x65, 0x10, 0x56, 0x12, 0x1a, 0x0a, 0x16, 0x6b, 0x5f, 0x45, 0x53, 0x74, 0x72,
    0x65, 0x61, 0x6d, 0x43, 0x6f, 0x6e, 0x74, 0x72, 0x6f, 0x6c, 0x53, 0x65, 0x74, 0x51, 0x6f, 0x53,
    0x10, 0x57, 0x12, 0x33, 0x0a, 0x2f, 0x6b, 0x5f, 0x45, 0x53, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x43,
    0x6f, 0x6e, 0x74, 0x72, 0x6f, 0x6c, 0x49, 0x6e, 0x70, 0x75, 0x74, 0x43, 0x6f, 0x6e, 0x74, 0x72,
    0x6f, 0x6c, 0x6c, 0x65, 0x72, 0x57, 0x69, 0x72, 0x65, 0x6c, 0x65, 0x73, 0x73, 0x50, 0x72, 0x65,
    0x73, 0x65, 0x6e, 0x63, 0x65, 0x10, 0x58, 0x12, 0x20, 0x0a, 0x1c, 0x6b, 0x5f, 0x45, 0x53, 0x74,
    0x72, 0x65, 0x61, 0x6d, 0x43, 0x6f, 0x6e, 0x74, 0x72, 0x6f, 0x6c, 0x53, 0x65, 0x74, 0x47, 0x61,
    0x6d, 0x6d, 0x61, 0x52, 0x61, 0x6d, 0x70, 0x10, 0x59, 0x12, 0x24, 0x0a, 0x20, 0x6b, 0x5f, 0x45,
    0x53, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x43, 0x6f, 0x6e, 0x74, 0x72, 0x6f, 0x6c, 0x56, 0x69, 0x64,
    0x65, 0x6f, 0x45, 0x6e, 0x63, 0x6f, 0x64, 0x65, 0x72, 0x49, 0x6e, 0x66, 0x6f, 0x10, 0x5a, 0x2a,
    0x47, 0x0a, 0x0e, 0x45, 0x53, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x56, 0x65, 0x72, 0x73, 0x69, 0x6f,
    0x6e, 0x12, 0x18, 0x0a, 0x14, 0x6b, 0x5f, 0x45, 0x53, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x56, 0x65,
    0x72, 0x73, 0x69, 0x6f, 0x6e, 0x4e, 0x6f, 0x6e, 0x65, 0x10, 0x00, 0x12, 0x1b, 0x0a, 0x17, 0x6b,
    0x5f, 0x45, 0x53, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x56, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x43,
    0x75, 0x72, 0x72, 0x65, 0x6e, 0x74, 0x10, 0x01, 0x2a, 0xc0, 0x01, 0x0a, 0x11, 0x45, 0x53, 0x74,
    0x72, 0x65, 0x61, 0x6d, 0x41, 0x75, 0x64, 0x69, 0x6f, 0x43, 0x6f, 0x64, 0x65, 0x63, 0x12, 0x1b,
    0x0a, 0x17, 0x6b, 0x5f, 0x45, 0x53, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x41, 0x75, 0x64, 0x69, 0x6f,
    0x43, 0x6f, 0x64, 0x65, 0x63, 0x4e, 0x6f, 0x6e, 0x65, 0x10, 0x00, 0x12, 0x1a, 0x0a, 0x16, 0x6b,
    0x5f, 0x45, 0x53, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x41, 0x75, 0x64, 0x69, 0x6f, 0x43, 0x6f, 0x64,
    0x65, 0x63, 0x52, 0x61, 0x77, 0x10, 0x01, 0x12, 0x1d, 0x0a, 0x19, 0x6b, 0x5f, 0x45, 0x53, 0x74,
    0x72, 0x65, 0x61, 0x6d, 0x41, 0x75, 0x64, 0x69, 0x6f, 0x43, 0x6f, 0x64, 0x65, 0x63, 0x56, 0x6f,
    0x72, 0x62, 0x69, 0x73, 0x10, 0x02, 0x12, 0x1b, 0x0a, 0x17, 0x6b, 0x5f, 0x45, 0x53, 0x74, 0x72,
    0x65, 0x61, 0x6d, 0x41, 0x75, 0x64, 0x69, 0x6f, 0x43, 0x6f, 0x64, 0x65, 0x63, 0x4f, 0x70, 0x75,
    0x73, 0x10, 0x03, 0x12, 0x1a, 0x0a, 0x16, 0x6b, 0x5f, 0x45, 0x53, 0x74, 0x72, 0x65, 0x61, 0x6d,
    0x41, 0x75, 0x64, 0x69, 0x6f, 0x43, 0x6f, 0x64, 0x65, 0x63, 0x4d, 0x50, 0x33, 0x10, 0x04, 0x12,
    0x1a, 0x0a, 0x16, 0x6b, 0x5f, 0x45, 0x53, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x41, 0x75, 0x64, 0x69,
    0x6f, 0x43, 0x6f, 0x64, 0x65, 0x63, 0x41, 0x41, 0x43, 0x10, 0x05, 0x2a, 0xfa, 0x01, 0x0a, 0x11,
    0x45, 0x53, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x56, 0x69, 0x64, 0x65, 0x6f, 0x43, 0x6f, 0x64, 0x65,
    0x63, 0x12, 0x1b, 0x0a, 0x17, 0x6b, 0x5f, 0x45, 0x53, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x56, 0x69,
    0x64, 0x65, 0x6f, 0x43, 0x6f, 0x64, 0x65, 0x63, 0x4e, 0x6f, 0x6e, 0x65, 0x10, 0x00, 0x12, 0x1a,
    0x0a, 0x16, 0x6b, 0x5f, 0x45, 0x53, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x56, 0x69, 0x64, 0x65, 0x6f,
    0x43, 0x6f, 0x64, 0x65, 0x63, 0x52, 0x61, 0x77, 0x10, 0x01, 0x12, 0x1a, 0x0a, 0x16, 0x6b, 0x5f,
    0x45, 0x53, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x56, 0x69, 0x64, 0x65, 0x6f, 0x43, 0x6f, 0x64, 0x65,
    0x63, 0x56, 0x50, 0x38, 0x10, 0x02, 0x12, 0x1a, 0x0a, 0x16, 0x6b, 0x5f, 0x45, 0x53, 0x74, 0x72,
    0x65, 0x61, 0x6d, 0x56, 0x69, 0x64, 0x65, 0x6f, 0x43, 0x6f, 0x64, 0x65, 0x63, 0x56, 0x50, 0x39,
    0x10, 0x03, 0x12, 0x1b, 0x0a, 0x17, 0x6b, 0x5f, 0x45, 0x53, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x56,
    0x69, 0x64, 0x65, 0x6f, 0x43, 0x6f, 0x64, 0x65, 0x63, 0x48, 0x32, 0x36, 0x34, 0x10, 0x04, 0x12,
    0x1b, 0x0a, 0x17, 0x6b, 0x5f, 0x45, 0x53, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x56, 0x69, 0x64, 0x65,
    0x6f, 0x43, 0x6f, 0x64, 0x65, 0x63, 0x48, 0x32, 0x36, 0x35, 0x10, 0x05, 0x12, 0x1c, 0x0a, 0x18,
    0x6b, 0x5f, 0x45, 0x53, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x56, 0x69, 0x64, 0x65, 0x6f, 0x43, 0x6f,
    0x64, 0x65, 0x63, 0x4f, 0x52, 0x42, 0x58, 0x31, 0x10, 0x06, 0x12, 0x1c, 0x0a, 0x18, 0x6b, 0x5f,
    0x45, 0x53, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x56, 0x69, 0x64, 0x65, 0x6f, 0x43, 0x6f, 0x64, 0x65,
    0x63, 0x4f, 0x52, 0x42, 0x58, 0x32, 0x10, 0x07, 0x2a, 0x4a, 0x0a, 0x12, 0x45, 0x53, 0x74, 0x72,
    0x65, 0x61, 0x6d, 0x69, 0x6e, 0x67, 0x44, 0x61, 0x74, 0x61, 0x54, 0x79, 0x70, 0x65, 0x12, 0x19,
    0x0a, 0x15, 0x6b, 0x5f, 0x45, 0x53, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x69, 0x6e, 0x67, 0x41, 0x75,
    0x64, 0x69, 0x6f, 0x44, 0x61, 0x74, 0x61, 0x10, 0x00, 0x12, 0x19, 0x0a, 0x15, 0x6b, 0x5f, 0x45,
    0x53, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x69, 0x6e, 0x67, 0x56, 0x69, 0x64, 0x65, 0x6f, 0x44, 0x61,
    0x74, 0x61, 0x10, 0x01, 0x2a, 0xcb, 0x01, 0x0a, 0x12, 0x45, 0x53, 0x74, 0x72, 0x65, 0x61, 0x6d,
    0x4d, 0x6f, 0x75, 0x73, 0x65, 0x42, 0x75, 0x74, 0x74, 0x6f, 0x6e, 0x12, 0x1c, 0x0a, 0x18, 0x6b,
    0x5f, 0x45, 0x53, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x4d, 0x6f, 0x75, 0x73, 0x65, 0x42, 0x75, 0x74,
    0x74, 0x6f, 0x6e, 0x4c, 0x65, 0x66, 0x74, 0x10, 0x01, 0x12, 0x1d, 0x0a, 0x19, 0x6b, 0x5f, 0x45,
    0x53, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x4d, 0x6f, 0x75, 0x73, 0x65, 0x42, 0x75, 0x74, 0x74, 0x6f,
    0x6e, 0x52, 0x69, 0x67, 0x68, 0x74, 0x10, 0x02, 0x12, 0x1e, 0x0a, 0x1a, 0x6b, 0x5f, 0x45, 0x53,
    0x74, 0x72, 0x65, 0x61, 0x6d, 0x4d, 0x6f, 0x75, 0x73, 0x65, 0x42, 0x75, 0x74, 0x74, 0x6f, 0x6e,
    0x4d, 0x69, 0x64, 0x64, 0x6c, 0x65, 0x10, 0x10, 0x12, 0x1a, 0x0a, 0x16, 0x6b, 0x5f, 0x45, 0x53,
    0x74, 0x72, 0x65, 0x61, 0x6d, 0x4d, 0x6f, 0x75, 0x73, 0x65, 0x42, 0x75, 0x74, 0x74, 0x6f, 0x6e,
    0x58, 0x31, 0x10, 0x20, 0x12, 0x1a, 0x0a, 0x16, 0x6b, 0x5f, 0x45, 0x53, 0x74, 0x72, 0x65, 0x61,
    0x6d, 0x4d, 0x6f, 0x75, 0x73, 0x65, 0x42, 0x75, 0x74, 0x74, 0x6f, 0x6e, 0x58, 0x32, 0x10, 0x40,
    0x12, 0x20, 0x0a, 0x1b, 0x6b, 0x5f, 0x45, 0x53, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x4d, 0x6f, 0x75,
    0x73, 0x65, 0x42, 0x75, 0x74, 0x74, 0x6f, 0x6e, 0x55, 0x6e, 0x6b, 0x6e, 0x6f, 0x77, 0x6e, 0x10,
    0x80, 0x20, 0x2a, 0x98, 0x01, 0x0a, 0x1a, 0x45, 0x53, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x4d, 0x6f,
    0x75, 0x73, 0x65, 0x57, 0x68, 0x65, 0x65, 0x6c, 0x44, 0x69, 0x72, 0x65, 0x63, 0x74, 0x69, 0x6f,
    0x6e, 0x12, 0x19, 0x0a, 0x15, 0x6b, 0x5f, 0x45, 0x53, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x4d, 0x6f,
    0x75, 0x73, 0x65, 0x57, 0x68, 0x65, 0x65, 0x6c, 0x55, 0x70, 0x10, 0x78, 0x12, 0x24, 0x0a, 0x17,
    0x6b, 0x5f, 0x45, 0x53, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x4d, 0x6f, 0x75, 0x73, 0x65, 0x57, 0x68,
    0x65, 0x65, 0x6c, 0x44, 0x6f, 0x77, 0x6e, 0x10, 0x88, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
    0xff, 0x01, 0x12, 0x1b, 0x0a, 0x17, 0x6b, 0x5f, 0x45, 0x53, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x4d,
    0x6f, 0x75, 0x73, 0x65, 0x57, 0x68, 0x65, 0x65, 0x6c, 0x4c, 0x65, 0x66, 0x74, 0x10, 0x03, 0x12,
    0x1c, 0x0a, 0x18, 0x6b, 0x5f, 0x45, 0x53, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x4d, 0x6f, 0x75, 0x73,
    0x65, 0x57, 0x68, 0x65, 0x65, 0x6c, 0x52, 0x69, 0x67, 0x68, 0x74, 0x10, 0x04, 0x2a, 0xa7, 0x06,
    0x0a, 0x17, 0x45, 0x53, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x47, 0x61, 0x6d, 0x65, 0x70, 0x61, 0x64,
    0x49, 0x6e, 0x70, 0x75, 0x74, 0x54, 0x79, 0x70, 0x65, 0x12, 0x20, 0x0a, 0x1c, 0x6b, 0x5f, 0x45,
    0x53, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x47, 0x61, 0x6d, 0x65, 0x70, 0x61, 0x64, 0x49, 0x6e, 0x70,
    0x75, 0x74, 0x49, 0x6e, 0x76, 0x61, 0x6c, 0x69, 0x64, 0x10, 0x00, 0x12, 0x1f, 0x0a, 0x1b, 0x6b,
    0x5f, 0x45, 0x53, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x47, 0x61, 0x6d, 0x65, 0x70, 0x61, 0x64, 0x49,
    0x6e, 0x70, 0x75, 0x74, 0x44, 0x50, 0x61, 0x64, 0x55, 0x70, 0x10, 0x01, 0x12, 0x21, 0x0a, 0x1d,
    0x6b, 0x5f, 0x45, 0x53, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x47, 0x61, 0x6d, 0x65, 0x70, 0x61, 0x64,
    0x49, 0x6e, 0x70, 0x75, 0x74, 0x44, 0x50, 0x61, 0x64, 0x44, 0x6f, 0x77, 0x6e, 0x10, 0x02, 0x12,
    0x21, 0x0a, 0x1d, 0x6b, 0x5f, 0x45, 0x53, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x47, 0x61, 0x6d, 0x65,
    0x70, 0x61, 0x64, 0x49, 0x6e, 0x70, 0x75, 0x74, 0x44, 0x50, 0x61, 0x64, 0x4c, 0x65, 0x66, 0x74,
    0x10, 0x04, 0x12, 0x22, 0x0a, 0x1e, 0x6b, 0x5f, 0x45, 0x53, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x47,
    0x61, 0x6d, 0x65, 0x70, 0x61, 0x64, 0x49, 0x6e, 0x70, 0x75, 0x74, 0x44, 0x50, 0x61, 0x64, 0x52,
    0x69, 0x67, 0x68, 0x74, 0x10, 0x08, 0x12, 0x1e, 0x0a, 0x1a, 0x6b, 0x5f, 0x45, 0x53, 0x74, 0x72,
    0x65, 0x61, 0x6d, 0x47, 0x61, 0x6d, 0x65, 0x70, 0x61, 0x64, 0x49, 0x6e, 0x70, 0x75, 0x74, 0x53,
    0x74, 0x61, 0x72, 0x74, 0x10, 0x10, 0x12, 0x1d, 0x0a, 0x19, 0x6b, 0x5f, 0x45, 0x53, 0x74, 0x72,
    0x65, 0x61, 0x6d, 0x47, 0x61, 0x6d, 0x65, 0x70, 0x61, 0x64, 0x49, 0x6e, 0x70, 0x75, 0x74, 0x42,
    0x61, 0x63, 0x6b, 0x10, 0x20, 0x12, 0x22, 0x0a, 0x1e, 0x6b, 0x5f, 0x45, 0x53, 0x74, 0x72, 0x65,
    0x61, 0x6d, 0x47, 0x61, 0x6d, 0x65, 0x70, 0x61, 0x64, 0x49, 0x6e, 0x70, 0x75, 0x74, 0x4c, 0x65,
    0x66, 0x74, 0x54, 0x68, 0x75, 0x6d, 0x62, 0x10, 0x40, 0x12, 0x24, 0x0a, 0x1f, 0x6b, 0x5f, 0x45,
    0x53, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x47, 0x61, 0x6d, 0x65, 0x70, 0x61, 0x64, 0x49, 0x6e, 0x70,
    0x75, 0x74, 0x52, 0x69, 0x67, 0x68, 0x74, 0x54, 0x68, 0x75, 0x6d, 0x62, 0x10, 0x80, 0x01, 0x12,
    0x26, 0x0a, 0x21, 0x6b, 0x5f, 0x45, 0x53, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x47, 0x61, 0x6d, 0x65,
    0x70, 0x61, 0x64, 0x49, 0x6e, 0x70, 0x75, 0x74, 0x4c, 0x65, 0x66, 0x74, 0x53, 0x68, 0x6f, 0x75,
    0x6c, 0x64, 0x65, 0x72, 0x10, 0x80, 0x02, 0x12, 0x27, 0x0a, 0x22, 0x6b, 0x5f, 0x45, 0x53, 0x74,
    0x72, 0x65, 0x61, 0x6d, 0x47, 0x61, 0x6d, 0x65, 0x70, 0x61, 0x64, 0x49, 0x6e, 0x70, 0x75, 0x74,
    0x52, 0x69, 0x67, 0x68, 0x74, 0x53, 0x68, 0x6f, 0x75, 0x6c, 0x64, 0x65, 0x72, 0x10, 0x80, 0x04,
    0x12, 0x1f, 0x0a, 0x1a, 0x6b, 0x5f, 0x45, 0x53, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x47, 0x61, 0x6d,
    0x65, 0x70, 0x61, 0x64, 0x49, 0x6e, 0x70, 0x75, 0x74, 0x47, 0x75, 0x69, 0x64, 0x65, 0x10, 0x80,
    0x08, 0x12, 0x1b, 0x0a, 0x16, 0x6b, 0x5f, 0x45, 0x53, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x47, 0x61,
    0x6d, 0x65, 0x70, 0x61, 0x64, 0x49, 0x6e, 0x70, 0x75, 0x74, 0x41, 0x10, 0x80, 0x20, 0x12, 0x1b,
    0x0a, 0x16, 0x6b, 0x5f, 0x45, 0x53, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x47, 0x61, 0x6d, 0x65, 0x70,
    0x61, 0x64, 0x49, 0x6e, 0x70, 0x75, 0x74, 0x42, 0x10, 0x80, 0x40, 0x12, 0x1c, 0x0a, 0x16, 0x6b,
    0x5f, 0x45, 0x53, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x47, 0x61, 0x6d, 0x65, 0x70, 0x61, 0x64, 0x49,
    0x6e, 0x70, 0x75, 0x74, 0x58, 0x10, 0x80, 0x80, 0x01, 0x12, 0x1c, 0x0a, 0x16, 0x6b, 0x5f, 0x45,
    0x53, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x47, 0x61, 0x6d, 0x65, 0x70, 0x61, 0x64, 0x49, 0x6e, 0x70,
    0x75, 0x74, 0x59, 0x10, 0x80, 0x80, 0x02, 0x12, 0x25, 0x0a, 0x1f, 0x6b, 0x5f, 0x45, 0x53, 0x74,
    0x72, 0x65, 0x61, 0x6d, 0x47, 0x61, 0x6d, 0x65, 0x70, 0x61, 0x64, 0x49, 0x6e, 0x70, 0x75, 0x74,
    0x4c, 0x65, 0x66, 0x74, 0x54, 0x68, 0x75, 0x6d, 0x62, 0x58, 0x10, 0x80, 0x80, 0x04, 0x12, 0x25,
    0x0a, 0x1f, 0x6b, 0x5f, 0x45, 0x53, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x47, 0x61, 0x6d, 0x65, 0x70,
    0x61, 0x64, 0x49, 0x6e, 0x70, 0x75, 0x74, 0x4c, 0x65, 0x66, 0x74, 0x54, 0x68, 0x75, 0x6d, 0x62,
    0x59, 0x10, 0x80, 0x80, 0x08, 0x12, 0x26, 0x0a, 0x20, 0x6b, 0x5f, 0x45, 0x53, 0x74, 0x72, 0x65,
    0x61, 0x6d, 0x47, 0x61, 0x6d, 0x65, 0x70, 0x61, 0x64, 0x49, 0x6e, 0x70, 0x75, 0x74, 0x52, 0x69,
    0x67, 0x68, 0x74, 0x54, 0x68, 0x75, 0x6d, 0x62, 0x58, 0x10, 0x80, 0x80, 0x10, 0x12, 0x26, 0x0a,
    0x20, 0x6b, 0x5f, 0x45, 0x53, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x47, 0x61, 0x6d, 0x65, 0x70, 0x61,
    0x64, 0x49, 0x6e, 0x70, 0x75, 0x74, 0x52, 0x69, 0x67, 0x68, 0x74, 0x54, 0x68, 0x75, 0x6d, 0x62,
    0x59, 0x10, 0x80, 0x80, 0x20, 0x12, 0x26, 0x0a, 0x20, 0x6b, 0x5f, 0x45, 0x53, 0x74, 0x72, 0x65,
    0x61, 0x6d, 0x47, 0x61, 0x6d, 0x65, 0x70, 0x61, 0x64, 0x49, 0x6e, 0x70, 0x75, 0x74, 0x4c, 0x65,
    0x66, 0x74, 0x54, 0x72, 0x69, 0x67, 0x67, 0x65, 0x72, 0x10, 0x80, 0x80, 0x40, 0x12, 0x28, 0x0a,
    0x21, 0x6b, 0x5f, 0x45, 0x53, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x47, 0x61, 0x6d, 0x65, 0x70, 0x61,
    0x64, 0x49, 0x6e, 0x70, 0x75, 0x74, 0x52, 0x69, 0x67, 0x68, 0x74, 0x54, 0x72, 0x69, 0x67, 0x67,
    0x65, 0x72, 0x10, 0x80, 0x80, 0x80, 0x01, 0x2a, 0x71, 0x0a, 0x18, 0x45, 0x53, 0x74, 0x72, 0x65,
    0x61, 0x6d, 0x51, 0x75, 0x61, 0x6c, 0x69, 0x74, 0x79, 0x50, 0x72, 0x65, 0x66, 0x65, 0x72, 0x65,
    0x6e, 0x63, 0x65, 0x12, 0x18, 0x0a, 0x14, 0x6b, 0x5f, 0x45, 0x53, 0x74, 0x72, 0x65, 0x61, 0x6d,
    0x51, 0x75, 0x61, 0x6c, 0x69, 0x74, 0x79, 0x46, 0x61, 0x73, 0x74, 0x10, 0x01, 0x12, 0x1c, 0x0a,
    0x18, 0x6b, 0x5f, 0x45, 0x53, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x51, 0x75, 0x61, 0x6c, 0x69, 0x74,
    0x79, 0x42, 0x61, 0x6c, 0x61, 0x6e, 0x63, 0x65, 0x64, 0x10, 0x02, 0x12, 0x1d, 0x0a, 0x19, 0x6b,
    0x5f, 0x45, 0x53, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x51, 0x75, 0x61, 0x6c, 0x69, 0x74, 0x79, 0x42,
    0x65, 0x61, 0x75, 0x74, 0x69, 0x66, 0x75, 0x6c, 0x10, 0x03, 0x2a, 0x58, 0x0a, 0x0e, 0x45, 0x53,
    0x74, 0x72, 0x65, 0x61, 0x6d, 0x42, 0x69, 0x74, 0x72, 0x61, 0x74, 0x65, 0x12, 0x27, 0x0a, 0x1a,
    0x6b, 0x5f, 0x45, 0x53, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x42, 0x69, 0x74, 0x72, 0x61, 0x74, 0x65,
    0x41, 0x75, 0x74, 0x6f, 0x64, 0x65, 0x74, 0x65, 0x63, 0x74, 0x10, 0xff, 0xff, 0xff, 0xff, 0xff,
    0xff, 0xff, 0xff, 0xff, 0x01, 0x12, 0x1d, 0x0a, 0x19, 0x6b, 0x5f, 0x45, 0x53, 0x74, 0x72, 0x65,
    0x61, 0x6d, 0x42, 0x69, 0x74, 0x72, 0x61, 0x74, 0x65, 0x55, 0x6e, 0x6c, 0x69, 0x6d, 0x69, 0x74,
    0x65, 0x64, 0x10, 0x00, 0x2a, 0x89, 0x02, 0x0a, 0x17, 0x45, 0x53, 0x74, 0x72, 0x65, 0x61, 0x6d,
    0x46, 0x72, 0x61, 0x6d, 0x65, 0x72, 0x61, 0x74, 0x65, 0x4c, 0x69, 0x6d, 0x69, 0x74, 0x65, 0x72,
    0x12, 0x21, 0x0a, 0x1d, 0x6b, 0x5f, 0x45, 0x53, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x46, 0x72, 0x61,
    0x6d, 0x65, 0x72, 0x61, 0x74, 0x65, 0x53, 0x6c, 0x6f, 0x77, 0x43, 0x61, 0x70, 0x74, 0x75, 0x72,
    0x65, 0x10, 0x01, 0x12, 0x21, 0x0a, 0x1d, 0x6b, 0x5f, 0x45, 0x53, 0x74, 0x72, 0x65, 0x61, 0x6d,
    0x46, 0x72, 0x61, 0x6d, 0x65, 0x72, 0x61, 0x74, 0x65, 0x53, 0x6c, 0x6f, 0x77, 0x43, 0x6f, 0x6e,
    0x76, 0x65, 0x72, 0x74, 0x10, 0x02, 0x12, 0x20, 0x0a, 0x1c, 0x6b, 0x5f, 0x45, 0x53, 0x74, 0x72,
    0x65, 0x61, 0x6d, 0x46, 0x72, 0x61, 0x6d, 0x65, 0x72, 0x61, 0x74, 0x65, 0x53, 0x6c, 0x6f, 0x77,
    0x45, 0x6e, 0x63, 0x6f, 0x64, 0x65, 0x10, 0x04, 0x12, 0x21, 0x0a, 0x1d, 0x6b, 0x5f, 0x45, 0x53,
    0x74, 0x72, 0x65, 0x61, 0x6d, 0x46, 0x72, 0x61, 0x6d, 0x65, 0x72, 0x61, 0x74, 0x65, 0x53, 0x6c,
    0x6f, 0x77, 0x4e, 0x65, 0x74, 0x77, 0x6f, 0x72, 0x6b, 0x10, 0x08, 0x12, 0x20, 0x0a, 0x1c, 0x6b,
    0x5f, 0x45, 0x53, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x46, 0x72, 0x61, 0x6d, 0x65, 0x72, 0x61, 0x74,
    0x65, 0x53, 0x6c, 0x6f, 0x77, 0x44, 0x65, 0x63, 0x6f, 0x64, 0x65, 0x10, 0x10, 0x12, 0x1e, 0x0a,
    0x1a, 0x6b, 0x5f, 0x45, 0x53, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x46, 0x72, 0x61, 0x6d, 0x65, 0x72,
    0x61, 0x74, 0x65, 0x53, 0x6c, 0x6f, 0x77, 0x47, 0x61, 0x6d, 0x65, 0x10, 0x20, 0x12, 0x21, 0x0a,
    0x1d, 0x6b, 0x5f, 0x45, 0x53, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x46, 0x72, 0x61, 0x6d, 0x65, 0x72,
    0x61, 0x74, 0x65, 0x53, 0x6c, 0x6f, 0x77, 0x44, 0x69, 0x73, 0x70, 0x6c, 0x61, 0x79, 0x10, 0x40,
    0x2a, 0x44, 0x0a, 0x12, 0x45, 0x53, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x44, 0x61, 0x74, 0x61, 0x4d,
    0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x12, 0x17, 0x0a, 0x13, 0x6b, 0x5f, 0x45, 0x53, 0x74, 0x72,
    0x65, 0x61, 0x6d, 0x44, 0x61, 0x74, 0x61, 0x50, 0x61, 0x63, 0x6b, 0x65, 0x74, 0x10, 0x01, 0x12,
    0x15, 0x0a, 0x11, 0x6b, 0x5f, 0x45, 0x53, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x44, 0x61, 0x74, 0x61,
    0x4c, 0x6f, 0x73, 0x74, 0x10, 0x02, 0x2a, 0x64, 0x0a, 0x0c, 0x45, 0x41, 0x75, 0x64, 0x69, 0x6f,
    0x46, 0x6f, 0x72, 0x6d, 0x61, 0x74, 0x12, 0x16, 0x0a, 0x12, 0x6b, 0x5f, 0x45, 0x41, 0x75, 0x64,
    0x69, 0x6f, 0x46, 0x6f, 0x72, 0x6d, 0x61, 0x74, 0x4e, 0x6f, 0x6e, 0x65, 0x10, 0x00, 0x12, 0x23,
    0x0a, 0x1f, 0x6b, 0x5f, 0x45, 0x41, 0x75, 0x64, 0x69, 0x6f, 0x46, 0x6f, 0x72, 0x6d, 0x61, 0x74,
    0x31, 0x36, 0x42, 0x69, 0x74, 0x4c, 0x69, 0x74, 0x74, 0x6c, 0x65, 0x45, 0x6e, 0x64, 0x69, 0x61,
    0x6e, 0x10, 0x01, 0x12, 0x17, 0x0a, 0x13, 0x6b, 0x5f, 0x45, 0x41, 0x75, 0x64, 0x69, 0x6f, 0x46,
    0x6f, 0x72, 0x6d, 0x61, 0x74, 0x46, 0x6c, 0x6f, 0x61, 0x74, 0x10, 0x02, 0x2a, 0x57, 0x0a, 0x0c,
    0x45, 0x56, 0x69, 0x64, 0x65, 0x6f, 0x46, 0x6f, 0x72, 0x6d, 0x61, 0x74, 0x12, 0x16, 0x0a, 0x12,
    0x6b, 0x5f, 0x45, 0x56, 0x69, 0x64, 0x65, 0x6f, 0x46, 0x6f, 0x72, 0x6d, 0x61, 0x74, 0x4e, 0x6f,
    0x6e, 0x65, 0x10, 0x00, 0x12, 0x16, 0x0a, 0x12, 0x6b, 0x5f, 0x45, 0x56, 0x69, 0x64, 0x65, 0x6f,
    0x46, 0x6f, 0x72, 0x6d, 0x61, 0x74, 0x59, 0x56, 0x31, 0x32, 0x10, 0x01, 0x12, 0x17, 0x0a, 0x13,
    0x6b, 0x5f, 0x45, 0x56, 0x69, 0x64, 0x65, 0x6f, 0x46, 0x6f, 0x72, 0x6d, 0x61, 0x74, 0x41, 0x63,
    0x63, 0x65, 0x6c, 0x10, 0x02, 0x2a, 0x51, 0x0a, 0x13, 0x45, 0x53, 0x74, 0x72, 0x65, 0x61, 0x6d,
    0x53, 0x74, 0x61, 0x74, 0x73, 0x4d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x12, 0x1d, 0x0a, 0x19,
    0x6b, 0x5f, 0x45, 0x53, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x53, 0x74, 0x61, 0x74, 0x73, 0x46, 0x72,
    0x61, 0x6d, 0x65, 0x45, 0x76, 0x65, 0x6e, 0x74, 0x73, 0x10, 0x01, 0x12, 0x1b, 0x0a, 0x17, 0x6b,
    0x5f, 0x45, 0x53, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x53, 0x74, 0x61, 0x74, 0x73, 0x44, 0x65, 0x62,
    0x75, 0x67, 0x44, 0x75, 0x6d, 0x70, 0x10, 0x02, 0x2a, 0x85, 0x05, 0x0a, 0x11, 0x45, 0x53, 0x74,
    0x72, 0x65, 0x61, 0x6d, 0x46, 0x72, 0x61, 0x6d, 0x65, 0x45, 0x76, 0x65, 0x6e, 0x74, 0x12, 0x1c,
    0x0a, 0x18, 0x6b, 0x5f, 0x45, 0x53, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x49, 0x6e, 0x70, 0x75, 0x74,
    0x45, 0x76, 0x65, 0x6e, 0x74, 0x53, 0x74, 0x61, 0x72, 0x74, 0x10, 0x00, 0x12, 0x1b, 0x0a, 0x17,
    0x6b, 0x5f, 0x45, 0x53, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x49, 0x6e, 0x70, 0x75, 0x74, 0x45, 0x76,
    0x65, 0x6e, 0x74, 0x53, 0x65, 0x6e, 0x64, 0x10, 0x01, 0x12, 0x1b, 0x0a, 0x17, 0x6b, 0x5f, 0x45,
    0x53, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x49, 0x6e, 0x70, 0x75, 0x74, 0x45, 0x76, 0x65, 0x6e, 0x74,
    0x52, 0x65, 0x63, 0x76, 0x10, 0x02, 0x12, 0x1d, 0x0a, 0x19, 0x6b, 0x5f, 0x45, 0x53, 0x74, 0x72,
    0x65, 0x61, 0x6d, 0x49, 0x6e, 0x70, 0x75, 0x74, 0x45, 0x76, 0x65, 0x6e, 0x74, 0x51, 0x75, 0x65,
    0x75, 0x65, 0x64, 0x10, 0x03, 0x12, 0x1e, 0x0a, 0x1a, 0x6b, 0x5f, 0x45, 0x53, 0x74, 0x72, 0x65,
    0x61, 0x6d, 0x49, 0x6e, 0x70, 0x75, 0x74, 0x45, 0x76, 0x65, 0x6e, 0x74, 0x48, 0x61, 0x6e, 0x64,
    0x6c, 0x65, 0x64, 0x10, 0x04, 0x12, 0x1c, 0x0a, 0x18, 0x6b, 0x5f, 0x45, 0x53, 0x74, 0x72, 0x65,
    0x61, 0x6d, 0x46, 0x72, 0x61, 0x6d, 0x65, 0x45, 0x76, 0x65, 0x6e, 0x74, 0x53, 0x74, 0x61, 0x72,
    0x74, 0x10, 0x05, 0x12, 0x23, 0x0a, 0x1f, 0x6b, 0x5f, 0x45, 0x53, 0x74, 0x72, 0x65, 0x61, 0x6d,
    0x46, 0x72, 0x61, 0x6d, 0x65, 0x45, 0x76, 0x65, 0x6e, 0x74, 0x43, 0x61, 0x70, 0x74, 0x75, 0x72,
    0x65, 0x42, 0x65, 0x67, 0x69, 0x6e, 0x10, 0x06, 0x12, 0x21, 0x0a, 0x1d, 0x6b, 0x5f, 0x45, 0x53,
    0x74, 0x72, 0x65, 0x61, 0x6d, 0x46, 0x72, 0x61, 0x6d, 0x65, 0x45, 0x76, 0x65, 0x6e, 0x74, 0x43,
    0x61, 0x70, 0x74, 0x75, 0x72, 0x65, 0x45, 0x6e, 0x64, 0x10, 0x07, 0x12, 0x23, 0x0a, 0x1f, 0x6b,
    0x5f, 0x45, 0x53, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x46, 0x72, 0x61, 0x6d, 0x65, 0x45, 0x76, 0x65,
    0x6e, 0x74, 0x43, 0x6f, 0x6e, 0x76, 0x65, 0x72, 0x74, 0x42, 0x65, 0x67, 0x69, 0x6e, 0x10, 0x08,
    0x12, 0x21, 0x0a, 0x1d, 0x6b, 0x5f, 0x45, 0x53, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x46, 0x72, 0x61,
    0x6d, 0x65, 0x45, 0x76, 0x65, 0x6e, 0x74, 0x43, 0x6f, 0x6e, 0x76, 0x65, 0x72, 0x74, 0x45, 0x6e,
    0x64, 0x10, 0x09, 0x12, 0x22, 0x0a, 0x1e, 0x6b, 0x5f, 0x45, 0x53, 0x74, 0x72, 0x65, 0x61, 0x6d,
    0x46, 0x72, 0x61, 0x6d, 0x65, 0x45, 0x76, 0x65, 0x6e, 0x74, 0x45, 0x6e, 0x63, 0x6f, 0x64, 0x65,
    0x42, 0x65, 0x67, 0x69, 0x6e, 0x10, 0x0a, 0x12, 0x20, 0x0a, 0x1c, 0x6b, 0x5f, 0x45, 0x53, 0x74,
    0x72, 0x65, 0x61, 0x6d, 0x46, 0x72, 0x61, 0x6d, 0x65, 0x45, 0x76, 0x65, 0x6e, 0x74, 0x45, 0x6e,
    0x63, 0x6f, 0x64, 0x65, 0x45, 0x6e, 0x64, 0x10, 0x0b, 0x12, 0x1b, 0x0a, 0x17, 0x6b, 0x5f, 0x45,
    0x53, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x46, 0x72, 0x61, 0x6d, 0x65, 0x45, 0x76, 0x65, 0x6e, 0x74,
    0x53, 0x65, 0x6e, 0x64, 0x10, 0x0c, 0x12, 0x1b, 0x0a, 0x17, 0x6b, 0x5f, 0x45, 0x53, 0x74, 0x72,
    0x65, 0x61, 0x6d, 0x46, 0x72, 0x61, 0x6d, 0x65, 0x45, 0x76, 0x65, 0x6e, 0x74, 0x52, 0x65, 0x63,
    0x76, 0x10, 0x0d, 0x12, 0x22, 0x0a, 0x1e, 0x6b, 0x5f, 0x45, 0x53, 0x74, 0x72, 0x65, 0x61, 0x6d,
    0x46, 0x72, 0x61, 0x6d, 0x65, 0x45, 0x76, 0x65, 0x6e, 0x74, 0x44, 0x65, 0x63, 0x6f, 0x64, 0x65,
    0x42, 0x65, 0x67, 0x69, 0x6e, 0x10, 0x0e, 0x12, 0x20, 0x0a, 0x1c, 0x6b, 0x5f, 0x45, 0x53, 0x74,
    0x72, 0x65, 0x61, 0x6d, 0x46, 0x72, 0x61, 0x6d, 0x65, 0x45, 0x76, 0x65, 0x6e, 0x74, 0x44, 0x65,
    0x63, 0x6f, 0x64, 0x65, 0x45, 0x6e, 0x64, 0x10, 0x0f, 0x12, 0x22, 0x0a, 0x1e, 0x6b, 0x5f, 0x45,
    0x53, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x46, 0x72, 0x61, 0x6d, 0x65, 0x45, 0x76, 0x65, 0x6e, 0x74,
    0x55, 0x70, 0x6c, 0x6f, 0x61, 0x64, 0x42, 0x65, 0x67, 0x69, 0x6e, 0x10, 0x10, 0x12, 0x20, 0x0a,
    0x1c, 0x6b, 0x5f, 0x45, 0x53, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x46, 0x72, 0x61, 0x6d, 0x65, 0x45,
    0x76, 0x65, 0x6e, 0x74, 0x55, 0x70, 0x6c, 0x6f, 0x61, 0x64, 0x45, 0x6e, 0x64, 0x10, 0x11, 0x12,
    0x1f, 0x0a, 0x1b, 0x6b, 0x5f, 0x45, 0x53, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x46, 0x72, 0x61, 0x6d,
    0x65, 0x45, 0x76, 0x65, 0x6e, 0x74, 0x43, 0x6f, 0x6d, 0x70, 0x6c, 0x65, 0x74, 0x65, 0x10, 0x12,
    0x2a, 0xd4, 0x02, 0x0a, 0x12, 0x45, 0x53, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x46, 0x72, 0x61, 0x6d,
    0x65, 0x52, 0x65, 0x73, 0x75, 0x6c, 0x74, 0x12, 0x1f, 0x0a, 0x1b, 0x6b, 0x5f, 0x45, 0x53, 0x74,
    0x72, 0x65, 0x61, 0x6d, 0x46, 0x72, 0x61, 0x6d, 0x65, 0x52, 0x65, 0x73, 0x75, 0x6c, 0x74, 0x50,
    0x65, 0x6e, 0x64, 0x69, 0x6e, 0x67, 0x10, 0x00, 0x12, 0x21, 0x0a, 0x1d, 0x6b, 0x5f, 0x45, 0x53,
    0x74, 0x72, 0x65, 0x61, 0x6d, 0x46, 0x72, 0x61, 0x6d, 0x65, 0x52, 0x65, 0x73, 0x75, 0x6c, 0x74,
    0x44, 0x69, 0x73, 0x70, 0x6c, 0x61, 0x79, 0x65, 0x64, 0x10, 0x01, 0x12, 0x2a, 0x0a, 0x26, 0x6b,
    0x5f, 0x45, 0x53, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x46, 0x72, 0x61, 0x6d, 0x65, 0x52, 0x65, 0x73,
    0x75, 0x6c, 0x74, 0x44, 0x72, 0x6f, 0x70, 0x70, 0x65, 0x64, 0x4e, 0x65, 0x74, 0x77, 0x6f, 0x72,
    0x6b, 0x53, 0x6c, 0x6f, 0x77, 0x10, 0x02, 0x12, 0x2a, 0x0a, 0x26, 0x6b, 0x5f, 0x45, 0x53, 0x74,
    0x72, 0x65, 0x61, 0x6d, 0x46, 0x72, 0x61, 0x6d, 0x65, 0x52, 0x65, 0x73, 0x75, 0x6c, 0x74, 0x44,
    0x72, 0x6f, 0x70, 0x70, 0x65, 0x64, 0x4e, 0x65, 0x74, 0x77, 0x6f, 0x72, 0x6b, 0x4c, 0x6f, 0x73,
    0x74, 0x10, 0x03, 0x12, 0x29, 0x0a, 0x25, 0x6b, 0x5f, 0x45, 0x53, 0x74, 0x72, 0x65, 0x61, 0x6d,
    0x46, 0x72, 0x61, 0x6d, 0x65, 0x52, 0x65, 0x73, 0x75, 0x6c, 0x74, 0x44, 0x72, 0x6f, 0x70, 0x70,
    0x65, 0x64, 0x44, 0x65, 0x63, 0x6f, 0x64, 0x65, 0x53, 0x6c, 0x6f, 0x77, 0x10, 0x04, 0x12, 0x2c,
    0x0a, 0x28, 0x6b, 0x5f, 0x45, 0x53, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x46, 0x72, 0x61, 0x6d, 0x65,
    0x52, 0x65, 0x73, 0x75, 0x6c, 0x74, 0x44, 0x72, 0x6f, 0x70, 0x70, 0x65, 0x64, 0x44, 0x65, 0x63,
    0x6f, 0x64, 0x65, 0x43, 0x6f, 0x72, 0x72, 0x75, 0x70, 0x74, 0x10, 0x05, 0x12, 0x23, 0x0a, 0x1f,
    0x6b, 0x5f, 0x45, 0x53, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x46, 0x72, 0x61, 0x6d, 0x65, 0x52, 0x65,
    0x73, 0x75, 0x6c, 0x74, 0x44, 0x72, 0x6f, 0x70, 0x70, 0x65, 0x64, 0x4c, 0x61, 0x74, 0x65, 0x10,
    0x06, 0x12, 0x24, 0x0a, 0x20, 0x6b, 0x5f, 0x45, 0x53, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x46, 0x72,
    0x61, 0x6d, 0x65, 0x52, 0x65, 0x73, 0x75, 0x6c, 0x74, 0x44, 0x72, 0x6f, 0x70, 0x70, 0x65, 0x64,
    0x52, 0x65, 0x73, 0x65, 0x74, 0x10, 0x07, 0x2a, 0xa2, 0x05, 0x0a, 0x15, 0x45, 0x46, 0x72, 0x61,
    0x6d, 0x65, 0x41, 0x63, 0x63, 0x75, 0x6d, 0x75, 0x6c, 0x61, 0x74, 0x65, 0x64, 0x53, 0x74, 0x61,
    0x74, 0x12, 0x13, 0x0a, 0x0f, 0x6b, 0x5f, 0x45, 0x46, 0x72, 0x61, 0x6d, 0x65, 0x53, 0x74, 0x61,
    0x74, 0x46, 0x50, 0x53, 0x10, 0x00, 0x12, 0x21, 0x0a, 0x1d, 0x6b, 0x5f, 0x45, 0x46, 0x72, 0x61,
    0x6d, 0x65, 0x53, 0x74, 0x61, 0x74, 0x43, 0x61, 0x70, 0x74, 0x75, 0x72, 0x65, 0x44, 0x75, 0x72,
    0x61, 0x74, 0x69, 0x6f, 0x6e, 0x4d, 0x53, 0x10, 0x01, 0x12, 0x21, 0x0a, 0x1d, 0x6b, 0x5f, 0x45,
    0x46, 0x72, 0x61, 0x6d, 0x65, 0x53, 0x74, 0x61, 0x74, 0x43, 0x6f, 0x6e, 0x76, 0x65, 0x72, 0x74,
    0x44, 0x75, 0x72, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x4d, 0x53, 0x10, 0x02, 0x12, 0x20, 0x0a, 0x1c,
    0x6b, 0x5f, 0x45, 0x46, 0x72, 0x61, 0x6d, 0x65, 0x53, 0x74, 0x61, 0x74, 0x45, 0x6e, 0x63, 0x6f,
    0x64, 0x65, 0x44, 0x75, 0x72, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x4d, 0x53, 0x10, 0x03, 0x12, 0x1f,
    0x0a, 0x1b, 0x6b, 0x5f, 0x45, 0x46, 0x72, 0x61, 0x6d, 0x65, 0x53, 0x74, 0x61, 0x74, 0x53, 0x74,
    0x65, 0x61, 0x6d, 0x44, 0x75, 0x72, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x4d, 0x53, 0x10, 0x04, 0x12,
    0x20, 0x0a, 0x1c, 0x6b, 0x5f, 0x45, 0x46, 0x72, 0x61, 0x6d, 0x65, 0x53, 0x74, 0x61, 0x74, 0x53,
    0x65, 0x72, 0x76, 0x65, 0x72, 0x44, 0x75, 0x72, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x4d, 0x53, 0x10,
    0x05, 0x12, 0x21, 0x0a, 0x1d, 0x6b, 0x5f, 0x45, 0x46, 0x72, 0x61, 0x6d, 0x65, 0x53, 0x74, 0x61,
    0x74, 0x4e, 0x65, 0x74, 0x77, 0x6f, 0x72, 0x6b, 0x44, 0x75, 0x72, 0x61, 0x74, 0x69, 0x6f, 0x6e,
    0x4d, 0x53, 0x10, 0x06, 0x12, 0x20, 0x0a, 0x1c, 0x6b, 0x5f, 0x45, 0x46, 0x72, 0x61, 0x6d, 0x65,
    0x53, 0x74, 0x61, 0x74, 0x44, 0x65, 0x63, 0x6f, 0x64, 0x65, 0x44, 0x75, 0x72, 0x61, 0x74, 0x69,
    0x6f, 0x6e, 0x4d, 0x53, 0x10, 0x07, 0x12, 0x21, 0x0a, 0x1d, 0x6b, 0x5f, 0x45, 0x46, 0x72, 0x61,
    0x6d, 0x65, 0x53, 0x74, 0x61, 0x74, 0x44, 0x69, 0x73, 0x70, 0x6c, 0x61, 0x79, 0x44, 0x75, 0x72,
    0x61, 0x74, 0x69, 0x6f, 0x6e, 0x4d, 0x53, 0x10, 0x08, 0x12, 0x20, 0x0a, 0x1c, 0x6b, 0x5f, 0x45,
    0x46, 0x72, 0x61, 0x6d, 0x65, 0x53, 0x74, 0x61, 0x74, 0x43, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x44,
    0x75, 0x72, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x4d, 0x53, 0x10, 0x09, 0x12, 0x1f, 0x0a, 0x1b, 0x6b,
    0x5f, 0x45, 0x46, 0x72, 0x61, 0x6d, 0x65, 0x53, 0x74, 0x61, 0x74, 0x46, 0x72, 0x61, 0x6d, 0x65,
    0x44, 0x75, 0x72, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x4d, 0x53, 0x10, 0x0a, 0x12, 0x1e, 0x0a, 0x1a,
    0x6b, 0x5f, 0x45, 0x46, 0x72, 0x61, 0x6d, 0x65, 0x53, 0x74, 0x61, 0x74, 0x49, 0x6e, 0x70, 0x75,
    0x74, 0x4c, 0x61, 0x74, 0x65, 0x6e, 0x63, 0x79, 0x4d, 0x53, 0x10, 0x0b, 0x12, 0x1d, 0x0a, 0x19,
    0x6b, 0x5f, 0x45, 0x46, 0x72, 0x61, 0x6d, 0x65, 0x53, 0x74, 0x61, 0x74, 0x47, 0x61, 0x6d, 0x65,
    0x4c, 0x61, 0x74, 0x65, 0x6e, 0x63, 0x79, 0x4d, 0x53, 0x10, 0x0c, 0x12, 0x22, 0x0a, 0x1e, 0x6b,
    0x5f, 0x45, 0x46, 0x72, 0x61, 0x6d, 0x65, 0x53, 0x74, 0x61, 0x74, 0x52, 0x6f, 0x75, 0x6e, 0x64,
    0x54, 0x72, 0x69, 0x70, 0x4c, 0x61, 0x74, 0x65, 0x6e, 0x63, 0x79, 0x4d, 0x53, 0x10, 0x0d, 0x12,
    0x1a, 0x0a, 0x16, 0x6b, 0x5f, 0x45, 0x46, 0x72, 0x61, 0x6d, 0x65, 0x53, 0x74, 0x61, 0x74, 0x50,
    0x69, 0x6e, 0x67, 0x54, 0x69, 0x6d, 0x65, 0x4d, 0x53, 0x10, 0x0e, 0x12, 0x27, 0x0a, 0x23, 0x6b,
    0x5f, 0x45, 0x46, 0x72, 0x61, 0x6d, 0x65, 0x53, 0x74, 0x61, 0x74, 0x53, 0x65, 0x72, 0x76, 0x65,
    0x72, 0x42, 0x69, 0x74, 0x72, 0x61, 0x74, 0x65, 0x4b, 0x62, 0x69, 0x74, 0x50, 0x65, 0x72, 0x53,
    0x65, 0x63, 0x10, 0x0f, 0x12, 0x27, 0x0a, 0x23, 0x6b, 0x5f, 0x45, 0x46, 0x72, 0x61, 0x6d, 0x65,
    0x53, 0x74, 0x61, 0x74, 0x43, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x42, 0x69, 0x74, 0x72, 0x61, 0x74,
    0x65, 0x4b, 0x62, 0x69, 0x74, 0x50, 0x65, 0x72, 0x53, 0x65, 0x63, 0x10, 0x10, 0x12, 0x27, 0x0a,
    0x23, 0x6b, 0x5f, 0x45, 0x46, 0x72, 0x61, 0x6d, 0x65, 0x53, 0x74, 0x61, 0x74, 0x4c, 0x69, 0x6e,
    0x6b, 0x42, 0x61, 0x6e, 0x64, 0x77, 0x69, 0x64, 0x74, 0x68, 0x4b, 0x62, 0x69, 0x74, 0x50, 0x65,
    0x72, 0x53, 0x65, 0x63, 0x10, 0x11, 0x12, 0x24, 0x0a, 0x20, 0x6b, 0x5f, 0x45, 0x46, 0x72, 0x61,
    0x6d, 0x65, 0x53, 0x74, 0x61, 0x74, 0x50, 0x61, 0x63, 0x6b, 0x65, 0x74, 0x4c, 0x6f, 0x73, 0x73,
    0x50, 0x65, 0x72, 0x63, 0x65, 0x6e, 0x74, 0x61, 0x67, 0x65, 0x10, 0x12, 0x42, 0x05, 0x48, 0x01,
    0x80, 0x01, 0x00, 0x4a, 0xc8, 0x9c, 0x01, 0x0a, 0x07, 0x12, 0x05, 0x00, 0x00, 0xa1, 0x04, 0x01,
    0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12, 0x03, 0x00, 0x00, 0x1c, 0x0a, 0x0b, 0x0a, 0x04, 0x08, 0xe7,
    0x07, 0x00, 0x12, 0x03, 0x00, 0x00, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x08, 0xe7, 0x07, 0x00, 0x02,
    0x12, 0x03, 0x00, 0x07, 0x13, 0x0a, 0x0d, 0x0a, 0x06, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x12,
    0x03, 0x00, 0x07, 0x13, 0x0a, 0x0e, 0x0a, 0x07, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x01, 0x12,
    0x03, 0x00, 0x07, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x08, 0xe7, 0x07, 0x00, 0x03, 0x12, 0x03, 0x00,
    0x16, 0x1b, 0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12, 0x03, 0x01, 0x00, 0x23, 0x0a, 0x0b, 0x0a, 0x04,
    0x08, 0xe7, 0x07, 0x01, 0x12, 0x03, 0x01, 0x00, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x08, 0xe7, 0x07,
    0x01, 0x02, 0x12, 0x03, 0x01, 0x07, 0x1a, 0x0a, 0x0d, 0x0a, 0x06, 0x08, 0xe7, 0x07, 0x01, 0x02,
    0x00, 0x12, 0x03, 0x01, 0x07, 0x1a, 0x0a, 0x0e, 0x0a, 0x07, 0x08, 0xe7, 0x07, 0x01, 0x02, 0x00,
    0x01, 0x12, 0x03, 0x01, 0x07, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x08, 0xe7, 0x07, 0x01, 0x03, 0x12,
    0x03, 0x01, 0x1d, 0x22, 0x0a, 0x0a, 0x0a, 0x02, 0x05, 0x00, 0x12, 0x04, 0x03, 0x00, 0x09, 0x01,
    0x0a, 0x0a, 0x0a, 0x03, 0x05, 0x00, 0x01, 0x12, 0x03, 0x03, 0x05, 0x13, 0x0a, 0x0b, 0x0a, 0x04,
    0x05, 0x00, 0x02, 0x00, 0x12, 0x03, 0x04, 0x08, 0x25, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02,
    0x00, 0x01, 0x12, 0x03, 0x04, 0x08, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x00, 0x02,
    0x12, 0x03, 0x04, 0x22, 0x24, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x01, 0x12, 0x03, 0x05,
    0x08, 0x26, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x05, 0x08, 0x21,
    0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x01, 0x02, 0x12, 0x03, 0x05, 0x24, 0x25, 0x0a, 0x0b,
    0x0a, 0x04, 0x05, 0x00, 0x02, 0x02, 0x12, 0x03, 0x06, 0x08, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x05,
    0x00, 0x02, 0x02, 0x01, 0x12, 0x03, 0x06, 0x08, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02,
    0x02, 0x02, 0x12, 0x03, 0x06, 0x22, 0x23, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x03, 0x12,
    0x03, 0x07, 0x08, 0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x03, 0x01, 0x12, 0x03, 0x07,
    0x08, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x03, 0x02, 0x12, 0x03, 0x07, 0x20, 0x21,
    0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x04, 0x12, 0x03, 0x08, 0x08, 0x2d, 0x0a, 0x0c, 0x0a,
    0x05, 0x05, 0x00, 0x02, 0x04, 0x01, 0x12, 0x03, 0x08, 0x08, 0x28, 0x0a, 0x0c, 0x0a, 0x05, 0x05,
    0x00, 0x02, 0x04, 0x02, 0x12, 0x03, 0x08, 0x2b, 0x2c, 0x0a, 0x0a, 0x0a, 0x02, 0x05, 0x01, 0x12,
    0x04, 0x0b, 0x00, 0x0e, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x05, 0x01, 0x01, 0x12, 0x03, 0x0b, 0x05,
    0x1c, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x01, 0x02, 0x00, 0x12, 0x03, 0x0c, 0x08, 0x26, 0x0a, 0x0c,
    0x0a, 0x05, 0x05, 0x01, 0x02, 0x00, 0x01, 0x12, 0x03, 0x0c, 0x08, 0x21, 0x0a, 0x0c, 0x0a, 0x05,
    0x05, 0x01, 0x02, 0x00, 0x02, 0x12, 0x03, 0x0c, 0x24, 0x25, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x01,
    0x02, 0x01, 0x12, 0x03, 0x0d, 0x08, 0x27, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x01, 0x02, 0x01, 0x01,
    0x12, 0x03, 0x0d, 0x08, 0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x01, 0x02, 0x01, 0x02, 0x12, 0x03,
    0x0d, 0x25, 0x26, 0x0a, 0x0a, 0x0a, 0x02, 0x05, 0x02, 0x12, 0x04, 0x10, 0x00, 0x40, 0x01, 0x0a,
    0x0a, 0x0a, 0x03, 0x05, 0x02, 0x01, 0x12, 0x03, 0x10, 0x05, 0x1a, 0x0a, 0x0b, 0x0a, 0x04, 0x05,
    0x02, 0x02, 0x00, 0x12, 0x03, 0x11, 0x08, 0x32, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x02, 0x02, 0x00,
    0x01, 0x12, 0x03, 0x11, 0x08, 0x2d, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x02, 0x02, 0x00, 0x02, 0x12,
    0x03, 0x11, 0x30, 0x31, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x02, 0x02, 0x01, 0x12, 0x03, 0x12, 0x08,
    0x33, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x02, 0x02, 0x01, 0x01, 0x12, 0x03, 0x12, 0x08, 0x2e, 0x0a,
    0x0c, 0x0a, 0x05, 0x05, 0x02, 0x02, 0x01, 0x02, 0x12, 0x03, 0x12, 0x31, 0x32, 0x0a, 0x0b, 0x0a,
    0x04, 0x05, 0x02, 0x02, 0x02, 0x12, 0x03, 0x13, 0x08, 0x2c, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x02,
    0x02, 0x02, 0x01, 0x12, 0x03, 0x13, 0x08, 0x27, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x02, 0x02, 0x02,
    0x02, 0x12, 0x03, 0x13, 0x2a, 0x2b, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x02, 0x02, 0x03, 0x12, 0x03,
    0x14, 0x08, 0x31, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x02, 0x02, 0x03, 0x01, 0x12, 0x03, 0x14, 0x08,
    0x2c, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x02, 0x02, 0x03, 0x02, 0x12, 0x03, 0x14, 0x2f, 0x30, 0x0a,
    0x0b, 0x0a, 0x04, 0x05, 0x02, 0x02, 0x04, 0x12, 0x03, 0x15, 0x08, 0x30, 0x0a, 0x0c, 0x0a, 0x05,
    0x05, 0x02, 0x02, 0x04, 0x01, 0x12, 0x03, 0x15, 0x08, 0x2b, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x02,
    0x02, 0x04, 0x02, 0x12, 0x03, 0x15, 0x2e, 0x2f, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x02, 0x02, 0x05,
    0x12, 0x03, 0x16, 0x08, 0x31, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x02, 0x02, 0x05, 0x01, 0x12, 0x03,
    0x16, 0x08, 0x2b, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x02, 0x02, 0x05, 0x02, 0x12, 0x03, 0x16, 0x2e,
    0x30, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x02, 0x02, 0x06, 0x12, 0x03, 0x17, 0x08, 0x2c, 0x0a, 0x0c,
    0x0a, 0x05, 0x05, 0x02, 0x02, 0x06, 0x01, 0x12, 0x03, 0x17, 0x08, 0x26, 0x0a, 0x0c, 0x0a, 0x05,
    0x05, 0x02, 0x02, 0x06, 0x02, 0x12, 0x03, 0x17, 0x29, 0x2b, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x02,
    0x02, 0x07, 0x12, 0x03, 0x18, 0x08, 0x2b, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x02, 0x02, 0x07, 0x01,
    0x12, 0x03, 0x18, 0x08, 0x25, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x02, 0x02, 0x07, 0x02, 0x12, 0x03,
    0x18, 0x28, 0x2a, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x02, 0x02, 0x08, 0x12, 0x03, 0x19, 0x08, 0x2c,
    0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x02, 0x02, 0x08, 0x01, 0x12, 0x03, 0x19, 0x08, 0x26, 0x0a, 0x0c,
    0x0a, 0x05, 0x05, 0x02, 0x02, 0x08, 0x02, 0x12, 0x03, 0x19, 0x29, 0x2b, 0x0a, 0x0b, 0x0a, 0x04,
    0x05, 0x02, 0x02, 0x09, 0x12, 0x03, 0x1a, 0x08, 0x2b, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x02, 0x02,
    0x09, 0x01, 0x12, 0x03, 0x1a, 0x08, 0x25, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x02, 0x02, 0x09, 0x02,
    0x12, 0x03, 0x1a, 0x28, 0x2a, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x02, 0x02, 0x0a, 0x12, 0x03, 0x1b,
    0x08, 0x2e, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x02, 0x02, 0x0a, 0x01, 0x12, 0x03, 0x1b, 0x08, 0x28,
    0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x02, 0x02, 0x0a, 0x02, 0x12, 0x03, 0x1b, 0x2b, 0x2d, 0x0a, 0x0b,
    0x0a, 0x04, 0x05, 0x02, 0x02, 0x0b, 0x12, 0x03, 0x1c, 0x08, 0x2d, 0x0a, 0x0c, 0x0a, 0x05, 0x05,
    0x02, 0x02, 0x0b, 0x01, 0x12, 0x03, 0x1c, 0x08, 0x27, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x02, 0x02,
    0x0b, 0x02, 0x12, 0x03, 0x1c, 0x2a, 0x2c, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x02, 0x02, 0x0c, 0x12,
    0x03, 0x1d, 0x08, 0x2c, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x02, 0x02, 0x0c, 0x01, 0x12, 0x03, 0x1d,
    0x08, 0x26, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x02, 0x02, 0x0c, 0x02, 0x12, 0x03, 0x1d, 0x29, 0x2b,
    0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x02, 0x02, 0x0d, 0x12, 0x03, 0x1e, 0x08, 0x2a, 0x0a, 0x0c, 0x0a,
    0x05, 0x05, 0x02, 0x02, 0x0d, 0x01, 0x12, 0x03, 0x1e, 0x08, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x05,
    0x02, 0x02, 0x0d, 0x02, 0x12, 0x03, 0x1e, 0x27, 0x29, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x02, 0x02,
    0x0e, 0x12, 0x03, 0x1f, 0x08, 0x2a, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x02, 0x02, 0x0e, 0x01, 0x12,
    0x03, 0x1f, 0x08, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x02, 0x02, 0x0e, 0x02, 0x12, 0x03, 0x1f,
    0x27, 0x29, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x02, 0x02, 0x0f, 0x12, 0x03, 0x20, 0x08, 0x28, 0x0a,
    0x0c, 0x0a, 0x05, 0x05, 0x02, 0x02, 0x0f, 0x01, 0x12, 0x03, 0x20, 0x08, 0x22, 0x0a, 0x0c, 0x0a,
    0x05, 0x05, 0x02, 0x02, 0x0f, 0x02, 0x12, 0x03, 0x20, 0x25, 0x27, 0x0a, 0x0b, 0x0a, 0x04, 0x05,
    0x02, 0x02, 0x10, 0x12, 0x03, 0x21, 0x08, 0x32, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x02, 0x02, 0x10,
    0x01, 0x12, 0x03, 0x21, 0x08, 0x2c, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x02, 0x02, 0x10, 0x02, 0x12,
    0x03, 0x21, 0x2f, 0x31, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x02, 0x02, 0x11, 0x12, 0x03, 0x22, 0x08,
    0x2f, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x02, 0x02, 0x11, 0x01, 0x12, 0x03, 0x22, 0x08, 0x29, 0x0a,
    0x0c, 0x0a, 0x05, 0x05, 0x02, 0x02, 0x11, 0x02, 0x12, 0x03, 0x22, 0x2c, 0x2e, 0x0a, 0x0b, 0x0a,
    0x04, 0x05, 0x02, 0x02, 0x12, 0x12, 0x03, 0x23, 0x08, 0x32, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x02,
    0x02, 0x12, 0x01, 0x12, 0x03, 0x23, 0x08, 0x2c, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x02, 0x02, 0x12,
    0x02, 0x12, 0x03, 0x23, 0x2f, 0x31, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x02, 0x02, 0x13, 0x12, 0x03,
    0x24, 0x08, 0x28, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x02, 0x02, 0x13, 0x01, 0x12, 0x03, 0x24, 0x08,
    0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x02, 0x02, 0x13, 0x02, 0x12, 0x03, 0x24, 0x25, 0x27, 0x0a,
    0x0b, 0x0a, 0x04, 0x05, 0x02, 0x02, 0x14, 0x12, 0x03, 0x25, 0x08, 0x28, 0x0a, 0x0c, 0x0a, 0x05,
    0x05, 0x02, 0x02, 0x14, 0x01, 0x12, 0x03, 0x25, 0x08, 0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x02,
    0x02, 0x14, 0x02, 0x12, 0x03, 0x25, 0x25, 0x27, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x02, 0x02, 0x15,
    0x12, 0x03, 0x26, 0x08, 0x27, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x02, 0x02, 0x15, 0x01, 0x12, 0x03,
    0x26, 0x08, 0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x02, 0x02, 0x15, 0x02, 0x12, 0x03, 0x26, 0x24,
    0x26, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x02, 0x02, 0x16, 0x12, 0x03, 0x27, 0x08, 0x2c, 0x0a, 0x0c,
    0x0a, 0x05, 0x05, 0x02, 0x02, 0x16, 0x01, 0x12, 0x03, 0x27, 0x08, 0x26, 0x0a, 0x0c, 0x0a, 0x05,
    0x05, 0x02, 0x02, 0x16, 0x02, 0x12, 0x03, 0x27, 0x29, 0x2b, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x02,
    0x02, 0x17, 0x12, 0x03, 0x28, 0x08, 0x2c, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x02, 0x02, 0x17, 0x01,
    0x12, 0x03, 0x28, 0x08, 0x26, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x02, 0x02, 0x17, 0x02, 0x12, 0x03,
    0x28, 0x29, 0x2b, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x02, 0x02, 0x18, 0x12, 0x03, 0x29, 0x08, 0x2a,
    0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x02, 0x02, 0x18, 0x01, 0x12, 0x03, 0x29, 0x08, 0x24, 0x0a, 0x0c,
    0x0a, 0x05, 0x05, 0x02, 0x02, 0x18, 0x02, 0x12, 0x03, 0x29, 0x27, 0x29, 0x0a, 0x0b, 0x0a, 0x04,
    0x05, 0x02, 0x02, 0x19, 0x12, 0x03, 0x2a, 0x08, 0x30, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x02, 0x02,
    0x19, 0x01, 0x12, 0x03, 0x2a, 0x08, 0x2a, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x02, 0x02, 0x19, 0x02,
    0x12, 0x03, 0x2a, 0x2d, 0x2f, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x02, 0x02, 0x1a, 0x12, 0x03, 0x2b,
    0x08, 0x2e, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x02, 0x02, 0x1a, 0x01, 0x12, 0x03, 0x2b, 0x08, 0x28,
    0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x02, 0x02, 0x1a, 0x02, 0x12, 0x03, 0x2b, 0x2b, 0x2d, 0x0a, 0x0b,
    0x0a, 0x04, 0x05, 0x02, 0x02, 0x1b, 0x12, 0x03, 0x2c, 0x08, 0x2b, 0x0a, 0x0c, 0x0a, 0x05, 0x05,
    0x02, 0x02, 0x1b, 0x01, 0x12, 0x03, 0x2c, 0x08, 0x25, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x02, 0x02,
    0x1b, 0x02, 0x12, 0x03, 0x2c, 0x28, 0x2a, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x02, 0x02, 0x1c, 0x12,
    0x03, 0x2d, 0x08, 0x31, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x02, 0x02, 0x1c, 0x01, 0x12, 0x03, 0x2d,
    0x08, 0x2b, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x02, 0x02, 0x1c, 0x02, 0x12, 0x03, 0x2d, 0x2e, 0x30,
    0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x02, 0x02, 0x1d, 0x12, 0x03, 0x2e, 0x08, 0x2f, 0x0a, 0x0c, 0x0a,
    0x05, 0x05, 0x02, 0x02, 0x1d, 0x01, 0x12, 0x03, 0x2e, 0x08, 0x29, 0x0a, 0x0c, 0x0a, 0x05, 0x05,
    0x02, 0x02, 0x1d, 0x02, 0x12, 0x03, 0x2e, 0x2c, 0x2e, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x02, 0x02,
    0x1e, 0x12, 0x03, 0x2f, 0x08, 0x2c, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x02, 0x02, 0x1e, 0x01, 0x12,
    0x03, 0x2f, 0x08, 0x26, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x02, 0x02, 0x1e, 0x02, 0x12, 0x03, 0x2f,
    0x29, 0x2b, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x02, 0x02, 0x1f, 0x12, 0x03, 0x30, 0x08, 0x35, 0x0a,
    0x0c, 0x0a, 0x05, 0x05, 0x02, 0x02, 0x1f, 0x01, 0x12, 0x03, 0x30, 0x08, 0x2f, 0x0a, 0x0c, 0x0a,
    0x05, 0x05, 0x02, 0x02, 0x1f, 0x02, 0x12, 0x03, 0x30, 0x32, 0x34, 0x0a, 0x0b, 0x0a, 0x04, 0x05,
    0x02, 0x02, 0x20, 0x12, 0x03, 0x31, 0x08, 0x32, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x02, 0x02, 0x20,
    0x01, 0x12, 0x03, 0x31, 0x08, 0x2c, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x02, 0x02, 0x20, 0x02, 0x12,
    0x03, 0x31, 0x2f, 0x31, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x02, 0x02, 0x21, 0x12, 0x03, 0x32, 0x08,
    0x30, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x02, 0x02, 0x21, 0x01, 0x12, 0x03, 0x32, 0x08, 0x2a, 0x0a,
    0x0c, 0x0a, 0x05, 0x05, 0x02, 0x02, 0x21, 0x02, 0x12, 0x03, 0x32, 0x2d, 0x2f, 0x0a, 0x0b, 0x0a,
    0x04, 0x05, 0x02, 0x02, 0x22, 0x12, 0x03, 0x33, 0x08, 0x35, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x02,
    0x02, 0x22, 0x01, 0x12, 0x03, 0x33, 0x08, 0x2f, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x02, 0x02, 0x22,
    0x02, 0x12, 0x03, 0x33, 0x32, 0x34, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x02, 0x02, 0x23, 0x12, 0x03,
    0x34, 0x08, 0x28, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x02, 0x02, 0x23, 0x01, 0x12, 0x03, 0x34, 0x08,
    0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x02, 0x02, 0x23, 0x02, 0x12, 0x03, 0x34, 0x25, 0x27, 0x0a,
    0x0b, 0x0a, 0x04, 0x05, 0x02, 0x02, 0x24, 0x12, 0x03, 0x35, 0x08, 0x2e, 0x0a, 0x0c, 0x0a, 0x05,
    0x05, 0x02, 0x02, 0x24, 0x01, 0x12, 0x03, 0x35, 0x08, 0x28, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x02,
    0x02, 0x24, 0x02, 0x12, 0x03, 0x35, 0x2b, 0x2d, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x02, 0x02, 0x25,
    0x12, 0x03, 0x36, 0x08, 0x26, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x02, 0x02, 0x25, 0x01, 0x12, 0x03,
    0x36, 0x08, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x02, 0x02, 0x25, 0x02, 0x12, 0x03, 0x36, 0x23,
    0x25, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x02, 0x02, 0x26, 0x12, 0x03, 0x37, 0x08, 0x25, 0x0a, 0x0c,
    0x0a, 0x05, 0x05, 0x02, 0x02, 0x26, 0x01, 0x12, 0x03, 0x37, 0x08, 0x1f, 0x0a, 0x0c, 0x0a, 0x05,
    0x05, 0x02, 0x02, 0x26, 0x02, 0x12, 0x03, 0x37, 0x22, 0x24, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x02,
    0x02, 0x27, 0x12, 0x03, 0x38, 0x08, 0x29, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x02, 0x02, 0x27, 0x01,
    0x12, 0x03, 0x38, 0x08, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x02, 0x02, 0x27, 0x02, 0x12, 0x03,
    0x38, 0x26, 0x28, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x02, 0x02, 0x28, 0x12, 0x03, 0x39, 0x08, 0x2d,
    0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x02, 0x02, 0x28, 0x01, 0x12, 0x03, 0x39, 0x08, 0x27, 0x0a, 0x0c,
    0x0a, 0x05, 0x05, 0x02, 0x02, 0x28, 0x02, 0x12, 0x03, 0x39, 0x2a, 0x2c, 0x0a, 0x0b, 0x0a, 0x04,
    0x05, 0x02, 0x02, 0x29, 0x12, 0x03, 0x3a, 0x08, 0x32, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x02, 0x02,
    0x29, 0x01, 0x12, 0x03, 0x3a, 0x08, 0x2c, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x02, 0x02, 0x29, 0x02,
    0x12, 0x03, 0x3a, 0x2f, 0x31, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x02, 0x02, 0x2a, 0x12, 0x03, 0x3b,
    0x08, 0x32, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x02, 0x02, 0x2a, 0x01, 0x12, 0x03, 0x3b, 0x08, 0x2c,
    0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x02, 0x02, 0x2a, 0x02, 0x12, 0x03, 0x3b, 0x2f, 0x31, 0x0a, 0x0b,
    0x0a, 0x04, 0x05, 0x02, 0x02, 0x2b, 0x12, 0x03, 0x3c, 0x08, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x05,
    0x02, 0x02, 0x2b, 0x01, 0x12, 0x03, 0x3c, 0x08, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x02, 0x02,
    0x2b, 0x02, 0x12, 0x03, 0x3c, 0x21, 0x23, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x02, 0x02, 0x2c, 0x12,
    0x03, 0x3d, 0x08, 0x3d, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x02, 0x02, 0x2c, 0x01, 0x12, 0x03, 0x3d,
    0x08, 0x37, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x02, 0x02, 0x2c, 0x02, 0x12, 0x03, 0x3d, 0x3a, 0x3c,
    0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x02, 0x02, 0x2d, 0x12, 0x03, 0x3e, 0x08, 0x2a, 0x0a, 0x0c, 0x0a,
    0x05, 0x05, 0x02, 0x02, 0x2d, 0x01, 0x12, 0x03, 0x3e, 0x08, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x05,
    0x02, 0x02, 0x2d, 0x02, 0x12, 0x03, 0x3e, 0x27, 0x29, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x02, 0x02,
    0x2e, 0x12, 0x03, 0x3f, 0x08, 0x2e, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x02, 0x02, 0x2e, 0x01, 0x12,
    0x03, 0x3f, 0x08, 0x28, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x02, 0x02, 0x2e, 0x02, 0x12, 0x03, 0x3f,
    0x2b, 0x2d, 0x0a, 0x0a, 0x0a, 0x02, 0x05, 0x03, 0x12, 0x04, 0x42, 0x00, 0x45, 0x01, 0x0a, 0x0a,
    0x0a, 0x03, 0x05, 0x03, 0x01, 0x12, 0x03, 0x42, 0x05, 0x13, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x03,
    0x02, 0x00, 0x12, 0x03, 0x43, 0x08, 0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x03, 0x02, 0x00, 0x01,
    0x12, 0x03, 0x43, 0x08, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x03, 0x02, 0x00, 0x02, 0x12, 0x03,
    0x43, 0x1f, 0x20, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x03, 0x02, 0x01, 0x12, 0x03, 0x44, 0x08, 0x24,
    0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x03, 0x02, 0x01, 0x01, 0x12, 0x03, 0x44, 0x08, 0x1f, 0x0a, 0x0c,
    0x0a, 0x05, 0x05, 0x03, 0x02, 0x01, 0x02, 0x12, 0x03, 0x44, 0x22, 0x23, 0x0a, 0x0a, 0x0a, 0x02,
    0x05, 0x04, 0x12, 0x04, 0x47, 0x00, 0x4e, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x05, 0x04, 0x01, 0x12,
    0x03, 0x47, 0x05, 0x16, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x04, 0x02, 0x00, 0x12, 0x03, 0x48, 0x08,
    0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x04, 0x02, 0x00, 0x01, 0x12, 0x03, 0x48, 0x08, 0x1f, 0x0a,
    0x0c, 0x0a, 0x05, 0x05, 0x04, 0x02, 0x00, 0x02, 0x12, 0x03, 0x48, 0x22, 0x23, 0x0a, 0x0b, 0x0a,
    0x04, 0x05, 0x04, 0x02, 0x01, 0x12, 0x03, 0x49, 0x08, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x04,
    0x02, 0x01, 0x01, 0x12, 0x03, 0x49, 0x08, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x04, 0x02, 0x01,
    0x02, 0x12, 0x03, 0x49, 0x21, 0x22, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x04, 0x02, 0x02, 0x12, 0x03,
    0x4a, 0x08, 0x26, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x04, 0x02, 0x02, 0x01, 0x12, 0x03, 0x4a, 0x08,
    0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x04, 0x02, 0x02, 0x02, 0x12, 0x03, 0x4a, 0x24, 0x25, 0x0a,
    0x0b, 0x0a, 0x04, 0x05, 0x04, 0x02, 0x03, 0x12, 0x03, 0x4b, 0x08, 0x24, 0x0a, 0x0c, 0x0a, 0x05,
    0x05, 0x04, 0x02, 0x03, 0x01, 0x12, 0x03, 0x4b, 0x08, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x04,
    0x02, 0x03, 0x02, 0x12, 0x03, 0x4b, 0x22, 0x23, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x04, 0x02, 0x04,
    0x12, 0x03, 0x4c, 0x08, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x04, 0x02, 0x04, 0x01, 0x12, 0x03,
    0x4c, 0x08, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x04, 0x02, 0x04, 0x02, 0x12, 0x03, 0x4c, 0x21,
    0x22, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x04, 0x02, 0x05, 0x12, 0x03, 0x4d, 0x08, 0x23, 0x0a, 0x0c,
    0x0a, 0x05, 0x05, 0x04, 0x02, 0x05, 0x01, 0x12, 0x03, 0x4d, 0x08, 0x1e, 0x0a, 0x0c, 0x0a, 0x05,
    0x05, 0x04, 0x02, 0x05, 0x02, 0x12, 0x03, 0x4d, 0x21, 0x22, 0x0a, 0x0a, 0x0a, 0x02, 0x05, 0x05,
    0x12, 0x04, 0x50, 0x00, 0x59, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x05, 0x05, 0x01, 0x12, 0x03, 0x50,
    0x05, 0x16, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x05, 0x02, 0x00, 0x12, 0x03, 0x51, 0x08, 0x24, 0x0a,
    0x0c, 0x0a, 0x05, 0x05, 0x05, 0x02, 0x00, 0x01, 0x12, 0x03, 0x51, 0x08, 0x1f, 0x0a, 0x0c, 0x0a,
    0x05, 0x05, 0x05, 0x02, 0x00, 0x02, 0x12, 0x03, 0x51, 0x22, 0x23, 0x0a, 0x0b, 0x0a, 0x04, 0x05,
    0x05, 0x02, 0x01, 0x12, 0x03, 0x52, 0x08, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x05, 0x02, 0x01,
    0x01, 0x12, 0x03, 0x52, 0x08, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x05, 0x02, 0x01, 0x02, 0x12,
    0x03, 0x52, 0x21, 0x22, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x05, 0x02, 0x02, 0x12, 0x03, 0x53, 0x08,
    0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x05, 0x02, 0x02, 0x01, 0x12, 0x03, 0x53, 0x08, 0x1e, 0x0a,
    0x0c, 0x0a, 0x05, 0x05, 0x05, 0x02, 0x02, 0x02, 0x12, 0x03, 0x53, 0x21, 0x22, 0x0a, 0x0b, 0x0a,
    0x04, 0x05, 0x05, 0x02, 0x03, 0x12, 0x03, 0x54, 0x08, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x05,
    0x02, 0x03, 0x01, 0x12, 0x03, 0x54, 0x08, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x05, 0x02, 0x03,
    0x02, 0x12, 0x03, 0x54, 0x21, 0x22, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x05, 0x02, 0x04, 0x12, 0x03,
    0x55, 0x08, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x05, 0x02, 0x04, 0x01, 0x12, 0x03, 0x55, 0x08,
    0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x05, 0x02, 0x04, 0x02, 0x12, 0x03, 0x55, 0x22, 0x23, 0x0a,
    0x0b, 0x0a, 0x04, 0x05, 0x05, 0x02, 0x05, 0x12, 0x03, 0x56, 0x08, 0x24, 0x0a, 0x0c, 0x0a, 0x05,
    0x05, 0x05, 0x02, 0x05, 0x01, 0x12, 0x03, 0x56, 0x08, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x05,
    0x02, 0x05, 0x02, 0x12, 0x03, 0x56, 0x22, 0x23, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x05, 0x02, 0x06,
    0x12, 0x03, 0x57, 0x08, 0x25, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x05, 0x02, 0x06, 0x01, 0x12, 0x03,
    0x57, 0x08, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x05, 0x02, 0x06, 0x02, 0x12, 0x03, 0x57, 0x23,
    0x24, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x05, 0x02, 0x07, 0x12, 0x03, 0x58, 0x08, 0x25, 0x0a, 0x0c,
    0x0a, 0x05, 0x05, 0x05, 0x02, 0x07, 0x01, 0x12, 0x03, 0x58, 0x08, 0x20, 0x0a, 0x0c, 0x0a, 0x05,
    0x05, 0x05, 0x02, 0x07, 0x02, 0x12, 0x03, 0x58, 0x23, 0x24, 0x0a, 0x0a, 0x0a, 0x02, 0x05, 0x06,
    0x12, 0x04, 0x5b, 0x00, 0x5e, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x05, 0x06, 0x01, 0x12, 0x03, 0x5b,
    0x05, 0x17, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x06, 0x02, 0x00, 0x12, 0x03, 0x5c, 0x08, 0x22, 0x0a,
    0x0c, 0x0a, 0x05, 0x05, 0x06, 0x02, 0x00, 0x01, 0x12, 0x03, 0x5c, 0x08, 0x1d, 0x0a, 0x0c, 0x0a,
    0x05, 0x05, 0x06, 0x02, 0x00, 0x02, 0x12, 0x03, 0x5c, 0x20, 0x21, 0x0a, 0x0b, 0x0a, 0x04, 0x05,
    0x06, 0x02, 0x01, 0x12, 0x03, 0x5d, 0x08, 0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x06, 0x02, 0x01,
    0x01, 0x12, 0x03, 0x5d, 0x08, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x06, 0x02, 0x01, 0x02, 0x12,
    0x03, 0x5d, 0x20, 0x21, 0x0a, 0x0a, 0x0a, 0x02, 0x05, 0x07, 0x12, 0x04, 0x60, 0x00, 0x67, 0x01,
    0x0a, 0x0a, 0x0a, 0x03, 0x05, 0x07, 0x01, 0x12, 0x03, 0x60, 0x05, 0x17, 0x0a, 0x0b, 0x0a, 0x04,
    0x05, 0x07, 0x02, 0x00, 0x12, 0x03, 0x61, 0x08, 0x25, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x07, 0x02,
    0x00, 0x01, 0x12, 0x03, 0x61, 0x08, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x07, 0x02, 0x00, 0x02,
    0x12, 0x03, 0x61, 0x23, 0x24, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x07, 0x02, 0x01, 0x12, 0x03, 0x62,
    0x08, 0x26, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x07, 0x02, 0x01, 0x01, 0x12, 0x03, 0x62, 0x08, 0x21,
    0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x07, 0x02, 0x01, 0x02, 0x12, 0x03, 0x62, 0x24, 0x25, 0x0a, 0x0b,
    0x0a, 0x04, 0x05, 0x07, 0x02, 0x02, 0x12, 0x03, 0x63, 0x08, 0x28, 0x0a, 0x0c, 0x0a, 0x05, 0x05,
    0x07, 0x02, 0x02, 0x01, 0x12, 0x03, 0x63, 0x08, 0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x07, 0x02,
    0x02, 0x02, 0x12, 0x03, 0x63, 0x25, 0x27, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x07, 0x02, 0x03, 0x12,
    0x03, 0x64, 0x08, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x07, 0x02, 0x03, 0x01, 0x12, 0x03, 0x64,
    0x08, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x07, 0x02, 0x03, 0x02, 0x12, 0x03, 0x64, 0x21, 0x23,
    0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x07, 0x02, 0x04, 0x12, 0x03, 0x65, 0x08, 0x24, 0x0a, 0x0c, 0x0a,
    0x05, 0x05, 0x07, 0x02, 0x04, 0x01, 0x12, 0x03, 0x65, 0x08, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x05,
    0x07, 0x02, 0x04, 0x02, 0x12, 0x03, 0x65, 0x21, 0x23, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x07, 0x02,
    0x05, 0x12, 0x03, 0x66, 0x08, 0x2b, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x07, 0x02, 0x05, 0x01, 0x12,
    0x03, 0x66, 0x08, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x07, 0x02, 0x05, 0x02, 0x12, 0x03, 0x66,
    0x26, 0x2a, 0x0a, 0x0a, 0x0a, 0x02, 0x05, 0x08, 0x12, 0x04, 0x69, 0x00, 0x6e, 0x01, 0x0a, 0x0a,
    0x0a, 0x03, 0x05, 0x08, 0x01, 0x12, 0x03, 0x69, 0x05, 0x1f, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x08,
    0x02, 0x00, 0x12, 0x03, 0x6a, 0x08, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x08, 0x02, 0x00, 0x01,
    0x12, 0x03, 0x6a, 0x08, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x08, 0x02, 0x00, 0x02, 0x12, 0x03,
    0x6a, 0x20, 0x23, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x08, 0x02, 0x01, 0x12, 0x03, 0x6b, 0x08, 0x27,
    0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x08, 0x02, 0x01, 0x01, 0x12, 0x03, 0x6b, 0x08, 0x1f, 0x0a, 0x0c,
    0x0a, 0x05, 0x05, 0x08, 0x02, 0x01, 0x02, 0x12, 0x03, 0x6b, 0x22, 0x26, 0x0a, 0x0b, 0x0a, 0x04,
    0x05, 0x08, 0x02, 0x02, 0x12, 0x03, 0x6c, 0x08, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x08, 0x02,
    0x02, 0x01, 0x12, 0x03, 0x6c, 0x08, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x08, 0x02, 0x02, 0x02,
    0x12, 0x03, 0x6c, 0x22, 0x23, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x08, 0x02, 0x03, 0x12, 0x03, 0x6d,
    0x08, 0x25, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x08, 0x02, 0x03, 0x01, 0x12, 0x03, 0x6d, 0x08, 0x20,
    0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x08, 0x02, 0x03, 0x02, 0x12, 0x03, 0x6d, 0x23, 0x24, 0x0a, 0x0b,
    0x0a, 0x02, 0x05, 0x09, 0x12, 0x05, 0x70, 0x00, 0x87, 0x01, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x05,
    0x09, 0x01, 0x12, 0x03, 0x70, 0x05, 0x1c, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x09, 0x02, 0x00, 0x12,
    0x03, 0x71, 0x08, 0x29, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x09, 0x02, 0x00, 0x01, 0x12, 0x03, 0x71,
    0x08, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x09, 0x02, 0x00, 0x02, 0x12, 0x03, 0x71, 0x27, 0x28,
    0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x09, 0x02, 0x01, 0x12, 0x03, 0x72, 0x08, 0x28, 0x0a, 0x0c, 0x0a,
    0x05, 0x05, 0x09, 0x02, 0x01, 0x01, 0x12, 0x03, 0x72, 0x08, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x05,
    0x09, 0x02, 0x01, 0x02, 0x12, 0x03, 0x72, 0x26, 0x27, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x09, 0x02,
    0x02, 0x12, 0x03, 0x73, 0x08, 0x2a, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x09, 0x02, 0x02, 0x01, 0x12,
    0x03, 0x73, 0x08, 0x25, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x09, 0x02, 0x02, 0x02, 0x12, 0x03, 0x73,
    0x28, 0x29, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x09, 0x02, 0x03, 0x12, 0x03, 0x74, 0x08, 0x2a, 0x0a,
    0x0c, 0x0a, 0x05, 0x05, 0x09, 0x02, 0x03, 0x01, 0x12, 0x03, 0x74, 0x08, 0x25, 0x0a, 0x0c, 0x0a,
    0x05, 0x05, 0x09, 0x02, 0x03, 0x02, 0x12, 0x03, 0x74, 0x28, 0x29, 0x0a, 0x0b, 0x0a, 0x04, 0x05,
    0x09, 0x02, 0x04, 0x12, 0x03, 0x75, 0x08, 0x2b, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x09, 0x02, 0x04,
    0x01, 0x12, 0x03, 0x75, 0x08, 0x26, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x09, 0x02, 0x04, 0x02, 0x12,
    0x03, 0x75, 0x29, 0x2a, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x09, 0x02, 0x05, 0x12, 0x03, 0x76, 0x08,
    0x28, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x09, 0x02, 0x05, 0x01, 0x12, 0x03, 0x76, 0x08, 0x22, 0x0a,
    0x0c, 0x0a, 0x05, 0x05, 0x09, 0x02, 0x05, 0x02, 0x12, 0x03, 0x76, 0x25, 0x27, 0x0a, 0x0b, 0x0a,
    0x04, 0x05, 0x09, 0x02, 0x06, 0x12, 0x03, 0x77, 0x08, 0x27, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x09,
    0x02, 0x06, 0x01, 0x12, 0x03, 0x77, 0x08, 0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x09, 0x02, 0x06,
    0x02, 0x12, 0x03, 0x77, 0x24, 0x26, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x09, 0x02, 0x07, 0x12, 0x03,
    0x78, 0x08, 0x2c, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x09, 0x02, 0x07, 0x01, 0x12, 0x03, 0x78, 0x08,
    0x26, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x09, 0x02, 0x07, 0x02, 0x12, 0x03, 0x78, 0x29, 0x2b, 0x0a,
    0x0b, 0x0a, 0x04, 0x05, 0x09, 0x02, 0x08, 0x12, 0x03, 0x79, 0x08, 0x2e, 0x0a, 0x0c, 0x0a, 0x05,
    0x05, 0x09, 0x02, 0x08, 0x01, 0x12, 0x03, 0x79, 0x08, 0x27, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x09,
    0x02, 0x08, 0x02, 0x12, 0x03, 0x79, 0x2a, 0x2d, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x09, 0x02, 0x09,
    0x12, 0x03, 0x7a, 0x08, 0x30, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x09, 0x02, 0x09, 0x01, 0x12, 0x03,
    0x7a, 0x08, 0x29, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x09, 0x02, 0x09, 0x02, 0x12, 0x03, 0x7a, 0x2c,
    0x2f, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x09, 0x02, 0x0a, 0x12, 0x03, 0x7b, 0x08, 0x31, 0x0a, 0x0c,
    0x0a, 0x05, 0x05, 0x09, 0x02, 0x0a, 0x01, 0x12, 0x03, 0x7b, 0x08, 0x2a, 0x0a, 0x0c, 0x0a, 0x05,
    0x05, 0x09, 0x02, 0x0a, 0x02, 0x12, 0x03, 0x7b, 0x2d, 0x30, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x09,
    0x02, 0x0b, 0x12, 0x03, 0x7c, 0x08, 0x2a, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x09, 0x02, 0x0b, 0x01,
    0x12, 0x03, 0x7c, 0x08, 0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x09, 0x02, 0x0b, 0x02, 0x12, 0x03,
    0x7c, 0x25, 0x29, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x09, 0x02, 0x0c, 0x12, 0x03, 0x7d, 0x08, 0x26,
    0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x09, 0x02, 0x0c, 0x01, 0x12, 0x03, 0x7d, 0x08, 0x1e, 0x0a, 0x0c,
    0x0a, 0x05, 0x05, 0x09, 0x02, 0x0c, 0x02, 0x12, 0x03, 0x7d, 0x21, 0x25, 0x0a, 0x0b, 0x0a, 0x04,
    0x05, 0x09, 0x02, 0x0d, 0x12, 0x03, 0x7e, 0x08, 0x26, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x09, 0x02,
    0x0d, 0x01, 0x12, 0x03, 0x7e, 0x08, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x09, 0x02, 0x0d, 0x02,
    0x12, 0x03, 0x7e, 0x21, 0x25, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x09, 0x02, 0x0e, 0x12, 0x03, 0x7f,
    0x08, 0x27, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x09, 0x02, 0x0e, 0x01, 0x12, 0x03, 0x7f, 0x08, 0x1e,
    0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x09, 0x02, 0x0e, 0x02, 0x12, 0x03, 0x7f, 0x21, 0x26, 0x0a, 0x0c,
    0x0a, 0x04, 0x05, 0x09, 0x02, 0x0f, 0x12, 0x04, 0x80, 0x01, 0x08, 0x27, 0x0a, 0x0d, 0x0a, 0x05,
    0x05, 0x09, 0x02, 0x0f, 0x01, 0x12, 0x04, 0x80, 0x01, 0x08, 0x1e, 0x0a, 0x0d, 0x0a, 0x05, 0x05,
    0x09, 0x02, 0x0f, 0x02, 0x12, 0x04, 0x80, 0x01, 0x21, 0x26, 0x0a, 0x0c, 0x0a, 0x04, 0x05, 0x09,
    0x02, 0x10, 0x12, 0x04, 0x81, 0x01, 0x08, 0x30, 0x0a, 0x0d, 0x0a, 0x05, 0x05, 0x09, 0x02, 0x10,
    0x01, 0x12, 0x04, 0x81, 0x01, 0x08, 0x27, 0x0a, 0x0d, 0x0a, 0x05, 0x05, 0x09, 0x02, 0x10, 0x02,
    0x12, 0x04, 0x81, 0x01, 0x2a, 0x2f, 0x0a, 0x0c, 0x0a, 0x04, 0x05, 0x09, 0x02, 0x11, 0x12, 0x04,
    0x82, 0x01, 0x08, 0x31, 0x0a, 0x0d, 0x0a, 0x05, 0x05, 0x09, 0x02, 0x11, 0x01, 0x12, 0x04, 0x82,
    0x01, 0x08, 0x27, 0x0a, 0x0d, 0x0a, 0x05, 0x05, 0x09, 0x02, 0x11, 0x02, 0x12, 0x04, 0x82, 0x01,
    0x2a, 0x30, 0x0a, 0x0c, 0x0a, 0x04, 0x05, 0x09, 0x02, 0x12, 0x12, 0x04, 0x83, 0x01, 0x08, 0x32,
    0x0a, 0x0d, 0x0a, 0x05, 0x05, 0x09, 0x02, 0x12, 0x01, 0x12, 0x04, 0x83, 0x01, 0x08, 0x28, 0x0a,
    0x0d, 0x0a, 0x05, 0x05, 0x09, 0x02, 0x12, 0x02, 0x12, 0x04, 0x83, 0x01, 0x2b, 0x31, 0x0a, 0x0c,
    0x0a, 0x04, 0x05, 0x09, 0x02, 0x13, 0x12, 0x04, 0x84, 0x01, 0x08, 0x32, 0x0a, 0x0d, 0x0a, 0x05,
    0x05, 0x09, 0x02, 0x13, 0x01, 0x12, 0x04, 0x84, 0x01, 0x08, 0x28, 0x0a, 0x0d, 0x0a, 0x05, 0x05,
    0x09, 0x02, 0x13, 0x02, 0x12, 0x04, 0x84, 0x01, 0x2b, 0x31, 0x0a, 0x0c, 0x0a, 0x04, 0x05, 0x09,
    0x02, 0x14, 0x12, 0x04, 0x85, 0x01, 0x08, 0x33, 0x0a, 0x0d, 0x0a, 0x05, 0x05, 0x09, 0x02, 0x14,
    0x01, 0x12, 0x04, 0x85, 0x01, 0x08, 0x28, 0x0a, 0x0d, 0x0a, 0x05, 0x05, 0x09, 0x02, 0x14, 0x02,
    0x12, 0x04, 0x85, 0x01, 0x2b, 0x32, 0x0a, 0x0c, 0x0a, 0x04, 0x05, 0x09, 0x02, 0x15, 0x12, 0x04,
    0x86, 0x01, 0x08, 0x34, 0x0a, 0x0d, 0x0a, 0x05, 0x05, 0x09, 0x02, 0x15, 0x01, 0x12, 0x04, 0x86,
    0x01, 0x08, 0x29, 0x0a, 0x0d, 0x0a, 0x05, 0x05, 0x09, 0x02, 0x15, 0x02, 0x12, 0x04, 0x86, 0x01,
    0x2c, 0x33, 0x0a, 0x0c, 0x0a, 0x02, 0x05, 0x0a, 0x12, 0x06, 0x89, 0x01, 0x00, 0x8d, 0x01, 0x01,
    0x0a, 0x0b, 0x0a, 0x03, 0x05, 0x0a, 0x01, 0x12, 0x04, 0x89, 0x01, 0x05, 0x1d, 0x0a, 0x0c, 0x0a,
    0x04, 0x05, 0x0a, 0x02, 0x00, 0x12, 0x04, 0x8a, 0x01, 0x08, 0x21, 0x0a, 0x0d, 0x0a, 0x05, 0x05,
    0x0a, 0x02, 0x00, 0x01, 0x12, 0x04, 0x8a, 0x01, 0x08, 0x1c, 0x0a, 0x0d, 0x0a, 0x05, 0x05, 0x0a,
    0x02, 0x00, 0x02, 0x12, 0x04, 0x8a, 0x01, 0x1f, 0x20, 0x0a, 0x0c, 0x0a, 0x04, 0x05, 0x0a, 0x02,
    0x01, 0x12, 0x04, 0x8b, 0x01, 0x08, 0x25, 0x0a, 0x0d, 0x0a, 0x05, 0x05, 0x0a, 0x02, 0x01, 0x01,
    0x12, 0x04, 0x8b, 0x01, 0x08, 0x20, 0x0a, 0x0d, 0x0a, 0x05, 0x05, 0x0a, 0x02, 0x01, 0x02, 0x12,
    0x04, 0x8b, 0x01, 0x23, 0x24, 0x0a, 0x0c, 0x0a, 0x04, 0x05, 0x0a, 0x02, 0x02, 0x12, 0x04, 0x8c,
    0x01, 0x08, 0x26, 0x0a, 0x0d, 0x0a, 0x05, 0x05, 0x0a, 0x02, 0x02, 0x01, 0x12, 0x04, 0x8c, 0x01,
    0x08, 0x21, 0x0a, 0x0d, 0x0a, 0x05, 0x05, 0x0a, 0x02, 0x02, 0x02, 0x12, 0x04, 0x8c, 0x01, 0x24,
    0x25, 0x0a, 0x0c, 0x0a, 0x02, 0x05, 0x0b, 0x12, 0x06, 0x8f, 0x01, 0x00, 0x92, 0x01, 0x01, 0x0a,
    0x0b, 0x0a, 0x03, 0x05, 0x0b, 0x01, 0x12, 0x04, 0x8f, 0x01, 0x05, 0x13, 0x0a, 0x0c, 0x0a, 0x04,
    0x05, 0x0b, 0x02, 0x00, 0x12, 0x04, 0x90, 0x01, 0x08, 0x28, 0x0a, 0x0d, 0x0a, 0x05, 0x05, 0x0b,
    0x02, 0x00, 0x01, 0x12, 0x04, 0x90, 0x01, 0x08, 0x22, 0x0a, 0x0d, 0x0a, 0x05, 0x05, 0x0b, 0x02,
    0x00, 0x02, 0x12, 0x04, 0x90, 0x01, 0x25, 0x27, 0x0a, 0x0c, 0x0a, 0x04, 0x05, 0x0b, 0x02, 0x01,
    0x12, 0x04, 0x91, 0x01, 0x08, 0x26, 0x0a, 0x0d, 0x0a, 0x05, 0x05, 0x0b, 0x02, 0x01, 0x01, 0x12,
    0x04, 0x91, 0x01, 0x08, 0x21, 0x0a, 0x0d, 0x0a, 0x05, 0x05, 0x0b, 0x02, 0x01, 0x02, 0x12, 0x04,
    0x91, 0x01, 0x24, 0x25, 0x0a, 0x0c, 0x0a, 0x02, 0x05, 0x0c, 0x12, 0x06, 0x94, 0x01, 0x00, 0x9c,
    0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x05, 0x0c, 0x01, 0x12, 0x04, 0x94, 0x01, 0x05, 0x1c, 0x0a,
    0x0c, 0x0a, 0x04, 0x05, 0x0c, 0x02, 0x00, 0x12, 0x04, 0x95, 0x01, 0x08, 0x2a, 0x0a, 0x0d, 0x0a,
    0x05, 0x05, 0x0c, 0x02, 0x00, 0x01, 0x12, 0x04, 0x95, 0x01, 0x08, 0x25, 0x0a, 0x0d, 0x0a, 0x05,
    0x05, 0x0c, 0x02, 0x00, 0x02, 0x12, 0x04, 0x95, 0x01, 0x28, 0x29, 0x0a, 0x0c, 0x0a, 0x04, 0x05,
    0x0c, 0x02, 0x01, 0x12, 0x04, 0x96, 0x01, 0x08, 0x2a, 0x0a, 0x0d, 0x0a, 0x05, 0x05, 0x0c, 0x02,
    0x01, 0x01, 0x12, 0x04, 0x96, 0x01, 0x08, 0x25, 0x0a, 0x0d, 0x0a, 0x05, 0x05, 0x0c, 0x02, 0x01,
    0x02, 0x12, 0x04, 0x96, 0x01, 0x28, 0x29, 0x0a, 0x0c, 0x0a, 0x04, 0x05, 0x0c, 0x02, 0x02, 0x12,
    0x04, 0x97, 0x01, 0x08, 0x29, 0x0a, 0x0d, 0x0a, 0x05, 0x05, 0x0c, 0x02, 0x02, 0x01, 0x12, 0x04,
    0x97, 0x01, 0x08, 0x24, 0x0a, 0x0d, 0x0a, 0x05, 0x05, 0x0c, 0x02, 0x02, 0x02, 0x12, 0x04, 0x97,
    0x01, 0x27, 0x28, 0x0a, 0x0c, 0x0a, 0x04, 0x05, 0x0c, 0x02, 0x03, 0x12, 0x04, 0x98, 0x01, 0x08,
    0x2a, 0x0a, 0x0d, 0x0a, 0x05, 0x05, 0x0c, 0x02, 0x03, 0x01, 0x12, 0x04, 0x98, 0x01, 0x08, 0x25,
    0x0a, 0x0d, 0x0a, 0x05, 0x05, 0x0c, 0x02, 0x03, 0x02, 0x12, 0x04, 0x98, 0x01, 0x28, 0x29, 0x0a,
    0x0c, 0x0a, 0x04, 0x05, 0x0c, 0x02, 0x04, 0x12, 0x04, 0x99, 0x01, 0x08, 0x2a, 0x0a, 0x0d, 0x0a,
    0x05, 0x05, 0x0c, 0x02, 0x04, 0x01, 0x12, 0x04, 0x99, 0x01, 0x08, 0x24, 0x0a, 0x0d, 0x0a, 0x05,
    0x05, 0x0c, 0x02, 0x04, 0x02, 0x12, 0x04, 0x99, 0x01, 0x27, 0x29, 0x0a, 0x0c, 0x0a, 0x04, 0x05,
    0x0c, 0x02, 0x05, 0x12, 0x04, 0x9a, 0x01, 0x08, 0x28, 0x0a, 0x0d, 0x0a, 0x05, 0x05, 0x0c, 0x02,
    0x05, 0x01, 0x12, 0x04, 0x9a, 0x01, 0x08, 0x22, 0x0a, 0x0d, 0x0a, 0x05, 0x05, 0x0c, 0x02, 0x05,
    0x02, 0x12, 0x04, 0x9a, 0x01, 0x25, 0x27, 0x0a, 0x0c, 0x0a, 0x04, 0x05, 0x0c, 0x02, 0x06, 0x12,
    0x04, 0x9b, 0x01, 0x08, 0x2b, 0x0a, 0x0d, 0x0a, 0x05, 0x05, 0x0c, 0x02, 0x06, 0x01, 0x12, 0x04,
    0x9b, 0x01, 0x08, 0x25, 0x0a, 0x0d, 0x0a, 0x05, 0x05, 0x0c, 0x02, 0x06, 0x02, 0x12, 0x04, 0x9b,
    0x01, 0x28, 0x2a, 0x0a, 0x0c, 0x0a, 0x02, 0x05, 0x0d, 0x12, 0x06, 0x9e, 0x01, 0x00, 0xa1, 0x01,
    0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x05, 0x0d, 0x01, 0x12, 0x04, 0x9e, 0x01, 0x05, 0x17, 0x0a, 0x0c,
    0x0a, 0x04, 0x05, 0x0d, 0x02, 0x00, 0x12, 0x04, 0x9f, 0x01, 0x08, 0x20, 0x0a, 0x0d, 0x0a, 0x05,
    0x05, 0x0d, 0x02, 0x00, 0x01, 0x12, 0x04, 0x9f, 0x01, 0x08, 0x1b, 0x0a, 0x0d, 0x0a, 0x05, 0x05,
    0x0d, 0x02, 0x00, 0x02, 0x12, 0x04, 0x9f, 0x01, 0x1e, 0x1f, 0x0a, 0x0c, 0x0a, 0x04, 0x05, 0x0d,
    0x02, 0x01, 0x12, 0x04, 0xa0, 0x01, 0x08, 0x1e, 0x0a, 0x0d, 0x0a, 0x05, 0x05, 0x0d, 0x02, 0x01,
    0x01, 0x12, 0x04, 0xa0, 0x01, 0x08, 0x19, 0x0a, 0x0d, 0x0a, 0x05, 0x05, 0x0d, 0x02, 0x01, 0x02,
    0x12, 0x04, 0xa0, 0x01, 0x1c, 0x1d, 0x0a, 0x0c, 0x0a, 0x02, 0x05, 0x0e, 0x12, 0x06, 0xa3, 0x01,
    0x00, 0xa7, 0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x05, 0x0e, 0x01, 0x12, 0x04, 0xa3, 0x01, 0x05,
    0x11, 0x0a, 0x0c, 0x0a, 0x04, 0x05, 0x0e, 0x02, 0x00, 0x12, 0x04, 0xa4, 0x01, 0x08, 0x1f, 0x0a,
    0x0d, 0x0a, 0x05, 0x05, 0x0e, 0x02, 0x00, 0x01, 0x12, 0x04, 0xa4, 0x01, 0x08, 0x1a, 0x0a, 0x0d,
    0x0a, 0x05, 0x05, 0x0e, 0x02, 0x00, 0x02, 0x12, 0x04, 0xa4, 0x01, 0x1d, 0x1e, 0x0a, 0x0c, 0x0a,
    0x04, 0x05, 0x0e, 0x02, 0x01, 0x12, 0x04, 0xa5, 0x01, 0x08, 0x2c, 0x0a, 0x0d, 0x0a, 0x05, 0x05,
    0x0e, 0x02, 0x01, 0x01, 0x12, 0x04, 0xa5, 0x01, 0x08, 0x27, 0x0a, 0x0d, 0x0a, 0x05, 0x05, 0x0e,
    0x02, 0x01, 0x02, 0x12, 0x04, 0xa5, 0x01, 0x2a, 0x2b, 0x0a, 0x0c, 0x0a, 0x04, 0x05, 0x0e, 0x02,
    0x02, 0x12, 0x04, 0xa6, 0x01, 0x08, 0x20, 0x0a, 0x0d, 0x0a, 0x05, 0x05, 0x0e, 0x02, 0x02, 0x01,
    0x12, 0x04, 0xa6, 0x01, 0x08, 0x1b, 0x0a, 0x0d, 0x0a, 0x05, 0x05, 0x0e, 0x02, 0x02, 0x02, 0x12,
    0x04, 0xa6, 0x01, 0x1e, 0x1f, 0x0a, 0x0c, 0x0a, 0x02, 0x05, 0x0f, 0x12, 0x06, 0xa9, 0x01, 0x00,
    0xad, 0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x05, 0x0f, 0x01, 0x12, 0x04, 0xa9, 0x01, 0x05, 0x11,
    0x0a, 0x0c, 0x0a, 0x04, 0x05, 0x0f, 0x02, 0x00, 0x12, 0x04, 0xaa, 0x01, 0x08, 0x1f, 0x0a, 0x0d,
    0x0a, 0x05, 0x05, 0x0f, 0x02, 0x00, 0x01, 0x12, 0x04, 0xaa, 0x01, 0x08, 0x1a, 0x0a, 0x0d, 0x0a,
    0x05, 0x05, 0x0f, 0x02, 0x00, 0x02, 0x12, 0x04, 0xaa, 0x01, 0x1d, 0x1e, 0x0a, 0x0c, 0x0a, 0x04,
    0x05, 0x0f, 0x02, 0x01, 0x12, 0x04, 0xab, 0x01, 0x08, 0x1f, 0x0a, 0x0d, 0x0a, 0x05, 0x05, 0x0f,
    0x02, 0x01, 0x01, 0x12, 0x04, 0xab, 0x01, 0x08, 0x1a, 0x0a, 0x0d, 0x0a, 0x05, 0x05, 0x0f, 0x02,
    0x01, 0x02, 0x12, 0x04, 0xab, 0x01, 0x1d, 0x1e, 0x0a, 0x0c, 0x0a, 0x04, 0x05, 0x0f, 0x02, 0x02,
    0x12, 0x04, 0xac, 0x01, 0x08, 0x20, 0x0a, 0x0d, 0x0a, 0x05, 0x05, 0x0f, 0x02, 0x02, 0x01, 0x12,
    0x04, 0xac, 0x01, 0x08, 0x1b, 0x0a, 0x0d, 0x0a, 0x05, 0x05, 0x0f, 0x02, 0x02, 0x02, 0x12, 0x04,
    0xac, 0x01, 0x1e, 0x1f, 0x0a, 0x0c, 0x0a, 0x02, 0x05, 0x10, 0x12, 0x06, 0xaf, 0x01, 0x00, 0xb2,
    0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x05, 0x10, 0x01, 0x12, 0x04, 0xaf, 0x01, 0x05, 0x18, 0x0a,
    0x0c, 0x0a, 0x04, 0x05, 0x10, 0x02, 0x00, 0x12, 0x04, 0xb0, 0x01, 0x08, 0x26, 0x0a, 0x0d, 0x0a,
    0x05, 0x05, 0x10, 0x02, 0x00, 0x01, 0x12, 0x04, 0xb0, 0x01, 0x08, 0x21, 0x0a, 0x0d, 0x0a, 0x05,
    0x05, 0x10, 0x02, 0x00, 0x02, 0x12, 0x04, 0xb0, 0x01, 0x24, 0x25, 0x0a, 0x0c, 0x0a, 0x04, 0x05,
    0x10, 0x02, 0x01, 0x12, 0x04, 0xb1, 0x01, 0x08, 0x24, 0x0a, 0x0d, 0x0a, 0x05, 0x05, 0x10, 0x02,
    0x01, 0x01, 0x12, 0x04, 0xb1, 0x01, 0x08, 0x1f, 0x0a, 0x0d, 0x0a, 0x05, 0x05, 0x10, 0x02, 0x01,
    0x02, 0x12, 0x04, 0xb1, 0x01, 0x22, 0x23, 0x0a, 0x0c, 0x0a, 0x02, 0x05, 0x11, 0x12, 0x06, 0xb4,
    0x01, 0x00, 0xc8, 0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x05, 0x11, 0x01, 0x12, 0x04, 0xb4, 0x01,
    0x05, 0x16, 0x0a, 0x0c, 0x0a, 0x04, 0x05, 0x11, 0x02, 0x00, 0x12, 0x04, 0xb5, 0x01, 0x08, 0x25,
    0x0a, 0x0d, 0x0a, 0x05, 0x05, 0x11, 0x02, 0x00, 0x01, 0x12, 0x04, 0xb5, 0x01, 0x08, 0x20, 0x0a,
    0x0d, 0x0a, 0x05, 0x05, 0x11, 0x02, 0x00, 0x02, 0x12, 0x04, 0xb5, 0x01, 0x23, 0x24, 0x0a, 0x0c,
    0x0a, 0x04, 0x05, 0x11, 0x02, 0x01, 0x12, 0x04, 0xb6, 0x01, 0x08, 0x24, 0x0a, 0x0d, 0x0a, 0x05,
    0x05, 0x11, 0x02, 0x01, 0x01, 0x12, 0x04, 0xb6, 0x01, 0x08, 0x1f, 0x0a, 0x0d, 0x0a, 0x05, 0x05,
    0x11, 0x02, 0x01, 0x02, 0x12, 0x04, 0xb6, 0x01, 0x22, 0x23, 0x0a, 0x0c, 0x0a, 0x04, 0x05, 0x11,
    0x02, 0x02, 0x12, 0x04, 0xb7, 0x01, 0x08, 0x24, 0x0a, 0x0d, 0x0a, 0x05, 0x05, 0x11, 0x02, 0x02,
    0x01, 0x12, 0x04, 0xb7, 0x01, 0x08, 0x1f, 0x0a, 0x0d, 0x0a, 0x05, 0x05, 0x11, 0x02, 0x02, 0x02,
    0x12, 0x04, 0xb7, 0x01, 0x22, 0x23, 0x0a, 0x0c, 0x0a, 0x04, 0x05, 0x11, 0x02, 0x03, 0x12, 0x04,
    0xb8, 0x01, 0x08, 0x26, 0x0a, 0x0d, 0x0a, 0x05, 0x05, 0x11, 0x02, 0x03, 0x01, 0x12, 0x04, 0xb8,
    0x01, 0x08, 0x21, 0x0a, 0x0d, 0x0a, 0x05, 0x05, 0x11, 0x02, 0x03, 0x02, 0x12, 0x04, 0xb8, 0x01,
    0x24, 0x25, 0x0a, 0x0c, 0x0a, 0x04, 0x05, 0x11, 0x02, 0x04, 0x12, 0x04, 0xb9, 0x01, 0x08, 0x27,
    0x0a, 0x0d, 0x0a, 0x05, 0x05, 0x11, 0x02, 0x04, 0x01, 0x12, 0x04, 0xb9, 0x01, 0x08, 0x22, 0x0a,
    0x0d, 0x0a, 0x05, 0x05, 0x11, 0x02, 0x04, 0x02, 0x12, 0x04, 0xb9, 0x01, 0x25, 0x26, 0x0a, 0x0c,
    0x0a, 0x04, 0x05, 0x11, 0x02, 0x05, 0x12, 0x04, 0xba, 0x01, 0x08, 0x25, 0x0a, 0x0d, 0x0a, 0x05,
    0x05, 0x11, 0x02, 0x05, 0x01, 0x12, 0x04, 0xba, 0x01, 0x08, 0x20, 0x0a, 0x0d, 0x0a, 0x05, 0x05,
    0x11, 0x02, 0x05, 0x02, 0x12, 0x04, 0xba, 0x01, 0x23, 0x24, 0x0a, 0x0c, 0x0a, 0x04, 0x05, 0x11,
    0x02, 0x06, 0x12, 0x04, 0xbb, 0x01, 0x08, 0x2c, 0x0a, 0x0d, 0x0a, 0x05, 0x05, 0x11, 0x02, 0x06,
    0x01, 0x12, 0x04, 0xbb, 0x01, 0x08, 0x27, 0x0a, 0x0d, 0x0a, 0x05, 0x05, 0x11, 0x02, 0x06, 0x02,
    0x12, 0x04, 0xbb, 0x01, 0x2a, 0x2b, 0x0a, 0x0c, 0x0a, 0x04, 0x05, 0x11, 0x02, 0x07, 0x12, 0x04,
    0xbc, 0x01, 0x08, 0x2a, 0x0a, 0x0d, 0x0a, 0x05, 0x05, 0x11, 0x02, 0x07, 0x01, 0x12, 0x04, 0xbc,
    0x01, 0x08, 0x25, 0x0a, 0x0d, 0x0a, 0x05, 0x05, 0x11, 0x02, 0x07, 0x02, 0x12, 0x04, 0xbc, 0x01,
    0x28, 0x29, 0x0a, 0x0c, 0x0a, 0x04, 0x05, 0x11, 0x02, 0x08, 0x12, 0x04, 0xbd, 0x01, 0x08, 0x2c,
    0x0a, 0x0d, 0x0a, 0x05, 0x05, 0x11, 0x02, 0x08, 0x01, 0x12, 0x04, 0xbd, 0x01, 0x08, 0x27, 0x0a,
    0x0d, 0x0a, 0x05, 0x05, 0x11, 0x02, 0x08, 0x02, 0x12, 0x04, 0xbd, 0x01, 0x2a, 0x2b, 0x0a, 0x0c,
    0x0a, 0x04, 0x05, 0x11, 0x02, 0x09, 0x12, 0x04, 0xbe, 0x01, 0x08, 0x2a, 0x0a, 0x0d, 0x0a, 0x05,
    0x05, 0x11, 0x02, 0x09, 0x01, 0x12, 0x04, 0xbe, 0x01, 0x08, 0x25, 0x0a, 0x0d, 0x0a, 0x05, 0x05,
    0x11, 0x02, 0x09, 0x02, 0x12, 0x04, 0xbe, 0x01, 0x28, 0x29, 0x0a, 0x0c, 0x0a, 0x04, 0x05, 0x11,
    0x02, 0x0a, 0x12, 0x04, 0xbf, 0x01, 0x08, 0x2c, 0x0a, 0x0d, 0x0a, 0x05, 0x05, 0x11, 0x02, 0x0a,
    0x01, 0x12, 0x04, 0xbf, 0x01, 0x08, 0x26, 0x0a, 0x0d, 0x0a, 0x05, 0x05, 0x11, 0x02, 0x0a, 0x02,
    0x12, 0x04, 0xbf, 0x01, 0x29, 0x2b, 0x0a, 0x0c, 0x0a, 0x04, 0x05, 0x11, 0x02, 0x0b, 0x12, 0x04,
    0xc0, 0x01, 0x08, 0x2a, 0x0a, 0x0d, 0x0a, 0x05, 0x05, 0x11, 0x02, 0x0b, 0x01, 0x12, 0x04, 0xc0,
    0x01, 0x08, 0x24, 0x0a, 0x0d, 0x0a, 0x05, 0x05, 0x11, 0x02, 0x0b, 0x02, 0x12, 0x04, 0xc0, 0x01,
    0x27, 0x29, 0x0a, 0x0c, 0x0a, 0x04, 0x05, 0x11, 0x02, 0x0c, 0x12, 0x04, 0xc1, 0x01, 0x08, 0x25,
    0x0a, 0x0d, 0x0a, 0x05, 0x05, 0x11, 0x02, 0x0c, 0x01, 0x12, 0x04, 0xc1, 0x01, 0x08, 0x1f, 0x0a,
    0x0d, 0x0a, 0x05, 0x05, 0x11, 0x02, 0x0c, 0x02, 0x12, 0x04, 0xc1, 0x01, 0x22, 0x24, 0x0a, 0x0c,
    0x0a, 0x04, 0x05, 0x11, 0x02, 0x0d, 0x12, 0x04, 0xc2, 0x01, 0x08, 0x25, 0x0a, 0x0d, 0x0a, 0x05,
    0x05, 0x11, 0x02, 0x0d, 0x01, 0x12, 0x04, 0xc2, 0x01, 0x08, 0x1f, 0x0a, 0x0d, 0x0a, 0x05, 0x05,
    0x11, 0x02, 0x0d, 0x02, 0x12, 0x04, 0xc2, 0x01, 0x22, 0x24, 0x0a, 0x0c, 0x0a, 0x04, 0x05, 0x11,
    0x02, 0x0e, 0x12, 0x04, 0xc3, 0x01, 0x08, 0x2c, 0x0a, 0x0d, 0x0a, 0x05, 0x05, 0x11, 0x02, 0x0e,
    0x01, 0x12, 0x04, 0xc3, 0x01, 0x08, 0x26, 0x0a, 0x0d, 0x0a, 0x05, 0x05, 0x11, 0x02, 0x0e, 0x02,
    0x12, 0x04, 0xc3, 0x01, 0x29, 0x2b, 0x0a, 0x0c, 0x0a, 0x04, 0x05, 0x11, 0x02, 0x0f, 0x12, 0x04,
    0xc4, 0x01, 0x08, 0x2a, 0x0a, 0x0d, 0x0a, 0x05, 0x05, 0x11, 0x02, 0x0f, 0x01, 0x12, 0x04, 0xc4,
    0x01, 0x08, 0x24, 0x0a, 0x0d, 0x0a, 0x05, 0x05, 0x11, 0x02, 0x0f, 0x02, 0x12, 0x04, 0xc4, 0x01,
    0x27, 0x29, 0x0a, 0x0c, 0x0a, 0x04, 0x05, 0x11, 0x02, 0x10, 0x12, 0x04, 0xc5, 0x01, 0x08, 0x2c,
    0x0a, 0x0d, 0x0a, 0x05, 0x05, 0x11, 0x02, 0x10, 0x01, 0x12, 0x04, 0xc5, 0x01, 0x08, 0x26, 0x0a,
    0x0d, 0x0a, 0x05, 0x05, 0x11, 0x02, 0x10, 0x02, 0x12, 0x04, 0xc5, 0x01, 0x29, 0x2b, 0x0a, 0x0c,
    0x0a, 0x04, 0x05, 0x11, 0x02, 0x11, 0x12, 0x04, 0xc6, 0x01, 0x08, 0x2a, 0x0a, 0x0d, 0x0a, 0x05,
    0x05, 0x11, 0x02, 0x11, 0x01, 0x12, 0x04, 0xc6, 0x01, 0x08, 0x24, 0x0a, 0x0d, 0x0a, 0x05, 0x05,
    0x11, 0x02, 0x11, 0x02, 0x12, 0x04, 0xc6, 0x01, 0x27, 0x29, 0x0a, 0x0c, 0x0a, 0x04, 0x05, 0x11,
    0x02, 0x12, 0x12, 0x04, 0xc7, 0x01, 0x08, 0x29, 0x0a, 0x0d, 0x0a, 0x05, 0x05, 0x11, 0x02, 0x12,
    0x01, 0x12, 0x04, 0xc7, 0x01, 0x08, 0x23, 0x0a, 0x0d, 0x0a, 0x05, 0x05, 0x11, 0x02, 0x12, 0x02,
    0x12, 0x04, 0xc7, 0x01, 0x26, 0x28, 0x0a, 0x0c, 0x0a, 0x02, 0x05, 0x12, 0x12, 0x06, 0xca, 0x01,
    0x00, 0xd3, 0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x05, 0x12, 0x01, 0x12, 0x04, 0xca, 0x01, 0x05,
    0x17, 0x0a, 0x0c, 0x0a, 0x04, 0x05, 0x12, 0x02, 0x00, 0x12, 0x04, 0xcb, 0x01, 0x08, 0x28, 0x0a,
    0x0d, 0x0a, 0x05, 0x05, 0x12, 0x02, 0x00, 0x01, 0x12, 0x04, 0xcb, 0x01, 0x08, 0x23, 0x0a, 0x0d,
    0x0a, 0x05, 0x05, 0x12, 0x02, 0x00, 0x02, 0x12, 0x04, 0xcb, 0x01, 0x26, 0x27, 0x0a, 0x0c, 0x0a,
    0x04, 0x05, 0x12, 0x02, 0x01, 0x12, 0x04, 0xcc, 0x01, 0x08, 0x2a, 0x0a, 0x0d, 0x0a, 0x05, 0x05,
    0x12, 0x02, 0x01, 0x01, 0x12, 0x04, 0xcc, 0x01, 0x08, 0x25, 0x0a, 0x0d, 0x0a, 0x05, 0x05, 0x12,
    0x02, 0x01, 0x02, 0x12, 0x04, 0xcc, 0x01, 0x28, 0x29, 0x0a, 0x0c, 0x0a, 0x04, 0x05, 0x12, 0x02,
    0x02, 0x12, 0x04, 0xcd, 0x01, 0x08, 0x33, 0x0a, 0x0d, 0x0a, 0x05, 0x05, 0x12, 0x02, 0x02, 0x01,
    0x12, 0x04, 0xcd, 0x01, 0x08, 0x2e, 0x0a, 0x0d, 0x0a, 0x05, 0x05, 0x12, 0x02, 0x02, 0x02, 0x12,
    0x04, 0xcd, 0x01, 0x31, 0x32, 0x0a, 0x0c, 0x0a, 0x04, 0x05, 0x12, 0x02, 0x03, 0x12, 0x04, 0xce,
    0x01, 0x08, 0x33, 0x0a, 0x0d, 0x0a, 0x05, 0x05, 0x12, 0x02, 0x03, 0x01, 0x12, 0x04, 0xce, 0x01,
    0x08, 0x2e, 0x0a, 0x0d, 0x0a, 0x05, 0x05, 0x12, 0x02, 0x03, 0x02, 0x12, 0x04, 0xce, 0x01, 0x31,
    0x32, 0x0a, 0x0c, 0x0a, 0x04, 0x05, 0x12, 0x02, 0x04, 0x12, 0x04, 0xcf, 0x01, 0x08, 0x32, 0x0a,
    0x0d, 0x0a, 0x05, 0x05, 0x12, 0x02, 0x04, 0x01, 0x12, 0x04, 0xcf, 0x01, 0x08, 0x2d, 0x0a, 0x0d,
    0x0a, 0x05, 0x05, 0x12, 0x02, 0x04, 0x02, 0x12, 0x04, 0xcf, 0x01, 0x30, 0x31, 0x0a, 0x0c, 0x0a,
    0x04, 0x05, 0x12, 0x02, 0x05, 0x12, 0x04, 0xd0, 0x01, 0x08, 0x35, 0x0a, 0x0d, 0x0a, 0x05, 0x05,
    0x12, 0x02, 0x05, 0x01, 0x12, 0x04, 0xd0, 0x01, 0x08, 0x30, 0x0a, 0x0d, 0x0a, 0x05, 0x05, 0x12,
    0x02, 0x05, 0x02, 0x12, 0x04, 0xd0, 0x01, 0x33, 0x34, 0x0a, 0x0c, 0x0a, 0x04, 0x05, 0x12, 0x02,
    0x06, 0x12, 0x04, 0xd1, 0x01, 0x08, 0x2c, 0x0a, 0x0d, 0x0a, 0x05, 0x05, 0x12, 0x02, 0x06, 0x01,
    0x12, 0x04, 0xd1, 0x01, 0x08, 0x27, 0x0a, 0x0d, 0x0a, 0x05, 0x05, 0x12, 0x02, 0x06, 0x02, 0x12,
    0x04, 0xd1, 0x01, 0x2a, 0x2b, 0x0a, 0x0c, 0x0a, 0x04, 0x05, 0x12, 0x02, 0x07, 0x12, 0x04, 0xd2,
    0x01, 0x08, 0x2d, 0x0a, 0x0d, 0x0a, 0x05, 0x05, 0x12, 0x02, 0x07, 0x01, 0x12, 0x04, 0xd2, 0x01,
    0x08, 0x28, 0x0a, 0x0d, 0x0a, 0x05, 0x05, 0x12, 0x02, 0x07, 0x02, 0x12, 0x04, 0xd2, 0x01, 0x2b,
    0x2c, 0x0a, 0x0c, 0x0a, 0x02, 0x05, 0x13, 0x12, 0x06, 0xd5, 0x01, 0x00, 0xe9, 0x01, 0x01, 0x0a,
    0x0b, 0x0a, 0x03, 0x05, 0x13, 0x01, 0x12, 0x04, 0xd5, 0x01, 0x05, 0x1a, 0x0a, 0x0c, 0x0a, 0x04,
    0x05, 0x13, 0x02, 0x00, 0x12, 0x04, 0xd6, 0x01, 0x08, 0x1c, 0x0a, 0x0d, 0x0a, 0x05, 0x05, 0x13,
    0x02, 0x00, 0x01, 0x12, 0x04, 0xd6, 0x01, 0x08, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x05, 0x13, 0x02,
    0x00, 0x02, 0x12, 0x04, 0xd6, 0x01, 0x1a, 0x1b, 0x0a, 0x0c, 0x0a, 0x04, 0x05, 0x13, 0x02, 0x01,
    0x12, 0x04, 0xd7, 0x01, 0x08, 0x2a, 0x0a, 0x0d, 0x0a, 0x05, 0x05, 0x13, 0x02, 0x01, 0x01, 0x12,
    0x04, 0xd7, 0x01, 0x08, 0x25, 0x0a, 0x0d, 0x0a, 0x05, 0x05, 0x13, 0x02, 0x01, 0x02, 0x12, 0x04,
    0xd7, 0x01, 0x28, 0x29, 0x0a, 0x0c, 0x0a, 0x04, 0x05, 0x13, 0x02, 0x02, 0x12, 0x04, 0xd8, 0x01,
    0x08, 0x2a, 0x0a, 0x0d, 0x0a, 0x05, 0x05, 0x13, 0x02, 0x02, 0x01, 0x12, 0x04, 0xd8, 0x01, 0x08,
    0x25, 0x0a, 0x0d, 0x0a, 0x05, 0x05, 0x13, 0x02, 0x02, 0x02, 0x12, 0x04, 0xd8, 0x01, 0x28, 0x29,
    0x0a, 0x0c, 0x0a, 0x04, 0x05, 0x13, 0x02, 0x03, 0x12, 0x04, 0xd9, 0x01, 0x08, 0x29, 0x0a, 0x0d,
    0x0a, 0x05, 0x05, 0x13, 0x02, 0x03, 0x01, 0x12, 0x04, 0xd9, 0x01, 0x08, 0x24, 0x0a, 0x0d, 0x0a,
    0x05, 0x05, 0x13, 0x02, 0x03, 0x02, 0x12, 0x04, 0xd9, 0x01, 0x27, 0x28, 0x0a, 0x0c, 0x0a, 0x04,
    0x05, 0x13, 0x02, 0x04, 0x12, 0x04, 0xda, 0x01, 0x08, 0x28, 0x0a, 0x0d, 0x0a, 0x05, 0x05, 0x13,
    0x02, 0x04, 0x01, 0x12, 0x04, 0xda, 0x01, 0x08, 0x23, 0x0a, 0x0d, 0x0a, 0x05, 0x05, 0x13, 0x02,
    0x04, 0x02, 0x12, 0x04, 0xda, 0x01, 0x26, 0x27, 0x0a, 0x0c, 0x0a, 0x04, 0x05, 0x13, 0x02, 0x05,
    0x12, 0x04, 0xdb, 0x01, 0x08, 0x29, 0x0a, 0x0d, 0x0a, 0x05, 0x05, 0x13, 0x02, 0x05, 0x01, 0x12,
    0x04, 0xdb, 0x01, 0x08, 0x24, 0x0a, 0x0d, 0x0a, 0x05, 0x05, 0x13, 0x02, 0x05, 0x02, 0x12, 0x04,
    0xdb, 0x01, 0x27, 0x28, 0x0a, 0x0c, 0x0a, 0x04, 0x05, 0x13, 0x02, 0x06, 0x12, 0x04, 0xdc, 0x01,
    0x08, 0x2a, 0x0a, 0x0d, 0x0a, 0x05, 0x05, 0x13, 0x02, 0x06, 0x01, 0x12, 0x04, 0xdc, 0x01, 0x08,
    0x25, 0x0a, 0x0d, 0x0a, 0x05, 0x05, 0x13, 0x02, 0x06, 0x02, 0x12, 0x04, 0xdc, 0x01, 0x28, 0x29,
    0x0a, 0x0c, 0x0a, 0x04, 0x05, 0x13, 0x02, 0x07, 0x12, 0x04, 0xdd, 0x01, 0x08, 0x29, 0x0a, 0x0d,
    0x0a, 0x05, 0x05, 0x13, 0x02, 0x07, 0x01, 0x12, 0x04, 0xdd, 0x01, 0x08, 0x24, 0x0a, 0x0d, 0x0a,
    0x05, 0x05, 0x13, 0x02, 0x07, 0x02, 0x12, 0x04, 0xdd, 0x01, 0x27, 0x28, 0x0a, 0x0c, 0x0a, 0x04,
    0x05, 0x13, 0x02, 0x08, 0x12, 0x04, 0xde, 0x01, 0x08, 0x2a, 0x0a, 0x0d, 0x0a, 0x05, 0x05, 0x13,
    0x02, 0x08, 0x01, 0x12, 0x04, 0xde, 0x01, 0x08, 0x25, 0x0a, 0x0d, 0x0a, 0x05, 0x05, 0x13, 0x02,
    0x08, 0x02, 0x12, 0x04, 0xde, 0x01, 0x28, 0x29, 0x0a, 0x0c, 0x0a, 0x04, 0x05, 0x13, 0x02, 0x09,
    0x12, 0x04, 0xdf, 0x01, 0x08, 0x29, 0x0a, 0x0d, 0x0a, 0x05, 0x05, 0x13, 0x02, 0x09, 0x01, 0x12,
    0x04, 0xdf, 0x01, 0x08, 0x24, 0x0a, 0x0d, 0x0a, 0x05, 0x05, 0x13, 0x02, 0x09, 0x02, 0x12, 0x04,
    0xdf, 0x01, 0x27, 0x28, 0x0a, 0x0c, 0x0a, 0x04, 0x05, 0x13, 0x02, 0x0a, 0x12, 0x04, 0xe0, 0x01,
    0x08, 0x29, 0x0a, 0x0d, 0x0a, 0x05, 0x05, 0x13, 0x02, 0x0a, 0x01, 0x12, 0x04, 0xe0, 0x01, 0x08,
    0x23, 0x0a, 0x0d, 0x0a, 0x05, 0x05, 0x13, 0x02, 0x0a, 0x02, 0x12, 0x04, 0xe0, 0x01, 0x26, 0x28,
    0x0a, 0x0c, 0x0a, 0x04, 0x05, 0x13, 0x02, 0x0b, 0x12, 0x04, 0xe1, 0x01, 0x08, 0x28, 0x0a, 0x0d,
    0x0a, 0x05, 0x05, 0x13, 0x02, 0x0b, 0x01, 0x12, 0x04, 0xe1, 0x01, 0x08, 0x22, 0x0a, 0x0d, 0x0a,
    0x05, 0x05, 0x13, 0x02, 0x0b, 0x02, 0x12, 0x04, 0xe1, 0x01, 0x25, 0x27, 0x0a, 0x0c, 0x0a, 0x04,
    0x05, 0x13, 0x02, 0x0c, 0x12, 0x04, 0xe2, 0x01, 0x08, 0x27, 0x0a, 0x0d, 0x0a, 0x05, 0x05, 0x13,
    0x02, 0x0c, 0x01, 0x12, 0x04, 0xe2, 0x01, 0x08, 0x21, 0x0a, 0x0d, 0x0a, 0x05, 0x05, 0x13, 0x02,
    0x0c, 0x02, 0x12, 0x04, 0xe2, 0x01, 0x24, 0x26, 0x0a, 0x0c, 0x0a, 0x04, 0x05, 0x13, 0x02, 0x0d,
    0x12, 0x04, 0xe3, 0x01, 0x08, 0x2c, 0x0a, 0x0d, 0x0a, 0x05, 0x05, 0x13, 0x02, 0x0d, 0x01, 0x12,
    0x04, 0xe3, 0x01, 0x08, 0x26, 0x0a, 0x0d, 0x0a, 0x05, 0x05, 0x13, 0x02, 0x0d, 0x02, 0x12, 0x04,
    0xe3, 0x01, 0x29, 0x2b, 0x0a, 0x0c, 0x0a, 0x04, 0x05, 0x13, 0x02, 0x0e, 0x12, 0x04, 0xe4, 0x01,
    0x08, 0x24, 0x0a, 0x0d, 0x0a, 0x05, 0x05, 0x13, 0x02, 0x0e, 0x01, 0x12, 0x04, 0xe4, 0x01, 0x08,
    0x1e, 0x0a, 0x0d, 0x0a, 0x05, 0x05, 0x13, 0x02, 0x0e, 0x02, 0x12, 0x04, 0xe4, 0x01, 0x21, 0x23,
    0x0a, 0x0c, 0x0a, 0x04, 0x05, 0x13, 0x02, 0x0f, 0x12, 0x04, 0xe5, 0x01, 0x08, 0x31, 0x0a, 0x0d,
    0x0a, 0x05, 0x05, 0x13, 0x02, 0x0f, 0x01, 0x12, 0x04, 0xe5, 0x01, 0x08, 0x2b, 0x0a, 0x0d, 0x0a,
    0x05, 0x05, 0x13, 0x02, 0x0f, 0x02, 0x12, 0x04, 0xe5, 0x01, 0x2e, 0x30, 0x0a, 0x0c, 0x0a, 0x04,
    0x05, 0x13, 0x02, 0x10, 0x12, 0x04, 0xe6, 0x01, 0x08, 0x31, 0x0a, 0x0d, 0x0a, 0x05, 0x05, 0x13,
    0x02, 0x10, 0x01, 0x12, 0x04, 0xe6, 0x01, 0x08, 0x2b, 0x0a, 0x0d, 0x0a, 0x05, 0x05, 0x13, 0x02,
    0x10, 0x02, 0x12, 0x04, 0xe6, 0x01, 0x2e, 0x30, 0x0a, 0x0c, 0x0a, 0x04, 0x05, 0x13, 0x02, 0x11,
    0x12, 0x04, 0xe7, 0x01, 0x08, 0x31, 0x0a, 0x0d, 0x0a, 0x05, 0x05, 0x13, 0x02, 0x11, 0x01, 0x12,
    0x04, 0xe7, 0x01, 0x08, 0x2b, 0x0a, 0x0d, 0x0a, 0x05, 0x05, 0x13, 0x02, 0x11, 0x02, 0x12, 0x04,
    0xe7, 0x01, 0x2e, 0x30, 0x0a, 0x0c, 0x0a, 0x04, 0x05, 0x13, 0x02, 0x12, 0x12, 0x04, 0xe8, 0x01,
    0x08, 0x2e, 0x0a, 0x0d, 0x0a, 0x05, 0x05, 0x13, 0x02, 0x12, 0x01, 0x12, 0x04, 0xe8, 0x01, 0x08,
    0x28, 0x0a, 0x0d, 0x0a, 0x05, 0x05, 0x13, 0x02, 0x12, 0x02, 0x12, 0x04, 0xe8, 0x01, 0x2b, 0x2d,
    0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x00, 0x12, 0x06, 0xeb, 0x01, 0x00, 0xed, 0x01, 0x01, 0x0a, 0x0b,
    0x0a, 0x03, 0x04, 0x00, 0x01, 0x12, 0x04, 0xeb, 0x01, 0x08, 0x19, 0x0a, 0x0c, 0x0a, 0x04, 0x04,
    0x00, 0x02, 0x00, 0x12, 0x04, 0xec, 0x01, 0x08, 0x21, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x00, 0x04, 0x12, 0x04, 0xec, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00,
    0x05, 0x12, 0x04, 0xec, 0x01, 0x11, 0x16, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x01,
    0x12, 0x04, 0xec, 0x01, 0x17, 0x1c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x03, 0x12,
    0x04, 0xec, 0x01, 0x1f, 0x20, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x01, 0x12, 0x06, 0xef, 0x01, 0x00,
    0xf1, 0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x01, 0x01, 0x12, 0x04, 0xef, 0x01, 0x08, 0x1a,
    0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x00, 0x12, 0x04, 0xf0, 0x01, 0x08, 0x21, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x04, 0x12, 0x04, 0xf0, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x01, 0x02, 0x00, 0x05, 0x12, 0x04, 0xf0, 0x01, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x01, 0x02, 0x00, 0x01, 0x12, 0x04, 0xf0, 0x01, 0x18, 0x1c, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x01, 0x02, 0x00, 0x03, 0x12, 0x04, 0xf0, 0x01, 0x1f, 0x20, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x02,
    0x12, 0x06, 0xf3, 0x01, 0x00, 0xf6, 0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x02, 0x01, 0x12,
    0x04, 0xf3, 0x01, 0x08, 0x21, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x00, 0x12, 0x04, 0xf4,
    0x01, 0x08, 0x21, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x04, 0x12, 0x04, 0xf4, 0x01,
    0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x05, 0x12, 0x04, 0xf4, 0x01, 0x11,
    0x16, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x01, 0x12, 0x04, 0xf4, 0x01, 0x17, 0x1c,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x03, 0x12, 0x04, 0xf4, 0x01, 0x1f, 0x20, 0x0a,
    0x0c, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x01, 0x12, 0x04, 0xf5, 0x01, 0x08, 0x4e, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x02, 0x02, 0x01, 0x04, 0x12, 0x04, 0xf5, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x02, 0x02, 0x01, 0x06, 0x12, 0x04, 0xf5, 0x01, 0x11, 0x20, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x02, 0x02, 0x01, 0x01, 0x12, 0x04, 0xf5, 0x01, 0x21, 0x28, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x02,
    0x02, 0x01, 0x03, 0x12, 0x04, 0xf5, 0x01, 0x2b, 0x2c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x02, 0x02,
    0x01, 0x08, 0x12, 0x04, 0xf5, 0x01, 0x2d, 0x4d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01,
    0x07, 0x12, 0x04, 0xf5, 0x01, 0x38, 0x4c, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x03, 0x12, 0x06, 0xf8,
    0x01, 0x00, 0x80, 0x02, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x03, 0x01, 0x12, 0x04, 0xf8, 0x01,
    0x08, 0x22, 0x0a, 0x0e, 0x0a, 0x04, 0x04, 0x03, 0x04, 0x00, 0x12, 0x06, 0xf9, 0x01, 0x08, 0xfc,
    0x01, 0x09, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x03, 0x04, 0x00, 0x01, 0x12, 0x04, 0xf9, 0x01, 0x0d,
    0x21, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x03, 0x04, 0x00, 0x02, 0x00, 0x12, 0x04, 0xfa, 0x01, 0x10,
    0x1e, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x03, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x04, 0xfa, 0x01,
    0x10, 0x19, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x03, 0x04, 0x00, 0x02, 0x00, 0x02, 0x12, 0x04, 0xfa,
    0x01, 0x1c, 0x1d, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x03, 0x04, 0x00, 0x02, 0x01, 0x12, 0x04, 0xfb,
    0x01, 0x10, 0x1b, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x03, 0x04, 0x00, 0x02, 0x01, 0x01, 0x12, 0x04,
    0xfb, 0x01, 0x10, 0x16, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x03, 0x04, 0x00, 0x02, 0x01, 0x02, 0x12,
    0x04, 0xfb, 0x01, 0x19, 0x1a, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x00, 0x12, 0x04, 0xfe,
    0x01, 0x08, 0x63, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x04, 0x12, 0x04, 0xfe, 0x01,
    0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x06, 0x12, 0x04, 0xfe, 0x01, 0x11,
    0x41, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x01, 0x12, 0x04, 0xfe, 0x01, 0x42, 0x48,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x03, 0x12, 0x04, 0xfe, 0x01, 0x4b, 0x4c, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x08, 0x12, 0x04, 0xfe, 0x01, 0x4d, 0x62, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x07, 0x12, 0x04, 0xfe, 0x01, 0x58, 0x61, 0x0a, 0x0c, 0x0a,
    0x04, 0x04, 0x03, 0x02, 0x01, 0x12, 0x04, 0xff, 0x01, 0x08, 0x4e, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x03, 0x02, 0x01, 0x04, 0x12, 0x04, 0xff, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x03,
    0x02, 0x01, 0x06, 0x12, 0x04, 0xff, 0x01, 0x11, 0x20, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x03, 0x02,
    0x01, 0x01, 0x12, 0x04, 0xff, 0x01, 0x21, 0x28, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x01,
    0x03, 0x12, 0x04, 0xff, 0x01, 0x2b, 0x2c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x01, 0x08,
    0x12, 0x04, 0xff, 0x01, 0x2d, 0x4d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x01, 0x07, 0x12,
    0x04, 0xff, 0x01, 0x38, 0x4c, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x04, 0x12, 0x06, 0x82, 0x02, 0x00,
    0x88, 0x02, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x04, 0x01, 0x12, 0x04, 0x82, 0x02, 0x08, 0x18,
    0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x04, 0x02, 0x00, 0x12, 0x04, 0x83, 0x02, 0x08, 0x22, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x04, 0x12, 0x04, 0x83, 0x02, 0x08, 0x10, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x04, 0x02, 0x00, 0x05, 0x12, 0x04, 0x83, 0x02, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x04, 0x02, 0x00, 0x01, 0x12, 0x04, 0x83, 0x02, 0x18, 0x1d, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x04, 0x02, 0x00, 0x03, 0x12, 0x04, 0x83, 0x02, 0x20, 0x21, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x04,
    0x02, 0x01, 0x12, 0x04, 0x84, 0x02, 0x08, 0x23, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x01,
    0x04, 0x12, 0x04, 0x84, 0x02, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x01, 0x05,
    0x12, 0x04, 0x84, 0x02, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x01, 0x01, 0x12,
    0x04, 0x84, 0x02, 0x18, 0x1e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x01, 0x03, 0x12, 0x04,
    0x84, 0x02, 0x21, 0x22, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x04, 0x02, 0x02, 0x12, 0x04, 0x85, 0x02,
    0x08, 0x29, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x02, 0x04, 0x12, 0x04, 0x85, 0x02, 0x08,
    0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x02, 0x05, 0x12, 0x04, 0x85, 0x02, 0x11, 0x17,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x02, 0x01, 0x12, 0x04, 0x85, 0x02, 0x18, 0x24, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x02, 0x03, 0x12, 0x04, 0x85, 0x02, 0x27, 0x28, 0x0a, 0x0c,
    0x0a, 0x04, 0x04, 0x04, 0x02, 0x03, 0x12, 0x04, 0x86, 0x02, 0x08, 0x33, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x04, 0x02, 0x03, 0x04, 0x12, 0x04, 0x86, 0x02, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x04, 0x02, 0x03, 0x05, 0x12, 0x04, 0x86, 0x02, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x04,
    0x02, 0x03, 0x01, 0x12, 0x04, 0x86, 0x02, 0x18, 0x2e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x04, 0x02,
    0x03, 0x03, 0x12, 0x04, 0x86, 0x02, 0x31, 0x32, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x04, 0x02, 0x04,
    0x12, 0x04, 0x87, 0x02, 0x08, 0x35, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x04, 0x04, 0x12,
    0x04, 0x87, 0x02, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x04, 0x05, 0x12, 0x04,
    0x87, 0x02, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x04, 0x01, 0x12, 0x04, 0x87,
    0x02, 0x18, 0x30, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x04, 0x03, 0x12, 0x04, 0x87, 0x02,
    0x33, 0x34, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x05, 0x12, 0x06, 0x8a, 0x02, 0x00, 0x8f, 0x02, 0x01,
    0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x05, 0x01, 0x12, 0x04, 0x8a, 0x02, 0x08, 0x19, 0x0a, 0x0c, 0x0a,
    0x04, 0x04, 0x05, 0x02, 0x00, 0x12, 0x04, 0x8b, 0x02, 0x08, 0x28, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x05, 0x02, 0x00, 0x04, 0x12, 0x04, 0x8b, 0x02, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x05,
    0x02, 0x00, 0x05, 0x12, 0x04, 0x8b, 0x02, 0x11, 0x15, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x05, 0x02,
    0x00, 0x01, 0x12, 0x04, 0x8b, 0x02, 0x16, 0x23, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00,
    0x03, 0x12, 0x04, 0x8b, 0x02, 0x26, 0x27, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x05, 0x02, 0x01, 0x12,
    0x04, 0x8c, 0x02, 0x08, 0x61, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x01, 0x04, 0x12, 0x04,
    0x8c, 0x02, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x01, 0x06, 0x12, 0x04, 0x8c,
    0x02, 0x11, 0x23, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x01, 0x01, 0x12, 0x04, 0x8c, 0x02,
    0x24, 0x38, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x01, 0x03, 0x12, 0x04, 0x8c, 0x02, 0x3b,
    0x3c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x01, 0x08, 0x12, 0x04, 0x8c, 0x02, 0x3d, 0x60,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x01, 0x07, 0x12, 0x04, 0x8c, 0x02, 0x48, 0x5f, 0x0a,
    0x0c, 0x0a, 0x04, 0x04, 0x05, 0x02, 0x02, 0x12, 0x04, 0x8d, 0x02, 0x08, 0x61, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x05, 0x02, 0x02, 0x04, 0x12, 0x04, 0x8d, 0x02, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x05, 0x02, 0x02, 0x06, 0x12, 0x04, 0x8d, 0x02, 0x11, 0x23, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x05, 0x02, 0x02, 0x01, 0x12, 0x04, 0x8d, 0x02, 0x24, 0x38, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x05,
    0x02, 0x02, 0x03, 0x12, 0x04, 0x8d, 0x02, 0x3b, 0x3c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x05, 0x02,
    0x02, 0x08, 0x12, 0x04, 0x8d, 0x02, 0x3d, 0x60, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x02,
    0x07, 0x12, 0x04, 0x8d, 0x02, 0x48, 0x5f, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x05, 0x02, 0x03, 0x12,
    0x04, 0x8e, 0x02, 0x08, 0x3d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x03, 0x04, 0x12, 0x04,
    0x8e, 0x02, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x03, 0x06, 0x12, 0x04, 0x8e,
    0x02, 0x11, 0x22, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x03, 0x01, 0x12, 0x04, 0x8e, 0x02,
    0x23, 0x38, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x03, 0x03, 0x12, 0x04, 0x8e, 0x02, 0x3b,
    0x3c, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x06, 0x12, 0x06, 0x91, 0x02, 0x00, 0x95, 0x02, 0x01, 0x0a,
    0x0b, 0x0a, 0x03, 0x04, 0x06, 0x01, 0x12, 0x04, 0x91, 0x02, 0x08, 0x1b, 0x0a, 0x0c, 0x0a, 0x04,
    0x04, 0x06, 0x02, 0x00, 0x12, 0x04, 0x92, 0x02, 0x08, 0x28, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x06,
    0x02, 0x00, 0x04, 0x12, 0x04, 0x92, 0x02, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x06, 0x02,
    0x00, 0x05, 0x12, 0x04, 0x92, 0x02, 0x11, 0x15, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x00,
    0x01, 0x12, 0x04, 0x92, 0x02, 0x16, 0x23, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x00, 0x03,
    0x12, 0x04, 0x92, 0x02, 0x26, 0x27, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x06, 0x02, 0x01, 0x12, 0x04,
    0x93, 0x02, 0x08, 0x3f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x01, 0x04, 0x12, 0x04, 0x93,
    0x02, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x01, 0x06, 0x12, 0x04, 0x93, 0x02,
    0x11, 0x23, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x01, 0x01, 0x12, 0x04, 0x93, 0x02, 0x24,
    0x3a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x01, 0x03, 0x12, 0x04, 0x93, 0x02, 0x3d, 0x3e,
    0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x06, 0x02, 0x02, 0x12, 0x04, 0x94, 0x02, 0x08, 0x3f, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x06, 0x02, 0x02, 0x04, 0x12, 0x04, 0x94, 0x02, 0x08, 0x10, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x06, 0x02, 0x02, 0x06, 0x12, 0x04, 0x94, 0x02, 0x11, 0x23, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x06, 0x02, 0x02, 0x01, 0x12, 0x04, 0x94, 0x02, 0x24, 0x3a, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x06, 0x02, 0x02, 0x03, 0x12, 0x04, 0x94, 0x02, 0x3d, 0x3e, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x07,
    0x12, 0x06, 0x97, 0x02, 0x00, 0x99, 0x02, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x07, 0x01, 0x12,
    0x04, 0x97, 0x02, 0x08, 0x20, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x07, 0x02, 0x00, 0x12, 0x04, 0x98,
    0x02, 0x08, 0x2f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x00, 0x04, 0x12, 0x04, 0x98, 0x02,
    0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x00, 0x06, 0x12, 0x04, 0x98, 0x02, 0x11,
    0x23, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x00, 0x01, 0x12, 0x04, 0x98, 0x02, 0x24, 0x2a,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x00, 0x03, 0x12, 0x04, 0x98, 0x02, 0x2d, 0x2e, 0x0a,
    0x0c, 0x0a, 0x02, 0x04, 0x08, 0x12, 0x06, 0x9b, 0x02, 0x00, 0x9c, 0x02, 0x01, 0x0a, 0x0b, 0x0a,
    0x03, 0x04, 0x08, 0x01, 0x12, 0x04, 0x9b, 0x02, 0x08, 0x1f, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x09,
    0x12, 0x06, 0x9e, 0x02, 0x00, 0xa4, 0x02, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x09, 0x01, 0x12,
    0x04, 0x9e, 0x02, 0x08, 0x1a, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x09, 0x02, 0x00, 0x12, 0x04, 0x9f,
    0x02, 0x08, 0x24, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x00, 0x04, 0x12, 0x04, 0x9f, 0x02,
    0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x00, 0x05, 0x12, 0x04, 0x9f, 0x02, 0x11,
    0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x00, 0x01, 0x12, 0x04, 0x9f, 0x02, 0x18, 0x1f,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x00, 0x03, 0x12, 0x04, 0x9f, 0x02, 0x22, 0x23, 0x0a,
    0x0c, 0x0a, 0x04, 0x04, 0x09, 0x02, 0x01, 0x12, 0x04, 0xa0, 0x02, 0x08, 0x52, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x09, 0x02, 0x01, 0x04, 0x12, 0x04, 0xa0, 0x02, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x09, 0x02, 0x01, 0x06, 0x12, 0x04, 0xa0, 0x02, 0x11, 0x23, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x09, 0x02, 0x01, 0x01, 0x12, 0x04, 0xa0, 0x02, 0x24, 0x29, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09,
    0x02, 0x01, 0x03, 0x12, 0x04, 0xa0, 0x02, 0x2c, 0x2d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02,
    0x01, 0x08, 0x12, 0x04, 0xa0, 0x02, 0x2e, 0x51, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x01,
    0x07, 0x12, 0x04, 0xa0, 0x02, 0x39, 0x50, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x09, 0x02, 0x02, 0x12,
    0x04, 0xa1, 0x02, 0x08, 0x26, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x02, 0x04, 0x12, 0x04,
    0xa1, 0x02, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x02, 0x05, 0x12, 0x04, 0xa1,
    0x02, 0x11, 0x16, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x02, 0x01, 0x12, 0x04, 0xa1, 0x02,
    0x17, 0x21, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x02, 0x03, 0x12, 0x04, 0xa1, 0x02, 0x24,
    0x25, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x09, 0x02, 0x03, 0x12, 0x04, 0xa2, 0x02, 0x08, 0x26, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x03, 0x04, 0x12, 0x04, 0xa2, 0x02, 0x08, 0x10, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x09, 0x02, 0x03, 0x05, 0x12, 0x04, 0xa2, 0x02, 0x11, 0x17, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x09, 0x02, 0x03, 0x01, 0x12, 0x04, 0xa2, 0x02, 0x18, 0x21, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x09, 0x02, 0x03, 0x03, 0x12, 0x04, 0xa2, 0x02, 0x24, 0x25, 0x0a, 0x0c, 0x0a, 0x04, 0x04,
    0x09, 0x02, 0x04, 0x12, 0x04, 0xa3, 0x02, 0x08, 0x25, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02,
    0x04, 0x04, 0x12, 0x04, 0xa3, 0x02, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x04,
    0x05, 0x12, 0x04, 0xa3, 0x02, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x04, 0x01,
    0x12, 0x04, 0xa3, 0x02, 0x18, 0x20, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x04, 0x03, 0x12,
    0x04, 0xa3, 0x02, 0x23, 0x24, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x0a, 0x12, 0x06, 0xa6, 0x02, 0x00,
    0xa7, 0x02, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x0a, 0x01, 0x12, 0x04, 0xa6, 0x02, 0x08, 0x19,
    0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x0b, 0x12, 0x06, 0xa9, 0x02, 0x00, 0xaf, 0x02, 0x01, 0x0a, 0x0b,
    0x0a, 0x03, 0x04, 0x0b, 0x01, 0x12, 0x04, 0xa9, 0x02, 0x08, 0x1a, 0x0a, 0x0c, 0x0a, 0x04, 0x04,
    0x0b, 0x02, 0x00, 0x12, 0x04, 0xaa, 0x02, 0x08, 0x24, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0b, 0x02,
    0x00, 0x04, 0x12, 0x04, 0xaa, 0x02, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x00,
    0x05, 0x12, 0x04, 0xaa, 0x02, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x00, 0x01,
    0x12, 0x04, 0xaa, 0x02, 0x18, 0x1f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x00, 0x03, 0x12,
    0x04, 0xaa, 0x02, 0x22, 0x23, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0b, 0x02, 0x01, 0x12, 0x04, 0xab,
    0x02, 0x08, 0x52, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x01, 0x04, 0x12, 0x04, 0xab, 0x02,
    0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x01, 0x06, 0x12, 0x04, 0xab, 0x02, 0x11,
    0x23, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x01, 0x01, 0x12, 0x04, 0xab, 0x02, 0x24, 0x29,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x01, 0x03, 0x12, 0x04, 0xab, 0x02, 0x2c, 0x2d, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x01, 0x08, 0x12, 0x04, 0xab, 0x02, 0x2e, 0x51, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x0b, 0x02, 0x01, 0x07, 0x12, 0x04, 0xab, 0x02, 0x39, 0x50, 0x0a, 0x0c, 0x0a,
    0x04, 0x04, 0x0b, 0x02, 0x02, 0x12, 0x04, 0xac, 0x02, 0x08, 0x26, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x0b, 0x02, 0x02, 0x04, 0x12, 0x04, 0xac, 0x02, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0b,
    0x02, 0x02, 0x05, 0x12, 0x04, 0xac, 0x02, 0x11, 0x16, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0b, 0x02,
    0x02, 0x01, 0x12, 0x04, 0xac, 0x02, 0x17, 0x21, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x02,
    0x03, 0x12, 0x04, 0xac, 0x02, 0x24, 0x25, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0b, 0x02, 0x03, 0x12,
    0x04, 0xad, 0x02, 0x08, 0x22, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x03, 0x04, 0x12, 0x04,
    0xad, 0x02, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x03, 0x05, 0x12, 0x04, 0xad,
    0x02, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x03, 0x01, 0x12, 0x04, 0xad, 0x02,
    0x18, 0x1d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x03, 0x03, 0x12, 0x04, 0xad, 0x02, 0x20,
    0x21, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0b, 0x02, 0x04, 0x12, 0x04, 0xae, 0x02, 0x08, 0x23, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x04, 0x04, 0x12, 0x04, 0xae, 0x02, 0x08, 0x10, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x0b, 0x02, 0x04, 0x05, 0x12, 0x04, 0xae, 0x02, 0x11, 0x17, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x0b, 0x02, 0x04, 0x01, 0x12, 0x04, 0xae, 0x02, 0x18, 0x1e, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x0b, 0x02, 0x04, 0x03, 0x12, 0x04, 0xae, 0x02, 0x21, 0x22, 0x0a, 0x0c, 0x0a, 0x02, 0x04,
    0x0c, 0x12, 0x06, 0xb1, 0x02, 0x00, 0xb2, 0x02, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x0c, 0x01,
    0x12, 0x04, 0xb1, 0x02, 0x08, 0x19, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x0d, 0x12, 0x06, 0xb4, 0x02,
    0x00, 0xb7, 0x02, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x0d, 0x01, 0x12, 0x04, 0xb4, 0x02, 0x08,
    0x1c, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0d, 0x02, 0x00, 0x12, 0x04, 0xb5, 0x02, 0x08, 0x27, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x00, 0x04, 0x12, 0x04, 0xb5, 0x02, 0x08, 0x10, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x0d, 0x02, 0x00, 0x05, 0x12, 0x04, 0xb5, 0x02, 0x11, 0x17, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x0d, 0x02, 0x00, 0x01, 0x12, 0x04, 0xb5, 0x02, 0x18, 0x22, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x0d, 0x02, 0x00, 0x03, 0x12, 0x04, 0xb5, 0x02, 0x25, 0x26, 0x0a, 0x0c, 0x0a, 0x04, 0x04,
    0x0d, 0x02, 0x01, 0x12, 0x04, 0xb6, 0x02, 0x08, 0x22, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02,
    0x01, 0x04, 0x12, 0x04, 0xb6, 0x02, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x01,
    0x05, 0x12, 0x04, 0xb6, 0x02, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x01, 0x01,
    0x12, 0x04, 0xb6, 0x02, 0x18, 0x1d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x01, 0x03, 0x12,
    0x04, 0xb6, 0x02, 0x20, 0x21, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x0e, 0x12, 0x06, 0xb9, 0x02, 0x00,
    0xbf, 0x02, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x0e, 0x01, 0x12, 0x04, 0xb9, 0x02, 0x08, 0x1c,
    0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0e, 0x02, 0x00, 0x12, 0x04, 0xba, 0x02, 0x08, 0x27, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x0e, 0x02, 0x00, 0x04, 0x12, 0x04, 0xba, 0x02, 0x08, 0x10, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x0e, 0x02, 0x00, 0x05, 0x12, 0x04, 0xba, 0x02, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x0e, 0x02, 0x00, 0x01, 0x12, 0x04, 0xba, 0x02, 0x18, 0x22, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x0e, 0x02, 0x00, 0x03, 0x12, 0x04, 0xba, 0x02, 0x25, 0x26, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0e,
    0x02, 0x01, 0x12, 0x04, 0xbb, 0x02, 0x08, 0x28, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x01,
    0x04, 0x12, 0x04, 0xbb, 0x02, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x01, 0x05,
    0x12, 0x04, 0xbb, 0x02, 0x11, 0x16, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x01, 0x01, 0x12,
    0x04, 0xbb, 0x02, 0x17, 0x23, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x01, 0x03, 0x12, 0x04,
    0xbb, 0x02, 0x26, 0x27, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0e, 0x02, 0x02, 0x12, 0x04, 0xbc, 0x02,
    0x08, 0x28, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x02, 0x04, 0x12, 0x04, 0xbc, 0x02, 0x08,
    0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x02, 0x05, 0x12, 0x04, 0xbc, 0x02, 0x11, 0x16,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x02, 0x01, 0x12, 0x04, 0xbc, 0x02, 0x17, 0x23, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x02, 0x03, 0x12, 0x04, 0xbc, 0x02, 0x26, 0x27, 0x0a, 0x0c,
    0x0a, 0x04, 0x04, 0x0e, 0x02, 0x03, 0x12, 0x04, 0xbd, 0x02, 0x08, 0x1e, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x0e, 0x02, 0x03, 0x04, 0x12, 0x04, 0xbd, 0x02, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x0e, 0x02, 0x03, 0x05, 0x12, 0x04, 0xbd, 0x02, 0x11, 0x16, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0e,
    0x02, 0x03, 0x01, 0x12, 0x04, 0xbd, 0x02, 0x17, 0x19, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0e, 0x02,
    0x03, 0x03, 0x12, 0x04, 0xbd, 0x02, 0x1c, 0x1d, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0e, 0x02, 0x04,
    0x12, 0x04, 0xbe, 0x02, 0x08, 0x1e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x04, 0x04, 0x12,
    0x04, 0xbe, 0x02, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x04, 0x05, 0x12, 0x04,
    0xbe, 0x02, 0x11, 0x16, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x04, 0x01, 0x12, 0x04, 0xbe,
    0x02, 0x17, 0x19, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x04, 0x03, 0x12, 0x04, 0xbe, 0x02,
    0x1c, 0x1d, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x0f, 0x12, 0x06, 0xc1, 0x02, 0x00, 0xc4, 0x02, 0x01,
    0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x0f, 0x01, 0x12, 0x04, 0xc1, 0x02, 0x08, 0x1b, 0x0a, 0x0c, 0x0a,
    0x04, 0x04, 0x0f, 0x02, 0x00, 0x12, 0x04, 0xc2, 0x02, 0x08, 0x27, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x0f, 0x02, 0x00, 0x04, 0x12, 0x04, 0xc2, 0x02, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f,
    0x02, 0x00, 0x05, 0x12, 0x04, 0xc2, 0x02, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02,
    0x00, 0x01, 0x12, 0x04, 0xc2, 0x02, 0x18, 0x22, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x00,
    0x03, 0x12, 0x04, 0xc2, 0x02, 0x25, 0x26, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0f, 0x02, 0x01, 0x12,
    0x04, 0xc3, 0x02, 0x08, 0x5d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x01, 0x04, 0x12, 0x04,
    0xc3, 0x02, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x01, 0x06, 0x12, 0x04, 0xc3,
    0x02, 0x11, 0x2c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x01, 0x01, 0x12, 0x04, 0xc3, 0x02,
    0x2d, 0x36, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x01, 0x03, 0x12, 0x04, 0xc3, 0x02, 0x39,
    0x3a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x01, 0x08, 0x12, 0x04, 0xc3, 0x02, 0x3b, 0x5c,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x01, 0x07, 0x12, 0x04, 0xc3, 0x02, 0x46, 0x5b, 0x0a,
    0x0c, 0x0a, 0x02, 0x04, 0x10, 0x12, 0x06, 0xc6, 0x02, 0x00, 0xc9, 0x02, 0x01, 0x0a, 0x0b, 0x0a,
    0x03, 0x04, 0x10, 0x01, 0x12, 0x04, 0xc6, 0x02, 0x08, 0x1a, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x10,
    0x02, 0x00, 0x12, 0x04, 0xc7, 0x02, 0x08, 0x27, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x00,
    0x04, 0x12, 0x04, 0xc7, 0x02, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x00, 0x05,
    0x12, 0x04, 0xc7, 0x02, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x00, 0x01, 0x12,
    0x04, 0xc7, 0x02, 0x18, 0x22, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x00, 0x03, 0x12, 0x04,
    0xc7, 0x02, 0x25, 0x26, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x10, 0x02, 0x01, 0x12, 0x04, 0xc8, 0x02,
    0x08, 0x55, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x01, 0x04, 0x12, 0x04, 0xc8, 0x02, 0x08,
    0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x01, 0x06, 0x12, 0x04, 0xc8, 0x02, 0x11, 0x24,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x01, 0x01, 0x12, 0x04, 0xc8, 0x02, 0x25, 0x2b, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x01, 0x03, 0x12, 0x04, 0xc8, 0x02, 0x2e, 0x2f, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x10, 0x02, 0x01, 0x08, 0x12, 0x04, 0xc8, 0x02, 0x30, 0x54, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x10, 0x02, 0x01, 0x07, 0x12, 0x04, 0xc8, 0x02, 0x3b, 0x53, 0x0a, 0x0c, 0x0a, 0x02,
    0x04, 0x11, 0x12, 0x06, 0xcb, 0x02, 0x00, 0xce, 0x02, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x11,
    0x01, 0x12, 0x04, 0xcb, 0x02, 0x08, 0x18, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x11, 0x02, 0x00, 0x12,
    0x04, 0xcc, 0x02, 0x08, 0x27, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x00, 0x04, 0x12, 0x04,
    0xcc, 0x02, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x00, 0x05, 0x12, 0x04, 0xcc,
    0x02, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x00, 0x01, 0x12, 0x04, 0xcc, 0x02,
    0x18, 0x22, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x00, 0x03, 0x12, 0x04, 0xcc, 0x02, 0x25,
    0x26, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x11, 0x02, 0x01, 0x12, 0x04, 0xcd, 0x02, 0x08, 0x55, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x01, 0x04, 0x12, 0x04, 0xcd, 0x02, 0x08, 0x10, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x11, 0x02, 0x01, 0x06, 0x12, 0x04, 0xcd, 0x02, 0x11, 0x24, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x11, 0x02, 0x01, 0x01, 0x12, 0x04, 0xcd, 0x02, 0x25, 0x2b, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x11, 0x02, 0x01, 0x03, 0x12, 0x04, 0xcd, 0x02, 0x2e, 0x2f, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x11, 0x02, 0x01, 0x08, 0x12, 0x04, 0xcd, 0x02, 0x30, 0x54, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11,
    0x02, 0x01, 0x07, 0x12, 0x04, 0xcd, 0x02, 0x3b, 0x53, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x12, 0x12,
    0x06, 0xd0, 0x02, 0x00, 0xd3, 0x02, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x12, 0x01, 0x12, 0x04,
    0xd0, 0x02, 0x08, 0x18, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x12, 0x02, 0x00, 0x12, 0x04, 0xd1, 0x02,
    0x08, 0x27, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x12, 0x02, 0x00, 0x04, 0x12, 0x04, 0xd1, 0x02, 0x08,
    0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x12, 0x02, 0x00, 0x05, 0x12, 0x04, 0xd1, 0x02, 0x11, 0x17,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x12, 0x02, 0x00, 0x01, 0x12, 0x04, 0xd1, 0x02, 0x18, 0x22, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x12, 0x02, 0x00, 0x03, 0x12, 0x04, 0xd1, 0x02, 0x25, 0x26, 0x0a, 0x0c,
    0x0a, 0x04, 0x04, 0x12, 0x02, 0x01, 0x12, 0x04, 0xd2, 0x02, 0x08, 0x25, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x12, 0x02, 0x01, 0x04, 0x12, 0x04, 0xd2, 0x02, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x12, 0x02, 0x01, 0x05, 0x12, 0x04, 0xd2, 0x02, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x12,
    0x02, 0x01, 0x01, 0x12, 0x04, 0xd2, 0x02, 0x18, 0x20, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x12, 0x02,
    0x01, 0x03, 0x12, 0x04, 0xd2, 0x02, 0x23, 0x24, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x13, 0x12, 0x06,
    0xd5, 0x02, 0x00, 0xd8, 0x02, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x13, 0x01, 0x12, 0x04, 0xd5,
    0x02, 0x08, 0x16, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x13, 0x02, 0x00, 0x12, 0x04, 0xd6, 0x02, 0x08,
    0x27, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x13, 0x02, 0x00, 0x04, 0x12, 0x04, 0xd6, 0x02, 0x08, 0x10,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x13, 0x02, 0x00, 0x05, 0x12, 0x04, 0xd6, 0x02, 0x11, 0x17, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x13, 0x02, 0x00, 0x01, 0x12, 0x04, 0xd6, 0x02, 0x18, 0x22, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x13, 0x02, 0x00, 0x03, 0x12, 0x04, 0xd6, 0x02, 0x25, 0x26, 0x0a, 0x0c, 0x0a,
    0x04, 0x04, 0x13, 0x02, 0x01, 0x12, 0x04, 0xd7, 0x02, 0x08, 0x25, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x13, 0x02, 0x01, 0x04, 0x12, 0x04, 0xd7, 0x02, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x13,
    0x02, 0x01, 0x05, 0x12, 0x04, 0xd7, 0x02, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x13, 0x02,
    0x01, 0x01, 0x12, 0x04, 0xd7, 0x02, 0x18, 0x20, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x13, 0x02, 0x01,
    0x03, 0x12, 0x04, 0xd7, 0x02, 0x23, 0x24, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x14, 0x12, 0x06, 0xda,
    0x02, 0x00, 0xde, 0x02, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x14, 0x01, 0x12, 0x04, 0xda, 0x02,
    0x08, 0x20, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x14, 0x02, 0x00, 0x12, 0x04, 0xdb, 0x02, 0x08, 0x29,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02, 0x00, 0x04, 0x12, 0x04, 0xdb, 0x02, 0x08, 0x10, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02, 0x00, 0x05, 0x12, 0x04, 0xdb, 0x02, 0x11, 0x16, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x14, 0x02, 0x00, 0x01, 0x12, 0x04, 0xdb, 0x02, 0x17, 0x24, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x14, 0x02, 0x00, 0x03, 0x12, 0x04, 0xdb, 0x02, 0x27, 0x28, 0x0a, 0x0c, 0x0a, 0x04,
    0x04, 0x14, 0x02, 0x01, 0x12, 0x04, 0xdc, 0x02, 0x08, 0x2c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14,
    0x02, 0x01, 0x04, 0x12, 0x04, 0xdc, 0x02, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02,
    0x01, 0x05, 0x12, 0x04, 0xdc, 0x02, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02, 0x01,
    0x01, 0x12, 0x04, 0xdc, 0x02, 0x18, 0x27, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02, 0x01, 0x03,
    0x12, 0x04, 0xdc, 0x02, 0x2a, 0x2b, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x14, 0x02, 0x02, 0x12, 0x04,
    0xdd, 0x02, 0x08, 0x2f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02, 0x02, 0x04, 0x12, 0x04, 0xdd,
    0x02, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02, 0x02, 0x05, 0x12, 0x04, 0xdd, 0x02,
    0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02, 0x02, 0x01, 0x12, 0x04, 0xdd, 0x02, 0x18,
    0x2a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02, 0x02, 0x03, 0x12, 0x04, 0xdd, 0x02, 0x2d, 0x2e,
    0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x15, 0x12, 0x06, 0xe0, 0x02, 0x00, 0xe5, 0x02, 0x01, 0x0a, 0x0b,
    0x0a, 0x03, 0x04, 0x15, 0x01, 0x12, 0x04, 0xe0, 0x02, 0x08, 0x1d, 0x0a, 0x0c, 0x0a, 0x04, 0x04,
    0x15, 0x02, 0x00, 0x12, 0x04, 0xe1, 0x02, 0x08, 0x27, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02,
    0x00, 0x04, 0x12, 0x04, 0xe1, 0x02, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x00,
    0x05, 0x12, 0x04, 0xe1, 0x02, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x00, 0x01,
    0x12, 0x04, 0xe1, 0x02, 0x18, 0x22, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x00, 0x03, 0x12,
    0x04, 0xe1, 0x02, 0x25, 0x26, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x15, 0x02, 0x01, 0x12, 0x04, 0xe2,
    0x02, 0x08, 0x29, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x01, 0x04, 0x12, 0x04, 0xe2, 0x02,
    0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x01, 0x05, 0x12, 0x04, 0xe2, 0x02, 0x11,
    0x16, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x01, 0x01, 0x12, 0x04, 0xe2, 0x02, 0x17, 0x24,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x01, 0x03, 0x12, 0x04, 0xe2, 0x02, 0x27, 0x28, 0x0a,
    0x0c, 0x0a, 0x04, 0x04, 0x15, 0x02, 0x02, 0x12, 0x04, 0xe3, 0x02, 0x08, 0x5d, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x15, 0x02, 0x02, 0x04, 0x12, 0x04, 0xe3, 0x02, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x15, 0x02, 0x02, 0x06, 0x12, 0x04, 0xe3, 0x02, 0x11, 0x29, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x15, 0x02, 0x02, 0x01, 0x12, 0x04, 0xe3, 0x02, 0x2a, 0x2f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15,
    0x02, 0x02, 0x03, 0x12, 0x04, 0xe3, 0x02, 0x32, 0x33, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02,
    0x02, 0x08, 0x12, 0x04, 0xe3, 0x02, 0x34, 0x5c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x02,
    0x07, 0x12, 0x04, 0xe3, 0x02, 0x3f, 0x5b, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x15, 0x02, 0x03, 0x12,
    0x04, 0xe4, 0x02, 0x08, 0x21, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x03, 0x04, 0x12, 0x04,
    0xe4, 0x02, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x03, 0x05, 0x12, 0x04, 0xe4,
    0x02, 0x11, 0x16, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x03, 0x01, 0x12, 0x04, 0xe4, 0x02,
    0x17, 0x1c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x03, 0x03, 0x12, 0x04, 0xe4, 0x02, 0x1f,
    0x20, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x16, 0x12, 0x06, 0xe7, 0x02, 0x00, 0xe9, 0x02, 0x01, 0x0a,
    0x0b, 0x0a, 0x03, 0x04, 0x16, 0x01, 0x12, 0x04, 0xe7, 0x02, 0x08, 0x20, 0x0a, 0x0c, 0x0a, 0x04,
    0x04, 0x16, 0x02, 0x00, 0x12, 0x04, 0xe8, 0x02, 0x08, 0x29, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x16,
    0x02, 0x00, 0x04, 0x12, 0x04, 0xe8, 0x02, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x16, 0x02,
    0x00, 0x05, 0x12, 0x04, 0xe8, 0x02, 0x11, 0x16, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x16, 0x02, 0x00,
    0x01, 0x12, 0x04, 0xe8, 0x02, 0x17, 0x24, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x16, 0x02, 0x00, 0x03,
    0x12, 0x04, 0xe8, 0x02, 0x27, 0x28, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x17, 0x12, 0x06, 0xeb, 0x02,
    0x00, 0xef, 0x02, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x17, 0x01, 0x12, 0x04, 0xeb, 0x02, 0x08,
    0x19, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x17, 0x02, 0x00, 0x12, 0x04, 0xec, 0x02, 0x08, 0x29, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x17, 0x02, 0x00, 0x04, 0x12, 0x04, 0xec, 0x02, 0x08, 0x10, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x17, 0x02, 0x00, 0x05, 0x12, 0x04, 0xec, 0x02, 0x11, 0x16, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x17, 0x02, 0x00, 0x01, 0x12, 0x04, 0xec, 0x02, 0x17, 0x24, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x17, 0x02, 0x00, 0x03, 0x12, 0x04, 0xec, 0x02, 0x27, 0x28, 0x0a, 0x0c, 0x0a, 0x04, 0x04,
    0x17, 0x02, 0x01, 0x12, 0x04, 0xed, 0x02, 0x08, 0x2c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x17, 0x02,
    0x01, 0x04, 0x12, 0x04, 0xed, 0x02, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x17, 0x02, 0x01,
    0x05, 0x12, 0x04, 0xed, 0x02, 0x11, 0x16, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x17, 0x02, 0x01, 0x01,
    0x12, 0x04, 0xed, 0x02, 0x17, 0x27, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x17, 0x02, 0x01, 0x03, 0x12,
    0x04, 0xed, 0x02, 0x2a, 0x2b, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x17, 0x02, 0x02, 0x12, 0x04, 0xee,
    0x02, 0x08, 0x2d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x17, 0x02, 0x02, 0x04, 0x12, 0x04, 0xee, 0x02,
    0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x17, 0x02, 0x02, 0x05, 0x12, 0x04, 0xee, 0x02, 0x11,
    0x16, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x17, 0x02, 0x02, 0x01, 0x12, 0x04, 0xee, 0x02, 0x17, 0x28,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x17, 0x02, 0x02, 0x03, 0x12, 0x04, 0xee, 0x02, 0x2b, 0x2c, 0x0a,
    0x0c, 0x0a, 0x02, 0x04, 0x18, 0x12, 0x06, 0xf1, 0x02, 0x00, 0xf3, 0x02, 0x01, 0x0a, 0x0b, 0x0a,
    0x03, 0x04, 0x18, 0x01, 0x12, 0x04, 0xf1, 0x02, 0x08, 0x14, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x18,
    0x02, 0x00, 0x12, 0x04, 0xf2, 0x02, 0x08, 0x21, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x18, 0x02, 0x00,
    0x04, 0x12, 0x04, 0xf2, 0x02, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x18, 0x02, 0x00, 0x05,
    0x12, 0x04, 0xf2, 0x02, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x18, 0x02, 0x00, 0x01, 0x12,
    0x04, 0xf2, 0x02, 0x18, 0x1c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x18, 0x02, 0x00, 0x03, 0x12, 0x04,
    0xf2, 0x02, 0x1f, 0x20, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x19, 0x12, 0x06, 0xf5, 0x02, 0x00, 0xf9,
    0x02, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x19, 0x01, 0x12, 0x04, 0xf5, 0x02, 0x08, 0x13, 0x0a,
    0x0c, 0x0a, 0x04, 0x04, 0x19, 0x02, 0x00, 0x12, 0x04, 0xf6, 0x02, 0x08, 0x21, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x19, 0x02, 0x00, 0x04, 0x12, 0x04, 0xf6, 0x02, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x19, 0x02, 0x00, 0x05, 0x12, 0x04, 0xf6, 0x02, 0x11, 0x16, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x19, 0x02, 0x00, 0x01, 0x12, 0x04, 0xf6, 0x02, 0x17, 0x1c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x19,
    0x02, 0x00, 0x03, 0x12, 0x04, 0xf6, 0x02, 0x1f, 0x20, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x19, 0x02,
    0x01, 0x12, 0x04, 0xf7, 0x02, 0x08, 0x22, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x19, 0x02, 0x01, 0x04,
    0x12, 0x04, 0xf7, 0x02, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x19, 0x02, 0x01, 0x05, 0x12,
    0x04, 0xf7, 0x02, 0x11, 0x16, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x19, 0x02, 0x01, 0x01, 0x12, 0x04,
    0xf7, 0x02, 0x17, 0x1d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x19, 0x02, 0x01, 0x03, 0x12, 0x04, 0xf7,
    0x02, 0x20, 0x21, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x19, 0x02, 0x02, 0x12, 0x04, 0xf8, 0x02, 0x08,
    0x21, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x19, 0x02, 0x02, 0x04, 0x12, 0x04, 0xf8, 0x02, 0x08, 0x10,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x19, 0x02, 0x02, 0x05, 0x12, 0x04, 0xf8, 0x02, 0x11, 0x16, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x19, 0x02, 0x02, 0x01, 0x12, 0x04, 0xf8, 0x02, 0x17, 0x1c, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x19, 0x02, 0x02, 0x03, 0x12, 0x04, 0xf8, 0x02, 0x1f, 0x20, 0x0a, 0x0c, 0x0a,
    0x02, 0x04, 0x1a, 0x12, 0x06, 0xfb, 0x02, 0x00, 0xfe, 0x02, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04,
    0x1a, 0x01, 0x12, 0x04, 0xfb, 0x02, 0x08, 0x16, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x1a, 0x02, 0x00,
    0x12, 0x04, 0xfc, 0x02, 0x08, 0x28, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1a, 0x02, 0x00, 0x04, 0x12,
    0x04, 0xfc, 0x02, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1a, 0x02, 0x00, 0x05, 0x12, 0x04,
    0xfc, 0x02, 0x11, 0x16, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1a, 0x02, 0x00, 0x01, 0x12, 0x04, 0xfc,
    0x02, 0x17, 0x23, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1a, 0x02, 0x00, 0x03, 0x12, 0x04, 0xfc, 0x02,
    0x26, 0x27, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x1a, 0x02, 0x01, 0x12, 0x04, 0xfd, 0x02, 0x08, 0x28,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1a, 0x02, 0x01, 0x04, 0x12, 0x04, 0xfd, 0x02, 0x08, 0x10, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x1a, 0x02, 0x01, 0x05, 0x12, 0x04, 0xfd, 0x02, 0x11, 0x16, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x1a, 0x02, 0x01, 0x01, 0x12, 0x04, 0xfd, 0x02, 0x17, 0x23, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x1a, 0x02, 0x01, 0x03, 0x12, 0x04, 0xfd, 0x02, 0x26, 0x27, 0x0a, 0x0c, 0x0a, 0x02,
    0x04, 0x1b, 0x12, 0x06, 0x80, 0x03, 0x00, 0x81, 0x03, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x1b,
    0x01, 0x12, 0x04, 0x80, 0x03, 0x08, 0x16, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x1c, 0x12, 0x06, 0x83,
    0x03, 0x00, 0x85, 0x03, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x1c, 0x01, 0x12, 0x04, 0x83, 0x03,
    0x08, 0x15, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x1c, 0x02, 0x00, 0x12, 0x04, 0x84, 0x03, 0x08, 0x26,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1c, 0x02, 0x00, 0x04, 0x12, 0x04, 0x84, 0x03, 0x08, 0x10, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x1c, 0x02, 0x00, 0x05, 0x12, 0x04, 0x84, 0x03, 0x11, 0x17, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x1c, 0x02, 0x00, 0x01, 0x12, 0x04, 0x84, 0x03, 0x18, 0x21, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x1c, 0x02, 0x00, 0x03, 0x12, 0x04, 0x84, 0x03, 0x24, 0x25, 0x0a, 0x0c, 0x0a, 0x02,
    0x04, 0x1d, 0x12, 0x06, 0x87, 0x03, 0x00, 0x89, 0x03, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x1d,
    0x01, 0x12, 0x04, 0x87, 0x03, 0x08, 0x1a, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x1d, 0x02, 0x00, 0x12,
    0x04, 0x88, 0x03, 0x08, 0x26, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1d, 0x02, 0x00, 0x04, 0x12, 0x04,
    0x88, 0x03, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1d, 0x02, 0x00, 0x05, 0x12, 0x04, 0x88,
    0x03, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1d, 0x02, 0x00, 0x01, 0x12, 0x04, 0x88, 0x03,
    0x18, 0x21, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1d, 0x02, 0x00, 0x03, 0x12, 0x04, 0x88, 0x03, 0x24,
    0x25, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x1e, 0x12, 0x06, 0x8b, 0x03, 0x00, 0x92, 0x03, 0x01, 0x0a,
    0x0b, 0x0a, 0x03, 0x04, 0x1e, 0x01, 0x12, 0x04, 0x8b, 0x03, 0x08, 0x1a, 0x0a, 0x0c, 0x0a, 0x04,
    0x04, 0x1e, 0x02, 0x00, 0x12, 0x04, 0x8c, 0x03, 0x08, 0x26, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1e,
    0x02, 0x00, 0x04, 0x12, 0x04, 0x8c, 0x03, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1e, 0x02,
    0x00, 0x05, 0x12, 0x04, 0x8c, 0x03, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1e, 0x02, 0x00,
    0x01, 0x12, 0x04, 0x8c, 0x03, 0x18, 0x21, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1e, 0x02, 0x00, 0x03,
    0x12, 0x04, 0x8c, 0x03, 0x24, 0x25, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x1e, 0x02, 0x01, 0x12, 0x04,
    0x8d, 0x03, 0x08, 0x21, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1e, 0x02, 0x01, 0x04, 0x12, 0x04, 0x8d,
    0x03, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1e, 0x02, 0x01, 0x05, 0x12, 0x04, 0x8d, 0x03,
    0x11, 0x16, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1e, 0x02, 0x01, 0x01, 0x12, 0x04, 0x8d, 0x03, 0x17,
    0x1c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1e, 0x02, 0x01, 0x03, 0x12, 0x04, 0x8d, 0x03, 0x1f, 0x20,
    0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x1e, 0x02, 0x02, 0x12, 0x04, 0x8e, 0x03, 0x08, 0x22, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x1e, 0x02, 0x02, 0x04, 0x12, 0x04, 0x8e, 0x03, 0x08, 0x10, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x1e, 0x02, 0x02, 0x05, 0x12, 0x04, 0x8e, 0x03, 0x11, 0x16, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x1e, 0x02, 0x02, 0x01, 0x12, 0x04, 0x8e, 0x03, 0x17, 0x1d, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x1e, 0x02, 0x02, 0x03, 0x12, 0x04, 0x8e, 0x03, 0x20, 0x21, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x1e,
    0x02, 0x03, 0x12, 0x04, 0x8f, 0x03, 0x08, 0x21, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1e, 0x02, 0x03,
    0x04, 0x12, 0x04, 0x8f, 0x03, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1e, 0x02, 0x03, 0x05,
    0x12, 0x04, 0x8f, 0x03, 0x11, 0x16, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1e, 0x02, 0x03, 0x01, 0x12,
    0x04, 0x8f, 0x03, 0x17, 0x1c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1e, 0x02, 0x03, 0x03, 0x12, 0x04,
    0x8f, 0x03, 0x1f, 0x20, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x1e, 0x02, 0x04, 0x12, 0x04, 0x90, 0x03,
    0x08, 0x21, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1e, 0x02, 0x04, 0x04, 0x12, 0x04, 0x90, 0x03, 0x08,
    0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1e, 0x02, 0x04, 0x05, 0x12, 0x04, 0x90, 0x03, 0x11, 0x16,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1e, 0x02, 0x04, 0x01, 0x12, 0x04, 0x90, 0x03, 0x17, 0x1c, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x1e, 0x02, 0x04, 0x03, 0x12, 0x04, 0x90, 0x03, 0x1f, 0x20, 0x0a, 0x0c,
    0x0a, 0x04, 0x04, 0x1e, 0x02, 0x05, 0x12, 0x04, 0x91, 0x03, 0x08, 0x21, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x1e, 0x02, 0x05, 0x04, 0x12, 0x04, 0x91, 0x03, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x1e, 0x02, 0x05, 0x05, 0x12, 0x04, 0x91, 0x03, 0x11, 0x16, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1e,
    0x02, 0x05, 0x01, 0x12, 0x04, 0x91, 0x03, 0x17, 0x1c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1e, 0x02,
    0x05, 0x03, 0x12, 0x04, 0x91, 0x03, 0x1f, 0x20, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x1f, 0x12, 0x06,
    0x94, 0x03, 0x00, 0x96, 0x03, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x1f, 0x01, 0x12, 0x04, 0x94,
    0x03, 0x08, 0x16, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x1f, 0x02, 0x00, 0x12, 0x04, 0x95, 0x03, 0x08,
    0x21, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1f, 0x02, 0x00, 0x04, 0x12, 0x04, 0x95, 0x03, 0x08, 0x10,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1f, 0x02, 0x00, 0x05, 0x12, 0x04, 0x95, 0x03, 0x11, 0x17, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x1f, 0x02, 0x00, 0x01, 0x12, 0x04, 0x95, 0x03, 0x18, 0x1c, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x1f, 0x02, 0x00, 0x03, 0x12, 0x04, 0x95, 0x03, 0x1f, 0x20, 0x0a, 0x0c, 0x0a,
    0x02, 0x04, 0x20, 0x12, 0x06, 0x98, 0x03, 0x00, 0x9a, 0x03, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04,
    0x20, 0x01, 0x12, 0x04, 0x98, 0x03, 0x08, 0x1c, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x20, 0x02, 0x00,
    0x12, 0x04, 0x99, 0x03, 0x08, 0x21, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x20, 0x02, 0x00, 0x04, 0x12,
    0x04, 0x99, 0x03, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x20, 0x02, 0x00, 0x05, 0x12, 0x04,
    0x99, 0x03, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x20, 0x02, 0x00, 0x01, 0x12, 0x04, 0x99,
    0x03, 0x18, 0x1c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x20, 0x02, 0x00, 0x03, 0x12, 0x04, 0x99, 0x03,
    0x1f, 0x20, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x21, 0x12, 0x06, 0x9c, 0x03, 0x00, 0x9e, 0x03, 0x01,
    0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x21, 0x01, 0x12, 0x04, 0x9c, 0x03, 0x08, 0x1c, 0x0a, 0x0c, 0x0a,
    0x04, 0x04, 0x21, 0x02, 0x00, 0x12, 0x04, 0x9d, 0x03, 0x08, 0x21, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x21, 0x02, 0x00, 0x04, 0x12, 0x04, 0x9d, 0x03, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x21,
    0x02, 0x00, 0x05, 0x12, 0x04, 0x9d, 0x03, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x21, 0x02,
    0x00, 0x01, 0x12, 0x04, 0x9d, 0x03, 0x18, 0x1c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x21, 0x02, 0x00,
    0x03, 0x12, 0x04, 0x9d, 0x03, 0x1f, 0x20, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x22, 0x12, 0x06, 0xa0,
    0x03, 0x00, 0xa1, 0x03, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x22, 0x01, 0x12, 0x04, 0xa0, 0x03,
    0x08, 0x14, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x23, 0x12, 0x06, 0xa3, 0x03, 0x00, 0xa5, 0x03, 0x01,
    0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x23, 0x01, 0x12, 0x04, 0xa3, 0x03, 0x08, 0x18, 0x0a, 0x0c, 0x0a,
    0x04, 0x04, 0x23, 0x02, 0x00, 0x12, 0x04, 0xa4, 0x03, 0x08, 0x26, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x23, 0x02, 0x00, 0x04, 0x12, 0x04, 0xa4, 0x03, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x23,
    0x02, 0x00, 0x05, 0x12, 0x04, 0xa4, 0x03, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x23, 0x02,
    0x00, 0x01, 0x12, 0x04, 0xa4, 0x03, 0x18, 0x21, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x23, 0x02, 0x00,
    0x03, 0x12, 0x04, 0xa4, 0x03, 0x24, 0x25, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x24, 0x12, 0x06, 0xa7,
    0x03, 0x00, 0xa9, 0x03, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x24, 0x01, 0x12, 0x04, 0xa7, 0x03,
    0x08, 0x20, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x24, 0x02, 0x00, 0x12, 0x04, 0xa8, 0x03, 0x08, 0x58,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x24, 0x02, 0x00, 0x04, 0x12, 0x04, 0xa8, 0x03, 0x08, 0x10, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x24, 0x02, 0x00, 0x06, 0x12, 0x04, 0xa8, 0x03, 0x11, 0x2a, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x24, 0x02, 0x00, 0x01, 0x12, 0x04, 0xa8, 0x03, 0x2b, 0x32, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x24, 0x02, 0x00, 0x03, 0x12, 0x04, 0xa8, 0x03, 0x35, 0x36, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x24, 0x02, 0x00, 0x08, 0x12, 0x04, 0xa8, 0x03, 0x37, 0x57, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x24, 0x02, 0x00, 0x07, 0x12, 0x04, 0xa8, 0x03, 0x42, 0x56, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x25,
    0x12, 0x06, 0xab, 0x03, 0x00, 0xae, 0x03, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x25, 0x01, 0x12,
    0x04, 0xab, 0x03, 0x08, 0x20, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x25, 0x02, 0x00, 0x12, 0x04, 0xac,
    0x03, 0x08, 0x1e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x25, 0x02, 0x00, 0x04, 0x12, 0x04, 0xac, 0x03,
    0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x25, 0x02, 0x00, 0x05, 0x12, 0x04, 0xac, 0x03, 0x11,
    0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x25, 0x02, 0x00, 0x01, 0x12, 0x04, 0xac, 0x03, 0x18, 0x19,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x25, 0x02, 0x00, 0x03, 0x12, 0x04, 0xac, 0x03, 0x1c, 0x1d, 0x0a,
    0x0c, 0x0a, 0x04, 0x04, 0x25, 0x02, 0x01, 0x12, 0x04, 0xad, 0x03, 0x08, 0x1e, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x25, 0x02, 0x01, 0x04, 0x12, 0x04, 0xad, 0x03, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x25, 0x02, 0x01, 0x05, 0x12, 0x04, 0xad, 0x03, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x25, 0x02, 0x01, 0x01, 0x12, 0x04, 0xad, 0x03, 0x18, 0x19, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x25,
    0x02, 0x01, 0x03, 0x12, 0x04, 0xad, 0x03, 0x1c, 0x1d, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x26, 0x12,
    0x06, 0xb0, 0x03, 0x00, 0xb2, 0x03, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x26, 0x01, 0x12, 0x04,
    0xb0, 0x03, 0x08, 0x1f, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x26, 0x02, 0x00, 0x12, 0x04, 0xb1, 0x03,
    0x08, 0x26, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x26, 0x02, 0x00, 0x04, 0x12, 0x04, 0xb1, 0x03, 0x08,
    0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x26, 0x02, 0x00, 0x05, 0x12, 0x04, 0xb1, 0x03, 0x11, 0x17,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x26, 0x02, 0x00, 0x01, 0x12, 0x04, 0xb1, 0x03, 0x18, 0x21, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x26, 0x02, 0x00, 0x03, 0x12, 0x04, 0xb1, 0x03, 0x24, 0x25, 0x0a, 0x0c,
    0x0a, 0x02, 0x04, 0x27, 0x12, 0x06, 0xb4, 0x03, 0x00, 0xb6, 0x03, 0x01, 0x0a, 0x0b, 0x0a, 0x03,
    0x04, 0x27, 0x01, 0x12, 0x04, 0xb4, 0x03, 0x08, 0x1d, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x27, 0x02,
    0x00, 0x12, 0x04, 0xb5, 0x03, 0x08, 0x23, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x27, 0x02, 0x00, 0x04,
    0x12, 0x04, 0xb5, 0x03, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x27, 0x02, 0x00, 0x05, 0x12,
    0x04, 0xb5, 0x03, 0x11, 0x16, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x27, 0x02, 0x00, 0x01, 0x12, 0x04,
    0xb5, 0x03, 0x17, 0x1e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x27, 0x02, 0x00, 0x03, 0x12, 0x04, 0xb5,
    0x03, 0x21, 0x22, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x28, 0x12, 0x06, 0xb8, 0x03, 0x00, 0xba, 0x03,
    0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x28, 0x01, 0x12, 0x04, 0xb8, 0x03, 0x08, 0x12, 0x0a, 0x0c,
    0x0a, 0x04, 0x04, 0x28, 0x02, 0x00, 0x12, 0x04, 0xb9, 0x03, 0x08, 0x22, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x28, 0x02, 0x00, 0x04, 0x12, 0x04, 0xb9, 0x03, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x28, 0x02, 0x00, 0x05, 0x12, 0x04, 0xb9, 0x03, 0x11, 0x15, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x28,
    0x02, 0x00, 0x01, 0x12, 0x04, 0xb9, 0x03, 0x16, 0x1d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x28, 0x02,
    0x00, 0x03, 0x12, 0x04, 0xb9, 0x03, 0x20, 0x21, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x29, 0x12, 0x06,
    0xbc, 0x03, 0x00, 0xc1, 0x03, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x29, 0x01, 0x12, 0x04, 0xbc,
    0x03, 0x08, 0x1e, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x29, 0x02, 0x00, 0x12, 0x04, 0xbd, 0x03, 0x08,
    0x26, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x29, 0x02, 0x00, 0x04, 0x12, 0x04, 0xbd, 0x03, 0x08, 0x10,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x29, 0x02, 0x00, 0x05, 0x12, 0x04, 0xbd, 0x03, 0x11, 0x17, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x29, 0x02, 0x00, 0x01, 0x12, 0x04, 0xbd, 0x03, 0x18, 0x21, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x29, 0x02, 0x00, 0x03, 0x12, 0x04, 0xbd, 0x03, 0x24, 0x25, 0x0a, 0x0c, 0x0a,
    0x04, 0x04, 0x29, 0x02, 0x01, 0x12, 0x04, 0xbe, 0x03, 0x08, 0x24, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x29, 0x02, 0x01, 0x04, 0x12, 0x04, 0xbe, 0x03, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x29,
    0x02, 0x01, 0x05, 0x12, 0x04, 0xbe, 0x03, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x29, 0x02,
    0x01, 0x01, 0x12, 0x04, 0xbe, 0x03, 0x18, 0x1f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x29, 0x02, 0x01,
    0x03, 0x12, 0x04, 0xbe, 0x03, 0x22, 0x23, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x29, 0x02, 0x02, 0x12,
    0x04, 0xbf, 0x03, 0x08, 0x30, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x29, 0x02, 0x02, 0x04, 0x12, 0x04,
    0xbf, 0x03, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x29, 0x02, 0x02, 0x05, 0x12, 0x04, 0xbf,
    0x03, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x29, 0x02, 0x02, 0x01, 0x12, 0x04, 0xbf, 0x03,
    0x18, 0x2b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x29, 0x02, 0x02, 0x03, 0x12, 0x04, 0xbf, 0x03, 0x2e,
    0x2f, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x29, 0x02, 0x03, 0x12, 0x04, 0xc0, 0x03, 0x08, 0x32, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x29, 0x02, 0x03, 0x04, 0x12, 0x04, 0xc0, 0x03, 0x08, 0x10, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x29, 0x02, 0x03, 0x05, 0x12, 0x04, 0xc0, 0x03, 0x11, 0x17, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x29, 0x02, 0x03, 0x01, 0x12, 0x04, 0xc0, 0x03, 0x18, 0x2d, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x29, 0x02, 0x03, 0x03, 0x12, 0x04, 0xc0, 0x03, 0x30, 0x31, 0x0a, 0x0c, 0x0a, 0x02, 0x04,
    0x2a, 0x12, 0x06, 0xc3, 0x03, 0x00, 0xc5, 0x03, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x2a, 0x01,
    0x12, 0x04, 0xc3, 0x03, 0x08, 0x1a, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x2a, 0x02, 0x00, 0x12, 0x04,
    0xc4, 0x03, 0x08, 0x22, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2a, 0x02, 0x00, 0x04, 0x12, 0x04, 0xc4,
    0x03, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2a, 0x02, 0x00, 0x05, 0x12, 0x04, 0xc4, 0x03,
    0x11, 0x15, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2a, 0x02, 0x00, 0x01, 0x12, 0x04, 0xc4, 0x03, 0x16,
    0x1d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2a, 0x02, 0x00, 0x03, 0x12, 0x04, 0xc4, 0x03, 0x20, 0x21,
    0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x2b, 0x12, 0x06, 0xc7, 0x03, 0x00, 0xc9, 0x03, 0x01, 0x0a, 0x0b,
    0x0a, 0x03, 0x04, 0x2b, 0x01, 0x12, 0x04, 0xc7, 0x03, 0x08, 0x23, 0x0a, 0x0c, 0x0a, 0x04, 0x04,
    0x2b, 0x02, 0x00, 0x12, 0x04, 0xc8, 0x03, 0x08, 0x2c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2b, 0x02,
    0x00, 0x04, 0x12, 0x04, 0xc8, 0x03, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2b, 0x02, 0x00,
    0x05, 0x12, 0x04, 0xc8, 0x03, 0x11, 0x16, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2b, 0x02, 0x00, 0x01,
    0x12, 0x04, 0xc8, 0x03, 0x17, 0x27, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2b, 0x02, 0x00, 0x03, 0x12,
    0x04, 0xc8, 0x03, 0x2a, 0x2b, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x2c, 0x12, 0x06, 0xcb, 0x03, 0x00,
    0xd3, 0x03, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x2c, 0x01, 0x12, 0x04, 0xcb, 0x03, 0x08, 0x20,
    0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x2c, 0x02, 0x00, 0x12, 0x04, 0xcc, 0x03, 0x08, 0x27, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x2c, 0x02, 0x00, 0x04, 0x12, 0x04, 0xcc, 0x03, 0x08, 0x10, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x2c, 0x02, 0x00, 0x05, 0x12, 0x04, 0xcc, 0x03, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x2c, 0x02, 0x00, 0x01, 0x12, 0x04, 0xcc, 0x03, 0x18, 0x22, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x2c, 0x02, 0x00, 0x03, 0x12, 0x04, 0xcc, 0x03, 0x25, 0x26, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x2c,
    0x02, 0x01, 0x12, 0x04, 0xcd, 0x03, 0x08, 0x2c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2c, 0x02, 0x01,
    0x04, 0x12, 0x04, 0xcd, 0x03, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2c, 0x02, 0x01, 0x05,
    0x12, 0x04, 0xcd, 0x03, 0x11, 0x16, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2c, 0x02, 0x01, 0x01, 0x12,
    0x04, 0xcd, 0x03, 0x17, 0x27, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2c, 0x02, 0x01, 0x03, 0x12, 0x04,
    0xcd, 0x03, 0x2a, 0x2b, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x2c, 0x02, 0x02, 0x12, 0x04, 0xce, 0x03,
    0x08, 0x24, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2c, 0x02, 0x02, 0x04, 0x12, 0x04, 0xce, 0x03, 0x08,
    0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2c, 0x02, 0x02, 0x05, 0x12, 0x04, 0xce, 0x03, 0x11, 0x17,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2c, 0x02, 0x02, 0x01, 0x12, 0x04, 0xce, 0x03, 0x18, 0x1f, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x2c, 0x02, 0x02, 0x03, 0x12, 0x04, 0xce, 0x03, 0x22, 0x23, 0x0a, 0x0c,
    0x0a, 0x04, 0x04, 0x2c, 0x02, 0x03, 0x12, 0x04, 0xcf, 0x03, 0x08, 0x27, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x2c, 0x02, 0x03, 0x04, 0x12, 0x04, 0xcf, 0x03, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x2c, 0x02, 0x03, 0x05, 0x12, 0x04, 0xcf, 0x03, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2c,
    0x02, 0x03, 0x01, 0x12, 0x04, 0xcf, 0x03, 0x18, 0x22, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2c, 0x02,
    0x03, 0x03, 0x12, 0x04, 0xcf, 0x03, 0x25, 0x26, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x2c, 0x02, 0x04,
    0x12, 0x04, 0xd0, 0x03, 0x08, 0x27, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2c, 0x02, 0x04, 0x04, 0x12,
    0x04, 0xd0, 0x03, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2c, 0x02, 0x04, 0x05, 0x12, 0x04,
    0xd0, 0x03, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2c, 0x02, 0x04, 0x01, 0x12, 0x04, 0xd0,
    0x03, 0x18, 0x22, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2c, 0x02, 0x04, 0x03, 0x12, 0x04, 0xd0, 0x03,
    0x25, 0x26, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x2c, 0x02, 0x05, 0x12, 0x04, 0xd1, 0x03, 0x08, 0x28,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2c, 0x02, 0x05, 0x04, 0x12, 0x04, 0xd1, 0x03, 0x08, 0x10, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x2c, 0x02, 0x05, 0x05, 0x12, 0x04, 0xd1, 0x03, 0x11, 0x17, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x2c, 0x02, 0x05, 0x01, 0x12, 0x04, 0xd1, 0x03, 0x18, 0x23, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x2c, 0x02, 0x05, 0x03, 0x12, 0x04, 0xd1, 0x03, 0x26, 0x27, 0x0a, 0x0c, 0x0a, 0x04,
    0x04, 0x2c, 0x02, 0x06, 0x12, 0x04, 0xd2, 0x03, 0x08, 0x28, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2c,
    0x02, 0x06, 0x04, 0x12, 0x04, 0xd2, 0x03, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2c, 0x02,
    0x06, 0x05, 0x12, 0x04, 0xd2, 0x03, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2c, 0x02, 0x06,
    0x01, 0x12, 0x04, 0xd2, 0x03, 0x18, 0x23, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2c, 0x02, 0x06, 0x03,
    0x12, 0x04, 0xd2, 0x03, 0x26, 0x27, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x2d, 0x12, 0x06, 0xd5, 0x03,
    0x00, 0xd8, 0x03, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x2d, 0x01, 0x12, 0x04, 0xd5, 0x03, 0x08,
    0x2b, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x2d, 0x02, 0x00, 0x12, 0x04, 0xd6, 0x03, 0x08, 0x2b, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x2d, 0x02, 0x00, 0x04, 0x12, 0x04, 0xd6, 0x03, 0x08, 0x10, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x2d, 0x02, 0x00, 0x05, 0x12, 0x04, 0xd6, 0x03, 0x11, 0x15, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x2d, 0x02, 0x00, 0x01, 0x12, 0x04, 0xd6, 0x03, 0x16, 0x26, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x2d, 0x02, 0x00, 0x03, 0x12, 0x04, 0xd6, 0x03, 0x29, 0x2a, 0x0a, 0x0c, 0x0a, 0x04, 0x04,
    0x2d, 0x02, 0x01, 0x12, 0x04, 0xd7, 0x03, 0x08, 0x2d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2d, 0x02,
    0x01, 0x04, 0x12, 0x04, 0xd7, 0x03, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2d, 0x02, 0x01,
    0x05, 0x12, 0x04, 0xd7, 0x03, 0x11, 0x15, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2d, 0x02, 0x01, 0x01,
    0x12, 0x04, 0xd7, 0x03, 0x16, 0x28, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2d, 0x02, 0x01, 0x03, 0x12,
    0x04, 0xd7, 0x03, 0x2b, 0x2c, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x2e, 0x12, 0x06, 0xda, 0x03, 0x00,
    0xde, 0x03, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x2e, 0x01, 0x12, 0x04, 0xda, 0x03, 0x08, 0x1e,
    0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x2e, 0x02, 0x00, 0x12, 0x04, 0xdb, 0x03, 0x08, 0x2c, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x2e, 0x02, 0x00, 0x04, 0x12, 0x04, 0xdb, 0x03, 0x08, 0x10, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x2e, 0x02, 0x00, 0x05, 0x12, 0x04, 0xdb, 0x03, 0x11, 0x16, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x2e, 0x02, 0x00, 0x01, 0x12, 0x04, 0xdb, 0x03, 0x17, 0x27, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x2e, 0x02, 0x00, 0x03, 0x12, 0x04, 0xdb, 0x03, 0x2a, 0x2b, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x2e,
    0x02, 0x01, 0x12, 0x04, 0xdc, 0x03, 0x08, 0x25, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2e, 0x02, 0x01,
    0x04, 0x12, 0x04, 0xdc, 0x03, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2e, 0x02, 0x01, 0x05,
    0x12, 0x04, 0xdc, 0x03, 0x11, 0x16, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2e, 0x02, 0x01, 0x01, 0x12,
    0x04, 0xdc, 0x03, 0x17, 0x20, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2e, 0x02, 0x01, 0x03, 0x12, 0x04,
    0xdc, 0x03, 0x23, 0x24, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x2e, 0x02, 0x02, 0x12, 0x04, 0xdd, 0x03,
    0x08, 0x2e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2e, 0x02, 0x02, 0x04, 0x12, 0x04, 0xdd, 0x03, 0x08,
    0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2e, 0x02, 0x02, 0x05, 0x12, 0x04, 0xdd, 0x03, 0x11, 0x17,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2e, 0x02, 0x02, 0x01, 0x12, 0x04, 0xdd, 0x03, 0x18, 0x29, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x2e, 0x02, 0x02, 0x03, 0x12, 0x04, 0xdd, 0x03, 0x2c, 0x2d, 0x0a, 0x0c,
    0x0a, 0x02, 0x04, 0x2f, 0x12, 0x06, 0xe0, 0x03, 0x00, 0xe3, 0x03, 0x01, 0x0a, 0x0b, 0x0a, 0x03,
    0x04, 0x2f, 0x01, 0x12, 0x04, 0xe0, 0x03, 0x08, 0x1b, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x2f, 0x02,
    0x00, 0x12, 0x04, 0xe1, 0x03, 0x08, 0x27, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2f, 0x02, 0x00, 0x04,
    0x12, 0x04, 0xe1, 0x03, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2f, 0x02, 0x00, 0x05, 0x12,
    0x04, 0xe1, 0x03, 0x11, 0x16, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2f, 0x02, 0x00, 0x01, 0x12, 0x04,
    0xe1, 0x03, 0x17, 0x22, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2f, 0x02, 0x00, 0x03, 0x12, 0x04, 0xe1,
    0x03, 0x25, 0x26, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x2f, 0x02, 0x01, 0x12, 0x04, 0xe2, 0x03, 0x08,
    0x2a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2f, 0x02, 0x01, 0x04, 0x12, 0x04, 0xe2, 0x03, 0x08, 0x10,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2f, 0x02, 0x01, 0x05, 0x12, 0x04, 0xe2, 0x03, 0x11, 0x17, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x2f, 0x02, 0x01, 0x01, 0x12, 0x04, 0xe2, 0x03, 0x18, 0x25, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x2f, 0x02, 0x01, 0x03, 0x12, 0x04, 0xe2, 0x03, 0x28, 0x29, 0x0a, 0x0c, 0x0a,
    0x02, 0x04, 0x30, 0x12, 0x06, 0xe5, 0x03, 0x00, 0xe7, 0x03, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04,
    0x30, 0x01, 0x12, 0x04, 0xe5, 0x03, 0x08, 0x18, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x30, 0x02, 0x00,
    0x12, 0x04, 0xe6, 0x03, 0x08, 0x26, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x30, 0x02, 0x00, 0x04, 0x12,
    0x04, 0xe6, 0x03, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x30, 0x02, 0x00, 0x05, 0x12, 0x04,
    0xe6, 0x03, 0x11, 0x16, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x30, 0x02, 0x00, 0x01, 0x12, 0x04, 0xe6,
    0x03, 0x17, 0x21, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x30, 0x02, 0x00, 0x03, 0x12, 0x04, 0xe6, 0x03,
    0x24, 0x25, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x31, 0x12, 0x06, 0xe9, 0x03, 0x00, 0xeb, 0x03, 0x01,
    0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x31, 0x01, 0x12, 0x04, 0xe9, 0x03, 0x08, 0x23, 0x0a, 0x0c, 0x0a,
    0x04, 0x04, 0x31, 0x02, 0x00, 0x12, 0x04, 0xea, 0x03, 0x08, 0x2c, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x31, 0x02, 0x00, 0x04, 0x12, 0x04, 0xea, 0x03, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x31,
    0x02, 0x00, 0x05, 0x12, 0x04, 0xea, 0x03, 0x11, 0x16, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x31, 0x02,
    0x00, 0x01, 0x12, 0x04, 0xea, 0x03, 0x17, 0x27, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x31, 0x02, 0x00,
    0x03, 0x12, 0x04, 0xea, 0x03, 0x2a, 0x2b, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x32, 0x12, 0x06, 0xed,
    0x03, 0x00, 0xef, 0x03, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x32, 0x01, 0x12, 0x04, 0xed, 0x03,
    0x08, 0x1a, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x32, 0x02, 0x00, 0x12, 0x04, 0xee, 0x03, 0x08, 0x24,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x32, 0x02, 0x00, 0x04, 0x12, 0x04, 0xee, 0x03, 0x08, 0x10, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x32, 0x02, 0x00, 0x05, 0x12, 0x04, 0xee, 0x03, 0x11, 0x17, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x32, 0x02, 0x00, 0x01, 0x12, 0x04, 0xee, 0x03, 0x18, 0x1f, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x32, 0x02, 0x00, 0x03, 0x12, 0x04, 0xee, 0x03, 0x22, 0x23, 0x0a, 0x0c, 0x0a, 0x02,
    0x04, 0x33, 0x12, 0x06, 0xf1, 0x03, 0x00, 0xf5, 0x03, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x33,
    0x01, 0x12, 0x04, 0xf1, 0x03, 0x08, 0x14, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x33, 0x02, 0x00, 0x12,
    0x04, 0xf2, 0x03, 0x08, 0x49, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x33, 0x02, 0x00, 0x04, 0x12, 0x04,
    0xf2, 0x03, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x33, 0x02, 0x00, 0x06, 0x12, 0x04, 0xf2,
    0x03, 0x11, 0x1e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x33, 0x02, 0x00, 0x01, 0x12, 0x04, 0xf2, 0x03,
    0x1f, 0x25, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x33, 0x02, 0x00, 0x03, 0x12, 0x04, 0xf2, 0x03, 0x28,
    0x29, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x33, 0x02, 0x00, 0x08, 0x12, 0x04, 0xf2, 0x03, 0x2a, 0x48,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x33, 0x02, 0x00, 0x07, 0x12, 0x04, 0xf2, 0x03, 0x35, 0x47, 0x0a,
    0x0c, 0x0a, 0x04, 0x04, 0x33, 0x02, 0x01, 0x12, 0x04, 0xf3, 0x03, 0x08, 0x26, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x33, 0x02, 0x01, 0x04, 0x12, 0x04, 0xf3, 0x03, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x33, 0x02, 0x01, 0x05, 0x12, 0x04, 0xf3, 0x03, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x33, 0x02, 0x01, 0x01, 0x12, 0x04, 0xf3, 0x03, 0x18, 0x21, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x33,
    0x02, 0x01, 0x03, 0x12, 0x04, 0xf3, 0x03, 0x24, 0x25, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x33, 0x02,
    0x02, 0x12, 0x04, 0xf4, 0x03, 0x08, 0x25, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x33, 0x02, 0x02, 0x04,
    0x12, 0x04, 0xf4, 0x03, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x33, 0x02, 0x02, 0x05, 0x12,
    0x04, 0xf4, 0x03, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x33, 0x02, 0x02, 0x01, 0x12, 0x04,
    0xf4, 0x03, 0x18, 0x20, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x33, 0x02, 0x02, 0x03, 0x12, 0x04, 0xf4,
    0x03, 0x23, 0x24, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x34, 0x12, 0x06, 0xf7, 0x03, 0x00, 0xfb, 0x03,
    0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x34, 0x01, 0x12, 0x04, 0xf7, 0x03, 0x08, 0x14, 0x0a, 0x0c,
    0x0a, 0x04, 0x04, 0x34, 0x02, 0x00, 0x12, 0x04, 0xf8, 0x03, 0x08, 0x49, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x34, 0x02, 0x00, 0x04, 0x12, 0x04, 0xf8, 0x03, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x34, 0x02, 0x00, 0x06, 0x12, 0x04, 0xf8, 0x03, 0x11, 0x1e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x34,
    0x02, 0x00, 0x01, 0x12, 0x04, 0xf8, 0x03, 0x1f, 0x25, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x34, 0x02,
    0x00, 0x03, 0x12, 0x04, 0xf8, 0x03, 0x28, 0x29, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x34, 0x02, 0x00,
    0x08, 0x12, 0x04, 0xf8, 0x03, 0x2a, 0x48, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x34, 0x02, 0x00, 0x07,
    0x12, 0x04, 0xf8, 0x03, 0x35, 0x47, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x34, 0x02, 0x01, 0x12, 0x04,
    0xf9, 0x03, 0x08, 0x22, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x34, 0x02, 0x01, 0x04, 0x12, 0x04, 0xf9,
    0x03, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x34, 0x02, 0x01, 0x05, 0x12, 0x04, 0xf9, 0x03,
    0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x34, 0x02, 0x01, 0x01, 0x12, 0x04, 0xf9, 0x03, 0x18,
    0x1d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x34, 0x02, 0x01, 0x03, 0x12, 0x04, 0xf9, 0x03, 0x20, 0x21,
    0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x34, 0x02, 0x02, 0x12, 0x04, 0xfa, 0x03, 0x08, 0x23, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x34, 0x02, 0x02, 0x04, 0x12, 0x04, 0xfa, 0x03, 0x08, 0x10, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x34, 0x02, 0x02, 0x05, 0x12, 0x04, 0xfa, 0x03, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x34, 0x02, 0x02, 0x01, 0x12, 0x04, 0xfa, 0x03, 0x18, 0x1e, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x34, 0x02, 0x02, 0x03, 0x12, 0x04, 0xfa, 0x03, 0x21, 0x22, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x35,
    0x12, 0x06, 0xfd, 0x03, 0x00, 0x80, 0x04, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x35, 0x01, 0x12,
    0x04, 0xfd, 0x03, 0x08, 0x13, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x35, 0x02, 0x00, 0x12, 0x04, 0xfe,
    0x03, 0x08, 0x56, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x35, 0x02, 0x00, 0x04, 0x12, 0x04, 0xfe, 0x03,
    0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x35, 0x02, 0x00, 0x06, 0x12, 0x04, 0xfe, 0x03, 0x11,
    0x23, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x35, 0x02, 0x00, 0x01, 0x12, 0x04, 0xfe, 0x03, 0x24, 0x2c,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x35, 0x02, 0x00, 0x03, 0x12, 0x04, 0xfe, 0x03, 0x2f, 0x30, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x35, 0x02, 0x00, 0x08, 0x12, 0x04, 0xfe, 0x03, 0x31, 0x55, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x35, 0x02, 0x00, 0x07, 0x12, 0x04, 0xfe, 0x03, 0x3c, 0x54, 0x0a, 0x0c, 0x0a,
    0x04, 0x04, 0x35, 0x02, 0x01, 0x12, 0x04, 0xff, 0x03, 0x08, 0x26, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x35, 0x02, 0x01, 0x04, 0x12, 0x04, 0xff, 0x03, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x35,
    0x02, 0x01, 0x05, 0x12, 0x04, 0xff, 0x03, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x35, 0x02,
    0x01, 0x01, 0x12, 0x04, 0xff, 0x03, 0x18, 0x21, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x35, 0x02, 0x01,
    0x03, 0x12, 0x04, 0xff, 0x03, 0x24, 0x25, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x36, 0x12, 0x06, 0x82,
    0x04, 0x00, 0x8f, 0x04, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x36, 0x01, 0x12, 0x04, 0x82, 0x04,
    0x08, 0x13, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x36, 0x02, 0x00, 0x12, 0x04, 0x83, 0x04, 0x08, 0x25,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x36, 0x02, 0x00, 0x04, 0x12, 0x04, 0x83, 0x04, 0x08, 0x10, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x36, 0x02, 0x00, 0x05, 0x12, 0x04, 0x83, 0x04, 0x11, 0x17, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x36, 0x02, 0x00, 0x01, 0x12, 0x04, 0x83, 0x04, 0x18, 0x20, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x36, 0x02, 0x00, 0x03, 0x12, 0x04, 0x83, 0x04, 0x23, 0x24, 0x0a, 0x0c, 0x0a, 0x04,
    0x04, 0x36, 0x02, 0x01, 0x12, 0x04, 0x84, 0x04, 0x08, 0x27, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x36,
    0x02, 0x01, 0x04, 0x12, 0x04, 0x84, 0x04, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x36, 0x02,
    0x01, 0x05, 0x12, 0x04, 0x84, 0x04, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x36, 0x02, 0x01,
    0x01, 0x12, 0x04, 0x84, 0x04, 0x18, 0x22, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x36, 0x02, 0x01, 0x03,
    0x12, 0x04, 0x84, 0x04, 0x25, 0x26, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x36, 0x02, 0x02, 0x12, 0x04,
    0x85, 0x04, 0x08, 0x29, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x36, 0x02, 0x02, 0x04, 0x12, 0x04, 0x85,
    0x04, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x36, 0x02, 0x02, 0x06, 0x12, 0x04, 0x85, 0x04,
    0x11, 0x1d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x36, 0x02, 0x02, 0x01, 0x12, 0x04, 0x85, 0x04, 0x1e,
    0x24, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x36, 0x02, 0x02, 0x03, 0x12, 0x04, 0x85, 0x04, 0x27, 0x28,
    0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x36, 0x02, 0x03, 0x12, 0x04, 0x86, 0x04, 0x08, 0x58, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x36, 0x02, 0x03, 0x04, 0x12, 0x04, 0x86, 0x04, 0x08, 0x10, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x36, 0x02, 0x03, 0x06, 0x12, 0x04, 0x86, 0x04, 0x11, 0x24, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x36, 0x02, 0x03, 0x01, 0x12, 0x04, 0x86, 0x04, 0x25, 0x2b, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x36, 0x02, 0x03, 0x03, 0x12, 0x04, 0x86, 0x04, 0x2e, 0x2f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x36,
    0x02, 0x03, 0x08, 0x12, 0x04, 0x86, 0x04, 0x30, 0x57, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x36, 0x02,
    0x03, 0x07, 0x12, 0x04, 0x86, 0x04, 0x3b, 0x56, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x36, 0x02, 0x04,
    0x12, 0x04, 0x87, 0x04, 0x08, 0x2d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x36, 0x02, 0x04, 0x04, 0x12,
    0x04, 0x87, 0x04, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x36, 0x02, 0x04, 0x05, 0x12, 0x04,
    0x87, 0x04, 0x11, 0x16, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x36, 0x02, 0x04, 0x01, 0x12, 0x04, 0x87,
    0x04, 0x17, 0x28, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x36, 0x02, 0x04, 0x03, 0x12, 0x04, 0x87, 0x04,
    0x2b, 0x2c, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x36, 0x02, 0x05, 0x12, 0x04, 0x88, 0x04, 0x08, 0x2f,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x36, 0x02, 0x05, 0x04, 0x12, 0x04, 0x88, 0x04, 0x08, 0x10, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x36, 0x02, 0x05, 0x05, 0x12, 0x04, 0x88, 0x04, 0x11, 0x16, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x36, 0x02, 0x05, 0x01, 0x12, 0x04, 0x88, 0x04, 0x17, 0x2a, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x36, 0x02, 0x05, 0x03, 0x12, 0x04, 0x88, 0x04, 0x2d, 0x2e, 0x0a, 0x0c, 0x0a, 0x04,
    0x04, 0x36, 0x02, 0x06, 0x12, 0x04, 0x89, 0x04, 0x08, 0x25, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x36,
    0x02, 0x06, 0x04, 0x12, 0x04, 0x89, 0x04, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x36, 0x02,
    0x06, 0x05, 0x12, 0x04, 0x89, 0x04, 0x11, 0x16, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x36, 0x02, 0x06,
    0x01, 0x12, 0x04, 0x89, 0x04, 0x17, 0x20, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x36, 0x02, 0x06, 0x03,
    0x12, 0x04, 0x89, 0x04, 0x23, 0x24, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x36, 0x02, 0x07, 0x12, 0x04,
    0x8a, 0x04, 0x08, 0x2a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x36, 0x02, 0x07, 0x04, 0x12, 0x04, 0x8a,
    0x04, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x36, 0x02, 0x07, 0x05, 0x12, 0x04, 0x8a, 0x04,
    0x11, 0x16, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x36, 0x02, 0x07, 0x01, 0x12, 0x04, 0x8a, 0x04, 0x17,
    0x25, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x36, 0x02, 0x07, 0x03, 0x12, 0x04, 0x8a, 0x04, 0x28, 0x29,
    0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x36, 0x02, 0x08, 0x12, 0x04, 0x8b, 0x04, 0x08, 0x2a, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x36, 0x02, 0x08, 0x04, 0x12, 0x04, 0x8b, 0x04, 0x08, 0x10, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x36, 0x02, 0x08, 0x05, 0x12, 0x04, 0x8b, 0x04, 0x11, 0x16, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x36, 0x02, 0x08, 0x01, 0x12, 0x04, 0x8b, 0x04, 0x17, 0x25, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x36, 0x02, 0x08, 0x03, 0x12, 0x04, 0x8b, 0x04, 0x28, 0x29, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x36,
    0x02, 0x09, 0x12, 0x04, 0x8c, 0x04, 0x08, 0x2b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x36, 0x02, 0x09,
    0x04, 0x12, 0x04, 0x8c, 0x04, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x36, 0x02, 0x09, 0x05,
    0x12, 0x04, 0x8c, 0x04, 0x11, 0x16, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x36, 0x02, 0x09, 0x01, 0x12,
    0x04, 0x8c, 0x04, 0x17, 0x25, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x36, 0x02, 0x09, 0x03, 0x12, 0x04,
    0x8c, 0x04, 0x28, 0x2a, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x36, 0x02, 0x0a, 0x12, 0x04, 0x8d, 0x04,
    0x08, 0x28, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x36, 0x02, 0x0a, 0x04, 0x12, 0x04, 0x8d, 0x04, 0x08,
    0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x36, 0x02, 0x0a, 0x05, 0x12, 0x04, 0x8d, 0x04, 0x11, 0x16,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x36, 0x02, 0x0a, 0x01, 0x12, 0x04, 0x8d, 0x04, 0x17, 0x22, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x36, 0x02, 0x0a, 0x03, 0x12, 0x04, 0x8d, 0x04, 0x25, 0x27, 0x0a, 0x0c,
    0x0a, 0x04, 0x04, 0x36, 0x02, 0x0b, 0x12, 0x04, 0x8e, 0x04, 0x08, 0x28, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x36, 0x02, 0x0b, 0x04, 0x12, 0x04, 0x8e, 0x04, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x36, 0x02, 0x0b, 0x05, 0x12, 0x04, 0x8e, 0x04, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x36,
    0x02, 0x0b, 0x01, 0x12, 0x04, 0x8e, 0x04, 0x18, 0x22, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x36, 0x02,
    0x0b, 0x03, 0x12, 0x04, 0x8e, 0x04, 0x25, 0x27, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x37, 0x12, 0x06,
    0x91, 0x04, 0x00, 0x96, 0x04, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x37, 0x01, 0x12, 0x04, 0x91,
    0x04, 0x08, 0x22, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x37, 0x02, 0x00, 0x12, 0x04, 0x92, 0x04, 0x08,
    0x52, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x37, 0x02, 0x00, 0x04, 0x12, 0x04, 0x92, 0x04, 0x08, 0x10,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x37, 0x02, 0x00, 0x06, 0x12, 0x04, 0x92, 0x04, 0x11, 0x27, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x37, 0x02, 0x00, 0x01, 0x12, 0x04, 0x92, 0x04, 0x28, 0x31, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x37, 0x02, 0x00, 0x03, 0x12, 0x04, 0x92, 0x04, 0x34, 0x35, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x37, 0x02, 0x00, 0x08, 0x12, 0x04, 0x92, 0x04, 0x36, 0x51, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x37, 0x02, 0x00, 0x07, 0x12, 0x04, 0x92, 0x04, 0x41, 0x50, 0x0a, 0x0c, 0x0a, 0x04, 0x04,
    0x37, 0x02, 0x01, 0x12, 0x04, 0x93, 0x04, 0x08, 0x21, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x37, 0x02,
    0x01, 0x04, 0x12, 0x04, 0x93, 0x04, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x37, 0x02, 0x01,
    0x05, 0x12, 0x04, 0x93, 0x04, 0x11, 0x16, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x37, 0x02, 0x01, 0x01,
    0x12, 0x04, 0x93, 0x04, 0x17, 0x1c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x37, 0x02, 0x01, 0x03, 0x12,
    0x04, 0x93, 0x04, 0x1f, 0x20, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x37, 0x02, 0x02, 0x12, 0x04, 0x94,
    0x04, 0x08, 0x23, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x37, 0x02, 0x02, 0x04, 0x12, 0x04, 0x94, 0x04,
    0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x37, 0x02, 0x02, 0x05, 0x12, 0x04, 0x94, 0x04, 0x11,
    0x16, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x37, 0x02, 0x02, 0x01, 0x12, 0x04, 0x94, 0x04, 0x17, 0x1e,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x37, 0x02, 0x02, 0x03, 0x12, 0x04, 0x94, 0x04, 0x21, 0x22, 0x0a,
    0x0c, 0x0a, 0x04, 0x04, 0x37, 0x02, 0x03, 0x12, 0x04, 0x95, 0x04, 0x08, 0x22, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x37, 0x02, 0x03, 0x04, 0x12, 0x04, 0x95, 0x04, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x37, 0x02, 0x03, 0x05, 0x12, 0x04, 0x95, 0x04, 0x11, 0x16, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x37, 0x02, 0x03, 0x01, 0x12, 0x04, 0x95, 0x04, 0x17, 0x1d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x37,
    0x02, 0x03, 0x03, 0x12, 0x04, 0x95, 0x04, 0x20, 0x21, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x38, 0x12,
    0x06, 0x98, 0x04, 0x00, 0x9d, 0x04, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x38, 0x01, 0x12, 0x04,
    0x98, 0x04, 0x08, 0x1a, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x38, 0x02, 0x00, 0x12, 0x04, 0x99, 0x04,
    0x08, 0x55, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x38, 0x02, 0x00, 0x04, 0x12, 0x04, 0x99, 0x04, 0x08,
    0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x38, 0x02, 0x00, 0x06, 0x12, 0x04, 0x99, 0x04, 0x11, 0x24,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x38, 0x02, 0x00, 0x01, 0x12, 0x04, 0x99, 0x04, 0x25, 0x2e, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x38, 0x02, 0x00, 0x03, 0x12, 0x04, 0x99, 0x04, 0x31, 0x32, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x38, 0x02, 0x00, 0x08, 0x12, 0x04, 0x99, 0x04, 0x33, 0x54, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x38, 0x02, 0x00, 0x07, 0x12, 0x04, 0x99, 0x04, 0x3e, 0x53, 0x0a, 0x0c, 0x0a, 0x04,
    0x04, 0x38, 0x02, 0x01, 0x12, 0x04, 0x9a, 0x04, 0x08, 0x28, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x38,
    0x02, 0x01, 0x04, 0x12, 0x04, 0x9a, 0x04, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x38, 0x02,
    0x01, 0x06, 0x12, 0x04, 0x9a, 0x04, 0x11, 0x1d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x38, 0x02, 0x01,
    0x01, 0x12, 0x04, 0x9a, 0x04, 0x1e, 0x23, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x38, 0x02, 0x01, 0x03,
    0x12, 0x04, 0x9a, 0x04, 0x26, 0x27, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x38, 0x02, 0x02, 0x12, 0x04,
    0x9b, 0x04, 0x08, 0x43, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x38, 0x02, 0x02, 0x04, 0x12, 0x04, 0x9b,
    0x04, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x38, 0x02, 0x02, 0x06, 0x12, 0x04, 0x9b, 0x04,
    0x11, 0x2c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x38, 0x02, 0x02, 0x01, 0x12, 0x04, 0x9b, 0x04, 0x2d,
    0x3e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x38, 0x02, 0x02, 0x03, 0x12, 0x04, 0x9b, 0x04, 0x41, 0x42,
    0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x38, 0x02, 0x03, 0x12, 0x04, 0x9c, 0x04, 0x08, 0x2b, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x38, 0x02, 0x03, 0x04, 0x12, 0x04, 0x9c, 0x04, 0x08, 0x10, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x38, 0x02, 0x03, 0x05, 0x12, 0x04, 0x9c, 0x04, 0x11, 0x16, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x38, 0x02, 0x03, 0x01, 0x12, 0x04, 0x9c, 0x04, 0x17, 0x26, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x38, 0x02, 0x03, 0x03, 0x12, 0x04, 0x9c, 0x04, 0x29, 0x2a, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x39,
    0x12, 0x06, 0x9f, 0x04, 0x00, 0xa1, 0x04, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x39, 0x01, 0x12,
    0x04, 0x9f, 0x04, 0x08, 0x15, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x39, 0x02, 0x00, 0x12, 0x04, 0xa0,
    0x04, 0x08, 0x26, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x39, 0x02, 0x00, 0x04, 0x12, 0x04, 0xa0, 0x04,
    0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x39, 0x02, 0x00, 0x05, 0x12, 0x04, 0xa0, 0x04, 0x11,
    0x16, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x39, 0x02, 0x00, 0x01, 0x12, 0x04, 0xa0, 0x04, 0x17, 0x21,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x39, 0x02, 0x00, 0x03, 0x12, 0x04, 0xa0, 0x04, 0x24, 0x25,
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
