# Node3dInput
## Properties

| Name | Type | Description | Notes |
|------------ | ------------- | ------------- | -------------|
| **additive** | **Boolean** | Whether the sprite composites additively. | [optional] [default to null] |
| **children** | [**List**](Node3dInput.md) | Child nodes. | [optional] [default to null] |
| **frameDelay** | **Double** | Delay between animation frames, in milliseconds. | [optional] [default to null] |
| **frames** | **List** | Animation frame paths (empty &#x3D; a still sprite). | [optional] [default to null] |
| **inheritsRotation** | **Boolean** | Whether the node inherits its parent&#39;s rotation. | [optional] [default to null] |
| **inheritsZDepth** | **Boolean** | Whether the node inherits its parent&#39;s depth. | [optional] [default to null] |
| **origin** | **Integer** | The sprite anchor origin id (&#x60;0..8&#x60;). | [optional] [default to null] |
| **pivot** | [**List**](Keyframe3.md) | The local pivot rotation/scale act about. | [optional] [default to null] |
| **position** | [**List**](Keyframe3.md) | World position keyframes. | [optional] [default to null] |
| **rotation** | [**List**](Keyframe3.md) | Euler rotation keyframes in radians. | [optional] [default to null] |
| **rotationMode** | **String** | The legacy orientation axis (&#x60;fixed&#x60;, &#x60;unitX&#x60;, &#x60;unitY&#x60;). | [optional] [default to null] |
| **scale** | [**List**](Keyframe3.md) | World scale keyframes. | [optional] [default to null] |
| **sprite** | **String** | The sprite texture path. | [default to null] |
| **spriteRotation** | [**List**](Keyframe1.md) | Per-sprite rotation keyframes (radians). | [optional] [default to null] |
| **spriteScale** | [**List**](Keyframe2.md) | Per-sprite scale keyframes. | [optional] [default to null] |
| **zDepth** | [**List**](Keyframe1.md) | Painter&#39;s-algorithm depth keyframes. | [optional] [default to null] |



