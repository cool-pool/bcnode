// This file is generated. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy)]

#![cfg_attr(rustfmt, rustfmt_skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unsafe_code)]
#![allow(unused_imports)]
#![allow(unused_results)]

use protobuf::Message as Message_imported_for_functions;
use protobuf::ProtobufEnum as ProtobufEnum_imported_for_functions;

#[derive(PartialEq,Clone,Default)]
pub struct DbValue {
    // message fields
    pub field_type: ::std::string::String,
    pub data: ::std::vec::Vec<u8>,
    pub version: i32,
    pub is_native: bool,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for DbValue {}

impl DbValue {
    pub fn new() -> DbValue {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static DbValue {
        static mut instance: ::protobuf::lazy::Lazy<DbValue> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const DbValue,
        };
        unsafe {
            instance.get(DbValue::new)
        }
    }

    // string type = 1;

    pub fn clear_field_type(&mut self) {
        self.field_type.clear();
    }

    // Param is passed by value, moved
    pub fn set_field_type(&mut self, v: ::std::string::String) {
        self.field_type = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_field_type(&mut self) -> &mut ::std::string::String {
        &mut self.field_type
    }

    // Take field
    pub fn take_field_type(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.field_type, ::std::string::String::new())
    }

    pub fn get_field_type(&self) -> &str {
        &self.field_type
    }

    fn get_field_type_for_reflect(&self) -> &::std::string::String {
        &self.field_type
    }

    fn mut_field_type_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.field_type
    }

    // bytes data = 2;

    pub fn clear_data(&mut self) {
        self.data.clear();
    }

    // Param is passed by value, moved
    pub fn set_data(&mut self, v: ::std::vec::Vec<u8>) {
        self.data = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_data(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.data
    }

    // Take field
    pub fn take_data(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.data, ::std::vec::Vec::new())
    }

    pub fn get_data(&self) -> &[u8] {
        &self.data
    }

    fn get_data_for_reflect(&self) -> &::std::vec::Vec<u8> {
        &self.data
    }

    fn mut_data_for_reflect(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.data
    }

    // int32 version = 3;

    pub fn clear_version(&mut self) {
        self.version = 0;
    }

    // Param is passed by value, moved
    pub fn set_version(&mut self, v: i32) {
        self.version = v;
    }

    pub fn get_version(&self) -> i32 {
        self.version
    }

    fn get_version_for_reflect(&self) -> &i32 {
        &self.version
    }

    fn mut_version_for_reflect(&mut self) -> &mut i32 {
        &mut self.version
    }

    // bool is_native = 4;

    pub fn clear_is_native(&mut self) {
        self.is_native = false;
    }

    // Param is passed by value, moved
    pub fn set_is_native(&mut self, v: bool) {
        self.is_native = v;
    }

    pub fn get_is_native(&self) -> bool {
        self.is_native
    }

    fn get_is_native_for_reflect(&self) -> &bool {
        &self.is_native
    }

    fn mut_is_native_for_reflect(&mut self) -> &mut bool {
        &mut self.is_native
    }
}

