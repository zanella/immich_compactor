# AssetOcrResponseDto

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**asset_id** | **uuid::Uuid** |  | 
**box_score** | **f64** | Confidence score for text detection box | 
**id** | **uuid::Uuid** |  | 
**text** | **String** | Recognized text | 
**text_score** | **f64** | Confidence score for text recognition | 
**x1** | **f64** | Normalized x coordinate of box corner 1 (0-1) | 
**x2** | **f64** | Normalized x coordinate of box corner 2 (0-1) | 
**x3** | **f64** | Normalized x coordinate of box corner 3 (0-1) | 
**x4** | **f64** | Normalized x coordinate of box corner 4 (0-1) | 
**y1** | **f64** | Normalized y coordinate of box corner 1 (0-1) | 
**y2** | **f64** | Normalized y coordinate of box corner 2 (0-1) | 
**y3** | **f64** | Normalized y coordinate of box corner 3 (0-1) | 
**y4** | **f64** | Normalized y coordinate of box corner 4 (0-1) | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


