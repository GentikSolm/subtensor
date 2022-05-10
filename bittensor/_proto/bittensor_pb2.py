# -*- coding: utf-8 -*-
# Generated by the protocol buffer compiler.  DO NOT EDIT!
# source: bittensor/_proto/bittensor.proto
"""Generated protocol buffer code."""
from google.protobuf.internal import enum_type_wrapper
from google.protobuf import descriptor as _descriptor
from google.protobuf import message as _message
from google.protobuf import reflection as _reflection
from google.protobuf import symbol_database as _symbol_database
# @@protoc_insertion_point(imports)

_sym_db = _symbol_database.Default()




DESCRIPTOR = _descriptor.FileDescriptor(
  name='bittensor/_proto/bittensor.proto',
  package='',
  syntax='proto3',
  serialized_options=None,
  create_key=_descriptor._internal_create_key,
  serialized_pb=b'\n bittensor/_proto/bittensor.proto\"\x8f\x01\n\x06Neuron\x12\x0f\n\x07version\x18\x01 \x01(\x05\x12\x0b\n\x03uid\x18\x02 \x01(\x03\x12\x0e\n\x06hotkey\x18\x03 \x01(\t\x12\x0f\n\x07\x63oldkey\x18\x04 \x01(\t\x12\n\n\x02ip\x18\x05 \x01(\t\x12\x0c\n\x04port\x18\x06 \x01(\x05\x12\x0f\n\x07ip_type\x18\x07 \x01(\x05\x12\x1b\n\x08modality\x18\x08 \x01(\x0e\x32\t.Modality\"\x94\x01\n\rTensorMessage\x12\x0f\n\x07version\x18\x01 \x01(\x05\x12\x0e\n\x06hotkey\x18\x02 \x01(\t\x12\x18\n\x07tensors\x18\x05 \x03(\x0b\x32\x07.Tensor\x12 \n\x0breturn_code\x18\x06 \x01(\x0e\x32\x0b.ReturnCode\x12\x0f\n\x07message\x18\x07 \x01(\t\x12\x15\n\rrequires_grad\x18\x08 \x01(\x08\"\xc9\x01\n\x06Tensor\x12\x0f\n\x07version\x18\x01 \x01(\x05\x12\x0e\n\x06\x62uffer\x18\x02 \x01(\x0c\x12\r\n\x05shape\x18\x03 \x03(\x03\x12\x1f\n\nserializer\x18\x04 \x01(\x0e\x32\x0b.Serializer\x12 \n\x0btensor_type\x18\x05 \x01(\x0e\x32\x0b.TensorType\x12\x18\n\x05\x64type\x18\x06 \x01(\x0e\x32\t.DataType\x12\x1b\n\x08modality\x18\x07 \x01(\x0e\x32\t.Modality\x12\x15\n\rrequires_grad\x18\x08 \x01(\x08*\xb8\x04\n\nReturnCode\x12\x0c\n\x08NoReturn\x10\x00\x12\x0b\n\x07Success\x10\x01\x12\x0b\n\x07Timeout\x10\x02\x12\x0b\n\x07\x42\x61\x63koff\x10\x03\x12\x0f\n\x0bUnavailable\x10\x04\x12\x12\n\x0eNotImplemented\x10\x05\x12\x10\n\x0c\x45mptyRequest\x10\x06\x12\x11\n\rEmptyResponse\x10\x07\x12\x13\n\x0fInvalidResponse\x10\x08\x12\x12\n\x0eInvalidRequest\x10\t\x12\x19\n\x15RequestShapeException\x10\n\x12\x1a\n\x16ResponseShapeException\x10\x0b\x12!\n\x1dRequestSerializationException\x10\x0c\x12\"\n\x1eResponseSerializationException\x10\r\x12#\n\x1fRequestDeserializationException\x10\x0e\x12$\n ResponseDeserializationException\x10\x0f\x12\x15\n\x11NotServingNucleus\x10\x10\x12\x12\n\x0eNucleusTimeout\x10\x11\x12\x0f\n\x0bNucleusFull\x10\x12\x12\x1e\n\x1aRequestIncompatibleVersion\x10\x13\x12\x1f\n\x1bResponseIncompatibleVersion\x10\x14\x12\x11\n\rSenderUnknown\x10\x15\x12\x14\n\x10UnknownException\x10\x16\x12\x13\n\x0fUnauthenticated\x10\x17*&\n\nSerializer\x12\x0b\n\x07MSGPACK\x10\x00\x12\x0b\n\x07\x43MPPACK\x10\x01*2\n\nTensorType\x12\t\n\x05TORCH\x10\x00\x12\x0e\n\nTENSORFLOW\x10\x01\x12\t\n\x05NUMPY\x10\x02*^\n\x08\x44\x61taType\x12\x0b\n\x07UNKNOWN\x10\x00\x12\x0b\n\x07\x46LOAT32\x10\x01\x12\x0b\n\x07\x46LOAT64\x10\x02\x12\t\n\x05INT32\x10\x03\x12\t\n\x05INT64\x10\x04\x12\x08\n\x04UTF8\x10\x05\x12\x0b\n\x07\x46LOAT16\x10\x06*+\n\x08Modality\x12\x08\n\x04TEXT\x10\x00\x12\t\n\x05IMAGE\x10\x01\x12\n\n\x06TENSOR\x10\x02*8\n\x0bRequestType\x12\x0e\n\nNOTDEFINED\x10\x00\x12\x0b\n\x07\x46ORWARD\x10\x01\x12\x0c\n\x08\x42\x41\x43KWARD\x10\x02\x32\x66\n\tBittensor\x12+\n\x07\x46orward\x12\x0e.TensorMessage\x1a\x0e.TensorMessage\"\x00\x12,\n\x08\x42\x61\x63kward\x12\x0e.TensorMessage\x1a\x0e.TensorMessage\"\x00\x62\x06proto3'
)

