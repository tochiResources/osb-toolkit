> Clean SDK type: `CommandOptions` ([`Sprite`](../runtime/Sprite.md)). Handle properties (`layerHandle`, `sprite` and numeric `layer`) are injected by the handle-bound client.

# CommandRequest
## Properties

| Name | Type | Description | Notes |
|------------ | ------------- | ------------- | -------------|
| **easing** | [**Easing**](Easing.md) | The easing function. | [default to null] |
| **end** | [**Object**](.md) | The end value (scalar, point or color). | [optional] [default to null] |
| **endTime** | **Double** | The command end time in milliseconds. | [default to null] |
| **kind** | **String** | The command kind (&#x60;move&#x60;, &#x60;moveX&#x60;, &#x60;moveY&#x60;, &#x60;scale&#x60;, &#x60;scaleVec&#x60;, &#x60;rotate&#x60;, &#x60;fade&#x60;, &#x60;color&#x60;, &#x60;colorHsb&#x60;, &#x60;anchor&#x60;, &#x60;parameter&#x60;, &#x60;flipH&#x60;, &#x60;flipV&#x60;, &#x60;additive&#x60;). | [default to null] |
| **layer** | **Integer** | The layer handle. | [default to null] |
| **parameter** | [**ParameterType**](ParameterType.md) | The parameter kind for a &#x60;parameter&#x60; command. | [optional] [default to null] |
| **sprite** | **Integer** | The sprite handle. | [default to null] |
| **start** | [**Object**](.md) | The start value (scalar, point or color). | [optional] [default to null] |
| **startTime** | **Double** | The command start time in milliseconds. | [default to null] |



