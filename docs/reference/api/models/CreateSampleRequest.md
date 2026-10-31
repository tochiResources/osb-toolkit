> Clean SDK type: `CreateSampleOptions` ([`Layer`](../runtime/Layer.md)). Handle properties (`layerHandle`, `sprite` and numeric `layer`) are injected by the handle-bound client.

# CreateSampleRequest
## Properties

| Name | Type | Description | Notes |
|------------ | ------------- | ------------- | -------------|
| **layer** | **String** | The target layer name (omitted when &#x60;layerHandle&#x60; is set). | [optional] [default to null] |
| **layerHandle** | **Integer** | The target segment handle (takes precedence over &#x60;layer&#x60;). | [optional] [default to null] |
| **path** | **String** | The sample path (relative to the mapset). | [default to null] |
| **time** | **Double** | The sample time in milliseconds. | [default to null] |
| **volume** | **Double** | The sample volume (&#x60;0..&#x3D;100&#x60;). | [optional] [default to null] |



