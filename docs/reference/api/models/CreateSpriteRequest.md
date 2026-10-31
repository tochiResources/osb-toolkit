> Clean SDK type: `CreateSpriteOptions` ([`Layer`](../runtime/Layer.md)). Handle properties (`layerHandle`, `sprite` and numeric `layer`) are injected by the handle-bound client.

# CreateSpriteRequest
## Properties

| Name | Type | Description | Notes |
|------------ | ------------- | ------------- | -------------|
| **layer** | **String** | The target layer name (omitted when &#x60;layerHandle&#x60; is set). | [optional] [default to null] |
| **layerHandle** | **Integer** | The target segment handle (takes precedence over &#x60;layer&#x60;). | [optional] [default to null] |
| **origin** | [**OsbOrigin**](OsbOrigin.md) | The sprite anchor origin. | [default to null] |
| **path** | **String** | The texture path (relative to the mapset). | [default to null] |
| **position** | [**Vector2**](Vector2.md) | The initial position; omitted means the sprite&#39;s default. | [optional] [default to null] |



