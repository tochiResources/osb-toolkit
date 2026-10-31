> Clean SDK type: `EncodeVideoOptions` ([`Media`](../libraries/Media.md)). Handle properties (`layerHandle`, `sprite` and numeric `layer`) are injected by the handle-bound client.

# EncodeVideoRequest
## Properties

| Name | Type | Description | Notes |
|------------ | ------------- | ------------- | -------------|
| **codec** | **String** | The video codec (default &#x60;libx264&#x60;). | [optional] [default to null] |
| **crf** | **Integer** | The constant-rate-factor quality (&#x60;0&#x60;-&#x60;51&#x60;; lower is better). | [optional] [default to null] |
| **fps** | **Double** | The frame rate. | [default to null] |
| **inputPattern** | **String** | The input filename pattern (e.g. &#x60;frames/frame_%04d.png&#x60;). | [default to null] |
| **output** | **String** | The output video path. | [default to null] |