_RETURNCODE = _descriptor.EnumDescriptor(
  name='ReturnCode',
  full_name='ReturnCode',
  filename=None,
  file=DESCRIPTOR,
  create_key=_descriptor._internal_create_key,
  values=[
    _descriptor.EnumValueDescriptor(
      name='NoReturn', index=0, number=0,
      serialized_options=None,
      type=None,
      create_key=_descriptor._internal_create_key),
    _descriptor.EnumValueDescriptor(
      name='Success', index=1, number=1,
      serialized_options=None,
      type=None,
      create_key=_descriptor._internal_create_key),
    _descriptor.EnumValueDescriptor(
      name='Timeout', index=2, number=2,
      serialized_options=None,
      type=None,
      create_key=_descriptor._internal_create_key),
    _descriptor.EnumValueDescriptor(
      name='Backoff', index=3, number=3,
      serialized_options=None,
      type=None,
      create_key=_descriptor._internal_create_key),
    _descriptor.EnumValueDescriptor(
      name='Unavailable', index=4, number=4,
      serialized_options=None,
      type=None,
      create_key=_descriptor._internal_create_key),
    _descriptor.EnumValueDescriptor(
      name='NotImplemented', index=5, number=5,
      serialized_options=None,
      type=None,
      create_key=_descriptor._internal_create_key),
    _descriptor.EnumValueDescriptor(
      name='EmptyRequest', index=6, number=6,
      serialized_options=None,
      type=None,
      create_key=_descriptor._internal_create_key),
    _descriptor.EnumValueDescriptor(
      name='EmptyResponse', index=7, number=7,
      serialized_options=None,
      type=None,
      create_key=_descriptor._internal_create_key),
    _descriptor.EnumValueDescriptor(
      name='InvalidResponse', index=8, number=8,
      serialized_options=None,
      type=None,
      create_key=_descriptor._internal_create_key),
    _descriptor.EnumValueDescriptor(
      name='InvalidRequest', index=9, number=9,
      serialized_options=None,
      type=None,
      create_key=_descriptor._internal_create_key),
    _descriptor.EnumValueDescriptor(
      name='RequestShapeException', index=10, number=10,
      serialized_options=None,
      type=None,
      create_key=_descriptor._internal_create_key),
    _descriptor.EnumValueDescriptor(
      name='ResponseShapeException', index=11, number=11,
      serialized_options=None,
      type=None,
      create_key=_descriptor._internal_create_key),
    _descriptor.EnumValueDescriptor(
      name='RequestSerializationException', index=12, number=12,
      serialized_options=None,
      type=None,
      create_key=_descriptor._internal_create_key),
    _descriptor.EnumValueDescriptor(
      name='ResponseSerializationException', index=13, number=13,
      serialized_options=None,
      type=None,
      create_key=_descriptor._internal_create_key),
    _descriptor.EnumValueDescriptor(
      name='RequestDeserializationException', index=14, number=14,
      serialized_options=None,
      type=None,
      create_key=_descriptor._internal_create_key),
    _descriptor.EnumValueDescriptor(
      name='ResponseDeserializationException', index=15, number=15,
      serialized_options=None,
      type=None,
      create_key=_descriptor._internal_create_key),
    _descriptor.EnumValueDescriptor(
      name='NotServingNucleus', index=16, number=16,
      serialized_options=None,
      type=None,
      create_key=_descriptor._internal_create_key),
    _descriptor.EnumValueDescriptor(
      name='NucleusTimeout', index=17, number=17,
      serialized_options=None,
      type=None,
      create_key=_descriptor._internal_create_key),
    _descriptor.EnumValueDescriptor(
      name='NucleusFull', index=18, number=18,
      serialized_options=None,
      type=None,
      create_key=_descriptor._internal_create_key),
    _descriptor.EnumValueDescriptor(
      name='RequestIncompatibleVersion', index=19, number=19,
      serialized_options=None,
      type=None,
      create_key=_descriptor._internal_create_key),
    _descriptor.EnumValueDescriptor(
      name='ResponseIncompatibleVersion', index=20, number=20,
      serialized_options=None,
      type=None,
      create_key=_descriptor._internal_create_key),
    _descriptor.EnumValueDescriptor(
      name='SenderUnknown', index=21, number=21,
      serialized_options=None,
      type=None,
      create_key=_descriptor._internal_create_key),
    _descriptor.EnumValueDescriptor(
      name='UnknownException', index=22, number=22,
      serialized_options=None,
      type=None,
      create_key=_descriptor._internal_create_key),
    _descriptor.EnumValueDescriptor(
      name='Unauthenticated', index=23, number=23,
      serialized_options=None,
      type=None,
      create_key=_descriptor._internal_create_key),
  ],
  containing_type=None,
  serialized_options=None,
  serialized_start=538,
  serialized_end=1106,
)
_sym_db.RegisterEnumDescriptor(_RETURNCODE)

