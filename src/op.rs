// This file is generated. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy)]

#![cfg_attr(rustfmt, rustfmt_skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unsafe_code)]
#![allow(unused_imports)]
#![allow(unused_results)]

use objecthash::{self, ObjectHash, ObjectHasher};
use protobuf::Message as Message_imported_for_functions;
use protobuf::ProtobufEnum as ProtobufEnum_imported_for_functions;

#[derive(PartialEq,Clone,Default)]
pub struct Op {
    // message fields
    pub optype: Type,
    pub path: ::std::string::String,
    object: ::protobuf::SingularPtrField<super::object::Object>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Op {}

impl Op {
    pub fn new() -> Op {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Op {
        static mut instance: ::protobuf::lazy::Lazy<Op> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Op,
        };
        unsafe { instance.get(Op::new) }
    }

    // .ithos.Type optype = 1;

    pub fn clear_optype(&mut self) {
        self.optype = Type::ADD;
    }

    // Param is passed by value, moved
    pub fn set_optype(&mut self, v: Type) {
        self.optype = v;
    }

    pub fn get_optype(&self) -> Type {
        self.optype
    }

    fn get_optype_for_reflect(&self) -> &Type {
        &self.optype
    }

    fn mut_optype_for_reflect(&mut self) -> &mut Type {
        &mut self.optype
    }

    // string path = 2;

    pub fn clear_path(&mut self) {
        self.path.clear();
    }

    // Param is passed by value, moved
    pub fn set_path(&mut self, v: ::std::string::String) {
        self.path = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_path(&mut self) -> &mut ::std::string::String {
        &mut self.path
    }

    // Take field
    pub fn take_path(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.path, ::std::string::String::new())
    }

    pub fn get_path(&self) -> &str {
        &self.path
    }

    fn get_path_for_reflect(&self) -> &::std::string::String {
        &self.path
    }

    fn mut_path_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.path
    }

    // .ithos.Object object = 3;

    pub fn clear_object(&mut self) {
        self.object.clear();
    }

    pub fn has_object(&self) -> bool {
        self.object.is_some()
    }

