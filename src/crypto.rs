// This file is generated. Do not edit

#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(unused_imports)]

use protobuf::Message as Message_imported_for_functions;
use protobuf::ProtobufEnum as ProtobufEnum_imported_for_functions;

#[derive(Clone,Default)]
pub struct PublicKey {
    // message fields
    Type: ::std::option::Option<KeyType>,
    Data: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl PublicKey {
    pub fn new() -> PublicKey {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static PublicKey {
        static mut instance: ::protobuf::lazy::Lazy<PublicKey> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const PublicKey,
        };
        unsafe {
            instance.get(|| {
                PublicKey {
                    Type: ::std::option::Option::None,
                    Data: ::protobuf::SingularField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required .crypto.pb.KeyType Type = 1;

    pub fn clear_Type(&mut self) {
        self.Type = ::std::option::Option::None;
    }

    pub fn has_Type(&self) -> bool {
        self.Type.is_some()
    }

    // Param is passed by value, moved
    pub fn set_Type(&mut self, v: KeyType) {
        self.Type = ::std::option::Option::Some(v);
    }

    pub fn get_Type<'a>(&self) -> KeyType {
        self.Type.unwrap_or(KeyType::RSA)
    }

    // required bytes Data = 2;

    pub fn clear_Data(&mut self) {
        self.Data.clear();
    }

    pub fn has_Data(&self) -> bool {
        self.Data.is_some()
    }

    // Param is passed by value, moved
    pub fn set_Data(&mut self, v: ::std::vec::Vec<u8>) {
        self.Data = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_Data<'a>(&'a mut self) -> &'a mut ::std::vec::Vec<u8> {
        if self.Data.is_none() {
            self.Data.set_default();
        };
        self.Data.as_mut().unwrap()
    }

    // Take field
    pub fn take_Data(&mut self) -> ::std::vec::Vec<u8> {
        self.Data.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_Data<'a>(&'a self) -> &'a [u8] {
        match self.Data.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }
}

impl ::protobuf::Message for PublicKey {
    fn is_initialized(&self) -> bool {
        if self.Type.is_none() {
            return false;
        };
        if self.Data.is_none() {
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
                    self.Type = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.Data.set_default();
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
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.Type.iter() {
            my_size += ::protobuf::rt::enum_size(1, *value);
        };
        for value in self.Data.iter() {
            my_size += ::protobuf::rt::bytes_size(2, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.Type {
            try!(os.write_enum(1, v as i32));
        };
        if let Some(v) = self.Data.as_ref() {
            try!(os.write_bytes(2, &v));
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

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<PublicKey>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for PublicKey {
    fn new() -> PublicKey {
        PublicKey::new()
    }

    fn descriptor_static(_: ::std::option::Option<PublicKey>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_enum_accessor(
                    "Type",
                    PublicKey::has_Type,
                    PublicKey::get_Type,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "Data",
                    PublicKey::has_Data,
                    PublicKey::get_Data,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<PublicKey>(
                    "PublicKey",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for PublicKey {
    fn clear(&mut self) {
        self.clear_Type();
        self.clear_Data();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for PublicKey {
    fn eq(&self, other: &PublicKey) -> bool {
        self.Type == other.Type &&
        self.Data == other.Data &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for PublicKey {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct PrivateKey {
    // message fields
    Type: ::std::option::Option<KeyType>,
    Data: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl PrivateKey {
    pub fn new() -> PrivateKey {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static PrivateKey {
        static mut instance: ::protobuf::lazy::Lazy<PrivateKey> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const PrivateKey,
        };
        unsafe {
            instance.get(|| {
                PrivateKey {
                    Type: ::std::option::Option::None,
                    Data: ::protobuf::SingularField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required .crypto.pb.KeyType Type = 1;

    pub fn clear_Type(&mut self) {
        self.Type = ::std::option::Option::None;
    }

    pub fn has_Type(&self) -> bool {
        self.Type.is_some()
    }

    // Param is passed by value, moved
    pub fn set_Type(&mut self, v: KeyType) {
        self.Type = ::std::option::Option::Some(v);
    }

    pub fn get_Type<'a>(&self) -> KeyType {
        self.Type.unwrap_or(KeyType::RSA)
    }

    // required bytes Data = 2;

    pub fn clear_Data(&mut self) {
        self.Data.clear();
    }

    pub fn has_Data(&self) -> bool {
        self.Data.is_some()
    }

    // Param is passed by value, moved
    pub fn set_Data(&mut self, v: ::std::vec::Vec<u8>) {
        self.Data = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_Data<'a>(&'a mut self) -> &'a mut ::std::vec::Vec<u8> {
        if self.Data.is_none() {
            self.Data.set_default();
        };
        self.Data.as_mut().unwrap()
    }

    // Take field
    pub fn take_Data(&mut self) -> ::std::vec::Vec<u8> {
        self.Data.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_Data<'a>(&'a self) -> &'a [u8] {
        match self.Data.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }
}

impl ::protobuf::Message for PrivateKey {
    fn is_initialized(&self) -> bool {
        if self.Type.is_none() {
            return false;
        };
        if self.Data.is_none() {
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
                    self.Type = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.Data.set_default();
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
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.Type.iter() {
            my_size += ::protobuf::rt::enum_size(1, *value);
        };
        for value in self.Data.iter() {
            my_size += ::protobuf::rt::bytes_size(2, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.Type {
            try!(os.write_enum(1, v as i32));
        };
        if let Some(v) = self.Data.as_ref() {
            try!(os.write_bytes(2, &v));
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

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<PrivateKey>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for PrivateKey {
    fn new() -> PrivateKey {
        PrivateKey::new()
    }

    fn descriptor_static(_: ::std::option::Option<PrivateKey>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_enum_accessor(
                    "Type",
                    PrivateKey::has_Type,
                    PrivateKey::get_Type,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "Data",
                    PrivateKey::has_Data,
                    PrivateKey::get_Data,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<PrivateKey>(
                    "PrivateKey",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for PrivateKey {
    fn clear(&mut self) {
        self.clear_Type();
        self.clear_Data();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for PrivateKey {
    fn eq(&self, other: &PrivateKey) -> bool {
        self.Type == other.Type &&
        self.Data == other.Data &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for PrivateKey {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,PartialEq,Eq,Debug)]
pub enum KeyType {
    RSA = 0,
}

impl ::protobuf::ProtobufEnum for KeyType {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<KeyType> {
        match value {
            0 => ::std::option::Option::Some(KeyType::RSA),
            _ => ::std::option::Option::None
        }
    }

    fn enum_descriptor_static(_: Option<KeyType>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("KeyType", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for KeyType {
}

static file_descriptor_proto_data: &'static [u8] = &[
    0x0a, 0x0c, 0x63, 0x72, 0x79, 0x70, 0x74, 0x6f, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x09,
    0x63, 0x72, 0x79, 0x70, 0x74, 0x6f, 0x2e, 0x70, 0x62, 0x22, 0x3b, 0x0a, 0x09, 0x50, 0x75, 0x62,
    0x6c, 0x69, 0x63, 0x4b, 0x65, 0x79, 0x12, 0x20, 0x0a, 0x04, 0x54, 0x79, 0x70, 0x65, 0x18, 0x01,
    0x20, 0x02, 0x28, 0x0e, 0x32, 0x12, 0x2e, 0x63, 0x72, 0x79, 0x70, 0x74, 0x6f, 0x2e, 0x70, 0x62,
    0x2e, 0x4b, 0x65, 0x79, 0x54, 0x79, 0x70, 0x65, 0x12, 0x0c, 0x0a, 0x04, 0x44, 0x61, 0x74, 0x61,
    0x18, 0x02, 0x20, 0x02, 0x28, 0x0c, 0x22, 0x3c, 0x0a, 0x0a, 0x50, 0x72, 0x69, 0x76, 0x61, 0x74,
    0x65, 0x4b, 0x65, 0x79, 0x12, 0x20, 0x0a, 0x04, 0x54, 0x79, 0x70, 0x65, 0x18, 0x01, 0x20, 0x02,
    0x28, 0x0e, 0x32, 0x12, 0x2e, 0x63, 0x72, 0x79, 0x70, 0x74, 0x6f, 0x2e, 0x70, 0x62, 0x2e, 0x4b,
    0x65, 0x79, 0x54, 0x79, 0x70, 0x65, 0x12, 0x0c, 0x0a, 0x04, 0x44, 0x61, 0x74, 0x61, 0x18, 0x02,
    0x20, 0x02, 0x28, 0x0c, 0x2a, 0x12, 0x0a, 0x07, 0x4b, 0x65, 0x79, 0x54, 0x79, 0x70, 0x65, 0x12,
    0x07, 0x0a, 0x03, 0x52, 0x53, 0x41, 0x10, 0x00, 0x4a, 0x97, 0x03, 0x0a, 0x06, 0x12, 0x04, 0x00,
    0x00, 0x0e, 0x01, 0x0a, 0x08, 0x0a, 0x01, 0x02, 0x12, 0x03, 0x00, 0x08, 0x11, 0x0a, 0x0a, 0x0a,
    0x02, 0x05, 0x00, 0x12, 0x04, 0x02, 0x00, 0x04, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x05, 0x00, 0x01,
    0x12, 0x03, 0x02, 0x05, 0x0c, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x00, 0x12, 0x03, 0x03,
    0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x03, 0x08, 0x0b,
    0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x00, 0x02, 0x12, 0x03, 0x03, 0x0e, 0x0f, 0x0a, 0x0a,
    0x0a, 0x02, 0x04, 0x00, 0x12, 0x04, 0x06, 0x00, 0x09, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00,
    0x01, 0x12, 0x03, 0x06, 0x08, 0x11, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03,
    0x07, 0x08, 0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x04, 0x12, 0x03, 0x07, 0x08,
    0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x06, 0x12, 0x03, 0x07, 0x11, 0x18, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x07, 0x19, 0x1d, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x07, 0x20, 0x21, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x00, 0x02, 0x01, 0x12, 0x03, 0x08, 0x08, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01,
    0x04, 0x12, 0x03, 0x08, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x05, 0x12,
    0x03, 0x08, 0x11, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x08,
    0x17, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x03, 0x12, 0x03, 0x08, 0x1e, 0x1f,
    0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x01, 0x12, 0x04, 0x0b, 0x00, 0x0e, 0x01, 0x0a, 0x0a, 0x0a, 0x03,
    0x04, 0x01, 0x01, 0x12, 0x03, 0x0b, 0x08, 0x12, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x00,
    0x12, 0x03, 0x0c, 0x08, 0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x04, 0x12, 0x03,
    0x0c, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x06, 0x12, 0x03, 0x0c, 0x11,
    0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x01, 0x12, 0x03, 0x0c, 0x19, 0x1d, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x03, 0x12, 0x03, 0x0c, 0x20, 0x21, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x01, 0x02, 0x01, 0x12, 0x03, 0x0d, 0x08, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01,
    0x02, 0x01, 0x04, 0x12, 0x03, 0x0d, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01,
    0x05, 0x12, 0x03, 0x0d, 0x11, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x01, 0x12,
    0x03, 0x0d, 0x17, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x03, 0x12, 0x03, 0x0d,
    0x1e, 0x1f,
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