ReturnCode = enum_type_wrapper.EnumTypeWrapper(_RETURNCODE)
_SERIALIZER = _descriptor.EnumDescriptor(
  name='Serializer',
  full_name='Serializer',
  filename=None,
  file=DESCRIPTOR,
  create_key=_descriptor._internal_create_key,
  values=[
    _descriptor.EnumValueDescriptor(
      name='MSGPACK', index=0, number=0,
      serialized_options=None,
      type=None,
      create_key=_descriptor._internal_create_key),
    _descriptor.EnumValueDescriptor(
      name='CMPPACK', index=1, number=1,
      serialized_options=None,
      type=None,
      create_key=_descriptor._internal_create_key),
  ],
  containing_type=None,
  serialized_options=None,
  serialized_start=1108,
  serialized_end=1146,
)
_sym_db.RegisterEnumDescriptor(_SERIALIZER)

Serializer = enum_type_wrapper.EnumTypeWrapper(_SERIALIZER)
_TENSORTYPE = _descriptor.EnumDescriptor(
  name='TensorType',
  full_name='TensorType',
  filename=None,
  file=DESCRIPTOR,
  create_key=_descriptor._internal_create_key,
  values=[
    _descriptor.EnumValueDescriptor(
      name='TORCH', index=0, number=0,
      serialized_options=None,
      type=None,
      create_key=_descriptor._internal_create_key),
    _descriptor.EnumValueDescriptor(
      name='TENSORFLOW', index=1, number=1,
      serialized_options=None,
      type=None,
      create_key=_descriptor._internal_create_key),
    _descriptor.EnumValueDescriptor(
      name='NUMPY', index=2, number=2,
      serialized_options=None,
      type=None,
      create_key=_descriptor._internal_create_key),
  ],
  containing_type=None,
  serialized_options=None,
  serialized_start=1148,
  serialized_end=1198,
)
_sym_db.RegisterEnumDescriptor(_TENSORTYPE)