    // Param is passed by value, moved
    pub fn set_object(&mut self, v: super::object::Object) {
        self.object = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_object(&mut self) -> &mut super::object::Object {
        if self.object.is_none() {
            self.object.set_default();
        };
        self.object.as_mut().unwrap()
    }

    // Take field
    pub fn take_object(&mut self) -> super::object::Object {
        self.object.take().unwrap_or_else(|| super::object::Object::new())
    }

    pub fn get_object(&self) -> &super::object::Object {
        self.object.as_ref().unwrap_or_else(|| super::object::Object::default_instance())
    }

    fn get_object_for_reflect(&self) -> &::protobuf::SingularPtrField<super::object::Object> {
        &self.object
    }

    fn mut_object_for_reflect(&mut self)
                              -> &mut ::protobuf::SingularPtrField<super::object::Object> {
        &mut self.object
    }
}

impl ::protobuf::Message for Op {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self,
                  is: &mut ::protobuf::CodedInputStream)
                  -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_enum()?;
                    self.optype = tmp;
                }
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type,
                                                                     is,
                                                                     &mut self.path)?;
                }
                3 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.object)?;
                }
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number,
                                                               wire_type,
                                                               is,
                                                               self.mut_unknown_fields())?;
                }
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if self.optype != Type::ADD {
            my_size += ::protobuf::rt::enum_size(1, self.optype);
        };
        if self.path != ::std::string::String::new() {
            my_size += ::protobuf::rt::string_size(2, &self.path);
        };
        if let Some(v) = self.object.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self,
                                  os: &mut ::protobuf::CodedOutputStream)
                                  -> ::protobuf::ProtobufResult<()> {
        if self.optype != Type::ADD {
            os.write_enum(1, self.optype.value())?;
        };
        if self.path != ::std::string::String::new() {
            os.write_string(2, &self.path)?;
        };
        if let Some(v) = self.object.as_ref() {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
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

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Op {
    fn new() -> Op {
        Op::new()
    }

    fn descriptor_static(_: ::std::option::Option<Op>)
                         -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> =
            ::protobuf::lazy::Lazy {
                lock: ::protobuf::lazy::ONCE_INIT,
                ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
            };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeEnum<Type>>(
                    "optype",
                    Op::get_optype_for_reflect,
                    Op::mut_optype_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "path",
                    Op::get_path_for_reflect,
                    Op::mut_path_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::object::Object>>(
                    "object",
                    Op::get_object_for_reflect,
                    Op::mut_object_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Op>(
                    "Op",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Op {
    fn clear(&mut self) {
        self.clear_optype();
        self.clear_path();
        self.clear_object();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Op {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Op {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum Type {
    ADD = 0,
}

impl ::protobuf::ProtobufEnum for Type {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<Type> {
        match value {
            0 => ::std::option::Option::Some(Type::ADD),
            _ => ::std::option::Option::None,
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [Type] = &[Type::ADD];
        values
    }

    fn enum_descriptor_static(_: Option<Type>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> =
            ::protobuf::lazy::Lazy {
                lock: ::protobuf::lazy::ONCE_INIT,
                ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
            };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("Type", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for Type {}

impl ::std::default::Default for Type {
    fn default() -> Self {
        Type::ADD
    }
}

impl ::protobuf::reflect::ProtobufValue for Type {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

static file_descriptor_proto_data: &'static [u8] =
    &[0x0a, 0x08, 0x6f, 0x70, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x05, 0x69, 0x74, 0x68,
      0x6f, 0x73, 0x1a, 0x0c, 0x6f, 0x62, 0x6a, 0x65, 0x63, 0x74, 0x2e, 0x70, 0x72, 0x6f, 0x74,
      0x6f, 0x22, 0x64, 0x0a, 0x02, 0x4f, 0x70, 0x12, 0x23, 0x0a, 0x06, 0x6f, 0x70, 0x74, 0x79,
      0x70, 0x65, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0e, 0x32, 0x0b, 0x2e, 0x69, 0x74, 0x68, 0x6f,
      0x73, 0x2e, 0x54, 0x79, 0x70, 0x65, 0x52, 0x06, 0x6f, 0x70, 0x74, 0x79, 0x70, 0x65, 0x12,
      0x12, 0x0a, 0x04, 0x70, 0x61, 0x74, 0x68, 0x18, 0x02, 0x20, 0x01, 0x28, 0x09, 0x52, 0x04,
      0x70, 0x61, 0x74, 0x68, 0x12, 0x25, 0x0a, 0x06, 0x6f, 0x62, 0x6a, 0x65, 0x63, 0x74, 0x18,
      0x03, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x0d, 0x2e, 0x69, 0x74, 0x68, 0x6f, 0x73, 0x2e, 0x4f,
      0x62, 0x6a, 0x65, 0x63, 0x74, 0x52, 0x06, 0x6f, 0x62, 0x6a, 0x65, 0x63, 0x74, 0x2a, 0x0f,
      0x0a, 0x04, 0x54, 0x79, 0x70, 0x65, 0x12, 0x07, 0x0a, 0x03, 0x41, 0x44, 0x44, 0x10, 0x00,
      0x4a, 0x89, 0x03, 0x0a, 0x06, 0x12, 0x04, 0x00, 0x00, 0x0f, 0x01, 0x0a, 0x08, 0x0a, 0x01,
      0x0c, 0x12, 0x03, 0x00, 0x00, 0x12, 0x0a, 0x08, 0x0a, 0x01, 0x02, 0x12, 0x03, 0x02, 0x08,
      0x0d, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x00, 0x12, 0x03, 0x04, 0x07, 0x15, 0x0a, 0x0a, 0x0a,
      0x02, 0x05, 0x00, 0x12, 0x04, 0x06, 0x00, 0x08, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x05, 0x00,
      0x01, 0x12, 0x03, 0x06, 0x05, 0x09, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x00, 0x12,
      0x03, 0x07, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03,
      0x07, 0x02, 0x05, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x00, 0x02, 0x12, 0x03, 0x07,
      0x08, 0x09, 0x0a, 0x41, 0x0a, 0x02, 0x04, 0x00, 0x12, 0x04, 0x0b, 0x00, 0x0f, 0x01, 0x1a,
      0x35, 0x20, 0x4f, 0x70, 0x73, 0x20, 0x6d, 0x61, 0x6b, 0x65, 0x20, 0x6d, 0x6f, 0x64, 0x69,
      0x66, 0x69, 0x63, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x20, 0x74, 0x6f, 0x20, 0x74, 0x68,
      0x65, 0x20, 0x73, 0x74, 0x61, 0x74, 0x65, 0x20, 0x6f, 0x66, 0x20, 0x74, 0x68, 0x65, 0x20,
      0x64, 0x61, 0x74, 0x61, 0x62, 0x61, 0x73, 0x65, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00,
      0x01, 0x12, 0x03, 0x0b, 0x08, 0x0a, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x00, 0x12,
      0x03, 0x0c, 0x02, 0x14, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x04, 0x12, 0x04,
      0x0c, 0x02, 0x0b, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x06, 0x12, 0x03,
      0x0c, 0x02, 0x06, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x0c,
      0x07, 0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x0c, 0x12,
      0x13, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x01, 0x12, 0x03, 0x0d, 0x02, 0x14, 0x0a,
      0x0d, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x04, 0x12, 0x04, 0x0d, 0x02, 0x0c, 0x14, 0x0a,
      0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x05, 0x12, 0x03, 0x0d, 0x02, 0x08, 0x0a, 0x0c,
      0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x0d, 0x09, 0x0d, 0x0a, 0x0c, 0x0a,
      0x05, 0x04, 0x00, 0x02, 0x01, 0x03, 0x12, 0x03, 0x0d, 0x12, 0x13, 0x0a, 0x0b, 0x0a, 0x04,
      0x04, 0x00, 0x02, 0x02, 0x12, 0x03, 0x0e, 0x02, 0x14, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x00,
      0x02, 0x02, 0x04, 0x12, 0x04, 0x0e, 0x02, 0x0d, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
      0x02, 0x02, 0x06, 0x12, 0x03, 0x0e, 0x02, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
      0x02, 0x01, 0x12, 0x03, 0x0e, 0x09, 0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02,
      0x03, 0x12, 0x03, 0x0e, 0x12, 0x13, 0x62, 0x06, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x33];

static mut file_descriptor_proto_lazy: ::protobuf::lazy::Lazy<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::lazy::Lazy {
    lock: ::protobuf::lazy::ONCE_INIT,
    ptr: 0 as *const ::protobuf::descriptor::FileDescriptorProto,
};

fn parse_descriptor_proto() -> ::protobuf::descriptor::FileDescriptorProto {
    ::protobuf::parse_from_bytes(file_descriptor_proto_data).unwrap()
}

pub fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    unsafe { file_descriptor_proto_lazy.get(|| parse_descriptor_proto()) }
}

// TODO: Hand edited! Figure out a better solution for objecthash support
impl ObjectHash for Op {
    #[inline]
    fn objecthash<H: ObjectHasher>(&self, hasher: &mut H) {
        objecthash_struct!(
            hasher,
            "optype" => &(self.optype as u32),
            "path" => &self.path,
            "object" => self.get_object()
        )
    }
}
