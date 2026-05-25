# \AuthenticationApi

All URIs are relative to */api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**change_password**](AuthenticationApi.md#change_password) | **POST** /auth/change-password | Change password
[**change_pin_code**](AuthenticationApi.md#change_pin_code) | **PUT** /auth/pin-code | Change pin code
[**finish_o_auth**](AuthenticationApi.md#finish_o_auth) | **POST** /oauth/callback | Finish OAuth
[**get_auth_status**](AuthenticationApi.md#get_auth_status) | **GET** /auth/status | Retrieve auth status
[**link_o_auth_account**](AuthenticationApi.md#link_o_auth_account) | **POST** /oauth/link | Link OAuth account
[**lock_auth_session**](AuthenticationApi.md#lock_auth_session) | **POST** /auth/session/lock | Lock auth session
[**login**](AuthenticationApi.md#login) | **POST** /auth/login | Login
[**logout**](AuthenticationApi.md#logout) | **POST** /auth/logout | Logout
[**logout_o_auth**](AuthenticationApi.md#logout_o_auth) | **POST** /oauth/backchannel-logout | Backchannel OAuth logout
[**redirect_o_auth_to_mobile**](AuthenticationApi.md#redirect_o_auth_to_mobile) | **GET** /oauth/mobile-redirect | Redirect OAuth to mobile
[**reset_pin_code**](AuthenticationApi.md#reset_pin_code) | **DELETE** /auth/pin-code | Reset pin code
[**setup_pin_code**](AuthenticationApi.md#setup_pin_code) | **POST** /auth/pin-code | Setup pin code
[**sign_up_admin**](AuthenticationApi.md#sign_up_admin) | **POST** /auth/admin-sign-up | Register admin
[**start_o_auth**](AuthenticationApi.md#start_o_auth) | **POST** /oauth/authorize | Start OAuth
[**unlink_o_auth_account**](AuthenticationApi.md#unlink_o_auth_account) | **POST** /oauth/unlink | Unlink OAuth account
[**unlock_auth_session**](AuthenticationApi.md#unlock_auth_session) | **POST** /auth/session/unlock | Unlock auth session
[**validate_access_token**](AuthenticationApi.md#validate_access_token) | **POST** /auth/validateToken | Validate access token



## change_password

> models::UserAdminResponseDto change_password(change_password_dto)
Change password

Change the password of the current user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**change_password_dto** | [**ChangePasswordDto**](ChangePasswordDto.md) |  | [required] |

### Return type

[**models::UserAdminResponseDto**](UserAdminResponseDto.md)

### Authorization

[cookie](../README.md#cookie), [api_key](../README.md#api_key), [bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## change_pin_code

> change_pin_code(pin_code_change_dto)
Change pin code

Change the pin code for the current user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**pin_code_change_dto** | [**PinCodeChangeDto**](PinCodeChangeDto.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[cookie](../README.md#cookie), [api_key](../README.md#api_key), [bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## finish_o_auth

> models::LoginResponseDto finish_o_auth(o_auth_callback_dto)
Finish OAuth

Complete the OAuth authorization process by exchanging the authorization code for a session token.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**o_auth_callback_dto** | [**OAuthCallbackDto**](OAuthCallbackDto.md) |  | [required] |

### Return type

[**models::LoginResponseDto**](LoginResponseDto.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_auth_status

> models::AuthStatusResponseDto get_auth_status()
Retrieve auth status

Get information about the current session, including whether the user has a password, and if the session can access locked assets.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::AuthStatusResponseDto**](AuthStatusResponseDto.md)

### Authorization

[cookie](../README.md#cookie), [api_key](../README.md#api_key), [bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## link_o_auth_account

> models::UserAdminResponseDto link_o_auth_account(o_auth_callback_dto)
Link OAuth account

Link an OAuth account to the authenticated user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**o_auth_callback_dto** | [**OAuthCallbackDto**](OAuthCallbackDto.md) |  | [required] |

### Return type

[**models::UserAdminResponseDto**](UserAdminResponseDto.md)

### Authorization

[cookie](../README.md#cookie), [api_key](../README.md#api_key), [bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## lock_auth_session

> lock_auth_session()
Lock auth session

Remove elevated access to locked assets from the current session.

### Parameters

This endpoint does not need any parameter.

### Return type

 (empty response body)

### Authorization

[cookie](../README.md#cookie), [api_key](../README.md#api_key), [bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## login

> models::LoginResponseDto login(login_credential_dto)
Login

Login with username and password and receive a session token.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**login_credential_dto** | [**LoginCredentialDto**](LoginCredentialDto.md) |  | [required] |

### Return type

[**models::LoginResponseDto**](LoginResponseDto.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## logout

> models::LogoutResponseDto logout()
Logout

Logout the current user and invalidate the session token.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::LogoutResponseDto**](LogoutResponseDto.md)

### Authorization

[cookie](../README.md#cookie), [api_key](../README.md#api_key), [bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## logout_o_auth

> logout_o_auth(logout_token)
Backchannel OAuth logout

Logout the OAuth account and invalidate the session specified by the sid claim or all sessions if the sid claim is not present.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**logout_token** | **String** | OAuth logout token | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## redirect_o_auth_to_mobile

> redirect_o_auth_to_mobile()
Redirect OAuth to mobile

Requests to this URL are automatically forwarded to the mobile app, and is used in some cases for OAuth redirecting.

### Parameters

This endpoint does not need any parameter.

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## reset_pin_code

> reset_pin_code(pin_code_reset_dto)
Reset pin code

Reset the pin code for the current user by providing the account password

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**pin_code_reset_dto** | [**PinCodeResetDto**](PinCodeResetDto.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[cookie](../README.md#cookie), [api_key](../README.md#api_key), [bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## setup_pin_code

> setup_pin_code(pin_code_setup_dto)
Setup pin code

Setup a new pin code for the current user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**pin_code_setup_dto** | [**PinCodeSetupDto**](PinCodeSetupDto.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[cookie](../README.md#cookie), [api_key](../README.md#api_key), [bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sign_up_admin

> models::UserAdminResponseDto sign_up_admin(sign_up_dto)
Register admin

Create the first admin user in the system.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**sign_up_dto** | [**SignUpDto**](SignUpDto.md) |  | [required] |

### Return type

[**models::UserAdminResponseDto**](UserAdminResponseDto.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## start_o_auth

> models::OAuthAuthorizeResponseDto start_o_auth(o_auth_config_dto)
Start OAuth

Initiate the OAuth authorization process.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**o_auth_config_dto** | [**OAuthConfigDto**](OAuthConfigDto.md) |  | [required] |

### Return type

[**models::OAuthAuthorizeResponseDto**](OAuthAuthorizeResponseDto.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## unlink_o_auth_account

> models::UserAdminResponseDto unlink_o_auth_account()
Unlink OAuth account

Unlink the OAuth account from the authenticated user.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::UserAdminResponseDto**](UserAdminResponseDto.md)

### Authorization

[cookie](../README.md#cookie), [api_key](../README.md#api_key), [bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## unlock_auth_session

> unlock_auth_session(session_unlock_dto)
Unlock auth session

Temporarily grant the session elevated access to locked assets by providing the correct PIN code.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**session_unlock_dto** | [**SessionUnlockDto**](SessionUnlockDto.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[cookie](../README.md#cookie), [api_key](../README.md#api_key), [bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## validate_access_token

> models::ValidateAccessTokenResponseDto validate_access_token()
Validate access token

Validate the current authorization method is still valid.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::ValidateAccessTokenResponseDto**](ValidateAccessTokenResponseDto.md)

### Authorization

[cookie](../README.md#cookie), [api_key](../README.md#api_key), [bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

