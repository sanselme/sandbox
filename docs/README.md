# Protocol Documentation
<a name="top"></a>

## Table of Contents

- [v1/cloudevent.proto](#v1_cloudevent-proto)
    - [CloudEvent](#api-v1-CloudEvent)
    - [CloudEvent.AttributesEntry](#api-v1-CloudEvent-AttributesEntry)
    - [CloudEvent.CloudEventAttributeValue](#api-v1-CloudEvent-CloudEventAttributeValue)
    - [CloudEventBatch](#api-v1-CloudEventBatch)
  
- [v1alpha1/helloworld.proto](#v1alpha1_helloworld-proto)
    - [SayHelloRequest](#api-v1alpha1-SayHelloRequest)
    - [SayHelloResponse](#api-v1alpha1-SayHelloResponse)
  
    - [GreeterService](#api-v1alpha1-GreeterService)
  
- [Scalar Value Types](#scalar-value-types)



<a name="v1_cloudevent-proto"></a>
<p align="right"><a href="#top">Top</a></p>

## v1/cloudevent.proto
CloudEvent Protobuf Format

- Required context attributes are explicitly represented.
- Optional and Extension context attributes are carried in a map structure.
- Data may be represented as binary, text, or protobuf messages.


<a name="api-v1-CloudEvent"></a>

### CloudEvent
CloudEvent Protobuf Format

-- CloudEvent Context Attributes


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| id | [string](#string) |  | Required Attributes |
| source | [string](#string) |  | URI-reference |
| spec_version | [string](#string) |  |  |
| type | [string](#string) |  |  |
| attributes | [CloudEvent.AttributesEntry](#api-v1-CloudEvent-AttributesEntry) | repeated | Optional &amp; Extension Attributes |
| binary_data | [bytes](#bytes) |  |  |
| text_data | [string](#string) |  |  |
| proto_data | [google.protobuf.Any](#google-protobuf-Any) |  |  |






<a name="api-v1-CloudEvent-AttributesEntry"></a>

### CloudEvent.AttributesEntry



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| key | [string](#string) |  |  |
| value | [CloudEvent.CloudEventAttributeValue](#api-v1-CloudEvent-CloudEventAttributeValue) |  |  |






<a name="api-v1-CloudEvent-CloudEventAttributeValue"></a>

### CloudEvent.CloudEventAttributeValue
The CloudEvent specification defines
seven attribute value types...


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| ce_boolean | [bool](#bool) |  |  |
| ce_integer | [int32](#int32) |  |  |
| ce_string | [string](#string) |  |  |
| ce_bytes | [bytes](#bytes) |  |  |
| ce_uri | [string](#string) |  |  |
| ce_uri_ref | [string](#string) |  |  |
| ce_timestamp | [google.protobuf.Timestamp](#google-protobuf-Timestamp) |  |  |






<a name="api-v1-CloudEventBatch"></a>

### CloudEventBatch
CloudEvent Protobuf Batch Format


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| events | [CloudEvent](#api-v1-CloudEvent) | repeated |  |





 

 

 

 



<a name="v1alpha1_helloworld-proto"></a>
<p align="right"><a href="#top">Top</a></p>

## v1alpha1/helloworld.proto



<a name="api-v1alpha1-SayHelloRequest"></a>

### SayHelloRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| name | [string](#string) |  |  |






<a name="api-v1alpha1-SayHelloResponse"></a>

### SayHelloResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| message | [string](#string) |  |  |





 

 

 


<a name="api-v1alpha1-GreeterService"></a>

### GreeterService


| Method Name | Request Type | Response Type | Description |
| ----------- | ------------ | ------------- | ------------|
| SayHello | [SayHelloRequest](#api-v1alpha1-SayHelloRequest) | [SayHelloResponse](#api-v1alpha1-SayHelloResponse) |  |

 



## Scalar Value Types

| .proto Type | Notes | C++ | Java | Python | Go | C# | PHP | Ruby |
| ----------- | ----- | --- | ---- | ------ | -- | -- | --- | ---- |
| <a name="double" /> double |  | double | double | float | float64 | double | float | Float |
| <a name="float" /> float |  | float | float | float | float32 | float | float | Float |
| <a name="int32" /> int32 | Uses variable-length encoding. Inefficient for encoding negative numbers – if your field is likely to have negative values, use sint32 instead. | int32 | int | int | int32 | int | integer | Bignum or Fixnum (as required) |
| <a name="int64" /> int64 | Uses variable-length encoding. Inefficient for encoding negative numbers – if your field is likely to have negative values, use sint64 instead. | int64 | long | int/long | int64 | long | integer/string | Bignum |
| <a name="uint32" /> uint32 | Uses variable-length encoding. | uint32 | int | int/long | uint32 | uint | integer | Bignum or Fixnum (as required) |
| <a name="uint64" /> uint64 | Uses variable-length encoding. | uint64 | long | int/long | uint64 | ulong | integer/string | Bignum or Fixnum (as required) |
| <a name="sint32" /> sint32 | Uses variable-length encoding. Signed int value. These more efficiently encode negative numbers than regular int32s. | int32 | int | int | int32 | int | integer | Bignum or Fixnum (as required) |
| <a name="sint64" /> sint64 | Uses variable-length encoding. Signed int value. These more efficiently encode negative numbers than regular int64s. | int64 | long | int/long | int64 | long | integer/string | Bignum |
| <a name="fixed32" /> fixed32 | Always four bytes. More efficient than uint32 if values are often greater than 2^28. | uint32 | int | int | uint32 | uint | integer | Bignum or Fixnum (as required) |
| <a name="fixed64" /> fixed64 | Always eight bytes. More efficient than uint64 if values are often greater than 2^56. | uint64 | long | int/long | uint64 | ulong | integer/string | Bignum |
| <a name="sfixed32" /> sfixed32 | Always four bytes. | int32 | int | int | int32 | int | integer | Bignum or Fixnum (as required) |
| <a name="sfixed64" /> sfixed64 | Always eight bytes. | int64 | long | int/long | int64 | long | integer/string | Bignum |
| <a name="bool" /> bool |  | bool | boolean | boolean | bool | bool | boolean | TrueClass/FalseClass |
| <a name="string" /> string | A string must always contain UTF-8 encoded or 7-bit ASCII text. | string | String | str/unicode | string | string | string | String (UTF-8) |
| <a name="bytes" /> bytes | May contain any arbitrary sequence of bytes. | string | ByteString | str | []byte | ByteString | string | String (ASCII-8BIT) |