TensorType = enum_type_wrapper.EnumTypeWrapper(_TENSORTYPE)
_DATATYPE = _descriptor.EnumDescriptor(
  name='DataType',
  full_name='DataType',
  filename=None,
  file=DESCRIPTOR,
  create_key=_descriptor._internal_create_key,
  values=[
    _descriptor.EnumValueDescriptor(
      name='UNKNOWN', index=0, number=0,
      serialized_options=None,
      type=None,
      create_key=_descriptor._internal_create_key),
    _descriptor.EnumValueDescriptor(
      name='FLOAT32', index=1, number=1,
      serialized_options=None,
      type=None,
      create_key=_descriptor._internal_create_key),
    _descriptor.EnumValueDescriptor(
      name='FLOAT64', index=2, number=2,
      serialized_options=None,
      type=None,
      create_key=_descriptor._internal_create_key),
    _descriptor.EnumValueDescriptor(
      name='INT32', index=3, number=3,
      serialized_options=None,
      type=None,
      create_key=_descriptor._internal_create_key),
    _descriptor.EnumValueDescriptor(
      name='INT64', index=4, number=4,
      serialized_options=None,
      type=None,
      create_key=_descriptor._internal_create_key),
    _descriptor.EnumValueDescriptor(
      name='UTF8', index=5, number=5,
      serialized_options=None,
      type=None,
      create_key=_descriptor._internal_create_key),
    _descriptor.EnumValueDescriptor(
      name='FLOAT16', index=6, number=6,
      serialized_options=None,
      type=None,
      create_key=_descriptor._internal_create_key),
  ],
  containing_type=None,
  serialized_options=None,
  serialized_start=1200,
  serialized_end=1294,
)
_sym_db.RegisterEnumDescriptor(_DATATYPE)

DataType = enum_type_wrapper.EnumTypeWrapper(_DATATYPE)
_MODALITY = _descriptor.EnumDescriptor(
  name='Modality',
  full_name='Modality',
  filename=None,
  file=DESCRIPTOR,
  create_key=_descriptor._internal_create_key,
  values=[
    _descriptor.EnumValueDescriptor(
      name='TEXT', index=0, number=0,
      serialized_options=None,
      type=None,
      create_key=_descriptor._internal_create_key),
    _descriptor.EnumValueDescriptor(
      name='IMAGE', index=1, number=1,
      serialized_options=None,
      type=None,
      create_key=_descriptor._internal_create_key),
    _descriptor.EnumValueDescriptor(
      name='TENSOR', index=2, number=2,
      serialized_options=None,
      type=None,
      create_key=_descriptor._internal_create_key),
  ],
  containing_type=None,
  serialized_options=None,
  serialized_start=1296,
  serialized_end=1339,
)
_sym_db.RegisterEnumDescriptor(_MODALITY)

