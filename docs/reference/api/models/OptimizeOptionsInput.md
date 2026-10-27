# OptimizeOptionsInput
## Properties

| Name | Type | Description | Notes |
|------------ | ------------- | ------------- | -------------|
| **deduplicate** | **Boolean** | Remove commands identical to an earlier command of the same type. | [optional] [default to null] |
| **mergeAdjacent** | **Boolean** | Merge a command with the next when they are continuous. | [optional] [default to null] |
| **removeRedundant** | **Boolean** | Remove zero-duration commands whose value does not change. | [optional] [default to null] |
| **splitThreshold** | **Double** | Split commands longer than this many milliseconds (into linear pieces). | [optional] [default to null] |



