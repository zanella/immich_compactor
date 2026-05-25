# SystemConfigFFmpegDto

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**accel** | [**models::TranscodeHwAccel**](TranscodeHWAccel.md) |  | 
**accel_decode** | **bool** | Accelerated decode | 
**accepted_audio_codecs** | [**Vec<models::AudioCodec>**](AudioCodec.md) | Accepted audio codecs | 
**accepted_containers** | [**Vec<models::VideoContainer>**](VideoContainer.md) | Accepted containers | 
**accepted_video_codecs** | [**Vec<models::VideoCodec>**](VideoCodec.md) | Accepted video codecs | 
**bframes** | **i32** | B-frames | 
**cq_mode** | [**models::CqMode**](CQMode.md) |  | 
**crf** | **i32** | CRF | 
**gop_size** | **i32** | GOP size | 
**max_bitrate** | **String** | Max bitrate | 
**preferred_hw_device** | **String** | Preferred hardware device | 
**preset** | **String** | Preset | 
**refs** | **i32** | References | 
**target_audio_codec** | [**models::AudioCodec**](AudioCodec.md) |  | 
**target_resolution** | **String** | Target resolution | 
**target_video_codec** | [**models::VideoCodec**](VideoCodec.md) |  | 
**temporal_aq** | **bool** | Temporal AQ | 
**threads** | **i32** | Threads | 
**tonemap** | [**models::ToneMapping**](ToneMapping.md) |  | 
**transcode** | [**models::TranscodePolicy**](TranscodePolicy.md) |  | 
**two_pass** | **bool** | Two pass | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


