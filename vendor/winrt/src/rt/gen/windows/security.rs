pub mod credentials { // Windows.Security.Credentials
use ::prelude::*;
DEFINE_IID!(IID_ICredentialFactory, 1424954273, 48934, 18357, 151, 221, 222, 119, 155, 124, 173, 88);
RT_INTERFACE!{static interface ICredentialFactory(ICredentialFactoryVtbl): IInspectable(IInspectableVtbl) [IID_ICredentialFactory] {
    fn CreatePasswordCredential(&self, resource: HSTRING, userName: HSTRING, password: HSTRING, out: *mut *mut PasswordCredential) -> HRESULT
}}
impl ICredentialFactory {
    #[inline] pub unsafe fn create_password_credential(&self, resource: &HStringArg, userName: &HStringArg, password: &HStringArg) -> Result<ComPtr<PasswordCredential>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).CreatePasswordCredential)(self as *const _ as *mut _, resource.get(), userName.get(), password.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
DEFINE_IID!(IID_IKeyCredential, 2508582797, 17787, 18503, 177, 26, 250, 150, 11, 189, 177, 56);
RT_INTERFACE!{interface IKeyCredential(IKeyCredentialVtbl): IInspectable(IInspectableVtbl) [IID_IKeyCredential] {
    fn get_Name(&self, out: *mut HSTRING) -> HRESULT,
    #[cfg(not(feature="windows-storage"))] fn __Dummy1(&self) -> (),
    #[cfg(feature="windows-storage")] fn RetrievePublicKeyWithDefaultBlobType(&self, out: *mut *mut super::super::storage::streams::IBuffer) -> HRESULT,
    #[cfg(not(feature="windows-storage"))] fn __Dummy2(&self) -> (),
    #[cfg(feature="windows-storage")] fn RetrievePublicKeyWithBlobType(&self, blobType: super::cryptography::core::CryptographicPublicKeyBlobType, out: *mut *mut super::super::storage::streams::IBuffer) -> HRESULT,
    #[cfg(not(feature="windows-storage"))] fn __Dummy3(&self) -> (),
    #[cfg(feature="windows-storage")] fn RequestSignAsync(&self, data: *mut super::super::storage::streams::IBuffer, out: *mut *mut super::super::foundation::IAsyncOperation<KeyCredentialOperationResult>) -> HRESULT,
    fn GetAttestationAsync(&self, out: *mut *mut super::super::foundation::IAsyncOperation<KeyCredentialAttestationResult>) -> HRESULT
}}
impl IKeyCredential {
    #[inline] pub unsafe fn get_name(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Name)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[cfg(feature="windows-storage")] #[inline] pub unsafe fn retrieve_public_key_with_default_blob_type(&self) -> Result<ComPtr<super::super::storage::streams::IBuffer>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).RetrievePublicKeyWithDefaultBlobType)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[cfg(feature="windows-storage")] #[inline] pub unsafe fn retrieve_public_key_with_blob_type(&self, blobType: super::cryptography::core::CryptographicPublicKeyBlobType) -> Result<ComPtr<super::super::storage::streams::IBuffer>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).RetrievePublicKeyWithBlobType)(self as *const _ as *mut _, blobType, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[cfg(feature="windows-storage")] #[inline] pub unsafe fn request_sign_async(&self, data: &super::super::storage::streams::IBuffer) -> Result<ComPtr<super::super::foundation::IAsyncOperation<KeyCredentialOperationResult>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).RequestSignAsync)(self as *const _ as *mut _, data as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_attestation_async(&self) -> Result<ComPtr<super::super::foundation::IAsyncOperation<KeyCredentialAttestationResult>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetAttestationAsync)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
RT_CLASS!{class KeyCredential: IKeyCredential}
DEFINE_IID!(IID_IKeyCredentialAttestationResult, 2024453025, 41921, 16643, 182, 204, 71, 44, 68, 23, 28, 187);
RT_INTERFACE!{interface IKeyCredentialAttestationResult(IKeyCredentialAttestationResultVtbl): IInspectable(IInspectableVtbl) [IID_IKeyCredentialAttestationResult] {
    #[cfg(not(feature="windows-storage"))] fn __Dummy0(&self) -> (),
    #[cfg(feature="windows-storage")] fn get_CertificateChainBuffer(&self, out: *mut *mut super::super::storage::streams::IBuffer) -> HRESULT,
    #[cfg(not(feature="windows-storage"))] fn __Dummy1(&self) -> (),
    #[cfg(feature="windows-storage")] fn get_AttestationBuffer(&self, out: *mut *mut super::super::storage::streams::IBuffer) -> HRESULT,
    fn get_Status(&self, out: *mut KeyCredentialAttestationStatus) -> HRESULT
}}
impl IKeyCredentialAttestationResult {
    #[cfg(feature="windows-storage")] #[inline] pub unsafe fn get_certificate_chain_buffer(&self) -> Result<ComPtr<super::super::storage::streams::IBuffer>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_CertificateChainBuffer)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[cfg(feature="windows-storage")] #[inline] pub unsafe fn get_attestation_buffer(&self) -> Result<ComPtr<super::super::storage::streams::IBuffer>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_AttestationBuffer)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_status(&self) -> Result<KeyCredentialAttestationStatus> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_Status)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
}
RT_CLASS!{class KeyCredentialAttestationResult: IKeyCredentialAttestationResult}
RT_ENUM! { enum KeyCredentialAttestationStatus: i32 {
    Success (KeyCredentialAttestationStatus_Success) = 0, UnknownError (KeyCredentialAttestationStatus_UnknownError) = 1, NotSupported (KeyCredentialAttestationStatus_NotSupported) = 2, TemporaryFailure (KeyCredentialAttestationStatus_TemporaryFailure) = 3,
}}
RT_ENUM! { enum KeyCredentialCreationOption: i32 {
    ReplaceExisting (KeyCredentialCreationOption_ReplaceExisting) = 0, FailIfExists (KeyCredentialCreationOption_FailIfExists) = 1,
}}
RT_CLASS!{static class KeyCredentialManager}
impl RtActivatable<IKeyCredentialManagerStatics> for KeyCredentialManager {}
impl KeyCredentialManager {
    #[inline] pub fn is_supported_async() -> Result<ComPtr<super::super::foundation::IAsyncOperation<bool>>> { unsafe {
        <Self as RtActivatable<IKeyCredentialManagerStatics>>::get_activation_factory().is_supported_async()
    }}
    #[inline] pub fn renew_attestation_async() -> Result<ComPtr<super::super::foundation::IAsyncAction>> { unsafe {
        <Self as RtActivatable<IKeyCredentialManagerStatics>>::get_activation_factory().renew_attestation_async()
    }}
    #[inline] pub fn request_create_async(name: &HStringArg, option: KeyCredentialCreationOption) -> Result<ComPtr<super::super::foundation::IAsyncOperation<KeyCredentialRetrievalResult>>> { unsafe {
        <Self as RtActivatable<IKeyCredentialManagerStatics>>::get_activation_factory().request_create_async(name, option)
    }}
    #[inline] pub fn open_async(name: &HStringArg) -> Result<ComPtr<super::super::foundation::IAsyncOperation<KeyCredentialRetrievalResult>>> { unsafe {
        <Self as RtActivatable<IKeyCredentialManagerStatics>>::get_activation_factory().open_async(name)
    }}
    #[inline] pub fn delete_async(name: &HStringArg) -> Result<ComPtr<super::super::foundation::IAsyncAction>> { unsafe {
        <Self as RtActivatable<IKeyCredentialManagerStatics>>::get_activation_factory().delete_async(name)
    }}
}
DEFINE_CLSID!(KeyCredentialManager(&[87,105,110,100,111,119,115,46,83,101,99,117,114,105,116,121,46,67,114,101,100,101,110,116,105,97,108,115,46,75,101,121,67,114,101,100,101,110,116,105,97,108,77,97,110,97,103,101,114,0]) [CLSID_KeyCredentialManager]);
DEFINE_IID!(IID_IKeyCredentialManagerStatics, 1789675147, 3825, 19680, 130, 144, 65, 6, 218, 106, 99, 181);
RT_INTERFACE!{static interface IKeyCredentialManagerStatics(IKeyCredentialManagerStaticsVtbl): IInspectable(IInspectableVtbl) [IID_IKeyCredentialManagerStatics] {
    fn IsSupportedAsync(&self, out: *mut *mut super::super::foundation::IAsyncOperation<bool>) -> HRESULT,
    fn RenewAttestationAsync(&self, out: *mut *mut super::super::foundation::IAsyncAction) -> HRESULT,
    fn RequestCreateAsync(&self, name: HSTRING, option: KeyCredentialCreationOption, out: *mut *mut super::super::foundation::IAsyncOperation<KeyCredentialRetrievalResult>) -> HRESULT,
    fn OpenAsync(&self, name: HSTRING, out: *mut *mut super::super::foundation::IAsyncOperation<KeyCredentialRetrievalResult>) -> HRESULT,
    fn DeleteAsync(&self, name: HSTRING, out: *mut *mut super::super::foundation::IAsyncAction) -> HRESULT
}}
impl IKeyCredentialManagerStatics {
    #[inline] pub unsafe fn is_supported_async(&self) -> Result<ComPtr<super::super::foundation::IAsyncOperation<bool>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).IsSupportedAsync)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn renew_attestation_async(&self) -> Result<ComPtr<super::super::foundation::IAsyncAction>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).RenewAttestationAsync)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn request_create_async(&self, name: &HStringArg, option: KeyCredentialCreationOption) -> Result<ComPtr<super::super::foundation::IAsyncOperation<KeyCredentialRetrievalResult>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).RequestCreateAsync)(self as *const _ as *mut _, name.get(), option, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn open_async(&self, name: &HStringArg) -> Result<ComPtr<super::super::foundation::IAsyncOperation<KeyCredentialRetrievalResult>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).OpenAsync)(self as *const _ as *mut _, name.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn delete_async(&self, name: &HStringArg) -> Result<ComPtr<super::super::foundation::IAsyncAction>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).DeleteAsync)(self as *const _ as *mut _, name.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
DEFINE_IID!(IID_IKeyCredentialOperationResult, 4114056897, 21089, 19677, 151, 109, 204, 144, 154, 199, 22, 32);
RT_INTERFACE!{interface IKeyCredentialOperationResult(IKeyCredentialOperationResultVtbl): IInspectable(IInspectableVtbl) [IID_IKeyCredentialOperationResult] {
    #[cfg(not(feature="windows-storage"))] fn __Dummy0(&self) -> (),
    #[cfg(feature="windows-storage")] fn get_Result(&self, out: *mut *mut super::super::storage::streams::IBuffer) -> HRESULT,
    fn get_Status(&self, out: *mut KeyCredentialStatus) -> HRESULT
}}
impl IKeyCredentialOperationResult {
    #[cfg(feature="windows-storage")] #[inline] pub unsafe fn get_result(&self) -> Result<ComPtr<super::super::storage::streams::IBuffer>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Result)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_status(&self) -> Result<KeyCredentialStatus> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_Status)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
}
RT_CLASS!{class KeyCredentialOperationResult: IKeyCredentialOperationResult}
DEFINE_IID!(IID_IKeyCredentialRetrievalResult, 1489860355, 36231, 16969, 155, 88, 246, 89, 140, 201, 100, 78);
RT_INTERFACE!{interface IKeyCredentialRetrievalResult(IKeyCredentialRetrievalResultVtbl): IInspectable(IInspectableVtbl) [IID_IKeyCredentialRetrievalResult] {
    fn get_Credential(&self, out: *mut *mut KeyCredential) -> HRESULT,
    fn get_Status(&self, out: *mut KeyCredentialStatus) -> HRESULT
}}
impl IKeyCredentialRetrievalResult {
    #[inline] pub unsafe fn get_credential(&self) -> Result<ComPtr<KeyCredential>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Credential)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_status(&self) -> Result<KeyCredentialStatus> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_Status)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
}
RT_CLASS!{class KeyCredentialRetrievalResult: IKeyCredentialRetrievalResult}
RT_ENUM! { enum KeyCredentialStatus: i32 {
    Success (KeyCredentialStatus_Success) = 0, UnknownError (KeyCredentialStatus_UnknownError) = 1, NotFound (KeyCredentialStatus_NotFound) = 2, UserCanceled (KeyCredentialStatus_UserCanceled) = 3, UserPrefersPassword (KeyCredentialStatus_UserPrefersPassword) = 4, CredentialAlreadyExists (KeyCredentialStatus_CredentialAlreadyExists) = 5, SecurityDeviceLocked (KeyCredentialStatus_SecurityDeviceLocked) = 6,
}}
DEFINE_IID!(IID_IPasswordCredential, 1790019977, 50976, 16807, 166, 193, 254, 173, 179, 99, 41, 160);
RT_INTERFACE!{interface IPasswordCredential(IPasswordCredentialVtbl): IInspectable(IInspectableVtbl) [IID_IPasswordCredential] {
    fn get_Resource(&self, out: *mut HSTRING) -> HRESULT,
    fn put_Resource(&self, resource: HSTRING) -> HRESULT,
    fn get_UserName(&self, out: *mut HSTRING) -> HRESULT,
    fn put_UserName(&self, userName: HSTRING) -> HRESULT,
    fn get_Password(&self, out: *mut HSTRING) -> HRESULT,
    fn put_Password(&self, password: HSTRING) -> HRESULT,
    fn RetrievePassword(&self) -> HRESULT,
    fn get_Properties(&self, out: *mut *mut super::super::foundation::collections::IPropertySet) -> HRESULT
}}
impl IPasswordCredential {
    #[inline] pub unsafe fn get_resource(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Resource)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_resource(&self, resource: &HStringArg) -> Result<()> {
        let hr = ((*self.lpVtbl).put_Resource)(self as *const _ as *mut _, resource.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_user_name(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_UserName)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_user_name(&self, userName: &HStringArg) -> Result<()> {
        let hr = ((*self.lpVtbl).put_UserName)(self as *const _ as *mut _, userName.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_password(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Password)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_password(&self, password: &HStringArg) -> Result<()> {
        let hr = ((*self.lpVtbl).put_Password)(self as *const _ as *mut _, password.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn retrieve_password(&self) -> Result<()> {
        let hr = ((*self.lpVtbl).RetrievePassword)(self as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_properties(&self) -> Result<ComPtr<super::super::foundation::collections::IPropertySet>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Properties)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
RT_CLASS!{class PasswordCredential: IPasswordCredential}
impl RtActivatable<ICredentialFactory> for PasswordCredential {}
impl RtActivatable<IActivationFactory> for PasswordCredential {}
impl PasswordCredential {
    #[inline] pub fn create_password_credential(resource: &HStringArg, userName: &HStringArg, password: &HStringArg) -> Result<ComPtr<PasswordCredential>> { unsafe {
        <Self as RtActivatable<ICredentialFactory>>::get_activation_factory().create_password_credential(resource, userName, password)
    }}
}
DEFINE_CLSID!(PasswordCredential(&[87,105,110,100,111,119,115,46,83,101,99,117,114,105,116,121,46,67,114,101,100,101,110,116,105,97,108,115,46,80,97,115,115,119,111,114,100,67,114,101,100,101,110,116,105,97,108,0]) [CLSID_PasswordCredential]);
RT_CLASS!{class PasswordCredentialPropertyStore: super::super::foundation::collections::IPropertySet}
impl RtActivatable<IActivationFactory> for PasswordCredentialPropertyStore {}
DEFINE_CLSID!(PasswordCredentialPropertyStore(&[87,105,110,100,111,119,115,46,83,101,99,117,114,105,116,121,46,67,114,101,100,101,110,116,105,97,108,115,46,80,97,115,115,119,111,114,100,67,114,101,100,101,110,116,105,97,108,80,114,111,112,101,114,116,121,83,116,111,114,101,0]) [CLSID_PasswordCredentialPropertyStore]);
DEFINE_IID!(IID_IPasswordVault, 1643981835, 51412, 18625, 165, 79, 188, 90, 100, 32, 90, 242);
RT_INTERFACE!{interface IPasswordVault(IPasswordVaultVtbl): IInspectable(IInspectableVtbl) [IID_IPasswordVault] {
    fn Add(&self, credential: *mut PasswordCredential) -> HRESULT,
    fn Remove(&self, credential: *mut PasswordCredential) -> HRESULT,
    fn Retrieve(&self, resource: HSTRING, userName: HSTRING, out: *mut *mut PasswordCredential) -> HRESULT,
    fn FindAllByResource(&self, resource: HSTRING, out: *mut *mut super::super::foundation::collections::IVectorView<PasswordCredential>) -> HRESULT,
    fn FindAllByUserName(&self, userName: HSTRING, out: *mut *mut super::super::foundation::collections::IVectorView<PasswordCredential>) -> HRESULT,
    fn RetrieveAll(&self, out: *mut *mut super::super::foundation::collections::IVectorView<PasswordCredential>) -> HRESULT
}}
impl IPasswordVault {
    #[inline] pub unsafe fn add(&self, credential: &PasswordCredential) -> Result<()> {
        let hr = ((*self.lpVtbl).Add)(self as *const _ as *mut _, credential as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn remove(&self, credential: &PasswordCredential) -> Result<()> {
        let hr = ((*self.lpVtbl).Remove)(self as *const _ as *mut _, credential as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn retrieve(&self, resource: &HStringArg, userName: &HStringArg) -> Result<ComPtr<PasswordCredential>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).Retrieve)(self as *const _ as *mut _, resource.get(), userName.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn find_all_by_resource(&self, resource: &HStringArg) -> Result<ComPtr<super::super::foundation::collections::IVectorView<PasswordCredential>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).FindAllByResource)(self as *const _ as *mut _, resource.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn find_all_by_user_name(&self, userName: &HStringArg) -> Result<ComPtr<super::super::foundation::collections::IVectorView<PasswordCredential>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).FindAllByUserName)(self as *const _ as *mut _, userName.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn retrieve_all(&self) -> Result<ComPtr<super::super::foundation::collections::IVectorView<PasswordCredential>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).RetrieveAll)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
RT_CLASS!{class PasswordVault: IPasswordVault}
impl RtActivatable<IActivationFactory> for PasswordVault {}
DEFINE_CLSID!(PasswordVault(&[87,105,110,100,111,119,115,46,83,101,99,117,114,105,116,121,46,67,114,101,100,101,110,116,105,97,108,115,46,80,97,115,115,119,111,114,100,86,97,117,108,116,0]) [CLSID_PasswordVault]);
DEFINE_IID!(IID_IWebAccount, 1766276786, 32817, 18878, 128, 187, 150, 203, 70, 217, 154, 186);
RT_INTERFACE!{interface IWebAccount(IWebAccountVtbl): IInspectable(IInspectableVtbl) [IID_IWebAccount] {
    fn get_WebAccountProvider(&self, out: *mut *mut WebAccountProvider) -> HRESULT,
    fn get_UserName(&self, out: *mut HSTRING) -> HRESULT,
    fn get_State(&self, out: *mut WebAccountState) -> HRESULT
}}
impl IWebAccount {
    #[inline] pub unsafe fn get_web_account_provider(&self) -> Result<ComPtr<WebAccountProvider>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_WebAccountProvider)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_user_name(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_UserName)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_state(&self) -> Result<WebAccountState> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_State)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
}
RT_CLASS!{class WebAccount: IWebAccount}
impl RtActivatable<IWebAccountFactory> for WebAccount {}
impl WebAccount {
    #[inline] pub fn create_web_account(webAccountProvider: &WebAccountProvider, userName: &HStringArg, state: WebAccountState) -> Result<ComPtr<WebAccount>> { unsafe {
        <Self as RtActivatable<IWebAccountFactory>>::get_activation_factory().create_web_account(webAccountProvider, userName, state)
    }}
}
DEFINE_CLSID!(WebAccount(&[87,105,110,100,111,119,115,46,83,101,99,117,114,105,116,121,46,67,114,101,100,101,110,116,105,97,108,115,46,87,101,98,65,99,99,111,117,110,116,0]) [CLSID_WebAccount]);
DEFINE_IID!(IID_IWebAccount2, 2069288696, 39179, 20149, 148, 167, 86, 33, 243, 168, 184, 36);
RT_INTERFACE!{interface IWebAccount2(IWebAccount2Vtbl): IInspectable(IInspectableVtbl) [IID_IWebAccount2] {
    fn get_Id(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Properties(&self, out: *mut *mut super::super::foundation::collections::IMapView<HString, HString>) -> HRESULT,
    #[cfg(not(feature="windows-storage"))] fn __Dummy2(&self) -> (),
    #[cfg(feature="windows-storage")] fn GetPictureAsync(&self, desizedSize: WebAccountPictureSize, out: *mut *mut super::super::foundation::IAsyncOperation<super::super::storage::streams::IRandomAccessStream>) -> HRESULT,
    fn SignOutAsync(&self, out: *mut *mut super::super::foundation::IAsyncAction) -> HRESULT,
    fn SignOutWithClientIdAsync(&self, clientId: HSTRING, out: *mut *mut super::super::foundation::IAsyncAction) -> HRESULT
}}
impl IWebAccount2 {
    #[inline] pub unsafe fn get_id(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Id)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_properties(&self) -> Result<ComPtr<super::super::foundation::collections::IMapView<HString, HString>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Properties)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[cfg(feature="windows-storage")] #[inline] pub unsafe fn get_picture_async(&self, desizedSize: WebAccountPictureSize) -> Result<ComPtr<super::super::foundation::IAsyncOperation<super::super::storage::streams::IRandomAccessStream>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetPictureAsync)(self as *const _ as *mut _, desizedSize, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn sign_out_async(&self) -> Result<ComPtr<super::super::foundation::IAsyncAction>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).SignOutAsync)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn sign_out_with_client_id_async(&self, clientId: &HStringArg) -> Result<ComPtr<super::super::foundation::IAsyncAction>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).SignOutWithClientIdAsync)(self as *const _ as *mut _, clientId.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
DEFINE_IID!(IID_IWebAccountFactory, 2895838009, 7657, 20114, 183, 143, 5, 129, 168, 127, 110, 92);
RT_INTERFACE!{static interface IWebAccountFactory(IWebAccountFactoryVtbl): IInspectable(IInspectableVtbl) [IID_IWebAccountFactory] {
    fn CreateWebAccount(&self, webAccountProvider: *mut WebAccountProvider, userName: HSTRING, state: WebAccountState, out: *mut *mut WebAccount) -> HRESULT
}}
impl IWebAccountFactory {
    #[inline] pub unsafe fn create_web_account(&self, webAccountProvider: &WebAccountProvider, userName: &HStringArg, state: WebAccountState) -> Result<ComPtr<WebAccount>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).CreateWebAccount)(self as *const _ as *mut _, webAccountProvider as *const _ as *mut _, userName.get(), state, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
RT_ENUM! { enum WebAccountPictureSize: i32 {
    Size64x64 (WebAccountPictureSize_Size64x64) = 64, Size208x208 (WebAccountPictureSize_Size208x208) = 208, Size424x424 (WebAccountPictureSize_Size424x424) = 424, Size1080x1080 (WebAccountPictureSize_Size1080x1080) = 1080,
}}
DEFINE_IID!(IID_IWebAccountProvider, 702335171, 31417, 19068, 163, 54, 185, 66, 249, 219, 247, 199);
RT_INTERFACE!{interface IWebAccountProvider(IWebAccountProviderVtbl): IInspectable(IInspectableVtbl) [IID_IWebAccountProvider] {
    fn get_Id(&self, out: *mut HSTRING) -> HRESULT,
    fn get_DisplayName(&self, out: *mut HSTRING) -> HRESULT,
    fn get_IconUri(&self, out: *mut *mut super::super::foundation::Uri) -> HRESULT
}}
impl IWebAccountProvider {
    #[inline] pub unsafe fn get_id(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Id)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_display_name(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_DisplayName)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_icon_uri(&self) -> Result<ComPtr<super::super::foundation::Uri>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_IconUri)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
RT_CLASS!{class WebAccountProvider: IWebAccountProvider}
impl RtActivatable<IWebAccountProviderFactory> for WebAccountProvider {}
impl WebAccountProvider {
    #[inline] pub fn create_web_account_provider(id: &HStringArg, displayName: &HStringArg, iconUri: &super::super::foundation::Uri) -> Result<ComPtr<WebAccountProvider>> { unsafe {
        <Self as RtActivatable<IWebAccountProviderFactory>>::get_activation_factory().create_web_account_provider(id, displayName, iconUri)
    }}
}
DEFINE_CLSID!(WebAccountProvider(&[87,105,110,100,111,119,115,46,83,101,99,117,114,105,116,121,46,67,114,101,100,101,110,116,105,97,108,115,46,87,101,98,65,99,99,111,117,110,116,80,114,111,118,105,100,101,114,0]) [CLSID_WebAccountProvider]);
DEFINE_IID!(IID_IWebAccountProvider2, 1241639685, 20034, 16852, 181, 24, 224, 8, 165, 22, 54, 20);
RT_INTERFACE!{interface IWebAccountProvider2(IWebAccountProvider2Vtbl): IInspectable(IInspectableVtbl) [IID_IWebAccountProvider2] {
    fn get_DisplayPurpose(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Authority(&self, out: *mut HSTRING) -> HRESULT
}}
impl IWebAccountProvider2 {
    #[inline] pub unsafe fn get_display_purpose(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_DisplayPurpose)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_authority(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Authority)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
}
DEFINE_IID!(IID_IWebAccountProvider3, 3659288971, 38669, 19785, 130, 92, 242, 112, 111, 140, 167, 254);
RT_INTERFACE!{interface IWebAccountProvider3(IWebAccountProvider3Vtbl): IInspectable(IInspectableVtbl) [IID_IWebAccountProvider3] {
    #[cfg(feature="windows-system")] fn get_User(&self, out: *mut *mut super::super::system::User) -> HRESULT
}}
impl IWebAccountProvider3 {
    #[cfg(feature="windows-system")] #[inline] pub unsafe fn get_user(&self) -> Result<ComPtr<super::super::system::User>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_User)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
DEFINE_IID!(IID_IWebAccountProviderFactory, 494304753, 57825, 19354, 167, 116, 92, 124, 126, 59, 243, 113);
RT_INTERFACE!{static interface IWebAccountProviderFactory(IWebAccountProviderFactoryVtbl): IInspectable(IInspectableVtbl) [IID_IWebAccountProviderFactory] {
    fn CreateWebAccountProvider(&self, id: HSTRING, displayName: HSTRING, iconUri: *mut super::super::foundation::Uri, out: *mut *mut WebAccountProvider) -> HRESULT
}}
impl IWebAccountProviderFactory {
    #[inline] pub unsafe fn create_web_account_provider(&self, id: &HStringArg, displayName: &HStringArg, iconUri: &super::super::foundation::Uri) -> Result<ComPtr<WebAccountProvider>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).CreateWebAccountProvider)(self as *const _ as *mut _, id.get(), displayName.get(), iconUri as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
RT_ENUM! { enum WebAccountState: i32 {
    None (WebAccountState_None) = 0, Connected (WebAccountState_Connected) = 1, Error (WebAccountState_Error) = 2,
}}
pub mod ui { // Windows.Security.Credentials.UI
use ::prelude::*;
RT_ENUM! { enum AuthenticationProtocol: i32 {
    Basic (AuthenticationProtocol_Basic) = 0, Digest (AuthenticationProtocol_Digest) = 1, Ntlm (AuthenticationProtocol_Ntlm) = 2, Kerberos (AuthenticationProtocol_Kerberos) = 3, Negotiate (AuthenticationProtocol_Negotiate) = 4, CredSsp (AuthenticationProtocol_CredSsp) = 5, Custom (AuthenticationProtocol_Custom) = 6,
}}
RT_CLASS!{static class CredentialPicker}
impl RtActivatable<ICredentialPickerStatics> for CredentialPicker {}
impl CredentialPicker {
    #[inline] pub fn pick_with_options_async(options: &CredentialPickerOptions) -> Result<ComPtr<::rt::gen::windows::foundation::IAsyncOperation<CredentialPickerResults>>> { unsafe {
        <Self as RtActivatable<ICredentialPickerStatics>>::get_activation_factory().pick_with_options_async(options)
    }}
    #[inline] pub fn pick_with_message_async(targetName: &HStringArg, message: &HStringArg) -> Result<ComPtr<::rt::gen::windows::foundation::IAsyncOperation<CredentialPickerResults>>> { unsafe {
        <Self as RtActivatable<ICredentialPickerStatics>>::get_activation_factory().pick_with_message_async(targetName, message)
    }}
    #[inline] pub fn pick_with_caption_async(targetName: &HStringArg, message: &HStringArg, caption: &HStringArg) -> Result<ComPtr<::rt::gen::windows::foundation::IAsyncOperation<CredentialPickerResults>>> { unsafe {
        <Self as RtActivatable<ICredentialPickerStatics>>::get_activation_factory().pick_with_caption_async(targetName, message, caption)
    }}
}
DEFINE_CLSID!(CredentialPicker(&[87,105,110,100,111,119,115,46,83,101,99,117,114,105,116,121,46,67,114,101,100,101,110,116,105,97,108,115,46,85,73,46,67,114,101,100,101,110,116,105,97,108,80,105,99,107,101,114,0]) [CLSID_CredentialPicker]);
DEFINE_IID!(IID_ICredentialPickerOptions, 2522483532, 38394, 18047, 153, 43, 11, 34, 229, 133, 155, 246);
RT_INTERFACE!{interface ICredentialPickerOptions(ICredentialPickerOptionsVtbl): IInspectable(IInspectableVtbl) [IID_ICredentialPickerOptions] {
    fn put_Caption(&self, value: HSTRING) -> HRESULT,
    fn get_Caption(&self, out: *mut HSTRING) -> HRESULT,
    fn put_Message(&self, value: HSTRING) -> HRESULT,
    fn get_Message(&self, out: *mut HSTRING) -> HRESULT,
    fn put_ErrorCode(&self, value: u32) -> HRESULT,
    fn get_ErrorCode(&self, out: *mut u32) -> HRESULT,
    fn put_TargetName(&self, value: HSTRING) -> HRESULT,
    fn get_TargetName(&self, out: *mut HSTRING) -> HRESULT,
    fn put_AuthenticationProtocol(&self, value: AuthenticationProtocol) -> HRESULT,
    fn get_AuthenticationProtocol(&self, out: *mut AuthenticationProtocol) -> HRESULT,
    fn put_CustomAuthenticationProtocol(&self, value: HSTRING) -> HRESULT,
    fn get_CustomAuthenticationProtocol(&self, out: *mut HSTRING) -> HRESULT,
    #[cfg(not(feature="windows-storage"))] fn __Dummy12(&self) -> (),
    #[cfg(feature="windows-storage")] fn put_PreviousCredential(&self, value: *mut ::rt::gen::windows::storage::streams::IBuffer) -> HRESULT,
    #[cfg(not(feature="windows-storage"))] fn __Dummy13(&self) -> (),
    #[cfg(feature="windows-storage")] fn get_PreviousCredential(&self, out: *mut *mut ::rt::gen::windows::storage::streams::IBuffer) -> HRESULT,
    fn put_AlwaysDisplayDialog(&self, value: bool) -> HRESULT,
    fn get_AlwaysDisplayDialog(&self, out: *mut bool) -> HRESULT,
    fn put_CallerSavesCredential(&self, value: bool) -> HRESULT,
    fn get_CallerSavesCredential(&self, out: *mut bool) -> HRESULT,
    fn put_CredentialSaveOption(&self, value: CredentialSaveOption) -> HRESULT,
    fn get_CredentialSaveOption(&self, out: *mut CredentialSaveOption) -> HRESULT
}}
impl ICredentialPickerOptions {
    #[inline] pub unsafe fn set_caption(&self, value: &HStringArg) -> Result<()> {
        let hr = ((*self.lpVtbl).put_Caption)(self as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_caption(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Caption)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_message(&self, value: &HStringArg) -> Result<()> {
        let hr = ((*self.lpVtbl).put_Message)(self as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_message(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Message)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_error_code(&self, value: u32) -> Result<()> {
        let hr = ((*self.lpVtbl).put_ErrorCode)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_error_code(&self) -> Result<u32> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_ErrorCode)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_target_name(&self, value: &HStringArg) -> Result<()> {
        let hr = ((*self.lpVtbl).put_TargetName)(self as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_target_name(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_TargetName)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_authentication_protocol(&self, value: AuthenticationProtocol) -> Result<()> {
        let hr = ((*self.lpVtbl).put_AuthenticationProtocol)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_authentication_protocol(&self) -> Result<AuthenticationProtocol> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_AuthenticationProtocol)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_custom_authentication_protocol(&self, value: &HStringArg) -> Result<()> {
        let hr = ((*self.lpVtbl).put_CustomAuthenticationProtocol)(self as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_custom_authentication_protocol(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_CustomAuthenticationProtocol)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[cfg(feature="windows-storage")] #[inline] pub unsafe fn set_previous_credential(&self, value: &::rt::gen::windows::storage::streams::IBuffer) -> Result<()> {
        let hr = ((*self.lpVtbl).put_PreviousCredential)(self as *const _ as *mut _, value as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[cfg(feature="windows-storage")] #[inline] pub unsafe fn get_previous_credential(&self) -> Result<ComPtr<::rt::gen::windows::storage::streams::IBuffer>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_PreviousCredential)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_always_display_dialog(&self, value: bool) -> Result<()> {
        let hr = ((*self.lpVtbl).put_AlwaysDisplayDialog)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_always_display_dialog(&self) -> Result<bool> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_AlwaysDisplayDialog)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_caller_saves_credential(&self, value: bool) -> Result<()> {
        let hr = ((*self.lpVtbl).put_CallerSavesCredential)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_caller_saves_credential(&self) -> Result<bool> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_CallerSavesCredential)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_credential_save_option(&self, value: CredentialSaveOption) -> Result<()> {
        let hr = ((*self.lpVtbl).put_CredentialSaveOption)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_credential_save_option(&self) -> Result<CredentialSaveOption> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_CredentialSaveOption)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
}
RT_CLASS!{class CredentialPickerOptions: ICredentialPickerOptions}
impl RtActivatable<IActivationFactory> for CredentialPickerOptions {}
DEFINE_CLSID!(CredentialPickerOptions(&[87,105,110,100,111,119,115,46,83,101,99,117,114,105,116,121,46,67,114,101,100,101,110,116,105,97,108,115,46,85,73,46,67,114,101,100,101,110,116,105,97,108,80,105,99,107,101,114,79,112,116,105,111,110,115,0]) [CLSID_CredentialPickerOptions]);
DEFINE_IID!(IID_ICredentialPickerResults, 424212890, 52272, 16652, 156, 56, 204, 8, 132, 197, 179, 215);
RT_INTERFACE!{interface ICredentialPickerResults(ICredentialPickerResultsVtbl): IInspectable(IInspectableVtbl) [IID_ICredentialPickerResults] {
    fn get_ErrorCode(&self, out: *mut u32) -> HRESULT,
    fn get_CredentialSaveOption(&self, out: *mut CredentialSaveOption) -> HRESULT,
    fn get_CredentialSaved(&self, out: *mut bool) -> HRESULT,
    #[cfg(not(feature="windows-storage"))] fn __Dummy3(&self) -> (),
    #[cfg(feature="windows-storage")] fn get_Credential(&self, out: *mut *mut ::rt::gen::windows::storage::streams::IBuffer) -> HRESULT,
    fn get_CredentialDomainName(&self, out: *mut HSTRING) -> HRESULT,
    fn get_CredentialUserName(&self, out: *mut HSTRING) -> HRESULT,
    fn get_CredentialPassword(&self, out: *mut HSTRING) -> HRESULT
}}
impl ICredentialPickerResults {
    #[inline] pub unsafe fn get_error_code(&self) -> Result<u32> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_ErrorCode)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_credential_save_option(&self) -> Result<CredentialSaveOption> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_CredentialSaveOption)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_credential_saved(&self) -> Result<bool> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_CredentialSaved)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[cfg(feature="windows-storage")] #[inline] pub unsafe fn get_credential(&self) -> Result<ComPtr<::rt::gen::windows::storage::streams::IBuffer>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Credential)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_credential_domain_name(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_CredentialDomainName)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_credential_user_name(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_CredentialUserName)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_credential_password(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_CredentialPassword)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
}
RT_CLASS!{class CredentialPickerResults: ICredentialPickerResults}
DEFINE_IID!(IID_ICredentialPickerStatics, 2855951475, 51690, 18306, 153, 251, 230, 215, 233, 56, 225, 45);
RT_INTERFACE!{static interface ICredentialPickerStatics(ICredentialPickerStaticsVtbl): IInspectable(IInspectableVtbl) [IID_ICredentialPickerStatics] {
    fn PickWithOptionsAsync(&self, options: *mut CredentialPickerOptions, out: *mut *mut ::rt::gen::windows::foundation::IAsyncOperation<CredentialPickerResults>) -> HRESULT,
    fn PickWithMessageAsync(&self, targetName: HSTRING, message: HSTRING, out: *mut *mut ::rt::gen::windows::foundation::IAsyncOperation<CredentialPickerResults>) -> HRESULT,
    fn PickWithCaptionAsync(&self, targetName: HSTRING, message: HSTRING, caption: HSTRING, out: *mut *mut ::rt::gen::windows::foundation::IAsyncOperation<CredentialPickerResults>) -> HRESULT
}}
impl ICredentialPickerStatics {
    #[inline] pub unsafe fn pick_with_options_async(&self, options: &CredentialPickerOptions) -> Result<ComPtr<::rt::gen::windows::foundation::IAsyncOperation<CredentialPickerResults>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).PickWithOptionsAsync)(self as *const _ as *mut _, options as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn pick_with_message_async(&self, targetName: &HStringArg, message: &HStringArg) -> Result<ComPtr<::rt::gen::windows::foundation::IAsyncOperation<CredentialPickerResults>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).PickWithMessageAsync)(self as *const _ as *mut _, targetName.get(), message.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn pick_with_caption_async(&self, targetName: &HStringArg, message: &HStringArg, caption: &HStringArg) -> Result<ComPtr<::rt::gen::windows::foundation::IAsyncOperation<CredentialPickerResults>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).PickWithCaptionAsync)(self as *const _ as *mut _, targetName.get(), message.get(), caption.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
RT_ENUM! { enum CredentialSaveOption: i32 {
    Unselected (CredentialSaveOption_Unselected) = 0, Selected (CredentialSaveOption_Selected) = 1, Hidden (CredentialSaveOption_Hidden) = 2,
}}
RT_ENUM! { enum UserConsentVerificationResult: i32 {
    Verified (UserConsentVerificationResult_Verified) = 0, DeviceNotPresent (UserConsentVerificationResult_DeviceNotPresent) = 1, NotConfiguredForUser (UserConsentVerificationResult_NotConfiguredForUser) = 2, DisabledByPolicy (UserConsentVerificationResult_DisabledByPolicy) = 3, DeviceBusy (UserConsentVerificationResult_DeviceBusy) = 4, RetriesExhausted (UserConsentVerificationResult_RetriesExhausted) = 5, Canceled (UserConsentVerificationResult_Canceled) = 6,
}}
RT_CLASS!{static class UserConsentVerifier}
impl RtActivatable<IUserConsentVerifierStatics> for UserConsentVerifier {}
impl UserConsentVerifier {
    #[inline] pub fn check_availability_async() -> Result<ComPtr<::rt::gen::windows::foundation::IAsyncOperation<UserConsentVerifierAvailability>>> { unsafe {
        <Self as RtActivatable<IUserConsentVerifierStatics>>::get_activation_factory().check_availability_async()
    }}
    #[inline] pub fn request_verification_async(message: &HStringArg) -> Result<ComPtr<::rt::gen::windows::foundation::IAsyncOperation<UserConsentVerificationResult>>> { unsafe {
        <Self as RtActivatable<IUserConsentVerifierStatics>>::get_activation_factory().request_verification_async(message)
    }}
}
DEFINE_CLSID!(UserConsentVerifier(&[87,105,110,100,111,119,115,46,83,101,99,117,114,105,116,121,46,67,114,101,100,101,110,116,105,97,108,115,46,85,73,46,85,115,101,114,67,111,110,115,101,110,116,86,101,114,105,102,105,101,114,0]) [CLSID_UserConsentVerifier]);
RT_ENUM! { enum UserConsentVerifierAvailability: i32 {
    Available (UserConsentVerifierAvailability_Available) = 0, DeviceNotPresent (UserConsentVerifierAvailability_DeviceNotPresent) = 1, NotConfiguredForUser (UserConsentVerifierAvailability_NotConfiguredForUser) = 2, DisabledByPolicy (UserConsentVerifierAvailability_DisabledByPolicy) = 3, DeviceBusy (UserConsentVerifierAvailability_DeviceBusy) = 4,
}}
DEFINE_IID!(IID_IUserConsentVerifierStatics, 2941206417, 22092, 19932, 184, 181, 151, 52, 71, 98, 124, 101);
RT_INTERFACE!{static interface IUserConsentVerifierStatics(IUserConsentVerifierStaticsVtbl): IInspectable(IInspectableVtbl) [IID_IUserConsentVerifierStatics] {
    fn CheckAvailabilityAsync(&self, out: *mut *mut ::rt::gen::windows::foundation::IAsyncOperation<UserConsentVerifierAvailability>) -> HRESULT,
    fn RequestVerificationAsync(&self, message: HSTRING, out: *mut *mut ::rt::gen::windows::foundation::IAsyncOperation<UserConsentVerificationResult>) -> HRESULT
}}
impl IUserConsentVerifierStatics {
    #[inline] pub unsafe fn check_availability_async(&self) -> Result<ComPtr<::rt::gen::windows::foundation::IAsyncOperation<UserConsentVerifierAvailability>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).CheckAvailabilityAsync)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn request_verification_async(&self, message: &HStringArg) -> Result<ComPtr<::rt::gen::windows::foundation::IAsyncOperation<UserConsentVerificationResult>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).RequestVerificationAsync)(self as *const _ as *mut _, message.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
} // Windows.Security.Credentials.UI
} // Windows.Security.Credentials
pub mod authentication { // Windows.Security.Authentication
pub mod identity { // Windows.Security.Authentication.Identity
use ::prelude::*;
DEFINE_IID!(IID_IEnterpriseKeyCredentialRegistrationInfo, 942807756, 26411, 18467, 182, 3, 107, 60, 117, 61, 175, 151);
RT_INTERFACE!{interface IEnterpriseKeyCredentialRegistrationInfo(IEnterpriseKeyCredentialRegistrationInfoVtbl): IInspectable(IInspectableVtbl) [IID_IEnterpriseKeyCredentialRegistrationInfo] {
    fn get_TenantId(&self, out: *mut HSTRING) -> HRESULT,
    fn get_TenantName(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Subject(&self, out: *mut HSTRING) -> HRESULT,
    fn get_KeyId(&self, out: *mut HSTRING) -> HRESULT,
    fn get_KeyName(&self, out: *mut HSTRING) -> HRESULT
}}
impl IEnterpriseKeyCredentialRegistrationInfo {
    #[inline] pub unsafe fn get_tenant_id(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_TenantId)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_tenant_name(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_TenantName)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_subject(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Subject)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_key_id(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_KeyId)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_key_name(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_KeyName)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
}
RT_CLASS!{class EnterpriseKeyCredentialRegistrationInfo: IEnterpriseKeyCredentialRegistrationInfo}
DEFINE_IID!(IID_IEnterpriseKeyCredentialRegistrationManager, 2213789247, 41567, 19642, 187, 142, 189, 195, 45, 3, 194, 151);
RT_INTERFACE!{interface IEnterpriseKeyCredentialRegistrationManager(IEnterpriseKeyCredentialRegistrationManagerVtbl): IInspectable(IInspectableVtbl) [IID_IEnterpriseKeyCredentialRegistrationManager] {
    fn GetRegistrationsAsync(&self, out: *mut *mut ::rt::gen::windows::foundation::IAsyncOperation<::rt::gen::windows::foundation::collections::IVectorView<EnterpriseKeyCredentialRegistrationInfo>>) -> HRESULT
}}
impl IEnterpriseKeyCredentialRegistrationManager {
    #[inline] pub unsafe fn get_registrations_async(&self) -> Result<ComPtr<::rt::gen::windows::foundation::IAsyncOperation<::rt::gen::windows::foundation::collections::IVectorView<EnterpriseKeyCredentialRegistrationInfo>>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetRegistrationsAsync)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
RT_CLASS!{class EnterpriseKeyCredentialRegistrationManager: IEnterpriseKeyCredentialRegistrationManager}
impl RtActivatable<IEnterpriseKeyCredentialRegistrationManagerStatics> for EnterpriseKeyCredentialRegistrationManager {}
impl EnterpriseKeyCredentialRegistrationManager {
    #[inline] pub fn get_current() -> Result<ComPtr<EnterpriseKeyCredentialRegistrationManager>> { unsafe {
        <Self as RtActivatable<IEnterpriseKeyCredentialRegistrationManagerStatics>>::get_activation_factory().get_current()
    }}
}
DEFINE_CLSID!(EnterpriseKeyCredentialRegistrationManager(&[87,105,110,100,111,119,115,46,83,101,99,117,114,105,116,121,46,65,117,116,104,101,110,116,105,99,97,116,105,111,110,46,73,100,101,110,116,105,116,121,46,69,110,116,101,114,112,114,105,115,101,75,101,121,67,114,101,100,101,110,116,105,97,108,82,101,103,105,115,116,114,97,116,105,111,110,77,97,110,97,103,101,114,0]) [CLSID_EnterpriseKeyCredentialRegistrationManager]);
DEFINE_IID!(IID_IEnterpriseKeyCredentialRegistrationManagerStatics, 2008571550, 44276, 19392, 186, 194, 64, 187, 70, 239, 187, 63);
RT_INTERFACE!{static interface IEnterpriseKeyCredentialRegistrationManagerStatics(IEnterpriseKeyCredentialRegistrationManagerStaticsVtbl): IInspectable(IInspectableVtbl) [IID_IEnterpriseKeyCredentialRegistrationManagerStatics] {
    fn get_Current(&self, out: *mut *mut EnterpriseKeyCredentialRegistrationManager) -> HRESULT
}}
impl IEnterpriseKeyCredentialRegistrationManagerStatics {
    #[inline] pub unsafe fn get_current(&self) -> Result<ComPtr<EnterpriseKeyCredentialRegistrationManager>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Current)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
pub mod provider { // Windows.Security.Authentication.Identity.Provider
use ::prelude::*;
DEFINE_IID!(IID_ISecondaryAuthenticationFactorAuthentication, 34215653, 27173, 16547, 140, 0, 80, 160, 35, 246, 25, 209);
RT_INTERFACE!{interface ISecondaryAuthenticationFactorAuthentication(ISecondaryAuthenticationFactorAuthenticationVtbl): IInspectable(IInspectableVtbl) [IID_ISecondaryAuthenticationFactorAuthentication] {
    #[cfg(not(feature="windows-storage"))] fn __Dummy0(&self) -> (),
    #[cfg(feature="windows-storage")] fn get_ServiceAuthenticationHmac(&self, out: *mut *mut ::rt::gen::windows::storage::streams::IBuffer) -> HRESULT,
    #[cfg(not(feature="windows-storage"))] fn __Dummy1(&self) -> (),
    #[cfg(feature="windows-storage")] fn get_SessionNonce(&self, out: *mut *mut ::rt::gen::windows::storage::streams::IBuffer) -> HRESULT,
    #[cfg(not(feature="windows-storage"))] fn __Dummy2(&self) -> (),
    #[cfg(feature="windows-storage")] fn get_DeviceNonce(&self, out: *mut *mut ::rt::gen::windows::storage::streams::IBuffer) -> HRESULT,
    #[cfg(not(feature="windows-storage"))] fn __Dummy3(&self) -> (),
    #[cfg(feature="windows-storage")] fn get_DeviceConfigurationData(&self, out: *mut *mut ::rt::gen::windows::storage::streams::IBuffer) -> HRESULT,
    #[cfg(not(feature="windows-storage"))] fn __Dummy4(&self) -> (),
    #[cfg(feature="windows-storage")] fn FinishAuthenticationAsync(&self, deviceHmac: *mut ::rt::gen::windows::storage::streams::IBuffer, sessionHmac: *mut ::rt::gen::windows::storage::streams::IBuffer, out: *mut *mut ::rt::gen::windows::foundation::IAsyncOperation<SecondaryAuthenticationFactorFinishAuthenticationStatus>) -> HRESULT,
    fn AbortAuthenticationAsync(&self, errorLogMessage: HSTRING, out: *mut *mut ::rt::gen::windows::foundation::IAsyncAction) -> HRESULT
}}
impl ISecondaryAuthenticationFactorAuthentication {
    #[cfg(feature="windows-storage")] #[inline] pub unsafe fn get_service_authentication_hmac(&self) -> Result<ComPtr<::rt::gen::windows::storage::streams::IBuffer>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_ServiceAuthenticationHmac)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[cfg(feature="windows-storage")] #[inline] pub unsafe fn get_session_nonce(&self) -> Result<ComPtr<::rt::gen::windows::storage::streams::IBuffer>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_SessionNonce)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[cfg(feature="windows-storage")] #[inline] pub unsafe fn get_device_nonce(&self) -> Result<ComPtr<::rt::gen::windows::storage::streams::IBuffer>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_DeviceNonce)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[cfg(feature="windows-storage")] #[inline] pub unsafe fn get_device_configuration_data(&self) -> Result<ComPtr<::rt::gen::windows::storage::streams::IBuffer>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_DeviceConfigurationData)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[cfg(feature="windows-storage")] #[inline] pub unsafe fn finish_authentication_async(&self, deviceHmac: &::rt::gen::windows::storage::streams::IBuffer, sessionHmac: &::rt::gen::windows::storage::streams::IBuffer) -> Result<ComPtr<::rt::gen::windows::foundation::IAsyncOperation<SecondaryAuthenticationFactorFinishAuthenticationStatus>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).FinishAuthenticationAsync)(self as *const _ as *mut _, deviceHmac as *const _ as *mut _, sessionHmac as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn abort_authentication_async(&self, errorLogMessage: &HStringArg) -> Result<ComPtr<::rt::gen::windows::foundation::IAsyncAction>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).AbortAuthenticationAsync)(self as *const _ as *mut _, errorLogMessage.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
RT_CLASS!{class SecondaryAuthenticationFactorAuthentication: ISecondaryAuthenticationFactorAuthentication}
impl RtActivatable<ISecondaryAuthenticationFactorAuthenticationStatics> for SecondaryAuthenticationFactorAuthentication {}
impl SecondaryAuthenticationFactorAuthentication {
    #[inline] pub fn show_notification_message_async(deviceName: &HStringArg, message: SecondaryAuthenticationFactorAuthenticationMessage) -> Result<ComPtr<::rt::gen::windows::foundation::IAsyncAction>> { unsafe {
        <Self as RtActivatable<ISecondaryAuthenticationFactorAuthenticationStatics>>::get_activation_factory().show_notification_message_async(deviceName, message)
    }}
    #[cfg(feature="windows-storage")] #[inline] pub fn start_authentication_async(deviceId: &HStringArg, serviceAuthenticationNonce: &::rt::gen::windows::storage::streams::IBuffer) -> Result<ComPtr<::rt::gen::windows::foundation::IAsyncOperation<SecondaryAuthenticationFactorAuthenticationResult>>> { unsafe {
        <Self as RtActivatable<ISecondaryAuthenticationFactorAuthenticationStatics>>::get_activation_factory().start_authentication_async(deviceId, serviceAuthenticationNonce)
    }}
    #[inline] pub fn add_authentication_stage_changed(handler: &::rt::gen::windows::foundation::EventHandler<SecondaryAuthenticationFactorAuthenticationStageChangedEventArgs>) -> Result<::rt::gen::windows::foundation::EventRegistrationToken> { unsafe {
        <Self as RtActivatable<ISecondaryAuthenticationFactorAuthenticationStatics>>::get_activation_factory().add_authentication_stage_changed(handler)
    }}
    #[inline] pub fn remove_authentication_stage_changed(token: ::rt::gen::windows::foundation::EventRegistrationToken) -> Result<()> { unsafe {
        <Self as RtActivatable<ISecondaryAuthenticationFactorAuthenticationStatics>>::get_activation_factory().remove_authentication_stage_changed(token)
    }}
    #[inline] pub fn get_authentication_stage_info_async() -> Result<ComPtr<::rt::gen::windows::foundation::IAsyncOperation<SecondaryAuthenticationFactorAuthenticationStageInfo>>> { unsafe {
        <Self as RtActivatable<ISecondaryAuthenticationFactorAuthenticationStatics>>::get_activation_factory().get_authentication_stage_info_async()
    }}
}
DEFINE_CLSID!(SecondaryAuthenticationFactorAuthentication(&[87,105,110,100,111,119,115,46,83,101,99,117,114,105,116,121,46,65,117,116,104,101,110,116,105,99,97,116,105,111,110,46,73,100,101,110,116,105,116,121,46,80,114,111,118,105,100,101,114,46,83,101,99,111,110,100,97,114,121,65,117,116,104,101,110,116,105,99,97,116,105,111,110,70,97,99,116,111,114,65,117,116,104,101,110,116,105,99,97,116,105,111,110,0]) [CLSID_SecondaryAuthenticationFactorAuthentication]);
RT_ENUM! { enum SecondaryAuthenticationFactorAuthenticationMessage: i32 {
    Invalid (SecondaryAuthenticationFactorAuthenticationMessage_Invalid) = 0, SwipeUpWelcome (SecondaryAuthenticationFactorAuthenticationMessage_SwipeUpWelcome) = 1, TapWelcome (SecondaryAuthenticationFactorAuthenticationMessage_TapWelcome) = 2, DeviceNeedsAttention (SecondaryAuthenticationFactorAuthenticationMessage_DeviceNeedsAttention) = 3, LookingForDevice (SecondaryAuthenticationFactorAuthenticationMessage_LookingForDevice) = 4, LookingForDevicePluggedin (SecondaryAuthenticationFactorAuthenticationMessage_LookingForDevicePluggedin) = 5, BluetoothIsDisabled (SecondaryAuthenticationFactorAuthenticationMessage_BluetoothIsDisabled) = 6, NfcIsDisabled (SecondaryAuthenticationFactorAuthenticationMessage_NfcIsDisabled) = 7, WiFiIsDisabled (SecondaryAuthenticationFactorAuthenticationMessage_WiFiIsDisabled) = 8, ExtraTapIsRequired (SecondaryAuthenticationFactorAuthenticationMessage_ExtraTapIsRequired) = 9, DisabledByPolicy (SecondaryAuthenticationFactorAuthenticationMessage_DisabledByPolicy) = 10, TapOnDeviceRequired (SecondaryAuthenticationFactorAuthenticationMessage_TapOnDeviceRequired) = 11, HoldFinger (SecondaryAuthenticationFactorAuthenticationMessage_HoldFinger) = 12, ScanFinger (SecondaryAuthenticationFactorAuthenticationMessage_ScanFinger) = 13, UnauthorizedUser (SecondaryAuthenticationFactorAuthenticationMessage_UnauthorizedUser) = 14, ReregisterRequired (SecondaryAuthenticationFactorAuthenticationMessage_ReregisterRequired) = 15, TryAgain (SecondaryAuthenticationFactorAuthenticationMessage_TryAgain) = 16, SayPassphrase (SecondaryAuthenticationFactorAuthenticationMessage_SayPassphrase) = 17, ReadyToSignIn (SecondaryAuthenticationFactorAuthenticationMessage_ReadyToSignIn) = 18, UseAnotherSignInOption (SecondaryAuthenticationFactorAuthenticationMessage_UseAnotherSignInOption) = 19,
}}
DEFINE_IID!(IID_ISecondaryAuthenticationFactorAuthenticationResult, 2629523847, 61293, 19394, 191, 73, 70, 23, 81, 90, 15, 154);
RT_INTERFACE!{interface ISecondaryAuthenticationFactorAuthenticationResult(ISecondaryAuthenticationFactorAuthenticationResultVtbl): IInspectable(IInspectableVtbl) [IID_ISecondaryAuthenticationFactorAuthenticationResult] {
    fn get_Status(&self, out: *mut SecondaryAuthenticationFactorAuthenticationStatus) -> HRESULT,
    fn get_Authentication(&self, out: *mut *mut SecondaryAuthenticationFactorAuthentication) -> HRESULT
}}
impl ISecondaryAuthenticationFactorAuthenticationResult {
    #[inline] pub unsafe fn get_status(&self) -> Result<SecondaryAuthenticationFactorAuthenticationStatus> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_Status)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_authentication(&self) -> Result<ComPtr<SecondaryAuthenticationFactorAuthentication>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Authentication)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
RT_CLASS!{class SecondaryAuthenticationFactorAuthenticationResult: ISecondaryAuthenticationFactorAuthenticationResult}
RT_ENUM! { enum SecondaryAuthenticationFactorAuthenticationScenario: i32 {
    SignIn (SecondaryAuthenticationFactorAuthenticationScenario_SignIn) = 0, CredentialPrompt (SecondaryAuthenticationFactorAuthenticationScenario_CredentialPrompt) = 1,
}}
RT_ENUM! { enum SecondaryAuthenticationFactorAuthenticationStage: i32 {
    NotStarted (SecondaryAuthenticationFactorAuthenticationStage_NotStarted) = 0, WaitingForUserConfirmation (SecondaryAuthenticationFactorAuthenticationStage_WaitingForUserConfirmation) = 1, CollectingCredential (SecondaryAuthenticationFactorAuthenticationStage_CollectingCredential) = 2, SuspendingAuthentication (SecondaryAuthenticationFactorAuthenticationStage_SuspendingAuthentication) = 3, CredentialCollected (SecondaryAuthenticationFactorAuthenticationStage_CredentialCollected) = 4, CredentialAuthenticated (SecondaryAuthenticationFactorAuthenticationStage_CredentialAuthenticated) = 5, StoppingAuthentication (SecondaryAuthenticationFactorAuthenticationStage_StoppingAuthentication) = 6, ReadyForLock (SecondaryAuthenticationFactorAuthenticationStage_ReadyForLock) = 7, CheckingDevicePresence (SecondaryAuthenticationFactorAuthenticationStage_CheckingDevicePresence) = 8,
}}
DEFINE_IID!(IID_ISecondaryAuthenticationFactorAuthenticationStageChangedEventArgs, 3567644246, 29329, 16499, 188, 31, 204, 184, 245, 175, 223, 150);
RT_INTERFACE!{interface ISecondaryAuthenticationFactorAuthenticationStageChangedEventArgs(ISecondaryAuthenticationFactorAuthenticationStageChangedEventArgsVtbl): IInspectable(IInspectableVtbl) [IID_ISecondaryAuthenticationFactorAuthenticationStageChangedEventArgs] {
    fn get_StageInfo(&self, out: *mut *mut SecondaryAuthenticationFactorAuthenticationStageInfo) -> HRESULT
}}
impl ISecondaryAuthenticationFactorAuthenticationStageChangedEventArgs {
    #[inline] pub unsafe fn get_stage_info(&self) -> Result<ComPtr<SecondaryAuthenticationFactorAuthenticationStageInfo>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_StageInfo)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
RT_CLASS!{class SecondaryAuthenticationFactorAuthenticationStageChangedEventArgs: ISecondaryAuthenticationFactorAuthenticationStageChangedEventArgs}
DEFINE_IID!(IID_ISecondaryAuthenticationFactorAuthenticationStageInfo, 1459536523, 59562, 19471, 142, 76, 165, 89, 231, 58, 221, 136);
RT_INTERFACE!{interface ISecondaryAuthenticationFactorAuthenticationStageInfo(ISecondaryAuthenticationFactorAuthenticationStageInfoVtbl): IInspectable(IInspectableVtbl) [IID_ISecondaryAuthenticationFactorAuthenticationStageInfo] {
    fn get_Stage(&self, out: *mut SecondaryAuthenticationFactorAuthenticationStage) -> HRESULT,
    fn get_Scenario(&self, out: *mut SecondaryAuthenticationFactorAuthenticationScenario) -> HRESULT,
    fn get_DeviceId(&self, out: *mut HSTRING) -> HRESULT
}}
impl ISecondaryAuthenticationFactorAuthenticationStageInfo {
    #[inline] pub unsafe fn get_stage(&self) -> Result<SecondaryAuthenticationFactorAuthenticationStage> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_Stage)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_scenario(&self) -> Result<SecondaryAuthenticationFactorAuthenticationScenario> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_Scenario)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_device_id(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_DeviceId)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
}
RT_CLASS!{class SecondaryAuthenticationFactorAuthenticationStageInfo: ISecondaryAuthenticationFactorAuthenticationStageInfo}
DEFINE_IID!(IID_ISecondaryAuthenticationFactorAuthenticationStatics, 1062741590, 10488, 19983, 174, 140, 88, 152, 185, 174, 36, 105);
RT_INTERFACE!{static interface ISecondaryAuthenticationFactorAuthenticationStatics(ISecondaryAuthenticationFactorAuthenticationStaticsVtbl): IInspectable(IInspectableVtbl) [IID_ISecondaryAuthenticationFactorAuthenticationStatics] {
    fn ShowNotificationMessageAsync(&self, deviceName: HSTRING, message: SecondaryAuthenticationFactorAuthenticationMessage, out: *mut *mut ::rt::gen::windows::foundation::IAsyncAction) -> HRESULT,
    #[cfg(not(feature="windows-storage"))] fn __Dummy1(&self) -> (),
    #[cfg(feature="windows-storage")] fn StartAuthenticationAsync(&self, deviceId: HSTRING, serviceAuthenticationNonce: *mut ::rt::gen::windows::storage::streams::IBuffer, out: *mut *mut ::rt::gen::windows::foundation::IAsyncOperation<SecondaryAuthenticationFactorAuthenticationResult>) -> HRESULT,
    fn add_AuthenticationStageChanged(&self, handler: *mut ::rt::gen::windows::foundation::EventHandler<SecondaryAuthenticationFactorAuthenticationStageChangedEventArgs>, out: *mut ::rt::gen::windows::foundation::EventRegistrationToken) -> HRESULT,
    fn remove_AuthenticationStageChanged(&self, token: ::rt::gen::windows::foundation::EventRegistrationToken) -> HRESULT,
    fn GetAuthenticationStageInfoAsync(&self, out: *mut *mut ::rt::gen::windows::foundation::IAsyncOperation<SecondaryAuthenticationFactorAuthenticationStageInfo>) -> HRESULT
}}
impl ISecondaryAuthenticationFactorAuthenticationStatics {
    #[inline] pub unsafe fn show_notification_message_async(&self, deviceName: &HStringArg, message: SecondaryAuthenticationFactorAuthenticationMessage) -> Result<ComPtr<::rt::gen::windows::foundation::IAsyncAction>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).ShowNotificationMessageAsync)(self as *const _ as *mut _, deviceName.get(), message, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[cfg(feature="windows-storage")] #[inline] pub unsafe fn start_authentication_async(&self, deviceId: &HStringArg, serviceAuthenticationNonce: &::rt::gen::windows::storage::streams::IBuffer) -> Result<ComPtr<::rt::gen::windows::foundation::IAsyncOperation<SecondaryAuthenticationFactorAuthenticationResult>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).StartAuthenticationAsync)(self as *const _ as *mut _, deviceId.get(), serviceAuthenticationNonce as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn add_authentication_stage_changed(&self, handler: &::rt::gen::windows::foundation::EventHandler<SecondaryAuthenticationFactorAuthenticationStageChangedEventArgs>) -> Result<::rt::gen::windows::foundation::EventRegistrationToken> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).add_AuthenticationStageChanged)(self as *const _ as *mut _, handler as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn remove_authentication_stage_changed(&self, token: ::rt::gen::windows::foundation::EventRegistrationToken) -> Result<()> {
        let hr = ((*self.lpVtbl).remove_AuthenticationStageChanged)(self as *const _ as *mut _, token);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_authentication_stage_info_async(&self) -> Result<ComPtr<::rt::gen::windows::foundation::IAsyncOperation<SecondaryAuthenticationFactorAuthenticationStageInfo>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetAuthenticationStageInfoAsync)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
RT_ENUM! { enum SecondaryAuthenticationFactorAuthenticationStatus: i32 {
    Failed (SecondaryAuthenticationFactorAuthenticationStatus_Failed) = 0, Started (SecondaryAuthenticationFactorAuthenticationStatus_Started) = 1, UnknownDevice (SecondaryAuthenticationFactorAuthenticationStatus_UnknownDevice) = 2, DisabledByPolicy (SecondaryAuthenticationFactorAuthenticationStatus_DisabledByPolicy) = 3, InvalidAuthenticationStage (SecondaryAuthenticationFactorAuthenticationStatus_InvalidAuthenticationStage) = 4,
}}
RT_ENUM! { enum SecondaryAuthenticationFactorDeviceCapabilities: u32 {
    None (SecondaryAuthenticationFactorDeviceCapabilities_None) = 0, SecureStorage (SecondaryAuthenticationFactorDeviceCapabilities_SecureStorage) = 1, StoreKeys (SecondaryAuthenticationFactorDeviceCapabilities_StoreKeys) = 2, ConfirmUserIntentToAuthenticate (SecondaryAuthenticationFactorDeviceCapabilities_ConfirmUserIntentToAuthenticate) = 4, SupportSecureUserPresenceCheck (SecondaryAuthenticationFactorDeviceCapabilities_SupportSecureUserPresenceCheck) = 8, TransmittedDataIsEncrypted (SecondaryAuthenticationFactorDeviceCapabilities_TransmittedDataIsEncrypted) = 16, HMacSha256 (SecondaryAuthenticationFactorDeviceCapabilities_HMacSha256) = 32, CloseRangeDataTransmission (SecondaryAuthenticationFactorDeviceCapabilities_CloseRangeDataTransmission) = 64,
}}
RT_ENUM! { enum SecondaryAuthenticationFactorDeviceFindScope: i32 {
    User (SecondaryAuthenticationFactorDeviceFindScope_User) = 0, AllUsers (SecondaryAuthenticationFactorDeviceFindScope_AllUsers) = 1,
}}
RT_ENUM! { enum SecondaryAuthenticationFactorDevicePresence: i32 {
    Absent (SecondaryAuthenticationFactorDevicePresence_Absent) = 0, Present (SecondaryAuthenticationFactorDevicePresence_Present) = 1,
}}
RT_ENUM! { enum SecondaryAuthenticationFactorDevicePresenceMonitoringMode: i32 {
    Unsupported (SecondaryAuthenticationFactorDevicePresenceMonitoringMode_Unsupported) = 0, AppManaged (SecondaryAuthenticationFactorDevicePresenceMonitoringMode_AppManaged) = 1, SystemManaged (SecondaryAuthenticationFactorDevicePresenceMonitoringMode_SystemManaged) = 2,
}}
DEFINE_IID!(IID_ISecondaryAuthenticationFactorDevicePresenceMonitoringRegistrationStatics, 2420742681, 32498, 17699, 149, 28, 164, 23, 162, 74, 207, 147);
RT_INTERFACE!{static interface ISecondaryAuthenticationFactorDevicePresenceMonitoringRegistrationStatics(ISecondaryAuthenticationFactorDevicePresenceMonitoringRegistrationStaticsVtbl): IInspectable(IInspectableVtbl) [IID_ISecondaryAuthenticationFactorDevicePresenceMonitoringRegistrationStatics] {
    fn RegisterDevicePresenceMonitoringAsync(&self, deviceId: HSTRING, deviceInstancePath: HSTRING, monitoringMode: SecondaryAuthenticationFactorDevicePresenceMonitoringMode, out: *mut *mut ::rt::gen::windows::foundation::IAsyncOperation<SecondaryAuthenticationFactorDevicePresenceMonitoringRegistrationStatus>) -> HRESULT,
    #[cfg(not(feature="windows-storage"))] fn __Dummy1(&self) -> (),
    #[cfg(feature="windows-storage")] fn RegisterDevicePresenceMonitoringWithNewDeviceAsync(&self, deviceId: HSTRING, deviceInstancePath: HSTRING, monitoringMode: SecondaryAuthenticationFactorDevicePresenceMonitoringMode, deviceFriendlyName: HSTRING, deviceModelNumber: HSTRING, deviceConfigurationData: *mut ::rt::gen::windows::storage::streams::IBuffer, out: *mut *mut ::rt::gen::windows::foundation::IAsyncOperation<SecondaryAuthenticationFactorDevicePresenceMonitoringRegistrationStatus>) -> HRESULT,
    fn UnregisterDevicePresenceMonitoringAsync(&self, deviceId: HSTRING, out: *mut *mut ::rt::gen::windows::foundation::IAsyncAction) -> HRESULT,
    fn IsDevicePresenceMonitoringSupported(&self, out: *mut bool) -> HRESULT
}}
impl ISecondaryAuthenticationFactorDevicePresenceMonitoringRegistrationStatics {
    #[inline] pub unsafe fn register_device_presence_monitoring_async(&self, deviceId: &HStringArg, deviceInstancePath: &HStringArg, monitoringMode: SecondaryAuthenticationFactorDevicePresenceMonitoringMode) -> Result<ComPtr<::rt::gen::windows::foundation::IAsyncOperation<SecondaryAuthenticationFactorDevicePresenceMonitoringRegistrationStatus>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).RegisterDevicePresenceMonitoringAsync)(self as *const _ as *mut _, deviceId.get(), deviceInstancePath.get(), monitoringMode, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[cfg(feature="windows-storage")] #[inline] pub unsafe fn register_device_presence_monitoring_with_new_device_async(&self, deviceId: &HStringArg, deviceInstancePath: &HStringArg, monitoringMode: SecondaryAuthenticationFactorDevicePresenceMonitoringMode, deviceFriendlyName: &HStringArg, deviceModelNumber: &HStringArg, deviceConfigurationData: &::rt::gen::windows::storage::streams::IBuffer) -> Result<ComPtr<::rt::gen::windows::foundation::IAsyncOperation<SecondaryAuthenticationFactorDevicePresenceMonitoringRegistrationStatus>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).RegisterDevicePresenceMonitoringWithNewDeviceAsync)(self as *const _ as *mut _, deviceId.get(), deviceInstancePath.get(), monitoringMode, deviceFriendlyName.get(), deviceModelNumber.get(), deviceConfigurationData as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn unregister_device_presence_monitoring_async(&self, deviceId: &HStringArg) -> Result<ComPtr<::rt::gen::windows::foundation::IAsyncAction>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).UnregisterDevicePresenceMonitoringAsync)(self as *const _ as *mut _, deviceId.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn is_device_presence_monitoring_supported(&self) -> Result<bool> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).IsDevicePresenceMonitoringSupported)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
}
RT_ENUM! { enum SecondaryAuthenticationFactorDevicePresenceMonitoringRegistrationStatus: i32 {
    Unsupported (SecondaryAuthenticationFactorDevicePresenceMonitoringRegistrationStatus_Unsupported) = 0, Succeeded (SecondaryAuthenticationFactorDevicePresenceMonitoringRegistrationStatus_Succeeded) = 1, DisabledByPolicy (SecondaryAuthenticationFactorDevicePresenceMonitoringRegistrationStatus_DisabledByPolicy) = 2,
}}
RT_ENUM! { enum SecondaryAuthenticationFactorFinishAuthenticationStatus: i32 {
    Failed (SecondaryAuthenticationFactorFinishAuthenticationStatus_Failed) = 0, Completed (SecondaryAuthenticationFactorFinishAuthenticationStatus_Completed) = 1, NonceExpired (SecondaryAuthenticationFactorFinishAuthenticationStatus_NonceExpired) = 2,
}}
DEFINE_IID!(IID_ISecondaryAuthenticationFactorInfo, 506177633, 34099, 20430, 131, 155, 236, 183, 36, 16, 172, 20);
RT_INTERFACE!{interface ISecondaryAuthenticationFactorInfo(ISecondaryAuthenticationFactorInfoVtbl): IInspectable(IInspectableVtbl) [IID_ISecondaryAuthenticationFactorInfo] {
    fn get_DeviceId(&self, out: *mut HSTRING) -> HRESULT,
    fn get_DeviceFriendlyName(&self, out: *mut HSTRING) -> HRESULT,
    fn get_DeviceModelNumber(&self, out: *mut HSTRING) -> HRESULT,
    #[cfg(feature="windows-storage")] fn get_DeviceConfigurationData(&self, out: *mut *mut ::rt::gen::windows::storage::streams::IBuffer) -> HRESULT
}}
impl ISecondaryAuthenticationFactorInfo {
    #[inline] pub unsafe fn get_device_id(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_DeviceId)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_device_friendly_name(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_DeviceFriendlyName)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_device_model_number(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_DeviceModelNumber)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[cfg(feature="windows-storage")] #[inline] pub unsafe fn get_device_configuration_data(&self) -> Result<ComPtr<::rt::gen::windows::storage::streams::IBuffer>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_DeviceConfigurationData)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
RT_CLASS!{class SecondaryAuthenticationFactorInfo: ISecondaryAuthenticationFactorInfo}
DEFINE_IID!(IID_ISecondaryAuthenticationFactorInfo2, 349798819, 64550, 20471, 171, 195, 72, 232, 42, 81, 42, 10);
RT_INTERFACE!{interface ISecondaryAuthenticationFactorInfo2(ISecondaryAuthenticationFactorInfo2Vtbl): IInspectable(IInspectableVtbl) [IID_ISecondaryAuthenticationFactorInfo2] {
    fn get_PresenceMonitoringMode(&self, out: *mut SecondaryAuthenticationFactorDevicePresenceMonitoringMode) -> HRESULT,
    fn UpdateDevicePresenceAsync(&self, presenceState: SecondaryAuthenticationFactorDevicePresence, out: *mut *mut ::rt::gen::windows::foundation::IAsyncAction) -> HRESULT,
    fn get_IsAuthenticationSupported(&self, out: *mut bool) -> HRESULT
}}
impl ISecondaryAuthenticationFactorInfo2 {
    #[inline] pub unsafe fn get_presence_monitoring_mode(&self) -> Result<SecondaryAuthenticationFactorDevicePresenceMonitoringMode> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_PresenceMonitoringMode)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn update_device_presence_async(&self, presenceState: SecondaryAuthenticationFactorDevicePresence) -> Result<ComPtr<::rt::gen::windows::foundation::IAsyncAction>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).UpdateDevicePresenceAsync)(self as *const _ as *mut _, presenceState, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_is_authentication_supported(&self) -> Result<bool> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_IsAuthenticationSupported)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
}
DEFINE_IID!(IID_ISecondaryAuthenticationFactorRegistration, 2672606132, 36026, 18608, 132, 13, 219, 178, 42, 84, 198, 120);
RT_INTERFACE!{interface ISecondaryAuthenticationFactorRegistration(ISecondaryAuthenticationFactorRegistrationVtbl): IInspectable(IInspectableVtbl) [IID_ISecondaryAuthenticationFactorRegistration] {
    #[cfg(not(feature="windows-storage"))] fn __Dummy0(&self) -> (),
    #[cfg(feature="windows-storage")] fn FinishRegisteringDeviceAsync(&self, deviceConfigurationData: *mut ::rt::gen::windows::storage::streams::IBuffer, out: *mut *mut ::rt::gen::windows::foundation::IAsyncAction) -> HRESULT,
    fn AbortRegisteringDeviceAsync(&self, errorLogMessage: HSTRING, out: *mut *mut ::rt::gen::windows::foundation::IAsyncAction) -> HRESULT
}}
impl ISecondaryAuthenticationFactorRegistration {
    #[cfg(feature="windows-storage")] #[inline] pub unsafe fn finish_registering_device_async(&self, deviceConfigurationData: &::rt::gen::windows::storage::streams::IBuffer) -> Result<ComPtr<::rt::gen::windows::foundation::IAsyncAction>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).FinishRegisteringDeviceAsync)(self as *const _ as *mut _, deviceConfigurationData as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn abort_registering_device_async(&self, errorLogMessage: &HStringArg) -> Result<ComPtr<::rt::gen::windows::foundation::IAsyncAction>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).AbortRegisteringDeviceAsync)(self as *const _ as *mut _, errorLogMessage.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
RT_CLASS!{class SecondaryAuthenticationFactorRegistration: ISecondaryAuthenticationFactorRegistration}
impl RtActivatable<ISecondaryAuthenticationFactorDevicePresenceMonitoringRegistrationStatics> for SecondaryAuthenticationFactorRegistration {}
impl RtActivatable<ISecondaryAuthenticationFactorRegistrationStatics> for SecondaryAuthenticationFactorRegistration {}
impl SecondaryAuthenticationFactorRegistration {
    #[inline] pub fn register_device_presence_monitoring_async(deviceId: &HStringArg, deviceInstancePath: &HStringArg, monitoringMode: SecondaryAuthenticationFactorDevicePresenceMonitoringMode) -> Result<ComPtr<::rt::gen::windows::foundation::IAsyncOperation<SecondaryAuthenticationFactorDevicePresenceMonitoringRegistrationStatus>>> { unsafe {
        <Self as RtActivatable<ISecondaryAuthenticationFactorDevicePresenceMonitoringRegistrationStatics>>::get_activation_factory().register_device_presence_monitoring_async(deviceId, deviceInstancePath, monitoringMode)
    }}
    #[cfg(feature="windows-storage")] #[inline] pub fn register_device_presence_monitoring_with_new_device_async(deviceId: &HStringArg, deviceInstancePath: &HStringArg, monitoringMode: SecondaryAuthenticationFactorDevicePresenceMonitoringMode, deviceFriendlyName: &HStringArg, deviceModelNumber: &HStringArg, deviceConfigurationData: &::rt::gen::windows::storage::streams::IBuffer) -> Result<ComPtr<::rt::gen::windows::foundation::IAsyncOperation<SecondaryAuthenticationFactorDevicePresenceMonitoringRegistrationStatus>>> { unsafe {
        <Self as RtActivatable<ISecondaryAuthenticationFactorDevicePresenceMonitoringRegistrationStatics>>::get_activation_factory().register_device_presence_monitoring_with_new_device_async(deviceId, deviceInstancePath, monitoringMode, deviceFriendlyName, deviceModelNumber, deviceConfigurationData)
    }}
    #[inline] pub fn unregister_device_presence_monitoring_async(deviceId: &HStringArg) -> Result<ComPtr<::rt::gen::windows::foundation::IAsyncAction>> { unsafe {
        <Self as RtActivatable<ISecondaryAuthenticationFactorDevicePresenceMonitoringRegistrationStatics>>::get_activation_factory().unregister_device_presence_monitoring_async(deviceId)
    }}
    #[inline] pub fn is_device_presence_monitoring_supported() -> Result<bool> { unsafe {
        <Self as RtActivatable<ISecondaryAuthenticationFactorDevicePresenceMonitoringRegistrationStatics>>::get_activation_factory().is_device_presence_monitoring_supported()
    }}
    #[cfg(feature="windows-storage")] #[inline] pub fn request_start_registering_device_async(deviceId: &HStringArg, capabilities: SecondaryAuthenticationFactorDeviceCapabilities, deviceFriendlyName: &HStringArg, deviceModelNumber: &HStringArg, deviceKey: &::rt::gen::windows::storage::streams::IBuffer, mutualAuthenticationKey: &::rt::gen::windows::storage::streams::IBuffer) -> Result<ComPtr<::rt::gen::windows::foundation::IAsyncOperation<SecondaryAuthenticationFactorRegistrationResult>>> { unsafe {
        <Self as RtActivatable<ISecondaryAuthenticationFactorRegistrationStatics>>::get_activation_factory().request_start_registering_device_async(deviceId, capabilities, deviceFriendlyName, deviceModelNumber, deviceKey, mutualAuthenticationKey)
    }}
    #[inline] pub fn find_all_registered_device_info_async(queryType: SecondaryAuthenticationFactorDeviceFindScope) -> Result<ComPtr<::rt::gen::windows::foundation::IAsyncOperation<::rt::gen::windows::foundation::collections::IVectorView<SecondaryAuthenticationFactorInfo>>>> { unsafe {
        <Self as RtActivatable<ISecondaryAuthenticationFactorRegistrationStatics>>::get_activation_factory().find_all_registered_device_info_async(queryType)
    }}
    #[inline] pub fn unregister_device_async(deviceId: &HStringArg) -> Result<ComPtr<::rt::gen::windows::foundation::IAsyncAction>> { unsafe {
        <Self as RtActivatable<ISecondaryAuthenticationFactorRegistrationStatics>>::get_activation_factory().unregister_device_async(deviceId)
    }}
    #[cfg(feature="windows-storage")] #[inline] pub fn update_device_configuration_data_async(deviceId: &HStringArg, deviceConfigurationData: &::rt::gen::windows::storage::streams::IBuffer) -> Result<ComPtr<::rt::gen::windows::foundation::IAsyncAction>> { unsafe {
        <Self as RtActivatable<ISecondaryAuthenticationFactorRegistrationStatics>>::get_activation_factory().update_device_configuration_data_async(deviceId, deviceConfigurationData)
    }}
}
DEFINE_CLSID!(SecondaryAuthenticationFactorRegistration(&[87,105,110,100,111,119,115,46,83,101,99,117,114,105,116,121,46,65,117,116,104,101,110,116,105,99,97,116,105,111,110,46,73,100,101,110,116,105,116,121,46,80,114,111,118,105,100,101,114,46,83,101,99,111,110,100,97,114,121,65,117,116,104,101,110,116,105,99,97,116,105,111,110,70,97,99,116,111,114,82,101,103,105,115,116,114,97,116,105,111,110,0]) [CLSID_SecondaryAuthenticationFactorRegistration]);
DEFINE_IID!(IID_ISecondaryAuthenticationFactorRegistrationResult, 2768123376, 44515, 18817, 175, 107, 236, 25, 89, 33, 104, 42);
RT_INTERFACE!{interface ISecondaryAuthenticationFactorRegistrationResult(ISecondaryAuthenticationFactorRegistrationResultVtbl): IInspectable(IInspectableVtbl) [IID_ISecondaryAuthenticationFactorRegistrationResult] {
    fn get_Status(&self, out: *mut SecondaryAuthenticationFactorRegistrationStatus) -> HRESULT,
    fn get_Registration(&self, out: *mut *mut SecondaryAuthenticationFactorRegistration) -> HRESULT
}}
impl ISecondaryAuthenticationFactorRegistrationResult {
    #[inline] pub unsafe fn get_status(&self) -> Result<SecondaryAuthenticationFactorRegistrationStatus> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_Status)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_registration(&self) -> Result<ComPtr<SecondaryAuthenticationFactorRegistration>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Registration)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
RT_CLASS!{class SecondaryAuthenticationFactorRegistrationResult: ISecondaryAuthenticationFactorRegistrationResult}
DEFINE_IID!(IID_ISecondaryAuthenticationFactorRegistrationStatics, 450826085, 58295, 16725, 153, 127, 183, 86, 239, 101, 190, 186);
RT_INTERFACE!{static interface ISecondaryAuthenticationFactorRegistrationStatics(ISecondaryAuthenticationFactorRegistrationStaticsVtbl): IInspectable(IInspectableVtbl) [IID_ISecondaryAuthenticationFactorRegistrationStatics] {
    #[cfg(feature="windows-storage")] fn RequestStartRegisteringDeviceAsync(&self, deviceId: HSTRING, capabilities: SecondaryAuthenticationFactorDeviceCapabilities, deviceFriendlyName: HSTRING, deviceModelNumber: HSTRING, deviceKey: *mut ::rt::gen::windows::storage::streams::IBuffer, mutualAuthenticationKey: *mut ::rt::gen::windows::storage::streams::IBuffer, out: *mut *mut ::rt::gen::windows::foundation::IAsyncOperation<SecondaryAuthenticationFactorRegistrationResult>) -> HRESULT,
    fn FindAllRegisteredDeviceInfoAsync(&self, queryType: SecondaryAuthenticationFactorDeviceFindScope, out: *mut *mut ::rt::gen::windows::foundation::IAsyncOperation<::rt::gen::windows::foundation::collections::IVectorView<SecondaryAuthenticationFactorInfo>>) -> HRESULT,
    fn UnregisterDeviceAsync(&self, deviceId: HSTRING, out: *mut *mut ::rt::gen::windows::foundation::IAsyncAction) -> HRESULT,
    #[cfg(feature="windows-storage")] fn UpdateDeviceConfigurationDataAsync(&self, deviceId: HSTRING, deviceConfigurationData: *mut ::rt::gen::windows::storage::streams::IBuffer, out: *mut *mut ::rt::gen::windows::foundation::IAsyncAction) -> HRESULT
}}
impl ISecondaryAuthenticationFactorRegistrationStatics {
    #[cfg(feature="windows-storage")] #[inline] pub unsafe fn request_start_registering_device_async(&self, deviceId: &HStringArg, capabilities: SecondaryAuthenticationFactorDeviceCapabilities, deviceFriendlyName: &HStringArg, deviceModelNumber: &HStringArg, deviceKey: &::rt::gen::windows::storage::streams::IBuffer, mutualAuthenticationKey: &::rt::gen::windows::storage::streams::IBuffer) -> Result<ComPtr<::rt::gen::windows::foundation::IAsyncOperation<SecondaryAuthenticationFactorRegistrationResult>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).RequestStartRegisteringDeviceAsync)(self as *const _ as *mut _, deviceId.get(), capabilities, deviceFriendlyName.get(), deviceModelNumber.get(), deviceKey as *const _ as *mut _, mutualAuthenticationKey as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn find_all_registered_device_info_async(&self, queryType: SecondaryAuthenticationFactorDeviceFindScope) -> Result<ComPtr<::rt::gen::windows::foundation::IAsyncOperation<::rt::gen::windows::foundation::collections::IVectorView<SecondaryAuthenticationFactorInfo>>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).FindAllRegisteredDeviceInfoAsync)(self as *const _ as *mut _, queryType, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn unregister_device_async(&self, deviceId: &HStringArg) -> Result<ComPtr<::rt::gen::windows::foundation::IAsyncAction>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).UnregisterDeviceAsync)(self as *const _ as *mut _, deviceId.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[cfg(feature="windows-storage")] #[inline] pub unsafe fn update_device_configuration_data_async(&self, deviceId: &HStringArg, deviceConfigurationData: &::rt::gen::windows::storage::streams::IBuffer) -> Result<ComPtr<::rt::gen::windows::foundation::IAsyncAction>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).UpdateDeviceConfigurationDataAsync)(self as *const _ as *mut _, deviceId.get(), deviceConfigurationData as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
RT_ENUM! { enum SecondaryAuthenticationFactorRegistrationStatus: i32 {
    Failed (SecondaryAuthenticationFactorRegistrationStatus_Failed) = 0, Started (SecondaryAuthenticationFactorRegistrationStatus_Started) = 1, CanceledByUser (SecondaryAuthenticationFactorRegistrationStatus_CanceledByUser) = 2, PinSetupRequired (SecondaryAuthenticationFactorRegistrationStatus_PinSetupRequired) = 3, DisabledByPolicy (SecondaryAuthenticationFactorRegistrationStatus_DisabledByPolicy) = 4,
}}
} // Windows.Security.Authentication.Identity.Provider
pub mod core { // Windows.Security.Authentication.Identity.Core
use ::prelude::*;
DEFINE_IID!(IID_IMicrosoftAccountMultiFactorAuthenticationManager, 265502885, 62836, 17184, 160, 142, 10, 25, 168, 35, 34, 170);
RT_INTERFACE!{interface IMicrosoftAccountMultiFactorAuthenticationManager(IMicrosoftAccountMultiFactorAuthenticationManagerVtbl): IInspectable(IInspectableVtbl) [IID_IMicrosoftAccountMultiFactorAuthenticationManager] {
    fn GetOneTimePassCodeAsync(&self, userAccountId: HSTRING, codeLength: u32, out: *mut *mut ::rt::gen::windows::foundation::IAsyncOperation<MicrosoftAccountMultiFactorOneTimeCodedInfo>) -> HRESULT,
    fn AddDeviceAsync(&self, userAccountId: HSTRING, authenticationToken: HSTRING, wnsChannelId: HSTRING, out: *mut *mut ::rt::gen::windows::foundation::IAsyncOperation<MicrosoftAccountMultiFactorServiceResponse>) -> HRESULT,
    fn RemoveDeviceAsync(&self, userAccountId: HSTRING, out: *mut *mut ::rt::gen::windows::foundation::IAsyncOperation<MicrosoftAccountMultiFactorServiceResponse>) -> HRESULT,
    fn UpdateWnsChannelAsync(&self, userAccountId: HSTRING, channelUri: HSTRING, out: *mut *mut ::rt::gen::windows::foundation::IAsyncOperation<MicrosoftAccountMultiFactorServiceResponse>) -> HRESULT,
    fn GetSessionsAsync(&self, userAccountIdList: *mut ::rt::gen::windows::foundation::collections::IIterable<HString>, out: *mut *mut ::rt::gen::windows::foundation::IAsyncOperation<MicrosoftAccountMultiFactorGetSessionsResult>) -> HRESULT,
    fn GetSessionsAndUnregisteredAccountsAsync(&self, userAccountIdList: *mut ::rt::gen::windows::foundation::collections::IIterable<HString>, out: *mut *mut ::rt::gen::windows::foundation::IAsyncOperation<MicrosoftAccountMultiFactorUnregisteredAccountsAndSessionInfo>) -> HRESULT,
    fn ApproveSessionUsingAuthSessionInfoAsync(&self, sessionAuthentictionStatus: MicrosoftAccountMultiFactorSessionAuthenticationStatus, authenticationSessionInfo: *mut MicrosoftAccountMultiFactorSessionInfo, out: *mut *mut ::rt::gen::windows::foundation::IAsyncOperation<MicrosoftAccountMultiFactorServiceResponse>) -> HRESULT,
    fn ApproveSessionAsync(&self, sessionAuthentictionStatus: MicrosoftAccountMultiFactorSessionAuthenticationStatus, userAccountId: HSTRING, sessionId: HSTRING, sessionAuthenticationType: MicrosoftAccountMultiFactorAuthenticationType, out: *mut *mut ::rt::gen::windows::foundation::IAsyncOperation<MicrosoftAccountMultiFactorServiceResponse>) -> HRESULT,
    fn DenySessionUsingAuthSessionInfoAsync(&self, authenticationSessionInfo: *mut MicrosoftAccountMultiFactorSessionInfo, out: *mut *mut ::rt::gen::windows::foundation::IAsyncOperation<MicrosoftAccountMultiFactorServiceResponse>) -> HRESULT,
    fn DenySessionAsync(&self, userAccountId: HSTRING, sessionId: HSTRING, sessionAuthenticationType: MicrosoftAccountMultiFactorAuthenticationType, out: *mut *mut ::rt::gen::windows::foundation::IAsyncOperation<MicrosoftAccountMultiFactorServiceResponse>) -> HRESULT
}}
impl IMicrosoftAccountMultiFactorAuthenticationManager {
    #[inline] pub unsafe fn get_one_time_pass_code_async(&self, userAccountId: &HStringArg, codeLength: u32) -> Result<ComPtr<::rt::gen::windows::foundation::IAsyncOperation<MicrosoftAccountMultiFactorOneTimeCodedInfo>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetOneTimePassCodeAsync)(self as *const _ as *mut _, userAccountId.get(), codeLength, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn add_device_async(&self, userAccountId: &HStringArg, authenticationToken: &HStringArg, wnsChannelId: &HStringArg) -> Result<ComPtr<::rt::gen::windows::foundation::IAsyncOperation<MicrosoftAccountMultiFactorServiceResponse>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).AddDeviceAsync)(self as *const _ as *mut _, userAccountId.get(), authenticationToken.get(), wnsChannelId.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn remove_device_async(&self, userAccountId: &HStringArg) -> Result<ComPtr<::rt::gen::windows::foundation::IAsyncOperation<MicrosoftAccountMultiFactorServiceResponse>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).RemoveDeviceAsync)(self as *const _ as *mut _, userAccountId.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn update_wns_channel_async(&self, userAccountId: &HStringArg, channelUri: &HStringArg) -> Result<ComPtr<::rt::gen::windows::foundation::IAsyncOperation<MicrosoftAccountMultiFactorServiceResponse>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).UpdateWnsChannelAsync)(self as *const _ as *mut _, userAccountId.get(), channelUri.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_sessions_async(&self, userAccountIdList: &::rt::gen::windows::foundation::collections::IIterable<HString>) -> Result<ComPtr<::rt::gen::windows::foundation::IAsyncOperation<MicrosoftAccountMultiFactorGetSessionsResult>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetSessionsAsync)(self as *const _ as *mut _, userAccountIdList as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_sessions_and_unregistered_accounts_async(&self, userAccountIdList: &::rt::gen::windows::foundation::collections::IIterable<HString>) -> Result<ComPtr<::rt::gen::windows::foundation::IAsyncOperation<MicrosoftAccountMultiFactorUnregisteredAccountsAndSessionInfo>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetSessionsAndUnregisteredAccountsAsync)(self as *const _ as *mut _, userAccountIdList as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn approve_session_using_auth_session_info_async(&self, sessionAuthentictionStatus: MicrosoftAccountMultiFactorSessionAuthenticationStatus, authenticationSessionInfo: &MicrosoftAccountMultiFactorSessionInfo) -> Result<ComPtr<::rt::gen::windows::foundation::IAsyncOperation<MicrosoftAccountMultiFactorServiceResponse>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).ApproveSessionUsingAuthSessionInfoAsync)(self as *const _ as *mut _, sessionAuthentictionStatus, authenticationSessionInfo as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn approve_session_async(&self, sessionAuthentictionStatus: MicrosoftAccountMultiFactorSessionAuthenticationStatus, userAccountId: &HStringArg, sessionId: &HStringArg, sessionAuthenticationType: MicrosoftAccountMultiFactorAuthenticationType) -> Result<ComPtr<::rt::gen::windows::foundation::IAsyncOperation<MicrosoftAccountMultiFactorServiceResponse>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).ApproveSessionAsync)(self as *const _ as *mut _, sessionAuthentictionStatus, userAccountId.get(), sessionId.get(), sessionAuthenticationType, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn deny_session_using_auth_session_info_async(&self, authenticationSessionInfo: &MicrosoftAccountMultiFactorSessionInfo) -> Result<ComPtr<::rt::gen::windows::foundation::IAsyncOperation<MicrosoftAccountMultiFactorServiceResponse>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).DenySessionUsingAuthSessionInfoAsync)(self as *const _ as *mut _, authenticationSessionInfo as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn deny_session_async(&self, userAccountId: &HStringArg, sessionId: &HStringArg, sessionAuthenticationType: MicrosoftAccountMultiFactorAuthenticationType) -> Result<ComPtr<::rt::gen::windows::foundation::IAsyncOperation<MicrosoftAccountMultiFactorServiceResponse>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).DenySessionAsync)(self as *const _ as *mut _, userAccountId.get(), sessionId.get(), sessionAuthenticationType, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
RT_CLASS!{class MicrosoftAccountMultiFactorAuthenticationManager: IMicrosoftAccountMultiFactorAuthenticationManager}
impl RtActivatable<IMicrosoftAccountMultiFactorAuthenticatorStatics> for MicrosoftAccountMultiFactorAuthenticationManager {}
impl MicrosoftAccountMultiFactorAuthenticationManager {
    #[inline] pub fn get_current() -> Result<ComPtr<MicrosoftAccountMultiFactorAuthenticationManager>> { unsafe {
        <Self as RtActivatable<IMicrosoftAccountMultiFactorAuthenticatorStatics>>::get_activation_factory().get_current()
    }}
}
DEFINE_CLSID!(MicrosoftAccountMultiFactorAuthenticationManager(&[87,105,110,100,111,119,115,46,83,101,99,117,114,105,116,121,46,65,117,116,104,101,110,116,105,99,97,116,105,111,110,46,73,100,101,110,116,105,116,121,46,67,111,114,101,46,77,105,99,114,111,115,111,102,116,65,99,99,111,117,110,116,77,117,108,116,105,70,97,99,116,111,114,65,117,116,104,101,110,116,105,99,97,116,105,111,110,77,97,110,97,103,101,114,0]) [CLSID_MicrosoftAccountMultiFactorAuthenticationManager]);
RT_ENUM! { enum MicrosoftAccountMultiFactorAuthenticationType: i32 {
    User (MicrosoftAccountMultiFactorAuthenticationType_User) = 0, Device (MicrosoftAccountMultiFactorAuthenticationType_Device) = 1,
}}
DEFINE_IID!(IID_IMicrosoftAccountMultiFactorAuthenticatorStatics, 3647259366, 62534, 19569, 139, 121, 110, 164, 2, 74, 169, 184);
RT_INTERFACE!{static interface IMicrosoftAccountMultiFactorAuthenticatorStatics(IMicrosoftAccountMultiFactorAuthenticatorStaticsVtbl): IInspectable(IInspectableVtbl) [IID_IMicrosoftAccountMultiFactorAuthenticatorStatics] {
    fn get_Current(&self, out: *mut *mut MicrosoftAccountMultiFactorAuthenticationManager) -> HRESULT
}}
impl IMicrosoftAccountMultiFactorAuthenticatorStatics {
    #[inline] pub unsafe fn get_current(&self) -> Result<ComPtr<MicrosoftAccountMultiFactorAuthenticationManager>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Current)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
DEFINE_IID!(IID_IMicrosoftAccountMultiFactorGetSessionsResult, 1310960032, 59898, 18810, 149, 222, 109, 87, 71, 191, 151, 76);
RT_INTERFACE!{interface IMicrosoftAccountMultiFactorGetSessionsResult(IMicrosoftAccountMultiFactorGetSessionsResultVtbl): IInspectable(IInspectableVtbl) [IID_IMicrosoftAccountMultiFactorGetSessionsResult] {
    fn get_Sessions(&self, out: *mut *mut ::rt::gen::windows::foundation::collections::IVectorView<MicrosoftAccountMultiFactorSessionInfo>) -> HRESULT,
    fn get_ServiceResponse(&self, out: *mut MicrosoftAccountMultiFactorServiceResponse) -> HRESULT
}}
impl IMicrosoftAccountMultiFactorGetSessionsResult {
    #[inline] pub unsafe fn get_sessions(&self) -> Result<ComPtr<::rt::gen::windows::foundation::collections::IVectorView<MicrosoftAccountMultiFactorSessionInfo>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Sessions)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_service_response(&self) -> Result<MicrosoftAccountMultiFactorServiceResponse> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_ServiceResponse)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
}
RT_CLASS!{class MicrosoftAccountMultiFactorGetSessionsResult: IMicrosoftAccountMultiFactorGetSessionsResult}
DEFINE_IID!(IID_IMicrosoftAccountMultiFactorOneTimeCodedInfo, 2193237579, 55420, 18024, 169, 118, 64, 207, 174, 84, 125, 8);
RT_INTERFACE!{interface IMicrosoftAccountMultiFactorOneTimeCodedInfo(IMicrosoftAccountMultiFactorOneTimeCodedInfoVtbl): IInspectable(IInspectableVtbl) [IID_IMicrosoftAccountMultiFactorOneTimeCodedInfo] {
    fn get_Code(&self, out: *mut HSTRING) -> HRESULT,
    fn get_TimeInterval(&self, out: *mut ::rt::gen::windows::foundation::TimeSpan) -> HRESULT,
    fn get_TimeToLive(&self, out: *mut ::rt::gen::windows::foundation::TimeSpan) -> HRESULT,
    fn get_ServiceResponse(&self, out: *mut MicrosoftAccountMultiFactorServiceResponse) -> HRESULT
}}
impl IMicrosoftAccountMultiFactorOneTimeCodedInfo {
    #[inline] pub unsafe fn get_code(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Code)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_time_interval(&self) -> Result<::rt::gen::windows::foundation::TimeSpan> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_TimeInterval)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_time_to_live(&self) -> Result<::rt::gen::windows::foundation::TimeSpan> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_TimeToLive)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_service_response(&self) -> Result<MicrosoftAccountMultiFactorServiceResponse> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_ServiceResponse)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
}
RT_CLASS!{class MicrosoftAccountMultiFactorOneTimeCodedInfo: IMicrosoftAccountMultiFactorOneTimeCodedInfo}
RT_ENUM! { enum MicrosoftAccountMultiFactorServiceResponse: i32 {
    Success (MicrosoftAccountMultiFactorServiceResponse_Success) = 0, Error (MicrosoftAccountMultiFactorServiceResponse_Error) = 1, NoNetworkConnection (MicrosoftAccountMultiFactorServiceResponse_NoNetworkConnection) = 2, ServiceUnavailable (MicrosoftAccountMultiFactorServiceResponse_ServiceUnavailable) = 3, TotpSetupDenied (MicrosoftAccountMultiFactorServiceResponse_TotpSetupDenied) = 4, NgcNotSetup (MicrosoftAccountMultiFactorServiceResponse_NgcNotSetup) = 5, SessionAlreadyDenied (MicrosoftAccountMultiFactorServiceResponse_SessionAlreadyDenied) = 6, SessionAlreadyApproved (MicrosoftAccountMultiFactorServiceResponse_SessionAlreadyApproved) = 7, SessionExpired (MicrosoftAccountMultiFactorServiceResponse_SessionExpired) = 8, NgcNonceExpired (MicrosoftAccountMultiFactorServiceResponse_NgcNonceExpired) = 9, InvalidSessionId (MicrosoftAccountMultiFactorServiceResponse_InvalidSessionId) = 10, InvalidSessionType (MicrosoftAccountMultiFactorServiceResponse_InvalidSessionType) = 11, InvalidOperation (MicrosoftAccountMultiFactorServiceResponse_InvalidOperation) = 12, InvalidStateTransition (MicrosoftAccountMultiFactorServiceResponse_InvalidStateTransition) = 13, DeviceNotFound (MicrosoftAccountMultiFactorServiceResponse_DeviceNotFound) = 14, FlowDisabled (MicrosoftAccountMultiFactorServiceResponse_FlowDisabled) = 15, SessionNotApproved (MicrosoftAccountMultiFactorServiceResponse_SessionNotApproved) = 16, OperationCanceledByUser (MicrosoftAccountMultiFactorServiceResponse_OperationCanceledByUser) = 17, NgcDisabledByServer (MicrosoftAccountMultiFactorServiceResponse_NgcDisabledByServer) = 18, NgcKeyNotFoundOnServer (MicrosoftAccountMultiFactorServiceResponse_NgcKeyNotFoundOnServer) = 19, UIRequired (MicrosoftAccountMultiFactorServiceResponse_UIRequired) = 20, DeviceIdChanged (MicrosoftAccountMultiFactorServiceResponse_DeviceIdChanged) = 21,
}}
RT_ENUM! { enum MicrosoftAccountMultiFactorSessionApprovalStatus: i32 {
    Pending (MicrosoftAccountMultiFactorSessionApprovalStatus_Pending) = 0, Approved (MicrosoftAccountMultiFactorSessionApprovalStatus_Approved) = 1, Denied (MicrosoftAccountMultiFactorSessionApprovalStatus_Denied) = 2,
}}
RT_ENUM! { enum MicrosoftAccountMultiFactorSessionAuthenticationStatus: i32 {
    Authenticated (MicrosoftAccountMultiFactorSessionAuthenticationStatus_Authenticated) = 0, Unauthenticated (MicrosoftAccountMultiFactorSessionAuthenticationStatus_Unauthenticated) = 1,
}}
DEFINE_IID!(IID_IMicrosoftAccountMultiFactorSessionInfo, 1602137012, 41592, 17973, 183, 101, 180, 148, 235, 38, 10, 244);
RT_INTERFACE!{interface IMicrosoftAccountMultiFactorSessionInfo(IMicrosoftAccountMultiFactorSessionInfoVtbl): IInspectable(IInspectableVtbl) [IID_IMicrosoftAccountMultiFactorSessionInfo] {
    fn get_UserAccountId(&self, out: *mut HSTRING) -> HRESULT,
    fn get_SessionId(&self, out: *mut HSTRING) -> HRESULT,
    fn get_DisplaySessionId(&self, out: *mut HSTRING) -> HRESULT,
    fn get_ApprovalStatus(&self, out: *mut MicrosoftAccountMultiFactorSessionApprovalStatus) -> HRESULT,
    fn get_AuthenticationType(&self, out: *mut MicrosoftAccountMultiFactorAuthenticationType) -> HRESULT,
    fn get_RequestTime(&self, out: *mut ::rt::gen::windows::foundation::DateTime) -> HRESULT,
    fn get_ExpirationTime(&self, out: *mut ::rt::gen::windows::foundation::DateTime) -> HRESULT
}}
impl IMicrosoftAccountMultiFactorSessionInfo {
    #[inline] pub unsafe fn get_user_account_id(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_UserAccountId)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_session_id(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_SessionId)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_display_session_id(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_DisplaySessionId)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_approval_status(&self) -> Result<MicrosoftAccountMultiFactorSessionApprovalStatus> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_ApprovalStatus)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_authentication_type(&self) -> Result<MicrosoftAccountMultiFactorAuthenticationType> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_AuthenticationType)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_request_time(&self) -> Result<::rt::gen::windows::foundation::DateTime> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_RequestTime)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_expiration_time(&self) -> Result<::rt::gen::windows::foundation::DateTime> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_ExpirationTime)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
}
RT_CLASS!{class MicrosoftAccountMultiFactorSessionInfo: IMicrosoftAccountMultiFactorSessionInfo}
DEFINE_IID!(IID_IMicrosoftAccountMultiFactorUnregisteredAccountsAndSessionInfo, 2860434939, 55871, 16520, 162, 13, 86, 24, 175, 173, 178, 229);
RT_INTERFACE!{interface IMicrosoftAccountMultiFactorUnregisteredAccountsAndSessionInfo(IMicrosoftAccountMultiFactorUnregisteredAccountsAndSessionInfoVtbl): IInspectable(IInspectableVtbl) [IID_IMicrosoftAccountMultiFactorUnregisteredAccountsAndSessionInfo] {
    fn get_Sessions(&self, out: *mut *mut ::rt::gen::windows::foundation::collections::IVectorView<MicrosoftAccountMultiFactorSessionInfo>) -> HRESULT,
    fn get_UnregisteredAccounts(&self, out: *mut *mut ::rt::gen::windows::foundation::collections::IVectorView<HString>) -> HRESULT,
    fn get_ServiceResponse(&self, out: *mut MicrosoftAccountMultiFactorServiceResponse) -> HRESULT
}}
impl IMicrosoftAccountMultiFactorUnregisteredAccountsAndSessionInfo {
    #[inline] pub unsafe fn get_sessions(&self) -> Result<ComPtr<::rt::gen::windows::foundation::collections::IVectorView<MicrosoftAccountMultiFactorSessionInfo>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Sessions)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_unregistered_accounts(&self) -> Result<ComPtr<::rt::gen::windows::foundation::collections::IVectorView<HString>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_UnregisteredAccounts)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_service_response(&self) -> Result<MicrosoftAccountMultiFactorServiceResponse> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_ServiceResponse)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
}
RT_CLASS!{class MicrosoftAccountMultiFactorUnregisteredAccountsAndSessionInfo: IMicrosoftAccountMultiFactorUnregisteredAccountsAndSessionInfo}
} // Windows.Security.Authentication.Identity.Core
} // Windows.Security.Authentication.Identity
pub mod onlineid { // Windows.Security.Authentication.OnlineId
use ::prelude::*;
RT_ENUM! { enum CredentialPromptType: i32 {
    PromptIfNeeded (CredentialPromptType_PromptIfNeeded) = 0, RetypeCredentials (CredentialPromptType_RetypeCredentials) = 1, DoNotPrompt (CredentialPromptType_DoNotPrompt) = 2,
}}
DEFINE_IID!(IID_IOnlineIdAuthenticator, 2684614026, 10667, 18455, 184, 132, 215, 81, 109, 173, 24, 185);
RT_INTERFACE!{interface IOnlineIdAuthenticator(IOnlineIdAuthenticatorVtbl): IInspectable(IInspectableVtbl) [IID_IOnlineIdAuthenticator] {
    fn AuthenticateUserAsync(&self, request: *mut OnlineIdServiceTicketRequest, out: *mut *mut UserAuthenticationOperation) -> HRESULT,
    fn AuthenticateUserAsyncAdvanced(&self, requests: *mut ::rt::gen::windows::foundation::collections::IIterable<OnlineIdServiceTicketRequest>, credentialPromptType: CredentialPromptType, out: *mut *mut UserAuthenticationOperation) -> HRESULT,
    fn SignOutUserAsync(&self, out: *mut *mut SignOutUserOperation) -> HRESULT,
    fn put_ApplicationId(&self, value: Guid) -> HRESULT,
    fn get_ApplicationId(&self, out: *mut Guid) -> HRESULT,
    fn get_CanSignOut(&self, out: *mut bool) -> HRESULT,
    fn get_AuthenticatedSafeCustomerId(&self, out: *mut HSTRING) -> HRESULT
}}
impl IOnlineIdAuthenticator {
    #[inline] pub unsafe fn authenticate_user_async(&self, request: &OnlineIdServiceTicketRequest) -> Result<ComPtr<UserAuthenticationOperation>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).AuthenticateUserAsync)(self as *const _ as *mut _, request as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn authenticate_user_async_advanced(&self, requests: &::rt::gen::windows::foundation::collections::IIterable<OnlineIdServiceTicketRequest>, credentialPromptType: CredentialPromptType) -> Result<ComPtr<UserAuthenticationOperation>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).AuthenticateUserAsyncAdvanced)(self as *const _ as *mut _, requests as *const _ as *mut _, credentialPromptType, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn sign_out_user_async(&self) -> Result<ComPtr<SignOutUserOperation>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).SignOutUserAsync)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_application_id(&self, value: Guid) -> Result<()> {
        let hr = ((*self.lpVtbl).put_ApplicationId)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_application_id(&self) -> Result<Guid> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_ApplicationId)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_can_sign_out(&self) -> Result<bool> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_CanSignOut)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_authenticated_safe_customer_id(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_AuthenticatedSafeCustomerId)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
}
RT_CLASS!{class OnlineIdAuthenticator: IOnlineIdAuthenticator}
impl RtActivatable<IActivationFactory> for OnlineIdAuthenticator {}
DEFINE_CLSID!(OnlineIdAuthenticator(&[87,105,110,100,111,119,115,46,83,101,99,117,114,105,116,121,46,65,117,116,104,101,110,116,105,99,97,116,105,111,110,46,79,110,108,105,110,101,73,100,46,79,110,108,105,110,101,73,100,65,117,116,104,101,110,116,105,99,97,116,111,114,0]) [CLSID_OnlineIdAuthenticator]);
DEFINE_IID!(IID_IOnlineIdServiceTicket, 3378271359, 55169, 19092, 172, 184, 197, 152, 116, 35, 140, 38);
RT_INTERFACE!{interface IOnlineIdServiceTicket(IOnlineIdServiceTicketVtbl): IInspectable(IInspectableVtbl) [IID_IOnlineIdServiceTicket] {
    fn get_Value(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Request(&self, out: *mut *mut OnlineIdServiceTicketRequest) -> HRESULT,
    fn get_ErrorCode(&self, out: *mut i32) -> HRESULT
}}
impl IOnlineIdServiceTicket {
    #[inline] pub unsafe fn get_value(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Value)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_request(&self) -> Result<ComPtr<OnlineIdServiceTicketRequest>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Request)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_error_code(&self) -> Result<i32> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_ErrorCode)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
}
RT_CLASS!{class OnlineIdServiceTicket: IOnlineIdServiceTicket}
DEFINE_IID!(IID_IOnlineIdServiceTicketRequest, 695485907, 64355, 16693, 137, 9, 78, 53, 76, 6, 20, 102);
RT_INTERFACE!{interface IOnlineIdServiceTicketRequest(IOnlineIdServiceTicketRequestVtbl): IInspectable(IInspectableVtbl) [IID_IOnlineIdServiceTicketRequest] {
    fn get_Service(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Policy(&self, out: *mut HSTRING) -> HRESULT
}}
impl IOnlineIdServiceTicketRequest {
    #[inline] pub unsafe fn get_service(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Service)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_policy(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Policy)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
}
RT_CLASS!{class OnlineIdServiceTicketRequest: IOnlineIdServiceTicketRequest}
impl RtActivatable<IOnlineIdServiceTicketRequestFactory> for OnlineIdServiceTicketRequest {}
impl OnlineIdServiceTicketRequest {
    #[inline] pub fn create_online_id_service_ticket_request(service: &HStringArg, policy: &HStringArg) -> Result<ComPtr<OnlineIdServiceTicketRequest>> { unsafe {
        <Self as RtActivatable<IOnlineIdServiceTicketRequestFactory>>::get_activation_factory().create_online_id_service_ticket_request(service, policy)
    }}
    #[inline] pub fn create_online_id_service_ticket_request_advanced(service: &HStringArg) -> Result<ComPtr<OnlineIdServiceTicketRequest>> { unsafe {
        <Self as RtActivatable<IOnlineIdServiceTicketRequestFactory>>::get_activation_factory().create_online_id_service_ticket_request_advanced(service)
    }}
}
DEFINE_CLSID!(OnlineIdServiceTicketRequest(&[87,105,110,100,111,119,115,46,83,101,99,117,114,105,116,121,46,65,117,116,104,101,110,116,105,99,97,116,105,111,110,46,79,110,108,105,110,101,73,100,46,79,110,108,105,110,101,73,100,83,101,114,118,105,99,101,84,105,99,107,101,116,82,101,113,117,101,115,116,0]) [CLSID_OnlineIdServiceTicketRequest]);
DEFINE_IID!(IID_IOnlineIdServiceTicketRequestFactory, 3199928840, 40563, 16503, 150, 20, 8, 97, 76, 11, 194, 69);
RT_INTERFACE!{static interface IOnlineIdServiceTicketRequestFactory(IOnlineIdServiceTicketRequestFactoryVtbl): IInspectable(IInspectableVtbl) [IID_IOnlineIdServiceTicketRequestFactory] {
    fn CreateOnlineIdServiceTicketRequest(&self, service: HSTRING, policy: HSTRING, out: *mut *mut OnlineIdServiceTicketRequest) -> HRESULT,
    fn CreateOnlineIdServiceTicketRequestAdvanced(&self, service: HSTRING, out: *mut *mut OnlineIdServiceTicketRequest) -> HRESULT
}}
impl IOnlineIdServiceTicketRequestFactory {
    #[inline] pub unsafe fn create_online_id_service_ticket_request(&self, service: &HStringArg, policy: &HStringArg) -> Result<ComPtr<OnlineIdServiceTicketRequest>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).CreateOnlineIdServiceTicketRequest)(self as *const _ as *mut _, service.get(), policy.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn create_online_id_service_ticket_request_advanced(&self, service: &HStringArg) -> Result<ComPtr<OnlineIdServiceTicketRequest>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).CreateOnlineIdServiceTicketRequestAdvanced)(self as *const _ as *mut _, service.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
RT_CLASS!{static class OnlineIdSystemAuthenticator}
impl RtActivatable<IOnlineIdSystemAuthenticatorStatics> for OnlineIdSystemAuthenticator {}
impl OnlineIdSystemAuthenticator {
    #[inline] pub fn get_default() -> Result<ComPtr<OnlineIdSystemAuthenticatorForUser>> { unsafe {
        <Self as RtActivatable<IOnlineIdSystemAuthenticatorStatics>>::get_activation_factory().get_default()
    }}
    #[cfg(feature="windows-system")] #[inline] pub fn get_for_user(user: &::rt::gen::windows::system::User) -> Result<ComPtr<OnlineIdSystemAuthenticatorForUser>> { unsafe {
        <Self as RtActivatable<IOnlineIdSystemAuthenticatorStatics>>::get_activation_factory().get_for_user(user)
    }}
}
DEFINE_CLSID!(OnlineIdSystemAuthenticator(&[87,105,110,100,111,119,115,46,83,101,99,117,114,105,116,121,46,65,117,116,104,101,110,116,105,99,97,116,105,111,110,46,79,110,108,105,110,101,73,100,46,79,110,108,105,110,101,73,100,83,121,115,116,101,109,65,117,116,104,101,110,116,105,99,97,116,111,114,0]) [CLSID_OnlineIdSystemAuthenticator]);
DEFINE_IID!(IID_IOnlineIdSystemAuthenticatorForUser, 1469628155, 7652, 16774, 162, 230, 181, 99, 248, 106, 175, 68);
RT_INTERFACE!{interface IOnlineIdSystemAuthenticatorForUser(IOnlineIdSystemAuthenticatorForUserVtbl): IInspectable(IInspectableVtbl) [IID_IOnlineIdSystemAuthenticatorForUser] {
    fn GetTicketAsync(&self, request: *mut OnlineIdServiceTicketRequest, out: *mut *mut ::rt::gen::windows::foundation::IAsyncOperation<OnlineIdSystemTicketResult>) -> HRESULT,
    fn put_ApplicationId(&self, value: Guid) -> HRESULT,
    fn get_ApplicationId(&self, out: *mut Guid) -> HRESULT,
    #[cfg(feature="windows-system")] fn get_User(&self, out: *mut *mut ::rt::gen::windows::system::User) -> HRESULT
}}
impl IOnlineIdSystemAuthenticatorForUser {
    #[inline] pub unsafe fn get_ticket_async(&self, request: &OnlineIdServiceTicketRequest) -> Result<ComPtr<::rt::gen::windows::foundation::IAsyncOperation<OnlineIdSystemTicketResult>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetTicketAsync)(self as *const _ as *mut _, request as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_application_id(&self, value: Guid) -> Result<()> {
        let hr = ((*self.lpVtbl).put_ApplicationId)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_application_id(&self) -> Result<Guid> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_ApplicationId)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[cfg(feature="windows-system")] #[inline] pub unsafe fn get_user(&self) -> Result<ComPtr<::rt::gen::windows::system::User>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_User)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
RT_CLASS!{class OnlineIdSystemAuthenticatorForUser: IOnlineIdSystemAuthenticatorForUser}
DEFINE_IID!(IID_IOnlineIdSystemAuthenticatorStatics, 2231662482, 63028, 16867, 150, 164, 81, 100, 233, 2, 199, 64);
RT_INTERFACE!{static interface IOnlineIdSystemAuthenticatorStatics(IOnlineIdSystemAuthenticatorStaticsVtbl): IInspectable(IInspectableVtbl) [IID_IOnlineIdSystemAuthenticatorStatics] {
    fn get_Default(&self, out: *mut *mut OnlineIdSystemAuthenticatorForUser) -> HRESULT,
    #[cfg(feature="windows-system")] fn GetForUser(&self, user: *mut ::rt::gen::windows::system::User, out: *mut *mut OnlineIdSystemAuthenticatorForUser) -> HRESULT
}}
impl IOnlineIdSystemAuthenticatorStatics {
    #[inline] pub unsafe fn get_default(&self) -> Result<ComPtr<OnlineIdSystemAuthenticatorForUser>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Default)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[cfg(feature="windows-system")] #[inline] pub unsafe fn get_for_user(&self, user: &::rt::gen::windows::system::User) -> Result<ComPtr<OnlineIdSystemAuthenticatorForUser>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetForUser)(self as *const _ as *mut _, user as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
DEFINE_IID!(IID_IOnlineIdSystemIdentity, 1950142989, 46794, 17229, 129, 36, 83, 234, 18, 104, 83, 7);
RT_INTERFACE!{interface IOnlineIdSystemIdentity(IOnlineIdSystemIdentityVtbl): IInspectable(IInspectableVtbl) [IID_IOnlineIdSystemIdentity] {
    fn get_Ticket(&self, out: *mut *mut OnlineIdServiceTicket) -> HRESULT,
    fn get_Id(&self, out: *mut HSTRING) -> HRESULT
}}
impl IOnlineIdSystemIdentity {
    #[inline] pub unsafe fn get_ticket(&self) -> Result<ComPtr<OnlineIdServiceTicket>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Ticket)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_id(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Id)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
}
RT_CLASS!{class OnlineIdSystemIdentity: IOnlineIdSystemIdentity}
DEFINE_IID!(IID_IOnlineIdSystemTicketResult, 3674890232, 45208, 19149, 157, 19, 158, 100, 6, 82, 181, 182);
RT_INTERFACE!{interface IOnlineIdSystemTicketResult(IOnlineIdSystemTicketResultVtbl): IInspectable(IInspectableVtbl) [IID_IOnlineIdSystemTicketResult] {
    fn get_Identity(&self, out: *mut *mut OnlineIdSystemIdentity) -> HRESULT,
    fn get_Status(&self, out: *mut OnlineIdSystemTicketStatus) -> HRESULT,
    fn get_ExtendedError(&self, out: *mut ::rt::gen::windows::foundation::HResult) -> HRESULT
}}
impl IOnlineIdSystemTicketResult {
    #[inline] pub unsafe fn get_identity(&self) -> Result<ComPtr<OnlineIdSystemIdentity>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Identity)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_status(&self) -> Result<OnlineIdSystemTicketStatus> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_Status)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_extended_error(&self) -> Result<::rt::gen::windows::foundation::HResult> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_ExtendedError)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
}
RT_CLASS!{class OnlineIdSystemTicketResult: IOnlineIdSystemTicketResult}
RT_ENUM! { enum OnlineIdSystemTicketStatus: i32 {
    Success (OnlineIdSystemTicketStatus_Success) = 0, Error (OnlineIdSystemTicketStatus_Error) = 1, ServiceConnectionError (OnlineIdSystemTicketStatus_ServiceConnectionError) = 2,
}}
RT_CLASS!{class SignOutUserOperation: ::rt::gen::windows::foundation::IAsyncAction}
RT_CLASS!{class UserAuthenticationOperation: ::rt::gen::windows::foundation::IAsyncOperation<UserIdentity>}
DEFINE_IID!(IID_IUserIdentity, 558291405, 1858, 19427, 138, 28, 124, 122, 230, 121, 170, 136);
RT_INTERFACE!{interface IUserIdentity(IUserIdentityVtbl): IInspectable(IInspectableVtbl) [IID_IUserIdentity] {
    fn get_Tickets(&self, out: *mut *mut ::rt::gen::windows::foundation::collections::IVectorView<OnlineIdServiceTicket>) -> HRESULT,
    fn get_Id(&self, out: *mut HSTRING) -> HRESULT,
    fn get_SafeCustomerId(&self, out: *mut HSTRING) -> HRESULT,
    fn get_SignInName(&self, out: *mut HSTRING) -> HRESULT,
    fn get_FirstName(&self, out: *mut HSTRING) -> HRESULT,
    fn get_LastName(&self, out: *mut HSTRING) -> HRESULT,
    fn get_IsBetaAccount(&self, out: *mut bool) -> HRESULT,
    fn get_IsConfirmedPC(&self, out: *mut bool) -> HRESULT
}}
impl IUserIdentity {
    #[inline] pub unsafe fn get_tickets(&self) -> Result<ComPtr<::rt::gen::windows::foundation::collections::IVectorView<OnlineIdServiceTicket>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Tickets)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_id(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Id)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_safe_customer_id(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_SafeCustomerId)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_sign_in_name(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_SignInName)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_first_name(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_FirstName)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_last_name(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_LastName)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_is_beta_account(&self) -> Result<bool> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_IsBetaAccount)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_is_confirmed_pc(&self) -> Result<bool> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_IsConfirmedPC)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
}
RT_CLASS!{class UserIdentity: IUserIdentity}
} // Windows.Security.Authentication.OnlineId
pub mod web { // Windows.Security.Authentication.Web
use ::prelude::*;
RT_ENUM! { enum TokenBindingKeyType: i32 {
    Rsa2048 (TokenBindingKeyType_Rsa2048) = 0, EcdsaP256 (TokenBindingKeyType_EcdsaP256) = 1, AnyExisting (TokenBindingKeyType_AnyExisting) = 2,
}}
RT_CLASS!{static class WebAuthenticationBroker}
impl RtActivatable<IWebAuthenticationBrokerStatics> for WebAuthenticationBroker {}
impl RtActivatable<IWebAuthenticationBrokerStatics2> for WebAuthenticationBroker {}
impl WebAuthenticationBroker {
    #[inline] pub fn authenticate_with_callback_uri_async(options: WebAuthenticationOptions, requestUri: &::rt::gen::windows::foundation::Uri, callbackUri: &::rt::gen::windows::foundation::Uri) -> Result<ComPtr<::rt::gen::windows::foundation::IAsyncOperation<WebAuthenticationResult>>> { unsafe {
        <Self as RtActivatable<IWebAuthenticationBrokerStatics>>::get_activation_factory().authenticate_with_callback_uri_async(options, requestUri, callbackUri)
    }}
    #[inline] pub fn authenticate_without_callback_uri_async(options: WebAuthenticationOptions, requestUri: &::rt::gen::windows::foundation::Uri) -> Result<ComPtr<::rt::gen::windows::foundation::IAsyncOperation<WebAuthenticationResult>>> { unsafe {
        <Self as RtActivatable<IWebAuthenticationBrokerStatics>>::get_activation_factory().authenticate_without_callback_uri_async(options, requestUri)
    }}
    #[inline] pub fn get_current_application_callback_uri() -> Result<ComPtr<::rt::gen::windows::foundation::Uri>> { unsafe {
        <Self as RtActivatable<IWebAuthenticationBrokerStatics>>::get_activation_factory().get_current_application_callback_uri()
    }}
    #[inline] pub fn authenticate_and_continue(requestUri: &::rt::gen::windows::foundation::Uri) -> Result<()> { unsafe {
        <Self as RtActivatable<IWebAuthenticationBrokerStatics2>>::get_activation_factory().authenticate_and_continue(requestUri)
    }}
    #[inline] pub fn authenticate_with_callback_uri_and_continue(requestUri: &::rt::gen::windows::foundation::Uri, callbackUri: &::rt::gen::windows::foundation::Uri) -> Result<()> { unsafe {
        <Self as RtActivatable<IWebAuthenticationBrokerStatics2>>::get_activation_factory().authenticate_with_callback_uri_and_continue(requestUri, callbackUri)
    }}
    #[inline] pub fn authenticate_with_callback_uri_continuation_data_and_options_and_continue(requestUri: &::rt::gen::windows::foundation::Uri, callbackUri: &::rt::gen::windows::foundation::Uri, continuationData: &::rt::gen::windows::foundation::collections::ValueSet, options: WebAuthenticationOptions) -> Result<()> { unsafe {
        <Self as RtActivatable<IWebAuthenticationBrokerStatics2>>::get_activation_factory().authenticate_with_callback_uri_continuation_data_and_options_and_continue(requestUri, callbackUri, continuationData, options)
    }}
    #[inline] pub fn authenticate_silently_async(requestUri: &::rt::gen::windows::foundation::Uri) -> Result<ComPtr<::rt::gen::windows::foundation::IAsyncOperation<WebAuthenticationResult>>> { unsafe {
        <Self as RtActivatable<IWebAuthenticationBrokerStatics2>>::get_activation_factory().authenticate_silently_async(requestUri)
    }}
    #[inline] pub fn authenticate_silently_with_options_async(requestUri: &::rt::gen::windows::foundation::Uri, options: WebAuthenticationOptions) -> Result<ComPtr<::rt::gen::windows::foundation::IAsyncOperation<WebAuthenticationResult>>> { unsafe {
        <Self as RtActivatable<IWebAuthenticationBrokerStatics2>>::get_activation_factory().authenticate_silently_with_options_async(requestUri, options)
    }}
}
DEFINE_CLSID!(WebAuthenticationBroker(&[87,105,110,100,111,119,115,46,83,101,99,117,114,105,116,121,46,65,117,116,104,101,110,116,105,99,97,116,105,111,110,46,87,101,98,46,87,101,98,65,117,116,104,101,110,116,105,99,97,116,105,111,110,66,114,111,107,101,114,0]) [CLSID_WebAuthenticationBroker]);
DEFINE_IID!(IID_IWebAuthenticationBrokerStatics, 789880602, 58995, 16565, 188, 34, 32, 26, 104, 100, 163, 123);
RT_INTERFACE!{static interface IWebAuthenticationBrokerStatics(IWebAuthenticationBrokerStaticsVtbl): IInspectable(IInspectableVtbl) [IID_IWebAuthenticationBrokerStatics] {
    fn AuthenticateWithCallbackUriAsync(&self, options: WebAuthenticationOptions, requestUri: *mut ::rt::gen::windows::foundation::Uri, callbackUri: *mut ::rt::gen::windows::foundation::Uri, out: *mut *mut ::rt::gen::windows::foundation::IAsyncOperation<WebAuthenticationResult>) -> HRESULT,
    fn AuthenticateWithoutCallbackUriAsync(&self, options: WebAuthenticationOptions, requestUri: *mut ::rt::gen::windows::foundation::Uri, out: *mut *mut ::rt::gen::windows::foundation::IAsyncOperation<WebAuthenticationResult>) -> HRESULT,
    fn GetCurrentApplicationCallbackUri(&self, out: *mut *mut ::rt::gen::windows::foundation::Uri) -> HRESULT
}}
impl IWebAuthenticationBrokerStatics {
    #[inline] pub unsafe fn authenticate_with_callback_uri_async(&self, options: WebAuthenticationOptions, requestUri: &::rt::gen::windows::foundation::Uri, callbackUri: &::rt::gen::windows::foundation::Uri) -> Result<ComPtr<::rt::gen::windows::foundation::IAsyncOperation<WebAuthenticationResult>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).AuthenticateWithCallbackUriAsync)(self as *const _ as *mut _, options, requestUri as *const _ as *mut _, callbackUri as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn authenticate_without_callback_uri_async(&self, options: WebAuthenticationOptions, requestUri: &::rt::gen::windows::foundation::Uri) -> Result<ComPtr<::rt::gen::windows::foundation::IAsyncOperation<WebAuthenticationResult>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).AuthenticateWithoutCallbackUriAsync)(self as *const _ as *mut _, options, requestUri as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_current_application_callback_uri(&self) -> Result<ComPtr<::rt::gen::windows::foundation::Uri>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetCurrentApplicationCallbackUri)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
DEFINE_IID!(IID_IWebAuthenticationBrokerStatics2, 1942879134, 5351, 16858, 169, 113, 170, 244, 65, 11, 98, 30);
RT_INTERFACE!{static interface IWebAuthenticationBrokerStatics2(IWebAuthenticationBrokerStatics2Vtbl): IInspectable(IInspectableVtbl) [IID_IWebAuthenticationBrokerStatics2] {
    fn AuthenticateAndContinue(&self, requestUri: *mut ::rt::gen::windows::foundation::Uri) -> HRESULT,
    fn AuthenticateWithCallbackUriAndContinue(&self, requestUri: *mut ::rt::gen::windows::foundation::Uri, callbackUri: *mut ::rt::gen::windows::foundation::Uri) -> HRESULT,
    fn AuthenticateWithCallbackUriContinuationDataAndOptionsAndContinue(&self, requestUri: *mut ::rt::gen::windows::foundation::Uri, callbackUri: *mut ::rt::gen::windows::foundation::Uri, continuationData: *mut ::rt::gen::windows::foundation::collections::ValueSet, options: WebAuthenticationOptions) -> HRESULT,
    fn AuthenticateSilentlyAsync(&self, requestUri: *mut ::rt::gen::windows::foundation::Uri, out: *mut *mut ::rt::gen::windows::foundation::IAsyncOperation<WebAuthenticationResult>) -> HRESULT,
    fn AuthenticateSilentlyWithOptionsAsync(&self, requestUri: *mut ::rt::gen::windows::foundation::Uri, options: WebAuthenticationOptions, out: *mut *mut ::rt::gen::windows::foundation::IAsyncOperation<WebAuthenticationResult>) -> HRESULT
}}
impl IWebAuthenticationBrokerStatics2 {
    #[inline] pub unsafe fn authenticate_and_continue(&self, requestUri: &::rt::gen::windows::foundation::Uri) -> Result<()> {
        let hr = ((*self.lpVtbl).AuthenticateAndContinue)(self as *const _ as *mut _, requestUri as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn authenticate_with_callback_uri_and_continue(&self, requestUri: &::rt::gen::windows::foundation::Uri, callbackUri: &::rt::gen::windows::foundation::Uri) -> Result<()> {
        let hr = ((*self.lpVtbl).AuthenticateWithCallbackUriAndContinue)(self as *const _ as *mut _, requestUri as *const _ as *mut _, callbackUri as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn authenticate_with_callback_uri_continuation_data_and_options_and_continue(&self, requestUri: &::rt::gen::windows::foundation::Uri, callbackUri: &::rt::gen::windows::foundation::Uri, continuationData: &::rt::gen::windows::foundation::collections::ValueSet, options: WebAuthenticationOptions) -> Result<()> {
        let hr = ((*self.lpVtbl).AuthenticateWithCallbackUriContinuationDataAndOptionsAndContinue)(self as *const _ as *mut _, requestUri as *const _ as *mut _, callbackUri as *const _ as *mut _, continuationData as *const _ as *mut _, options);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn authenticate_silently_async(&self, requestUri: &::rt::gen::windows::foundation::Uri) -> Result<ComPtr<::rt::gen::windows::foundation::IAsyncOperation<WebAuthenticationResult>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).AuthenticateSilentlyAsync)(self as *const _ as *mut _, requestUri as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn authenticate_silently_with_options_async(&self, requestUri: &::rt::gen::windows::foundation::Uri, options: WebAuthenticationOptions) -> Result<ComPtr<::rt::gen::windows::foundation::IAsyncOperation<WebAuthenticationResult>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).AuthenticateSilentlyWithOptionsAsync)(self as *const _ as *mut _, requestUri as *const _ as *mut _, options, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
RT_ENUM! { enum WebAuthenticationOptions: u32 {
    None (WebAuthenticationOptions_None) = 0, SilentMode (WebAuthenticationOptions_SilentMode) = 1, UseTitle (WebAuthenticationOptions_UseTitle) = 2, UseHttpPost (WebAuthenticationOptions_UseHttpPost) = 4, UseCorporateNetwork (WebAuthenticationOptions_UseCorporateNetwork) = 8,
}}
DEFINE_IID!(IID_IWebAuthenticationResult, 1677732683, 60905, 18186, 165, 205, 3, 35, 250, 246, 226, 98);
RT_INTERFACE!{interface IWebAuthenticationResult(IWebAuthenticationResultVtbl): IInspectable(IInspectableVtbl) [IID_IWebAuthenticationResult] {
    fn get_ResponseData(&self, out: *mut HSTRING) -> HRESULT,
    fn get_ResponseStatus(&self, out: *mut WebAuthenticationStatus) -> HRESULT,
    fn get_ResponseErrorDetail(&self, out: *mut u32) -> HRESULT
}}
impl IWebAuthenticationResult {
    #[inline] pub unsafe fn get_response_data(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_ResponseData)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_response_status(&self) -> Result<WebAuthenticationStatus> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_ResponseStatus)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_response_error_detail(&self) -> Result<u32> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_ResponseErrorDetail)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
}
RT_CLASS!{class WebAuthenticationResult: IWebAuthenticationResult}
RT_ENUM! { enum WebAuthenticationStatus: i32 {
    Success (WebAuthenticationStatus_Success) = 0, UserCancel (WebAuthenticationStatus_UserCancel) = 1, ErrorHttp (WebAuthenticationStatus_ErrorHttp) = 2,
}}
pub mod provider { // Windows.Security.Authentication.Web.Provider
use ::prelude::*;
DEFINE_IID!(IID_IWebAccountClientView, 3887949498, 3015, 19558, 191, 212, 101, 211, 8, 44, 188, 168);
RT_INTERFACE!{interface IWebAccountClientView(IWebAccountClientViewVtbl): IInspectable(IInspectableVtbl) [IID_IWebAccountClientView] {
    fn get_ApplicationCallbackUri(&self, out: *mut *mut ::rt::gen::windows::foundation::Uri) -> HRESULT,
    fn get_Type(&self, out: *mut WebAccountClientViewType) -> HRESULT,
    fn get_AccountPairwiseId(&self, out: *mut HSTRING) -> HRESULT
}}
impl IWebAccountClientView {
    #[inline] pub unsafe fn get_application_callback_uri(&self) -> Result<ComPtr<::rt::gen::windows::foundation::Uri>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_ApplicationCallbackUri)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_type(&self) -> Result<WebAccountClientViewType> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_Type)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_account_pairwise_id(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_AccountPairwiseId)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
}
RT_CLASS!{class WebAccountClientView: IWebAccountClientView}
impl RtActivatable<IWebAccountClientViewFactory> for WebAccountClientView {}
impl WebAccountClientView {
    #[inline] pub fn create(viewType: WebAccountClientViewType, applicationCallbackUri: &::rt::gen::windows::foundation::Uri) -> Result<ComPtr<WebAccountClientView>> { unsafe {
        <Self as RtActivatable<IWebAccountClientViewFactory>>::get_activation_factory().create(viewType, applicationCallbackUri)
    }}
    #[inline] pub fn create_with_pairwise_id(viewType: WebAccountClientViewType, applicationCallbackUri: &::rt::gen::windows::foundation::Uri, accountPairwiseId: &HStringArg) -> Result<ComPtr<WebAccountClientView>> { unsafe {
        <Self as RtActivatable<IWebAccountClientViewFactory>>::get_activation_factory().create_with_pairwise_id(viewType, applicationCallbackUri, accountPairwiseId)
    }}
}
DEFINE_CLSID!(WebAccountClientView(&[87,105,110,100,111,119,115,46,83,101,99,117,114,105,116,121,46,65,117,116,104,101,110,116,105,99,97,116,105,111,110,46,87,101,98,46,80,114,111,118,105,100,101,114,46,87,101,98,65,99,99,111,117,110,116,67,108,105,101,110,116,86,105,101,119,0]) [CLSID_WebAccountClientView]);
DEFINE_IID!(IID_IWebAccountClientViewFactory, 1634539172, 56866, 18517, 163, 38, 6, 206, 191, 42, 63, 35);
RT_INTERFACE!{static interface IWebAccountClientViewFactory(IWebAccountClientViewFactoryVtbl): IInspectable(IInspectableVtbl) [IID_IWebAccountClientViewFactory] {
    fn Create(&self, viewType: WebAccountClientViewType, applicationCallbackUri: *mut ::rt::gen::windows::foundation::Uri, out: *mut *mut WebAccountClientView) -> HRESULT,
    fn CreateWithPairwiseId(&self, viewType: WebAccountClientViewType, applicationCallbackUri: *mut ::rt::gen::windows::foundation::Uri, accountPairwiseId: HSTRING, out: *mut *mut WebAccountClientView) -> HRESULT
}}
impl IWebAccountClientViewFactory {
    #[inline] pub unsafe fn create(&self, viewType: WebAccountClientViewType, applicationCallbackUri: &::rt::gen::windows::foundation::Uri) -> Result<ComPtr<WebAccountClientView>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).Create)(self as *const _ as *mut _, viewType, applicationCallbackUri as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn create_with_pairwise_id(&self, viewType: WebAccountClientViewType, applicationCallbackUri: &::rt::gen::windows::foundation::Uri, accountPairwiseId: &HStringArg) -> Result<ComPtr<WebAccountClientView>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).CreateWithPairwiseId)(self as *const _ as *mut _, viewType, applicationCallbackUri as *const _ as *mut _, accountPairwiseId.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
RT_ENUM! { enum WebAccountClientViewType: i32 {
    IdOnly (WebAccountClientViewType_IdOnly) = 0, IdAndProperties (WebAccountClientViewType_IdAndProperties) = 1,
}}
RT_CLASS!{static class WebAccountManager}
impl RtActivatable<IWebAccountManagerStatics> for WebAccountManager {}
impl RtActivatable<IWebAccountManagerStatics2> for WebAccountManager {}
impl RtActivatable<IWebAccountManagerStatics3> for WebAccountManager {}
impl RtActivatable<IWebAccountManagerStatics4> for WebAccountManager {}
impl RtActivatable<IWebAccountMapManagerStatics> for WebAccountManager {}
impl RtActivatable<IWebAccountScopeManagerStatics> for WebAccountManager {}
impl WebAccountManager {
    #[inline] pub fn update_web_account_properties_async(webAccount: &super::super::super::credentials::WebAccount, webAccountUserName: &HStringArg, additionalProperties: &::rt::gen::windows::foundation::collections::IMapView<HString, HString>) -> Result<ComPtr<::rt::gen::windows::foundation::IAsyncAction>> { unsafe {
        <Self as RtActivatable<IWebAccountManagerStatics>>::get_activation_factory().update_web_account_properties_async(webAccount, webAccountUserName, additionalProperties)
    }}
    #[inline] pub fn add_web_account_async(webAccountId: &HStringArg, webAccountUserName: &HStringArg, props: &::rt::gen::windows::foundation::collections::IMapView<HString, HString>) -> Result<ComPtr<::rt::gen::windows::foundation::IAsyncOperation<super::super::super::credentials::WebAccount>>> { unsafe {
        <Self as RtActivatable<IWebAccountManagerStatics>>::get_activation_factory().add_web_account_async(webAccountId, webAccountUserName, props)
    }}
    #[inline] pub fn delete_web_account_async(webAccount: &super::super::super::credentials::WebAccount) -> Result<ComPtr<::rt::gen::windows::foundation::IAsyncAction>> { unsafe {
        <Self as RtActivatable<IWebAccountManagerStatics>>::get_activation_factory().delete_web_account_async(webAccount)
    }}
    #[inline] pub fn find_all_provider_web_accounts_async() -> Result<ComPtr<::rt::gen::windows::foundation::IAsyncOperation<::rt::gen::windows::foundation::collections::IVectorView<super::super::super::credentials::WebAccount>>>> { unsafe {
        <Self as RtActivatable<IWebAccountManagerStatics>>::get_activation_factory().find_all_provider_web_accounts_async()
    }}
    #[cfg(feature="windows-web")] #[inline] pub fn push_cookies_async(uri: &::rt::gen::windows::foundation::Uri, cookies: &::rt::gen::windows::foundation::collections::IVectorView<::rt::gen::windows::web::http::HttpCookie>) -> Result<ComPtr<::rt::gen::windows::foundation::IAsyncAction>> { unsafe {
        <Self as RtActivatable<IWebAccountManagerStatics>>::get_activation_factory().push_cookies_async(uri, cookies)
    }}
    #[inline] pub fn set_view_async(webAccount: &super::super::super::credentials::WebAccount, view: &WebAccountClientView) -> Result<ComPtr<::rt::gen::windows::foundation::IAsyncAction>> { unsafe {
        <Self as RtActivatable<IWebAccountManagerStatics>>::get_activation_factory().set_view_async(webAccount, view)
    }}
    #[inline] pub fn clear_view_async(webAccount: &super::super::super::credentials::WebAccount, applicationCallbackUri: &::rt::gen::windows::foundation::Uri) -> Result<ComPtr<::rt::gen::windows::foundation::IAsyncAction>> { unsafe {
        <Self as RtActivatable<IWebAccountManagerStatics>>::get_activation_factory().clear_view_async(webAccount, applicationCallbackUri)
    }}
    #[inline] pub fn get_views_async(webAccount: &super::super::super::credentials::WebAccount) -> Result<ComPtr<::rt::gen::windows::foundation::IAsyncOperation<::rt::gen::windows::foundation::collections::IVectorView<WebAccountClientView>>>> { unsafe {
        <Self as RtActivatable<IWebAccountManagerStatics>>::get_activation_factory().get_views_async(webAccount)
    }}
    #[cfg(feature="windows-storage")] #[inline] pub fn set_web_account_picture_async(webAccount: &super::super::super::credentials::WebAccount, webAccountPicture: &::rt::gen::windows::storage::streams::IRandomAccessStream) -> Result<ComPtr<::rt::gen::windows::foundation::IAsyncAction>> { unsafe {
        <Self as RtActivatable<IWebAccountManagerStatics>>::get_activation_factory().set_web_account_picture_async(webAccount, webAccountPicture)
    }}
    #[inline] pub fn clear_web_account_picture_async(webAccount: &super::super::super::credentials::WebAccount) -> Result<ComPtr<::rt::gen::windows::foundation::IAsyncAction>> { unsafe {
        <Self as RtActivatable<IWebAccountManagerStatics>>::get_activation_factory().clear_web_account_picture_async(webAccount)
    }}
    #[inline] pub fn pull_cookies_async(uriString: &HStringArg, callerPFN: &HStringArg) -> Result<ComPtr<::rt::gen::windows::foundation::IAsyncAction>> { unsafe {
        <Self as RtActivatable<IWebAccountManagerStatics2>>::get_activation_factory().pull_cookies_async(uriString, callerPFN)
    }}
    #[cfg(feature="windows-system")] #[inline] pub fn find_all_provider_web_accounts_for_user_async(user: &::rt::gen::windows::system::User) -> Result<ComPtr<::rt::gen::windows::foundation::IAsyncOperation<::rt::gen::windows::foundation::collections::IVectorView<super::super::super::credentials::WebAccount>>>> { unsafe {
        <Self as RtActivatable<IWebAccountManagerStatics3>>::get_activation_factory().find_all_provider_web_accounts_for_user_async(user)
    }}
    #[cfg(feature="windows-system")] #[inline] pub fn add_web_account_for_user_async(user: &::rt::gen::windows::system::User, webAccountId: &HStringArg, webAccountUserName: &HStringArg, props: &::rt::gen::windows::foundation::collections::IMapView<HString, HString>) -> Result<ComPtr<::rt::gen::windows::foundation::IAsyncOperation<super::super::super::credentials::WebAccount>>> { unsafe {
        <Self as RtActivatable<IWebAccountManagerStatics3>>::get_activation_factory().add_web_account_for_user_async(user, webAccountId, webAccountUserName, props)
    }}
    #[cfg(feature="windows-system")] #[inline] pub fn add_web_account_with_scope_for_user_async(user: &::rt::gen::windows::system::User, webAccountId: &HStringArg, webAccountUserName: &HStringArg, props: &::rt::gen::windows::foundation::collections::IMapView<HString, HString>, scope: WebAccountScope) -> Result<ComPtr<::rt::gen::windows::foundation::IAsyncOperation<super::super::super::credentials::WebAccount>>> { unsafe {
        <Self as RtActivatable<IWebAccountManagerStatics3>>::get_activation_factory().add_web_account_with_scope_for_user_async(user, webAccountId, webAccountUserName, props, scope)
    }}
    #[cfg(feature="windows-system")] #[inline] pub fn add_web_account_with_scope_and_map_for_user_async(user: &::rt::gen::windows::system::User, webAccountId: &HStringArg, webAccountUserName: &HStringArg, props: &::rt::gen::windows::foundation::collections::IMapView<HString, HString>, scope: WebAccountScope, perUserWebAccountId: &HStringArg) -> Result<ComPtr<::rt::gen::windows::foundation::IAsyncOperation<super::super::super::credentials::WebAccount>>> { unsafe {
        <Self as RtActivatable<IWebAccountManagerStatics3>>::get_activation_factory().add_web_account_with_scope_and_map_for_user_async(user, webAccountId, webAccountUserName, props, scope, perUserWebAccountId)
    }}
    #[inline] pub fn invalidate_app_cache_for_all_accounts_async() -> Result<ComPtr<::rt::gen::windows::foundation::IAsyncAction>> { unsafe {
        <Self as RtActivatable<IWebAccountManagerStatics4>>::get_activation_factory().invalidate_app_cache_for_all_accounts_async()
    }}
    #[inline] pub fn invalidate_app_cache_for_account_async(webAccount: &super::super::super::credentials::WebAccount) -> Result<ComPtr<::rt::gen::windows::foundation::IAsyncAction>> { unsafe {
        <Self as RtActivatable<IWebAccountManagerStatics4>>::get_activation_factory().invalidate_app_cache_for_account_async(webAccount)
    }}
    #[inline] pub fn add_web_account_with_scope_and_map_async(webAccountId: &HStringArg, webAccountUserName: &HStringArg, props: &::rt::gen::windows::foundation::collections::IMapView<HString, HString>, scope: WebAccountScope, perUserWebAccountId: &HStringArg) -> Result<ComPtr<::rt::gen::windows::foundation::IAsyncOperation<super::super::super::credentials::WebAccount>>> { unsafe {
        <Self as RtActivatable<IWebAccountMapManagerStatics>>::get_activation_factory().add_web_account_with_scope_and_map_async(webAccountId, webAccountUserName, props, scope, perUserWebAccountId)
    }}
    #[inline] pub fn set_per_app_to_per_user_account_async(perAppAccount: &super::super::super::credentials::WebAccount, perUserWebAccountId: &HStringArg) -> Result<ComPtr<::rt::gen::windows::foundation::IAsyncAction>> { unsafe {
        <Self as RtActivatable<IWebAccountMapManagerStatics>>::get_activation_factory().set_per_app_to_per_user_account_async(perAppAccount, perUserWebAccountId)
    }}
    #[inline] pub fn get_per_user_from_per_app_account_async(perAppAccount: &super::super::super::credentials::WebAccount) -> Result<ComPtr<::rt::gen::windows::foundation::IAsyncOperation<super::super::super::credentials::WebAccount>>> { unsafe {
        <Self as RtActivatable<IWebAccountMapManagerStatics>>::get_activation_factory().get_per_user_from_per_app_account_async(perAppAccount)
    }}
    #[inline] pub fn clear_per_user_from_per_app_account_async(perAppAccount: &super::super::super::credentials::WebAccount) -> Result<ComPtr<::rt::gen::windows::foundation::IAsyncAction>> { unsafe {
        <Self as RtActivatable<IWebAccountMapManagerStatics>>::get_activation_factory().clear_per_user_from_per_app_account_async(perAppAccount)
    }}
    #[inline] pub fn add_web_account_with_scope_async(webAccountId: &HStringArg, webAccountUserName: &HStringArg, props: &::rt::gen::windows::foundation::collections::IMapView<HString, HString>, scope: WebAccountScope) -> Result<ComPtr<::rt::gen::windows::foundation::IAsyncOperation<super::super::super::credentials::WebAccount>>> { unsafe {
        <Self as RtActivatable<IWebAccountScopeManagerStatics>>::get_activation_factory().add_web_account_with_scope_async(webAccountId, webAccountUserName, props, scope)
    }}
    #[inline] pub fn set_scope_async(webAccount: &super::super::super::credentials::WebAccount, scope: WebAccountScope) -> Result<ComPtr<::rt::gen::windows::foundation::IAsyncAction>> { unsafe {
        <Self as RtActivatable<IWebAccountScopeManagerStatics>>::get_activation_factory().set_scope_async(webAccount, scope)
    }}
    #[inline] pub fn get_scope(webAccount: &super::super::super::credentials::WebAccount) -> Result<WebAccountScope> { unsafe {
        <Self as RtActivatable<IWebAccountScopeManagerStatics>>::get_activation_factory().get_scope(webAccount)
    }}
}
DEFINE_CLSID!(WebAccountManager(&[87,105,110,100,111,119,115,46,83,101,99,117,114,105,116,121,46,65,117,116,104,101,110,116,105,99,97,116,105,111,110,46,87,101,98,46,80,114,111,118,105,100,101,114,46,87,101,98,65,99,99,111,117,110,116,77,97,110,97,103,101,114,0]) [CLSID_WebAccountManager]);
DEFINE_IID!(IID_IWebAccountManagerStatics, 3001606566, 54426, 16434, 132, 191, 26, 40, 71, 116, 123, 241);
RT_INTERFACE!{static interface IWebAccountManagerStatics(IWebAccountManagerStaticsVtbl): IInspectable(IInspectableVtbl) [IID_IWebAccountManagerStatics] {
    fn UpdateWebAccountPropertiesAsync(&self, webAccount: *mut super::super::super::credentials::WebAccount, webAccountUserName: HSTRING, additionalProperties: *mut ::rt::gen::windows::foundation::collections::IMapView<HString, HString>, out: *mut *mut ::rt::gen::windows::foundation::IAsyncAction) -> HRESULT,
    fn AddWebAccountAsync(&self, webAccountId: HSTRING, webAccountUserName: HSTRING, props: *mut ::rt::gen::windows::foundation::collections::IMapView<HString, HString>, out: *mut *mut ::rt::gen::windows::foundation::IAsyncOperation<super::super::super::credentials::WebAccount>) -> HRESULT,
    fn DeleteWebAccountAsync(&self, webAccount: *mut super::super::super::credentials::WebAccount, out: *mut *mut ::rt::gen::windows::foundation::IAsyncAction) -> HRESULT,
    fn FindAllProviderWebAccountsAsync(&self, out: *mut *mut ::rt::gen::windows::foundation::IAsyncOperation<::rt::gen::windows::foundation::collections::IVectorView<super::super::super::credentials::WebAccount>>) -> HRESULT,
    #[cfg(not(feature="windows-web"))] fn __Dummy4(&self) -> (),
    #[cfg(feature="windows-web")] fn PushCookiesAsync(&self, uri: *mut ::rt::gen::windows::foundation::Uri, cookies: *mut ::rt::gen::windows::foundation::collections::IVectorView<::rt::gen::windows::web::http::HttpCookie>, out: *mut *mut ::rt::gen::windows::foundation::IAsyncAction) -> HRESULT,
    fn SetViewAsync(&self, webAccount: *mut super::super::super::credentials::WebAccount, view: *mut WebAccountClientView, out: *mut *mut ::rt::gen::windows::foundation::IAsyncAction) -> HRESULT,
    fn ClearViewAsync(&self, webAccount: *mut super::super::super::credentials::WebAccount, applicationCallbackUri: *mut ::rt::gen::windows::foundation::Uri, out: *mut *mut ::rt::gen::windows::foundation::IAsyncAction) -> HRESULT,
    fn GetViewsAsync(&self, webAccount: *mut super::super::super::credentials::WebAccount, out: *mut *mut ::rt::gen::windows::foundation::IAsyncOperation<::rt::gen::windows::foundation::collections::IVectorView<WebAccountClientView>>) -> HRESULT,
    #[cfg(not(feature="windows-storage"))] fn __Dummy8(&self) -> (),
    #[cfg(feature="windows-storage")] fn SetWebAccountPictureAsync(&self, webAccount: *mut super::super::super::credentials::WebAccount, webAccountPicture: *mut ::rt::gen::windows::storage::streams::IRandomAccessStream, out: *mut *mut ::rt::gen::windows::foundation::IAsyncAction) -> HRESULT,
    fn ClearWebAccountPictureAsync(&self, webAccount: *mut super::super::super::credentials::WebAccount, out: *mut *mut ::rt::gen::windows::foundation::IAsyncAction) -> HRESULT
}}
impl IWebAccountManagerStatics {
    #[inline] pub unsafe fn update_web_account_properties_async(&self, webAccount: &super::super::super::credentials::WebAccount, webAccountUserName: &HStringArg, additionalProperties: &::rt::gen::windows::foundation::collections::IMapView<HString, HString>) -> Result<ComPtr<::rt::gen::windows::foundation::IAsyncAction>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).UpdateWebAccountPropertiesAsync)(self as *const _ as *mut _, webAccount as *const _ as *mut _, webAccountUserName.get(), additionalProperties as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn add_web_account_async(&self, webAccountId: &HStringArg, webAccountUserName: &HStringArg, props: &::rt::gen::windows::foundation::collections::IMapView<HString, HString>) -> Result<ComPtr<::rt::gen::windows::foundation::IAsyncOperation<super::super::super::credentials::WebAccount>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).AddWebAccountAsync)(self as *const _ as *mut _, webAccountId.get(), webAccountUserName.get(), props as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn delete_web_account_async(&self, webAccount: &super::super::super::credentials::WebAccount) -> Result<ComPtr<::rt::gen::windows::foundation::IAsyncAction>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).DeleteWebAccountAsync)(self as *const _ as *mut _, webAccount as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn find_all_provider_web_accounts_async(&self) -> Result<ComPtr<::rt::gen::windows::foundation::IAsyncOperation<::rt::gen::windows::foundation::collections::IVectorView<super::super::super::credentials::WebAccount>>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).FindAllProviderWebAccountsAsync)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[cfg(feature="windows-web")] #[inline] pub unsafe fn push_cookies_async(&self, uri: &::rt::gen::windows::foundation::Uri, cookies: &::rt::gen::windows::foundation::collections::IVectorView<::rt::gen::windows::web::http::HttpCookie>) -> Result<ComPtr<::rt::gen::windows::foundation::IAsyncAction>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).PushCookiesAsync)(self as *const _ as *mut _, uri as *const _ as *mut _, cookies as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_view_async(&self, webAccount: &super::super::super::credentials::WebAccount, view: &WebAccountClientView) -> Result<ComPtr<::rt::gen::windows::foundation::IAsyncAction>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).SetViewAsync)(self as *const _ as *mut _, webAccount as *const _ as *mut _, view as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn clear_view_async(&self, webAccount: &super::super::super::credentials::WebAccount, applicationCallbackUri: &::rt::gen::windows::foundation::Uri) -> Result<ComPtr<::rt::gen::windows::foundation::IAsyncAction>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).ClearViewAsync)(self as *const _ as *mut _, webAccount as *const _ as *mut _, applicationCallbackUri as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_views_async(&self, webAccount: &super::super::super::credentials::WebAccount) -> Result<ComPtr<::rt::gen::windows::foundation::IAsyncOperation<::rt::gen::windows::foundation::collections::IVectorView<WebAccountClientView>>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetViewsAsync)(self as *const _ as *mut _, webAccount as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[cfg(feature="windows-storage")] #[inline] pub unsafe fn set_web_account_picture_async(&self, webAccount: &super::super::super::credentials::WebAccount, webAccountPicture: &::rt::gen::windows::storage::streams::IRandomAccessStream) -> Result<ComPtr<::rt::gen::windows::foundation::IAsyncAction>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).SetWebAccountPictureAsync)(self as *const _ as *mut _, webAccount as *const _ as *mut _, webAccountPicture as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn clear_web_account_picture_async(&self, webAccount: &super::super::super::credentials::WebAccount) -> Result<ComPtr<::rt::gen::windows::foundation::IAsyncAction>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).ClearWebAccountPictureAsync)(self as *const _ as *mut _, webAccount as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
DEFINE_IID!(IID_IWebAccountManagerStatics2, 1755818025, 11615, 18003, 139, 176, 189, 47, 166, 189, 45, 135);
RT_INTERFACE!{static interface IWebAccountManagerStatics2(IWebAccountManagerStatics2Vtbl): IInspectable(IInspectableVtbl) [IID_IWebAccountManagerStatics2] {
    fn PullCookiesAsync(&self, uriString: HSTRING, callerPFN: HSTRING, out: *mut *mut ::rt::gen::windows::foundation::IAsyncAction) -> HRESULT
}}
impl IWebAccountManagerStatics2 {
    #[inline] pub unsafe fn pull_cookies_async(&self, uriString: &HStringArg, callerPFN: &HStringArg) -> Result<ComPtr<::rt::gen::windows::foundation::IAsyncAction>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).PullCookiesAsync)(self as *const _ as *mut _, uriString.get(), callerPFN.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
DEFINE_IID!(IID_IWebAccountManagerStatics3, 3712295846, 35407, 19106, 177, 94, 3, 245, 80, 175, 19, 89);
RT_INTERFACE!{static interface IWebAccountManagerStatics3(IWebAccountManagerStatics3Vtbl): IInspectable(IInspectableVtbl) [IID_IWebAccountManagerStatics3] {
    #[cfg(feature="windows-system")] fn FindAllProviderWebAccountsForUserAsync(&self, user: *mut ::rt::gen::windows::system::User, out: *mut *mut ::rt::gen::windows::foundation::IAsyncOperation<::rt::gen::windows::foundation::collections::IVectorView<super::super::super::credentials::WebAccount>>) -> HRESULT,
    #[cfg(feature="windows-system")] fn AddWebAccountForUserAsync(&self, user: *mut ::rt::gen::windows::system::User, webAccountId: HSTRING, webAccountUserName: HSTRING, props: *mut ::rt::gen::windows::foundation::collections::IMapView<HString, HString>, out: *mut *mut ::rt::gen::windows::foundation::IAsyncOperation<super::super::super::credentials::WebAccount>) -> HRESULT,
    #[cfg(feature="windows-system")] fn AddWebAccountWithScopeForUserAsync(&self, user: *mut ::rt::gen::windows::system::User, webAccountId: HSTRING, webAccountUserName: HSTRING, props: *mut ::rt::gen::windows::foundation::collections::IMapView<HString, HString>, scope: WebAccountScope, out: *mut *mut ::rt::gen::windows::foundation::IAsyncOperation<super::super::super::credentials::WebAccount>) -> HRESULT,
    #[cfg(feature="windows-system")] fn AddWebAccountWithScopeAndMapForUserAsync(&self, user: *mut ::rt::gen::windows::system::User, webAccountId: HSTRING, webAccountUserName: HSTRING, props: *mut ::rt::gen::windows::foundation::collections::IMapView<HString, HString>, scope: WebAccountScope, perUserWebAccountId: HSTRING, out: *mut *mut ::rt::gen::windows::foundation::IAsyncOperation<super::super::super::credentials::WebAccount>) -> HRESULT
}}
impl IWebAccountManagerStatics3 {
    #[cfg(feature="windows-system")] #[inline] pub unsafe fn find_all_provider_web_accounts_for_user_async(&self, user: &::rt::gen::windows::system::User) -> Result<ComPtr<::rt::gen::windows::foundation::IAsyncOperation<::rt::gen::windows::foundation::collections::IVectorView<super::super::super::credentials::WebAccount>>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).FindAllProviderWebAccountsForUserAsync)(self as *const _ as *mut _, user as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[cfg(feature="windows-system")] #[inline] pub unsafe fn add_web_account_for_user_async(&self, user: &::rt::gen::windows::system::User, webAccountId: &HStringArg, webAccountUserName: &HStringArg, props: &::rt::gen::windows::foundation::collections::IMapView<HString, HString>) -> Result<ComPtr<::rt::gen::windows::foundation::IAsyncOperation<super::super::super::credentials::WebAccount>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).AddWebAccountForUserAsync)(self as *const _ as *mut _, user as *const _ as *mut _, webAccountId.get(), webAccountUserName.get(), props as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[cfg(feature="windows-system")] #[inline] pub unsafe fn add_web_account_with_scope_for_user_async(&self, user: &::rt::gen::windows::system::User, webAccountId: &HStringArg, webAccountUserName: &HStringArg, props: &::rt::gen::windows::foundation::collections::IMapView<HString, HString>, scope: WebAccountScope) -> Result<ComPtr<::rt::gen::windows::foundation::IAsyncOperation<super::super::super::credentials::WebAccount>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).AddWebAccountWithScopeForUserAsync)(self as *const _ as *mut _, user as *const _ as *mut _, webAccountId.get(), webAccountUserName.get(), props as *const _ as *mut _, scope, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[cfg(feature="windows-system")] #[inline] pub unsafe fn add_web_account_with_scope_and_map_for_user_async(&self, user: &::rt::gen::windows::system::User, webAccountId: &HStringArg, webAccountUserName: &HStringArg, props: &::rt::gen::windows::foundation::collections::IMapView<HString, HString>, scope: WebAccountScope, perUserWebAccountId: &HStringArg) -> Result<ComPtr<::rt::gen::windows::foundation::IAsyncOperation<super::super::super::credentials::WebAccount>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).AddWebAccountWithScopeAndMapForUserAsync)(self as *const _ as *mut _, user as *const _ as *mut _, webAccountId.get(), webAccountUserName.get(), props as *const _ as *mut _, scope, perUserWebAccountId.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
DEFINE_IID!(IID_IWebAccountManagerStatics4, 1508623058, 63451, 16687, 188, 63, 242, 254, 160, 68, 48, 180);
RT_INTERFACE!{static interface IWebAccountManagerStatics4(IWebAccountManagerStatics4Vtbl): IInspectable(IInspectableVtbl) [IID_IWebAccountManagerStatics4] {
    fn InvalidateAppCacheForAllAccountsAsync(&self, out: *mut *mut ::rt::gen::windows::foundation::IAsyncAction) -> HRESULT,
    fn InvalidateAppCacheForAccountAsync(&self, webAccount: *mut super::super::super::credentials::WebAccount, out: *mut *mut ::rt::gen::windows::foundation::IAsyncAction) -> HRESULT
}}
impl IWebAccountManagerStatics4 {
    #[inline] pub unsafe fn invalidate_app_cache_for_all_accounts_async(&self) -> Result<ComPtr<::rt::gen::windows::foundation::IAsyncAction>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).InvalidateAppCacheForAllAccountsAsync)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn invalidate_app_cache_for_account_async(&self, webAccount: &super::super::super::credentials::WebAccount) -> Result<ComPtr<::rt::gen::windows::foundation::IAsyncAction>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).InvalidateAppCacheForAccountAsync)(self as *const _ as *mut _, webAccount as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
DEFINE_IID!(IID_IWebAccountMapManagerStatics, 3908715631, 14875, 18596, 142, 144, 30, 89, 202, 111, 84, 219);
RT_INTERFACE!{static interface IWebAccountMapManagerStatics(IWebAccountMapManagerStaticsVtbl): IInspectable(IInspectableVtbl) [IID_IWebAccountMapManagerStatics] {
    fn AddWebAccountWithScopeAndMapAsync(&self, webAccountId: HSTRING, webAccountUserName: HSTRING, props: *mut ::rt::gen::windows::foundation::collections::IMapView<HString, HString>, scope: WebAccountScope, perUserWebAccountId: HSTRING, out: *mut *mut ::rt::gen::windows::foundation::IAsyncOperation<super::super::super::credentials::WebAccount>) -> HRESULT,
    fn SetPerAppToPerUserAccountAsync(&self, perAppAccount: *mut super::super::super::credentials::WebAccount, perUserWebAccountId: HSTRING, out: *mut *mut ::rt::gen::windows::foundation::IAsyncAction) -> HRESULT,
    fn GetPerUserFromPerAppAccountAsync(&self, perAppAccount: *mut super::super::super::credentials::WebAccount, out: *mut *mut ::rt::gen::windows::foundation::IAsyncOperation<super::super::super::credentials::WebAccount>) -> HRESULT,
    fn ClearPerUserFromPerAppAccountAsync(&self, perAppAccount: *mut super::super::super::credentials::WebAccount, out: *mut *mut ::rt::gen::windows::foundation::IAsyncAction) -> HRESULT
}}
impl IWebAccountMapManagerStatics {
    #[inline] pub unsafe fn add_web_account_with_scope_and_map_async(&self, webAccountId: &HStringArg, webAccountUserName: &HStringArg, props: &::rt::gen::windows::foundation::collections::IMapView<HString, HString>, scope: WebAccountScope, perUserWebAccountId: &HStringArg) -> Result<ComPtr<::rt::gen::windows::foundation::IAsyncOperation<super::super::super::credentials::WebAccount>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).AddWebAccountWithScopeAndMapAsync)(self as *const _ as *mut _, webAccountId.get(), webAccountUserName.get(), props as *const _ as *mut _, scope, perUserWebAccountId.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_per_app_to_per_user_account_async(&self, perAppAccount: &super::super::super::credentials::WebAccount, perUserWebAccountId: &HStringArg) -> Result<ComPtr<::rt::gen::windows::foundation::IAsyncAction>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).SetPerAppToPerUserAccountAsync)(self as *const _ as *mut _, perAppAccount as *const _ as *mut _, perUserWebAccountId.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_per_user_from_per_app_account_async(&self, perAppAccount: &super::super::super::credentials::WebAccount) -> Result<ComPtr<::rt::gen::windows::foundation::IAsyncOperation<super::super::super::credentials::WebAccount>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetPerUserFromPerAppAccountAsync)(self as *const _ as *mut _, perAppAccount as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn clear_per_user_from_per_app_account_async(&self, perAppAccount: &super::super::super::credentials::WebAccount) -> Result<ComPtr<::rt::gen::windows::foundation::IAsyncAction>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).ClearPerUserFromPerAppAccountAsync)(self as *const _ as *mut _, perAppAccount as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
DEFINE_IID!(IID_IWebAccountProviderAddAccountOperation, 1944837327, 17272, 19577, 147, 53, 165, 215, 171, 129, 89, 78);
RT_INTERFACE!{interface IWebAccountProviderAddAccountOperation(IWebAccountProviderAddAccountOperationVtbl): IInspectable(IInspectableVtbl) [IID_IWebAccountProviderAddAccountOperation] {
    fn ReportCompleted(&self) -> HRESULT
}}
impl IWebAccountProviderAddAccountOperation {
    #[inline] pub unsafe fn report_completed(&self) -> Result<()> {
        let hr = ((*self.lpVtbl).ReportCompleted)(self as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
}
RT_CLASS!{class WebAccountProviderAddAccountOperation: IWebAccountProviderAddAccountOperation}
DEFINE_IID!(IID_IWebAccountProviderBaseReportOperation, 3148131515, 39227, 19799, 187, 228, 20, 33, 227, 102, 139, 76);
RT_INTERFACE!{interface IWebAccountProviderBaseReportOperation(IWebAccountProviderBaseReportOperationVtbl): IInspectable(IInspectableVtbl) [IID_IWebAccountProviderBaseReportOperation] {
    fn ReportCompleted(&self) -> HRESULT,
    fn ReportError(&self, value: *mut super::core::WebProviderError) -> HRESULT
}}
impl IWebAccountProviderBaseReportOperation {
    #[inline] pub unsafe fn report_completed(&self) -> Result<()> {
        let hr = ((*self.lpVtbl).ReportCompleted)(self as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn report_error(&self, value: &super::core::WebProviderError) -> Result<()> {
        let hr = ((*self.lpVtbl).ReportError)(self as *const _ as *mut _, value as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
}
DEFINE_IID!(IID_IWebAccountProviderDeleteAccountOperation, 180046008, 40449, 18889, 163, 85, 125, 72, 202, 247, 214, 202);
RT_INTERFACE!{interface IWebAccountProviderDeleteAccountOperation(IWebAccountProviderDeleteAccountOperationVtbl): IInspectable(IInspectableVtbl) [IID_IWebAccountProviderDeleteAccountOperation] {
    fn get_WebAccount(&self, out: *mut *mut super::super::super::credentials::WebAccount) -> HRESULT
}}
impl IWebAccountProviderDeleteAccountOperation {
    #[inline] pub unsafe fn get_web_account(&self) -> Result<ComPtr<super::super::super::credentials::WebAccount>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_WebAccount)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
RT_CLASS!{class WebAccountProviderDeleteAccountOperation: IWebAccountProviderDeleteAccountOperation}
RT_CLASS!{class WebAccountProviderGetTokenSilentOperation: IWebAccountProviderTokenOperation}
DEFINE_IID!(IID_IWebAccountProviderManageAccountOperation, 3978353756, 53787, 17982, 169, 183, 193, 253, 14, 218, 233, 120);
RT_INTERFACE!{interface IWebAccountProviderManageAccountOperation(IWebAccountProviderManageAccountOperationVtbl): IInspectable(IInspectableVtbl) [IID_IWebAccountProviderManageAccountOperation] {
    fn get_WebAccount(&self, out: *mut *mut super::super::super::credentials::WebAccount) -> HRESULT,
    fn ReportCompleted(&self) -> HRESULT
}}
impl IWebAccountProviderManageAccountOperation {
    #[inline] pub unsafe fn get_web_account(&self) -> Result<ComPtr<super::super::super::credentials::WebAccount>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_WebAccount)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn report_completed(&self) -> Result<()> {
        let hr = ((*self.lpVtbl).ReportCompleted)(self as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
}
RT_CLASS!{class WebAccountProviderManageAccountOperation: IWebAccountProviderManageAccountOperation}
DEFINE_IID!(IID_IWebAccountProviderOperation, 1834820646, 4273, 16794, 164, 78, 249, 197, 22, 21, 116, 230);
RT_INTERFACE!{interface IWebAccountProviderOperation(IWebAccountProviderOperationVtbl): IInspectable(IInspectableVtbl) [IID_IWebAccountProviderOperation] {
    fn get_Kind(&self, out: *mut WebAccountProviderOperationKind) -> HRESULT
}}
impl IWebAccountProviderOperation {
    #[inline] pub unsafe fn get_kind(&self) -> Result<WebAccountProviderOperationKind> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_Kind)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
}
RT_ENUM! { enum WebAccountProviderOperationKind: i32 {
    RequestToken (WebAccountProviderOperationKind_RequestToken) = 0, GetTokenSilently (WebAccountProviderOperationKind_GetTokenSilently) = 1, AddAccount (WebAccountProviderOperationKind_AddAccount) = 2, ManageAccount (WebAccountProviderOperationKind_ManageAccount) = 3, DeleteAccount (WebAccountProviderOperationKind_DeleteAccount) = 4, RetrieveCookies (WebAccountProviderOperationKind_RetrieveCookies) = 5, SignOutAccount (WebAccountProviderOperationKind_SignOutAccount) = 6,
}}
RT_CLASS!{class WebAccountProviderRequestTokenOperation: IWebAccountProviderTokenOperation}
DEFINE_IID!(IID_IWebAccountProviderRetrieveCookiesOperation, 1510212673, 4003, 19121, 160, 28, 32, 177, 16, 53, 133, 148);
RT_INTERFACE!{interface IWebAccountProviderRetrieveCookiesOperation(IWebAccountProviderRetrieveCookiesOperationVtbl): IInspectable(IInspectableVtbl) [IID_IWebAccountProviderRetrieveCookiesOperation] {
    fn get_Context(&self, out: *mut *mut ::rt::gen::windows::foundation::Uri) -> HRESULT,
    #[cfg(not(feature="windows-web"))] fn __Dummy1(&self) -> (),
    #[cfg(feature="windows-web")] fn get_Cookies(&self, out: *mut *mut ::rt::gen::windows::foundation::collections::IVector<::rt::gen::windows::web::http::HttpCookie>) -> HRESULT,
    fn put_Uri(&self, uri: *mut ::rt::gen::windows::foundation::Uri) -> HRESULT,
    fn get_Uri(&self, out: *mut *mut ::rt::gen::windows::foundation::Uri) -> HRESULT,
    fn get_ApplicationCallbackUri(&self, out: *mut *mut ::rt::gen::windows::foundation::Uri) -> HRESULT
}}
impl IWebAccountProviderRetrieveCookiesOperation {
    #[inline] pub unsafe fn get_context(&self) -> Result<ComPtr<::rt::gen::windows::foundation::Uri>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Context)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[cfg(feature="windows-web")] #[inline] pub unsafe fn get_cookies(&self) -> Result<ComPtr<::rt::gen::windows::foundation::collections::IVector<::rt::gen::windows::web::http::HttpCookie>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Cookies)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_uri(&self, uri: &::rt::gen::windows::foundation::Uri) -> Result<()> {
        let hr = ((*self.lpVtbl).put_Uri)(self as *const _ as *mut _, uri as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_uri(&self) -> Result<ComPtr<::rt::gen::windows::foundation::Uri>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Uri)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_application_callback_uri(&self) -> Result<ComPtr<::rt::gen::windows::foundation::Uri>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_ApplicationCallbackUri)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
RT_CLASS!{class WebAccountProviderRetrieveCookiesOperation: IWebAccountProviderRetrieveCookiesOperation}
DEFINE_IID!(IID_IWebAccountProviderSignOutAccountOperation, 3096502813, 3157, 18364, 140, 114, 4, 166, 252, 124, 172, 7);
RT_INTERFACE!{interface IWebAccountProviderSignOutAccountOperation(IWebAccountProviderSignOutAccountOperationVtbl): IInspectable(IInspectableVtbl) [IID_IWebAccountProviderSignOutAccountOperation] {
    fn get_WebAccount(&self, out: *mut *mut super::super::super::credentials::WebAccount) -> HRESULT,
    fn get_ApplicationCallbackUri(&self, out: *mut *mut ::rt::gen::windows::foundation::Uri) -> HRESULT,
    fn get_ClientId(&self, out: *mut HSTRING) -> HRESULT
}}
impl IWebAccountProviderSignOutAccountOperation {
    #[inline] pub unsafe fn get_web_account(&self) -> Result<ComPtr<super::super::super::credentials::WebAccount>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_WebAccount)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_application_callback_uri(&self) -> Result<ComPtr<::rt::gen::windows::foundation::Uri>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_ApplicationCallbackUri)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_client_id(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_ClientId)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
}
RT_CLASS!{class WebAccountProviderSignOutAccountOperation: IWebAccountProviderSignOutAccountOperation}
DEFINE_IID!(IID_IWebAccountProviderSilentReportOperation, 3769976312, 15119, 17626, 146, 76, 123, 24, 186, 170, 98, 169);
RT_INTERFACE!{interface IWebAccountProviderSilentReportOperation(IWebAccountProviderSilentReportOperationVtbl): IInspectable(IInspectableVtbl) [IID_IWebAccountProviderSilentReportOperation] {
    fn ReportUserInteractionRequired(&self) -> HRESULT,
    fn ReportUserInteractionRequiredWithError(&self, value: *mut super::core::WebProviderError) -> HRESULT
}}
impl IWebAccountProviderSilentReportOperation {
    #[inline] pub unsafe fn report_user_interaction_required(&self) -> Result<()> {
        let hr = ((*self.lpVtbl).ReportUserInteractionRequired)(self as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn report_user_interaction_required_with_error(&self, value: &super::core::WebProviderError) -> Result<()> {
        let hr = ((*self.lpVtbl).ReportUserInteractionRequiredWithError)(self as *const _ as *mut _, value as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
}
DEFINE_IID!(IID_IWebAccountProviderTokenObjects, 1083123787, 4904, 17115, 137, 164, 11, 206, 122, 113, 125, 142);
RT_INTERFACE!{interface IWebAccountProviderTokenObjects(IWebAccountProviderTokenObjectsVtbl): IInspectable(IInspectableVtbl) [IID_IWebAccountProviderTokenObjects] {
    fn get_Operation(&self, out: *mut *mut IWebAccountProviderOperation) -> HRESULT
}}
impl IWebAccountProviderTokenObjects {
    #[inline] pub unsafe fn get_operation(&self) -> Result<ComPtr<IWebAccountProviderOperation>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Operation)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
DEFINE_IID!(IID_IWebAccountProviderTokenObjects2, 270579859, 23717, 20479, 149, 251, 184, 32, 39, 63, 195, 149);
RT_INTERFACE!{interface IWebAccountProviderTokenObjects2(IWebAccountProviderTokenObjects2Vtbl): IInspectable(IInspectableVtbl) [IID_IWebAccountProviderTokenObjects2] {
    #[cfg(feature="windows-system")] fn get_User(&self, out: *mut *mut ::rt::gen::windows::system::User) -> HRESULT
}}
impl IWebAccountProviderTokenObjects2 {
    #[cfg(feature="windows-system")] #[inline] pub unsafe fn get_user(&self) -> Result<ComPtr<::rt::gen::windows::system::User>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_User)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
DEFINE_IID!(IID_IWebAccountProviderTokenOperation, 2512786366, 8244, 19512, 148, 52, 210, 108, 20, 178, 180, 178);
RT_INTERFACE!{interface IWebAccountProviderTokenOperation(IWebAccountProviderTokenOperationVtbl): IInspectable(IInspectableVtbl) [IID_IWebAccountProviderTokenOperation] {
    fn get_ProviderRequest(&self, out: *mut *mut WebProviderTokenRequest) -> HRESULT,
    fn get_ProviderResponses(&self, out: *mut *mut ::rt::gen::windows::foundation::collections::IVector<WebProviderTokenResponse>) -> HRESULT,
    fn put_CacheExpirationTime(&self, value: ::rt::gen::windows::foundation::DateTime) -> HRESULT,
    fn get_CacheExpirationTime(&self, out: *mut ::rt::gen::windows::foundation::DateTime) -> HRESULT
}}
impl IWebAccountProviderTokenOperation {
    #[inline] pub unsafe fn get_provider_request(&self) -> Result<ComPtr<WebProviderTokenRequest>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_ProviderRequest)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_provider_responses(&self) -> Result<ComPtr<::rt::gen::windows::foundation::collections::IVector<WebProviderTokenResponse>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_ProviderResponses)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_cache_expiration_time(&self, value: ::rt::gen::windows::foundation::DateTime) -> Result<()> {
        let hr = ((*self.lpVtbl).put_CacheExpirationTime)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_cache_expiration_time(&self) -> Result<::rt::gen::windows::foundation::DateTime> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_CacheExpirationTime)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
}
RT_CLASS!{class WebAccountProviderTriggerDetails: IWebAccountProviderTokenObjects}
DEFINE_IID!(IID_IWebAccountProviderUIReportOperation, 687837907, 36736, 17147, 148, 79, 178, 16, 123, 189, 66, 230);
RT_INTERFACE!{interface IWebAccountProviderUIReportOperation(IWebAccountProviderUIReportOperationVtbl): IInspectable(IInspectableVtbl) [IID_IWebAccountProviderUIReportOperation] {
    fn ReportUserCanceled(&self) -> HRESULT
}}
impl IWebAccountProviderUIReportOperation {
    #[inline] pub unsafe fn report_user_canceled(&self) -> Result<()> {
        let hr = ((*self.lpVtbl).ReportUserCanceled)(self as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
}
RT_ENUM! { enum WebAccountScope: i32 {
    PerUser (WebAccountScope_PerUser) = 0, PerApplication (WebAccountScope_PerApplication) = 1,
}}
DEFINE_IID!(IID_IWebAccountScopeManagerStatics, 1550639996, 4786, 16954, 191, 61, 133, 184, 215, 229, 54, 86);
RT_INTERFACE!{static interface IWebAccountScopeManagerStatics(IWebAccountScopeManagerStaticsVtbl): IInspectable(IInspectableVtbl) [IID_IWebAccountScopeManagerStatics] {
    fn AddWebAccountWithScopeAsync(&self, webAccountId: HSTRING, webAccountUserName: HSTRING, props: *mut ::rt::gen::windows::foundation::collections::IMapView<HString, HString>, scope: WebAccountScope, out: *mut *mut ::rt::gen::windows::foundation::IAsyncOperation<super::super::super::credentials::WebAccount>) -> HRESULT,
    fn SetScopeAsync(&self, webAccount: *mut super::super::super::credentials::WebAccount, scope: WebAccountScope, out: *mut *mut ::rt::gen::windows::foundation::IAsyncAction) -> HRESULT,
    fn GetScope(&self, webAccount: *mut super::super::super::credentials::WebAccount, out: *mut WebAccountScope) -> HRESULT
}}
impl IWebAccountScopeManagerStatics {
    #[inline] pub unsafe fn add_web_account_with_scope_async(&self, webAccountId: &HStringArg, webAccountUserName: &HStringArg, props: &::rt::gen::windows::foundation::collections::IMapView<HString, HString>, scope: WebAccountScope) -> Result<ComPtr<::rt::gen::windows::foundation::IAsyncOperation<super::super::super::credentials::WebAccount>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).AddWebAccountWithScopeAsync)(self as *const _ as *mut _, webAccountId.get(), webAccountUserName.get(), props as *const _ as *mut _, scope, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_scope_async(&self, webAccount: &super::super::super::credentials::WebAccount, scope: WebAccountScope) -> Result<ComPtr<::rt::gen::windows::foundation::IAsyncAction>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).SetScopeAsync)(self as *const _ as *mut _, webAccount as *const _ as *mut _, scope, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_scope(&self, webAccount: &super::super::super::credentials::WebAccount) -> Result<WebAccountScope> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).GetScope)(self as *const _ as *mut _, webAccount as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
}
RT_ENUM! { enum WebAccountSelectionOptions: u32 {
    Default (WebAccountSelectionOptions_Default) = 0, New (WebAccountSelectionOptions_New) = 1,
}}
DEFINE_IID!(IID_IWebProviderTokenRequest, 504919947, 34821, 17739, 159, 17, 70, 141, 42, 241, 9, 90);
RT_INTERFACE!{interface IWebProviderTokenRequest(IWebProviderTokenRequestVtbl): IInspectable(IInspectableVtbl) [IID_IWebProviderTokenRequest] {
    fn get_ClientRequest(&self, out: *mut *mut super::core::WebTokenRequest) -> HRESULT,
    fn get_WebAccounts(&self, out: *mut *mut ::rt::gen::windows::foundation::collections::IVectorView<super::super::super::credentials::WebAccount>) -> HRESULT,
    fn get_WebAccountSelectionOptions(&self, out: *mut WebAccountSelectionOptions) -> HRESULT,
    fn get_ApplicationCallbackUri(&self, out: *mut *mut ::rt::gen::windows::foundation::Uri) -> HRESULT,
    fn GetApplicationTokenBindingKeyAsync(&self, keyType: super::TokenBindingKeyType, target: *mut ::rt::gen::windows::foundation::Uri, out: *mut *mut ::rt::gen::windows::foundation::IAsyncOperation<super::super::super::cryptography::core::CryptographicKey>) -> HRESULT
}}
impl IWebProviderTokenRequest {
    #[inline] pub unsafe fn get_client_request(&self) -> Result<ComPtr<super::core::WebTokenRequest>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_ClientRequest)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_web_accounts(&self) -> Result<ComPtr<::rt::gen::windows::foundation::collections::IVectorView<super::super::super::credentials::WebAccount>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_WebAccounts)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_web_account_selection_options(&self) -> Result<WebAccountSelectionOptions> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_WebAccountSelectionOptions)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_application_callback_uri(&self) -> Result<ComPtr<::rt::gen::windows::foundation::Uri>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_ApplicationCallbackUri)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_application_token_binding_key_async(&self, keyType: super::TokenBindingKeyType, target: &::rt::gen::windows::foundation::Uri) -> Result<ComPtr<::rt::gen::windows::foundation::IAsyncOperation<super::super::super::cryptography::core::CryptographicKey>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetApplicationTokenBindingKeyAsync)(self as *const _ as *mut _, keyType, target as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
RT_CLASS!{class WebProviderTokenRequest: IWebProviderTokenRequest}
DEFINE_IID!(IID_IWebProviderTokenRequest2, 3050778188, 4273, 19110, 136, 177, 11, 108, 158, 12, 30, 70);
RT_INTERFACE!{interface IWebProviderTokenRequest2(IWebProviderTokenRequest2Vtbl): IInspectable(IInspectableVtbl) [IID_IWebProviderTokenRequest2] {
    #[cfg(feature="windows-storage")] fn GetApplicationTokenBindingKeyIdAsync(&self, keyType: super::TokenBindingKeyType, target: *mut ::rt::gen::windows::foundation::Uri, out: *mut *mut ::rt::gen::windows::foundation::IAsyncOperation<::rt::gen::windows::storage::streams::IBuffer>) -> HRESULT
}}
impl IWebProviderTokenRequest2 {
    #[cfg(feature="windows-storage")] #[inline] pub unsafe fn get_application_token_binding_key_id_async(&self, keyType: super::TokenBindingKeyType, target: &::rt::gen::windows::foundation::Uri) -> Result<ComPtr<::rt::gen::windows::foundation::IAsyncOperation<::rt::gen::windows::storage::streams::IBuffer>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetApplicationTokenBindingKeyIdAsync)(self as *const _ as *mut _, keyType, target as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
DEFINE_IID!(IID_IWebProviderTokenResponse, 4011931539, 61269, 16774, 183, 206, 140, 178, 231, 249, 132, 158);
RT_INTERFACE!{interface IWebProviderTokenResponse(IWebProviderTokenResponseVtbl): IInspectable(IInspectableVtbl) [IID_IWebProviderTokenResponse] {
    fn get_ClientResponse(&self, out: *mut *mut super::core::WebTokenResponse) -> HRESULT
}}
impl IWebProviderTokenResponse {
    #[inline] pub unsafe fn get_client_response(&self) -> Result<ComPtr<super::core::WebTokenResponse>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_ClientResponse)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
RT_CLASS!{class WebProviderTokenResponse: IWebProviderTokenResponse}
impl RtActivatable<IWebProviderTokenResponseFactory> for WebProviderTokenResponse {}
impl WebProviderTokenResponse {
    #[inline] pub fn create(webTokenResponse: &super::core::WebTokenResponse) -> Result<ComPtr<WebProviderTokenResponse>> { unsafe {
        <Self as RtActivatable<IWebProviderTokenResponseFactory>>::get_activation_factory().create(webTokenResponse)
    }}
}
DEFINE_CLSID!(WebProviderTokenResponse(&[87,105,110,100,111,119,115,46,83,101,99,117,114,105,116,121,46,65,117,116,104,101,110,116,105,99,97,116,105,111,110,46,87,101,98,46,80,114,111,118,105,100,101,114,46,87,101,98,80,114,111,118,105,100,101,114,84,111,107,101,110,82,101,115,112,111,110,115,101,0]) [CLSID_WebProviderTokenResponse]);
DEFINE_IID!(IID_IWebProviderTokenResponseFactory, 4199143834, 9658, 16503, 156, 250, 157, 180, 222, 167, 183, 26);
RT_INTERFACE!{static interface IWebProviderTokenResponseFactory(IWebProviderTokenResponseFactoryVtbl): IInspectable(IInspectableVtbl) [IID_IWebProviderTokenResponseFactory] {
    fn Create(&self, webTokenResponse: *mut super::core::WebTokenResponse, out: *mut *mut WebProviderTokenResponse) -> HRESULT
}}
impl IWebProviderTokenResponseFactory {
    #[inline] pub unsafe fn create(&self, webTokenResponse: &super::core::WebTokenResponse) -> Result<ComPtr<WebProviderTokenResponse>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).Create)(self as *const _ as *mut _, webTokenResponse as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
} // Windows.Security.Authentication.Web.Provider
pub mod core { // Windows.Security.Authentication.Web.Core
use ::prelude::*;
DEFINE_IID!(IID_IWebAccountEventArgs, 1874264957, 16974, 17644, 151, 124, 239, 36, 21, 70, 42, 90);
RT_INTERFACE!{interface IWebAccountEventArgs(IWebAccountEventArgsVtbl): IInspectable(IInspectableVtbl) [IID_IWebAccountEventArgs] {
    fn get_Account(&self, out: *mut *mut super::super::super::credentials::WebAccount) -> HRESULT
}}
impl IWebAccountEventArgs {
    #[inline] pub unsafe fn get_account(&self) -> Result<ComPtr<super::super::super::credentials::WebAccount>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Account)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
RT_CLASS!{class WebAccountEventArgs: IWebAccountEventArgs}
DEFINE_IID!(IID_IWebAccountMonitor, 1950742013, 43677, 17945, 141, 93, 193, 56, 164, 237, 227, 229);
RT_INTERFACE!{interface IWebAccountMonitor(IWebAccountMonitorVtbl): IInspectable(IInspectableVtbl) [IID_IWebAccountMonitor] {
    fn add_Updated(&self, handler: *mut ::rt::gen::windows::foundation::TypedEventHandler<WebAccountMonitor, WebAccountEventArgs>, out: *mut ::rt::gen::windows::foundation::EventRegistrationToken) -> HRESULT,
    fn remove_Updated(&self, token: ::rt::gen::windows::foundation::EventRegistrationToken) -> HRESULT,
    fn add_Removed(&self, handler: *mut ::rt::gen::windows::foundation::TypedEventHandler<WebAccountMonitor, WebAccountEventArgs>, out: *mut ::rt::gen::windows::foundation::EventRegistrationToken) -> HRESULT,
    fn remove_Removed(&self, token: ::rt::gen::windows::foundation::EventRegistrationToken) -> HRESULT,
    fn add_DefaultSignInAccountChanged(&self, handler: *mut ::rt::gen::windows::foundation::TypedEventHandler<WebAccountMonitor, IInspectable>, out: *mut ::rt::gen::windows::foundation::EventRegistrationToken) -> HRESULT,
    fn remove_DefaultSignInAccountChanged(&self, token: ::rt::gen::windows::foundation::EventRegistrationToken) -> HRESULT
}}
impl IWebAccountMonitor {
    #[inline] pub unsafe fn add_updated(&self, handler: &::rt::gen::windows::foundation::TypedEventHandler<WebAccountMonitor, WebAccountEventArgs>) -> Result<::rt::gen::windows::foundation::EventRegistrationToken> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).add_Updated)(self as *const _ as *mut _, handler as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn remove_updated(&self, token: ::rt::gen::windows::foundation::EventRegistrationToken) -> Result<()> {
        let hr = ((*self.lpVtbl).remove_Updated)(self as *const _ as *mut _, token);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn add_removed(&self, handler: &::rt::gen::windows::foundation::TypedEventHandler<WebAccountMonitor, WebAccountEventArgs>) -> Result<::rt::gen::windows::foundation::EventRegistrationToken> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).add_Removed)(self as *const _ as *mut _, handler as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn remove_removed(&self, token: ::rt::gen::windows::foundation::EventRegistrationToken) -> Result<()> {
        let hr = ((*self.lpVtbl).remove_Removed)(self as *const _ as *mut _, token);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn add_default_sign_in_account_changed(&self, handler: &::rt::gen::windows::foundation::TypedEventHandler<WebAccountMonitor, IInspectable>) -> Result<::rt::gen::windows::foundation::EventRegistrationToken> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).add_DefaultSignInAccountChanged)(self as *const _ as *mut _, handler as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn remove_default_sign_in_account_changed(&self, token: ::rt::gen::windows::foundation::EventRegistrationToken) -> Result<()> {
        let hr = ((*self.lpVtbl).remove_DefaultSignInAccountChanged)(self as *const _ as *mut _, token);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
}
RT_CLASS!{class WebAccountMonitor: IWebAccountMonitor}
RT_CLASS!{static class WebAuthenticationCoreManager}
impl RtActivatable<IWebAuthenticationCoreManagerStatics> for WebAuthenticationCoreManager {}
impl RtActivatable<IWebAuthenticationCoreManagerStatics2> for WebAuthenticationCoreManager {}
impl RtActivatable<IWebAuthenticationCoreManagerStatics3> for WebAuthenticationCoreManager {}
impl WebAuthenticationCoreManager {
    #[inline] pub fn get_token_silently_async(request: &WebTokenRequest) -> Result<ComPtr<::rt::gen::windows::foundation::IAsyncOperation<WebTokenRequestResult>>> { unsafe {
        <Self as RtActivatable<IWebAuthenticationCoreManagerStatics>>::get_activation_factory().get_token_silently_async(request)
    }}
    #[inline] pub fn get_token_silently_with_web_account_async(request: &WebTokenRequest, webAccount: &super::super::super::credentials::WebAccount) -> Result<ComPtr<::rt::gen::windows::foundation::IAsyncOperation<WebTokenRequestResult>>> { unsafe {
        <Self as RtActivatable<IWebAuthenticationCoreManagerStatics>>::get_activation_factory().get_token_silently_with_web_account_async(request, webAccount)
    }}
    #[inline] pub fn request_token_async(request: &WebTokenRequest) -> Result<ComPtr<::rt::gen::windows::foundation::IAsyncOperation<WebTokenRequestResult>>> { unsafe {
        <Self as RtActivatable<IWebAuthenticationCoreManagerStatics>>::get_activation_factory().request_token_async(request)
    }}
    #[inline] pub fn request_token_with_web_account_async(request: &WebTokenRequest, webAccount: &super::super::super::credentials::WebAccount) -> Result<ComPtr<::rt::gen::windows::foundation::IAsyncOperation<WebTokenRequestResult>>> { unsafe {
        <Self as RtActivatable<IWebAuthenticationCoreManagerStatics>>::get_activation_factory().request_token_with_web_account_async(request, webAccount)
    }}
    #[inline] pub fn find_account_async(provider: &super::super::super::credentials::WebAccountProvider, webAccountId: &HStringArg) -> Result<ComPtr<::rt::gen::windows::foundation::IAsyncOperation<super::super::super::credentials::WebAccount>>> { unsafe {
        <Self as RtActivatable<IWebAuthenticationCoreManagerStatics>>::get_activation_factory().find_account_async(provider, webAccountId)
    }}
    #[inline] pub fn find_account_provider_async(webAccountProviderId: &HStringArg) -> Result<ComPtr<::rt::gen::windows::foundation::IAsyncOperation<super::super::super::credentials::WebAccountProvider>>> { unsafe {
        <Self as RtActivatable<IWebAuthenticationCoreManagerStatics>>::get_activation_factory().find_account_provider_async(webAccountProviderId)
    }}
    #[inline] pub fn find_account_provider_with_authority_async(webAccountProviderId: &HStringArg, authority: &HStringArg) -> Result<ComPtr<::rt::gen::windows::foundation::IAsyncOperation<super::super::super::credentials::WebAccountProvider>>> { unsafe {
        <Self as RtActivatable<IWebAuthenticationCoreManagerStatics>>::get_activation_factory().find_account_provider_with_authority_async(webAccountProviderId, authority)
    }}
    #[cfg(feature="windows-system")] #[inline] pub fn find_account_provider_with_authority_for_user_async(webAccountProviderId: &HStringArg, authority: &HStringArg, user: &::rt::gen::windows::system::User) -> Result<ComPtr<::rt::gen::windows::foundation::IAsyncOperation<super::super::super::credentials::WebAccountProvider>>> { unsafe {
        <Self as RtActivatable<IWebAuthenticationCoreManagerStatics2>>::get_activation_factory().find_account_provider_with_authority_for_user_async(webAccountProviderId, authority, user)
    }}
    #[inline] pub fn create_web_account_monitor(webAccounts: &::rt::gen::windows::foundation::collections::IIterable<super::super::super::credentials::WebAccount>) -> Result<ComPtr<WebAccountMonitor>> { unsafe {
        <Self as RtActivatable<IWebAuthenticationCoreManagerStatics3>>::get_activation_factory().create_web_account_monitor(webAccounts)
    }}
}
DEFINE_CLSID!(WebAuthenticationCoreManager(&[87,105,110,100,111,119,115,46,83,101,99,117,114,105,116,121,46,65,117,116,104,101,110,116,105,99,97,116,105,111,110,46,87,101,98,46,67,111,114,101,46,87,101,98,65,117,116,104,101,110,116,105,99,97,116,105,111,110,67,111,114,101,77,97,110,97,103,101,114,0]) [CLSID_WebAuthenticationCoreManager]);
DEFINE_IID!(IID_IWebAuthenticationCoreManagerStatics, 1791655058, 42369, 17529, 156, 16, 117, 46, 255, 68, 253, 52);
RT_INTERFACE!{static interface IWebAuthenticationCoreManagerStatics(IWebAuthenticationCoreManagerStaticsVtbl): IInspectable(IInspectableVtbl) [IID_IWebAuthenticationCoreManagerStatics] {
    fn GetTokenSilentlyAsync(&self, request: *mut WebTokenRequest, out: *mut *mut ::rt::gen::windows::foundation::IAsyncOperation<WebTokenRequestResult>) -> HRESULT,
    fn GetTokenSilentlyWithWebAccountAsync(&self, request: *mut WebTokenRequest, webAccount: *mut super::super::super::credentials::WebAccount, out: *mut *mut ::rt::gen::windows::foundation::IAsyncOperation<WebTokenRequestResult>) -> HRESULT,
    fn RequestTokenAsync(&self, request: *mut WebTokenRequest, out: *mut *mut ::rt::gen::windows::foundation::IAsyncOperation<WebTokenRequestResult>) -> HRESULT,
    fn RequestTokenWithWebAccountAsync(&self, request: *mut WebTokenRequest, webAccount: *mut super::super::super::credentials::WebAccount, out: *mut *mut ::rt::gen::windows::foundation::IAsyncOperation<WebTokenRequestResult>) -> HRESULT,
    fn FindAccountAsync(&self, provider: *mut super::super::super::credentials::WebAccountProvider, webAccountId: HSTRING, out: *mut *mut ::rt::gen::windows::foundation::IAsyncOperation<super::super::super::credentials::WebAccount>) -> HRESULT,
    fn FindAccountProviderAsync(&self, webAccountProviderId: HSTRING, out: *mut *mut ::rt::gen::windows::foundation::IAsyncOperation<super::super::super::credentials::WebAccountProvider>) -> HRESULT,
    fn FindAccountProviderWithAuthorityAsync(&self, webAccountProviderId: HSTRING, authority: HSTRING, out: *mut *mut ::rt::gen::windows::foundation::IAsyncOperation<super::super::super::credentials::WebAccountProvider>) -> HRESULT
}}
impl IWebAuthenticationCoreManagerStatics {
    #[inline] pub unsafe fn get_token_silently_async(&self, request: &WebTokenRequest) -> Result<ComPtr<::rt::gen::windows::foundation::IAsyncOperation<WebTokenRequestResult>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetTokenSilentlyAsync)(self as *const _ as *mut _, request as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_token_silently_with_web_account_async(&self, request: &WebTokenRequest, webAccount: &super::super::super::credentials::WebAccount) -> Result<ComPtr<::rt::gen::windows::foundation::IAsyncOperation<WebTokenRequestResult>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetTokenSilentlyWithWebAccountAsync)(self as *const _ as *mut _, request as *const _ as *mut _, webAccount as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn request_token_async(&self, request: &WebTokenRequest) -> Result<ComPtr<::rt::gen::windows::foundation::IAsyncOperation<WebTokenRequestResult>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).RequestTokenAsync)(self as *const _ as *mut _, request as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn request_token_with_web_account_async(&self, request: &WebTokenRequest, webAccount: &super::super::super::credentials::WebAccount) -> Result<ComPtr<::rt::gen::windows::foundation::IAsyncOperation<WebTokenRequestResult>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).RequestTokenWithWebAccountAsync)(self as *const _ as *mut _, request as *const _ as *mut _, webAccount as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn find_account_async(&self, provider: &super::super::super::credentials::WebAccountProvider, webAccountId: &HStringArg) -> Result<ComPtr<::rt::gen::windows::foundation::IAsyncOperation<super::super::super::credentials::WebAccount>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).FindAccountAsync)(self as *const _ as *mut _, provider as *const _ as *mut _, webAccountId.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn find_account_provider_async(&self, webAccountProviderId: &HStringArg) -> Result<ComPtr<::rt::gen::windows::foundation::IAsyncOperation<super::super::super::credentials::WebAccountProvider>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).FindAccountProviderAsync)(self as *const _ as *mut _, webAccountProviderId.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn find_account_provider_with_authority_async(&self, webAccountProviderId: &HStringArg, authority: &HStringArg) -> Result<ComPtr<::rt::gen::windows::foundation::IAsyncOperation<super::super::super::credentials::WebAccountProvider>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).FindAccountProviderWithAuthorityAsync)(self as *const _ as *mut _, webAccountProviderId.get(), authority.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
DEFINE_IID!(IID_IWebAuthenticationCoreManagerStatics2, 4119074890, 35671, 18464, 182, 164, 112, 165, 182, 252, 244, 74);
RT_INTERFACE!{static interface IWebAuthenticationCoreManagerStatics2(IWebAuthenticationCoreManagerStatics2Vtbl): IInspectable(IInspectableVtbl) [IID_IWebAuthenticationCoreManagerStatics2] {
    #[cfg(feature="windows-system")] fn FindAccountProviderWithAuthorityForUserAsync(&self, webAccountProviderId: HSTRING, authority: HSTRING, user: *mut ::rt::gen::windows::system::User, out: *mut *mut ::rt::gen::windows::foundation::IAsyncOperation<super::super::super::credentials::WebAccountProvider>) -> HRESULT
}}
impl IWebAuthenticationCoreManagerStatics2 {
    #[cfg(feature="windows-system")] #[inline] pub unsafe fn find_account_provider_with_authority_for_user_async(&self, webAccountProviderId: &HStringArg, authority: &HStringArg, user: &::rt::gen::windows::system::User) -> Result<ComPtr<::rt::gen::windows::foundation::IAsyncOperation<super::super::super::credentials::WebAccountProvider>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).FindAccountProviderWithAuthorityForUserAsync)(self as *const _ as *mut _, webAccountProviderId.get(), authority.get(), user as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
DEFINE_IID!(IID_IWebAuthenticationCoreManagerStatics3, 604303026, 35108, 19859, 171, 58, 153, 104, 139, 65, 157, 86);
RT_INTERFACE!{static interface IWebAuthenticationCoreManagerStatics3(IWebAuthenticationCoreManagerStatics3Vtbl): IInspectable(IInspectableVtbl) [IID_IWebAuthenticationCoreManagerStatics3] {
    fn CreateWebAccountMonitor(&self, webAccounts: *mut ::rt::gen::windows::foundation::collections::IIterable<super::super::super::credentials::WebAccount>, out: *mut *mut WebAccountMonitor) -> HRESULT
}}
impl IWebAuthenticationCoreManagerStatics3 {
    #[inline] pub unsafe fn create_web_account_monitor(&self, webAccounts: &::rt::gen::windows::foundation::collections::IIterable<super::super::super::credentials::WebAccount>) -> Result<ComPtr<WebAccountMonitor>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).CreateWebAccountMonitor)(self as *const _ as *mut _, webAccounts as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
DEFINE_IID!(IID_IWebProviderError, 3675855793, 20677, 18441, 141, 202, 9, 201, 148, 16, 36, 92);
RT_INTERFACE!{interface IWebProviderError(IWebProviderErrorVtbl): IInspectable(IInspectableVtbl) [IID_IWebProviderError] {
    fn get_ErrorCode(&self, out: *mut u32) -> HRESULT,
    fn get_ErrorMessage(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Properties(&self, out: *mut *mut ::rt::gen::windows::foundation::collections::IMap<HString, HString>) -> HRESULT
}}
impl IWebProviderError {
    #[inline] pub unsafe fn get_error_code(&self) -> Result<u32> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_ErrorCode)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_error_message(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_ErrorMessage)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_properties(&self) -> Result<ComPtr<::rt::gen::windows::foundation::collections::IMap<HString, HString>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Properties)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
RT_CLASS!{class WebProviderError: IWebProviderError}
impl RtActivatable<IWebProviderErrorFactory> for WebProviderError {}
impl WebProviderError {
    #[inline] pub fn create(errorCode: u32, errorMessage: &HStringArg) -> Result<ComPtr<WebProviderError>> { unsafe {
        <Self as RtActivatable<IWebProviderErrorFactory>>::get_activation_factory().create(errorCode, errorMessage)
    }}
}
DEFINE_CLSID!(WebProviderError(&[87,105,110,100,111,119,115,46,83,101,99,117,114,105,116,121,46,65,117,116,104,101,110,116,105,99,97,116,105,111,110,46,87,101,98,46,67,111,114,101,46,87,101,98,80,114,111,118,105,100,101,114,69,114,114,111,114,0]) [CLSID_WebProviderError]);
DEFINE_IID!(IID_IWebProviderErrorFactory, 3821275693, 35311, 20023, 132, 127, 168, 185, 213, 163, 41, 16);
RT_INTERFACE!{static interface IWebProviderErrorFactory(IWebProviderErrorFactoryVtbl): IInspectable(IInspectableVtbl) [IID_IWebProviderErrorFactory] {
    fn Create(&self, errorCode: u32, errorMessage: HSTRING, out: *mut *mut WebProviderError) -> HRESULT
}}
impl IWebProviderErrorFactory {
    #[inline] pub unsafe fn create(&self, errorCode: u32, errorMessage: &HStringArg) -> Result<ComPtr<WebProviderError>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).Create)(self as *const _ as *mut _, errorCode, errorMessage.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
DEFINE_IID!(IID_IWebTokenRequest, 3078311272, 44491, 18035, 179, 100, 12, 247, 179, 92, 175, 151);
RT_INTERFACE!{interface IWebTokenRequest(IWebTokenRequestVtbl): IInspectable(IInspectableVtbl) [IID_IWebTokenRequest] {
    fn get_WebAccountProvider(&self, out: *mut *mut super::super::super::credentials::WebAccountProvider) -> HRESULT,
    fn get_Scope(&self, out: *mut HSTRING) -> HRESULT,
    fn get_ClientId(&self, out: *mut HSTRING) -> HRESULT,
    fn get_PromptType(&self, out: *mut WebTokenRequestPromptType) -> HRESULT,
    fn get_Properties(&self, out: *mut *mut ::rt::gen::windows::foundation::collections::IMap<HString, HString>) -> HRESULT
}}
impl IWebTokenRequest {
    #[inline] pub unsafe fn get_web_account_provider(&self) -> Result<ComPtr<super::super::super::credentials::WebAccountProvider>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_WebAccountProvider)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_scope(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Scope)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_client_id(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_ClientId)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_prompt_type(&self) -> Result<WebTokenRequestPromptType> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_PromptType)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_properties(&self) -> Result<ComPtr<::rt::gen::windows::foundation::collections::IMap<HString, HString>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Properties)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
RT_CLASS!{class WebTokenRequest: IWebTokenRequest}
impl RtActivatable<IWebTokenRequestFactory> for WebTokenRequest {}
impl WebTokenRequest {
    #[inline] pub fn create(provider: &super::super::super::credentials::WebAccountProvider, scope: &HStringArg, clientId: &HStringArg) -> Result<ComPtr<WebTokenRequest>> { unsafe {
        <Self as RtActivatable<IWebTokenRequestFactory>>::get_activation_factory().create(provider, scope, clientId)
    }}
    #[inline] pub fn create_with_prompt_type(provider: &super::super::super::credentials::WebAccountProvider, scope: &HStringArg, clientId: &HStringArg, promptType: WebTokenRequestPromptType) -> Result<ComPtr<WebTokenRequest>> { unsafe {
        <Self as RtActivatable<IWebTokenRequestFactory>>::get_activation_factory().create_with_prompt_type(provider, scope, clientId, promptType)
    }}
    #[inline] pub fn create_with_provider(provider: &super::super::super::credentials::WebAccountProvider) -> Result<ComPtr<WebTokenRequest>> { unsafe {
        <Self as RtActivatable<IWebTokenRequestFactory>>::get_activation_factory().create_with_provider(provider)
    }}
    #[inline] pub fn create_with_scope(provider: &super::super::super::credentials::WebAccountProvider, scope: &HStringArg) -> Result<ComPtr<WebTokenRequest>> { unsafe {
        <Self as RtActivatable<IWebTokenRequestFactory>>::get_activation_factory().create_with_scope(provider, scope)
    }}
}
DEFINE_CLSID!(WebTokenRequest(&[87,105,110,100,111,119,115,46,83,101,99,117,114,105,116,121,46,65,117,116,104,101,110,116,105,99,97,116,105,111,110,46,87,101,98,46,67,111,114,101,46,87,101,98,84,111,107,101,110,82,101,113,117,101,115,116,0]) [CLSID_WebTokenRequest]);
DEFINE_IID!(IID_IWebTokenRequest2, 3607150713, 12488, 17303, 150, 84, 150, 28, 59, 232, 184, 85);
RT_INTERFACE!{interface IWebTokenRequest2(IWebTokenRequest2Vtbl): IInspectable(IInspectableVtbl) [IID_IWebTokenRequest2] {
    fn get_AppProperties(&self, out: *mut *mut ::rt::gen::windows::foundation::collections::IMap<HString, HString>) -> HRESULT
}}
impl IWebTokenRequest2 {
    #[inline] pub unsafe fn get_app_properties(&self) -> Result<ComPtr<::rt::gen::windows::foundation::collections::IMap<HString, HString>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_AppProperties)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
DEFINE_IID!(IID_IWebTokenRequest3, 1517640529, 15281, 16805, 166, 61, 144, 188, 50, 199, 219, 154);
RT_INTERFACE!{interface IWebTokenRequest3(IWebTokenRequest3Vtbl): IInspectable(IInspectableVtbl) [IID_IWebTokenRequest3] {
    fn get_CorrelationId(&self, out: *mut HSTRING) -> HRESULT,
    fn put_CorrelationId(&self, value: HSTRING) -> HRESULT
}}
impl IWebTokenRequest3 {
    #[inline] pub unsafe fn get_correlation_id(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_CorrelationId)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_correlation_id(&self, value: &HStringArg) -> Result<()> {
        let hr = ((*self.lpVtbl).put_CorrelationId)(self as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
}
DEFINE_IID!(IID_IWebTokenRequestFactory, 1827804188, 4080, 19559, 184, 79, 153, 221, 190, 74, 114, 201);
RT_INTERFACE!{static interface IWebTokenRequestFactory(IWebTokenRequestFactoryVtbl): IInspectable(IInspectableVtbl) [IID_IWebTokenRequestFactory] {
    fn Create(&self, provider: *mut super::super::super::credentials::WebAccountProvider, scope: HSTRING, clientId: HSTRING, out: *mut *mut WebTokenRequest) -> HRESULT,
    fn CreateWithPromptType(&self, provider: *mut super::super::super::credentials::WebAccountProvider, scope: HSTRING, clientId: HSTRING, promptType: WebTokenRequestPromptType, out: *mut *mut WebTokenRequest) -> HRESULT,
    fn CreateWithProvider(&self, provider: *mut super::super::super::credentials::WebAccountProvider, out: *mut *mut WebTokenRequest) -> HRESULT,
    fn CreateWithScope(&self, provider: *mut super::super::super::credentials::WebAccountProvider, scope: HSTRING, out: *mut *mut WebTokenRequest) -> HRESULT
}}
impl IWebTokenRequestFactory {
    #[inline] pub unsafe fn create(&self, provider: &super::super::super::credentials::WebAccountProvider, scope: &HStringArg, clientId: &HStringArg) -> Result<ComPtr<WebTokenRequest>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).Create)(self as *const _ as *mut _, provider as *const _ as *mut _, scope.get(), clientId.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn create_with_prompt_type(&self, provider: &super::super::super::credentials::WebAccountProvider, scope: &HStringArg, clientId: &HStringArg, promptType: WebTokenRequestPromptType) -> Result<ComPtr<WebTokenRequest>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).CreateWithPromptType)(self as *const _ as *mut _, provider as *const _ as *mut _, scope.get(), clientId.get(), promptType, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn create_with_provider(&self, provider: &super::super::super::credentials::WebAccountProvider) -> Result<ComPtr<WebTokenRequest>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).CreateWithProvider)(self as *const _ as *mut _, provider as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn create_with_scope(&self, provider: &super::super::super::credentials::WebAccountProvider, scope: &HStringArg) -> Result<ComPtr<WebTokenRequest>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).CreateWithScope)(self as *const _ as *mut _, provider as *const _ as *mut _, scope.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
RT_ENUM! { enum WebTokenRequestPromptType: i32 {
    Default (WebTokenRequestPromptType_Default) = 0, ForceAuthentication (WebTokenRequestPromptType_ForceAuthentication) = 1,
}}
DEFINE_IID!(IID_IWebTokenRequestResult, 3240788741, 53752, 17539, 141, 84, 56, 254, 41, 39, 132, 255);
RT_INTERFACE!{interface IWebTokenRequestResult(IWebTokenRequestResultVtbl): IInspectable(IInspectableVtbl) [IID_IWebTokenRequestResult] {
    fn get_ResponseData(&self, out: *mut *mut ::rt::gen::windows::foundation::collections::IVectorView<WebTokenResponse>) -> HRESULT,
    fn get_ResponseStatus(&self, out: *mut WebTokenRequestStatus) -> HRESULT,
    fn get_ResponseError(&self, out: *mut *mut WebProviderError) -> HRESULT,
    fn InvalidateCacheAsync(&self, out: *mut *mut ::rt::gen::windows::foundation::IAsyncAction) -> HRESULT
}}
impl IWebTokenRequestResult {
    #[inline] pub unsafe fn get_response_data(&self) -> Result<ComPtr<::rt::gen::windows::foundation::collections::IVectorView<WebTokenResponse>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_ResponseData)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_response_status(&self) -> Result<WebTokenRequestStatus> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_ResponseStatus)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_response_error(&self) -> Result<ComPtr<WebProviderError>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_ResponseError)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn invalidate_cache_async(&self) -> Result<ComPtr<::rt::gen::windows::foundation::IAsyncAction>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).InvalidateCacheAsync)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
RT_CLASS!{class WebTokenRequestResult: IWebTokenRequestResult}
RT_ENUM! { enum WebTokenRequestStatus: i32 {
    Success (WebTokenRequestStatus_Success) = 0, UserCancel (WebTokenRequestStatus_UserCancel) = 1, AccountSwitch (WebTokenRequestStatus_AccountSwitch) = 2, UserInteractionRequired (WebTokenRequestStatus_UserInteractionRequired) = 3, AccountProviderNotAvailable (WebTokenRequestStatus_AccountProviderNotAvailable) = 4, ProviderError (WebTokenRequestStatus_ProviderError) = 5,
}}
DEFINE_IID!(IID_IWebTokenResponse, 1739048394, 33782, 17606, 163, 177, 14, 182, 158, 65, 250, 138);
RT_INTERFACE!{interface IWebTokenResponse(IWebTokenResponseVtbl): IInspectable(IInspectableVtbl) [IID_IWebTokenResponse] {
    fn get_Token(&self, out: *mut HSTRING) -> HRESULT,
    fn get_ProviderError(&self, out: *mut *mut WebProviderError) -> HRESULT,
    fn get_WebAccount(&self, out: *mut *mut super::super::super::credentials::WebAccount) -> HRESULT,
    fn get_Properties(&self, out: *mut *mut ::rt::gen::windows::foundation::collections::IMap<HString, HString>) -> HRESULT
}}
impl IWebTokenResponse {
    #[inline] pub unsafe fn get_token(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Token)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_provider_error(&self) -> Result<ComPtr<WebProviderError>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_ProviderError)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_web_account(&self) -> Result<ComPtr<super::super::super::credentials::WebAccount>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_WebAccount)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_properties(&self) -> Result<ComPtr<::rt::gen::windows::foundation::collections::IMap<HString, HString>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Properties)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
RT_CLASS!{class WebTokenResponse: IWebTokenResponse}
impl RtActivatable<IWebTokenResponseFactory> for WebTokenResponse {}
impl RtActivatable<IActivationFactory> for WebTokenResponse {}
impl WebTokenResponse {
    #[inline] pub fn create_with_token(token: &HStringArg) -> Result<ComPtr<WebTokenResponse>> { unsafe {
        <Self as RtActivatable<IWebTokenResponseFactory>>::get_activation_factory().create_with_token(token)
    }}
    #[inline] pub fn create_with_token_and_account(token: &HStringArg, webAccount: &super::super::super::credentials::WebAccount) -> Result<ComPtr<WebTokenResponse>> { unsafe {
        <Self as RtActivatable<IWebTokenResponseFactory>>::get_activation_factory().create_with_token_and_account(token, webAccount)
    }}
    #[inline] pub fn create_with_token_account_and_error(token: &HStringArg, webAccount: &super::super::super::credentials::WebAccount, error: &WebProviderError) -> Result<ComPtr<WebTokenResponse>> { unsafe {
        <Self as RtActivatable<IWebTokenResponseFactory>>::get_activation_factory().create_with_token_account_and_error(token, webAccount, error)
    }}
}
DEFINE_CLSID!(WebTokenResponse(&[87,105,110,100,111,119,115,46,83,101,99,117,114,105,116,121,46,65,117,116,104,101,110,116,105,99,97,116,105,111,110,46,87,101,98,46,67,111,114,101,46,87,101,98,84,111,107,101,110,82,101,115,112,111,110,115,101,0]) [CLSID_WebTokenResponse]);
DEFINE_IID!(IID_IWebTokenResponseFactory, 2875979768, 21584, 20214, 151, 247, 5, 43, 4, 49, 192, 240);
RT_INTERFACE!{static interface IWebTokenResponseFactory(IWebTokenResponseFactoryVtbl): IInspectable(IInspectableVtbl) [IID_IWebTokenResponseFactory] {
    fn CreateWithToken(&self, token: HSTRING, out: *mut *mut WebTokenResponse) -> HRESULT,
    fn CreateWithTokenAndAccount(&self, token: HSTRING, webAccount: *mut super::super::super::credentials::WebAccount, out: *mut *mut WebTokenResponse) -> HRESULT,
    fn CreateWithTokenAccountAndError(&self, token: HSTRING, webAccount: *mut super::super::super::credentials::WebAccount, error: *mut WebProviderError, out: *mut *mut WebTokenResponse) -> HRESULT
}}
impl IWebTokenResponseFactory {
    #[inline] pub unsafe fn create_with_token(&self, token: &HStringArg) -> Result<ComPtr<WebTokenResponse>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).CreateWithToken)(self as *const _ as *mut _, token.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn create_with_token_and_account(&self, token: &HStringArg, webAccount: &super::super::super::credentials::WebAccount) -> Result<ComPtr<WebTokenResponse>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).CreateWithTokenAndAccount)(self as *const _ as *mut _, token.get(), webAccount as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn create_with_token_account_and_error(&self, token: &HStringArg, webAccount: &super::super::super::credentials::WebAccount, error: &WebProviderError) -> Result<ComPtr<WebTokenResponse>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).CreateWithTokenAccountAndError)(self as *const _ as *mut _, token.get(), webAccount as *const _ as *mut _, error as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
} // Windows.Security.Authentication.Web.Core
} // Windows.Security.Authentication.Web
} // Windows.Security.Authentication
pub mod cryptography { // Windows.Security.Cryptography
use ::prelude::*;
RT_ENUM! { enum BinaryStringEncoding: i32 {
    Utf8 (BinaryStringEncoding_Utf8) = 0, Utf16LE (BinaryStringEncoding_Utf16LE) = 1, Utf16BE (BinaryStringEncoding_Utf16BE) = 2,
}}
RT_CLASS!{static class CryptographicBuffer}
impl RtActivatable<ICryptographicBufferStatics> for CryptographicBuffer {}
impl CryptographicBuffer {
    #[cfg(feature="windows-storage")] #[inline] pub fn compare(object1: &super::super::storage::streams::IBuffer, object2: &super::super::storage::streams::IBuffer) -> Result<bool> { unsafe {
        <Self as RtActivatable<ICryptographicBufferStatics>>::get_activation_factory().compare(object1, object2)
    }}
    #[cfg(feature="windows-storage")] #[inline] pub fn generate_random(length: u32) -> Result<ComPtr<super::super::storage::streams::IBuffer>> { unsafe {
        <Self as RtActivatable<ICryptographicBufferStatics>>::get_activation_factory().generate_random(length)
    }}
    #[inline] pub fn generate_random_number() -> Result<u32> { unsafe {
        <Self as RtActivatable<ICryptographicBufferStatics>>::get_activation_factory().generate_random_number()
    }}
    #[cfg(feature="windows-storage")] #[inline] pub fn create_from_byte_array(value: &[u8]) -> Result<ComPtr<super::super::storage::streams::IBuffer>> { unsafe {
        <Self as RtActivatable<ICryptographicBufferStatics>>::get_activation_factory().create_from_byte_array(value)
    }}
    #[cfg(feature="windows-storage")] #[inline] pub fn copy_to_byte_array(buffer: &super::super::storage::streams::IBuffer) -> Result<ComArray<u8>> { unsafe {
        <Self as RtActivatable<ICryptographicBufferStatics>>::get_activation_factory().copy_to_byte_array(buffer)
    }}
    #[cfg(feature="windows-storage")] #[inline] pub fn decode_from_hex_string(value: &HStringArg) -> Result<ComPtr<super::super::storage::streams::IBuffer>> { unsafe {
        <Self as RtActivatable<ICryptographicBufferStatics>>::get_activation_factory().decode_from_hex_string(value)
    }}
    #[cfg(feature="windows-storage")] #[inline] pub fn encode_to_hex_string(buffer: &super::super::storage::streams::IBuffer) -> Result<HString> { unsafe {
        <Self as RtActivatable<ICryptographicBufferStatics>>::get_activation_factory().encode_to_hex_string(buffer)
    }}
    #[cfg(feature="windows-storage")] #[inline] pub fn decode_from_base64_string(value: &HStringArg) -> Result<ComPtr<super::super::storage::streams::IBuffer>> { unsafe {
        <Self as RtActivatable<ICryptographicBufferStatics>>::get_activation_factory().decode_from_base64_string(value)
    }}
    #[cfg(feature="windows-storage")] #[inline] pub fn encode_to_base64_string(buffer: &super::super::storage::streams::IBuffer) -> Result<HString> { unsafe {
        <Self as RtActivatable<ICryptographicBufferStatics>>::get_activation_factory().encode_to_base64_string(buffer)
    }}
    #[cfg(feature="windows-storage")] #[inline] pub fn convert_string_to_binary(value: &HStringArg, encoding: BinaryStringEncoding) -> Result<ComPtr<super::super::storage::streams::IBuffer>> { unsafe {
        <Self as RtActivatable<ICryptographicBufferStatics>>::get_activation_factory().convert_string_to_binary(value, encoding)
    }}
    #[cfg(feature="windows-storage")] #[inline] pub fn convert_binary_to_string(encoding: BinaryStringEncoding, buffer: &super::super::storage::streams::IBuffer) -> Result<HString> { unsafe {
        <Self as RtActivatable<ICryptographicBufferStatics>>::get_activation_factory().convert_binary_to_string(encoding, buffer)
    }}
}
DEFINE_CLSID!(CryptographicBuffer(&[87,105,110,100,111,119,115,46,83,101,99,117,114,105,116,121,46,67,114,121,112,116,111,103,114,97,112,104,121,46,67,114,121,112,116,111,103,114,97,112,104,105,99,66,117,102,102,101,114,0]) [CLSID_CryptographicBuffer]);
DEFINE_IID!(IID_ICryptographicBufferStatics, 839613986, 15536, 19679, 134, 99, 29, 40, 145, 0, 101, 235);
RT_INTERFACE!{static interface ICryptographicBufferStatics(ICryptographicBufferStaticsVtbl): IInspectable(IInspectableVtbl) [IID_ICryptographicBufferStatics] {
    #[cfg(feature="windows-storage")] fn Compare(&self, object1: *mut super::super::storage::streams::IBuffer, object2: *mut super::super::storage::streams::IBuffer, out: *mut bool) -> HRESULT,
    #[cfg(feature="windows-storage")] fn GenerateRandom(&self, length: u32, out: *mut *mut super::super::storage::streams::IBuffer) -> HRESULT,
    fn GenerateRandomNumber(&self, out: *mut u32) -> HRESULT,
    #[cfg(feature="windows-storage")] fn CreateFromByteArray(&self, valueSize: u32, value: *mut u8, out: *mut *mut super::super::storage::streams::IBuffer) -> HRESULT,
    #[cfg(feature="windows-storage")] fn CopyToByteArray(&self, buffer: *mut super::super::storage::streams::IBuffer, valueSize: *mut u32, value: *mut *mut u8) -> HRESULT,
    #[cfg(feature="windows-storage")] fn DecodeFromHexString(&self, value: HSTRING, out: *mut *mut super::super::storage::streams::IBuffer) -> HRESULT,
    #[cfg(feature="windows-storage")] fn EncodeToHexString(&self, buffer: *mut super::super::storage::streams::IBuffer, out: *mut HSTRING) -> HRESULT,
    #[cfg(feature="windows-storage")] fn DecodeFromBase64String(&self, value: HSTRING, out: *mut *mut super::super::storage::streams::IBuffer) -> HRESULT,
    #[cfg(feature="windows-storage")] fn EncodeToBase64String(&self, buffer: *mut super::super::storage::streams::IBuffer, out: *mut HSTRING) -> HRESULT,
    #[cfg(feature="windows-storage")] fn ConvertStringToBinary(&self, value: HSTRING, encoding: BinaryStringEncoding, out: *mut *mut super::super::storage::streams::IBuffer) -> HRESULT,
    #[cfg(feature="windows-storage")] fn ConvertBinaryToString(&self, encoding: BinaryStringEncoding, buffer: *mut super::super::storage::streams::IBuffer, out: *mut HSTRING) -> HRESULT
}}
impl ICryptographicBufferStatics {
    #[cfg(feature="windows-storage")] #[inline] pub unsafe fn compare(&self, object1: &super::super::storage::streams::IBuffer, object2: &super::super::storage::streams::IBuffer) -> Result<bool> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).Compare)(self as *const _ as *mut _, object1 as *const _ as *mut _, object2 as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[cfg(feature="windows-storage")] #[inline] pub unsafe fn generate_random(&self, length: u32) -> Result<ComPtr<super::super::storage::streams::IBuffer>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GenerateRandom)(self as *const _ as *mut _, length, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn generate_random_number(&self) -> Result<u32> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).GenerateRandomNumber)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[cfg(feature="windows-storage")] #[inline] pub unsafe fn create_from_byte_array(&self, value: &[u8]) -> Result<ComPtr<super::super::storage::streams::IBuffer>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).CreateFromByteArray)(self as *const _ as *mut _, value.len() as u32, value.as_ptr() as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[cfg(feature="windows-storage")] #[inline] pub unsafe fn copy_to_byte_array(&self, buffer: &super::super::storage::streams::IBuffer) -> Result<ComArray<u8>> {
        let mut valueSize = 0; let mut value = null_mut();
        let hr = ((*self.lpVtbl).CopyToByteArray)(self as *const _ as *mut _, buffer as *const _ as *mut _, &mut valueSize, &mut value);
        if hr == S_OK { Ok(ComArray::from_raw(valueSize, value)) } else { err(hr) }
    }
    #[cfg(feature="windows-storage")] #[inline] pub unsafe fn decode_from_hex_string(&self, value: &HStringArg) -> Result<ComPtr<super::super::storage::streams::IBuffer>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).DecodeFromHexString)(self as *const _ as *mut _, value.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[cfg(feature="windows-storage")] #[inline] pub unsafe fn encode_to_hex_string(&self, buffer: &super::super::storage::streams::IBuffer) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).EncodeToHexString)(self as *const _ as *mut _, buffer as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[cfg(feature="windows-storage")] #[inline] pub unsafe fn decode_from_base64_string(&self, value: &HStringArg) -> Result<ComPtr<super::super::storage::streams::IBuffer>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).DecodeFromBase64String)(self as *const _ as *mut _, value.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[cfg(feature="windows-storage")] #[inline] pub unsafe fn encode_to_base64_string(&self, buffer: &super::super::storage::streams::IBuffer) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).EncodeToBase64String)(self as *const _ as *mut _, buffer as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[cfg(feature="windows-storage")] #[inline] pub unsafe fn convert_string_to_binary(&self, value: &HStringArg, encoding: BinaryStringEncoding) -> Result<ComPtr<super::super::storage::streams::IBuffer>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).ConvertStringToBinary)(self as *const _ as *mut _, value.get(), encoding, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[cfg(feature="windows-storage")] #[inline] pub unsafe fn convert_binary_to_string(&self, encoding: BinaryStringEncoding, buffer: &super::super::storage::streams::IBuffer) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).ConvertBinaryToString)(self as *const _ as *mut _, encoding, buffer as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
}
pub mod certificates { // Windows.Security.Cryptography.Certificates
use ::prelude::*;
DEFINE_IID!(IID_ICertificate, 859796492, 1240, 17331, 178, 120, 140, 95, 204, 155, 229, 160);
RT_INTERFACE!{interface ICertificate(ICertificateVtbl): IInspectable(IInspectableVtbl) [IID_ICertificate] {
    fn BuildChainAsync(&self, certificates: *mut ::rt::gen::windows::foundation::collections::IIterable<Certificate>, out: *mut *mut ::rt::gen::windows::foundation::IAsyncOperation<CertificateChain>) -> HRESULT,
    fn BuildChainWithParametersAsync(&self, certificates: *mut ::rt::gen::windows::foundation::collections::IIterable<Certificate>, parameters: *mut ChainBuildingParameters, out: *mut *mut ::rt::gen::windows::foundation::IAsyncOperation<CertificateChain>) -> HRESULT,
    fn get_SerialNumber(&self, outSize: *mut u32, out: *mut *mut u8) -> HRESULT,
    fn GetHashValue(&self, outSize: *mut u32, out: *mut *mut u8) -> HRESULT,
    fn GetHashValueWithAlgorithm(&self, hashAlgorithmName: HSTRING, outSize: *mut u32, out: *mut *mut u8) -> HRESULT,
    #[cfg(not(feature="windows-storage"))] fn __Dummy5(&self) -> (),
    #[cfg(feature="windows-storage")] fn GetCertificateBlob(&self, out: *mut *mut ::rt::gen::windows::storage::streams::IBuffer) -> HRESULT,
    fn get_Subject(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Issuer(&self, out: *mut HSTRING) -> HRESULT,
    fn get_HasPrivateKey(&self, out: *mut bool) -> HRESULT,
    fn get_IsStronglyProtected(&self, out: *mut bool) -> HRESULT,
    fn get_ValidFrom(&self, out: *mut ::rt::gen::windows::foundation::DateTime) -> HRESULT,
    fn get_ValidTo(&self, out: *mut ::rt::gen::windows::foundation::DateTime) -> HRESULT,
    fn get_EnhancedKeyUsages(&self, out: *mut *mut ::rt::gen::windows::foundation::collections::IVectorView<HString>) -> HRESULT,
    fn put_FriendlyName(&self, value: HSTRING) -> HRESULT,
    fn get_FriendlyName(&self, out: *mut HSTRING) -> HRESULT
}}
impl ICertificate {
    #[inline] pub unsafe fn build_chain_async(&self, certificates: &::rt::gen::windows::foundation::collections::IIterable<Certificate>) -> Result<ComPtr<::rt::gen::windows::foundation::IAsyncOperation<CertificateChain>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).BuildChainAsync)(self as *const _ as *mut _, certificates as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn build_chain_with_parameters_async(&self, certificates: &::rt::gen::windows::foundation::collections::IIterable<Certificate>, parameters: &ChainBuildingParameters) -> Result<ComPtr<::rt::gen::windows::foundation::IAsyncOperation<CertificateChain>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).BuildChainWithParametersAsync)(self as *const _ as *mut _, certificates as *const _ as *mut _, parameters as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_serial_number(&self) -> Result<ComArray<u8>> {
        let mut outSize = 0; let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_SerialNumber)(self as *const _ as *mut _, &mut outSize, &mut out);
        if hr == S_OK { Ok(ComArray::from_raw(outSize, out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_hash_value(&self) -> Result<ComArray<u8>> {
        let mut outSize = 0; let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetHashValue)(self as *const _ as *mut _, &mut outSize, &mut out);
        if hr == S_OK { Ok(ComArray::from_raw(outSize, out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_hash_value_with_algorithm(&self, hashAlgorithmName: &HStringArg) -> Result<ComArray<u8>> {
        let mut outSize = 0; let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetHashValueWithAlgorithm)(self as *const _ as *mut _, hashAlgorithmName.get(), &mut outSize, &mut out);
        if hr == S_OK { Ok(ComArray::from_raw(outSize, out)) } else { err(hr) }
    }
    #[cfg(feature="windows-storage")] #[inline] pub unsafe fn get_certificate_blob(&self) -> Result<ComPtr<::rt::gen::windows::storage::streams::IBuffer>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetCertificateBlob)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_subject(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Subject)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_issuer(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Issuer)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_has_private_key(&self) -> Result<bool> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_HasPrivateKey)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_is_strongly_protected(&self) -> Result<bool> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_IsStronglyProtected)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_valid_from(&self) -> Result<::rt::gen::windows::foundation::DateTime> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_ValidFrom)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_valid_to(&self) -> Result<::rt::gen::windows::foundation::DateTime> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_ValidTo)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_enhanced_key_usages(&self) -> Result<ComPtr<::rt::gen::windows::foundation::collections::IVectorView<HString>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_EnhancedKeyUsages)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_friendly_name(&self, value: &HStringArg) -> Result<()> {
        let hr = ((*self.lpVtbl).put_FriendlyName)(self as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_friendly_name(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_FriendlyName)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
}
RT_CLASS!{class Certificate: ICertificate}
impl RtActivatable<ICertificateFactory> for Certificate {}
impl Certificate {
    #[cfg(feature="windows-storage")] #[inline] pub fn create_certificate(certBlob: &::rt::gen::windows::storage::streams::IBuffer) -> Result<ComPtr<Certificate>> { unsafe {
        <Self as RtActivatable<ICertificateFactory>>::get_activation_factory().create_certificate(certBlob)
    }}
}
DEFINE_CLSID!(Certificate(&[87,105,110,100,111,119,115,46,83,101,99,117,114,105,116,121,46,67,114,121,112,116,111,103,114,97,112,104,121,46,67,101,114,116,105,102,105,99,97,116,101,115,46,67,101,114,116,105,102,105,99,97,116,101,0]) [CLSID_Certificate]);
DEFINE_IID!(IID_ICertificate2, 397948748, 35365, 19862, 164, 146, 143, 194, 154, 196, 253, 166);
RT_INTERFACE!{interface ICertificate2(ICertificate2Vtbl): IInspectable(IInspectableVtbl) [IID_ICertificate2] {
    fn get_IsSecurityDeviceBound(&self, out: *mut bool) -> HRESULT,
    fn get_KeyUsages(&self, out: *mut *mut CertificateKeyUsages) -> HRESULT,
    fn get_KeyAlgorithmName(&self, out: *mut HSTRING) -> HRESULT,
    fn get_SignatureAlgorithmName(&self, out: *mut HSTRING) -> HRESULT,
    fn get_SignatureHashAlgorithmName(&self, out: *mut HSTRING) -> HRESULT,
    fn get_SubjectAlternativeName(&self, out: *mut *mut SubjectAlternativeNameInfo) -> HRESULT
}}
impl ICertificate2 {
    #[inline] pub unsafe fn get_is_security_device_bound(&self) -> Result<bool> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_IsSecurityDeviceBound)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_key_usages(&self) -> Result<ComPtr<CertificateKeyUsages>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_KeyUsages)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_key_algorithm_name(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_KeyAlgorithmName)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_signature_algorithm_name(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_SignatureAlgorithmName)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_signature_hash_algorithm_name(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_SignatureHashAlgorithmName)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_subject_alternative_name(&self) -> Result<ComPtr<SubjectAlternativeNameInfo>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_SubjectAlternativeName)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
DEFINE_IID!(IID_ICertificate3, 3193022822, 44639, 18002, 172, 231, 198, 215, 231, 114, 76, 243);
RT_INTERFACE!{interface ICertificate3(ICertificate3Vtbl): IInspectable(IInspectableVtbl) [IID_ICertificate3] {
    fn get_IsPerUser(&self, out: *mut bool) -> HRESULT,
    fn get_StoreName(&self, out: *mut HSTRING) -> HRESULT,
    fn get_KeyStorageProviderName(&self, out: *mut HSTRING) -> HRESULT
}}
impl ICertificate3 {
    #[inline] pub unsafe fn get_is_per_user(&self) -> Result<bool> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_IsPerUser)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_store_name(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_StoreName)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_key_storage_provider_name(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_KeyStorageProviderName)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
}
DEFINE_IID!(IID_ICertificateChain, 549409669, 13969, 17665, 166, 44, 253, 151, 39, 139, 49, 238);
RT_INTERFACE!{interface ICertificateChain(ICertificateChainVtbl): IInspectable(IInspectableVtbl) [IID_ICertificateChain] {
    fn Validate(&self, out: *mut ChainValidationResult) -> HRESULT,
    fn ValidateWithParameters(&self, parameter: *mut ChainValidationParameters, out: *mut ChainValidationResult) -> HRESULT,
    fn GetCertificates(&self, includeRoot: bool, out: *mut *mut ::rt::gen::windows::foundation::collections::IVectorView<Certificate>) -> HRESULT
}}
impl ICertificateChain {
    #[inline] pub unsafe fn validate(&self) -> Result<ChainValidationResult> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).Validate)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn validate_with_parameters(&self, parameter: &ChainValidationParameters) -> Result<ChainValidationResult> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).ValidateWithParameters)(self as *const _ as *mut _, parameter as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_certificates(&self, includeRoot: bool) -> Result<ComPtr<::rt::gen::windows::foundation::collections::IVectorView<Certificate>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetCertificates)(self as *const _ as *mut _, includeRoot, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
RT_CLASS!{class CertificateChain: ICertificateChain}
RT_ENUM! { enum CertificateChainPolicy: i32 {
    Base (CertificateChainPolicy_Base) = 0, Ssl (CertificateChainPolicy_Ssl) = 1, NTAuthentication (CertificateChainPolicy_NTAuthentication) = 2, MicrosoftRoot (CertificateChainPolicy_MicrosoftRoot) = 3,
}}
RT_CLASS!{static class CertificateEnrollmentManager}
impl RtActivatable<ICertificateEnrollmentManagerStatics> for CertificateEnrollmentManager {}
impl RtActivatable<ICertificateEnrollmentManagerStatics2> for CertificateEnrollmentManager {}
impl RtActivatable<ICertificateEnrollmentManagerStatics3> for CertificateEnrollmentManager {}
impl CertificateEnrollmentManager {
    #[inline] pub fn create_request_async(request: &CertificateRequestProperties) -> Result<ComPtr<::rt::gen::windows::foundation::IAsyncOperation<HString>>> { unsafe {
        <Self as RtActivatable<ICertificateEnrollmentManagerStatics>>::get_activation_factory().create_request_async(request)
    }}
    #[inline] pub fn install_certificate_async(certificate: &HStringArg, installOption: InstallOptions) -> Result<ComPtr<::rt::gen::windows::foundation::IAsyncAction>> { unsafe {
        <Self as RtActivatable<ICertificateEnrollmentManagerStatics>>::get_activation_factory().install_certificate_async(certificate, installOption)
    }}
    #[inline] pub fn import_pfx_data_async(pfxData: &HStringArg, password: &HStringArg, exportable: ExportOption, keyProtectionLevel: KeyProtectionLevel, installOption: InstallOptions, friendlyName: &HStringArg) -> Result<ComPtr<::rt::gen::windows::foundation::IAsyncAction>> { unsafe {
        <Self as RtActivatable<ICertificateEnrollmentManagerStatics>>::get_activation_factory().import_pfx_data_async(pfxData, password, exportable, keyProtectionLevel, installOption, friendlyName)
    }}
    #[inline] pub fn get_user_certificate_enrollment_manager() -> Result<ComPtr<UserCertificateEnrollmentManager>> { unsafe {
        <Self as RtActivatable<ICertificateEnrollmentManagerStatics2>>::get_activation_factory().get_user_certificate_enrollment_manager()
    }}
    #[inline] pub fn import_pfx_data_to_ksp_async(pfxData: &HStringArg, password: &HStringArg, exportable: ExportOption, keyProtectionLevel: KeyProtectionLevel, installOption: InstallOptions, friendlyName: &HStringArg, keyStorageProvider: &HStringArg) -> Result<ComPtr<::rt::gen::windows::foundation::IAsyncAction>> { unsafe {
        <Self as RtActivatable<ICertificateEnrollmentManagerStatics2>>::get_activation_factory().import_pfx_data_to_ksp_async(pfxData, password, exportable, keyProtectionLevel, installOption, friendlyName, keyStorageProvider)
    }}
    #[inline] pub fn import_pfx_data_to_ksp_with_parameters_async(pfxData: &HStringArg, password: &HStringArg, pfxImportParameters: &PfxImportParameters) -> Result<ComPtr<::rt::gen::windows::foundation::IAsyncAction>> { unsafe {
        <Self as RtActivatable<ICertificateEnrollmentManagerStatics3>>::get_activation_factory().import_pfx_data_to_ksp_with_parameters_async(pfxData, password, pfxImportParameters)
    }}
}
DEFINE_CLSID!(CertificateEnrollmentManager(&[87,105,110,100,111,119,115,46,83,101,99,117,114,105,116,121,46,67,114,121,112,116,111,103,114,97,112,104,121,46,67,101,114,116,105,102,105,99,97,116,101,115,46,67,101,114,116,105,102,105,99,97,116,101,69,110,114,111,108,108,109,101,110,116,77,97,110,97,103,101,114,0]) [CLSID_CertificateEnrollmentManager]);
DEFINE_IID!(IID_ICertificateEnrollmentManagerStatics, 2286350143, 43398, 18683, 159, 215, 154, 236, 6, 147, 91, 241);
RT_INTERFACE!{static interface ICertificateEnrollmentManagerStatics(ICertificateEnrollmentManagerStaticsVtbl): IInspectable(IInspectableVtbl) [IID_ICertificateEnrollmentManagerStatics] {
    fn CreateRequestAsync(&self, request: *mut CertificateRequestProperties, out: *mut *mut ::rt::gen::windows::foundation::IAsyncOperation<HString>) -> HRESULT,
    fn InstallCertificateAsync(&self, certificate: HSTRING, installOption: InstallOptions, out: *mut *mut ::rt::gen::windows::foundation::IAsyncAction) -> HRESULT,
    fn ImportPfxDataAsync(&self, pfxData: HSTRING, password: HSTRING, exportable: ExportOption, keyProtectionLevel: KeyProtectionLevel, installOption: InstallOptions, friendlyName: HSTRING, out: *mut *mut ::rt::gen::windows::foundation::IAsyncAction) -> HRESULT
}}
impl ICertificateEnrollmentManagerStatics {
    #[inline] pub unsafe fn create_request_async(&self, request: &CertificateRequestProperties) -> Result<ComPtr<::rt::gen::windows::foundation::IAsyncOperation<HString>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).CreateRequestAsync)(self as *const _ as *mut _, request as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn install_certificate_async(&self, certificate: &HStringArg, installOption: InstallOptions) -> Result<ComPtr<::rt::gen::windows::foundation::IAsyncAction>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).InstallCertificateAsync)(self as *const _ as *mut _, certificate.get(), installOption, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn import_pfx_data_async(&self, pfxData: &HStringArg, password: &HStringArg, exportable: ExportOption, keyProtectionLevel: KeyProtectionLevel, installOption: InstallOptions, friendlyName: &HStringArg) -> Result<ComPtr<::rt::gen::windows::foundation::IAsyncAction>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).ImportPfxDataAsync)(self as *const _ as *mut _, pfxData.get(), password.get(), exportable, keyProtectionLevel, installOption, friendlyName.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
DEFINE_IID!(IID_ICertificateEnrollmentManagerStatics2, 3696958515, 25641, 16404, 153, 156, 93, 151, 53, 128, 45, 29);
RT_INTERFACE!{static interface ICertificateEnrollmentManagerStatics2(ICertificateEnrollmentManagerStatics2Vtbl): IInspectable(IInspectableVtbl) [IID_ICertificateEnrollmentManagerStatics2] {
    fn get_UserCertificateEnrollmentManager(&self, out: *mut *mut UserCertificateEnrollmentManager) -> HRESULT,
    fn ImportPfxDataToKspAsync(&self, pfxData: HSTRING, password: HSTRING, exportable: ExportOption, keyProtectionLevel: KeyProtectionLevel, installOption: InstallOptions, friendlyName: HSTRING, keyStorageProvider: HSTRING, out: *mut *mut ::rt::gen::windows::foundation::IAsyncAction) -> HRESULT
}}
impl ICertificateEnrollmentManagerStatics2 {
    #[inline] pub unsafe fn get_user_certificate_enrollment_manager(&self) -> Result<ComPtr<UserCertificateEnrollmentManager>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_UserCertificateEnrollmentManager)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn import_pfx_data_to_ksp_async(&self, pfxData: &HStringArg, password: &HStringArg, exportable: ExportOption, keyProtectionLevel: KeyProtectionLevel, installOption: InstallOptions, friendlyName: &HStringArg, keyStorageProvider: &HStringArg) -> Result<ComPtr<::rt::gen::windows::foundation::IAsyncAction>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).ImportPfxDataToKspAsync)(self as *const _ as *mut _, pfxData.get(), password.get(), exportable, keyProtectionLevel, installOption, friendlyName.get(), keyStorageProvider.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
DEFINE_IID!(IID_ICertificateEnrollmentManagerStatics3, 4260135614, 24956, 16986, 183, 45, 57, 139, 38, 172, 114, 100);
RT_INTERFACE!{static interface ICertificateEnrollmentManagerStatics3(ICertificateEnrollmentManagerStatics3Vtbl): IInspectable(IInspectableVtbl) [IID_ICertificateEnrollmentManagerStatics3] {
    fn ImportPfxDataToKspWithParametersAsync(&self, pfxData: HSTRING, password: HSTRING, pfxImportParameters: *mut PfxImportParameters, out: *mut *mut ::rt::gen::windows::foundation::IAsyncAction) -> HRESULT
}}
impl ICertificateEnrollmentManagerStatics3 {
    #[inline] pub unsafe fn import_pfx_data_to_ksp_with_parameters_async(&self, pfxData: &HStringArg, password: &HStringArg, pfxImportParameters: &PfxImportParameters) -> Result<ComPtr<::rt::gen::windows::foundation::IAsyncAction>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).ImportPfxDataToKspWithParametersAsync)(self as *const _ as *mut _, pfxData.get(), password.get(), pfxImportParameters as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
DEFINE_IID!(IID_ICertificateExtension, 2228160086, 43494, 17741, 142, 69, 46, 167, 196, 188, 213, 59);
RT_INTERFACE!{interface ICertificateExtension(ICertificateExtensionVtbl): IInspectable(IInspectableVtbl) [IID_ICertificateExtension] {
    fn get_ObjectId(&self, out: *mut HSTRING) -> HRESULT,
    fn put_ObjectId(&self, value: HSTRING) -> HRESULT,
    fn get_IsCritical(&self, out: *mut bool) -> HRESULT,
    fn put_IsCritical(&self, value: bool) -> HRESULT,
    fn EncodeValue(&self, value: HSTRING) -> HRESULT,
    fn get_Value(&self, outSize: *mut u32, out: *mut *mut u8) -> HRESULT,
    fn put_Value(&self, valueSize: u32, value: *mut u8) -> HRESULT
}}
impl ICertificateExtension {
    #[inline] pub unsafe fn get_object_id(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_ObjectId)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_object_id(&self, value: &HStringArg) -> Result<()> {
        let hr = ((*self.lpVtbl).put_ObjectId)(self as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_is_critical(&self) -> Result<bool> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_IsCritical)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_is_critical(&self, value: bool) -> Result<()> {
        let hr = ((*self.lpVtbl).put_IsCritical)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn encode_value(&self, value: &HStringArg) -> Result<()> {
        let hr = ((*self.lpVtbl).EncodeValue)(self as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_value(&self) -> Result<ComArray<u8>> {
        let mut outSize = 0; let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Value)(self as *const _ as *mut _, &mut outSize, &mut out);
        if hr == S_OK { Ok(ComArray::from_raw(outSize, out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_value(&self, value: &[u8]) -> Result<()> {
        let hr = ((*self.lpVtbl).put_Value)(self as *const _ as *mut _, value.len() as u32, value.as_ptr() as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
}
RT_CLASS!{class CertificateExtension: ICertificateExtension}
impl RtActivatable<IActivationFactory> for CertificateExtension {}
DEFINE_CLSID!(CertificateExtension(&[87,105,110,100,111,119,115,46,83,101,99,117,114,105,116,121,46,67,114,121,112,116,111,103,114,97,112,104,121,46,67,101,114,116,105,102,105,99,97,116,101,115,46,67,101,114,116,105,102,105,99,97,116,101,69,120,116,101,110,115,105,111,110,0]) [CLSID_CertificateExtension]);
DEFINE_IID!(IID_ICertificateFactory, 397681180, 19375, 17570, 150, 8, 4, 251, 98, 177, 105, 66);
RT_INTERFACE!{static interface ICertificateFactory(ICertificateFactoryVtbl): IInspectable(IInspectableVtbl) [IID_ICertificateFactory] {
    #[cfg(feature="windows-storage")] fn CreateCertificate(&self, certBlob: *mut ::rt::gen::windows::storage::streams::IBuffer, out: *mut *mut Certificate) -> HRESULT
}}
impl ICertificateFactory {
    #[cfg(feature="windows-storage")] #[inline] pub unsafe fn create_certificate(&self, certBlob: &::rt::gen::windows::storage::streams::IBuffer) -> Result<ComPtr<Certificate>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).CreateCertificate)(self as *const _ as *mut _, certBlob as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
DEFINE_IID!(IID_ICertificateKeyUsages, 1791369327, 57807, 18538, 180, 133, 166, 156, 131, 228, 111, 209);
RT_INTERFACE!{interface ICertificateKeyUsages(ICertificateKeyUsagesVtbl): IInspectable(IInspectableVtbl) [IID_ICertificateKeyUsages] {
    fn get_EncipherOnly(&self, out: *mut bool) -> HRESULT,
    fn put_EncipherOnly(&self, value: bool) -> HRESULT,
    fn get_CrlSign(&self, out: *mut bool) -> HRESULT,
    fn put_CrlSign(&self, value: bool) -> HRESULT,
    fn get_KeyCertificateSign(&self, out: *mut bool) -> HRESULT,
    fn put_KeyCertificateSign(&self, value: bool) -> HRESULT,
    fn get_KeyAgreement(&self, out: *mut bool) -> HRESULT,
    fn put_KeyAgreement(&self, value: bool) -> HRESULT,
    fn get_DataEncipherment(&self, out: *mut bool) -> HRESULT,
    fn put_DataEncipherment(&self, value: bool) -> HRESULT,
    fn get_KeyEncipherment(&self, out: *mut bool) -> HRESULT,
    fn put_KeyEncipherment(&self, value: bool) -> HRESULT,
    fn get_NonRepudiation(&self, out: *mut bool) -> HRESULT,
    fn put_NonRepudiation(&self, value: bool) -> HRESULT,
    fn get_DigitalSignature(&self, out: *mut bool) -> HRESULT,
    fn put_DigitalSignature(&self, value: bool) -> HRESULT
}}
impl ICertificateKeyUsages {
    #[inline] pub unsafe fn get_encipher_only(&self) -> Result<bool> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_EncipherOnly)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_encipher_only(&self, value: bool) -> Result<()> {
        let hr = ((*self.lpVtbl).put_EncipherOnly)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_crl_sign(&self) -> Result<bool> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_CrlSign)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_crl_sign(&self, value: bool) -> Result<()> {
        let hr = ((*self.lpVtbl).put_CrlSign)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_key_certificate_sign(&self) -> Result<bool> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_KeyCertificateSign)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_key_certificate_sign(&self, value: bool) -> Result<()> {
        let hr = ((*self.lpVtbl).put_KeyCertificateSign)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_key_agreement(&self) -> Result<bool> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_KeyAgreement)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_key_agreement(&self, value: bool) -> Result<()> {
        let hr = ((*self.lpVtbl).put_KeyAgreement)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_data_encipherment(&self) -> Result<bool> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_DataEncipherment)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_data_encipherment(&self, value: bool) -> Result<()> {
        let hr = ((*self.lpVtbl).put_DataEncipherment)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_key_encipherment(&self) -> Result<bool> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_KeyEncipherment)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_key_encipherment(&self, value: bool) -> Result<()> {
        let hr = ((*self.lpVtbl).put_KeyEncipherment)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_non_repudiation(&self) -> Result<bool> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_NonRepudiation)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_non_repudiation(&self, value: bool) -> Result<()> {
        let hr = ((*self.lpVtbl).put_NonRepudiation)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_digital_signature(&self) -> Result<bool> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_DigitalSignature)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_digital_signature(&self, value: bool) -> Result<()> {
        let hr = ((*self.lpVtbl).put_DigitalSignature)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
}
RT_CLASS!{class CertificateKeyUsages: ICertificateKeyUsages}
impl RtActivatable<IActivationFactory> for CertificateKeyUsages {}
DEFINE_CLSID!(CertificateKeyUsages(&[87,105,110,100,111,119,115,46,83,101,99,117,114,105,116,121,46,67,114,121,112,116,111,103,114,97,112,104,121,46,67,101,114,116,105,102,105,99,97,116,101,115,46,67,101,114,116,105,102,105,99,97,116,101,75,101,121,85,115,97,103,101,115,0]) [CLSID_CertificateKeyUsages]);
DEFINE_IID!(IID_ICertificateQuery, 1527261745, 42792, 18710, 181, 238, 255, 203, 138, 207, 36, 23);
RT_INTERFACE!{interface ICertificateQuery(ICertificateQueryVtbl): IInspectable(IInspectableVtbl) [IID_ICertificateQuery] {
    fn get_EnhancedKeyUsages(&self, out: *mut *mut ::rt::gen::windows::foundation::collections::IVector<HString>) -> HRESULT,
    fn get_IssuerName(&self, out: *mut HSTRING) -> HRESULT,
    fn put_IssuerName(&self, value: HSTRING) -> HRESULT,
    fn get_FriendlyName(&self, out: *mut HSTRING) -> HRESULT,
    fn put_FriendlyName(&self, value: HSTRING) -> HRESULT,
    fn get_Thumbprint(&self, outSize: *mut u32, out: *mut *mut u8) -> HRESULT,
    fn put_Thumbprint(&self, valueSize: u32, value: *mut u8) -> HRESULT,
    fn get_HardwareOnly(&self, out: *mut bool) -> HRESULT,
    fn put_HardwareOnly(&self, value: bool) -> HRESULT
}}
impl ICertificateQuery {
    #[inline] pub unsafe fn get_enhanced_key_usages(&self) -> Result<ComPtr<::rt::gen::windows::foundation::collections::IVector<HString>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_EnhancedKeyUsages)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_issuer_name(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_IssuerName)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_issuer_name(&self, value: &HStringArg) -> Result<()> {
        let hr = ((*self.lpVtbl).put_IssuerName)(self as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_friendly_name(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_FriendlyName)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_friendly_name(&self, value: &HStringArg) -> Result<()> {
        let hr = ((*self.lpVtbl).put_FriendlyName)(self as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_thumbprint(&self) -> Result<ComArray<u8>> {
        let mut outSize = 0; let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Thumbprint)(self as *const _ as *mut _, &mut outSize, &mut out);
        if hr == S_OK { Ok(ComArray::from_raw(outSize, out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_thumbprint(&self, value: &[u8]) -> Result<()> {
        let hr = ((*self.lpVtbl).put_Thumbprint)(self as *const _ as *mut _, value.len() as u32, value.as_ptr() as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_hardware_only(&self) -> Result<bool> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_HardwareOnly)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_hardware_only(&self, value: bool) -> Result<()> {
        let hr = ((*self.lpVtbl).put_HardwareOnly)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
}
RT_CLASS!{class CertificateQuery: ICertificateQuery}
impl RtActivatable<IActivationFactory> for CertificateQuery {}
DEFINE_CLSID!(CertificateQuery(&[87,105,110,100,111,119,115,46,83,101,99,117,114,105,116,121,46,67,114,121,112,116,111,103,114,97,112,104,121,46,67,101,114,116,105,102,105,99,97,116,101,115,46,67,101,114,116,105,102,105,99,97,116,101,81,117,101,114,121,0]) [CLSID_CertificateQuery]);
DEFINE_IID!(IID_ICertificateQuery2, 2472151799, 3033, 20341, 184, 194, 226, 122, 127, 116, 238, 205);
RT_INTERFACE!{interface ICertificateQuery2(ICertificateQuery2Vtbl): IInspectable(IInspectableVtbl) [IID_ICertificateQuery2] {
    fn get_IncludeDuplicates(&self, out: *mut bool) -> HRESULT,
    fn put_IncludeDuplicates(&self, value: bool) -> HRESULT,
    fn get_IncludeExpiredCertificates(&self, out: *mut bool) -> HRESULT,
    fn put_IncludeExpiredCertificates(&self, value: bool) -> HRESULT,
    fn get_StoreName(&self, out: *mut HSTRING) -> HRESULT,
    fn put_StoreName(&self, value: HSTRING) -> HRESULT
}}
impl ICertificateQuery2 {
    #[inline] pub unsafe fn get_include_duplicates(&self) -> Result<bool> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_IncludeDuplicates)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_include_duplicates(&self, value: bool) -> Result<()> {
        let hr = ((*self.lpVtbl).put_IncludeDuplicates)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_include_expired_certificates(&self) -> Result<bool> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_IncludeExpiredCertificates)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_include_expired_certificates(&self, value: bool) -> Result<()> {
        let hr = ((*self.lpVtbl).put_IncludeExpiredCertificates)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_store_name(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_StoreName)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_store_name(&self, value: &HStringArg) -> Result<()> {
        let hr = ((*self.lpVtbl).put_StoreName)(self as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
}
DEFINE_IID!(IID_ICertificateRequestProperties, 1216251126, 38114, 19918, 136, 51, 26, 112, 10, 55, 162, 154);
RT_INTERFACE!{interface ICertificateRequestProperties(ICertificateRequestPropertiesVtbl): IInspectable(IInspectableVtbl) [IID_ICertificateRequestProperties] {
    fn get_Subject(&self, out: *mut HSTRING) -> HRESULT,
    fn put_Subject(&self, value: HSTRING) -> HRESULT,
    fn get_KeyAlgorithmName(&self, out: *mut HSTRING) -> HRESULT,
    fn put_KeyAlgorithmName(&self, value: HSTRING) -> HRESULT,
    fn get_KeySize(&self, out: *mut u32) -> HRESULT,
    fn put_KeySize(&self, value: u32) -> HRESULT,
    fn get_FriendlyName(&self, out: *mut HSTRING) -> HRESULT,
    fn put_FriendlyName(&self, value: HSTRING) -> HRESULT,
    fn get_HashAlgorithmName(&self, out: *mut HSTRING) -> HRESULT,
    fn put_HashAlgorithmName(&self, value: HSTRING) -> HRESULT,
    fn get_Exportable(&self, out: *mut ExportOption) -> HRESULT,
    fn put_Exportable(&self, value: ExportOption) -> HRESULT,
    fn get_KeyUsages(&self, out: *mut EnrollKeyUsages) -> HRESULT,
    fn put_KeyUsages(&self, value: EnrollKeyUsages) -> HRESULT,
    fn get_KeyProtectionLevel(&self, out: *mut KeyProtectionLevel) -> HRESULT,
    fn put_KeyProtectionLevel(&self, value: KeyProtectionLevel) -> HRESULT,
    fn get_KeyStorageProviderName(&self, out: *mut HSTRING) -> HRESULT,
    fn put_KeyStorageProviderName(&self, value: HSTRING) -> HRESULT
}}
impl ICertificateRequestProperties {
    #[inline] pub unsafe fn get_subject(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Subject)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_subject(&self, value: &HStringArg) -> Result<()> {
        let hr = ((*self.lpVtbl).put_Subject)(self as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_key_algorithm_name(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_KeyAlgorithmName)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_key_algorithm_name(&self, value: &HStringArg) -> Result<()> {
        let hr = ((*self.lpVtbl).put_KeyAlgorithmName)(self as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_key_size(&self) -> Result<u32> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_KeySize)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_key_size(&self, value: u32) -> Result<()> {
        let hr = ((*self.lpVtbl).put_KeySize)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_friendly_name(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_FriendlyName)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_friendly_name(&self, value: &HStringArg) -> Result<()> {
        let hr = ((*self.lpVtbl).put_FriendlyName)(self as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_hash_algorithm_name(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_HashAlgorithmName)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_hash_algorithm_name(&self, value: &HStringArg) -> Result<()> {
        let hr = ((*self.lpVtbl).put_HashAlgorithmName)(self as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_exportable(&self) -> Result<ExportOption> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_Exportable)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_exportable(&self, value: ExportOption) -> Result<()> {
        let hr = ((*self.lpVtbl).put_Exportable)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_key_usages(&self) -> Result<EnrollKeyUsages> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_KeyUsages)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_key_usages(&self, value: EnrollKeyUsages) -> Result<()> {
        let hr = ((*self.lpVtbl).put_KeyUsages)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_key_protection_level(&self) -> Result<KeyProtectionLevel> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_KeyProtectionLevel)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_key_protection_level(&self, value: KeyProtectionLevel) -> Result<()> {
        let hr = ((*self.lpVtbl).put_KeyProtectionLevel)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_key_storage_provider_name(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_KeyStorageProviderName)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_key_storage_provider_name(&self, value: &HStringArg) -> Result<()> {
        let hr = ((*self.lpVtbl).put_KeyStorageProviderName)(self as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
}
RT_CLASS!{class CertificateRequestProperties: ICertificateRequestProperties}
impl RtActivatable<IActivationFactory> for CertificateRequestProperties {}
DEFINE_CLSID!(CertificateRequestProperties(&[87,105,110,100,111,119,115,46,83,101,99,117,114,105,116,121,46,67,114,121,112,116,111,103,114,97,112,104,121,46,67,101,114,116,105,102,105,99,97,116,101,115,46,67,101,114,116,105,102,105,99,97,116,101,82,101,113,117,101,115,116,80,114,111,112,101,114,116,105,101,115,0]) [CLSID_CertificateRequestProperties]);
DEFINE_IID!(IID_ICertificateRequestProperties2, 1033947476, 55103, 20467, 160, 166, 6, 119, 192, 173, 160, 91);
RT_INTERFACE!{interface ICertificateRequestProperties2(ICertificateRequestProperties2Vtbl): IInspectable(IInspectableVtbl) [IID_ICertificateRequestProperties2] {
    fn get_SmartcardReaderName(&self, out: *mut HSTRING) -> HRESULT,
    fn put_SmartcardReaderName(&self, value: HSTRING) -> HRESULT,
    fn get_SigningCertificate(&self, out: *mut *mut Certificate) -> HRESULT,
    fn put_SigningCertificate(&self, value: *mut Certificate) -> HRESULT,
    fn get_AttestationCredentialCertificate(&self, out: *mut *mut Certificate) -> HRESULT,
    fn put_AttestationCredentialCertificate(&self, value: *mut Certificate) -> HRESULT
}}
impl ICertificateRequestProperties2 {
    #[inline] pub unsafe fn get_smartcard_reader_name(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_SmartcardReaderName)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_smartcard_reader_name(&self, value: &HStringArg) -> Result<()> {
        let hr = ((*self.lpVtbl).put_SmartcardReaderName)(self as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_signing_certificate(&self) -> Result<ComPtr<Certificate>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_SigningCertificate)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_signing_certificate(&self, value: &Certificate) -> Result<()> {
        let hr = ((*self.lpVtbl).put_SigningCertificate)(self as *const _ as *mut _, value as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_attestation_credential_certificate(&self) -> Result<ComPtr<Certificate>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_AttestationCredentialCertificate)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_attestation_credential_certificate(&self, value: &Certificate) -> Result<()> {
        let hr = ((*self.lpVtbl).put_AttestationCredentialCertificate)(self as *const _ as *mut _, value as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
}
DEFINE_IID!(IID_ICertificateRequestProperties3, 3867670038, 29517, 18097, 157, 76, 110, 223, 219, 252, 132, 91);
RT_INTERFACE!{interface ICertificateRequestProperties3(ICertificateRequestProperties3Vtbl): IInspectable(IInspectableVtbl) [IID_ICertificateRequestProperties3] {
    fn get_CurveName(&self, out: *mut HSTRING) -> HRESULT,
    fn put_CurveName(&self, value: HSTRING) -> HRESULT,
    fn get_CurveParameters(&self, outSize: *mut u32, out: *mut *mut u8) -> HRESULT,
    fn put_CurveParameters(&self, valueSize: u32, value: *mut u8) -> HRESULT,
    fn get_ContainerNamePrefix(&self, out: *mut HSTRING) -> HRESULT,
    fn put_ContainerNamePrefix(&self, value: HSTRING) -> HRESULT,
    fn get_ContainerName(&self, out: *mut HSTRING) -> HRESULT,
    fn put_ContainerName(&self, value: HSTRING) -> HRESULT,
    fn get_UseExistingKey(&self, out: *mut bool) -> HRESULT,
    fn put_UseExistingKey(&self, value: bool) -> HRESULT
}}
impl ICertificateRequestProperties3 {
    #[inline] pub unsafe fn get_curve_name(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_CurveName)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_curve_name(&self, value: &HStringArg) -> Result<()> {
        let hr = ((*self.lpVtbl).put_CurveName)(self as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_curve_parameters(&self) -> Result<ComArray<u8>> {
        let mut outSize = 0; let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_CurveParameters)(self as *const _ as *mut _, &mut outSize, &mut out);
        if hr == S_OK { Ok(ComArray::from_raw(outSize, out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_curve_parameters(&self, value: &[u8]) -> Result<()> {
        let hr = ((*self.lpVtbl).put_CurveParameters)(self as *const _ as *mut _, value.len() as u32, value.as_ptr() as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_container_name_prefix(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_ContainerNamePrefix)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_container_name_prefix(&self, value: &HStringArg) -> Result<()> {
        let hr = ((*self.lpVtbl).put_ContainerNamePrefix)(self as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_container_name(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_ContainerName)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_container_name(&self, value: &HStringArg) -> Result<()> {
        let hr = ((*self.lpVtbl).put_ContainerName)(self as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_use_existing_key(&self) -> Result<bool> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_UseExistingKey)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_use_existing_key(&self, value: bool) -> Result<()> {
        let hr = ((*self.lpVtbl).put_UseExistingKey)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
}
DEFINE_IID!(IID_ICertificateRequestProperties4, 1312987858, 7265, 20458, 184, 254, 19, 95, 177, 156, 220, 228);
RT_INTERFACE!{interface ICertificateRequestProperties4(ICertificateRequestProperties4Vtbl): IInspectable(IInspectableVtbl) [IID_ICertificateRequestProperties4] {
    fn get_SuppressedDefaults(&self, out: *mut *mut ::rt::gen::windows::foundation::collections::IVector<HString>) -> HRESULT,
    fn get_SubjectAlternativeName(&self, out: *mut *mut SubjectAlternativeNameInfo) -> HRESULT,
    fn get_Extensions(&self, out: *mut *mut ::rt::gen::windows::foundation::collections::IVector<CertificateExtension>) -> HRESULT
}}
impl ICertificateRequestProperties4 {
    #[inline] pub unsafe fn get_suppressed_defaults(&self) -> Result<ComPtr<::rt::gen::windows::foundation::collections::IVector<HString>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_SuppressedDefaults)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_subject_alternative_name(&self) -> Result<ComPtr<SubjectAlternativeNameInfo>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_SubjectAlternativeName)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_extensions(&self) -> Result<ComPtr<::rt::gen::windows::foundation::collections::IVector<CertificateExtension>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Extensions)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
DEFINE_IID!(IID_ICertificateStore, 2965370656, 13390, 17201, 175, 20, 167, 247, 167, 235, 201, 58);
RT_INTERFACE!{interface ICertificateStore(ICertificateStoreVtbl): IInspectable(IInspectableVtbl) [IID_ICertificateStore] {
    fn Add(&self, certificate: *mut Certificate) -> HRESULT,
    fn Delete(&self, certificate: *mut Certificate) -> HRESULT
}}
impl ICertificateStore {
    #[inline] pub unsafe fn add(&self, certificate: &Certificate) -> Result<()> {
        let hr = ((*self.lpVtbl).Add)(self as *const _ as *mut _, certificate as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn delete(&self, certificate: &Certificate) -> Result<()> {
        let hr = ((*self.lpVtbl).Delete)(self as *const _ as *mut _, certificate as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
}
RT_CLASS!{class CertificateStore: ICertificateStore}
DEFINE_IID!(IID_ICertificateStore2, 3353775690, 16765, 19738, 186, 189, 21, 104, 126, 84, 153, 116);
RT_INTERFACE!{interface ICertificateStore2(ICertificateStore2Vtbl): IInspectable(IInspectableVtbl) [IID_ICertificateStore2] {
    fn get_Name(&self, out: *mut HSTRING) -> HRESULT
}}
impl ICertificateStore2 {
    #[inline] pub unsafe fn get_name(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Name)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
}
RT_CLASS!{static class CertificateStores}
impl RtActivatable<ICertificateStoresStatics> for CertificateStores {}
impl RtActivatable<ICertificateStoresStatics2> for CertificateStores {}
impl CertificateStores {
    #[inline] pub fn find_all_async() -> Result<ComPtr<::rt::gen::windows::foundation::IAsyncOperation<::rt::gen::windows::foundation::collections::IVectorView<Certificate>>>> { unsafe {
        <Self as RtActivatable<ICertificateStoresStatics>>::get_activation_factory().find_all_async()
    }}
    #[inline] pub fn find_all_with_query_async(query: &CertificateQuery) -> Result<ComPtr<::rt::gen::windows::foundation::IAsyncOperation<::rt::gen::windows::foundation::collections::IVectorView<Certificate>>>> { unsafe {
        <Self as RtActivatable<ICertificateStoresStatics>>::get_activation_factory().find_all_with_query_async(query)
    }}
    #[inline] pub fn get_trusted_root_certification_authorities() -> Result<ComPtr<CertificateStore>> { unsafe {
        <Self as RtActivatable<ICertificateStoresStatics>>::get_activation_factory().get_trusted_root_certification_authorities()
    }}
    #[inline] pub fn get_intermediate_certification_authorities() -> Result<ComPtr<CertificateStore>> { unsafe {
        <Self as RtActivatable<ICertificateStoresStatics>>::get_activation_factory().get_intermediate_certification_authorities()
    }}
    #[inline] pub fn get_store_by_name(storeName: &HStringArg) -> Result<ComPtr<CertificateStore>> { unsafe {
        <Self as RtActivatable<ICertificateStoresStatics>>::get_activation_factory().get_store_by_name(storeName)
    }}
    #[inline] pub fn get_user_store_by_name(storeName: &HStringArg) -> Result<ComPtr<UserCertificateStore>> { unsafe {
        <Self as RtActivatable<ICertificateStoresStatics2>>::get_activation_factory().get_user_store_by_name(storeName)
    }}
}
DEFINE_CLSID!(CertificateStores(&[87,105,110,100,111,119,115,46,83,101,99,117,114,105,116,121,46,67,114,121,112,116,111,103,114,97,112,104,121,46,67,101,114,116,105,102,105,99,97,116,101,115,46,67,101,114,116,105,102,105,99,97,116,101,83,116,111,114,101,115,0]) [CLSID_CertificateStores]);
DEFINE_IID!(IID_ICertificateStoresStatics, 4226598713, 50942, 19943, 153, 207, 116, 195, 229, 150, 224, 50);
RT_INTERFACE!{static interface ICertificateStoresStatics(ICertificateStoresStaticsVtbl): IInspectable(IInspectableVtbl) [IID_ICertificateStoresStatics] {
    fn FindAllAsync(&self, out: *mut *mut ::rt::gen::windows::foundation::IAsyncOperation<::rt::gen::windows::foundation::collections::IVectorView<Certificate>>) -> HRESULT,
    fn FindAllWithQueryAsync(&self, query: *mut CertificateQuery, out: *mut *mut ::rt::gen::windows::foundation::IAsyncOperation<::rt::gen::windows::foundation::collections::IVectorView<Certificate>>) -> HRESULT,
    fn get_TrustedRootCertificationAuthorities(&self, out: *mut *mut CertificateStore) -> HRESULT,
    fn get_IntermediateCertificationAuthorities(&self, out: *mut *mut CertificateStore) -> HRESULT,
    fn GetStoreByName(&self, storeName: HSTRING, out: *mut *mut CertificateStore) -> HRESULT
}}
impl ICertificateStoresStatics {
    #[inline] pub unsafe fn find_all_async(&self) -> Result<ComPtr<::rt::gen::windows::foundation::IAsyncOperation<::rt::gen::windows::foundation::collections::IVectorView<Certificate>>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).FindAllAsync)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn find_all_with_query_async(&self, query: &CertificateQuery) -> Result<ComPtr<::rt::gen::windows::foundation::IAsyncOperation<::rt::gen::windows::foundation::collections::IVectorView<Certificate>>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).FindAllWithQueryAsync)(self as *const _ as *mut _, query as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_trusted_root_certification_authorities(&self) -> Result<ComPtr<CertificateStore>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_TrustedRootCertificationAuthorities)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_intermediate_certification_authorities(&self) -> Result<ComPtr<CertificateStore>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_IntermediateCertificationAuthorities)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_store_by_name(&self, storeName: &HStringArg) -> Result<ComPtr<CertificateStore>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetStoreByName)(self as *const _ as *mut _, storeName.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
DEFINE_IID!(IID_ICertificateStoresStatics2, 4203744121, 41172, 19340, 188, 85, 192, 163, 126, 177, 65, 237);
RT_INTERFACE!{static interface ICertificateStoresStatics2(ICertificateStoresStatics2Vtbl): IInspectable(IInspectableVtbl) [IID_ICertificateStoresStatics2] {
    fn GetUserStoreByName(&self, storeName: HSTRING, out: *mut *mut UserCertificateStore) -> HRESULT
}}
impl ICertificateStoresStatics2 {
    #[inline] pub unsafe fn get_user_store_by_name(&self, storeName: &HStringArg) -> Result<ComPtr<UserCertificateStore>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetUserStoreByName)(self as *const _ as *mut _, storeName.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
DEFINE_IID!(IID_IChainBuildingParameters, 1110157602, 31885, 18359, 181, 155, 177, 39, 3, 115, 58, 195);
RT_INTERFACE!{interface IChainBuildingParameters(IChainBuildingParametersVtbl): IInspectable(IInspectableVtbl) [IID_IChainBuildingParameters] {
    fn get_EnhancedKeyUsages(&self, out: *mut *mut ::rt::gen::windows::foundation::collections::IVector<HString>) -> HRESULT,
    fn get_ValidationTimestamp(&self, out: *mut ::rt::gen::windows::foundation::DateTime) -> HRESULT,
    fn put_ValidationTimestamp(&self, value: ::rt::gen::windows::foundation::DateTime) -> HRESULT,
    fn get_RevocationCheckEnabled(&self, out: *mut bool) -> HRESULT,
    fn put_RevocationCheckEnabled(&self, value: bool) -> HRESULT,
    fn get_NetworkRetrievalEnabled(&self, out: *mut bool) -> HRESULT,
    fn put_NetworkRetrievalEnabled(&self, value: bool) -> HRESULT,
    fn get_AuthorityInformationAccessEnabled(&self, out: *mut bool) -> HRESULT,
    fn put_AuthorityInformationAccessEnabled(&self, value: bool) -> HRESULT,
    fn get_CurrentTimeValidationEnabled(&self, out: *mut bool) -> HRESULT,
    fn put_CurrentTimeValidationEnabled(&self, value: bool) -> HRESULT,
    fn get_ExclusiveTrustRoots(&self, out: *mut *mut ::rt::gen::windows::foundation::collections::IVector<Certificate>) -> HRESULT
}}
impl IChainBuildingParameters {
    #[inline] pub unsafe fn get_enhanced_key_usages(&self) -> Result<ComPtr<::rt::gen::windows::foundation::collections::IVector<HString>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_EnhancedKeyUsages)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_validation_timestamp(&self) -> Result<::rt::gen::windows::foundation::DateTime> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_ValidationTimestamp)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_validation_timestamp(&self, value: ::rt::gen::windows::foundation::DateTime) -> Result<()> {
        let hr = ((*self.lpVtbl).put_ValidationTimestamp)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_revocation_check_enabled(&self) -> Result<bool> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_RevocationCheckEnabled)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_revocation_check_enabled(&self, value: bool) -> Result<()> {
        let hr = ((*self.lpVtbl).put_RevocationCheckEnabled)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_network_retrieval_enabled(&self) -> Result<bool> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_NetworkRetrievalEnabled)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_network_retrieval_enabled(&self, value: bool) -> Result<()> {
        let hr = ((*self.lpVtbl).put_NetworkRetrievalEnabled)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_authority_information_access_enabled(&self) -> Result<bool> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_AuthorityInformationAccessEnabled)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_authority_information_access_enabled(&self, value: bool) -> Result<()> {
        let hr = ((*self.lpVtbl).put_AuthorityInformationAccessEnabled)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_current_time_validation_enabled(&self) -> Result<bool> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_CurrentTimeValidationEnabled)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_current_time_validation_enabled(&self, value: bool) -> Result<()> {
        let hr = ((*self.lpVtbl).put_CurrentTimeValidationEnabled)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_exclusive_trust_roots(&self) -> Result<ComPtr<::rt::gen::windows::foundation::collections::IVector<Certificate>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_ExclusiveTrustRoots)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
RT_CLASS!{class ChainBuildingParameters: IChainBuildingParameters}
impl RtActivatable<IActivationFactory> for ChainBuildingParameters {}
DEFINE_CLSID!(ChainBuildingParameters(&[87,105,110,100,111,119,115,46,83,101,99,117,114,105,116,121,46,67,114,121,112,116,111,103,114,97,112,104,121,46,67,101,114,116,105,102,105,99,97,116,101,115,46,67,104,97,105,110,66,117,105,108,100,105,110,103,80,97,114,97,109,101,116,101,114,115,0]) [CLSID_ChainBuildingParameters]);
DEFINE_IID!(IID_IChainValidationParameters, 3295951690, 32432, 19286, 160, 64, 185, 200, 230, 85, 221, 243);
RT_INTERFACE!{interface IChainValidationParameters(IChainValidationParametersVtbl): IInspectable(IInspectableVtbl) [IID_IChainValidationParameters] {
    fn get_CertificateChainPolicy(&self, out: *mut CertificateChainPolicy) -> HRESULT,
    fn put_CertificateChainPolicy(&self, value: CertificateChainPolicy) -> HRESULT,
    #[cfg(feature="windows-networking")] fn get_ServerDnsName(&self, out: *mut *mut ::rt::gen::windows::networking::HostName) -> HRESULT,
    #[cfg(feature="windows-networking")] fn put_ServerDnsName(&self, value: *mut ::rt::gen::windows::networking::HostName) -> HRESULT
}}
impl IChainValidationParameters {
    #[inline] pub unsafe fn get_certificate_chain_policy(&self) -> Result<CertificateChainPolicy> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_CertificateChainPolicy)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_certificate_chain_policy(&self, value: CertificateChainPolicy) -> Result<()> {
        let hr = ((*self.lpVtbl).put_CertificateChainPolicy)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[cfg(feature="windows-networking")] #[inline] pub unsafe fn get_server_dns_name(&self) -> Result<ComPtr<::rt::gen::windows::networking::HostName>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_ServerDnsName)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[cfg(feature="windows-networking")] #[inline] pub unsafe fn set_server_dns_name(&self, value: &::rt::gen::windows::networking::HostName) -> Result<()> {
        let hr = ((*self.lpVtbl).put_ServerDnsName)(self as *const _ as *mut _, value as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
}
RT_CLASS!{class ChainValidationParameters: IChainValidationParameters}
impl RtActivatable<IActivationFactory> for ChainValidationParameters {}
DEFINE_CLSID!(ChainValidationParameters(&[87,105,110,100,111,119,115,46,83,101,99,117,114,105,116,121,46,67,114,121,112,116,111,103,114,97,112,104,121,46,67,101,114,116,105,102,105,99,97,116,101,115,46,67,104,97,105,110,86,97,108,105,100,97,116,105,111,110,80,97,114,97,109,101,116,101,114,115,0]) [CLSID_ChainValidationParameters]);
RT_ENUM! { enum ChainValidationResult: i32 {
    Success (ChainValidationResult_Success) = 0, Untrusted (ChainValidationResult_Untrusted) = 1, Revoked (ChainValidationResult_Revoked) = 2, Expired (ChainValidationResult_Expired) = 3, IncompleteChain (ChainValidationResult_IncompleteChain) = 4, InvalidSignature (ChainValidationResult_InvalidSignature) = 5, WrongUsage (ChainValidationResult_WrongUsage) = 6, InvalidName (ChainValidationResult_InvalidName) = 7, InvalidCertificateAuthorityPolicy (ChainValidationResult_InvalidCertificateAuthorityPolicy) = 8, BasicConstraintsError (ChainValidationResult_BasicConstraintsError) = 9, UnknownCriticalExtension (ChainValidationResult_UnknownCriticalExtension) = 10, RevocationInformationMissing (ChainValidationResult_RevocationInformationMissing) = 11, RevocationFailure (ChainValidationResult_RevocationFailure) = 12, OtherErrors (ChainValidationResult_OtherErrors) = 13,
}}
DEFINE_IID!(IID_ICmsAttachedSignature, 1636408733, 14167, 20171, 189, 220, 12, 163, 87, 215, 169, 54);
RT_INTERFACE!{interface ICmsAttachedSignature(ICmsAttachedSignatureVtbl): IInspectable(IInspectableVtbl) [IID_ICmsAttachedSignature] {
    fn get_Certificates(&self, out: *mut *mut ::rt::gen::windows::foundation::collections::IVectorView<Certificate>) -> HRESULT,
    fn get_Content(&self, outSize: *mut u32, out: *mut *mut u8) -> HRESULT,
    fn get_Signers(&self, out: *mut *mut ::rt::gen::windows::foundation::collections::IVectorView<CmsSignerInfo>) -> HRESULT,
    fn VerifySignature(&self, out: *mut SignatureValidationResult) -> HRESULT
}}
impl ICmsAttachedSignature {
    #[inline] pub unsafe fn get_certificates(&self) -> Result<ComPtr<::rt::gen::windows::foundation::collections::IVectorView<Certificate>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Certificates)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_content(&self) -> Result<ComArray<u8>> {
        let mut outSize = 0; let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Content)(self as *const _ as *mut _, &mut outSize, &mut out);
        if hr == S_OK { Ok(ComArray::from_raw(outSize, out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_signers(&self) -> Result<ComPtr<::rt::gen::windows::foundation::collections::IVectorView<CmsSignerInfo>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Signers)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn verify_signature(&self) -> Result<SignatureValidationResult> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).VerifySignature)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
}
RT_CLASS!{class CmsAttachedSignature: ICmsAttachedSignature}
impl RtActivatable<ICmsAttachedSignatureFactory> for CmsAttachedSignature {}
impl RtActivatable<ICmsAttachedSignatureStatics> for CmsAttachedSignature {}
impl CmsAttachedSignature {
    #[cfg(feature="windows-storage")] #[inline] pub fn create_cms_attached_signature(inputBlob: &::rt::gen::windows::storage::streams::IBuffer) -> Result<ComPtr<CmsAttachedSignature>> { unsafe {
        <Self as RtActivatable<ICmsAttachedSignatureFactory>>::get_activation_factory().create_cms_attached_signature(inputBlob)
    }}
    #[cfg(feature="windows-storage")] #[inline] pub fn generate_signature_async(data: &::rt::gen::windows::storage::streams::IBuffer, signers: &::rt::gen::windows::foundation::collections::IIterable<CmsSignerInfo>, certificates: &::rt::gen::windows::foundation::collections::IIterable<Certificate>) -> Result<ComPtr<::rt::gen::windows::foundation::IAsyncOperation<::rt::gen::windows::storage::streams::IBuffer>>> { unsafe {
        <Self as RtActivatable<ICmsAttachedSignatureStatics>>::get_activation_factory().generate_signature_async(data, signers, certificates)
    }}
}
DEFINE_CLSID!(CmsAttachedSignature(&[87,105,110,100,111,119,115,46,83,101,99,117,114,105,116,121,46,67,114,121,112,116,111,103,114,97,112,104,121,46,67,101,114,116,105,102,105,99,97,116,101,115,46,67,109,115,65,116,116,97,99,104,101,100,83,105,103,110,97,116,117,114,101,0]) [CLSID_CmsAttachedSignature]);
DEFINE_IID!(IID_ICmsAttachedSignatureFactory, 3502832661, 63319, 19556, 163, 98, 82, 204, 28, 119, 207, 251);
RT_INTERFACE!{static interface ICmsAttachedSignatureFactory(ICmsAttachedSignatureFactoryVtbl): IInspectable(IInspectableVtbl) [IID_ICmsAttachedSignatureFactory] {
    #[cfg(feature="windows-storage")] fn CreateCmsAttachedSignature(&self, inputBlob: *mut ::rt::gen::windows::storage::streams::IBuffer, out: *mut *mut CmsAttachedSignature) -> HRESULT
}}
impl ICmsAttachedSignatureFactory {
    #[cfg(feature="windows-storage")] #[inline] pub unsafe fn create_cms_attached_signature(&self, inputBlob: &::rt::gen::windows::storage::streams::IBuffer) -> Result<ComPtr<CmsAttachedSignature>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).CreateCmsAttachedSignature)(self as *const _ as *mut _, inputBlob as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
DEFINE_IID!(IID_ICmsAttachedSignatureStatics, 2274925710, 45229, 18829, 167, 245, 120, 181, 155, 206, 75, 54);
RT_INTERFACE!{static interface ICmsAttachedSignatureStatics(ICmsAttachedSignatureStaticsVtbl): IInspectable(IInspectableVtbl) [IID_ICmsAttachedSignatureStatics] {
    #[cfg(feature="windows-storage")] fn GenerateSignatureAsync(&self, data: *mut ::rt::gen::windows::storage::streams::IBuffer, signers: *mut ::rt::gen::windows::foundation::collections::IIterable<CmsSignerInfo>, certificates: *mut ::rt::gen::windows::foundation::collections::IIterable<Certificate>, out: *mut *mut ::rt::gen::windows::foundation::IAsyncOperation<::rt::gen::windows::storage::streams::IBuffer>) -> HRESULT
}}
impl ICmsAttachedSignatureStatics {
    #[cfg(feature="windows-storage")] #[inline] pub unsafe fn generate_signature_async(&self, data: &::rt::gen::windows::storage::streams::IBuffer, signers: &::rt::gen::windows::foundation::collections::IIterable<CmsSignerInfo>, certificates: &::rt::gen::windows::foundation::collections::IIterable<Certificate>) -> Result<ComPtr<::rt::gen::windows::foundation::IAsyncOperation<::rt::gen::windows::storage::streams::IBuffer>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GenerateSignatureAsync)(self as *const _ as *mut _, data as *const _ as *mut _, signers as *const _ as *mut _, certificates as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
DEFINE_IID!(IID_ICmsDetachedSignature, 253686100, 63070, 17718, 131, 57, 89, 68, 8, 29, 178, 202);
RT_INTERFACE!{interface ICmsDetachedSignature(ICmsDetachedSignatureVtbl): IInspectable(IInspectableVtbl) [IID_ICmsDetachedSignature] {
    fn get_Certificates(&self, out: *mut *mut ::rt::gen::windows::foundation::collections::IVectorView<Certificate>) -> HRESULT,
    fn get_Signers(&self, out: *mut *mut ::rt::gen::windows::foundation::collections::IVectorView<CmsSignerInfo>) -> HRESULT,
    #[cfg(feature="windows-storage")] fn VerifySignatureAsync(&self, data: *mut ::rt::gen::windows::storage::streams::IInputStream, out: *mut *mut ::rt::gen::windows::foundation::IAsyncOperation<SignatureValidationResult>) -> HRESULT
}}
impl ICmsDetachedSignature {
    #[inline] pub unsafe fn get_certificates(&self) -> Result<ComPtr<::rt::gen::windows::foundation::collections::IVectorView<Certificate>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Certificates)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_signers(&self) -> Result<ComPtr<::rt::gen::windows::foundation::collections::IVectorView<CmsSignerInfo>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Signers)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[cfg(feature="windows-storage")] #[inline] pub unsafe fn verify_signature_async(&self, data: &::rt::gen::windows::storage::streams::IInputStream) -> Result<ComPtr<::rt::gen::windows::foundation::IAsyncOperation<SignatureValidationResult>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).VerifySignatureAsync)(self as *const _ as *mut _, data as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
RT_CLASS!{class CmsDetachedSignature: ICmsDetachedSignature}
impl RtActivatable<ICmsDetachedSignatureFactory> for CmsDetachedSignature {}
impl RtActivatable<ICmsDetachedSignatureStatics> for CmsDetachedSignature {}
impl CmsDetachedSignature {
    #[cfg(feature="windows-storage")] #[inline] pub fn create_cms_detached_signature(inputBlob: &::rt::gen::windows::storage::streams::IBuffer) -> Result<ComPtr<CmsDetachedSignature>> { unsafe {
        <Self as RtActivatable<ICmsDetachedSignatureFactory>>::get_activation_factory().create_cms_detached_signature(inputBlob)
    }}
    #[cfg(feature="windows-storage")] #[inline] pub fn generate_signature_async(data: &::rt::gen::windows::storage::streams::IInputStream, signers: &::rt::gen::windows::foundation::collections::IIterable<CmsSignerInfo>, certificates: &::rt::gen::windows::foundation::collections::IIterable<Certificate>) -> Result<ComPtr<::rt::gen::windows::foundation::IAsyncOperation<::rt::gen::windows::storage::streams::IBuffer>>> { unsafe {
        <Self as RtActivatable<ICmsDetachedSignatureStatics>>::get_activation_factory().generate_signature_async(data, signers, certificates)
    }}
}
DEFINE_CLSID!(CmsDetachedSignature(&[87,105,110,100,111,119,115,46,83,101,99,117,114,105,116,121,46,67,114,121,112,116,111,103,114,97,112,104,121,46,67,101,114,116,105,102,105,99,97,116,101,115,46,67,109,115,68,101,116,97,99,104,101,100,83,105,103,110,97,116,117,114,101,0]) [CLSID_CmsDetachedSignature]);
DEFINE_IID!(IID_ICmsDetachedSignatureFactory, 3299554563, 44671, 17287, 173, 25, 0, 241, 80, 228, 142, 187);
RT_INTERFACE!{static interface ICmsDetachedSignatureFactory(ICmsDetachedSignatureFactoryVtbl): IInspectable(IInspectableVtbl) [IID_ICmsDetachedSignatureFactory] {
    #[cfg(feature="windows-storage")] fn CreateCmsDetachedSignature(&self, inputBlob: *mut ::rt::gen::windows::storage::streams::IBuffer, out: *mut *mut CmsDetachedSignature) -> HRESULT
}}
impl ICmsDetachedSignatureFactory {
    #[cfg(feature="windows-storage")] #[inline] pub unsafe fn create_cms_detached_signature(&self, inputBlob: &::rt::gen::windows::storage::streams::IBuffer) -> Result<ComPtr<CmsDetachedSignature>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).CreateCmsDetachedSignature)(self as *const _ as *mut _, inputBlob as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
DEFINE_IID!(IID_ICmsDetachedSignatureStatics, 1024543997, 49051, 18050, 155, 230, 145, 245, 124, 5, 56, 8);
RT_INTERFACE!{static interface ICmsDetachedSignatureStatics(ICmsDetachedSignatureStaticsVtbl): IInspectable(IInspectableVtbl) [IID_ICmsDetachedSignatureStatics] {
    #[cfg(feature="windows-storage")] fn GenerateSignatureAsync(&self, data: *mut ::rt::gen::windows::storage::streams::IInputStream, signers: *mut ::rt::gen::windows::foundation::collections::IIterable<CmsSignerInfo>, certificates: *mut ::rt::gen::windows::foundation::collections::IIterable<Certificate>, out: *mut *mut ::rt::gen::windows::foundation::IAsyncOperation<::rt::gen::windows::storage::streams::IBuffer>) -> HRESULT
}}
impl ICmsDetachedSignatureStatics {
    #[cfg(feature="windows-storage")] #[inline] pub unsafe fn generate_signature_async(&self, data: &::rt::gen::windows::storage::streams::IInputStream, signers: &::rt::gen::windows::foundation::collections::IIterable<CmsSignerInfo>, certificates: &::rt::gen::windows::foundation::collections::IIterable<Certificate>) -> Result<ComPtr<::rt::gen::windows::foundation::IAsyncOperation<::rt::gen::windows::storage::streams::IBuffer>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GenerateSignatureAsync)(self as *const _ as *mut _, data as *const _ as *mut _, signers as *const _ as *mut _, certificates as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
DEFINE_IID!(IID_ICmsSignerInfo, 1355817179, 7471, 19482, 181, 197, 208, 24, 143, 249, 31, 71);
RT_INTERFACE!{interface ICmsSignerInfo(ICmsSignerInfoVtbl): IInspectable(IInspectableVtbl) [IID_ICmsSignerInfo] {
    fn get_Certificate(&self, out: *mut *mut Certificate) -> HRESULT,
    fn put_Certificate(&self, value: *mut Certificate) -> HRESULT,
    fn get_HashAlgorithmName(&self, out: *mut HSTRING) -> HRESULT,
    fn put_HashAlgorithmName(&self, value: HSTRING) -> HRESULT,
    fn get_TimestampInfo(&self, out: *mut *mut CmsTimestampInfo) -> HRESULT
}}
impl ICmsSignerInfo {
    #[inline] pub unsafe fn get_certificate(&self) -> Result<ComPtr<Certificate>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Certificate)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_certificate(&self, value: &Certificate) -> Result<()> {
        let hr = ((*self.lpVtbl).put_Certificate)(self as *const _ as *mut _, value as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_hash_algorithm_name(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_HashAlgorithmName)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_hash_algorithm_name(&self, value: &HStringArg) -> Result<()> {
        let hr = ((*self.lpVtbl).put_HashAlgorithmName)(self as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_timestamp_info(&self) -> Result<ComPtr<CmsTimestampInfo>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_TimestampInfo)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
RT_CLASS!{class CmsSignerInfo: ICmsSignerInfo}
impl RtActivatable<IActivationFactory> for CmsSignerInfo {}
DEFINE_CLSID!(CmsSignerInfo(&[87,105,110,100,111,119,115,46,83,101,99,117,114,105,116,121,46,67,114,121,112,116,111,103,114,97,112,104,121,46,67,101,114,116,105,102,105,99,97,116,101,115,46,67,109,115,83,105,103,110,101,114,73,110,102,111,0]) [CLSID_CmsSignerInfo]);
DEFINE_IID!(IID_ICmsTimestampInfo, 794755314, 11288, 20360, 132, 53, 197, 52, 8, 96, 118, 245);
RT_INTERFACE!{interface ICmsTimestampInfo(ICmsTimestampInfoVtbl): IInspectable(IInspectableVtbl) [IID_ICmsTimestampInfo] {
    fn get_SigningCertificate(&self, out: *mut *mut Certificate) -> HRESULT,
    fn get_Certificates(&self, out: *mut *mut ::rt::gen::windows::foundation::collections::IVectorView<Certificate>) -> HRESULT,
    fn get_Timestamp(&self, out: *mut ::rt::gen::windows::foundation::DateTime) -> HRESULT
}}
impl ICmsTimestampInfo {
    #[inline] pub unsafe fn get_signing_certificate(&self) -> Result<ComPtr<Certificate>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_SigningCertificate)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_certificates(&self) -> Result<ComPtr<::rt::gen::windows::foundation::collections::IVectorView<Certificate>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Certificates)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_timestamp(&self) -> Result<::rt::gen::windows::foundation::DateTime> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_Timestamp)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
}
RT_CLASS!{class CmsTimestampInfo: ICmsTimestampInfo}
RT_ENUM! { enum EnrollKeyUsages: u32 {
    None (EnrollKeyUsages_None) = 0, Decryption (EnrollKeyUsages_Decryption) = 1, Signing (EnrollKeyUsages_Signing) = 2, KeyAgreement (EnrollKeyUsages_KeyAgreement) = 4, All (EnrollKeyUsages_All) = 16777215,
}}
RT_ENUM! { enum ExportOption: i32 {
    NotExportable (ExportOption_NotExportable) = 0, Exportable (ExportOption_Exportable) = 1,
}}
RT_ENUM! { enum InstallOptions: u32 {
    None (InstallOptions_None) = 0, DeleteExpired (InstallOptions_DeleteExpired) = 1,
}}
RT_CLASS!{static class KeyAlgorithmNames}
impl RtActivatable<IKeyAlgorithmNamesStatics> for KeyAlgorithmNames {}
impl RtActivatable<IKeyAlgorithmNamesStatics2> for KeyAlgorithmNames {}
impl KeyAlgorithmNames {
    #[inline] pub fn get_rsa() -> Result<HString> { unsafe {
        <Self as RtActivatable<IKeyAlgorithmNamesStatics>>::get_activation_factory().get_rsa()
    }}
    #[inline] pub fn get_dsa() -> Result<HString> { unsafe {
        <Self as RtActivatable<IKeyAlgorithmNamesStatics>>::get_activation_factory().get_dsa()
    }}
    #[inline] pub fn get_ecdh256() -> Result<HString> { unsafe {
        <Self as RtActivatable<IKeyAlgorithmNamesStatics>>::get_activation_factory().get_ecdh256()
    }}
    #[inline] pub fn get_ecdh384() -> Result<HString> { unsafe {
        <Self as RtActivatable<IKeyAlgorithmNamesStatics>>::get_activation_factory().get_ecdh384()
    }}
    #[inline] pub fn get_ecdh521() -> Result<HString> { unsafe {
        <Self as RtActivatable<IKeyAlgorithmNamesStatics>>::get_activation_factory().get_ecdh521()
    }}
    #[inline] pub fn get_ecdsa256() -> Result<HString> { unsafe {
        <Self as RtActivatable<IKeyAlgorithmNamesStatics>>::get_activation_factory().get_ecdsa256()
    }}
    #[inline] pub fn get_ecdsa384() -> Result<HString> { unsafe {
        <Self as RtActivatable<IKeyAlgorithmNamesStatics>>::get_activation_factory().get_ecdsa384()
    }}
    #[inline] pub fn get_ecdsa521() -> Result<HString> { unsafe {
        <Self as RtActivatable<IKeyAlgorithmNamesStatics>>::get_activation_factory().get_ecdsa521()
    }}
    #[inline] pub fn get_ecdsa() -> Result<HString> { unsafe {
        <Self as RtActivatable<IKeyAlgorithmNamesStatics2>>::get_activation_factory().get_ecdsa()
    }}
    #[inline] pub fn get_ecdh() -> Result<HString> { unsafe {
        <Self as RtActivatable<IKeyAlgorithmNamesStatics2>>::get_activation_factory().get_ecdh()
    }}
}
DEFINE_CLSID!(KeyAlgorithmNames(&[87,105,110,100,111,119,115,46,83,101,99,117,114,105,116,121,46,67,114,121,112,116,111,103,114,97,112,104,121,46,67,101,114,116,105,102,105,99,97,116,101,115,46,75,101,121,65,108,103,111,114,105,116,104,109,78,97,109,101,115,0]) [CLSID_KeyAlgorithmNames]);
DEFINE_IID!(IID_IKeyAlgorithmNamesStatics, 1200645591, 31431, 17793, 140, 59, 208, 112, 39, 20, 4, 72);
RT_INTERFACE!{static interface IKeyAlgorithmNamesStatics(IKeyAlgorithmNamesStaticsVtbl): IInspectable(IInspectableVtbl) [IID_IKeyAlgorithmNamesStatics] {
    fn get_Rsa(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Dsa(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Ecdh256(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Ecdh384(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Ecdh521(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Ecdsa256(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Ecdsa384(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Ecdsa521(&self, out: *mut HSTRING) -> HRESULT
}}
impl IKeyAlgorithmNamesStatics {
    #[inline] pub unsafe fn get_rsa(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Rsa)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_dsa(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Dsa)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_ecdh256(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Ecdh256)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_ecdh384(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Ecdh384)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_ecdh521(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Ecdh521)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_ecdsa256(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Ecdsa256)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_ecdsa384(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Ecdsa384)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_ecdsa521(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Ecdsa521)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
}
DEFINE_IID!(IID_IKeyAlgorithmNamesStatics2, 3382400646, 57853, 19018, 137, 61, 162, 111, 51, 221, 139, 180);
RT_INTERFACE!{static interface IKeyAlgorithmNamesStatics2(IKeyAlgorithmNamesStatics2Vtbl): IInspectable(IInspectableVtbl) [IID_IKeyAlgorithmNamesStatics2] {
    fn get_Ecdsa(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Ecdh(&self, out: *mut HSTRING) -> HRESULT
}}
impl IKeyAlgorithmNamesStatics2 {
    #[inline] pub unsafe fn get_ecdsa(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Ecdsa)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_ecdh(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Ecdh)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
}
RT_CLASS!{static class KeyAttestationHelper}
impl RtActivatable<IKeyAttestationHelperStatics> for KeyAttestationHelper {}
impl RtActivatable<IKeyAttestationHelperStatics2> for KeyAttestationHelper {}
impl KeyAttestationHelper {
    #[inline] pub fn decrypt_tpm_attestation_credential_async(credential: &HStringArg) -> Result<ComPtr<::rt::gen::windows::foundation::IAsyncOperation<HString>>> { unsafe {
        <Self as RtActivatable<IKeyAttestationHelperStatics>>::get_activation_factory().decrypt_tpm_attestation_credential_async(credential)
    }}
    #[inline] pub fn get_tpm_attestation_credential_id(credential: &HStringArg) -> Result<HString> { unsafe {
        <Self as RtActivatable<IKeyAttestationHelperStatics>>::get_activation_factory().get_tpm_attestation_credential_id(credential)
    }}
    #[inline] pub fn decrypt_tpm_attestation_credential_with_container_name_async(credential: &HStringArg, containerName: &HStringArg) -> Result<ComPtr<::rt::gen::windows::foundation::IAsyncOperation<HString>>> { unsafe {
        <Self as RtActivatable<IKeyAttestationHelperStatics2>>::get_activation_factory().decrypt_tpm_attestation_credential_with_container_name_async(credential, containerName)
    }}
}
DEFINE_CLSID!(KeyAttestationHelper(&[87,105,110,100,111,119,115,46,83,101,99,117,114,105,116,121,46,67,114,121,112,116,111,103,114,97,112,104,121,46,67,101,114,116,105,102,105,99,97,116,101,115,46,75,101,121,65,116,116,101,115,116,97,116,105,111,110,72,101,108,112,101,114,0]) [CLSID_KeyAttestationHelper]);
DEFINE_IID!(IID_IKeyAttestationHelperStatics, 373875270, 63044, 17190, 136, 190, 58, 241, 2, 211, 14, 12);
RT_INTERFACE!{static interface IKeyAttestationHelperStatics(IKeyAttestationHelperStaticsVtbl): IInspectable(IInspectableVtbl) [IID_IKeyAttestationHelperStatics] {
    fn DecryptTpmAttestationCredentialAsync(&self, credential: HSTRING, out: *mut *mut ::rt::gen::windows::foundation::IAsyncOperation<HString>) -> HRESULT,
    fn GetTpmAttestationCredentialId(&self, credential: HSTRING, out: *mut HSTRING) -> HRESULT
}}
impl IKeyAttestationHelperStatics {
    #[inline] pub unsafe fn decrypt_tpm_attestation_credential_async(&self, credential: &HStringArg) -> Result<ComPtr<::rt::gen::windows::foundation::IAsyncOperation<HString>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).DecryptTpmAttestationCredentialAsync)(self as *const _ as *mut _, credential.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_tpm_attestation_credential_id(&self, credential: &HStringArg) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetTpmAttestationCredentialId)(self as *const _ as *mut _, credential.get(), &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
}
DEFINE_IID!(IID_IKeyAttestationHelperStatics2, 2623081260, 42694, 19038, 158, 100, 232, 93, 82, 121, 223, 151);
RT_INTERFACE!{static interface IKeyAttestationHelperStatics2(IKeyAttestationHelperStatics2Vtbl): IInspectable(IInspectableVtbl) [IID_IKeyAttestationHelperStatics2] {
    fn DecryptTpmAttestationCredentialWithContainerNameAsync(&self, credential: HSTRING, containerName: HSTRING, out: *mut *mut ::rt::gen::windows::foundation::IAsyncOperation<HString>) -> HRESULT
}}
impl IKeyAttestationHelperStatics2 {
    #[inline] pub unsafe fn decrypt_tpm_attestation_credential_with_container_name_async(&self, credential: &HStringArg, containerName: &HStringArg) -> Result<ComPtr<::rt::gen::windows::foundation::IAsyncOperation<HString>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).DecryptTpmAttestationCredentialWithContainerNameAsync)(self as *const _ as *mut _, credential.get(), containerName.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
RT_ENUM! { enum KeyProtectionLevel: i32 {
    NoConsent (KeyProtectionLevel_NoConsent) = 0, ConsentOnly (KeyProtectionLevel_ConsentOnly) = 1, ConsentWithPassword (KeyProtectionLevel_ConsentWithPassword) = 2, ConsentWithFingerprint (KeyProtectionLevel_ConsentWithFingerprint) = 3,
}}
RT_ENUM! { enum KeySize: i32 {
    Invalid (KeySize_Invalid) = 0, Rsa2048 (KeySize_Rsa2048) = 2048, Rsa4096 (KeySize_Rsa4096) = 4096,
}}
RT_CLASS!{static class KeyStorageProviderNames}
impl RtActivatable<IKeyStorageProviderNamesStatics> for KeyStorageProviderNames {}
impl RtActivatable<IKeyStorageProviderNamesStatics2> for KeyStorageProviderNames {}
impl KeyStorageProviderNames {
    #[inline] pub fn get_software_key_storage_provider() -> Result<HString> { unsafe {
        <Self as RtActivatable<IKeyStorageProviderNamesStatics>>::get_activation_factory().get_software_key_storage_provider()
    }}
    #[inline] pub fn get_smartcard_key_storage_provider() -> Result<HString> { unsafe {
        <Self as RtActivatable<IKeyStorageProviderNamesStatics>>::get_activation_factory().get_smartcard_key_storage_provider()
    }}
    #[inline] pub fn get_platform_key_storage_provider() -> Result<HString> { unsafe {
        <Self as RtActivatable<IKeyStorageProviderNamesStatics>>::get_activation_factory().get_platform_key_storage_provider()
    }}
    #[inline] pub fn get_passport_key_storage_provider() -> Result<HString> { unsafe {
        <Self as RtActivatable<IKeyStorageProviderNamesStatics2>>::get_activation_factory().get_passport_key_storage_provider()
    }}
}
DEFINE_CLSID!(KeyStorageProviderNames(&[87,105,110,100,111,119,115,46,83,101,99,117,114,105,116,121,46,67,114,121,112,116,111,103,114,97,112,104,121,46,67,101,114,116,105,102,105,99,97,116,101,115,46,75,101,121,83,116,111,114,97,103,101,80,114,111,118,105,100,101,114,78,97,109,101,115,0]) [CLSID_KeyStorageProviderNames]);
DEFINE_IID!(IID_IKeyStorageProviderNamesStatics, 2937613024, 21801, 17922, 189, 148, 10, 171, 145, 149, 123, 92);
RT_INTERFACE!{static interface IKeyStorageProviderNamesStatics(IKeyStorageProviderNamesStaticsVtbl): IInspectable(IInspectableVtbl) [IID_IKeyStorageProviderNamesStatics] {
    fn get_SoftwareKeyStorageProvider(&self, out: *mut HSTRING) -> HRESULT,
    fn get_SmartcardKeyStorageProvider(&self, out: *mut HSTRING) -> HRESULT,
    fn get_PlatformKeyStorageProvider(&self, out: *mut HSTRING) -> HRESULT
}}
impl IKeyStorageProviderNamesStatics {
    #[inline] pub unsafe fn get_software_key_storage_provider(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_SoftwareKeyStorageProvider)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_smartcard_key_storage_provider(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_SmartcardKeyStorageProvider)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_platform_key_storage_provider(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_PlatformKeyStorageProvider)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
}
DEFINE_IID!(IID_IKeyStorageProviderNamesStatics2, 640513085, 39982, 16844, 136, 18, 196, 217, 113, 221, 124, 96);
RT_INTERFACE!{static interface IKeyStorageProviderNamesStatics2(IKeyStorageProviderNamesStatics2Vtbl): IInspectable(IInspectableVtbl) [IID_IKeyStorageProviderNamesStatics2] {
    fn get_PassportKeyStorageProvider(&self, out: *mut HSTRING) -> HRESULT
}}
impl IKeyStorageProviderNamesStatics2 {
    #[inline] pub unsafe fn get_passport_key_storage_provider(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_PassportKeyStorageProvider)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
}
DEFINE_IID!(IID_IPfxImportParameters, 1745696017, 39432, 18376, 134, 74, 46, 221, 77, 142, 180, 108);
RT_INTERFACE!{interface IPfxImportParameters(IPfxImportParametersVtbl): IInspectable(IInspectableVtbl) [IID_IPfxImportParameters] {
    fn get_Exportable(&self, out: *mut ExportOption) -> HRESULT,
    fn put_Exportable(&self, value: ExportOption) -> HRESULT,
    fn get_KeyProtectionLevel(&self, out: *mut KeyProtectionLevel) -> HRESULT,
    fn put_KeyProtectionLevel(&self, value: KeyProtectionLevel) -> HRESULT,
    fn get_InstallOptions(&self, out: *mut InstallOptions) -> HRESULT,
    fn put_InstallOptions(&self, value: InstallOptions) -> HRESULT,
    fn get_FriendlyName(&self, out: *mut HSTRING) -> HRESULT,
    fn put_FriendlyName(&self, value: HSTRING) -> HRESULT,
    fn get_KeyStorageProviderName(&self, out: *mut HSTRING) -> HRESULT,
    fn put_KeyStorageProviderName(&self, value: HSTRING) -> HRESULT,
    fn get_ContainerNamePrefix(&self, out: *mut HSTRING) -> HRESULT,
    fn put_ContainerNamePrefix(&self, value: HSTRING) -> HRESULT,
    fn get_ReaderName(&self, out: *mut HSTRING) -> HRESULT,
    fn put_ReaderName(&self, value: HSTRING) -> HRESULT
}}
impl IPfxImportParameters {
    #[inline] pub unsafe fn get_exportable(&self) -> Result<ExportOption> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_Exportable)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_exportable(&self, value: ExportOption) -> Result<()> {
        let hr = ((*self.lpVtbl).put_Exportable)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_key_protection_level(&self) -> Result<KeyProtectionLevel> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_KeyProtectionLevel)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_key_protection_level(&self, value: KeyProtectionLevel) -> Result<()> {
        let hr = ((*self.lpVtbl).put_KeyProtectionLevel)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_install_options(&self) -> Result<InstallOptions> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_InstallOptions)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_install_options(&self, value: InstallOptions) -> Result<()> {
        let hr = ((*self.lpVtbl).put_InstallOptions)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_friendly_name(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_FriendlyName)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_friendly_name(&self, value: &HStringArg) -> Result<()> {
        let hr = ((*self.lpVtbl).put_FriendlyName)(self as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_key_storage_provider_name(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_KeyStorageProviderName)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_key_storage_provider_name(&self, value: &HStringArg) -> Result<()> {
        let hr = ((*self.lpVtbl).put_KeyStorageProviderName)(self as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_container_name_prefix(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_ContainerNamePrefix)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_container_name_prefix(&self, value: &HStringArg) -> Result<()> {
        let hr = ((*self.lpVtbl).put_ContainerNamePrefix)(self as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_reader_name(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_ReaderName)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_reader_name(&self, value: &HStringArg) -> Result<()> {
        let hr = ((*self.lpVtbl).put_ReaderName)(self as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
}
RT_CLASS!{class PfxImportParameters: IPfxImportParameters}
impl RtActivatable<IActivationFactory> for PfxImportParameters {}
DEFINE_CLSID!(PfxImportParameters(&[87,105,110,100,111,119,115,46,83,101,99,117,114,105,116,121,46,67,114,121,112,116,111,103,114,97,112,104,121,46,67,101,114,116,105,102,105,99,97,116,101,115,46,80,102,120,73,109,112,111,114,116,80,97,114,97,109,101,116,101,114,115,0]) [CLSID_PfxImportParameters]);
RT_ENUM! { enum SignatureValidationResult: i32 {
    Success (SignatureValidationResult_Success) = 0, InvalidParameter (SignatureValidationResult_InvalidParameter) = 1, BadMessage (SignatureValidationResult_BadMessage) = 2, InvalidSignature (SignatureValidationResult_InvalidSignature) = 3, OtherErrors (SignatureValidationResult_OtherErrors) = 4,
}}
RT_CLASS!{static class StandardCertificateStoreNames}
impl RtActivatable<IStandardCertificateStoreNamesStatics> for StandardCertificateStoreNames {}
impl StandardCertificateStoreNames {
    #[inline] pub fn get_personal() -> Result<HString> { unsafe {
        <Self as RtActivatable<IStandardCertificateStoreNamesStatics>>::get_activation_factory().get_personal()
    }}
    #[inline] pub fn get_trusted_root_certification_authorities() -> Result<HString> { unsafe {
        <Self as RtActivatable<IStandardCertificateStoreNamesStatics>>::get_activation_factory().get_trusted_root_certification_authorities()
    }}
    #[inline] pub fn get_intermediate_certification_authorities() -> Result<HString> { unsafe {
        <Self as RtActivatable<IStandardCertificateStoreNamesStatics>>::get_activation_factory().get_intermediate_certification_authorities()
    }}
}
DEFINE_CLSID!(StandardCertificateStoreNames(&[87,105,110,100,111,119,115,46,83,101,99,117,114,105,116,121,46,67,114,121,112,116,111,103,114,97,112,104,121,46,67,101,114,116,105,102,105,99,97,116,101,115,46,83,116,97,110,100,97,114,100,67,101,114,116,105,102,105,99,97,116,101,83,116,111,114,101,78,97,109,101,115,0]) [CLSID_StandardCertificateStoreNames]);
DEFINE_IID!(IID_IStandardCertificateStoreNamesStatics, 202722011, 42134, 16888, 143, 229, 158, 150, 243, 110, 251, 248);
RT_INTERFACE!{static interface IStandardCertificateStoreNamesStatics(IStandardCertificateStoreNamesStaticsVtbl): IInspectable(IInspectableVtbl) [IID_IStandardCertificateStoreNamesStatics] {
    fn get_Personal(&self, out: *mut HSTRING) -> HRESULT,
    fn get_TrustedRootCertificationAuthorities(&self, out: *mut HSTRING) -> HRESULT,
    fn get_IntermediateCertificationAuthorities(&self, out: *mut HSTRING) -> HRESULT
}}
impl IStandardCertificateStoreNamesStatics {
    #[inline] pub unsafe fn get_personal(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Personal)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_trusted_root_certification_authorities(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_TrustedRootCertificationAuthorities)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_intermediate_certification_authorities(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_IntermediateCertificationAuthorities)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
}
DEFINE_IID!(IID_ISubjectAlternativeNameInfo, 1479039473, 22173, 19488, 190, 123, 78, 28, 154, 11, 197, 43);
RT_INTERFACE!{interface ISubjectAlternativeNameInfo(ISubjectAlternativeNameInfoVtbl): IInspectable(IInspectableVtbl) [IID_ISubjectAlternativeNameInfo] {
    fn get_EmailName(&self, out: *mut *mut ::rt::gen::windows::foundation::collections::IVectorView<HString>) -> HRESULT,
    fn get_IPAddress(&self, out: *mut *mut ::rt::gen::windows::foundation::collections::IVectorView<HString>) -> HRESULT,
    fn get_Url(&self, out: *mut *mut ::rt::gen::windows::foundation::collections::IVectorView<HString>) -> HRESULT,
    fn get_DnsName(&self, out: *mut *mut ::rt::gen::windows::foundation::collections::IVectorView<HString>) -> HRESULT,
    fn get_DistinguishedName(&self, out: *mut *mut ::rt::gen::windows::foundation::collections::IVectorView<HString>) -> HRESULT,
    fn get_PrincipalName(&self, out: *mut *mut ::rt::gen::windows::foundation::collections::IVectorView<HString>) -> HRESULT
}}
impl ISubjectAlternativeNameInfo {
    #[inline] pub unsafe fn get_email_name(&self) -> Result<ComPtr<::rt::gen::windows::foundation::collections::IVectorView<HString>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_EmailName)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_ipaddress(&self) -> Result<ComPtr<::rt::gen::windows::foundation::collections::IVectorView<HString>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_IPAddress)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_url(&self) -> Result<ComPtr<::rt::gen::windows::foundation::collections::IVectorView<HString>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Url)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_dns_name(&self) -> Result<ComPtr<::rt::gen::windows::foundation::collections::IVectorView<HString>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_DnsName)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_distinguished_name(&self) -> Result<ComPtr<::rt::gen::windows::foundation::collections::IVectorView<HString>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_DistinguishedName)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_principal_name(&self) -> Result<ComPtr<::rt::gen::windows::foundation::collections::IVectorView<HString>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_PrincipalName)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
RT_CLASS!{class SubjectAlternativeNameInfo: ISubjectAlternativeNameInfo}
impl RtActivatable<IActivationFactory> for SubjectAlternativeNameInfo {}
DEFINE_CLSID!(SubjectAlternativeNameInfo(&[87,105,110,100,111,119,115,46,83,101,99,117,114,105,116,121,46,67,114,121,112,116,111,103,114,97,112,104,121,46,67,101,114,116,105,102,105,99,97,116,101,115,46,83,117,98,106,101,99,116,65,108,116,101,114,110,97,116,105,118,101,78,97,109,101,73,110,102,111,0]) [CLSID_SubjectAlternativeNameInfo]);
DEFINE_IID!(IID_ISubjectAlternativeNameInfo2, 1132099782, 7249, 16874, 179, 74, 61, 101, 67, 152, 163, 112);
RT_INTERFACE!{interface ISubjectAlternativeNameInfo2(ISubjectAlternativeNameInfo2Vtbl): IInspectable(IInspectableVtbl) [IID_ISubjectAlternativeNameInfo2] {
    fn get_EmailNames(&self, out: *mut *mut ::rt::gen::windows::foundation::collections::IVector<HString>) -> HRESULT,
    fn get_IPAddresses(&self, out: *mut *mut ::rt::gen::windows::foundation::collections::IVector<HString>) -> HRESULT,
    fn get_Urls(&self, out: *mut *mut ::rt::gen::windows::foundation::collections::IVector<HString>) -> HRESULT,
    fn get_DnsNames(&self, out: *mut *mut ::rt::gen::windows::foundation::collections::IVector<HString>) -> HRESULT,
    fn get_DistinguishedNames(&self, out: *mut *mut ::rt::gen::windows::foundation::collections::IVector<HString>) -> HRESULT,
    fn get_PrincipalNames(&self, out: *mut *mut ::rt::gen::windows::foundation::collections::IVector<HString>) -> HRESULT,
    fn get_Extension(&self, out: *mut *mut CertificateExtension) -> HRESULT
}}
impl ISubjectAlternativeNameInfo2 {
    #[inline] pub unsafe fn get_email_names(&self) -> Result<ComPtr<::rt::gen::windows::foundation::collections::IVector<HString>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_EmailNames)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_ipaddresses(&self) -> Result<ComPtr<::rt::gen::windows::foundation::collections::IVector<HString>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_IPAddresses)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_urls(&self) -> Result<ComPtr<::rt::gen::windows::foundation::collections::IVector<HString>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Urls)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_dns_names(&self) -> Result<ComPtr<::rt::gen::windows::foundation::collections::IVector<HString>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_DnsNames)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_distinguished_names(&self) -> Result<ComPtr<::rt::gen::windows::foundation::collections::IVector<HString>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_DistinguishedNames)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_principal_names(&self) -> Result<ComPtr<::rt::gen::windows::foundation::collections::IVector<HString>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_PrincipalNames)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_extension(&self) -> Result<ComPtr<CertificateExtension>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Extension)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
DEFINE_IID!(IID_IUserCertificateEnrollmentManager, 2519807768, 8929, 18457, 178, 11, 171, 70, 166, 236, 160, 110);
RT_INTERFACE!{interface IUserCertificateEnrollmentManager(IUserCertificateEnrollmentManagerVtbl): IInspectable(IInspectableVtbl) [IID_IUserCertificateEnrollmentManager] {
    fn CreateRequestAsync(&self, request: *mut CertificateRequestProperties, out: *mut *mut ::rt::gen::windows::foundation::IAsyncOperation<HString>) -> HRESULT,
    fn InstallCertificateAsync(&self, certificate: HSTRING, installOption: InstallOptions, out: *mut *mut ::rt::gen::windows::foundation::IAsyncAction) -> HRESULT,
    fn ImportPfxDataAsync(&self, pfxData: HSTRING, password: HSTRING, exportable: ExportOption, keyProtectionLevel: KeyProtectionLevel, installOption: InstallOptions, friendlyName: HSTRING, out: *mut *mut ::rt::gen::windows::foundation::IAsyncAction) -> HRESULT,
    fn ImportPfxDataToKspAsync(&self, pfxData: HSTRING, password: HSTRING, exportable: ExportOption, keyProtectionLevel: KeyProtectionLevel, installOption: InstallOptions, friendlyName: HSTRING, keyStorageProvider: HSTRING, out: *mut *mut ::rt::gen::windows::foundation::IAsyncAction) -> HRESULT
}}
impl IUserCertificateEnrollmentManager {
    #[inline] pub unsafe fn create_request_async(&self, request: &CertificateRequestProperties) -> Result<ComPtr<::rt::gen::windows::foundation::IAsyncOperation<HString>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).CreateRequestAsync)(self as *const _ as *mut _, request as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn install_certificate_async(&self, certificate: &HStringArg, installOption: InstallOptions) -> Result<ComPtr<::rt::gen::windows::foundation::IAsyncAction>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).InstallCertificateAsync)(self as *const _ as *mut _, certificate.get(), installOption, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn import_pfx_data_async(&self, pfxData: &HStringArg, password: &HStringArg, exportable: ExportOption, keyProtectionLevel: KeyProtectionLevel, installOption: InstallOptions, friendlyName: &HStringArg) -> Result<ComPtr<::rt::gen::windows::foundation::IAsyncAction>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).ImportPfxDataAsync)(self as *const _ as *mut _, pfxData.get(), password.get(), exportable, keyProtectionLevel, installOption, friendlyName.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn import_pfx_data_to_ksp_async(&self, pfxData: &HStringArg, password: &HStringArg, exportable: ExportOption, keyProtectionLevel: KeyProtectionLevel, installOption: InstallOptions, friendlyName: &HStringArg, keyStorageProvider: &HStringArg) -> Result<ComPtr<::rt::gen::windows::foundation::IAsyncAction>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).ImportPfxDataToKspAsync)(self as *const _ as *mut _, pfxData.get(), password.get(), exportable, keyProtectionLevel, installOption, friendlyName.get(), keyStorageProvider.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
RT_CLASS!{class UserCertificateEnrollmentManager: IUserCertificateEnrollmentManager}
DEFINE_IID!(IID_IUserCertificateEnrollmentManager2, 229481649, 26078, 18730, 184, 109, 252, 92, 72, 44, 55, 71);
RT_INTERFACE!{interface IUserCertificateEnrollmentManager2(IUserCertificateEnrollmentManager2Vtbl): IInspectable(IInspectableVtbl) [IID_IUserCertificateEnrollmentManager2] {
    fn ImportPfxDataToKspWithParametersAsync(&self, pfxData: HSTRING, password: HSTRING, pfxImportParameters: *mut PfxImportParameters, out: *mut *mut ::rt::gen::windows::foundation::IAsyncAction) -> HRESULT
}}
impl IUserCertificateEnrollmentManager2 {
    #[inline] pub unsafe fn import_pfx_data_to_ksp_with_parameters_async(&self, pfxData: &HStringArg, password: &HStringArg, pfxImportParameters: &PfxImportParameters) -> Result<ComPtr<::rt::gen::windows::foundation::IAsyncAction>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).ImportPfxDataToKspWithParametersAsync)(self as *const _ as *mut _, pfxData.get(), password.get(), pfxImportParameters as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
DEFINE_IID!(IID_IUserCertificateStore, 3388677507, 30879, 19278, 145, 128, 4, 90, 117, 122, 172, 109);
RT_INTERFACE!{interface IUserCertificateStore(IUserCertificateStoreVtbl): IInspectable(IInspectableVtbl) [IID_IUserCertificateStore] {
    fn RequestAddAsync(&self, certificate: *mut Certificate, out: *mut *mut ::rt::gen::windows::foundation::IAsyncOperation<bool>) -> HRESULT,
    fn RequestDeleteAsync(&self, certificate: *mut Certificate, out: *mut *mut ::rt::gen::windows::foundation::IAsyncOperation<bool>) -> HRESULT,
    fn get_Name(&self, out: *mut HSTRING) -> HRESULT
}}
impl IUserCertificateStore {
    #[inline] pub unsafe fn request_add_async(&self, certificate: &Certificate) -> Result<ComPtr<::rt::gen::windows::foundation::IAsyncOperation<bool>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).RequestAddAsync)(self as *const _ as *mut _, certificate as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn request_delete_async(&self, certificate: &Certificate) -> Result<ComPtr<::rt::gen::windows::foundation::IAsyncOperation<bool>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).RequestDeleteAsync)(self as *const _ as *mut _, certificate as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_name(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Name)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
}
RT_CLASS!{class UserCertificateStore: IUserCertificateStore}
} // Windows.Security.Cryptography.Certificates
pub mod core { // Windows.Security.Cryptography.Core
use ::prelude::*;
RT_CLASS!{static class AsymmetricAlgorithmNames}
impl RtActivatable<IAsymmetricAlgorithmNamesStatics> for AsymmetricAlgorithmNames {}
impl RtActivatable<IAsymmetricAlgorithmNamesStatics2> for AsymmetricAlgorithmNames {}
impl AsymmetricAlgorithmNames {
    #[inline] pub fn get_rsa_pkcs1() -> Result<HString> { unsafe {
        <Self as RtActivatable<IAsymmetricAlgorithmNamesStatics>>::get_activation_factory().get_rsa_pkcs1()
    }}
    #[inline] pub fn get_rsa_oaep_sha1() -> Result<HString> { unsafe {
        <Self as RtActivatable<IAsymmetricAlgorithmNamesStatics>>::get_activation_factory().get_rsa_oaep_sha1()
    }}
    #[inline] pub fn get_rsa_oaep_sha256() -> Result<HString> { unsafe {
        <Self as RtActivatable<IAsymmetricAlgorithmNamesStatics>>::get_activation_factory().get_rsa_oaep_sha256()
    }}
    #[inline] pub fn get_rsa_oaep_sha384() -> Result<HString> { unsafe {
        <Self as RtActivatable<IAsymmetricAlgorithmNamesStatics>>::get_activation_factory().get_rsa_oaep_sha384()
    }}
    #[inline] pub fn get_rsa_oaep_sha512() -> Result<HString> { unsafe {
        <Self as RtActivatable<IAsymmetricAlgorithmNamesStatics>>::get_activation_factory().get_rsa_oaep_sha512()
    }}
    #[inline] pub fn get_ecdsa_p256_sha256() -> Result<HString> { unsafe {
        <Self as RtActivatable<IAsymmetricAlgorithmNamesStatics>>::get_activation_factory().get_ecdsa_p256_sha256()
    }}
    #[inline] pub fn get_ecdsa_p384_sha384() -> Result<HString> { unsafe {
        <Self as RtActivatable<IAsymmetricAlgorithmNamesStatics>>::get_activation_factory().get_ecdsa_p384_sha384()
    }}
    #[inline] pub fn get_ecdsa_p521_sha512() -> Result<HString> { unsafe {
        <Self as RtActivatable<IAsymmetricAlgorithmNamesStatics>>::get_activation_factory().get_ecdsa_p521_sha512()
    }}
    #[inline] pub fn get_dsa_sha1() -> Result<HString> { unsafe {
        <Self as RtActivatable<IAsymmetricAlgorithmNamesStatics>>::get_activation_factory().get_dsa_sha1()
    }}
    #[inline] pub fn get_dsa_sha256() -> Result<HString> { unsafe {
        <Self as RtActivatable<IAsymmetricAlgorithmNamesStatics>>::get_activation_factory().get_dsa_sha256()
    }}
    #[inline] pub fn get_rsa_sign_pkcs1_sha1() -> Result<HString> { unsafe {
        <Self as RtActivatable<IAsymmetricAlgorithmNamesStatics>>::get_activation_factory().get_rsa_sign_pkcs1_sha1()
    }}
    #[inline] pub fn get_rsa_sign_pkcs1_sha256() -> Result<HString> { unsafe {
        <Self as RtActivatable<IAsymmetricAlgorithmNamesStatics>>::get_activation_factory().get_rsa_sign_pkcs1_sha256()
    }}
    #[inline] pub fn get_rsa_sign_pkcs1_sha384() -> Result<HString> { unsafe {
        <Self as RtActivatable<IAsymmetricAlgorithmNamesStatics>>::get_activation_factory().get_rsa_sign_pkcs1_sha384()
    }}
    #[inline] pub fn get_rsa_sign_pkcs1_sha512() -> Result<HString> { unsafe {
        <Self as RtActivatable<IAsymmetricAlgorithmNamesStatics>>::get_activation_factory().get_rsa_sign_pkcs1_sha512()
    }}
    #[inline] pub fn get_rsa_sign_pss_sha1() -> Result<HString> { unsafe {
        <Self as RtActivatable<IAsymmetricAlgorithmNamesStatics>>::get_activation_factory().get_rsa_sign_pss_sha1()
    }}
    #[inline] pub fn get_rsa_sign_pss_sha256() -> Result<HString> { unsafe {
        <Self as RtActivatable<IAsymmetricAlgorithmNamesStatics>>::get_activation_factory().get_rsa_sign_pss_sha256()
    }}
    #[inline] pub fn get_rsa_sign_pss_sha384() -> Result<HString> { unsafe {
        <Self as RtActivatable<IAsymmetricAlgorithmNamesStatics>>::get_activation_factory().get_rsa_sign_pss_sha384()
    }}
    #[inline] pub fn get_rsa_sign_pss_sha512() -> Result<HString> { unsafe {
        <Self as RtActivatable<IAsymmetricAlgorithmNamesStatics>>::get_activation_factory().get_rsa_sign_pss_sha512()
    }}
    #[inline] pub fn get_ecdsa_sha256() -> Result<HString> { unsafe {
        <Self as RtActivatable<IAsymmetricAlgorithmNamesStatics2>>::get_activation_factory().get_ecdsa_sha256()
    }}
    #[inline] pub fn get_ecdsa_sha384() -> Result<HString> { unsafe {
        <Self as RtActivatable<IAsymmetricAlgorithmNamesStatics2>>::get_activation_factory().get_ecdsa_sha384()
    }}
    #[inline] pub fn get_ecdsa_sha512() -> Result<HString> { unsafe {
        <Self as RtActivatable<IAsymmetricAlgorithmNamesStatics2>>::get_activation_factory().get_ecdsa_sha512()
    }}
}
DEFINE_CLSID!(AsymmetricAlgorithmNames(&[87,105,110,100,111,119,115,46,83,101,99,117,114,105,116,121,46,67,114,121,112,116,111,103,114,97,112,104,121,46,67,111,114,101,46,65,115,121,109,109,101,116,114,105,99,65,108,103,111,114,105,116,104,109,78,97,109,101,115,0]) [CLSID_AsymmetricAlgorithmNames]);
DEFINE_IID!(IID_IAsymmetricAlgorithmNamesStatics, 3405184228, 26560, 18090, 132, 249, 117, 46, 119, 68, 159, 155);
RT_INTERFACE!{static interface IAsymmetricAlgorithmNamesStatics(IAsymmetricAlgorithmNamesStaticsVtbl): IInspectable(IInspectableVtbl) [IID_IAsymmetricAlgorithmNamesStatics] {
    fn get_RsaPkcs1(&self, out: *mut HSTRING) -> HRESULT,
    fn get_RsaOaepSha1(&self, out: *mut HSTRING) -> HRESULT,
    fn get_RsaOaepSha256(&self, out: *mut HSTRING) -> HRESULT,
    fn get_RsaOaepSha384(&self, out: *mut HSTRING) -> HRESULT,
    fn get_RsaOaepSha512(&self, out: *mut HSTRING) -> HRESULT,
    fn get_EcdsaP256Sha256(&self, out: *mut HSTRING) -> HRESULT,
    fn get_EcdsaP384Sha384(&self, out: *mut HSTRING) -> HRESULT,
    fn get_EcdsaP521Sha512(&self, out: *mut HSTRING) -> HRESULT,
    fn get_DsaSha1(&self, out: *mut HSTRING) -> HRESULT,
    fn get_DsaSha256(&self, out: *mut HSTRING) -> HRESULT,
    fn get_RsaSignPkcs1Sha1(&self, out: *mut HSTRING) -> HRESULT,
    fn get_RsaSignPkcs1Sha256(&self, out: *mut HSTRING) -> HRESULT,
    fn get_RsaSignPkcs1Sha384(&self, out: *mut HSTRING) -> HRESULT,
    fn get_RsaSignPkcs1Sha512(&self, out: *mut HSTRING) -> HRESULT,
    fn get_RsaSignPssSha1(&self, out: *mut HSTRING) -> HRESULT,
    fn get_RsaSignPssSha256(&self, out: *mut HSTRING) -> HRESULT,
    fn get_RsaSignPssSha384(&self, out: *mut HSTRING) -> HRESULT,
    fn get_RsaSignPssSha512(&self, out: *mut HSTRING) -> HRESULT
}}
impl IAsymmetricAlgorithmNamesStatics {
    #[inline] pub unsafe fn get_rsa_pkcs1(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_RsaPkcs1)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_rsa_oaep_sha1(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_RsaOaepSha1)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_rsa_oaep_sha256(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_RsaOaepSha256)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_rsa_oaep_sha384(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_RsaOaepSha384)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_rsa_oaep_sha512(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_RsaOaepSha512)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_ecdsa_p256_sha256(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_EcdsaP256Sha256)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_ecdsa_p384_sha384(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_EcdsaP384Sha384)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_ecdsa_p521_sha512(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_EcdsaP521Sha512)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_dsa_sha1(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_DsaSha1)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_dsa_sha256(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_DsaSha256)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_rsa_sign_pkcs1_sha1(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_RsaSignPkcs1Sha1)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_rsa_sign_pkcs1_sha256(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_RsaSignPkcs1Sha256)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_rsa_sign_pkcs1_sha384(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_RsaSignPkcs1Sha384)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_rsa_sign_pkcs1_sha512(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_RsaSignPkcs1Sha512)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_rsa_sign_pss_sha1(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_RsaSignPssSha1)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_rsa_sign_pss_sha256(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_RsaSignPssSha256)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_rsa_sign_pss_sha384(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_RsaSignPssSha384)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_rsa_sign_pss_sha512(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_RsaSignPssSha512)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
}
DEFINE_IID!(IID_IAsymmetricAlgorithmNamesStatics2, 4047618262, 19455, 20259, 186, 102, 96, 69, 177, 55, 213, 223);
RT_INTERFACE!{static interface IAsymmetricAlgorithmNamesStatics2(IAsymmetricAlgorithmNamesStatics2Vtbl): IInspectable(IInspectableVtbl) [IID_IAsymmetricAlgorithmNamesStatics2] {
    fn get_EcdsaSha256(&self, out: *mut HSTRING) -> HRESULT,
    fn get_EcdsaSha384(&self, out: *mut HSTRING) -> HRESULT,
    fn get_EcdsaSha512(&self, out: *mut HSTRING) -> HRESULT
}}
impl IAsymmetricAlgorithmNamesStatics2 {
    #[inline] pub unsafe fn get_ecdsa_sha256(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_EcdsaSha256)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_ecdsa_sha384(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_EcdsaSha384)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_ecdsa_sha512(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_EcdsaSha512)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
}
DEFINE_IID!(IID_IAsymmetricKeyAlgorithmProvider, 3906142007, 25177, 20104, 183, 224, 148, 25, 31, 222, 105, 158);
RT_INTERFACE!{interface IAsymmetricKeyAlgorithmProvider(IAsymmetricKeyAlgorithmProviderVtbl): IInspectable(IInspectableVtbl) [IID_IAsymmetricKeyAlgorithmProvider] {
    fn get_AlgorithmName(&self, out: *mut HSTRING) -> HRESULT,
    fn CreateKeyPair(&self, keySize: u32, out: *mut *mut CryptographicKey) -> HRESULT,
    #[cfg(feature="windows-storage")] fn ImportDefaultPrivateKeyBlob(&self, keyBlob: *mut ::rt::gen::windows::storage::streams::IBuffer, out: *mut *mut CryptographicKey) -> HRESULT,
    #[cfg(feature="windows-storage")] fn ImportKeyPairWithBlobType(&self, keyBlob: *mut ::rt::gen::windows::storage::streams::IBuffer, blobType: CryptographicPrivateKeyBlobType, out: *mut *mut CryptographicKey) -> HRESULT,
    #[cfg(feature="windows-storage")] fn ImportDefaultPublicKeyBlob(&self, keyBlob: *mut ::rt::gen::windows::storage::streams::IBuffer, out: *mut *mut CryptographicKey) -> HRESULT,
    #[cfg(feature="windows-storage")] fn ImportPublicKeyWithBlobType(&self, keyBlob: *mut ::rt::gen::windows::storage::streams::IBuffer, blobType: CryptographicPublicKeyBlobType, out: *mut *mut CryptographicKey) -> HRESULT
}}
impl IAsymmetricKeyAlgorithmProvider {
    #[inline] pub unsafe fn get_algorithm_name(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_AlgorithmName)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn create_key_pair(&self, keySize: u32) -> Result<ComPtr<CryptographicKey>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).CreateKeyPair)(self as *const _ as *mut _, keySize, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[cfg(feature="windows-storage")] #[inline] pub unsafe fn import_default_private_key_blob(&self, keyBlob: &::rt::gen::windows::storage::streams::IBuffer) -> Result<ComPtr<CryptographicKey>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).ImportDefaultPrivateKeyBlob)(self as *const _ as *mut _, keyBlob as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[cfg(feature="windows-storage")] #[inline] pub unsafe fn import_key_pair_with_blob_type(&self, keyBlob: &::rt::gen::windows::storage::streams::IBuffer, blobType: CryptographicPrivateKeyBlobType) -> Result<ComPtr<CryptographicKey>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).ImportKeyPairWithBlobType)(self as *const _ as *mut _, keyBlob as *const _ as *mut _, blobType, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[cfg(feature="windows-storage")] #[inline] pub unsafe fn import_default_public_key_blob(&self, keyBlob: &::rt::gen::windows::storage::streams::IBuffer) -> Result<ComPtr<CryptographicKey>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).ImportDefaultPublicKeyBlob)(self as *const _ as *mut _, keyBlob as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[cfg(feature="windows-storage")] #[inline] pub unsafe fn import_public_key_with_blob_type(&self, keyBlob: &::rt::gen::windows::storage::streams::IBuffer, blobType: CryptographicPublicKeyBlobType) -> Result<ComPtr<CryptographicKey>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).ImportPublicKeyWithBlobType)(self as *const _ as *mut _, keyBlob as *const _ as *mut _, blobType, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
RT_CLASS!{class AsymmetricKeyAlgorithmProvider: IAsymmetricKeyAlgorithmProvider}
impl RtActivatable<IAsymmetricKeyAlgorithmProviderStatics> for AsymmetricKeyAlgorithmProvider {}
impl AsymmetricKeyAlgorithmProvider {
    #[inline] pub fn open_algorithm(algorithm: &HStringArg) -> Result<ComPtr<AsymmetricKeyAlgorithmProvider>> { unsafe {
        <Self as RtActivatable<IAsymmetricKeyAlgorithmProviderStatics>>::get_activation_factory().open_algorithm(algorithm)
    }}
}
DEFINE_CLSID!(AsymmetricKeyAlgorithmProvider(&[87,105,110,100,111,119,115,46,83,101,99,117,114,105,116,121,46,67,114,121,112,116,111,103,114,97,112,104,121,46,67,111,114,101,46,65,115,121,109,109,101,116,114,105,99,75,101,121,65,108,103,111,114,105,116,104,109,80,114,111,118,105,100,101,114,0]) [CLSID_AsymmetricKeyAlgorithmProvider]);
DEFINE_IID!(IID_IAsymmetricKeyAlgorithmProvider2, 1311910526, 31821, 18839, 172, 79, 27, 132, 139, 54, 48, 110);
RT_INTERFACE!{interface IAsymmetricKeyAlgorithmProvider2(IAsymmetricKeyAlgorithmProvider2Vtbl): IInspectable(IInspectableVtbl) [IID_IAsymmetricKeyAlgorithmProvider2] {
    fn CreateKeyPairWithCurveName(&self, curveName: HSTRING, out: *mut *mut CryptographicKey) -> HRESULT,
    fn CreateKeyPairWithCurveParameters(&self, parametersSize: u32, parameters: *mut u8, out: *mut *mut CryptographicKey) -> HRESULT
}}
impl IAsymmetricKeyAlgorithmProvider2 {
    #[inline] pub unsafe fn create_key_pair_with_curve_name(&self, curveName: &HStringArg) -> Result<ComPtr<CryptographicKey>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).CreateKeyPairWithCurveName)(self as *const _ as *mut _, curveName.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn create_key_pair_with_curve_parameters(&self, parameters: &[u8]) -> Result<ComPtr<CryptographicKey>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).CreateKeyPairWithCurveParameters)(self as *const _ as *mut _, parameters.len() as u32, parameters.as_ptr() as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
DEFINE_IID!(IID_IAsymmetricKeyAlgorithmProviderStatics, 1113316888, 42995, 18342, 168, 210, 196, 141, 96, 51, 166, 92);
RT_INTERFACE!{static interface IAsymmetricKeyAlgorithmProviderStatics(IAsymmetricKeyAlgorithmProviderStaticsVtbl): IInspectable(IInspectableVtbl) [IID_IAsymmetricKeyAlgorithmProviderStatics] {
    fn OpenAlgorithm(&self, algorithm: HSTRING, out: *mut *mut AsymmetricKeyAlgorithmProvider) -> HRESULT
}}
impl IAsymmetricKeyAlgorithmProviderStatics {
    #[inline] pub unsafe fn open_algorithm(&self, algorithm: &HStringArg) -> Result<ComPtr<AsymmetricKeyAlgorithmProvider>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).OpenAlgorithm)(self as *const _ as *mut _, algorithm.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
RT_ENUM! { enum Capi1KdfTargetAlgorithm: i32 {
    NotAes (Capi1KdfTargetAlgorithm_NotAes) = 0, Aes (Capi1KdfTargetAlgorithm_Aes) = 1,
}}
RT_CLASS!{static class CryptographicEngine}
impl RtActivatable<ICryptographicEngineStatics> for CryptographicEngine {}
impl RtActivatable<ICryptographicEngineStatics2> for CryptographicEngine {}
impl CryptographicEngine {
    #[cfg(feature="windows-storage")] #[inline] pub fn encrypt(key: &CryptographicKey, data: &::rt::gen::windows::storage::streams::IBuffer, iv: &::rt::gen::windows::storage::streams::IBuffer) -> Result<ComPtr<::rt::gen::windows::storage::streams::IBuffer>> { unsafe {
        <Self as RtActivatable<ICryptographicEngineStatics>>::get_activation_factory().encrypt(key, data, iv)
    }}
    #[cfg(feature="windows-storage")] #[inline] pub fn decrypt(key: &CryptographicKey, data: &::rt::gen::windows::storage::streams::IBuffer, iv: &::rt::gen::windows::storage::streams::IBuffer) -> Result<ComPtr<::rt::gen::windows::storage::streams::IBuffer>> { unsafe {
        <Self as RtActivatable<ICryptographicEngineStatics>>::get_activation_factory().decrypt(key, data, iv)
    }}
    #[cfg(feature="windows-storage")] #[inline] pub fn encrypt_and_authenticate(key: &CryptographicKey, data: &::rt::gen::windows::storage::streams::IBuffer, nonce: &::rt::gen::windows::storage::streams::IBuffer, authenticatedData: &::rt::gen::windows::storage::streams::IBuffer) -> Result<ComPtr<EncryptedAndAuthenticatedData>> { unsafe {
        <Self as RtActivatable<ICryptographicEngineStatics>>::get_activation_factory().encrypt_and_authenticate(key, data, nonce, authenticatedData)
    }}
    #[cfg(feature="windows-storage")] #[inline] pub fn decrypt_and_authenticate(key: &CryptographicKey, data: &::rt::gen::windows::storage::streams::IBuffer, nonce: &::rt::gen::windows::storage::streams::IBuffer, authenticationTag: &::rt::gen::windows::storage::streams::IBuffer, authenticatedData: &::rt::gen::windows::storage::streams::IBuffer) -> Result<ComPtr<::rt::gen::windows::storage::streams::IBuffer>> { unsafe {
        <Self as RtActivatable<ICryptographicEngineStatics>>::get_activation_factory().decrypt_and_authenticate(key, data, nonce, authenticationTag, authenticatedData)
    }}
    #[cfg(feature="windows-storage")] #[inline] pub fn sign(key: &CryptographicKey, data: &::rt::gen::windows::storage::streams::IBuffer) -> Result<ComPtr<::rt::gen::windows::storage::streams::IBuffer>> { unsafe {
        <Self as RtActivatable<ICryptographicEngineStatics>>::get_activation_factory().sign(key, data)
    }}
    #[cfg(feature="windows-storage")] #[inline] pub fn verify_signature(key: &CryptographicKey, data: &::rt::gen::windows::storage::streams::IBuffer, signature: &::rt::gen::windows::storage::streams::IBuffer) -> Result<bool> { unsafe {
        <Self as RtActivatable<ICryptographicEngineStatics>>::get_activation_factory().verify_signature(key, data, signature)
    }}
    #[cfg(feature="windows-storage")] #[inline] pub fn derive_key_material(key: &CryptographicKey, parameters: &KeyDerivationParameters, desiredKeySize: u32) -> Result<ComPtr<::rt::gen::windows::storage::streams::IBuffer>> { unsafe {
        <Self as RtActivatable<ICryptographicEngineStatics>>::get_activation_factory().derive_key_material(key, parameters, desiredKeySize)
    }}
    #[cfg(feature="windows-storage")] #[inline] pub fn sign_hashed_data(key: &CryptographicKey, data: &::rt::gen::windows::storage::streams::IBuffer) -> Result<ComPtr<::rt::gen::windows::storage::streams::IBuffer>> { unsafe {
        <Self as RtActivatable<ICryptographicEngineStatics2>>::get_activation_factory().sign_hashed_data(key, data)
    }}
    #[cfg(feature="windows-storage")] #[inline] pub fn verify_signature_with_hash_input(key: &CryptographicKey, data: &::rt::gen::windows::storage::streams::IBuffer, signature: &::rt::gen::windows::storage::streams::IBuffer) -> Result<bool> { unsafe {
        <Self as RtActivatable<ICryptographicEngineStatics2>>::get_activation_factory().verify_signature_with_hash_input(key, data, signature)
    }}
    #[cfg(feature="windows-storage")] #[inline] pub fn decrypt_async(key: &CryptographicKey, data: &::rt::gen::windows::storage::streams::IBuffer, iv: &::rt::gen::windows::storage::streams::IBuffer) -> Result<ComPtr<::rt::gen::windows::foundation::IAsyncOperation<::rt::gen::windows::storage::streams::IBuffer>>> { unsafe {
        <Self as RtActivatable<ICryptographicEngineStatics2>>::get_activation_factory().decrypt_async(key, data, iv)
    }}
    #[cfg(feature="windows-storage")] #[inline] pub fn sign_async(key: &CryptographicKey, data: &::rt::gen::windows::storage::streams::IBuffer) -> Result<ComPtr<::rt::gen::windows::foundation::IAsyncOperation<::rt::gen::windows::storage::streams::IBuffer>>> { unsafe {
        <Self as RtActivatable<ICryptographicEngineStatics2>>::get_activation_factory().sign_async(key, data)
    }}
    #[cfg(feature="windows-storage")] #[inline] pub fn sign_hashed_data_async(key: &CryptographicKey, data: &::rt::gen::windows::storage::streams::IBuffer) -> Result<ComPtr<::rt::gen::windows::foundation::IAsyncOperation<::rt::gen::windows::storage::streams::IBuffer>>> { unsafe {
        <Self as RtActivatable<ICryptographicEngineStatics2>>::get_activation_factory().sign_hashed_data_async(key, data)
    }}
}
DEFINE_CLSID!(CryptographicEngine(&[87,105,110,100,111,119,115,46,83,101,99,117,114,105,116,121,46,67,114,121,112,116,111,103,114,97,112,104,121,46,67,111,114,101,46,67,114,121,112,116,111,103,114,97,112,104,105,99,69,110,103,105,110,101,0]) [CLSID_CryptographicEngine]);
DEFINE_IID!(IID_ICryptographicEngineStatics, 2682914361, 28663, 19589, 160, 149, 149, 235, 49, 113, 94, 185);
RT_INTERFACE!{static interface ICryptographicEngineStatics(ICryptographicEngineStaticsVtbl): IInspectable(IInspectableVtbl) [IID_ICryptographicEngineStatics] {
    #[cfg(feature="windows-storage")] fn Encrypt(&self, key: *mut CryptographicKey, data: *mut ::rt::gen::windows::storage::streams::IBuffer, iv: *mut ::rt::gen::windows::storage::streams::IBuffer, out: *mut *mut ::rt::gen::windows::storage::streams::IBuffer) -> HRESULT,
    #[cfg(feature="windows-storage")] fn Decrypt(&self, key: *mut CryptographicKey, data: *mut ::rt::gen::windows::storage::streams::IBuffer, iv: *mut ::rt::gen::windows::storage::streams::IBuffer, out: *mut *mut ::rt::gen::windows::storage::streams::IBuffer) -> HRESULT,
    #[cfg(feature="windows-storage")] fn EncryptAndAuthenticate(&self, key: *mut CryptographicKey, data: *mut ::rt::gen::windows::storage::streams::IBuffer, nonce: *mut ::rt::gen::windows::storage::streams::IBuffer, authenticatedData: *mut ::rt::gen::windows::storage::streams::IBuffer, out: *mut *mut EncryptedAndAuthenticatedData) -> HRESULT,
    #[cfg(feature="windows-storage")] fn DecryptAndAuthenticate(&self, key: *mut CryptographicKey, data: *mut ::rt::gen::windows::storage::streams::IBuffer, nonce: *mut ::rt::gen::windows::storage::streams::IBuffer, authenticationTag: *mut ::rt::gen::windows::storage::streams::IBuffer, authenticatedData: *mut ::rt::gen::windows::storage::streams::IBuffer, out: *mut *mut ::rt::gen::windows::storage::streams::IBuffer) -> HRESULT,
    #[cfg(feature="windows-storage")] fn Sign(&self, key: *mut CryptographicKey, data: *mut ::rt::gen::windows::storage::streams::IBuffer, out: *mut *mut ::rt::gen::windows::storage::streams::IBuffer) -> HRESULT,
    #[cfg(feature="windows-storage")] fn VerifySignature(&self, key: *mut CryptographicKey, data: *mut ::rt::gen::windows::storage::streams::IBuffer, signature: *mut ::rt::gen::windows::storage::streams::IBuffer, out: *mut bool) -> HRESULT,
    #[cfg(feature="windows-storage")] fn DeriveKeyMaterial(&self, key: *mut CryptographicKey, parameters: *mut KeyDerivationParameters, desiredKeySize: u32, out: *mut *mut ::rt::gen::windows::storage::streams::IBuffer) -> HRESULT
}}
impl ICryptographicEngineStatics {
    #[cfg(feature="windows-storage")] #[inline] pub unsafe fn encrypt(&self, key: &CryptographicKey, data: &::rt::gen::windows::storage::streams::IBuffer, iv: &::rt::gen::windows::storage::streams::IBuffer) -> Result<ComPtr<::rt::gen::windows::storage::streams::IBuffer>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).Encrypt)(self as *const _ as *mut _, key as *const _ as *mut _, data as *const _ as *mut _, iv as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[cfg(feature="windows-storage")] #[inline] pub unsafe fn decrypt(&self, key: &CryptographicKey, data: &::rt::gen::windows::storage::streams::IBuffer, iv: &::rt::gen::windows::storage::streams::IBuffer) -> Result<ComPtr<::rt::gen::windows::storage::streams::IBuffer>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).Decrypt)(self as *const _ as *mut _, key as *const _ as *mut _, data as *const _ as *mut _, iv as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[cfg(feature="windows-storage")] #[inline] pub unsafe fn encrypt_and_authenticate(&self, key: &CryptographicKey, data: &::rt::gen::windows::storage::streams::IBuffer, nonce: &::rt::gen::windows::storage::streams::IBuffer, authenticatedData: &::rt::gen::windows::storage::streams::IBuffer) -> Result<ComPtr<EncryptedAndAuthenticatedData>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).EncryptAndAuthenticate)(self as *const _ as *mut _, key as *const _ as *mut _, data as *const _ as *mut _, nonce as *const _ as *mut _, authenticatedData as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[cfg(feature="windows-storage")] #[inline] pub unsafe fn decrypt_and_authenticate(&self, key: &CryptographicKey, data: &::rt::gen::windows::storage::streams::IBuffer, nonce: &::rt::gen::windows::storage::streams::IBuffer, authenticationTag: &::rt::gen::windows::storage::streams::IBuffer, authenticatedData: &::rt::gen::windows::storage::streams::IBuffer) -> Result<ComPtr<::rt::gen::windows::storage::streams::IBuffer>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).DecryptAndAuthenticate)(self as *const _ as *mut _, key as *const _ as *mut _, data as *const _ as *mut _, nonce as *const _ as *mut _, authenticationTag as *const _ as *mut _, authenticatedData as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[cfg(feature="windows-storage")] #[inline] pub unsafe fn sign(&self, key: &CryptographicKey, data: &::rt::gen::windows::storage::streams::IBuffer) -> Result<ComPtr<::rt::gen::windows::storage::streams::IBuffer>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).Sign)(self as *const _ as *mut _, key as *const _ as *mut _, data as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[cfg(feature="windows-storage")] #[inline] pub unsafe fn verify_signature(&self, key: &CryptographicKey, data: &::rt::gen::windows::storage::streams::IBuffer, signature: &::rt::gen::windows::storage::streams::IBuffer) -> Result<bool> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).VerifySignature)(self as *const _ as *mut _, key as *const _ as *mut _, data as *const _ as *mut _, signature as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[cfg(feature="windows-storage")] #[inline] pub unsafe fn derive_key_material(&self, key: &CryptographicKey, parameters: &KeyDerivationParameters, desiredKeySize: u32) -> Result<ComPtr<::rt::gen::windows::storage::streams::IBuffer>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).DeriveKeyMaterial)(self as *const _ as *mut _, key as *const _ as *mut _, parameters as *const _ as *mut _, desiredKeySize, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
DEFINE_IID!(IID_ICryptographicEngineStatics2, 1733904638, 57247, 16785, 146, 199, 108, 230, 245, 132, 32, 224);
RT_INTERFACE!{static interface ICryptographicEngineStatics2(ICryptographicEngineStatics2Vtbl): IInspectable(IInspectableVtbl) [IID_ICryptographicEngineStatics2] {
    #[cfg(feature="windows-storage")] fn SignHashedData(&self, key: *mut CryptographicKey, data: *mut ::rt::gen::windows::storage::streams::IBuffer, out: *mut *mut ::rt::gen::windows::storage::streams::IBuffer) -> HRESULT,
    #[cfg(feature="windows-storage")] fn VerifySignatureWithHashInput(&self, key: *mut CryptographicKey, data: *mut ::rt::gen::windows::storage::streams::IBuffer, signature: *mut ::rt::gen::windows::storage::streams::IBuffer, out: *mut bool) -> HRESULT,
    #[cfg(feature="windows-storage")] fn DecryptAsync(&self, key: *mut CryptographicKey, data: *mut ::rt::gen::windows::storage::streams::IBuffer, iv: *mut ::rt::gen::windows::storage::streams::IBuffer, out: *mut *mut ::rt::gen::windows::foundation::IAsyncOperation<::rt::gen::windows::storage::streams::IBuffer>) -> HRESULT,
    #[cfg(feature="windows-storage")] fn SignAsync(&self, key: *mut CryptographicKey, data: *mut ::rt::gen::windows::storage::streams::IBuffer, out: *mut *mut ::rt::gen::windows::foundation::IAsyncOperation<::rt::gen::windows::storage::streams::IBuffer>) -> HRESULT,
    #[cfg(feature="windows-storage")] fn SignHashedDataAsync(&self, key: *mut CryptographicKey, data: *mut ::rt::gen::windows::storage::streams::IBuffer, out: *mut *mut ::rt::gen::windows::foundation::IAsyncOperation<::rt::gen::windows::storage::streams::IBuffer>) -> HRESULT
}}
impl ICryptographicEngineStatics2 {
    #[cfg(feature="windows-storage")] #[inline] pub unsafe fn sign_hashed_data(&self, key: &CryptographicKey, data: &::rt::gen::windows::storage::streams::IBuffer) -> Result<ComPtr<::rt::gen::windows::storage::streams::IBuffer>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).SignHashedData)(self as *const _ as *mut _, key as *const _ as *mut _, data as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[cfg(feature="windows-storage")] #[inline] pub unsafe fn verify_signature_with_hash_input(&self, key: &CryptographicKey, data: &::rt::gen::windows::storage::streams::IBuffer, signature: &::rt::gen::windows::storage::streams::IBuffer) -> Result<bool> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).VerifySignatureWithHashInput)(self as *const _ as *mut _, key as *const _ as *mut _, data as *const _ as *mut _, signature as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[cfg(feature="windows-storage")] #[inline] pub unsafe fn decrypt_async(&self, key: &CryptographicKey, data: &::rt::gen::windows::storage::streams::IBuffer, iv: &::rt::gen::windows::storage::streams::IBuffer) -> Result<ComPtr<::rt::gen::windows::foundation::IAsyncOperation<::rt::gen::windows::storage::streams::IBuffer>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).DecryptAsync)(self as *const _ as *mut _, key as *const _ as *mut _, data as *const _ as *mut _, iv as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[cfg(feature="windows-storage")] #[inline] pub unsafe fn sign_async(&self, key: &CryptographicKey, data: &::rt::gen::windows::storage::streams::IBuffer) -> Result<ComPtr<::rt::gen::windows::foundation::IAsyncOperation<::rt::gen::windows::storage::streams::IBuffer>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).SignAsync)(self as *const _ as *mut _, key as *const _ as *mut _, data as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[cfg(feature="windows-storage")] #[inline] pub unsafe fn sign_hashed_data_async(&self, key: &CryptographicKey, data: &::rt::gen::windows::storage::streams::IBuffer) -> Result<ComPtr<::rt::gen::windows::foundation::IAsyncOperation<::rt::gen::windows::storage::streams::IBuffer>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).SignHashedDataAsync)(self as *const _ as *mut _, key as *const _ as *mut _, data as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
RT_CLASS!{class CryptographicHash: IHashComputation}
DEFINE_IID!(IID_ICryptographicKey, 3978967920, 36475, 16393, 132, 1, 255, 209, 166, 46, 235, 39);
RT_INTERFACE!{interface ICryptographicKey(ICryptographicKeyVtbl): IInspectable(IInspectableVtbl) [IID_ICryptographicKey] {
    fn get_KeySize(&self, out: *mut u32) -> HRESULT,
    #[cfg(feature="windows-storage")] fn ExportDefaultPrivateKeyBlobType(&self, out: *mut *mut ::rt::gen::windows::storage::streams::IBuffer) -> HRESULT,
    #[cfg(feature="windows-storage")] fn ExportPrivateKeyWithBlobType(&self, blobType: CryptographicPrivateKeyBlobType, out: *mut *mut ::rt::gen::windows::storage::streams::IBuffer) -> HRESULT,
    #[cfg(feature="windows-storage")] fn ExportDefaultPublicKeyBlobType(&self, out: *mut *mut ::rt::gen::windows::storage::streams::IBuffer) -> HRESULT,
    #[cfg(feature="windows-storage")] fn ExportPublicKeyWithBlobType(&self, blobType: CryptographicPublicKeyBlobType, out: *mut *mut ::rt::gen::windows::storage::streams::IBuffer) -> HRESULT
}}
impl ICryptographicKey {
    #[inline] pub unsafe fn get_key_size(&self) -> Result<u32> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_KeySize)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[cfg(feature="windows-storage")] #[inline] pub unsafe fn export_default_private_key_blob_type(&self) -> Result<ComPtr<::rt::gen::windows::storage::streams::IBuffer>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).ExportDefaultPrivateKeyBlobType)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[cfg(feature="windows-storage")] #[inline] pub unsafe fn export_private_key_with_blob_type(&self, blobType: CryptographicPrivateKeyBlobType) -> Result<ComPtr<::rt::gen::windows::storage::streams::IBuffer>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).ExportPrivateKeyWithBlobType)(self as *const _ as *mut _, blobType, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[cfg(feature="windows-storage")] #[inline] pub unsafe fn export_default_public_key_blob_type(&self) -> Result<ComPtr<::rt::gen::windows::storage::streams::IBuffer>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).ExportDefaultPublicKeyBlobType)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[cfg(feature="windows-storage")] #[inline] pub unsafe fn export_public_key_with_blob_type(&self, blobType: CryptographicPublicKeyBlobType) -> Result<ComPtr<::rt::gen::windows::storage::streams::IBuffer>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).ExportPublicKeyWithBlobType)(self as *const _ as *mut _, blobType, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
RT_CLASS!{class CryptographicKey: ICryptographicKey}
RT_ENUM! { enum CryptographicPadding: i32 {
    None (CryptographicPadding_None) = 0, RsaOaep (CryptographicPadding_RsaOaep) = 1, RsaPkcs1V15 (CryptographicPadding_RsaPkcs1V15) = 2, RsaPss (CryptographicPadding_RsaPss) = 3,
}}
RT_ENUM! { enum CryptographicPrivateKeyBlobType: i32 {
    Pkcs8RawPrivateKeyInfo (CryptographicPrivateKeyBlobType_Pkcs8RawPrivateKeyInfo) = 0, Pkcs1RsaPrivateKey (CryptographicPrivateKeyBlobType_Pkcs1RsaPrivateKey) = 1, BCryptPrivateKey (CryptographicPrivateKeyBlobType_BCryptPrivateKey) = 2, Capi1PrivateKey (CryptographicPrivateKeyBlobType_Capi1PrivateKey) = 3, BCryptEccFullPrivateKey (CryptographicPrivateKeyBlobType_BCryptEccFullPrivateKey) = 4,
}}
RT_ENUM! { enum CryptographicPublicKeyBlobType: i32 {
    X509SubjectPublicKeyInfo (CryptographicPublicKeyBlobType_X509SubjectPublicKeyInfo) = 0, Pkcs1RsaPublicKey (CryptographicPublicKeyBlobType_Pkcs1RsaPublicKey) = 1, BCryptPublicKey (CryptographicPublicKeyBlobType_BCryptPublicKey) = 2, Capi1PublicKey (CryptographicPublicKeyBlobType_Capi1PublicKey) = 3, BCryptEccFullPublicKey (CryptographicPublicKeyBlobType_BCryptEccFullPublicKey) = 4,
}}
RT_CLASS!{static class EccCurveNames}
impl RtActivatable<IEccCurveNamesStatics> for EccCurveNames {}
impl EccCurveNames {
    #[inline] pub fn get_brainpool_p160r1() -> Result<HString> { unsafe {
        <Self as RtActivatable<IEccCurveNamesStatics>>::get_activation_factory().get_brainpool_p160r1()
    }}
    #[inline] pub fn get_brainpool_p160t1() -> Result<HString> { unsafe {
        <Self as RtActivatable<IEccCurveNamesStatics>>::get_activation_factory().get_brainpool_p160t1()
    }}
    #[inline] pub fn get_brainpool_p192r1() -> Result<HString> { unsafe {
        <Self as RtActivatable<IEccCurveNamesStatics>>::get_activation_factory().get_brainpool_p192r1()
    }}
    #[inline] pub fn get_brainpool_p192t1() -> Result<HString> { unsafe {
        <Self as RtActivatable<IEccCurveNamesStatics>>::get_activation_factory().get_brainpool_p192t1()
    }}
    #[inline] pub fn get_brainpool_p224r1() -> Result<HString> { unsafe {
        <Self as RtActivatable<IEccCurveNamesStatics>>::get_activation_factory().get_brainpool_p224r1()
    }}
    #[inline] pub fn get_brainpool_p224t1() -> Result<HString> { unsafe {
        <Self as RtActivatable<IEccCurveNamesStatics>>::get_activation_factory().get_brainpool_p224t1()
    }}
    #[inline] pub fn get_brainpool_p256r1() -> Result<HString> { unsafe {
        <Self as RtActivatable<IEccCurveNamesStatics>>::get_activation_factory().get_brainpool_p256r1()
    }}
    #[inline] pub fn get_brainpool_p256t1() -> Result<HString> { unsafe {
        <Self as RtActivatable<IEccCurveNamesStatics>>::get_activation_factory().get_brainpool_p256t1()
    }}
    #[inline] pub fn get_brainpool_p320r1() -> Result<HString> { unsafe {
        <Self as RtActivatable<IEccCurveNamesStatics>>::get_activation_factory().get_brainpool_p320r1()
    }}
    #[inline] pub fn get_brainpool_p320t1() -> Result<HString> { unsafe {
        <Self as RtActivatable<IEccCurveNamesStatics>>::get_activation_factory().get_brainpool_p320t1()
    }}
    #[inline] pub fn get_brainpool_p384r1() -> Result<HString> { unsafe {
        <Self as RtActivatable<IEccCurveNamesStatics>>::get_activation_factory().get_brainpool_p384r1()
    }}
    #[inline] pub fn get_brainpool_p384t1() -> Result<HString> { unsafe {
        <Self as RtActivatable<IEccCurveNamesStatics>>::get_activation_factory().get_brainpool_p384t1()
    }}
    #[inline] pub fn get_brainpool_p512r1() -> Result<HString> { unsafe {
        <Self as RtActivatable<IEccCurveNamesStatics>>::get_activation_factory().get_brainpool_p512r1()
    }}
    #[inline] pub fn get_brainpool_p512t1() -> Result<HString> { unsafe {
        <Self as RtActivatable<IEccCurveNamesStatics>>::get_activation_factory().get_brainpool_p512t1()
    }}
    #[inline] pub fn get_curve25519() -> Result<HString> { unsafe {
        <Self as RtActivatable<IEccCurveNamesStatics>>::get_activation_factory().get_curve25519()
    }}
    #[inline] pub fn get_ec192wapi() -> Result<HString> { unsafe {
        <Self as RtActivatable<IEccCurveNamesStatics>>::get_activation_factory().get_ec192wapi()
    }}
    #[inline] pub fn get_nist_p192() -> Result<HString> { unsafe {
        <Self as RtActivatable<IEccCurveNamesStatics>>::get_activation_factory().get_nist_p192()
    }}
    #[inline] pub fn get_nist_p224() -> Result<HString> { unsafe {
        <Self as RtActivatable<IEccCurveNamesStatics>>::get_activation_factory().get_nist_p224()
    }}
    #[inline] pub fn get_nist_p256() -> Result<HString> { unsafe {
        <Self as RtActivatable<IEccCurveNamesStatics>>::get_activation_factory().get_nist_p256()
    }}
    #[inline] pub fn get_nist_p384() -> Result<HString> { unsafe {
        <Self as RtActivatable<IEccCurveNamesStatics>>::get_activation_factory().get_nist_p384()
    }}
    #[inline] pub fn get_nist_p521() -> Result<HString> { unsafe {
        <Self as RtActivatable<IEccCurveNamesStatics>>::get_activation_factory().get_nist_p521()
    }}
    #[inline] pub fn get_nums_p256t1() -> Result<HString> { unsafe {
        <Self as RtActivatable<IEccCurveNamesStatics>>::get_activation_factory().get_nums_p256t1()
    }}
    #[inline] pub fn get_nums_p384t1() -> Result<HString> { unsafe {
        <Self as RtActivatable<IEccCurveNamesStatics>>::get_activation_factory().get_nums_p384t1()
    }}
    #[inline] pub fn get_nums_p512t1() -> Result<HString> { unsafe {
        <Self as RtActivatable<IEccCurveNamesStatics>>::get_activation_factory().get_nums_p512t1()
    }}
    #[inline] pub fn get_sec_p160k1() -> Result<HString> { unsafe {
        <Self as RtActivatable<IEccCurveNamesStatics>>::get_activation_factory().get_sec_p160k1()
    }}
    #[inline] pub fn get_sec_p160r1() -> Result<HString> { unsafe {
        <Self as RtActivatable<IEccCurveNamesStatics>>::get_activation_factory().get_sec_p160r1()
    }}
    #[inline] pub fn get_sec_p160r2() -> Result<HString> { unsafe {
        <Self as RtActivatable<IEccCurveNamesStatics>>::get_activation_factory().get_sec_p160r2()
    }}
    #[inline] pub fn get_sec_p192k1() -> Result<HString> { unsafe {
        <Self as RtActivatable<IEccCurveNamesStatics>>::get_activation_factory().get_sec_p192k1()
    }}
    #[inline] pub fn get_sec_p192r1() -> Result<HString> { unsafe {
        <Self as RtActivatable<IEccCurveNamesStatics>>::get_activation_factory().get_sec_p192r1()
    }}
    #[inline] pub fn get_sec_p224k1() -> Result<HString> { unsafe {
        <Self as RtActivatable<IEccCurveNamesStatics>>::get_activation_factory().get_sec_p224k1()
    }}
    #[inline] pub fn get_sec_p224r1() -> Result<HString> { unsafe {
        <Self as RtActivatable<IEccCurveNamesStatics>>::get_activation_factory().get_sec_p224r1()
    }}
    #[inline] pub fn get_sec_p256k1() -> Result<HString> { unsafe {
        <Self as RtActivatable<IEccCurveNamesStatics>>::get_activation_factory().get_sec_p256k1()
    }}
    #[inline] pub fn get_sec_p256r1() -> Result<HString> { unsafe {
        <Self as RtActivatable<IEccCurveNamesStatics>>::get_activation_factory().get_sec_p256r1()
    }}
    #[inline] pub fn get_sec_p384r1() -> Result<HString> { unsafe {
        <Self as RtActivatable<IEccCurveNamesStatics>>::get_activation_factory().get_sec_p384r1()
    }}
    #[inline] pub fn get_sec_p521r1() -> Result<HString> { unsafe {
        <Self as RtActivatable<IEccCurveNamesStatics>>::get_activation_factory().get_sec_p521r1()
    }}
    #[inline] pub fn get_wtls7() -> Result<HString> { unsafe {
        <Self as RtActivatable<IEccCurveNamesStatics>>::get_activation_factory().get_wtls7()
    }}
    #[inline] pub fn get_wtls9() -> Result<HString> { unsafe {
        <Self as RtActivatable<IEccCurveNamesStatics>>::get_activation_factory().get_wtls9()
    }}
    #[inline] pub fn get_wtls12() -> Result<HString> { unsafe {
        <Self as RtActivatable<IEccCurveNamesStatics>>::get_activation_factory().get_wtls12()
    }}
    #[inline] pub fn get_x962_p192v1() -> Result<HString> { unsafe {
        <Self as RtActivatable<IEccCurveNamesStatics>>::get_activation_factory().get_x962_p192v1()
    }}
    #[inline] pub fn get_x962_p192v2() -> Result<HString> { unsafe {
        <Self as RtActivatable<IEccCurveNamesStatics>>::get_activation_factory().get_x962_p192v2()
    }}
    #[inline] pub fn get_x962_p192v3() -> Result<HString> { unsafe {
        <Self as RtActivatable<IEccCurveNamesStatics>>::get_activation_factory().get_x962_p192v3()
    }}
    #[inline] pub fn get_x962_p239v1() -> Result<HString> { unsafe {
        <Self as RtActivatable<IEccCurveNamesStatics>>::get_activation_factory().get_x962_p239v1()
    }}
    #[inline] pub fn get_x962_p239v2() -> Result<HString> { unsafe {
        <Self as RtActivatable<IEccCurveNamesStatics>>::get_activation_factory().get_x962_p239v2()
    }}
    #[inline] pub fn get_x962_p239v3() -> Result<HString> { unsafe {
        <Self as RtActivatable<IEccCurveNamesStatics>>::get_activation_factory().get_x962_p239v3()
    }}
    #[inline] pub fn get_x962_p256v1() -> Result<HString> { unsafe {
        <Self as RtActivatable<IEccCurveNamesStatics>>::get_activation_factory().get_x962_p256v1()
    }}
    #[inline] pub fn get_all_ecc_curve_names() -> Result<ComPtr<::rt::gen::windows::foundation::collections::IVectorView<HString>>> { unsafe {
        <Self as RtActivatable<IEccCurveNamesStatics>>::get_activation_factory().get_all_ecc_curve_names()
    }}
}
DEFINE_CLSID!(EccCurveNames(&[87,105,110,100,111,119,115,46,83,101,99,117,114,105,116,121,46,67,114,121,112,116,111,103,114,97,112,104,121,46,67,111,114,101,46,69,99,99,67,117,114,118,101,78,97,109,101,115,0]) [CLSID_EccCurveNames]);
DEFINE_IID!(IID_IEccCurveNamesStatics, 3019870988, 44779, 16542, 183, 212, 155, 149, 41, 90, 174, 207);
RT_INTERFACE!{static interface IEccCurveNamesStatics(IEccCurveNamesStaticsVtbl): IInspectable(IInspectableVtbl) [IID_IEccCurveNamesStatics] {
    fn get_BrainpoolP160r1(&self, out: *mut HSTRING) -> HRESULT,
    fn get_BrainpoolP160t1(&self, out: *mut HSTRING) -> HRESULT,
    fn get_BrainpoolP192r1(&self, out: *mut HSTRING) -> HRESULT,
    fn get_BrainpoolP192t1(&self, out: *mut HSTRING) -> HRESULT,
    fn get_BrainpoolP224r1(&self, out: *mut HSTRING) -> HRESULT,
    fn get_BrainpoolP224t1(&self, out: *mut HSTRING) -> HRESULT,
    fn get_BrainpoolP256r1(&self, out: *mut HSTRING) -> HRESULT,
    fn get_BrainpoolP256t1(&self, out: *mut HSTRING) -> HRESULT,
    fn get_BrainpoolP320r1(&self, out: *mut HSTRING) -> HRESULT,
    fn get_BrainpoolP320t1(&self, out: *mut HSTRING) -> HRESULT,
    fn get_BrainpoolP384r1(&self, out: *mut HSTRING) -> HRESULT,
    fn get_BrainpoolP384t1(&self, out: *mut HSTRING) -> HRESULT,
    fn get_BrainpoolP512r1(&self, out: *mut HSTRING) -> HRESULT,
    fn get_BrainpoolP512t1(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Curve25519(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Ec192wapi(&self, out: *mut HSTRING) -> HRESULT,
    fn get_NistP192(&self, out: *mut HSTRING) -> HRESULT,
    fn get_NistP224(&self, out: *mut HSTRING) -> HRESULT,
    fn get_NistP256(&self, out: *mut HSTRING) -> HRESULT,
    fn get_NistP384(&self, out: *mut HSTRING) -> HRESULT,
    fn get_NistP521(&self, out: *mut HSTRING) -> HRESULT,
    fn get_NumsP256t1(&self, out: *mut HSTRING) -> HRESULT,
    fn get_NumsP384t1(&self, out: *mut HSTRING) -> HRESULT,
    fn get_NumsP512t1(&self, out: *mut HSTRING) -> HRESULT,
    fn get_SecP160k1(&self, out: *mut HSTRING) -> HRESULT,
    fn get_SecP160r1(&self, out: *mut HSTRING) -> HRESULT,
    fn get_SecP160r2(&self, out: *mut HSTRING) -> HRESULT,
    fn get_SecP192k1(&self, out: *mut HSTRING) -> HRESULT,
    fn get_SecP192r1(&self, out: *mut HSTRING) -> HRESULT,
    fn get_SecP224k1(&self, out: *mut HSTRING) -> HRESULT,
    fn get_SecP224r1(&self, out: *mut HSTRING) -> HRESULT,
    fn get_SecP256k1(&self, out: *mut HSTRING) -> HRESULT,
    fn get_SecP256r1(&self, out: *mut HSTRING) -> HRESULT,
    fn get_SecP384r1(&self, out: *mut HSTRING) -> HRESULT,
    fn get_SecP521r1(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Wtls7(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Wtls9(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Wtls12(&self, out: *mut HSTRING) -> HRESULT,
    fn get_X962P192v1(&self, out: *mut HSTRING) -> HRESULT,
    fn get_X962P192v2(&self, out: *mut HSTRING) -> HRESULT,
    fn get_X962P192v3(&self, out: *mut HSTRING) -> HRESULT,
    fn get_X962P239v1(&self, out: *mut HSTRING) -> HRESULT,
    fn get_X962P239v2(&self, out: *mut HSTRING) -> HRESULT,
    fn get_X962P239v3(&self, out: *mut HSTRING) -> HRESULT,
    fn get_X962P256v1(&self, out: *mut HSTRING) -> HRESULT,
    fn get_AllEccCurveNames(&self, out: *mut *mut ::rt::gen::windows::foundation::collections::IVectorView<HString>) -> HRESULT
}}
impl IEccCurveNamesStatics {
    #[inline] pub unsafe fn get_brainpool_p160r1(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_BrainpoolP160r1)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_brainpool_p160t1(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_BrainpoolP160t1)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_brainpool_p192r1(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_BrainpoolP192r1)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_brainpool_p192t1(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_BrainpoolP192t1)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_brainpool_p224r1(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_BrainpoolP224r1)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_brainpool_p224t1(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_BrainpoolP224t1)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_brainpool_p256r1(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_BrainpoolP256r1)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_brainpool_p256t1(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_BrainpoolP256t1)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_brainpool_p320r1(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_BrainpoolP320r1)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_brainpool_p320t1(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_BrainpoolP320t1)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_brainpool_p384r1(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_BrainpoolP384r1)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_brainpool_p384t1(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_BrainpoolP384t1)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_brainpool_p512r1(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_BrainpoolP512r1)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_brainpool_p512t1(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_BrainpoolP512t1)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_curve25519(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Curve25519)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_ec192wapi(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Ec192wapi)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_nist_p192(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_NistP192)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_nist_p224(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_NistP224)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_nist_p256(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_NistP256)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_nist_p384(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_NistP384)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_nist_p521(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_NistP521)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_nums_p256t1(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_NumsP256t1)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_nums_p384t1(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_NumsP384t1)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_nums_p512t1(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_NumsP512t1)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_sec_p160k1(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_SecP160k1)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_sec_p160r1(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_SecP160r1)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_sec_p160r2(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_SecP160r2)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_sec_p192k1(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_SecP192k1)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_sec_p192r1(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_SecP192r1)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_sec_p224k1(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_SecP224k1)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_sec_p224r1(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_SecP224r1)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_sec_p256k1(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_SecP256k1)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_sec_p256r1(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_SecP256r1)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_sec_p384r1(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_SecP384r1)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_sec_p521r1(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_SecP521r1)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_wtls7(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Wtls7)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_wtls9(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Wtls9)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_wtls12(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Wtls12)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_x962_p192v1(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_X962P192v1)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_x962_p192v2(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_X962P192v2)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_x962_p192v3(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_X962P192v3)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_x962_p239v1(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_X962P239v1)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_x962_p239v2(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_X962P239v2)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_x962_p239v3(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_X962P239v3)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_x962_p256v1(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_X962P256v1)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_all_ecc_curve_names(&self) -> Result<ComPtr<::rt::gen::windows::foundation::collections::IVectorView<HString>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_AllEccCurveNames)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
DEFINE_IID!(IID_IEncryptedAndAuthenticatedData, 1873031143, 7883, 19200, 190, 165, 96, 184, 63, 134, 47, 23);
RT_INTERFACE!{interface IEncryptedAndAuthenticatedData(IEncryptedAndAuthenticatedDataVtbl): IInspectable(IInspectableVtbl) [IID_IEncryptedAndAuthenticatedData] {
    #[cfg(feature="windows-storage")] fn get_EncryptedData(&self, out: *mut *mut ::rt::gen::windows::storage::streams::IBuffer) -> HRESULT,
    #[cfg(feature="windows-storage")] fn get_AuthenticationTag(&self, out: *mut *mut ::rt::gen::windows::storage::streams::IBuffer) -> HRESULT
}}
impl IEncryptedAndAuthenticatedData {
    #[cfg(feature="windows-storage")] #[inline] pub unsafe fn get_encrypted_data(&self) -> Result<ComPtr<::rt::gen::windows::storage::streams::IBuffer>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_EncryptedData)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[cfg(feature="windows-storage")] #[inline] pub unsafe fn get_authentication_tag(&self) -> Result<ComPtr<::rt::gen::windows::storage::streams::IBuffer>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_AuthenticationTag)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
RT_CLASS!{class EncryptedAndAuthenticatedData: IEncryptedAndAuthenticatedData}
RT_CLASS!{static class HashAlgorithmNames}
impl RtActivatable<IHashAlgorithmNamesStatics> for HashAlgorithmNames {}
impl HashAlgorithmNames {
    #[inline] pub fn get_md5() -> Result<HString> { unsafe {
        <Self as RtActivatable<IHashAlgorithmNamesStatics>>::get_activation_factory().get_md5()
    }}
    #[inline] pub fn get_sha1() -> Result<HString> { unsafe {
        <Self as RtActivatable<IHashAlgorithmNamesStatics>>::get_activation_factory().get_sha1()
    }}
    #[inline] pub fn get_sha256() -> Result<HString> { unsafe {
        <Self as RtActivatable<IHashAlgorithmNamesStatics>>::get_activation_factory().get_sha256()
    }}
    #[inline] pub fn get_sha384() -> Result<HString> { unsafe {
        <Self as RtActivatable<IHashAlgorithmNamesStatics>>::get_activation_factory().get_sha384()
    }}
    #[inline] pub fn get_sha512() -> Result<HString> { unsafe {
        <Self as RtActivatable<IHashAlgorithmNamesStatics>>::get_activation_factory().get_sha512()
    }}
}
DEFINE_CLSID!(HashAlgorithmNames(&[87,105,110,100,111,119,115,46,83,101,99,117,114,105,116,121,46,67,114,121,112,116,111,103,114,97,112,104,121,46,67,111,114,101,46,72,97,115,104,65,108,103,111,114,105,116,104,109,78,97,109,101,115,0]) [CLSID_HashAlgorithmNames]);
DEFINE_IID!(IID_IHashAlgorithmNamesStatics, 1801323798, 56982, 20234, 141, 87, 220, 201, 218, 227, 108, 118);
RT_INTERFACE!{static interface IHashAlgorithmNamesStatics(IHashAlgorithmNamesStaticsVtbl): IInspectable(IInspectableVtbl) [IID_IHashAlgorithmNamesStatics] {
    fn get_Md5(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Sha1(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Sha256(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Sha384(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Sha512(&self, out: *mut HSTRING) -> HRESULT
}}
impl IHashAlgorithmNamesStatics {
    #[inline] pub unsafe fn get_md5(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Md5)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_sha1(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Sha1)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_sha256(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Sha256)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_sha384(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Sha384)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_sha512(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Sha512)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
}
DEFINE_IID!(IID_IHashAlgorithmProvider, 3197841536, 45763, 16939, 188, 225, 236, 144, 239, 181, 215, 181);
RT_INTERFACE!{interface IHashAlgorithmProvider(IHashAlgorithmProviderVtbl): IInspectable(IInspectableVtbl) [IID_IHashAlgorithmProvider] {
    fn get_AlgorithmName(&self, out: *mut HSTRING) -> HRESULT,
    fn get_HashLength(&self, out: *mut u32) -> HRESULT,
    #[cfg(not(feature="windows-storage"))] fn __Dummy2(&self) -> (),
    #[cfg(feature="windows-storage")] fn HashData(&self, data: *mut ::rt::gen::windows::storage::streams::IBuffer, out: *mut *mut ::rt::gen::windows::storage::streams::IBuffer) -> HRESULT,
    fn CreateHash(&self, out: *mut *mut CryptographicHash) -> HRESULT
}}
impl IHashAlgorithmProvider {
    #[inline] pub unsafe fn get_algorithm_name(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_AlgorithmName)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_hash_length(&self) -> Result<u32> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_HashLength)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[cfg(feature="windows-storage")] #[inline] pub unsafe fn hash_data(&self, data: &::rt::gen::windows::storage::streams::IBuffer) -> Result<ComPtr<::rt::gen::windows::storage::streams::IBuffer>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).HashData)(self as *const _ as *mut _, data as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn create_hash(&self) -> Result<ComPtr<CryptographicHash>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).CreateHash)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
RT_CLASS!{class HashAlgorithmProvider: IHashAlgorithmProvider}
impl RtActivatable<IHashAlgorithmProviderStatics> for HashAlgorithmProvider {}
impl HashAlgorithmProvider {
    #[inline] pub fn open_algorithm(algorithm: &HStringArg) -> Result<ComPtr<HashAlgorithmProvider>> { unsafe {
        <Self as RtActivatable<IHashAlgorithmProviderStatics>>::get_activation_factory().open_algorithm(algorithm)
    }}
}
DEFINE_CLSID!(HashAlgorithmProvider(&[87,105,110,100,111,119,115,46,83,101,99,117,114,105,116,121,46,67,114,121,112,116,111,103,114,97,112,104,121,46,67,111,114,101,46,72,97,115,104,65,108,103,111,114,105,116,104,109,80,114,111,118,105,100,101,114,0]) [CLSID_HashAlgorithmProvider]);
DEFINE_IID!(IID_IHashAlgorithmProviderStatics, 2678888257, 23748, 17206, 174, 56, 98, 18, 183, 90, 145, 90);
RT_INTERFACE!{static interface IHashAlgorithmProviderStatics(IHashAlgorithmProviderStaticsVtbl): IInspectable(IInspectableVtbl) [IID_IHashAlgorithmProviderStatics] {
    fn OpenAlgorithm(&self, algorithm: HSTRING, out: *mut *mut HashAlgorithmProvider) -> HRESULT
}}
impl IHashAlgorithmProviderStatics {
    #[inline] pub unsafe fn open_algorithm(&self, algorithm: &HStringArg) -> Result<ComPtr<HashAlgorithmProvider>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).OpenAlgorithm)(self as *const _ as *mut _, algorithm.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
DEFINE_IID!(IID_IHashComputation, 1493488054, 44337, 17923, 163, 164, 177, 189, 169, 142, 37, 98);
RT_INTERFACE!{interface IHashComputation(IHashComputationVtbl): IInspectable(IInspectableVtbl) [IID_IHashComputation] {
    #[cfg(feature="windows-storage")] fn Append(&self, data: *mut ::rt::gen::windows::storage::streams::IBuffer) -> HRESULT,
    #[cfg(feature="windows-storage")] fn GetValueAndReset(&self, out: *mut *mut ::rt::gen::windows::storage::streams::IBuffer) -> HRESULT
}}
impl IHashComputation {
    #[cfg(feature="windows-storage")] #[inline] pub unsafe fn append(&self, data: &::rt::gen::windows::storage::streams::IBuffer) -> Result<()> {
        let hr = ((*self.lpVtbl).Append)(self as *const _ as *mut _, data as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[cfg(feature="windows-storage")] #[inline] pub unsafe fn get_value_and_reset(&self) -> Result<ComPtr<::rt::gen::windows::storage::streams::IBuffer>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetValueAndReset)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
RT_CLASS!{static class KeyDerivationAlgorithmNames}
impl RtActivatable<IKeyDerivationAlgorithmNamesStatics> for KeyDerivationAlgorithmNames {}
impl RtActivatable<IKeyDerivationAlgorithmNamesStatics2> for KeyDerivationAlgorithmNames {}
impl KeyDerivationAlgorithmNames {
    #[inline] pub fn get_pbkdf2_md5() -> Result<HString> { unsafe {
        <Self as RtActivatable<IKeyDerivationAlgorithmNamesStatics>>::get_activation_factory().get_pbkdf2_md5()
    }}
    #[inline] pub fn get_pbkdf2_sha1() -> Result<HString> { unsafe {
        <Self as RtActivatable<IKeyDerivationAlgorithmNamesStatics>>::get_activation_factory().get_pbkdf2_sha1()
    }}
    #[inline] pub fn get_pbkdf2_sha256() -> Result<HString> { unsafe {
        <Self as RtActivatable<IKeyDerivationAlgorithmNamesStatics>>::get_activation_factory().get_pbkdf2_sha256()
    }}
    #[inline] pub fn get_pbkdf2_sha384() -> Result<HString> { unsafe {
        <Self as RtActivatable<IKeyDerivationAlgorithmNamesStatics>>::get_activation_factory().get_pbkdf2_sha384()
    }}
    #[inline] pub fn get_pbkdf2_sha512() -> Result<HString> { unsafe {
        <Self as RtActivatable<IKeyDerivationAlgorithmNamesStatics>>::get_activation_factory().get_pbkdf2_sha512()
    }}
    #[inline] pub fn get_sp800108_ctr_hmac_md5() -> Result<HString> { unsafe {
        <Self as RtActivatable<IKeyDerivationAlgorithmNamesStatics>>::get_activation_factory().get_sp800108_ctr_hmac_md5()
    }}
    #[inline] pub fn get_sp800108_ctr_hmac_sha1() -> Result<HString> { unsafe {
        <Self as RtActivatable<IKeyDerivationAlgorithmNamesStatics>>::get_activation_factory().get_sp800108_ctr_hmac_sha1()
    }}
    #[inline] pub fn get_sp800108_ctr_hmac_sha256() -> Result<HString> { unsafe {
        <Self as RtActivatable<IKeyDerivationAlgorithmNamesStatics>>::get_activation_factory().get_sp800108_ctr_hmac_sha256()
    }}
    #[inline] pub fn get_sp800108_ctr_hmac_sha384() -> Result<HString> { unsafe {
        <Self as RtActivatable<IKeyDerivationAlgorithmNamesStatics>>::get_activation_factory().get_sp800108_ctr_hmac_sha384()
    }}
    #[inline] pub fn get_sp800108_ctr_hmac_sha512() -> Result<HString> { unsafe {
        <Self as RtActivatable<IKeyDerivationAlgorithmNamesStatics>>::get_activation_factory().get_sp800108_ctr_hmac_sha512()
    }}
    #[inline] pub fn get_sp80056a_concat_md5() -> Result<HString> { unsafe {
        <Self as RtActivatable<IKeyDerivationAlgorithmNamesStatics>>::get_activation_factory().get_sp80056a_concat_md5()
    }}
    #[inline] pub fn get_sp80056a_concat_sha1() -> Result<HString> { unsafe {
        <Self as RtActivatable<IKeyDerivationAlgorithmNamesStatics>>::get_activation_factory().get_sp80056a_concat_sha1()
    }}
    #[inline] pub fn get_sp80056a_concat_sha256() -> Result<HString> { unsafe {
        <Self as RtActivatable<IKeyDerivationAlgorithmNamesStatics>>::get_activation_factory().get_sp80056a_concat_sha256()
    }}
    #[inline] pub fn get_sp80056a_concat_sha384() -> Result<HString> { unsafe {
        <Self as RtActivatable<IKeyDerivationAlgorithmNamesStatics>>::get_activation_factory().get_sp80056a_concat_sha384()
    }}
    #[inline] pub fn get_sp80056a_concat_sha512() -> Result<HString> { unsafe {
        <Self as RtActivatable<IKeyDerivationAlgorithmNamesStatics>>::get_activation_factory().get_sp80056a_concat_sha512()
    }}
    #[inline] pub fn get_capi_kdf_md5() -> Result<HString> { unsafe {
        <Self as RtActivatable<IKeyDerivationAlgorithmNamesStatics2>>::get_activation_factory().get_capi_kdf_md5()
    }}
    #[inline] pub fn get_capi_kdf_sha1() -> Result<HString> { unsafe {
        <Self as RtActivatable<IKeyDerivationAlgorithmNamesStatics2>>::get_activation_factory().get_capi_kdf_sha1()
    }}
    #[inline] pub fn get_capi_kdf_sha256() -> Result<HString> { unsafe {
        <Self as RtActivatable<IKeyDerivationAlgorithmNamesStatics2>>::get_activation_factory().get_capi_kdf_sha256()
    }}
    #[inline] pub fn get_capi_kdf_sha384() -> Result<HString> { unsafe {
        <Self as RtActivatable<IKeyDerivationAlgorithmNamesStatics2>>::get_activation_factory().get_capi_kdf_sha384()
    }}
    #[inline] pub fn get_capi_kdf_sha512() -> Result<HString> { unsafe {
        <Self as RtActivatable<IKeyDerivationAlgorithmNamesStatics2>>::get_activation_factory().get_capi_kdf_sha512()
    }}
}
DEFINE_CLSID!(KeyDerivationAlgorithmNames(&[87,105,110,100,111,119,115,46,83,101,99,117,114,105,116,121,46,67,114,121,112,116,111,103,114,97,112,104,121,46,67,111,114,101,46,75,101,121,68,101,114,105,118,97,116,105,111,110,65,108,103,111,114,105,116,104,109,78,97,109,101,115,0]) [CLSID_KeyDerivationAlgorithmNames]);
DEFINE_IID!(IID_IKeyDerivationAlgorithmNamesStatics, 2070820414, 38098, 18233, 165, 123, 2, 46, 12, 58, 64, 42);
RT_INTERFACE!{static interface IKeyDerivationAlgorithmNamesStatics(IKeyDerivationAlgorithmNamesStaticsVtbl): IInspectable(IInspectableVtbl) [IID_IKeyDerivationAlgorithmNamesStatics] {
    fn get_Pbkdf2Md5(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Pbkdf2Sha1(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Pbkdf2Sha256(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Pbkdf2Sha384(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Pbkdf2Sha512(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Sp800108CtrHmacMd5(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Sp800108CtrHmacSha1(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Sp800108CtrHmacSha256(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Sp800108CtrHmacSha384(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Sp800108CtrHmacSha512(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Sp80056aConcatMd5(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Sp80056aConcatSha1(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Sp80056aConcatSha256(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Sp80056aConcatSha384(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Sp80056aConcatSha512(&self, out: *mut HSTRING) -> HRESULT
}}
impl IKeyDerivationAlgorithmNamesStatics {
    #[inline] pub unsafe fn get_pbkdf2_md5(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Pbkdf2Md5)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_pbkdf2_sha1(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Pbkdf2Sha1)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_pbkdf2_sha256(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Pbkdf2Sha256)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_pbkdf2_sha384(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Pbkdf2Sha384)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_pbkdf2_sha512(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Pbkdf2Sha512)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_sp800108_ctr_hmac_md5(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Sp800108CtrHmacMd5)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_sp800108_ctr_hmac_sha1(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Sp800108CtrHmacSha1)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_sp800108_ctr_hmac_sha256(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Sp800108CtrHmacSha256)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_sp800108_ctr_hmac_sha384(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Sp800108CtrHmacSha384)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_sp800108_ctr_hmac_sha512(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Sp800108CtrHmacSha512)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_sp80056a_concat_md5(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Sp80056aConcatMd5)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_sp80056a_concat_sha1(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Sp80056aConcatSha1)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_sp80056a_concat_sha256(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Sp80056aConcatSha256)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_sp80056a_concat_sha384(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Sp80056aConcatSha384)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_sp80056a_concat_sha512(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Sp80056aConcatSha512)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
}
DEFINE_IID!(IID_IKeyDerivationAlgorithmNamesStatics2, 1469398955, 24644, 18031, 151, 244, 51, 123, 120, 8, 56, 77);
RT_INTERFACE!{static interface IKeyDerivationAlgorithmNamesStatics2(IKeyDerivationAlgorithmNamesStatics2Vtbl): IInspectable(IInspectableVtbl) [IID_IKeyDerivationAlgorithmNamesStatics2] {
    fn get_CapiKdfMd5(&self, out: *mut HSTRING) -> HRESULT,
    fn get_CapiKdfSha1(&self, out: *mut HSTRING) -> HRESULT,
    fn get_CapiKdfSha256(&self, out: *mut HSTRING) -> HRESULT,
    fn get_CapiKdfSha384(&self, out: *mut HSTRING) -> HRESULT,
    fn get_CapiKdfSha512(&self, out: *mut HSTRING) -> HRESULT
}}
impl IKeyDerivationAlgorithmNamesStatics2 {
    #[inline] pub unsafe fn get_capi_kdf_md5(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_CapiKdfMd5)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_capi_kdf_sha1(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_CapiKdfSha1)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_capi_kdf_sha256(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_CapiKdfSha256)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_capi_kdf_sha384(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_CapiKdfSha384)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_capi_kdf_sha512(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_CapiKdfSha512)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
}
DEFINE_IID!(IID_IKeyDerivationAlgorithmProvider, 3791366203, 18033, 17335, 145, 88, 118, 58, 170, 152, 182, 191);
RT_INTERFACE!{interface IKeyDerivationAlgorithmProvider(IKeyDerivationAlgorithmProviderVtbl): IInspectable(IInspectableVtbl) [IID_IKeyDerivationAlgorithmProvider] {
    fn get_AlgorithmName(&self, out: *mut HSTRING) -> HRESULT,
    #[cfg(feature="windows-storage")] fn CreateKey(&self, keyMaterial: *mut ::rt::gen::windows::storage::streams::IBuffer, out: *mut *mut CryptographicKey) -> HRESULT
}}
impl IKeyDerivationAlgorithmProvider {
    #[inline] pub unsafe fn get_algorithm_name(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_AlgorithmName)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[cfg(feature="windows-storage")] #[inline] pub unsafe fn create_key(&self, keyMaterial: &::rt::gen::windows::storage::streams::IBuffer) -> Result<ComPtr<CryptographicKey>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).CreateKey)(self as *const _ as *mut _, keyMaterial as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
RT_CLASS!{class KeyDerivationAlgorithmProvider: IKeyDerivationAlgorithmProvider}
impl RtActivatable<IKeyDerivationAlgorithmProviderStatics> for KeyDerivationAlgorithmProvider {}
impl KeyDerivationAlgorithmProvider {
    #[inline] pub fn open_algorithm(algorithm: &HStringArg) -> Result<ComPtr<KeyDerivationAlgorithmProvider>> { unsafe {
        <Self as RtActivatable<IKeyDerivationAlgorithmProviderStatics>>::get_activation_factory().open_algorithm(algorithm)
    }}
}
DEFINE_CLSID!(KeyDerivationAlgorithmProvider(&[87,105,110,100,111,119,115,46,83,101,99,117,114,105,116,121,46,67,114,121,112,116,111,103,114,97,112,104,121,46,67,111,114,101,46,75,101,121,68,101,114,105,118,97,116,105,111,110,65,108,103,111,114,105,116,104,109,80,114,111,118,105,100,101,114,0]) [CLSID_KeyDerivationAlgorithmProvider]);
DEFINE_IID!(IID_IKeyDerivationAlgorithmProviderStatics, 170002810, 2588, 17467, 148, 24, 185, 73, 138, 235, 22, 3);
RT_INTERFACE!{static interface IKeyDerivationAlgorithmProviderStatics(IKeyDerivationAlgorithmProviderStaticsVtbl): IInspectable(IInspectableVtbl) [IID_IKeyDerivationAlgorithmProviderStatics] {
    fn OpenAlgorithm(&self, algorithm: HSTRING, out: *mut *mut KeyDerivationAlgorithmProvider) -> HRESULT
}}
impl IKeyDerivationAlgorithmProviderStatics {
    #[inline] pub unsafe fn open_algorithm(&self, algorithm: &HStringArg) -> Result<ComPtr<KeyDerivationAlgorithmProvider>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).OpenAlgorithm)(self as *const _ as *mut _, algorithm.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
DEFINE_IID!(IID_IKeyDerivationParameters, 2079349095, 1147, 19084, 150, 74, 70, 159, 253, 85, 34, 226);
RT_INTERFACE!{interface IKeyDerivationParameters(IKeyDerivationParametersVtbl): IInspectable(IInspectableVtbl) [IID_IKeyDerivationParameters] {
    #[cfg(not(feature="windows-storage"))] fn __Dummy0(&self) -> (),
    #[cfg(feature="windows-storage")] fn get_KdfGenericBinary(&self, out: *mut *mut ::rt::gen::windows::storage::streams::IBuffer) -> HRESULT,
    #[cfg(not(feature="windows-storage"))] fn __Dummy1(&self) -> (),
    #[cfg(feature="windows-storage")] fn put_KdfGenericBinary(&self, value: *mut ::rt::gen::windows::storage::streams::IBuffer) -> HRESULT,
    fn get_IterationCount(&self, out: *mut u32) -> HRESULT
}}
impl IKeyDerivationParameters {
    #[cfg(feature="windows-storage")] #[inline] pub unsafe fn get_kdf_generic_binary(&self) -> Result<ComPtr<::rt::gen::windows::storage::streams::IBuffer>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_KdfGenericBinary)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[cfg(feature="windows-storage")] #[inline] pub unsafe fn set_kdf_generic_binary(&self, value: &::rt::gen::windows::storage::streams::IBuffer) -> Result<()> {
        let hr = ((*self.lpVtbl).put_KdfGenericBinary)(self as *const _ as *mut _, value as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_iteration_count(&self) -> Result<u32> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_IterationCount)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
}
RT_CLASS!{class KeyDerivationParameters: IKeyDerivationParameters}
impl RtActivatable<IKeyDerivationParametersStatics> for KeyDerivationParameters {}
impl RtActivatable<IKeyDerivationParametersStatics2> for KeyDerivationParameters {}
impl KeyDerivationParameters {
    #[cfg(feature="windows-storage")] #[inline] pub fn build_for_pbkdf2(pbkdf2Salt: &::rt::gen::windows::storage::streams::IBuffer, iterationCount: u32) -> Result<ComPtr<KeyDerivationParameters>> { unsafe {
        <Self as RtActivatable<IKeyDerivationParametersStatics>>::get_activation_factory().build_for_pbkdf2(pbkdf2Salt, iterationCount)
    }}
    #[cfg(feature="windows-storage")] #[inline] pub fn build_for_sp800108(label: &::rt::gen::windows::storage::streams::IBuffer, context: &::rt::gen::windows::storage::streams::IBuffer) -> Result<ComPtr<KeyDerivationParameters>> { unsafe {
        <Self as RtActivatable<IKeyDerivationParametersStatics>>::get_activation_factory().build_for_sp800108(label, context)
    }}
    #[cfg(feature="windows-storage")] #[inline] pub fn build_for_sp80056a(algorithmId: &::rt::gen::windows::storage::streams::IBuffer, partyUInfo: &::rt::gen::windows::storage::streams::IBuffer, partyVInfo: &::rt::gen::windows::storage::streams::IBuffer, suppPubInfo: &::rt::gen::windows::storage::streams::IBuffer, suppPrivInfo: &::rt::gen::windows::storage::streams::IBuffer) -> Result<ComPtr<KeyDerivationParameters>> { unsafe {
        <Self as RtActivatable<IKeyDerivationParametersStatics>>::get_activation_factory().build_for_sp80056a(algorithmId, partyUInfo, partyVInfo, suppPubInfo, suppPrivInfo)
    }}
    #[inline] pub fn build_for_capi1_kdf(capi1KdfTargetAlgorithm: Capi1KdfTargetAlgorithm) -> Result<ComPtr<KeyDerivationParameters>> { unsafe {
        <Self as RtActivatable<IKeyDerivationParametersStatics2>>::get_activation_factory().build_for_capi1_kdf(capi1KdfTargetAlgorithm)
    }}
}
DEFINE_CLSID!(KeyDerivationParameters(&[87,105,110,100,111,119,115,46,83,101,99,117,114,105,116,121,46,67,114,121,112,116,111,103,114,97,112,104,121,46,67,111,114,101,46,75,101,121,68,101,114,105,118,97,116,105,111,110,80,97,114,97,109,101,116,101,114,115,0]) [CLSID_KeyDerivationParameters]);
DEFINE_IID!(IID_IKeyDerivationParameters2, 3443615441, 16766, 20300, 182, 102, 192, 216, 121, 243, 248, 224);
RT_INTERFACE!{interface IKeyDerivationParameters2(IKeyDerivationParameters2Vtbl): IInspectable(IInspectableVtbl) [IID_IKeyDerivationParameters2] {
    fn get_Capi1KdfTargetAlgorithm(&self, out: *mut Capi1KdfTargetAlgorithm) -> HRESULT,
    fn put_Capi1KdfTargetAlgorithm(&self, value: Capi1KdfTargetAlgorithm) -> HRESULT
}}
impl IKeyDerivationParameters2 {
    #[inline] pub unsafe fn get_capi1_kdf_target_algorithm(&self) -> Result<Capi1KdfTargetAlgorithm> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_Capi1KdfTargetAlgorithm)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_capi1_kdf_target_algorithm(&self, value: Capi1KdfTargetAlgorithm) -> Result<()> {
        let hr = ((*self.lpVtbl).put_Capi1KdfTargetAlgorithm)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
}
DEFINE_IID!(IID_IKeyDerivationParametersStatics, 3935707070, 62335, 16710, 157, 254, 164, 86, 241, 115, 95, 75);
RT_INTERFACE!{static interface IKeyDerivationParametersStatics(IKeyDerivationParametersStaticsVtbl): IInspectable(IInspectableVtbl) [IID_IKeyDerivationParametersStatics] {
    #[cfg(feature="windows-storage")] fn BuildForPbkdf2(&self, pbkdf2Salt: *mut ::rt::gen::windows::storage::streams::IBuffer, iterationCount: u32, out: *mut *mut KeyDerivationParameters) -> HRESULT,
    #[cfg(feature="windows-storage")] fn BuildForSP800108(&self, label: *mut ::rt::gen::windows::storage::streams::IBuffer, context: *mut ::rt::gen::windows::storage::streams::IBuffer, out: *mut *mut KeyDerivationParameters) -> HRESULT,
    #[cfg(feature="windows-storage")] fn BuildForSP80056a(&self, algorithmId: *mut ::rt::gen::windows::storage::streams::IBuffer, partyUInfo: *mut ::rt::gen::windows::storage::streams::IBuffer, partyVInfo: *mut ::rt::gen::windows::storage::streams::IBuffer, suppPubInfo: *mut ::rt::gen::windows::storage::streams::IBuffer, suppPrivInfo: *mut ::rt::gen::windows::storage::streams::IBuffer, out: *mut *mut KeyDerivationParameters) -> HRESULT
}}
impl IKeyDerivationParametersStatics {
    #[cfg(feature="windows-storage")] #[inline] pub unsafe fn build_for_pbkdf2(&self, pbkdf2Salt: &::rt::gen::windows::storage::streams::IBuffer, iterationCount: u32) -> Result<ComPtr<KeyDerivationParameters>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).BuildForPbkdf2)(self as *const _ as *mut _, pbkdf2Salt as *const _ as *mut _, iterationCount, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[cfg(feature="windows-storage")] #[inline] pub unsafe fn build_for_sp800108(&self, label: &::rt::gen::windows::storage::streams::IBuffer, context: &::rt::gen::windows::storage::streams::IBuffer) -> Result<ComPtr<KeyDerivationParameters>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).BuildForSP800108)(self as *const _ as *mut _, label as *const _ as *mut _, context as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[cfg(feature="windows-storage")] #[inline] pub unsafe fn build_for_sp80056a(&self, algorithmId: &::rt::gen::windows::storage::streams::IBuffer, partyUInfo: &::rt::gen::windows::storage::streams::IBuffer, partyVInfo: &::rt::gen::windows::storage::streams::IBuffer, suppPubInfo: &::rt::gen::windows::storage::streams::IBuffer, suppPrivInfo: &::rt::gen::windows::storage::streams::IBuffer) -> Result<ComPtr<KeyDerivationParameters>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).BuildForSP80056a)(self as *const _ as *mut _, algorithmId as *const _ as *mut _, partyUInfo as *const _ as *mut _, partyVInfo as *const _ as *mut _, suppPubInfo as *const _ as *mut _, suppPrivInfo as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
DEFINE_IID!(IID_IKeyDerivationParametersStatics2, 2776120789, 22755, 20219, 178, 131, 161, 101, 49, 38, 225, 190);
RT_INTERFACE!{static interface IKeyDerivationParametersStatics2(IKeyDerivationParametersStatics2Vtbl): IInspectable(IInspectableVtbl) [IID_IKeyDerivationParametersStatics2] {
    fn BuildForCapi1Kdf(&self, capi1KdfTargetAlgorithm: Capi1KdfTargetAlgorithm, out: *mut *mut KeyDerivationParameters) -> HRESULT
}}
impl IKeyDerivationParametersStatics2 {
    #[inline] pub unsafe fn build_for_capi1_kdf(&self, capi1KdfTargetAlgorithm: Capi1KdfTargetAlgorithm) -> Result<ComPtr<KeyDerivationParameters>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).BuildForCapi1Kdf)(self as *const _ as *mut _, capi1KdfTargetAlgorithm, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
RT_CLASS!{static class MacAlgorithmNames}
impl RtActivatable<IMacAlgorithmNamesStatics> for MacAlgorithmNames {}
impl MacAlgorithmNames {
    #[inline] pub fn get_hmac_md5() -> Result<HString> { unsafe {
        <Self as RtActivatable<IMacAlgorithmNamesStatics>>::get_activation_factory().get_hmac_md5()
    }}
    #[inline] pub fn get_hmac_sha1() -> Result<HString> { unsafe {
        <Self as RtActivatable<IMacAlgorithmNamesStatics>>::get_activation_factory().get_hmac_sha1()
    }}
    #[inline] pub fn get_hmac_sha256() -> Result<HString> { unsafe {
        <Self as RtActivatable<IMacAlgorithmNamesStatics>>::get_activation_factory().get_hmac_sha256()
    }}
    #[inline] pub fn get_hmac_sha384() -> Result<HString> { unsafe {
        <Self as RtActivatable<IMacAlgorithmNamesStatics>>::get_activation_factory().get_hmac_sha384()
    }}
    #[inline] pub fn get_hmac_sha512() -> Result<HString> { unsafe {
        <Self as RtActivatable<IMacAlgorithmNamesStatics>>::get_activation_factory().get_hmac_sha512()
    }}
    #[inline] pub fn get_aes_cmac() -> Result<HString> { unsafe {
        <Self as RtActivatable<IMacAlgorithmNamesStatics>>::get_activation_factory().get_aes_cmac()
    }}
}
DEFINE_CLSID!(MacAlgorithmNames(&[87,105,110,100,111,119,115,46,83,101,99,117,114,105,116,121,46,67,114,121,112,116,111,103,114,97,112,104,121,46,67,111,114,101,46,77,97,99,65,108,103,111,114,105,116,104,109,78,97,109,101,115,0]) [CLSID_MacAlgorithmNames]);
DEFINE_IID!(IID_IMacAlgorithmNamesStatics, 1094788728, 64286, 17316, 137, 94, 169, 2, 110, 67, 144, 163);
RT_INTERFACE!{static interface IMacAlgorithmNamesStatics(IMacAlgorithmNamesStaticsVtbl): IInspectable(IInspectableVtbl) [IID_IMacAlgorithmNamesStatics] {
    fn get_HmacMd5(&self, out: *mut HSTRING) -> HRESULT,
    fn get_HmacSha1(&self, out: *mut HSTRING) -> HRESULT,
    fn get_HmacSha256(&self, out: *mut HSTRING) -> HRESULT,
    fn get_HmacSha384(&self, out: *mut HSTRING) -> HRESULT,
    fn get_HmacSha512(&self, out: *mut HSTRING) -> HRESULT,
    fn get_AesCmac(&self, out: *mut HSTRING) -> HRESULT
}}
impl IMacAlgorithmNamesStatics {
    #[inline] pub unsafe fn get_hmac_md5(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_HmacMd5)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_hmac_sha1(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_HmacSha1)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_hmac_sha256(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_HmacSha256)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_hmac_sha384(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_HmacSha384)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_hmac_sha512(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_HmacSha512)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_aes_cmac(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_AesCmac)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
}
DEFINE_IID!(IID_IMacAlgorithmProvider, 1245693379, 7357, 16846, 160, 146, 170, 11, 197, 210, 210, 245);
RT_INTERFACE!{interface IMacAlgorithmProvider(IMacAlgorithmProviderVtbl): IInspectable(IInspectableVtbl) [IID_IMacAlgorithmProvider] {
    fn get_AlgorithmName(&self, out: *mut HSTRING) -> HRESULT,
    fn get_MacLength(&self, out: *mut u32) -> HRESULT,
    #[cfg(feature="windows-storage")] fn CreateKey(&self, keyMaterial: *mut ::rt::gen::windows::storage::streams::IBuffer, out: *mut *mut CryptographicKey) -> HRESULT
}}
impl IMacAlgorithmProvider {
    #[inline] pub unsafe fn get_algorithm_name(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_AlgorithmName)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_mac_length(&self) -> Result<u32> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_MacLength)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[cfg(feature="windows-storage")] #[inline] pub unsafe fn create_key(&self, keyMaterial: &::rt::gen::windows::storage::streams::IBuffer) -> Result<ComPtr<CryptographicKey>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).CreateKey)(self as *const _ as *mut _, keyMaterial as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
RT_CLASS!{class MacAlgorithmProvider: IMacAlgorithmProvider}
impl RtActivatable<IMacAlgorithmProviderStatics> for MacAlgorithmProvider {}
impl MacAlgorithmProvider {
    #[inline] pub fn open_algorithm(algorithm: &HStringArg) -> Result<ComPtr<MacAlgorithmProvider>> { unsafe {
        <Self as RtActivatable<IMacAlgorithmProviderStatics>>::get_activation_factory().open_algorithm(algorithm)
    }}
}
DEFINE_CLSID!(MacAlgorithmProvider(&[87,105,110,100,111,119,115,46,83,101,99,117,114,105,116,121,46,67,114,121,112,116,111,103,114,97,112,104,121,46,67,111,114,101,46,77,97,99,65,108,103,111,114,105,116,104,109,80,114,111,118,105,100,101,114,0]) [CLSID_MacAlgorithmProvider]);
DEFINE_IID!(IID_IMacAlgorithmProvider2, 1839409685, 55601, 17133, 142, 126, 195, 1, 202, 238, 17, 156);
RT_INTERFACE!{interface IMacAlgorithmProvider2(IMacAlgorithmProvider2Vtbl): IInspectable(IInspectableVtbl) [IID_IMacAlgorithmProvider2] {
    #[cfg(feature="windows-storage")] fn CreateHash(&self, keyMaterial: *mut ::rt::gen::windows::storage::streams::IBuffer, out: *mut *mut CryptographicHash) -> HRESULT
}}
impl IMacAlgorithmProvider2 {
    #[cfg(feature="windows-storage")] #[inline] pub unsafe fn create_hash(&self, keyMaterial: &::rt::gen::windows::storage::streams::IBuffer) -> Result<ComPtr<CryptographicHash>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).CreateHash)(self as *const _ as *mut _, keyMaterial as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
DEFINE_IID!(IID_IMacAlgorithmProviderStatics, 3384656199, 52343, 19952, 158, 78, 185, 33, 224, 128, 100, 76);
RT_INTERFACE!{static interface IMacAlgorithmProviderStatics(IMacAlgorithmProviderStaticsVtbl): IInspectable(IInspectableVtbl) [IID_IMacAlgorithmProviderStatics] {
    fn OpenAlgorithm(&self, algorithm: HSTRING, out: *mut *mut MacAlgorithmProvider) -> HRESULT
}}
impl IMacAlgorithmProviderStatics {
    #[inline] pub unsafe fn open_algorithm(&self, algorithm: &HStringArg) -> Result<ComPtr<MacAlgorithmProvider>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).OpenAlgorithm)(self as *const _ as *mut _, algorithm.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
RT_CLASS!{static class PersistedKeyProvider}
impl RtActivatable<IPersistedKeyProviderStatics> for PersistedKeyProvider {}
impl PersistedKeyProvider {
    #[inline] pub fn open_key_pair_from_certificate_async(certificate: &super::certificates::Certificate, hashAlgorithmName: &HStringArg, padding: CryptographicPadding) -> Result<ComPtr<::rt::gen::windows::foundation::IAsyncOperation<CryptographicKey>>> { unsafe {
        <Self as RtActivatable<IPersistedKeyProviderStatics>>::get_activation_factory().open_key_pair_from_certificate_async(certificate, hashAlgorithmName, padding)
    }}
    #[inline] pub fn open_public_key_from_certificate(certificate: &super::certificates::Certificate, hashAlgorithmName: &HStringArg, padding: CryptographicPadding) -> Result<ComPtr<CryptographicKey>> { unsafe {
        <Self as RtActivatable<IPersistedKeyProviderStatics>>::get_activation_factory().open_public_key_from_certificate(certificate, hashAlgorithmName, padding)
    }}
}
DEFINE_CLSID!(PersistedKeyProvider(&[87,105,110,100,111,119,115,46,83,101,99,117,114,105,116,121,46,67,114,121,112,116,111,103,114,97,112,104,121,46,67,111,114,101,46,80,101,114,115,105,115,116,101,100,75,101,121,80,114,111,118,105,100,101,114,0]) [CLSID_PersistedKeyProvider]);
DEFINE_IID!(IID_IPersistedKeyProviderStatics, 1999063060, 55764, 19701, 182, 104, 224, 69, 125, 243, 8, 148);
RT_INTERFACE!{static interface IPersistedKeyProviderStatics(IPersistedKeyProviderStaticsVtbl): IInspectable(IInspectableVtbl) [IID_IPersistedKeyProviderStatics] {
    fn OpenKeyPairFromCertificateAsync(&self, certificate: *mut super::certificates::Certificate, hashAlgorithmName: HSTRING, padding: CryptographicPadding, out: *mut *mut ::rt::gen::windows::foundation::IAsyncOperation<CryptographicKey>) -> HRESULT,
    fn OpenPublicKeyFromCertificate(&self, certificate: *mut super::certificates::Certificate, hashAlgorithmName: HSTRING, padding: CryptographicPadding, out: *mut *mut CryptographicKey) -> HRESULT
}}
impl IPersistedKeyProviderStatics {
    #[inline] pub unsafe fn open_key_pair_from_certificate_async(&self, certificate: &super::certificates::Certificate, hashAlgorithmName: &HStringArg, padding: CryptographicPadding) -> Result<ComPtr<::rt::gen::windows::foundation::IAsyncOperation<CryptographicKey>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).OpenKeyPairFromCertificateAsync)(self as *const _ as *mut _, certificate as *const _ as *mut _, hashAlgorithmName.get(), padding, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn open_public_key_from_certificate(&self, certificate: &super::certificates::Certificate, hashAlgorithmName: &HStringArg, padding: CryptographicPadding) -> Result<ComPtr<CryptographicKey>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).OpenPublicKeyFromCertificate)(self as *const _ as *mut _, certificate as *const _ as *mut _, hashAlgorithmName.get(), padding, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
RT_CLASS!{static class SymmetricAlgorithmNames}
impl RtActivatable<ISymmetricAlgorithmNamesStatics> for SymmetricAlgorithmNames {}
impl SymmetricAlgorithmNames {
    #[inline] pub fn get_des_cbc() -> Result<HString> { unsafe {
        <Self as RtActivatable<ISymmetricAlgorithmNamesStatics>>::get_activation_factory().get_des_cbc()
    }}
    #[inline] pub fn get_des_ecb() -> Result<HString> { unsafe {
        <Self as RtActivatable<ISymmetricAlgorithmNamesStatics>>::get_activation_factory().get_des_ecb()
    }}
    #[inline] pub fn get_triple_des_cbc() -> Result<HString> { unsafe {
        <Self as RtActivatable<ISymmetricAlgorithmNamesStatics>>::get_activation_factory().get_triple_des_cbc()
    }}
    #[inline] pub fn get_triple_des_ecb() -> Result<HString> { unsafe {
        <Self as RtActivatable<ISymmetricAlgorithmNamesStatics>>::get_activation_factory().get_triple_des_ecb()
    }}
    #[inline] pub fn get_rc2_cbc() -> Result<HString> { unsafe {
        <Self as RtActivatable<ISymmetricAlgorithmNamesStatics>>::get_activation_factory().get_rc2_cbc()
    }}
    #[inline] pub fn get_rc2_ecb() -> Result<HString> { unsafe {
        <Self as RtActivatable<ISymmetricAlgorithmNamesStatics>>::get_activation_factory().get_rc2_ecb()
    }}
    #[inline] pub fn get_aes_cbc() -> Result<HString> { unsafe {
        <Self as RtActivatable<ISymmetricAlgorithmNamesStatics>>::get_activation_factory().get_aes_cbc()
    }}
    #[inline] pub fn get_aes_ecb() -> Result<HString> { unsafe {
        <Self as RtActivatable<ISymmetricAlgorithmNamesStatics>>::get_activation_factory().get_aes_ecb()
    }}
    #[inline] pub fn get_aes_gcm() -> Result<HString> { unsafe {
        <Self as RtActivatable<ISymmetricAlgorithmNamesStatics>>::get_activation_factory().get_aes_gcm()
    }}
    #[inline] pub fn get_aes_ccm() -> Result<HString> { unsafe {
        <Self as RtActivatable<ISymmetricAlgorithmNamesStatics>>::get_activation_factory().get_aes_ccm()
    }}
    #[inline] pub fn get_aes_cbc_pkcs7() -> Result<HString> { unsafe {
        <Self as RtActivatable<ISymmetricAlgorithmNamesStatics>>::get_activation_factory().get_aes_cbc_pkcs7()
    }}
    #[inline] pub fn get_aes_ecb_pkcs7() -> Result<HString> { unsafe {
        <Self as RtActivatable<ISymmetricAlgorithmNamesStatics>>::get_activation_factory().get_aes_ecb_pkcs7()
    }}
    #[inline] pub fn get_des_cbc_pkcs7() -> Result<HString> { unsafe {
        <Self as RtActivatable<ISymmetricAlgorithmNamesStatics>>::get_activation_factory().get_des_cbc_pkcs7()
    }}
    #[inline] pub fn get_des_ecb_pkcs7() -> Result<HString> { unsafe {
        <Self as RtActivatable<ISymmetricAlgorithmNamesStatics>>::get_activation_factory().get_des_ecb_pkcs7()
    }}
    #[inline] pub fn get_triple_des_cbc_pkcs7() -> Result<HString> { unsafe {
        <Self as RtActivatable<ISymmetricAlgorithmNamesStatics>>::get_activation_factory().get_triple_des_cbc_pkcs7()
    }}
    #[inline] pub fn get_triple_des_ecb_pkcs7() -> Result<HString> { unsafe {
        <Self as RtActivatable<ISymmetricAlgorithmNamesStatics>>::get_activation_factory().get_triple_des_ecb_pkcs7()
    }}
    #[inline] pub fn get_rc2_cbc_pkcs7() -> Result<HString> { unsafe {
        <Self as RtActivatable<ISymmetricAlgorithmNamesStatics>>::get_activation_factory().get_rc2_cbc_pkcs7()
    }}
    #[inline] pub fn get_rc2_ecb_pkcs7() -> Result<HString> { unsafe {
        <Self as RtActivatable<ISymmetricAlgorithmNamesStatics>>::get_activation_factory().get_rc2_ecb_pkcs7()
    }}
    #[inline] pub fn get_rc4() -> Result<HString> { unsafe {
        <Self as RtActivatable<ISymmetricAlgorithmNamesStatics>>::get_activation_factory().get_rc4()
    }}
}
DEFINE_CLSID!(SymmetricAlgorithmNames(&[87,105,110,100,111,119,115,46,83,101,99,117,114,105,116,121,46,67,114,121,112,116,111,103,114,97,112,104,121,46,67,111,114,101,46,83,121,109,109,101,116,114,105,99,65,108,103,111,114,105,116,104,109,78,97,109,101,115,0]) [CLSID_SymmetricAlgorithmNames]);
DEFINE_IID!(IID_ISymmetricAlgorithmNamesStatics, 1752199803, 51606, 20142, 132, 215, 121, 178, 174, 183, 59, 156);
RT_INTERFACE!{static interface ISymmetricAlgorithmNamesStatics(ISymmetricAlgorithmNamesStaticsVtbl): IInspectable(IInspectableVtbl) [IID_ISymmetricAlgorithmNamesStatics] {
    fn get_DesCbc(&self, out: *mut HSTRING) -> HRESULT,
    fn get_DesEcb(&self, out: *mut HSTRING) -> HRESULT,
    fn get_TripleDesCbc(&self, out: *mut HSTRING) -> HRESULT,
    fn get_TripleDesEcb(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Rc2Cbc(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Rc2Ecb(&self, out: *mut HSTRING) -> HRESULT,
    fn get_AesCbc(&self, out: *mut HSTRING) -> HRESULT,
    fn get_AesEcb(&self, out: *mut HSTRING) -> HRESULT,
    fn get_AesGcm(&self, out: *mut HSTRING) -> HRESULT,
    fn get_AesCcm(&self, out: *mut HSTRING) -> HRESULT,
    fn get_AesCbcPkcs7(&self, out: *mut HSTRING) -> HRESULT,
    fn get_AesEcbPkcs7(&self, out: *mut HSTRING) -> HRESULT,
    fn get_DesCbcPkcs7(&self, out: *mut HSTRING) -> HRESULT,
    fn get_DesEcbPkcs7(&self, out: *mut HSTRING) -> HRESULT,
    fn get_TripleDesCbcPkcs7(&self, out: *mut HSTRING) -> HRESULT,
    fn get_TripleDesEcbPkcs7(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Rc2CbcPkcs7(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Rc2EcbPkcs7(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Rc4(&self, out: *mut HSTRING) -> HRESULT
}}
impl ISymmetricAlgorithmNamesStatics {
    #[inline] pub unsafe fn get_des_cbc(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_DesCbc)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_des_ecb(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_DesEcb)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_triple_des_cbc(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_TripleDesCbc)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_triple_des_ecb(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_TripleDesEcb)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_rc2_cbc(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Rc2Cbc)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_rc2_ecb(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Rc2Ecb)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_aes_cbc(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_AesCbc)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_aes_ecb(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_AesEcb)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_aes_gcm(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_AesGcm)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_aes_ccm(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_AesCcm)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_aes_cbc_pkcs7(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_AesCbcPkcs7)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_aes_ecb_pkcs7(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_AesEcbPkcs7)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_des_cbc_pkcs7(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_DesCbcPkcs7)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_des_ecb_pkcs7(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_DesEcbPkcs7)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_triple_des_cbc_pkcs7(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_TripleDesCbcPkcs7)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_triple_des_ecb_pkcs7(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_TripleDesEcbPkcs7)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_rc2_cbc_pkcs7(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Rc2CbcPkcs7)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_rc2_ecb_pkcs7(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Rc2EcbPkcs7)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_rc4(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Rc4)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
}
DEFINE_IID!(IID_ISymmetricKeyAlgorithmProvider, 1031686707, 15312, 18690, 138, 200, 71, 13, 80, 210, 19, 118);
RT_INTERFACE!{interface ISymmetricKeyAlgorithmProvider(ISymmetricKeyAlgorithmProviderVtbl): IInspectable(IInspectableVtbl) [IID_ISymmetricKeyAlgorithmProvider] {
    fn get_AlgorithmName(&self, out: *mut HSTRING) -> HRESULT,
    fn get_BlockLength(&self, out: *mut u32) -> HRESULT,
    #[cfg(feature="windows-storage")] fn CreateSymmetricKey(&self, keyMaterial: *mut ::rt::gen::windows::storage::streams::IBuffer, out: *mut *mut CryptographicKey) -> HRESULT
}}
impl ISymmetricKeyAlgorithmProvider {
    #[inline] pub unsafe fn get_algorithm_name(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_AlgorithmName)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_block_length(&self) -> Result<u32> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_BlockLength)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[cfg(feature="windows-storage")] #[inline] pub unsafe fn create_symmetric_key(&self, keyMaterial: &::rt::gen::windows::storage::streams::IBuffer) -> Result<ComPtr<CryptographicKey>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).CreateSymmetricKey)(self as *const _ as *mut _, keyMaterial as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
RT_CLASS!{class SymmetricKeyAlgorithmProvider: ISymmetricKeyAlgorithmProvider}
impl RtActivatable<ISymmetricKeyAlgorithmProviderStatics> for SymmetricKeyAlgorithmProvider {}
impl SymmetricKeyAlgorithmProvider {
    #[inline] pub fn open_algorithm(algorithm: &HStringArg) -> Result<ComPtr<SymmetricKeyAlgorithmProvider>> { unsafe {
        <Self as RtActivatable<ISymmetricKeyAlgorithmProviderStatics>>::get_activation_factory().open_algorithm(algorithm)
    }}
}
DEFINE_CLSID!(SymmetricKeyAlgorithmProvider(&[87,105,110,100,111,119,115,46,83,101,99,117,114,105,116,121,46,67,114,121,112,116,111,103,114,97,112,104,121,46,67,111,114,101,46,83,121,109,109,101,116,114,105,99,75,101,121,65,108,103,111,114,105,116,104,109,80,114,111,118,105,100,101,114,0]) [CLSID_SymmetricKeyAlgorithmProvider]);
DEFINE_IID!(IID_ISymmetricKeyAlgorithmProviderStatics, 2369463078, 7991, 18719, 182, 14, 245, 67, 27, 38, 180, 131);
RT_INTERFACE!{static interface ISymmetricKeyAlgorithmProviderStatics(ISymmetricKeyAlgorithmProviderStaticsVtbl): IInspectable(IInspectableVtbl) [IID_ISymmetricKeyAlgorithmProviderStatics] {
    fn OpenAlgorithm(&self, algorithm: HSTRING, out: *mut *mut SymmetricKeyAlgorithmProvider) -> HRESULT
}}
impl ISymmetricKeyAlgorithmProviderStatics {
    #[inline] pub unsafe fn open_algorithm(&self, algorithm: &HStringArg) -> Result<ComPtr<SymmetricKeyAlgorithmProvider>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).OpenAlgorithm)(self as *const _ as *mut _, algorithm.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
} // Windows.Security.Cryptography.Core
pub mod dataprotection { // Windows.Security.Cryptography.DataProtection
use ::prelude::*;
DEFINE_IID!(IID_IDataProtectionProvider, 157522248, 60706, 17008, 189, 28, 109, 114, 192, 15, 135, 135);
RT_INTERFACE!{interface IDataProtectionProvider(IDataProtectionProviderVtbl): IInspectable(IInspectableVtbl) [IID_IDataProtectionProvider] {
    #[cfg(feature="windows-storage")] fn ProtectAsync(&self, data: *mut ::rt::gen::windows::storage::streams::IBuffer, out: *mut *mut ::rt::gen::windows::foundation::IAsyncOperation<::rt::gen::windows::storage::streams::IBuffer>) -> HRESULT,
    #[cfg(feature="windows-storage")] fn UnprotectAsync(&self, data: *mut ::rt::gen::windows::storage::streams::IBuffer, out: *mut *mut ::rt::gen::windows::foundation::IAsyncOperation<::rt::gen::windows::storage::streams::IBuffer>) -> HRESULT,
    #[cfg(feature="windows-storage")] fn ProtectStreamAsync(&self, src: *mut ::rt::gen::windows::storage::streams::IInputStream, dest: *mut ::rt::gen::windows::storage::streams::IOutputStream, out: *mut *mut ::rt::gen::windows::foundation::IAsyncAction) -> HRESULT,
    #[cfg(feature="windows-storage")] fn UnprotectStreamAsync(&self, src: *mut ::rt::gen::windows::storage::streams::IInputStream, dest: *mut ::rt::gen::windows::storage::streams::IOutputStream, out: *mut *mut ::rt::gen::windows::foundation::IAsyncAction) -> HRESULT
}}
impl IDataProtectionProvider {
    #[cfg(feature="windows-storage")] #[inline] pub unsafe fn protect_async(&self, data: &::rt::gen::windows::storage::streams::IBuffer) -> Result<ComPtr<::rt::gen::windows::foundation::IAsyncOperation<::rt::gen::windows::storage::streams::IBuffer>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).ProtectAsync)(self as *const _ as *mut _, data as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[cfg(feature="windows-storage")] #[inline] pub unsafe fn unprotect_async(&self, data: &::rt::gen::windows::storage::streams::IBuffer) -> Result<ComPtr<::rt::gen::windows::foundation::IAsyncOperation<::rt::gen::windows::storage::streams::IBuffer>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).UnprotectAsync)(self as *const _ as *mut _, data as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[cfg(feature="windows-storage")] #[inline] pub unsafe fn protect_stream_async(&self, src: &::rt::gen::windows::storage::streams::IInputStream, dest: &::rt::gen::windows::storage::streams::IOutputStream) -> Result<ComPtr<::rt::gen::windows::foundation::IAsyncAction>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).ProtectStreamAsync)(self as *const _ as *mut _, src as *const _ as *mut _, dest as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[cfg(feature="windows-storage")] #[inline] pub unsafe fn unprotect_stream_async(&self, src: &::rt::gen::windows::storage::streams::IInputStream, dest: &::rt::gen::windows::storage::streams::IOutputStream) -> Result<ComPtr<::rt::gen::windows::foundation::IAsyncAction>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).UnprotectStreamAsync)(self as *const _ as *mut _, src as *const _ as *mut _, dest as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
RT_CLASS!{class DataProtectionProvider: IDataProtectionProvider}
impl RtActivatable<IDataProtectionProviderFactory> for DataProtectionProvider {}
impl RtActivatable<IActivationFactory> for DataProtectionProvider {}
impl DataProtectionProvider {
    #[inline] pub fn create_overload_explicit(protectionDescriptor: &HStringArg) -> Result<ComPtr<DataProtectionProvider>> { unsafe {
        <Self as RtActivatable<IDataProtectionProviderFactory>>::get_activation_factory().create_overload_explicit(protectionDescriptor)
    }}
}
DEFINE_CLSID!(DataProtectionProvider(&[87,105,110,100,111,119,115,46,83,101,99,117,114,105,116,121,46,67,114,121,112,116,111,103,114,97,112,104,121,46,68,97,116,97,80,114,111,116,101,99,116,105,111,110,46,68,97,116,97,80,114,111,116,101,99,116,105,111,110,80,114,111,118,105,100,101,114,0]) [CLSID_DataProtectionProvider]);
DEFINE_IID!(IID_IDataProtectionProviderFactory, 2918399404, 18738, 19679, 172, 65, 114, 20, 51, 53, 20, 202);
RT_INTERFACE!{static interface IDataProtectionProviderFactory(IDataProtectionProviderFactoryVtbl): IInspectable(IInspectableVtbl) [IID_IDataProtectionProviderFactory] {
    fn CreateOverloadExplicit(&self, protectionDescriptor: HSTRING, out: *mut *mut DataProtectionProvider) -> HRESULT
}}
impl IDataProtectionProviderFactory {
    #[inline] pub unsafe fn create_overload_explicit(&self, protectionDescriptor: &HStringArg) -> Result<ComPtr<DataProtectionProvider>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).CreateOverloadExplicit)(self as *const _ as *mut _, protectionDescriptor.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
} // Windows.Security.Cryptography.DataProtection
} // Windows.Security.Cryptography
pub mod enterprisedata { // Windows.Security.EnterpriseData
use ::prelude::*;
DEFINE_IID!(IID_IBufferProtectUnprotectResult, 1201233628, 27884, 20026, 178, 81, 158, 116, 133, 215, 158, 122);
RT_INTERFACE!{interface IBufferProtectUnprotectResult(IBufferProtectUnprotectResultVtbl): IInspectable(IInspectableVtbl) [IID_IBufferProtectUnprotectResult] {
    #[cfg(not(feature="windows-storage"))] fn __Dummy0(&self) -> (),
    #[cfg(feature="windows-storage")] fn get_Buffer(&self, out: *mut *mut super::super::storage::streams::IBuffer) -> HRESULT,
    fn get_ProtectionInfo(&self, out: *mut *mut DataProtectionInfo) -> HRESULT
}}
impl IBufferProtectUnprotectResult {
    #[cfg(feature="windows-storage")] #[inline] pub unsafe fn get_buffer(&self) -> Result<ComPtr<super::super::storage::streams::IBuffer>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Buffer)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_protection_info(&self) -> Result<ComPtr<DataProtectionInfo>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_ProtectionInfo)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
RT_CLASS!{class BufferProtectUnprotectResult: IBufferProtectUnprotectResult}
DEFINE_IID!(IID_IDataProtectionInfo, 2216734913, 24113, 17413, 149, 64, 63, 148, 58, 240, 203, 38);
RT_INTERFACE!{interface IDataProtectionInfo(IDataProtectionInfoVtbl): IInspectable(IInspectableVtbl) [IID_IDataProtectionInfo] {
    fn get_Status(&self, out: *mut DataProtectionStatus) -> HRESULT,
    fn get_Identity(&self, out: *mut HSTRING) -> HRESULT
}}
impl IDataProtectionInfo {
    #[inline] pub unsafe fn get_status(&self) -> Result<DataProtectionStatus> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_Status)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_identity(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Identity)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
}
RT_CLASS!{class DataProtectionInfo: IDataProtectionInfo}
RT_CLASS!{static class DataProtectionManager}
impl RtActivatable<IDataProtectionManagerStatics> for DataProtectionManager {}
impl DataProtectionManager {
    #[cfg(feature="windows-storage")] #[inline] pub fn protect_async(data: &super::super::storage::streams::IBuffer, identity: &HStringArg) -> Result<ComPtr<super::super::foundation::IAsyncOperation<BufferProtectUnprotectResult>>> { unsafe {
        <Self as RtActivatable<IDataProtectionManagerStatics>>::get_activation_factory().protect_async(data, identity)
    }}
    #[cfg(feature="windows-storage")] #[inline] pub fn unprotect_async(data: &super::super::storage::streams::IBuffer) -> Result<ComPtr<super::super::foundation::IAsyncOperation<BufferProtectUnprotectResult>>> { unsafe {
        <Self as RtActivatable<IDataProtectionManagerStatics>>::get_activation_factory().unprotect_async(data)
    }}
    #[cfg(feature="windows-storage")] #[inline] pub fn protect_stream_async(unprotectedStream: &super::super::storage::streams::IInputStream, identity: &HStringArg, protectedStream: &super::super::storage::streams::IOutputStream) -> Result<ComPtr<super::super::foundation::IAsyncOperation<DataProtectionInfo>>> { unsafe {
        <Self as RtActivatable<IDataProtectionManagerStatics>>::get_activation_factory().protect_stream_async(unprotectedStream, identity, protectedStream)
    }}
    #[cfg(feature="windows-storage")] #[inline] pub fn unprotect_stream_async(protectedStream: &super::super::storage::streams::IInputStream, unprotectedStream: &super::super::storage::streams::IOutputStream) -> Result<ComPtr<super::super::foundation::IAsyncOperation<DataProtectionInfo>>> { unsafe {
        <Self as RtActivatable<IDataProtectionManagerStatics>>::get_activation_factory().unprotect_stream_async(protectedStream, unprotectedStream)
    }}
    #[cfg(feature="windows-storage")] #[inline] pub fn get_protection_info_async(protectedData: &super::super::storage::streams::IBuffer) -> Result<ComPtr<super::super::foundation::IAsyncOperation<DataProtectionInfo>>> { unsafe {
        <Self as RtActivatable<IDataProtectionManagerStatics>>::get_activation_factory().get_protection_info_async(protectedData)
    }}
    #[cfg(feature="windows-storage")] #[inline] pub fn get_stream_protection_info_async(protectedStream: &super::super::storage::streams::IInputStream) -> Result<ComPtr<super::super::foundation::IAsyncOperation<DataProtectionInfo>>> { unsafe {
        <Self as RtActivatable<IDataProtectionManagerStatics>>::get_activation_factory().get_stream_protection_info_async(protectedStream)
    }}
}
DEFINE_CLSID!(DataProtectionManager(&[87,105,110,100,111,119,115,46,83,101,99,117,114,105,116,121,46,69,110,116,101,114,112,114,105,115,101,68,97,116,97,46,68,97,116,97,80,114,111,116,101,99,116,105,111,110,77,97,110,97,103,101,114,0]) [CLSID_DataProtectionManager]);
DEFINE_IID!(IID_IDataProtectionManagerStatics, 3054803828, 37188, 20196, 138, 138, 48, 181, 243, 97, 67, 14);
RT_INTERFACE!{static interface IDataProtectionManagerStatics(IDataProtectionManagerStaticsVtbl): IInspectable(IInspectableVtbl) [IID_IDataProtectionManagerStatics] {
    #[cfg(feature="windows-storage")] fn ProtectAsync(&self, data: *mut super::super::storage::streams::IBuffer, identity: HSTRING, out: *mut *mut super::super::foundation::IAsyncOperation<BufferProtectUnprotectResult>) -> HRESULT,
    #[cfg(feature="windows-storage")] fn UnprotectAsync(&self, data: *mut super::super::storage::streams::IBuffer, out: *mut *mut super::super::foundation::IAsyncOperation<BufferProtectUnprotectResult>) -> HRESULT,
    #[cfg(feature="windows-storage")] fn ProtectStreamAsync(&self, unprotectedStream: *mut super::super::storage::streams::IInputStream, identity: HSTRING, protectedStream: *mut super::super::storage::streams::IOutputStream, out: *mut *mut super::super::foundation::IAsyncOperation<DataProtectionInfo>) -> HRESULT,
    #[cfg(feature="windows-storage")] fn UnprotectStreamAsync(&self, protectedStream: *mut super::super::storage::streams::IInputStream, unprotectedStream: *mut super::super::storage::streams::IOutputStream, out: *mut *mut super::super::foundation::IAsyncOperation<DataProtectionInfo>) -> HRESULT,
    #[cfg(feature="windows-storage")] fn GetProtectionInfoAsync(&self, protectedData: *mut super::super::storage::streams::IBuffer, out: *mut *mut super::super::foundation::IAsyncOperation<DataProtectionInfo>) -> HRESULT,
    #[cfg(feature="windows-storage")] fn GetStreamProtectionInfoAsync(&self, protectedStream: *mut super::super::storage::streams::IInputStream, out: *mut *mut super::super::foundation::IAsyncOperation<DataProtectionInfo>) -> HRESULT
}}
impl IDataProtectionManagerStatics {
    #[cfg(feature="windows-storage")] #[inline] pub unsafe fn protect_async(&self, data: &super::super::storage::streams::IBuffer, identity: &HStringArg) -> Result<ComPtr<super::super::foundation::IAsyncOperation<BufferProtectUnprotectResult>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).ProtectAsync)(self as *const _ as *mut _, data as *const _ as *mut _, identity.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[cfg(feature="windows-storage")] #[inline] pub unsafe fn unprotect_async(&self, data: &super::super::storage::streams::IBuffer) -> Result<ComPtr<super::super::foundation::IAsyncOperation<BufferProtectUnprotectResult>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).UnprotectAsync)(self as *const _ as *mut _, data as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[cfg(feature="windows-storage")] #[inline] pub unsafe fn protect_stream_async(&self, unprotectedStream: &super::super::storage::streams::IInputStream, identity: &HStringArg, protectedStream: &super::super::storage::streams::IOutputStream) -> Result<ComPtr<super::super::foundation::IAsyncOperation<DataProtectionInfo>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).ProtectStreamAsync)(self as *const _ as *mut _, unprotectedStream as *const _ as *mut _, identity.get(), protectedStream as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[cfg(feature="windows-storage")] #[inline] pub unsafe fn unprotect_stream_async(&self, protectedStream: &super::super::storage::streams::IInputStream, unprotectedStream: &super::super::storage::streams::IOutputStream) -> Result<ComPtr<super::super::foundation::IAsyncOperation<DataProtectionInfo>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).UnprotectStreamAsync)(self as *const _ as *mut _, protectedStream as *const _ as *mut _, unprotectedStream as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[cfg(feature="windows-storage")] #[inline] pub unsafe fn get_protection_info_async(&self, protectedData: &super::super::storage::streams::IBuffer) -> Result<ComPtr<super::super::foundation::IAsyncOperation<DataProtectionInfo>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetProtectionInfoAsync)(self as *const _ as *mut _, protectedData as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[cfg(feature="windows-storage")] #[inline] pub unsafe fn get_stream_protection_info_async(&self, protectedStream: &super::super::storage::streams::IInputStream) -> Result<ComPtr<super::super::foundation::IAsyncOperation<DataProtectionInfo>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetStreamProtectionInfoAsync)(self as *const _ as *mut _, protectedStream as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
RT_ENUM! { enum DataProtectionStatus: i32 {
    ProtectedToOtherIdentity (DataProtectionStatus_ProtectedToOtherIdentity) = 0, Protected (DataProtectionStatus_Protected) = 1, Revoked (DataProtectionStatus_Revoked) = 2, Unprotected (DataProtectionStatus_Unprotected) = 3, LicenseExpired (DataProtectionStatus_LicenseExpired) = 4, AccessSuspended (DataProtectionStatus_AccessSuspended) = 5,
}}
RT_ENUM! { enum EnforcementLevel: i32 {
    NoProtection (EnforcementLevel_NoProtection) = 0, Silent (EnforcementLevel_Silent) = 1, Override (EnforcementLevel_Override) = 2, Block (EnforcementLevel_Block) = 3,
}}
DEFINE_IID!(IID_IFileProtectionInfo, 1323918470, 5246, 19920, 143, 175, 82, 83, 237, 145, 173, 12);
RT_INTERFACE!{interface IFileProtectionInfo(IFileProtectionInfoVtbl): IInspectable(IInspectableVtbl) [IID_IFileProtectionInfo] {
    fn get_Status(&self, out: *mut FileProtectionStatus) -> HRESULT,
    fn get_IsRoamable(&self, out: *mut bool) -> HRESULT,
    fn get_Identity(&self, out: *mut HSTRING) -> HRESULT
}}
impl IFileProtectionInfo {
    #[inline] pub unsafe fn get_status(&self) -> Result<FileProtectionStatus> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_Status)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_is_roamable(&self) -> Result<bool> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_IsRoamable)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_identity(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Identity)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
}
RT_CLASS!{class FileProtectionInfo: IFileProtectionInfo}
DEFINE_IID!(IID_IFileProtectionInfo2, 2182232652, 21882, 18829, 142, 148, 148, 76, 213, 131, 100, 50);
RT_INTERFACE!{interface IFileProtectionInfo2(IFileProtectionInfo2Vtbl): IInspectable(IInspectableVtbl) [IID_IFileProtectionInfo2] {
    fn get_IsProtectWhileOpenSupported(&self, out: *mut bool) -> HRESULT
}}
impl IFileProtectionInfo2 {
    #[inline] pub unsafe fn get_is_protect_while_open_supported(&self) -> Result<bool> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_IsProtectWhileOpenSupported)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
}
RT_CLASS!{static class FileProtectionManager}
impl RtActivatable<IFileProtectionManagerStatics> for FileProtectionManager {}
impl RtActivatable<IFileProtectionManagerStatics2> for FileProtectionManager {}
impl RtActivatable<IFileProtectionManagerStatics3> for FileProtectionManager {}
impl FileProtectionManager {
    #[cfg(feature="windows-storage")] #[inline] pub fn protect_async(target: &super::super::storage::IStorageItem, identity: &HStringArg) -> Result<ComPtr<super::super::foundation::IAsyncOperation<FileProtectionInfo>>> { unsafe {
        <Self as RtActivatable<IFileProtectionManagerStatics>>::get_activation_factory().protect_async(target, identity)
    }}
    #[cfg(feature="windows-storage")] #[inline] pub fn copy_protection_async(source: &super::super::storage::IStorageItem, target: &super::super::storage::IStorageItem) -> Result<ComPtr<super::super::foundation::IAsyncOperation<bool>>> { unsafe {
        <Self as RtActivatable<IFileProtectionManagerStatics>>::get_activation_factory().copy_protection_async(source, target)
    }}
    #[cfg(feature="windows-storage")] #[inline] pub fn get_protection_info_async(source: &super::super::storage::IStorageItem) -> Result<ComPtr<super::super::foundation::IAsyncOperation<FileProtectionInfo>>> { unsafe {
        <Self as RtActivatable<IFileProtectionManagerStatics>>::get_activation_factory().get_protection_info_async(source)
    }}
    #[cfg(feature="windows-storage")] #[inline] pub fn save_file_as_container_async(protectedFile: &super::super::storage::IStorageFile) -> Result<ComPtr<super::super::foundation::IAsyncOperation<ProtectedContainerExportResult>>> { unsafe {
        <Self as RtActivatable<IFileProtectionManagerStatics>>::get_activation_factory().save_file_as_container_async(protectedFile)
    }}
    #[cfg(feature="windows-storage")] #[inline] pub fn load_file_from_container_async(containerFile: &super::super::storage::IStorageFile) -> Result<ComPtr<super::super::foundation::IAsyncOperation<ProtectedContainerImportResult>>> { unsafe {
        <Self as RtActivatable<IFileProtectionManagerStatics>>::get_activation_factory().load_file_from_container_async(containerFile)
    }}
    #[cfg(feature="windows-storage")] #[inline] pub fn load_file_from_container_with_target_async(containerFile: &super::super::storage::IStorageFile, target: &super::super::storage::IStorageItem) -> Result<ComPtr<super::super::foundation::IAsyncOperation<ProtectedContainerImportResult>>> { unsafe {
        <Self as RtActivatable<IFileProtectionManagerStatics>>::get_activation_factory().load_file_from_container_with_target_async(containerFile, target)
    }}
    #[cfg(feature="windows-storage")] #[inline] pub fn create_protected_and_open_async(parentFolder: &super::super::storage::IStorageFolder, desiredName: &HStringArg, identity: &HStringArg, collisionOption: super::super::storage::CreationCollisionOption) -> Result<ComPtr<super::super::foundation::IAsyncOperation<ProtectedFileCreateResult>>> { unsafe {
        <Self as RtActivatable<IFileProtectionManagerStatics>>::get_activation_factory().create_protected_and_open_async(parentFolder, desiredName, identity, collisionOption)
    }}
    #[cfg(feature="windows-storage")] #[inline] pub fn is_container_async(file: &super::super::storage::IStorageFile) -> Result<ComPtr<super::super::foundation::IAsyncOperation<bool>>> { unsafe {
        <Self as RtActivatable<IFileProtectionManagerStatics2>>::get_activation_factory().is_container_async(file)
    }}
    #[cfg(feature="windows-storage")] #[inline] pub fn load_file_from_container_with_target_and_name_collision_option_async(containerFile: &super::super::storage::IStorageFile, target: &super::super::storage::IStorageItem, collisionOption: super::super::storage::NameCollisionOption) -> Result<ComPtr<super::super::foundation::IAsyncOperation<ProtectedContainerImportResult>>> { unsafe {
        <Self as RtActivatable<IFileProtectionManagerStatics2>>::get_activation_factory().load_file_from_container_with_target_and_name_collision_option_async(containerFile, target, collisionOption)
    }}
    #[cfg(feature="windows-storage")] #[inline] pub fn save_file_as_container_with_sharing_async(protectedFile: &super::super::storage::IStorageFile, sharedWithIdentities: &super::super::foundation::collections::IIterable<HString>) -> Result<ComPtr<super::super::foundation::IAsyncOperation<ProtectedContainerExportResult>>> { unsafe {
        <Self as RtActivatable<IFileProtectionManagerStatics2>>::get_activation_factory().save_file_as_container_with_sharing_async(protectedFile, sharedWithIdentities)
    }}
    #[cfg(feature="windows-storage")] #[inline] pub fn unprotect_async(target: &super::super::storage::IStorageItem) -> Result<ComPtr<super::super::foundation::IAsyncOperation<FileProtectionInfo>>> { unsafe {
        <Self as RtActivatable<IFileProtectionManagerStatics3>>::get_activation_factory().unprotect_async(target)
    }}
    #[cfg(feature="windows-storage")] #[inline] pub fn unprotect_with_options_async(target: &super::super::storage::IStorageItem, options: &FileUnprotectOptions) -> Result<ComPtr<super::super::foundation::IAsyncOperation<FileProtectionInfo>>> { unsafe {
        <Self as RtActivatable<IFileProtectionManagerStatics3>>::get_activation_factory().unprotect_with_options_async(target, options)
    }}
}
DEFINE_CLSID!(FileProtectionManager(&[87,105,110,100,111,119,115,46,83,101,99,117,114,105,116,121,46,69,110,116,101,114,112,114,105,115,101,68,97,116,97,46,70,105,108,101,80,114,111,116,101,99,116,105,111,110,77,97,110,97,103,101,114,0]) [CLSID_FileProtectionManager]);
DEFINE_IID!(IID_IFileProtectionManagerStatics, 1481047195, 58899, 17003, 187, 56, 136, 203, 161, 220, 154, 219);
RT_INTERFACE!{static interface IFileProtectionManagerStatics(IFileProtectionManagerStaticsVtbl): IInspectable(IInspectableVtbl) [IID_IFileProtectionManagerStatics] {
    #[cfg(feature="windows-storage")] fn ProtectAsync(&self, target: *mut super::super::storage::IStorageItem, identity: HSTRING, out: *mut *mut super::super::foundation::IAsyncOperation<FileProtectionInfo>) -> HRESULT,
    #[cfg(feature="windows-storage")] fn CopyProtectionAsync(&self, source: *mut super::super::storage::IStorageItem, target: *mut super::super::storage::IStorageItem, out: *mut *mut super::super::foundation::IAsyncOperation<bool>) -> HRESULT,
    #[cfg(feature="windows-storage")] fn GetProtectionInfoAsync(&self, source: *mut super::super::storage::IStorageItem, out: *mut *mut super::super::foundation::IAsyncOperation<FileProtectionInfo>) -> HRESULT,
    #[cfg(feature="windows-storage")] fn SaveFileAsContainerAsync(&self, protectedFile: *mut super::super::storage::IStorageFile, out: *mut *mut super::super::foundation::IAsyncOperation<ProtectedContainerExportResult>) -> HRESULT,
    #[cfg(feature="windows-storage")] fn LoadFileFromContainerAsync(&self, containerFile: *mut super::super::storage::IStorageFile, out: *mut *mut super::super::foundation::IAsyncOperation<ProtectedContainerImportResult>) -> HRESULT,
    #[cfg(feature="windows-storage")] fn LoadFileFromContainerWithTargetAsync(&self, containerFile: *mut super::super::storage::IStorageFile, target: *mut super::super::storage::IStorageItem, out: *mut *mut super::super::foundation::IAsyncOperation<ProtectedContainerImportResult>) -> HRESULT,
    #[cfg(feature="windows-storage")] fn CreateProtectedAndOpenAsync(&self, parentFolder: *mut super::super::storage::IStorageFolder, desiredName: HSTRING, identity: HSTRING, collisionOption: super::super::storage::CreationCollisionOption, out: *mut *mut super::super::foundation::IAsyncOperation<ProtectedFileCreateResult>) -> HRESULT
}}
impl IFileProtectionManagerStatics {
    #[cfg(feature="windows-storage")] #[inline] pub unsafe fn protect_async(&self, target: &super::super::storage::IStorageItem, identity: &HStringArg) -> Result<ComPtr<super::super::foundation::IAsyncOperation<FileProtectionInfo>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).ProtectAsync)(self as *const _ as *mut _, target as *const _ as *mut _, identity.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[cfg(feature="windows-storage")] #[inline] pub unsafe fn copy_protection_async(&self, source: &super::super::storage::IStorageItem, target: &super::super::storage::IStorageItem) -> Result<ComPtr<super::super::foundation::IAsyncOperation<bool>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).CopyProtectionAsync)(self as *const _ as *mut _, source as *const _ as *mut _, target as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[cfg(feature="windows-storage")] #[inline] pub unsafe fn get_protection_info_async(&self, source: &super::super::storage::IStorageItem) -> Result<ComPtr<super::super::foundation::IAsyncOperation<FileProtectionInfo>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetProtectionInfoAsync)(self as *const _ as *mut _, source as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[cfg(feature="windows-storage")] #[inline] pub unsafe fn save_file_as_container_async(&self, protectedFile: &super::super::storage::IStorageFile) -> Result<ComPtr<super::super::foundation::IAsyncOperation<ProtectedContainerExportResult>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).SaveFileAsContainerAsync)(self as *const _ as *mut _, protectedFile as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[cfg(feature="windows-storage")] #[inline] pub unsafe fn load_file_from_container_async(&self, containerFile: &super::super::storage::IStorageFile) -> Result<ComPtr<super::super::foundation::IAsyncOperation<ProtectedContainerImportResult>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).LoadFileFromContainerAsync)(self as *const _ as *mut _, containerFile as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[cfg(feature="windows-storage")] #[inline] pub unsafe fn load_file_from_container_with_target_async(&self, containerFile: &super::super::storage::IStorageFile, target: &super::super::storage::IStorageItem) -> Result<ComPtr<super::super::foundation::IAsyncOperation<ProtectedContainerImportResult>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).LoadFileFromContainerWithTargetAsync)(self as *const _ as *mut _, containerFile as *const _ as *mut _, target as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[cfg(feature="windows-storage")] #[inline] pub unsafe fn create_protected_and_open_async(&self, parentFolder: &super::super::storage::IStorageFolder, desiredName: &HStringArg, identity: &HStringArg, collisionOption: super::super::storage::CreationCollisionOption) -> Result<ComPtr<super::super::foundation::IAsyncOperation<ProtectedFileCreateResult>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).CreateProtectedAndOpenAsync)(self as *const _ as *mut _, parentFolder as *const _ as *mut _, desiredName.get(), identity.get(), collisionOption, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
DEFINE_IID!(IID_IFileProtectionManagerStatics2, 2211620677, 1155, 16811, 178, 213, 188, 127, 35, 215, 78, 187);
RT_INTERFACE!{static interface IFileProtectionManagerStatics2(IFileProtectionManagerStatics2Vtbl): IInspectable(IInspectableVtbl) [IID_IFileProtectionManagerStatics2] {
    #[cfg(feature="windows-storage")] fn IsContainerAsync(&self, file: *mut super::super::storage::IStorageFile, out: *mut *mut super::super::foundation::IAsyncOperation<bool>) -> HRESULT,
    #[cfg(feature="windows-storage")] fn LoadFileFromContainerWithTargetAndNameCollisionOptionAsync(&self, containerFile: *mut super::super::storage::IStorageFile, target: *mut super::super::storage::IStorageItem, collisionOption: super::super::storage::NameCollisionOption, out: *mut *mut super::super::foundation::IAsyncOperation<ProtectedContainerImportResult>) -> HRESULT,
    #[cfg(feature="windows-storage")] fn SaveFileAsContainerWithSharingAsync(&self, protectedFile: *mut super::super::storage::IStorageFile, sharedWithIdentities: *mut super::super::foundation::collections::IIterable<HString>, out: *mut *mut super::super::foundation::IAsyncOperation<ProtectedContainerExportResult>) -> HRESULT
}}
impl IFileProtectionManagerStatics2 {
    #[cfg(feature="windows-storage")] #[inline] pub unsafe fn is_container_async(&self, file: &super::super::storage::IStorageFile) -> Result<ComPtr<super::super::foundation::IAsyncOperation<bool>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).IsContainerAsync)(self as *const _ as *mut _, file as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[cfg(feature="windows-storage")] #[inline] pub unsafe fn load_file_from_container_with_target_and_name_collision_option_async(&self, containerFile: &super::super::storage::IStorageFile, target: &super::super::storage::IStorageItem, collisionOption: super::super::storage::NameCollisionOption) -> Result<ComPtr<super::super::foundation::IAsyncOperation<ProtectedContainerImportResult>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).LoadFileFromContainerWithTargetAndNameCollisionOptionAsync)(self as *const _ as *mut _, containerFile as *const _ as *mut _, target as *const _ as *mut _, collisionOption, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[cfg(feature="windows-storage")] #[inline] pub unsafe fn save_file_as_container_with_sharing_async(&self, protectedFile: &super::super::storage::IStorageFile, sharedWithIdentities: &super::super::foundation::collections::IIterable<HString>) -> Result<ComPtr<super::super::foundation::IAsyncOperation<ProtectedContainerExportResult>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).SaveFileAsContainerWithSharingAsync)(self as *const _ as *mut _, protectedFile as *const _ as *mut _, sharedWithIdentities as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
DEFINE_IID!(IID_IFileProtectionManagerStatics3, 1763214490, 25167, 18134, 178, 65, 233, 205, 95, 223, 62, 63);
RT_INTERFACE!{static interface IFileProtectionManagerStatics3(IFileProtectionManagerStatics3Vtbl): IInspectable(IInspectableVtbl) [IID_IFileProtectionManagerStatics3] {
    #[cfg(feature="windows-storage")] fn UnprotectAsync(&self, target: *mut super::super::storage::IStorageItem, out: *mut *mut super::super::foundation::IAsyncOperation<FileProtectionInfo>) -> HRESULT,
    #[cfg(feature="windows-storage")] fn UnprotectWithOptionsAsync(&self, target: *mut super::super::storage::IStorageItem, options: *mut FileUnprotectOptions, out: *mut *mut super::super::foundation::IAsyncOperation<FileProtectionInfo>) -> HRESULT
}}
impl IFileProtectionManagerStatics3 {
    #[cfg(feature="windows-storage")] #[inline] pub unsafe fn unprotect_async(&self, target: &super::super::storage::IStorageItem) -> Result<ComPtr<super::super::foundation::IAsyncOperation<FileProtectionInfo>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).UnprotectAsync)(self as *const _ as *mut _, target as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[cfg(feature="windows-storage")] #[inline] pub unsafe fn unprotect_with_options_async(&self, target: &super::super::storage::IStorageItem, options: &FileUnprotectOptions) -> Result<ComPtr<super::super::foundation::IAsyncOperation<FileProtectionInfo>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).UnprotectWithOptionsAsync)(self as *const _ as *mut _, target as *const _ as *mut _, options as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
RT_ENUM! { enum FileProtectionStatus: i32 {
    Undetermined (FileProtectionStatus_Undetermined) = 0, Unknown (FileProtectionStatus_Unknown) = 0, Unprotected (FileProtectionStatus_Unprotected) = 1, Revoked (FileProtectionStatus_Revoked) = 2, Protected (FileProtectionStatus_Protected) = 3, ProtectedByOtherUser (FileProtectionStatus_ProtectedByOtherUser) = 4, ProtectedToOtherEnterprise (FileProtectionStatus_ProtectedToOtherEnterprise) = 5, NotProtectable (FileProtectionStatus_NotProtectable) = 6, ProtectedToOtherIdentity (FileProtectionStatus_ProtectedToOtherIdentity) = 7, LicenseExpired (FileProtectionStatus_LicenseExpired) = 8, AccessSuspended (FileProtectionStatus_AccessSuspended) = 9, FileInUse (FileProtectionStatus_FileInUse) = 10,
}}
RT_CLASS!{static class FileRevocationManager}
impl RtActivatable<IFileRevocationManagerStatics> for FileRevocationManager {}
impl FileRevocationManager {
    #[cfg(feature="windows-storage")] #[inline] pub fn protect_async(storageItem: &super::super::storage::IStorageItem, enterpriseIdentity: &HStringArg) -> Result<ComPtr<super::super::foundation::IAsyncOperation<FileProtectionStatus>>> { unsafe {
        <Self as RtActivatable<IFileRevocationManagerStatics>>::get_activation_factory().protect_async(storageItem, enterpriseIdentity)
    }}
    #[cfg(feature="windows-storage")] #[inline] pub fn copy_protection_async(sourceStorageItem: &super::super::storage::IStorageItem, targetStorageItem: &super::super::storage::IStorageItem) -> Result<ComPtr<super::super::foundation::IAsyncOperation<bool>>> { unsafe {
        <Self as RtActivatable<IFileRevocationManagerStatics>>::get_activation_factory().copy_protection_async(sourceStorageItem, targetStorageItem)
    }}
    #[inline] pub fn revoke(enterpriseIdentity: &HStringArg) -> Result<()> { unsafe {
        <Self as RtActivatable<IFileRevocationManagerStatics>>::get_activation_factory().revoke(enterpriseIdentity)
    }}
    #[cfg(feature="windows-storage")] #[inline] pub fn get_status_async(storageItem: &super::super::storage::IStorageItem) -> Result<ComPtr<super::super::foundation::IAsyncOperation<FileProtectionStatus>>> { unsafe {
        <Self as RtActivatable<IFileRevocationManagerStatics>>::get_activation_factory().get_status_async(storageItem)
    }}
}
DEFINE_CLSID!(FileRevocationManager(&[87,105,110,100,111,119,115,46,83,101,99,117,114,105,116,121,46,69,110,116,101,114,112,114,105,115,101,68,97,116,97,46,70,105,108,101,82,101,118,111,99,97,116,105,111,110,77,97,110,97,103,101,114,0]) [CLSID_FileRevocationManager]);
DEFINE_IID!(IID_IFileRevocationManagerStatics, 627817533, 7261, 16992, 140, 117, 145, 68, 207, 183, 139, 169);
RT_INTERFACE!{static interface IFileRevocationManagerStatics(IFileRevocationManagerStaticsVtbl): IInspectable(IInspectableVtbl) [IID_IFileRevocationManagerStatics] {
    #[cfg(feature="windows-storage")] fn ProtectAsync(&self, storageItem: *mut super::super::storage::IStorageItem, enterpriseIdentity: HSTRING, out: *mut *mut super::super::foundation::IAsyncOperation<FileProtectionStatus>) -> HRESULT,
    #[cfg(feature="windows-storage")] fn CopyProtectionAsync(&self, sourceStorageItem: *mut super::super::storage::IStorageItem, targetStorageItem: *mut super::super::storage::IStorageItem, out: *mut *mut super::super::foundation::IAsyncOperation<bool>) -> HRESULT,
    fn Revoke(&self, enterpriseIdentity: HSTRING) -> HRESULT,
    #[cfg(feature="windows-storage")] fn GetStatusAsync(&self, storageItem: *mut super::super::storage::IStorageItem, out: *mut *mut super::super::foundation::IAsyncOperation<FileProtectionStatus>) -> HRESULT
}}
impl IFileRevocationManagerStatics {
    #[cfg(feature="windows-storage")] #[inline] pub unsafe fn protect_async(&self, storageItem: &super::super::storage::IStorageItem, enterpriseIdentity: &HStringArg) -> Result<ComPtr<super::super::foundation::IAsyncOperation<FileProtectionStatus>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).ProtectAsync)(self as *const _ as *mut _, storageItem as *const _ as *mut _, enterpriseIdentity.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[cfg(feature="windows-storage")] #[inline] pub unsafe fn copy_protection_async(&self, sourceStorageItem: &super::super::storage::IStorageItem, targetStorageItem: &super::super::storage::IStorageItem) -> Result<ComPtr<super::super::foundation::IAsyncOperation<bool>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).CopyProtectionAsync)(self as *const _ as *mut _, sourceStorageItem as *const _ as *mut _, targetStorageItem as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn revoke(&self, enterpriseIdentity: &HStringArg) -> Result<()> {
        let hr = ((*self.lpVtbl).Revoke)(self as *const _ as *mut _, enterpriseIdentity.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[cfg(feature="windows-storage")] #[inline] pub unsafe fn get_status_async(&self, storageItem: &super::super::storage::IStorageItem) -> Result<ComPtr<super::super::foundation::IAsyncOperation<FileProtectionStatus>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetStatusAsync)(self as *const _ as *mut _, storageItem as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
DEFINE_IID!(IID_IFileUnprotectOptions, 2098402033, 15117, 19928, 161, 248, 30, 197, 56, 34, 226, 243);
RT_INTERFACE!{interface IFileUnprotectOptions(IFileUnprotectOptionsVtbl): IInspectable(IInspectableVtbl) [IID_IFileUnprotectOptions] {
    fn put_Audit(&self, value: bool) -> HRESULT,
    fn get_Audit(&self, out: *mut bool) -> HRESULT
}}
impl IFileUnprotectOptions {
    #[inline] pub unsafe fn set_audit(&self, value: bool) -> Result<()> {
        let hr = ((*self.lpVtbl).put_Audit)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_audit(&self) -> Result<bool> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_Audit)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
}
RT_CLASS!{class FileUnprotectOptions: IFileUnprotectOptions}
impl RtActivatable<IFileUnprotectOptionsFactory> for FileUnprotectOptions {}
impl FileUnprotectOptions {
    #[inline] pub fn create(audit: bool) -> Result<ComPtr<FileUnprotectOptions>> { unsafe {
        <Self as RtActivatable<IFileUnprotectOptionsFactory>>::get_activation_factory().create(audit)
    }}
}
DEFINE_CLSID!(FileUnprotectOptions(&[87,105,110,100,111,119,115,46,83,101,99,117,114,105,116,121,46,69,110,116,101,114,112,114,105,115,101,68,97,116,97,46,70,105,108,101,85,110,112,114,111,116,101,99,116,79,112,116,105,111,110,115,0]) [CLSID_FileUnprotectOptions]);
DEFINE_IID!(IID_IFileUnprotectOptionsFactory, 1370403740, 55948, 19519, 155, 251, 203, 115, 167, 204, 224, 221);
RT_INTERFACE!{static interface IFileUnprotectOptionsFactory(IFileUnprotectOptionsFactoryVtbl): IInspectable(IInspectableVtbl) [IID_IFileUnprotectOptionsFactory] {
    fn Create(&self, audit: bool, out: *mut *mut FileUnprotectOptions) -> HRESULT
}}
impl IFileUnprotectOptionsFactory {
    #[inline] pub unsafe fn create(&self, audit: bool) -> Result<ComPtr<FileUnprotectOptions>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).Create)(self as *const _ as *mut _, audit, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
DEFINE_IID!(IID_IProtectedAccessResumedEventArgs, 2890779225, 23936, 20117, 140, 95, 133, 57, 69, 14, 235, 224);
RT_INTERFACE!{interface IProtectedAccessResumedEventArgs(IProtectedAccessResumedEventArgsVtbl): IInspectable(IInspectableVtbl) [IID_IProtectedAccessResumedEventArgs] {
    fn get_Identities(&self, out: *mut *mut super::super::foundation::collections::IVectorView<HString>) -> HRESULT
}}
impl IProtectedAccessResumedEventArgs {
    #[inline] pub unsafe fn get_identities(&self) -> Result<ComPtr<super::super::foundation::collections::IVectorView<HString>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Identities)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
RT_CLASS!{class ProtectedAccessResumedEventArgs: IProtectedAccessResumedEventArgs}
DEFINE_IID!(IID_IProtectedAccessSuspendingEventArgs, 1973523424, 41796, 17055, 185, 117, 4, 252, 31, 136, 193, 133);
RT_INTERFACE!{interface IProtectedAccessSuspendingEventArgs(IProtectedAccessSuspendingEventArgsVtbl): IInspectable(IInspectableVtbl) [IID_IProtectedAccessSuspendingEventArgs] {
    fn get_Identities(&self, out: *mut *mut super::super::foundation::collections::IVectorView<HString>) -> HRESULT,
    fn get_Deadline(&self, out: *mut super::super::foundation::DateTime) -> HRESULT,
    fn GetDeferral(&self, out: *mut *mut super::super::foundation::Deferral) -> HRESULT
}}
impl IProtectedAccessSuspendingEventArgs {
    #[inline] pub unsafe fn get_identities(&self) -> Result<ComPtr<super::super::foundation::collections::IVectorView<HString>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Identities)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_deadline(&self) -> Result<super::super::foundation::DateTime> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_Deadline)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_deferral(&self) -> Result<ComPtr<super::super::foundation::Deferral>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetDeferral)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
RT_CLASS!{class ProtectedAccessSuspendingEventArgs: IProtectedAccessSuspendingEventArgs}
DEFINE_IID!(IID_IProtectedContainerExportResult, 961081237, 63483, 19266, 175, 176, 223, 112, 180, 21, 67, 193);
RT_INTERFACE!{interface IProtectedContainerExportResult(IProtectedContainerExportResultVtbl): IInspectable(IInspectableVtbl) [IID_IProtectedContainerExportResult] {
    fn get_Status(&self, out: *mut ProtectedImportExportStatus) -> HRESULT,
    #[cfg(feature="windows-storage")] fn get_File(&self, out: *mut *mut super::super::storage::StorageFile) -> HRESULT
}}
impl IProtectedContainerExportResult {
    #[inline] pub unsafe fn get_status(&self) -> Result<ProtectedImportExportStatus> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_Status)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[cfg(feature="windows-storage")] #[inline] pub unsafe fn get_file(&self) -> Result<ComPtr<super::super::storage::StorageFile>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_File)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
RT_CLASS!{class ProtectedContainerExportResult: IProtectedContainerExportResult}
DEFINE_IID!(IID_IProtectedContainerImportResult, 3451355345, 59323, 19738, 147, 57, 52, 220, 65, 20, 159, 155);
RT_INTERFACE!{interface IProtectedContainerImportResult(IProtectedContainerImportResultVtbl): IInspectable(IInspectableVtbl) [IID_IProtectedContainerImportResult] {
    fn get_Status(&self, out: *mut ProtectedImportExportStatus) -> HRESULT,
    #[cfg(feature="windows-storage")] fn get_File(&self, out: *mut *mut super::super::storage::StorageFile) -> HRESULT
}}
impl IProtectedContainerImportResult {
    #[inline] pub unsafe fn get_status(&self) -> Result<ProtectedImportExportStatus> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_Status)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[cfg(feature="windows-storage")] #[inline] pub unsafe fn get_file(&self) -> Result<ComPtr<super::super::storage::StorageFile>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_File)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
RT_CLASS!{class ProtectedContainerImportResult: IProtectedContainerImportResult}
DEFINE_IID!(IID_IProtectedContentRevokedEventArgs, 1667786785, 22713, 18414, 147, 217, 240, 247, 65, 207, 67, 240);
RT_INTERFACE!{interface IProtectedContentRevokedEventArgs(IProtectedContentRevokedEventArgsVtbl): IInspectable(IInspectableVtbl) [IID_IProtectedContentRevokedEventArgs] {
    fn get_Identities(&self, out: *mut *mut super::super::foundation::collections::IVectorView<HString>) -> HRESULT
}}
impl IProtectedContentRevokedEventArgs {
    #[inline] pub unsafe fn get_identities(&self) -> Result<ComPtr<super::super::foundation::collections::IVectorView<HString>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Identities)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
RT_CLASS!{class ProtectedContentRevokedEventArgs: IProtectedContentRevokedEventArgs}
DEFINE_IID!(IID_IProtectedFileCreateResult, 686026090, 59879, 18947, 159, 83, 189, 177, 97, 114, 105, 155);
RT_INTERFACE!{interface IProtectedFileCreateResult(IProtectedFileCreateResultVtbl): IInspectable(IInspectableVtbl) [IID_IProtectedFileCreateResult] {
    #[cfg(not(feature="windows-storage"))] fn __Dummy0(&self) -> (),
    #[cfg(feature="windows-storage")] fn get_File(&self, out: *mut *mut super::super::storage::StorageFile) -> HRESULT,
    #[cfg(not(feature="windows-storage"))] fn __Dummy1(&self) -> (),
    #[cfg(feature="windows-storage")] fn get_Stream(&self, out: *mut *mut super::super::storage::streams::IRandomAccessStream) -> HRESULT,
    fn get_ProtectionInfo(&self, out: *mut *mut FileProtectionInfo) -> HRESULT
}}
impl IProtectedFileCreateResult {
    #[cfg(feature="windows-storage")] #[inline] pub unsafe fn get_file(&self) -> Result<ComPtr<super::super::storage::StorageFile>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_File)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[cfg(feature="windows-storage")] #[inline] pub unsafe fn get_stream(&self) -> Result<ComPtr<super::super::storage::streams::IRandomAccessStream>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Stream)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_protection_info(&self) -> Result<ComPtr<FileProtectionInfo>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_ProtectionInfo)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
RT_CLASS!{class ProtectedFileCreateResult: IProtectedFileCreateResult}
RT_ENUM! { enum ProtectedImportExportStatus: i32 {
    Ok (ProtectedImportExportStatus_Ok) = 0, Undetermined (ProtectedImportExportStatus_Undetermined) = 1, Unprotected (ProtectedImportExportStatus_Unprotected) = 2, Revoked (ProtectedImportExportStatus_Revoked) = 3, NotRoamable (ProtectedImportExportStatus_NotRoamable) = 4, ProtectedToOtherIdentity (ProtectedImportExportStatus_ProtectedToOtherIdentity) = 5, LicenseExpired (ProtectedImportExportStatus_LicenseExpired) = 6, AccessSuspended (ProtectedImportExportStatus_AccessSuspended) = 7,
}}
RT_ENUM! { enum ProtectionPolicyAuditAction: i32 {
    Decrypt (ProtectionPolicyAuditAction_Decrypt) = 0, CopyToLocation (ProtectionPolicyAuditAction_CopyToLocation) = 1, SendToRecipient (ProtectionPolicyAuditAction_SendToRecipient) = 2, Other (ProtectionPolicyAuditAction_Other) = 3,
}}
DEFINE_IID!(IID_IProtectionPolicyAuditInfo, 1113241572, 65207, 17660, 179, 187, 195, 196, 215, 236, 190, 187);
RT_INTERFACE!{interface IProtectionPolicyAuditInfo(IProtectionPolicyAuditInfoVtbl): IInspectable(IInspectableVtbl) [IID_IProtectionPolicyAuditInfo] {
    fn put_Action(&self, value: ProtectionPolicyAuditAction) -> HRESULT,
    fn get_Action(&self, out: *mut ProtectionPolicyAuditAction) -> HRESULT,
    fn put_DataDescription(&self, value: HSTRING) -> HRESULT,
    fn get_DataDescription(&self, out: *mut HSTRING) -> HRESULT,
    fn put_SourceDescription(&self, value: HSTRING) -> HRESULT,
    fn get_SourceDescription(&self, out: *mut HSTRING) -> HRESULT,
    fn put_TargetDescription(&self, value: HSTRING) -> HRESULT,
    fn get_TargetDescription(&self, out: *mut HSTRING) -> HRESULT
}}
impl IProtectionPolicyAuditInfo {
    #[inline] pub unsafe fn set_action(&self, value: ProtectionPolicyAuditAction) -> Result<()> {
        let hr = ((*self.lpVtbl).put_Action)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_action(&self) -> Result<ProtectionPolicyAuditAction> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_Action)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_data_description(&self, value: &HStringArg) -> Result<()> {
        let hr = ((*self.lpVtbl).put_DataDescription)(self as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_data_description(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_DataDescription)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_source_description(&self, value: &HStringArg) -> Result<()> {
        let hr = ((*self.lpVtbl).put_SourceDescription)(self as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_source_description(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_SourceDescription)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_target_description(&self, value: &HStringArg) -> Result<()> {
        let hr = ((*self.lpVtbl).put_TargetDescription)(self as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_target_description(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_TargetDescription)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
}
RT_CLASS!{class ProtectionPolicyAuditInfo: IProtectionPolicyAuditInfo}
impl RtActivatable<IProtectionPolicyAuditInfoFactory> for ProtectionPolicyAuditInfo {}
impl ProtectionPolicyAuditInfo {
    #[inline] pub fn create(action: ProtectionPolicyAuditAction, dataDescription: &HStringArg, sourceDescription: &HStringArg, targetDescription: &HStringArg) -> Result<ComPtr<ProtectionPolicyAuditInfo>> { unsafe {
        <Self as RtActivatable<IProtectionPolicyAuditInfoFactory>>::get_activation_factory().create(action, dataDescription, sourceDescription, targetDescription)
    }}
    #[inline] pub fn create_with_action_and_data_description(action: ProtectionPolicyAuditAction, dataDescription: &HStringArg) -> Result<ComPtr<ProtectionPolicyAuditInfo>> { unsafe {
        <Self as RtActivatable<IProtectionPolicyAuditInfoFactory>>::get_activation_factory().create_with_action_and_data_description(action, dataDescription)
    }}
}
DEFINE_CLSID!(ProtectionPolicyAuditInfo(&[87,105,110,100,111,119,115,46,83,101,99,117,114,105,116,121,46,69,110,116,101,114,112,114,105,115,101,68,97,116,97,46,80,114,111,116,101,99,116,105,111,110,80,111,108,105,99,121,65,117,100,105,116,73,110,102,111,0]) [CLSID_ProtectionPolicyAuditInfo]);
DEFINE_IID!(IID_IProtectionPolicyAuditInfoFactory, 2127829003, 37608, 17109, 131, 212, 37, 68, 11, 66, 53, 73);
RT_INTERFACE!{static interface IProtectionPolicyAuditInfoFactory(IProtectionPolicyAuditInfoFactoryVtbl): IInspectable(IInspectableVtbl) [IID_IProtectionPolicyAuditInfoFactory] {
    fn Create(&self, action: ProtectionPolicyAuditAction, dataDescription: HSTRING, sourceDescription: HSTRING, targetDescription: HSTRING, out: *mut *mut ProtectionPolicyAuditInfo) -> HRESULT,
    fn CreateWithActionAndDataDescription(&self, action: ProtectionPolicyAuditAction, dataDescription: HSTRING, out: *mut *mut ProtectionPolicyAuditInfo) -> HRESULT
}}
impl IProtectionPolicyAuditInfoFactory {
    #[inline] pub unsafe fn create(&self, action: ProtectionPolicyAuditAction, dataDescription: &HStringArg, sourceDescription: &HStringArg, targetDescription: &HStringArg) -> Result<ComPtr<ProtectionPolicyAuditInfo>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).Create)(self as *const _ as *mut _, action, dataDescription.get(), sourceDescription.get(), targetDescription.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn create_with_action_and_data_description(&self, action: ProtectionPolicyAuditAction, dataDescription: &HStringArg) -> Result<ComPtr<ProtectionPolicyAuditInfo>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).CreateWithActionAndDataDescription)(self as *const _ as *mut _, action, dataDescription.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
RT_ENUM! { enum ProtectionPolicyEvaluationResult: i32 {
    Allowed (ProtectionPolicyEvaluationResult_Allowed) = 0, Blocked (ProtectionPolicyEvaluationResult_Blocked) = 1, ConsentRequired (ProtectionPolicyEvaluationResult_ConsentRequired) = 2,
}}
DEFINE_IID!(IID_IProtectionPolicyManager, 3580902936, 41101, 18406, 162, 64, 153, 52, 215, 22, 94, 181);
RT_INTERFACE!{interface IProtectionPolicyManager(IProtectionPolicyManagerVtbl): IInspectable(IInspectableVtbl) [IID_IProtectionPolicyManager] {
    fn put_Identity(&self, value: HSTRING) -> HRESULT,
    fn get_Identity(&self, out: *mut HSTRING) -> HRESULT
}}
impl IProtectionPolicyManager {
    #[inline] pub unsafe fn set_identity(&self, value: &HStringArg) -> Result<()> {
        let hr = ((*self.lpVtbl).put_Identity)(self as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_identity(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Identity)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
}
RT_CLASS!{class ProtectionPolicyManager: IProtectionPolicyManager}
impl RtActivatable<IProtectionPolicyManagerStatics> for ProtectionPolicyManager {}
impl RtActivatable<IProtectionPolicyManagerStatics2> for ProtectionPolicyManager {}
impl RtActivatable<IProtectionPolicyManagerStatics3> for ProtectionPolicyManager {}
impl RtActivatable<IProtectionPolicyManagerStatics4> for ProtectionPolicyManager {}
impl ProtectionPolicyManager {
    #[inline] pub fn is_identity_managed(identity: &HStringArg) -> Result<bool> { unsafe {
        <Self as RtActivatable<IProtectionPolicyManagerStatics>>::get_activation_factory().is_identity_managed(identity)
    }}
    #[inline] pub fn try_apply_process_uipolicy(identity: &HStringArg) -> Result<bool> { unsafe {
        <Self as RtActivatable<IProtectionPolicyManagerStatics>>::get_activation_factory().try_apply_process_uipolicy(identity)
    }}
    #[inline] pub fn clear_process_uipolicy() -> Result<()> { unsafe {
        <Self as RtActivatable<IProtectionPolicyManagerStatics>>::get_activation_factory().clear_process_uipolicy()
    }}
    #[inline] pub fn create_current_thread_network_context(identity: &HStringArg) -> Result<ComPtr<ThreadNetworkContext>> { unsafe {
        <Self as RtActivatable<IProtectionPolicyManagerStatics>>::get_activation_factory().create_current_thread_network_context(identity)
    }}
    #[cfg(feature="windows-networking")] #[inline] pub fn get_primary_managed_identity_for_network_endpoint_async(endpointHost: &super::super::networking::HostName) -> Result<ComPtr<super::super::foundation::IAsyncOperation<HString>>> { unsafe {
        <Self as RtActivatable<IProtectionPolicyManagerStatics>>::get_activation_factory().get_primary_managed_identity_for_network_endpoint_async(endpointHost)
    }}
    #[inline] pub fn revoke_content(identity: &HStringArg) -> Result<()> { unsafe {
        <Self as RtActivatable<IProtectionPolicyManagerStatics>>::get_activation_factory().revoke_content(identity)
    }}
    #[inline] pub fn get_for_current_view() -> Result<ComPtr<ProtectionPolicyManager>> { unsafe {
        <Self as RtActivatable<IProtectionPolicyManagerStatics>>::get_activation_factory().get_for_current_view()
    }}
    #[inline] pub fn add_protected_access_suspending(handler: &super::super::foundation::EventHandler<ProtectedAccessSuspendingEventArgs>) -> Result<super::super::foundation::EventRegistrationToken> { unsafe {
        <Self as RtActivatable<IProtectionPolicyManagerStatics>>::get_activation_factory().add_protected_access_suspending(handler)
    }}
    #[inline] pub fn remove_protected_access_suspending(token: super::super::foundation::EventRegistrationToken) -> Result<()> { unsafe {
        <Self as RtActivatable<IProtectionPolicyManagerStatics>>::get_activation_factory().remove_protected_access_suspending(token)
    }}
    #[inline] pub fn add_protected_access_resumed(handler: &super::super::foundation::EventHandler<ProtectedAccessResumedEventArgs>) -> Result<super::super::foundation::EventRegistrationToken> { unsafe {
        <Self as RtActivatable<IProtectionPolicyManagerStatics>>::get_activation_factory().add_protected_access_resumed(handler)
    }}
    #[inline] pub fn remove_protected_access_resumed(token: super::super::foundation::EventRegistrationToken) -> Result<()> { unsafe {
        <Self as RtActivatable<IProtectionPolicyManagerStatics>>::get_activation_factory().remove_protected_access_resumed(token)
    }}
    #[inline] pub fn add_protected_content_revoked(handler: &super::super::foundation::EventHandler<ProtectedContentRevokedEventArgs>) -> Result<super::super::foundation::EventRegistrationToken> { unsafe {
        <Self as RtActivatable<IProtectionPolicyManagerStatics>>::get_activation_factory().add_protected_content_revoked(handler)
    }}
    #[inline] pub fn remove_protected_content_revoked(token: super::super::foundation::EventRegistrationToken) -> Result<()> { unsafe {
        <Self as RtActivatable<IProtectionPolicyManagerStatics>>::get_activation_factory().remove_protected_content_revoked(token)
    }}
    #[inline] pub fn check_access(sourceIdentity: &HStringArg, targetIdentity: &HStringArg) -> Result<ProtectionPolicyEvaluationResult> { unsafe {
        <Self as RtActivatable<IProtectionPolicyManagerStatics>>::get_activation_factory().check_access(sourceIdentity, targetIdentity)
    }}
    #[inline] pub fn request_access_async(sourceIdentity: &HStringArg, targetIdentity: &HStringArg) -> Result<ComPtr<super::super::foundation::IAsyncOperation<ProtectionPolicyEvaluationResult>>> { unsafe {
        <Self as RtActivatable<IProtectionPolicyManagerStatics>>::get_activation_factory().request_access_async(sourceIdentity, targetIdentity)
    }}
    #[inline] pub fn has_content_been_revoked_since(identity: &HStringArg, since: super::super::foundation::DateTime) -> Result<bool> { unsafe {
        <Self as RtActivatable<IProtectionPolicyManagerStatics2>>::get_activation_factory().has_content_been_revoked_since(identity, since)
    }}
    #[inline] pub fn check_access_for_app(sourceIdentity: &HStringArg, appPackageFamilyName: &HStringArg) -> Result<ProtectionPolicyEvaluationResult> { unsafe {
        <Self as RtActivatable<IProtectionPolicyManagerStatics2>>::get_activation_factory().check_access_for_app(sourceIdentity, appPackageFamilyName)
    }}
    #[inline] pub fn request_access_for_app_async(sourceIdentity: &HStringArg, appPackageFamilyName: &HStringArg) -> Result<ComPtr<super::super::foundation::IAsyncOperation<ProtectionPolicyEvaluationResult>>> { unsafe {
        <Self as RtActivatable<IProtectionPolicyManagerStatics2>>::get_activation_factory().request_access_for_app_async(sourceIdentity, appPackageFamilyName)
    }}
    #[inline] pub fn get_enforcement_level(identity: &HStringArg) -> Result<EnforcementLevel> { unsafe {
        <Self as RtActivatable<IProtectionPolicyManagerStatics2>>::get_activation_factory().get_enforcement_level(identity)
    }}
    #[inline] pub fn is_user_decryption_allowed(identity: &HStringArg) -> Result<bool> { unsafe {
        <Self as RtActivatable<IProtectionPolicyManagerStatics2>>::get_activation_factory().is_user_decryption_allowed(identity)
    }}
    #[inline] pub fn is_protection_under_lock_required(identity: &HStringArg) -> Result<bool> { unsafe {
        <Self as RtActivatable<IProtectionPolicyManagerStatics2>>::get_activation_factory().is_protection_under_lock_required(identity)
    }}
    #[inline] pub fn add_policy_changed(handler: &super::super::foundation::EventHandler<IInspectable>) -> Result<super::super::foundation::EventRegistrationToken> { unsafe {
        <Self as RtActivatable<IProtectionPolicyManagerStatics2>>::get_activation_factory().add_policy_changed(handler)
    }}
    #[inline] pub fn remove_policy_changed(token: super::super::foundation::EventRegistrationToken) -> Result<()> { unsafe {
        <Self as RtActivatable<IProtectionPolicyManagerStatics2>>::get_activation_factory().remove_policy_changed(token)
    }}
    #[inline] pub fn get_is_protection_enabled() -> Result<bool> { unsafe {
        <Self as RtActivatable<IProtectionPolicyManagerStatics2>>::get_activation_factory().get_is_protection_enabled()
    }}
    #[inline] pub fn request_access_with_auditing_info_async(sourceIdentity: &HStringArg, targetIdentity: &HStringArg, auditInfo: &ProtectionPolicyAuditInfo) -> Result<ComPtr<super::super::foundation::IAsyncOperation<ProtectionPolicyEvaluationResult>>> { unsafe {
        <Self as RtActivatable<IProtectionPolicyManagerStatics3>>::get_activation_factory().request_access_with_auditing_info_async(sourceIdentity, targetIdentity, auditInfo)
    }}
    #[inline] pub fn request_access_with_message_async(sourceIdentity: &HStringArg, targetIdentity: &HStringArg, auditInfo: &ProtectionPolicyAuditInfo, messageFromApp: &HStringArg) -> Result<ComPtr<super::super::foundation::IAsyncOperation<ProtectionPolicyEvaluationResult>>> { unsafe {
        <Self as RtActivatable<IProtectionPolicyManagerStatics3>>::get_activation_factory().request_access_with_message_async(sourceIdentity, targetIdentity, auditInfo, messageFromApp)
    }}
    #[inline] pub fn request_access_for_app_with_auditing_info_async(sourceIdentity: &HStringArg, appPackageFamilyName: &HStringArg, auditInfo: &ProtectionPolicyAuditInfo) -> Result<ComPtr<super::super::foundation::IAsyncOperation<ProtectionPolicyEvaluationResult>>> { unsafe {
        <Self as RtActivatable<IProtectionPolicyManagerStatics3>>::get_activation_factory().request_access_for_app_with_auditing_info_async(sourceIdentity, appPackageFamilyName, auditInfo)
    }}
    #[inline] pub fn request_access_for_app_with_message_async(sourceIdentity: &HStringArg, appPackageFamilyName: &HStringArg, auditInfo: &ProtectionPolicyAuditInfo, messageFromApp: &HStringArg) -> Result<ComPtr<super::super::foundation::IAsyncOperation<ProtectionPolicyEvaluationResult>>> { unsafe {
        <Self as RtActivatable<IProtectionPolicyManagerStatics3>>::get_activation_factory().request_access_for_app_with_message_async(sourceIdentity, appPackageFamilyName, auditInfo, messageFromApp)
    }}
    #[inline] pub fn log_audit_event(sourceIdentity: &HStringArg, targetIdentity: &HStringArg, auditInfo: &ProtectionPolicyAuditInfo) -> Result<()> { unsafe {
        <Self as RtActivatable<IProtectionPolicyManagerStatics3>>::get_activation_factory().log_audit_event(sourceIdentity, targetIdentity, auditInfo)
    }}
    #[inline] pub fn is_roamable_protection_enabled(identity: &HStringArg) -> Result<bool> { unsafe {
        <Self as RtActivatable<IProtectionPolicyManagerStatics4>>::get_activation_factory().is_roamable_protection_enabled(identity)
    }}
    #[inline] pub fn request_access_with_behavior_async(sourceIdentity: &HStringArg, targetIdentity: &HStringArg, auditInfo: &ProtectionPolicyAuditInfo, messageFromApp: &HStringArg, behavior: ProtectionPolicyRequestAccessBehavior) -> Result<ComPtr<super::super::foundation::IAsyncOperation<ProtectionPolicyEvaluationResult>>> { unsafe {
        <Self as RtActivatable<IProtectionPolicyManagerStatics4>>::get_activation_factory().request_access_with_behavior_async(sourceIdentity, targetIdentity, auditInfo, messageFromApp, behavior)
    }}
    #[inline] pub fn request_access_for_app_with_behavior_async(sourceIdentity: &HStringArg, appPackageFamilyName: &HStringArg, auditInfo: &ProtectionPolicyAuditInfo, messageFromApp: &HStringArg, behavior: ProtectionPolicyRequestAccessBehavior) -> Result<ComPtr<super::super::foundation::IAsyncOperation<ProtectionPolicyEvaluationResult>>> { unsafe {
        <Self as RtActivatable<IProtectionPolicyManagerStatics4>>::get_activation_factory().request_access_for_app_with_behavior_async(sourceIdentity, appPackageFamilyName, auditInfo, messageFromApp, behavior)
    }}
    #[cfg(feature="windows-storage")] #[inline] pub fn request_access_to_files_for_app_async(sourceItemList: &super::super::foundation::collections::IIterable<super::super::storage::IStorageItem>, appPackageFamilyName: &HStringArg, auditInfo: &ProtectionPolicyAuditInfo) -> Result<ComPtr<super::super::foundation::IAsyncOperation<ProtectionPolicyEvaluationResult>>> { unsafe {
        <Self as RtActivatable<IProtectionPolicyManagerStatics4>>::get_activation_factory().request_access_to_files_for_app_async(sourceItemList, appPackageFamilyName, auditInfo)
    }}
    #[cfg(feature="windows-storage")] #[inline] pub fn request_access_to_files_for_app_with_message_and_behavior_async(sourceItemList: &super::super::foundation::collections::IIterable<super::super::storage::IStorageItem>, appPackageFamilyName: &HStringArg, auditInfo: &ProtectionPolicyAuditInfo, messageFromApp: &HStringArg, behavior: ProtectionPolicyRequestAccessBehavior) -> Result<ComPtr<super::super::foundation::IAsyncOperation<ProtectionPolicyEvaluationResult>>> { unsafe {
        <Self as RtActivatable<IProtectionPolicyManagerStatics4>>::get_activation_factory().request_access_to_files_for_app_with_message_and_behavior_async(sourceItemList, appPackageFamilyName, auditInfo, messageFromApp, behavior)
    }}
    #[cfg(feature="windows-storage")] #[inline] pub fn request_access_to_files_for_process_async(sourceItemList: &super::super::foundation::collections::IIterable<super::super::storage::IStorageItem>, processId: u32, auditInfo: &ProtectionPolicyAuditInfo) -> Result<ComPtr<super::super::foundation::IAsyncOperation<ProtectionPolicyEvaluationResult>>> { unsafe {
        <Self as RtActivatable<IProtectionPolicyManagerStatics4>>::get_activation_factory().request_access_to_files_for_process_async(sourceItemList, processId, auditInfo)
    }}
    #[cfg(feature="windows-storage")] #[inline] pub fn request_access_to_files_for_process_with_message_and_behavior_async(sourceItemList: &super::super::foundation::collections::IIterable<super::super::storage::IStorageItem>, processId: u32, auditInfo: &ProtectionPolicyAuditInfo, messageFromApp: &HStringArg, behavior: ProtectionPolicyRequestAccessBehavior) -> Result<ComPtr<super::super::foundation::IAsyncOperation<ProtectionPolicyEvaluationResult>>> { unsafe {
        <Self as RtActivatable<IProtectionPolicyManagerStatics4>>::get_activation_factory().request_access_to_files_for_process_with_message_and_behavior_async(sourceItemList, processId, auditInfo, messageFromApp, behavior)
    }}
    #[cfg(feature="windows-storage")] #[inline] pub fn is_file_protection_required_async(target: &super::super::storage::IStorageItem, identity: &HStringArg) -> Result<ComPtr<super::super::foundation::IAsyncOperation<bool>>> { unsafe {
        <Self as RtActivatable<IProtectionPolicyManagerStatics4>>::get_activation_factory().is_file_protection_required_async(target, identity)
    }}
    #[cfg(feature="windows-storage")] #[inline] pub fn is_file_protection_required_for_new_file_async(parentFolder: &super::super::storage::IStorageFolder, identity: &HStringArg, desiredName: &HStringArg) -> Result<ComPtr<super::super::foundation::IAsyncOperation<bool>>> { unsafe {
        <Self as RtActivatable<IProtectionPolicyManagerStatics4>>::get_activation_factory().is_file_protection_required_for_new_file_async(parentFolder, identity, desiredName)
    }}
    #[inline] pub fn get_primary_managed_identity() -> Result<HString> { unsafe {
        <Self as RtActivatable<IProtectionPolicyManagerStatics4>>::get_activation_factory().get_primary_managed_identity()
    }}
    #[inline] pub fn get_primary_managed_identity_for_identity(identity: &HStringArg) -> Result<HString> { unsafe {
        <Self as RtActivatable<IProtectionPolicyManagerStatics4>>::get_activation_factory().get_primary_managed_identity_for_identity(identity)
    }}
}
DEFINE_CLSID!(ProtectionPolicyManager(&[87,105,110,100,111,119,115,46,83,101,99,117,114,105,116,121,46,69,110,116,101,114,112,114,105,115,101,68,97,116,97,46,80,114,111,116,101,99,116,105,111,110,80,111,108,105,99,121,77,97,110,97,103,101,114,0]) [CLSID_ProtectionPolicyManager]);
DEFINE_IID!(IID_IProtectionPolicyManager2, 2885112442, 33845, 16767, 153, 182, 81, 190, 175, 54, 88, 136);
RT_INTERFACE!{interface IProtectionPolicyManager2(IProtectionPolicyManager2Vtbl): IInspectable(IInspectableVtbl) [IID_IProtectionPolicyManager2] {
    fn put_ShowEnterpriseIndicator(&self, value: bool) -> HRESULT,
    fn get_ShowEnterpriseIndicator(&self, out: *mut bool) -> HRESULT
}}
impl IProtectionPolicyManager2 {
    #[inline] pub unsafe fn set_show_enterprise_indicator(&self, value: bool) -> Result<()> {
        let hr = ((*self.lpVtbl).put_ShowEnterpriseIndicator)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_show_enterprise_indicator(&self) -> Result<bool> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_ShowEnterpriseIndicator)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
}
DEFINE_IID!(IID_IProtectionPolicyManagerStatics, 3233807462, 35901, 19798, 136, 4, 198, 143, 10, 211, 46, 197);
RT_INTERFACE!{static interface IProtectionPolicyManagerStatics(IProtectionPolicyManagerStaticsVtbl): IInspectable(IInspectableVtbl) [IID_IProtectionPolicyManagerStatics] {
    fn IsIdentityManaged(&self, identity: HSTRING, out: *mut bool) -> HRESULT,
    fn TryApplyProcessUIPolicy(&self, identity: HSTRING, out: *mut bool) -> HRESULT,
    fn ClearProcessUIPolicy(&self) -> HRESULT,
    fn CreateCurrentThreadNetworkContext(&self, identity: HSTRING, out: *mut *mut ThreadNetworkContext) -> HRESULT,
    #[cfg(not(feature="windows-networking"))] fn __Dummy4(&self) -> (),
    #[cfg(feature="windows-networking")] fn GetPrimaryManagedIdentityForNetworkEndpointAsync(&self, endpointHost: *mut super::super::networking::HostName, out: *mut *mut super::super::foundation::IAsyncOperation<HString>) -> HRESULT,
    fn RevokeContent(&self, identity: HSTRING) -> HRESULT,
    fn GetForCurrentView(&self, out: *mut *mut ProtectionPolicyManager) -> HRESULT,
    fn add_ProtectedAccessSuspending(&self, handler: *mut super::super::foundation::EventHandler<ProtectedAccessSuspendingEventArgs>, out: *mut super::super::foundation::EventRegistrationToken) -> HRESULT,
    fn remove_ProtectedAccessSuspending(&self, token: super::super::foundation::EventRegistrationToken) -> HRESULT,
    fn add_ProtectedAccessResumed(&self, handler: *mut super::super::foundation::EventHandler<ProtectedAccessResumedEventArgs>, out: *mut super::super::foundation::EventRegistrationToken) -> HRESULT,
    fn remove_ProtectedAccessResumed(&self, token: super::super::foundation::EventRegistrationToken) -> HRESULT,
    fn add_ProtectedContentRevoked(&self, handler: *mut super::super::foundation::EventHandler<ProtectedContentRevokedEventArgs>, out: *mut super::super::foundation::EventRegistrationToken) -> HRESULT,
    fn remove_ProtectedContentRevoked(&self, token: super::super::foundation::EventRegistrationToken) -> HRESULT,
    fn CheckAccess(&self, sourceIdentity: HSTRING, targetIdentity: HSTRING, out: *mut ProtectionPolicyEvaluationResult) -> HRESULT,
    fn RequestAccessAsync(&self, sourceIdentity: HSTRING, targetIdentity: HSTRING, out: *mut *mut super::super::foundation::IAsyncOperation<ProtectionPolicyEvaluationResult>) -> HRESULT
}}
impl IProtectionPolicyManagerStatics {
    #[inline] pub unsafe fn is_identity_managed(&self, identity: &HStringArg) -> Result<bool> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).IsIdentityManaged)(self as *const _ as *mut _, identity.get(), &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn try_apply_process_uipolicy(&self, identity: &HStringArg) -> Result<bool> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).TryApplyProcessUIPolicy)(self as *const _ as *mut _, identity.get(), &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn clear_process_uipolicy(&self) -> Result<()> {
        let hr = ((*self.lpVtbl).ClearProcessUIPolicy)(self as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn create_current_thread_network_context(&self, identity: &HStringArg) -> Result<ComPtr<ThreadNetworkContext>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).CreateCurrentThreadNetworkContext)(self as *const _ as *mut _, identity.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[cfg(feature="windows-networking")] #[inline] pub unsafe fn get_primary_managed_identity_for_network_endpoint_async(&self, endpointHost: &super::super::networking::HostName) -> Result<ComPtr<super::super::foundation::IAsyncOperation<HString>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetPrimaryManagedIdentityForNetworkEndpointAsync)(self as *const _ as *mut _, endpointHost as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn revoke_content(&self, identity: &HStringArg) -> Result<()> {
        let hr = ((*self.lpVtbl).RevokeContent)(self as *const _ as *mut _, identity.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_for_current_view(&self) -> Result<ComPtr<ProtectionPolicyManager>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetForCurrentView)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn add_protected_access_suspending(&self, handler: &super::super::foundation::EventHandler<ProtectedAccessSuspendingEventArgs>) -> Result<super::super::foundation::EventRegistrationToken> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).add_ProtectedAccessSuspending)(self as *const _ as *mut _, handler as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn remove_protected_access_suspending(&self, token: super::super::foundation::EventRegistrationToken) -> Result<()> {
        let hr = ((*self.lpVtbl).remove_ProtectedAccessSuspending)(self as *const _ as *mut _, token);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn add_protected_access_resumed(&self, handler: &super::super::foundation::EventHandler<ProtectedAccessResumedEventArgs>) -> Result<super::super::foundation::EventRegistrationToken> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).add_ProtectedAccessResumed)(self as *const _ as *mut _, handler as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn remove_protected_access_resumed(&self, token: super::super::foundation::EventRegistrationToken) -> Result<()> {
        let hr = ((*self.lpVtbl).remove_ProtectedAccessResumed)(self as *const _ as *mut _, token);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn add_protected_content_revoked(&self, handler: &super::super::foundation::EventHandler<ProtectedContentRevokedEventArgs>) -> Result<super::super::foundation::EventRegistrationToken> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).add_ProtectedContentRevoked)(self as *const _ as *mut _, handler as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn remove_protected_content_revoked(&self, token: super::super::foundation::EventRegistrationToken) -> Result<()> {
        let hr = ((*self.lpVtbl).remove_ProtectedContentRevoked)(self as *const _ as *mut _, token);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn check_access(&self, sourceIdentity: &HStringArg, targetIdentity: &HStringArg) -> Result<ProtectionPolicyEvaluationResult> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).CheckAccess)(self as *const _ as *mut _, sourceIdentity.get(), targetIdentity.get(), &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn request_access_async(&self, sourceIdentity: &HStringArg, targetIdentity: &HStringArg) -> Result<ComPtr<super::super::foundation::IAsyncOperation<ProtectionPolicyEvaluationResult>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).RequestAccessAsync)(self as *const _ as *mut _, sourceIdentity.get(), targetIdentity.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
DEFINE_IID!(IID_IProtectionPolicyManagerStatics2, 3062864524, 14816, 17993, 178, 228, 7, 10, 184, 165, 121, 179);
RT_INTERFACE!{static interface IProtectionPolicyManagerStatics2(IProtectionPolicyManagerStatics2Vtbl): IInspectable(IInspectableVtbl) [IID_IProtectionPolicyManagerStatics2] {
    fn HasContentBeenRevokedSince(&self, identity: HSTRING, since: super::super::foundation::DateTime, out: *mut bool) -> HRESULT,
    fn CheckAccessForApp(&self, sourceIdentity: HSTRING, appPackageFamilyName: HSTRING, out: *mut ProtectionPolicyEvaluationResult) -> HRESULT,
    fn RequestAccessForAppAsync(&self, sourceIdentity: HSTRING, appPackageFamilyName: HSTRING, out: *mut *mut super::super::foundation::IAsyncOperation<ProtectionPolicyEvaluationResult>) -> HRESULT,
    fn GetEnforcementLevel(&self, identity: HSTRING, out: *mut EnforcementLevel) -> HRESULT,
    fn IsUserDecryptionAllowed(&self, identity: HSTRING, out: *mut bool) -> HRESULT,
    fn IsProtectionUnderLockRequired(&self, identity: HSTRING, out: *mut bool) -> HRESULT,
    fn add_PolicyChanged(&self, handler: *mut super::super::foundation::EventHandler<IInspectable>, out: *mut super::super::foundation::EventRegistrationToken) -> HRESULT,
    fn remove_PolicyChanged(&self, token: super::super::foundation::EventRegistrationToken) -> HRESULT,
    fn get_IsProtectionEnabled(&self, out: *mut bool) -> HRESULT
}}
impl IProtectionPolicyManagerStatics2 {
    #[inline] pub unsafe fn has_content_been_revoked_since(&self, identity: &HStringArg, since: super::super::foundation::DateTime) -> Result<bool> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).HasContentBeenRevokedSince)(self as *const _ as *mut _, identity.get(), since, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn check_access_for_app(&self, sourceIdentity: &HStringArg, appPackageFamilyName: &HStringArg) -> Result<ProtectionPolicyEvaluationResult> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).CheckAccessForApp)(self as *const _ as *mut _, sourceIdentity.get(), appPackageFamilyName.get(), &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn request_access_for_app_async(&self, sourceIdentity: &HStringArg, appPackageFamilyName: &HStringArg) -> Result<ComPtr<super::super::foundation::IAsyncOperation<ProtectionPolicyEvaluationResult>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).RequestAccessForAppAsync)(self as *const _ as *mut _, sourceIdentity.get(), appPackageFamilyName.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_enforcement_level(&self, identity: &HStringArg) -> Result<EnforcementLevel> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).GetEnforcementLevel)(self as *const _ as *mut _, identity.get(), &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn is_user_decryption_allowed(&self, identity: &HStringArg) -> Result<bool> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).IsUserDecryptionAllowed)(self as *const _ as *mut _, identity.get(), &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn is_protection_under_lock_required(&self, identity: &HStringArg) -> Result<bool> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).IsProtectionUnderLockRequired)(self as *const _ as *mut _, identity.get(), &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn add_policy_changed(&self, handler: &super::super::foundation::EventHandler<IInspectable>) -> Result<super::super::foundation::EventRegistrationToken> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).add_PolicyChanged)(self as *const _ as *mut _, handler as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn remove_policy_changed(&self, token: super::super::foundation::EventRegistrationToken) -> Result<()> {
        let hr = ((*self.lpVtbl).remove_PolicyChanged)(self as *const _ as *mut _, token);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_is_protection_enabled(&self) -> Result<bool> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_IsProtectionEnabled)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
}
DEFINE_IID!(IID_IProtectionPolicyManagerStatics3, 1224711820, 27247, 19871, 188, 237, 24, 171, 83, 122, 160, 21);
RT_INTERFACE!{static interface IProtectionPolicyManagerStatics3(IProtectionPolicyManagerStatics3Vtbl): IInspectable(IInspectableVtbl) [IID_IProtectionPolicyManagerStatics3] {
    fn RequestAccessWithAuditingInfoAsync(&self, sourceIdentity: HSTRING, targetIdentity: HSTRING, auditInfo: *mut ProtectionPolicyAuditInfo, out: *mut *mut super::super::foundation::IAsyncOperation<ProtectionPolicyEvaluationResult>) -> HRESULT,
    fn RequestAccessWithMessageAsync(&self, sourceIdentity: HSTRING, targetIdentity: HSTRING, auditInfo: *mut ProtectionPolicyAuditInfo, messageFromApp: HSTRING, out: *mut *mut super::super::foundation::IAsyncOperation<ProtectionPolicyEvaluationResult>) -> HRESULT,
    fn RequestAccessForAppWithAuditingInfoAsync(&self, sourceIdentity: HSTRING, appPackageFamilyName: HSTRING, auditInfo: *mut ProtectionPolicyAuditInfo, out: *mut *mut super::super::foundation::IAsyncOperation<ProtectionPolicyEvaluationResult>) -> HRESULT,
    fn RequestAccessForAppWithMessageAsync(&self, sourceIdentity: HSTRING, appPackageFamilyName: HSTRING, auditInfo: *mut ProtectionPolicyAuditInfo, messageFromApp: HSTRING, out: *mut *mut super::super::foundation::IAsyncOperation<ProtectionPolicyEvaluationResult>) -> HRESULT,
    fn LogAuditEvent(&self, sourceIdentity: HSTRING, targetIdentity: HSTRING, auditInfo: *mut ProtectionPolicyAuditInfo) -> HRESULT
}}
impl IProtectionPolicyManagerStatics3 {
    #[inline] pub unsafe fn request_access_with_auditing_info_async(&self, sourceIdentity: &HStringArg, targetIdentity: &HStringArg, auditInfo: &ProtectionPolicyAuditInfo) -> Result<ComPtr<super::super::foundation::IAsyncOperation<ProtectionPolicyEvaluationResult>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).RequestAccessWithAuditingInfoAsync)(self as *const _ as *mut _, sourceIdentity.get(), targetIdentity.get(), auditInfo as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn request_access_with_message_async(&self, sourceIdentity: &HStringArg, targetIdentity: &HStringArg, auditInfo: &ProtectionPolicyAuditInfo, messageFromApp: &HStringArg) -> Result<ComPtr<super::super::foundation::IAsyncOperation<ProtectionPolicyEvaluationResult>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).RequestAccessWithMessageAsync)(self as *const _ as *mut _, sourceIdentity.get(), targetIdentity.get(), auditInfo as *const _ as *mut _, messageFromApp.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn request_access_for_app_with_auditing_info_async(&self, sourceIdentity: &HStringArg, appPackageFamilyName: &HStringArg, auditInfo: &ProtectionPolicyAuditInfo) -> Result<ComPtr<super::super::foundation::IAsyncOperation<ProtectionPolicyEvaluationResult>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).RequestAccessForAppWithAuditingInfoAsync)(self as *const _ as *mut _, sourceIdentity.get(), appPackageFamilyName.get(), auditInfo as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn request_access_for_app_with_message_async(&self, sourceIdentity: &HStringArg, appPackageFamilyName: &HStringArg, auditInfo: &ProtectionPolicyAuditInfo, messageFromApp: &HStringArg) -> Result<ComPtr<super::super::foundation::IAsyncOperation<ProtectionPolicyEvaluationResult>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).RequestAccessForAppWithMessageAsync)(self as *const _ as *mut _, sourceIdentity.get(), appPackageFamilyName.get(), auditInfo as *const _ as *mut _, messageFromApp.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn log_audit_event(&self, sourceIdentity: &HStringArg, targetIdentity: &HStringArg, auditInfo: &ProtectionPolicyAuditInfo) -> Result<()> {
        let hr = ((*self.lpVtbl).LogAuditEvent)(self as *const _ as *mut _, sourceIdentity.get(), targetIdentity.get(), auditInfo as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
}
DEFINE_IID!(IID_IProtectionPolicyManagerStatics4, 548902107, 52413, 18703, 140, 131, 73, 204, 183, 122, 234, 108);
RT_INTERFACE!{static interface IProtectionPolicyManagerStatics4(IProtectionPolicyManagerStatics4Vtbl): IInspectable(IInspectableVtbl) [IID_IProtectionPolicyManagerStatics4] {
    fn IsRoamableProtectionEnabled(&self, identity: HSTRING, out: *mut bool) -> HRESULT,
    fn RequestAccessWithBehaviorAsync(&self, sourceIdentity: HSTRING, targetIdentity: HSTRING, auditInfo: *mut ProtectionPolicyAuditInfo, messageFromApp: HSTRING, behavior: ProtectionPolicyRequestAccessBehavior, out: *mut *mut super::super::foundation::IAsyncOperation<ProtectionPolicyEvaluationResult>) -> HRESULT,
    fn RequestAccessForAppWithBehaviorAsync(&self, sourceIdentity: HSTRING, appPackageFamilyName: HSTRING, auditInfo: *mut ProtectionPolicyAuditInfo, messageFromApp: HSTRING, behavior: ProtectionPolicyRequestAccessBehavior, out: *mut *mut super::super::foundation::IAsyncOperation<ProtectionPolicyEvaluationResult>) -> HRESULT,
    #[cfg(not(feature="windows-storage"))] fn __Dummy3(&self) -> (),
    #[cfg(feature="windows-storage")] fn RequestAccessToFilesForAppAsync(&self, sourceItemList: *mut super::super::foundation::collections::IIterable<super::super::storage::IStorageItem>, appPackageFamilyName: HSTRING, auditInfo: *mut ProtectionPolicyAuditInfo, out: *mut *mut super::super::foundation::IAsyncOperation<ProtectionPolicyEvaluationResult>) -> HRESULT,
    #[cfg(not(feature="windows-storage"))] fn __Dummy4(&self) -> (),
    #[cfg(feature="windows-storage")] fn RequestAccessToFilesForAppWithMessageAndBehaviorAsync(&self, sourceItemList: *mut super::super::foundation::collections::IIterable<super::super::storage::IStorageItem>, appPackageFamilyName: HSTRING, auditInfo: *mut ProtectionPolicyAuditInfo, messageFromApp: HSTRING, behavior: ProtectionPolicyRequestAccessBehavior, out: *mut *mut super::super::foundation::IAsyncOperation<ProtectionPolicyEvaluationResult>) -> HRESULT,
    #[cfg(not(feature="windows-storage"))] fn __Dummy5(&self) -> (),
    #[cfg(feature="windows-storage")] fn RequestAccessToFilesForProcessAsync(&self, sourceItemList: *mut super::super::foundation::collections::IIterable<super::super::storage::IStorageItem>, processId: u32, auditInfo: *mut ProtectionPolicyAuditInfo, out: *mut *mut super::super::foundation::IAsyncOperation<ProtectionPolicyEvaluationResult>) -> HRESULT,
    #[cfg(not(feature="windows-storage"))] fn __Dummy6(&self) -> (),
    #[cfg(feature="windows-storage")] fn RequestAccessToFilesForProcessWithMessageAndBehaviorAsync(&self, sourceItemList: *mut super::super::foundation::collections::IIterable<super::super::storage::IStorageItem>, processId: u32, auditInfo: *mut ProtectionPolicyAuditInfo, messageFromApp: HSTRING, behavior: ProtectionPolicyRequestAccessBehavior, out: *mut *mut super::super::foundation::IAsyncOperation<ProtectionPolicyEvaluationResult>) -> HRESULT,
    #[cfg(not(feature="windows-storage"))] fn __Dummy7(&self) -> (),
    #[cfg(feature="windows-storage")] fn IsFileProtectionRequiredAsync(&self, target: *mut super::super::storage::IStorageItem, identity: HSTRING, out: *mut *mut super::super::foundation::IAsyncOperation<bool>) -> HRESULT,
    #[cfg(not(feature="windows-storage"))] fn __Dummy8(&self) -> (),
    #[cfg(feature="windows-storage")] fn IsFileProtectionRequiredForNewFileAsync(&self, parentFolder: *mut super::super::storage::IStorageFolder, identity: HSTRING, desiredName: HSTRING, out: *mut *mut super::super::foundation::IAsyncOperation<bool>) -> HRESULT,
    fn get_PrimaryManagedIdentity(&self, out: *mut HSTRING) -> HRESULT,
    fn GetPrimaryManagedIdentityForIdentity(&self, identity: HSTRING, out: *mut HSTRING) -> HRESULT
}}
impl IProtectionPolicyManagerStatics4 {
    #[inline] pub unsafe fn is_roamable_protection_enabled(&self, identity: &HStringArg) -> Result<bool> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).IsRoamableProtectionEnabled)(self as *const _ as *mut _, identity.get(), &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn request_access_with_behavior_async(&self, sourceIdentity: &HStringArg, targetIdentity: &HStringArg, auditInfo: &ProtectionPolicyAuditInfo, messageFromApp: &HStringArg, behavior: ProtectionPolicyRequestAccessBehavior) -> Result<ComPtr<super::super::foundation::IAsyncOperation<ProtectionPolicyEvaluationResult>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).RequestAccessWithBehaviorAsync)(self as *const _ as *mut _, sourceIdentity.get(), targetIdentity.get(), auditInfo as *const _ as *mut _, messageFromApp.get(), behavior, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn request_access_for_app_with_behavior_async(&self, sourceIdentity: &HStringArg, appPackageFamilyName: &HStringArg, auditInfo: &ProtectionPolicyAuditInfo, messageFromApp: &HStringArg, behavior: ProtectionPolicyRequestAccessBehavior) -> Result<ComPtr<super::super::foundation::IAsyncOperation<ProtectionPolicyEvaluationResult>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).RequestAccessForAppWithBehaviorAsync)(self as *const _ as *mut _, sourceIdentity.get(), appPackageFamilyName.get(), auditInfo as *const _ as *mut _, messageFromApp.get(), behavior, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[cfg(feature="windows-storage")] #[inline] pub unsafe fn request_access_to_files_for_app_async(&self, sourceItemList: &super::super::foundation::collections::IIterable<super::super::storage::IStorageItem>, appPackageFamilyName: &HStringArg, auditInfo: &ProtectionPolicyAuditInfo) -> Result<ComPtr<super::super::foundation::IAsyncOperation<ProtectionPolicyEvaluationResult>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).RequestAccessToFilesForAppAsync)(self as *const _ as *mut _, sourceItemList as *const _ as *mut _, appPackageFamilyName.get(), auditInfo as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[cfg(feature="windows-storage")] #[inline] pub unsafe fn request_access_to_files_for_app_with_message_and_behavior_async(&self, sourceItemList: &super::super::foundation::collections::IIterable<super::super::storage::IStorageItem>, appPackageFamilyName: &HStringArg, auditInfo: &ProtectionPolicyAuditInfo, messageFromApp: &HStringArg, behavior: ProtectionPolicyRequestAccessBehavior) -> Result<ComPtr<super::super::foundation::IAsyncOperation<ProtectionPolicyEvaluationResult>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).RequestAccessToFilesForAppWithMessageAndBehaviorAsync)(self as *const _ as *mut _, sourceItemList as *const _ as *mut _, appPackageFamilyName.get(), auditInfo as *const _ as *mut _, messageFromApp.get(), behavior, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[cfg(feature="windows-storage")] #[inline] pub unsafe fn request_access_to_files_for_process_async(&self, sourceItemList: &super::super::foundation::collections::IIterable<super::super::storage::IStorageItem>, processId: u32, auditInfo: &ProtectionPolicyAuditInfo) -> Result<ComPtr<super::super::foundation::IAsyncOperation<ProtectionPolicyEvaluationResult>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).RequestAccessToFilesForProcessAsync)(self as *const _ as *mut _, sourceItemList as *const _ as *mut _, processId, auditInfo as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[cfg(feature="windows-storage")] #[inline] pub unsafe fn request_access_to_files_for_process_with_message_and_behavior_async(&self, sourceItemList: &super::super::foundation::collections::IIterable<super::super::storage::IStorageItem>, processId: u32, auditInfo: &ProtectionPolicyAuditInfo, messageFromApp: &HStringArg, behavior: ProtectionPolicyRequestAccessBehavior) -> Result<ComPtr<super::super::foundation::IAsyncOperation<ProtectionPolicyEvaluationResult>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).RequestAccessToFilesForProcessWithMessageAndBehaviorAsync)(self as *const _ as *mut _, sourceItemList as *const _ as *mut _, processId, auditInfo as *const _ as *mut _, messageFromApp.get(), behavior, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[cfg(feature="windows-storage")] #[inline] pub unsafe fn is_file_protection_required_async(&self, target: &super::super::storage::IStorageItem, identity: &HStringArg) -> Result<ComPtr<super::super::foundation::IAsyncOperation<bool>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).IsFileProtectionRequiredAsync)(self as *const _ as *mut _, target as *const _ as *mut _, identity.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[cfg(feature="windows-storage")] #[inline] pub unsafe fn is_file_protection_required_for_new_file_async(&self, parentFolder: &super::super::storage::IStorageFolder, identity: &HStringArg, desiredName: &HStringArg) -> Result<ComPtr<super::super::foundation::IAsyncOperation<bool>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).IsFileProtectionRequiredForNewFileAsync)(self as *const _ as *mut _, parentFolder as *const _ as *mut _, identity.get(), desiredName.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_primary_managed_identity(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_PrimaryManagedIdentity)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_primary_managed_identity_for_identity(&self, identity: &HStringArg) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetPrimaryManagedIdentityForIdentity)(self as *const _ as *mut _, identity.get(), &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
}
RT_ENUM! { enum ProtectionPolicyRequestAccessBehavior: i32 {
    Decrypt (ProtectionPolicyRequestAccessBehavior_Decrypt) = 0, TreatOverridePolicyAsBlock (ProtectionPolicyRequestAccessBehavior_TreatOverridePolicyAsBlock) = 1,
}}
DEFINE_IID!(IID_IThreadNetworkContext, 4199459049, 61203, 16474, 177, 44, 215, 52, 140, 111, 65, 252);
RT_INTERFACE!{interface IThreadNetworkContext(IThreadNetworkContextVtbl): IInspectable(IInspectableVtbl) [IID_IThreadNetworkContext] {
    
}}
RT_CLASS!{class ThreadNetworkContext: IThreadNetworkContext}
} // Windows.Security.EnterpriseData
pub mod exchangeactivesyncprovisioning { // Windows.Security.ExchangeActiveSyncProvisioning
use ::prelude::*;
DEFINE_IID!(IID_IEasClientDeviceInformation, 1423956353, 6504, 19619, 185, 88, 229, 149, 209, 101, 5, 235);
RT_INTERFACE!{interface IEasClientDeviceInformation(IEasClientDeviceInformationVtbl): IInspectable(IInspectableVtbl) [IID_IEasClientDeviceInformation] {
    fn get_Id(&self, out: *mut Guid) -> HRESULT,
    fn get_OperatingSystem(&self, out: *mut HSTRING) -> HRESULT,
    fn get_FriendlyName(&self, out: *mut HSTRING) -> HRESULT,
    fn get_SystemManufacturer(&self, out: *mut HSTRING) -> HRESULT,
    fn get_SystemProductName(&self, out: *mut HSTRING) -> HRESULT,
    fn get_SystemSku(&self, out: *mut HSTRING) -> HRESULT
}}
impl IEasClientDeviceInformation {
    #[inline] pub unsafe fn get_id(&self) -> Result<Guid> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_Id)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_operating_system(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_OperatingSystem)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_friendly_name(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_FriendlyName)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_system_manufacturer(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_SystemManufacturer)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_system_product_name(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_SystemProductName)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_system_sku(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_SystemSku)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
}
RT_CLASS!{class EasClientDeviceInformation: IEasClientDeviceInformation}
impl RtActivatable<IActivationFactory> for EasClientDeviceInformation {}
DEFINE_CLSID!(EasClientDeviceInformation(&[87,105,110,100,111,119,115,46,83,101,99,117,114,105,116,121,46,69,120,99,104,97,110,103,101,65,99,116,105,118,101,83,121,110,99,80,114,111,118,105,115,105,111,110,105,110,103,46,69,97,115,67,108,105,101,110,116,68,101,118,105,99,101,73,110,102,111,114,109,97,116,105,111,110,0]) [CLSID_EasClientDeviceInformation]);
DEFINE_IID!(IID_IEasClientDeviceInformation2, 4289943843, 47910, 19818, 129, 188, 22, 90, 238, 10, 215, 84);
RT_INTERFACE!{interface IEasClientDeviceInformation2(IEasClientDeviceInformation2Vtbl): IInspectable(IInspectableVtbl) [IID_IEasClientDeviceInformation2] {
    fn get_SystemHardwareVersion(&self, out: *mut HSTRING) -> HRESULT,
    fn get_SystemFirmwareVersion(&self, out: *mut HSTRING) -> HRESULT
}}
impl IEasClientDeviceInformation2 {
    #[inline] pub unsafe fn get_system_hardware_version(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_SystemHardwareVersion)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_system_firmware_version(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_SystemFirmwareVersion)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
}
DEFINE_IID!(IID_IEasClientSecurityPolicy, 1169630050, 57274, 19099, 172, 237, 111, 226, 173, 203, 100, 32);
RT_INTERFACE!{interface IEasClientSecurityPolicy(IEasClientSecurityPolicyVtbl): IInspectable(IInspectableVtbl) [IID_IEasClientSecurityPolicy] {
    fn get_RequireEncryption(&self, out: *mut bool) -> HRESULT,
    fn put_RequireEncryption(&self, value: bool) -> HRESULT,
    fn get_MinPasswordLength(&self, out: *mut u8) -> HRESULT,
    fn put_MinPasswordLength(&self, value: u8) -> HRESULT,
    fn get_DisallowConvenienceLogon(&self, out: *mut bool) -> HRESULT,
    fn put_DisallowConvenienceLogon(&self, value: bool) -> HRESULT,
    fn get_MinPasswordComplexCharacters(&self, out: *mut u8) -> HRESULT,
    fn put_MinPasswordComplexCharacters(&self, value: u8) -> HRESULT,
    fn get_PasswordExpiration(&self, out: *mut super::super::foundation::TimeSpan) -> HRESULT,
    fn put_PasswordExpiration(&self, value: super::super::foundation::TimeSpan) -> HRESULT,
    fn get_PasswordHistory(&self, out: *mut u32) -> HRESULT,
    fn put_PasswordHistory(&self, value: u32) -> HRESULT,
    fn get_MaxPasswordFailedAttempts(&self, out: *mut u8) -> HRESULT,
    fn put_MaxPasswordFailedAttempts(&self, value: u8) -> HRESULT,
    fn get_MaxInactivityTimeLock(&self, out: *mut super::super::foundation::TimeSpan) -> HRESULT,
    fn put_MaxInactivityTimeLock(&self, value: super::super::foundation::TimeSpan) -> HRESULT,
    fn CheckCompliance(&self, out: *mut *mut EasComplianceResults) -> HRESULT,
    fn ApplyAsync(&self, out: *mut *mut super::super::foundation::IAsyncOperation<EasComplianceResults>) -> HRESULT
}}
impl IEasClientSecurityPolicy {
    #[inline] pub unsafe fn get_require_encryption(&self) -> Result<bool> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_RequireEncryption)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_require_encryption(&self, value: bool) -> Result<()> {
        let hr = ((*self.lpVtbl).put_RequireEncryption)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_min_password_length(&self) -> Result<u8> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_MinPasswordLength)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_min_password_length(&self, value: u8) -> Result<()> {
        let hr = ((*self.lpVtbl).put_MinPasswordLength)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_disallow_convenience_logon(&self) -> Result<bool> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_DisallowConvenienceLogon)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_disallow_convenience_logon(&self, value: bool) -> Result<()> {
        let hr = ((*self.lpVtbl).put_DisallowConvenienceLogon)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_min_password_complex_characters(&self) -> Result<u8> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_MinPasswordComplexCharacters)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_min_password_complex_characters(&self, value: u8) -> Result<()> {
        let hr = ((*self.lpVtbl).put_MinPasswordComplexCharacters)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_password_expiration(&self) -> Result<super::super::foundation::TimeSpan> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_PasswordExpiration)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_password_expiration(&self, value: super::super::foundation::TimeSpan) -> Result<()> {
        let hr = ((*self.lpVtbl).put_PasswordExpiration)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_password_history(&self) -> Result<u32> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_PasswordHistory)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_password_history(&self, value: u32) -> Result<()> {
        let hr = ((*self.lpVtbl).put_PasswordHistory)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_max_password_failed_attempts(&self) -> Result<u8> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_MaxPasswordFailedAttempts)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_max_password_failed_attempts(&self, value: u8) -> Result<()> {
        let hr = ((*self.lpVtbl).put_MaxPasswordFailedAttempts)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_max_inactivity_time_lock(&self) -> Result<super::super::foundation::TimeSpan> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_MaxInactivityTimeLock)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_max_inactivity_time_lock(&self, value: super::super::foundation::TimeSpan) -> Result<()> {
        let hr = ((*self.lpVtbl).put_MaxInactivityTimeLock)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn check_compliance(&self) -> Result<ComPtr<EasComplianceResults>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).CheckCompliance)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn apply_async(&self) -> Result<ComPtr<super::super::foundation::IAsyncOperation<EasComplianceResults>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).ApplyAsync)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
RT_CLASS!{class EasClientSecurityPolicy: IEasClientSecurityPolicy}
impl RtActivatable<IActivationFactory> for EasClientSecurityPolicy {}
DEFINE_CLSID!(EasClientSecurityPolicy(&[87,105,110,100,111,119,115,46,83,101,99,117,114,105,116,121,46,69,120,99,104,97,110,103,101,65,99,116,105,118,101,83,121,110,99,80,114,111,118,105,115,105,111,110,105,110,103,46,69,97,115,67,108,105,101,110,116,83,101,99,117,114,105,116,121,80,111,108,105,99,121,0]) [CLSID_EasClientSecurityPolicy]);
DEFINE_IID!(IID_IEasComplianceResults, 1178347932, 32537, 19558, 180, 3, 203, 69, 221, 87, 162, 179);
RT_INTERFACE!{interface IEasComplianceResults(IEasComplianceResultsVtbl): IInspectable(IInspectableVtbl) [IID_IEasComplianceResults] {
    fn get_Compliant(&self, out: *mut bool) -> HRESULT,
    fn get_RequireEncryptionResult(&self, out: *mut EasRequireEncryptionResult) -> HRESULT,
    fn get_MinPasswordLengthResult(&self, out: *mut EasMinPasswordLengthResult) -> HRESULT,
    fn get_DisallowConvenienceLogonResult(&self, out: *mut EasDisallowConvenienceLogonResult) -> HRESULT,
    fn get_MinPasswordComplexCharactersResult(&self, out: *mut EasMinPasswordComplexCharactersResult) -> HRESULT,
    fn get_PasswordExpirationResult(&self, out: *mut EasPasswordExpirationResult) -> HRESULT,
    fn get_PasswordHistoryResult(&self, out: *mut EasPasswordHistoryResult) -> HRESULT,
    fn get_MaxPasswordFailedAttemptsResult(&self, out: *mut EasMaxPasswordFailedAttemptsResult) -> HRESULT,
    fn get_MaxInactivityTimeLockResult(&self, out: *mut EasMaxInactivityTimeLockResult) -> HRESULT
}}
impl IEasComplianceResults {
    #[inline] pub unsafe fn get_compliant(&self) -> Result<bool> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_Compliant)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_require_encryption_result(&self) -> Result<EasRequireEncryptionResult> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_RequireEncryptionResult)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_min_password_length_result(&self) -> Result<EasMinPasswordLengthResult> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_MinPasswordLengthResult)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_disallow_convenience_logon_result(&self) -> Result<EasDisallowConvenienceLogonResult> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_DisallowConvenienceLogonResult)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_min_password_complex_characters_result(&self) -> Result<EasMinPasswordComplexCharactersResult> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_MinPasswordComplexCharactersResult)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_password_expiration_result(&self) -> Result<EasPasswordExpirationResult> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_PasswordExpirationResult)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_password_history_result(&self) -> Result<EasPasswordHistoryResult> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_PasswordHistoryResult)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_max_password_failed_attempts_result(&self) -> Result<EasMaxPasswordFailedAttemptsResult> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_MaxPasswordFailedAttemptsResult)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_max_inactivity_time_lock_result(&self) -> Result<EasMaxInactivityTimeLockResult> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_MaxInactivityTimeLockResult)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
}
RT_CLASS!{class EasComplianceResults: IEasComplianceResults}
DEFINE_IID!(IID_IEasComplianceResults2, 801005769, 6824, 18421, 136, 187, 203, 62, 240, 191, 251, 21);
RT_INTERFACE!{interface IEasComplianceResults2(IEasComplianceResults2Vtbl): IInspectable(IInspectableVtbl) [IID_IEasComplianceResults2] {
    fn get_EncryptionProviderType(&self, out: *mut EasEncryptionProviderType) -> HRESULT
}}
impl IEasComplianceResults2 {
    #[inline] pub unsafe fn get_encryption_provider_type(&self) -> Result<EasEncryptionProviderType> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_EncryptionProviderType)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
}
RT_ENUM! { enum EasDisallowConvenienceLogonResult: i32 {
    NotEvaluated (EasDisallowConvenienceLogonResult_NotEvaluated) = 0, Compliant (EasDisallowConvenienceLogonResult_Compliant) = 1, CanBeCompliant (EasDisallowConvenienceLogonResult_CanBeCompliant) = 2, RequestedPolicyIsStricter (EasDisallowConvenienceLogonResult_RequestedPolicyIsStricter) = 3,
}}
RT_ENUM! { enum EasEncryptionProviderType: i32 {
    NotEvaluated (EasEncryptionProviderType_NotEvaluated) = 0, WindowsEncryption (EasEncryptionProviderType_WindowsEncryption) = 1, OtherEncryption (EasEncryptionProviderType_OtherEncryption) = 2,
}}
RT_ENUM! { enum EasMaxInactivityTimeLockResult: i32 {
    NotEvaluated (EasMaxInactivityTimeLockResult_NotEvaluated) = 0, Compliant (EasMaxInactivityTimeLockResult_Compliant) = 1, CanBeCompliant (EasMaxInactivityTimeLockResult_CanBeCompliant) = 2, RequestedPolicyIsStricter (EasMaxInactivityTimeLockResult_RequestedPolicyIsStricter) = 3, InvalidParameter (EasMaxInactivityTimeLockResult_InvalidParameter) = 4,
}}
RT_ENUM! { enum EasMaxPasswordFailedAttemptsResult: i32 {
    NotEvaluated (EasMaxPasswordFailedAttemptsResult_NotEvaluated) = 0, Compliant (EasMaxPasswordFailedAttemptsResult_Compliant) = 1, CanBeCompliant (EasMaxPasswordFailedAttemptsResult_CanBeCompliant) = 2, RequestedPolicyIsStricter (EasMaxPasswordFailedAttemptsResult_RequestedPolicyIsStricter) = 3, InvalidParameter (EasMaxPasswordFailedAttemptsResult_InvalidParameter) = 4,
}}
RT_ENUM! { enum EasMinPasswordComplexCharactersResult: i32 {
    NotEvaluated (EasMinPasswordComplexCharactersResult_NotEvaluated) = 0, Compliant (EasMinPasswordComplexCharactersResult_Compliant) = 1, CanBeCompliant (EasMinPasswordComplexCharactersResult_CanBeCompliant) = 2, RequestedPolicyIsStricter (EasMinPasswordComplexCharactersResult_RequestedPolicyIsStricter) = 3, RequestedPolicyNotEnforceable (EasMinPasswordComplexCharactersResult_RequestedPolicyNotEnforceable) = 4, InvalidParameter (EasMinPasswordComplexCharactersResult_InvalidParameter) = 5, CurrentUserHasBlankPassword (EasMinPasswordComplexCharactersResult_CurrentUserHasBlankPassword) = 6, AdminsHaveBlankPassword (EasMinPasswordComplexCharactersResult_AdminsHaveBlankPassword) = 7, UserCannotChangePassword (EasMinPasswordComplexCharactersResult_UserCannotChangePassword) = 8, AdminsCannotChangePassword (EasMinPasswordComplexCharactersResult_AdminsCannotChangePassword) = 9, LocalControlledUsersCannotChangePassword (EasMinPasswordComplexCharactersResult_LocalControlledUsersCannotChangePassword) = 10, ConnectedAdminsProviderPolicyIsWeak (EasMinPasswordComplexCharactersResult_ConnectedAdminsProviderPolicyIsWeak) = 11, ConnectedUserProviderPolicyIsWeak (EasMinPasswordComplexCharactersResult_ConnectedUserProviderPolicyIsWeak) = 12, ChangeConnectedAdminsPassword (EasMinPasswordComplexCharactersResult_ChangeConnectedAdminsPassword) = 13, ChangeConnectedUserPassword (EasMinPasswordComplexCharactersResult_ChangeConnectedUserPassword) = 14,
}}
RT_ENUM! { enum EasMinPasswordLengthResult: i32 {
    NotEvaluated (EasMinPasswordLengthResult_NotEvaluated) = 0, Compliant (EasMinPasswordLengthResult_Compliant) = 1, CanBeCompliant (EasMinPasswordLengthResult_CanBeCompliant) = 2, RequestedPolicyIsStricter (EasMinPasswordLengthResult_RequestedPolicyIsStricter) = 3, RequestedPolicyNotEnforceable (EasMinPasswordLengthResult_RequestedPolicyNotEnforceable) = 4, InvalidParameter (EasMinPasswordLengthResult_InvalidParameter) = 5, CurrentUserHasBlankPassword (EasMinPasswordLengthResult_CurrentUserHasBlankPassword) = 6, AdminsHaveBlankPassword (EasMinPasswordLengthResult_AdminsHaveBlankPassword) = 7, UserCannotChangePassword (EasMinPasswordLengthResult_UserCannotChangePassword) = 8, AdminsCannotChangePassword (EasMinPasswordLengthResult_AdminsCannotChangePassword) = 9, LocalControlledUsersCannotChangePassword (EasMinPasswordLengthResult_LocalControlledUsersCannotChangePassword) = 10, ConnectedAdminsProviderPolicyIsWeak (EasMinPasswordLengthResult_ConnectedAdminsProviderPolicyIsWeak) = 11, ConnectedUserProviderPolicyIsWeak (EasMinPasswordLengthResult_ConnectedUserProviderPolicyIsWeak) = 12, ChangeConnectedAdminsPassword (EasMinPasswordLengthResult_ChangeConnectedAdminsPassword) = 13, ChangeConnectedUserPassword (EasMinPasswordLengthResult_ChangeConnectedUserPassword) = 14,
}}
RT_ENUM! { enum EasPasswordExpirationResult: i32 {
    NotEvaluated (EasPasswordExpirationResult_NotEvaluated) = 0, Compliant (EasPasswordExpirationResult_Compliant) = 1, CanBeCompliant (EasPasswordExpirationResult_CanBeCompliant) = 2, RequestedPolicyIsStricter (EasPasswordExpirationResult_RequestedPolicyIsStricter) = 3, RequestedExpirationIncompatible (EasPasswordExpirationResult_RequestedExpirationIncompatible) = 4, InvalidParameter (EasPasswordExpirationResult_InvalidParameter) = 5, UserCannotChangePassword (EasPasswordExpirationResult_UserCannotChangePassword) = 6, AdminsCannotChangePassword (EasPasswordExpirationResult_AdminsCannotChangePassword) = 7, LocalControlledUsersCannotChangePassword (EasPasswordExpirationResult_LocalControlledUsersCannotChangePassword) = 8,
}}
RT_ENUM! { enum EasPasswordHistoryResult: i32 {
    NotEvaluated (EasPasswordHistoryResult_NotEvaluated) = 0, Compliant (EasPasswordHistoryResult_Compliant) = 1, CanBeCompliant (EasPasswordHistoryResult_CanBeCompliant) = 2, RequestedPolicyIsStricter (EasPasswordHistoryResult_RequestedPolicyIsStricter) = 3, InvalidParameter (EasPasswordHistoryResult_InvalidParameter) = 4,
}}
RT_ENUM! { enum EasRequireEncryptionResult: i32 {
    NotEvaluated (EasRequireEncryptionResult_NotEvaluated) = 0, Compliant (EasRequireEncryptionResult_Compliant) = 1, CanBeCompliant (EasRequireEncryptionResult_CanBeCompliant) = 2, NotProvisionedOnAllVolumes (EasRequireEncryptionResult_NotProvisionedOnAllVolumes) = 3, DeFixedDataNotSupported (EasRequireEncryptionResult_DeFixedDataNotSupported) = 4, FixedDataNotSupported (EasRequireEncryptionResult_FixedDataNotSupported) = 4, DeHardwareNotCompliant (EasRequireEncryptionResult_DeHardwareNotCompliant) = 5, HardwareNotCompliant (EasRequireEncryptionResult_HardwareNotCompliant) = 5, DeWinReNotConfigured (EasRequireEncryptionResult_DeWinReNotConfigured) = 6, LockNotConfigured (EasRequireEncryptionResult_LockNotConfigured) = 6, DeProtectionSuspended (EasRequireEncryptionResult_DeProtectionSuspended) = 7, ProtectionSuspended (EasRequireEncryptionResult_ProtectionSuspended) = 7, DeOsVolumeNotProtected (EasRequireEncryptionResult_DeOsVolumeNotProtected) = 8, OsVolumeNotProtected (EasRequireEncryptionResult_OsVolumeNotProtected) = 8, DeProtectionNotYetEnabled (EasRequireEncryptionResult_DeProtectionNotYetEnabled) = 9, ProtectionNotYetEnabled (EasRequireEncryptionResult_ProtectionNotYetEnabled) = 9, NoFeatureLicense (EasRequireEncryptionResult_NoFeatureLicense) = 10, OsNotProtected (EasRequireEncryptionResult_OsNotProtected) = 11, UnexpectedFailure (EasRequireEncryptionResult_UnexpectedFailure) = 12,
}}
} // Windows.Security.ExchangeActiveSyncProvisioning