Modality = enum_type_wrapper.EnumTypeWrapper(_MODALITY)
_REQUESTTYPE = _descriptor.EnumDescriptor(
  name='RequestType',
  full_name='RequestType',
  filename=None,
  file=DESCRIPTOR,
  create_key=_descriptor._internal_create_key,
  values=[
    _descriptor.EnumValueDescriptor(
      name='NOTDEFINED', index=0, number=0,
      serialized_options=None,
      type=None,
      create_key=_descriptor._internal_create_key),
    _descriptor.EnumValueDescriptor(
      name='FORWARD', index=1, number=1,
      serialized_options=None,
      type=None,
      create_key=_descriptor._internal_create_key),
    _descriptor.EnumValueDescriptor(
      name='BACKWARD', index=2, number=2,
      serialized_options=None,
      type=None,
      create_key=_descriptor._internal_create_key),
  ],
  containing_type=None,
  serialized_options=None,
  serialized_start=1341,
  serialized_end=1397,
)
_sym_db.RegisterEnumDescriptor(_REQUESTTYPE)

RequestType = enum_type_wrapper.EnumTypeWrapper(_REQUESTTYPE)
NoReturn = 0
Success = 1
Timeout = 2
Backoff = 3
Unavailable = 4
NotImplemented = 5
EmptyRequest = 6
EmptyResponse = 7
InvalidResponse = 8
InvalidRequest = 9
RequestShapeException = 10
ResponseShapeException = 11
RequestSerializationException = 12
ResponseSerializationException = 13
RequestDeserializationException = 14
ResponseDeserializationException = 15
NotServingNucleus = 16
NucleusTimeout = 17
NucleusFull = 18
RequestIncompatibleVersion = 19
ResponseIncompatibleVersion = 20
SenderUnknown = 21
UnknownException = 22
Unauthenticated = 23
MSGPACK = 0
CMPPACK = 1
TORCH = 0
TENSORFLOW = 1
NUMPY = 2
UNKNOWN = 0
FLOAT32 = 1
FLOAT64 = 2
INT32 = 3
INT64 = 4
UTF8 = 5
FLOAT16 = 6
TEXT = 0
IMAGE = 1
TENSOR = 2
NOTDEFINED = 0
FORWARD = 1
BACKWARD = 2



_NEURON = _descriptor.Descriptor(
  name='Neuron',
  full_name='Neuron',
  filename=None,
  file=DESCRIPTOR,
  containing_type=None,
  create_key=_descriptor._internal_create_key,
  fields=[
    _descriptor.FieldDescriptor(
      name='version', full_name='Neuron.version', index=0,
      number=1, type=5, cpp_type=1, label=1,
      has_default_value=False, default_value=0,
      message_type=None, enum_type=None, containing_type=None,
      is_extension=False, extension_scope=None,
      serialized_options=None, file=DESCRIPTOR,  create_key=_descriptor._internal_create_key),
    _descriptor.FieldDescriptor(
      name='uid', full_name='Neuron.uid', index=1,
      number=2, type=3, cpp_type=2, label=1,
      has_default_value=False, default_value=0,
      message_type=None, enum_type=None, containing_type=None,
      is_extension=False, extension_scope=None,
      serialized_options=None, file=DESCRIPTOR,  create_key=_descriptor._internal_create_key),
    _descriptor.FieldDescriptor(
      name='hotkey', full_name='Neuron.hotkey', index=2,
      number=3, type=9, cpp_type=9, label=1,
      has_default_value=False, default_value=b"".decode('utf-8'),
      message_type=None, enum_type=None, containing_type=None,
      is_extension=False, extension_scope=None,
      serialized_options=None, file=DESCRIPTOR,  create_key=_descriptor._internal_create_key),
    _descriptor.FieldDescriptor(
      name='coldkey', full_name='Neuron.coldkey', index=3,
      number=4, type=9, cpp_type=9, label=1,
      has_default_value=False, default_value=b"".decode('utf-8'),
      message_type=None, enum_type=None, containing_type=None,
      is_extension=False, extension_scope=None,
      serialized_options=None, file=DESCRIPTOR,  create_key=_descriptor._internal_create_key),
    _descriptor.FieldDescriptor(
      name='ip', full_name='Neuron.ip', index=4,
      number=5, type=9, cpp_type=9, label=1,
      has_default_value=False, default_value=b"".decode('utf-8'),
      message_type=None, enum_type=None, containing_type=None,
      is_extension=False, extension_scope=None,
      serialized_options=None, file=DESCRIPTOR,  create_key=_descriptor._internal_create_key),
    _descriptor.FieldDescriptor(
      name='port', full_name='Neuron.port', index=5,
      number=6, type=5, cpp_type=1, label=1,
      has_default_value=False, default_value=0,
      message_type=None, enum_type=None, containing_type=None,
      is_extension=False, extension_scope=None,
      serialized_options=None, file=DESCRIPTOR,  create_key=_descriptor._internal_create_key),
    _descriptor.FieldDescriptor(
      name='ip_type', full_name='Neuron.ip_type', index=6,
      number=7, type=5, cpp_type=1, label=1,
      has_default_value=False, default_value=0,
      message_type=None, enum_type=None, containing_type=None,
      is_extension=False, extension_scope=None,
      serialized_options=None, file=DESCRIPTOR,  create_key=_descriptor._internal_create_key),
    _descriptor.FieldDescriptor(
      name='modality', full_name='Neuron.modality', index=7,
      number=8, type=14, cpp_type=8, label=1,
      has_default_value=False, default_value=0,
      message_type=None, enum_type=None, containing_type=None,
      is_extension=False, extension_scope=None,
      serialized_options=None, file=DESCRIPTOR,  create_key=_descriptor._internal_create_key),
  ],
  extensions=[
  ],
  nested_types=[],
  enum_types=[
  ],
  serialized_options=None,
  is_extendable=False,
  syntax='proto3',
  extension_ranges=[],
  oneofs=[
  ],
  serialized_start=37,
  serialized_end=180,
)


