# Protocol Documentation

<a name="top"></a>

## Table of Contents

- [v1/hello.proto](#v1_hello-proto)

  - [HelloReply](#api-v1-HelloReply)
  - [HelloRequest](#api-v1-HelloRequest)

  - [Greeter](#api-v1-Greeter)

- [v1alpha1/cloudevent.proto](#v1alpha1_cloudevent-proto)

  - [CloudEvent](#api-v1alpha1-CloudEvent)
  - [CloudEvent.AttributesEntry](#api-v1alpha1-CloudEvent-AttributesEntry)
  - [CloudEvent.CloudEventAttributeValue](#api-v1alpha1-CloudEvent-CloudEventAttributeValue)
  - [CloudEventBatch](#api-v1alpha1-CloudEventBatch)
  - [PublishRequest](#api-v1alpha1-PublishRequest)
  - [SubscriptionRequest](#api-v1alpha1-SubscriptionRequest)

  - [CloudEventService](#api-v1alpha1-CloudEventService)

- [v1alpha1/health.proto](#v1alpha1_health-proto)

  - [HealthCheckRequest](#api-v1alpha1-HealthCheckRequest)
  - [HealthCheckResponse](#api-v1alpha1-HealthCheckResponse)

  - [HealthCheckResponse.ServingStatus](#api-v1alpha1-HealthCheckResponse-ServingStatus)

  - [Health](#api-v1alpha1-Health)

- [v1alpha1/route_guide.proto](#v1alpha1_route_guide-proto)

  - [Feature](#api-v1alpha1-Feature)
  - [Point](#api-v1alpha1-Point)
  - [Rectangle](#api-v1alpha1-Rectangle)
  - [RouteNote](#api-v1alpha1-RouteNote)
  - [RouteSummary](#api-v1alpha1-RouteSummary)

  - [RouteGuide](#api-v1alpha1-RouteGuide)

- [Scalar Value Types](#scalar-value-types)

<a name="v1_hello-proto"></a>

<p align="right"><a href="#top">Top</a></p>

## v1/hello.proto

<a name="api-v1-HelloReply"></a>

### HelloReply

The response message containing the greetings

| Field   | Type              | Label | Description |
| ------- | ----------------- | ----- | ----------- |
| message | [string](#string) |       |             |

<a name="api-v1-HelloRequest"></a>

### HelloRequest

The request message containing the user&#39;s name

| Field | Type              | Label | Description |
| ----- | ----------------- | ----- | ----------- |
| name  | [string](#string) |       |             |

<a name="api-v1-Greeter"></a>

### Greeter

Here is the overall greeting service definition where we define all our endpoints

| Method Name | Request Type                         | Response Type                    | Description      |
| ----------- | ------------------------------------ | -------------------------------- | ---------------- |
| SayHello    | [HelloRequest](#api-v1-HelloRequest) | [HelloReply](#api-v1-HelloReply) | Sends a greeting |

<a name="v1alpha1_cloudevent-proto"></a>

<p align="right"><a href="#top">Top</a></p>

## v1alpha1/cloudevent.proto

CloudEvent Protobuf Format

- Required context attributes are explicitly represented.
- Optional and Extension context attributes are carried in a map structure.
- Data may be represented as binary, text, or protobuf messages.

<a name="api-v1alpha1-CloudEvent"></a>

### CloudEvent

CloudEvent Protobuf Format

-- CloudEvent Context Attributes

| Field        | Type                                                                   | Label    | Description                         |
| ------------ | ---------------------------------------------------------------------- | -------- | ----------------------------------- |
| id           | [string](#string)                                                      |          | Required Attributes                 |
| source       | [string](#string)                                                      |          | URI-reference                       |
| spec_version | [string](#string)                                                      |          |                                     |
| type         | [string](#string)                                                      |          |                                     |
| attributes   | [CloudEvent.AttributesEntry](#api-v1alpha1-CloudEvent-AttributesEntry) | repeated | Optional &amp; Extension Attributes |
| binary_data  | [bytes](#bytes)                                                        |          |                                     |
| text_data    | [string](#string)                                                      |          |                                     |
| proto_data   | [google.protobuf.Any](#google-protobuf-Any)                            |          |                                     |

<a name="api-v1alpha1-CloudEvent-AttributesEntry"></a>

### CloudEvent.AttributesEntry

| Field | Type                                                                                     | Label | Description |
| ----- | ---------------------------------------------------------------------------------------- | ----- | ----------- |
| key   | [string](#string)                                                                        |       |             |
| value | [CloudEvent.CloudEventAttributeValue](#api-v1alpha1-CloudEvent-CloudEventAttributeValue) |       |             |

<a name="api-v1alpha1-CloudEvent-CloudEventAttributeValue"></a>

### CloudEvent.CloudEventAttributeValue

The CloudEvent specification defines
seven attribute value types...

| Field        | Type                                                    | Label | Description |
| ------------ | ------------------------------------------------------- | ----- | ----------- |
| ce_boolean   | [bool](#bool)                                           |       |             |
| ce_integer   | [int32](#int32)                                         |       |             |
| ce_string    | [string](#string)                                       |       |             |
| ce_bytes     | [bytes](#bytes)                                         |       |             |
| ce_uri       | [string](#string)                                       |       |             |
| ce_uri_ref   | [string](#string)                                       |       |             |
| ce_timestamp | [google.protobuf.Timestamp](#google-protobuf-Timestamp) |       |             |

<a name="api-v1alpha1-CloudEventBatch"></a>

### CloudEventBatch

CloudEvent Protobuf Batch Format

| Field  | Type                                   | Label    | Description |
| ------ | -------------------------------------- | -------- | ----------- |
| events | [CloudEvent](#api-v1alpha1-CloudEvent) | repeated |             |

<a name="api-v1alpha1-PublishRequest"></a>

### PublishRequest

| Field | Type                                   | Label | Description |
| ----- | -------------------------------------- | ----- | ----------- |
| topic | [string](#string)                      |       |             |
| event | [CloudEvent](#api-v1alpha1-CloudEvent) |       |             |

<a name="api-v1alpha1-SubscriptionRequest"></a>

### SubscriptionRequest

| Field | Type              | Label | Description |
| ----- | ----------------- | ----- | ----------- |
| topic | [string](#string) |       |             |

<a name="api-v1alpha1-CloudEventService"></a>

### CloudEventService

| Method Name | Request Type                                             | Response Type                                    | Description |
| ----------- | -------------------------------------------------------- | ------------------------------------------------ | ----------- |
| Publish     | [PublishRequest](#api-v1alpha1-PublishRequest)           | [.google.protobuf.Empty](#google-protobuf-Empty) |             |
| Subscribe   | [SubscriptionRequest](#api-v1alpha1-SubscriptionRequest) | [CloudEvent](#api-v1alpha1-CloudEvent) stream    |             |

<a name="v1alpha1_health-proto"></a>

<p align="right"><a href="#top">Top</a></p>

## v1alpha1/health.proto

<a name="api-v1alpha1-HealthCheckRequest"></a>

### HealthCheckRequest

| Field   | Type              | Label | Description |
| ------- | ----------------- | ----- | ----------- |
| service | [string](#string) |       |             |

<a name="api-v1alpha1-HealthCheckResponse"></a>

### HealthCheckResponse

| Field  | Type                                                                                 | Label | Description |
| ------ | ------------------------------------------------------------------------------------ | ----- | ----------- |
| status | [HealthCheckResponse.ServingStatus](#api-v1alpha1-HealthCheckResponse-ServingStatus) |       |             |

<a name="api-v1alpha1-HealthCheckResponse-ServingStatus"></a>

### HealthCheckResponse.ServingStatus

| Name            | Number | Description        |
| --------------- | ------ | ------------------ |
| UNKNOWN         | 0      |                    |
| SERVING         | 1      |                    |
| NOT_SERVING     | 2      |                    |
| SERVICE_UNKNOWN | 3      | used only by Watch |

<a name="api-v1alpha1-Health"></a>

### Health

| Method Name | Request Type                                           | Response Type                                                   | Description                                                                                                                                                                                                                                          |
| ----------- | ------------------------------------------------------ | --------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Check       | [HealthCheckRequest](#api-v1alpha1-HealthCheckRequest) | [HealthCheckResponse](#api-v1alpha1-HealthCheckResponse)        | if the requested service is unknown, the call will fail with status NOT_FOUND                                                                                                                                                                        |
| Watch       | [HealthCheckRequest](#api-v1alpha1-HealthCheckRequest) | [HealthCheckResponse](#api-v1alpha1-HealthCheckResponse) stream | performs a watch for the serving status of the requested service the server will immediately send back a message indicating the curren serving status it will then subsequently send a new message whenever the service&#39;s serving status changes |

if the requested service is unknown when the call is received, the server will send a message setting the serving status to SERVICE_UNKNOWN but will _not_ terminate the call if at some future point, the serving status of the service becomes known, the server will send a new message with the service&#39;s serving status

if the call terminates with status UNIMPLEMENTED, then clients should assume this method is not supported and should not retry the call if the call terminates with any other status (including OK), clients should retry the call with appropriate exponential backoff |

<a name="v1alpha1_route_guide-proto"></a>

<p align="right"><a href="#top">Top</a></p>

## v1alpha1/route_guide.proto

<a name="api-v1alpha1-Feature"></a>

### Feature

A feature names something at a given point.
If a feature could not be named, the name is empty.

| Field    | Type                         | Label | Description                              |
| -------- | ---------------------------- | ----- | ---------------------------------------- |
| name     | [string](#string)            |       | The name of the feature.                 |
| location | [Point](#api-v1alpha1-Point) |       | The point where the feature is detected. |

<a name="api-v1alpha1-Point"></a>

### Point

Points are represented as latitude-longitude pairs in the E7 representation
(degrees multiplied by 10\*\*7 and rounded to the nearest integer).
Latitudes should be in the range &#43;/- 90 degrees and longitude should be in
the range &#43;/- 180 degrees (inclusive).

| Field     | Type            | Label | Description |
| --------- | --------------- | ----- | ----------- |
| latitude  | [int32](#int32) |       |             |
| longitude | [int32](#int32) |       |             |

<a name="api-v1alpha1-Rectangle"></a>

### Rectangle

A latitude-longitude rectangle, represented as two diagonally oposite
points &#34;lo&#34; and &#34;hi&#34;.

| Field | Type                         | Label | Description                        |
| ----- | ---------------------------- | ----- | ---------------------------------- |
| lo    | [Point](#api-v1alpha1-Point) |       | One corner of the rectangle.       |
| hi    | [Point](#api-v1alpha1-Point) |       | The other corner of the rectangle. |

<a name="api-v1alpha1-RouteNote"></a>

### RouteNote

A RouteNote is a message sent while at a given point.

| Field    | Type                         | Label | Description                                  |
| -------- | ---------------------------- | ----- | -------------------------------------------- |
| location | [Point](#api-v1alpha1-Point) |       | The location from which the message is sent. |
| message  | [string](#string)            |       | The message to be sent.                      |

<a name="api-v1alpha1-RouteSummary"></a>

### RouteSummary

A RouteSummary is received in response to a RecordRoute rpc.

| Field         | Type            | Label | Description                                                    |
| ------------- | --------------- | ----- | -------------------------------------------------------------- |
| point_count   | [int32](#int32) |       | The number of points received.                                 |
| feature_count | [int32](#int32) |       | The number of known feature passed while traversing the route. |
| distance      | [int32](#int32) |       | The distance covered in metres.                                |
| elapsed_time  | [int32](#int32) |       | The duration of the traversal in seconds.                      |

<a name="api-v1alpha1-RouteGuide"></a>

### RouteGuide

Interface exported by the server.

| Method Name    | Request Type                                | Response Type                               | Description                                                                                                                                                                                                                                     |
| -------------- | ------------------------------------------- | ------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| GetFeature     | [Point](#api-v1alpha1-Point)                | [Feature](#api-v1alpha1-Feature)            | Obtains the feature at a given position.                                                                                                                                                                                                        |
| ListenFeatures | [Rectangle](#api-v1alpha1-Rectangle)        | [Feature](#api-v1alpha1-Feature) stream     | Obtains the Features available within the given Rectangle. Results are streamed rather than returned at once (e.g. in a response message with a repeated field), as the rectangle may cover a large area and contain a huge number of features. |
| RecordRoute    | [Point](#api-v1alpha1-Point) stream         | [RouteSummary](#api-v1alpha1-RouteSummary)  | Accepts a stream of Points on a route being traversed, returning a RouteSummary when traversal is completed.                                                                                                                                    |
| Routechat      | [RouteNote](#api-v1alpha1-RouteNote) stream | [RouteNote](#api-v1alpha1-RouteNote) stream | Accepts a stream of RouteNotes sent while a route is being traversed, while receiving other RouteNotes (e.g. from other users).                                                                                                                 |

## Scalar Value Types

| .proto Type                    | Notes                                                                                                                                           | C++    | Java       | Python      | Go      | C#         | PHP            | Ruby                           |
| ------------------------------ | ----------------------------------------------------------------------------------------------------------------------------------------------- | ------ | ---------- | ----------- | ------- | ---------- | -------------- | ------------------------------ |
| <a name="double" /> double     |                                                                                                                                                 | double | double     | float       | float64 | double     | float          | Float                          |
| <a name="float" /> float       |                                                                                                                                                 | float  | float      | float       | float32 | float      | float          | Float                          |
| <a name="int32" /> int32       | Uses variable-length encoding. Inefficient for encoding negative numbers – if your field is likely to have negative values, use sint32 instead. | int32  | int        | int         | int32   | int        | integer        | Bignum or Fixnum (as required) |
| <a name="int64" /> int64       | Uses variable-length encoding. Inefficient for encoding negative numbers – if your field is likely to have negative values, use sint64 instead. | int64  | long       | int/long    | int64   | long       | integer/string | Bignum                         |
| <a name="uint32" /> uint32     | Uses variable-length encoding.                                                                                                                  | uint32 | int        | int/long    | uint32  | uint       | integer        | Bignum or Fixnum (as required) |
| <a name="uint64" /> uint64     | Uses variable-length encoding.                                                                                                                  | uint64 | long       | int/long    | uint64  | ulong      | integer/string | Bignum or Fixnum (as required) |
| <a name="sint32" /> sint32     | Uses variable-length encoding. Signed int value. These more efficiently encode negative numbers than regular int32s.                            | int32  | int        | int         | int32   | int        | integer        | Bignum or Fixnum (as required) |
| <a name="sint64" /> sint64     | Uses variable-length encoding. Signed int value. These more efficiently encode negative numbers than regular int64s.                            | int64  | long       | int/long    | int64   | long       | integer/string | Bignum                         |
| <a name="fixed32" /> fixed32   | Always four bytes. More efficient than uint32 if values are often greater than 2^28.                                                            | uint32 | int        | int         | uint32  | uint       | integer        | Bignum or Fixnum (as required) |
| <a name="fixed64" /> fixed64   | Always eight bytes. More efficient than uint64 if values are often greater than 2^56.                                                           | uint64 | long       | int/long    | uint64  | ulong      | integer/string | Bignum                         |
| <a name="sfixed32" /> sfixed32 | Always four bytes.                                                                                                                              | int32  | int        | int         | int32   | int        | integer        | Bignum or Fixnum (as required) |
| <a name="sfixed64" /> sfixed64 | Always eight bytes.                                                                                                                             | int64  | long       | int/long    | int64   | long       | integer/string | Bignum                         |
| <a name="bool" /> bool         |                                                                                                                                                 | bool   | boolean    | boolean     | bool    | bool       | boolean        | TrueClass/FalseClass           |
| <a name="string" /> string     | A string must always contain UTF-8 encoded or 7-bit ASCII text.                                                                                 | string | String     | str/unicode | string  | string     | string         | String (UTF-8)                 |
| <a name="bytes" /> bytes       | May contain any arbitrary sequence of bytes.                                                                                                    | string | ByteString | str         | []byte  | ByteString | string         | String (ASCII-8BIT)            |