impl ::protobuf::Message for DbValue {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.field_type)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.data)?;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.version = tmp;
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.is_native = tmp;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if !self.field_type.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.field_type);
        }
        if !self.data.is_empty() {
            my_size += ::protobuf::rt::bytes_size(2, &self.data);
        }
        if self.version != 0 {
            my_size += ::protobuf::rt::value_size(3, self.version, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.is_native != false {
            my_size += 2;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.field_type.is_empty() {
            os.write_string(1, &self.field_type)?;
        }
        if !self.data.is_empty() {
            os.write_bytes(2, &self.data)?;
        }
        if self.version != 0 {
            os.write_int32(3, self.version)?;
        }
        if self.is_native != false {
            os.write_bool(4, self.is_native)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for DbValue {
    fn new() -> DbValue {
        DbValue::new()
    }

    fn descriptor_static(_: ::std::option::Option<DbValue>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "type",
                    DbValue::get_field_type_for_reflect,
                    DbValue::mut_field_type_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "data",
                    DbValue::get_data_for_reflect,
                    DbValue::mut_data_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "version",
                    DbValue::get_version_for_reflect,
                    DbValue::mut_version_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "is_native",
                    DbValue::get_is_native_for_reflect,
                    DbValue::mut_is_native_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<DbValue>(
                    "DbValue",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for DbValue {
    fn clear(&mut self) {
        self.clear_field_type();
        self.clear_data();
        self.clear_version();
        self.clear_is_native();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for DbValue {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for DbValue {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct GetObject {
    // message fields
    pub collection: ::std::string::String,
    pub field_type: i32,
    pub key: ::std::string::String,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for GetObject {}

impl GetObject {
    pub fn new() -> GetObject {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static GetObject {
        static mut instance: ::protobuf::lazy::Lazy<GetObject> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const GetObject,
        };
        unsafe {
            instance.get(GetObject::new)
        }
    }

    // string collection = 1;

    pub fn clear_collection(&mut self) {
        self.collection.clear();
    }

    // Param is passed by value, moved
    pub fn set_collection(&mut self, v: ::std::string::String) {
        self.collection = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_collection(&mut self) -> &mut ::std::string::String {
        &mut self.collection
    }

    // Take field
    pub fn take_collection(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.collection, ::std::string::String::new())
    }

    pub fn get_collection(&self) -> &str {
        &self.collection
    }

    fn get_collection_for_reflect(&self) -> &::std::string::String {
        &self.collection
    }

    fn mut_collection_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.collection
    }

    // int32 type = 2;

    pub fn clear_field_type(&mut self) {
        self.field_type = 0;
    }

    // Param is passed by value, moved
    pub fn set_field_type(&mut self, v: i32) {
        self.field_type = v;
    }

    pub fn get_field_type(&self) -> i32 {
        self.field_type
    }

    fn get_field_type_for_reflect(&self) -> &i32 {
        &self.field_type
    }

    fn mut_field_type_for_reflect(&mut self) -> &mut i32 {
        &mut self.field_type
    }

    // string key = 3;

    pub fn clear_key(&mut self) {
        self.key.clear();
    }

    // Param is passed by value, moved
    pub fn set_key(&mut self, v: ::std::string::String) {
        self.key = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_key(&mut self) -> &mut ::std::string::String {
        &mut self.key
    }

    // Take field
    pub fn take_key(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.key, ::std::string::String::new())
    }

    pub fn get_key(&self) -> &str {
        &self.key
    }

    fn get_key_for_reflect(&self) -> &::std::string::String {
        &self.key
    }

    fn mut_key_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.key
    }
}

impl ::protobuf::Message for GetObject {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.collection)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.field_type = tmp;
                },
                3 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.key)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if !self.collection.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.collection);
        }
        if self.field_type != 0 {
            my_size += ::protobuf::rt::value_size(2, self.field_type, ::protobuf::wire_format::WireTypeVarint);
        }
        if !self.key.is_empty() {
            my_size += ::protobuf::rt::string_size(3, &self.key);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.collection.is_empty() {
            os.write_string(1, &self.collection)?;
        }
        if self.field_type != 0 {
            os.write_int32(2, self.field_type)?;
        }
        if !self.key.is_empty() {
            os.write_string(3, &self.key)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for GetObject {
    fn new() -> GetObject {
        GetObject::new()
    }

    fn descriptor_static(_: ::std::option::Option<GetObject>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "collection",
                    GetObject::get_collection_for_reflect,
                    GetObject::mut_collection_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "type",
                    GetObject::get_field_type_for_reflect,
                    GetObject::mut_field_type_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "key",
                    GetObject::get_key_for_reflect,
                    GetObject::mut_key_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<GetObject>(
                    "GetObject",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for GetObject {
    fn clear(&mut self) {
        self.clear_collection();
        self.clear_field_type();
        self.clear_key();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for GetObject {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for GetObject {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct PutObject {
    // message fields
    pub collection: ::std::string::String,
    pub field_type: i32,
    pub key: ::std::string::String,
    pub data: ::std::string::String,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for PutObject {}

impl PutObject {
    pub fn new() -> PutObject {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static PutObject {
        static mut instance: ::protobuf::lazy::Lazy<PutObject> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const PutObject,
        };
        unsafe {
            instance.get(PutObject::new)
        }
    }

    // string collection = 1;

    pub fn clear_collection(&mut self) {
        self.collection.clear();
    }

    // Param is passed by value, moved
    pub fn set_collection(&mut self, v: ::std::string::String) {
        self.collection = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_collection(&mut self) -> &mut ::std::string::String {
        &mut self.collection
    }

    // Take field
    pub fn take_collection(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.collection, ::std::string::String::new())
    }

    pub fn get_collection(&self) -> &str {
        &self.collection
    }

    fn get_collection_for_reflect(&self) -> &::std::string::String {
        &self.collection
    }

    fn mut_collection_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.collection
    }

    // int32 type = 2;

    pub fn clear_field_type(&mut self) {
        self.field_type = 0;
    }

    // Param is passed by value, moved
    pub fn set_field_type(&mut self, v: i32) {
        self.field_type = v;
    }

    pub fn get_field_type(&self) -> i32 {
        self.field_type
    }

    fn get_field_type_for_reflect(&self) -> &i32 {
        &self.field_type
    }

    fn mut_field_type_for_reflect(&mut self) -> &mut i32 {
        &mut self.field_type
    }

    // string key = 3;

    pub fn clear_key(&mut self) {
        self.key.clear();
    }

    // Param is passed by value, moved
    pub fn set_key(&mut self, v: ::std::string::String) {
        self.key = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_key(&mut self) -> &mut ::std::string::String {
        &mut self.key
    }

    // Take field
    pub fn take_key(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.key, ::std::string::String::new())
    }

    pub fn get_key(&self) -> &str {
        &self.key
    }

    fn get_key_for_reflect(&self) -> &::std::string::String {
        &self.key
    }

    fn mut_key_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.key
    }

    // string data = 4;

    pub fn clear_data(&mut self) {
        self.data.clear();
    }

    // Param is passed by value, moved
    pub fn set_data(&mut self, v: ::std::string::String) {
        self.data = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_data(&mut self) -> &mut ::std::string::String {
        &mut self.data
    }

    // Take field
    pub fn take_data(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.data, ::std::string::String::new())
    }

    pub fn get_data(&self) -> &str {
        &self.data
    }

    fn get_data_for_reflect(&self) -> &::std::string::String {
        &self.data
    }

    fn mut_data_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.data
    }
}

impl ::protobuf::Message for PutObject {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.collection)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.field_type = tmp;
                },
                3 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.key)?;
                },
                4 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.data)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if !self.collection.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.collection);
        }
        if self.field_type != 0 {
            my_size += ::protobuf::rt::value_size(2, self.field_type, ::protobuf::wire_format::WireTypeVarint);
        }
        if !self.key.is_empty() {
            my_size += ::protobuf::rt::string_size(3, &self.key);
        }
        if !self.data.is_empty() {
            my_size += ::protobuf::rt::string_size(4, &self.data);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.collection.is_empty() {
            os.write_string(1, &self.collection)?;
        }
        if self.field_type != 0 {
            os.write_int32(2, self.field_type)?;
        }
        if !self.key.is_empty() {
            os.write_string(3, &self.key)?;
        }
        if !self.data.is_empty() {
            os.write_string(4, &self.data)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for PutObject {
    fn new() -> PutObject {
        PutObject::new()
    }

    fn descriptor_static(_: ::std::option::Option<PutObject>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "collection",
                    PutObject::get_collection_for_reflect,
                    PutObject::mut_collection_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "type",
                    PutObject::get_field_type_for_reflect,
                    PutObject::mut_field_type_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "key",
                    PutObject::get_key_for_reflect,
                    PutObject::mut_key_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "data",
                    PutObject::get_data_for_reflect,
                    PutObject::mut_data_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<PutObject>(
                    "PutObject",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for PutObject {
    fn clear(&mut self) {
        self.clear_collection();
        self.clear_field_type();
        self.clear_key();
        self.clear_data();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for PutObject {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for PutObject {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x08db.proto\x12\x02bc\"h\n\x07DbValue\x12\x12\n\x04type\x18\x01\x20\
    \x01(\tR\x04type\x12\x12\n\x04data\x18\x02\x20\x01(\x0cR\x04data\x12\x18\
    \n\x07version\x18\x03\x20\x01(\x05R\x07version\x12\x1b\n\tis_native\x18\
    \x04\x20\x01(\x08R\x08isNative\"Q\n\tGetObject\x12\x1e\n\ncollection\x18\
    \x01\x20\x01(\tR\ncollection\x12\x12\n\x04type\x18\x02\x20\x01(\x05R\x04\
    type\x12\x10\n\x03key\x18\x03\x20\x01(\tR\x03key\"e\n\tPutObject\x12\x1e\
    \n\ncollection\x18\x01\x20\x01(\tR\ncollection\x12\x12\n\x04type\x18\x02\
    \x20\x01(\x05R\x04type\x12\x10\n\x03key\x18\x03\x20\x01(\tR\x03key\x12\
    \x12\n\x04data\x18\x04\x20\x01(\tR\x04datab\x06proto3\
";

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