_TENSORMESSAGE = _descriptor.Descriptor(
  name='TensorMessage',
  full_name='TensorMessage',
  filename=None,
  file=DESCRIPTOR,
  containing_type=None,
  create_key=_descriptor._internal_create_key,
  fields=[
    _descriptor.FieldDescriptor(
      name='version', full_name='TensorMessage.version', index=0,
      number=1, type=5, cpp_type=1, label=1,
      has_default_value=False, default_value=0,
      message_type=None, enum_type=None, containing_type=None,
      is_extension=False, extension_scope=None,
      serialized_options=None, file=DESCRIPTOR,  create_key=_descriptor._internal_create_key),
    _descriptor.FieldDescriptor(
      name='hotkey', full_name='TensorMessage.hotkey', index=1,
      number=2, type=9, cpp_type=9, label=1,
      has_default_value=False, default_value=b"".decode('utf-8'),
      message_type=None, enum_type=None, containing_type=None,
      is_extension=False, extension_scope=None,
      serialized_options=None, file=DESCRIPTOR,  create_key=_descriptor._internal_create_key),
    _descriptor.FieldDescriptor(
      name='tensors', full_name='TensorMessage.tensors', index=2,
      number=5, type=11, cpp_type=10, label=3,
      has_default_value=False, default_value=[],
      message_type=None, enum_type=None, containing_type=None,
      is_extension=False, extension_scope=None,
      serialized_options=None, file=DESCRIPTOR,  create_key=_descriptor._internal_create_key),
    _descriptor.FieldDescriptor(
      name='return_code', full_name='TensorMessage.return_code', index=3,
      number=6, type=14, cpp_type=8, label=1,
      has_default_value=False, default_value=0,
      message_type=None, enum_type=None, containing_type=None,
      is_extension=False, extension_scope=None,
      serialized_options=None, file=DESCRIPTOR,  create_key=_descriptor._internal_create_key),
    _descriptor.FieldDescriptor(
      name='message', full_name='TensorMessage.message', index=4,
      number=7, type=9, cpp_type=9, label=1,
      has_default_value=False, default_value=b"".decode('utf-8'),
      message_type=None, enum_type=None, containing_type=None,
      is_extension=False, extension_scope=None,
      serialized_options=None, file=DESCRIPTOR,  create_key=_descriptor._internal_create_key),
    _descriptor.FieldDescriptor(
      name='requires_grad', full_name='TensorMessage.requires_grad', index=5,
      number=8, type=8, cpp_type=7, label=1,
      has_default_value=False, default_value=False,
      message_type=None, enum_type=None, containing_type=None,
      is_extension=False, extension_scope=None,
      serialized_options=None, file=DESCRIPTOR,  create_key=_descriptor._internal_create_key),
  ],
  extensions=[
  ],
  nested_types=[],
  enum_types=[
  ],
  serialized_options=None,
  is_extendable=False,
  syntax='proto3',
  extension_ranges=[],
  oneofs=[
  ],
  serialized_start=183,
  serialized_end=331,
)


