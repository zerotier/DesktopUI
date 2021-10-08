use ::prelude::*;
DEFINE_IID!(IID_IUriToStreamResolver, 2964039786, 39659, 19770, 149, 144, 0, 62, 60, 167, 226, 144);
RT_INTERFACE!{interface IUriToStreamResolver(IUriToStreamResolverVtbl): IInspectable(IInspectableVtbl) [IID_IUriToStreamResolver] {
    #[cfg(feature="windows-storage")] fn UriToStreamAsync(&self, uri: *mut super::foundation::Uri, out: *mut *mut super::foundation::IAsyncOperation<super::storage::streams::IInputStream>) -> HRESULT
}}
impl IUriToStreamResolver {
    #[cfg(feature="windows-storage")] #[inline] pub unsafe fn uri_to_stream_async(&self, uri: &super::foundation::Uri) -> Result<ComPtr<super::foundation::IAsyncOperation<super::storage::streams::IInputStream>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).UriToStreamAsync)(self as *const _ as *mut _, uri as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
RT_CLASS!{static class WebError}
impl RtActivatable<IWebErrorStatics> for WebError {}
impl WebError {
    #[inline] pub fn get_status(hresult: i32) -> Result<WebErrorStatus> { unsafe {
        <Self as RtActivatable<IWebErrorStatics>>::get_activation_factory().get_status(hresult)
    }}
}
DEFINE_CLSID!(WebError(&[87,105,110,100,111,119,115,46,87,101,98,46,87,101,98,69,114,114,111,114,0]) [CLSID_WebError]);
DEFINE_IID!(IID_IWebErrorStatics, 4267796326, 48935, 16484, 135, 183, 101, 99, 187, 17, 206, 46);
RT_INTERFACE!{static interface IWebErrorStatics(IWebErrorStaticsVtbl): IInspectable(IInspectableVtbl) [IID_IWebErrorStatics] {
    fn GetStatus(&self, hresult: i32, out: *mut WebErrorStatus) -> HRESULT
}}
impl IWebErrorStatics {
    #[inline] pub unsafe fn get_status(&self, hresult: i32) -> Result<WebErrorStatus> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).GetStatus)(self as *const _ as *mut _, hresult, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
}
RT_ENUM! { enum WebErrorStatus: i32 {
    Unknown (WebErrorStatus_Unknown) = 0, CertificateCommonNameIsIncorrect (WebErrorStatus_CertificateCommonNameIsIncorrect) = 1, CertificateExpired (WebErrorStatus_CertificateExpired) = 2, CertificateContainsErrors (WebErrorStatus_CertificateContainsErrors) = 3, CertificateRevoked (WebErrorStatus_CertificateRevoked) = 4, CertificateIsInvalid (WebErrorStatus_CertificateIsInvalid) = 5, ServerUnreachable (WebErrorStatus_ServerUnreachable) = 6, Timeout (WebErrorStatus_Timeout) = 7, ErrorHttpInvalidServerResponse (WebErrorStatus_ErrorHttpInvalidServerResponse) = 8, ConnectionAborted (WebErrorStatus_ConnectionAborted) = 9, ConnectionReset (WebErrorStatus_ConnectionReset) = 10, Disconnected (WebErrorStatus_Disconnected) = 11, HttpToHttpsOnRedirection (WebErrorStatus_HttpToHttpsOnRedirection) = 12, HttpsToHttpOnRedirection (WebErrorStatus_HttpsToHttpOnRedirection) = 13, CannotConnect (WebErrorStatus_CannotConnect) = 14, HostNameNotResolved (WebErrorStatus_HostNameNotResolved) = 15, OperationCanceled (WebErrorStatus_OperationCanceled) = 16, RedirectFailed (WebErrorStatus_RedirectFailed) = 17, UnexpectedStatusCode (WebErrorStatus_UnexpectedStatusCode) = 18, UnexpectedRedirection (WebErrorStatus_UnexpectedRedirection) = 19, UnexpectedClientError (WebErrorStatus_UnexpectedClientError) = 20, UnexpectedServerError (WebErrorStatus_UnexpectedServerError) = 21, InsufficientRangeSupport (WebErrorStatus_InsufficientRangeSupport) = 22, MissingContentLengthSupport (WebErrorStatus_MissingContentLengthSupport) = 23, MultipleChoices (WebErrorStatus_MultipleChoices) = 300, MovedPermanently (WebErrorStatus_MovedPermanently) = 301, Found (WebErrorStatus_Found) = 302, SeeOther (WebErrorStatus_SeeOther) = 303, NotModified (WebErrorStatus_NotModified) = 304, UseProxy (WebErrorStatus_UseProxy) = 305, TemporaryRedirect (WebErrorStatus_TemporaryRedirect) = 307, BadRequest (WebErrorStatus_BadRequest) = 400, Unauthorized (WebErrorStatus_Unauthorized) = 401, PaymentRequired (WebErrorStatus_PaymentRequired) = 402, Forbidden (WebErrorStatus_Forbidden) = 403, NotFound (WebErrorStatus_NotFound) = 404, MethodNotAllowed (WebErrorStatus_MethodNotAllowed) = 405, NotAcceptable (WebErrorStatus_NotAcceptable) = 406, ProxyAuthenticationRequired (WebErrorStatus_ProxyAuthenticationRequired) = 407, RequestTimeout (WebErrorStatus_RequestTimeout) = 408, Conflict (WebErrorStatus_Conflict) = 409, Gone (WebErrorStatus_Gone) = 410, LengthRequired (WebErrorStatus_LengthRequired) = 411, PreconditionFailed (WebErrorStatus_PreconditionFailed) = 412, RequestEntityTooLarge (WebErrorStatus_RequestEntityTooLarge) = 413, RequestUriTooLong (WebErrorStatus_RequestUriTooLong) = 414, UnsupportedMediaType (WebErrorStatus_UnsupportedMediaType) = 415, RequestedRangeNotSatisfiable (WebErrorStatus_RequestedRangeNotSatisfiable) = 416, ExpectationFailed (WebErrorStatus_ExpectationFailed) = 417, InternalServerError (WebErrorStatus_InternalServerError) = 500, NotImplemented (WebErrorStatus_NotImplemented) = 501, BadGateway (WebErrorStatus_BadGateway) = 502, ServiceUnavailable (WebErrorStatus_ServiceUnavailable) = 503, GatewayTimeout (WebErrorStatus_GatewayTimeout) = 504, HttpVersionNotSupported (WebErrorStatus_HttpVersionNotSupported) = 505,
}}
pub mod http { // Windows.Web.Http
use ::prelude::*;
RT_CLASS!{class HttpBufferContent: IHttpContent}
impl RtActivatable<IHttpBufferContentFactory> for HttpBufferContent {}
impl HttpBufferContent {
    #[cfg(feature="windows-storage")] #[inline] pub fn create_from_buffer(content: &super::super::storage::streams::IBuffer) -> Result<ComPtr<HttpBufferContent>> { unsafe {
        <Self as RtActivatable<IHttpBufferContentFactory>>::get_activation_factory().create_from_buffer(content)
    }}
    #[cfg(feature="windows-storage")] #[inline] pub fn create_from_buffer_with_offset(content: &super::super::storage::streams::IBuffer, offset: u32, count: u32) -> Result<ComPtr<HttpBufferContent>> { unsafe {
        <Self as RtActivatable<IHttpBufferContentFactory>>::get_activation_factory().create_from_buffer_with_offset(content, offset, count)
    }}
}
DEFINE_CLSID!(HttpBufferContent(&[87,105,110,100,111,119,115,46,87,101,98,46,72,116,116,112,46,72,116,116,112,66,117,102,102,101,114,67,111,110,116,101,110,116,0]) [CLSID_HttpBufferContent]);
DEFINE_IID!(IID_IHttpBufferContentFactory, 3156263315, 50207, 20471, 145, 35, 100, 53, 115, 110, 173, 194);
RT_INTERFACE!{static interface IHttpBufferContentFactory(IHttpBufferContentFactoryVtbl): IInspectable(IInspectableVtbl) [IID_IHttpBufferContentFactory] {
    #[cfg(feature="windows-storage")] fn CreateFromBuffer(&self, content: *mut super::super::storage::streams::IBuffer, out: *mut *mut HttpBufferContent) -> HRESULT,
    #[cfg(feature="windows-storage")] fn CreateFromBufferWithOffset(&self, content: *mut super::super::storage::streams::IBuffer, offset: u32, count: u32, out: *mut *mut HttpBufferContent) -> HRESULT
}}
impl IHttpBufferContentFactory {
    #[cfg(feature="windows-storage")] #[inline] pub unsafe fn create_from_buffer(&self, content: &super::super::storage::streams::IBuffer) -> Result<ComPtr<HttpBufferContent>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).CreateFromBuffer)(self as *const _ as *mut _, content as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[cfg(feature="windows-storage")] #[inline] pub unsafe fn create_from_buffer_with_offset(&self, content: &super::super::storage::streams::IBuffer, offset: u32, count: u32) -> Result<ComPtr<HttpBufferContent>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).CreateFromBufferWithOffset)(self as *const _ as *mut _, content as *const _ as *mut _, offset, count, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
DEFINE_IID!(IID_IHttpClient, 2144997713, 13684, 18560, 168, 186, 230, 177, 224, 6, 31, 61);
RT_INTERFACE!{interface IHttpClient(IHttpClientVtbl): IInspectable(IInspectableVtbl) [IID_IHttpClient] {
    fn DeleteAsync(&self, uri: *mut super::super::foundation::Uri, out: *mut *mut super::super::foundation::IAsyncOperationWithProgress<HttpResponseMessage, HttpProgress>) -> HRESULT,
    fn GetAsync(&self, uri: *mut super::super::foundation::Uri, out: *mut *mut super::super::foundation::IAsyncOperationWithProgress<HttpResponseMessage, HttpProgress>) -> HRESULT,
    fn GetWithOptionAsync(&self, uri: *mut super::super::foundation::Uri, completionOption: HttpCompletionOption, out: *mut *mut super::super::foundation::IAsyncOperationWithProgress<HttpResponseMessage, HttpProgress>) -> HRESULT,
    #[cfg(not(feature="windows-storage"))] fn __Dummy3(&self) -> (),
    #[cfg(feature="windows-storage")] fn GetBufferAsync(&self, uri: *mut super::super::foundation::Uri, out: *mut *mut super::super::foundation::IAsyncOperationWithProgress<super::super::storage::streams::IBuffer, HttpProgress>) -> HRESULT,
    #[cfg(not(feature="windows-storage"))] fn __Dummy4(&self) -> (),
    #[cfg(feature="windows-storage")] fn GetInputStreamAsync(&self, uri: *mut super::super::foundation::Uri, out: *mut *mut super::super::foundation::IAsyncOperationWithProgress<super::super::storage::streams::IInputStream, HttpProgress>) -> HRESULT,
    fn GetStringAsync(&self, uri: *mut super::super::foundation::Uri, out: *mut *mut super::super::foundation::IAsyncOperationWithProgress<HString, HttpProgress>) -> HRESULT,
    fn PostAsync(&self, uri: *mut super::super::foundation::Uri, content: *mut IHttpContent, out: *mut *mut super::super::foundation::IAsyncOperationWithProgress<HttpResponseMessage, HttpProgress>) -> HRESULT,
    fn PutAsync(&self, uri: *mut super::super::foundation::Uri, content: *mut IHttpContent, out: *mut *mut super::super::foundation::IAsyncOperationWithProgress<HttpResponseMessage, HttpProgress>) -> HRESULT,
    fn SendRequestAsync(&self, request: *mut HttpRequestMessage, out: *mut *mut super::super::foundation::IAsyncOperationWithProgress<HttpResponseMessage, HttpProgress>) -> HRESULT,
    fn SendRequestWithOptionAsync(&self, request: *mut HttpRequestMessage, completionOption: HttpCompletionOption, out: *mut *mut super::super::foundation::IAsyncOperationWithProgress<HttpResponseMessage, HttpProgress>) -> HRESULT,
    fn get_DefaultRequestHeaders(&self, out: *mut *mut headers::HttpRequestHeaderCollection) -> HRESULT
}}
impl IHttpClient {
    #[inline] pub unsafe fn delete_async(&self, uri: &super::super::foundation::Uri) -> Result<ComPtr<super::super::foundation::IAsyncOperationWithProgress<HttpResponseMessage, HttpProgress>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).DeleteAsync)(self as *const _ as *mut _, uri as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_async(&self, uri: &super::super::foundation::Uri) -> Result<ComPtr<super::super::foundation::IAsyncOperationWithProgress<HttpResponseMessage, HttpProgress>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetAsync)(self as *const _ as *mut _, uri as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_with_option_async(&self, uri: &super::super::foundation::Uri, completionOption: HttpCompletionOption) -> Result<ComPtr<super::super::foundation::IAsyncOperationWithProgress<HttpResponseMessage, HttpProgress>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetWithOptionAsync)(self as *const _ as *mut _, uri as *const _ as *mut _, completionOption, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[cfg(feature="windows-storage")] #[inline] pub unsafe fn get_buffer_async(&self, uri: &super::super::foundation::Uri) -> Result<ComPtr<super::super::foundation::IAsyncOperationWithProgress<super::super::storage::streams::IBuffer, HttpProgress>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetBufferAsync)(self as *const _ as *mut _, uri as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[cfg(feature="windows-storage")] #[inline] pub unsafe fn get_input_stream_async(&self, uri: &super::super::foundation::Uri) -> Result<ComPtr<super::super::foundation::IAsyncOperationWithProgress<super::super::storage::streams::IInputStream, HttpProgress>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetInputStreamAsync)(self as *const _ as *mut _, uri as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_string_async(&self, uri: &super::super::foundation::Uri) -> Result<ComPtr<super::super::foundation::IAsyncOperationWithProgress<HString, HttpProgress>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetStringAsync)(self as *const _ as *mut _, uri as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn post_async(&self, uri: &super::super::foundation::Uri, content: &IHttpContent) -> Result<ComPtr<super::super::foundation::IAsyncOperationWithProgress<HttpResponseMessage, HttpProgress>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).PostAsync)(self as *const _ as *mut _, uri as *const _ as *mut _, content as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn put_async(&self, uri: &super::super::foundation::Uri, content: &IHttpContent) -> Result<ComPtr<super::super::foundation::IAsyncOperationWithProgress<HttpResponseMessage, HttpProgress>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).PutAsync)(self as *const _ as *mut _, uri as *const _ as *mut _, content as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn send_request_async(&self, request: &HttpRequestMessage) -> Result<ComPtr<super::super::foundation::IAsyncOperationWithProgress<HttpResponseMessage, HttpProgress>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).SendRequestAsync)(self as *const _ as *mut _, request as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn send_request_with_option_async(&self, request: &HttpRequestMessage, completionOption: HttpCompletionOption) -> Result<ComPtr<super::super::foundation::IAsyncOperationWithProgress<HttpResponseMessage, HttpProgress>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).SendRequestWithOptionAsync)(self as *const _ as *mut _, request as *const _ as *mut _, completionOption, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_default_request_headers(&self) -> Result<ComPtr<headers::HttpRequestHeaderCollection>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_DefaultRequestHeaders)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
RT_CLASS!{class HttpClient: IHttpClient}
impl RtActivatable<IHttpClientFactory> for HttpClient {}
impl RtActivatable<IActivationFactory> for HttpClient {}
impl HttpClient {
    #[inline] pub fn create(filter: &filters::IHttpFilter) -> Result<ComPtr<HttpClient>> { unsafe {
        <Self as RtActivatable<IHttpClientFactory>>::get_activation_factory().create(filter)
    }}
}
DEFINE_CLSID!(HttpClient(&[87,105,110,100,111,119,115,46,87,101,98,46,72,116,116,112,46,72,116,116,112,67,108,105,101,110,116,0]) [CLSID_HttpClient]);
DEFINE_IID!(IID_IHttpClientFactory, 3272363722, 58362, 20377, 175, 180, 99, 204, 101, 0, 148, 98);
RT_INTERFACE!{static interface IHttpClientFactory(IHttpClientFactoryVtbl): IInspectable(IInspectableVtbl) [IID_IHttpClientFactory] {
    fn Create(&self, filter: *mut filters::IHttpFilter, out: *mut *mut HttpClient) -> HRESULT
}}
impl IHttpClientFactory {
    #[inline] pub unsafe fn create(&self, filter: &filters::IHttpFilter) -> Result<ComPtr<HttpClient>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).Create)(self as *const _ as *mut _, filter as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
RT_ENUM! { enum HttpCompletionOption: i32 {
    ResponseContentRead (HttpCompletionOption_ResponseContentRead) = 0, ResponseHeadersRead (HttpCompletionOption_ResponseHeadersRead) = 1,
}}
DEFINE_IID!(IID_IHttpContent, 1796514881, 64423, 19410, 175, 10, 131, 157, 231, 194, 149, 218);
RT_INTERFACE!{interface IHttpContent(IHttpContentVtbl): IInspectable(IInspectableVtbl) [IID_IHttpContent] {
    fn get_Headers(&self, out: *mut *mut headers::HttpContentHeaderCollection) -> HRESULT,
    fn BufferAllAsync(&self, out: *mut *mut super::super::foundation::IAsyncOperationWithProgress<u64, u64>) -> HRESULT,
    #[cfg(feature="windows-storage")] fn ReadAsBufferAsync(&self, out: *mut *mut super::super::foundation::IAsyncOperationWithProgress<super::super::storage::streams::IBuffer, u64>) -> HRESULT,
    #[cfg(feature="windows-storage")] fn ReadAsInputStreamAsync(&self, out: *mut *mut super::super::foundation::IAsyncOperationWithProgress<super::super::storage::streams::IInputStream, u64>) -> HRESULT,
    fn ReadAsStringAsync(&self, out: *mut *mut super::super::foundation::IAsyncOperationWithProgress<HString, u64>) -> HRESULT,
    fn TryComputeLength(&self, length: *mut u64, out: *mut bool) -> HRESULT,
    #[cfg(feature="windows-storage")] fn WriteToStreamAsync(&self, outputStream: *mut super::super::storage::streams::IOutputStream, out: *mut *mut super::super::foundation::IAsyncOperationWithProgress<u64, u64>) -> HRESULT
}}
impl IHttpContent {
    #[inline] pub unsafe fn get_headers(&self) -> Result<ComPtr<headers::HttpContentHeaderCollection>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Headers)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn buffer_all_async(&self) -> Result<ComPtr<super::super::foundation::IAsyncOperationWithProgress<u64, u64>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).BufferAllAsync)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[cfg(feature="windows-storage")] #[inline] pub unsafe fn read_as_buffer_async(&self) -> Result<ComPtr<super::super::foundation::IAsyncOperationWithProgress<super::super::storage::streams::IBuffer, u64>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).ReadAsBufferAsync)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[cfg(feature="windows-storage")] #[inline] pub unsafe fn read_as_input_stream_async(&self) -> Result<ComPtr<super::super::foundation::IAsyncOperationWithProgress<super::super::storage::streams::IInputStream, u64>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).ReadAsInputStreamAsync)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn read_as_string_async(&self) -> Result<ComPtr<super::super::foundation::IAsyncOperationWithProgress<HString, u64>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).ReadAsStringAsync)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn try_compute_length(&self) -> Result<(u64, bool)> {
        let mut length = zeroed(); let mut out = zeroed();
        let hr = ((*self.lpVtbl).TryComputeLength)(self as *const _ as *mut _, &mut length, &mut out);
        if hr == S_OK { Ok((length, out)) } else { err(hr) }
    }
    #[cfg(feature="windows-storage")] #[inline] pub unsafe fn write_to_stream_async(&self, outputStream: &super::super::storage::streams::IOutputStream) -> Result<ComPtr<super::super::foundation::IAsyncOperationWithProgress<u64, u64>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).WriteToStreamAsync)(self as *const _ as *mut _, outputStream as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
DEFINE_IID!(IID_IHttpCookie, 525633762, 52269, 18297, 134, 167, 136, 241, 6, 135, 210, 73);
RT_INTERFACE!{interface IHttpCookie(IHttpCookieVtbl): IInspectable(IInspectableVtbl) [IID_IHttpCookie] {
    fn get_Name(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Domain(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Path(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Expires(&self, out: *mut *mut super::super::foundation::IReference<super::super::foundation::DateTime>) -> HRESULT,
    fn put_Expires(&self, value: *mut super::super::foundation::IReference<super::super::foundation::DateTime>) -> HRESULT,
    fn get_HttpOnly(&self, out: *mut bool) -> HRESULT,
    fn put_HttpOnly(&self, value: bool) -> HRESULT,
    fn get_Secure(&self, out: *mut bool) -> HRESULT,
    fn put_Secure(&self, value: bool) -> HRESULT,
    fn get_Value(&self, out: *mut HSTRING) -> HRESULT,
    fn put_Value(&self, value: HSTRING) -> HRESULT
}}
impl IHttpCookie {
    #[inline] pub unsafe fn get_name(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Name)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_domain(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Domain)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_path(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Path)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_expires(&self) -> Result<ComPtr<super::super::foundation::IReference<super::super::foundation::DateTime>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Expires)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_expires(&self, value: &super::super::foundation::IReference<super::super::foundation::DateTime>) -> Result<()> {
        let hr = ((*self.lpVtbl).put_Expires)(self as *const _ as *mut _, value as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_http_only(&self) -> Result<bool> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_HttpOnly)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_http_only(&self, value: bool) -> Result<()> {
        let hr = ((*self.lpVtbl).put_HttpOnly)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_secure(&self) -> Result<bool> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_Secure)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_secure(&self, value: bool) -> Result<()> {
        let hr = ((*self.lpVtbl).put_Secure)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_value(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Value)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_value(&self, value: &HStringArg) -> Result<()> {
        let hr = ((*self.lpVtbl).put_Value)(self as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
}
RT_CLASS!{class HttpCookie: IHttpCookie}
impl RtActivatable<IHttpCookieFactory> for HttpCookie {}
impl HttpCookie {
    #[inline] pub fn create(name: &HStringArg, domain: &HStringArg, path: &HStringArg) -> Result<ComPtr<HttpCookie>> { unsafe {
        <Self as RtActivatable<IHttpCookieFactory>>::get_activation_factory().create(name, domain, path)
    }}
}
DEFINE_CLSID!(HttpCookie(&[87,105,110,100,111,119,115,46,87,101,98,46,72,116,116,112,46,72,116,116,112,67,111,111,107,105,101,0]) [CLSID_HttpCookie]);
RT_CLASS!{class HttpCookieCollection: super::super::foundation::collections::IVectorView<HttpCookie>}
DEFINE_IID!(IID_IHttpCookieFactory, 1778746793, 37660, 19665, 169, 109, 194, 23, 1, 120, 92, 95);
RT_INTERFACE!{static interface IHttpCookieFactory(IHttpCookieFactoryVtbl): IInspectable(IInspectableVtbl) [IID_IHttpCookieFactory] {
    fn Create(&self, name: HSTRING, domain: HSTRING, path: HSTRING, out: *mut *mut HttpCookie) -> HRESULT
}}
impl IHttpCookieFactory {
    #[inline] pub unsafe fn create(&self, name: &HStringArg, domain: &HStringArg, path: &HStringArg) -> Result<ComPtr<HttpCookie>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).Create)(self as *const _ as *mut _, name.get(), domain.get(), path.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
DEFINE_IID!(IID_IHttpCookieManager, 2051217280, 52559, 20055, 168, 74, 91, 10, 83, 214, 187, 150);
RT_INTERFACE!{interface IHttpCookieManager(IHttpCookieManagerVtbl): IInspectable(IInspectableVtbl) [IID_IHttpCookieManager] {
    fn SetCookie(&self, cookie: *mut HttpCookie, out: *mut bool) -> HRESULT,
    fn SetCookieWithThirdParty(&self, cookie: *mut HttpCookie, thirdParty: bool, out: *mut bool) -> HRESULT,
    fn DeleteCookie(&self, cookie: *mut HttpCookie) -> HRESULT,
    fn GetCookies(&self, uri: *mut super::super::foundation::Uri, out: *mut *mut HttpCookieCollection) -> HRESULT
}}
impl IHttpCookieManager {
    #[inline] pub unsafe fn set_cookie(&self, cookie: &HttpCookie) -> Result<bool> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).SetCookie)(self as *const _ as *mut _, cookie as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_cookie_with_third_party(&self, cookie: &HttpCookie, thirdParty: bool) -> Result<bool> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).SetCookieWithThirdParty)(self as *const _ as *mut _, cookie as *const _ as *mut _, thirdParty, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn delete_cookie(&self, cookie: &HttpCookie) -> Result<()> {
        let hr = ((*self.lpVtbl).DeleteCookie)(self as *const _ as *mut _, cookie as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_cookies(&self, uri: &super::super::foundation::Uri) -> Result<ComPtr<HttpCookieCollection>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetCookies)(self as *const _ as *mut _, uri as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
RT_CLASS!{class HttpCookieManager: IHttpCookieManager}
RT_CLASS!{class HttpFormUrlEncodedContent: IHttpContent}
impl RtActivatable<IHttpFormUrlEncodedContentFactory> for HttpFormUrlEncodedContent {}
impl HttpFormUrlEncodedContent {
    #[inline] pub fn create(content: &super::super::foundation::collections::IIterable<super::super::foundation::collections::IKeyValuePair<HString, HString>>) -> Result<ComPtr<HttpFormUrlEncodedContent>> { unsafe {
        <Self as RtActivatable<IHttpFormUrlEncodedContentFactory>>::get_activation_factory().create(content)
    }}
}
DEFINE_CLSID!(HttpFormUrlEncodedContent(&[87,105,110,100,111,119,115,46,87,101,98,46,72,116,116,112,46,72,116,116,112,70,111,114,109,85,114,108,69,110,99,111,100,101,100,67,111,110,116,101,110,116,0]) [CLSID_HttpFormUrlEncodedContent]);
DEFINE_IID!(IID_IHttpFormUrlEncodedContentFactory, 1139807116, 12147, 17154, 181, 243, 234, 233, 35, 138, 94, 1);
RT_INTERFACE!{static interface IHttpFormUrlEncodedContentFactory(IHttpFormUrlEncodedContentFactoryVtbl): IInspectable(IInspectableVtbl) [IID_IHttpFormUrlEncodedContentFactory] {
    fn Create(&self, content: *mut super::super::foundation::collections::IIterable<super::super::foundation::collections::IKeyValuePair<HString, HString>>, out: *mut *mut HttpFormUrlEncodedContent) -> HRESULT
}}
impl IHttpFormUrlEncodedContentFactory {
    #[inline] pub unsafe fn create(&self, content: &super::super::foundation::collections::IIterable<super::super::foundation::collections::IKeyValuePair<HString, HString>>) -> Result<ComPtr<HttpFormUrlEncodedContent>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).Create)(self as *const _ as *mut _, content as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
DEFINE_IID!(IID_IHttpMethod, 1921859618, 28685, 20448, 175, 165, 64, 41, 156, 88, 219, 253);
RT_INTERFACE!{interface IHttpMethod(IHttpMethodVtbl): IInspectable(IInspectableVtbl) [IID_IHttpMethod] {
    fn get_Method(&self, out: *mut HSTRING) -> HRESULT
}}
impl IHttpMethod {
    #[inline] pub unsafe fn get_method(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Method)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
}
RT_CLASS!{class HttpMethod: IHttpMethod}
impl RtActivatable<IHttpMethodFactory> for HttpMethod {}
impl RtActivatable<IHttpMethodStatics> for HttpMethod {}
impl HttpMethod {
    #[inline] pub fn create(method: &HStringArg) -> Result<ComPtr<HttpMethod>> { unsafe {
        <Self as RtActivatable<IHttpMethodFactory>>::get_activation_factory().create(method)
    }}
    #[inline] pub fn get_delete() -> Result<ComPtr<HttpMethod>> { unsafe {
        <Self as RtActivatable<IHttpMethodStatics>>::get_activation_factory().get_delete()
    }}
    #[inline] pub fn get_get() -> Result<ComPtr<HttpMethod>> { unsafe {
        <Self as RtActivatable<IHttpMethodStatics>>::get_activation_factory().get_get()
    }}
    #[inline] pub fn get_head() -> Result<ComPtr<HttpMethod>> { unsafe {
        <Self as RtActivatable<IHttpMethodStatics>>::get_activation_factory().get_head()
    }}
    #[inline] pub fn get_options() -> Result<ComPtr<HttpMethod>> { unsafe {
        <Self as RtActivatable<IHttpMethodStatics>>::get_activation_factory().get_options()
    }}
    #[inline] pub fn get_patch() -> Result<ComPtr<HttpMethod>> { unsafe {
        <Self as RtActivatable<IHttpMethodStatics>>::get_activation_factory().get_patch()
    }}
    #[inline] pub fn get_post() -> Result<ComPtr<HttpMethod>> { unsafe {
        <Self as RtActivatable<IHttpMethodStatics>>::get_activation_factory().get_post()
    }}
    #[inline] pub fn get_put() -> Result<ComPtr<HttpMethod>> { unsafe {
        <Self as RtActivatable<IHttpMethodStatics>>::get_activation_factory().get_put()
    }}
}
DEFINE_CLSID!(HttpMethod(&[87,105,110,100,111,119,115,46,87,101,98,46,72,116,116,112,46,72,116,116,112,77,101,116,104,111,100,0]) [CLSID_HttpMethod]);
DEFINE_IID!(IID_IHttpMethodFactory, 1011994893, 14039, 16632, 168, 109, 231, 89, 202, 242, 248, 63);
RT_INTERFACE!{static interface IHttpMethodFactory(IHttpMethodFactoryVtbl): IInspectable(IInspectableVtbl) [IID_IHttpMethodFactory] {
    fn Create(&self, method: HSTRING, out: *mut *mut HttpMethod) -> HRESULT
}}
impl IHttpMethodFactory {
    #[inline] pub unsafe fn create(&self, method: &HStringArg) -> Result<ComPtr<HttpMethod>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).Create)(self as *const _ as *mut _, method.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
DEFINE_IID!(IID_IHttpMethodStatics, 1691447792, 55706, 16723, 141, 198, 214, 140, 196, 204, 227, 23);
RT_INTERFACE!{static interface IHttpMethodStatics(IHttpMethodStaticsVtbl): IInspectable(IInspectableVtbl) [IID_IHttpMethodStatics] {
    fn get_Delete(&self, out: *mut *mut HttpMethod) -> HRESULT,
    fn get_Get(&self, out: *mut *mut HttpMethod) -> HRESULT,
    fn get_Head(&self, out: *mut *mut HttpMethod) -> HRESULT,
    fn get_Options(&self, out: *mut *mut HttpMethod) -> HRESULT,
    fn get_Patch(&self, out: *mut *mut HttpMethod) -> HRESULT,
    fn get_Post(&self, out: *mut *mut HttpMethod) -> HRESULT,
    fn get_Put(&self, out: *mut *mut HttpMethod) -> HRESULT
}}
impl IHttpMethodStatics {
    #[inline] pub unsafe fn get_delete(&self) -> Result<ComPtr<HttpMethod>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Delete)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_get(&self) -> Result<ComPtr<HttpMethod>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Get)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_head(&self) -> Result<ComPtr<HttpMethod>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Head)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_options(&self) -> Result<ComPtr<HttpMethod>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Options)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_patch(&self) -> Result<ComPtr<HttpMethod>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Patch)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_post(&self) -> Result<ComPtr<HttpMethod>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Post)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_put(&self) -> Result<ComPtr<HttpMethod>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Put)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
DEFINE_IID!(IID_IHttpMultipartContent, 3750849279, 39206, 19145, 170, 241, 224, 208, 78, 240, 155, 185);
RT_INTERFACE!{interface IHttpMultipartContent(IHttpMultipartContentVtbl): IInspectable(IInspectableVtbl) [IID_IHttpMultipartContent] {
    fn Add(&self, content: *mut IHttpContent) -> HRESULT
}}
impl IHttpMultipartContent {
    #[inline] pub unsafe fn add(&self, content: &IHttpContent) -> Result<()> {
        let hr = ((*self.lpVtbl).Add)(self as *const _ as *mut _, content as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
}
RT_CLASS!{class HttpMultipartContent: IHttpContent}
impl RtActivatable<IHttpMultipartContentFactory> for HttpMultipartContent {}
impl RtActivatable<IActivationFactory> for HttpMultipartContent {}
impl HttpMultipartContent {
    #[inline] pub fn create_with_subtype(subtype: &HStringArg) -> Result<ComPtr<HttpMultipartContent>> { unsafe {
        <Self as RtActivatable<IHttpMultipartContentFactory>>::get_activation_factory().create_with_subtype(subtype)
    }}
    #[inline] pub fn create_with_subtype_and_boundary(subtype: &HStringArg, boundary: &HStringArg) -> Result<ComPtr<HttpMultipartContent>> { unsafe {
        <Self as RtActivatable<IHttpMultipartContentFactory>>::get_activation_factory().create_with_subtype_and_boundary(subtype, boundary)
    }}
}
DEFINE_CLSID!(HttpMultipartContent(&[87,105,110,100,111,119,115,46,87,101,98,46,72,116,116,112,46,72,116,116,112,77,117,108,116,105,112,97,114,116,67,111,110,116,101,110,116,0]) [CLSID_HttpMultipartContent]);
DEFINE_IID!(IID_IHttpMultipartContentFactory, 2125737570, 546, 20256, 179, 114, 71, 213, 219, 93, 51, 180);
RT_INTERFACE!{static interface IHttpMultipartContentFactory(IHttpMultipartContentFactoryVtbl): IInspectable(IInspectableVtbl) [IID_IHttpMultipartContentFactory] {
    fn CreateWithSubtype(&self, subtype: HSTRING, out: *mut *mut HttpMultipartContent) -> HRESULT,
    fn CreateWithSubtypeAndBoundary(&self, subtype: HSTRING, boundary: HSTRING, out: *mut *mut HttpMultipartContent) -> HRESULT
}}
impl IHttpMultipartContentFactory {
    #[inline] pub unsafe fn create_with_subtype(&self, subtype: &HStringArg) -> Result<ComPtr<HttpMultipartContent>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).CreateWithSubtype)(self as *const _ as *mut _, subtype.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn create_with_subtype_and_boundary(&self, subtype: &HStringArg, boundary: &HStringArg) -> Result<ComPtr<HttpMultipartContent>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).CreateWithSubtypeAndBoundary)(self as *const _ as *mut _, subtype.get(), boundary.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
DEFINE_IID!(IID_IHttpMultipartFormDataContent, 1691564002, 59751, 17956, 182, 209, 207, 116, 96, 74, 74, 66);
RT_INTERFACE!{interface IHttpMultipartFormDataContent(IHttpMultipartFormDataContentVtbl): IInspectable(IInspectableVtbl) [IID_IHttpMultipartFormDataContent] {
    fn Add(&self, content: *mut IHttpContent) -> HRESULT,
    fn AddWithName(&self, content: *mut IHttpContent, name: HSTRING) -> HRESULT,
    fn AddWithNameAndFileName(&self, content: *mut IHttpContent, name: HSTRING, fileName: HSTRING) -> HRESULT
}}
impl IHttpMultipartFormDataContent {
    #[inline] pub unsafe fn add(&self, content: &IHttpContent) -> Result<()> {
        let hr = ((*self.lpVtbl).Add)(self as *const _ as *mut _, content as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn add_with_name(&self, content: &IHttpContent, name: &HStringArg) -> Result<()> {
        let hr = ((*self.lpVtbl).AddWithName)(self as *const _ as *mut _, content as *const _ as *mut _, name.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn add_with_name_and_file_name(&self, content: &IHttpContent, name: &HStringArg, fileName: &HStringArg) -> Result<()> {
        let hr = ((*self.lpVtbl).AddWithNameAndFileName)(self as *const _ as *mut _, content as *const _ as *mut _, name.get(), fileName.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
}
RT_CLASS!{class HttpMultipartFormDataContent: IHttpContent}
impl RtActivatable<IHttpMultipartFormDataContentFactory> for HttpMultipartFormDataContent {}
impl RtActivatable<IActivationFactory> for HttpMultipartFormDataContent {}
impl HttpMultipartFormDataContent {
    #[inline] pub fn create_with_boundary(boundary: &HStringArg) -> Result<ComPtr<HttpMultipartFormDataContent>> { unsafe {
        <Self as RtActivatable<IHttpMultipartFormDataContentFactory>>::get_activation_factory().create_with_boundary(boundary)
    }}
}
DEFINE_CLSID!(HttpMultipartFormDataContent(&[87,105,110,100,111,119,115,46,87,101,98,46,72,116,116,112,46,72,116,116,112,77,117,108,116,105,112,97,114,116,70,111,114,109,68,97,116,97,67,111,110,116,101,110,116,0]) [CLSID_HttpMultipartFormDataContent]);
DEFINE_IID!(IID_IHttpMultipartFormDataContentFactory, 2689430289, 20503, 17954, 147, 168, 73, 178, 74, 79, 203, 252);
RT_INTERFACE!{static interface IHttpMultipartFormDataContentFactory(IHttpMultipartFormDataContentFactoryVtbl): IInspectable(IInspectableVtbl) [IID_IHttpMultipartFormDataContentFactory] {
    fn CreateWithBoundary(&self, boundary: HSTRING, out: *mut *mut HttpMultipartFormDataContent) -> HRESULT
}}
impl IHttpMultipartFormDataContentFactory {
    #[inline] pub unsafe fn create_with_boundary(&self, boundary: &HStringArg) -> Result<ComPtr<HttpMultipartFormDataContent>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).CreateWithBoundary)(self as *const _ as *mut _, boundary.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
RT_STRUCT! { struct HttpProgress {
    Stage: HttpProgressStage, BytesSent: u64, TotalBytesToSend: *mut super::super::foundation::IReference<u64>, BytesReceived: u64, TotalBytesToReceive: *mut super::super::foundation::IReference<u64>, Retries: u32,
}}
RT_ENUM! { enum HttpProgressStage: i32 {
    None (HttpProgressStage_None) = 0, DetectingProxy (HttpProgressStage_DetectingProxy) = 10, ResolvingName (HttpProgressStage_ResolvingName) = 20, ConnectingToServer (HttpProgressStage_ConnectingToServer) = 30, NegotiatingSsl (HttpProgressStage_NegotiatingSsl) = 40, SendingHeaders (HttpProgressStage_SendingHeaders) = 50, SendingContent (HttpProgressStage_SendingContent) = 60, WaitingForResponse (HttpProgressStage_WaitingForResponse) = 70, ReceivingHeaders (HttpProgressStage_ReceivingHeaders) = 80, ReceivingContent (HttpProgressStage_ReceivingContent) = 90,
}}
DEFINE_IID!(IID_IHttpRequestMessage, 4118162236, 29908, 18449, 181, 220, 159, 139, 78, 47, 154, 191);
RT_INTERFACE!{interface IHttpRequestMessage(IHttpRequestMessageVtbl): IInspectable(IInspectableVtbl) [IID_IHttpRequestMessage] {
    fn get_Content(&self, out: *mut *mut IHttpContent) -> HRESULT,
    fn put_Content(&self, value: *mut IHttpContent) -> HRESULT,
    fn get_Headers(&self, out: *mut *mut headers::HttpRequestHeaderCollection) -> HRESULT,
    fn get_Method(&self, out: *mut *mut HttpMethod) -> HRESULT,
    fn put_Method(&self, value: *mut HttpMethod) -> HRESULT,
    fn get_Properties(&self, out: *mut *mut super::super::foundation::collections::IMap<HString, IInspectable>) -> HRESULT,
    fn get_RequestUri(&self, out: *mut *mut super::super::foundation::Uri) -> HRESULT,
    fn put_RequestUri(&self, value: *mut super::super::foundation::Uri) -> HRESULT,
    fn get_TransportInformation(&self, out: *mut *mut HttpTransportInformation) -> HRESULT
}}
impl IHttpRequestMessage {
    #[inline] pub unsafe fn get_content(&self) -> Result<ComPtr<IHttpContent>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Content)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_content(&self, value: &IHttpContent) -> Result<()> {
        let hr = ((*self.lpVtbl).put_Content)(self as *const _ as *mut _, value as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_headers(&self) -> Result<ComPtr<headers::HttpRequestHeaderCollection>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Headers)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_method(&self) -> Result<ComPtr<HttpMethod>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Method)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_method(&self, value: &HttpMethod) -> Result<()> {
        let hr = ((*self.lpVtbl).put_Method)(self as *const _ as *mut _, value as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_properties(&self) -> Result<ComPtr<super::super::foundation::collections::IMap<HString, IInspectable>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Properties)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_request_uri(&self) -> Result<ComPtr<super::super::foundation::Uri>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_RequestUri)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_request_uri(&self, value: &super::super::foundation::Uri) -> Result<()> {
        let hr = ((*self.lpVtbl).put_RequestUri)(self as *const _ as *mut _, value as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_transport_information(&self) -> Result<ComPtr<HttpTransportInformation>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_TransportInformation)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
RT_CLASS!{class HttpRequestMessage: IHttpRequestMessage}
impl RtActivatable<IHttpRequestMessageFactory> for HttpRequestMessage {}
impl RtActivatable<IActivationFactory> for HttpRequestMessage {}
impl HttpRequestMessage {
    #[inline] pub fn create(method: &HttpMethod, uri: &super::super::foundation::Uri) -> Result<ComPtr<HttpRequestMessage>> { unsafe {
        <Self as RtActivatable<IHttpRequestMessageFactory>>::get_activation_factory().create(method, uri)
    }}
}
DEFINE_CLSID!(HttpRequestMessage(&[87,105,110,100,111,119,115,46,87,101,98,46,72,116,116,112,46,72,116,116,112,82,101,113,117,101,115,116,77,101,115,115,97,103,101,0]) [CLSID_HttpRequestMessage]);
DEFINE_IID!(IID_IHttpRequestMessageFactory, 1538038094, 14470, 16686, 174, 195, 82, 236, 127, 37, 97, 111);
RT_INTERFACE!{static interface IHttpRequestMessageFactory(IHttpRequestMessageFactoryVtbl): IInspectable(IInspectableVtbl) [IID_IHttpRequestMessageFactory] {
    fn Create(&self, method: *mut HttpMethod, uri: *mut super::super::foundation::Uri, out: *mut *mut HttpRequestMessage) -> HRESULT
}}
impl IHttpRequestMessageFactory {
    #[inline] pub unsafe fn create(&self, method: &HttpMethod, uri: &super::super::foundation::Uri) -> Result<ComPtr<HttpRequestMessage>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).Create)(self as *const _ as *mut _, method as *const _ as *mut _, uri as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
DEFINE_IID!(IID_IHttpResponseMessage, 4276224251, 34404, 17632, 149, 217, 66, 105, 97, 153, 191, 252);
RT_INTERFACE!{interface IHttpResponseMessage(IHttpResponseMessageVtbl): IInspectable(IInspectableVtbl) [IID_IHttpResponseMessage] {
    fn get_Content(&self, out: *mut *mut IHttpContent) -> HRESULT,
    fn put_Content(&self, value: *mut IHttpContent) -> HRESULT,
    fn get_Headers(&self, out: *mut *mut headers::HttpResponseHeaderCollection) -> HRESULT,
    fn get_IsSuccessStatusCode(&self, out: *mut bool) -> HRESULT,
    fn get_ReasonPhrase(&self, out: *mut HSTRING) -> HRESULT,
    fn put_ReasonPhrase(&self, value: HSTRING) -> HRESULT,
    fn get_RequestMessage(&self, out: *mut *mut HttpRequestMessage) -> HRESULT,
    fn put_RequestMessage(&self, value: *mut HttpRequestMessage) -> HRESULT,
    fn get_Source(&self, out: *mut HttpResponseMessageSource) -> HRESULT,
    fn put_Source(&self, value: HttpResponseMessageSource) -> HRESULT,
    fn get_StatusCode(&self, out: *mut HttpStatusCode) -> HRESULT,
    fn put_StatusCode(&self, value: HttpStatusCode) -> HRESULT,
    fn get_Version(&self, out: *mut HttpVersion) -> HRESULT,
    fn put_Version(&self, value: HttpVersion) -> HRESULT,
    fn EnsureSuccessStatusCode(&self, out: *mut *mut HttpResponseMessage) -> HRESULT
}}
impl IHttpResponseMessage {
    #[inline] pub unsafe fn get_content(&self) -> Result<ComPtr<IHttpContent>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Content)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_content(&self, value: &IHttpContent) -> Result<()> {
        let hr = ((*self.lpVtbl).put_Content)(self as *const _ as *mut _, value as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_headers(&self) -> Result<ComPtr<headers::HttpResponseHeaderCollection>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Headers)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_is_success_status_code(&self) -> Result<bool> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_IsSuccessStatusCode)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_reason_phrase(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_ReasonPhrase)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_reason_phrase(&self, value: &HStringArg) -> Result<()> {
        let hr = ((*self.lpVtbl).put_ReasonPhrase)(self as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_request_message(&self) -> Result<ComPtr<HttpRequestMessage>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_RequestMessage)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_request_message(&self, value: &HttpRequestMessage) -> Result<()> {
        let hr = ((*self.lpVtbl).put_RequestMessage)(self as *const _ as *mut _, value as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_source(&self) -> Result<HttpResponseMessageSource> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_Source)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_source(&self, value: HttpResponseMessageSource) -> Result<()> {
        let hr = ((*self.lpVtbl).put_Source)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_status_code(&self) -> Result<HttpStatusCode> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_StatusCode)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_status_code(&self, value: HttpStatusCode) -> Result<()> {
        let hr = ((*self.lpVtbl).put_StatusCode)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_version(&self) -> Result<HttpVersion> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_Version)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_version(&self, value: HttpVersion) -> Result<()> {
        let hr = ((*self.lpVtbl).put_Version)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn ensure_success_status_code(&self) -> Result<ComPtr<HttpResponseMessage>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).EnsureSuccessStatusCode)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
RT_CLASS!{class HttpResponseMessage: IHttpResponseMessage}
impl RtActivatable<IHttpResponseMessageFactory> for HttpResponseMessage {}
impl RtActivatable<IActivationFactory> for HttpResponseMessage {}
impl HttpResponseMessage {
    #[inline] pub fn create(statusCode: HttpStatusCode) -> Result<ComPtr<HttpResponseMessage>> { unsafe {
        <Self as RtActivatable<IHttpResponseMessageFactory>>::get_activation_factory().create(statusCode)
    }}
}
DEFINE_CLSID!(HttpResponseMessage(&[87,105,110,100,111,119,115,46,87,101,98,46,72,116,116,112,46,72,116,116,112,82,101,115,112,111,110,115,101,77,101,115,115,97,103,101,0]) [CLSID_HttpResponseMessage]);
DEFINE_IID!(IID_IHttpResponseMessageFactory, 1386786713, 61589, 17370, 182, 15, 124, 252, 43, 199, 234, 47);
RT_INTERFACE!{static interface IHttpResponseMessageFactory(IHttpResponseMessageFactoryVtbl): IInspectable(IInspectableVtbl) [IID_IHttpResponseMessageFactory] {
    fn Create(&self, statusCode: HttpStatusCode, out: *mut *mut HttpResponseMessage) -> HRESULT
}}
impl IHttpResponseMessageFactory {
    #[inline] pub unsafe fn create(&self, statusCode: HttpStatusCode) -> Result<ComPtr<HttpResponseMessage>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).Create)(self as *const _ as *mut _, statusCode, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
RT_ENUM! { enum HttpResponseMessageSource: i32 {
    None (HttpResponseMessageSource_None) = 0, Cache (HttpResponseMessageSource_Cache) = 1, Network (HttpResponseMessageSource_Network) = 2,
}}
RT_ENUM! { enum HttpStatusCode: i32 {
    None (HttpStatusCode_None) = 0, Continue (HttpStatusCode_Continue) = 100, SwitchingProtocols (HttpStatusCode_SwitchingProtocols) = 101, Processing (HttpStatusCode_Processing) = 102, Ok (HttpStatusCode_Ok) = 200, Created (HttpStatusCode_Created) = 201, Accepted (HttpStatusCode_Accepted) = 202, NonAuthoritativeInformation (HttpStatusCode_NonAuthoritativeInformation) = 203, NoContent (HttpStatusCode_NoContent) = 204, ResetContent (HttpStatusCode_ResetContent) = 205, PartialContent (HttpStatusCode_PartialContent) = 206, MultiStatus (HttpStatusCode_MultiStatus) = 207, AlreadyReported (HttpStatusCode_AlreadyReported) = 208, IMUsed (HttpStatusCode_IMUsed) = 226, MultipleChoices (HttpStatusCode_MultipleChoices) = 300, MovedPermanently (HttpStatusCode_MovedPermanently) = 301, Found (HttpStatusCode_Found) = 302, SeeOther (HttpStatusCode_SeeOther) = 303, NotModified (HttpStatusCode_NotModified) = 304, UseProxy (HttpStatusCode_UseProxy) = 305, TemporaryRedirect (HttpStatusCode_TemporaryRedirect) = 307, PermanentRedirect (HttpStatusCode_PermanentRedirect) = 308, BadRequest (HttpStatusCode_BadRequest) = 400, Unauthorized (HttpStatusCode_Unauthorized) = 401, PaymentRequired (HttpStatusCode_PaymentRequired) = 402, Forbidden (HttpStatusCode_Forbidden) = 403, NotFound (HttpStatusCode_NotFound) = 404, MethodNotAllowed (HttpStatusCode_MethodNotAllowed) = 405, NotAcceptable (HttpStatusCode_NotAcceptable) = 406, ProxyAuthenticationRequired (HttpStatusCode_ProxyAuthenticationRequired) = 407, RequestTimeout (HttpStatusCode_RequestTimeout) = 408, Conflict (HttpStatusCode_Conflict) = 409, Gone (HttpStatusCode_Gone) = 410, LengthRequired (HttpStatusCode_LengthRequired) = 411, PreconditionFailed (HttpStatusCode_PreconditionFailed) = 412, RequestEntityTooLarge (HttpStatusCode_RequestEntityTooLarge) = 413, RequestUriTooLong (HttpStatusCode_RequestUriTooLong) = 414, UnsupportedMediaType (HttpStatusCode_UnsupportedMediaType) = 415, RequestedRangeNotSatisfiable (HttpStatusCode_RequestedRangeNotSatisfiable) = 416, ExpectationFailed (HttpStatusCode_ExpectationFailed) = 417, UnprocessableEntity (HttpStatusCode_UnprocessableEntity) = 422, Locked (HttpStatusCode_Locked) = 423, FailedDependency (HttpStatusCode_FailedDependency) = 424, UpgradeRequired (HttpStatusCode_UpgradeRequired) = 426, PreconditionRequired (HttpStatusCode_PreconditionRequired) = 428, TooManyRequests (HttpStatusCode_TooManyRequests) = 429, RequestHeaderFieldsTooLarge (HttpStatusCode_RequestHeaderFieldsTooLarge) = 431, InternalServerError (HttpStatusCode_InternalServerError) = 500, NotImplemented (HttpStatusCode_NotImplemented) = 501, BadGateway (HttpStatusCode_BadGateway) = 502, ServiceUnavailable (HttpStatusCode_ServiceUnavailable) = 503, GatewayTimeout (HttpStatusCode_GatewayTimeout) = 504, HttpVersionNotSupported (HttpStatusCode_HttpVersionNotSupported) = 505, VariantAlsoNegotiates (HttpStatusCode_VariantAlsoNegotiates) = 506, InsufficientStorage (HttpStatusCode_InsufficientStorage) = 507, LoopDetected (HttpStatusCode_LoopDetected) = 508, NotExtended (HttpStatusCode_NotExtended) = 510, NetworkAuthenticationRequired (HttpStatusCode_NetworkAuthenticationRequired) = 511,
}}
RT_CLASS!{class HttpStreamContent: IHttpContent}
impl RtActivatable<IHttpStreamContentFactory> for HttpStreamContent {}
impl HttpStreamContent {
    #[cfg(feature="windows-storage")] #[inline] pub fn create_from_input_stream(content: &super::super::storage::streams::IInputStream) -> Result<ComPtr<HttpStreamContent>> { unsafe {
        <Self as RtActivatable<IHttpStreamContentFactory>>::get_activation_factory().create_from_input_stream(content)
    }}
}
DEFINE_CLSID!(HttpStreamContent(&[87,105,110,100,111,119,115,46,87,101,98,46,72,116,116,112,46,72,116,116,112,83,116,114,101,97,109,67,111,110,116,101,110,116,0]) [CLSID_HttpStreamContent]);
DEFINE_IID!(IID_IHttpStreamContentFactory, 4091956637, 63269, 16510, 148, 47, 14, 218, 24, 152, 9, 244);
RT_INTERFACE!{static interface IHttpStreamContentFactory(IHttpStreamContentFactoryVtbl): IInspectable(IInspectableVtbl) [IID_IHttpStreamContentFactory] {
    #[cfg(feature="windows-storage")] fn CreateFromInputStream(&self, content: *mut super::super::storage::streams::IInputStream, out: *mut *mut HttpStreamContent) -> HRESULT
}}
impl IHttpStreamContentFactory {
    #[cfg(feature="windows-storage")] #[inline] pub unsafe fn create_from_input_stream(&self, content: &super::super::storage::streams::IInputStream) -> Result<ComPtr<HttpStreamContent>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).CreateFromInputStream)(self as *const _ as *mut _, content as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
RT_CLASS!{class HttpStringContent: IHttpContent}
impl RtActivatable<IHttpStringContentFactory> for HttpStringContent {}
impl HttpStringContent {
    #[inline] pub fn create_from_string(content: &HStringArg) -> Result<ComPtr<HttpStringContent>> { unsafe {
        <Self as RtActivatable<IHttpStringContentFactory>>::get_activation_factory().create_from_string(content)
    }}
    #[cfg(feature="windows-storage")] #[inline] pub fn create_from_string_with_encoding(content: &HStringArg, encoding: super::super::storage::streams::UnicodeEncoding) -> Result<ComPtr<HttpStringContent>> { unsafe {
        <Self as RtActivatable<IHttpStringContentFactory>>::get_activation_factory().create_from_string_with_encoding(content, encoding)
    }}
    #[cfg(feature="windows-storage")] #[inline] pub fn create_from_string_with_encoding_and_media_type(content: &HStringArg, encoding: super::super::storage::streams::UnicodeEncoding, mediaType: &HStringArg) -> Result<ComPtr<HttpStringContent>> { unsafe {
        <Self as RtActivatable<IHttpStringContentFactory>>::get_activation_factory().create_from_string_with_encoding_and_media_type(content, encoding, mediaType)
    }}
}
DEFINE_CLSID!(HttpStringContent(&[87,105,110,100,111,119,115,46,87,101,98,46,72,116,116,112,46,72,116,116,112,83,116,114,105,110,103,67,111,110,116,101,110,116,0]) [CLSID_HttpStringContent]);
DEFINE_IID!(IID_IHttpStringContentFactory, 1180999003, 11923, 18667, 142, 97, 25, 103, 120, 120, 229, 127);
RT_INTERFACE!{static interface IHttpStringContentFactory(IHttpStringContentFactoryVtbl): IInspectable(IInspectableVtbl) [IID_IHttpStringContentFactory] {
    fn CreateFromString(&self, content: HSTRING, out: *mut *mut HttpStringContent) -> HRESULT,
    #[cfg(feature="windows-storage")] fn CreateFromStringWithEncoding(&self, content: HSTRING, encoding: super::super::storage::streams::UnicodeEncoding, out: *mut *mut HttpStringContent) -> HRESULT,
    #[cfg(feature="windows-storage")] fn CreateFromStringWithEncodingAndMediaType(&self, content: HSTRING, encoding: super::super::storage::streams::UnicodeEncoding, mediaType: HSTRING, out: *mut *mut HttpStringContent) -> HRESULT
}}
impl IHttpStringContentFactory {
    #[inline] pub unsafe fn create_from_string(&self, content: &HStringArg) -> Result<ComPtr<HttpStringContent>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).CreateFromString)(self as *const _ as *mut _, content.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[cfg(feature="windows-storage")] #[inline] pub unsafe fn create_from_string_with_encoding(&self, content: &HStringArg, encoding: super::super::storage::streams::UnicodeEncoding) -> Result<ComPtr<HttpStringContent>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).CreateFromStringWithEncoding)(self as *const _ as *mut _, content.get(), encoding, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[cfg(feature="windows-storage")] #[inline] pub unsafe fn create_from_string_with_encoding_and_media_type(&self, content: &HStringArg, encoding: super::super::storage::streams::UnicodeEncoding, mediaType: &HStringArg) -> Result<ComPtr<HttpStringContent>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).CreateFromStringWithEncodingAndMediaType)(self as *const _ as *mut _, content.get(), encoding, mediaType.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
DEFINE_IID!(IID_IHttpTransportInformation, 1880256920, 50855, 20176, 131, 58, 131, 253, 139, 143, 23, 141);
RT_INTERFACE!{interface IHttpTransportInformation(IHttpTransportInformationVtbl): IInspectable(IInspectableVtbl) [IID_IHttpTransportInformation] {
    #[cfg(feature="windows-security")] fn get_ServerCertificate(&self, out: *mut *mut super::super::security::cryptography::certificates::Certificate) -> HRESULT,
    #[cfg(not(feature="windows-networking"))] fn __Dummy1(&self) -> (),
    #[cfg(feature="windows-networking")] fn get_ServerCertificateErrorSeverity(&self, out: *mut super::super::networking::sockets::SocketSslErrorSeverity) -> HRESULT,
    #[cfg(feature="windows-security")] fn get_ServerCertificateErrors(&self, out: *mut *mut super::super::foundation::collections::IVectorView<super::super::security::cryptography::certificates::ChainValidationResult>) -> HRESULT,
    #[cfg(feature="windows-security")] fn get_ServerIntermediateCertificates(&self, out: *mut *mut super::super::foundation::collections::IVectorView<super::super::security::cryptography::certificates::Certificate>) -> HRESULT
}}
impl IHttpTransportInformation {
    #[cfg(feature="windows-security")] #[inline] pub unsafe fn get_server_certificate(&self) -> Result<ComPtr<super::super::security::cryptography::certificates::Certificate>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_ServerCertificate)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[cfg(feature="windows-networking")] #[inline] pub unsafe fn get_server_certificate_error_severity(&self) -> Result<super::super::networking::sockets::SocketSslErrorSeverity> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_ServerCertificateErrorSeverity)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[cfg(feature="windows-security")] #[inline] pub unsafe fn get_server_certificate_errors(&self) -> Result<ComPtr<super::super::foundation::collections::IVectorView<super::super::security::cryptography::certificates::ChainValidationResult>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_ServerCertificateErrors)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[cfg(feature="windows-security")] #[inline] pub unsafe fn get_server_intermediate_certificates(&self) -> Result<ComPtr<super::super::foundation::collections::IVectorView<super::super::security::cryptography::certificates::Certificate>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_ServerIntermediateCertificates)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
RT_CLASS!{class HttpTransportInformation: IHttpTransportInformation}
RT_ENUM! { enum HttpVersion: i32 {
    None (HttpVersion_None) = 0, Http10 (HttpVersion_Http10) = 1, Http11 (HttpVersion_Http11) = 2, Http20 (HttpVersion_Http20) = 3,
}}
pub mod filters { // Windows.Web.Http.Filters
use ::prelude::*;
DEFINE_IID!(IID_IHttpBaseProtocolFilter, 1908972297, 57649, 19284, 165, 60, 235, 67, 255, 55, 233, 187);
RT_INTERFACE!{interface IHttpBaseProtocolFilter(IHttpBaseProtocolFilterVtbl): IInspectable(IInspectableVtbl) [IID_IHttpBaseProtocolFilter] {
    fn get_AllowAutoRedirect(&self, out: *mut bool) -> HRESULT,
    fn put_AllowAutoRedirect(&self, value: bool) -> HRESULT,
    fn get_AllowUI(&self, out: *mut bool) -> HRESULT,
    fn put_AllowUI(&self, value: bool) -> HRESULT,
    fn get_AutomaticDecompression(&self, out: *mut bool) -> HRESULT,
    fn put_AutomaticDecompression(&self, value: bool) -> HRESULT,
    fn get_CacheControl(&self, out: *mut *mut HttpCacheControl) -> HRESULT,
    fn get_CookieManager(&self, out: *mut *mut super::HttpCookieManager) -> HRESULT,
    #[cfg(not(feature="windows-security"))] fn __Dummy8(&self) -> (),
    #[cfg(feature="windows-security")] fn get_ClientCertificate(&self, out: *mut *mut ::rt::gen::windows::security::cryptography::certificates::Certificate) -> HRESULT,
    #[cfg(not(feature="windows-security"))] fn __Dummy9(&self) -> (),
    #[cfg(feature="windows-security")] fn put_ClientCertificate(&self, value: *mut ::rt::gen::windows::security::cryptography::certificates::Certificate) -> HRESULT,
    #[cfg(not(feature="windows-security"))] fn __Dummy10(&self) -> (),
    #[cfg(feature="windows-security")] fn get_IgnorableServerCertificateErrors(&self, out: *mut *mut ::rt::gen::windows::foundation::collections::IVector<::rt::gen::windows::security::cryptography::certificates::ChainValidationResult>) -> HRESULT,
    fn get_MaxConnectionsPerServer(&self, out: *mut u32) -> HRESULT,
    fn put_MaxConnectionsPerServer(&self, value: u32) -> HRESULT,
    #[cfg(not(feature="windows-security"))] fn __Dummy13(&self) -> (),
    #[cfg(feature="windows-security")] fn get_ProxyCredential(&self, out: *mut *mut ::rt::gen::windows::security::credentials::PasswordCredential) -> HRESULT,
    #[cfg(not(feature="windows-security"))] fn __Dummy14(&self) -> (),
    #[cfg(feature="windows-security")] fn put_ProxyCredential(&self, value: *mut ::rt::gen::windows::security::credentials::PasswordCredential) -> HRESULT,
    #[cfg(not(feature="windows-security"))] fn __Dummy15(&self) -> (),
    #[cfg(feature="windows-security")] fn get_ServerCredential(&self, out: *mut *mut ::rt::gen::windows::security::credentials::PasswordCredential) -> HRESULT,
    #[cfg(not(feature="windows-security"))] fn __Dummy16(&self) -> (),
    #[cfg(feature="windows-security")] fn put_ServerCredential(&self, value: *mut ::rt::gen::windows::security::credentials::PasswordCredential) -> HRESULT,
    fn get_UseProxy(&self, out: *mut bool) -> HRESULT,
    fn put_UseProxy(&self, value: bool) -> HRESULT
}}
impl IHttpBaseProtocolFilter {
    #[inline] pub unsafe fn get_allow_auto_redirect(&self) -> Result<bool> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_AllowAutoRedirect)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_allow_auto_redirect(&self, value: bool) -> Result<()> {
        let hr = ((*self.lpVtbl).put_AllowAutoRedirect)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_allow_ui(&self) -> Result<bool> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_AllowUI)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_allow_ui(&self, value: bool) -> Result<()> {
        let hr = ((*self.lpVtbl).put_AllowUI)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_automatic_decompression(&self) -> Result<bool> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_AutomaticDecompression)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_automatic_decompression(&self, value: bool) -> Result<()> {
        let hr = ((*self.lpVtbl).put_AutomaticDecompression)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_cache_control(&self) -> Result<ComPtr<HttpCacheControl>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_CacheControl)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_cookie_manager(&self) -> Result<ComPtr<super::HttpCookieManager>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_CookieManager)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[cfg(feature="windows-security")] #[inline] pub unsafe fn get_client_certificate(&self) -> Result<ComPtr<::rt::gen::windows::security::cryptography::certificates::Certificate>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_ClientCertificate)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[cfg(feature="windows-security")] #[inline] pub unsafe fn set_client_certificate(&self, value: &::rt::gen::windows::security::cryptography::certificates::Certificate) -> Result<()> {
        let hr = ((*self.lpVtbl).put_ClientCertificate)(self as *const _ as *mut _, value as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[cfg(feature="windows-security")] #[inline] pub unsafe fn get_ignorable_server_certificate_errors(&self) -> Result<ComPtr<::rt::gen::windows::foundation::collections::IVector<::rt::gen::windows::security::cryptography::certificates::ChainValidationResult>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_IgnorableServerCertificateErrors)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_max_connections_per_server(&self) -> Result<u32> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_MaxConnectionsPerServer)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_max_connections_per_server(&self, value: u32) -> Result<()> {
        let hr = ((*self.lpVtbl).put_MaxConnectionsPerServer)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[cfg(feature="windows-security")] #[inline] pub unsafe fn get_proxy_credential(&self) -> Result<ComPtr<::rt::gen::windows::security::credentials::PasswordCredential>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_ProxyCredential)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[cfg(feature="windows-security")] #[inline] pub unsafe fn set_proxy_credential(&self, value: &::rt::gen::windows::security::credentials::PasswordCredential) -> Result<()> {
        let hr = ((*self.lpVtbl).put_ProxyCredential)(self as *const _ as *mut _, value as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[cfg(feature="windows-security")] #[inline] pub unsafe fn get_server_credential(&self) -> Result<ComPtr<::rt::gen::windows::security::credentials::PasswordCredential>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_ServerCredential)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[cfg(feature="windows-security")] #[inline] pub unsafe fn set_server_credential(&self, value: &::rt::gen::windows::security::credentials::PasswordCredential) -> Result<()> {
        let hr = ((*self.lpVtbl).put_ServerCredential)(self as *const _ as *mut _, value as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_use_proxy(&self) -> Result<bool> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_UseProxy)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_use_proxy(&self, value: bool) -> Result<()> {
        let hr = ((*self.lpVtbl).put_UseProxy)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
}
RT_CLASS!{class HttpBaseProtocolFilter: IHttpBaseProtocolFilter}
impl RtActivatable<IActivationFactory> for HttpBaseProtocolFilter {}
DEFINE_CLSID!(HttpBaseProtocolFilter(&[87,105,110,100,111,119,115,46,87,101,98,46,72,116,116,112,46,70,105,108,116,101,114,115,46,72,116,116,112,66,97,115,101,80,114,111,116,111,99,111,108,70,105,108,116,101,114,0]) [CLSID_HttpBaseProtocolFilter]);
DEFINE_IID!(IID_IHttpBaseProtocolFilter2, 784531475, 37927, 18688, 160, 23, 250, 125, 163, 181, 201, 174);
RT_INTERFACE!{interface IHttpBaseProtocolFilter2(IHttpBaseProtocolFilter2Vtbl): IInspectable(IInspectableVtbl) [IID_IHttpBaseProtocolFilter2] {
    fn get_MaxVersion(&self, out: *mut super::HttpVersion) -> HRESULT,
    fn put_MaxVersion(&self, value: super::HttpVersion) -> HRESULT
}}
impl IHttpBaseProtocolFilter2 {
    #[inline] pub unsafe fn get_max_version(&self) -> Result<super::HttpVersion> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_MaxVersion)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_max_version(&self, value: super::HttpVersion) -> Result<()> {
        let hr = ((*self.lpVtbl).put_MaxVersion)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
}
DEFINE_IID!(IID_IHttpBaseProtocolFilter3, 3560918348, 48450, 17326, 135, 23, 173, 44, 143, 75, 41, 55);
RT_INTERFACE!{interface IHttpBaseProtocolFilter3(IHttpBaseProtocolFilter3Vtbl): IInspectable(IInspectableVtbl) [IID_IHttpBaseProtocolFilter3] {
    fn get_CookieUsageBehavior(&self, out: *mut HttpCookieUsageBehavior) -> HRESULT,
    fn put_CookieUsageBehavior(&self, value: HttpCookieUsageBehavior) -> HRESULT
}}
impl IHttpBaseProtocolFilter3 {
    #[inline] pub unsafe fn get_cookie_usage_behavior(&self) -> Result<HttpCookieUsageBehavior> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_CookieUsageBehavior)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_cookie_usage_behavior(&self, value: HttpCookieUsageBehavior) -> Result<()> {
        let hr = ((*self.lpVtbl).put_CookieUsageBehavior)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
}
DEFINE_IID!(IID_IHttpBaseProtocolFilter4, 2682481871, 10627, 18579, 148, 31, 235, 81, 140, 168, 206, 249);
RT_INTERFACE!{interface IHttpBaseProtocolFilter4(IHttpBaseProtocolFilter4Vtbl): IInspectable(IInspectableVtbl) [IID_IHttpBaseProtocolFilter4] {
    fn add_ServerCustomValidationRequested(&self, eventHandler: *mut ::rt::gen::windows::foundation::TypedEventHandler<HttpBaseProtocolFilter, HttpServerCustomValidationRequestedEventArgs>, out: *mut ::rt::gen::windows::foundation::EventRegistrationToken) -> HRESULT,
    fn remove_ServerCustomValidationRequested(&self, eventCookie: ::rt::gen::windows::foundation::EventRegistrationToken) -> HRESULT,
    fn ClearAuthenticationCache(&self) -> HRESULT
}}
impl IHttpBaseProtocolFilter4 {
    #[inline] pub unsafe fn add_server_custom_validation_requested(&self, eventHandler: &::rt::gen::windows::foundation::TypedEventHandler<HttpBaseProtocolFilter, HttpServerCustomValidationRequestedEventArgs>) -> Result<::rt::gen::windows::foundation::EventRegistrationToken> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).add_ServerCustomValidationRequested)(self as *const _ as *mut _, eventHandler as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn remove_server_custom_validation_requested(&self, eventCookie: ::rt::gen::windows::foundation::EventRegistrationToken) -> Result<()> {
        let hr = ((*self.lpVtbl).remove_ServerCustomValidationRequested)(self as *const _ as *mut _, eventCookie);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn clear_authentication_cache(&self) -> Result<()> {
        let hr = ((*self.lpVtbl).ClearAuthenticationCache)(self as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
}
DEFINE_IID!(IID_IHttpCacheControl, 3346930868, 15594, 20149, 172, 133, 4, 225, 134, 230, 58, 183);
RT_INTERFACE!{interface IHttpCacheControl(IHttpCacheControlVtbl): IInspectable(IInspectableVtbl) [IID_IHttpCacheControl] {
    fn get_ReadBehavior(&self, out: *mut HttpCacheReadBehavior) -> HRESULT,
    fn put_ReadBehavior(&self, value: HttpCacheReadBehavior) -> HRESULT,
    fn get_WriteBehavior(&self, out: *mut HttpCacheWriteBehavior) -> HRESULT,
    fn put_WriteBehavior(&self, value: HttpCacheWriteBehavior) -> HRESULT
}}
impl IHttpCacheControl {
    #[inline] pub unsafe fn get_read_behavior(&self) -> Result<HttpCacheReadBehavior> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_ReadBehavior)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_read_behavior(&self, value: HttpCacheReadBehavior) -> Result<()> {
        let hr = ((*self.lpVtbl).put_ReadBehavior)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_write_behavior(&self) -> Result<HttpCacheWriteBehavior> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_WriteBehavior)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_write_behavior(&self, value: HttpCacheWriteBehavior) -> Result<()> {
        let hr = ((*self.lpVtbl).put_WriteBehavior)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
}
RT_CLASS!{class HttpCacheControl: IHttpCacheControl}
RT_ENUM! { enum HttpCacheReadBehavior: i32 {
    Default (HttpCacheReadBehavior_Default) = 0, MostRecent (HttpCacheReadBehavior_MostRecent) = 1, OnlyFromCache (HttpCacheReadBehavior_OnlyFromCache) = 2, NoCache (HttpCacheReadBehavior_NoCache) = 3,
}}
RT_ENUM! { enum HttpCacheWriteBehavior: i32 {
    Default (HttpCacheWriteBehavior_Default) = 0, NoCache (HttpCacheWriteBehavior_NoCache) = 1,
}}
RT_ENUM! { enum HttpCookieUsageBehavior: i32 {
    Default (HttpCookieUsageBehavior_Default) = 0, NoCookies (HttpCookieUsageBehavior_NoCookies) = 1,
}}
DEFINE_IID!(IID_IHttpFilter, 2764795349, 2306, 17310, 191, 215, 225, 37, 82, 177, 101, 206);
RT_INTERFACE!{interface IHttpFilter(IHttpFilterVtbl): IInspectable(IInspectableVtbl) [IID_IHttpFilter] {
    fn SendRequestAsync(&self, request: *mut super::HttpRequestMessage, out: *mut *mut ::rt::gen::windows::foundation::IAsyncOperationWithProgress<super::HttpResponseMessage, super::HttpProgress>) -> HRESULT
}}
impl IHttpFilter {
    #[inline] pub unsafe fn send_request_async(&self, request: &super::HttpRequestMessage) -> Result<ComPtr<::rt::gen::windows::foundation::IAsyncOperationWithProgress<super::HttpResponseMessage, super::HttpProgress>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).SendRequestAsync)(self as *const _ as *mut _, request as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
DEFINE_IID!(IID_IHttpServerCustomValidationRequestedEventArgs, 828767794, 59357, 18615, 163, 97, 147, 156, 117, 14, 99, 204);
RT_INTERFACE!{interface IHttpServerCustomValidationRequestedEventArgs(IHttpServerCustomValidationRequestedEventArgsVtbl): IInspectable(IInspectableVtbl) [IID_IHttpServerCustomValidationRequestedEventArgs] {
    fn get_RequestMessage(&self, out: *mut *mut super::HttpRequestMessage) -> HRESULT,
    #[cfg(not(feature="windows-security"))] fn __Dummy1(&self) -> (),
    #[cfg(feature="windows-security")] fn get_ServerCertificate(&self, out: *mut *mut ::rt::gen::windows::security::cryptography::certificates::Certificate) -> HRESULT,
    #[cfg(not(feature="windows-networking"))] fn __Dummy2(&self) -> (),
    #[cfg(feature="windows-networking")] fn get_ServerCertificateErrorSeverity(&self, out: *mut ::rt::gen::windows::networking::sockets::SocketSslErrorSeverity) -> HRESULT,
    #[cfg(not(feature="windows-security"))] fn __Dummy3(&self) -> (),
    #[cfg(feature="windows-security")] fn get_ServerCertificateErrors(&self, out: *mut *mut ::rt::gen::windows::foundation::collections::IVectorView<::rt::gen::windows::security::cryptography::certificates::ChainValidationResult>) -> HRESULT,
    #[cfg(not(feature="windows-security"))] fn __Dummy4(&self) -> (),
    #[cfg(feature="windows-security")] fn get_ServerIntermediateCertificates(&self, out: *mut *mut ::rt::gen::windows::foundation::collections::IVectorView<::rt::gen::windows::security::cryptography::certificates::Certificate>) -> HRESULT,
    fn Reject(&self) -> HRESULT,
    fn GetDeferral(&self, out: *mut *mut ::rt::gen::windows::foundation::Deferral) -> HRESULT
}}
impl IHttpServerCustomValidationRequestedEventArgs {
    #[inline] pub unsafe fn get_request_message(&self) -> Result<ComPtr<super::HttpRequestMessage>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_RequestMessage)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[cfg(feature="windows-security")] #[inline] pub unsafe fn get_server_certificate(&self) -> Result<ComPtr<::rt::gen::windows::security::cryptography::certificates::Certificate>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_ServerCertificate)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[cfg(feature="windows-networking")] #[inline] pub unsafe fn get_server_certificate_error_severity(&self) -> Result<::rt::gen::windows::networking::sockets::SocketSslErrorSeverity> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_ServerCertificateErrorSeverity)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[cfg(feature="windows-security")] #[inline] pub unsafe fn get_server_certificate_errors(&self) -> Result<ComPtr<::rt::gen::windows::foundation::collections::IVectorView<::rt::gen::windows::security::cryptography::certificates::ChainValidationResult>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_ServerCertificateErrors)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[cfg(feature="windows-security")] #[inline] pub unsafe fn get_server_intermediate_certificates(&self) -> Result<ComPtr<::rt::gen::windows::foundation::collections::IVectorView<::rt::gen::windows::security::cryptography::certificates::Certificate>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_ServerIntermediateCertificates)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn reject(&self) -> Result<()> {
        let hr = ((*self.lpVtbl).Reject)(self as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_deferral(&self) -> Result<ComPtr<::rt::gen::windows::foundation::Deferral>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetDeferral)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
RT_CLASS!{class HttpServerCustomValidationRequestedEventArgs: IHttpServerCustomValidationRequestedEventArgs}
} // Windows.Web.Http.Filters
pub mod headers { // Windows.Web.Http.Headers
use ::prelude::*;
DEFINE_IID!(IID_IHttpCacheDirectiveHeaderValueCollection, 2589485961, 54736, 20414, 189, 157, 181, 179, 99, 104, 17, 180);
RT_INTERFACE!{interface IHttpCacheDirectiveHeaderValueCollection(IHttpCacheDirectiveHeaderValueCollectionVtbl): IInspectable(IInspectableVtbl) [IID_IHttpCacheDirectiveHeaderValueCollection] {
    fn get_MaxAge(&self, out: *mut *mut ::rt::gen::windows::foundation::IReference<::rt::gen::windows::foundation::TimeSpan>) -> HRESULT,
    fn put_MaxAge(&self, value: *mut ::rt::gen::windows::foundation::IReference<::rt::gen::windows::foundation::TimeSpan>) -> HRESULT,
    fn get_MaxStale(&self, out: *mut *mut ::rt::gen::windows::foundation::IReference<::rt::gen::windows::foundation::TimeSpan>) -> HRESULT,
    fn put_MaxStale(&self, value: *mut ::rt::gen::windows::foundation::IReference<::rt::gen::windows::foundation::TimeSpan>) -> HRESULT,
    fn get_MinFresh(&self, out: *mut *mut ::rt::gen::windows::foundation::IReference<::rt::gen::windows::foundation::TimeSpan>) -> HRESULT,
    fn put_MinFresh(&self, value: *mut ::rt::gen::windows::foundation::IReference<::rt::gen::windows::foundation::TimeSpan>) -> HRESULT,
    fn get_SharedMaxAge(&self, out: *mut *mut ::rt::gen::windows::foundation::IReference<::rt::gen::windows::foundation::TimeSpan>) -> HRESULT,
    fn put_SharedMaxAge(&self, value: *mut ::rt::gen::windows::foundation::IReference<::rt::gen::windows::foundation::TimeSpan>) -> HRESULT,
    fn ParseAdd(&self, input: HSTRING) -> HRESULT,
    fn TryParseAdd(&self, input: HSTRING, out: *mut bool) -> HRESULT
}}
impl IHttpCacheDirectiveHeaderValueCollection {
    #[inline] pub unsafe fn get_max_age(&self) -> Result<ComPtr<::rt::gen::windows::foundation::IReference<::rt::gen::windows::foundation::TimeSpan>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_MaxAge)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_max_age(&self, value: &::rt::gen::windows::foundation::IReference<::rt::gen::windows::foundation::TimeSpan>) -> Result<()> {
        let hr = ((*self.lpVtbl).put_MaxAge)(self as *const _ as *mut _, value as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_max_stale(&self) -> Result<ComPtr<::rt::gen::windows::foundation::IReference<::rt::gen::windows::foundation::TimeSpan>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_MaxStale)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_max_stale(&self, value: &::rt::gen::windows::foundation::IReference<::rt::gen::windows::foundation::TimeSpan>) -> Result<()> {
        let hr = ((*self.lpVtbl).put_MaxStale)(self as *const _ as *mut _, value as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_min_fresh(&self) -> Result<ComPtr<::rt::gen::windows::foundation::IReference<::rt::gen::windows::foundation::TimeSpan>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_MinFresh)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_min_fresh(&self, value: &::rt::gen::windows::foundation::IReference<::rt::gen::windows::foundation::TimeSpan>) -> Result<()> {
        let hr = ((*self.lpVtbl).put_MinFresh)(self as *const _ as *mut _, value as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_shared_max_age(&self) -> Result<ComPtr<::rt::gen::windows::foundation::IReference<::rt::gen::windows::foundation::TimeSpan>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_SharedMaxAge)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_shared_max_age(&self, value: &::rt::gen::windows::foundation::IReference<::rt::gen::windows::foundation::TimeSpan>) -> Result<()> {
        let hr = ((*self.lpVtbl).put_SharedMaxAge)(self as *const _ as *mut _, value as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn parse_add(&self, input: &HStringArg) -> Result<()> {
        let hr = ((*self.lpVtbl).ParseAdd)(self as *const _ as *mut _, input.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn try_parse_add(&self, input: &HStringArg) -> Result<bool> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).TryParseAdd)(self as *const _ as *mut _, input.get(), &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
}
RT_CLASS!{class HttpCacheDirectiveHeaderValueCollection: IHttpCacheDirectiveHeaderValueCollection}
DEFINE_IID!(IID_IHttpChallengeHeaderValue, 959668655, 3965, 18464, 159, 221, 162, 185, 86, 238, 174, 171);
RT_INTERFACE!{interface IHttpChallengeHeaderValue(IHttpChallengeHeaderValueVtbl): IInspectable(IInspectableVtbl) [IID_IHttpChallengeHeaderValue] {
    fn get_Parameters(&self, out: *mut *mut ::rt::gen::windows::foundation::collections::IVector<HttpNameValueHeaderValue>) -> HRESULT,
    fn get_Scheme(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Token(&self, out: *mut HSTRING) -> HRESULT
}}
impl IHttpChallengeHeaderValue {
    #[inline] pub unsafe fn get_parameters(&self) -> Result<ComPtr<::rt::gen::windows::foundation::collections::IVector<HttpNameValueHeaderValue>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Parameters)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_scheme(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Scheme)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_token(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Token)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
}
RT_CLASS!{class HttpChallengeHeaderValue: IHttpChallengeHeaderValue}
impl RtActivatable<IHttpChallengeHeaderValueFactory> for HttpChallengeHeaderValue {}
impl RtActivatable<IHttpChallengeHeaderValueStatics> for HttpChallengeHeaderValue {}
impl HttpChallengeHeaderValue {
    #[inline] pub fn create_from_scheme(scheme: &HStringArg) -> Result<ComPtr<HttpChallengeHeaderValue>> { unsafe {
        <Self as RtActivatable<IHttpChallengeHeaderValueFactory>>::get_activation_factory().create_from_scheme(scheme)
    }}
    #[inline] pub fn create_from_scheme_with_token(scheme: &HStringArg, token: &HStringArg) -> Result<ComPtr<HttpChallengeHeaderValue>> { unsafe {
        <Self as RtActivatable<IHttpChallengeHeaderValueFactory>>::get_activation_factory().create_from_scheme_with_token(scheme, token)
    }}
    #[inline] pub fn parse(input: &HStringArg) -> Result<ComPtr<HttpChallengeHeaderValue>> { unsafe {
        <Self as RtActivatable<IHttpChallengeHeaderValueStatics>>::get_activation_factory().parse(input)
    }}
    #[inline] pub fn try_parse(input: &HStringArg) -> Result<(ComPtr<HttpChallengeHeaderValue>, bool)> { unsafe {
        <Self as RtActivatable<IHttpChallengeHeaderValueStatics>>::get_activation_factory().try_parse(input)
    }}
}
DEFINE_CLSID!(HttpChallengeHeaderValue(&[87,105,110,100,111,119,115,46,87,101,98,46,72,116,116,112,46,72,101,97,100,101,114,115,46,72,116,116,112,67,104,97,108,108,101,110,103,101,72,101,97,100,101,114,86,97,108,117,101,0]) [CLSID_HttpChallengeHeaderValue]);
DEFINE_IID!(IID_IHttpChallengeHeaderValueCollection, 3399376769, 44768, 17235, 161, 11, 230, 37, 186, 189, 100, 194);
RT_INTERFACE!{interface IHttpChallengeHeaderValueCollection(IHttpChallengeHeaderValueCollectionVtbl): IInspectable(IInspectableVtbl) [IID_IHttpChallengeHeaderValueCollection] {
    fn ParseAdd(&self, input: HSTRING) -> HRESULT,
    fn TryParseAdd(&self, input: HSTRING, out: *mut bool) -> HRESULT
}}
impl IHttpChallengeHeaderValueCollection {
    #[inline] pub unsafe fn parse_add(&self, input: &HStringArg) -> Result<()> {
        let hr = ((*self.lpVtbl).ParseAdd)(self as *const _ as *mut _, input.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn try_parse_add(&self, input: &HStringArg) -> Result<bool> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).TryParseAdd)(self as *const _ as *mut _, input.get(), &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
}
RT_CLASS!{class HttpChallengeHeaderValueCollection: IHttpChallengeHeaderValueCollection}
DEFINE_IID!(IID_IHttpChallengeHeaderValueFactory, 3293758545, 55708, 16554, 147, 153, 144, 238, 185, 143, 198, 19);
RT_INTERFACE!{static interface IHttpChallengeHeaderValueFactory(IHttpChallengeHeaderValueFactoryVtbl): IInspectable(IInspectableVtbl) [IID_IHttpChallengeHeaderValueFactory] {
    fn CreateFromScheme(&self, scheme: HSTRING, out: *mut *mut HttpChallengeHeaderValue) -> HRESULT,
    fn CreateFromSchemeWithToken(&self, scheme: HSTRING, token: HSTRING, out: *mut *mut HttpChallengeHeaderValue) -> HRESULT
}}
impl IHttpChallengeHeaderValueFactory {
    #[inline] pub unsafe fn create_from_scheme(&self, scheme: &HStringArg) -> Result<ComPtr<HttpChallengeHeaderValue>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).CreateFromScheme)(self as *const _ as *mut _, scheme.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn create_from_scheme_with_token(&self, scheme: &HStringArg, token: &HStringArg) -> Result<ComPtr<HttpChallengeHeaderValue>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).CreateFromSchemeWithToken)(self as *const _ as *mut _, scheme.get(), token.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
DEFINE_IID!(IID_IHttpChallengeHeaderValueStatics, 4090727026, 64513, 19713, 160, 8, 252, 183, 196, 89, 214, 53);
RT_INTERFACE!{static interface IHttpChallengeHeaderValueStatics(IHttpChallengeHeaderValueStaticsVtbl): IInspectable(IInspectableVtbl) [IID_IHttpChallengeHeaderValueStatics] {
    fn Parse(&self, input: HSTRING, out: *mut *mut HttpChallengeHeaderValue) -> HRESULT,
    fn TryParse(&self, input: HSTRING, challengeHeaderValue: *mut *mut HttpChallengeHeaderValue, out: *mut bool) -> HRESULT
}}
impl IHttpChallengeHeaderValueStatics {
    #[inline] pub unsafe fn parse(&self, input: &HStringArg) -> Result<ComPtr<HttpChallengeHeaderValue>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).Parse)(self as *const _ as *mut _, input.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn try_parse(&self, input: &HStringArg) -> Result<(ComPtr<HttpChallengeHeaderValue>, bool)> {
        let mut challengeHeaderValue = null_mut(); let mut out = zeroed();
        let hr = ((*self.lpVtbl).TryParse)(self as *const _ as *mut _, input.get(), &mut challengeHeaderValue, &mut out);
        if hr == S_OK { Ok((ComPtr::wrap(challengeHeaderValue), out)) } else { err(hr) }
    }
}
DEFINE_IID!(IID_IHttpConnectionOptionHeaderValue, 3410686586, 20112, 17899, 141, 205, 253, 20, 8, 244, 196, 79);
RT_INTERFACE!{interface IHttpConnectionOptionHeaderValue(IHttpConnectionOptionHeaderValueVtbl): IInspectable(IInspectableVtbl) [IID_IHttpConnectionOptionHeaderValue] {
    fn get_Token(&self, out: *mut HSTRING) -> HRESULT
}}
impl IHttpConnectionOptionHeaderValue {
    #[inline] pub unsafe fn get_token(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Token)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
}
RT_CLASS!{class HttpConnectionOptionHeaderValue: IHttpConnectionOptionHeaderValue}
impl RtActivatable<IHttpConnectionOptionHeaderValueFactory> for HttpConnectionOptionHeaderValue {}
impl RtActivatable<IHttpConnectionOptionHeaderValueStatics> for HttpConnectionOptionHeaderValue {}
impl HttpConnectionOptionHeaderValue {
    #[inline] pub fn create(token: &HStringArg) -> Result<ComPtr<HttpConnectionOptionHeaderValue>> { unsafe {
        <Self as RtActivatable<IHttpConnectionOptionHeaderValueFactory>>::get_activation_factory().create(token)
    }}
    #[inline] pub fn parse(input: &HStringArg) -> Result<ComPtr<HttpConnectionOptionHeaderValue>> { unsafe {
        <Self as RtActivatable<IHttpConnectionOptionHeaderValueStatics>>::get_activation_factory().parse(input)
    }}
    #[inline] pub fn try_parse(input: &HStringArg) -> Result<(ComPtr<HttpConnectionOptionHeaderValue>, bool)> { unsafe {
        <Self as RtActivatable<IHttpConnectionOptionHeaderValueStatics>>::get_activation_factory().try_parse(input)
    }}
}
DEFINE_CLSID!(HttpConnectionOptionHeaderValue(&[87,105,110,100,111,119,115,46,87,101,98,46,72,116,116,112,46,72,101,97,100,101,114,115,46,72,116,116,112,67,111,110,110,101,99,116,105,111,110,79,112,116,105,111,110,72,101,97,100,101,114,86,97,108,117,101,0]) [CLSID_HttpConnectionOptionHeaderValue]);
DEFINE_IID!(IID_IHttpConnectionOptionHeaderValueCollection, 3841289245, 20802, 19968, 142, 15, 1, 149, 9, 51, 118, 41);
RT_INTERFACE!{interface IHttpConnectionOptionHeaderValueCollection(IHttpConnectionOptionHeaderValueCollectionVtbl): IInspectable(IInspectableVtbl) [IID_IHttpConnectionOptionHeaderValueCollection] {
    fn ParseAdd(&self, input: HSTRING) -> HRESULT,
    fn TryParseAdd(&self, input: HSTRING, out: *mut bool) -> HRESULT
}}
impl IHttpConnectionOptionHeaderValueCollection {
    #[inline] pub unsafe fn parse_add(&self, input: &HStringArg) -> Result<()> {
        let hr = ((*self.lpVtbl).ParseAdd)(self as *const _ as *mut _, input.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn try_parse_add(&self, input: &HStringArg) -> Result<bool> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).TryParseAdd)(self as *const _ as *mut _, input.get(), &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
}
RT_CLASS!{class HttpConnectionOptionHeaderValueCollection: IHttpConnectionOptionHeaderValueCollection}
DEFINE_IID!(IID_IHttpConnectionOptionHeaderValueFactory, 3644640286, 2941, 19519, 165, 141, 162, 161, 189, 234, 188, 10);
RT_INTERFACE!{static interface IHttpConnectionOptionHeaderValueFactory(IHttpConnectionOptionHeaderValueFactoryVtbl): IInspectable(IInspectableVtbl) [IID_IHttpConnectionOptionHeaderValueFactory] {
    fn Create(&self, token: HSTRING, out: *mut *mut HttpConnectionOptionHeaderValue) -> HRESULT
}}
impl IHttpConnectionOptionHeaderValueFactory {
    #[inline] pub unsafe fn create(&self, token: &HStringArg) -> Result<ComPtr<HttpConnectionOptionHeaderValue>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).Create)(self as *const _ as *mut _, token.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
DEFINE_IID!(IID_IHttpConnectionOptionHeaderValueStatics, 2863095095, 43334, 19231, 133, 175, 72, 182, 139, 60, 80, 189);
RT_INTERFACE!{static interface IHttpConnectionOptionHeaderValueStatics(IHttpConnectionOptionHeaderValueStaticsVtbl): IInspectable(IInspectableVtbl) [IID_IHttpConnectionOptionHeaderValueStatics] {
    fn Parse(&self, input: HSTRING, out: *mut *mut HttpConnectionOptionHeaderValue) -> HRESULT,
    fn TryParse(&self, input: HSTRING, connectionOptionHeaderValue: *mut *mut HttpConnectionOptionHeaderValue, out: *mut bool) -> HRESULT
}}
impl IHttpConnectionOptionHeaderValueStatics {
    #[inline] pub unsafe fn parse(&self, input: &HStringArg) -> Result<ComPtr<HttpConnectionOptionHeaderValue>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).Parse)(self as *const _ as *mut _, input.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn try_parse(&self, input: &HStringArg) -> Result<(ComPtr<HttpConnectionOptionHeaderValue>, bool)> {
        let mut connectionOptionHeaderValue = null_mut(); let mut out = zeroed();
        let hr = ((*self.lpVtbl).TryParse)(self as *const _ as *mut _, input.get(), &mut connectionOptionHeaderValue, &mut out);
        if hr == S_OK { Ok((ComPtr::wrap(connectionOptionHeaderValue), out)) } else { err(hr) }
    }
}
DEFINE_IID!(IID_IHttpContentCodingHeaderValue, 3170367786, 37750, 19845, 188, 204, 159, 79, 154, 202, 180, 52);
RT_INTERFACE!{interface IHttpContentCodingHeaderValue(IHttpContentCodingHeaderValueVtbl): IInspectable(IInspectableVtbl) [IID_IHttpContentCodingHeaderValue] {
    fn get_ContentCoding(&self, out: *mut HSTRING) -> HRESULT
}}
impl IHttpContentCodingHeaderValue {
    #[inline] pub unsafe fn get_content_coding(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_ContentCoding)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
}
RT_CLASS!{class HttpContentCodingHeaderValue: IHttpContentCodingHeaderValue}
impl RtActivatable<IHttpContentCodingHeaderValueFactory> for HttpContentCodingHeaderValue {}
impl RtActivatable<IHttpContentCodingHeaderValueStatics> for HttpContentCodingHeaderValue {}
impl HttpContentCodingHeaderValue {
    #[inline] pub fn create(contentCoding: &HStringArg) -> Result<ComPtr<HttpContentCodingHeaderValue>> { unsafe {
        <Self as RtActivatable<IHttpContentCodingHeaderValueFactory>>::get_activation_factory().create(contentCoding)
    }}
    #[inline] pub fn parse(input: &HStringArg) -> Result<ComPtr<HttpContentCodingHeaderValue>> { unsafe {
        <Self as RtActivatable<IHttpContentCodingHeaderValueStatics>>::get_activation_factory().parse(input)
    }}
    #[inline] pub fn try_parse(input: &HStringArg) -> Result<(ComPtr<HttpContentCodingHeaderValue>, bool)> { unsafe {
        <Self as RtActivatable<IHttpContentCodingHeaderValueStatics>>::get_activation_factory().try_parse(input)
    }}
}
DEFINE_CLSID!(HttpContentCodingHeaderValue(&[87,105,110,100,111,119,115,46,87,101,98,46,72,116,116,112,46,72,101,97,100,101,114,115,46,72,116,116,112,67,111,110,116,101,110,116,67,111,100,105,110,103,72,101,97,100,101,114,86,97,108,117,101,0]) [CLSID_HttpContentCodingHeaderValue]);
DEFINE_IID!(IID_IHttpContentCodingHeaderValueCollection, 2099386145, 42715, 17262, 142, 131, 145, 89, 97, 146, 129, 156);
RT_INTERFACE!{interface IHttpContentCodingHeaderValueCollection(IHttpContentCodingHeaderValueCollectionVtbl): IInspectable(IInspectableVtbl) [IID_IHttpContentCodingHeaderValueCollection] {
    fn ParseAdd(&self, input: HSTRING) -> HRESULT,
    fn TryParseAdd(&self, input: HSTRING, out: *mut bool) -> HRESULT
}}
impl IHttpContentCodingHeaderValueCollection {
    #[inline] pub unsafe fn parse_add(&self, input: &HStringArg) -> Result<()> {
        let hr = ((*self.lpVtbl).ParseAdd)(self as *const _ as *mut _, input.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn try_parse_add(&self, input: &HStringArg) -> Result<bool> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).TryParseAdd)(self as *const _ as *mut _, input.get(), &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
}
RT_CLASS!{class HttpContentCodingHeaderValueCollection: IHttpContentCodingHeaderValueCollection}
DEFINE_IID!(IID_IHttpContentCodingHeaderValueFactory, 3309120471, 13099, 17232, 133, 16, 46, 103, 162, 40, 154, 90);
RT_INTERFACE!{static interface IHttpContentCodingHeaderValueFactory(IHttpContentCodingHeaderValueFactoryVtbl): IInspectable(IInspectableVtbl) [IID_IHttpContentCodingHeaderValueFactory] {
    fn Create(&self, contentCoding: HSTRING, out: *mut *mut HttpContentCodingHeaderValue) -> HRESULT
}}
impl IHttpContentCodingHeaderValueFactory {
    #[inline] pub unsafe fn create(&self, contentCoding: &HStringArg) -> Result<ComPtr<HttpContentCodingHeaderValue>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).Create)(self as *const _ as *mut _, contentCoding.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
DEFINE_IID!(IID_IHttpContentCodingHeaderValueStatics, 2497208366, 63935, 17143, 170, 70, 237, 39, 42, 65, 226, 18);
RT_INTERFACE!{static interface IHttpContentCodingHeaderValueStatics(IHttpContentCodingHeaderValueStaticsVtbl): IInspectable(IInspectableVtbl) [IID_IHttpContentCodingHeaderValueStatics] {
    fn Parse(&self, input: HSTRING, out: *mut *mut HttpContentCodingHeaderValue) -> HRESULT,
    fn TryParse(&self, input: HSTRING, contentCodingHeaderValue: *mut *mut HttpContentCodingHeaderValue, out: *mut bool) -> HRESULT
}}
impl IHttpContentCodingHeaderValueStatics {
    #[inline] pub unsafe fn parse(&self, input: &HStringArg) -> Result<ComPtr<HttpContentCodingHeaderValue>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).Parse)(self as *const _ as *mut _, input.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn try_parse(&self, input: &HStringArg) -> Result<(ComPtr<HttpContentCodingHeaderValue>, bool)> {
        let mut contentCodingHeaderValue = null_mut(); let mut out = zeroed();
        let hr = ((*self.lpVtbl).TryParse)(self as *const _ as *mut _, input.get(), &mut contentCodingHeaderValue, &mut out);
        if hr == S_OK { Ok((ComPtr::wrap(contentCodingHeaderValue), out)) } else { err(hr) }
    }
}
DEFINE_IID!(IID_IHttpContentCodingWithQualityHeaderValue, 2488474837, 35603, 19827, 134, 81, 247, 107, 56, 248, 132, 149);
RT_INTERFACE!{interface IHttpContentCodingWithQualityHeaderValue(IHttpContentCodingWithQualityHeaderValueVtbl): IInspectable(IInspectableVtbl) [IID_IHttpContentCodingWithQualityHeaderValue] {
    fn get_ContentCoding(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Quality(&self, out: *mut *mut ::rt::gen::windows::foundation::IReference<f64>) -> HRESULT
}}
impl IHttpContentCodingWithQualityHeaderValue {
    #[inline] pub unsafe fn get_content_coding(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_ContentCoding)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_quality(&self) -> Result<ComPtr<::rt::gen::windows::foundation::IReference<f64>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Quality)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
RT_CLASS!{class HttpContentCodingWithQualityHeaderValue: IHttpContentCodingWithQualityHeaderValue}
impl RtActivatable<IHttpContentCodingWithQualityHeaderValueFactory> for HttpContentCodingWithQualityHeaderValue {}
impl RtActivatable<IHttpContentCodingWithQualityHeaderValueStatics> for HttpContentCodingWithQualityHeaderValue {}
impl HttpContentCodingWithQualityHeaderValue {
    #[inline] pub fn create_from_value(contentCoding: &HStringArg) -> Result<ComPtr<HttpContentCodingWithQualityHeaderValue>> { unsafe {
        <Self as RtActivatable<IHttpContentCodingWithQualityHeaderValueFactory>>::get_activation_factory().create_from_value(contentCoding)
    }}
    #[inline] pub fn create_from_value_with_quality(contentCoding: &HStringArg, quality: f64) -> Result<ComPtr<HttpContentCodingWithQualityHeaderValue>> { unsafe {
        <Self as RtActivatable<IHttpContentCodingWithQualityHeaderValueFactory>>::get_activation_factory().create_from_value_with_quality(contentCoding, quality)
    }}
    #[inline] pub fn parse(input: &HStringArg) -> Result<ComPtr<HttpContentCodingWithQualityHeaderValue>> { unsafe {
        <Self as RtActivatable<IHttpContentCodingWithQualityHeaderValueStatics>>::get_activation_factory().parse(input)
    }}
    #[inline] pub fn try_parse(input: &HStringArg) -> Result<(ComPtr<HttpContentCodingWithQualityHeaderValue>, bool)> { unsafe {
        <Self as RtActivatable<IHttpContentCodingWithQualityHeaderValueStatics>>::get_activation_factory().try_parse(input)
    }}
}
DEFINE_CLSID!(HttpContentCodingWithQualityHeaderValue(&[87,105,110,100,111,119,115,46,87,101,98,46,72,116,116,112,46,72,101,97,100,101,114,115,46,72,116,116,112,67,111,110,116,101,110,116,67,111,100,105,110,103,87,105,116,104,81,117,97,108,105,116,121,72,101,97,100,101,114,86,97,108,117,101,0]) [CLSID_HttpContentCodingWithQualityHeaderValue]);
DEFINE_IID!(IID_IHttpContentCodingWithQualityHeaderValueCollection, 2081256766, 59545, 17272, 181, 200, 65, 45, 130, 7, 17, 204);
RT_INTERFACE!{interface IHttpContentCodingWithQualityHeaderValueCollection(IHttpContentCodingWithQualityHeaderValueCollectionVtbl): IInspectable(IInspectableVtbl) [IID_IHttpContentCodingWithQualityHeaderValueCollection] {
    fn ParseAdd(&self, input: HSTRING) -> HRESULT,
    fn TryParseAdd(&self, input: HSTRING, out: *mut bool) -> HRESULT
}}
impl IHttpContentCodingWithQualityHeaderValueCollection {
    #[inline] pub unsafe fn parse_add(&self, input: &HStringArg) -> Result<()> {
        let hr = ((*self.lpVtbl).ParseAdd)(self as *const _ as *mut _, input.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn try_parse_add(&self, input: &HStringArg) -> Result<bool> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).TryParseAdd)(self as *const _ as *mut _, input.get(), &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
}
RT_CLASS!{class HttpContentCodingWithQualityHeaderValueCollection: IHttpContentCodingWithQualityHeaderValueCollection}
DEFINE_IID!(IID_IHttpContentCodingWithQualityHeaderValueFactory, 3294555674, 50515, 18172, 173, 226, 215, 92, 29, 83, 223, 123);
RT_INTERFACE!{static interface IHttpContentCodingWithQualityHeaderValueFactory(IHttpContentCodingWithQualityHeaderValueFactoryVtbl): IInspectable(IInspectableVtbl) [IID_IHttpContentCodingWithQualityHeaderValueFactory] {
    fn CreateFromValue(&self, contentCoding: HSTRING, out: *mut *mut HttpContentCodingWithQualityHeaderValue) -> HRESULT,
    fn CreateFromValueWithQuality(&self, contentCoding: HSTRING, quality: f64, out: *mut *mut HttpContentCodingWithQualityHeaderValue) -> HRESULT
}}
impl IHttpContentCodingWithQualityHeaderValueFactory {
    #[inline] pub unsafe fn create_from_value(&self, contentCoding: &HStringArg) -> Result<ComPtr<HttpContentCodingWithQualityHeaderValue>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).CreateFromValue)(self as *const _ as *mut _, contentCoding.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn create_from_value_with_quality(&self, contentCoding: &HStringArg, quality: f64) -> Result<ComPtr<HttpContentCodingWithQualityHeaderValue>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).CreateFromValueWithQuality)(self as *const _ as *mut _, contentCoding.get(), quality, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
DEFINE_IID!(IID_IHttpContentCodingWithQualityHeaderValueStatics, 3905500540, 36745, 18433, 142, 117, 76, 154, 191, 195, 222, 113);
RT_INTERFACE!{static interface IHttpContentCodingWithQualityHeaderValueStatics(IHttpContentCodingWithQualityHeaderValueStaticsVtbl): IInspectable(IInspectableVtbl) [IID_IHttpContentCodingWithQualityHeaderValueStatics] {
    fn Parse(&self, input: HSTRING, out: *mut *mut HttpContentCodingWithQualityHeaderValue) -> HRESULT,
    fn TryParse(&self, input: HSTRING, contentCodingWithQualityHeaderValue: *mut *mut HttpContentCodingWithQualityHeaderValue, out: *mut bool) -> HRESULT
}}
impl IHttpContentCodingWithQualityHeaderValueStatics {
    #[inline] pub unsafe fn parse(&self, input: &HStringArg) -> Result<ComPtr<HttpContentCodingWithQualityHeaderValue>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).Parse)(self as *const _ as *mut _, input.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn try_parse(&self, input: &HStringArg) -> Result<(ComPtr<HttpContentCodingWithQualityHeaderValue>, bool)> {
        let mut contentCodingWithQualityHeaderValue = null_mut(); let mut out = zeroed();
        let hr = ((*self.lpVtbl).TryParse)(self as *const _ as *mut _, input.get(), &mut contentCodingWithQualityHeaderValue, &mut out);
        if hr == S_OK { Ok((ComPtr::wrap(contentCodingWithQualityHeaderValue), out)) } else { err(hr) }
    }
}
DEFINE_IID!(IID_IHttpContentDispositionHeaderValue, 4070764252, 9769, 19273, 153, 8, 150, 161, 104, 233, 54, 94);
RT_INTERFACE!{interface IHttpContentDispositionHeaderValue(IHttpContentDispositionHeaderValueVtbl): IInspectable(IInspectableVtbl) [IID_IHttpContentDispositionHeaderValue] {
    fn get_DispositionType(&self, out: *mut HSTRING) -> HRESULT,
    fn put_DispositionType(&self, value: HSTRING) -> HRESULT,
    fn get_FileName(&self, out: *mut HSTRING) -> HRESULT,
    fn put_FileName(&self, value: HSTRING) -> HRESULT,
    fn get_FileNameStar(&self, out: *mut HSTRING) -> HRESULT,
    fn put_FileNameStar(&self, value: HSTRING) -> HRESULT,
    fn get_Name(&self, out: *mut HSTRING) -> HRESULT,
    fn put_Name(&self, value: HSTRING) -> HRESULT,
    fn get_Parameters(&self, out: *mut *mut ::rt::gen::windows::foundation::collections::IVector<HttpNameValueHeaderValue>) -> HRESULT,
    fn get_Size(&self, out: *mut *mut ::rt::gen::windows::foundation::IReference<u64>) -> HRESULT,
    fn put_Size(&self, value: *mut ::rt::gen::windows::foundation::IReference<u64>) -> HRESULT
}}
impl IHttpContentDispositionHeaderValue {
    #[inline] pub unsafe fn get_disposition_type(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_DispositionType)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_disposition_type(&self, value: &HStringArg) -> Result<()> {
        let hr = ((*self.lpVtbl).put_DispositionType)(self as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_file_name(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_FileName)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_file_name(&self, value: &HStringArg) -> Result<()> {
        let hr = ((*self.lpVtbl).put_FileName)(self as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_file_name_star(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_FileNameStar)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_file_name_star(&self, value: &HStringArg) -> Result<()> {
        let hr = ((*self.lpVtbl).put_FileNameStar)(self as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_name(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Name)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_name(&self, value: &HStringArg) -> Result<()> {
        let hr = ((*self.lpVtbl).put_Name)(self as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_parameters(&self) -> Result<ComPtr<::rt::gen::windows::foundation::collections::IVector<HttpNameValueHeaderValue>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Parameters)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_size(&self) -> Result<ComPtr<::rt::gen::windows::foundation::IReference<u64>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Size)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_size(&self, value: &::rt::gen::windows::foundation::IReference<u64>) -> Result<()> {
        let hr = ((*self.lpVtbl).put_Size)(self as *const _ as *mut _, value as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
}
RT_CLASS!{class HttpContentDispositionHeaderValue: IHttpContentDispositionHeaderValue}
impl RtActivatable<IHttpContentDispositionHeaderValueFactory> for HttpContentDispositionHeaderValue {}
impl RtActivatable<IHttpContentDispositionHeaderValueStatics> for HttpContentDispositionHeaderValue {}
impl HttpContentDispositionHeaderValue {
    #[inline] pub fn create(dispositionType: &HStringArg) -> Result<ComPtr<HttpContentDispositionHeaderValue>> { unsafe {
        <Self as RtActivatable<IHttpContentDispositionHeaderValueFactory>>::get_activation_factory().create(dispositionType)
    }}
    #[inline] pub fn parse(input: &HStringArg) -> Result<ComPtr<HttpContentDispositionHeaderValue>> { unsafe {
        <Self as RtActivatable<IHttpContentDispositionHeaderValueStatics>>::get_activation_factory().parse(input)
    }}
    #[inline] pub fn try_parse(input: &HStringArg) -> Result<(ComPtr<HttpContentDispositionHeaderValue>, bool)> { unsafe {
        <Self as RtActivatable<IHttpContentDispositionHeaderValueStatics>>::get_activation_factory().try_parse(input)
    }}
}
DEFINE_CLSID!(HttpContentDispositionHeaderValue(&[87,105,110,100,111,119,115,46,87,101,98,46,72,116,116,112,46,72,101,97,100,101,114,115,46,72,116,116,112,67,111,110,116,101,110,116,68,105,115,112,111,115,105,116,105,111,110,72,101,97,100,101,114,86,97,108,117,101,0]) [CLSID_HttpContentDispositionHeaderValue]);
DEFINE_IID!(IID_IHttpContentDispositionHeaderValueFactory, 2568338372, 17772, 20097, 130, 149, 178, 171, 60, 188, 245, 69);
RT_INTERFACE!{static interface IHttpContentDispositionHeaderValueFactory(IHttpContentDispositionHeaderValueFactoryVtbl): IInspectable(IInspectableVtbl) [IID_IHttpContentDispositionHeaderValueFactory] {
    fn Create(&self, dispositionType: HSTRING, out: *mut *mut HttpContentDispositionHeaderValue) -> HRESULT
}}
impl IHttpContentDispositionHeaderValueFactory {
    #[inline] pub unsafe fn create(&self, dispositionType: &HStringArg) -> Result<ComPtr<HttpContentDispositionHeaderValue>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).Create)(self as *const _ as *mut _, dispositionType.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
DEFINE_IID!(IID_IHttpContentDispositionHeaderValueStatics, 700801127, 23095, 18148, 176, 116, 197, 23, 125, 105, 202, 102);
RT_INTERFACE!{static interface IHttpContentDispositionHeaderValueStatics(IHttpContentDispositionHeaderValueStaticsVtbl): IInspectable(IInspectableVtbl) [IID_IHttpContentDispositionHeaderValueStatics] {
    fn Parse(&self, input: HSTRING, out: *mut *mut HttpContentDispositionHeaderValue) -> HRESULT,
    fn TryParse(&self, input: HSTRING, contentDispositionHeaderValue: *mut *mut HttpContentDispositionHeaderValue, out: *mut bool) -> HRESULT
}}
impl IHttpContentDispositionHeaderValueStatics {
    #[inline] pub unsafe fn parse(&self, input: &HStringArg) -> Result<ComPtr<HttpContentDispositionHeaderValue>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).Parse)(self as *const _ as *mut _, input.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn try_parse(&self, input: &HStringArg) -> Result<(ComPtr<HttpContentDispositionHeaderValue>, bool)> {
        let mut contentDispositionHeaderValue = null_mut(); let mut out = zeroed();
        let hr = ((*self.lpVtbl).TryParse)(self as *const _ as *mut _, input.get(), &mut contentDispositionHeaderValue, &mut out);
        if hr == S_OK { Ok((ComPtr::wrap(contentDispositionHeaderValue), out)) } else { err(hr) }
    }
}
DEFINE_IID!(IID_IHttpContentHeaderCollection, 1080109636, 18350, 19326, 145, 36, 105, 98, 139, 100, 170, 24);
RT_INTERFACE!{interface IHttpContentHeaderCollection(IHttpContentHeaderCollectionVtbl): IInspectable(IInspectableVtbl) [IID_IHttpContentHeaderCollection] {
    fn get_ContentDisposition(&self, out: *mut *mut HttpContentDispositionHeaderValue) -> HRESULT,
    fn put_ContentDisposition(&self, value: *mut HttpContentDispositionHeaderValue) -> HRESULT,
    fn get_ContentEncoding(&self, out: *mut *mut HttpContentCodingHeaderValueCollection) -> HRESULT,
    fn get_ContentLanguage(&self, out: *mut *mut HttpLanguageHeaderValueCollection) -> HRESULT,
    fn get_ContentLength(&self, out: *mut *mut ::rt::gen::windows::foundation::IReference<u64>) -> HRESULT,
    fn put_ContentLength(&self, value: *mut ::rt::gen::windows::foundation::IReference<u64>) -> HRESULT,
    fn get_ContentLocation(&self, out: *mut *mut ::rt::gen::windows::foundation::Uri) -> HRESULT,
    fn put_ContentLocation(&self, value: *mut ::rt::gen::windows::foundation::Uri) -> HRESULT,
    #[cfg(not(feature="windows-storage"))] fn __Dummy8(&self) -> (),
    #[cfg(feature="windows-storage")] fn get_ContentMD5(&self, out: *mut *mut ::rt::gen::windows::storage::streams::IBuffer) -> HRESULT,
    #[cfg(not(feature="windows-storage"))] fn __Dummy9(&self) -> (),
    #[cfg(feature="windows-storage")] fn put_ContentMD5(&self, value: *mut ::rt::gen::windows::storage::streams::IBuffer) -> HRESULT,
    fn get_ContentRange(&self, out: *mut *mut HttpContentRangeHeaderValue) -> HRESULT,
    fn put_ContentRange(&self, value: *mut HttpContentRangeHeaderValue) -> HRESULT,
    fn get_ContentType(&self, out: *mut *mut HttpMediaTypeHeaderValue) -> HRESULT,
    fn put_ContentType(&self, value: *mut HttpMediaTypeHeaderValue) -> HRESULT,
    fn get_Expires(&self, out: *mut *mut ::rt::gen::windows::foundation::IReference<::rt::gen::windows::foundation::DateTime>) -> HRESULT,
    fn put_Expires(&self, value: *mut ::rt::gen::windows::foundation::IReference<::rt::gen::windows::foundation::DateTime>) -> HRESULT,
    fn get_LastModified(&self, out: *mut *mut ::rt::gen::windows::foundation::IReference<::rt::gen::windows::foundation::DateTime>) -> HRESULT,
    fn put_LastModified(&self, value: *mut ::rt::gen::windows::foundation::IReference<::rt::gen::windows::foundation::DateTime>) -> HRESULT,
    fn Append(&self, name: HSTRING, value: HSTRING) -> HRESULT,
    fn TryAppendWithoutValidation(&self, name: HSTRING, value: HSTRING, out: *mut bool) -> HRESULT
}}
impl IHttpContentHeaderCollection {
    #[inline] pub unsafe fn get_content_disposition(&self) -> Result<ComPtr<HttpContentDispositionHeaderValue>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_ContentDisposition)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_content_disposition(&self, value: &HttpContentDispositionHeaderValue) -> Result<()> {
        let hr = ((*self.lpVtbl).put_ContentDisposition)(self as *const _ as *mut _, value as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_content_encoding(&self) -> Result<ComPtr<HttpContentCodingHeaderValueCollection>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_ContentEncoding)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_content_language(&self) -> Result<ComPtr<HttpLanguageHeaderValueCollection>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_ContentLanguage)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_content_length(&self) -> Result<ComPtr<::rt::gen::windows::foundation::IReference<u64>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_ContentLength)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_content_length(&self, value: &::rt::gen::windows::foundation::IReference<u64>) -> Result<()> {
        let hr = ((*self.lpVtbl).put_ContentLength)(self as *const _ as *mut _, value as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_content_location(&self) -> Result<ComPtr<::rt::gen::windows::foundation::Uri>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_ContentLocation)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_content_location(&self, value: &::rt::gen::windows::foundation::Uri) -> Result<()> {
        let hr = ((*self.lpVtbl).put_ContentLocation)(self as *const _ as *mut _, value as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[cfg(feature="windows-storage")] #[inline] pub unsafe fn get_content_md5(&self) -> Result<ComPtr<::rt::gen::windows::storage::streams::IBuffer>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_ContentMD5)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[cfg(feature="windows-storage")] #[inline] pub unsafe fn set_content_md5(&self, value: &::rt::gen::windows::storage::streams::IBuffer) -> Result<()> {
        let hr = ((*self.lpVtbl).put_ContentMD5)(self as *const _ as *mut _, value as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_content_range(&self) -> Result<ComPtr<HttpContentRangeHeaderValue>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_ContentRange)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_content_range(&self, value: &HttpContentRangeHeaderValue) -> Result<()> {
        let hr = ((*self.lpVtbl).put_ContentRange)(self as *const _ as *mut _, value as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_content_type(&self) -> Result<ComPtr<HttpMediaTypeHeaderValue>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_ContentType)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_content_type(&self, value: &HttpMediaTypeHeaderValue) -> Result<()> {
        let hr = ((*self.lpVtbl).put_ContentType)(self as *const _ as *mut _, value as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_expires(&self) -> Result<ComPtr<::rt::gen::windows::foundation::IReference<::rt::gen::windows::foundation::DateTime>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Expires)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_expires(&self, value: &::rt::gen::windows::foundation::IReference<::rt::gen::windows::foundation::DateTime>) -> Result<()> {
        let hr = ((*self.lpVtbl).put_Expires)(self as *const _ as *mut _, value as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_last_modified(&self) -> Result<ComPtr<::rt::gen::windows::foundation::IReference<::rt::gen::windows::foundation::DateTime>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_LastModified)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_last_modified(&self, value: &::rt::gen::windows::foundation::IReference<::rt::gen::windows::foundation::DateTime>) -> Result<()> {
        let hr = ((*self.lpVtbl).put_LastModified)(self as *const _ as *mut _, value as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn append(&self, name: &HStringArg, value: &HStringArg) -> Result<()> {
        let hr = ((*self.lpVtbl).Append)(self as *const _ as *mut _, name.get(), value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn try_append_without_validation(&self, name: &HStringArg, value: &HStringArg) -> Result<bool> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).TryAppendWithoutValidation)(self as *const _ as *mut _, name.get(), value.get(), &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
}
RT_CLASS!{class HttpContentHeaderCollection: IHttpContentHeaderCollection}
impl RtActivatable<IActivationFactory> for HttpContentHeaderCollection {}
DEFINE_CLSID!(HttpContentHeaderCollection(&[87,105,110,100,111,119,115,46,87,101,98,46,72,116,116,112,46,72,101,97,100,101,114,115,46,72,116,116,112,67,111,110,116,101,110,116,72,101,97,100,101,114,67,111,108,108,101,99,116,105,111,110,0]) [CLSID_HttpContentHeaderCollection]);
DEFINE_IID!(IID_IHttpContentRangeHeaderValue, 81356755, 42230, 18780, 149, 48, 133, 121, 252, 186, 138, 169);
RT_INTERFACE!{interface IHttpContentRangeHeaderValue(IHttpContentRangeHeaderValueVtbl): IInspectable(IInspectableVtbl) [IID_IHttpContentRangeHeaderValue] {
    fn get_FirstBytePosition(&self, out: *mut *mut ::rt::gen::windows::foundation::IReference<u64>) -> HRESULT,
    fn get_LastBytePosition(&self, out: *mut *mut ::rt::gen::windows::foundation::IReference<u64>) -> HRESULT,
    fn get_Length(&self, out: *mut *mut ::rt::gen::windows::foundation::IReference<u64>) -> HRESULT,
    fn get_Unit(&self, out: *mut HSTRING) -> HRESULT,
    fn put_Unit(&self, value: HSTRING) -> HRESULT
}}
impl IHttpContentRangeHeaderValue {
    #[inline] pub unsafe fn get_first_byte_position(&self) -> Result<ComPtr<::rt::gen::windows::foundation::IReference<u64>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_FirstBytePosition)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_last_byte_position(&self) -> Result<ComPtr<::rt::gen::windows::foundation::IReference<u64>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_LastBytePosition)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_length(&self) -> Result<ComPtr<::rt::gen::windows::foundation::IReference<u64>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Length)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_unit(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Unit)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_unit(&self, value: &HStringArg) -> Result<()> {
        let hr = ((*self.lpVtbl).put_Unit)(self as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
}
RT_CLASS!{class HttpContentRangeHeaderValue: IHttpContentRangeHeaderValue}
impl RtActivatable<IHttpContentRangeHeaderValueFactory> for HttpContentRangeHeaderValue {}
impl RtActivatable<IHttpContentRangeHeaderValueStatics> for HttpContentRangeHeaderValue {}
impl HttpContentRangeHeaderValue {
    #[inline] pub fn create_from_length(length: u64) -> Result<ComPtr<HttpContentRangeHeaderValue>> { unsafe {
        <Self as RtActivatable<IHttpContentRangeHeaderValueFactory>>::get_activation_factory().create_from_length(length)
    }}
    #[inline] pub fn create_from_range(from: u64, to: u64) -> Result<ComPtr<HttpContentRangeHeaderValue>> { unsafe {
        <Self as RtActivatable<IHttpContentRangeHeaderValueFactory>>::get_activation_factory().create_from_range(from, to)
    }}
    #[inline] pub fn create_from_range_with_length(from: u64, to: u64, length: u64) -> Result<ComPtr<HttpContentRangeHeaderValue>> { unsafe {
        <Self as RtActivatable<IHttpContentRangeHeaderValueFactory>>::get_activation_factory().create_from_range_with_length(from, to, length)
    }}
    #[inline] pub fn parse(input: &HStringArg) -> Result<ComPtr<HttpContentRangeHeaderValue>> { unsafe {
        <Self as RtActivatable<IHttpContentRangeHeaderValueStatics>>::get_activation_factory().parse(input)
    }}
    #[inline] pub fn try_parse(input: &HStringArg) -> Result<(ComPtr<HttpContentRangeHeaderValue>, bool)> { unsafe {
        <Self as RtActivatable<IHttpContentRangeHeaderValueStatics>>::get_activation_factory().try_parse(input)
    }}
}
DEFINE_CLSID!(HttpContentRangeHeaderValue(&[87,105,110,100,111,119,115,46,87,101,98,46,72,116,116,112,46,72,101,97,100,101,114,115,46,72,116,116,112,67,111,110,116,101,110,116,82,97,110,103,101,72,101,97,100,101,114,86,97,108,117,101,0]) [CLSID_HttpContentRangeHeaderValue]);
DEFINE_IID!(IID_IHttpContentRangeHeaderValueFactory, 1062983313, 41020, 17494, 154, 111, 239, 39, 236, 208, 60, 174);
RT_INTERFACE!{static interface IHttpContentRangeHeaderValueFactory(IHttpContentRangeHeaderValueFactoryVtbl): IInspectable(IInspectableVtbl) [IID_IHttpContentRangeHeaderValueFactory] {
    fn CreateFromLength(&self, length: u64, out: *mut *mut HttpContentRangeHeaderValue) -> HRESULT,
    fn CreateFromRange(&self, from: u64, to: u64, out: *mut *mut HttpContentRangeHeaderValue) -> HRESULT,
    fn CreateFromRangeWithLength(&self, from: u64, to: u64, length: u64, out: *mut *mut HttpContentRangeHeaderValue) -> HRESULT
}}
impl IHttpContentRangeHeaderValueFactory {
    #[inline] pub unsafe fn create_from_length(&self, length: u64) -> Result<ComPtr<HttpContentRangeHeaderValue>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).CreateFromLength)(self as *const _ as *mut _, length, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn create_from_range(&self, from: u64, to: u64) -> Result<ComPtr<HttpContentRangeHeaderValue>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).CreateFromRange)(self as *const _ as *mut _, from, to, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn create_from_range_with_length(&self, from: u64, to: u64, length: u64) -> Result<ComPtr<HttpContentRangeHeaderValue>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).CreateFromRangeWithLength)(self as *const _ as *mut _, from, to, length, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
DEFINE_IID!(IID_IHttpContentRangeHeaderValueStatics, 2158184138, 5964, 20398, 130, 28, 19, 76, 210, 148, 170, 56);
RT_INTERFACE!{static interface IHttpContentRangeHeaderValueStatics(IHttpContentRangeHeaderValueStaticsVtbl): IInspectable(IInspectableVtbl) [IID_IHttpContentRangeHeaderValueStatics] {
    fn Parse(&self, input: HSTRING, out: *mut *mut HttpContentRangeHeaderValue) -> HRESULT,
    fn TryParse(&self, input: HSTRING, contentRangeHeaderValue: *mut *mut HttpContentRangeHeaderValue, out: *mut bool) -> HRESULT
}}
impl IHttpContentRangeHeaderValueStatics {
    #[inline] pub unsafe fn parse(&self, input: &HStringArg) -> Result<ComPtr<HttpContentRangeHeaderValue>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).Parse)(self as *const _ as *mut _, input.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn try_parse(&self, input: &HStringArg) -> Result<(ComPtr<HttpContentRangeHeaderValue>, bool)> {
        let mut contentRangeHeaderValue = null_mut(); let mut out = zeroed();
        let hr = ((*self.lpVtbl).TryParse)(self as *const _ as *mut _, input.get(), &mut contentRangeHeaderValue, &mut out);
        if hr == S_OK { Ok((ComPtr::wrap(contentRangeHeaderValue), out)) } else { err(hr) }
    }
}
DEFINE_IID!(IID_IHttpCookiePairHeaderValue, 3419693591, 19241, 16683, 189, 144, 179, 216, 20, 171, 142, 27);
RT_INTERFACE!{interface IHttpCookiePairHeaderValue(IHttpCookiePairHeaderValueVtbl): IInspectable(IInspectableVtbl) [IID_IHttpCookiePairHeaderValue] {
    fn get_Name(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Value(&self, out: *mut HSTRING) -> HRESULT,
    fn put_Value(&self, value: HSTRING) -> HRESULT
}}
impl IHttpCookiePairHeaderValue {
    #[inline] pub unsafe fn get_name(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Name)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_value(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Value)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_value(&self, value: &HStringArg) -> Result<()> {
        let hr = ((*self.lpVtbl).put_Value)(self as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
}
RT_CLASS!{class HttpCookiePairHeaderValue: IHttpCookiePairHeaderValue}
impl RtActivatable<IHttpCookiePairHeaderValueFactory> for HttpCookiePairHeaderValue {}
impl RtActivatable<IHttpCookiePairHeaderValueStatics> for HttpCookiePairHeaderValue {}
impl HttpCookiePairHeaderValue {
    #[inline] pub fn create_from_name(name: &HStringArg) -> Result<ComPtr<HttpCookiePairHeaderValue>> { unsafe {
        <Self as RtActivatable<IHttpCookiePairHeaderValueFactory>>::get_activation_factory().create_from_name(name)
    }}
    #[inline] pub fn create_from_name_with_value(name: &HStringArg, value: &HStringArg) -> Result<ComPtr<HttpCookiePairHeaderValue>> { unsafe {
        <Self as RtActivatable<IHttpCookiePairHeaderValueFactory>>::get_activation_factory().create_from_name_with_value(name, value)
    }}
    #[inline] pub fn parse(input: &HStringArg) -> Result<ComPtr<HttpCookiePairHeaderValue>> { unsafe {
        <Self as RtActivatable<IHttpCookiePairHeaderValueStatics>>::get_activation_factory().parse(input)
    }}
    #[inline] pub fn try_parse(input: &HStringArg) -> Result<(ComPtr<HttpCookiePairHeaderValue>, bool)> { unsafe {
        <Self as RtActivatable<IHttpCookiePairHeaderValueStatics>>::get_activation_factory().try_parse(input)
    }}
}
DEFINE_CLSID!(HttpCookiePairHeaderValue(&[87,105,110,100,111,119,115,46,87,101,98,46,72,116,116,112,46,72,101,97,100,101,114,115,46,72,116,116,112,67,111,111,107,105,101,80,97,105,114,72,101,97,100,101,114,86,97,108,117,101,0]) [CLSID_HttpCookiePairHeaderValue]);
DEFINE_IID!(IID_IHttpCookiePairHeaderValueCollection, 4092871504, 22558, 20172, 159, 89, 229, 7, 208, 79, 6, 230);
RT_INTERFACE!{interface IHttpCookiePairHeaderValueCollection(IHttpCookiePairHeaderValueCollectionVtbl): IInspectable(IInspectableVtbl) [IID_IHttpCookiePairHeaderValueCollection] {
    fn ParseAdd(&self, input: HSTRING) -> HRESULT,
    fn TryParseAdd(&self, input: HSTRING, out: *mut bool) -> HRESULT
}}
impl IHttpCookiePairHeaderValueCollection {
    #[inline] pub unsafe fn parse_add(&self, input: &HStringArg) -> Result<()> {
        let hr = ((*self.lpVtbl).ParseAdd)(self as *const _ as *mut _, input.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn try_parse_add(&self, input: &HStringArg) -> Result<bool> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).TryParseAdd)(self as *const _ as *mut _, input.get(), &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
}
RT_CLASS!{class HttpCookiePairHeaderValueCollection: IHttpCookiePairHeaderValueCollection}
DEFINE_IID!(IID_IHttpCookiePairHeaderValueFactory, 1667117679, 5231, 20310, 170, 33, 44, 183, 214, 213, 139, 30);
RT_INTERFACE!{static interface IHttpCookiePairHeaderValueFactory(IHttpCookiePairHeaderValueFactoryVtbl): IInspectable(IInspectableVtbl) [IID_IHttpCookiePairHeaderValueFactory] {
    fn CreateFromName(&self, name: HSTRING, out: *mut *mut HttpCookiePairHeaderValue) -> HRESULT,
    fn CreateFromNameWithValue(&self, name: HSTRING, value: HSTRING, out: *mut *mut HttpCookiePairHeaderValue) -> HRESULT
}}
impl IHttpCookiePairHeaderValueFactory {
    #[inline] pub unsafe fn create_from_name(&self, name: &HStringArg) -> Result<ComPtr<HttpCookiePairHeaderValue>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).CreateFromName)(self as *const _ as *mut _, name.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn create_from_name_with_value(&self, name: &HStringArg, value: &HStringArg) -> Result<ComPtr<HttpCookiePairHeaderValue>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).CreateFromNameWithValue)(self as *const _ as *mut _, name.get(), value.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
DEFINE_IID!(IID_IHttpCookiePairHeaderValueStatics, 1854303560, 1711, 17506, 129, 88, 153, 56, 141, 93, 202, 129);
RT_INTERFACE!{static interface IHttpCookiePairHeaderValueStatics(IHttpCookiePairHeaderValueStaticsVtbl): IInspectable(IInspectableVtbl) [IID_IHttpCookiePairHeaderValueStatics] {
    fn Parse(&self, input: HSTRING, out: *mut *mut HttpCookiePairHeaderValue) -> HRESULT,
    fn TryParse(&self, input: HSTRING, cookiePairHeaderValue: *mut *mut HttpCookiePairHeaderValue, out: *mut bool) -> HRESULT
}}
impl IHttpCookiePairHeaderValueStatics {
    #[inline] pub unsafe fn parse(&self, input: &HStringArg) -> Result<ComPtr<HttpCookiePairHeaderValue>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).Parse)(self as *const _ as *mut _, input.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn try_parse(&self, input: &HStringArg) -> Result<(ComPtr<HttpCookiePairHeaderValue>, bool)> {
        let mut cookiePairHeaderValue = null_mut(); let mut out = zeroed();
        let hr = ((*self.lpVtbl).TryParse)(self as *const _ as *mut _, input.get(), &mut cookiePairHeaderValue, &mut out);
        if hr == S_OK { Ok((ComPtr::wrap(cookiePairHeaderValue), out)) } else { err(hr) }
    }
}
DEFINE_IID!(IID_IHttpCredentialsHeaderValue, 3276587979, 21550, 16759, 166, 199, 182, 116, 206, 25, 63, 191);
RT_INTERFACE!{interface IHttpCredentialsHeaderValue(IHttpCredentialsHeaderValueVtbl): IInspectable(IInspectableVtbl) [IID_IHttpCredentialsHeaderValue] {
    fn get_Parameters(&self, out: *mut *mut ::rt::gen::windows::foundation::collections::IVector<HttpNameValueHeaderValue>) -> HRESULT,
    fn get_Scheme(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Token(&self, out: *mut HSTRING) -> HRESULT
}}
impl IHttpCredentialsHeaderValue {
    #[inline] pub unsafe fn get_parameters(&self) -> Result<ComPtr<::rt::gen::windows::foundation::collections::IVector<HttpNameValueHeaderValue>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Parameters)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_scheme(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Scheme)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_token(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Token)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
}
RT_CLASS!{class HttpCredentialsHeaderValue: IHttpCredentialsHeaderValue}
impl RtActivatable<IHttpCredentialsHeaderValueFactory> for HttpCredentialsHeaderValue {}
impl RtActivatable<IHttpCredentialsHeaderValueStatics> for HttpCredentialsHeaderValue {}
impl HttpCredentialsHeaderValue {
    #[inline] pub fn create_from_scheme(scheme: &HStringArg) -> Result<ComPtr<HttpCredentialsHeaderValue>> { unsafe {
        <Self as RtActivatable<IHttpCredentialsHeaderValueFactory>>::get_activation_factory().create_from_scheme(scheme)
    }}
    #[inline] pub fn create_from_scheme_with_token(scheme: &HStringArg, token: &HStringArg) -> Result<ComPtr<HttpCredentialsHeaderValue>> { unsafe {
        <Self as RtActivatable<IHttpCredentialsHeaderValueFactory>>::get_activation_factory().create_from_scheme_with_token(scheme, token)
    }}
    #[inline] pub fn parse(input: &HStringArg) -> Result<ComPtr<HttpCredentialsHeaderValue>> { unsafe {
        <Self as RtActivatable<IHttpCredentialsHeaderValueStatics>>::get_activation_factory().parse(input)
    }}
    #[inline] pub fn try_parse(input: &HStringArg) -> Result<(ComPtr<HttpCredentialsHeaderValue>, bool)> { unsafe {
        <Self as RtActivatable<IHttpCredentialsHeaderValueStatics>>::get_activation_factory().try_parse(input)
    }}
}
DEFINE_CLSID!(HttpCredentialsHeaderValue(&[87,105,110,100,111,119,115,46,87,101,98,46,72,116,116,112,46,72,101,97,100,101,114,115,46,72,116,116,112,67,114,101,100,101,110,116,105,97,108,115,72,101,97,100,101,114,86,97,108,117,101,0]) [CLSID_HttpCredentialsHeaderValue]);
DEFINE_IID!(IID_IHttpCredentialsHeaderValueFactory, 4062027409, 19740, 16770, 191, 209, 52, 71, 10, 98, 249, 80);
RT_INTERFACE!{static interface IHttpCredentialsHeaderValueFactory(IHttpCredentialsHeaderValueFactoryVtbl): IInspectable(IInspectableVtbl) [IID_IHttpCredentialsHeaderValueFactory] {
    fn CreateFromScheme(&self, scheme: HSTRING, out: *mut *mut HttpCredentialsHeaderValue) -> HRESULT,
    fn CreateFromSchemeWithToken(&self, scheme: HSTRING, token: HSTRING, out: *mut *mut HttpCredentialsHeaderValue) -> HRESULT
}}
impl IHttpCredentialsHeaderValueFactory {
    #[inline] pub unsafe fn create_from_scheme(&self, scheme: &HStringArg) -> Result<ComPtr<HttpCredentialsHeaderValue>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).CreateFromScheme)(self as *const _ as *mut _, scheme.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn create_from_scheme_with_token(&self, scheme: &HStringArg, token: &HStringArg) -> Result<ComPtr<HttpCredentialsHeaderValue>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).CreateFromSchemeWithToken)(self as *const _ as *mut _, scheme.get(), token.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
DEFINE_IID!(IID_IHttpCredentialsHeaderValueStatics, 2795187174, 52876, 17475, 163, 90, 27, 114, 123, 19, 16, 54);
RT_INTERFACE!{static interface IHttpCredentialsHeaderValueStatics(IHttpCredentialsHeaderValueStaticsVtbl): IInspectable(IInspectableVtbl) [IID_IHttpCredentialsHeaderValueStatics] {
    fn Parse(&self, input: HSTRING, out: *mut *mut HttpCredentialsHeaderValue) -> HRESULT,
    fn TryParse(&self, input: HSTRING, credentialsHeaderValue: *mut *mut HttpCredentialsHeaderValue, out: *mut bool) -> HRESULT
}}
impl IHttpCredentialsHeaderValueStatics {
    #[inline] pub unsafe fn parse(&self, input: &HStringArg) -> Result<ComPtr<HttpCredentialsHeaderValue>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).Parse)(self as *const _ as *mut _, input.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn try_parse(&self, input: &HStringArg) -> Result<(ComPtr<HttpCredentialsHeaderValue>, bool)> {
        let mut credentialsHeaderValue = null_mut(); let mut out = zeroed();
        let hr = ((*self.lpVtbl).TryParse)(self as *const _ as *mut _, input.get(), &mut credentialsHeaderValue, &mut out);
        if hr == S_OK { Ok((ComPtr::wrap(credentialsHeaderValue), out)) } else { err(hr) }
    }
}
DEFINE_IID!(IID_IHttpDateOrDeltaHeaderValue, 3942427242, 50396, 18914, 162, 125, 4, 58, 223, 88, 103, 163);
RT_INTERFACE!{interface IHttpDateOrDeltaHeaderValue(IHttpDateOrDeltaHeaderValueVtbl): IInspectable(IInspectableVtbl) [IID_IHttpDateOrDeltaHeaderValue] {
    fn get_Date(&self, out: *mut *mut ::rt::gen::windows::foundation::IReference<::rt::gen::windows::foundation::DateTime>) -> HRESULT,
    fn get_Delta(&self, out: *mut *mut ::rt::gen::windows::foundation::IReference<::rt::gen::windows::foundation::TimeSpan>) -> HRESULT
}}
impl IHttpDateOrDeltaHeaderValue {
    #[inline] pub unsafe fn get_date(&self) -> Result<ComPtr<::rt::gen::windows::foundation::IReference<::rt::gen::windows::foundation::DateTime>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Date)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_delta(&self) -> Result<ComPtr<::rt::gen::windows::foundation::IReference<::rt::gen::windows::foundation::TimeSpan>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Delta)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
RT_CLASS!{class HttpDateOrDeltaHeaderValue: IHttpDateOrDeltaHeaderValue}
impl RtActivatable<IHttpDateOrDeltaHeaderValueStatics> for HttpDateOrDeltaHeaderValue {}
impl HttpDateOrDeltaHeaderValue {
    #[inline] pub fn parse(input: &HStringArg) -> Result<ComPtr<HttpDateOrDeltaHeaderValue>> { unsafe {
        <Self as RtActivatable<IHttpDateOrDeltaHeaderValueStatics>>::get_activation_factory().parse(input)
    }}
    #[inline] pub fn try_parse(input: &HStringArg) -> Result<(ComPtr<HttpDateOrDeltaHeaderValue>, bool)> { unsafe {
        <Self as RtActivatable<IHttpDateOrDeltaHeaderValueStatics>>::get_activation_factory().try_parse(input)
    }}
}
DEFINE_CLSID!(HttpDateOrDeltaHeaderValue(&[87,105,110,100,111,119,115,46,87,101,98,46,72,116,116,112,46,72,101,97,100,101,114,115,46,72,116,116,112,68,97,116,101,79,114,68,101,108,116,97,72,101,97,100,101,114,86,97,108,117,101,0]) [CLSID_HttpDateOrDeltaHeaderValue]);
DEFINE_IID!(IID_IHttpDateOrDeltaHeaderValueStatics, 2082888104, 26226, 20112, 154, 154, 243, 151, 102, 247, 245, 118);
RT_INTERFACE!{static interface IHttpDateOrDeltaHeaderValueStatics(IHttpDateOrDeltaHeaderValueStaticsVtbl): IInspectable(IInspectableVtbl) [IID_IHttpDateOrDeltaHeaderValueStatics] {
    fn Parse(&self, input: HSTRING, out: *mut *mut HttpDateOrDeltaHeaderValue) -> HRESULT,
    fn TryParse(&self, input: HSTRING, dateOrDeltaHeaderValue: *mut *mut HttpDateOrDeltaHeaderValue, out: *mut bool) -> HRESULT
}}
impl IHttpDateOrDeltaHeaderValueStatics {
    #[inline] pub unsafe fn parse(&self, input: &HStringArg) -> Result<ComPtr<HttpDateOrDeltaHeaderValue>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).Parse)(self as *const _ as *mut _, input.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn try_parse(&self, input: &HStringArg) -> Result<(ComPtr<HttpDateOrDeltaHeaderValue>, bool)> {
        let mut dateOrDeltaHeaderValue = null_mut(); let mut out = zeroed();
        let hr = ((*self.lpVtbl).TryParse)(self as *const _ as *mut _, input.get(), &mut dateOrDeltaHeaderValue, &mut out);
        if hr == S_OK { Ok((ComPtr::wrap(dateOrDeltaHeaderValue), out)) } else { err(hr) }
    }
}
DEFINE_IID!(IID_IHttpExpectationHeaderValue, 1290110413, 15001, 17327, 162, 230, 236, 35, 47, 234, 150, 88);
RT_INTERFACE!{interface IHttpExpectationHeaderValue(IHttpExpectationHeaderValueVtbl): IInspectable(IInspectableVtbl) [IID_IHttpExpectationHeaderValue] {
    fn get_Name(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Value(&self, out: *mut HSTRING) -> HRESULT,
    fn put_Value(&self, value: HSTRING) -> HRESULT,
    fn get_Parameters(&self, out: *mut *mut ::rt::gen::windows::foundation::collections::IVector<HttpNameValueHeaderValue>) -> HRESULT
}}
impl IHttpExpectationHeaderValue {
    #[inline] pub unsafe fn get_name(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Name)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_value(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Value)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_value(&self, value: &HStringArg) -> Result<()> {
        let hr = ((*self.lpVtbl).put_Value)(self as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_parameters(&self) -> Result<ComPtr<::rt::gen::windows::foundation::collections::IVector<HttpNameValueHeaderValue>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Parameters)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
RT_CLASS!{class HttpExpectationHeaderValue: IHttpExpectationHeaderValue}
impl RtActivatable<IHttpExpectationHeaderValueFactory> for HttpExpectationHeaderValue {}
impl RtActivatable<IHttpExpectationHeaderValueStatics> for HttpExpectationHeaderValue {}
impl HttpExpectationHeaderValue {
    #[inline] pub fn create_from_name(name: &HStringArg) -> Result<ComPtr<HttpExpectationHeaderValue>> { unsafe {
        <Self as RtActivatable<IHttpExpectationHeaderValueFactory>>::get_activation_factory().create_from_name(name)
    }}
    #[inline] pub fn create_from_name_with_value(name: &HStringArg, value: &HStringArg) -> Result<ComPtr<HttpExpectationHeaderValue>> { unsafe {
        <Self as RtActivatable<IHttpExpectationHeaderValueFactory>>::get_activation_factory().create_from_name_with_value(name, value)
    }}
    #[inline] pub fn parse(input: &HStringArg) -> Result<ComPtr<HttpExpectationHeaderValue>> { unsafe {
        <Self as RtActivatable<IHttpExpectationHeaderValueStatics>>::get_activation_factory().parse(input)
    }}
    #[inline] pub fn try_parse(input: &HStringArg) -> Result<(ComPtr<HttpExpectationHeaderValue>, bool)> { unsafe {
        <Self as RtActivatable<IHttpExpectationHeaderValueStatics>>::get_activation_factory().try_parse(input)
    }}
}
DEFINE_CLSID!(HttpExpectationHeaderValue(&[87,105,110,100,111,119,115,46,87,101,98,46,72,116,116,112,46,72,101,97,100,101,114,115,46,72,116,116,112,69,120,112,101,99,116,97,116,105,111,110,72,101,97,100,101,114,86,97,108,117,101,0]) [CLSID_HttpExpectationHeaderValue]);
DEFINE_IID!(IID_IHttpExpectationHeaderValueCollection, 3884261811, 41186, 19140, 158, 102, 121, 112, 108, 185, 253, 88);
RT_INTERFACE!{interface IHttpExpectationHeaderValueCollection(IHttpExpectationHeaderValueCollectionVtbl): IInspectable(IInspectableVtbl) [IID_IHttpExpectationHeaderValueCollection] {
    fn ParseAdd(&self, input: HSTRING) -> HRESULT,
    fn TryParseAdd(&self, input: HSTRING, out: *mut bool) -> HRESULT
}}
impl IHttpExpectationHeaderValueCollection {
    #[inline] pub unsafe fn parse_add(&self, input: &HStringArg) -> Result<()> {
        let hr = ((*self.lpVtbl).ParseAdd)(self as *const _ as *mut _, input.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn try_parse_add(&self, input: &HStringArg) -> Result<bool> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).TryParseAdd)(self as *const _ as *mut _, input.get(), &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
}
RT_CLASS!{class HttpExpectationHeaderValueCollection: IHttpExpectationHeaderValueCollection}
DEFINE_IID!(IID_IHttpExpectationHeaderValueFactory, 1319269835, 54590, 18536, 136, 86, 30, 33, 165, 3, 13, 192);
RT_INTERFACE!{static interface IHttpExpectationHeaderValueFactory(IHttpExpectationHeaderValueFactoryVtbl): IInspectable(IInspectableVtbl) [IID_IHttpExpectationHeaderValueFactory] {
    fn CreateFromName(&self, name: HSTRING, out: *mut *mut HttpExpectationHeaderValue) -> HRESULT,
    fn CreateFromNameWithValue(&self, name: HSTRING, value: HSTRING, out: *mut *mut HttpExpectationHeaderValue) -> HRESULT
}}
impl IHttpExpectationHeaderValueFactory {
    #[inline] pub unsafe fn create_from_name(&self, name: &HStringArg) -> Result<ComPtr<HttpExpectationHeaderValue>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).CreateFromName)(self as *const _ as *mut _, name.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn create_from_name_with_value(&self, name: &HStringArg, value: &HStringArg) -> Result<ComPtr<HttpExpectationHeaderValue>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).CreateFromNameWithValue)(self as *const _ as *mut _, name.get(), value.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
DEFINE_IID!(IID_IHttpExpectationHeaderValueStatics, 806988770, 53221, 18235, 165, 127, 251, 165, 177, 78, 178, 87);
RT_INTERFACE!{static interface IHttpExpectationHeaderValueStatics(IHttpExpectationHeaderValueStaticsVtbl): IInspectable(IInspectableVtbl) [IID_IHttpExpectationHeaderValueStatics] {
    fn Parse(&self, input: HSTRING, out: *mut *mut HttpExpectationHeaderValue) -> HRESULT,
    fn TryParse(&self, input: HSTRING, expectationHeaderValue: *mut *mut HttpExpectationHeaderValue, out: *mut bool) -> HRESULT
}}
impl IHttpExpectationHeaderValueStatics {
    #[inline] pub unsafe fn parse(&self, input: &HStringArg) -> Result<ComPtr<HttpExpectationHeaderValue>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).Parse)(self as *const _ as *mut _, input.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn try_parse(&self, input: &HStringArg) -> Result<(ComPtr<HttpExpectationHeaderValue>, bool)> {
        let mut expectationHeaderValue = null_mut(); let mut out = zeroed();
        let hr = ((*self.lpVtbl).TryParse)(self as *const _ as *mut _, input.get(), &mut expectationHeaderValue, &mut out);
        if hr == S_OK { Ok((ComPtr::wrap(expectationHeaderValue), out)) } else { err(hr) }
    }
}
DEFINE_IID!(IID_IHttpLanguageHeaderValueCollection, 2663218339, 33305, 17654, 153, 2, 140, 86, 223, 211, 52, 12);
RT_INTERFACE!{interface IHttpLanguageHeaderValueCollection(IHttpLanguageHeaderValueCollectionVtbl): IInspectable(IInspectableVtbl) [IID_IHttpLanguageHeaderValueCollection] {
    fn ParseAdd(&self, input: HSTRING) -> HRESULT,
    fn TryParseAdd(&self, input: HSTRING, out: *mut bool) -> HRESULT
}}
impl IHttpLanguageHeaderValueCollection {
    #[inline] pub unsafe fn parse_add(&self, input: &HStringArg) -> Result<()> {
        let hr = ((*self.lpVtbl).ParseAdd)(self as *const _ as *mut _, input.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn try_parse_add(&self, input: &HStringArg) -> Result<bool> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).TryParseAdd)(self as *const _ as *mut _, input.get(), &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
}
RT_CLASS!{class HttpLanguageHeaderValueCollection: IHttpLanguageHeaderValueCollection}
DEFINE_IID!(IID_IHttpLanguageRangeWithQualityHeaderValue, 1918296322, 128, 19892, 160, 131, 125, 231, 178, 229, 186, 76);
RT_INTERFACE!{interface IHttpLanguageRangeWithQualityHeaderValue(IHttpLanguageRangeWithQualityHeaderValueVtbl): IInspectable(IInspectableVtbl) [IID_IHttpLanguageRangeWithQualityHeaderValue] {
    fn get_LanguageRange(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Quality(&self, out: *mut *mut ::rt::gen::windows::foundation::IReference<f64>) -> HRESULT
}}
impl IHttpLanguageRangeWithQualityHeaderValue {
    #[inline] pub unsafe fn get_language_range(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_LanguageRange)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_quality(&self) -> Result<ComPtr<::rt::gen::windows::foundation::IReference<f64>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Quality)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
RT_CLASS!{class HttpLanguageRangeWithQualityHeaderValue: IHttpLanguageRangeWithQualityHeaderValue}
impl RtActivatable<IHttpLanguageRangeWithQualityHeaderValueFactory> for HttpLanguageRangeWithQualityHeaderValue {}
impl RtActivatable<IHttpLanguageRangeWithQualityHeaderValueStatics> for HttpLanguageRangeWithQualityHeaderValue {}
impl HttpLanguageRangeWithQualityHeaderValue {
    #[inline] pub fn create_from_language_range(languageRange: &HStringArg) -> Result<ComPtr<HttpLanguageRangeWithQualityHeaderValue>> { unsafe {
        <Self as RtActivatable<IHttpLanguageRangeWithQualityHeaderValueFactory>>::get_activation_factory().create_from_language_range(languageRange)
    }}
    #[inline] pub fn create_from_language_range_with_quality(languageRange: &HStringArg, quality: f64) -> Result<ComPtr<HttpLanguageRangeWithQualityHeaderValue>> { unsafe {
        <Self as RtActivatable<IHttpLanguageRangeWithQualityHeaderValueFactory>>::get_activation_factory().create_from_language_range_with_quality(languageRange, quality)
    }}
    #[inline] pub fn parse(input: &HStringArg) -> Result<ComPtr<HttpLanguageRangeWithQualityHeaderValue>> { unsafe {
        <Self as RtActivatable<IHttpLanguageRangeWithQualityHeaderValueStatics>>::get_activation_factory().parse(input)
    }}
    #[inline] pub fn try_parse(input: &HStringArg) -> Result<(ComPtr<HttpLanguageRangeWithQualityHeaderValue>, bool)> { unsafe {
        <Self as RtActivatable<IHttpLanguageRangeWithQualityHeaderValueStatics>>::get_activation_factory().try_parse(input)
    }}
}
DEFINE_CLSID!(HttpLanguageRangeWithQualityHeaderValue(&[87,105,110,100,111,119,115,46,87,101,98,46,72,116,116,112,46,72,101,97,100,101,114,115,46,72,116,116,112,76,97,110,103,117,97,103,101,82,97,110,103,101,87,105,116,104,81,117,97,108,105,116,121,72,101,97,100,101,114,86,97,108,117,101,0]) [CLSID_HttpLanguageRangeWithQualityHeaderValue]);
DEFINE_IID!(IID_IHttpLanguageRangeWithQualityHeaderValueCollection, 2287819453, 19279, 18442, 137, 206, 138, 237, 206, 230, 227, 160);
RT_INTERFACE!{interface IHttpLanguageRangeWithQualityHeaderValueCollection(IHttpLanguageRangeWithQualityHeaderValueCollectionVtbl): IInspectable(IInspectableVtbl) [IID_IHttpLanguageRangeWithQualityHeaderValueCollection] {
    fn ParseAdd(&self, input: HSTRING) -> HRESULT,
    fn TryParseAdd(&self, input: HSTRING, out: *mut bool) -> HRESULT
}}
impl IHttpLanguageRangeWithQualityHeaderValueCollection {
    #[inline] pub unsafe fn parse_add(&self, input: &HStringArg) -> Result<()> {
        let hr = ((*self.lpVtbl).ParseAdd)(self as *const _ as *mut _, input.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn try_parse_add(&self, input: &HStringArg) -> Result<bool> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).TryParseAdd)(self as *const _ as *mut _, input.get(), &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
}
RT_CLASS!{class HttpLanguageRangeWithQualityHeaderValueCollection: IHttpLanguageRangeWithQualityHeaderValueCollection}
DEFINE_IID!(IID_IHttpLanguageRangeWithQualityHeaderValueFactory, 2075670896, 30735, 19587, 159, 228, 220, 48, 135, 246, 189, 85);
RT_INTERFACE!{static interface IHttpLanguageRangeWithQualityHeaderValueFactory(IHttpLanguageRangeWithQualityHeaderValueFactoryVtbl): IInspectable(IInspectableVtbl) [IID_IHttpLanguageRangeWithQualityHeaderValueFactory] {
    fn CreateFromLanguageRange(&self, languageRange: HSTRING, out: *mut *mut HttpLanguageRangeWithQualityHeaderValue) -> HRESULT,
    fn CreateFromLanguageRangeWithQuality(&self, languageRange: HSTRING, quality: f64, out: *mut *mut HttpLanguageRangeWithQualityHeaderValue) -> HRESULT
}}
impl IHttpLanguageRangeWithQualityHeaderValueFactory {
    #[inline] pub unsafe fn create_from_language_range(&self, languageRange: &HStringArg) -> Result<ComPtr<HttpLanguageRangeWithQualityHeaderValue>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).CreateFromLanguageRange)(self as *const _ as *mut _, languageRange.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn create_from_language_range_with_quality(&self, languageRange: &HStringArg, quality: f64) -> Result<ComPtr<HttpLanguageRangeWithQualityHeaderValue>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).CreateFromLanguageRangeWithQuality)(self as *const _ as *mut _, languageRange.get(), quality, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
DEFINE_IID!(IID_IHttpLanguageRangeWithQualityHeaderValueStatics, 625074502, 62216, 18165, 182, 149, 66, 245, 64, 36, 236, 104);
RT_INTERFACE!{static interface IHttpLanguageRangeWithQualityHeaderValueStatics(IHttpLanguageRangeWithQualityHeaderValueStaticsVtbl): IInspectable(IInspectableVtbl) [IID_IHttpLanguageRangeWithQualityHeaderValueStatics] {
    fn Parse(&self, input: HSTRING, out: *mut *mut HttpLanguageRangeWithQualityHeaderValue) -> HRESULT,
    fn TryParse(&self, input: HSTRING, languageRangeWithQualityHeaderValue: *mut *mut HttpLanguageRangeWithQualityHeaderValue, out: *mut bool) -> HRESULT
}}
impl IHttpLanguageRangeWithQualityHeaderValueStatics {
    #[inline] pub unsafe fn parse(&self, input: &HStringArg) -> Result<ComPtr<HttpLanguageRangeWithQualityHeaderValue>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).Parse)(self as *const _ as *mut _, input.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn try_parse(&self, input: &HStringArg) -> Result<(ComPtr<HttpLanguageRangeWithQualityHeaderValue>, bool)> {
        let mut languageRangeWithQualityHeaderValue = null_mut(); let mut out = zeroed();
        let hr = ((*self.lpVtbl).TryParse)(self as *const _ as *mut _, input.get(), &mut languageRangeWithQualityHeaderValue, &mut out);
        if hr == S_OK { Ok((ComPtr::wrap(languageRangeWithQualityHeaderValue), out)) } else { err(hr) }
    }
}
DEFINE_IID!(IID_IHttpMediaTypeHeaderValue, 380798259, 59176, 20427, 189, 176, 8, 164, 49, 161, 72, 68);
RT_INTERFACE!{interface IHttpMediaTypeHeaderValue(IHttpMediaTypeHeaderValueVtbl): IInspectable(IInspectableVtbl) [IID_IHttpMediaTypeHeaderValue] {
    fn get_CharSet(&self, out: *mut HSTRING) -> HRESULT,
    fn put_CharSet(&self, value: HSTRING) -> HRESULT,
    fn get_MediaType(&self, out: *mut HSTRING) -> HRESULT,
    fn put_MediaType(&self, value: HSTRING) -> HRESULT,
    fn get_Parameters(&self, out: *mut *mut ::rt::gen::windows::foundation::collections::IVector<HttpNameValueHeaderValue>) -> HRESULT
}}
impl IHttpMediaTypeHeaderValue {
    #[inline] pub unsafe fn get_char_set(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_CharSet)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_char_set(&self, value: &HStringArg) -> Result<()> {
        let hr = ((*self.lpVtbl).put_CharSet)(self as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_media_type(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_MediaType)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_media_type(&self, value: &HStringArg) -> Result<()> {
        let hr = ((*self.lpVtbl).put_MediaType)(self as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_parameters(&self) -> Result<ComPtr<::rt::gen::windows::foundation::collections::IVector<HttpNameValueHeaderValue>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Parameters)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
RT_CLASS!{class HttpMediaTypeHeaderValue: IHttpMediaTypeHeaderValue}
impl RtActivatable<IHttpMediaTypeHeaderValueFactory> for HttpMediaTypeHeaderValue {}
impl RtActivatable<IHttpMediaTypeHeaderValueStatics> for HttpMediaTypeHeaderValue {}
impl HttpMediaTypeHeaderValue {
    #[inline] pub fn create(mediaType: &HStringArg) -> Result<ComPtr<HttpMediaTypeHeaderValue>> { unsafe {
        <Self as RtActivatable<IHttpMediaTypeHeaderValueFactory>>::get_activation_factory().create(mediaType)
    }}
    #[inline] pub fn parse(input: &HStringArg) -> Result<ComPtr<HttpMediaTypeHeaderValue>> { unsafe {
        <Self as RtActivatable<IHttpMediaTypeHeaderValueStatics>>::get_activation_factory().parse(input)
    }}
    #[inline] pub fn try_parse(input: &HStringArg) -> Result<(ComPtr<HttpMediaTypeHeaderValue>, bool)> { unsafe {
        <Self as RtActivatable<IHttpMediaTypeHeaderValueStatics>>::get_activation_factory().try_parse(input)
    }}
}
DEFINE_CLSID!(HttpMediaTypeHeaderValue(&[87,105,110,100,111,119,115,46,87,101,98,46,72,116,116,112,46,72,101,97,100,101,114,115,46,72,116,116,112,77,101,100,105,97,84,121,112,101,72,101,97,100,101,114,86,97,108,117,101,0]) [CLSID_HttpMediaTypeHeaderValue]);
DEFINE_IID!(IID_IHttpMediaTypeHeaderValueFactory, 3201779624, 52503, 17117, 147, 103, 171, 156, 91, 86, 221, 125);
RT_INTERFACE!{static interface IHttpMediaTypeHeaderValueFactory(IHttpMediaTypeHeaderValueFactoryVtbl): IInspectable(IInspectableVtbl) [IID_IHttpMediaTypeHeaderValueFactory] {
    fn Create(&self, mediaType: HSTRING, out: *mut *mut HttpMediaTypeHeaderValue) -> HRESULT
}}
impl IHttpMediaTypeHeaderValueFactory {
    #[inline] pub unsafe fn create(&self, mediaType: &HStringArg) -> Result<ComPtr<HttpMediaTypeHeaderValue>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).Create)(self as *const _ as *mut _, mediaType.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
DEFINE_IID!(IID_IHttpMediaTypeHeaderValueStatics, 3763176415, 7489, 19852, 162, 222, 111, 210, 237, 135, 57, 155);
RT_INTERFACE!{static interface IHttpMediaTypeHeaderValueStatics(IHttpMediaTypeHeaderValueStaticsVtbl): IInspectable(IInspectableVtbl) [IID_IHttpMediaTypeHeaderValueStatics] {
    fn Parse(&self, input: HSTRING, out: *mut *mut HttpMediaTypeHeaderValue) -> HRESULT,
    fn TryParse(&self, input: HSTRING, mediaTypeHeaderValue: *mut *mut HttpMediaTypeHeaderValue, out: *mut bool) -> HRESULT
}}
impl IHttpMediaTypeHeaderValueStatics {
    #[inline] pub unsafe fn parse(&self, input: &HStringArg) -> Result<ComPtr<HttpMediaTypeHeaderValue>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).Parse)(self as *const _ as *mut _, input.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn try_parse(&self, input: &HStringArg) -> Result<(ComPtr<HttpMediaTypeHeaderValue>, bool)> {
        let mut mediaTypeHeaderValue = null_mut(); let mut out = zeroed();
        let hr = ((*self.lpVtbl).TryParse)(self as *const _ as *mut _, input.get(), &mut mediaTypeHeaderValue, &mut out);
        if hr == S_OK { Ok((ComPtr::wrap(mediaTypeHeaderValue), out)) } else { err(hr) }
    }
}
DEFINE_IID!(IID_IHttpMediaTypeWithQualityHeaderValue, 411917874, 30398, 17568, 177, 205, 32, 116, 189, 237, 45, 222);
RT_INTERFACE!{interface IHttpMediaTypeWithQualityHeaderValue(IHttpMediaTypeWithQualityHeaderValueVtbl): IInspectable(IInspectableVtbl) [IID_IHttpMediaTypeWithQualityHeaderValue] {
    fn get_CharSet(&self, out: *mut HSTRING) -> HRESULT,
    fn put_CharSet(&self, value: HSTRING) -> HRESULT,
    fn get_MediaType(&self, out: *mut HSTRING) -> HRESULT,
    fn put_MediaType(&self, value: HSTRING) -> HRESULT,
    fn get_Parameters(&self, out: *mut *mut ::rt::gen::windows::foundation::collections::IVector<HttpNameValueHeaderValue>) -> HRESULT,
    fn get_Quality(&self, out: *mut *mut ::rt::gen::windows::foundation::IReference<f64>) -> HRESULT,
    fn put_Quality(&self, value: *mut ::rt::gen::windows::foundation::IReference<f64>) -> HRESULT
}}
impl IHttpMediaTypeWithQualityHeaderValue {
    #[inline] pub unsafe fn get_char_set(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_CharSet)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_char_set(&self, value: &HStringArg) -> Result<()> {
        let hr = ((*self.lpVtbl).put_CharSet)(self as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_media_type(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_MediaType)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_media_type(&self, value: &HStringArg) -> Result<()> {
        let hr = ((*self.lpVtbl).put_MediaType)(self as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_parameters(&self) -> Result<ComPtr<::rt::gen::windows::foundation::collections::IVector<HttpNameValueHeaderValue>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Parameters)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_quality(&self) -> Result<ComPtr<::rt::gen::windows::foundation::IReference<f64>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Quality)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_quality(&self, value: &::rt::gen::windows::foundation::IReference<f64>) -> Result<()> {
        let hr = ((*self.lpVtbl).put_Quality)(self as *const _ as *mut _, value as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
}
RT_CLASS!{class HttpMediaTypeWithQualityHeaderValue: IHttpMediaTypeWithQualityHeaderValue}
impl RtActivatable<IHttpMediaTypeWithQualityHeaderValueFactory> for HttpMediaTypeWithQualityHeaderValue {}
impl RtActivatable<IHttpMediaTypeWithQualityHeaderValueStatics> for HttpMediaTypeWithQualityHeaderValue {}
impl HttpMediaTypeWithQualityHeaderValue {
    #[inline] pub fn create_from_media_type(mediaType: &HStringArg) -> Result<ComPtr<HttpMediaTypeWithQualityHeaderValue>> { unsafe {
        <Self as RtActivatable<IHttpMediaTypeWithQualityHeaderValueFactory>>::get_activation_factory().create_from_media_type(mediaType)
    }}
    #[inline] pub fn create_from_media_type_with_quality(mediaType: &HStringArg, quality: f64) -> Result<ComPtr<HttpMediaTypeWithQualityHeaderValue>> { unsafe {
        <Self as RtActivatable<IHttpMediaTypeWithQualityHeaderValueFactory>>::get_activation_factory().create_from_media_type_with_quality(mediaType, quality)
    }}
    #[inline] pub fn parse(input: &HStringArg) -> Result<ComPtr<HttpMediaTypeWithQualityHeaderValue>> { unsafe {
        <Self as RtActivatable<IHttpMediaTypeWithQualityHeaderValueStatics>>::get_activation_factory().parse(input)
    }}
    #[inline] pub fn try_parse(input: &HStringArg) -> Result<(ComPtr<HttpMediaTypeWithQualityHeaderValue>, bool)> { unsafe {
        <Self as RtActivatable<IHttpMediaTypeWithQualityHeaderValueStatics>>::get_activation_factory().try_parse(input)
    }}
}
DEFINE_CLSID!(HttpMediaTypeWithQualityHeaderValue(&[87,105,110,100,111,119,115,46,87,101,98,46,72,116,116,112,46,72,101,97,100,101,114,115,46,72,116,116,112,77,101,100,105,97,84,121,112,101,87,105,116,104,81,117,97,108,105,116,121,72,101,97,100,101,114,86,97,108,117,101,0]) [CLSID_HttpMediaTypeWithQualityHeaderValue]);
DEFINE_IID!(IID_IHttpMediaTypeWithQualityHeaderValueCollection, 1007446899, 4930, 17799, 160, 86, 24, 208, 47, 246, 113, 101);
RT_INTERFACE!{interface IHttpMediaTypeWithQualityHeaderValueCollection(IHttpMediaTypeWithQualityHeaderValueCollectionVtbl): IInspectable(IInspectableVtbl) [IID_IHttpMediaTypeWithQualityHeaderValueCollection] {
    fn ParseAdd(&self, input: HSTRING) -> HRESULT,
    fn TryParseAdd(&self, input: HSTRING, out: *mut bool) -> HRESULT
}}
impl IHttpMediaTypeWithQualityHeaderValueCollection {
    #[inline] pub unsafe fn parse_add(&self, input: &HStringArg) -> Result<()> {
        let hr = ((*self.lpVtbl).ParseAdd)(self as *const _ as *mut _, input.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn try_parse_add(&self, input: &HStringArg) -> Result<bool> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).TryParseAdd)(self as *const _ as *mut _, input.get(), &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
}
RT_CLASS!{class HttpMediaTypeWithQualityHeaderValueCollection: IHttpMediaTypeWithQualityHeaderValueCollection}
DEFINE_IID!(IID_IHttpMediaTypeWithQualityHeaderValueFactory, 1282220276, 37975, 17638, 163, 35, 209, 34, 185, 88, 120, 11);
RT_INTERFACE!{static interface IHttpMediaTypeWithQualityHeaderValueFactory(IHttpMediaTypeWithQualityHeaderValueFactoryVtbl): IInspectable(IInspectableVtbl) [IID_IHttpMediaTypeWithQualityHeaderValueFactory] {
    fn CreateFromMediaType(&self, mediaType: HSTRING, out: *mut *mut HttpMediaTypeWithQualityHeaderValue) -> HRESULT,
    fn CreateFromMediaTypeWithQuality(&self, mediaType: HSTRING, quality: f64, out: *mut *mut HttpMediaTypeWithQualityHeaderValue) -> HRESULT
}}
impl IHttpMediaTypeWithQualityHeaderValueFactory {
    #[inline] pub unsafe fn create_from_media_type(&self, mediaType: &HStringArg) -> Result<ComPtr<HttpMediaTypeWithQualityHeaderValue>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).CreateFromMediaType)(self as *const _ as *mut _, mediaType.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn create_from_media_type_with_quality(&self, mediaType: &HStringArg, quality: f64) -> Result<ComPtr<HttpMediaTypeWithQualityHeaderValue>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).CreateFromMediaTypeWithQuality)(self as *const _ as *mut _, mediaType.get(), quality, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
DEFINE_IID!(IID_IHttpMediaTypeWithQualityHeaderValueStatics, 1527188697, 46432, 20424, 152, 53, 126, 108, 10, 101, 123, 36);
RT_INTERFACE!{static interface IHttpMediaTypeWithQualityHeaderValueStatics(IHttpMediaTypeWithQualityHeaderValueStaticsVtbl): IInspectable(IInspectableVtbl) [IID_IHttpMediaTypeWithQualityHeaderValueStatics] {
    fn Parse(&self, input: HSTRING, out: *mut *mut HttpMediaTypeWithQualityHeaderValue) -> HRESULT,
    fn TryParse(&self, input: HSTRING, mediaTypeWithQualityHeaderValue: *mut *mut HttpMediaTypeWithQualityHeaderValue, out: *mut bool) -> HRESULT
}}
impl IHttpMediaTypeWithQualityHeaderValueStatics {
    #[inline] pub unsafe fn parse(&self, input: &HStringArg) -> Result<ComPtr<HttpMediaTypeWithQualityHeaderValue>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).Parse)(self as *const _ as *mut _, input.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn try_parse(&self, input: &HStringArg) -> Result<(ComPtr<HttpMediaTypeWithQualityHeaderValue>, bool)> {
        let mut mediaTypeWithQualityHeaderValue = null_mut(); let mut out = zeroed();
        let hr = ((*self.lpVtbl).TryParse)(self as *const _ as *mut _, input.get(), &mut mediaTypeWithQualityHeaderValue, &mut out);
        if hr == S_OK { Ok((ComPtr::wrap(mediaTypeWithQualityHeaderValue), out)) } else { err(hr) }
    }
}
DEFINE_IID!(IID_IHttpMethodHeaderValueCollection, 1136410612, 24857, 19167, 147, 140, 52, 191, 255, 207, 146, 237);
RT_INTERFACE!{interface IHttpMethodHeaderValueCollection(IHttpMethodHeaderValueCollectionVtbl): IInspectable(IInspectableVtbl) [IID_IHttpMethodHeaderValueCollection] {
    fn ParseAdd(&self, input: HSTRING) -> HRESULT,
    fn TryParseAdd(&self, input: HSTRING, out: *mut bool) -> HRESULT
}}
impl IHttpMethodHeaderValueCollection {
    #[inline] pub unsafe fn parse_add(&self, input: &HStringArg) -> Result<()> {
        let hr = ((*self.lpVtbl).ParseAdd)(self as *const _ as *mut _, input.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn try_parse_add(&self, input: &HStringArg) -> Result<bool> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).TryParseAdd)(self as *const _ as *mut _, input.get(), &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
}
RT_CLASS!{class HttpMethodHeaderValueCollection: IHttpMethodHeaderValueCollection}
DEFINE_IID!(IID_IHttpNameValueHeaderValue, 3636098147, 23450, 19739, 147, 249, 170, 91, 68, 236, 253, 223);
RT_INTERFACE!{interface IHttpNameValueHeaderValue(IHttpNameValueHeaderValueVtbl): IInspectable(IInspectableVtbl) [IID_IHttpNameValueHeaderValue] {
    fn get_Name(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Value(&self, out: *mut HSTRING) -> HRESULT,
    fn put_Value(&self, value: HSTRING) -> HRESULT
}}
impl IHttpNameValueHeaderValue {
    #[inline] pub unsafe fn get_name(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Name)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_value(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Value)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_value(&self, value: &HStringArg) -> Result<()> {
        let hr = ((*self.lpVtbl).put_Value)(self as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
}
RT_CLASS!{class HttpNameValueHeaderValue: IHttpNameValueHeaderValue}
impl RtActivatable<IHttpNameValueHeaderValueFactory> for HttpNameValueHeaderValue {}
impl RtActivatable<IHttpNameValueHeaderValueStatics> for HttpNameValueHeaderValue {}
impl HttpNameValueHeaderValue {
    #[inline] pub fn create_from_name(name: &HStringArg) -> Result<ComPtr<HttpNameValueHeaderValue>> { unsafe {
        <Self as RtActivatable<IHttpNameValueHeaderValueFactory>>::get_activation_factory().create_from_name(name)
    }}
    #[inline] pub fn create_from_name_with_value(name: &HStringArg, value: &HStringArg) -> Result<ComPtr<HttpNameValueHeaderValue>> { unsafe {
        <Self as RtActivatable<IHttpNameValueHeaderValueFactory>>::get_activation_factory().create_from_name_with_value(name, value)
    }}
    #[inline] pub fn parse(input: &HStringArg) -> Result<ComPtr<HttpNameValueHeaderValue>> { unsafe {
        <Self as RtActivatable<IHttpNameValueHeaderValueStatics>>::get_activation_factory().parse(input)
    }}
    #[inline] pub fn try_parse(input: &HStringArg) -> Result<(ComPtr<HttpNameValueHeaderValue>, bool)> { unsafe {
        <Self as RtActivatable<IHttpNameValueHeaderValueStatics>>::get_activation_factory().try_parse(input)
    }}
}
DEFINE_CLSID!(HttpNameValueHeaderValue(&[87,105,110,100,111,119,115,46,87,101,98,46,72,116,116,112,46,72,101,97,100,101,114,115,46,72,116,116,112,78,97,109,101,86,97,108,117,101,72,101,97,100,101,114,86,97,108,117,101,0]) [CLSID_HttpNameValueHeaderValue]);
DEFINE_IID!(IID_IHttpNameValueHeaderValueFactory, 1997415015, 52216, 18230, 169, 37, 147, 251, 225, 12, 124, 168);
RT_INTERFACE!{static interface IHttpNameValueHeaderValueFactory(IHttpNameValueHeaderValueFactoryVtbl): IInspectable(IInspectableVtbl) [IID_IHttpNameValueHeaderValueFactory] {
    fn CreateFromName(&self, name: HSTRING, out: *mut *mut HttpNameValueHeaderValue) -> HRESULT,
    fn CreateFromNameWithValue(&self, name: HSTRING, value: HSTRING, out: *mut *mut HttpNameValueHeaderValue) -> HRESULT
}}
impl IHttpNameValueHeaderValueFactory {
    #[inline] pub unsafe fn create_from_name(&self, name: &HStringArg) -> Result<ComPtr<HttpNameValueHeaderValue>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).CreateFromName)(self as *const _ as *mut _, name.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn create_from_name_with_value(&self, name: &HStringArg, value: &HStringArg) -> Result<ComPtr<HttpNameValueHeaderValue>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).CreateFromNameWithValue)(self as *const _ as *mut _, name.get(), value.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
DEFINE_IID!(IID_IHttpNameValueHeaderValueStatics, 4292084495, 4400, 16722, 134, 89, 37, 105, 9, 169, 209, 21);
RT_INTERFACE!{static interface IHttpNameValueHeaderValueStatics(IHttpNameValueHeaderValueStaticsVtbl): IInspectable(IInspectableVtbl) [IID_IHttpNameValueHeaderValueStatics] {
    fn Parse(&self, input: HSTRING, out: *mut *mut HttpNameValueHeaderValue) -> HRESULT,
    fn TryParse(&self, input: HSTRING, nameValueHeaderValue: *mut *mut HttpNameValueHeaderValue, out: *mut bool) -> HRESULT
}}
impl IHttpNameValueHeaderValueStatics {
    #[inline] pub unsafe fn parse(&self, input: &HStringArg) -> Result<ComPtr<HttpNameValueHeaderValue>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).Parse)(self as *const _ as *mut _, input.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn try_parse(&self, input: &HStringArg) -> Result<(ComPtr<HttpNameValueHeaderValue>, bool)> {
        let mut nameValueHeaderValue = null_mut(); let mut out = zeroed();
        let hr = ((*self.lpVtbl).TryParse)(self as *const _ as *mut _, input.get(), &mut nameValueHeaderValue, &mut out);
        if hr == S_OK { Ok((ComPtr::wrap(nameValueHeaderValue), out)) } else { err(hr) }
    }
}
DEFINE_IID!(IID_IHttpProductHeaderValue, 4110347779, 60372, 16736, 185, 255, 128, 124, 81, 131, 182, 230);
RT_INTERFACE!{interface IHttpProductHeaderValue(IHttpProductHeaderValueVtbl): IInspectable(IInspectableVtbl) [IID_IHttpProductHeaderValue] {
    fn get_Name(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Version(&self, out: *mut HSTRING) -> HRESULT
}}
impl IHttpProductHeaderValue {
    #[inline] pub unsafe fn get_name(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Name)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_version(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Version)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
}
RT_CLASS!{class HttpProductHeaderValue: IHttpProductHeaderValue}
impl RtActivatable<IHttpProductHeaderValueFactory> for HttpProductHeaderValue {}
impl RtActivatable<IHttpProductHeaderValueStatics> for HttpProductHeaderValue {}
impl HttpProductHeaderValue {
    #[inline] pub fn create_from_name(productName: &HStringArg) -> Result<ComPtr<HttpProductHeaderValue>> { unsafe {
        <Self as RtActivatable<IHttpProductHeaderValueFactory>>::get_activation_factory().create_from_name(productName)
    }}
    #[inline] pub fn create_from_name_with_version(productName: &HStringArg, productVersion: &HStringArg) -> Result<ComPtr<HttpProductHeaderValue>> { unsafe {
        <Self as RtActivatable<IHttpProductHeaderValueFactory>>::get_activation_factory().create_from_name_with_version(productName, productVersion)
    }}
    #[inline] pub fn parse(input: &HStringArg) -> Result<ComPtr<HttpProductHeaderValue>> { unsafe {
        <Self as RtActivatable<IHttpProductHeaderValueStatics>>::get_activation_factory().parse(input)
    }}
    #[inline] pub fn try_parse(input: &HStringArg) -> Result<(ComPtr<HttpProductHeaderValue>, bool)> { unsafe {
        <Self as RtActivatable<IHttpProductHeaderValueStatics>>::get_activation_factory().try_parse(input)
    }}
}
DEFINE_CLSID!(HttpProductHeaderValue(&[87,105,110,100,111,119,115,46,87,101,98,46,72,116,116,112,46,72,101,97,100,101,114,115,46,72,116,116,112,80,114,111,100,117,99,116,72,101,97,100,101,114,86,97,108,117,101,0]) [CLSID_HttpProductHeaderValue]);
DEFINE_IID!(IID_IHttpProductHeaderValueFactory, 1629136117, 33468, 17147, 151, 123, 220, 0, 83, 110, 94, 134);
RT_INTERFACE!{static interface IHttpProductHeaderValueFactory(IHttpProductHeaderValueFactoryVtbl): IInspectable(IInspectableVtbl) [IID_IHttpProductHeaderValueFactory] {
    fn CreateFromName(&self, productName: HSTRING, out: *mut *mut HttpProductHeaderValue) -> HRESULT,
    fn CreateFromNameWithVersion(&self, productName: HSTRING, productVersion: HSTRING, out: *mut *mut HttpProductHeaderValue) -> HRESULT
}}
impl IHttpProductHeaderValueFactory {
    #[inline] pub unsafe fn create_from_name(&self, productName: &HStringArg) -> Result<ComPtr<HttpProductHeaderValue>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).CreateFromName)(self as *const _ as *mut _, productName.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn create_from_name_with_version(&self, productName: &HStringArg, productVersion: &HStringArg) -> Result<ComPtr<HttpProductHeaderValue>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).CreateFromNameWithVersion)(self as *const _ as *mut _, productName.get(), productVersion.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
DEFINE_IID!(IID_IHttpProductHeaderValueStatics, 2428714537, 48892, 17207, 190, 98, 73, 240, 151, 151, 95, 83);
RT_INTERFACE!{static interface IHttpProductHeaderValueStatics(IHttpProductHeaderValueStaticsVtbl): IInspectable(IInspectableVtbl) [IID_IHttpProductHeaderValueStatics] {
    fn Parse(&self, input: HSTRING, out: *mut *mut HttpProductHeaderValue) -> HRESULT,
    fn TryParse(&self, input: HSTRING, productHeaderValue: *mut *mut HttpProductHeaderValue, out: *mut bool) -> HRESULT
}}
impl IHttpProductHeaderValueStatics {
    #[inline] pub unsafe fn parse(&self, input: &HStringArg) -> Result<ComPtr<HttpProductHeaderValue>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).Parse)(self as *const _ as *mut _, input.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn try_parse(&self, input: &HStringArg) -> Result<(ComPtr<HttpProductHeaderValue>, bool)> {
        let mut productHeaderValue = null_mut(); let mut out = zeroed();
        let hr = ((*self.lpVtbl).TryParse)(self as *const _ as *mut _, input.get(), &mut productHeaderValue, &mut out);
        if hr == S_OK { Ok((ComPtr::wrap(productHeaderValue), out)) } else { err(hr) }
    }
}
DEFINE_IID!(IID_IHttpProductInfoHeaderValue, 454723378, 19509, 18538, 150, 111, 100, 100, 137, 25, 142, 77);
RT_INTERFACE!{interface IHttpProductInfoHeaderValue(IHttpProductInfoHeaderValueVtbl): IInspectable(IInspectableVtbl) [IID_IHttpProductInfoHeaderValue] {
    fn get_Product(&self, out: *mut *mut HttpProductHeaderValue) -> HRESULT,
    fn get_Comment(&self, out: *mut HSTRING) -> HRESULT
}}
impl IHttpProductInfoHeaderValue {
    #[inline] pub unsafe fn get_product(&self) -> Result<ComPtr<HttpProductHeaderValue>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Product)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_comment(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Comment)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
}
RT_CLASS!{class HttpProductInfoHeaderValue: IHttpProductInfoHeaderValue}
impl RtActivatable<IHttpProductInfoHeaderValueFactory> for HttpProductInfoHeaderValue {}
impl RtActivatable<IHttpProductInfoHeaderValueStatics> for HttpProductInfoHeaderValue {}
impl HttpProductInfoHeaderValue {
    #[inline] pub fn create_from_comment(productComment: &HStringArg) -> Result<ComPtr<HttpProductInfoHeaderValue>> { unsafe {
        <Self as RtActivatable<IHttpProductInfoHeaderValueFactory>>::get_activation_factory().create_from_comment(productComment)
    }}
    #[inline] pub fn create_from_name_with_version(productName: &HStringArg, productVersion: &HStringArg) -> Result<ComPtr<HttpProductInfoHeaderValue>> { unsafe {
        <Self as RtActivatable<IHttpProductInfoHeaderValueFactory>>::get_activation_factory().create_from_name_with_version(productName, productVersion)
    }}
    #[inline] pub fn parse(input: &HStringArg) -> Result<ComPtr<HttpProductInfoHeaderValue>> { unsafe {
        <Self as RtActivatable<IHttpProductInfoHeaderValueStatics>>::get_activation_factory().parse(input)
    }}
    #[inline] pub fn try_parse(input: &HStringArg) -> Result<(ComPtr<HttpProductInfoHeaderValue>, bool)> { unsafe {
        <Self as RtActivatable<IHttpProductInfoHeaderValueStatics>>::get_activation_factory().try_parse(input)
    }}
}
DEFINE_CLSID!(HttpProductInfoHeaderValue(&[87,105,110,100,111,119,115,46,87,101,98,46,72,116,116,112,46,72,101,97,100,101,114,115,46,72,116,116,112,80,114,111,100,117,99,116,73,110,102,111,72,101,97,100,101,114,86,97,108,117,101,0]) [CLSID_HttpProductInfoHeaderValue]);
DEFINE_IID!(IID_IHttpProductInfoHeaderValueCollection, 2273179466, 54939, 17656, 173, 79, 69, 58, 249, 196, 46, 208);
RT_INTERFACE!{interface IHttpProductInfoHeaderValueCollection(IHttpProductInfoHeaderValueCollectionVtbl): IInspectable(IInspectableVtbl) [IID_IHttpProductInfoHeaderValueCollection] {
    fn ParseAdd(&self, input: HSTRING) -> HRESULT,
    fn TryParseAdd(&self, input: HSTRING, out: *mut bool) -> HRESULT
}}
impl IHttpProductInfoHeaderValueCollection {
    #[inline] pub unsafe fn parse_add(&self, input: &HStringArg) -> Result<()> {
        let hr = ((*self.lpVtbl).ParseAdd)(self as *const _ as *mut _, input.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn try_parse_add(&self, input: &HStringArg) -> Result<bool> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).TryParseAdd)(self as *const _ as *mut _, input.get(), &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
}
RT_CLASS!{class HttpProductInfoHeaderValueCollection: IHttpProductInfoHeaderValueCollection}
DEFINE_IID!(IID_IHttpProductInfoHeaderValueFactory, 606212030, 60094, 17508, 180, 96, 236, 1, 11, 124, 65, 226);
RT_INTERFACE!{static interface IHttpProductInfoHeaderValueFactory(IHttpProductInfoHeaderValueFactoryVtbl): IInspectable(IInspectableVtbl) [IID_IHttpProductInfoHeaderValueFactory] {
    fn CreateFromComment(&self, productComment: HSTRING, out: *mut *mut HttpProductInfoHeaderValue) -> HRESULT,
    fn CreateFromNameWithVersion(&self, productName: HSTRING, productVersion: HSTRING, out: *mut *mut HttpProductInfoHeaderValue) -> HRESULT
}}
impl IHttpProductInfoHeaderValueFactory {
    #[inline] pub unsafe fn create_from_comment(&self, productComment: &HStringArg) -> Result<ComPtr<HttpProductInfoHeaderValue>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).CreateFromComment)(self as *const _ as *mut _, productComment.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn create_from_name_with_version(&self, productName: &HStringArg, productVersion: &HStringArg) -> Result<ComPtr<HttpProductInfoHeaderValue>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).CreateFromNameWithVersion)(self as *const _ as *mut _, productName.get(), productVersion.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
DEFINE_IID!(IID_IHttpProductInfoHeaderValueStatics, 3682588759, 12922, 20083, 129, 229, 112, 89, 163, 2, 176, 66);
RT_INTERFACE!{static interface IHttpProductInfoHeaderValueStatics(IHttpProductInfoHeaderValueStaticsVtbl): IInspectable(IInspectableVtbl) [IID_IHttpProductInfoHeaderValueStatics] {
    fn Parse(&self, input: HSTRING, out: *mut *mut HttpProductInfoHeaderValue) -> HRESULT,
    fn TryParse(&self, input: HSTRING, productInfoHeaderValue: *mut *mut HttpProductInfoHeaderValue, out: *mut bool) -> HRESULT
}}
impl IHttpProductInfoHeaderValueStatics {
    #[inline] pub unsafe fn parse(&self, input: &HStringArg) -> Result<ComPtr<HttpProductInfoHeaderValue>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).Parse)(self as *const _ as *mut _, input.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn try_parse(&self, input: &HStringArg) -> Result<(ComPtr<HttpProductInfoHeaderValue>, bool)> {
        let mut productInfoHeaderValue = null_mut(); let mut out = zeroed();
        let hr = ((*self.lpVtbl).TryParse)(self as *const _ as *mut _, input.get(), &mut productInfoHeaderValue, &mut out);
        if hr == S_OK { Ok((ComPtr::wrap(productInfoHeaderValue), out)) } else { err(hr) }
    }
}
DEFINE_IID!(IID_IHttpRequestHeaderCollection, 2940220059, 46404, 18075, 134, 185, 172, 61, 70, 111, 234, 54);
RT_INTERFACE!{interface IHttpRequestHeaderCollection(IHttpRequestHeaderCollectionVtbl): IInspectable(IInspectableVtbl) [IID_IHttpRequestHeaderCollection] {
    fn get_Accept(&self, out: *mut *mut HttpMediaTypeWithQualityHeaderValueCollection) -> HRESULT,
    fn get_AcceptEncoding(&self, out: *mut *mut HttpContentCodingWithQualityHeaderValueCollection) -> HRESULT,
    fn get_AcceptLanguage(&self, out: *mut *mut HttpLanguageRangeWithQualityHeaderValueCollection) -> HRESULT,
    fn get_Authorization(&self, out: *mut *mut HttpCredentialsHeaderValue) -> HRESULT,
    fn put_Authorization(&self, value: *mut HttpCredentialsHeaderValue) -> HRESULT,
    fn get_CacheControl(&self, out: *mut *mut HttpCacheDirectiveHeaderValueCollection) -> HRESULT,
    fn get_Connection(&self, out: *mut *mut HttpConnectionOptionHeaderValueCollection) -> HRESULT,
    fn get_Cookie(&self, out: *mut *mut HttpCookiePairHeaderValueCollection) -> HRESULT,
    fn get_Date(&self, out: *mut *mut ::rt::gen::windows::foundation::IReference<::rt::gen::windows::foundation::DateTime>) -> HRESULT,
    fn put_Date(&self, value: *mut ::rt::gen::windows::foundation::IReference<::rt::gen::windows::foundation::DateTime>) -> HRESULT,
    fn get_Expect(&self, out: *mut *mut HttpExpectationHeaderValueCollection) -> HRESULT,
    fn get_From(&self, out: *mut HSTRING) -> HRESULT,
    fn put_From(&self, value: HSTRING) -> HRESULT,
    #[cfg(not(feature="windows-networking"))] fn __Dummy13(&self) -> (),
    #[cfg(feature="windows-networking")] fn get_Host(&self, out: *mut *mut ::rt::gen::windows::networking::HostName) -> HRESULT,
    #[cfg(not(feature="windows-networking"))] fn __Dummy14(&self) -> (),
    #[cfg(feature="windows-networking")] fn put_Host(&self, value: *mut ::rt::gen::windows::networking::HostName) -> HRESULT,
    fn get_IfModifiedSince(&self, out: *mut *mut ::rt::gen::windows::foundation::IReference<::rt::gen::windows::foundation::DateTime>) -> HRESULT,
    fn put_IfModifiedSince(&self, value: *mut ::rt::gen::windows::foundation::IReference<::rt::gen::windows::foundation::DateTime>) -> HRESULT,
    fn get_IfUnmodifiedSince(&self, out: *mut *mut ::rt::gen::windows::foundation::IReference<::rt::gen::windows::foundation::DateTime>) -> HRESULT,
    fn put_IfUnmodifiedSince(&self, value: *mut ::rt::gen::windows::foundation::IReference<::rt::gen::windows::foundation::DateTime>) -> HRESULT,
    fn get_MaxForwards(&self, out: *mut *mut ::rt::gen::windows::foundation::IReference<u32>) -> HRESULT,
    fn put_MaxForwards(&self, value: *mut ::rt::gen::windows::foundation::IReference<u32>) -> HRESULT,
    fn get_ProxyAuthorization(&self, out: *mut *mut HttpCredentialsHeaderValue) -> HRESULT,
    fn put_ProxyAuthorization(&self, value: *mut HttpCredentialsHeaderValue) -> HRESULT,
    fn get_Referer(&self, out: *mut *mut ::rt::gen::windows::foundation::Uri) -> HRESULT,
    fn put_Referer(&self, value: *mut ::rt::gen::windows::foundation::Uri) -> HRESULT,
    fn get_TransferEncoding(&self, out: *mut *mut HttpTransferCodingHeaderValueCollection) -> HRESULT,
    fn get_UserAgent(&self, out: *mut *mut HttpProductInfoHeaderValueCollection) -> HRESULT,
    fn Append(&self, name: HSTRING, value: HSTRING) -> HRESULT,
    fn TryAppendWithoutValidation(&self, name: HSTRING, value: HSTRING, out: *mut bool) -> HRESULT
}}
impl IHttpRequestHeaderCollection {
    #[inline] pub unsafe fn get_accept(&self) -> Result<ComPtr<HttpMediaTypeWithQualityHeaderValueCollection>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Accept)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_accept_encoding(&self) -> Result<ComPtr<HttpContentCodingWithQualityHeaderValueCollection>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_AcceptEncoding)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_accept_language(&self) -> Result<ComPtr<HttpLanguageRangeWithQualityHeaderValueCollection>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_AcceptLanguage)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_authorization(&self) -> Result<ComPtr<HttpCredentialsHeaderValue>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Authorization)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_authorization(&self, value: &HttpCredentialsHeaderValue) -> Result<()> {
        let hr = ((*self.lpVtbl).put_Authorization)(self as *const _ as *mut _, value as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_cache_control(&self) -> Result<ComPtr<HttpCacheDirectiveHeaderValueCollection>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_CacheControl)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_connection(&self) -> Result<ComPtr<HttpConnectionOptionHeaderValueCollection>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Connection)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_cookie(&self) -> Result<ComPtr<HttpCookiePairHeaderValueCollection>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Cookie)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_date(&self) -> Result<ComPtr<::rt::gen::windows::foundation::IReference<::rt::gen::windows::foundation::DateTime>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Date)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_date(&self, value: &::rt::gen::windows::foundation::IReference<::rt::gen::windows::foundation::DateTime>) -> Result<()> {
        let hr = ((*self.lpVtbl).put_Date)(self as *const _ as *mut _, value as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_expect(&self) -> Result<ComPtr<HttpExpectationHeaderValueCollection>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Expect)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_from(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_From)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_from(&self, value: &HStringArg) -> Result<()> {
        let hr = ((*self.lpVtbl).put_From)(self as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[cfg(feature="windows-networking")] #[inline] pub unsafe fn get_host(&self) -> Result<ComPtr<::rt::gen::windows::networking::HostName>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Host)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[cfg(feature="windows-networking")] #[inline] pub unsafe fn set_host(&self, value: &::rt::gen::windows::networking::HostName) -> Result<()> {
        let hr = ((*self.lpVtbl).put_Host)(self as *const _ as *mut _, value as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_if_modified_since(&self) -> Result<ComPtr<::rt::gen::windows::foundation::IReference<::rt::gen::windows::foundation::DateTime>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_IfModifiedSince)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_if_modified_since(&self, value: &::rt::gen::windows::foundation::IReference<::rt::gen::windows::foundation::DateTime>) -> Result<()> {
        let hr = ((*self.lpVtbl).put_IfModifiedSince)(self as *const _ as *mut _, value as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_if_unmodified_since(&self) -> Result<ComPtr<::rt::gen::windows::foundation::IReference<::rt::gen::windows::foundation::DateTime>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_IfUnmodifiedSince)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_if_unmodified_since(&self, value: &::rt::gen::windows::foundation::IReference<::rt::gen::windows::foundation::DateTime>) -> Result<()> {
        let hr = ((*self.lpVtbl).put_IfUnmodifiedSince)(self as *const _ as *mut _, value as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_max_forwards(&self) -> Result<ComPtr<::rt::gen::windows::foundation::IReference<u32>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_MaxForwards)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_max_forwards(&self, value: &::rt::gen::windows::foundation::IReference<u32>) -> Result<()> {
        let hr = ((*self.lpVtbl).put_MaxForwards)(self as *const _ as *mut _, value as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_proxy_authorization(&self) -> Result<ComPtr<HttpCredentialsHeaderValue>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_ProxyAuthorization)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_proxy_authorization(&self, value: &HttpCredentialsHeaderValue) -> Result<()> {
        let hr = ((*self.lpVtbl).put_ProxyAuthorization)(self as *const _ as *mut _, value as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_referer(&self) -> Result<ComPtr<::rt::gen::windows::foundation::Uri>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Referer)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_referer(&self, value: &::rt::gen::windows::foundation::Uri) -> Result<()> {
        let hr = ((*self.lpVtbl).put_Referer)(self as *const _ as *mut _, value as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_transfer_encoding(&self) -> Result<ComPtr<HttpTransferCodingHeaderValueCollection>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_TransferEncoding)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_user_agent(&self) -> Result<ComPtr<HttpProductInfoHeaderValueCollection>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_UserAgent)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn append(&self, name: &HStringArg, value: &HStringArg) -> Result<()> {
        let hr = ((*self.lpVtbl).Append)(self as *const _ as *mut _, name.get(), value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn try_append_without_validation(&self, name: &HStringArg, value: &HStringArg) -> Result<bool> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).TryAppendWithoutValidation)(self as *const _ as *mut _, name.get(), value.get(), &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
}
RT_CLASS!{class HttpRequestHeaderCollection: IHttpRequestHeaderCollection}
DEFINE_IID!(IID_IHttpResponseHeaderCollection, 2056849769, 64063, 16877, 170, 198, 191, 149, 121, 117, 193, 107);
RT_INTERFACE!{interface IHttpResponseHeaderCollection(IHttpResponseHeaderCollectionVtbl): IInspectable(IInspectableVtbl) [IID_IHttpResponseHeaderCollection] {
    fn get_Age(&self, out: *mut *mut ::rt::gen::windows::foundation::IReference<::rt::gen::windows::foundation::TimeSpan>) -> HRESULT,
    fn put_Age(&self, value: *mut ::rt::gen::windows::foundation::IReference<::rt::gen::windows::foundation::TimeSpan>) -> HRESULT,
    fn get_Allow(&self, out: *mut *mut HttpMethodHeaderValueCollection) -> HRESULT,
    fn get_CacheControl(&self, out: *mut *mut HttpCacheDirectiveHeaderValueCollection) -> HRESULT,
    fn get_Connection(&self, out: *mut *mut HttpConnectionOptionHeaderValueCollection) -> HRESULT,
    fn get_Date(&self, out: *mut *mut ::rt::gen::windows::foundation::IReference<::rt::gen::windows::foundation::DateTime>) -> HRESULT,
    fn put_Date(&self, value: *mut ::rt::gen::windows::foundation::IReference<::rt::gen::windows::foundation::DateTime>) -> HRESULT,
    fn get_Location(&self, out: *mut *mut ::rt::gen::windows::foundation::Uri) -> HRESULT,
    fn put_Location(&self, value: *mut ::rt::gen::windows::foundation::Uri) -> HRESULT,
    fn get_ProxyAuthenticate(&self, out: *mut *mut HttpChallengeHeaderValueCollection) -> HRESULT,
    fn get_RetryAfter(&self, out: *mut *mut HttpDateOrDeltaHeaderValue) -> HRESULT,
    fn put_RetryAfter(&self, value: *mut HttpDateOrDeltaHeaderValue) -> HRESULT,
    fn get_TransferEncoding(&self, out: *mut *mut HttpTransferCodingHeaderValueCollection) -> HRESULT,
    fn get_WwwAuthenticate(&self, out: *mut *mut HttpChallengeHeaderValueCollection) -> HRESULT,
    fn Append(&self, name: HSTRING, value: HSTRING) -> HRESULT,
    fn TryAppendWithoutValidation(&self, name: HSTRING, value: HSTRING, out: *mut bool) -> HRESULT
}}
impl IHttpResponseHeaderCollection {
    #[inline] pub unsafe fn get_age(&self) -> Result<ComPtr<::rt::gen::windows::foundation::IReference<::rt::gen::windows::foundation::TimeSpan>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Age)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_age(&self, value: &::rt::gen::windows::foundation::IReference<::rt::gen::windows::foundation::TimeSpan>) -> Result<()> {
        let hr = ((*self.lpVtbl).put_Age)(self as *const _ as *mut _, value as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_allow(&self) -> Result<ComPtr<HttpMethodHeaderValueCollection>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Allow)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_cache_control(&self) -> Result<ComPtr<HttpCacheDirectiveHeaderValueCollection>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_CacheControl)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_connection(&self) -> Result<ComPtr<HttpConnectionOptionHeaderValueCollection>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Connection)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_date(&self) -> Result<ComPtr<::rt::gen::windows::foundation::IReference<::rt::gen::windows::foundation::DateTime>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Date)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_date(&self, value: &::rt::gen::windows::foundation::IReference<::rt::gen::windows::foundation::DateTime>) -> Result<()> {
        let hr = ((*self.lpVtbl).put_Date)(self as *const _ as *mut _, value as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_location(&self) -> Result<ComPtr<::rt::gen::windows::foundation::Uri>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Location)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_location(&self, value: &::rt::gen::windows::foundation::Uri) -> Result<()> {
        let hr = ((*self.lpVtbl).put_Location)(self as *const _ as *mut _, value as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_proxy_authenticate(&self) -> Result<ComPtr<HttpChallengeHeaderValueCollection>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_ProxyAuthenticate)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_retry_after(&self) -> Result<ComPtr<HttpDateOrDeltaHeaderValue>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_RetryAfter)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_retry_after(&self, value: &HttpDateOrDeltaHeaderValue) -> Result<()> {
        let hr = ((*self.lpVtbl).put_RetryAfter)(self as *const _ as *mut _, value as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_transfer_encoding(&self) -> Result<ComPtr<HttpTransferCodingHeaderValueCollection>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_TransferEncoding)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_www_authenticate(&self) -> Result<ComPtr<HttpChallengeHeaderValueCollection>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_WwwAuthenticate)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn append(&self, name: &HStringArg, value: &HStringArg) -> Result<()> {
        let hr = ((*self.lpVtbl).Append)(self as *const _ as *mut _, name.get(), value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn try_append_without_validation(&self, name: &HStringArg, value: &HStringArg) -> Result<bool> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).TryAppendWithoutValidation)(self as *const _ as *mut _, name.get(), value.get(), &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
}
RT_CLASS!{class HttpResponseHeaderCollection: IHttpResponseHeaderCollection}
DEFINE_IID!(IID_IHttpTransferCodingHeaderValue, 1131361017, 15853, 17085, 179, 138, 84, 150, 162, 81, 28, 230);
RT_INTERFACE!{interface IHttpTransferCodingHeaderValue(IHttpTransferCodingHeaderValueVtbl): IInspectable(IInspectableVtbl) [IID_IHttpTransferCodingHeaderValue] {
    fn get_Parameters(&self, out: *mut *mut ::rt::gen::windows::foundation::collections::IVector<HttpNameValueHeaderValue>) -> HRESULT,
    fn get_Value(&self, out: *mut HSTRING) -> HRESULT
}}
impl IHttpTransferCodingHeaderValue {
    #[inline] pub unsafe fn get_parameters(&self) -> Result<ComPtr<::rt::gen::windows::foundation::collections::IVector<HttpNameValueHeaderValue>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Parameters)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_value(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Value)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
}
RT_CLASS!{class HttpTransferCodingHeaderValue: IHttpTransferCodingHeaderValue}
impl RtActivatable<IHttpTransferCodingHeaderValueFactory> for HttpTransferCodingHeaderValue {}
impl RtActivatable<IHttpTransferCodingHeaderValueStatics> for HttpTransferCodingHeaderValue {}
impl HttpTransferCodingHeaderValue {
    #[inline] pub fn create(input: &HStringArg) -> Result<ComPtr<HttpTransferCodingHeaderValue>> { unsafe {
        <Self as RtActivatable<IHttpTransferCodingHeaderValueFactory>>::get_activation_factory().create(input)
    }}
    #[inline] pub fn parse(input: &HStringArg) -> Result<ComPtr<HttpTransferCodingHeaderValue>> { unsafe {
        <Self as RtActivatable<IHttpTransferCodingHeaderValueStatics>>::get_activation_factory().parse(input)
    }}
    #[inline] pub fn try_parse(input: &HStringArg) -> Result<(ComPtr<HttpTransferCodingHeaderValue>, bool)> { unsafe {
        <Self as RtActivatable<IHttpTransferCodingHeaderValueStatics>>::get_activation_factory().try_parse(input)
    }}
}
DEFINE_CLSID!(HttpTransferCodingHeaderValue(&[87,105,110,100,111,119,115,46,87,101,98,46,72,116,116,112,46,72,101,97,100,101,114,115,46,72,116,116,112,84,114,97,110,115,102,101,114,67,111,100,105,110,103,72,101,97,100,101,114,86,97,108,117,101,0]) [CLSID_HttpTransferCodingHeaderValue]);
DEFINE_IID!(IID_IHttpTransferCodingHeaderValueCollection, 539790388, 11267, 18872, 150, 101, 115, 226, 124, 178, 252, 121);
RT_INTERFACE!{interface IHttpTransferCodingHeaderValueCollection(IHttpTransferCodingHeaderValueCollectionVtbl): IInspectable(IInspectableVtbl) [IID_IHttpTransferCodingHeaderValueCollection] {
    fn ParseAdd(&self, input: HSTRING) -> HRESULT,
    fn TryParseAdd(&self, input: HSTRING, out: *mut bool) -> HRESULT
}}
impl IHttpTransferCodingHeaderValueCollection {
    #[inline] pub unsafe fn parse_add(&self, input: &HStringArg) -> Result<()> {
        let hr = ((*self.lpVtbl).ParseAdd)(self as *const _ as *mut _, input.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn try_parse_add(&self, input: &HStringArg) -> Result<bool> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).TryParseAdd)(self as *const _ as *mut _, input.get(), &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
}
RT_CLASS!{class HttpTransferCodingHeaderValueCollection: IHttpTransferCodingHeaderValueCollection}
DEFINE_IID!(IID_IHttpTransferCodingHeaderValueFactory, 3143819260, 58209, 20232, 142, 79, 201, 231, 35, 222, 112, 59);
RT_INTERFACE!{static interface IHttpTransferCodingHeaderValueFactory(IHttpTransferCodingHeaderValueFactoryVtbl): IInspectable(IInspectableVtbl) [IID_IHttpTransferCodingHeaderValueFactory] {
    fn Create(&self, input: HSTRING, out: *mut *mut HttpTransferCodingHeaderValue) -> HRESULT
}}
impl IHttpTransferCodingHeaderValueFactory {
    #[inline] pub unsafe fn create(&self, input: &HStringArg) -> Result<ComPtr<HttpTransferCodingHeaderValue>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).Create)(self as *const _ as *mut _, input.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
DEFINE_IID!(IID_IHttpTransferCodingHeaderValueStatics, 1790478634, 6808, 19762, 169, 6, 116, 112, 169, 135, 92, 229);
RT_INTERFACE!{static interface IHttpTransferCodingHeaderValueStatics(IHttpTransferCodingHeaderValueStaticsVtbl): IInspectable(IInspectableVtbl) [IID_IHttpTransferCodingHeaderValueStatics] {
    fn Parse(&self, input: HSTRING, out: *mut *mut HttpTransferCodingHeaderValue) -> HRESULT,
    fn TryParse(&self, input: HSTRING, transferCodingHeaderValue: *mut *mut HttpTransferCodingHeaderValue, out: *mut bool) -> HRESULT
}}
impl IHttpTransferCodingHeaderValueStatics {
    #[inline] pub unsafe fn parse(&self, input: &HStringArg) -> Result<ComPtr<HttpTransferCodingHeaderValue>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).Parse)(self as *const _ as *mut _, input.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn try_parse(&self, input: &HStringArg) -> Result<(ComPtr<HttpTransferCodingHeaderValue>, bool)> {
        let mut transferCodingHeaderValue = null_mut(); let mut out = zeroed();
        let hr = ((*self.lpVtbl).TryParse)(self as *const _ as *mut _, input.get(), &mut transferCodingHeaderValue, &mut out);
        if hr == S_OK { Ok((ComPtr::wrap(transferCodingHeaderValue), out)) } else { err(hr) }
    }
}
} // Windows.Web.Http.Headers
pub mod diagnostics { // Windows.Web.Http.Diagnostics
use ::prelude::*;
DEFINE_IID!(IID_IHttpDiagnosticProvider, 3179353345, 41046, 19769, 177, 116, 131, 59, 123, 3, 176, 44);
RT_INTERFACE!{interface IHttpDiagnosticProvider(IHttpDiagnosticProviderVtbl): IInspectable(IInspectableVtbl) [IID_IHttpDiagnosticProvider] {
    fn Start(&self) -> HRESULT,
    fn Stop(&self) -> HRESULT,
    fn add_RequestSent(&self, handler: *mut ::rt::gen::windows::foundation::TypedEventHandler<HttpDiagnosticProvider, HttpDiagnosticProviderRequestSentEventArgs>, out: *mut ::rt::gen::windows::foundation::EventRegistrationToken) -> HRESULT,
    fn remove_RequestSent(&self, token: ::rt::gen::windows::foundation::EventRegistrationToken) -> HRESULT,
    fn add_ResponseReceived(&self, handler: *mut ::rt::gen::windows::foundation::TypedEventHandler<HttpDiagnosticProvider, HttpDiagnosticProviderResponseReceivedEventArgs>, out: *mut ::rt::gen::windows::foundation::EventRegistrationToken) -> HRESULT,
    fn remove_ResponseReceived(&self, token: ::rt::gen::windows::foundation::EventRegistrationToken) -> HRESULT,
    fn add_RequestResponseCompleted(&self, handler: *mut ::rt::gen::windows::foundation::TypedEventHandler<HttpDiagnosticProvider, HttpDiagnosticProviderRequestResponseCompletedEventArgs>, out: *mut ::rt::gen::windows::foundation::EventRegistrationToken) -> HRESULT,
    fn remove_RequestResponseCompleted(&self, token: ::rt::gen::windows::foundation::EventRegistrationToken) -> HRESULT
}}
impl IHttpDiagnosticProvider {
    #[inline] pub unsafe fn start(&self) -> Result<()> {
        let hr = ((*self.lpVtbl).Start)(self as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn stop(&self) -> Result<()> {
        let hr = ((*self.lpVtbl).Stop)(self as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn add_request_sent(&self, handler: &::rt::gen::windows::foundation::TypedEventHandler<HttpDiagnosticProvider, HttpDiagnosticProviderRequestSentEventArgs>) -> Result<::rt::gen::windows::foundation::EventRegistrationToken> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).add_RequestSent)(self as *const _ as *mut _, handler as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn remove_request_sent(&self, token: ::rt::gen::windows::foundation::EventRegistrationToken) -> Result<()> {
        let hr = ((*self.lpVtbl).remove_RequestSent)(self as *const _ as *mut _, token);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn add_response_received(&self, handler: &::rt::gen::windows::foundation::TypedEventHandler<HttpDiagnosticProvider, HttpDiagnosticProviderResponseReceivedEventArgs>) -> Result<::rt::gen::windows::foundation::EventRegistrationToken> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).add_ResponseReceived)(self as *const _ as *mut _, handler as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn remove_response_received(&self, token: ::rt::gen::windows::foundation::EventRegistrationToken) -> Result<()> {
        let hr = ((*self.lpVtbl).remove_ResponseReceived)(self as *const _ as *mut _, token);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn add_request_response_completed(&self, handler: &::rt::gen::windows::foundation::TypedEventHandler<HttpDiagnosticProvider, HttpDiagnosticProviderRequestResponseCompletedEventArgs>) -> Result<::rt::gen::windows::foundation::EventRegistrationToken> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).add_RequestResponseCompleted)(self as *const _ as *mut _, handler as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn remove_request_response_completed(&self, token: ::rt::gen::windows::foundation::EventRegistrationToken) -> Result<()> {
        let hr = ((*self.lpVtbl).remove_RequestResponseCompleted)(self as *const _ as *mut _, token);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
}
RT_CLASS!{class HttpDiagnosticProvider: IHttpDiagnosticProvider}
impl RtActivatable<IHttpDiagnosticProviderStatics> for HttpDiagnosticProvider {}
impl HttpDiagnosticProvider {
    #[cfg(feature="windows-system")] #[inline] pub fn create_from_process_diagnostic_info(processDiagnosticInfo: &::rt::gen::windows::system::diagnostics::ProcessDiagnosticInfo) -> Result<ComPtr<HttpDiagnosticProvider>> { unsafe {
        <Self as RtActivatable<IHttpDiagnosticProviderStatics>>::get_activation_factory().create_from_process_diagnostic_info(processDiagnosticInfo)
    }}
}
DEFINE_CLSID!(HttpDiagnosticProvider(&[87,105,110,100,111,119,115,46,87,101,98,46,72,116,116,112,46,68,105,97,103,110,111,115,116,105,99,115,46,72,116,116,112,68,105,97,103,110,111,115,116,105,99,80,114,111,118,105,100,101,114,0]) [CLSID_HttpDiagnosticProvider]);
DEFINE_IID!(IID_IHttpDiagnosticProviderRequestResponseCompletedEventArgs, 1935644910, 38134, 17714, 178, 110, 97, 225, 177, 228, 239, 212);
RT_INTERFACE!{interface IHttpDiagnosticProviderRequestResponseCompletedEventArgs(IHttpDiagnosticProviderRequestResponseCompletedEventArgsVtbl): IInspectable(IInspectableVtbl) [IID_IHttpDiagnosticProviderRequestResponseCompletedEventArgs] {
    fn get_ActivityId(&self, out: *mut Guid) -> HRESULT,
    fn get_Timestamps(&self, out: *mut *mut HttpDiagnosticProviderRequestResponseTimestamps) -> HRESULT,
    fn get_RequestedUri(&self, out: *mut *mut ::rt::gen::windows::foundation::Uri) -> HRESULT,
    fn get_ProcessId(&self, out: *mut u32) -> HRESULT,
    fn get_ThreadId(&self, out: *mut u32) -> HRESULT,
    fn get_Initiator(&self, out: *mut HttpDiagnosticRequestInitiator) -> HRESULT,
    fn get_SourceLocations(&self, out: *mut *mut ::rt::gen::windows::foundation::collections::IVectorView<HttpDiagnosticSourceLocation>) -> HRESULT
}}
impl IHttpDiagnosticProviderRequestResponseCompletedEventArgs {
    #[inline] pub unsafe fn get_activity_id(&self) -> Result<Guid> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_ActivityId)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_timestamps(&self) -> Result<ComPtr<HttpDiagnosticProviderRequestResponseTimestamps>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Timestamps)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_requested_uri(&self) -> Result<ComPtr<::rt::gen::windows::foundation::Uri>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_RequestedUri)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_process_id(&self) -> Result<u32> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_ProcessId)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_thread_id(&self) -> Result<u32> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_ThreadId)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_initiator(&self) -> Result<HttpDiagnosticRequestInitiator> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_Initiator)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_source_locations(&self) -> Result<ComPtr<::rt::gen::windows::foundation::collections::IVectorView<HttpDiagnosticSourceLocation>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_SourceLocations)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
RT_CLASS!{class HttpDiagnosticProviderRequestResponseCompletedEventArgs: IHttpDiagnosticProviderRequestResponseCompletedEventArgs}
DEFINE_IID!(IID_IHttpDiagnosticProviderRequestResponseTimestamps, 3769622032, 21967, 19457, 145, 212, 162, 5, 87, 216, 73, 240);
RT_INTERFACE!{interface IHttpDiagnosticProviderRequestResponseTimestamps(IHttpDiagnosticProviderRequestResponseTimestampsVtbl): IInspectable(IInspectableVtbl) [IID_IHttpDiagnosticProviderRequestResponseTimestamps] {
    fn get_CacheCheckedTimestamp(&self, out: *mut *mut ::rt::gen::windows::foundation::IReference<::rt::gen::windows::foundation::DateTime>) -> HRESULT,
    fn get_ConnectionInitiatedTimestamp(&self, out: *mut *mut ::rt::gen::windows::foundation::IReference<::rt::gen::windows::foundation::DateTime>) -> HRESULT,
    fn get_NameResolvedTimestamp(&self, out: *mut *mut ::rt::gen::windows::foundation::IReference<::rt::gen::windows::foundation::DateTime>) -> HRESULT,
    fn get_SslNegotiatedTimestamp(&self, out: *mut *mut ::rt::gen::windows::foundation::IReference<::rt::gen::windows::foundation::DateTime>) -> HRESULT,
    fn get_ConnectionCompletedTimestamp(&self, out: *mut *mut ::rt::gen::windows::foundation::IReference<::rt::gen::windows::foundation::DateTime>) -> HRESULT,
    fn get_RequestSentTimestamp(&self, out: *mut *mut ::rt::gen::windows::foundation::IReference<::rt::gen::windows::foundation::DateTime>) -> HRESULT,
    fn get_RequestCompletedTimestamp(&self, out: *mut *mut ::rt::gen::windows::foundation::IReference<::rt::gen::windows::foundation::DateTime>) -> HRESULT,
    fn get_ResponseReceivedTimestamp(&self, out: *mut *mut ::rt::gen::windows::foundation::IReference<::rt::gen::windows::foundation::DateTime>) -> HRESULT,
    fn get_ResponseCompletedTimestamp(&self, out: *mut *mut ::rt::gen::windows::foundation::IReference<::rt::gen::windows::foundation::DateTime>) -> HRESULT
}}
impl IHttpDiagnosticProviderRequestResponseTimestamps {
    #[inline] pub unsafe fn get_cache_checked_timestamp(&self) -> Result<ComPtr<::rt::gen::windows::foundation::IReference<::rt::gen::windows::foundation::DateTime>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_CacheCheckedTimestamp)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_connection_initiated_timestamp(&self) -> Result<ComPtr<::rt::gen::windows::foundation::IReference<::rt::gen::windows::foundation::DateTime>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_ConnectionInitiatedTimestamp)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_name_resolved_timestamp(&self) -> Result<ComPtr<::rt::gen::windows::foundation::IReference<::rt::gen::windows::foundation::DateTime>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_NameResolvedTimestamp)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_ssl_negotiated_timestamp(&self) -> Result<ComPtr<::rt::gen::windows::foundation::IReference<::rt::gen::windows::foundation::DateTime>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_SslNegotiatedTimestamp)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_connection_completed_timestamp(&self) -> Result<ComPtr<::rt::gen::windows::foundation::IReference<::rt::gen::windows::foundation::DateTime>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_ConnectionCompletedTimestamp)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_request_sent_timestamp(&self) -> Result<ComPtr<::rt::gen::windows::foundation::IReference<::rt::gen::windows::foundation::DateTime>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_RequestSentTimestamp)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_request_completed_timestamp(&self) -> Result<ComPtr<::rt::gen::windows::foundation::IReference<::rt::gen::windows::foundation::DateTime>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_RequestCompletedTimestamp)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_response_received_timestamp(&self) -> Result<ComPtr<::rt::gen::windows::foundation::IReference<::rt::gen::windows::foundation::DateTime>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_ResponseReceivedTimestamp)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_response_completed_timestamp(&self) -> Result<ComPtr<::rt::gen::windows::foundation::IReference<::rt::gen::windows::foundation::DateTime>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_ResponseCompletedTimestamp)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
RT_CLASS!{class HttpDiagnosticProviderRequestResponseTimestamps: IHttpDiagnosticProviderRequestResponseTimestamps}
DEFINE_IID!(IID_IHttpDiagnosticProviderRequestSentEventArgs, 1062311632, 19487, 20158, 165, 122, 6, 147, 7, 113, 197, 13);
RT_INTERFACE!{interface IHttpDiagnosticProviderRequestSentEventArgs(IHttpDiagnosticProviderRequestSentEventArgsVtbl): IInspectable(IInspectableVtbl) [IID_IHttpDiagnosticProviderRequestSentEventArgs] {
    fn get_Timestamp(&self, out: *mut ::rt::gen::windows::foundation::DateTime) -> HRESULT,
    fn get_ActivityId(&self, out: *mut Guid) -> HRESULT,
    fn get_Message(&self, out: *mut *mut super::HttpRequestMessage) -> HRESULT,
    fn get_ProcessId(&self, out: *mut u32) -> HRESULT,
    fn get_ThreadId(&self, out: *mut u32) -> HRESULT,
    fn get_Initiator(&self, out: *mut HttpDiagnosticRequestInitiator) -> HRESULT,
    fn get_SourceLocations(&self, out: *mut *mut ::rt::gen::windows::foundation::collections::IVectorView<HttpDiagnosticSourceLocation>) -> HRESULT
}}
impl IHttpDiagnosticProviderRequestSentEventArgs {
    #[inline] pub unsafe fn get_timestamp(&self) -> Result<::rt::gen::windows::foundation::DateTime> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_Timestamp)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_activity_id(&self) -> Result<Guid> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_ActivityId)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_message(&self) -> Result<ComPtr<super::HttpRequestMessage>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Message)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_process_id(&self) -> Result<u32> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_ProcessId)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_thread_id(&self) -> Result<u32> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_ThreadId)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_initiator(&self) -> Result<HttpDiagnosticRequestInitiator> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_Initiator)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_source_locations(&self) -> Result<ComPtr<::rt::gen::windows::foundation::collections::IVectorView<HttpDiagnosticSourceLocation>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_SourceLocations)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
RT_CLASS!{class HttpDiagnosticProviderRequestSentEventArgs: IHttpDiagnosticProviderRequestSentEventArgs}
DEFINE_IID!(IID_IHttpDiagnosticProviderResponseReceivedEventArgs, 2694993516, 43871, 19814, 187, 45, 8, 76, 244, 22, 53, 208);
RT_INTERFACE!{interface IHttpDiagnosticProviderResponseReceivedEventArgs(IHttpDiagnosticProviderResponseReceivedEventArgsVtbl): IInspectable(IInspectableVtbl) [IID_IHttpDiagnosticProviderResponseReceivedEventArgs] {
    fn get_Timestamp(&self, out: *mut ::rt::gen::windows::foundation::DateTime) -> HRESULT,
    fn get_ActivityId(&self, out: *mut Guid) -> HRESULT,
    fn get_Message(&self, out: *mut *mut super::HttpResponseMessage) -> HRESULT
}}
impl IHttpDiagnosticProviderResponseReceivedEventArgs {
    #[inline] pub unsafe fn get_timestamp(&self) -> Result<::rt::gen::windows::foundation::DateTime> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_Timestamp)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_activity_id(&self) -> Result<Guid> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_ActivityId)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_message(&self) -> Result<ComPtr<super::HttpResponseMessage>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Message)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
RT_CLASS!{class HttpDiagnosticProviderResponseReceivedEventArgs: IHttpDiagnosticProviderResponseReceivedEventArgs}
DEFINE_IID!(IID_IHttpDiagnosticProviderStatics, 1535266497, 27244, 18380, 175, 236, 30, 134, 188, 38, 5, 59);
RT_INTERFACE!{static interface IHttpDiagnosticProviderStatics(IHttpDiagnosticProviderStaticsVtbl): IInspectable(IInspectableVtbl) [IID_IHttpDiagnosticProviderStatics] {
    #[cfg(feature="windows-system")] fn CreateFromProcessDiagnosticInfo(&self, processDiagnosticInfo: *mut ::rt::gen::windows::system::diagnostics::ProcessDiagnosticInfo, out: *mut *mut HttpDiagnosticProvider) -> HRESULT
}}
impl IHttpDiagnosticProviderStatics {
    #[cfg(feature="windows-system")] #[inline] pub unsafe fn create_from_process_diagnostic_info(&self, processDiagnosticInfo: &::rt::gen::windows::system::diagnostics::ProcessDiagnosticInfo) -> Result<ComPtr<HttpDiagnosticProvider>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).CreateFromProcessDiagnosticInfo)(self as *const _ as *mut _, processDiagnosticInfo as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
RT_ENUM! { enum HttpDiagnosticRequestInitiator: i32 {
    ParsedElement (HttpDiagnosticRequestInitiator_ParsedElement) = 0, Script (HttpDiagnosticRequestInitiator_Script) = 1, Image (HttpDiagnosticRequestInitiator_Image) = 2, Link (HttpDiagnosticRequestInitiator_Link) = 3, Style (HttpDiagnosticRequestInitiator_Style) = 4, XmlHttpRequest (HttpDiagnosticRequestInitiator_XmlHttpRequest) = 5, Media (HttpDiagnosticRequestInitiator_Media) = 6, HtmlDownload (HttpDiagnosticRequestInitiator_HtmlDownload) = 7, Prefetch (HttpDiagnosticRequestInitiator_Prefetch) = 8, Other (HttpDiagnosticRequestInitiator_Other) = 9, CrossOriginPreFlight (HttpDiagnosticRequestInitiator_CrossOriginPreFlight) = 10, Fetch (HttpDiagnosticRequestInitiator_Fetch) = 11, Beacon (HttpDiagnosticRequestInitiator_Beacon) = 12,
}}
DEFINE_IID!(IID_IHttpDiagnosticSourceLocation, 1420415584, 34912, 16959, 182, 250, 215, 119, 22, 246, 71, 167);
RT_INTERFACE!{interface IHttpDiagnosticSourceLocation(IHttpDiagnosticSourceLocationVtbl): IInspectable(IInspectableVtbl) [IID_IHttpDiagnosticSourceLocation] {
    fn get_SourceUri(&self, out: *mut *mut ::rt::gen::windows::foundation::Uri) -> HRESULT,
    fn get_LineNumber(&self, out: *mut u64) -> HRESULT,
    fn get_ColumnNumber(&self, out: *mut u64) -> HRESULT
}}
impl IHttpDiagnosticSourceLocation {
    #[inline] pub unsafe fn get_source_uri(&self) -> Result<ComPtr<::rt::gen::windows::foundation::Uri>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_SourceUri)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_line_number(&self) -> Result<u64> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_LineNumber)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_column_number(&self) -> Result<u64> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_ColumnNumber)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
}
RT_CLASS!{class HttpDiagnosticSourceLocation: IHttpDiagnosticSourceLocation}
} // Windows.Web.Http.Diagnostics
} // Windows.Web.Http
pub mod syndication { // Windows.Web.Syndication
use ::prelude::*;
RT_STRUCT! { struct RetrievalProgress {
    BytesRetrieved: u32, TotalBytesToRetrieve: u32,
}}
DEFINE_IID!(IID_ISyndicationAttribute, 1911093609, 21102, 16385, 154, 145, 232, 79, 131, 22, 26, 177);
RT_INTERFACE!{interface ISyndicationAttribute(ISyndicationAttributeVtbl): IInspectable(IInspectableVtbl) [IID_ISyndicationAttribute] {
    fn get_Name(&self, out: *mut HSTRING) -> HRESULT,
    fn put_Name(&self, value: HSTRING) -> HRESULT,
    fn get_Namespace(&self, out: *mut HSTRING) -> HRESULT,
    fn put_Namespace(&self, value: HSTRING) -> HRESULT,
    fn get_Value(&self, out: *mut HSTRING) -> HRESULT,
    fn put_Value(&self, value: HSTRING) -> HRESULT
}}
impl ISyndicationAttribute {
    #[inline] pub unsafe fn get_name(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Name)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_name(&self, value: &HStringArg) -> Result<()> {
        let hr = ((*self.lpVtbl).put_Name)(self as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_namespace(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Namespace)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_namespace(&self, value: &HStringArg) -> Result<()> {
        let hr = ((*self.lpVtbl).put_Namespace)(self as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_value(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Value)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_value(&self, value: &HStringArg) -> Result<()> {
        let hr = ((*self.lpVtbl).put_Value)(self as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
}
RT_CLASS!{class SyndicationAttribute: ISyndicationAttribute}
impl RtActivatable<ISyndicationAttributeFactory> for SyndicationAttribute {}
impl RtActivatable<IActivationFactory> for SyndicationAttribute {}
impl SyndicationAttribute {
    #[inline] pub fn create_syndication_attribute(attributeName: &HStringArg, attributeNamespace: &HStringArg, attributeValue: &HStringArg) -> Result<ComPtr<SyndicationAttribute>> { unsafe {
        <Self as RtActivatable<ISyndicationAttributeFactory>>::get_activation_factory().create_syndication_attribute(attributeName, attributeNamespace, attributeValue)
    }}
}
DEFINE_CLSID!(SyndicationAttribute(&[87,105,110,100,111,119,115,46,87,101,98,46,83,121,110,100,105,99,97,116,105,111,110,46,83,121,110,100,105,99,97,116,105,111,110,65,116,116,114,105,98,117,116,101,0]) [CLSID_SyndicationAttribute]);
DEFINE_IID!(IID_ISyndicationAttributeFactory, 1649350041, 60734, 16911, 190, 134, 100, 4, 20, 136, 110, 75);
RT_INTERFACE!{static interface ISyndicationAttributeFactory(ISyndicationAttributeFactoryVtbl): IInspectable(IInspectableVtbl) [IID_ISyndicationAttributeFactory] {
    fn CreateSyndicationAttribute(&self, attributeName: HSTRING, attributeNamespace: HSTRING, attributeValue: HSTRING, out: *mut *mut SyndicationAttribute) -> HRESULT
}}
impl ISyndicationAttributeFactory {
    #[inline] pub unsafe fn create_syndication_attribute(&self, attributeName: &HStringArg, attributeNamespace: &HStringArg, attributeValue: &HStringArg) -> Result<ComPtr<SyndicationAttribute>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).CreateSyndicationAttribute)(self as *const _ as *mut _, attributeName.get(), attributeNamespace.get(), attributeValue.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
DEFINE_IID!(IID_ISyndicationCategory, 2266325615, 3258, 19071, 137, 255, 236, 181, 40, 20, 35, 182);
RT_INTERFACE!{interface ISyndicationCategory(ISyndicationCategoryVtbl): IInspectable(IInspectableVtbl) [IID_ISyndicationCategory] {
    fn get_Label(&self, out: *mut HSTRING) -> HRESULT,
    fn put_Label(&self, value: HSTRING) -> HRESULT,
    fn get_Scheme(&self, out: *mut HSTRING) -> HRESULT,
    fn put_Scheme(&self, value: HSTRING) -> HRESULT,
    fn get_Term(&self, out: *mut HSTRING) -> HRESULT,
    fn put_Term(&self, value: HSTRING) -> HRESULT
}}
impl ISyndicationCategory {
    #[inline] pub unsafe fn get_label(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Label)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_label(&self, value: &HStringArg) -> Result<()> {
        let hr = ((*self.lpVtbl).put_Label)(self as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_scheme(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Scheme)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_scheme(&self, value: &HStringArg) -> Result<()> {
        let hr = ((*self.lpVtbl).put_Scheme)(self as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_term(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Term)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_term(&self, value: &HStringArg) -> Result<()> {
        let hr = ((*self.lpVtbl).put_Term)(self as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
}
RT_CLASS!{class SyndicationCategory: ISyndicationCategory}
impl RtActivatable<ISyndicationCategoryFactory> for SyndicationCategory {}
impl RtActivatable<IActivationFactory> for SyndicationCategory {}
impl SyndicationCategory {
    #[inline] pub fn create_syndication_category(term: &HStringArg) -> Result<ComPtr<SyndicationCategory>> { unsafe {
        <Self as RtActivatable<ISyndicationCategoryFactory>>::get_activation_factory().create_syndication_category(term)
    }}
    #[inline] pub fn create_syndication_category_ex(term: &HStringArg, scheme: &HStringArg, label: &HStringArg) -> Result<ComPtr<SyndicationCategory>> { unsafe {
        <Self as RtActivatable<ISyndicationCategoryFactory>>::get_activation_factory().create_syndication_category_ex(term, scheme, label)
    }}
}
DEFINE_CLSID!(SyndicationCategory(&[87,105,110,100,111,119,115,46,87,101,98,46,83,121,110,100,105,99,97,116,105,111,110,46,83,121,110,100,105,99,97,116,105,111,110,67,97,116,101,103,111,114,121,0]) [CLSID_SyndicationCategory]);
DEFINE_IID!(IID_ISyndicationCategoryFactory, 2873262127, 18912, 17701, 138, 178, 171, 69, 192, 37, 40, 255);
RT_INTERFACE!{static interface ISyndicationCategoryFactory(ISyndicationCategoryFactoryVtbl): IInspectable(IInspectableVtbl) [IID_ISyndicationCategoryFactory] {
    fn CreateSyndicationCategory(&self, term: HSTRING, out: *mut *mut SyndicationCategory) -> HRESULT,
    fn CreateSyndicationCategoryEx(&self, term: HSTRING, scheme: HSTRING, label: HSTRING, out: *mut *mut SyndicationCategory) -> HRESULT
}}
impl ISyndicationCategoryFactory {
    #[inline] pub unsafe fn create_syndication_category(&self, term: &HStringArg) -> Result<ComPtr<SyndicationCategory>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).CreateSyndicationCategory)(self as *const _ as *mut _, term.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn create_syndication_category_ex(&self, term: &HStringArg, scheme: &HStringArg, label: &HStringArg) -> Result<ComPtr<SyndicationCategory>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).CreateSyndicationCategoryEx)(self as *const _ as *mut _, term.get(), scheme.get(), label.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
DEFINE_IID!(IID_ISyndicationClient, 2652416439, 29257, 19269, 178, 41, 125, 248, 149, 165, 161, 245);
RT_INTERFACE!{interface ISyndicationClient(ISyndicationClientVtbl): IInspectable(IInspectableVtbl) [IID_ISyndicationClient] {
    #[cfg(not(feature="windows-security"))] fn __Dummy0(&self) -> (),
    #[cfg(feature="windows-security")] fn get_ServerCredential(&self, out: *mut *mut super::super::security::credentials::PasswordCredential) -> HRESULT,
    #[cfg(not(feature="windows-security"))] fn __Dummy1(&self) -> (),
    #[cfg(feature="windows-security")] fn put_ServerCredential(&self, value: *mut super::super::security::credentials::PasswordCredential) -> HRESULT,
    #[cfg(not(feature="windows-security"))] fn __Dummy2(&self) -> (),
    #[cfg(feature="windows-security")] fn get_ProxyCredential(&self, out: *mut *mut super::super::security::credentials::PasswordCredential) -> HRESULT,
    #[cfg(not(feature="windows-security"))] fn __Dummy3(&self) -> (),
    #[cfg(feature="windows-security")] fn put_ProxyCredential(&self, value: *mut super::super::security::credentials::PasswordCredential) -> HRESULT,
    fn get_MaxResponseBufferSize(&self, out: *mut u32) -> HRESULT,
    fn put_MaxResponseBufferSize(&self, value: u32) -> HRESULT,
    fn get_Timeout(&self, out: *mut u32) -> HRESULT,
    fn put_Timeout(&self, value: u32) -> HRESULT,
    fn get_BypassCacheOnRetrieve(&self, out: *mut bool) -> HRESULT,
    fn put_BypassCacheOnRetrieve(&self, value: bool) -> HRESULT,
    fn SetRequestHeader(&self, name: HSTRING, value: HSTRING) -> HRESULT,
    fn RetrieveFeedAsync(&self, uri: *mut super::super::foundation::Uri, out: *mut *mut super::super::foundation::IAsyncOperationWithProgress<SyndicationFeed, RetrievalProgress>) -> HRESULT
}}
impl ISyndicationClient {
    #[cfg(feature="windows-security")] #[inline] pub unsafe fn get_server_credential(&self) -> Result<ComPtr<super::super::security::credentials::PasswordCredential>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_ServerCredential)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[cfg(feature="windows-security")] #[inline] pub unsafe fn set_server_credential(&self, value: &super::super::security::credentials::PasswordCredential) -> Result<()> {
        let hr = ((*self.lpVtbl).put_ServerCredential)(self as *const _ as *mut _, value as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[cfg(feature="windows-security")] #[inline] pub unsafe fn get_proxy_credential(&self) -> Result<ComPtr<super::super::security::credentials::PasswordCredential>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_ProxyCredential)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[cfg(feature="windows-security")] #[inline] pub unsafe fn set_proxy_credential(&self, value: &super::super::security::credentials::PasswordCredential) -> Result<()> {
        let hr = ((*self.lpVtbl).put_ProxyCredential)(self as *const _ as *mut _, value as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_max_response_buffer_size(&self) -> Result<u32> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_MaxResponseBufferSize)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_max_response_buffer_size(&self, value: u32) -> Result<()> {
        let hr = ((*self.lpVtbl).put_MaxResponseBufferSize)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_timeout(&self) -> Result<u32> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_Timeout)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_timeout(&self, value: u32) -> Result<()> {
        let hr = ((*self.lpVtbl).put_Timeout)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_bypass_cache_on_retrieve(&self) -> Result<bool> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_BypassCacheOnRetrieve)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_bypass_cache_on_retrieve(&self, value: bool) -> Result<()> {
        let hr = ((*self.lpVtbl).put_BypassCacheOnRetrieve)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_request_header(&self, name: &HStringArg, value: &HStringArg) -> Result<()> {
        let hr = ((*self.lpVtbl).SetRequestHeader)(self as *const _ as *mut _, name.get(), value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn retrieve_feed_async(&self, uri: &super::super::foundation::Uri) -> Result<ComPtr<super::super::foundation::IAsyncOperationWithProgress<SyndicationFeed, RetrievalProgress>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).RetrieveFeedAsync)(self as *const _ as *mut _, uri as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
RT_CLASS!{class SyndicationClient: ISyndicationClient}
impl RtActivatable<ISyndicationClientFactory> for SyndicationClient {}
impl RtActivatable<IActivationFactory> for SyndicationClient {}
impl SyndicationClient {
    #[cfg(feature="windows-security")] #[inline] pub fn create_syndication_client(serverCredential: &super::super::security::credentials::PasswordCredential) -> Result<ComPtr<SyndicationClient>> { unsafe {
        <Self as RtActivatable<ISyndicationClientFactory>>::get_activation_factory().create_syndication_client(serverCredential)
    }}
}
DEFINE_CLSID!(SyndicationClient(&[87,105,110,100,111,119,115,46,87,101,98,46,83,121,110,100,105,99,97,116,105,111,110,46,83,121,110,100,105,99,97,116,105,111,110,67,108,105,101,110,116,0]) [CLSID_SyndicationClient]);
DEFINE_IID!(IID_ISyndicationClientFactory, 784642860, 42907, 16660, 178, 154, 5, 223, 251, 175, 185, 164);
RT_INTERFACE!{static interface ISyndicationClientFactory(ISyndicationClientFactoryVtbl): IInspectable(IInspectableVtbl) [IID_ISyndicationClientFactory] {
    #[cfg(feature="windows-security")] fn CreateSyndicationClient(&self, serverCredential: *mut super::super::security::credentials::PasswordCredential, out: *mut *mut SyndicationClient) -> HRESULT
}}
impl ISyndicationClientFactory {
    #[cfg(feature="windows-security")] #[inline] pub unsafe fn create_syndication_client(&self, serverCredential: &super::super::security::credentials::PasswordCredential) -> Result<ComPtr<SyndicationClient>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).CreateSyndicationClient)(self as *const _ as *mut _, serverCredential as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
DEFINE_IID!(IID_ISyndicationContent, 1178730238, 3669, 16592, 184, 208, 106, 44, 203, 169, 252, 124);
RT_INTERFACE!{interface ISyndicationContent(ISyndicationContentVtbl): IInspectable(IInspectableVtbl) [IID_ISyndicationContent] {
    fn get_SourceUri(&self, out: *mut *mut super::super::foundation::Uri) -> HRESULT,
    fn put_SourceUri(&self, value: *mut super::super::foundation::Uri) -> HRESULT
}}
impl ISyndicationContent {
    #[inline] pub unsafe fn get_source_uri(&self) -> Result<ComPtr<super::super::foundation::Uri>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_SourceUri)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_source_uri(&self, value: &super::super::foundation::Uri) -> Result<()> {
        let hr = ((*self.lpVtbl).put_SourceUri)(self as *const _ as *mut _, value as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
}
RT_CLASS!{class SyndicationContent: ISyndicationContent}
impl RtActivatable<ISyndicationContentFactory> for SyndicationContent {}
impl RtActivatable<IActivationFactory> for SyndicationContent {}
impl SyndicationContent {
    #[inline] pub fn create_syndication_content(text: &HStringArg, type_: SyndicationTextType) -> Result<ComPtr<SyndicationContent>> { unsafe {
        <Self as RtActivatable<ISyndicationContentFactory>>::get_activation_factory().create_syndication_content(text, type_)
    }}
    #[inline] pub fn create_syndication_content_with_source_uri(sourceUri: &super::super::foundation::Uri) -> Result<ComPtr<SyndicationContent>> { unsafe {
        <Self as RtActivatable<ISyndicationContentFactory>>::get_activation_factory().create_syndication_content_with_source_uri(sourceUri)
    }}
}
DEFINE_CLSID!(SyndicationContent(&[87,105,110,100,111,119,115,46,87,101,98,46,83,121,110,100,105,99,97,116,105,111,110,46,83,121,110,100,105,99,97,116,105,111,110,67,111,110,116,101,110,116,0]) [CLSID_SyndicationContent]);
DEFINE_IID!(IID_ISyndicationContentFactory, 1026538387, 38176, 16755, 147, 136, 126, 45, 243, 36, 168, 160);
RT_INTERFACE!{static interface ISyndicationContentFactory(ISyndicationContentFactoryVtbl): IInspectable(IInspectableVtbl) [IID_ISyndicationContentFactory] {
    fn CreateSyndicationContent(&self, text: HSTRING, type_: SyndicationTextType, out: *mut *mut SyndicationContent) -> HRESULT,
    fn CreateSyndicationContentWithSourceUri(&self, sourceUri: *mut super::super::foundation::Uri, out: *mut *mut SyndicationContent) -> HRESULT
}}
impl ISyndicationContentFactory {
    #[inline] pub unsafe fn create_syndication_content(&self, text: &HStringArg, type_: SyndicationTextType) -> Result<ComPtr<SyndicationContent>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).CreateSyndicationContent)(self as *const _ as *mut _, text.get(), type_, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn create_syndication_content_with_source_uri(&self, sourceUri: &super::super::foundation::Uri) -> Result<ComPtr<SyndicationContent>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).CreateSyndicationContentWithSourceUri)(self as *const _ as *mut _, sourceUri as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
RT_CLASS!{static class SyndicationError}
impl RtActivatable<ISyndicationErrorStatics> for SyndicationError {}
impl SyndicationError {
    #[inline] pub fn get_status(hresult: i32) -> Result<SyndicationErrorStatus> { unsafe {
        <Self as RtActivatable<ISyndicationErrorStatics>>::get_activation_factory().get_status(hresult)
    }}
}
DEFINE_CLSID!(SyndicationError(&[87,105,110,100,111,119,115,46,87,101,98,46,83,121,110,100,105,99,97,116,105,111,110,46,83,121,110,100,105,99,97,116,105,111,110,69,114,114,111,114,0]) [CLSID_SyndicationError]);
DEFINE_IID!(IID_ISyndicationErrorStatics, 532357985, 17863, 18483, 138, 160, 190, 95, 59, 88, 167, 244);
RT_INTERFACE!{static interface ISyndicationErrorStatics(ISyndicationErrorStaticsVtbl): IInspectable(IInspectableVtbl) [IID_ISyndicationErrorStatics] {
    fn GetStatus(&self, hresult: i32, out: *mut SyndicationErrorStatus) -> HRESULT
}}
impl ISyndicationErrorStatics {
    #[inline] pub unsafe fn get_status(&self, hresult: i32) -> Result<SyndicationErrorStatus> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).GetStatus)(self as *const _ as *mut _, hresult, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
}
RT_ENUM! { enum SyndicationErrorStatus: i32 {
    Unknown (SyndicationErrorStatus_Unknown) = 0, MissingRequiredElement (SyndicationErrorStatus_MissingRequiredElement) = 1, MissingRequiredAttribute (SyndicationErrorStatus_MissingRequiredAttribute) = 2, InvalidXml (SyndicationErrorStatus_InvalidXml) = 3, UnexpectedContent (SyndicationErrorStatus_UnexpectedContent) = 4, UnsupportedFormat (SyndicationErrorStatus_UnsupportedFormat) = 5,
}}
DEFINE_IID!(IID_ISyndicationFeed, 2147368146, 23398, 19810, 132, 3, 27, 193, 13, 145, 13, 107);
RT_INTERFACE!{interface ISyndicationFeed(ISyndicationFeedVtbl): IInspectable(IInspectableVtbl) [IID_ISyndicationFeed] {
    fn get_Authors(&self, out: *mut *mut super::super::foundation::collections::IVector<SyndicationPerson>) -> HRESULT,
    fn get_Categories(&self, out: *mut *mut super::super::foundation::collections::IVector<SyndicationCategory>) -> HRESULT,
    fn get_Contributors(&self, out: *mut *mut super::super::foundation::collections::IVector<SyndicationPerson>) -> HRESULT,
    fn get_Generator(&self, out: *mut *mut SyndicationGenerator) -> HRESULT,
    fn put_Generator(&self, value: *mut SyndicationGenerator) -> HRESULT,
    fn get_IconUri(&self, out: *mut *mut super::super::foundation::Uri) -> HRESULT,
    fn put_IconUri(&self, value: *mut super::super::foundation::Uri) -> HRESULT,
    fn get_Id(&self, out: *mut HSTRING) -> HRESULT,
    fn put_Id(&self, value: HSTRING) -> HRESULT,
    fn get_Items(&self, out: *mut *mut super::super::foundation::collections::IVector<SyndicationItem>) -> HRESULT,
    fn get_LastUpdatedTime(&self, out: *mut super::super::foundation::DateTime) -> HRESULT,
    fn put_LastUpdatedTime(&self, value: super::super::foundation::DateTime) -> HRESULT,
    fn get_Links(&self, out: *mut *mut super::super::foundation::collections::IVector<SyndicationLink>) -> HRESULT,
    fn get_ImageUri(&self, out: *mut *mut super::super::foundation::Uri) -> HRESULT,
    fn put_ImageUri(&self, value: *mut super::super::foundation::Uri) -> HRESULT,
    fn get_Rights(&self, out: *mut *mut ISyndicationText) -> HRESULT,
    fn put_Rights(&self, value: *mut ISyndicationText) -> HRESULT,
    fn get_Subtitle(&self, out: *mut *mut ISyndicationText) -> HRESULT,
    fn put_Subtitle(&self, value: *mut ISyndicationText) -> HRESULT,
    fn get_Title(&self, out: *mut *mut ISyndicationText) -> HRESULT,
    fn put_Title(&self, value: *mut ISyndicationText) -> HRESULT,
    fn get_FirstUri(&self, out: *mut *mut super::super::foundation::Uri) -> HRESULT,
    fn get_LastUri(&self, out: *mut *mut super::super::foundation::Uri) -> HRESULT,
    fn get_NextUri(&self, out: *mut *mut super::super::foundation::Uri) -> HRESULT,
    fn get_PreviousUri(&self, out: *mut *mut super::super::foundation::Uri) -> HRESULT,
    fn get_SourceFormat(&self, out: *mut SyndicationFormat) -> HRESULT,
    fn Load(&self, feed: HSTRING) -> HRESULT,
    #[cfg(feature="windows-data")] fn LoadFromXml(&self, feedDocument: *mut super::super::data::xml::dom::XmlDocument) -> HRESULT
}}
impl ISyndicationFeed {
    #[inline] pub unsafe fn get_authors(&self) -> Result<ComPtr<super::super::foundation::collections::IVector<SyndicationPerson>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Authors)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_categories(&self) -> Result<ComPtr<super::super::foundation::collections::IVector<SyndicationCategory>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Categories)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_contributors(&self) -> Result<ComPtr<super::super::foundation::collections::IVector<SyndicationPerson>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Contributors)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_generator(&self) -> Result<ComPtr<SyndicationGenerator>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Generator)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_generator(&self, value: &SyndicationGenerator) -> Result<()> {
        let hr = ((*self.lpVtbl).put_Generator)(self as *const _ as *mut _, value as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_icon_uri(&self) -> Result<ComPtr<super::super::foundation::Uri>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_IconUri)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_icon_uri(&self, value: &super::super::foundation::Uri) -> Result<()> {
        let hr = ((*self.lpVtbl).put_IconUri)(self as *const _ as *mut _, value as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_id(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Id)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_id(&self, value: &HStringArg) -> Result<()> {
        let hr = ((*self.lpVtbl).put_Id)(self as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_items(&self) -> Result<ComPtr<super::super::foundation::collections::IVector<SyndicationItem>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Items)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_last_updated_time(&self) -> Result<super::super::foundation::DateTime> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_LastUpdatedTime)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_last_updated_time(&self, value: super::super::foundation::DateTime) -> Result<()> {
        let hr = ((*self.lpVtbl).put_LastUpdatedTime)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_links(&self) -> Result<ComPtr<super::super::foundation::collections::IVector<SyndicationLink>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Links)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_image_uri(&self) -> Result<ComPtr<super::super::foundation::Uri>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_ImageUri)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_image_uri(&self, value: &super::super::foundation::Uri) -> Result<()> {
        let hr = ((*self.lpVtbl).put_ImageUri)(self as *const _ as *mut _, value as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_rights(&self) -> Result<ComPtr<ISyndicationText>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Rights)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_rights(&self, value: &ISyndicationText) -> Result<()> {
        let hr = ((*self.lpVtbl).put_Rights)(self as *const _ as *mut _, value as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_subtitle(&self) -> Result<ComPtr<ISyndicationText>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Subtitle)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_subtitle(&self, value: &ISyndicationText) -> Result<()> {
        let hr = ((*self.lpVtbl).put_Subtitle)(self as *const _ as *mut _, value as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_title(&self) -> Result<ComPtr<ISyndicationText>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Title)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_title(&self, value: &ISyndicationText) -> Result<()> {
        let hr = ((*self.lpVtbl).put_Title)(self as *const _ as *mut _, value as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_first_uri(&self) -> Result<ComPtr<super::super::foundation::Uri>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_FirstUri)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_last_uri(&self) -> Result<ComPtr<super::super::foundation::Uri>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_LastUri)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_next_uri(&self) -> Result<ComPtr<super::super::foundation::Uri>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_NextUri)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_previous_uri(&self) -> Result<ComPtr<super::super::foundation::Uri>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_PreviousUri)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_source_format(&self) -> Result<SyndicationFormat> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_SourceFormat)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn load(&self, feed: &HStringArg) -> Result<()> {
        let hr = ((*self.lpVtbl).Load)(self as *const _ as *mut _, feed.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[cfg(feature="windows-data")] #[inline] pub unsafe fn load_from_xml(&self, feedDocument: &super::super::data::xml::dom::XmlDocument) -> Result<()> {
        let hr = ((*self.lpVtbl).LoadFromXml)(self as *const _ as *mut _, feedDocument as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
}
RT_CLASS!{class SyndicationFeed: ISyndicationFeed}
impl RtActivatable<ISyndicationFeedFactory> for SyndicationFeed {}
impl RtActivatable<IActivationFactory> for SyndicationFeed {}
impl SyndicationFeed {
    #[inline] pub fn create_syndication_feed(title: &HStringArg, subtitle: &HStringArg, uri: &super::super::foundation::Uri) -> Result<ComPtr<SyndicationFeed>> { unsafe {
        <Self as RtActivatable<ISyndicationFeedFactory>>::get_activation_factory().create_syndication_feed(title, subtitle, uri)
    }}
}
DEFINE_CLSID!(SyndicationFeed(&[87,105,110,100,111,119,115,46,87,101,98,46,83,121,110,100,105,99,97,116,105,111,110,46,83,121,110,100,105,99,97,116,105,111,110,70,101,101,100,0]) [CLSID_SyndicationFeed]);
DEFINE_IID!(IID_ISyndicationFeedFactory, 591864370, 35817, 18615, 137, 52, 98, 5, 19, 29, 147, 87);
RT_INTERFACE!{static interface ISyndicationFeedFactory(ISyndicationFeedFactoryVtbl): IInspectable(IInspectableVtbl) [IID_ISyndicationFeedFactory] {
    fn CreateSyndicationFeed(&self, title: HSTRING, subtitle: HSTRING, uri: *mut super::super::foundation::Uri, out: *mut *mut SyndicationFeed) -> HRESULT
}}
impl ISyndicationFeedFactory {
    #[inline] pub unsafe fn create_syndication_feed(&self, title: &HStringArg, subtitle: &HStringArg, uri: &super::super::foundation::Uri) -> Result<ComPtr<SyndicationFeed>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).CreateSyndicationFeed)(self as *const _ as *mut _, title.get(), subtitle.get(), uri as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
RT_ENUM! { enum SyndicationFormat: i32 {
    Atom10 (SyndicationFormat_Atom10) = 0, Rss20 (SyndicationFormat_Rss20) = 1, Rss10 (SyndicationFormat_Rss10) = 2, Rss092 (SyndicationFormat_Rss092) = 3, Rss091 (SyndicationFormat_Rss091) = 4, Atom03 (SyndicationFormat_Atom03) = 5,
}}
DEFINE_IID!(IID_ISyndicationGenerator, 2540221305, 64299, 20333, 180, 28, 8, 138, 88, 104, 130, 92);
RT_INTERFACE!{interface ISyndicationGenerator(ISyndicationGeneratorVtbl): IInspectable(IInspectableVtbl) [IID_ISyndicationGenerator] {
    fn get_Text(&self, out: *mut HSTRING) -> HRESULT,
    fn put_Text(&self, value: HSTRING) -> HRESULT,
    fn get_Uri(&self, out: *mut *mut super::super::foundation::Uri) -> HRESULT,
    fn put_Uri(&self, value: *mut super::super::foundation::Uri) -> HRESULT,
    fn get_Version(&self, out: *mut HSTRING) -> HRESULT,
    fn put_Version(&self, value: HSTRING) -> HRESULT
}}
impl ISyndicationGenerator {
    #[inline] pub unsafe fn get_text(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Text)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_text(&self, value: &HStringArg) -> Result<()> {
        let hr = ((*self.lpVtbl).put_Text)(self as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_uri(&self) -> Result<ComPtr<super::super::foundation::Uri>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Uri)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_uri(&self, value: &super::super::foundation::Uri) -> Result<()> {
        let hr = ((*self.lpVtbl).put_Uri)(self as *const _ as *mut _, value as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_version(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Version)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_version(&self, value: &HStringArg) -> Result<()> {
        let hr = ((*self.lpVtbl).put_Version)(self as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
}
RT_CLASS!{class SyndicationGenerator: ISyndicationGenerator}
impl RtActivatable<ISyndicationGeneratorFactory> for SyndicationGenerator {}
impl RtActivatable<IActivationFactory> for SyndicationGenerator {}
impl SyndicationGenerator {
    #[inline] pub fn create_syndication_generator(text: &HStringArg) -> Result<ComPtr<SyndicationGenerator>> { unsafe {
        <Self as RtActivatable<ISyndicationGeneratorFactory>>::get_activation_factory().create_syndication_generator(text)
    }}
}
DEFINE_CLSID!(SyndicationGenerator(&[87,105,110,100,111,119,115,46,87,101,98,46,83,121,110,100,105,99,97,116,105,111,110,46,83,121,110,100,105,99,97,116,105,111,110,71,101,110,101,114,97,116,111,114,0]) [CLSID_SyndicationGenerator]);
DEFINE_IID!(IID_ISyndicationGeneratorFactory, 2738914275, 7718, 19900, 186, 157, 26, 184, 75, 239, 249, 123);
RT_INTERFACE!{static interface ISyndicationGeneratorFactory(ISyndicationGeneratorFactoryVtbl): IInspectable(IInspectableVtbl) [IID_ISyndicationGeneratorFactory] {
    fn CreateSyndicationGenerator(&self, text: HSTRING, out: *mut *mut SyndicationGenerator) -> HRESULT
}}
impl ISyndicationGeneratorFactory {
    #[inline] pub unsafe fn create_syndication_generator(&self, text: &HStringArg) -> Result<ComPtr<SyndicationGenerator>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).CreateSyndicationGenerator)(self as *const _ as *mut _, text.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
DEFINE_IID!(IID_ISyndicationItem, 1418573955, 50052, 17857, 138, 232, 163, 120, 196, 236, 72, 108);
RT_INTERFACE!{interface ISyndicationItem(ISyndicationItemVtbl): IInspectable(IInspectableVtbl) [IID_ISyndicationItem] {
    fn get_Authors(&self, out: *mut *mut super::super::foundation::collections::IVector<SyndicationPerson>) -> HRESULT,
    fn get_Categories(&self, out: *mut *mut super::super::foundation::collections::IVector<SyndicationCategory>) -> HRESULT,
    fn get_Contributors(&self, out: *mut *mut super::super::foundation::collections::IVector<SyndicationPerson>) -> HRESULT,
    fn get_Content(&self, out: *mut *mut SyndicationContent) -> HRESULT,
    fn put_Content(&self, value: *mut SyndicationContent) -> HRESULT,
    fn get_Id(&self, out: *mut HSTRING) -> HRESULT,
    fn put_Id(&self, value: HSTRING) -> HRESULT,
    fn get_LastUpdatedTime(&self, out: *mut super::super::foundation::DateTime) -> HRESULT,
    fn put_LastUpdatedTime(&self, value: super::super::foundation::DateTime) -> HRESULT,
    fn get_Links(&self, out: *mut *mut super::super::foundation::collections::IVector<SyndicationLink>) -> HRESULT,
    fn get_PublishedDate(&self, out: *mut super::super::foundation::DateTime) -> HRESULT,
    fn put_PublishedDate(&self, value: super::super::foundation::DateTime) -> HRESULT,
    fn get_Rights(&self, out: *mut *mut ISyndicationText) -> HRESULT,
    fn put_Rights(&self, value: *mut ISyndicationText) -> HRESULT,
    fn get_Source(&self, out: *mut *mut SyndicationFeed) -> HRESULT,
    fn put_Source(&self, value: *mut SyndicationFeed) -> HRESULT,
    fn get_Summary(&self, out: *mut *mut ISyndicationText) -> HRESULT,
    fn put_Summary(&self, value: *mut ISyndicationText) -> HRESULT,
    fn get_Title(&self, out: *mut *mut ISyndicationText) -> HRESULT,
    fn put_Title(&self, value: *mut ISyndicationText) -> HRESULT,
    fn get_CommentsUri(&self, out: *mut *mut super::super::foundation::Uri) -> HRESULT,
    fn put_CommentsUri(&self, value: *mut super::super::foundation::Uri) -> HRESULT,
    fn get_EditUri(&self, out: *mut *mut super::super::foundation::Uri) -> HRESULT,
    fn get_EditMediaUri(&self, out: *mut *mut super::super::foundation::Uri) -> HRESULT,
    fn get_ETag(&self, out: *mut HSTRING) -> HRESULT,
    fn get_ItemUri(&self, out: *mut *mut super::super::foundation::Uri) -> HRESULT,
    fn Load(&self, item: HSTRING) -> HRESULT,
    #[cfg(feature="windows-data")] fn LoadFromXml(&self, itemDocument: *mut super::super::data::xml::dom::XmlDocument) -> HRESULT
}}
impl ISyndicationItem {
    #[inline] pub unsafe fn get_authors(&self) -> Result<ComPtr<super::super::foundation::collections::IVector<SyndicationPerson>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Authors)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_categories(&self) -> Result<ComPtr<super::super::foundation::collections::IVector<SyndicationCategory>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Categories)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_contributors(&self) -> Result<ComPtr<super::super::foundation::collections::IVector<SyndicationPerson>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Contributors)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_content(&self) -> Result<ComPtr<SyndicationContent>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Content)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_content(&self, value: &SyndicationContent) -> Result<()> {
        let hr = ((*self.lpVtbl).put_Content)(self as *const _ as *mut _, value as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_id(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Id)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_id(&self, value: &HStringArg) -> Result<()> {
        let hr = ((*self.lpVtbl).put_Id)(self as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_last_updated_time(&self) -> Result<super::super::foundation::DateTime> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_LastUpdatedTime)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_last_updated_time(&self, value: super::super::foundation::DateTime) -> Result<()> {
        let hr = ((*self.lpVtbl).put_LastUpdatedTime)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_links(&self) -> Result<ComPtr<super::super::foundation::collections::IVector<SyndicationLink>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Links)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_published_date(&self) -> Result<super::super::foundation::DateTime> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_PublishedDate)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_published_date(&self, value: super::super::foundation::DateTime) -> Result<()> {
        let hr = ((*self.lpVtbl).put_PublishedDate)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_rights(&self) -> Result<ComPtr<ISyndicationText>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Rights)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_rights(&self, value: &ISyndicationText) -> Result<()> {
        let hr = ((*self.lpVtbl).put_Rights)(self as *const _ as *mut _, value as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_source(&self) -> Result<ComPtr<SyndicationFeed>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Source)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_source(&self, value: &SyndicationFeed) -> Result<()> {
        let hr = ((*self.lpVtbl).put_Source)(self as *const _ as *mut _, value as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_summary(&self) -> Result<ComPtr<ISyndicationText>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Summary)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_summary(&self, value: &ISyndicationText) -> Result<()> {
        let hr = ((*self.lpVtbl).put_Summary)(self as *const _ as *mut _, value as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_title(&self) -> Result<ComPtr<ISyndicationText>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Title)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_title(&self, value: &ISyndicationText) -> Result<()> {
        let hr = ((*self.lpVtbl).put_Title)(self as *const _ as *mut _, value as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_comments_uri(&self) -> Result<ComPtr<super::super::foundation::Uri>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_CommentsUri)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_comments_uri(&self, value: &super::super::foundation::Uri) -> Result<()> {
        let hr = ((*self.lpVtbl).put_CommentsUri)(self as *const _ as *mut _, value as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_edit_uri(&self) -> Result<ComPtr<super::super::foundation::Uri>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_EditUri)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_edit_media_uri(&self) -> Result<ComPtr<super::super::foundation::Uri>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_EditMediaUri)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_etag(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_ETag)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_item_uri(&self) -> Result<ComPtr<super::super::foundation::Uri>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_ItemUri)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn load(&self, item: &HStringArg) -> Result<()> {
        let hr = ((*self.lpVtbl).Load)(self as *const _ as *mut _, item.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[cfg(feature="windows-data")] #[inline] pub unsafe fn load_from_xml(&self, itemDocument: &super::super::data::xml::dom::XmlDocument) -> Result<()> {
        let hr = ((*self.lpVtbl).LoadFromXml)(self as *const _ as *mut _, itemDocument as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
}
RT_CLASS!{class SyndicationItem: ISyndicationItem}
impl RtActivatable<ISyndicationItemFactory> for SyndicationItem {}
impl RtActivatable<IActivationFactory> for SyndicationItem {}
impl SyndicationItem {
    #[inline] pub fn create_syndication_item(title: &HStringArg, content: &SyndicationContent, uri: &super::super::foundation::Uri) -> Result<ComPtr<SyndicationItem>> { unsafe {
        <Self as RtActivatable<ISyndicationItemFactory>>::get_activation_factory().create_syndication_item(title, content, uri)
    }}
}
DEFINE_CLSID!(SyndicationItem(&[87,105,110,100,111,119,115,46,87,101,98,46,83,121,110,100,105,99,97,116,105,111,110,46,83,121,110,100,105,99,97,116,105,111,110,73,116,101,109,0]) [CLSID_SyndicationItem]);
DEFINE_IID!(IID_ISyndicationItemFactory, 622674767, 32184, 18554, 133, 228, 16, 209, 145, 230, 110, 187);
RT_INTERFACE!{static interface ISyndicationItemFactory(ISyndicationItemFactoryVtbl): IInspectable(IInspectableVtbl) [IID_ISyndicationItemFactory] {
    fn CreateSyndicationItem(&self, title: HSTRING, content: *mut SyndicationContent, uri: *mut super::super::foundation::Uri, out: *mut *mut SyndicationItem) -> HRESULT
}}
impl ISyndicationItemFactory {
    #[inline] pub unsafe fn create_syndication_item(&self, title: &HStringArg, content: &SyndicationContent, uri: &super::super::foundation::Uri) -> Result<ComPtr<SyndicationItem>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).CreateSyndicationItem)(self as *const _ as *mut _, title.get(), content as *const _ as *mut _, uri as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
DEFINE_IID!(IID_ISyndicationLink, 659897021, 41230, 16821, 134, 189, 151, 89, 8, 110, 176, 197);
RT_INTERFACE!{interface ISyndicationLink(ISyndicationLinkVtbl): IInspectable(IInspectableVtbl) [IID_ISyndicationLink] {
    fn get_Length(&self, out: *mut u32) -> HRESULT,
    fn put_Length(&self, value: u32) -> HRESULT,
    fn get_MediaType(&self, out: *mut HSTRING) -> HRESULT,
    fn put_MediaType(&self, value: HSTRING) -> HRESULT,
    fn get_Relationship(&self, out: *mut HSTRING) -> HRESULT,
    fn put_Relationship(&self, value: HSTRING) -> HRESULT,
    fn get_Title(&self, out: *mut HSTRING) -> HRESULT,
    fn put_Title(&self, value: HSTRING) -> HRESULT,
    fn get_Uri(&self, out: *mut *mut super::super::foundation::Uri) -> HRESULT,
    fn put_Uri(&self, value: *mut super::super::foundation::Uri) -> HRESULT,
    fn get_ResourceLanguage(&self, out: *mut HSTRING) -> HRESULT,
    fn put_ResourceLanguage(&self, value: HSTRING) -> HRESULT
}}
impl ISyndicationLink {
    #[inline] pub unsafe fn get_length(&self) -> Result<u32> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_Length)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_length(&self, value: u32) -> Result<()> {
        let hr = ((*self.lpVtbl).put_Length)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_media_type(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_MediaType)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_media_type(&self, value: &HStringArg) -> Result<()> {
        let hr = ((*self.lpVtbl).put_MediaType)(self as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_relationship(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Relationship)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_relationship(&self, value: &HStringArg) -> Result<()> {
        let hr = ((*self.lpVtbl).put_Relationship)(self as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_title(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Title)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_title(&self, value: &HStringArg) -> Result<()> {
        let hr = ((*self.lpVtbl).put_Title)(self as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_uri(&self) -> Result<ComPtr<super::super::foundation::Uri>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Uri)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_uri(&self, value: &super::super::foundation::Uri) -> Result<()> {
        let hr = ((*self.lpVtbl).put_Uri)(self as *const _ as *mut _, value as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_resource_language(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_ResourceLanguage)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_resource_language(&self, value: &HStringArg) -> Result<()> {
        let hr = ((*self.lpVtbl).put_ResourceLanguage)(self as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
}
RT_CLASS!{class SyndicationLink: ISyndicationLink}
impl RtActivatable<ISyndicationLinkFactory> for SyndicationLink {}
impl RtActivatable<IActivationFactory> for SyndicationLink {}
impl SyndicationLink {
    #[inline] pub fn create_syndication_link(uri: &super::super::foundation::Uri) -> Result<ComPtr<SyndicationLink>> { unsafe {
        <Self as RtActivatable<ISyndicationLinkFactory>>::get_activation_factory().create_syndication_link(uri)
    }}
    #[inline] pub fn create_syndication_link_ex(uri: &super::super::foundation::Uri, relationship: &HStringArg, title: &HStringArg, mediaType: &HStringArg, length: u32) -> Result<ComPtr<SyndicationLink>> { unsafe {
        <Self as RtActivatable<ISyndicationLinkFactory>>::get_activation_factory().create_syndication_link_ex(uri, relationship, title, mediaType, length)
    }}
}
DEFINE_CLSID!(SyndicationLink(&[87,105,110,100,111,119,115,46,87,101,98,46,83,121,110,100,105,99,97,116,105,111,110,46,83,121,110,100,105,99,97,116,105,111,110,76,105,110,107,0]) [CLSID_SyndicationLink]);
DEFINE_IID!(IID_ISyndicationLinkFactory, 1591239636, 21813, 18604, 152, 212, 193, 144, 153, 80, 128, 179);
RT_INTERFACE!{static interface ISyndicationLinkFactory(ISyndicationLinkFactoryVtbl): IInspectable(IInspectableVtbl) [IID_ISyndicationLinkFactory] {
    fn CreateSyndicationLink(&self, uri: *mut super::super::foundation::Uri, out: *mut *mut SyndicationLink) -> HRESULT,
    fn CreateSyndicationLinkEx(&self, uri: *mut super::super::foundation::Uri, relationship: HSTRING, title: HSTRING, mediaType: HSTRING, length: u32, out: *mut *mut SyndicationLink) -> HRESULT
}}
impl ISyndicationLinkFactory {
    #[inline] pub unsafe fn create_syndication_link(&self, uri: &super::super::foundation::Uri) -> Result<ComPtr<SyndicationLink>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).CreateSyndicationLink)(self as *const _ as *mut _, uri as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn create_syndication_link_ex(&self, uri: &super::super::foundation::Uri, relationship: &HStringArg, title: &HStringArg, mediaType: &HStringArg, length: u32) -> Result<ComPtr<SyndicationLink>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).CreateSyndicationLinkEx)(self as *const _ as *mut _, uri as *const _ as *mut _, relationship.get(), title.get(), mediaType.get(), length, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
DEFINE_IID!(IID_ISyndicationNode, 1966927736, 20984, 17856, 169, 245, 241, 113, 157, 236, 63, 178);
RT_INTERFACE!{interface ISyndicationNode(ISyndicationNodeVtbl): IInspectable(IInspectableVtbl) [IID_ISyndicationNode] {
    fn get_NodeName(&self, out: *mut HSTRING) -> HRESULT,
    fn put_NodeName(&self, value: HSTRING) -> HRESULT,
    fn get_NodeNamespace(&self, out: *mut HSTRING) -> HRESULT,
    fn put_NodeNamespace(&self, value: HSTRING) -> HRESULT,
    fn get_NodeValue(&self, out: *mut HSTRING) -> HRESULT,
    fn put_NodeValue(&self, value: HSTRING) -> HRESULT,
    fn get_Language(&self, out: *mut HSTRING) -> HRESULT,
    fn put_Language(&self, value: HSTRING) -> HRESULT,
    fn get_BaseUri(&self, out: *mut *mut super::super::foundation::Uri) -> HRESULT,
    fn put_BaseUri(&self, value: *mut super::super::foundation::Uri) -> HRESULT,
    fn get_AttributeExtensions(&self, out: *mut *mut super::super::foundation::collections::IVector<SyndicationAttribute>) -> HRESULT,
    fn get_ElementExtensions(&self, out: *mut *mut super::super::foundation::collections::IVector<ISyndicationNode>) -> HRESULT,
    #[cfg(feature="windows-data")] fn GetXmlDocument(&self, format: SyndicationFormat, out: *mut *mut super::super::data::xml::dom::XmlDocument) -> HRESULT
}}
impl ISyndicationNode {
    #[inline] pub unsafe fn get_node_name(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_NodeName)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_node_name(&self, value: &HStringArg) -> Result<()> {
        let hr = ((*self.lpVtbl).put_NodeName)(self as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_node_namespace(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_NodeNamespace)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_node_namespace(&self, value: &HStringArg) -> Result<()> {
        let hr = ((*self.lpVtbl).put_NodeNamespace)(self as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_node_value(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_NodeValue)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_node_value(&self, value: &HStringArg) -> Result<()> {
        let hr = ((*self.lpVtbl).put_NodeValue)(self as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_language(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Language)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_language(&self, value: &HStringArg) -> Result<()> {
        let hr = ((*self.lpVtbl).put_Language)(self as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_base_uri(&self) -> Result<ComPtr<super::super::foundation::Uri>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_BaseUri)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_base_uri(&self, value: &super::super::foundation::Uri) -> Result<()> {
        let hr = ((*self.lpVtbl).put_BaseUri)(self as *const _ as *mut _, value as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_attribute_extensions(&self) -> Result<ComPtr<super::super::foundation::collections::IVector<SyndicationAttribute>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_AttributeExtensions)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_element_extensions(&self) -> Result<ComPtr<super::super::foundation::collections::IVector<ISyndicationNode>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_ElementExtensions)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[cfg(feature="windows-data")] #[inline] pub unsafe fn get_xml_document(&self, format: SyndicationFormat) -> Result<ComPtr<super::super::data::xml::dom::XmlDocument>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetXmlDocument)(self as *const _ as *mut _, format, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
RT_CLASS!{class SyndicationNode: ISyndicationNode}
impl RtActivatable<ISyndicationNodeFactory> for SyndicationNode {}
impl RtActivatable<IActivationFactory> for SyndicationNode {}
impl SyndicationNode {
    #[inline] pub fn create_syndication_node(nodeName: &HStringArg, nodeNamespace: &HStringArg, nodeValue: &HStringArg) -> Result<ComPtr<SyndicationNode>> { unsafe {
        <Self as RtActivatable<ISyndicationNodeFactory>>::get_activation_factory().create_syndication_node(nodeName, nodeNamespace, nodeValue)
    }}
}
DEFINE_CLSID!(SyndicationNode(&[87,105,110,100,111,119,115,46,87,101,98,46,83,121,110,100,105,99,97,116,105,111,110,46,83,121,110,100,105,99,97,116,105,111,110,78,111,100,101,0]) [CLSID_SyndicationNode]);
DEFINE_IID!(IID_ISyndicationNodeFactory, 311435656, 19147, 18856, 183, 119, 165, 235, 146, 225, 138, 121);
RT_INTERFACE!{static interface ISyndicationNodeFactory(ISyndicationNodeFactoryVtbl): IInspectable(IInspectableVtbl) [IID_ISyndicationNodeFactory] {
    fn CreateSyndicationNode(&self, nodeName: HSTRING, nodeNamespace: HSTRING, nodeValue: HSTRING, out: *mut *mut SyndicationNode) -> HRESULT
}}
impl ISyndicationNodeFactory {
    #[inline] pub unsafe fn create_syndication_node(&self, nodeName: &HStringArg, nodeNamespace: &HStringArg, nodeValue: &HStringArg) -> Result<ComPtr<SyndicationNode>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).CreateSyndicationNode)(self as *const _ as *mut _, nodeName.get(), nodeNamespace.get(), nodeValue.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
DEFINE_IID!(IID_ISyndicationPerson, 4196328922, 42950, 17687, 160, 150, 1, 67, 250, 242, 147, 39);
RT_INTERFACE!{interface ISyndicationPerson(ISyndicationPersonVtbl): IInspectable(IInspectableVtbl) [IID_ISyndicationPerson] {
    fn get_Email(&self, out: *mut HSTRING) -> HRESULT,
    fn put_Email(&self, value: HSTRING) -> HRESULT,
    fn get_Name(&self, out: *mut HSTRING) -> HRESULT,
    fn put_Name(&self, value: HSTRING) -> HRESULT,
    fn get_Uri(&self, out: *mut *mut super::super::foundation::Uri) -> HRESULT,
    fn put_Uri(&self, value: *mut super::super::foundation::Uri) -> HRESULT
}}
impl ISyndicationPerson {
    #[inline] pub unsafe fn get_email(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Email)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_email(&self, value: &HStringArg) -> Result<()> {
        let hr = ((*self.lpVtbl).put_Email)(self as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_name(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Name)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_name(&self, value: &HStringArg) -> Result<()> {
        let hr = ((*self.lpVtbl).put_Name)(self as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_uri(&self) -> Result<ComPtr<super::super::foundation::Uri>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Uri)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_uri(&self, value: &super::super::foundation::Uri) -> Result<()> {
        let hr = ((*self.lpVtbl).put_Uri)(self as *const _ as *mut _, value as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
}
RT_CLASS!{class SyndicationPerson: ISyndicationPerson}
impl RtActivatable<ISyndicationPersonFactory> for SyndicationPerson {}
impl RtActivatable<IActivationFactory> for SyndicationPerson {}
impl SyndicationPerson {
    #[inline] pub fn create_syndication_person(name: &HStringArg) -> Result<ComPtr<SyndicationPerson>> { unsafe {
        <Self as RtActivatable<ISyndicationPersonFactory>>::get_activation_factory().create_syndication_person(name)
    }}
    #[inline] pub fn create_syndication_person_ex(name: &HStringArg, email: &HStringArg, uri: &super::super::foundation::Uri) -> Result<ComPtr<SyndicationPerson>> { unsafe {
        <Self as RtActivatable<ISyndicationPersonFactory>>::get_activation_factory().create_syndication_person_ex(name, email, uri)
    }}
}
DEFINE_CLSID!(SyndicationPerson(&[87,105,110,100,111,119,115,46,87,101,98,46,83,121,110,100,105,99,97,116,105,111,110,46,83,121,110,100,105,99,97,116,105,111,110,80,101,114,115,111,110,0]) [CLSID_SyndicationPerson]);
DEFINE_IID!(IID_ISyndicationPersonFactory, 3707013229, 8861, 19288, 164, 155, 243, 210, 240, 245, 201, 159);
RT_INTERFACE!{static interface ISyndicationPersonFactory(ISyndicationPersonFactoryVtbl): IInspectable(IInspectableVtbl) [IID_ISyndicationPersonFactory] {
    fn CreateSyndicationPerson(&self, name: HSTRING, out: *mut *mut SyndicationPerson) -> HRESULT,
    fn CreateSyndicationPersonEx(&self, name: HSTRING, email: HSTRING, uri: *mut super::super::foundation::Uri, out: *mut *mut SyndicationPerson) -> HRESULT
}}
impl ISyndicationPersonFactory {
    #[inline] pub unsafe fn create_syndication_person(&self, name: &HStringArg) -> Result<ComPtr<SyndicationPerson>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).CreateSyndicationPerson)(self as *const _ as *mut _, name.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn create_syndication_person_ex(&self, name: &HStringArg, email: &HStringArg, uri: &super::super::foundation::Uri) -> Result<ComPtr<SyndicationPerson>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).CreateSyndicationPersonEx)(self as *const _ as *mut _, name.get(), email.get(), uri as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
DEFINE_IID!(IID_ISyndicationText, 3117178496, 12602, 16529, 162, 166, 36, 62, 14, 233, 35, 249);
RT_INTERFACE!{interface ISyndicationText(ISyndicationTextVtbl): IInspectable(IInspectableVtbl) [IID_ISyndicationText] {
    fn get_Text(&self, out: *mut HSTRING) -> HRESULT,
    fn put_Text(&self, value: HSTRING) -> HRESULT,
    fn get_Type(&self, out: *mut HSTRING) -> HRESULT,
    fn put_Type(&self, value: HSTRING) -> HRESULT,
    #[cfg(feature="windows-data")] fn get_Xml(&self, out: *mut *mut super::super::data::xml::dom::XmlDocument) -> HRESULT,
    #[cfg(feature="windows-data")] fn put_Xml(&self, value: *mut super::super::data::xml::dom::XmlDocument) -> HRESULT
}}
impl ISyndicationText {
    #[inline] pub unsafe fn get_text(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Text)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_text(&self, value: &HStringArg) -> Result<()> {
        let hr = ((*self.lpVtbl).put_Text)(self as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_type(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Type)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_type(&self, value: &HStringArg) -> Result<()> {
        let hr = ((*self.lpVtbl).put_Type)(self as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[cfg(feature="windows-data")] #[inline] pub unsafe fn get_xml(&self) -> Result<ComPtr<super::super::data::xml::dom::XmlDocument>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Xml)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[cfg(feature="windows-data")] #[inline] pub unsafe fn set_xml(&self, value: &super::super::data::xml::dom::XmlDocument) -> Result<()> {
        let hr = ((*self.lpVtbl).put_Xml)(self as *const _ as *mut _, value as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
}
RT_CLASS!{class SyndicationText: ISyndicationText}
impl RtActivatable<ISyndicationTextFactory> for SyndicationText {}
impl RtActivatable<IActivationFactory> for SyndicationText {}
impl SyndicationText {
    #[inline] pub fn create_syndication_text(text: &HStringArg) -> Result<ComPtr<SyndicationText>> { unsafe {
        <Self as RtActivatable<ISyndicationTextFactory>>::get_activation_factory().create_syndication_text(text)
    }}
    #[inline] pub fn create_syndication_text_ex(text: &HStringArg, type_: SyndicationTextType) -> Result<ComPtr<SyndicationText>> { unsafe {
        <Self as RtActivatable<ISyndicationTextFactory>>::get_activation_factory().create_syndication_text_ex(text, type_)
    }}
}
DEFINE_CLSID!(SyndicationText(&[87,105,110,100,111,119,115,46,87,101,98,46,83,121,110,100,105,99,97,116,105,111,110,46,83,121,110,100,105,99,97,116,105,111,110,84,101,120,116,0]) [CLSID_SyndicationText]);
DEFINE_IID!(IID_ISyndicationTextFactory, 4000531191, 4550, 19237, 171, 98, 229, 150, 189, 22, 41, 70);
RT_INTERFACE!{static interface ISyndicationTextFactory(ISyndicationTextFactoryVtbl): IInspectable(IInspectableVtbl) [IID_ISyndicationTextFactory] {
    fn CreateSyndicationText(&self, text: HSTRING, out: *mut *mut SyndicationText) -> HRESULT,
    fn CreateSyndicationTextEx(&self, text: HSTRING, type_: SyndicationTextType, out: *mut *mut SyndicationText) -> HRESULT
}}
impl ISyndicationTextFactory {
    #[inline] pub unsafe fn create_syndication_text(&self, text: &HStringArg) -> Result<ComPtr<SyndicationText>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).CreateSyndicationText)(self as *const _ as *mut _, text.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn create_syndication_text_ex(&self, text: &HStringArg, type_: SyndicationTextType) -> Result<ComPtr<SyndicationText>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).CreateSyndicationTextEx)(self as *const _ as *mut _, text.get(), type_, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
RT_ENUM! { enum SyndicationTextType: i32 {
    Text (SyndicationTextType_Text) = 0, Html (SyndicationTextType_Html) = 1, Xhtml (SyndicationTextType_Xhtml) = 2,
}}
RT_STRUCT! { struct TransferProgress {
    BytesSent: u32, TotalBytesToSend: u32, BytesRetrieved: u32, TotalBytesToRetrieve: u32,
}}
} // Windows.Web.Syndication
pub mod atompub { // Windows.Web.AtomPub
use ::prelude::*;
DEFINE_IID!(IID_IAtomPubClient, 892939320, 52717, 19788, 150, 55, 5, 241, 92, 28, 148, 6);
RT_INTERFACE!{interface IAtomPubClient(IAtomPubClientVtbl): IInspectable(IInspectableVtbl) [IID_IAtomPubClient] {
    fn RetrieveServiceDocumentAsync(&self, uri: *mut super::super::foundation::Uri, out: *mut *mut super::super::foundation::IAsyncOperationWithProgress<ServiceDocument, super::syndication::RetrievalProgress>) -> HRESULT,
    #[cfg(not(feature="windows-storage"))] fn __Dummy1(&self) -> (),
    #[cfg(feature="windows-storage")] fn RetrieveMediaResourceAsync(&self, uri: *mut super::super::foundation::Uri, out: *mut *mut super::super::foundation::IAsyncOperationWithProgress<super::super::storage::streams::IInputStream, super::syndication::RetrievalProgress>) -> HRESULT,
    fn RetrieveResourceAsync(&self, uri: *mut super::super::foundation::Uri, out: *mut *mut super::super::foundation::IAsyncOperationWithProgress<super::syndication::SyndicationItem, super::syndication::RetrievalProgress>) -> HRESULT,
    fn CreateResourceAsync(&self, uri: *mut super::super::foundation::Uri, description: HSTRING, item: *mut super::syndication::SyndicationItem, out: *mut *mut super::super::foundation::IAsyncOperationWithProgress<super::syndication::SyndicationItem, super::syndication::TransferProgress>) -> HRESULT,
    #[cfg(not(feature="windows-storage"))] fn __Dummy4(&self) -> (),
    #[cfg(feature="windows-storage")] fn CreateMediaResourceAsync(&self, uri: *mut super::super::foundation::Uri, mediaType: HSTRING, description: HSTRING, mediaStream: *mut super::super::storage::streams::IInputStream, out: *mut *mut super::super::foundation::IAsyncOperationWithProgress<super::syndication::SyndicationItem, super::syndication::TransferProgress>) -> HRESULT,
    #[cfg(not(feature="windows-storage"))] fn __Dummy5(&self) -> (),
    #[cfg(feature="windows-storage")] fn UpdateMediaResourceAsync(&self, uri: *mut super::super::foundation::Uri, mediaType: HSTRING, mediaStream: *mut super::super::storage::streams::IInputStream, out: *mut *mut super::super::foundation::IAsyncActionWithProgress<super::syndication::TransferProgress>) -> HRESULT,
    fn UpdateResourceAsync(&self, uri: *mut super::super::foundation::Uri, item: *mut super::syndication::SyndicationItem, out: *mut *mut super::super::foundation::IAsyncActionWithProgress<super::syndication::TransferProgress>) -> HRESULT,
    fn UpdateResourceItemAsync(&self, item: *mut super::syndication::SyndicationItem, out: *mut *mut super::super::foundation::IAsyncActionWithProgress<super::syndication::TransferProgress>) -> HRESULT,
    fn DeleteResourceAsync(&self, uri: *mut super::super::foundation::Uri, out: *mut *mut super::super::foundation::IAsyncActionWithProgress<super::syndication::TransferProgress>) -> HRESULT,
    fn DeleteResourceItemAsync(&self, item: *mut super::syndication::SyndicationItem, out: *mut *mut super::super::foundation::IAsyncActionWithProgress<super::syndication::TransferProgress>) -> HRESULT,
    fn CancelAsyncOperations(&self) -> HRESULT
}}
impl IAtomPubClient {
    #[inline] pub unsafe fn retrieve_service_document_async(&self, uri: &super::super::foundation::Uri) -> Result<ComPtr<super::super::foundation::IAsyncOperationWithProgress<ServiceDocument, super::syndication::RetrievalProgress>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).RetrieveServiceDocumentAsync)(self as *const _ as *mut _, uri as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[cfg(feature="windows-storage")] #[inline] pub unsafe fn retrieve_media_resource_async(&self, uri: &super::super::foundation::Uri) -> Result<ComPtr<super::super::foundation::IAsyncOperationWithProgress<super::super::storage::streams::IInputStream, super::syndication::RetrievalProgress>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).RetrieveMediaResourceAsync)(self as *const _ as *mut _, uri as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn retrieve_resource_async(&self, uri: &super::super::foundation::Uri) -> Result<ComPtr<super::super::foundation::IAsyncOperationWithProgress<super::syndication::SyndicationItem, super::syndication::RetrievalProgress>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).RetrieveResourceAsync)(self as *const _ as *mut _, uri as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn create_resource_async(&self, uri: &super::super::foundation::Uri, description: &HStringArg, item: &super::syndication::SyndicationItem) -> Result<ComPtr<super::super::foundation::IAsyncOperationWithProgress<super::syndication::SyndicationItem, super::syndication::TransferProgress>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).CreateResourceAsync)(self as *const _ as *mut _, uri as *const _ as *mut _, description.get(), item as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[cfg(feature="windows-storage")] #[inline] pub unsafe fn create_media_resource_async(&self, uri: &super::super::foundation::Uri, mediaType: &HStringArg, description: &HStringArg, mediaStream: &super::super::storage::streams::IInputStream) -> Result<ComPtr<super::super::foundation::IAsyncOperationWithProgress<super::syndication::SyndicationItem, super::syndication::TransferProgress>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).CreateMediaResourceAsync)(self as *const _ as *mut _, uri as *const _ as *mut _, mediaType.get(), description.get(), mediaStream as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[cfg(feature="windows-storage")] #[inline] pub unsafe fn update_media_resource_async(&self, uri: &super::super::foundation::Uri, mediaType: &HStringArg, mediaStream: &super::super::storage::streams::IInputStream) -> Result<ComPtr<super::super::foundation::IAsyncActionWithProgress<super::syndication::TransferProgress>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).UpdateMediaResourceAsync)(self as *const _ as *mut _, uri as *const _ as *mut _, mediaType.get(), mediaStream as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn update_resource_async(&self, uri: &super::super::foundation::Uri, item: &super::syndication::SyndicationItem) -> Result<ComPtr<super::super::foundation::IAsyncActionWithProgress<super::syndication::TransferProgress>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).UpdateResourceAsync)(self as *const _ as *mut _, uri as *const _ as *mut _, item as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn update_resource_item_async(&self, item: &super::syndication::SyndicationItem) -> Result<ComPtr<super::super::foundation::IAsyncActionWithProgress<super::syndication::TransferProgress>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).UpdateResourceItemAsync)(self as *const _ as *mut _, item as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn delete_resource_async(&self, uri: &super::super::foundation::Uri) -> Result<ComPtr<super::super::foundation::IAsyncActionWithProgress<super::syndication::TransferProgress>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).DeleteResourceAsync)(self as *const _ as *mut _, uri as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn delete_resource_item_async(&self, item: &super::syndication::SyndicationItem) -> Result<ComPtr<super::super::foundation::IAsyncActionWithProgress<super::syndication::TransferProgress>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).DeleteResourceItemAsync)(self as *const _ as *mut _, item as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn cancel_async_operations(&self) -> Result<()> {
        let hr = ((*self.lpVtbl).CancelAsyncOperations)(self as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
}
RT_CLASS!{class AtomPubClient: IAtomPubClient}
impl RtActivatable<IAtomPubClientFactory> for AtomPubClient {}
impl RtActivatable<IActivationFactory> for AtomPubClient {}
impl AtomPubClient {
    #[cfg(feature="windows-security")] #[inline] pub fn create_atom_pub_client_with_credentials(serverCredential: &super::super::security::credentials::PasswordCredential) -> Result<ComPtr<AtomPubClient>> { unsafe {
        <Self as RtActivatable<IAtomPubClientFactory>>::get_activation_factory().create_atom_pub_client_with_credentials(serverCredential)
    }}
}
DEFINE_CLSID!(AtomPubClient(&[87,105,110,100,111,119,115,46,87,101,98,46,65,116,111,109,80,117,98,46,65,116,111,109,80,117,98,67,108,105,101,110,116,0]) [CLSID_AtomPubClient]);
DEFINE_IID!(IID_IAtomPubClientFactory, 1238716434, 22475, 19422, 171, 159, 38, 16, 177, 114, 119, 123);
RT_INTERFACE!{static interface IAtomPubClientFactory(IAtomPubClientFactoryVtbl): IInspectable(IInspectableVtbl) [IID_IAtomPubClientFactory] {
    #[cfg(feature="windows-security")] fn CreateAtomPubClientWithCredentials(&self, serverCredential: *mut super::super::security::credentials::PasswordCredential, out: *mut *mut AtomPubClient) -> HRESULT
}}
impl IAtomPubClientFactory {
    #[cfg(feature="windows-security")] #[inline] pub unsafe fn create_atom_pub_client_with_credentials(&self, serverCredential: &super::super::security::credentials::PasswordCredential) -> Result<ComPtr<AtomPubClient>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).CreateAtomPubClientWithCredentials)(self as *const _ as *mut _, serverCredential as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
DEFINE_IID!(IID_IResourceCollection, 2136987145, 48264, 16852, 136, 250, 61, 230, 112, 77, 66, 142);
RT_INTERFACE!{interface IResourceCollection(IResourceCollectionVtbl): IInspectable(IInspectableVtbl) [IID_IResourceCollection] {
    fn get_Title(&self, out: *mut *mut super::syndication::ISyndicationText) -> HRESULT,
    fn get_Uri(&self, out: *mut *mut super::super::foundation::Uri) -> HRESULT,
    fn get_Categories(&self, out: *mut *mut super::super::foundation::collections::IVectorView<super::syndication::SyndicationCategory>) -> HRESULT,
    fn get_Accepts(&self, out: *mut *mut super::super::foundation::collections::IVectorView<HString>) -> HRESULT
}}
impl IResourceCollection {
    #[inline] pub unsafe fn get_title(&self) -> Result<ComPtr<super::syndication::ISyndicationText>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Title)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_uri(&self) -> Result<ComPtr<super::super::foundation::Uri>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Uri)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_categories(&self) -> Result<ComPtr<super::super::foundation::collections::IVectorView<super::syndication::SyndicationCategory>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Categories)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_accepts(&self) -> Result<ComPtr<super::super::foundation::collections::IVectorView<HString>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Accepts)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
RT_CLASS!{class ResourceCollection: IResourceCollection}
DEFINE_IID!(IID_IServiceDocument, 2340341617, 10931, 19902, 139, 204, 119, 143, 146, 183, 94, 81);
RT_INTERFACE!{interface IServiceDocument(IServiceDocumentVtbl): IInspectable(IInspectableVtbl) [IID_IServiceDocument] {
    fn get_Workspaces(&self, out: *mut *mut super::super::foundation::collections::IVectorView<Workspace>) -> HRESULT
}}
impl IServiceDocument {
    #[inline] pub unsafe fn get_workspaces(&self) -> Result<ComPtr<super::super::foundation::collections::IVectorView<Workspace>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Workspaces)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
RT_CLASS!{class ServiceDocument: IServiceDocument}
DEFINE_IID!(IID_IWorkspace, 3021841979, 42168, 16438, 137, 197, 131, 195, 18, 102, 186, 73);
RT_INTERFACE!{interface IWorkspace(IWorkspaceVtbl): IInspectable(IInspectableVtbl) [IID_IWorkspace] {
    fn get_Title(&self, out: *mut *mut super::syndication::ISyndicationText) -> HRESULT,
    fn get_Collections(&self, out: *mut *mut super::super::foundation::collections::IVectorView<ResourceCollection>) -> HRESULT
}}
impl IWorkspace {
    #[inline] pub unsafe fn get_title(&self) -> Result<ComPtr<super::syndication::ISyndicationText>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Title)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_collections(&self) -> Result<ComPtr<super::super::foundation::collections::IVectorView<ResourceCollection>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Collections)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
RT_CLASS!{class Workspace: IWorkspace}
} // Windows.Web.AtomPub