_TENSOR = _descriptor.Descriptor(
  name='Tensor',
  full_name='Tensor',
  filename=None,
  file=DESCRIPTOR,
  containing_type=None,
  create_key=_descriptor._internal_create_key,
  fields=[
    _descriptor.FieldDescriptor(
      name='version', full_name='Tensor.version', index=0,
      number=1, type=5, cpp_type=1, label=1,
      has_default_value=False, default_value=0,
      message_type=None, enum_type=None, containing_type=None,
      is_extension=False, extension_scope=None,
      serialized_options=None, file=DESCRIPTOR,  create_key=_descriptor._internal_create_key),
    _descriptor.FieldDescriptor(
      name='buffer', full_name='Tensor.buffer', index=1,
      number=2, type=12, cpp_type=9, label=1,
      has_default_value=False, default_value=b"",
      message_type=None, enum_type=None, containing_type=None,
      is_extension=False, extension_scope=None,
      serialized_options=None, file=DESCRIPTOR,  create_key=_descriptor._internal_create_key),
    _descriptor.FieldDescriptor(
      name='shape', full_name='Tensor.shape', index=2,
      number=3, type=3, cpp_type=2, label=3,
      has_default_value=False, default_value=[],
      message_type=None, enum_type=None, containing_type=None,
      is_extension=False, extension_scope=None,
      serialized_options=None, file=DESCRIPTOR,  create_key=_descriptor._internal_create_key),
    _descriptor.FieldDescriptor(
      name='serializer', full_name='Tensor.serializer', index=3,
      number=4, type=14, cpp_type=8, label=1,
      has_default_value=False, default_value=0,
      message_type=None, enum_type=None, containing_type=None,
      is_extension=False, extension_scope=None,
      serialized_options=None, file=DESCRIPTOR,  create_key=_descriptor._internal_create_key),
    _descriptor.FieldDescriptor(
      name='tensor_type', full_name='Tensor.tensor_type', index=4,
      number=5, type=14, cpp_type=8, label=1,
      has_default_value=False, default_value=0,
      message_type=None, enum_type=None, containing_type=None,
      is_extension=False, extension_scope=None,
      serialized_options=None, file=DESCRIPTOR,  create_key=_descriptor._internal_create_key),
    _descriptor.FieldDescriptor(
      name='dtype', full_name='Tensor.dtype', index=5,
      number=6, type=14, cpp_type=8, label=1,
      has_default_value=False, default_value=0,
      message_type=None, enum_type=None, containing_type=None,
      is_extension=False, extension_scope=None,
      serialized_options=None, file=DESCRIPTOR,  create_key=_descriptor._internal_create_key),
    _descriptor.FieldDescriptor(
      name='modality', full_name='Tensor.modality', index=6,
      number=7, type=14, cpp_type=8, label=1,
      has_default_value=False, default_value=0,
      message_type=None, enum_type=None, containing_type=None,
      is_extension=False, extension_scope=None,
      serialized_options=None, file=DESCRIPTOR,  create_key=_descriptor._internal_create_key),
    _descriptor.FieldDescriptor(
      name='requires_grad', full_name='Tensor.requires_grad', index=7,
      number=8, type=8, cpp_type=7, label=1,
      has_default_value=False, default_value=False,
      message_type=None, enum_type=None, containing_type=None,
      is_extension=False, extension_scope=None,
      serialized_options=None, file=DESCRIPTOR,  create_key=_descriptor._internal_create_key),
  ],
  extensions=[
  ],
  nested_types=[],
  enum_types=[
  ],
  serialized_options=None,
  is_extendable=False,
  syntax='proto3',
  extension_ranges=[],
  oneofs=[
  ],
  serialized_start=334,
  serialized_end=535,
)

_NEURON.fields_by_name['modality'].enum_type = _MODALITY
_TENSORMESSAGE.fields_by_name['tensors'].message_type = _TENSOR
_TENSORMESSAGE.fields_by_name['return_code'].enum_type = _RETURNCODE
_TENSOR.fields_by_name['serializer'].enum_type = _SERIALIZER
_TENSOR.fields_by_name['tensor_type'].enum_type = _TENSORTYPE
_TENSOR.fields_by_name['dtype'].enum_type = _DATATYPE
_TENSOR.fields_by_name['modality'].enum_type = _MODALITY
DESCRIPTOR.message_types_by_name['Neuron'] = _NEURON
DESCRIPTOR.message_types_by_name['TensorMessage'] = _TENSORMESSAGE
DESCRIPTOR.message_types_by_name['Tensor'] = _TENSOR
DESCRIPTOR.enum_types_by_name['ReturnCode'] = _RETURNCODE
DESCRIPTOR.enum_types_by_name['Serializer'] = _SERIALIZER
DESCRIPTOR.enum_types_by_name['TensorType'] = _TENSORTYPE
DESCRIPTOR.enum_types_by_name['DataType'] = _DATATYPE
DESCRIPTOR.enum_types_by_name['Modality'] = _MODALITY
DESCRIPTOR.enum_types_by_name['RequestType'] = _REQUESTTYPE
_sym_db.RegisterFileDescriptor(DESCRIPTOR)

Neuron = _reflection.GeneratedProtocolMessageType('Neuron', (_message.Message,), {
  'DESCRIPTOR' : _NEURON,
  '__module__' : 'bittensor._proto.bittensor_pb2'
  # @@protoc_insertion_point(class_scope:Neuron)
  })
_sym_db.RegisterMessage(Neuron)

TensorMessage = _reflection.GeneratedProtocolMessageType('TensorMessage', (_message.Message,), {
  'DESCRIPTOR' : _TENSORMESSAGE,
  '__module__' : 'bittensor._proto.bittensor_pb2'
  # @@protoc_insertion_point(class_scope:TensorMessage)
  })
_sym_db.RegisterMessage(TensorMessage)

Tensor = _reflection.GeneratedProtocolMessageType('Tensor', (_message.Message,), {
  'DESCRIPTOR' : _TENSOR,
  '__module__' : 'bittensor._proto.bittensor_pb2'
  # @@protoc_insertion_point(class_scope:Tensor)
  })
_sym_db.RegisterMessage(Tensor)



_BITTENSOR = _descriptor.ServiceDescriptor(
  name='Bittensor',
  full_name='Bittensor',
  file=DESCRIPTOR,
  index=0,
  serialized_options=None,
  create_key=_descriptor._internal_create_key,
  serialized_start=1399,
  serialized_end=1501,
  methods=[
  _descriptor.MethodDescriptor(
    name='Forward',
    full_name='Bittensor.Forward',
    index=0,
    containing_service=None,
    input_type=_TENSORMESSAGE,
    output_type=_TENSORMESSAGE,
    serialized_options=None,
    create_key=_descriptor._internal_create_key,
  ),
  _descriptor.MethodDescriptor(
    name='Backward',
    full_name='Bittensor.Backward',
    index=1,
    containing_service=None,
    input_type=_TENSORMESSAGE,
    output_type=_TENSORMESSAGE,
    serialized_options=None,
    create_key=_descriptor._internal_create_key,
  ),
])
_sym_db.RegisterServiceDescriptor(_BITTENSOR)

DESCRIPTOR.services_by_name['Bittensor'] = _BITTENSOR

# @@protoc_insertion_point(module_scope)
