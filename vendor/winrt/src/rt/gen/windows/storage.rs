use ::prelude::*;
DEFINE_IID!(IID_IAppDataPaths, 1929500170, 31138, 18633, 158, 192, 63, 218, 9, 47, 121, 225);
RT_INTERFACE!{interface IAppDataPaths(IAppDataPathsVtbl): IInspectable(IInspectableVtbl) [IID_IAppDataPaths] {
    fn get_Cookies(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Desktop(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Documents(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Favorites(&self, out: *mut HSTRING) -> HRESULT,
    fn get_History(&self, out: *mut HSTRING) -> HRESULT,
    fn get_InternetCache(&self, out: *mut HSTRING) -> HRESULT,
    fn get_LocalAppData(&self, out: *mut HSTRING) -> HRESULT,
    fn get_ProgramData(&self, out: *mut HSTRING) -> HRESULT,
    fn get_RoamingAppData(&self, out: *mut HSTRING) -> HRESULT
}}
impl IAppDataPaths {
    #[inline] pub unsafe fn get_cookies(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Cookies)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_desktop(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Desktop)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_documents(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Documents)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_favorites(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Favorites)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_history(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_History)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_internet_cache(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_InternetCache)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_local_app_data(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_LocalAppData)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_program_data(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_ProgramData)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_roaming_app_data(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_RoamingAppData)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
}
RT_CLASS!{class AppDataPaths: IAppDataPaths}
impl RtActivatable<IAppDataPathsStatics> for AppDataPaths {}
impl AppDataPaths {
    #[cfg(feature="windows-system")] #[inline] pub fn get_for_user(user: &super::system::User) -> Result<ComPtr<AppDataPaths>> { unsafe {
        <Self as RtActivatable<IAppDataPathsStatics>>::get_activation_factory().get_for_user(user)
    }}
    #[inline] pub fn get_default() -> Result<ComPtr<AppDataPaths>> { unsafe {
        <Self as RtActivatable<IAppDataPathsStatics>>::get_activation_factory().get_default()
    }}
}
DEFINE_CLSID!(AppDataPaths(&[87,105,110,100,111,119,115,46,83,116,111,114,97,103,101,46,65,112,112,68,97,116,97,80,97,116,104,115,0]) [CLSID_AppDataPaths]);
DEFINE_IID!(IID_IAppDataPathsStatics, 3639290622, 43481, 19220, 185, 153, 227, 146, 19, 121, 217, 3);
RT_INTERFACE!{static interface IAppDataPathsStatics(IAppDataPathsStaticsVtbl): IInspectable(IInspectableVtbl) [IID_IAppDataPathsStatics] {
    #[cfg(not(feature="windows-system"))] fn __Dummy0(&self) -> (),
    #[cfg(feature="windows-system")] fn GetForUser(&self, user: *mut super::system::User, out: *mut *mut AppDataPaths) -> HRESULT,
    fn GetDefault(&self, out: *mut *mut AppDataPaths) -> HRESULT
}}
impl IAppDataPathsStatics {
    #[cfg(feature="windows-system")] #[inline] pub unsafe fn get_for_user(&self, user: &super::system::User) -> Result<ComPtr<AppDataPaths>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetForUser)(self as *const _ as *mut _, user as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_default(&self) -> Result<ComPtr<AppDataPaths>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetDefault)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
DEFINE_IID!(IID_IApplicationData, 3285872567, 46916, 19269, 176, 184, 34, 58, 9, 56, 208, 220);
RT_INTERFACE!{interface IApplicationData(IApplicationDataVtbl): IInspectable(IInspectableVtbl) [IID_IApplicationData] {
    fn get_Version(&self, out: *mut u32) -> HRESULT,
    fn SetVersionAsync(&self, desiredVersion: u32, handler: *mut ApplicationDataSetVersionHandler, out: *mut *mut super::foundation::IAsyncAction) -> HRESULT,
    fn ClearAllAsync(&self, out: *mut *mut super::foundation::IAsyncAction) -> HRESULT,
    fn ClearAsync(&self, locality: ApplicationDataLocality, out: *mut *mut super::foundation::IAsyncAction) -> HRESULT,
    fn get_LocalSettings(&self, out: *mut *mut ApplicationDataContainer) -> HRESULT,
    fn get_RoamingSettings(&self, out: *mut *mut ApplicationDataContainer) -> HRESULT,
    fn get_LocalFolder(&self, out: *mut *mut StorageFolder) -> HRESULT,
    fn get_RoamingFolder(&self, out: *mut *mut StorageFolder) -> HRESULT,
    fn get_TemporaryFolder(&self, out: *mut *mut StorageFolder) -> HRESULT,
    fn add_DataChanged(&self, handler: *mut super::foundation::TypedEventHandler<ApplicationData, IInspectable>, out: *mut super::foundation::EventRegistrationToken) -> HRESULT,
    fn remove_DataChanged(&self, token: super::foundation::EventRegistrationToken) -> HRESULT,
    fn SignalDataChanged(&self) -> HRESULT,
    fn get_RoamingStorageQuota(&self, out: *mut u64) -> HRESULT
}}
impl IApplicationData {
    #[inline] pub unsafe fn get_version(&self) -> Result<u32> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_Version)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_version_async(&self, desiredVersion: u32, handler: &ApplicationDataSetVersionHandler) -> Result<ComPtr<super::foundation::IAsyncAction>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).SetVersionAsync)(self as *const _ as *mut _, desiredVersion, handler as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn clear_all_async(&self) -> Result<ComPtr<super::foundation::IAsyncAction>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).ClearAllAsync)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn clear_async(&self, locality: ApplicationDataLocality) -> Result<ComPtr<super::foundation::IAsyncAction>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).ClearAsync)(self as *const _ as *mut _, locality, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_local_settings(&self) -> Result<ComPtr<ApplicationDataContainer>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_LocalSettings)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_roaming_settings(&self) -> Result<ComPtr<ApplicationDataContainer>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_RoamingSettings)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_local_folder(&self) -> Result<ComPtr<StorageFolder>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_LocalFolder)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_roaming_folder(&self) -> Result<ComPtr<StorageFolder>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_RoamingFolder)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_temporary_folder(&self) -> Result<ComPtr<StorageFolder>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_TemporaryFolder)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn add_data_changed(&self, handler: &super::foundation::TypedEventHandler<ApplicationData, IInspectable>) -> Result<super::foundation::EventRegistrationToken> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).add_DataChanged)(self as *const _ as *mut _, handler as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn remove_data_changed(&self, token: super::foundation::EventRegistrationToken) -> Result<()> {
        let hr = ((*self.lpVtbl).remove_DataChanged)(self as *const _ as *mut _, token);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn signal_data_changed(&self) -> Result<()> {
        let hr = ((*self.lpVtbl).SignalDataChanged)(self as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_roaming_storage_quota(&self) -> Result<u64> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_RoamingStorageQuota)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
}
RT_CLASS!{class ApplicationData: IApplicationData}
impl RtActivatable<IApplicationDataStatics> for ApplicationData {}
impl RtActivatable<IApplicationDataStatics2> for ApplicationData {}
impl ApplicationData {
    #[inline] pub fn get_current() -> Result<ComPtr<ApplicationData>> { unsafe {
        <Self as RtActivatable<IApplicationDataStatics>>::get_activation_factory().get_current()
    }}
    #[cfg(feature="windows-system")] #[inline] pub fn get_for_user_async(user: &super::system::User) -> Result<ComPtr<super::foundation::IAsyncOperation<ApplicationData>>> { unsafe {
        <Self as RtActivatable<IApplicationDataStatics2>>::get_activation_factory().get_for_user_async(user)
    }}
}
DEFINE_CLSID!(ApplicationData(&[87,105,110,100,111,119,115,46,83,116,111,114,97,103,101,46,65,112,112,108,105,99,97,116,105,111,110,68,97,116,97,0]) [CLSID_ApplicationData]);
DEFINE_IID!(IID_IApplicationData2, 2657471849, 2979, 20018, 190, 41, 176, 45, 230, 96, 118, 56);
RT_INTERFACE!{interface IApplicationData2(IApplicationData2Vtbl): IInspectable(IInspectableVtbl) [IID_IApplicationData2] {
    fn get_LocalCacheFolder(&self, out: *mut *mut StorageFolder) -> HRESULT
}}
impl IApplicationData2 {
    #[inline] pub unsafe fn get_local_cache_folder(&self) -> Result<ComPtr<StorageFolder>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_LocalCacheFolder)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
DEFINE_IID!(IID_IApplicationData3, 3693227252, 10098, 19485, 170, 44, 201, 247, 67, 173, 232, 209);
RT_INTERFACE!{interface IApplicationData3(IApplicationData3Vtbl): IInspectable(IInspectableVtbl) [IID_IApplicationData3] {
    fn GetPublisherCacheFolder(&self, folderName: HSTRING, out: *mut *mut StorageFolder) -> HRESULT,
    fn ClearPublisherCacheFolderAsync(&self, folderName: HSTRING, out: *mut *mut super::foundation::IAsyncAction) -> HRESULT,
    fn get_SharedLocalFolder(&self, out: *mut *mut StorageFolder) -> HRESULT
}}
impl IApplicationData3 {
    #[inline] pub unsafe fn get_publisher_cache_folder(&self, folderName: &HStringArg) -> Result<ComPtr<StorageFolder>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetPublisherCacheFolder)(self as *const _ as *mut _, folderName.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn clear_publisher_cache_folder_async(&self, folderName: &HStringArg) -> Result<ComPtr<super::foundation::IAsyncAction>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).ClearPublisherCacheFolderAsync)(self as *const _ as *mut _, folderName.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_shared_local_folder(&self) -> Result<ComPtr<StorageFolder>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_SharedLocalFolder)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
RT_CLASS!{class ApplicationDataCompositeValue: super::foundation::collections::IPropertySet}
impl RtActivatable<IActivationFactory> for ApplicationDataCompositeValue {}
DEFINE_CLSID!(ApplicationDataCompositeValue(&[87,105,110,100,111,119,115,46,83,116,111,114,97,103,101,46,65,112,112,108,105,99,97,116,105,111,110,68,97,116,97,67,111,109,112,111,115,105,116,101,86,97,108,117,101,0]) [CLSID_ApplicationDataCompositeValue]);
DEFINE_IID!(IID_IApplicationDataContainer, 3316579614, 62567, 16570, 133, 102, 171, 100, 10, 68, 30, 29);
RT_INTERFACE!{interface IApplicationDataContainer(IApplicationDataContainerVtbl): IInspectable(IInspectableVtbl) [IID_IApplicationDataContainer] {
    fn get_Name(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Locality(&self, out: *mut ApplicationDataLocality) -> HRESULT,
    fn get_Values(&self, out: *mut *mut super::foundation::collections::IPropertySet) -> HRESULT,
    fn get_Containers(&self, out: *mut *mut super::foundation::collections::IMapView<HString, ApplicationDataContainer>) -> HRESULT,
    fn CreateContainer(&self, name: HSTRING, disposition: ApplicationDataCreateDisposition, out: *mut *mut ApplicationDataContainer) -> HRESULT,
    fn DeleteContainer(&self, name: HSTRING) -> HRESULT
}}
impl IApplicationDataContainer {
    #[inline] pub unsafe fn get_name(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Name)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_locality(&self) -> Result<ApplicationDataLocality> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_Locality)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_values(&self) -> Result<ComPtr<super::foundation::collections::IPropertySet>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Values)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_containers(&self) -> Result<ComPtr<super::foundation::collections::IMapView<HString, ApplicationDataContainer>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Containers)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn create_container(&self, name: &HStringArg, disposition: ApplicationDataCreateDisposition) -> Result<ComPtr<ApplicationDataContainer>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).CreateContainer)(self as *const _ as *mut _, name.get(), disposition, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn delete_container(&self, name: &HStringArg) -> Result<()> {
        let hr = ((*self.lpVtbl).DeleteContainer)(self as *const _ as *mut _, name.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
}
RT_CLASS!{class ApplicationDataContainer: IApplicationDataContainer}
RT_CLASS!{class ApplicationDataContainerSettings: super::foundation::collections::IPropertySet}
RT_ENUM! { enum ApplicationDataCreateDisposition: i32 {
    Always (ApplicationDataCreateDisposition_Always) = 0, Existing (ApplicationDataCreateDisposition_Existing) = 1,
}}
RT_ENUM! { enum ApplicationDataLocality: i32 {
    Local (ApplicationDataLocality_Local) = 0, Roaming (ApplicationDataLocality_Roaming) = 1, Temporary (ApplicationDataLocality_Temporary) = 2, LocalCache (ApplicationDataLocality_LocalCache) = 3,
}}
DEFINE_IID!(IID_ApplicationDataSetVersionHandler, 2690093542, 52383, 18055, 172, 171, 163, 100, 253, 120, 84, 99);
RT_DELEGATE!{delegate ApplicationDataSetVersionHandler(ApplicationDataSetVersionHandlerVtbl, ApplicationDataSetVersionHandlerImpl) [IID_ApplicationDataSetVersionHandler] {
    fn Invoke(&self, setVersionRequest: *mut SetVersionRequest) -> HRESULT
}}
impl ApplicationDataSetVersionHandler {
    #[inline] pub unsafe fn invoke(&self, setVersionRequest: &SetVersionRequest) -> Result<()> {
        let hr = ((*self.lpVtbl).Invoke)(self as *const _ as *mut _, setVersionRequest as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
}
DEFINE_IID!(IID_IApplicationDataStatics, 1444025467, 59459, 17891, 148, 216, 6, 22, 158, 60, 142, 23);
RT_INTERFACE!{static interface IApplicationDataStatics(IApplicationDataStaticsVtbl): IInspectable(IInspectableVtbl) [IID_IApplicationDataStatics] {
    fn get_Current(&self, out: *mut *mut ApplicationData) -> HRESULT
}}
impl IApplicationDataStatics {
    #[inline] pub unsafe fn get_current(&self) -> Result<ComPtr<ApplicationData>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Current)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
DEFINE_IID!(IID_IApplicationDataStatics2, 3445645841, 53065, 16548, 164, 124, 199, 240, 219, 186, 129, 7);
RT_INTERFACE!{static interface IApplicationDataStatics2(IApplicationDataStatics2Vtbl): IInspectable(IInspectableVtbl) [IID_IApplicationDataStatics2] {
    #[cfg(feature="windows-system")] fn GetForUserAsync(&self, user: *mut super::system::User, out: *mut *mut super::foundation::IAsyncOperation<ApplicationData>) -> HRESULT
}}
impl IApplicationDataStatics2 {
    #[cfg(feature="windows-system")] #[inline] pub unsafe fn get_for_user_async(&self, user: &super::system::User) -> Result<ComPtr<super::foundation::IAsyncOperation<ApplicationData>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetForUserAsync)(self as *const _ as *mut _, user as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
RT_CLASS!{static class CachedFileManager}
impl RtActivatable<ICachedFileManagerStatics> for CachedFileManager {}
impl CachedFileManager {
    #[inline] pub fn defer_updates(file: &IStorageFile) -> Result<()> { unsafe {
        <Self as RtActivatable<ICachedFileManagerStatics>>::get_activation_factory().defer_updates(file)
    }}
    #[inline] pub fn complete_updates_async(file: &IStorageFile) -> Result<ComPtr<super::foundation::IAsyncOperation<provider::FileUpdateStatus>>> { unsafe {
        <Self as RtActivatable<ICachedFileManagerStatics>>::get_activation_factory().complete_updates_async(file)
    }}
}
DEFINE_CLSID!(CachedFileManager(&[87,105,110,100,111,119,115,46,83,116,111,114,97,103,101,46,67,97,99,104,101,100,70,105,108,101,77,97,110,97,103,101,114,0]) [CLSID_CachedFileManager]);
DEFINE_IID!(IID_ICachedFileManagerStatics, 2415665738, 59266, 18781, 182, 20, 101, 76, 79, 11, 35, 112);
RT_INTERFACE!{static interface ICachedFileManagerStatics(ICachedFileManagerStaticsVtbl): IInspectable(IInspectableVtbl) [IID_ICachedFileManagerStatics] {
    fn DeferUpdates(&self, file: *mut IStorageFile) -> HRESULT,
    fn CompleteUpdatesAsync(&self, file: *mut IStorageFile, out: *mut *mut super::foundation::IAsyncOperation<provider::FileUpdateStatus>) -> HRESULT
}}
impl ICachedFileManagerStatics {
    #[inline] pub unsafe fn defer_updates(&self, file: &IStorageFile) -> Result<()> {
        let hr = ((*self.lpVtbl).DeferUpdates)(self as *const _ as *mut _, file as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn complete_updates_async(&self, file: &IStorageFile) -> Result<ComPtr<super::foundation::IAsyncOperation<provider::FileUpdateStatus>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).CompleteUpdatesAsync)(self as *const _ as *mut _, file as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
RT_ENUM! { enum CreationCollisionOption: i32 {
    GenerateUniqueName (CreationCollisionOption_GenerateUniqueName) = 0, ReplaceExisting (CreationCollisionOption_ReplaceExisting) = 1, FailIfExists (CreationCollisionOption_FailIfExists) = 2, OpenIfExists (CreationCollisionOption_OpenIfExists) = 3,
}}
RT_CLASS!{static class DownloadsFolder}
impl RtActivatable<IDownloadsFolderStatics> for DownloadsFolder {}
impl RtActivatable<IDownloadsFolderStatics2> for DownloadsFolder {}
impl DownloadsFolder {
    #[inline] pub fn create_file_async(desiredName: &HStringArg) -> Result<ComPtr<super::foundation::IAsyncOperation<StorageFile>>> { unsafe {
        <Self as RtActivatable<IDownloadsFolderStatics>>::get_activation_factory().create_file_async(desiredName)
    }}
    #[inline] pub fn create_folder_async(desiredName: &HStringArg) -> Result<ComPtr<super::foundation::IAsyncOperation<StorageFolder>>> { unsafe {
        <Self as RtActivatable<IDownloadsFolderStatics>>::get_activation_factory().create_folder_async(desiredName)
    }}
    #[inline] pub fn create_file_with_collision_option_async(desiredName: &HStringArg, option: CreationCollisionOption) -> Result<ComPtr<super::foundation::IAsyncOperation<StorageFile>>> { unsafe {
        <Self as RtActivatable<IDownloadsFolderStatics>>::get_activation_factory().create_file_with_collision_option_async(desiredName, option)
    }}
    #[inline] pub fn create_folder_with_collision_option_async(desiredName: &HStringArg, option: CreationCollisionOption) -> Result<ComPtr<super::foundation::IAsyncOperation<StorageFolder>>> { unsafe {
        <Self as RtActivatable<IDownloadsFolderStatics>>::get_activation_factory().create_folder_with_collision_option_async(desiredName, option)
    }}
    #[cfg(feature="windows-system")] #[inline] pub fn create_file_for_user_async(user: &super::system::User, desiredName: &HStringArg) -> Result<ComPtr<super::foundation::IAsyncOperation<StorageFile>>> { unsafe {
        <Self as RtActivatable<IDownloadsFolderStatics2>>::get_activation_factory().create_file_for_user_async(user, desiredName)
    }}
    #[cfg(feature="windows-system")] #[inline] pub fn create_folder_for_user_async(user: &super::system::User, desiredName: &HStringArg) -> Result<ComPtr<super::foundation::IAsyncOperation<StorageFolder>>> { unsafe {
        <Self as RtActivatable<IDownloadsFolderStatics2>>::get_activation_factory().create_folder_for_user_async(user, desiredName)
    }}
    #[cfg(feature="windows-system")] #[inline] pub fn create_file_for_user_with_collision_option_async(user: &super::system::User, desiredName: &HStringArg, option: CreationCollisionOption) -> Result<ComPtr<super::foundation::IAsyncOperation<StorageFile>>> { unsafe {
        <Self as RtActivatable<IDownloadsFolderStatics2>>::get_activation_factory().create_file_for_user_with_collision_option_async(user, desiredName, option)
    }}
    #[cfg(feature="windows-system")] #[inline] pub fn create_folder_for_user_with_collision_option_async(user: &super::system::User, desiredName: &HStringArg, option: CreationCollisionOption) -> Result<ComPtr<super::foundation::IAsyncOperation<StorageFolder>>> { unsafe {
        <Self as RtActivatable<IDownloadsFolderStatics2>>::get_activation_factory().create_folder_for_user_with_collision_option_async(user, desiredName, option)
    }}
}
DEFINE_CLSID!(DownloadsFolder(&[87,105,110,100,111,119,115,46,83,116,111,114,97,103,101,46,68,111,119,110,108,111,97,100,115,70,111,108,100,101,114,0]) [CLSID_DownloadsFolder]);
DEFINE_IID!(IID_IDownloadsFolderStatics, 663105232, 16462, 18399, 161, 226, 227, 115, 8, 190, 123, 55);
RT_INTERFACE!{static interface IDownloadsFolderStatics(IDownloadsFolderStaticsVtbl): IInspectable(IInspectableVtbl) [IID_IDownloadsFolderStatics] {
    fn CreateFileAsync(&self, desiredName: HSTRING, out: *mut *mut super::foundation::IAsyncOperation<StorageFile>) -> HRESULT,
    fn CreateFolderAsync(&self, desiredName: HSTRING, out: *mut *mut super::foundation::IAsyncOperation<StorageFolder>) -> HRESULT,
    fn CreateFileWithCollisionOptionAsync(&self, desiredName: HSTRING, option: CreationCollisionOption, out: *mut *mut super::foundation::IAsyncOperation<StorageFile>) -> HRESULT,
    fn CreateFolderWithCollisionOptionAsync(&self, desiredName: HSTRING, option: CreationCollisionOption, out: *mut *mut super::foundation::IAsyncOperation<StorageFolder>) -> HRESULT
}}
impl IDownloadsFolderStatics {
    #[inline] pub unsafe fn create_file_async(&self, desiredName: &HStringArg) -> Result<ComPtr<super::foundation::IAsyncOperation<StorageFile>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).CreateFileAsync)(self as *const _ as *mut _, desiredName.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn create_folder_async(&self, desiredName: &HStringArg) -> Result<ComPtr<super::foundation::IAsyncOperation<StorageFolder>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).CreateFolderAsync)(self as *const _ as *mut _, desiredName.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn create_file_with_collision_option_async(&self, desiredName: &HStringArg, option: CreationCollisionOption) -> Result<ComPtr<super::foundation::IAsyncOperation<StorageFile>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).CreateFileWithCollisionOptionAsync)(self as *const _ as *mut _, desiredName.get(), option, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn create_folder_with_collision_option_async(&self, desiredName: &HStringArg, option: CreationCollisionOption) -> Result<ComPtr<super::foundation::IAsyncOperation<StorageFolder>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).CreateFolderWithCollisionOptionAsync)(self as *const _ as *mut _, desiredName.get(), option, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
DEFINE_IID!(IID_IDownloadsFolderStatics2, 3912254909, 36600, 20366, 141, 21, 172, 14, 38, 95, 57, 13);
RT_INTERFACE!{static interface IDownloadsFolderStatics2(IDownloadsFolderStatics2Vtbl): IInspectable(IInspectableVtbl) [IID_IDownloadsFolderStatics2] {
    #[cfg(feature="windows-system")] fn CreateFileForUserAsync(&self, user: *mut super::system::User, desiredName: HSTRING, out: *mut *mut super::foundation::IAsyncOperation<StorageFile>) -> HRESULT,
    #[cfg(feature="windows-system")] fn CreateFolderForUserAsync(&self, user: *mut super::system::User, desiredName: HSTRING, out: *mut *mut super::foundation::IAsyncOperation<StorageFolder>) -> HRESULT,
    #[cfg(feature="windows-system")] fn CreateFileForUserWithCollisionOptionAsync(&self, user: *mut super::system::User, desiredName: HSTRING, option: CreationCollisionOption, out: *mut *mut super::foundation::IAsyncOperation<StorageFile>) -> HRESULT,
    #[cfg(feature="windows-system")] fn CreateFolderForUserWithCollisionOptionAsync(&self, user: *mut super::system::User, desiredName: HSTRING, option: CreationCollisionOption, out: *mut *mut super::foundation::IAsyncOperation<StorageFolder>) -> HRESULT
}}
impl IDownloadsFolderStatics2 {
    #[cfg(feature="windows-system")] #[inline] pub unsafe fn create_file_for_user_async(&self, user: &super::system::User, desiredName: &HStringArg) -> Result<ComPtr<super::foundation::IAsyncOperation<StorageFile>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).CreateFileForUserAsync)(self as *const _ as *mut _, user as *const _ as *mut _, desiredName.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[cfg(feature="windows-system")] #[inline] pub unsafe fn create_folder_for_user_async(&self, user: &super::system::User, desiredName: &HStringArg) -> Result<ComPtr<super::foundation::IAsyncOperation<StorageFolder>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).CreateFolderForUserAsync)(self as *const _ as *mut _, user as *const _ as *mut _, desiredName.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[cfg(feature="windows-system")] #[inline] pub unsafe fn create_file_for_user_with_collision_option_async(&self, user: &super::system::User, desiredName: &HStringArg, option: CreationCollisionOption) -> Result<ComPtr<super::foundation::IAsyncOperation<StorageFile>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).CreateFileForUserWithCollisionOptionAsync)(self as *const _ as *mut _, user as *const _ as *mut _, desiredName.get(), option, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[cfg(feature="windows-system")] #[inline] pub unsafe fn create_folder_for_user_with_collision_option_async(&self, user: &super::system::User, desiredName: &HStringArg, option: CreationCollisionOption) -> Result<ComPtr<super::foundation::IAsyncOperation<StorageFolder>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).CreateFolderForUserWithCollisionOptionAsync)(self as *const _ as *mut _, user as *const _ as *mut _, desiredName.get(), option, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
RT_ENUM! { enum FileAccessMode: i32 {
    Read (FileAccessMode_Read) = 0, ReadWrite (FileAccessMode_ReadWrite) = 1,
}}
RT_ENUM! { enum FileAttributes: u32 {
    Normal (FileAttributes_Normal) = 0, ReadOnly (FileAttributes_ReadOnly) = 1, Directory (FileAttributes_Directory) = 16, Archive (FileAttributes_Archive) = 32, Temporary (FileAttributes_Temporary) = 256, LocallyIncomplete (FileAttributes_LocallyIncomplete) = 512,
}}
RT_CLASS!{static class FileIO}
impl RtActivatable<IFileIOStatics> for FileIO {}
impl FileIO {
    #[inline] pub fn read_text_async(file: &IStorageFile) -> Result<ComPtr<super::foundation::IAsyncOperation<HString>>> { unsafe {
        <Self as RtActivatable<IFileIOStatics>>::get_activation_factory().read_text_async(file)
    }}
    #[inline] pub fn read_text_with_encoding_async(file: &IStorageFile, encoding: streams::UnicodeEncoding) -> Result<ComPtr<super::foundation::IAsyncOperation<HString>>> { unsafe {
        <Self as RtActivatable<IFileIOStatics>>::get_activation_factory().read_text_with_encoding_async(file, encoding)
    }}
    #[inline] pub fn write_text_async(file: &IStorageFile, contents: &HStringArg) -> Result<ComPtr<super::foundation::IAsyncAction>> { unsafe {
        <Self as RtActivatable<IFileIOStatics>>::get_activation_factory().write_text_async(file, contents)
    }}
    #[inline] pub fn write_text_with_encoding_async(file: &IStorageFile, contents: &HStringArg, encoding: streams::UnicodeEncoding) -> Result<ComPtr<super::foundation::IAsyncAction>> { unsafe {
        <Self as RtActivatable<IFileIOStatics>>::get_activation_factory().write_text_with_encoding_async(file, contents, encoding)
    }}
    #[inline] pub fn append_text_async(file: &IStorageFile, contents: &HStringArg) -> Result<ComPtr<super::foundation::IAsyncAction>> { unsafe {
        <Self as RtActivatable<IFileIOStatics>>::get_activation_factory().append_text_async(file, contents)
    }}
    #[inline] pub fn append_text_with_encoding_async(file: &IStorageFile, contents: &HStringArg, encoding: streams::UnicodeEncoding) -> Result<ComPtr<super::foundation::IAsyncAction>> { unsafe {
        <Self as RtActivatable<IFileIOStatics>>::get_activation_factory().append_text_with_encoding_async(file, contents, encoding)
    }}
    #[inline] pub fn read_lines_async(file: &IStorageFile) -> Result<ComPtr<super::foundation::IAsyncOperation<super::foundation::collections::IVector<HString>>>> { unsafe {
        <Self as RtActivatable<IFileIOStatics>>::get_activation_factory().read_lines_async(file)
    }}
    #[inline] pub fn read_lines_with_encoding_async(file: &IStorageFile, encoding: streams::UnicodeEncoding) -> Result<ComPtr<super::foundation::IAsyncOperation<super::foundation::collections::IVector<HString>>>> { unsafe {
        <Self as RtActivatable<IFileIOStatics>>::get_activation_factory().read_lines_with_encoding_async(file, encoding)
    }}
    #[inline] pub fn write_lines_async(file: &IStorageFile, lines: &super::foundation::collections::IIterable<HString>) -> Result<ComPtr<super::foundation::IAsyncAction>> { unsafe {
        <Self as RtActivatable<IFileIOStatics>>::get_activation_factory().write_lines_async(file, lines)
    }}
    #[inline] pub fn write_lines_with_encoding_async(file: &IStorageFile, lines: &super::foundation::collections::IIterable<HString>, encoding: streams::UnicodeEncoding) -> Result<ComPtr<super::foundation::IAsyncAction>> { unsafe {
        <Self as RtActivatable<IFileIOStatics>>::get_activation_factory().write_lines_with_encoding_async(file, lines, encoding)
    }}
    #[inline] pub fn append_lines_async(file: &IStorageFile, lines: &super::foundation::collections::IIterable<HString>) -> Result<ComPtr<super::foundation::IAsyncAction>> { unsafe {
        <Self as RtActivatable<IFileIOStatics>>::get_activation_factory().append_lines_async(file, lines)
    }}
    #[inline] pub fn append_lines_with_encoding_async(file: &IStorageFile, lines: &super::foundation::collections::IIterable<HString>, encoding: streams::UnicodeEncoding) -> Result<ComPtr<super::foundation::IAsyncAction>> { unsafe {
        <Self as RtActivatable<IFileIOStatics>>::get_activation_factory().append_lines_with_encoding_async(file, lines, encoding)
    }}
    #[inline] pub fn read_buffer_async(file: &IStorageFile) -> Result<ComPtr<super::foundation::IAsyncOperation<streams::IBuffer>>> { unsafe {
        <Self as RtActivatable<IFileIOStatics>>::get_activation_factory().read_buffer_async(file)
    }}
    #[inline] pub fn write_buffer_async(file: &IStorageFile, buffer: &streams::IBuffer) -> Result<ComPtr<super::foundation::IAsyncAction>> { unsafe {
        <Self as RtActivatable<IFileIOStatics>>::get_activation_factory().write_buffer_async(file, buffer)
    }}
    #[inline] pub fn write_bytes_async(file: &IStorageFile, buffer: &[u8]) -> Result<ComPtr<super::foundation::IAsyncAction>> { unsafe {
        <Self as RtActivatable<IFileIOStatics>>::get_activation_factory().write_bytes_async(file, buffer)
    }}
}
DEFINE_CLSID!(FileIO(&[87,105,110,100,111,119,115,46,83,116,111,114,97,103,101,46,70,105,108,101,73,79,0]) [CLSID_FileIO]);
DEFINE_IID!(IID_IFileIOStatics, 2289308139, 32596, 18226, 165, 240, 94, 67, 227, 184, 194, 245);
RT_INTERFACE!{static interface IFileIOStatics(IFileIOStaticsVtbl): IInspectable(IInspectableVtbl) [IID_IFileIOStatics] {
    fn ReadTextAsync(&self, file: *mut IStorageFile, out: *mut *mut super::foundation::IAsyncOperation<HString>) -> HRESULT,
    fn ReadTextWithEncodingAsync(&self, file: *mut IStorageFile, encoding: streams::UnicodeEncoding, out: *mut *mut super::foundation::IAsyncOperation<HString>) -> HRESULT,
    fn WriteTextAsync(&self, file: *mut IStorageFile, contents: HSTRING, out: *mut *mut super::foundation::IAsyncAction) -> HRESULT,
    fn WriteTextWithEncodingAsync(&self, file: *mut IStorageFile, contents: HSTRING, encoding: streams::UnicodeEncoding, out: *mut *mut super::foundation::IAsyncAction) -> HRESULT,
    fn AppendTextAsync(&self, file: *mut IStorageFile, contents: HSTRING, out: *mut *mut super::foundation::IAsyncAction) -> HRESULT,
    fn AppendTextWithEncodingAsync(&self, file: *mut IStorageFile, contents: HSTRING, encoding: streams::UnicodeEncoding, out: *mut *mut super::foundation::IAsyncAction) -> HRESULT,
    fn ReadLinesAsync(&self, file: *mut IStorageFile, out: *mut *mut super::foundation::IAsyncOperation<super::foundation::collections::IVector<HString>>) -> HRESULT,
    fn ReadLinesWithEncodingAsync(&self, file: *mut IStorageFile, encoding: streams::UnicodeEncoding, out: *mut *mut super::foundation::IAsyncOperation<super::foundation::collections::IVector<HString>>) -> HRESULT,
    fn WriteLinesAsync(&self, file: *mut IStorageFile, lines: *mut super::foundation::collections::IIterable<HString>, out: *mut *mut super::foundation::IAsyncAction) -> HRESULT,
    fn WriteLinesWithEncodingAsync(&self, file: *mut IStorageFile, lines: *mut super::foundation::collections::IIterable<HString>, encoding: streams::UnicodeEncoding, out: *mut *mut super::foundation::IAsyncAction) -> HRESULT,
    fn AppendLinesAsync(&self, file: *mut IStorageFile, lines: *mut super::foundation::collections::IIterable<HString>, out: *mut *mut super::foundation::IAsyncAction) -> HRESULT,
    fn AppendLinesWithEncodingAsync(&self, file: *mut IStorageFile, lines: *mut super::foundation::collections::IIterable<HString>, encoding: streams::UnicodeEncoding, out: *mut *mut super::foundation::IAsyncAction) -> HRESULT,
    fn ReadBufferAsync(&self, file: *mut IStorageFile, out: *mut *mut super::foundation::IAsyncOperation<streams::IBuffer>) -> HRESULT,
    fn WriteBufferAsync(&self, file: *mut IStorageFile, buffer: *mut streams::IBuffer, out: *mut *mut super::foundation::IAsyncAction) -> HRESULT,
    fn WriteBytesAsync(&self, file: *mut IStorageFile, bufferSize: u32, buffer: *mut u8, out: *mut *mut super::foundation::IAsyncAction) -> HRESULT
}}
impl IFileIOStatics {
    #[inline] pub unsafe fn read_text_async(&self, file: &IStorageFile) -> Result<ComPtr<super::foundation::IAsyncOperation<HString>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).ReadTextAsync)(self as *const _ as *mut _, file as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn read_text_with_encoding_async(&self, file: &IStorageFile, encoding: streams::UnicodeEncoding) -> Result<ComPtr<super::foundation::IAsyncOperation<HString>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).ReadTextWithEncodingAsync)(self as *const _ as *mut _, file as *const _ as *mut _, encoding, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn write_text_async(&self, file: &IStorageFile, contents: &HStringArg) -> Result<ComPtr<super::foundation::IAsyncAction>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).WriteTextAsync)(self as *const _ as *mut _, file as *const _ as *mut _, contents.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn write_text_with_encoding_async(&self, file: &IStorageFile, contents: &HStringArg, encoding: streams::UnicodeEncoding) -> Result<ComPtr<super::foundation::IAsyncAction>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).WriteTextWithEncodingAsync)(self as *const _ as *mut _, file as *const _ as *mut _, contents.get(), encoding, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn append_text_async(&self, file: &IStorageFile, contents: &HStringArg) -> Result<ComPtr<super::foundation::IAsyncAction>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).AppendTextAsync)(self as *const _ as *mut _, file as *const _ as *mut _, contents.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn append_text_with_encoding_async(&self, file: &IStorageFile, contents: &HStringArg, encoding: streams::UnicodeEncoding) -> Result<ComPtr<super::foundation::IAsyncAction>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).AppendTextWithEncodingAsync)(self as *const _ as *mut _, file as *const _ as *mut _, contents.get(), encoding, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn read_lines_async(&self, file: &IStorageFile) -> Result<ComPtr<super::foundation::IAsyncOperation<super::foundation::collections::IVector<HString>>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).ReadLinesAsync)(self as *const _ as *mut _, file as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn read_lines_with_encoding_async(&self, file: &IStorageFile, encoding: streams::UnicodeEncoding) -> Result<ComPtr<super::foundation::IAsyncOperation<super::foundation::collections::IVector<HString>>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).ReadLinesWithEncodingAsync)(self as *const _ as *mut _, file as *const _ as *mut _, encoding, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn write_lines_async(&self, file: &IStorageFile, lines: &super::foundation::collections::IIterable<HString>) -> Result<ComPtr<super::foundation::IAsyncAction>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).WriteLinesAsync)(self as *const _ as *mut _, file as *const _ as *mut _, lines as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn write_lines_with_encoding_async(&self, file: &IStorageFile, lines: &super::foundation::collections::IIterable<HString>, encoding: streams::UnicodeEncoding) -> Result<ComPtr<super::foundation::IAsyncAction>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).WriteLinesWithEncodingAsync)(self as *const _ as *mut _, file as *const _ as *mut _, lines as *const _ as *mut _, encoding, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn append_lines_async(&self, file: &IStorageFile, lines: &super::foundation::collections::IIterable<HString>) -> Result<ComPtr<super::foundation::IAsyncAction>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).AppendLinesAsync)(self as *const _ as *mut _, file as *const _ as *mut _, lines as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn append_lines_with_encoding_async(&self, file: &IStorageFile, lines: &super::foundation::collections::IIterable<HString>, encoding: streams::UnicodeEncoding) -> Result<ComPtr<super::foundation::IAsyncAction>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).AppendLinesWithEncodingAsync)(self as *const _ as *mut _, file as *const _ as *mut _, lines as *const _ as *mut _, encoding, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn read_buffer_async(&self, file: &IStorageFile) -> Result<ComPtr<super::foundation::IAsyncOperation<streams::IBuffer>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).ReadBufferAsync)(self as *const _ as *mut _, file as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn write_buffer_async(&self, file: &IStorageFile, buffer: &streams::IBuffer) -> Result<ComPtr<super::foundation::IAsyncAction>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).WriteBufferAsync)(self as *const _ as *mut _, file as *const _ as *mut _, buffer as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn write_bytes_async(&self, file: &IStorageFile, buffer: &[u8]) -> Result<ComPtr<super::foundation::IAsyncAction>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).WriteBytesAsync)(self as *const _ as *mut _, file as *const _ as *mut _, buffer.len() as u32, buffer.as_ptr() as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
RT_ENUM! { enum KnownFolderId: i32 {
    AppCaptures (KnownFolderId_AppCaptures) = 0, CameraRoll (KnownFolderId_CameraRoll) = 1, DocumentsLibrary (KnownFolderId_DocumentsLibrary) = 2, HomeGroup (KnownFolderId_HomeGroup) = 3, MediaServerDevices (KnownFolderId_MediaServerDevices) = 4, MusicLibrary (KnownFolderId_MusicLibrary) = 5, Objects3D (KnownFolderId_Objects3D) = 6, PicturesLibrary (KnownFolderId_PicturesLibrary) = 7, Playlists (KnownFolderId_Playlists) = 8, RecordedCalls (KnownFolderId_RecordedCalls) = 9, RemovableDevices (KnownFolderId_RemovableDevices) = 10, SavedPictures (KnownFolderId_SavedPictures) = 11, Screenshots (KnownFolderId_Screenshots) = 12, VideosLibrary (KnownFolderId_VideosLibrary) = 13, AllAppMods (KnownFolderId_AllAppMods) = 14, CurrentAppMods (KnownFolderId_CurrentAppMods) = 15,
}}
RT_CLASS!{static class KnownFolders}
impl RtActivatable<IKnownFoldersCameraRollStatics> for KnownFolders {}
impl RtActivatable<IKnownFoldersPlaylistsStatics> for KnownFolders {}
impl RtActivatable<IKnownFoldersSavedPicturesStatics> for KnownFolders {}
impl RtActivatable<IKnownFoldersStatics> for KnownFolders {}
impl RtActivatable<IKnownFoldersStatics2> for KnownFolders {}
impl RtActivatable<IKnownFoldersStatics3> for KnownFolders {}
impl KnownFolders {
    #[inline] pub fn get_camera_roll() -> Result<ComPtr<StorageFolder>> { unsafe {
        <Self as RtActivatable<IKnownFoldersCameraRollStatics>>::get_activation_factory().get_camera_roll()
    }}
    #[inline] pub fn get_playlists() -> Result<ComPtr<StorageFolder>> { unsafe {
        <Self as RtActivatable<IKnownFoldersPlaylistsStatics>>::get_activation_factory().get_playlists()
    }}
    #[inline] pub fn get_saved_pictures() -> Result<ComPtr<StorageFolder>> { unsafe {
        <Self as RtActivatable<IKnownFoldersSavedPicturesStatics>>::get_activation_factory().get_saved_pictures()
    }}
    #[inline] pub fn get_music_library() -> Result<ComPtr<StorageFolder>> { unsafe {
        <Self as RtActivatable<IKnownFoldersStatics>>::get_activation_factory().get_music_library()
    }}
    #[inline] pub fn get_pictures_library() -> Result<ComPtr<StorageFolder>> { unsafe {
        <Self as RtActivatable<IKnownFoldersStatics>>::get_activation_factory().get_pictures_library()
    }}
    #[inline] pub fn get_videos_library() -> Result<ComPtr<StorageFolder>> { unsafe {
        <Self as RtActivatable<IKnownFoldersStatics>>::get_activation_factory().get_videos_library()
    }}
    #[inline] pub fn get_documents_library() -> Result<ComPtr<StorageFolder>> { unsafe {
        <Self as RtActivatable<IKnownFoldersStatics>>::get_activation_factory().get_documents_library()
    }}
    #[inline] pub fn get_home_group() -> Result<ComPtr<StorageFolder>> { unsafe {
        <Self as RtActivatable<IKnownFoldersStatics>>::get_activation_factory().get_home_group()
    }}
    #[inline] pub fn get_removable_devices() -> Result<ComPtr<StorageFolder>> { unsafe {
        <Self as RtActivatable<IKnownFoldersStatics>>::get_activation_factory().get_removable_devices()
    }}
    #[inline] pub fn get_media_server_devices() -> Result<ComPtr<StorageFolder>> { unsafe {
        <Self as RtActivatable<IKnownFoldersStatics>>::get_activation_factory().get_media_server_devices()
    }}
    #[inline] pub fn get_objects3_d() -> Result<ComPtr<StorageFolder>> { unsafe {
        <Self as RtActivatable<IKnownFoldersStatics2>>::get_activation_factory().get_objects3_d()
    }}
    #[inline] pub fn get_app_captures() -> Result<ComPtr<StorageFolder>> { unsafe {
        <Self as RtActivatable<IKnownFoldersStatics2>>::get_activation_factory().get_app_captures()
    }}
    #[inline] pub fn get_recorded_calls() -> Result<ComPtr<StorageFolder>> { unsafe {
        <Self as RtActivatable<IKnownFoldersStatics2>>::get_activation_factory().get_recorded_calls()
    }}
    #[cfg(feature="windows-system")] #[inline] pub fn get_folder_for_user_async(user: &super::system::User, folderId: KnownFolderId) -> Result<ComPtr<super::foundation::IAsyncOperation<StorageFolder>>> { unsafe {
        <Self as RtActivatable<IKnownFoldersStatics3>>::get_activation_factory().get_folder_for_user_async(user, folderId)
    }}
}
DEFINE_CLSID!(KnownFolders(&[87,105,110,100,111,119,115,46,83,116,111,114,97,103,101,46,75,110,111,119,110,70,111,108,100,101,114,115,0]) [CLSID_KnownFolders]);
DEFINE_IID!(IID_IKnownFoldersCameraRollStatics, 1561419366, 10216, 18735, 184, 229, 47, 144, 137, 108, 212, 205);
RT_INTERFACE!{static interface IKnownFoldersCameraRollStatics(IKnownFoldersCameraRollStaticsVtbl): IInspectable(IInspectableVtbl) [IID_IKnownFoldersCameraRollStatics] {
    fn get_CameraRoll(&self, out: *mut *mut StorageFolder) -> HRESULT
}}
impl IKnownFoldersCameraRollStatics {
    #[inline] pub unsafe fn get_camera_roll(&self) -> Result<ComPtr<StorageFolder>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_CameraRoll)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
DEFINE_IID!(IID_IKnownFoldersPlaylistsStatics, 3671452886, 12399, 19818, 180, 150, 70, 186, 142, 177, 6, 206);
RT_INTERFACE!{static interface IKnownFoldersPlaylistsStatics(IKnownFoldersPlaylistsStaticsVtbl): IInspectable(IInspectableVtbl) [IID_IKnownFoldersPlaylistsStatics] {
    fn get_Playlists(&self, out: *mut *mut StorageFolder) -> HRESULT
}}
impl IKnownFoldersPlaylistsStatics {
    #[inline] pub unsafe fn get_playlists(&self) -> Result<ComPtr<StorageFolder>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Playlists)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
DEFINE_IID!(IID_IKnownFoldersSavedPicturesStatics, 89953258, 9533, 18044, 182, 202, 169, 125, 161, 233, 161, 141);
RT_INTERFACE!{static interface IKnownFoldersSavedPicturesStatics(IKnownFoldersSavedPicturesStaticsVtbl): IInspectable(IInspectableVtbl) [IID_IKnownFoldersSavedPicturesStatics] {
    fn get_SavedPictures(&self, out: *mut *mut StorageFolder) -> HRESULT
}}
impl IKnownFoldersSavedPicturesStatics {
    #[inline] pub unsafe fn get_saved_pictures(&self) -> Result<ComPtr<StorageFolder>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_SavedPictures)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
DEFINE_IID!(IID_IKnownFoldersStatics, 1512731936, 18434, 17709, 154, 217, 67, 81, 173, 167, 236, 53);
RT_INTERFACE!{static interface IKnownFoldersStatics(IKnownFoldersStaticsVtbl): IInspectable(IInspectableVtbl) [IID_IKnownFoldersStatics] {
    fn get_MusicLibrary(&self, out: *mut *mut StorageFolder) -> HRESULT,
    fn get_PicturesLibrary(&self, out: *mut *mut StorageFolder) -> HRESULT,
    fn get_VideosLibrary(&self, out: *mut *mut StorageFolder) -> HRESULT,
    fn get_DocumentsLibrary(&self, out: *mut *mut StorageFolder) -> HRESULT,
    fn get_HomeGroup(&self, out: *mut *mut StorageFolder) -> HRESULT,
    fn get_RemovableDevices(&self, out: *mut *mut StorageFolder) -> HRESULT,
    fn get_MediaServerDevices(&self, out: *mut *mut StorageFolder) -> HRESULT
}}
impl IKnownFoldersStatics {
    #[inline] pub unsafe fn get_music_library(&self) -> Result<ComPtr<StorageFolder>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_MusicLibrary)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_pictures_library(&self) -> Result<ComPtr<StorageFolder>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_PicturesLibrary)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_videos_library(&self) -> Result<ComPtr<StorageFolder>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_VideosLibrary)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_documents_library(&self) -> Result<ComPtr<StorageFolder>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_DocumentsLibrary)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_home_group(&self) -> Result<ComPtr<StorageFolder>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_HomeGroup)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_removable_devices(&self) -> Result<ComPtr<StorageFolder>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_RemovableDevices)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_media_server_devices(&self) -> Result<ComPtr<StorageFolder>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_MediaServerDevices)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
DEFINE_IID!(IID_IKnownFoldersStatics2, 424399053, 53102, 19719, 157, 83, 233, 22, 58, 37, 54, 233);
RT_INTERFACE!{static interface IKnownFoldersStatics2(IKnownFoldersStatics2Vtbl): IInspectable(IInspectableVtbl) [IID_IKnownFoldersStatics2] {
    fn get_Objects3D(&self, out: *mut *mut StorageFolder) -> HRESULT,
    fn get_AppCaptures(&self, out: *mut *mut StorageFolder) -> HRESULT,
    fn get_RecordedCalls(&self, out: *mut *mut StorageFolder) -> HRESULT
}}
impl IKnownFoldersStatics2 {
    #[inline] pub unsafe fn get_objects3_d(&self) -> Result<ComPtr<StorageFolder>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Objects3D)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_app_captures(&self) -> Result<ComPtr<StorageFolder>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_AppCaptures)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_recorded_calls(&self) -> Result<ComPtr<StorageFolder>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_RecordedCalls)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
DEFINE_IID!(IID_IKnownFoldersStatics3, 3306767169, 38722, 20181, 130, 61, 252, 20, 1, 20, 135, 100);
RT_INTERFACE!{static interface IKnownFoldersStatics3(IKnownFoldersStatics3Vtbl): IInspectable(IInspectableVtbl) [IID_IKnownFoldersStatics3] {
    #[cfg(feature="windows-system")] fn GetFolderForUserAsync(&self, user: *mut super::system::User, folderId: KnownFolderId, out: *mut *mut super::foundation::IAsyncOperation<StorageFolder>) -> HRESULT
}}
impl IKnownFoldersStatics3 {
    #[cfg(feature="windows-system")] #[inline] pub unsafe fn get_folder_for_user_async(&self, user: &super::system::User, folderId: KnownFolderId) -> Result<ComPtr<super::foundation::IAsyncOperation<StorageFolder>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetFolderForUserAsync)(self as *const _ as *mut _, user as *const _ as *mut _, folderId, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
RT_ENUM! { enum KnownLibraryId: i32 {
    Music (KnownLibraryId_Music) = 0, Pictures (KnownLibraryId_Pictures) = 1, Videos (KnownLibraryId_Videos) = 2, Documents (KnownLibraryId_Documents) = 3,
}}
RT_ENUM! { enum NameCollisionOption: i32 {
    GenerateUniqueName (NameCollisionOption_GenerateUniqueName) = 0, ReplaceExisting (NameCollisionOption_ReplaceExisting) = 1, FailIfExists (NameCollisionOption_FailIfExists) = 2,
}}
RT_CLASS!{static class PathIO}
impl RtActivatable<IPathIOStatics> for PathIO {}
impl PathIO {
    #[inline] pub fn read_text_async(absolutePath: &HStringArg) -> Result<ComPtr<super::foundation::IAsyncOperation<HString>>> { unsafe {
        <Self as RtActivatable<IPathIOStatics>>::get_activation_factory().read_text_async(absolutePath)
    }}
    #[inline] pub fn read_text_with_encoding_async(absolutePath: &HStringArg, encoding: streams::UnicodeEncoding) -> Result<ComPtr<super::foundation::IAsyncOperation<HString>>> { unsafe {
        <Self as RtActivatable<IPathIOStatics>>::get_activation_factory().read_text_with_encoding_async(absolutePath, encoding)
    }}
    #[inline] pub fn write_text_async(absolutePath: &HStringArg, contents: &HStringArg) -> Result<ComPtr<super::foundation::IAsyncAction>> { unsafe {
        <Self as RtActivatable<IPathIOStatics>>::get_activation_factory().write_text_async(absolutePath, contents)
    }}
    #[inline] pub fn write_text_with_encoding_async(absolutePath: &HStringArg, contents: &HStringArg, encoding: streams::UnicodeEncoding) -> Result<ComPtr<super::foundation::IAsyncAction>> { unsafe {
        <Self as RtActivatable<IPathIOStatics>>::get_activation_factory().write_text_with_encoding_async(absolutePath, contents, encoding)
    }}
    #[inline] pub fn append_text_async(absolutePath: &HStringArg, contents: &HStringArg) -> Result<ComPtr<super::foundation::IAsyncAction>> { unsafe {
        <Self as RtActivatable<IPathIOStatics>>::get_activation_factory().append_text_async(absolutePath, contents)
    }}
    #[inline] pub fn append_text_with_encoding_async(absolutePath: &HStringArg, contents: &HStringArg, encoding: streams::UnicodeEncoding) -> Result<ComPtr<super::foundation::IAsyncAction>> { unsafe {
        <Self as RtActivatable<IPathIOStatics>>::get_activation_factory().append_text_with_encoding_async(absolutePath, contents, encoding)
    }}
    #[inline] pub fn read_lines_async(absolutePath: &HStringArg) -> Result<ComPtr<super::foundation::IAsyncOperation<super::foundation::collections::IVector<HString>>>> { unsafe {
        <Self as RtActivatable<IPathIOStatics>>::get_activation_factory().read_lines_async(absolutePath)
    }}
    #[inline] pub fn read_lines_with_encoding_async(absolutePath: &HStringArg, encoding: streams::UnicodeEncoding) -> Result<ComPtr<super::foundation::IAsyncOperation<super::foundation::collections::IVector<HString>>>> { unsafe {
        <Self as RtActivatable<IPathIOStatics>>::get_activation_factory().read_lines_with_encoding_async(absolutePath, encoding)
    }}
    #[inline] pub fn write_lines_async(absolutePath: &HStringArg, lines: &super::foundation::collections::IIterable<HString>) -> Result<ComPtr<super::foundation::IAsyncAction>> { unsafe {
        <Self as RtActivatable<IPathIOStatics>>::get_activation_factory().write_lines_async(absolutePath, lines)
    }}
    #[inline] pub fn write_lines_with_encoding_async(absolutePath: &HStringArg, lines: &super::foundation::collections::IIterable<HString>, encoding: streams::UnicodeEncoding) -> Result<ComPtr<super::foundation::IAsyncAction>> { unsafe {
        <Self as RtActivatable<IPathIOStatics>>::get_activation_factory().write_lines_with_encoding_async(absolutePath, lines, encoding)
    }}
    #[inline] pub fn append_lines_async(absolutePath: &HStringArg, lines: &super::foundation::collections::IIterable<HString>) -> Result<ComPtr<super::foundation::IAsyncAction>> { unsafe {
        <Self as RtActivatable<IPathIOStatics>>::get_activation_factory().append_lines_async(absolutePath, lines)
    }}
    #[inline] pub fn append_lines_with_encoding_async(absolutePath: &HStringArg, lines: &super::foundation::collections::IIterable<HString>, encoding: streams::UnicodeEncoding) -> Result<ComPtr<super::foundation::IAsyncAction>> { unsafe {
        <Self as RtActivatable<IPathIOStatics>>::get_activation_factory().append_lines_with_encoding_async(absolutePath, lines, encoding)
    }}
    #[inline] pub fn read_buffer_async(absolutePath: &HStringArg) -> Result<ComPtr<super::foundation::IAsyncOperation<streams::IBuffer>>> { unsafe {
        <Self as RtActivatable<IPathIOStatics>>::get_activation_factory().read_buffer_async(absolutePath)
    }}
    #[inline] pub fn write_buffer_async(absolutePath: &HStringArg, buffer: &streams::IBuffer) -> Result<ComPtr<super::foundation::IAsyncAction>> { unsafe {
        <Self as RtActivatable<IPathIOStatics>>::get_activation_factory().write_buffer_async(absolutePath, buffer)
    }}
    #[inline] pub fn write_bytes_async(absolutePath: &HStringArg, buffer: &[u8]) -> Result<ComPtr<super::foundation::IAsyncAction>> { unsafe {
        <Self as RtActivatable<IPathIOStatics>>::get_activation_factory().write_bytes_async(absolutePath, buffer)
    }}
}
DEFINE_CLSID!(PathIO(&[87,105,110,100,111,119,115,46,83,116,111,114,97,103,101,46,80,97,116,104,73,79,0]) [CLSID_PathIO]);
DEFINE_IID!(IID_IPathIOStatics, 254752600, 36551, 17281, 146, 43, 143, 108, 7, 210, 136, 243);
RT_INTERFACE!{static interface IPathIOStatics(IPathIOStaticsVtbl): IInspectable(IInspectableVtbl) [IID_IPathIOStatics] {
    fn ReadTextAsync(&self, absolutePath: HSTRING, out: *mut *mut super::foundation::IAsyncOperation<HString>) -> HRESULT,
    fn ReadTextWithEncodingAsync(&self, absolutePath: HSTRING, encoding: streams::UnicodeEncoding, out: *mut *mut super::foundation::IAsyncOperation<HString>) -> HRESULT,
    fn WriteTextAsync(&self, absolutePath: HSTRING, contents: HSTRING, out: *mut *mut super::foundation::IAsyncAction) -> HRESULT,
    fn WriteTextWithEncodingAsync(&self, absolutePath: HSTRING, contents: HSTRING, encoding: streams::UnicodeEncoding, out: *mut *mut super::foundation::IAsyncAction) -> HRESULT,
    fn AppendTextAsync(&self, absolutePath: HSTRING, contents: HSTRING, out: *mut *mut super::foundation::IAsyncAction) -> HRESULT,
    fn AppendTextWithEncodingAsync(&self, absolutePath: HSTRING, contents: HSTRING, encoding: streams::UnicodeEncoding, out: *mut *mut super::foundation::IAsyncAction) -> HRESULT,
    fn ReadLinesAsync(&self, absolutePath: HSTRING, out: *mut *mut super::foundation::IAsyncOperation<super::foundation::collections::IVector<HString>>) -> HRESULT,
    fn ReadLinesWithEncodingAsync(&self, absolutePath: HSTRING, encoding: streams::UnicodeEncoding, out: *mut *mut super::foundation::IAsyncOperation<super::foundation::collections::IVector<HString>>) -> HRESULT,
    fn WriteLinesAsync(&self, absolutePath: HSTRING, lines: *mut super::foundation::collections::IIterable<HString>, out: *mut *mut super::foundation::IAsyncAction) -> HRESULT,
    fn WriteLinesWithEncodingAsync(&self, absolutePath: HSTRING, lines: *mut super::foundation::collections::IIterable<HString>, encoding: streams::UnicodeEncoding, out: *mut *mut super::foundation::IAsyncAction) -> HRESULT,
    fn AppendLinesAsync(&self, absolutePath: HSTRING, lines: *mut super::foundation::collections::IIterable<HString>, out: *mut *mut super::foundation::IAsyncAction) -> HRESULT,
    fn AppendLinesWithEncodingAsync(&self, absolutePath: HSTRING, lines: *mut super::foundation::collections::IIterable<HString>, encoding: streams::UnicodeEncoding, out: *mut *mut super::foundation::IAsyncAction) -> HRESULT,
    fn ReadBufferAsync(&self, absolutePath: HSTRING, out: *mut *mut super::foundation::IAsyncOperation<streams::IBuffer>) -> HRESULT,
    fn WriteBufferAsync(&self, absolutePath: HSTRING, buffer: *mut streams::IBuffer, out: *mut *mut super::foundation::IAsyncAction) -> HRESULT,
    fn WriteBytesAsync(&self, absolutePath: HSTRING, bufferSize: u32, buffer: *mut u8, out: *mut *mut super::foundation::IAsyncAction) -> HRESULT
}}
impl IPathIOStatics {
    #[inline] pub unsafe fn read_text_async(&self, absolutePath: &HStringArg) -> Result<ComPtr<super::foundation::IAsyncOperation<HString>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).ReadTextAsync)(self as *const _ as *mut _, absolutePath.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn read_text_with_encoding_async(&self, absolutePath: &HStringArg, encoding: streams::UnicodeEncoding) -> Result<ComPtr<super::foundation::IAsyncOperation<HString>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).ReadTextWithEncodingAsync)(self as *const _ as *mut _, absolutePath.get(), encoding, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn write_text_async(&self, absolutePath: &HStringArg, contents: &HStringArg) -> Result<ComPtr<super::foundation::IAsyncAction>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).WriteTextAsync)(self as *const _ as *mut _, absolutePath.get(), contents.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn write_text_with_encoding_async(&self, absolutePath: &HStringArg, contents: &HStringArg, encoding: streams::UnicodeEncoding) -> Result<ComPtr<super::foundation::IAsyncAction>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).WriteTextWithEncodingAsync)(self as *const _ as *mut _, absolutePath.get(), contents.get(), encoding, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn append_text_async(&self, absolutePath: &HStringArg, contents: &HStringArg) -> Result<ComPtr<super::foundation::IAsyncAction>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).AppendTextAsync)(self as *const _ as *mut _, absolutePath.get(), contents.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn append_text_with_encoding_async(&self, absolutePath: &HStringArg, contents: &HStringArg, encoding: streams::UnicodeEncoding) -> Result<ComPtr<super::foundation::IAsyncAction>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).AppendTextWithEncodingAsync)(self as *const _ as *mut _, absolutePath.get(), contents.get(), encoding, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn read_lines_async(&self, absolutePath: &HStringArg) -> Result<ComPtr<super::foundation::IAsyncOperation<super::foundation::collections::IVector<HString>>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).ReadLinesAsync)(self as *const _ as *mut _, absolutePath.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn read_lines_with_encoding_async(&self, absolutePath: &HStringArg, encoding: streams::UnicodeEncoding) -> Result<ComPtr<super::foundation::IAsyncOperation<super::foundation::collections::IVector<HString>>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).ReadLinesWithEncodingAsync)(self as *const _ as *mut _, absolutePath.get(), encoding, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn write_lines_async(&self, absolutePath: &HStringArg, lines: &super::foundation::collections::IIterable<HString>) -> Result<ComPtr<super::foundation::IAsyncAction>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).WriteLinesAsync)(self as *const _ as *mut _, absolutePath.get(), lines as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn write_lines_with_encoding_async(&self, absolutePath: &HStringArg, lines: &super::foundation::collections::IIterable<HString>, encoding: streams::UnicodeEncoding) -> Result<ComPtr<super::foundation::IAsyncAction>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).WriteLinesWithEncodingAsync)(self as *const _ as *mut _, absolutePath.get(), lines as *const _ as *mut _, encoding, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn append_lines_async(&self, absolutePath: &HStringArg, lines: &super::foundation::collections::IIterable<HString>) -> Result<ComPtr<super::foundation::IAsyncAction>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).AppendLinesAsync)(self as *const _ as *mut _, absolutePath.get(), lines as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn append_lines_with_encoding_async(&self, absolutePath: &HStringArg, lines: &super::foundation::collections::IIterable<HString>, encoding: streams::UnicodeEncoding) -> Result<ComPtr<super::foundation::IAsyncAction>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).AppendLinesWithEncodingAsync)(self as *const _ as *mut _, absolutePath.get(), lines as *const _ as *mut _, encoding, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn read_buffer_async(&self, absolutePath: &HStringArg) -> Result<ComPtr<super::foundation::IAsyncOperation<streams::IBuffer>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).ReadBufferAsync)(self as *const _ as *mut _, absolutePath.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn write_buffer_async(&self, absolutePath: &HStringArg, buffer: &streams::IBuffer) -> Result<ComPtr<super::foundation::IAsyncAction>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).WriteBufferAsync)(self as *const _ as *mut _, absolutePath.get(), buffer as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn write_bytes_async(&self, absolutePath: &HStringArg, buffer: &[u8]) -> Result<ComPtr<super::foundation::IAsyncAction>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).WriteBytesAsync)(self as *const _ as *mut _, absolutePath.get(), buffer.len() as u32, buffer.as_ptr() as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
DEFINE_IID!(IID_ISetVersionDeferral, 53807266, 30746, 17274, 176, 120, 63, 50, 186, 220, 254, 71);
RT_INTERFACE!{interface ISetVersionDeferral(ISetVersionDeferralVtbl): IInspectable(IInspectableVtbl) [IID_ISetVersionDeferral] {
    fn Complete(&self) -> HRESULT
}}
impl ISetVersionDeferral {
    #[inline] pub unsafe fn complete(&self) -> Result<()> {
        let hr = ((*self.lpVtbl).Complete)(self as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
}
RT_CLASS!{class SetVersionDeferral: ISetVersionDeferral}
DEFINE_IID!(IID_ISetVersionRequest, 3116854171, 4182, 20073, 131, 48, 22, 38, 25, 149, 111, 155);
RT_INTERFACE!{interface ISetVersionRequest(ISetVersionRequestVtbl): IInspectable(IInspectableVtbl) [IID_ISetVersionRequest] {
    fn get_CurrentVersion(&self, out: *mut u32) -> HRESULT,
    fn get_DesiredVersion(&self, out: *mut u32) -> HRESULT,
    fn GetDeferral(&self, out: *mut *mut SetVersionDeferral) -> HRESULT
}}
impl ISetVersionRequest {
    #[inline] pub unsafe fn get_current_version(&self) -> Result<u32> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_CurrentVersion)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_desired_version(&self) -> Result<u32> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_DesiredVersion)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_deferral(&self) -> Result<ComPtr<SetVersionDeferral>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetDeferral)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
RT_CLASS!{class SetVersionRequest: ISetVersionRequest}
RT_ENUM! { enum StorageDeleteOption: i32 {
    Default (StorageDeleteOption_Default) = 0, PermanentDelete (StorageDeleteOption_PermanentDelete) = 1,
}}
DEFINE_IID!(IID_IStorageFile, 4198457734, 16916, 17036, 166, 76, 20, 201, 172, 115, 21, 234);
RT_INTERFACE!{interface IStorageFile(IStorageFileVtbl): IInspectable(IInspectableVtbl) [IID_IStorageFile] {
    fn get_FileType(&self, out: *mut HSTRING) -> HRESULT,
    fn get_ContentType(&self, out: *mut HSTRING) -> HRESULT,
    fn OpenAsync(&self, accessMode: FileAccessMode, out: *mut *mut super::foundation::IAsyncOperation<streams::IRandomAccessStream>) -> HRESULT,
    fn OpenTransactedWriteAsync(&self, out: *mut *mut super::foundation::IAsyncOperation<StorageStreamTransaction>) -> HRESULT,
    fn CopyOverloadDefaultNameAndOptions(&self, destinationFolder: *mut IStorageFolder, out: *mut *mut super::foundation::IAsyncOperation<StorageFile>) -> HRESULT,
    fn CopyOverloadDefaultOptions(&self, destinationFolder: *mut IStorageFolder, desiredNewName: HSTRING, out: *mut *mut super::foundation::IAsyncOperation<StorageFile>) -> HRESULT,
    fn CopyOverload(&self, destinationFolder: *mut IStorageFolder, desiredNewName: HSTRING, option: NameCollisionOption, out: *mut *mut super::foundation::IAsyncOperation<StorageFile>) -> HRESULT,
    fn CopyAndReplaceAsync(&self, fileToReplace: *mut IStorageFile, out: *mut *mut super::foundation::IAsyncAction) -> HRESULT,
    fn MoveOverloadDefaultNameAndOptions(&self, destinationFolder: *mut IStorageFolder, out: *mut *mut super::foundation::IAsyncAction) -> HRESULT,
    fn MoveOverloadDefaultOptions(&self, destinationFolder: *mut IStorageFolder, desiredNewName: HSTRING, out: *mut *mut super::foundation::IAsyncAction) -> HRESULT,
    fn MoveOverload(&self, destinationFolder: *mut IStorageFolder, desiredNewName: HSTRING, option: NameCollisionOption, out: *mut *mut super::foundation::IAsyncAction) -> HRESULT,
    fn MoveAndReplaceAsync(&self, fileToReplace: *mut IStorageFile, out: *mut *mut super::foundation::IAsyncAction) -> HRESULT
}}
impl IStorageFile {
    #[inline] pub unsafe fn get_file_type(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_FileType)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_content_type(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_ContentType)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn open_async(&self, accessMode: FileAccessMode) -> Result<ComPtr<super::foundation::IAsyncOperation<streams::IRandomAccessStream>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).OpenAsync)(self as *const _ as *mut _, accessMode, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn open_transacted_write_async(&self) -> Result<ComPtr<super::foundation::IAsyncOperation<StorageStreamTransaction>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).OpenTransactedWriteAsync)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn copy_overload_default_name_and_options(&self, destinationFolder: &IStorageFolder) -> Result<ComPtr<super::foundation::IAsyncOperation<StorageFile>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).CopyOverloadDefaultNameAndOptions)(self as *const _ as *mut _, destinationFolder as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn copy_overload_default_options(&self, destinationFolder: &IStorageFolder, desiredNewName: &HStringArg) -> Result<ComPtr<super::foundation::IAsyncOperation<StorageFile>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).CopyOverloadDefaultOptions)(self as *const _ as *mut _, destinationFolder as *const _ as *mut _, desiredNewName.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn copy_overload(&self, destinationFolder: &IStorageFolder, desiredNewName: &HStringArg, option: NameCollisionOption) -> Result<ComPtr<super::foundation::IAsyncOperation<StorageFile>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).CopyOverload)(self as *const _ as *mut _, destinationFolder as *const _ as *mut _, desiredNewName.get(), option, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn copy_and_replace_async(&self, fileToReplace: &IStorageFile) -> Result<ComPtr<super::foundation::IAsyncAction>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).CopyAndReplaceAsync)(self as *const _ as *mut _, fileToReplace as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn move_overload_default_name_and_options(&self, destinationFolder: &IStorageFolder) -> Result<ComPtr<super::foundation::IAsyncAction>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).MoveOverloadDefaultNameAndOptions)(self as *const _ as *mut _, destinationFolder as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn move_overload_default_options(&self, destinationFolder: &IStorageFolder, desiredNewName: &HStringArg) -> Result<ComPtr<super::foundation::IAsyncAction>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).MoveOverloadDefaultOptions)(self as *const _ as *mut _, destinationFolder as *const _ as *mut _, desiredNewName.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn move_overload(&self, destinationFolder: &IStorageFolder, desiredNewName: &HStringArg, option: NameCollisionOption) -> Result<ComPtr<super::foundation::IAsyncAction>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).MoveOverload)(self as *const _ as *mut _, destinationFolder as *const _ as *mut _, desiredNewName.get(), option, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn move_and_replace_async(&self, fileToReplace: &IStorageFile) -> Result<ComPtr<super::foundation::IAsyncAction>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).MoveAndReplaceAsync)(self as *const _ as *mut _, fileToReplace as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
RT_CLASS!{class StorageFile: IStorageFile}
impl RtActivatable<IStorageFileStatics> for StorageFile {}
impl StorageFile {
    #[inline] pub fn get_file_from_path_async(path: &HStringArg) -> Result<ComPtr<super::foundation::IAsyncOperation<StorageFile>>> { unsafe {
        <Self as RtActivatable<IStorageFileStatics>>::get_activation_factory().get_file_from_path_async(path)
    }}
    #[inline] pub fn get_file_from_application_uri_async(uri: &super::foundation::Uri) -> Result<ComPtr<super::foundation::IAsyncOperation<StorageFile>>> { unsafe {
        <Self as RtActivatable<IStorageFileStatics>>::get_activation_factory().get_file_from_application_uri_async(uri)
    }}
    #[inline] pub fn create_streamed_file_async(displayNameWithExtension: &HStringArg, dataRequested: &StreamedFileDataRequestedHandler, thumbnail: &streams::IRandomAccessStreamReference) -> Result<ComPtr<super::foundation::IAsyncOperation<StorageFile>>> { unsafe {
        <Self as RtActivatable<IStorageFileStatics>>::get_activation_factory().create_streamed_file_async(displayNameWithExtension, dataRequested, thumbnail)
    }}
    #[inline] pub fn replace_with_streamed_file_async(fileToReplace: &IStorageFile, dataRequested: &StreamedFileDataRequestedHandler, thumbnail: &streams::IRandomAccessStreamReference) -> Result<ComPtr<super::foundation::IAsyncOperation<StorageFile>>> { unsafe {
        <Self as RtActivatable<IStorageFileStatics>>::get_activation_factory().replace_with_streamed_file_async(fileToReplace, dataRequested, thumbnail)
    }}
    #[inline] pub fn create_streamed_file_from_uri_async(displayNameWithExtension: &HStringArg, uri: &super::foundation::Uri, thumbnail: &streams::IRandomAccessStreamReference) -> Result<ComPtr<super::foundation::IAsyncOperation<StorageFile>>> { unsafe {
        <Self as RtActivatable<IStorageFileStatics>>::get_activation_factory().create_streamed_file_from_uri_async(displayNameWithExtension, uri, thumbnail)
    }}
    #[inline] pub fn replace_with_streamed_file_from_uri_async(fileToReplace: &IStorageFile, uri: &super::foundation::Uri, thumbnail: &streams::IRandomAccessStreamReference) -> Result<ComPtr<super::foundation::IAsyncOperation<StorageFile>>> { unsafe {
        <Self as RtActivatable<IStorageFileStatics>>::get_activation_factory().replace_with_streamed_file_from_uri_async(fileToReplace, uri, thumbnail)
    }}
}
DEFINE_CLSID!(StorageFile(&[87,105,110,100,111,119,115,46,83,116,111,114,97,103,101,46,83,116,111,114,97,103,101,70,105,108,101,0]) [CLSID_StorageFile]);
DEFINE_IID!(IID_IStorageFile2, 2504936399, 2679, 17147, 183, 119, 194, 237, 88, 165, 46, 68);
RT_INTERFACE!{interface IStorageFile2(IStorageFile2Vtbl): IInspectable(IInspectableVtbl) [IID_IStorageFile2] {
    fn OpenWithOptionsAsync(&self, accessMode: FileAccessMode, options: StorageOpenOptions, out: *mut *mut super::foundation::IAsyncOperation<streams::IRandomAccessStream>) -> HRESULT,
    fn OpenTransactedWriteWithOptionsAsync(&self, options: StorageOpenOptions, out: *mut *mut super::foundation::IAsyncOperation<StorageStreamTransaction>) -> HRESULT
}}
impl IStorageFile2 {
    #[inline] pub unsafe fn open_with_options_async(&self, accessMode: FileAccessMode, options: StorageOpenOptions) -> Result<ComPtr<super::foundation::IAsyncOperation<streams::IRandomAccessStream>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).OpenWithOptionsAsync)(self as *const _ as *mut _, accessMode, options, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn open_transacted_write_with_options_async(&self, options: StorageOpenOptions) -> Result<ComPtr<super::foundation::IAsyncOperation<StorageStreamTransaction>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).OpenTransactedWriteWithOptionsAsync)(self as *const _ as *mut _, options, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
DEFINE_IID!(IID_IStorageFilePropertiesWithAvailability, 2949365403, 22571, 16691, 150, 72, 228, 76, 164, 110, 228, 145);
RT_INTERFACE!{interface IStorageFilePropertiesWithAvailability(IStorageFilePropertiesWithAvailabilityVtbl): IInspectable(IInspectableVtbl) [IID_IStorageFilePropertiesWithAvailability] {
    fn get_IsAvailable(&self, out: *mut bool) -> HRESULT
}}
impl IStorageFilePropertiesWithAvailability {
    #[inline] pub unsafe fn get_is_available(&self) -> Result<bool> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_IsAvailable)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
}
DEFINE_IID!(IID_IStorageFileStatics, 1501873936, 56050, 17352, 139, 180, 164, 211, 234, 207, 208, 63);
RT_INTERFACE!{static interface IStorageFileStatics(IStorageFileStaticsVtbl): IInspectable(IInspectableVtbl) [IID_IStorageFileStatics] {
    fn GetFileFromPathAsync(&self, path: HSTRING, out: *mut *mut super::foundation::IAsyncOperation<StorageFile>) -> HRESULT,
    fn GetFileFromApplicationUriAsync(&self, uri: *mut super::foundation::Uri, out: *mut *mut super::foundation::IAsyncOperation<StorageFile>) -> HRESULT,
    fn CreateStreamedFileAsync(&self, displayNameWithExtension: HSTRING, dataRequested: *mut StreamedFileDataRequestedHandler, thumbnail: *mut streams::IRandomAccessStreamReference, out: *mut *mut super::foundation::IAsyncOperation<StorageFile>) -> HRESULT,
    fn ReplaceWithStreamedFileAsync(&self, fileToReplace: *mut IStorageFile, dataRequested: *mut StreamedFileDataRequestedHandler, thumbnail: *mut streams::IRandomAccessStreamReference, out: *mut *mut super::foundation::IAsyncOperation<StorageFile>) -> HRESULT,
    fn CreateStreamedFileFromUriAsync(&self, displayNameWithExtension: HSTRING, uri: *mut super::foundation::Uri, thumbnail: *mut streams::IRandomAccessStreamReference, out: *mut *mut super::foundation::IAsyncOperation<StorageFile>) -> HRESULT,
    fn ReplaceWithStreamedFileFromUriAsync(&self, fileToReplace: *mut IStorageFile, uri: *mut super::foundation::Uri, thumbnail: *mut streams::IRandomAccessStreamReference, out: *mut *mut super::foundation::IAsyncOperation<StorageFile>) -> HRESULT
}}
impl IStorageFileStatics {
    #[inline] pub unsafe fn get_file_from_path_async(&self, path: &HStringArg) -> Result<ComPtr<super::foundation::IAsyncOperation<StorageFile>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetFileFromPathAsync)(self as *const _ as *mut _, path.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_file_from_application_uri_async(&self, uri: &super::foundation::Uri) -> Result<ComPtr<super::foundation::IAsyncOperation<StorageFile>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetFileFromApplicationUriAsync)(self as *const _ as *mut _, uri as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn create_streamed_file_async(&self, displayNameWithExtension: &HStringArg, dataRequested: &StreamedFileDataRequestedHandler, thumbnail: &streams::IRandomAccessStreamReference) -> Result<ComPtr<super::foundation::IAsyncOperation<StorageFile>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).CreateStreamedFileAsync)(self as *const _ as *mut _, displayNameWithExtension.get(), dataRequested as *const _ as *mut _, thumbnail as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn replace_with_streamed_file_async(&self, fileToReplace: &IStorageFile, dataRequested: &StreamedFileDataRequestedHandler, thumbnail: &streams::IRandomAccessStreamReference) -> Result<ComPtr<super::foundation::IAsyncOperation<StorageFile>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).ReplaceWithStreamedFileAsync)(self as *const _ as *mut _, fileToReplace as *const _ as *mut _, dataRequested as *const _ as *mut _, thumbnail as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn create_streamed_file_from_uri_async(&self, displayNameWithExtension: &HStringArg, uri: &super::foundation::Uri, thumbnail: &streams::IRandomAccessStreamReference) -> Result<ComPtr<super::foundation::IAsyncOperation<StorageFile>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).CreateStreamedFileFromUriAsync)(self as *const _ as *mut _, displayNameWithExtension.get(), uri as *const _ as *mut _, thumbnail as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn replace_with_streamed_file_from_uri_async(&self, fileToReplace: &IStorageFile, uri: &super::foundation::Uri, thumbnail: &streams::IRandomAccessStreamReference) -> Result<ComPtr<super::foundation::IAsyncOperation<StorageFile>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).ReplaceWithStreamedFileFromUriAsync)(self as *const _ as *mut _, fileToReplace as *const _ as *mut _, uri as *const _ as *mut _, thumbnail as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
DEFINE_IID!(IID_IStorageFolder, 1926351736, 46063, 20341, 168, 11, 111, 217, 218, 226, 148, 75);
RT_INTERFACE!{interface IStorageFolder(IStorageFolderVtbl): IInspectable(IInspectableVtbl) [IID_IStorageFolder] {
    fn CreateFileAsyncOverloadDefaultOptions(&self, desiredName: HSTRING, out: *mut *mut super::foundation::IAsyncOperation<StorageFile>) -> HRESULT,
    fn CreateFileAsync(&self, desiredName: HSTRING, options: CreationCollisionOption, out: *mut *mut super::foundation::IAsyncOperation<StorageFile>) -> HRESULT,
    fn CreateFolderAsyncOverloadDefaultOptions(&self, desiredName: HSTRING, out: *mut *mut super::foundation::IAsyncOperation<StorageFolder>) -> HRESULT,
    fn CreateFolderAsync(&self, desiredName: HSTRING, options: CreationCollisionOption, out: *mut *mut super::foundation::IAsyncOperation<StorageFolder>) -> HRESULT,
    fn GetFileAsync(&self, name: HSTRING, out: *mut *mut super::foundation::IAsyncOperation<StorageFile>) -> HRESULT,
    fn GetFolderAsync(&self, name: HSTRING, out: *mut *mut super::foundation::IAsyncOperation<StorageFolder>) -> HRESULT,
    fn GetItemAsync(&self, name: HSTRING, out: *mut *mut super::foundation::IAsyncOperation<IStorageItem>) -> HRESULT,
    fn GetFilesAsyncOverloadDefaultOptionsStartAndCount(&self, out: *mut *mut super::foundation::IAsyncOperation<super::foundation::collections::IVectorView<StorageFile>>) -> HRESULT,
    fn GetFoldersAsyncOverloadDefaultOptionsStartAndCount(&self, out: *mut *mut super::foundation::IAsyncOperation<super::foundation::collections::IVectorView<StorageFolder>>) -> HRESULT,
    fn GetItemsAsyncOverloadDefaultStartAndCount(&self, out: *mut *mut super::foundation::IAsyncOperation<super::foundation::collections::IVectorView<IStorageItem>>) -> HRESULT
}}
impl IStorageFolder {
    #[inline] pub unsafe fn create_file_async_overload_default_options(&self, desiredName: &HStringArg) -> Result<ComPtr<super::foundation::IAsyncOperation<StorageFile>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).CreateFileAsyncOverloadDefaultOptions)(self as *const _ as *mut _, desiredName.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn create_file_async(&self, desiredName: &HStringArg, options: CreationCollisionOption) -> Result<ComPtr<super::foundation::IAsyncOperation<StorageFile>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).CreateFileAsync)(self as *const _ as *mut _, desiredName.get(), options, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn create_folder_async_overload_default_options(&self, desiredName: &HStringArg) -> Result<ComPtr<super::foundation::IAsyncOperation<StorageFolder>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).CreateFolderAsyncOverloadDefaultOptions)(self as *const _ as *mut _, desiredName.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn create_folder_async(&self, desiredName: &HStringArg, options: CreationCollisionOption) -> Result<ComPtr<super::foundation::IAsyncOperation<StorageFolder>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).CreateFolderAsync)(self as *const _ as *mut _, desiredName.get(), options, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_file_async(&self, name: &HStringArg) -> Result<ComPtr<super::foundation::IAsyncOperation<StorageFile>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetFileAsync)(self as *const _ as *mut _, name.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_folder_async(&self, name: &HStringArg) -> Result<ComPtr<super::foundation::IAsyncOperation<StorageFolder>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetFolderAsync)(self as *const _ as *mut _, name.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_item_async(&self, name: &HStringArg) -> Result<ComPtr<super::foundation::IAsyncOperation<IStorageItem>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetItemAsync)(self as *const _ as *mut _, name.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_files_async_overload_default_options_start_and_count(&self) -> Result<ComPtr<super::foundation::IAsyncOperation<super::foundation::collections::IVectorView<StorageFile>>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetFilesAsyncOverloadDefaultOptionsStartAndCount)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_folders_async_overload_default_options_start_and_count(&self) -> Result<ComPtr<super::foundation::IAsyncOperation<super::foundation::collections::IVectorView<StorageFolder>>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetFoldersAsyncOverloadDefaultOptionsStartAndCount)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_items_async_overload_default_start_and_count(&self) -> Result<ComPtr<super::foundation::IAsyncOperation<super::foundation::collections::IVectorView<IStorageItem>>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetItemsAsyncOverloadDefaultStartAndCount)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
RT_CLASS!{class StorageFolder: IStorageFolder}
impl RtActivatable<IStorageFolderStatics> for StorageFolder {}
impl StorageFolder {
    #[inline] pub fn get_folder_from_path_async(path: &HStringArg) -> Result<ComPtr<super::foundation::IAsyncOperation<StorageFolder>>> { unsafe {
        <Self as RtActivatable<IStorageFolderStatics>>::get_activation_factory().get_folder_from_path_async(path)
    }}
}
DEFINE_CLSID!(StorageFolder(&[87,105,110,100,111,119,115,46,83,116,111,114,97,103,101,46,83,116,111,114,97,103,101,70,111,108,100,101,114,0]) [CLSID_StorageFolder]);
DEFINE_IID!(IID_IStorageFolder2, 3894929593, 2265, 19086, 160, 172, 254, 94, 211, 203, 187, 211);
RT_INTERFACE!{interface IStorageFolder2(IStorageFolder2Vtbl): IInspectable(IInspectableVtbl) [IID_IStorageFolder2] {
    fn TryGetItemAsync(&self, name: HSTRING, out: *mut *mut super::foundation::IAsyncOperation<IStorageItem>) -> HRESULT
}}
impl IStorageFolder2 {
    #[inline] pub unsafe fn try_get_item_async(&self, name: &HStringArg) -> Result<ComPtr<super::foundation::IAsyncOperation<IStorageItem>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).TryGetItemAsync)(self as *const _ as *mut _, name.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
DEFINE_IID!(IID_IStorageFolderStatics, 150153215, 34261, 18617, 174, 233, 40, 81, 30, 51, 159, 159);
RT_INTERFACE!{static interface IStorageFolderStatics(IStorageFolderStaticsVtbl): IInspectable(IInspectableVtbl) [IID_IStorageFolderStatics] {
    fn GetFolderFromPathAsync(&self, path: HSTRING, out: *mut *mut super::foundation::IAsyncOperation<StorageFolder>) -> HRESULT
}}
impl IStorageFolderStatics {
    #[inline] pub unsafe fn get_folder_from_path_async(&self, path: &HStringArg) -> Result<ComPtr<super::foundation::IAsyncOperation<StorageFolder>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetFolderFromPathAsync)(self as *const _ as *mut _, path.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
DEFINE_IID!(IID_IStorageItem, 1107798422, 51759, 17143, 189, 232, 139, 16, 69, 122, 127, 48);
RT_INTERFACE!{interface IStorageItem(IStorageItemVtbl): IInspectable(IInspectableVtbl) [IID_IStorageItem] {
    fn RenameAsyncOverloadDefaultOptions(&self, desiredName: HSTRING, out: *mut *mut super::foundation::IAsyncAction) -> HRESULT,
    fn RenameAsync(&self, desiredName: HSTRING, option: NameCollisionOption, out: *mut *mut super::foundation::IAsyncAction) -> HRESULT,
    fn DeleteAsyncOverloadDefaultOptions(&self, out: *mut *mut super::foundation::IAsyncAction) -> HRESULT,
    fn DeleteAsync(&self, option: StorageDeleteOption, out: *mut *mut super::foundation::IAsyncAction) -> HRESULT,
    fn GetBasicPropertiesAsync(&self, out: *mut *mut super::foundation::IAsyncOperation<fileproperties::BasicProperties>) -> HRESULT,
    fn get_Name(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Path(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Attributes(&self, out: *mut FileAttributes) -> HRESULT,
    fn get_DateCreated(&self, out: *mut super::foundation::DateTime) -> HRESULT,
    fn IsOfType(&self, type_: StorageItemTypes, out: *mut bool) -> HRESULT
}}
impl IStorageItem {
    #[inline] pub unsafe fn rename_async_overload_default_options(&self, desiredName: &HStringArg) -> Result<ComPtr<super::foundation::IAsyncAction>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).RenameAsyncOverloadDefaultOptions)(self as *const _ as *mut _, desiredName.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn rename_async(&self, desiredName: &HStringArg, option: NameCollisionOption) -> Result<ComPtr<super::foundation::IAsyncAction>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).RenameAsync)(self as *const _ as *mut _, desiredName.get(), option, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn delete_async_overload_default_options(&self) -> Result<ComPtr<super::foundation::IAsyncAction>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).DeleteAsyncOverloadDefaultOptions)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn delete_async(&self, option: StorageDeleteOption) -> Result<ComPtr<super::foundation::IAsyncAction>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).DeleteAsync)(self as *const _ as *mut _, option, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_basic_properties_async(&self) -> Result<ComPtr<super::foundation::IAsyncOperation<fileproperties::BasicProperties>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetBasicPropertiesAsync)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_name(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Name)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_path(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Path)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_attributes(&self) -> Result<FileAttributes> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_Attributes)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_date_created(&self) -> Result<super::foundation::DateTime> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_DateCreated)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn is_of_type(&self, type_: StorageItemTypes) -> Result<bool> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).IsOfType)(self as *const _ as *mut _, type_, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
}
DEFINE_IID!(IID_IStorageItem2, 1408837330, 2108, 17027, 180, 91, 129, 192, 7, 35, 126, 68);
RT_INTERFACE!{interface IStorageItem2(IStorageItem2Vtbl): IInspectable(IInspectableVtbl) [IID_IStorageItem2] {
    fn GetParentAsync(&self, out: *mut *mut super::foundation::IAsyncOperation<StorageFolder>) -> HRESULT,
    fn IsEqual(&self, item: *mut IStorageItem, out: *mut bool) -> HRESULT
}}
impl IStorageItem2 {
    #[inline] pub unsafe fn get_parent_async(&self) -> Result<ComPtr<super::foundation::IAsyncOperation<StorageFolder>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetParentAsync)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn is_equal(&self, item: &IStorageItem) -> Result<bool> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).IsEqual)(self as *const _ as *mut _, item as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
}
DEFINE_IID!(IID_IStorageItemProperties, 2254849144, 32809, 18174, 167, 137, 28, 47, 62, 47, 251, 92);
RT_INTERFACE!{interface IStorageItemProperties(IStorageItemPropertiesVtbl): IInspectable(IInspectableVtbl) [IID_IStorageItemProperties] {
    fn GetThumbnailAsyncOverloadDefaultSizeDefaultOptions(&self, mode: fileproperties::ThumbnailMode, out: *mut *mut super::foundation::IAsyncOperation<fileproperties::StorageItemThumbnail>) -> HRESULT,
    fn GetThumbnailAsyncOverloadDefaultOptions(&self, mode: fileproperties::ThumbnailMode, requestedSize: u32, out: *mut *mut super::foundation::IAsyncOperation<fileproperties::StorageItemThumbnail>) -> HRESULT,
    fn GetThumbnailAsync(&self, mode: fileproperties::ThumbnailMode, requestedSize: u32, options: fileproperties::ThumbnailOptions, out: *mut *mut super::foundation::IAsyncOperation<fileproperties::StorageItemThumbnail>) -> HRESULT,
    fn get_DisplayName(&self, out: *mut HSTRING) -> HRESULT,
    fn get_DisplayType(&self, out: *mut HSTRING) -> HRESULT,
    fn get_FolderRelativeId(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Properties(&self, out: *mut *mut fileproperties::StorageItemContentProperties) -> HRESULT
}}
impl IStorageItemProperties {
    #[inline] pub unsafe fn get_thumbnail_async_overload_default_size_default_options(&self, mode: fileproperties::ThumbnailMode) -> Result<ComPtr<super::foundation::IAsyncOperation<fileproperties::StorageItemThumbnail>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetThumbnailAsyncOverloadDefaultSizeDefaultOptions)(self as *const _ as *mut _, mode, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_thumbnail_async_overload_default_options(&self, mode: fileproperties::ThumbnailMode, requestedSize: u32) -> Result<ComPtr<super::foundation::IAsyncOperation<fileproperties::StorageItemThumbnail>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetThumbnailAsyncOverloadDefaultOptions)(self as *const _ as *mut _, mode, requestedSize, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_thumbnail_async(&self, mode: fileproperties::ThumbnailMode, requestedSize: u32, options: fileproperties::ThumbnailOptions) -> Result<ComPtr<super::foundation::IAsyncOperation<fileproperties::StorageItemThumbnail>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetThumbnailAsync)(self as *const _ as *mut _, mode, requestedSize, options, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_display_name(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_DisplayName)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_display_type(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_DisplayType)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_folder_relative_id(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_FolderRelativeId)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_properties(&self) -> Result<ComPtr<fileproperties::StorageItemContentProperties>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Properties)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
DEFINE_IID!(IID_IStorageItemProperties2, 2391189841, 1209, 19410, 146, 157, 254, 243, 247, 22, 33, 208);
RT_INTERFACE!{interface IStorageItemProperties2(IStorageItemProperties2Vtbl): IInspectable(IInspectableVtbl) [IID_IStorageItemProperties2] {
    fn GetScaledImageAsThumbnailAsyncOverloadDefaultSizeDefaultOptions(&self, mode: fileproperties::ThumbnailMode, out: *mut *mut super::foundation::IAsyncOperation<fileproperties::StorageItemThumbnail>) -> HRESULT,
    fn GetScaledImageAsThumbnailAsyncOverloadDefaultOptions(&self, mode: fileproperties::ThumbnailMode, requestedSize: u32, out: *mut *mut super::foundation::IAsyncOperation<fileproperties::StorageItemThumbnail>) -> HRESULT,
    fn GetScaledImageAsThumbnailAsync(&self, mode: fileproperties::ThumbnailMode, requestedSize: u32, options: fileproperties::ThumbnailOptions, out: *mut *mut super::foundation::IAsyncOperation<fileproperties::StorageItemThumbnail>) -> HRESULT
}}
impl IStorageItemProperties2 {
    #[inline] pub unsafe fn get_scaled_image_as_thumbnail_async_overload_default_size_default_options(&self, mode: fileproperties::ThumbnailMode) -> Result<ComPtr<super::foundation::IAsyncOperation<fileproperties::StorageItemThumbnail>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetScaledImageAsThumbnailAsyncOverloadDefaultSizeDefaultOptions)(self as *const _ as *mut _, mode, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_scaled_image_as_thumbnail_async_overload_default_options(&self, mode: fileproperties::ThumbnailMode, requestedSize: u32) -> Result<ComPtr<super::foundation::IAsyncOperation<fileproperties::StorageItemThumbnail>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetScaledImageAsThumbnailAsyncOverloadDefaultOptions)(self as *const _ as *mut _, mode, requestedSize, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_scaled_image_as_thumbnail_async(&self, mode: fileproperties::ThumbnailMode, requestedSize: u32, options: fileproperties::ThumbnailOptions) -> Result<ComPtr<super::foundation::IAsyncOperation<fileproperties::StorageItemThumbnail>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetScaledImageAsThumbnailAsync)(self as *const _ as *mut _, mode, requestedSize, options, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
DEFINE_IID!(IID_IStorageItemPropertiesWithProvider, 2249978779, 25448, 19950, 180, 14, 116, 104, 74, 92, 231, 20);
RT_INTERFACE!{interface IStorageItemPropertiesWithProvider(IStorageItemPropertiesWithProviderVtbl): IInspectable(IInspectableVtbl) [IID_IStorageItemPropertiesWithProvider] {
    fn get_Provider(&self, out: *mut *mut StorageProvider) -> HRESULT
}}
impl IStorageItemPropertiesWithProvider {
    #[inline] pub unsafe fn get_provider(&self) -> Result<ComPtr<StorageProvider>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Provider)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
RT_ENUM! { enum StorageItemTypes: u32 {
    None (StorageItemTypes_None) = 0, File (StorageItemTypes_File) = 1, Folder (StorageItemTypes_Folder) = 2,
}}
DEFINE_IID!(IID_IStorageLibrary, 517828867, 3678, 19820, 181, 232, 147, 24, 152, 61, 106, 3);
RT_INTERFACE!{interface IStorageLibrary(IStorageLibraryVtbl): IInspectable(IInspectableVtbl) [IID_IStorageLibrary] {
    fn RequestAddFolderAsync(&self, out: *mut *mut super::foundation::IAsyncOperation<StorageFolder>) -> HRESULT,
    fn RequestRemoveFolderAsync(&self, folder: *mut StorageFolder, out: *mut *mut super::foundation::IAsyncOperation<bool>) -> HRESULT,
    fn get_Folders(&self, out: *mut *mut super::foundation::collections::IObservableVector<StorageFolder>) -> HRESULT,
    fn get_SaveFolder(&self, out: *mut *mut StorageFolder) -> HRESULT,
    fn add_DefinitionChanged(&self, handler: *mut super::foundation::TypedEventHandler<StorageLibrary, IInspectable>, out: *mut super::foundation::EventRegistrationToken) -> HRESULT,
    fn remove_DefinitionChanged(&self, eventCookie: super::foundation::EventRegistrationToken) -> HRESULT
}}
impl IStorageLibrary {
    #[inline] pub unsafe fn request_add_folder_async(&self) -> Result<ComPtr<super::foundation::IAsyncOperation<StorageFolder>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).RequestAddFolderAsync)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn request_remove_folder_async(&self, folder: &StorageFolder) -> Result<ComPtr<super::foundation::IAsyncOperation<bool>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).RequestRemoveFolderAsync)(self as *const _ as *mut _, folder as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_folders(&self) -> Result<ComPtr<super::foundation::collections::IObservableVector<StorageFolder>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Folders)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_save_folder(&self) -> Result<ComPtr<StorageFolder>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_SaveFolder)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn add_definition_changed(&self, handler: &super::foundation::TypedEventHandler<StorageLibrary, IInspectable>) -> Result<super::foundation::EventRegistrationToken> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).add_DefinitionChanged)(self as *const _ as *mut _, handler as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn remove_definition_changed(&self, eventCookie: super::foundation::EventRegistrationToken) -> Result<()> {
        let hr = ((*self.lpVtbl).remove_DefinitionChanged)(self as *const _ as *mut _, eventCookie);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
}
RT_CLASS!{class StorageLibrary: IStorageLibrary}
impl RtActivatable<IStorageLibraryStatics> for StorageLibrary {}
impl RtActivatable<IStorageLibraryStatics2> for StorageLibrary {}
impl StorageLibrary {
    #[inline] pub fn get_library_async(libraryId: KnownLibraryId) -> Result<ComPtr<super::foundation::IAsyncOperation<StorageLibrary>>> { unsafe {
        <Self as RtActivatable<IStorageLibraryStatics>>::get_activation_factory().get_library_async(libraryId)
    }}
    #[cfg(feature="windows-system")] #[inline] pub fn get_library_for_user_async(user: &super::system::User, libraryId: KnownLibraryId) -> Result<ComPtr<super::foundation::IAsyncOperation<StorageLibrary>>> { unsafe {
        <Self as RtActivatable<IStorageLibraryStatics2>>::get_activation_factory().get_library_for_user_async(user, libraryId)
    }}
}
DEFINE_CLSID!(StorageLibrary(&[87,105,110,100,111,119,115,46,83,116,111,114,97,103,101,46,83,116,111,114,97,103,101,76,105,98,114,97,114,121,0]) [CLSID_StorageLibrary]);
DEFINE_IID!(IID_IStorageLibrary2, 1527571272, 64691, 16433, 175, 176, 166, 141, 123, 212, 69, 52);
RT_INTERFACE!{interface IStorageLibrary2(IStorageLibrary2Vtbl): IInspectable(IInspectableVtbl) [IID_IStorageLibrary2] {
    fn get_ChangeTracker(&self, out: *mut *mut StorageLibraryChangeTracker) -> HRESULT
}}
impl IStorageLibrary2 {
    #[inline] pub unsafe fn get_change_tracker(&self) -> Result<ComPtr<StorageLibraryChangeTracker>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_ChangeTracker)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
DEFINE_IID!(IID_IStorageLibrary3, 2317882001, 8532, 16897, 129, 19, 210, 192, 92, 225, 173, 35);
RT_INTERFACE!{interface IStorageLibrary3(IStorageLibrary3Vtbl): IInspectable(IInspectableVtbl) [IID_IStorageLibrary3] {
    fn AreFolderSuggestionsAvailableAsync(&self, out: *mut *mut super::foundation::IAsyncOperation<bool>) -> HRESULT
}}
impl IStorageLibrary3 {
    #[inline] pub unsafe fn are_folder_suggestions_available_async(&self) -> Result<ComPtr<super::foundation::IAsyncOperation<bool>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).AreFolderSuggestionsAvailableAsync)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
DEFINE_IID!(IID_IStorageLibraryChange, 9964323, 11234, 18697, 170, 72, 21, 159, 82, 3, 165, 30);
RT_INTERFACE!{interface IStorageLibraryChange(IStorageLibraryChangeVtbl): IInspectable(IInspectableVtbl) [IID_IStorageLibraryChange] {
    fn get_ChangeType(&self, out: *mut StorageLibraryChangeType) -> HRESULT,
    fn get_Path(&self, out: *mut HSTRING) -> HRESULT,
    fn get_PreviousPath(&self, out: *mut HSTRING) -> HRESULT,
    fn IsOfType(&self, type_: StorageItemTypes, out: *mut bool) -> HRESULT,
    fn GetStorageItemAsync(&self, out: *mut *mut super::foundation::IAsyncOperation<IStorageItem>) -> HRESULT
}}
impl IStorageLibraryChange {
    #[inline] pub unsafe fn get_change_type(&self) -> Result<StorageLibraryChangeType> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_ChangeType)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_path(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Path)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_previous_path(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_PreviousPath)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn is_of_type(&self, type_: StorageItemTypes) -> Result<bool> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).IsOfType)(self as *const _ as *mut _, type_, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_storage_item_async(&self) -> Result<ComPtr<super::foundation::IAsyncOperation<IStorageItem>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetStorageItemAsync)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
RT_CLASS!{class StorageLibraryChange: IStorageLibraryChange}
DEFINE_IID!(IID_IStorageLibraryChangeReader, 4060462211, 64674, 16889, 137, 84, 238, 46, 153, 30, 185, 111);
RT_INTERFACE!{interface IStorageLibraryChangeReader(IStorageLibraryChangeReaderVtbl): IInspectable(IInspectableVtbl) [IID_IStorageLibraryChangeReader] {
    fn ReadBatchAsync(&self, out: *mut *mut super::foundation::IAsyncOperation<super::foundation::collections::IVectorView<StorageLibraryChange>>) -> HRESULT,
    fn AcceptChangesAsync(&self, out: *mut *mut super::foundation::IAsyncAction) -> HRESULT
}}
impl IStorageLibraryChangeReader {
    #[inline] pub unsafe fn read_batch_async(&self) -> Result<ComPtr<super::foundation::IAsyncOperation<super::foundation::collections::IVectorView<StorageLibraryChange>>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).ReadBatchAsync)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn accept_changes_async(&self) -> Result<ComPtr<super::foundation::IAsyncAction>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).AcceptChangesAsync)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
RT_CLASS!{class StorageLibraryChangeReader: IStorageLibraryChangeReader}
DEFINE_IID!(IID_IStorageLibraryChangeTracker, 2652205846, 24691, 17654, 150, 129, 116, 146, 209, 40, 108, 144);
RT_INTERFACE!{interface IStorageLibraryChangeTracker(IStorageLibraryChangeTrackerVtbl): IInspectable(IInspectableVtbl) [IID_IStorageLibraryChangeTracker] {
    fn GetChangeReader(&self, out: *mut *mut StorageLibraryChangeReader) -> HRESULT,
    fn Enable(&self) -> HRESULT,
    fn Reset(&self) -> HRESULT
}}
impl IStorageLibraryChangeTracker {
    #[inline] pub unsafe fn get_change_reader(&self) -> Result<ComPtr<StorageLibraryChangeReader>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetChangeReader)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn enable(&self) -> Result<()> {
        let hr = ((*self.lpVtbl).Enable)(self as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn reset(&self) -> Result<()> {
        let hr = ((*self.lpVtbl).Reset)(self as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
}
RT_CLASS!{class StorageLibraryChangeTracker: IStorageLibraryChangeTracker}
RT_ENUM! { enum StorageLibraryChangeType: i32 {
    Created (StorageLibraryChangeType_Created) = 0, Deleted (StorageLibraryChangeType_Deleted) = 1, MovedOrRenamed (StorageLibraryChangeType_MovedOrRenamed) = 2, ContentsChanged (StorageLibraryChangeType_ContentsChanged) = 3, MovedOutOfLibrary (StorageLibraryChangeType_MovedOutOfLibrary) = 4, MovedIntoLibrary (StorageLibraryChangeType_MovedIntoLibrary) = 5, ContentsReplaced (StorageLibraryChangeType_ContentsReplaced) = 6, IndexingStatusChanged (StorageLibraryChangeType_IndexingStatusChanged) = 7, EncryptionChanged (StorageLibraryChangeType_EncryptionChanged) = 8, ChangeTrackingLost (StorageLibraryChangeType_ChangeTrackingLost) = 9,
}}
DEFINE_IID!(IID_IStorageLibraryStatics, 1107863259, 26698, 18886, 158, 89, 144, 18, 30, 224, 80, 214);
RT_INTERFACE!{static interface IStorageLibraryStatics(IStorageLibraryStaticsVtbl): IInspectable(IInspectableVtbl) [IID_IStorageLibraryStatics] {
    fn GetLibraryAsync(&self, libraryId: KnownLibraryId, out: *mut *mut super::foundation::IAsyncOperation<StorageLibrary>) -> HRESULT
}}
impl IStorageLibraryStatics {
    #[inline] pub unsafe fn get_library_async(&self, libraryId: KnownLibraryId) -> Result<ComPtr<super::foundation::IAsyncOperation<StorageLibrary>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetLibraryAsync)(self as *const _ as *mut _, libraryId, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
DEFINE_IID!(IID_IStorageLibraryStatics2, 4289760732, 64117, 18069, 185, 209, 127, 129, 249, 120, 50, 227);
RT_INTERFACE!{static interface IStorageLibraryStatics2(IStorageLibraryStatics2Vtbl): IInspectable(IInspectableVtbl) [IID_IStorageLibraryStatics2] {
    #[cfg(feature="windows-system")] fn GetLibraryForUserAsync(&self, user: *mut super::system::User, libraryId: KnownLibraryId, out: *mut *mut super::foundation::IAsyncOperation<StorageLibrary>) -> HRESULT
}}
impl IStorageLibraryStatics2 {
    #[cfg(feature="windows-system")] #[inline] pub unsafe fn get_library_for_user_async(&self, user: &super::system::User, libraryId: KnownLibraryId) -> Result<ComPtr<super::foundation::IAsyncOperation<StorageLibrary>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetLibraryForUserAsync)(self as *const _ as *mut _, user as *const _ as *mut _, libraryId, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
RT_ENUM! { enum StorageOpenOptions: u32 {
    None (StorageOpenOptions_None) = 0, AllowOnlyReaders (StorageOpenOptions_AllowOnlyReaders) = 1, AllowReadersAndWriters (StorageOpenOptions_AllowReadersAndWriters) = 2,
}}
DEFINE_IID!(IID_IStorageProvider, 3875925716, 54392, 18390, 186, 70, 26, 142, 190, 17, 74, 32);
RT_INTERFACE!{interface IStorageProvider(IStorageProviderVtbl): IInspectable(IInspectableVtbl) [IID_IStorageProvider] {
    fn get_Id(&self, out: *mut HSTRING) -> HRESULT,
    fn get_DisplayName(&self, out: *mut HSTRING) -> HRESULT
}}
impl IStorageProvider {
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
}
RT_CLASS!{class StorageProvider: IStorageProvider}
DEFINE_IID!(IID_IStorageProvider2, 17635607, 13316, 16715, 159, 215, 205, 68, 71, 46, 170, 57);
RT_INTERFACE!{interface IStorageProvider2(IStorageProvider2Vtbl): IInspectable(IInspectableVtbl) [IID_IStorageProvider2] {
    fn IsPropertySupportedForPartialFileAsync(&self, propertyCanonicalName: HSTRING, out: *mut *mut super::foundation::IAsyncOperation<bool>) -> HRESULT
}}
impl IStorageProvider2 {
    #[inline] pub unsafe fn is_property_supported_for_partial_file_async(&self, propertyCanonicalName: &HStringArg) -> Result<ComPtr<super::foundation::IAsyncOperation<bool>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).IsPropertySupportedForPartialFileAsync)(self as *const _ as *mut _, propertyCanonicalName.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
DEFINE_IID!(IID_IStorageStreamTransaction, 4135383907, 42301, 19860, 174, 44, 103, 35, 45, 147, 172, 221);
RT_INTERFACE!{interface IStorageStreamTransaction(IStorageStreamTransactionVtbl): IInspectable(IInspectableVtbl) [IID_IStorageStreamTransaction] {
    fn get_Stream(&self, out: *mut *mut streams::IRandomAccessStream) -> HRESULT,
    fn CommitAsync(&self, out: *mut *mut super::foundation::IAsyncAction) -> HRESULT
}}
impl IStorageStreamTransaction {
    #[inline] pub unsafe fn get_stream(&self) -> Result<ComPtr<streams::IRandomAccessStream>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Stream)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn commit_async(&self) -> Result<ComPtr<super::foundation::IAsyncAction>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).CommitAsync)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
RT_CLASS!{class StorageStreamTransaction: IStorageStreamTransaction}
DEFINE_IID!(IID_IStreamedFileDataRequest, 376700110, 55997, 19792, 190, 238, 24, 11, 138, 129, 145, 182);
RT_INTERFACE!{interface IStreamedFileDataRequest(IStreamedFileDataRequestVtbl): IInspectable(IInspectableVtbl) [IID_IStreamedFileDataRequest] {
    fn FailAndClose(&self, failureMode: StreamedFileFailureMode) -> HRESULT
}}
impl IStreamedFileDataRequest {
    #[inline] pub unsafe fn fail_and_close(&self, failureMode: StreamedFileFailureMode) -> Result<()> {
        let hr = ((*self.lpVtbl).FailAndClose)(self as *const _ as *mut _, failureMode);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
}
RT_CLASS!{class StreamedFileDataRequest: streams::IOutputStream}
DEFINE_IID!(IID_StreamedFileDataRequestedHandler, 4277577764, 12257, 19719, 163, 91, 183, 124, 80, 181, 244, 204);
RT_DELEGATE!{delegate StreamedFileDataRequestedHandler(StreamedFileDataRequestedHandlerVtbl, StreamedFileDataRequestedHandlerImpl) [IID_StreamedFileDataRequestedHandler] {
    fn Invoke(&self, stream: *mut StreamedFileDataRequest) -> HRESULT
}}
impl StreamedFileDataRequestedHandler {
    #[inline] pub unsafe fn invoke(&self, stream: &StreamedFileDataRequest) -> Result<()> {
        let hr = ((*self.lpVtbl).Invoke)(self as *const _ as *mut _, stream as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
}
RT_ENUM! { enum StreamedFileFailureMode: i32 {
    Failed (StreamedFileFailureMode_Failed) = 0, CurrentlyUnavailable (StreamedFileFailureMode_CurrentlyUnavailable) = 1, Incomplete (StreamedFileFailureMode_Incomplete) = 2,
}}
DEFINE_IID!(IID_ISystemAudioProperties, 1066350775, 12428, 18401, 146, 77, 134, 69, 52, 142, 93, 183);
RT_INTERFACE!{interface ISystemAudioProperties(ISystemAudioPropertiesVtbl): IInspectable(IInspectableVtbl) [IID_ISystemAudioProperties] {
    fn get_EncodingBitrate(&self, out: *mut HSTRING) -> HRESULT
}}
impl ISystemAudioProperties {
    #[inline] pub unsafe fn get_encoding_bitrate(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_EncodingBitrate)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
}
RT_CLASS!{class SystemAudioProperties: ISystemAudioProperties}
DEFINE_IID!(IID_ISystemDataPaths, 3811229552, 55546, 17900, 169, 66, 210, 226, 111, 182, 11, 165);
RT_INTERFACE!{interface ISystemDataPaths(ISystemDataPathsVtbl): IInspectable(IInspectableVtbl) [IID_ISystemDataPaths] {
    fn get_Fonts(&self, out: *mut HSTRING) -> HRESULT,
    fn get_ProgramData(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Public(&self, out: *mut HSTRING) -> HRESULT,
    fn get_PublicDesktop(&self, out: *mut HSTRING) -> HRESULT,
    fn get_PublicDocuments(&self, out: *mut HSTRING) -> HRESULT,
    fn get_PublicDownloads(&self, out: *mut HSTRING) -> HRESULT,
    fn get_PublicMusic(&self, out: *mut HSTRING) -> HRESULT,
    fn get_PublicPictures(&self, out: *mut HSTRING) -> HRESULT,
    fn get_PublicVideos(&self, out: *mut HSTRING) -> HRESULT,
    fn get_System(&self, out: *mut HSTRING) -> HRESULT,
    fn get_SystemHost(&self, out: *mut HSTRING) -> HRESULT,
    fn get_SystemX86(&self, out: *mut HSTRING) -> HRESULT,
    fn get_SystemX64(&self, out: *mut HSTRING) -> HRESULT,
    fn get_SystemArm(&self, out: *mut HSTRING) -> HRESULT,
    fn get_UserProfiles(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Windows(&self, out: *mut HSTRING) -> HRESULT
}}
impl ISystemDataPaths {
    #[inline] pub unsafe fn get_fonts(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Fonts)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_program_data(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_ProgramData)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_public(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Public)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_public_desktop(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_PublicDesktop)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_public_documents(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_PublicDocuments)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_public_downloads(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_PublicDownloads)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_public_music(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_PublicMusic)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_public_pictures(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_PublicPictures)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_public_videos(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_PublicVideos)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_system(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_System)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_system_host(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_SystemHost)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_system_x86(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_SystemX86)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_system_x64(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_SystemX64)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_system_arm(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_SystemArm)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_user_profiles(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_UserProfiles)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_windows(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Windows)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
}
RT_CLASS!{class SystemDataPaths: ISystemDataPaths}
impl RtActivatable<ISystemDataPathsStatics> for SystemDataPaths {}
impl SystemDataPaths {
    #[inline] pub fn get_default() -> Result<ComPtr<SystemDataPaths>> { unsafe {
        <Self as RtActivatable<ISystemDataPathsStatics>>::get_activation_factory().get_default()
    }}
}
DEFINE_CLSID!(SystemDataPaths(&[87,105,110,100,111,119,115,46,83,116,111,114,97,103,101,46,83,121,115,116,101,109,68,97,116,97,80,97,116,104,115,0]) [CLSID_SystemDataPaths]);
DEFINE_IID!(IID_ISystemDataPathsStatics, 3774443472, 39200, 19402, 179, 121, 249, 111, 223, 124, 170, 216);
RT_INTERFACE!{static interface ISystemDataPathsStatics(ISystemDataPathsStaticsVtbl): IInspectable(IInspectableVtbl) [IID_ISystemDataPathsStatics] {
    fn GetDefault(&self, out: *mut *mut SystemDataPaths) -> HRESULT
}}
impl ISystemDataPathsStatics {
    #[inline] pub unsafe fn get_default(&self) -> Result<ComPtr<SystemDataPaths>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetDefault)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
DEFINE_IID!(IID_ISystemGPSProperties, 3237244596, 49524, 18458, 188, 37, 146, 25, 134, 246, 166, 243);
RT_INTERFACE!{interface ISystemGPSProperties(ISystemGPSPropertiesVtbl): IInspectable(IInspectableVtbl) [IID_ISystemGPSProperties] {
    fn get_LatitudeDecimal(&self, out: *mut HSTRING) -> HRESULT,
    fn get_LongitudeDecimal(&self, out: *mut HSTRING) -> HRESULT
}}
impl ISystemGPSProperties {
    #[inline] pub unsafe fn get_latitude_decimal(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_LatitudeDecimal)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_longitude_decimal(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_LongitudeDecimal)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
}
RT_CLASS!{class SystemGPSProperties: ISystemGPSProperties}
DEFINE_IID!(IID_ISystemImageProperties, 18558512, 35641, 17160, 190, 161, 232, 170, 97, 228, 120, 38);
RT_INTERFACE!{interface ISystemImageProperties(ISystemImagePropertiesVtbl): IInspectable(IInspectableVtbl) [IID_ISystemImageProperties] {
    fn get_HorizontalSize(&self, out: *mut HSTRING) -> HRESULT,
    fn get_VerticalSize(&self, out: *mut HSTRING) -> HRESULT
}}
impl ISystemImageProperties {
    #[inline] pub unsafe fn get_horizontal_size(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_HorizontalSize)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_vertical_size(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_VerticalSize)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
}
RT_CLASS!{class SystemImageProperties: ISystemImageProperties}
DEFINE_IID!(IID_ISystemMediaProperties, 2754294550, 33813, 16604, 140, 68, 152, 54, 29, 35, 84, 48);
RT_INTERFACE!{interface ISystemMediaProperties(ISystemMediaPropertiesVtbl): IInspectable(IInspectableVtbl) [IID_ISystemMediaProperties] {
    fn get_Duration(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Producer(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Publisher(&self, out: *mut HSTRING) -> HRESULT,
    fn get_SubTitle(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Writer(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Year(&self, out: *mut HSTRING) -> HRESULT
}}
impl ISystemMediaProperties {
    #[inline] pub unsafe fn get_duration(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Duration)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_producer(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Producer)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_publisher(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Publisher)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_sub_title(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_SubTitle)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_writer(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Writer)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_year(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Year)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
}
RT_CLASS!{class SystemMediaProperties: ISystemMediaProperties}
DEFINE_IID!(IID_ISystemMusicProperties, 3027863765, 26543, 19395, 141, 57, 91, 137, 2, 32, 38, 161);
RT_INTERFACE!{interface ISystemMusicProperties(ISystemMusicPropertiesVtbl): IInspectable(IInspectableVtbl) [IID_ISystemMusicProperties] {
    fn get_AlbumArtist(&self, out: *mut HSTRING) -> HRESULT,
    fn get_AlbumTitle(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Artist(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Composer(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Conductor(&self, out: *mut HSTRING) -> HRESULT,
    fn get_DisplayArtist(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Genre(&self, out: *mut HSTRING) -> HRESULT,
    fn get_TrackNumber(&self, out: *mut HSTRING) -> HRESULT
}}
impl ISystemMusicProperties {
    #[inline] pub unsafe fn get_album_artist(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_AlbumArtist)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_album_title(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_AlbumTitle)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_artist(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Artist)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_composer(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Composer)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_conductor(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Conductor)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_display_artist(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_DisplayArtist)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_genre(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Genre)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_track_number(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_TrackNumber)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
}
RT_CLASS!{class SystemMusicProperties: ISystemMusicProperties}
DEFINE_IID!(IID_ISystemPhotoProperties, 1194654781, 43809, 17444, 183, 53, 244, 53, 58, 86, 200, 252);
RT_INTERFACE!{interface ISystemPhotoProperties(ISystemPhotoPropertiesVtbl): IInspectable(IInspectableVtbl) [IID_ISystemPhotoProperties] {
    fn get_CameraManufacturer(&self, out: *mut HSTRING) -> HRESULT,
    fn get_CameraModel(&self, out: *mut HSTRING) -> HRESULT,
    fn get_DateTaken(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Orientation(&self, out: *mut HSTRING) -> HRESULT,
    fn get_PeopleNames(&self, out: *mut HSTRING) -> HRESULT
}}
impl ISystemPhotoProperties {
    #[inline] pub unsafe fn get_camera_manufacturer(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_CameraManufacturer)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_camera_model(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_CameraModel)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_date_taken(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_DateTaken)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_orientation(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Orientation)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_people_names(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_PeopleNames)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
}
RT_CLASS!{class SystemPhotoProperties: ISystemPhotoProperties}
DEFINE_IID!(IID_ISystemProperties, 2440720833, 34291, 19921, 176, 1, 165, 11, 253, 33, 200, 210);
RT_INTERFACE!{static interface ISystemProperties(ISystemPropertiesVtbl): IInspectable(IInspectableVtbl) [IID_ISystemProperties] {
    fn get_Author(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Comment(&self, out: *mut HSTRING) -> HRESULT,
    fn get_ItemNameDisplay(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Keywords(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Rating(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Title(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Audio(&self, out: *mut *mut SystemAudioProperties) -> HRESULT,
    fn get_GPS(&self, out: *mut *mut SystemGPSProperties) -> HRESULT,
    fn get_Media(&self, out: *mut *mut SystemMediaProperties) -> HRESULT,
    fn get_Music(&self, out: *mut *mut SystemMusicProperties) -> HRESULT,
    fn get_Photo(&self, out: *mut *mut SystemPhotoProperties) -> HRESULT,
    fn get_Video(&self, out: *mut *mut SystemVideoProperties) -> HRESULT,
    fn get_Image(&self, out: *mut *mut SystemImageProperties) -> HRESULT
}}
impl ISystemProperties {
    #[inline] pub unsafe fn get_author(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Author)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_comment(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Comment)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_item_name_display(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_ItemNameDisplay)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_keywords(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Keywords)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_rating(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Rating)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_title(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Title)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_audio(&self) -> Result<ComPtr<SystemAudioProperties>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Audio)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_gps(&self) -> Result<ComPtr<SystemGPSProperties>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_GPS)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_media(&self) -> Result<ComPtr<SystemMediaProperties>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Media)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_music(&self) -> Result<ComPtr<SystemMusicProperties>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Music)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_photo(&self) -> Result<ComPtr<SystemPhotoProperties>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Photo)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_video(&self) -> Result<ComPtr<SystemVideoProperties>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Video)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_image(&self) -> Result<ComPtr<SystemImageProperties>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Image)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
RT_CLASS!{static class SystemProperties}
impl RtActivatable<ISystemProperties> for SystemProperties {}
impl SystemProperties {
    #[inline] pub fn get_author() -> Result<HString> { unsafe {
        <Self as RtActivatable<ISystemProperties>>::get_activation_factory().get_author()
    }}
    #[inline] pub fn get_comment() -> Result<HString> { unsafe {
        <Self as RtActivatable<ISystemProperties>>::get_activation_factory().get_comment()
    }}
    #[inline] pub fn get_item_name_display() -> Result<HString> { unsafe {
        <Self as RtActivatable<ISystemProperties>>::get_activation_factory().get_item_name_display()
    }}
    #[inline] pub fn get_keywords() -> Result<HString> { unsafe {
        <Self as RtActivatable<ISystemProperties>>::get_activation_factory().get_keywords()
    }}
    #[inline] pub fn get_rating() -> Result<HString> { unsafe {
        <Self as RtActivatable<ISystemProperties>>::get_activation_factory().get_rating()
    }}
    #[inline] pub fn get_title() -> Result<HString> { unsafe {
        <Self as RtActivatable<ISystemProperties>>::get_activation_factory().get_title()
    }}
    #[inline] pub fn get_audio() -> Result<ComPtr<SystemAudioProperties>> { unsafe {
        <Self as RtActivatable<ISystemProperties>>::get_activation_factory().get_audio()
    }}
    #[inline] pub fn get_gps() -> Result<ComPtr<SystemGPSProperties>> { unsafe {
        <Self as RtActivatable<ISystemProperties>>::get_activation_factory().get_gps()
    }}
    #[inline] pub fn get_media() -> Result<ComPtr<SystemMediaProperties>> { unsafe {
        <Self as RtActivatable<ISystemProperties>>::get_activation_factory().get_media()
    }}
    #[inline] pub fn get_music() -> Result<ComPtr<SystemMusicProperties>> { unsafe {
        <Self as RtActivatable<ISystemProperties>>::get_activation_factory().get_music()
    }}
    #[inline] pub fn get_photo() -> Result<ComPtr<SystemPhotoProperties>> { unsafe {
        <Self as RtActivatable<ISystemProperties>>::get_activation_factory().get_photo()
    }}
    #[inline] pub fn get_video() -> Result<ComPtr<SystemVideoProperties>> { unsafe {
        <Self as RtActivatable<ISystemProperties>>::get_activation_factory().get_video()
    }}
    #[inline] pub fn get_image() -> Result<ComPtr<SystemImageProperties>> { unsafe {
        <Self as RtActivatable<ISystemProperties>>::get_activation_factory().get_image()
    }}
}
DEFINE_CLSID!(SystemProperties(&[87,105,110,100,111,119,115,46,83,116,111,114,97,103,101,46,83,121,115,116,101,109,80,114,111,112,101,114,116,105,101,115,0]) [CLSID_SystemProperties]);
DEFINE_IID!(IID_ISystemVideoProperties, 541128469, 26616, 17186, 155, 128, 79, 169, 254, 251, 131, 232);
RT_INTERFACE!{interface ISystemVideoProperties(ISystemVideoPropertiesVtbl): IInspectable(IInspectableVtbl) [IID_ISystemVideoProperties] {
    fn get_Director(&self, out: *mut HSTRING) -> HRESULT,
    fn get_FrameHeight(&self, out: *mut HSTRING) -> HRESULT,
    fn get_FrameWidth(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Orientation(&self, out: *mut HSTRING) -> HRESULT,
    fn get_TotalBitrate(&self, out: *mut HSTRING) -> HRESULT
}}
impl ISystemVideoProperties {
    #[inline] pub unsafe fn get_director(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Director)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_frame_height(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_FrameHeight)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_frame_width(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_FrameWidth)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_orientation(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Orientation)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_total_bitrate(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_TotalBitrate)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
}
RT_CLASS!{class SystemVideoProperties: ISystemVideoProperties}
DEFINE_IID!(IID_IUserDataPaths, 4190451986, 43972, 18175, 138, 43, 220, 157, 127, 166, 229, 47);
RT_INTERFACE!{interface IUserDataPaths(IUserDataPathsVtbl): IInspectable(IInspectableVtbl) [IID_IUserDataPaths] {
    fn get_CameraRoll(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Cookies(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Desktop(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Documents(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Downloads(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Favorites(&self, out: *mut HSTRING) -> HRESULT,
    fn get_History(&self, out: *mut HSTRING) -> HRESULT,
    fn get_InternetCache(&self, out: *mut HSTRING) -> HRESULT,
    fn get_LocalAppData(&self, out: *mut HSTRING) -> HRESULT,
    fn get_LocalAppDataLow(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Music(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Pictures(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Profile(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Recent(&self, out: *mut HSTRING) -> HRESULT,
    fn get_RoamingAppData(&self, out: *mut HSTRING) -> HRESULT,
    fn get_SavedPictures(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Screenshots(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Templates(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Videos(&self, out: *mut HSTRING) -> HRESULT
}}
impl IUserDataPaths {
    #[inline] pub unsafe fn get_camera_roll(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_CameraRoll)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_cookies(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Cookies)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_desktop(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Desktop)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_documents(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Documents)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_downloads(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Downloads)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_favorites(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Favorites)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_history(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_History)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_internet_cache(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_InternetCache)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_local_app_data(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_LocalAppData)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_local_app_data_low(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_LocalAppDataLow)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_music(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Music)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_pictures(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Pictures)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_profile(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Profile)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_recent(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Recent)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_roaming_app_data(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_RoamingAppData)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_saved_pictures(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_SavedPictures)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_screenshots(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Screenshots)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_templates(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Templates)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_videos(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Videos)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
}
RT_CLASS!{class UserDataPaths: IUserDataPaths}
impl RtActivatable<IUserDataPathsStatics> for UserDataPaths {}
impl UserDataPaths {
    #[cfg(feature="windows-system")] #[inline] pub fn get_for_user(user: &super::system::User) -> Result<ComPtr<UserDataPaths>> { unsafe {
        <Self as RtActivatable<IUserDataPathsStatics>>::get_activation_factory().get_for_user(user)
    }}
    #[inline] pub fn get_default() -> Result<ComPtr<UserDataPaths>> { unsafe {
        <Self as RtActivatable<IUserDataPathsStatics>>::get_activation_factory().get_default()
    }}
}
DEFINE_CLSID!(UserDataPaths(&[87,105,110,100,111,119,115,46,83,116,111,114,97,103,101,46,85,115,101,114,68,97,116,97,80,97,116,104,115,0]) [CLSID_UserDataPaths]);
DEFINE_IID!(IID_IUserDataPathsStatics, 28483055, 57442, 18593, 139, 12, 242, 199, 169, 202, 86, 192);
RT_INTERFACE!{static interface IUserDataPathsStatics(IUserDataPathsStaticsVtbl): IInspectable(IInspectableVtbl) [IID_IUserDataPathsStatics] {
    #[cfg(not(feature="windows-system"))] fn __Dummy0(&self) -> (),
    #[cfg(feature="windows-system")] fn GetForUser(&self, user: *mut super::system::User, out: *mut *mut UserDataPaths) -> HRESULT,
    fn GetDefault(&self, out: *mut *mut UserDataPaths) -> HRESULT
}}
impl IUserDataPathsStatics {
    #[cfg(feature="windows-system")] #[inline] pub unsafe fn get_for_user(&self, user: &super::system::User) -> Result<ComPtr<UserDataPaths>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetForUser)(self as *const _ as *mut _, user as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_default(&self) -> Result<ComPtr<UserDataPaths>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetDefault)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
pub mod streams { // Windows.Storage.Streams
use ::prelude::*;
DEFINE_IID!(IID_IBuffer, 2421821408, 48211, 4575, 140, 73, 0, 30, 79, 198, 134, 218);
RT_INTERFACE!{interface IBuffer(IBufferVtbl): IInspectable(IInspectableVtbl) [IID_IBuffer] {
    fn get_Capacity(&self, out: *mut u32) -> HRESULT,
    fn get_Length(&self, out: *mut u32) -> HRESULT,
    fn put_Length(&self, value: u32) -> HRESULT
}}
impl IBuffer {
    #[inline] pub unsafe fn get_capacity(&self) -> Result<u32> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_Capacity)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_length(&self) -> Result<u32> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_Length)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_length(&self, value: u32) -> Result<()> {
        let hr = ((*self.lpVtbl).put_Length)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
}
RT_CLASS!{class Buffer: IBuffer}
impl RtActivatable<IBufferFactory> for Buffer {}
impl RtActivatable<IBufferStatics> for Buffer {}
impl Buffer {
    #[inline] pub fn create(capacity: u32) -> Result<ComPtr<Buffer>> { unsafe {
        <Self as RtActivatable<IBufferFactory>>::get_activation_factory().create(capacity)
    }}
    #[inline] pub fn create_copy_from_memory_buffer(input: &super::super::foundation::IMemoryBuffer) -> Result<ComPtr<Buffer>> { unsafe {
        <Self as RtActivatable<IBufferStatics>>::get_activation_factory().create_copy_from_memory_buffer(input)
    }}
    #[inline] pub fn create_memory_buffer_over_ibuffer(input: &IBuffer) -> Result<ComPtr<super::super::foundation::MemoryBuffer>> { unsafe {
        <Self as RtActivatable<IBufferStatics>>::get_activation_factory().create_memory_buffer_over_ibuffer(input)
    }}
}
DEFINE_CLSID!(Buffer(&[87,105,110,100,111,119,115,46,83,116,111,114,97,103,101,46,83,116,114,101,97,109,115,46,66,117,102,102,101,114,0]) [CLSID_Buffer]);
DEFINE_IID!(IID_IBufferFactory, 1907331405, 49423, 18507, 188, 80, 20, 188, 98, 59, 58, 39);
RT_INTERFACE!{static interface IBufferFactory(IBufferFactoryVtbl): IInspectable(IInspectableVtbl) [IID_IBufferFactory] {
    fn Create(&self, capacity: u32, out: *mut *mut Buffer) -> HRESULT
}}
impl IBufferFactory {
    #[inline] pub unsafe fn create(&self, capacity: u32) -> Result<ComPtr<Buffer>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).Create)(self as *const _ as *mut _, capacity, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
DEFINE_IID!(IID_IBufferStatics, 3909215835, 55062, 18266, 169, 10, 175, 114, 41, 177, 231, 65);
RT_INTERFACE!{static interface IBufferStatics(IBufferStaticsVtbl): IInspectable(IInspectableVtbl) [IID_IBufferStatics] {
    fn CreateCopyFromMemoryBuffer(&self, input: *mut super::super::foundation::IMemoryBuffer, out: *mut *mut Buffer) -> HRESULT,
    fn CreateMemoryBufferOverIBuffer(&self, input: *mut IBuffer, out: *mut *mut super::super::foundation::MemoryBuffer) -> HRESULT
}}
impl IBufferStatics {
    #[inline] pub unsafe fn create_copy_from_memory_buffer(&self, input: &super::super::foundation::IMemoryBuffer) -> Result<ComPtr<Buffer>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).CreateCopyFromMemoryBuffer)(self as *const _ as *mut _, input as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn create_memory_buffer_over_ibuffer(&self, input: &IBuffer) -> Result<ComPtr<super::super::foundation::MemoryBuffer>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).CreateMemoryBufferOverIBuffer)(self as *const _ as *mut _, input as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
RT_ENUM! { enum ByteOrder: i32 {
    LittleEndian (ByteOrder_LittleEndian) = 0, BigEndian (ByteOrder_BigEndian) = 1,
}}
DEFINE_IID!(IID_IContentTypeProvider, 2547030181, 15257, 19945, 136, 165, 225, 29, 47, 80, 199, 149);
RT_INTERFACE!{interface IContentTypeProvider(IContentTypeProviderVtbl): IInspectable(IInspectableVtbl) [IID_IContentTypeProvider] {
    fn get_ContentType(&self, out: *mut HSTRING) -> HRESULT
}}
impl IContentTypeProvider {
    #[inline] pub unsafe fn get_content_type(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_ContentType)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
}
DEFINE_IID!(IID_IDataReader, 3803512873, 46273, 17172, 164, 184, 251, 129, 58, 47, 39, 94);
RT_INTERFACE!{interface IDataReader(IDataReaderVtbl): IInspectable(IInspectableVtbl) [IID_IDataReader] {
    fn get_UnconsumedBufferLength(&self, out: *mut u32) -> HRESULT,
    fn get_UnicodeEncoding(&self, out: *mut UnicodeEncoding) -> HRESULT,
    fn put_UnicodeEncoding(&self, value: UnicodeEncoding) -> HRESULT,
    fn get_ByteOrder(&self, out: *mut ByteOrder) -> HRESULT,
    fn put_ByteOrder(&self, value: ByteOrder) -> HRESULT,
    fn get_InputStreamOptions(&self, out: *mut InputStreamOptions) -> HRESULT,
    fn put_InputStreamOptions(&self, value: InputStreamOptions) -> HRESULT,
    fn ReadByte(&self, out: *mut u8) -> HRESULT,
    fn ReadBytes(&self, valueSize: u32, value: *mut u8) -> HRESULT,
    fn ReadBuffer(&self, length: u32, out: *mut *mut IBuffer) -> HRESULT,
    fn ReadBoolean(&self, out: *mut bool) -> HRESULT,
    fn ReadGuid(&self, out: *mut Guid) -> HRESULT,
    fn ReadInt16(&self, out: *mut i16) -> HRESULT,
    fn ReadInt32(&self, out: *mut i32) -> HRESULT,
    fn ReadInt64(&self, out: *mut i64) -> HRESULT,
    fn ReadUInt16(&self, out: *mut u16) -> HRESULT,
    fn ReadUInt32(&self, out: *mut u32) -> HRESULT,
    fn ReadUInt64(&self, out: *mut u64) -> HRESULT,
    fn ReadSingle(&self, out: *mut f32) -> HRESULT,
    fn ReadDouble(&self, out: *mut f64) -> HRESULT,
    fn ReadString(&self, codeUnitCount: u32, out: *mut HSTRING) -> HRESULT,
    fn ReadDateTime(&self, out: *mut super::super::foundation::DateTime) -> HRESULT,
    fn ReadTimeSpan(&self, out: *mut super::super::foundation::TimeSpan) -> HRESULT,
    fn LoadAsync(&self, count: u32, out: *mut *mut DataReaderLoadOperation) -> HRESULT,
    fn DetachBuffer(&self, out: *mut *mut IBuffer) -> HRESULT,
    fn DetachStream(&self, out: *mut *mut IInputStream) -> HRESULT
}}
impl IDataReader {
    #[inline] pub unsafe fn get_unconsumed_buffer_length(&self) -> Result<u32> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_UnconsumedBufferLength)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_unicode_encoding(&self) -> Result<UnicodeEncoding> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_UnicodeEncoding)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_unicode_encoding(&self, value: UnicodeEncoding) -> Result<()> {
        let hr = ((*self.lpVtbl).put_UnicodeEncoding)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_byte_order(&self) -> Result<ByteOrder> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_ByteOrder)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_byte_order(&self, value: ByteOrder) -> Result<()> {
        let hr = ((*self.lpVtbl).put_ByteOrder)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_input_stream_options(&self) -> Result<InputStreamOptions> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_InputStreamOptions)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_input_stream_options(&self, value: InputStreamOptions) -> Result<()> {
        let hr = ((*self.lpVtbl).put_InputStreamOptions)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn read_byte(&self) -> Result<u8> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).ReadByte)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn read_bytes(&self, value: &mut [u8]) -> Result<()> {
        let hr = ((*self.lpVtbl).ReadBytes)(self as *const _ as *mut _, value.len() as u32, value.as_mut_ptr() as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn read_buffer(&self, length: u32) -> Result<ComPtr<IBuffer>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).ReadBuffer)(self as *const _ as *mut _, length, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn read_boolean(&self) -> Result<bool> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).ReadBoolean)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn read_guid(&self) -> Result<Guid> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).ReadGuid)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn read_int16(&self) -> Result<i16> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).ReadInt16)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn read_int32(&self) -> Result<i32> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).ReadInt32)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn read_int64(&self) -> Result<i64> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).ReadInt64)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn read_uint16(&self) -> Result<u16> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).ReadUInt16)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn read_uint32(&self) -> Result<u32> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).ReadUInt32)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn read_uint64(&self) -> Result<u64> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).ReadUInt64)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn read_single(&self) -> Result<f32> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).ReadSingle)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn read_double(&self) -> Result<f64> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).ReadDouble)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn read_string(&self, codeUnitCount: u32) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).ReadString)(self as *const _ as *mut _, codeUnitCount, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn read_date_time(&self) -> Result<super::super::foundation::DateTime> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).ReadDateTime)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn read_time_span(&self) -> Result<super::super::foundation::TimeSpan> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).ReadTimeSpan)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn load_async(&self, count: u32) -> Result<ComPtr<DataReaderLoadOperation>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).LoadAsync)(self as *const _ as *mut _, count, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn detach_buffer(&self) -> Result<ComPtr<IBuffer>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).DetachBuffer)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn detach_stream(&self) -> Result<ComPtr<IInputStream>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).DetachStream)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
RT_CLASS!{class DataReader: IDataReader}
impl RtActivatable<IDataReaderFactory> for DataReader {}
impl RtActivatable<IDataReaderStatics> for DataReader {}
impl DataReader {
    #[inline] pub fn create_data_reader(inputStream: &IInputStream) -> Result<ComPtr<DataReader>> { unsafe {
        <Self as RtActivatable<IDataReaderFactory>>::get_activation_factory().create_data_reader(inputStream)
    }}
    #[inline] pub fn from_buffer(buffer: &IBuffer) -> Result<ComPtr<DataReader>> { unsafe {
        <Self as RtActivatable<IDataReaderStatics>>::get_activation_factory().from_buffer(buffer)
    }}
}
DEFINE_CLSID!(DataReader(&[87,105,110,100,111,119,115,46,83,116,111,114,97,103,101,46,83,116,114,101,97,109,115,46,68,97,116,97,82,101,97,100,101,114,0]) [CLSID_DataReader]);
DEFINE_IID!(IID_IDataReaderFactory, 3612506183, 22490, 19989, 145, 76, 6, 128, 102, 153, 160, 152);
RT_INTERFACE!{static interface IDataReaderFactory(IDataReaderFactoryVtbl): IInspectable(IInspectableVtbl) [IID_IDataReaderFactory] {
    fn CreateDataReader(&self, inputStream: *mut IInputStream, out: *mut *mut DataReader) -> HRESULT
}}
impl IDataReaderFactory {
    #[inline] pub unsafe fn create_data_reader(&self, inputStream: &IInputStream) -> Result<ComPtr<DataReader>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).CreateDataReader)(self as *const _ as *mut _, inputStream as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
RT_CLASS!{class DataReaderLoadOperation: super::super::foundation::IAsyncOperation<u32>}
DEFINE_IID!(IID_IDataReaderStatics, 301776840, 63802, 18203, 177, 33, 243, 121, 227, 73, 49, 60);
RT_INTERFACE!{static interface IDataReaderStatics(IDataReaderStaticsVtbl): IInspectable(IInspectableVtbl) [IID_IDataReaderStatics] {
    fn FromBuffer(&self, buffer: *mut IBuffer, out: *mut *mut DataReader) -> HRESULT
}}
impl IDataReaderStatics {
    #[inline] pub unsafe fn from_buffer(&self, buffer: &IBuffer) -> Result<ComPtr<DataReader>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).FromBuffer)(self as *const _ as *mut _, buffer as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
DEFINE_IID!(IID_IDataWriter, 1689817701, 54081, 18722, 179, 138, 221, 74, 248, 128, 140, 78);
RT_INTERFACE!{interface IDataWriter(IDataWriterVtbl): IInspectable(IInspectableVtbl) [IID_IDataWriter] {
    fn get_UnstoredBufferLength(&self, out: *mut u32) -> HRESULT,
    fn get_UnicodeEncoding(&self, out: *mut UnicodeEncoding) -> HRESULT,
    fn put_UnicodeEncoding(&self, value: UnicodeEncoding) -> HRESULT,
    fn get_ByteOrder(&self, out: *mut ByteOrder) -> HRESULT,
    fn put_ByteOrder(&self, value: ByteOrder) -> HRESULT,
    fn WriteByte(&self, value: u8) -> HRESULT,
    fn WriteBytes(&self, valueSize: u32, value: *mut u8) -> HRESULT,
    fn WriteBuffer(&self, buffer: *mut IBuffer) -> HRESULT,
    fn WriteBufferRange(&self, buffer: *mut IBuffer, start: u32, count: u32) -> HRESULT,
    fn WriteBoolean(&self, value: bool) -> HRESULT,
    fn WriteGuid(&self, value: Guid) -> HRESULT,
    fn WriteInt16(&self, value: i16) -> HRESULT,
    fn WriteInt32(&self, value: i32) -> HRESULT,
    fn WriteInt64(&self, value: i64) -> HRESULT,
    fn WriteUInt16(&self, value: u16) -> HRESULT,
    fn WriteUInt32(&self, value: u32) -> HRESULT,
    fn WriteUInt64(&self, value: u64) -> HRESULT,
    fn WriteSingle(&self, value: f32) -> HRESULT,
    fn WriteDouble(&self, value: f64) -> HRESULT,
    fn WriteDateTime(&self, value: super::super::foundation::DateTime) -> HRESULT,
    fn WriteTimeSpan(&self, value: super::super::foundation::TimeSpan) -> HRESULT,
    fn WriteString(&self, value: HSTRING, out: *mut u32) -> HRESULT,
    fn MeasureString(&self, value: HSTRING, out: *mut u32) -> HRESULT,
    fn StoreAsync(&self, out: *mut *mut DataWriterStoreOperation) -> HRESULT,
    fn FlushAsync(&self, out: *mut *mut super::super::foundation::IAsyncOperation<bool>) -> HRESULT,
    fn DetachBuffer(&self, out: *mut *mut IBuffer) -> HRESULT,
    fn DetachStream(&self, out: *mut *mut IOutputStream) -> HRESULT
}}
impl IDataWriter {
    #[inline] pub unsafe fn get_unstored_buffer_length(&self) -> Result<u32> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_UnstoredBufferLength)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_unicode_encoding(&self) -> Result<UnicodeEncoding> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_UnicodeEncoding)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_unicode_encoding(&self, value: UnicodeEncoding) -> Result<()> {
        let hr = ((*self.lpVtbl).put_UnicodeEncoding)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_byte_order(&self) -> Result<ByteOrder> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_ByteOrder)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_byte_order(&self, value: ByteOrder) -> Result<()> {
        let hr = ((*self.lpVtbl).put_ByteOrder)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn write_byte(&self, value: u8) -> Result<()> {
        let hr = ((*self.lpVtbl).WriteByte)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn write_bytes(&self, value: &[u8]) -> Result<()> {
        let hr = ((*self.lpVtbl).WriteBytes)(self as *const _ as *mut _, value.len() as u32, value.as_ptr() as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn write_buffer(&self, buffer: &IBuffer) -> Result<()> {
        let hr = ((*self.lpVtbl).WriteBuffer)(self as *const _ as *mut _, buffer as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn write_buffer_range(&self, buffer: &IBuffer, start: u32, count: u32) -> Result<()> {
        let hr = ((*self.lpVtbl).WriteBufferRange)(self as *const _ as *mut _, buffer as *const _ as *mut _, start, count);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn write_boolean(&self, value: bool) -> Result<()> {
        let hr = ((*self.lpVtbl).WriteBoolean)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn write_guid(&self, value: Guid) -> Result<()> {
        let hr = ((*self.lpVtbl).WriteGuid)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn write_int16(&self, value: i16) -> Result<()> {
        let hr = ((*self.lpVtbl).WriteInt16)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn write_int32(&self, value: i32) -> Result<()> {
        let hr = ((*self.lpVtbl).WriteInt32)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn write_int64(&self, value: i64) -> Result<()> {
        let hr = ((*self.lpVtbl).WriteInt64)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn write_uint16(&self, value: u16) -> Result<()> {
        let hr = ((*self.lpVtbl).WriteUInt16)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn write_uint32(&self, value: u32) -> Result<()> {
        let hr = ((*self.lpVtbl).WriteUInt32)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn write_uint64(&self, value: u64) -> Result<()> {
        let hr = ((*self.lpVtbl).WriteUInt64)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn write_single(&self, value: f32) -> Result<()> {
        let hr = ((*self.lpVtbl).WriteSingle)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn write_double(&self, value: f64) -> Result<()> {
        let hr = ((*self.lpVtbl).WriteDouble)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn write_date_time(&self, value: super::super::foundation::DateTime) -> Result<()> {
        let hr = ((*self.lpVtbl).WriteDateTime)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn write_time_span(&self, value: super::super::foundation::TimeSpan) -> Result<()> {
        let hr = ((*self.lpVtbl).WriteTimeSpan)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn write_string(&self, value: &HStringArg) -> Result<u32> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).WriteString)(self as *const _ as *mut _, value.get(), &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn measure_string(&self, value: &HStringArg) -> Result<u32> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).MeasureString)(self as *const _ as *mut _, value.get(), &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn store_async(&self) -> Result<ComPtr<DataWriterStoreOperation>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).StoreAsync)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn flush_async(&self) -> Result<ComPtr<super::super::foundation::IAsyncOperation<bool>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).FlushAsync)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn detach_buffer(&self) -> Result<ComPtr<IBuffer>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).DetachBuffer)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn detach_stream(&self) -> Result<ComPtr<IOutputStream>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).DetachStream)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
RT_CLASS!{class DataWriter: IDataWriter}
impl RtActivatable<IDataWriterFactory> for DataWriter {}
impl RtActivatable<IActivationFactory> for DataWriter {}
impl DataWriter {
    #[inline] pub fn create_data_writer(outputStream: &IOutputStream) -> Result<ComPtr<DataWriter>> { unsafe {
        <Self as RtActivatable<IDataWriterFactory>>::get_activation_factory().create_data_writer(outputStream)
    }}
}
DEFINE_CLSID!(DataWriter(&[87,105,110,100,111,119,115,46,83,116,111,114,97,103,101,46,83,116,114,101,97,109,115,46,68,97,116,97,87,114,105,116,101,114,0]) [CLSID_DataWriter]);
DEFINE_IID!(IID_IDataWriterFactory, 864839618, 35716, 19499, 156, 80, 123, 135, 103, 132, 122, 31);
RT_INTERFACE!{static interface IDataWriterFactory(IDataWriterFactoryVtbl): IInspectable(IInspectableVtbl) [IID_IDataWriterFactory] {
    fn CreateDataWriter(&self, outputStream: *mut IOutputStream, out: *mut *mut DataWriter) -> HRESULT
}}
impl IDataWriterFactory {
    #[inline] pub unsafe fn create_data_writer(&self, outputStream: &IOutputStream) -> Result<ComPtr<DataWriter>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).CreateDataWriter)(self as *const _ as *mut _, outputStream as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
RT_CLASS!{class DataWriterStoreOperation: super::super::foundation::IAsyncOperation<u32>}
RT_CLASS!{class FileInputStream: IInputStream}
RT_ENUM! { enum FileOpenDisposition: i32 {
    OpenExisting (FileOpenDisposition_OpenExisting) = 0, OpenAlways (FileOpenDisposition_OpenAlways) = 1, CreateNew (FileOpenDisposition_CreateNew) = 2, CreateAlways (FileOpenDisposition_CreateAlways) = 3, TruncateExisting (FileOpenDisposition_TruncateExisting) = 4,
}}
RT_CLASS!{class FileOutputStream: IOutputStream}
RT_CLASS!{class FileRandomAccessStream: IRandomAccessStream}
impl RtActivatable<IFileRandomAccessStreamStatics> for FileRandomAccessStream {}
impl FileRandomAccessStream {
    #[inline] pub fn open_async(filePath: &HStringArg, accessMode: super::FileAccessMode) -> Result<ComPtr<super::super::foundation::IAsyncOperation<IRandomAccessStream>>> { unsafe {
        <Self as RtActivatable<IFileRandomAccessStreamStatics>>::get_activation_factory().open_async(filePath, accessMode)
    }}
    #[inline] pub fn open_with_options_async(filePath: &HStringArg, accessMode: super::FileAccessMode, sharingOptions: super::StorageOpenOptions, openDisposition: FileOpenDisposition) -> Result<ComPtr<super::super::foundation::IAsyncOperation<IRandomAccessStream>>> { unsafe {
        <Self as RtActivatable<IFileRandomAccessStreamStatics>>::get_activation_factory().open_with_options_async(filePath, accessMode, sharingOptions, openDisposition)
    }}
    #[inline] pub fn open_transacted_write_async(filePath: &HStringArg) -> Result<ComPtr<super::super::foundation::IAsyncOperation<super::StorageStreamTransaction>>> { unsafe {
        <Self as RtActivatable<IFileRandomAccessStreamStatics>>::get_activation_factory().open_transacted_write_async(filePath)
    }}
    #[inline] pub fn open_transacted_write_with_options_async(filePath: &HStringArg, openOptions: super::StorageOpenOptions, openDisposition: FileOpenDisposition) -> Result<ComPtr<super::super::foundation::IAsyncOperation<super::StorageStreamTransaction>>> { unsafe {
        <Self as RtActivatable<IFileRandomAccessStreamStatics>>::get_activation_factory().open_transacted_write_with_options_async(filePath, openOptions, openDisposition)
    }}
    #[cfg(feature="windows-system")] #[inline] pub fn open_for_user_async(user: &super::super::system::User, filePath: &HStringArg, accessMode: super::FileAccessMode) -> Result<ComPtr<super::super::foundation::IAsyncOperation<IRandomAccessStream>>> { unsafe {
        <Self as RtActivatable<IFileRandomAccessStreamStatics>>::get_activation_factory().open_for_user_async(user, filePath, accessMode)
    }}
    #[cfg(feature="windows-system")] #[inline] pub fn open_for_user_with_options_async(user: &super::super::system::User, filePath: &HStringArg, accessMode: super::FileAccessMode, sharingOptions: super::StorageOpenOptions, openDisposition: FileOpenDisposition) -> Result<ComPtr<super::super::foundation::IAsyncOperation<IRandomAccessStream>>> { unsafe {
        <Self as RtActivatable<IFileRandomAccessStreamStatics>>::get_activation_factory().open_for_user_with_options_async(user, filePath, accessMode, sharingOptions, openDisposition)
    }}
    #[cfg(feature="windows-system")] #[inline] pub fn open_transacted_write_for_user_async(user: &super::super::system::User, filePath: &HStringArg) -> Result<ComPtr<super::super::foundation::IAsyncOperation<super::StorageStreamTransaction>>> { unsafe {
        <Self as RtActivatable<IFileRandomAccessStreamStatics>>::get_activation_factory().open_transacted_write_for_user_async(user, filePath)
    }}
    #[cfg(feature="windows-system")] #[inline] pub fn open_transacted_write_for_user_with_options_async(user: &super::super::system::User, filePath: &HStringArg, openOptions: super::StorageOpenOptions, openDisposition: FileOpenDisposition) -> Result<ComPtr<super::super::foundation::IAsyncOperation<super::StorageStreamTransaction>>> { unsafe {
        <Self as RtActivatable<IFileRandomAccessStreamStatics>>::get_activation_factory().open_transacted_write_for_user_with_options_async(user, filePath, openOptions, openDisposition)
    }}
}
DEFINE_CLSID!(FileRandomAccessStream(&[87,105,110,100,111,119,115,46,83,116,111,114,97,103,101,46,83,116,114,101,97,109,115,46,70,105,108,101,82,97,110,100,111,109,65,99,99,101,115,115,83,116,114,101,97,109,0]) [CLSID_FileRandomAccessStream]);
DEFINE_IID!(IID_IFileRandomAccessStreamStatics, 1934950663, 15191, 19293, 131, 69, 85, 77, 47, 198, 33, 240);
RT_INTERFACE!{static interface IFileRandomAccessStreamStatics(IFileRandomAccessStreamStaticsVtbl): IInspectable(IInspectableVtbl) [IID_IFileRandomAccessStreamStatics] {
    fn OpenAsync(&self, filePath: HSTRING, accessMode: super::FileAccessMode, out: *mut *mut super::super::foundation::IAsyncOperation<IRandomAccessStream>) -> HRESULT,
    fn OpenWithOptionsAsync(&self, filePath: HSTRING, accessMode: super::FileAccessMode, sharingOptions: super::StorageOpenOptions, openDisposition: FileOpenDisposition, out: *mut *mut super::super::foundation::IAsyncOperation<IRandomAccessStream>) -> HRESULT,
    fn OpenTransactedWriteAsync(&self, filePath: HSTRING, out: *mut *mut super::super::foundation::IAsyncOperation<super::StorageStreamTransaction>) -> HRESULT,
    fn OpenTransactedWriteWithOptionsAsync(&self, filePath: HSTRING, openOptions: super::StorageOpenOptions, openDisposition: FileOpenDisposition, out: *mut *mut super::super::foundation::IAsyncOperation<super::StorageStreamTransaction>) -> HRESULT,
    #[cfg(feature="windows-system")] fn OpenForUserAsync(&self, user: *mut super::super::system::User, filePath: HSTRING, accessMode: super::FileAccessMode, out: *mut *mut super::super::foundation::IAsyncOperation<IRandomAccessStream>) -> HRESULT,
    #[cfg(feature="windows-system")] fn OpenForUserWithOptionsAsync(&self, user: *mut super::super::system::User, filePath: HSTRING, accessMode: super::FileAccessMode, sharingOptions: super::StorageOpenOptions, openDisposition: FileOpenDisposition, out: *mut *mut super::super::foundation::IAsyncOperation<IRandomAccessStream>) -> HRESULT,
    #[cfg(feature="windows-system")] fn OpenTransactedWriteForUserAsync(&self, user: *mut super::super::system::User, filePath: HSTRING, out: *mut *mut super::super::foundation::IAsyncOperation<super::StorageStreamTransaction>) -> HRESULT,
    #[cfg(feature="windows-system")] fn OpenTransactedWriteForUserWithOptionsAsync(&self, user: *mut super::super::system::User, filePath: HSTRING, openOptions: super::StorageOpenOptions, openDisposition: FileOpenDisposition, out: *mut *mut super::super::foundation::IAsyncOperation<super::StorageStreamTransaction>) -> HRESULT
}}
impl IFileRandomAccessStreamStatics {
    #[inline] pub unsafe fn open_async(&self, filePath: &HStringArg, accessMode: super::FileAccessMode) -> Result<ComPtr<super::super::foundation::IAsyncOperation<IRandomAccessStream>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).OpenAsync)(self as *const _ as *mut _, filePath.get(), accessMode, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn open_with_options_async(&self, filePath: &HStringArg, accessMode: super::FileAccessMode, sharingOptions: super::StorageOpenOptions, openDisposition: FileOpenDisposition) -> Result<ComPtr<super::super::foundation::IAsyncOperation<IRandomAccessStream>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).OpenWithOptionsAsync)(self as *const _ as *mut _, filePath.get(), accessMode, sharingOptions, openDisposition, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn open_transacted_write_async(&self, filePath: &HStringArg) -> Result<ComPtr<super::super::foundation::IAsyncOperation<super::StorageStreamTransaction>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).OpenTransactedWriteAsync)(self as *const _ as *mut _, filePath.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn open_transacted_write_with_options_async(&self, filePath: &HStringArg, openOptions: super::StorageOpenOptions, openDisposition: FileOpenDisposition) -> Result<ComPtr<super::super::foundation::IAsyncOperation<super::StorageStreamTransaction>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).OpenTransactedWriteWithOptionsAsync)(self as *const _ as *mut _, filePath.get(), openOptions, openDisposition, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[cfg(feature="windows-system")] #[inline] pub unsafe fn open_for_user_async(&self, user: &super::super::system::User, filePath: &HStringArg, accessMode: super::FileAccessMode) -> Result<ComPtr<super::super::foundation::IAsyncOperation<IRandomAccessStream>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).OpenForUserAsync)(self as *const _ as *mut _, user as *const _ as *mut _, filePath.get(), accessMode, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[cfg(feature="windows-system")] #[inline] pub unsafe fn open_for_user_with_options_async(&self, user: &super::super::system::User, filePath: &HStringArg, accessMode: super::FileAccessMode, sharingOptions: super::StorageOpenOptions, openDisposition: FileOpenDisposition) -> Result<ComPtr<super::super::foundation::IAsyncOperation<IRandomAccessStream>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).OpenForUserWithOptionsAsync)(self as *const _ as *mut _, user as *const _ as *mut _, filePath.get(), accessMode, sharingOptions, openDisposition, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[cfg(feature="windows-system")] #[inline] pub unsafe fn open_transacted_write_for_user_async(&self, user: &super::super::system::User, filePath: &HStringArg) -> Result<ComPtr<super::super::foundation::IAsyncOperation<super::StorageStreamTransaction>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).OpenTransactedWriteForUserAsync)(self as *const _ as *mut _, user as *const _ as *mut _, filePath.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[cfg(feature="windows-system")] #[inline] pub unsafe fn open_transacted_write_for_user_with_options_async(&self, user: &super::super::system::User, filePath: &HStringArg, openOptions: super::StorageOpenOptions, openDisposition: FileOpenDisposition) -> Result<ComPtr<super::super::foundation::IAsyncOperation<super::StorageStreamTransaction>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).OpenTransactedWriteForUserWithOptionsAsync)(self as *const _ as *mut _, user as *const _ as *mut _, filePath.get(), openOptions, openDisposition, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
RT_CLASS!{class InMemoryRandomAccessStream: IRandomAccessStream}
impl RtActivatable<IActivationFactory> for InMemoryRandomAccessStream {}
DEFINE_CLSID!(InMemoryRandomAccessStream(&[87,105,110,100,111,119,115,46,83,116,111,114,97,103,101,46,83,116,114,101,97,109,115,46,73,110,77,101,109,111,114,121,82,97,110,100,111,109,65,99,99,101,115,115,83,116,114,101,97,109,0]) [CLSID_InMemoryRandomAccessStream]);
DEFINE_IID!(IID_IInputStream, 2421821410, 48211, 4575, 140, 73, 0, 30, 79, 198, 134, 218);
RT_INTERFACE!{interface IInputStream(IInputStreamVtbl): IInspectable(IInspectableVtbl) [IID_IInputStream] {
    fn ReadAsync(&self, buffer: *mut IBuffer, count: u32, options: InputStreamOptions, out: *mut *mut super::super::foundation::IAsyncOperationWithProgress<IBuffer, u32>) -> HRESULT
}}
impl IInputStream {
    #[inline] pub unsafe fn read_async(&self, buffer: &IBuffer, count: u32, options: InputStreamOptions) -> Result<ComPtr<super::super::foundation::IAsyncOperationWithProgress<IBuffer, u32>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).ReadAsync)(self as *const _ as *mut _, buffer as *const _ as *mut _, count, options, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
RT_ENUM! { enum InputStreamOptions: u32 {
    None (InputStreamOptions_None) = 0, Partial (InputStreamOptions_Partial) = 1, ReadAhead (InputStreamOptions_ReadAhead) = 2,
}}
RT_CLASS!{class InputStreamOverStream: IInputStream}
DEFINE_IID!(IID_IInputStreamReference, 1133681944, 24265, 19290, 145, 156, 66, 5, 176, 200, 4, 182);
RT_INTERFACE!{interface IInputStreamReference(IInputStreamReferenceVtbl): IInspectable(IInspectableVtbl) [IID_IInputStreamReference] {
    fn OpenSequentialReadAsync(&self, out: *mut *mut super::super::foundation::IAsyncOperation<IInputStream>) -> HRESULT
}}
impl IInputStreamReference {
    #[inline] pub unsafe fn open_sequential_read_async(&self) -> Result<ComPtr<super::super::foundation::IAsyncOperation<IInputStream>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).OpenSequentialReadAsync)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
DEFINE_IID!(IID_IOutputStream, 2421821414, 48211, 4575, 140, 73, 0, 30, 79, 198, 134, 218);
RT_INTERFACE!{interface IOutputStream(IOutputStreamVtbl): IInspectable(IInspectableVtbl) [IID_IOutputStream] {
    fn WriteAsync(&self, buffer: *mut IBuffer, out: *mut *mut super::super::foundation::IAsyncOperationWithProgress<u32, u32>) -> HRESULT,
    fn FlushAsync(&self, out: *mut *mut super::super::foundation::IAsyncOperation<bool>) -> HRESULT
}}
impl IOutputStream {
    #[inline] pub unsafe fn write_async(&self, buffer: &IBuffer) -> Result<ComPtr<super::super::foundation::IAsyncOperationWithProgress<u32, u32>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).WriteAsync)(self as *const _ as *mut _, buffer as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn flush_async(&self) -> Result<ComPtr<super::super::foundation::IAsyncOperation<bool>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).FlushAsync)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
RT_CLASS!{class OutputStreamOverStream: IOutputStream}
DEFINE_IID!(IID_IRandomAccessStream, 2421821409, 48211, 4575, 140, 73, 0, 30, 79, 198, 134, 218);
RT_INTERFACE!{interface IRandomAccessStream(IRandomAccessStreamVtbl): IInspectable(IInspectableVtbl) [IID_IRandomAccessStream] {
    fn get_Size(&self, out: *mut u64) -> HRESULT,
    fn put_Size(&self, value: u64) -> HRESULT,
    fn GetInputStreamAt(&self, position: u64, out: *mut *mut IInputStream) -> HRESULT,
    fn GetOutputStreamAt(&self, position: u64, out: *mut *mut IOutputStream) -> HRESULT,
    fn get_Position(&self, out: *mut u64) -> HRESULT,
    fn Seek(&self, position: u64) -> HRESULT,
    fn CloneStream(&self, out: *mut *mut IRandomAccessStream) -> HRESULT,
    fn get_CanRead(&self, out: *mut bool) -> HRESULT,
    fn get_CanWrite(&self, out: *mut bool) -> HRESULT
}}
impl IRandomAccessStream {
    #[inline] pub unsafe fn get_size(&self) -> Result<u64> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_Size)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_size(&self, value: u64) -> Result<()> {
        let hr = ((*self.lpVtbl).put_Size)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_input_stream_at(&self, position: u64) -> Result<ComPtr<IInputStream>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetInputStreamAt)(self as *const _ as *mut _, position, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_output_stream_at(&self, position: u64) -> Result<ComPtr<IOutputStream>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetOutputStreamAt)(self as *const _ as *mut _, position, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_position(&self) -> Result<u64> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_Position)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn seek(&self, position: u64) -> Result<()> {
        let hr = ((*self.lpVtbl).Seek)(self as *const _ as *mut _, position);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn clone_stream(&self) -> Result<ComPtr<IRandomAccessStream>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).CloneStream)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_can_read(&self) -> Result<bool> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_CanRead)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_can_write(&self) -> Result<bool> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_CanWrite)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
}
RT_CLASS!{static class RandomAccessStream}
impl RtActivatable<IRandomAccessStreamStatics> for RandomAccessStream {}
impl RandomAccessStream {
    #[inline] pub fn copy_async(source: &IInputStream, destination: &IOutputStream) -> Result<ComPtr<super::super::foundation::IAsyncOperationWithProgress<u64, u64>>> { unsafe {
        <Self as RtActivatable<IRandomAccessStreamStatics>>::get_activation_factory().copy_async(source, destination)
    }}
    #[inline] pub fn copy_size_async(source: &IInputStream, destination: &IOutputStream, bytesToCopy: u64) -> Result<ComPtr<super::super::foundation::IAsyncOperationWithProgress<u64, u64>>> { unsafe {
        <Self as RtActivatable<IRandomAccessStreamStatics>>::get_activation_factory().copy_size_async(source, destination, bytesToCopy)
    }}
    #[inline] pub fn copy_and_close_async(source: &IInputStream, destination: &IOutputStream) -> Result<ComPtr<super::super::foundation::IAsyncOperationWithProgress<u64, u64>>> { unsafe {
        <Self as RtActivatable<IRandomAccessStreamStatics>>::get_activation_factory().copy_and_close_async(source, destination)
    }}
}
DEFINE_CLSID!(RandomAccessStream(&[87,105,110,100,111,119,115,46,83,116,111,114,97,103,101,46,83,116,114,101,97,109,115,46,82,97,110,100,111,109,65,99,99,101,115,115,83,116,114,101,97,109,0]) [CLSID_RandomAccessStream]);
RT_CLASS!{class RandomAccessStreamOverStream: IRandomAccessStream}
DEFINE_IID!(IID_IRandomAccessStreamReference, 871248180, 7638, 20026, 128, 103, 209, 193, 98, 232, 100, 43);
RT_INTERFACE!{interface IRandomAccessStreamReference(IRandomAccessStreamReferenceVtbl): IInspectable(IInspectableVtbl) [IID_IRandomAccessStreamReference] {
    fn OpenReadAsync(&self, out: *mut *mut super::super::foundation::IAsyncOperation<IRandomAccessStreamWithContentType>) -> HRESULT
}}
impl IRandomAccessStreamReference {
    #[inline] pub unsafe fn open_read_async(&self) -> Result<ComPtr<super::super::foundation::IAsyncOperation<IRandomAccessStreamWithContentType>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).OpenReadAsync)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
RT_CLASS!{class RandomAccessStreamReference: IRandomAccessStreamReference}
impl RtActivatable<IRandomAccessStreamReferenceStatics> for RandomAccessStreamReference {}
impl RandomAccessStreamReference {
    #[inline] pub fn create_from_file(file: &super::IStorageFile) -> Result<ComPtr<RandomAccessStreamReference>> { unsafe {
        <Self as RtActivatable<IRandomAccessStreamReferenceStatics>>::get_activation_factory().create_from_file(file)
    }}
    #[inline] pub fn create_from_uri(uri: &super::super::foundation::Uri) -> Result<ComPtr<RandomAccessStreamReference>> { unsafe {
        <Self as RtActivatable<IRandomAccessStreamReferenceStatics>>::get_activation_factory().create_from_uri(uri)
    }}
    #[inline] pub fn create_from_stream(stream: &IRandomAccessStream) -> Result<ComPtr<RandomAccessStreamReference>> { unsafe {
        <Self as RtActivatable<IRandomAccessStreamReferenceStatics>>::get_activation_factory().create_from_stream(stream)
    }}
}
DEFINE_CLSID!(RandomAccessStreamReference(&[87,105,110,100,111,119,115,46,83,116,111,114,97,103,101,46,83,116,114,101,97,109,115,46,82,97,110,100,111,109,65,99,99,101,115,115,83,116,114,101,97,109,82,101,102,101,114,101,110,99,101,0]) [CLSID_RandomAccessStreamReference]);
DEFINE_IID!(IID_IRandomAccessStreamReferenceStatics, 2238908892, 16319, 20093, 152, 111, 239, 59, 26, 7, 169, 100);
RT_INTERFACE!{static interface IRandomAccessStreamReferenceStatics(IRandomAccessStreamReferenceStaticsVtbl): IInspectable(IInspectableVtbl) [IID_IRandomAccessStreamReferenceStatics] {
    fn CreateFromFile(&self, file: *mut super::IStorageFile, out: *mut *mut RandomAccessStreamReference) -> HRESULT,
    fn CreateFromUri(&self, uri: *mut super::super::foundation::Uri, out: *mut *mut RandomAccessStreamReference) -> HRESULT,
    fn CreateFromStream(&self, stream: *mut IRandomAccessStream, out: *mut *mut RandomAccessStreamReference) -> HRESULT
}}
impl IRandomAccessStreamReferenceStatics {
    #[inline] pub unsafe fn create_from_file(&self, file: &super::IStorageFile) -> Result<ComPtr<RandomAccessStreamReference>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).CreateFromFile)(self as *const _ as *mut _, file as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn create_from_uri(&self, uri: &super::super::foundation::Uri) -> Result<ComPtr<RandomAccessStreamReference>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).CreateFromUri)(self as *const _ as *mut _, uri as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn create_from_stream(&self, stream: &IRandomAccessStream) -> Result<ComPtr<RandomAccessStreamReference>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).CreateFromStream)(self as *const _ as *mut _, stream as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
DEFINE_IID!(IID_IRandomAccessStreamStatics, 1380773327, 28201, 19685, 149, 115, 107, 117, 61, 182, 108, 58);
RT_INTERFACE!{static interface IRandomAccessStreamStatics(IRandomAccessStreamStaticsVtbl): IInspectable(IInspectableVtbl) [IID_IRandomAccessStreamStatics] {
    fn CopyAsync(&self, source: *mut IInputStream, destination: *mut IOutputStream, out: *mut *mut super::super::foundation::IAsyncOperationWithProgress<u64, u64>) -> HRESULT,
    fn CopySizeAsync(&self, source: *mut IInputStream, destination: *mut IOutputStream, bytesToCopy: u64, out: *mut *mut super::super::foundation::IAsyncOperationWithProgress<u64, u64>) -> HRESULT,
    fn CopyAndCloseAsync(&self, source: *mut IInputStream, destination: *mut IOutputStream, out: *mut *mut super::super::foundation::IAsyncOperationWithProgress<u64, u64>) -> HRESULT
}}
impl IRandomAccessStreamStatics {
    #[inline] pub unsafe fn copy_async(&self, source: &IInputStream, destination: &IOutputStream) -> Result<ComPtr<super::super::foundation::IAsyncOperationWithProgress<u64, u64>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).CopyAsync)(self as *const _ as *mut _, source as *const _ as *mut _, destination as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn copy_size_async(&self, source: &IInputStream, destination: &IOutputStream, bytesToCopy: u64) -> Result<ComPtr<super::super::foundation::IAsyncOperationWithProgress<u64, u64>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).CopySizeAsync)(self as *const _ as *mut _, source as *const _ as *mut _, destination as *const _ as *mut _, bytesToCopy, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn copy_and_close_async(&self, source: &IInputStream, destination: &IOutputStream) -> Result<ComPtr<super::super::foundation::IAsyncOperationWithProgress<u64, u64>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).CopyAndCloseAsync)(self as *const _ as *mut _, source as *const _ as *mut _, destination as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
DEFINE_IID!(IID_IRandomAccessStreamWithContentType, 3424995367, 19261, 17295, 146, 50, 16, 199, 107, 199, 224, 56);
RT_INTERFACE!{interface IRandomAccessStreamWithContentType(IRandomAccessStreamWithContentTypeVtbl): IInspectable(IInspectableVtbl) [IID_IRandomAccessStreamWithContentType] {
    
}}
RT_ENUM! { enum UnicodeEncoding: i32 {
    Utf8 (UnicodeEncoding_Utf8) = 0, Utf16LE (UnicodeEncoding_Utf16LE) = 1, Utf16BE (UnicodeEncoding_Utf16BE) = 2,
}}
} // Windows.Storage.Streams
pub mod compression { // Windows.Storage.Compression
use ::prelude::*;
RT_ENUM! { enum CompressAlgorithm: i32 {
    InvalidAlgorithm (CompressAlgorithm_InvalidAlgorithm) = 0, NullAlgorithm (CompressAlgorithm_NullAlgorithm) = 1, Mszip (CompressAlgorithm_Mszip) = 2, Xpress (CompressAlgorithm_Xpress) = 3, XpressHuff (CompressAlgorithm_XpressHuff) = 4, Lzms (CompressAlgorithm_Lzms) = 5,
}}
DEFINE_IID!(IID_ICompressor, 180577370, 22444, 20193, 183, 2, 132, 211, 157, 84, 36, 224);
RT_INTERFACE!{interface ICompressor(ICompressorVtbl): IInspectable(IInspectableVtbl) [IID_ICompressor] {
    fn FinishAsync(&self, out: *mut *mut super::super::foundation::IAsyncOperation<bool>) -> HRESULT,
    fn DetachStream(&self, out: *mut *mut super::streams::IOutputStream) -> HRESULT
}}
impl ICompressor {
    #[inline] pub unsafe fn finish_async(&self) -> Result<ComPtr<super::super::foundation::IAsyncOperation<bool>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).FinishAsync)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn detach_stream(&self) -> Result<ComPtr<super::streams::IOutputStream>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).DetachStream)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
RT_CLASS!{class Compressor: ICompressor}
impl RtActivatable<ICompressorFactory> for Compressor {}
impl Compressor {
    #[inline] pub fn create_compressor(underlyingStream: &super::streams::IOutputStream) -> Result<ComPtr<Compressor>> { unsafe {
        <Self as RtActivatable<ICompressorFactory>>::get_activation_factory().create_compressor(underlyingStream)
    }}
    #[inline] pub fn create_compressor_ex(underlyingStream: &super::streams::IOutputStream, algorithm: CompressAlgorithm, blockSize: u32) -> Result<ComPtr<Compressor>> { unsafe {
        <Self as RtActivatable<ICompressorFactory>>::get_activation_factory().create_compressor_ex(underlyingStream, algorithm, blockSize)
    }}
}
DEFINE_CLSID!(Compressor(&[87,105,110,100,111,119,115,46,83,116,111,114,97,103,101,46,67,111,109,112,114,101,115,115,105,111,110,46,67,111,109,112,114,101,115,115,111,114,0]) [CLSID_Compressor]);
DEFINE_IID!(IID_ICompressorFactory, 1597871780, 11515, 17452, 168, 186, 215, 209, 27, 3, 157, 160);
RT_INTERFACE!{static interface ICompressorFactory(ICompressorFactoryVtbl): IInspectable(IInspectableVtbl) [IID_ICompressorFactory] {
    fn CreateCompressor(&self, underlyingStream: *mut super::streams::IOutputStream, out: *mut *mut Compressor) -> HRESULT,
    fn CreateCompressorEx(&self, underlyingStream: *mut super::streams::IOutputStream, algorithm: CompressAlgorithm, blockSize: u32, out: *mut *mut Compressor) -> HRESULT
}}
impl ICompressorFactory {
    #[inline] pub unsafe fn create_compressor(&self, underlyingStream: &super::streams::IOutputStream) -> Result<ComPtr<Compressor>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).CreateCompressor)(self as *const _ as *mut _, underlyingStream as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn create_compressor_ex(&self, underlyingStream: &super::streams::IOutputStream, algorithm: CompressAlgorithm, blockSize: u32) -> Result<ComPtr<Compressor>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).CreateCompressorEx)(self as *const _ as *mut _, underlyingStream as *const _ as *mut _, algorithm, blockSize, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
DEFINE_IID!(IID_IDecompressor, 3095658054, 54922, 19595, 173, 160, 78, 232, 19, 252, 82, 131);
RT_INTERFACE!{interface IDecompressor(IDecompressorVtbl): IInspectable(IInspectableVtbl) [IID_IDecompressor] {
    fn DetachStream(&self, out: *mut *mut super::streams::IInputStream) -> HRESULT
}}
impl IDecompressor {
    #[inline] pub unsafe fn detach_stream(&self) -> Result<ComPtr<super::streams::IInputStream>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).DetachStream)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
RT_CLASS!{class Decompressor: IDecompressor}
impl RtActivatable<IDecompressorFactory> for Decompressor {}
impl Decompressor {
    #[inline] pub fn create_decompressor(underlyingStream: &super::streams::IInputStream) -> Result<ComPtr<Decompressor>> { unsafe {
        <Self as RtActivatable<IDecompressorFactory>>::get_activation_factory().create_decompressor(underlyingStream)
    }}
}
DEFINE_CLSID!(Decompressor(&[87,105,110,100,111,119,115,46,83,116,111,114,97,103,101,46,67,111,109,112,114,101,115,115,105,111,110,46,68,101,99,111,109,112,114,101,115,115,111,114,0]) [CLSID_Decompressor]);
DEFINE_IID!(IID_IDecompressorFactory, 1396171346, 7586, 17121, 136, 52, 3, 121, 210, 141, 116, 47);
RT_INTERFACE!{static interface IDecompressorFactory(IDecompressorFactoryVtbl): IInspectable(IInspectableVtbl) [IID_IDecompressorFactory] {
    fn CreateDecompressor(&self, underlyingStream: *mut super::streams::IInputStream, out: *mut *mut Decompressor) -> HRESULT
}}
impl IDecompressorFactory {
    #[inline] pub unsafe fn create_decompressor(&self, underlyingStream: &super::streams::IInputStream) -> Result<ComPtr<Decompressor>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).CreateDecompressor)(self as *const _ as *mut _, underlyingStream as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
} // Windows.Storage.Compression
pub mod search { // Windows.Storage.Search
use ::prelude::*;
RT_ENUM! { enum CommonFileQuery: i32 {
    DefaultQuery (CommonFileQuery_DefaultQuery) = 0, OrderByName (CommonFileQuery_OrderByName) = 1, OrderByTitle (CommonFileQuery_OrderByTitle) = 2, OrderByMusicProperties (CommonFileQuery_OrderByMusicProperties) = 3, OrderBySearchRank (CommonFileQuery_OrderBySearchRank) = 4, OrderByDate (CommonFileQuery_OrderByDate) = 5,
}}
RT_ENUM! { enum CommonFolderQuery: i32 {
    DefaultQuery (CommonFolderQuery_DefaultQuery) = 0, GroupByYear (CommonFolderQuery_GroupByYear) = 100, GroupByMonth (CommonFolderQuery_GroupByMonth) = 101, GroupByArtist (CommonFolderQuery_GroupByArtist) = 102, GroupByAlbum (CommonFolderQuery_GroupByAlbum) = 103, GroupByAlbumArtist (CommonFolderQuery_GroupByAlbumArtist) = 104, GroupByComposer (CommonFolderQuery_GroupByComposer) = 105, GroupByGenre (CommonFolderQuery_GroupByGenre) = 106, GroupByPublishedYear (CommonFolderQuery_GroupByPublishedYear) = 107, GroupByRating (CommonFolderQuery_GroupByRating) = 108, GroupByTag (CommonFolderQuery_GroupByTag) = 109, GroupByAuthor (CommonFolderQuery_GroupByAuthor) = 110, GroupByType (CommonFolderQuery_GroupByType) = 111,
}}
DEFINE_IID!(IID_IContentIndexer, 2977333133, 63128, 18818, 176, 95, 58, 110, 140, 171, 1, 162);
RT_INTERFACE!{interface IContentIndexer(IContentIndexerVtbl): IInspectable(IInspectableVtbl) [IID_IContentIndexer] {
    fn AddAsync(&self, indexableContent: *mut IIndexableContent, out: *mut *mut super::super::foundation::IAsyncAction) -> HRESULT,
    fn UpdateAsync(&self, indexableContent: *mut IIndexableContent, out: *mut *mut super::super::foundation::IAsyncAction) -> HRESULT,
    fn DeleteAsync(&self, contentId: HSTRING, out: *mut *mut super::super::foundation::IAsyncAction) -> HRESULT,
    fn DeleteMultipleAsync(&self, contentIds: *mut super::super::foundation::collections::IIterable<HString>, out: *mut *mut super::super::foundation::IAsyncAction) -> HRESULT,
    fn DeleteAllAsync(&self, out: *mut *mut super::super::foundation::IAsyncAction) -> HRESULT,
    fn RetrievePropertiesAsync(&self, contentId: HSTRING, propertiesToRetrieve: *mut super::super::foundation::collections::IIterable<HString>, out: *mut *mut super::super::foundation::IAsyncOperation<super::super::foundation::collections::IMapView<HString, IInspectable>>) -> HRESULT,
    fn get_Revision(&self, out: *mut u64) -> HRESULT
}}
impl IContentIndexer {
    #[inline] pub unsafe fn add_async(&self, indexableContent: &IIndexableContent) -> Result<ComPtr<super::super::foundation::IAsyncAction>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).AddAsync)(self as *const _ as *mut _, indexableContent as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn update_async(&self, indexableContent: &IIndexableContent) -> Result<ComPtr<super::super::foundation::IAsyncAction>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).UpdateAsync)(self as *const _ as *mut _, indexableContent as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn delete_async(&self, contentId: &HStringArg) -> Result<ComPtr<super::super::foundation::IAsyncAction>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).DeleteAsync)(self as *const _ as *mut _, contentId.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn delete_multiple_async(&self, contentIds: &super::super::foundation::collections::IIterable<HString>) -> Result<ComPtr<super::super::foundation::IAsyncAction>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).DeleteMultipleAsync)(self as *const _ as *mut _, contentIds as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn delete_all_async(&self) -> Result<ComPtr<super::super::foundation::IAsyncAction>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).DeleteAllAsync)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn retrieve_properties_async(&self, contentId: &HStringArg, propertiesToRetrieve: &super::super::foundation::collections::IIterable<HString>) -> Result<ComPtr<super::super::foundation::IAsyncOperation<super::super::foundation::collections::IMapView<HString, IInspectable>>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).RetrievePropertiesAsync)(self as *const _ as *mut _, contentId.get(), propertiesToRetrieve as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_revision(&self) -> Result<u64> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_Revision)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
}
RT_CLASS!{class ContentIndexer: IContentIndexer}
impl RtActivatable<IContentIndexerStatics> for ContentIndexer {}
impl ContentIndexer {
    #[inline] pub fn get_indexer_with_name(indexName: &HStringArg) -> Result<ComPtr<ContentIndexer>> { unsafe {
        <Self as RtActivatable<IContentIndexerStatics>>::get_activation_factory().get_indexer_with_name(indexName)
    }}
    #[inline] pub fn get_indexer() -> Result<ComPtr<ContentIndexer>> { unsafe {
        <Self as RtActivatable<IContentIndexerStatics>>::get_activation_factory().get_indexer()
    }}
}
DEFINE_CLSID!(ContentIndexer(&[87,105,110,100,111,119,115,46,83,116,111,114,97,103,101,46,83,101,97,114,99,104,46,67,111,110,116,101,110,116,73,110,100,101,120,101,114,0]) [CLSID_ContentIndexer]);
DEFINE_IID!(IID_IContentIndexerQuery, 1893970168, 19452, 17034, 136, 137, 204, 81, 218, 154, 123, 157);
RT_INTERFACE!{interface IContentIndexerQuery(IContentIndexerQueryVtbl): IInspectable(IInspectableVtbl) [IID_IContentIndexerQuery] {
    fn GetCountAsync(&self, out: *mut *mut super::super::foundation::IAsyncOperation<u32>) -> HRESULT,
    fn GetPropertiesAsync(&self, out: *mut *mut super::super::foundation::IAsyncOperation<super::super::foundation::collections::IVectorView<super::super::foundation::collections::IMapView<HString, IInspectable>>>) -> HRESULT,
    fn GetPropertiesRangeAsync(&self, startIndex: u32, maxItems: u32, out: *mut *mut super::super::foundation::IAsyncOperation<super::super::foundation::collections::IVectorView<super::super::foundation::collections::IMapView<HString, IInspectable>>>) -> HRESULT,
    fn GetAsync(&self, out: *mut *mut super::super::foundation::IAsyncOperation<super::super::foundation::collections::IVectorView<IIndexableContent>>) -> HRESULT,
    fn GetRangeAsync(&self, startIndex: u32, maxItems: u32, out: *mut *mut super::super::foundation::IAsyncOperation<super::super::foundation::collections::IVectorView<IIndexableContent>>) -> HRESULT,
    fn get_QueryFolder(&self, out: *mut *mut super::StorageFolder) -> HRESULT
}}
impl IContentIndexerQuery {
    #[inline] pub unsafe fn get_count_async(&self) -> Result<ComPtr<super::super::foundation::IAsyncOperation<u32>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetCountAsync)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_properties_async(&self) -> Result<ComPtr<super::super::foundation::IAsyncOperation<super::super::foundation::collections::IVectorView<super::super::foundation::collections::IMapView<HString, IInspectable>>>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetPropertiesAsync)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_properties_range_async(&self, startIndex: u32, maxItems: u32) -> Result<ComPtr<super::super::foundation::IAsyncOperation<super::super::foundation::collections::IVectorView<super::super::foundation::collections::IMapView<HString, IInspectable>>>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetPropertiesRangeAsync)(self as *const _ as *mut _, startIndex, maxItems, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_async(&self) -> Result<ComPtr<super::super::foundation::IAsyncOperation<super::super::foundation::collections::IVectorView<IIndexableContent>>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetAsync)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_range_async(&self, startIndex: u32, maxItems: u32) -> Result<ComPtr<super::super::foundation::IAsyncOperation<super::super::foundation::collections::IVectorView<IIndexableContent>>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetRangeAsync)(self as *const _ as *mut _, startIndex, maxItems, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_query_folder(&self) -> Result<ComPtr<super::StorageFolder>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_QueryFolder)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
RT_CLASS!{class ContentIndexerQuery: IContentIndexerQuery}
DEFINE_IID!(IID_IContentIndexerQueryOperations, 679624208, 18310, 17137, 151, 48, 121, 43, 53, 102, 177, 80);
RT_INTERFACE!{interface IContentIndexerQueryOperations(IContentIndexerQueryOperationsVtbl): IInspectable(IInspectableVtbl) [IID_IContentIndexerQueryOperations] {
    fn CreateQueryWithSortOrderAndLanguage(&self, searchFilter: HSTRING, propertiesToRetrieve: *mut super::super::foundation::collections::IIterable<HString>, sortOrder: *mut super::super::foundation::collections::IIterable<SortEntry>, searchFilterLanguage: HSTRING, out: *mut *mut ContentIndexerQuery) -> HRESULT,
    fn CreateQueryWithSortOrder(&self, searchFilter: HSTRING, propertiesToRetrieve: *mut super::super::foundation::collections::IIterable<HString>, sortOrder: *mut super::super::foundation::collections::IIterable<SortEntry>, out: *mut *mut ContentIndexerQuery) -> HRESULT,
    fn CreateQuery(&self, searchFilter: HSTRING, propertiesToRetrieve: *mut super::super::foundation::collections::IIterable<HString>, out: *mut *mut ContentIndexerQuery) -> HRESULT
}}
impl IContentIndexerQueryOperations {
    #[inline] pub unsafe fn create_query_with_sort_order_and_language(&self, searchFilter: &HStringArg, propertiesToRetrieve: &super::super::foundation::collections::IIterable<HString>, sortOrder: &super::super::foundation::collections::IIterable<SortEntry>, searchFilterLanguage: &HStringArg) -> Result<ComPtr<ContentIndexerQuery>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).CreateQueryWithSortOrderAndLanguage)(self as *const _ as *mut _, searchFilter.get(), propertiesToRetrieve as *const _ as *mut _, sortOrder as *const _ as *mut _, searchFilterLanguage.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn create_query_with_sort_order(&self, searchFilter: &HStringArg, propertiesToRetrieve: &super::super::foundation::collections::IIterable<HString>, sortOrder: &super::super::foundation::collections::IIterable<SortEntry>) -> Result<ComPtr<ContentIndexerQuery>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).CreateQueryWithSortOrder)(self as *const _ as *mut _, searchFilter.get(), propertiesToRetrieve as *const _ as *mut _, sortOrder as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn create_query(&self, searchFilter: &HStringArg, propertiesToRetrieve: &super::super::foundation::collections::IIterable<HString>) -> Result<ComPtr<ContentIndexerQuery>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).CreateQuery)(self as *const _ as *mut _, searchFilter.get(), propertiesToRetrieve as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
DEFINE_IID!(IID_IContentIndexerStatics, 2353562485, 45950, 19552, 155, 168, 183, 96, 253, 163, 229, 157);
RT_INTERFACE!{static interface IContentIndexerStatics(IContentIndexerStaticsVtbl): IInspectable(IInspectableVtbl) [IID_IContentIndexerStatics] {
    fn GetIndexerWithName(&self, indexName: HSTRING, out: *mut *mut ContentIndexer) -> HRESULT,
    fn GetIndexer(&self, out: *mut *mut ContentIndexer) -> HRESULT
}}
impl IContentIndexerStatics {
    #[inline] pub unsafe fn get_indexer_with_name(&self, indexName: &HStringArg) -> Result<ComPtr<ContentIndexer>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetIndexerWithName)(self as *const _ as *mut _, indexName.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_indexer(&self) -> Result<ComPtr<ContentIndexer>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetIndexer)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
RT_ENUM! { enum DateStackOption: i32 {
    None (DateStackOption_None) = 0, Year (DateStackOption_Year) = 1, Month (DateStackOption_Month) = 2,
}}
RT_ENUM! { enum FolderDepth: i32 {
    Shallow (FolderDepth_Shallow) = 0, Deep (FolderDepth_Deep) = 1,
}}
DEFINE_IID!(IID_IIndexableContent, 3438387295, 54453, 18490, 176, 110, 224, 219, 30, 196, 32, 228);
RT_INTERFACE!{interface IIndexableContent(IIndexableContentVtbl): IInspectable(IInspectableVtbl) [IID_IIndexableContent] {
    fn get_Id(&self, out: *mut HSTRING) -> HRESULT,
    fn put_Id(&self, value: HSTRING) -> HRESULT,
    fn get_Properties(&self, out: *mut *mut super::super::foundation::collections::IMap<HString, IInspectable>) -> HRESULT,
    fn get_Stream(&self, out: *mut *mut super::streams::IRandomAccessStream) -> HRESULT,
    fn put_Stream(&self, value: *mut super::streams::IRandomAccessStream) -> HRESULT,
    fn get_StreamContentType(&self, out: *mut HSTRING) -> HRESULT,
    fn put_StreamContentType(&self, value: HSTRING) -> HRESULT
}}
impl IIndexableContent {
    #[inline] pub unsafe fn get_id(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Id)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_id(&self, value: &HStringArg) -> Result<()> {
        let hr = ((*self.lpVtbl).put_Id)(self as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_properties(&self) -> Result<ComPtr<super::super::foundation::collections::IMap<HString, IInspectable>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Properties)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_stream(&self) -> Result<ComPtr<super::streams::IRandomAccessStream>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Stream)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_stream(&self, value: &super::streams::IRandomAccessStream) -> Result<()> {
        let hr = ((*self.lpVtbl).put_Stream)(self as *const _ as *mut _, value as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_stream_content_type(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_StreamContentType)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_stream_content_type(&self, value: &HStringArg) -> Result<()> {
        let hr = ((*self.lpVtbl).put_StreamContentType)(self as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
}
RT_CLASS!{class IndexableContent: IIndexableContent}
impl RtActivatable<IActivationFactory> for IndexableContent {}
DEFINE_CLSID!(IndexableContent(&[87,105,110,100,111,119,115,46,83,116,111,114,97,103,101,46,83,101,97,114,99,104,46,73,110,100,101,120,97,98,108,101,67,111,110,116,101,110,116,0]) [CLSID_IndexableContent]);
RT_ENUM! { enum IndexedState: i32 {
    Unknown (IndexedState_Unknown) = 0, NotIndexed (IndexedState_NotIndexed) = 1, PartiallyIndexed (IndexedState_PartiallyIndexed) = 2, FullyIndexed (IndexedState_FullyIndexed) = 3,
}}
RT_ENUM! { enum IndexerOption: i32 {
    UseIndexerWhenAvailable (IndexerOption_UseIndexerWhenAvailable) = 0, OnlyUseIndexer (IndexerOption_OnlyUseIndexer) = 1, DoNotUseIndexer (IndexerOption_DoNotUseIndexer) = 2, OnlyUseIndexerAndOptimizeForIndexedProperties (IndexerOption_OnlyUseIndexerAndOptimizeForIndexedProperties) = 3,
}}
DEFINE_IID!(IID_IQueryOptions, 509495022, 3909, 18488, 168, 233, 208, 71, 157, 68, 108, 48);
RT_INTERFACE!{interface IQueryOptions(IQueryOptionsVtbl): IInspectable(IInspectableVtbl) [IID_IQueryOptions] {
    fn get_FileTypeFilter(&self, out: *mut *mut super::super::foundation::collections::IVector<HString>) -> HRESULT,
    fn get_FolderDepth(&self, out: *mut FolderDepth) -> HRESULT,
    fn put_FolderDepth(&self, value: FolderDepth) -> HRESULT,
    fn get_ApplicationSearchFilter(&self, out: *mut HSTRING) -> HRESULT,
    fn put_ApplicationSearchFilter(&self, value: HSTRING) -> HRESULT,
    fn get_UserSearchFilter(&self, out: *mut HSTRING) -> HRESULT,
    fn put_UserSearchFilter(&self, value: HSTRING) -> HRESULT,
    fn get_Language(&self, out: *mut HSTRING) -> HRESULT,
    fn put_Language(&self, value: HSTRING) -> HRESULT,
    fn get_IndexerOption(&self, out: *mut IndexerOption) -> HRESULT,
    fn put_IndexerOption(&self, value: IndexerOption) -> HRESULT,
    fn get_SortOrder(&self, out: *mut *mut super::super::foundation::collections::IVector<SortEntry>) -> HRESULT,
    fn get_GroupPropertyName(&self, out: *mut HSTRING) -> HRESULT,
    fn get_DateStackOption(&self, out: *mut DateStackOption) -> HRESULT,
    fn SaveToString(&self, out: *mut HSTRING) -> HRESULT,
    fn LoadFromString(&self, value: HSTRING) -> HRESULT,
    fn SetThumbnailPrefetch(&self, mode: super::fileproperties::ThumbnailMode, requestedSize: u32, options: super::fileproperties::ThumbnailOptions) -> HRESULT,
    fn SetPropertyPrefetch(&self, options: super::fileproperties::PropertyPrefetchOptions, propertiesToRetrieve: *mut super::super::foundation::collections::IIterable<HString>) -> HRESULT
}}
impl IQueryOptions {
    #[inline] pub unsafe fn get_file_type_filter(&self) -> Result<ComPtr<super::super::foundation::collections::IVector<HString>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_FileTypeFilter)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_folder_depth(&self) -> Result<FolderDepth> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_FolderDepth)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_folder_depth(&self, value: FolderDepth) -> Result<()> {
        let hr = ((*self.lpVtbl).put_FolderDepth)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_application_search_filter(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_ApplicationSearchFilter)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_application_search_filter(&self, value: &HStringArg) -> Result<()> {
        let hr = ((*self.lpVtbl).put_ApplicationSearchFilter)(self as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_user_search_filter(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_UserSearchFilter)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_user_search_filter(&self, value: &HStringArg) -> Result<()> {
        let hr = ((*self.lpVtbl).put_UserSearchFilter)(self as *const _ as *mut _, value.get());
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
    #[inline] pub unsafe fn get_indexer_option(&self) -> Result<IndexerOption> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_IndexerOption)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_indexer_option(&self, value: IndexerOption) -> Result<()> {
        let hr = ((*self.lpVtbl).put_IndexerOption)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_sort_order(&self) -> Result<ComPtr<super::super::foundation::collections::IVector<SortEntry>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_SortOrder)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_group_property_name(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_GroupPropertyName)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_date_stack_option(&self) -> Result<DateStackOption> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_DateStackOption)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn save_to_string(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).SaveToString)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn load_from_string(&self, value: &HStringArg) -> Result<()> {
        let hr = ((*self.lpVtbl).LoadFromString)(self as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_thumbnail_prefetch(&self, mode: super::fileproperties::ThumbnailMode, requestedSize: u32, options: super::fileproperties::ThumbnailOptions) -> Result<()> {
        let hr = ((*self.lpVtbl).SetThumbnailPrefetch)(self as *const _ as *mut _, mode, requestedSize, options);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_property_prefetch(&self, options: super::fileproperties::PropertyPrefetchOptions, propertiesToRetrieve: &super::super::foundation::collections::IIterable<HString>) -> Result<()> {
        let hr = ((*self.lpVtbl).SetPropertyPrefetch)(self as *const _ as *mut _, options, propertiesToRetrieve as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
}
RT_CLASS!{class QueryOptions: IQueryOptions}
impl RtActivatable<IQueryOptionsFactory> for QueryOptions {}
impl RtActivatable<IActivationFactory> for QueryOptions {}
impl QueryOptions {
    #[inline] pub fn create_common_file_query(query: CommonFileQuery, fileTypeFilter: &super::super::foundation::collections::IIterable<HString>) -> Result<ComPtr<QueryOptions>> { unsafe {
        <Self as RtActivatable<IQueryOptionsFactory>>::get_activation_factory().create_common_file_query(query, fileTypeFilter)
    }}
    #[inline] pub fn create_common_folder_query(query: CommonFolderQuery) -> Result<ComPtr<QueryOptions>> { unsafe {
        <Self as RtActivatable<IQueryOptionsFactory>>::get_activation_factory().create_common_folder_query(query)
    }}
}
DEFINE_CLSID!(QueryOptions(&[87,105,110,100,111,119,115,46,83,116,111,114,97,103,101,46,83,101,97,114,99,104,46,81,117,101,114,121,79,112,116,105,111,110,115,0]) [CLSID_QueryOptions]);
DEFINE_IID!(IID_IQueryOptionsFactory, 53354380, 43457, 20081, 128, 17, 13, 238, 157, 72, 17, 163);
RT_INTERFACE!{static interface IQueryOptionsFactory(IQueryOptionsFactoryVtbl): IInspectable(IInspectableVtbl) [IID_IQueryOptionsFactory] {
    fn CreateCommonFileQuery(&self, query: CommonFileQuery, fileTypeFilter: *mut super::super::foundation::collections::IIterable<HString>, out: *mut *mut QueryOptions) -> HRESULT,
    fn CreateCommonFolderQuery(&self, query: CommonFolderQuery, out: *mut *mut QueryOptions) -> HRESULT
}}
impl IQueryOptionsFactory {
    #[inline] pub unsafe fn create_common_file_query(&self, query: CommonFileQuery, fileTypeFilter: &super::super::foundation::collections::IIterable<HString>) -> Result<ComPtr<QueryOptions>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).CreateCommonFileQuery)(self as *const _ as *mut _, query, fileTypeFilter as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn create_common_folder_query(&self, query: CommonFolderQuery) -> Result<ComPtr<QueryOptions>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).CreateCommonFolderQuery)(self as *const _ as *mut _, query, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
DEFINE_IID!(IID_IQueryOptionsWithProviderFilter, 1537019942, 5572, 17629, 184, 154, 71, 165, 155, 125, 124, 79);
RT_INTERFACE!{interface IQueryOptionsWithProviderFilter(IQueryOptionsWithProviderFilterVtbl): IInspectable(IInspectableVtbl) [IID_IQueryOptionsWithProviderFilter] {
    fn get_StorageProviderIdFilter(&self, out: *mut *mut super::super::foundation::collections::IVector<HString>) -> HRESULT
}}
impl IQueryOptionsWithProviderFilter {
    #[inline] pub unsafe fn get_storage_provider_id_filter(&self) -> Result<ComPtr<super::super::foundation::collections::IVector<HString>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_StorageProviderIdFilter)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
RT_STRUCT! { struct SortEntry {
    PropertyName: HSTRING, AscendingOrder: bool,
}}
RT_CLASS!{class SortEntryVector: super::super::foundation::collections::IVector<SortEntry>}
DEFINE_IID!(IID_IStorageFileQueryResult, 1392354375, 11178, 16684, 178, 159, 212, 177, 119, 142, 250, 30);
RT_INTERFACE!{interface IStorageFileQueryResult(IStorageFileQueryResultVtbl): IInspectable(IInspectableVtbl) [IID_IStorageFileQueryResult] {
    fn GetFilesAsync(&self, startIndex: u32, maxNumberOfItems: u32, out: *mut *mut super::super::foundation::IAsyncOperation<super::super::foundation::collections::IVectorView<super::StorageFile>>) -> HRESULT,
    fn GetFilesAsyncDefaultStartAndCount(&self, out: *mut *mut super::super::foundation::IAsyncOperation<super::super::foundation::collections::IVectorView<super::StorageFile>>) -> HRESULT
}}
impl IStorageFileQueryResult {
    #[inline] pub unsafe fn get_files_async(&self, startIndex: u32, maxNumberOfItems: u32) -> Result<ComPtr<super::super::foundation::IAsyncOperation<super::super::foundation::collections::IVectorView<super::StorageFile>>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetFilesAsync)(self as *const _ as *mut _, startIndex, maxNumberOfItems, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_files_async_default_start_and_count(&self) -> Result<ComPtr<super::super::foundation::IAsyncOperation<super::super::foundation::collections::IVectorView<super::StorageFile>>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetFilesAsyncDefaultStartAndCount)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
RT_CLASS!{class StorageFileQueryResult: IStorageFileQueryResult}
DEFINE_IID!(IID_IStorageFileQueryResult2, 1314765277, 28993, 18116, 139, 227, 233, 220, 158, 39, 39, 92);
RT_INTERFACE!{interface IStorageFileQueryResult2(IStorageFileQueryResult2Vtbl): IInspectable(IInspectableVtbl) [IID_IStorageFileQueryResult2] {
    #[cfg(feature="windows-data")] fn GetMatchingPropertiesWithRanges(&self, file: *mut super::StorageFile, out: *mut *mut super::super::foundation::collections::IMap<HString, super::super::foundation::collections::IVectorView<super::super::data::text::TextSegment>>) -> HRESULT
}}
impl IStorageFileQueryResult2 {
    #[cfg(feature="windows-data")] #[inline] pub unsafe fn get_matching_properties_with_ranges(&self, file: &super::StorageFile) -> Result<ComPtr<super::super::foundation::collections::IMap<HString, super::super::foundation::collections::IVectorView<super::super::data::text::TextSegment>>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetMatchingPropertiesWithRanges)(self as *const _ as *mut _, file as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
DEFINE_IID!(IID_IStorageFolderQueryOperations, 3410218185, 17515, 19023, 190, 151, 117, 119, 113, 190, 82, 3);
RT_INTERFACE!{interface IStorageFolderQueryOperations(IStorageFolderQueryOperationsVtbl): IInspectable(IInspectableVtbl) [IID_IStorageFolderQueryOperations] {
    fn GetIndexedStateAsync(&self, out: *mut *mut super::super::foundation::IAsyncOperation<IndexedState>) -> HRESULT,
    fn CreateFileQueryOverloadDefault(&self, out: *mut *mut StorageFileQueryResult) -> HRESULT,
    fn CreateFileQuery(&self, query: CommonFileQuery, out: *mut *mut StorageFileQueryResult) -> HRESULT,
    fn CreateFileQueryWithOptions(&self, queryOptions: *mut QueryOptions, out: *mut *mut StorageFileQueryResult) -> HRESULT,
    fn CreateFolderQueryOverloadDefault(&self, out: *mut *mut StorageFolderQueryResult) -> HRESULT,
    fn CreateFolderQuery(&self, query: CommonFolderQuery, out: *mut *mut StorageFolderQueryResult) -> HRESULT,
    fn CreateFolderQueryWithOptions(&self, queryOptions: *mut QueryOptions, out: *mut *mut StorageFolderQueryResult) -> HRESULT,
    fn CreateItemQuery(&self, out: *mut *mut StorageItemQueryResult) -> HRESULT,
    fn CreateItemQueryWithOptions(&self, queryOptions: *mut QueryOptions, out: *mut *mut StorageItemQueryResult) -> HRESULT,
    fn GetFilesAsync(&self, query: CommonFileQuery, startIndex: u32, maxItemsToRetrieve: u32, out: *mut *mut super::super::foundation::IAsyncOperation<super::super::foundation::collections::IVectorView<super::StorageFile>>) -> HRESULT,
    fn GetFilesAsyncOverloadDefaultStartAndCount(&self, query: CommonFileQuery, out: *mut *mut super::super::foundation::IAsyncOperation<super::super::foundation::collections::IVectorView<super::StorageFile>>) -> HRESULT,
    fn GetFoldersAsync(&self, query: CommonFolderQuery, startIndex: u32, maxItemsToRetrieve: u32, out: *mut *mut super::super::foundation::IAsyncOperation<super::super::foundation::collections::IVectorView<super::StorageFolder>>) -> HRESULT,
    fn GetFoldersAsyncOverloadDefaultStartAndCount(&self, query: CommonFolderQuery, out: *mut *mut super::super::foundation::IAsyncOperation<super::super::foundation::collections::IVectorView<super::StorageFolder>>) -> HRESULT,
    fn GetItemsAsync(&self, startIndex: u32, maxItemsToRetrieve: u32, out: *mut *mut super::super::foundation::IAsyncOperation<super::super::foundation::collections::IVectorView<super::IStorageItem>>) -> HRESULT,
    fn AreQueryOptionsSupported(&self, queryOptions: *mut QueryOptions, out: *mut bool) -> HRESULT,
    fn IsCommonFolderQuerySupported(&self, query: CommonFolderQuery, out: *mut bool) -> HRESULT,
    fn IsCommonFileQuerySupported(&self, query: CommonFileQuery, out: *mut bool) -> HRESULT
}}
impl IStorageFolderQueryOperations {
    #[inline] pub unsafe fn get_indexed_state_async(&self) -> Result<ComPtr<super::super::foundation::IAsyncOperation<IndexedState>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetIndexedStateAsync)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn create_file_query_overload_default(&self) -> Result<ComPtr<StorageFileQueryResult>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).CreateFileQueryOverloadDefault)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn create_file_query(&self, query: CommonFileQuery) -> Result<ComPtr<StorageFileQueryResult>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).CreateFileQuery)(self as *const _ as *mut _, query, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn create_file_query_with_options(&self, queryOptions: &QueryOptions) -> Result<ComPtr<StorageFileQueryResult>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).CreateFileQueryWithOptions)(self as *const _ as *mut _, queryOptions as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn create_folder_query_overload_default(&self) -> Result<ComPtr<StorageFolderQueryResult>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).CreateFolderQueryOverloadDefault)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn create_folder_query(&self, query: CommonFolderQuery) -> Result<ComPtr<StorageFolderQueryResult>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).CreateFolderQuery)(self as *const _ as *mut _, query, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn create_folder_query_with_options(&self, queryOptions: &QueryOptions) -> Result<ComPtr<StorageFolderQueryResult>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).CreateFolderQueryWithOptions)(self as *const _ as *mut _, queryOptions as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn create_item_query(&self) -> Result<ComPtr<StorageItemQueryResult>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).CreateItemQuery)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn create_item_query_with_options(&self, queryOptions: &QueryOptions) -> Result<ComPtr<StorageItemQueryResult>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).CreateItemQueryWithOptions)(self as *const _ as *mut _, queryOptions as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_files_async(&self, query: CommonFileQuery, startIndex: u32, maxItemsToRetrieve: u32) -> Result<ComPtr<super::super::foundation::IAsyncOperation<super::super::foundation::collections::IVectorView<super::StorageFile>>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetFilesAsync)(self as *const _ as *mut _, query, startIndex, maxItemsToRetrieve, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_files_async_overload_default_start_and_count(&self, query: CommonFileQuery) -> Result<ComPtr<super::super::foundation::IAsyncOperation<super::super::foundation::collections::IVectorView<super::StorageFile>>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetFilesAsyncOverloadDefaultStartAndCount)(self as *const _ as *mut _, query, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_folders_async(&self, query: CommonFolderQuery, startIndex: u32, maxItemsToRetrieve: u32) -> Result<ComPtr<super::super::foundation::IAsyncOperation<super::super::foundation::collections::IVectorView<super::StorageFolder>>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetFoldersAsync)(self as *const _ as *mut _, query, startIndex, maxItemsToRetrieve, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_folders_async_overload_default_start_and_count(&self, query: CommonFolderQuery) -> Result<ComPtr<super::super::foundation::IAsyncOperation<super::super::foundation::collections::IVectorView<super::StorageFolder>>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetFoldersAsyncOverloadDefaultStartAndCount)(self as *const _ as *mut _, query, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_items_async(&self, startIndex: u32, maxItemsToRetrieve: u32) -> Result<ComPtr<super::super::foundation::IAsyncOperation<super::super::foundation::collections::IVectorView<super::IStorageItem>>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetItemsAsync)(self as *const _ as *mut _, startIndex, maxItemsToRetrieve, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn are_query_options_supported(&self, queryOptions: &QueryOptions) -> Result<bool> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).AreQueryOptionsSupported)(self as *const _ as *mut _, queryOptions as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn is_common_folder_query_supported(&self, query: CommonFolderQuery) -> Result<bool> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).IsCommonFolderQuerySupported)(self as *const _ as *mut _, query, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn is_common_file_query_supported(&self, query: CommonFileQuery) -> Result<bool> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).IsCommonFileQuerySupported)(self as *const _ as *mut _, query, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
}
DEFINE_IID!(IID_IStorageFolderQueryResult, 1716832529, 32102, 18170, 174, 207, 228, 164, 186, 169, 58, 184);
RT_INTERFACE!{interface IStorageFolderQueryResult(IStorageFolderQueryResultVtbl): IInspectable(IInspectableVtbl) [IID_IStorageFolderQueryResult] {
    fn GetFoldersAsync(&self, startIndex: u32, maxNumberOfItems: u32, out: *mut *mut super::super::foundation::IAsyncOperation<super::super::foundation::collections::IVectorView<super::StorageFolder>>) -> HRESULT,
    fn GetFoldersAsyncDefaultStartAndCount(&self, out: *mut *mut super::super::foundation::IAsyncOperation<super::super::foundation::collections::IVectorView<super::StorageFolder>>) -> HRESULT
}}
impl IStorageFolderQueryResult {
    #[inline] pub unsafe fn get_folders_async(&self, startIndex: u32, maxNumberOfItems: u32) -> Result<ComPtr<super::super::foundation::IAsyncOperation<super::super::foundation::collections::IVectorView<super::StorageFolder>>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetFoldersAsync)(self as *const _ as *mut _, startIndex, maxNumberOfItems, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_folders_async_default_start_and_count(&self) -> Result<ComPtr<super::super::foundation::IAsyncOperation<super::super::foundation::collections::IVectorView<super::StorageFolder>>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetFoldersAsyncDefaultStartAndCount)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
RT_CLASS!{class StorageFolderQueryResult: IStorageFolderQueryResult}
DEFINE_IID!(IID_IStorageItemQueryResult, 3902046329, 40280, 18360, 178, 178, 65, 176, 127, 71, 149, 249);
RT_INTERFACE!{interface IStorageItemQueryResult(IStorageItemQueryResultVtbl): IInspectable(IInspectableVtbl) [IID_IStorageItemQueryResult] {
    fn GetItemsAsync(&self, startIndex: u32, maxNumberOfItems: u32, out: *mut *mut super::super::foundation::IAsyncOperation<super::super::foundation::collections::IVectorView<super::IStorageItem>>) -> HRESULT,
    fn GetItemsAsyncDefaultStartAndCount(&self, out: *mut *mut super::super::foundation::IAsyncOperation<super::super::foundation::collections::IVectorView<super::IStorageItem>>) -> HRESULT
}}
impl IStorageItemQueryResult {
    #[inline] pub unsafe fn get_items_async(&self, startIndex: u32, maxNumberOfItems: u32) -> Result<ComPtr<super::super::foundation::IAsyncOperation<super::super::foundation::collections::IVectorView<super::IStorageItem>>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetItemsAsync)(self as *const _ as *mut _, startIndex, maxNumberOfItems, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_items_async_default_start_and_count(&self) -> Result<ComPtr<super::super::foundation::IAsyncOperation<super::super::foundation::collections::IVectorView<super::IStorageItem>>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetItemsAsyncDefaultStartAndCount)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
RT_CLASS!{class StorageItemQueryResult: IStorageItemQueryResult}
DEFINE_IID!(IID_IStorageLibraryContentChangedTriggerDetails, 708254071, 43967, 19997, 138, 165, 99, 133, 216, 136, 71, 153);
RT_INTERFACE!{interface IStorageLibraryContentChangedTriggerDetails(IStorageLibraryContentChangedTriggerDetailsVtbl): IInspectable(IInspectableVtbl) [IID_IStorageLibraryContentChangedTriggerDetails] {
    fn get_Folder(&self, out: *mut *mut super::StorageFolder) -> HRESULT,
    fn CreateModifiedSinceQuery(&self, lastQueryTime: super::super::foundation::DateTime, out: *mut *mut StorageItemQueryResult) -> HRESULT
}}
impl IStorageLibraryContentChangedTriggerDetails {
    #[inline] pub unsafe fn get_folder(&self) -> Result<ComPtr<super::StorageFolder>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Folder)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn create_modified_since_query(&self, lastQueryTime: super::super::foundation::DateTime) -> Result<ComPtr<StorageItemQueryResult>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).CreateModifiedSinceQuery)(self as *const _ as *mut _, lastQueryTime, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
RT_CLASS!{class StorageLibraryContentChangedTriggerDetails: IStorageLibraryContentChangedTriggerDetails}
DEFINE_IID!(IID_IStorageQueryResultBase, 3264730893, 29523, 18347, 186, 88, 140, 97, 66, 93, 197, 75);
RT_INTERFACE!{interface IStorageQueryResultBase(IStorageQueryResultBaseVtbl): IInspectable(IInspectableVtbl) [IID_IStorageQueryResultBase] {
    fn GetItemCountAsync(&self, out: *mut *mut super::super::foundation::IAsyncOperation<u32>) -> HRESULT,
    fn get_Folder(&self, out: *mut *mut super::StorageFolder) -> HRESULT,
    fn add_ContentsChanged(&self, handler: *mut super::super::foundation::TypedEventHandler<IStorageQueryResultBase, IInspectable>, out: *mut super::super::foundation::EventRegistrationToken) -> HRESULT,
    fn remove_ContentsChanged(&self, eventCookie: super::super::foundation::EventRegistrationToken) -> HRESULT,
    fn add_OptionsChanged(&self, changedHandler: *mut super::super::foundation::TypedEventHandler<IStorageQueryResultBase, IInspectable>, out: *mut super::super::foundation::EventRegistrationToken) -> HRESULT,
    fn remove_OptionsChanged(&self, eventCookie: super::super::foundation::EventRegistrationToken) -> HRESULT,
    fn FindStartIndexAsync(&self, value: *mut IInspectable, out: *mut *mut super::super::foundation::IAsyncOperation<u32>) -> HRESULT,
    fn GetCurrentQueryOptions(&self, out: *mut *mut QueryOptions) -> HRESULT,
    fn ApplyNewQueryOptions(&self, newQueryOptions: *mut QueryOptions) -> HRESULT
}}
impl IStorageQueryResultBase {
    #[inline] pub unsafe fn get_item_count_async(&self) -> Result<ComPtr<super::super::foundation::IAsyncOperation<u32>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetItemCountAsync)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_folder(&self) -> Result<ComPtr<super::StorageFolder>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Folder)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn add_contents_changed(&self, handler: &super::super::foundation::TypedEventHandler<IStorageQueryResultBase, IInspectable>) -> Result<super::super::foundation::EventRegistrationToken> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).add_ContentsChanged)(self as *const _ as *mut _, handler as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn remove_contents_changed(&self, eventCookie: super::super::foundation::EventRegistrationToken) -> Result<()> {
        let hr = ((*self.lpVtbl).remove_ContentsChanged)(self as *const _ as *mut _, eventCookie);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn add_options_changed(&self, changedHandler: &super::super::foundation::TypedEventHandler<IStorageQueryResultBase, IInspectable>) -> Result<super::super::foundation::EventRegistrationToken> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).add_OptionsChanged)(self as *const _ as *mut _, changedHandler as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn remove_options_changed(&self, eventCookie: super::super::foundation::EventRegistrationToken) -> Result<()> {
        let hr = ((*self.lpVtbl).remove_OptionsChanged)(self as *const _ as *mut _, eventCookie);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn find_start_index_async(&self, value: &IInspectable) -> Result<ComPtr<super::super::foundation::IAsyncOperation<u32>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).FindStartIndexAsync)(self as *const _ as *mut _, value as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_current_query_options(&self) -> Result<ComPtr<QueryOptions>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetCurrentQueryOptions)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn apply_new_query_options(&self, newQueryOptions: &QueryOptions) -> Result<()> {
        let hr = ((*self.lpVtbl).ApplyNewQueryOptions)(self as *const _ as *mut _, newQueryOptions as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
}
DEFINE_IID!(IID_IValueAndLanguage, 3113306241, 41454, 19396, 146, 165, 70, 105, 104, 227, 4, 54);
RT_INTERFACE!{interface IValueAndLanguage(IValueAndLanguageVtbl): IInspectable(IInspectableVtbl) [IID_IValueAndLanguage] {
    fn get_Language(&self, out: *mut HSTRING) -> HRESULT,
    fn put_Language(&self, value: HSTRING) -> HRESULT,
    fn get_Value(&self, out: *mut *mut IInspectable) -> HRESULT,
    fn put_Value(&self, value: *mut IInspectable) -> HRESULT
}}
impl IValueAndLanguage {
    #[inline] pub unsafe fn get_language(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Language)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_language(&self, value: &HStringArg) -> Result<()> {
        let hr = ((*self.lpVtbl).put_Language)(self as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_value(&self) -> Result<ComPtr<IInspectable>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Value)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_value(&self, value: &IInspectable) -> Result<()> {
        let hr = ((*self.lpVtbl).put_Value)(self as *const _ as *mut _, value as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
}
RT_CLASS!{class ValueAndLanguage: IValueAndLanguage}
impl RtActivatable<IActivationFactory> for ValueAndLanguage {}
DEFINE_CLSID!(ValueAndLanguage(&[87,105,110,100,111,119,115,46,83,116,111,114,97,103,101,46,83,101,97,114,99,104,46,86,97,108,117,101,65,110,100,76,97,110,103,117,97,103,101,0]) [CLSID_ValueAndLanguage]);
} // Windows.Storage.Search
pub mod pickers { // Windows.Storage.Pickers
use ::prelude::*;
RT_CLASS!{class FileExtensionVector: super::super::foundation::collections::IVector<HString>}
DEFINE_IID!(IID_IFileOpenPicker, 749217674, 4805, 19551, 137, 119, 148, 84, 119, 147, 194, 65);
RT_INTERFACE!{interface IFileOpenPicker(IFileOpenPickerVtbl): IInspectable(IInspectableVtbl) [IID_IFileOpenPicker] {
    fn get_ViewMode(&self, out: *mut PickerViewMode) -> HRESULT,
    fn put_ViewMode(&self, value: PickerViewMode) -> HRESULT,
    fn get_SettingsIdentifier(&self, out: *mut HSTRING) -> HRESULT,
    fn put_SettingsIdentifier(&self, value: HSTRING) -> HRESULT,
    fn get_SuggestedStartLocation(&self, out: *mut PickerLocationId) -> HRESULT,
    fn put_SuggestedStartLocation(&self, value: PickerLocationId) -> HRESULT,
    fn get_CommitButtonText(&self, out: *mut HSTRING) -> HRESULT,
    fn put_CommitButtonText(&self, value: HSTRING) -> HRESULT,
    fn get_FileTypeFilter(&self, out: *mut *mut super::super::foundation::collections::IVector<HString>) -> HRESULT,
    fn PickSingleFileAsync(&self, out: *mut *mut super::super::foundation::IAsyncOperation<super::StorageFile>) -> HRESULT,
    fn PickMultipleFilesAsync(&self, out: *mut *mut super::super::foundation::IAsyncOperation<super::super::foundation::collections::IVectorView<super::StorageFile>>) -> HRESULT
}}
impl IFileOpenPicker {
    #[inline] pub unsafe fn get_view_mode(&self) -> Result<PickerViewMode> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_ViewMode)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_view_mode(&self, value: PickerViewMode) -> Result<()> {
        let hr = ((*self.lpVtbl).put_ViewMode)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_settings_identifier(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_SettingsIdentifier)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_settings_identifier(&self, value: &HStringArg) -> Result<()> {
        let hr = ((*self.lpVtbl).put_SettingsIdentifier)(self as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_suggested_start_location(&self) -> Result<PickerLocationId> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_SuggestedStartLocation)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_suggested_start_location(&self, value: PickerLocationId) -> Result<()> {
        let hr = ((*self.lpVtbl).put_SuggestedStartLocation)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_commit_button_text(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_CommitButtonText)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_commit_button_text(&self, value: &HStringArg) -> Result<()> {
        let hr = ((*self.lpVtbl).put_CommitButtonText)(self as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_file_type_filter(&self) -> Result<ComPtr<super::super::foundation::collections::IVector<HString>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_FileTypeFilter)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn pick_single_file_async(&self) -> Result<ComPtr<super::super::foundation::IAsyncOperation<super::StorageFile>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).PickSingleFileAsync)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn pick_multiple_files_async(&self) -> Result<ComPtr<super::super::foundation::IAsyncOperation<super::super::foundation::collections::IVectorView<super::StorageFile>>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).PickMultipleFilesAsync)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
RT_CLASS!{class FileOpenPicker: IFileOpenPicker}
impl RtActivatable<IFileOpenPickerStatics> for FileOpenPicker {}
impl RtActivatable<IActivationFactory> for FileOpenPicker {}
impl FileOpenPicker {
    #[inline] pub fn resume_pick_single_file_async() -> Result<ComPtr<super::super::foundation::IAsyncOperation<super::StorageFile>>> { unsafe {
        <Self as RtActivatable<IFileOpenPickerStatics>>::get_activation_factory().resume_pick_single_file_async()
    }}
}
DEFINE_CLSID!(FileOpenPicker(&[87,105,110,100,111,119,115,46,83,116,111,114,97,103,101,46,80,105,99,107,101,114,115,46,70,105,108,101,79,112,101,110,80,105,99,107,101,114,0]) [CLSID_FileOpenPicker]);
DEFINE_IID!(IID_IFileOpenPicker2, 2364239058, 46150, 18167, 178, 101, 144, 248, 229, 90, 214, 80);
RT_INTERFACE!{interface IFileOpenPicker2(IFileOpenPicker2Vtbl): IInspectable(IInspectableVtbl) [IID_IFileOpenPicker2] {
    fn get_ContinuationData(&self, out: *mut *mut super::super::foundation::collections::ValueSet) -> HRESULT,
    fn PickSingleFileAndContinue(&self) -> HRESULT,
    fn PickMultipleFilesAndContinue(&self) -> HRESULT
}}
impl IFileOpenPicker2 {
    #[inline] pub unsafe fn get_continuation_data(&self) -> Result<ComPtr<super::super::foundation::collections::ValueSet>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_ContinuationData)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn pick_single_file_and_continue(&self) -> Result<()> {
        let hr = ((*self.lpVtbl).PickSingleFileAndContinue)(self as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn pick_multiple_files_and_continue(&self) -> Result<()> {
        let hr = ((*self.lpVtbl).PickMultipleFilesAndContinue)(self as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
}
DEFINE_IID!(IID_IFileOpenPickerStatics, 1747015483, 12034, 18483, 150, 212, 171, 191, 173, 114, 182, 123);
RT_INTERFACE!{static interface IFileOpenPickerStatics(IFileOpenPickerStaticsVtbl): IInspectable(IInspectableVtbl) [IID_IFileOpenPickerStatics] {
    fn ResumePickSingleFileAsync(&self, out: *mut *mut super::super::foundation::IAsyncOperation<super::StorageFile>) -> HRESULT
}}
impl IFileOpenPickerStatics {
    #[inline] pub unsafe fn resume_pick_single_file_async(&self) -> Result<ComPtr<super::super::foundation::IAsyncOperation<super::StorageFile>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).ResumePickSingleFileAsync)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
DEFINE_IID!(IID_IFileOpenPickerWithOperationId, 1062712681, 9506, 19621, 170, 115, 161, 85, 9, 241, 252, 191);
RT_INTERFACE!{interface IFileOpenPickerWithOperationId(IFileOpenPickerWithOperationIdVtbl): IInspectable(IInspectableVtbl) [IID_IFileOpenPickerWithOperationId] {
    fn PickSingleFileAsync(&self, pickerOperationId: HSTRING, out: *mut *mut super::super::foundation::IAsyncOperation<super::StorageFile>) -> HRESULT
}}
impl IFileOpenPickerWithOperationId {
    #[inline] pub unsafe fn pick_single_file_async(&self, pickerOperationId: &HStringArg) -> Result<ComPtr<super::super::foundation::IAsyncOperation<super::StorageFile>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).PickSingleFileAsync)(self as *const _ as *mut _, pickerOperationId.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
RT_CLASS!{class FilePickerFileTypesOrderedMap: super::super::foundation::collections::IMap<HString, super::super::foundation::collections::IVector<HString>>}
RT_CLASS!{class FilePickerSelectedFilesArray: super::super::foundation::collections::IVectorView<super::StorageFile>}
DEFINE_IID!(IID_IFileSavePicker, 847708107, 24959, 19653, 175, 106, 179, 253, 242, 154, 209, 69);
RT_INTERFACE!{interface IFileSavePicker(IFileSavePickerVtbl): IInspectable(IInspectableVtbl) [IID_IFileSavePicker] {
    fn get_SettingsIdentifier(&self, out: *mut HSTRING) -> HRESULT,
    fn put_SettingsIdentifier(&self, value: HSTRING) -> HRESULT,
    fn get_SuggestedStartLocation(&self, out: *mut PickerLocationId) -> HRESULT,
    fn put_SuggestedStartLocation(&self, value: PickerLocationId) -> HRESULT,
    fn get_CommitButtonText(&self, out: *mut HSTRING) -> HRESULT,
    fn put_CommitButtonText(&self, value: HSTRING) -> HRESULT,
    fn get_FileTypeChoices(&self, out: *mut *mut super::super::foundation::collections::IMap<HString, super::super::foundation::collections::IVector<HString>>) -> HRESULT,
    fn get_DefaultFileExtension(&self, out: *mut HSTRING) -> HRESULT,
    fn put_DefaultFileExtension(&self, value: HSTRING) -> HRESULT,
    fn get_SuggestedSaveFile(&self, out: *mut *mut super::StorageFile) -> HRESULT,
    fn put_SuggestedSaveFile(&self, value: *mut super::StorageFile) -> HRESULT,
    fn get_SuggestedFileName(&self, out: *mut HSTRING) -> HRESULT,
    fn put_SuggestedFileName(&self, value: HSTRING) -> HRESULT,
    fn PickSaveFileAsync(&self, out: *mut *mut super::super::foundation::IAsyncOperation<super::StorageFile>) -> HRESULT
}}
impl IFileSavePicker {
    #[inline] pub unsafe fn get_settings_identifier(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_SettingsIdentifier)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_settings_identifier(&self, value: &HStringArg) -> Result<()> {
        let hr = ((*self.lpVtbl).put_SettingsIdentifier)(self as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_suggested_start_location(&self) -> Result<PickerLocationId> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_SuggestedStartLocation)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_suggested_start_location(&self, value: PickerLocationId) -> Result<()> {
        let hr = ((*self.lpVtbl).put_SuggestedStartLocation)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_commit_button_text(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_CommitButtonText)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_commit_button_text(&self, value: &HStringArg) -> Result<()> {
        let hr = ((*self.lpVtbl).put_CommitButtonText)(self as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_file_type_choices(&self) -> Result<ComPtr<super::super::foundation::collections::IMap<HString, super::super::foundation::collections::IVector<HString>>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_FileTypeChoices)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_default_file_extension(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_DefaultFileExtension)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_default_file_extension(&self, value: &HStringArg) -> Result<()> {
        let hr = ((*self.lpVtbl).put_DefaultFileExtension)(self as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_suggested_save_file(&self) -> Result<ComPtr<super::StorageFile>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_SuggestedSaveFile)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_suggested_save_file(&self, value: &super::StorageFile) -> Result<()> {
        let hr = ((*self.lpVtbl).put_SuggestedSaveFile)(self as *const _ as *mut _, value as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_suggested_file_name(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_SuggestedFileName)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_suggested_file_name(&self, value: &HStringArg) -> Result<()> {
        let hr = ((*self.lpVtbl).put_SuggestedFileName)(self as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn pick_save_file_async(&self) -> Result<ComPtr<super::super::foundation::IAsyncOperation<super::StorageFile>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).PickSaveFileAsync)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
RT_CLASS!{class FileSavePicker: IFileSavePicker}
impl RtActivatable<IActivationFactory> for FileSavePicker {}
DEFINE_CLSID!(FileSavePicker(&[87,105,110,100,111,119,115,46,83,116,111,114,97,103,101,46,80,105,99,107,101,114,115,46,70,105,108,101,83,97,118,101,80,105,99,107,101,114,0]) [CLSID_FileSavePicker]);
DEFINE_IID!(IID_IFileSavePicker2, 247665570, 53835, 17562, 129, 151, 232, 145, 4, 253, 66, 204);
RT_INTERFACE!{interface IFileSavePicker2(IFileSavePicker2Vtbl): IInspectable(IInspectableVtbl) [IID_IFileSavePicker2] {
    fn get_ContinuationData(&self, out: *mut *mut super::super::foundation::collections::ValueSet) -> HRESULT,
    fn PickSaveFileAndContinue(&self) -> HRESULT
}}
impl IFileSavePicker2 {
    #[inline] pub unsafe fn get_continuation_data(&self) -> Result<ComPtr<super::super::foundation::collections::ValueSet>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_ContinuationData)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn pick_save_file_and_continue(&self) -> Result<()> {
        let hr = ((*self.lpVtbl).PickSaveFileAndContinue)(self as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
}
DEFINE_IID!(IID_IFileSavePicker3, 1770712169, 47676, 20049, 189, 144, 74, 188, 187, 244, 207, 175);
RT_INTERFACE!{interface IFileSavePicker3(IFileSavePicker3Vtbl): IInspectable(IInspectableVtbl) [IID_IFileSavePicker3] {
    fn get_EnterpriseId(&self, out: *mut HSTRING) -> HRESULT,
    fn put_EnterpriseId(&self, value: HSTRING) -> HRESULT
}}
impl IFileSavePicker3 {
    #[inline] pub unsafe fn get_enterprise_id(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_EnterpriseId)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_enterprise_id(&self, value: &HStringArg) -> Result<()> {
        let hr = ((*self.lpVtbl).put_EnterpriseId)(self as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
}
DEFINE_IID!(IID_IFolderPicker, 139425689, 62459, 16394, 153, 177, 123, 74, 119, 47, 214, 13);
RT_INTERFACE!{interface IFolderPicker(IFolderPickerVtbl): IInspectable(IInspectableVtbl) [IID_IFolderPicker] {
    fn get_ViewMode(&self, out: *mut PickerViewMode) -> HRESULT,
    fn put_ViewMode(&self, value: PickerViewMode) -> HRESULT,
    fn get_SettingsIdentifier(&self, out: *mut HSTRING) -> HRESULT,
    fn put_SettingsIdentifier(&self, value: HSTRING) -> HRESULT,
    fn get_SuggestedStartLocation(&self, out: *mut PickerLocationId) -> HRESULT,
    fn put_SuggestedStartLocation(&self, value: PickerLocationId) -> HRESULT,
    fn get_CommitButtonText(&self, out: *mut HSTRING) -> HRESULT,
    fn put_CommitButtonText(&self, value: HSTRING) -> HRESULT,
    fn get_FileTypeFilter(&self, out: *mut *mut super::super::foundation::collections::IVector<HString>) -> HRESULT,
    fn PickSingleFolderAsync(&self, out: *mut *mut super::super::foundation::IAsyncOperation<super::StorageFolder>) -> HRESULT
}}
impl IFolderPicker {
    #[inline] pub unsafe fn get_view_mode(&self) -> Result<PickerViewMode> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_ViewMode)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_view_mode(&self, value: PickerViewMode) -> Result<()> {
        let hr = ((*self.lpVtbl).put_ViewMode)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_settings_identifier(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_SettingsIdentifier)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_settings_identifier(&self, value: &HStringArg) -> Result<()> {
        let hr = ((*self.lpVtbl).put_SettingsIdentifier)(self as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_suggested_start_location(&self) -> Result<PickerLocationId> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_SuggestedStartLocation)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_suggested_start_location(&self, value: PickerLocationId) -> Result<()> {
        let hr = ((*self.lpVtbl).put_SuggestedStartLocation)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_commit_button_text(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_CommitButtonText)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_commit_button_text(&self, value: &HStringArg) -> Result<()> {
        let hr = ((*self.lpVtbl).put_CommitButtonText)(self as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_file_type_filter(&self) -> Result<ComPtr<super::super::foundation::collections::IVector<HString>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_FileTypeFilter)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn pick_single_folder_async(&self) -> Result<ComPtr<super::super::foundation::IAsyncOperation<super::StorageFolder>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).PickSingleFolderAsync)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
RT_CLASS!{class FolderPicker: IFolderPicker}
impl RtActivatable<IActivationFactory> for FolderPicker {}
DEFINE_CLSID!(FolderPicker(&[87,105,110,100,111,119,115,46,83,116,111,114,97,103,101,46,80,105,99,107,101,114,115,46,70,111,108,100,101,114,80,105,99,107,101,114,0]) [CLSID_FolderPicker]);
DEFINE_IID!(IID_IFolderPicker2, 2394143383, 56453, 17942, 190, 148, 150, 96, 136, 31, 47, 93);
RT_INTERFACE!{interface IFolderPicker2(IFolderPicker2Vtbl): IInspectable(IInspectableVtbl) [IID_IFolderPicker2] {
    fn get_ContinuationData(&self, out: *mut *mut super::super::foundation::collections::ValueSet) -> HRESULT,
    fn PickFolderAndContinue(&self) -> HRESULT
}}
impl IFolderPicker2 {
    #[inline] pub unsafe fn get_continuation_data(&self) -> Result<ComPtr<super::super::foundation::collections::ValueSet>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_ContinuationData)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn pick_folder_and_continue(&self) -> Result<()> {
        let hr = ((*self.lpVtbl).PickFolderAndContinue)(self as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
}
RT_ENUM! { enum PickerLocationId: i32 {
    DocumentsLibrary (PickerLocationId_DocumentsLibrary) = 0, ComputerFolder (PickerLocationId_ComputerFolder) = 1, Desktop (PickerLocationId_Desktop) = 2, Downloads (PickerLocationId_Downloads) = 3, HomeGroup (PickerLocationId_HomeGroup) = 4, MusicLibrary (PickerLocationId_MusicLibrary) = 5, PicturesLibrary (PickerLocationId_PicturesLibrary) = 6, VideosLibrary (PickerLocationId_VideosLibrary) = 7, Objects3D (PickerLocationId_Objects3D) = 8, Unspecified (PickerLocationId_Unspecified) = 9,
}}
RT_ENUM! { enum PickerViewMode: i32 {
    List (PickerViewMode_List) = 0, Thumbnail (PickerViewMode_Thumbnail) = 1,
}}
pub mod provider { // Windows.Storage.Pickers.Provider
use ::prelude::*;
RT_ENUM! { enum AddFileResult: i32 {
    Added (AddFileResult_Added) = 0, AlreadyAdded (AddFileResult_AlreadyAdded) = 1, NotAllowed (AddFileResult_NotAllowed) = 2, Unavailable (AddFileResult_Unavailable) = 3,
}}
DEFINE_IID!(IID_IFileOpenPickerUI, 3718535696, 63956, 16580, 138, 245, 197, 182, 181, 166, 29, 29);
RT_INTERFACE!{interface IFileOpenPickerUI(IFileOpenPickerUIVtbl): IInspectable(IInspectableVtbl) [IID_IFileOpenPickerUI] {
    fn AddFile(&self, id: HSTRING, file: *mut super::super::IStorageFile, out: *mut AddFileResult) -> HRESULT,
    fn RemoveFile(&self, id: HSTRING) -> HRESULT,
    fn ContainsFile(&self, id: HSTRING, out: *mut bool) -> HRESULT,
    fn CanAddFile(&self, file: *mut super::super::IStorageFile, out: *mut bool) -> HRESULT,
    fn get_AllowedFileTypes(&self, out: *mut *mut ::rt::gen::windows::foundation::collections::IVectorView<HString>) -> HRESULT,
    fn get_SelectionMode(&self, out: *mut FileSelectionMode) -> HRESULT,
    fn get_SettingsIdentifier(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Title(&self, out: *mut HSTRING) -> HRESULT,
    fn put_Title(&self, value: HSTRING) -> HRESULT,
    fn add_FileRemoved(&self, handler: *mut ::rt::gen::windows::foundation::TypedEventHandler<FileOpenPickerUI, FileRemovedEventArgs>, out: *mut ::rt::gen::windows::foundation::EventRegistrationToken) -> HRESULT,
    fn remove_FileRemoved(&self, token: ::rt::gen::windows::foundation::EventRegistrationToken) -> HRESULT,
    fn add_Closing(&self, handler: *mut ::rt::gen::windows::foundation::TypedEventHandler<FileOpenPickerUI, PickerClosingEventArgs>, out: *mut ::rt::gen::windows::foundation::EventRegistrationToken) -> HRESULT,
    fn remove_Closing(&self, token: ::rt::gen::windows::foundation::EventRegistrationToken) -> HRESULT
}}
impl IFileOpenPickerUI {
    #[inline] pub unsafe fn add_file(&self, id: &HStringArg, file: &super::super::IStorageFile) -> Result<AddFileResult> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).AddFile)(self as *const _ as *mut _, id.get(), file as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn remove_file(&self, id: &HStringArg) -> Result<()> {
        let hr = ((*self.lpVtbl).RemoveFile)(self as *const _ as *mut _, id.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn contains_file(&self, id: &HStringArg) -> Result<bool> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).ContainsFile)(self as *const _ as *mut _, id.get(), &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn can_add_file(&self, file: &super::super::IStorageFile) -> Result<bool> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).CanAddFile)(self as *const _ as *mut _, file as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_allowed_file_types(&self) -> Result<ComPtr<::rt::gen::windows::foundation::collections::IVectorView<HString>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_AllowedFileTypes)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_selection_mode(&self) -> Result<FileSelectionMode> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_SelectionMode)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_settings_identifier(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_SettingsIdentifier)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
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
    #[inline] pub unsafe fn add_file_removed(&self, handler: &::rt::gen::windows::foundation::TypedEventHandler<FileOpenPickerUI, FileRemovedEventArgs>) -> Result<::rt::gen::windows::foundation::EventRegistrationToken> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).add_FileRemoved)(self as *const _ as *mut _, handler as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn remove_file_removed(&self, token: ::rt::gen::windows::foundation::EventRegistrationToken) -> Result<()> {
        let hr = ((*self.lpVtbl).remove_FileRemoved)(self as *const _ as *mut _, token);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn add_closing(&self, handler: &::rt::gen::windows::foundation::TypedEventHandler<FileOpenPickerUI, PickerClosingEventArgs>) -> Result<::rt::gen::windows::foundation::EventRegistrationToken> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).add_Closing)(self as *const _ as *mut _, handler as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn remove_closing(&self, token: ::rt::gen::windows::foundation::EventRegistrationToken) -> Result<()> {
        let hr = ((*self.lpVtbl).remove_Closing)(self as *const _ as *mut _, token);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
}
RT_CLASS!{class FileOpenPickerUI: IFileOpenPickerUI}
DEFINE_IID!(IID_IFileRemovedEventArgs, 319045031, 32714, 19499, 158, 202, 104, 144, 249, 240, 1, 133);
RT_INTERFACE!{interface IFileRemovedEventArgs(IFileRemovedEventArgsVtbl): IInspectable(IInspectableVtbl) [IID_IFileRemovedEventArgs] {
    fn get_Id(&self, out: *mut HSTRING) -> HRESULT
}}
impl IFileRemovedEventArgs {
    #[inline] pub unsafe fn get_id(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Id)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
}
RT_CLASS!{class FileRemovedEventArgs: IFileRemovedEventArgs}
DEFINE_IID!(IID_IFileSavePickerUI, 2522268135, 15958, 17356, 138, 57, 51, 199, 61, 157, 84, 43);
RT_INTERFACE!{interface IFileSavePickerUI(IFileSavePickerUIVtbl): IInspectable(IInspectableVtbl) [IID_IFileSavePickerUI] {
    fn get_Title(&self, out: *mut HSTRING) -> HRESULT,
    fn put_Title(&self, value: HSTRING) -> HRESULT,
    fn get_AllowedFileTypes(&self, out: *mut *mut ::rt::gen::windows::foundation::collections::IVectorView<HString>) -> HRESULT,
    fn get_SettingsIdentifier(&self, out: *mut HSTRING) -> HRESULT,
    fn get_FileName(&self, out: *mut HSTRING) -> HRESULT,
    fn TrySetFileName(&self, value: HSTRING, out: *mut SetFileNameResult) -> HRESULT,
    fn add_FileNameChanged(&self, handler: *mut ::rt::gen::windows::foundation::TypedEventHandler<FileSavePickerUI, IInspectable>, out: *mut ::rt::gen::windows::foundation::EventRegistrationToken) -> HRESULT,
    fn remove_FileNameChanged(&self, token: ::rt::gen::windows::foundation::EventRegistrationToken) -> HRESULT,
    fn add_TargetFileRequested(&self, handler: *mut ::rt::gen::windows::foundation::TypedEventHandler<FileSavePickerUI, TargetFileRequestedEventArgs>, out: *mut ::rt::gen::windows::foundation::EventRegistrationToken) -> HRESULT,
    fn remove_TargetFileRequested(&self, token: ::rt::gen::windows::foundation::EventRegistrationToken) -> HRESULT
}}
impl IFileSavePickerUI {
    #[inline] pub unsafe fn get_title(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Title)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_title(&self, value: &HStringArg) -> Result<()> {
        let hr = ((*self.lpVtbl).put_Title)(self as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_allowed_file_types(&self) -> Result<ComPtr<::rt::gen::windows::foundation::collections::IVectorView<HString>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_AllowedFileTypes)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_settings_identifier(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_SettingsIdentifier)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_file_name(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_FileName)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn try_set_file_name(&self, value: &HStringArg) -> Result<SetFileNameResult> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).TrySetFileName)(self as *const _ as *mut _, value.get(), &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn add_file_name_changed(&self, handler: &::rt::gen::windows::foundation::TypedEventHandler<FileSavePickerUI, IInspectable>) -> Result<::rt::gen::windows::foundation::EventRegistrationToken> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).add_FileNameChanged)(self as *const _ as *mut _, handler as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn remove_file_name_changed(&self, token: ::rt::gen::windows::foundation::EventRegistrationToken) -> Result<()> {
        let hr = ((*self.lpVtbl).remove_FileNameChanged)(self as *const _ as *mut _, token);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn add_target_file_requested(&self, handler: &::rt::gen::windows::foundation::TypedEventHandler<FileSavePickerUI, TargetFileRequestedEventArgs>) -> Result<::rt::gen::windows::foundation::EventRegistrationToken> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).add_TargetFileRequested)(self as *const _ as *mut _, handler as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn remove_target_file_requested(&self, token: ::rt::gen::windows::foundation::EventRegistrationToken) -> Result<()> {
        let hr = ((*self.lpVtbl).remove_TargetFileRequested)(self as *const _ as *mut _, token);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
}
RT_CLASS!{class FileSavePickerUI: IFileSavePickerUI}
RT_ENUM! { enum FileSelectionMode: i32 {
    Single (FileSelectionMode_Single) = 0, Multiple (FileSelectionMode_Multiple) = 1,
}}
DEFINE_IID!(IID_IPickerClosingDeferral, 2063071006, 6759, 18993, 174, 128, 233, 7, 112, 138, 97, 155);
RT_INTERFACE!{interface IPickerClosingDeferral(IPickerClosingDeferralVtbl): IInspectable(IInspectableVtbl) [IID_IPickerClosingDeferral] {
    fn Complete(&self) -> HRESULT
}}
impl IPickerClosingDeferral {
    #[inline] pub unsafe fn complete(&self) -> Result<()> {
        let hr = ((*self.lpVtbl).Complete)(self as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
}
RT_CLASS!{class PickerClosingDeferral: IPickerClosingDeferral}
DEFINE_IID!(IID_IPickerClosingEventArgs, 2119823908, 45874, 20242, 139, 159, 168, 194, 240, 107, 50, 205);
RT_INTERFACE!{interface IPickerClosingEventArgs(IPickerClosingEventArgsVtbl): IInspectable(IInspectableVtbl) [IID_IPickerClosingEventArgs] {
    fn get_ClosingOperation(&self, out: *mut *mut PickerClosingOperation) -> HRESULT,
    fn get_IsCanceled(&self, out: *mut bool) -> HRESULT
}}
impl IPickerClosingEventArgs {
    #[inline] pub unsafe fn get_closing_operation(&self) -> Result<ComPtr<PickerClosingOperation>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_ClosingOperation)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_is_canceled(&self) -> Result<bool> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_IsCanceled)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
}
RT_CLASS!{class PickerClosingEventArgs: IPickerClosingEventArgs}
DEFINE_IID!(IID_IPickerClosingOperation, 1290402692, 48878, 20025, 167, 115, 252, 95, 14, 174, 50, 141);
RT_INTERFACE!{interface IPickerClosingOperation(IPickerClosingOperationVtbl): IInspectable(IInspectableVtbl) [IID_IPickerClosingOperation] {
    fn GetDeferral(&self, out: *mut *mut PickerClosingDeferral) -> HRESULT,
    fn get_Deadline(&self, out: *mut ::rt::gen::windows::foundation::DateTime) -> HRESULT
}}
impl IPickerClosingOperation {
    #[inline] pub unsafe fn get_deferral(&self) -> Result<ComPtr<PickerClosingDeferral>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetDeferral)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_deadline(&self) -> Result<::rt::gen::windows::foundation::DateTime> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_Deadline)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
}
RT_CLASS!{class PickerClosingOperation: IPickerClosingOperation}
RT_ENUM! { enum SetFileNameResult: i32 {
    Succeeded (SetFileNameResult_Succeeded) = 0, NotAllowed (SetFileNameResult_NotAllowed) = 1, Unavailable (SetFileNameResult_Unavailable) = 2,
}}
DEFINE_IID!(IID_ITargetFileRequest, 1119695701, 32648, 18315, 142, 129, 105, 11, 32, 52, 6, 120);
RT_INTERFACE!{interface ITargetFileRequest(ITargetFileRequestVtbl): IInspectable(IInspectableVtbl) [IID_ITargetFileRequest] {
    fn get_TargetFile(&self, out: *mut *mut super::super::IStorageFile) -> HRESULT,
    fn put_TargetFile(&self, value: *mut super::super::IStorageFile) -> HRESULT,
    fn GetDeferral(&self, out: *mut *mut TargetFileRequestDeferral) -> HRESULT
}}
impl ITargetFileRequest {
    #[inline] pub unsafe fn get_target_file(&self) -> Result<ComPtr<super::super::IStorageFile>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_TargetFile)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_target_file(&self, value: &super::super::IStorageFile) -> Result<()> {
        let hr = ((*self.lpVtbl).put_TargetFile)(self as *const _ as *mut _, value as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_deferral(&self) -> Result<ComPtr<TargetFileRequestDeferral>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetDeferral)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
RT_CLASS!{class TargetFileRequest: ITargetFileRequest}
DEFINE_IID!(IID_ITargetFileRequestDeferral, 1257151889, 48917, 19881, 149, 246, 246, 183, 213, 88, 34, 91);
RT_INTERFACE!{interface ITargetFileRequestDeferral(ITargetFileRequestDeferralVtbl): IInspectable(IInspectableVtbl) [IID_ITargetFileRequestDeferral] {
    fn Complete(&self) -> HRESULT
}}
impl ITargetFileRequestDeferral {
    #[inline] pub unsafe fn complete(&self) -> Result<()> {
        let hr = ((*self.lpVtbl).Complete)(self as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
}
RT_CLASS!{class TargetFileRequestDeferral: ITargetFileRequestDeferral}
DEFINE_IID!(IID_ITargetFileRequestedEventArgs, 2976111553, 6993, 19593, 165, 145, 15, 212, 11, 60, 87, 201);
RT_INTERFACE!{interface ITargetFileRequestedEventArgs(ITargetFileRequestedEventArgsVtbl): IInspectable(IInspectableVtbl) [IID_ITargetFileRequestedEventArgs] {
    fn get_Request(&self, out: *mut *mut TargetFileRequest) -> HRESULT
}}
impl ITargetFileRequestedEventArgs {
    #[inline] pub unsafe fn get_request(&self) -> Result<ComPtr<TargetFileRequest>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Request)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
RT_CLASS!{class TargetFileRequestedEventArgs: ITargetFileRequestedEventArgs}
} // Windows.Storage.Pickers.Provider
} // Windows.Storage.Pickers
pub mod provider { // Windows.Storage.Provider
use ::prelude::*;
RT_ENUM! { enum CachedFileOptions: u32 {
    None (CachedFileOptions_None) = 0, RequireUpdateOnAccess (CachedFileOptions_RequireUpdateOnAccess) = 1, UseCachedFileWhenOffline (CachedFileOptions_UseCachedFileWhenOffline) = 2, DenyAccessWhenOffline (CachedFileOptions_DenyAccessWhenOffline) = 4,
}}
RT_ENUM! { enum CachedFileTarget: i32 {
    Local (CachedFileTarget_Local) = 0, Remote (CachedFileTarget_Remote) = 1,
}}
RT_CLASS!{static class CachedFileUpdater}
impl RtActivatable<ICachedFileUpdaterStatics> for CachedFileUpdater {}
impl CachedFileUpdater {
    #[inline] pub fn set_update_information(file: &super::IStorageFile, contentId: &HStringArg, readMode: ReadActivationMode, writeMode: WriteActivationMode, options: CachedFileOptions) -> Result<()> { unsafe {
        <Self as RtActivatable<ICachedFileUpdaterStatics>>::get_activation_factory().set_update_information(file, contentId, readMode, writeMode, options)
    }}
}
DEFINE_CLSID!(CachedFileUpdater(&[87,105,110,100,111,119,115,46,83,116,111,114,97,103,101,46,80,114,111,118,105,100,101,114,46,67,97,99,104,101,100,70,105,108,101,85,112,100,97,116,101,114,0]) [CLSID_CachedFileUpdater]);
DEFINE_IID!(IID_ICachedFileUpdaterStatics, 2680752416, 31695, 18568, 168, 30, 16, 45, 112, 52, 215, 206);
RT_INTERFACE!{static interface ICachedFileUpdaterStatics(ICachedFileUpdaterStaticsVtbl): IInspectable(IInspectableVtbl) [IID_ICachedFileUpdaterStatics] {
    fn SetUpdateInformation(&self, file: *mut super::IStorageFile, contentId: HSTRING, readMode: ReadActivationMode, writeMode: WriteActivationMode, options: CachedFileOptions) -> HRESULT
}}
impl ICachedFileUpdaterStatics {
    #[inline] pub unsafe fn set_update_information(&self, file: &super::IStorageFile, contentId: &HStringArg, readMode: ReadActivationMode, writeMode: WriteActivationMode, options: CachedFileOptions) -> Result<()> {
        let hr = ((*self.lpVtbl).SetUpdateInformation)(self as *const _ as *mut _, file as *const _ as *mut _, contentId.get(), readMode, writeMode, options);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
}
DEFINE_IID!(IID_ICachedFileUpdaterUI, 2658091494, 47858, 19095, 182, 0, 147, 51, 245, 223, 128, 253);
RT_INTERFACE!{interface ICachedFileUpdaterUI(ICachedFileUpdaterUIVtbl): IInspectable(IInspectableVtbl) [IID_ICachedFileUpdaterUI] {
    fn get_Title(&self, out: *mut HSTRING) -> HRESULT,
    fn put_Title(&self, value: HSTRING) -> HRESULT,
    fn get_UpdateTarget(&self, out: *mut CachedFileTarget) -> HRESULT,
    fn add_FileUpdateRequested(&self, handler: *mut super::super::foundation::TypedEventHandler<CachedFileUpdaterUI, FileUpdateRequestedEventArgs>, out: *mut super::super::foundation::EventRegistrationToken) -> HRESULT,
    fn remove_FileUpdateRequested(&self, token: super::super::foundation::EventRegistrationToken) -> HRESULT,
    fn add_UIRequested(&self, handler: *mut super::super::foundation::TypedEventHandler<CachedFileUpdaterUI, IInspectable>, out: *mut super::super::foundation::EventRegistrationToken) -> HRESULT,
    fn remove_UIRequested(&self, token: super::super::foundation::EventRegistrationToken) -> HRESULT,
    fn get_UIStatus(&self, out: *mut UIStatus) -> HRESULT
}}
impl ICachedFileUpdaterUI {
    #[inline] pub unsafe fn get_title(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Title)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_title(&self, value: &HStringArg) -> Result<()> {
        let hr = ((*self.lpVtbl).put_Title)(self as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_update_target(&self) -> Result<CachedFileTarget> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_UpdateTarget)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn add_file_update_requested(&self, handler: &super::super::foundation::TypedEventHandler<CachedFileUpdaterUI, FileUpdateRequestedEventArgs>) -> Result<super::super::foundation::EventRegistrationToken> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).add_FileUpdateRequested)(self as *const _ as *mut _, handler as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn remove_file_update_requested(&self, token: super::super::foundation::EventRegistrationToken) -> Result<()> {
        let hr = ((*self.lpVtbl).remove_FileUpdateRequested)(self as *const _ as *mut _, token);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn add_uirequested(&self, handler: &super::super::foundation::TypedEventHandler<CachedFileUpdaterUI, IInspectable>) -> Result<super::super::foundation::EventRegistrationToken> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).add_UIRequested)(self as *const _ as *mut _, handler as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn remove_uirequested(&self, token: super::super::foundation::EventRegistrationToken) -> Result<()> {
        let hr = ((*self.lpVtbl).remove_UIRequested)(self as *const _ as *mut _, token);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_uistatus(&self) -> Result<UIStatus> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_UIStatus)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
}
RT_CLASS!{class CachedFileUpdaterUI: ICachedFileUpdaterUI}
DEFINE_IID!(IID_ICachedFileUpdaterUI2, 2287378972, 34457, 17216, 159, 73, 247, 202, 215, 254, 137, 145);
RT_INTERFACE!{interface ICachedFileUpdaterUI2(ICachedFileUpdaterUI2Vtbl): IInspectable(IInspectableVtbl) [IID_ICachedFileUpdaterUI2] {
    fn get_UpdateRequest(&self, out: *mut *mut FileUpdateRequest) -> HRESULT,
    fn GetDeferral(&self, out: *mut *mut FileUpdateRequestDeferral) -> HRESULT
}}
impl ICachedFileUpdaterUI2 {
    #[inline] pub unsafe fn get_update_request(&self) -> Result<ComPtr<FileUpdateRequest>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_UpdateRequest)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_deferral(&self) -> Result<ComPtr<FileUpdateRequestDeferral>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetDeferral)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
DEFINE_IID!(IID_IFileUpdateRequest, 1086858550, 49662, 19859, 167, 146, 30, 115, 107, 199, 8, 55);
RT_INTERFACE!{interface IFileUpdateRequest(IFileUpdateRequestVtbl): IInspectable(IInspectableVtbl) [IID_IFileUpdateRequest] {
    fn get_ContentId(&self, out: *mut HSTRING) -> HRESULT,
    fn get_File(&self, out: *mut *mut super::StorageFile) -> HRESULT,
    fn get_Status(&self, out: *mut FileUpdateStatus) -> HRESULT,
    fn put_Status(&self, value: FileUpdateStatus) -> HRESULT,
    fn GetDeferral(&self, out: *mut *mut FileUpdateRequestDeferral) -> HRESULT,
    fn UpdateLocalFile(&self, value: *mut super::IStorageFile) -> HRESULT
}}
impl IFileUpdateRequest {
    #[inline] pub unsafe fn get_content_id(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_ContentId)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_file(&self) -> Result<ComPtr<super::StorageFile>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_File)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_status(&self) -> Result<FileUpdateStatus> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_Status)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_status(&self, value: FileUpdateStatus) -> Result<()> {
        let hr = ((*self.lpVtbl).put_Status)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_deferral(&self) -> Result<ComPtr<FileUpdateRequestDeferral>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetDeferral)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn update_local_file(&self, value: &super::IStorageFile) -> Result<()> {
        let hr = ((*self.lpVtbl).UpdateLocalFile)(self as *const _ as *mut _, value as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
}
RT_CLASS!{class FileUpdateRequest: IFileUpdateRequest}
DEFINE_IID!(IID_IFileUpdateRequest2, 2185774664, 48574, 17531, 162, 238, 122, 254, 106, 3, 42, 148);
RT_INTERFACE!{interface IFileUpdateRequest2(IFileUpdateRequest2Vtbl): IInspectable(IInspectableVtbl) [IID_IFileUpdateRequest2] {
    fn get_UserInputNeededMessage(&self, out: *mut HSTRING) -> HRESULT,
    fn put_UserInputNeededMessage(&self, value: HSTRING) -> HRESULT
}}
impl IFileUpdateRequest2 {
    #[inline] pub unsafe fn get_user_input_needed_message(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_UserInputNeededMessage)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_user_input_needed_message(&self, value: &HStringArg) -> Result<()> {
        let hr = ((*self.lpVtbl).put_UserInputNeededMessage)(self as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
}
DEFINE_IID!(IID_IFileUpdateRequestDeferral, 4291746603, 35550, 17573, 187, 0, 22, 76, 78, 114, 241, 58);
RT_INTERFACE!{interface IFileUpdateRequestDeferral(IFileUpdateRequestDeferralVtbl): IInspectable(IInspectableVtbl) [IID_IFileUpdateRequestDeferral] {
    fn Complete(&self) -> HRESULT
}}
impl IFileUpdateRequestDeferral {
    #[inline] pub unsafe fn complete(&self) -> Result<()> {
        let hr = ((*self.lpVtbl).Complete)(self as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
}
RT_CLASS!{class FileUpdateRequestDeferral: IFileUpdateRequestDeferral}
DEFINE_IID!(IID_IFileUpdateRequestedEventArgs, 2064290626, 14597, 17293, 170, 239, 120, 174, 38, 95, 141, 210);
RT_INTERFACE!{interface IFileUpdateRequestedEventArgs(IFileUpdateRequestedEventArgsVtbl): IInspectable(IInspectableVtbl) [IID_IFileUpdateRequestedEventArgs] {
    fn get_Request(&self, out: *mut *mut FileUpdateRequest) -> HRESULT
}}
impl IFileUpdateRequestedEventArgs {
    #[inline] pub unsafe fn get_request(&self) -> Result<ComPtr<FileUpdateRequest>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Request)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
RT_CLASS!{class FileUpdateRequestedEventArgs: IFileUpdateRequestedEventArgs}
RT_ENUM! { enum FileUpdateStatus: i32 {
    Incomplete (FileUpdateStatus_Incomplete) = 0, Complete (FileUpdateStatus_Complete) = 1, UserInputNeeded (FileUpdateStatus_UserInputNeeded) = 2, CurrentlyUnavailable (FileUpdateStatus_CurrentlyUnavailable) = 3, Failed (FileUpdateStatus_Failed) = 4, CompleteAndRenamed (FileUpdateStatus_CompleteAndRenamed) = 5,
}}
RT_ENUM! { enum ReadActivationMode: i32 {
    NotNeeded (ReadActivationMode_NotNeeded) = 0, BeforeAccess (ReadActivationMode_BeforeAccess) = 1,
}}
RT_ENUM! { enum StorageProviderHardlinkPolicy: u32 {
    None (StorageProviderHardlinkPolicy_None) = 0, Allowed (StorageProviderHardlinkPolicy_Allowed) = 1,
}}
RT_ENUM! { enum StorageProviderHydrationPolicy: i32 {
    Partial (StorageProviderHydrationPolicy_Partial) = 0, Progressive (StorageProviderHydrationPolicy_Progressive) = 1, Full (StorageProviderHydrationPolicy_Full) = 2, AlwaysFull (StorageProviderHydrationPolicy_AlwaysFull) = 3,
}}
RT_ENUM! { enum StorageProviderHydrationPolicyModifier: u32 {
    None (StorageProviderHydrationPolicyModifier_None) = 0, ValidationRequired (StorageProviderHydrationPolicyModifier_ValidationRequired) = 1, StreamingAllowed (StorageProviderHydrationPolicyModifier_StreamingAllowed) = 2,
}}
RT_ENUM! { enum StorageProviderInSyncPolicy: u32 {
    Default (StorageProviderInSyncPolicy_Default) = 0, FileCreationTime (StorageProviderInSyncPolicy_FileCreationTime) = 1, FileReadOnlyAttribute (StorageProviderInSyncPolicy_FileReadOnlyAttribute) = 2, FileHiddenAttribute (StorageProviderInSyncPolicy_FileHiddenAttribute) = 4, FileSystemAttribute (StorageProviderInSyncPolicy_FileSystemAttribute) = 8, DirectoryCreationTime (StorageProviderInSyncPolicy_DirectoryCreationTime) = 16, DirectoryReadOnlyAttribute (StorageProviderInSyncPolicy_DirectoryReadOnlyAttribute) = 32, DirectoryHiddenAttribute (StorageProviderInSyncPolicy_DirectoryHiddenAttribute) = 64, DirectorySystemAttribute (StorageProviderInSyncPolicy_DirectorySystemAttribute) = 128, FileLastWriteTime (StorageProviderInSyncPolicy_FileLastWriteTime) = 256, DirectoryLastWriteTime (StorageProviderInSyncPolicy_DirectoryLastWriteTime) = 512, PreserveInsyncForSyncEngine (StorageProviderInSyncPolicy_PreserveInsyncForSyncEngine) = 2147483648,
}}
RT_CLASS!{static class StorageProviderItemProperties}
impl RtActivatable<IStorageProviderItemPropertiesStatics> for StorageProviderItemProperties {}
impl StorageProviderItemProperties {
    #[inline] pub fn set_async(item: &super::IStorageItem, itemProperties: &super::super::foundation::collections::IIterable<StorageProviderItemProperty>) -> Result<ComPtr<super::super::foundation::IAsyncAction>> { unsafe {
        <Self as RtActivatable<IStorageProviderItemPropertiesStatics>>::get_activation_factory().set_async(item, itemProperties)
    }}
}
DEFINE_CLSID!(StorageProviderItemProperties(&[87,105,110,100,111,119,115,46,83,116,111,114,97,103,101,46,80,114,111,118,105,100,101,114,46,83,116,111,114,97,103,101,80,114,111,118,105,100,101,114,73,116,101,109,80,114,111,112,101,114,116,105,101,115,0]) [CLSID_StorageProviderItemProperties]);
DEFINE_IID!(IID_IStorageProviderItemPropertiesStatics, 757865623, 9988, 18217, 143, 169, 126, 107, 142, 21, 140, 47);
RT_INTERFACE!{static interface IStorageProviderItemPropertiesStatics(IStorageProviderItemPropertiesStaticsVtbl): IInspectable(IInspectableVtbl) [IID_IStorageProviderItemPropertiesStatics] {
    fn SetAsync(&self, item: *mut super::IStorageItem, itemProperties: *mut super::super::foundation::collections::IIterable<StorageProviderItemProperty>, out: *mut *mut super::super::foundation::IAsyncAction) -> HRESULT
}}
impl IStorageProviderItemPropertiesStatics {
    #[inline] pub unsafe fn set_async(&self, item: &super::IStorageItem, itemProperties: &super::super::foundation::collections::IIterable<StorageProviderItemProperty>) -> Result<ComPtr<super::super::foundation::IAsyncAction>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).SetAsync)(self as *const _ as *mut _, item as *const _ as *mut _, itemProperties as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
DEFINE_IID!(IID_IStorageProviderItemProperty, 1198306648, 29451, 16776, 183, 181, 99, 183, 22, 237, 71, 109);
RT_INTERFACE!{interface IStorageProviderItemProperty(IStorageProviderItemPropertyVtbl): IInspectable(IInspectableVtbl) [IID_IStorageProviderItemProperty] {
    fn put_Id(&self, value: i32) -> HRESULT,
    fn get_Id(&self, out: *mut i32) -> HRESULT,
    fn put_Value(&self, value: HSTRING) -> HRESULT,
    fn get_Value(&self, out: *mut HSTRING) -> HRESULT,
    fn put_IconResource(&self, value: HSTRING) -> HRESULT,
    fn get_IconResource(&self, out: *mut HSTRING) -> HRESULT
}}
impl IStorageProviderItemProperty {
    #[inline] pub unsafe fn set_id(&self, value: i32) -> Result<()> {
        let hr = ((*self.lpVtbl).put_Id)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_id(&self) -> Result<i32> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_Id)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_value(&self, value: &HStringArg) -> Result<()> {
        let hr = ((*self.lpVtbl).put_Value)(self as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_value(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Value)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_icon_resource(&self, value: &HStringArg) -> Result<()> {
        let hr = ((*self.lpVtbl).put_IconResource)(self as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_icon_resource(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_IconResource)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
}
RT_CLASS!{class StorageProviderItemProperty: IStorageProviderItemProperty}
impl RtActivatable<IActivationFactory> for StorageProviderItemProperty {}
DEFINE_CLSID!(StorageProviderItemProperty(&[87,105,110,100,111,119,115,46,83,116,111,114,97,103,101,46,80,114,111,118,105,100,101,114,46,83,116,111,114,97,103,101,80,114,111,118,105,100,101,114,73,116,101,109,80,114,111,112,101,114,116,121,0]) [CLSID_StorageProviderItemProperty]);
DEFINE_IID!(IID_IStorageProviderItemPropertyDefinition, 3316876219, 65311, 17048, 131, 30, 255, 28, 8, 8, 150, 144);
RT_INTERFACE!{interface IStorageProviderItemPropertyDefinition(IStorageProviderItemPropertyDefinitionVtbl): IInspectable(IInspectableVtbl) [IID_IStorageProviderItemPropertyDefinition] {
    fn get_Id(&self, out: *mut i32) -> HRESULT,
    fn put_Id(&self, value: i32) -> HRESULT,
    fn get_DisplayNameResource(&self, out: *mut HSTRING) -> HRESULT,
    fn put_DisplayNameResource(&self, value: HSTRING) -> HRESULT
}}
impl IStorageProviderItemPropertyDefinition {
    #[inline] pub unsafe fn get_id(&self) -> Result<i32> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_Id)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_id(&self, value: i32) -> Result<()> {
        let hr = ((*self.lpVtbl).put_Id)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_display_name_resource(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_DisplayNameResource)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_display_name_resource(&self, value: &HStringArg) -> Result<()> {
        let hr = ((*self.lpVtbl).put_DisplayNameResource)(self as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
}
RT_CLASS!{class StorageProviderItemPropertyDefinition: IStorageProviderItemPropertyDefinition}
impl RtActivatable<IActivationFactory> for StorageProviderItemPropertyDefinition {}
DEFINE_CLSID!(StorageProviderItemPropertyDefinition(&[87,105,110,100,111,119,115,46,83,116,111,114,97,103,101,46,80,114,111,118,105,100,101,114,46,83,116,111,114,97,103,101,80,114,111,118,105,100,101,114,73,116,101,109,80,114,111,112,101,114,116,121,68,101,102,105,110,105,116,105,111,110,0]) [CLSID_StorageProviderItemPropertyDefinition]);
DEFINE_IID!(IID_IStorageProviderItemPropertySource, 2406456382, 63026, 19099, 141, 153, 210, 215, 161, 29, 245, 106);
RT_INTERFACE!{interface IStorageProviderItemPropertySource(IStorageProviderItemPropertySourceVtbl): IInspectable(IInspectableVtbl) [IID_IStorageProviderItemPropertySource] {
    fn GetItemProperties(&self, itemPath: HSTRING, out: *mut *mut super::super::foundation::collections::IIterable<StorageProviderItemProperty>) -> HRESULT
}}
impl IStorageProviderItemPropertySource {
    #[inline] pub unsafe fn get_item_properties(&self, itemPath: &HStringArg) -> Result<ComPtr<super::super::foundation::collections::IIterable<StorageProviderItemProperty>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetItemProperties)(self as *const _ as *mut _, itemPath.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
RT_ENUM! { enum StorageProviderPopulationPolicy: i32 {
    Full (StorageProviderPopulationPolicy_Full) = 1, AlwaysFull (StorageProviderPopulationPolicy_AlwaysFull) = 2,
}}
DEFINE_IID!(IID_IStorageProviderPropertyCapabilities, 1703751438, 25527, 17767, 172, 249, 81, 171, 227, 1, 221, 165);
RT_INTERFACE!{interface IStorageProviderPropertyCapabilities(IStorageProviderPropertyCapabilitiesVtbl): IInspectable(IInspectableVtbl) [IID_IStorageProviderPropertyCapabilities] {
    fn IsPropertySupported(&self, propertyCanonicalName: HSTRING, out: *mut bool) -> HRESULT
}}
impl IStorageProviderPropertyCapabilities {
    #[inline] pub unsafe fn is_property_supported(&self, propertyCanonicalName: &HStringArg) -> Result<bool> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).IsPropertySupported)(self as *const _ as *mut _, propertyCanonicalName.get(), &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
}
RT_ENUM! { enum StorageProviderProtectionMode: i32 {
    Unknown (StorageProviderProtectionMode_Unknown) = 0, Personal (StorageProviderProtectionMode_Personal) = 1,
}}
DEFINE_IID!(IID_IStorageProviderSyncRootInfo, 2081621444, 39417, 16812, 137, 4, 171, 5, 93, 101, 73, 38);
RT_INTERFACE!{interface IStorageProviderSyncRootInfo(IStorageProviderSyncRootInfoVtbl): IInspectable(IInspectableVtbl) [IID_IStorageProviderSyncRootInfo] {
    fn get_Id(&self, out: *mut HSTRING) -> HRESULT,
    fn put_Id(&self, value: HSTRING) -> HRESULT,
    fn get_Context(&self, out: *mut *mut super::streams::IBuffer) -> HRESULT,
    fn put_Context(&self, value: *mut super::streams::IBuffer) -> HRESULT,
    fn get_Path(&self, out: *mut *mut super::IStorageFolder) -> HRESULT,
    fn put_Path(&self, folder: *mut super::IStorageFolder) -> HRESULT,
    fn get_DisplayNameResource(&self, out: *mut HSTRING) -> HRESULT,
    fn put_DisplayNameResource(&self, value: HSTRING) -> HRESULT,
    fn get_IconResource(&self, out: *mut HSTRING) -> HRESULT,
    fn put_IconResource(&self, value: HSTRING) -> HRESULT,
    fn get_HydrationPolicy(&self, out: *mut StorageProviderHydrationPolicy) -> HRESULT,
    fn put_HydrationPolicy(&self, value: StorageProviderHydrationPolicy) -> HRESULT,
    fn get_HydrationPolicyModifier(&self, out: *mut StorageProviderHydrationPolicyModifier) -> HRESULT,
    fn put_HydrationPolicyModifier(&self, value: StorageProviderHydrationPolicyModifier) -> HRESULT,
    fn get_PopulationPolicy(&self, out: *mut StorageProviderPopulationPolicy) -> HRESULT,
    fn put_PopulationPolicy(&self, value: StorageProviderPopulationPolicy) -> HRESULT,
    fn get_InSyncPolicy(&self, out: *mut StorageProviderInSyncPolicy) -> HRESULT,
    fn put_InSyncPolicy(&self, value: StorageProviderInSyncPolicy) -> HRESULT,
    fn get_HardlinkPolicy(&self, out: *mut StorageProviderHardlinkPolicy) -> HRESULT,
    fn put_HardlinkPolicy(&self, value: StorageProviderHardlinkPolicy) -> HRESULT,
    fn get_ShowSiblingsAsGroup(&self, out: *mut bool) -> HRESULT,
    fn put_ShowSiblingsAsGroup(&self, value: bool) -> HRESULT,
    fn get_Version(&self, out: *mut HSTRING) -> HRESULT,
    fn put_Version(&self, value: HSTRING) -> HRESULT,
    fn get_ProtectionMode(&self, out: *mut StorageProviderProtectionMode) -> HRESULT,
    fn put_ProtectionMode(&self, value: StorageProviderProtectionMode) -> HRESULT,
    fn get_AllowPinning(&self, out: *mut bool) -> HRESULT,
    fn put_AllowPinning(&self, value: bool) -> HRESULT,
    fn get_StorageProviderItemPropertyDefinitions(&self, out: *mut *mut super::super::foundation::collections::IVector<StorageProviderItemPropertyDefinition>) -> HRESULT,
    fn get_RecycleBinUri(&self, out: *mut *mut super::super::foundation::Uri) -> HRESULT,
    fn put_RecycleBinUri(&self, value: *mut super::super::foundation::Uri) -> HRESULT
}}
impl IStorageProviderSyncRootInfo {
    #[inline] pub unsafe fn get_id(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Id)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_id(&self, value: &HStringArg) -> Result<()> {
        let hr = ((*self.lpVtbl).put_Id)(self as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_context(&self) -> Result<ComPtr<super::streams::IBuffer>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Context)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_context(&self, value: &super::streams::IBuffer) -> Result<()> {
        let hr = ((*self.lpVtbl).put_Context)(self as *const _ as *mut _, value as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_path(&self) -> Result<ComPtr<super::IStorageFolder>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Path)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_path(&self, folder: &super::IStorageFolder) -> Result<()> {
        let hr = ((*self.lpVtbl).put_Path)(self as *const _ as *mut _, folder as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_display_name_resource(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_DisplayNameResource)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_display_name_resource(&self, value: &HStringArg) -> Result<()> {
        let hr = ((*self.lpVtbl).put_DisplayNameResource)(self as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_icon_resource(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_IconResource)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_icon_resource(&self, value: &HStringArg) -> Result<()> {
        let hr = ((*self.lpVtbl).put_IconResource)(self as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_hydration_policy(&self) -> Result<StorageProviderHydrationPolicy> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_HydrationPolicy)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_hydration_policy(&self, value: StorageProviderHydrationPolicy) -> Result<()> {
        let hr = ((*self.lpVtbl).put_HydrationPolicy)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_hydration_policy_modifier(&self) -> Result<StorageProviderHydrationPolicyModifier> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_HydrationPolicyModifier)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_hydration_policy_modifier(&self, value: StorageProviderHydrationPolicyModifier) -> Result<()> {
        let hr = ((*self.lpVtbl).put_HydrationPolicyModifier)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_population_policy(&self) -> Result<StorageProviderPopulationPolicy> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_PopulationPolicy)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_population_policy(&self, value: StorageProviderPopulationPolicy) -> Result<()> {
        let hr = ((*self.lpVtbl).put_PopulationPolicy)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_in_sync_policy(&self) -> Result<StorageProviderInSyncPolicy> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_InSyncPolicy)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_in_sync_policy(&self, value: StorageProviderInSyncPolicy) -> Result<()> {
        let hr = ((*self.lpVtbl).put_InSyncPolicy)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_hardlink_policy(&self) -> Result<StorageProviderHardlinkPolicy> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_HardlinkPolicy)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_hardlink_policy(&self, value: StorageProviderHardlinkPolicy) -> Result<()> {
        let hr = ((*self.lpVtbl).put_HardlinkPolicy)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_show_siblings_as_group(&self) -> Result<bool> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_ShowSiblingsAsGroup)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_show_siblings_as_group(&self, value: bool) -> Result<()> {
        let hr = ((*self.lpVtbl).put_ShowSiblingsAsGroup)(self as *const _ as *mut _, value);
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
    #[inline] pub unsafe fn get_protection_mode(&self) -> Result<StorageProviderProtectionMode> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_ProtectionMode)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_protection_mode(&self, value: StorageProviderProtectionMode) -> Result<()> {
        let hr = ((*self.lpVtbl).put_ProtectionMode)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_allow_pinning(&self) -> Result<bool> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_AllowPinning)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_allow_pinning(&self, value: bool) -> Result<()> {
        let hr = ((*self.lpVtbl).put_AllowPinning)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_storage_provider_item_property_definitions(&self) -> Result<ComPtr<super::super::foundation::collections::IVector<StorageProviderItemPropertyDefinition>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_StorageProviderItemPropertyDefinitions)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_recycle_bin_uri(&self) -> Result<ComPtr<super::super::foundation::Uri>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_RecycleBinUri)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_recycle_bin_uri(&self, value: &super::super::foundation::Uri) -> Result<()> {
        let hr = ((*self.lpVtbl).put_RecycleBinUri)(self as *const _ as *mut _, value as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
}
RT_CLASS!{class StorageProviderSyncRootInfo: IStorageProviderSyncRootInfo}
impl RtActivatable<IActivationFactory> for StorageProviderSyncRootInfo {}
DEFINE_CLSID!(StorageProviderSyncRootInfo(&[87,105,110,100,111,119,115,46,83,116,111,114,97,103,101,46,80,114,111,118,105,100,101,114,46,83,116,111,114,97,103,101,80,114,111,118,105,100,101,114,83,121,110,99,82,111,111,116,73,110,102,111,0]) [CLSID_StorageProviderSyncRootInfo]);
RT_CLASS!{static class StorageProviderSyncRootManager}
impl RtActivatable<IStorageProviderSyncRootManagerStatics> for StorageProviderSyncRootManager {}
impl StorageProviderSyncRootManager {
    #[inline] pub fn register(syncRootInformation: &StorageProviderSyncRootInfo) -> Result<()> { unsafe {
        <Self as RtActivatable<IStorageProviderSyncRootManagerStatics>>::get_activation_factory().register(syncRootInformation)
    }}
    #[inline] pub fn unregister(id: &HStringArg) -> Result<()> { unsafe {
        <Self as RtActivatable<IStorageProviderSyncRootManagerStatics>>::get_activation_factory().unregister(id)
    }}
    #[inline] pub fn get_sync_root_information_for_folder(folder: &super::IStorageFolder) -> Result<ComPtr<StorageProviderSyncRootInfo>> { unsafe {
        <Self as RtActivatable<IStorageProviderSyncRootManagerStatics>>::get_activation_factory().get_sync_root_information_for_folder(folder)
    }}
    #[inline] pub fn get_sync_root_information_for_id(id: &HStringArg) -> Result<ComPtr<StorageProviderSyncRootInfo>> { unsafe {
        <Self as RtActivatable<IStorageProviderSyncRootManagerStatics>>::get_activation_factory().get_sync_root_information_for_id(id)
    }}
    #[inline] pub fn get_current_sync_roots() -> Result<ComPtr<super::super::foundation::collections::IVectorView<StorageProviderSyncRootInfo>>> { unsafe {
        <Self as RtActivatable<IStorageProviderSyncRootManagerStatics>>::get_activation_factory().get_current_sync_roots()
    }}
}
DEFINE_CLSID!(StorageProviderSyncRootManager(&[87,105,110,100,111,119,115,46,83,116,111,114,97,103,101,46,80,114,111,118,105,100,101,114,46,83,116,111,114,97,103,101,80,114,111,118,105,100,101,114,83,121,110,99,82,111,111,116,77,97,110,97,103,101,114,0]) [CLSID_StorageProviderSyncRootManager]);
DEFINE_IID!(IID_IStorageProviderSyncRootManagerStatics, 1050278847, 36835, 19264, 171, 199, 246, 252, 61, 116, 201, 142);
RT_INTERFACE!{static interface IStorageProviderSyncRootManagerStatics(IStorageProviderSyncRootManagerStaticsVtbl): IInspectable(IInspectableVtbl) [IID_IStorageProviderSyncRootManagerStatics] {
    fn Register(&self, syncRootInformation: *mut StorageProviderSyncRootInfo) -> HRESULT,
    fn Unregister(&self, id: HSTRING) -> HRESULT,
    fn GetSyncRootInformationForFolder(&self, folder: *mut super::IStorageFolder, out: *mut *mut StorageProviderSyncRootInfo) -> HRESULT,
    fn GetSyncRootInformationForId(&self, id: HSTRING, out: *mut *mut StorageProviderSyncRootInfo) -> HRESULT,
    fn GetCurrentSyncRoots(&self, out: *mut *mut super::super::foundation::collections::IVectorView<StorageProviderSyncRootInfo>) -> HRESULT
}}
impl IStorageProviderSyncRootManagerStatics {
    #[inline] pub unsafe fn register(&self, syncRootInformation: &StorageProviderSyncRootInfo) -> Result<()> {
        let hr = ((*self.lpVtbl).Register)(self as *const _ as *mut _, syncRootInformation as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn unregister(&self, id: &HStringArg) -> Result<()> {
        let hr = ((*self.lpVtbl).Unregister)(self as *const _ as *mut _, id.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_sync_root_information_for_folder(&self, folder: &super::IStorageFolder) -> Result<ComPtr<StorageProviderSyncRootInfo>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetSyncRootInformationForFolder)(self as *const _ as *mut _, folder as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_sync_root_information_for_id(&self, id: &HStringArg) -> Result<ComPtr<StorageProviderSyncRootInfo>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetSyncRootInformationForId)(self as *const _ as *mut _, id.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_current_sync_roots(&self) -> Result<ComPtr<super::super::foundation::collections::IVectorView<StorageProviderSyncRootInfo>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetCurrentSyncRoots)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
RT_ENUM! { enum UIStatus: i32 {
    Unavailable (UIStatus_Unavailable) = 0, Hidden (UIStatus_Hidden) = 1, Visible (UIStatus_Visible) = 2, Complete (UIStatus_Complete) = 3,
}}
RT_ENUM! { enum WriteActivationMode: i32 {
    ReadOnly (WriteActivationMode_ReadOnly) = 0, NotNeeded (WriteActivationMode_NotNeeded) = 1, AfterWrite (WriteActivationMode_AfterWrite) = 2,
}}
} // Windows.Storage.Provider
pub mod fileproperties { // Windows.Storage.FileProperties
use ::prelude::*;
DEFINE_IID!(IID_IBasicProperties, 3495777755, 30814, 19046, 190, 2, 155, 238, 197, 138, 234, 129);
RT_INTERFACE!{interface IBasicProperties(IBasicPropertiesVtbl): IInspectable(IInspectableVtbl) [IID_IBasicProperties] {
    fn get_Size(&self, out: *mut u64) -> HRESULT,
    fn get_DateModified(&self, out: *mut super::super::foundation::DateTime) -> HRESULT,
    fn get_ItemDate(&self, out: *mut super::super::foundation::DateTime) -> HRESULT
}}
impl IBasicProperties {
    #[inline] pub unsafe fn get_size(&self) -> Result<u64> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_Size)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_date_modified(&self) -> Result<super::super::foundation::DateTime> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_DateModified)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_item_date(&self) -> Result<super::super::foundation::DateTime> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_ItemDate)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
}
RT_CLASS!{class BasicProperties: IBasicProperties}
DEFINE_IID!(IID_IDocumentProperties, 2125142460, 6177, 18723, 180, 169, 10, 234, 64, 77, 0, 112);
RT_INTERFACE!{interface IDocumentProperties(IDocumentPropertiesVtbl): IInspectable(IInspectableVtbl) [IID_IDocumentProperties] {
    fn get_Author(&self, out: *mut *mut super::super::foundation::collections::IVector<HString>) -> HRESULT,
    fn get_Title(&self, out: *mut HSTRING) -> HRESULT,
    fn put_Title(&self, value: HSTRING) -> HRESULT,
    fn get_Keywords(&self, out: *mut *mut super::super::foundation::collections::IVector<HString>) -> HRESULT,
    fn get_Comment(&self, out: *mut HSTRING) -> HRESULT,
    fn put_Comment(&self, value: HSTRING) -> HRESULT
}}
impl IDocumentProperties {
    #[inline] pub unsafe fn get_author(&self) -> Result<ComPtr<super::super::foundation::collections::IVector<HString>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Author)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
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
    #[inline] pub unsafe fn get_keywords(&self) -> Result<ComPtr<super::super::foundation::collections::IVector<HString>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Keywords)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_comment(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Comment)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_comment(&self, value: &HStringArg) -> Result<()> {
        let hr = ((*self.lpVtbl).put_Comment)(self as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
}
RT_CLASS!{class DocumentProperties: IDocumentProperties}
RT_CLASS!{static class GeotagHelper}
impl RtActivatable<IGeotagHelperStatics> for GeotagHelper {}
impl GeotagHelper {
    #[cfg(feature="windows-devices")] #[inline] pub fn get_geotag_async(file: &super::IStorageFile) -> Result<ComPtr<super::super::foundation::IAsyncOperation<super::super::devices::geolocation::Geopoint>>> { unsafe {
        <Self as RtActivatable<IGeotagHelperStatics>>::get_activation_factory().get_geotag_async(file)
    }}
    #[cfg(feature="windows-devices")] #[inline] pub fn set_geotag_from_geolocator_async(file: &super::IStorageFile, geolocator: &super::super::devices::geolocation::Geolocator) -> Result<ComPtr<super::super::foundation::IAsyncAction>> { unsafe {
        <Self as RtActivatable<IGeotagHelperStatics>>::get_activation_factory().set_geotag_from_geolocator_async(file, geolocator)
    }}
    #[cfg(feature="windows-devices")] #[inline] pub fn set_geotag_async(file: &super::IStorageFile, geopoint: &super::super::devices::geolocation::Geopoint) -> Result<ComPtr<super::super::foundation::IAsyncAction>> { unsafe {
        <Self as RtActivatable<IGeotagHelperStatics>>::get_activation_factory().set_geotag_async(file, geopoint)
    }}
}
DEFINE_CLSID!(GeotagHelper(&[87,105,110,100,111,119,115,46,83,116,111,114,97,103,101,46,70,105,108,101,80,114,111,112,101,114,116,105,101,115,46,71,101,111,116,97,103,72,101,108,112,101,114,0]) [CLSID_GeotagHelper]);
DEFINE_IID!(IID_IGeotagHelperStatics, 1095316036, 9508, 18005, 134, 166, 237, 22, 245, 252, 113, 107);
RT_INTERFACE!{static interface IGeotagHelperStatics(IGeotagHelperStaticsVtbl): IInspectable(IInspectableVtbl) [IID_IGeotagHelperStatics] {
    #[cfg(feature="windows-devices")] fn GetGeotagAsync(&self, file: *mut super::IStorageFile, out: *mut *mut super::super::foundation::IAsyncOperation<super::super::devices::geolocation::Geopoint>) -> HRESULT,
    #[cfg(feature="windows-devices")] fn SetGeotagFromGeolocatorAsync(&self, file: *mut super::IStorageFile, geolocator: *mut super::super::devices::geolocation::Geolocator, out: *mut *mut super::super::foundation::IAsyncAction) -> HRESULT,
    #[cfg(feature="windows-devices")] fn SetGeotagAsync(&self, file: *mut super::IStorageFile, geopoint: *mut super::super::devices::geolocation::Geopoint, out: *mut *mut super::super::foundation::IAsyncAction) -> HRESULT
}}
impl IGeotagHelperStatics {
    #[cfg(feature="windows-devices")] #[inline] pub unsafe fn get_geotag_async(&self, file: &super::IStorageFile) -> Result<ComPtr<super::super::foundation::IAsyncOperation<super::super::devices::geolocation::Geopoint>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetGeotagAsync)(self as *const _ as *mut _, file as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[cfg(feature="windows-devices")] #[inline] pub unsafe fn set_geotag_from_geolocator_async(&self, file: &super::IStorageFile, geolocator: &super::super::devices::geolocation::Geolocator) -> Result<ComPtr<super::super::foundation::IAsyncAction>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).SetGeotagFromGeolocatorAsync)(self as *const _ as *mut _, file as *const _ as *mut _, geolocator as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[cfg(feature="windows-devices")] #[inline] pub unsafe fn set_geotag_async(&self, file: &super::IStorageFile, geopoint: &super::super::devices::geolocation::Geopoint) -> Result<ComPtr<super::super::foundation::IAsyncAction>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).SetGeotagAsync)(self as *const _ as *mut _, file as *const _ as *mut _, geopoint as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
DEFINE_IID!(IID_IImageProperties, 1379701796, 64767, 17013, 175, 238, 236, 219, 154, 180, 121, 115);
RT_INTERFACE!{interface IImageProperties(IImagePropertiesVtbl): IInspectable(IInspectableVtbl) [IID_IImageProperties] {
    fn get_Rating(&self, out: *mut u32) -> HRESULT,
    fn put_Rating(&self, value: u32) -> HRESULT,
    fn get_Keywords(&self, out: *mut *mut super::super::foundation::collections::IVector<HString>) -> HRESULT,
    fn get_DateTaken(&self, out: *mut super::super::foundation::DateTime) -> HRESULT,
    fn put_DateTaken(&self, value: super::super::foundation::DateTime) -> HRESULT,
    fn get_Width(&self, out: *mut u32) -> HRESULT,
    fn get_Height(&self, out: *mut u32) -> HRESULT,
    fn get_Title(&self, out: *mut HSTRING) -> HRESULT,
    fn put_Title(&self, value: HSTRING) -> HRESULT,
    fn get_Latitude(&self, out: *mut *mut super::super::foundation::IReference<f64>) -> HRESULT,
    fn get_Longitude(&self, out: *mut *mut super::super::foundation::IReference<f64>) -> HRESULT,
    fn get_CameraManufacturer(&self, out: *mut HSTRING) -> HRESULT,
    fn put_CameraManufacturer(&self, value: HSTRING) -> HRESULT,
    fn get_CameraModel(&self, out: *mut HSTRING) -> HRESULT,
    fn put_CameraModel(&self, value: HSTRING) -> HRESULT,
    fn get_Orientation(&self, out: *mut PhotoOrientation) -> HRESULT,
    fn get_PeopleNames(&self, out: *mut *mut super::super::foundation::collections::IVectorView<HString>) -> HRESULT
}}
impl IImageProperties {
    #[inline] pub unsafe fn get_rating(&self) -> Result<u32> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_Rating)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_rating(&self, value: u32) -> Result<()> {
        let hr = ((*self.lpVtbl).put_Rating)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_keywords(&self) -> Result<ComPtr<super::super::foundation::collections::IVector<HString>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Keywords)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_date_taken(&self) -> Result<super::super::foundation::DateTime> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_DateTaken)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_date_taken(&self, value: super::super::foundation::DateTime) -> Result<()> {
        let hr = ((*self.lpVtbl).put_DateTaken)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_width(&self) -> Result<u32> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_Width)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_height(&self) -> Result<u32> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_Height)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
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
    #[inline] pub unsafe fn get_latitude(&self) -> Result<ComPtr<super::super::foundation::IReference<f64>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Latitude)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_longitude(&self) -> Result<ComPtr<super::super::foundation::IReference<f64>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Longitude)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_camera_manufacturer(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_CameraManufacturer)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_camera_manufacturer(&self, value: &HStringArg) -> Result<()> {
        let hr = ((*self.lpVtbl).put_CameraManufacturer)(self as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_camera_model(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_CameraModel)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_camera_model(&self, value: &HStringArg) -> Result<()> {
        let hr = ((*self.lpVtbl).put_CameraModel)(self as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_orientation(&self) -> Result<PhotoOrientation> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_Orientation)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_people_names(&self) -> Result<ComPtr<super::super::foundation::collections::IVectorView<HString>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_PeopleNames)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
RT_CLASS!{class ImageProperties: IImageProperties}
DEFINE_IID!(IID_IMusicProperties, 3163204450, 26348, 16794, 188, 93, 202, 101, 164, 203, 70, 218);
RT_INTERFACE!{interface IMusicProperties(IMusicPropertiesVtbl): IInspectable(IInspectableVtbl) [IID_IMusicProperties] {
    fn get_Album(&self, out: *mut HSTRING) -> HRESULT,
    fn put_Album(&self, value: HSTRING) -> HRESULT,
    fn get_Artist(&self, out: *mut HSTRING) -> HRESULT,
    fn put_Artist(&self, value: HSTRING) -> HRESULT,
    fn get_Genre(&self, out: *mut *mut super::super::foundation::collections::IVector<HString>) -> HRESULT,
    fn get_TrackNumber(&self, out: *mut u32) -> HRESULT,
    fn put_TrackNumber(&self, value: u32) -> HRESULT,
    fn get_Title(&self, out: *mut HSTRING) -> HRESULT,
    fn put_Title(&self, value: HSTRING) -> HRESULT,
    fn get_Rating(&self, out: *mut u32) -> HRESULT,
    fn put_Rating(&self, value: u32) -> HRESULT,
    fn get_Duration(&self, out: *mut super::super::foundation::TimeSpan) -> HRESULT,
    fn get_Bitrate(&self, out: *mut u32) -> HRESULT,
    fn get_AlbumArtist(&self, out: *mut HSTRING) -> HRESULT,
    fn put_AlbumArtist(&self, value: HSTRING) -> HRESULT,
    fn get_Composers(&self, out: *mut *mut super::super::foundation::collections::IVector<HString>) -> HRESULT,
    fn get_Conductors(&self, out: *mut *mut super::super::foundation::collections::IVector<HString>) -> HRESULT,
    fn get_Subtitle(&self, out: *mut HSTRING) -> HRESULT,
    fn put_Subtitle(&self, value: HSTRING) -> HRESULT,
    fn get_Producers(&self, out: *mut *mut super::super::foundation::collections::IVector<HString>) -> HRESULT,
    fn get_Publisher(&self, out: *mut HSTRING) -> HRESULT,
    fn put_Publisher(&self, value: HSTRING) -> HRESULT,
    fn get_Writers(&self, out: *mut *mut super::super::foundation::collections::IVector<HString>) -> HRESULT,
    fn get_Year(&self, out: *mut u32) -> HRESULT,
    fn put_Year(&self, value: u32) -> HRESULT
}}
impl IMusicProperties {
    #[inline] pub unsafe fn get_album(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Album)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_album(&self, value: &HStringArg) -> Result<()> {
        let hr = ((*self.lpVtbl).put_Album)(self as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_artist(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Artist)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_artist(&self, value: &HStringArg) -> Result<()> {
        let hr = ((*self.lpVtbl).put_Artist)(self as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_genre(&self) -> Result<ComPtr<super::super::foundation::collections::IVector<HString>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Genre)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_track_number(&self) -> Result<u32> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_TrackNumber)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_track_number(&self, value: u32) -> Result<()> {
        let hr = ((*self.lpVtbl).put_TrackNumber)(self as *const _ as *mut _, value);
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
    #[inline] pub unsafe fn get_rating(&self) -> Result<u32> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_Rating)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_rating(&self, value: u32) -> Result<()> {
        let hr = ((*self.lpVtbl).put_Rating)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_duration(&self) -> Result<super::super::foundation::TimeSpan> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_Duration)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_bitrate(&self) -> Result<u32> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_Bitrate)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_album_artist(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_AlbumArtist)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_album_artist(&self, value: &HStringArg) -> Result<()> {
        let hr = ((*self.lpVtbl).put_AlbumArtist)(self as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_composers(&self) -> Result<ComPtr<super::super::foundation::collections::IVector<HString>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Composers)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_conductors(&self) -> Result<ComPtr<super::super::foundation::collections::IVector<HString>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Conductors)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_subtitle(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Subtitle)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_subtitle(&self, value: &HStringArg) -> Result<()> {
        let hr = ((*self.lpVtbl).put_Subtitle)(self as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_producers(&self) -> Result<ComPtr<super::super::foundation::collections::IVector<HString>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Producers)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_publisher(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Publisher)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_publisher(&self, value: &HStringArg) -> Result<()> {
        let hr = ((*self.lpVtbl).put_Publisher)(self as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_writers(&self) -> Result<ComPtr<super::super::foundation::collections::IVector<HString>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Writers)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_year(&self) -> Result<u32> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_Year)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_year(&self, value: u32) -> Result<()> {
        let hr = ((*self.lpVtbl).put_Year)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
}
RT_CLASS!{class MusicProperties: IMusicProperties}
RT_ENUM! { enum PhotoOrientation: i32 {
    Unspecified (PhotoOrientation_Unspecified) = 0, Normal (PhotoOrientation_Normal) = 1, FlipHorizontal (PhotoOrientation_FlipHorizontal) = 2, Rotate180 (PhotoOrientation_Rotate180) = 3, FlipVertical (PhotoOrientation_FlipVertical) = 4, Transpose (PhotoOrientation_Transpose) = 5, Rotate270 (PhotoOrientation_Rotate270) = 6, Transverse (PhotoOrientation_Transverse) = 7, Rotate90 (PhotoOrientation_Rotate90) = 8,
}}
RT_ENUM! { enum PropertyPrefetchOptions: u32 {
    None (PropertyPrefetchOptions_None) = 0, MusicProperties (PropertyPrefetchOptions_MusicProperties) = 1, VideoProperties (PropertyPrefetchOptions_VideoProperties) = 2, ImageProperties (PropertyPrefetchOptions_ImageProperties) = 4, DocumentProperties (PropertyPrefetchOptions_DocumentProperties) = 8, BasicProperties (PropertyPrefetchOptions_BasicProperties) = 16,
}}
DEFINE_IID!(IID_IStorageItemContentProperties, 86592429, 48184, 18623, 133, 215, 119, 14, 14, 42, 224, 186);
RT_INTERFACE!{interface IStorageItemContentProperties(IStorageItemContentPropertiesVtbl): IInspectable(IInspectableVtbl) [IID_IStorageItemContentProperties] {
    fn GetMusicPropertiesAsync(&self, out: *mut *mut super::super::foundation::IAsyncOperation<MusicProperties>) -> HRESULT,
    fn GetVideoPropertiesAsync(&self, out: *mut *mut super::super::foundation::IAsyncOperation<VideoProperties>) -> HRESULT,
    fn GetImagePropertiesAsync(&self, out: *mut *mut super::super::foundation::IAsyncOperation<ImageProperties>) -> HRESULT,
    fn GetDocumentPropertiesAsync(&self, out: *mut *mut super::super::foundation::IAsyncOperation<DocumentProperties>) -> HRESULT
}}
impl IStorageItemContentProperties {
    #[inline] pub unsafe fn get_music_properties_async(&self) -> Result<ComPtr<super::super::foundation::IAsyncOperation<MusicProperties>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetMusicPropertiesAsync)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_video_properties_async(&self) -> Result<ComPtr<super::super::foundation::IAsyncOperation<VideoProperties>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetVideoPropertiesAsync)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_image_properties_async(&self) -> Result<ComPtr<super::super::foundation::IAsyncOperation<ImageProperties>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetImagePropertiesAsync)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_document_properties_async(&self) -> Result<ComPtr<super::super::foundation::IAsyncOperation<DocumentProperties>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetDocumentPropertiesAsync)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
RT_CLASS!{class StorageItemContentProperties: IStorageItemContentProperties}
DEFINE_IID!(IID_IStorageItemExtraProperties, 3309527474, 21709, 17195, 189, 188, 75, 25, 196, 180, 112, 215);
RT_INTERFACE!{interface IStorageItemExtraProperties(IStorageItemExtraPropertiesVtbl): IInspectable(IInspectableVtbl) [IID_IStorageItemExtraProperties] {
    fn RetrievePropertiesAsync(&self, propertiesToRetrieve: *mut super::super::foundation::collections::IIterable<HString>, out: *mut *mut super::super::foundation::IAsyncOperation<super::super::foundation::collections::IMap<HString, IInspectable>>) -> HRESULT,
    fn SavePropertiesAsync(&self, propertiesToSave: *mut super::super::foundation::collections::IIterable<super::super::foundation::collections::IKeyValuePair<HString, IInspectable>>, out: *mut *mut super::super::foundation::IAsyncAction) -> HRESULT,
    fn SavePropertiesAsyncOverloadDefault(&self, out: *mut *mut super::super::foundation::IAsyncAction) -> HRESULT
}}
impl IStorageItemExtraProperties {
    #[inline] pub unsafe fn retrieve_properties_async(&self, propertiesToRetrieve: &super::super::foundation::collections::IIterable<HString>) -> Result<ComPtr<super::super::foundation::IAsyncOperation<super::super::foundation::collections::IMap<HString, IInspectable>>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).RetrievePropertiesAsync)(self as *const _ as *mut _, propertiesToRetrieve as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn save_properties_async(&self, propertiesToSave: &super::super::foundation::collections::IIterable<super::super::foundation::collections::IKeyValuePair<HString, IInspectable>>) -> Result<ComPtr<super::super::foundation::IAsyncAction>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).SavePropertiesAsync)(self as *const _ as *mut _, propertiesToSave as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn save_properties_async_overload_default(&self) -> Result<ComPtr<super::super::foundation::IAsyncAction>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).SavePropertiesAsyncOverloadDefault)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
RT_CLASS!{class StorageItemThumbnail: super::streams::IRandomAccessStreamWithContentType}
RT_ENUM! { enum ThumbnailMode: i32 {
    PicturesView (ThumbnailMode_PicturesView) = 0, VideosView (ThumbnailMode_VideosView) = 1, MusicView (ThumbnailMode_MusicView) = 2, DocumentsView (ThumbnailMode_DocumentsView) = 3, ListView (ThumbnailMode_ListView) = 4, SingleItem (ThumbnailMode_SingleItem) = 5,
}}
RT_ENUM! { enum ThumbnailOptions: u32 {
    None (ThumbnailOptions_None) = 0, ReturnOnlyIfCached (ThumbnailOptions_ReturnOnlyIfCached) = 1, ResizeThumbnail (ThumbnailOptions_ResizeThumbnail) = 2, UseCurrentScale (ThumbnailOptions_UseCurrentScale) = 4,
}}
DEFINE_IID!(IID_IThumbnailProperties, 1765659695, 56295, 18869, 179, 179, 40, 147, 172, 93, 52, 35);
RT_INTERFACE!{interface IThumbnailProperties(IThumbnailPropertiesVtbl): IInspectable(IInspectableVtbl) [IID_IThumbnailProperties] {
    fn get_OriginalWidth(&self, out: *mut u32) -> HRESULT,
    fn get_OriginalHeight(&self, out: *mut u32) -> HRESULT,
    fn get_ReturnedSmallerCachedSize(&self, out: *mut bool) -> HRESULT,
    fn get_Type(&self, out: *mut ThumbnailType) -> HRESULT
}}
impl IThumbnailProperties {
    #[inline] pub unsafe fn get_original_width(&self) -> Result<u32> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_OriginalWidth)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_original_height(&self) -> Result<u32> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_OriginalHeight)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_returned_smaller_cached_size(&self) -> Result<bool> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_ReturnedSmallerCachedSize)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_type(&self) -> Result<ThumbnailType> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_Type)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
}
RT_ENUM! { enum ThumbnailType: i32 {
    Image (ThumbnailType_Image) = 0, Icon (ThumbnailType_Icon) = 1,
}}
RT_ENUM! { enum VideoOrientation: i32 {
    Normal (VideoOrientation_Normal) = 0, Rotate90 (VideoOrientation_Rotate90) = 90, Rotate180 (VideoOrientation_Rotate180) = 180, Rotate270 (VideoOrientation_Rotate270) = 270,
}}
DEFINE_IID!(IID_IVideoProperties, 1905976583, 26846, 19896, 151, 222, 73, 153, 140, 5, 159, 47);
RT_INTERFACE!{interface IVideoProperties(IVideoPropertiesVtbl): IInspectable(IInspectableVtbl) [IID_IVideoProperties] {
    fn get_Rating(&self, out: *mut u32) -> HRESULT,
    fn put_Rating(&self, value: u32) -> HRESULT,
    fn get_Keywords(&self, out: *mut *mut super::super::foundation::collections::IVector<HString>) -> HRESULT,
    fn get_Width(&self, out: *mut u32) -> HRESULT,
    fn get_Height(&self, out: *mut u32) -> HRESULT,
    fn get_Duration(&self, out: *mut super::super::foundation::TimeSpan) -> HRESULT,
    fn get_Latitude(&self, out: *mut *mut super::super::foundation::IReference<f64>) -> HRESULT,
    fn get_Longitude(&self, out: *mut *mut super::super::foundation::IReference<f64>) -> HRESULT,
    fn get_Title(&self, out: *mut HSTRING) -> HRESULT,
    fn put_Title(&self, value: HSTRING) -> HRESULT,
    fn get_Subtitle(&self, out: *mut HSTRING) -> HRESULT,
    fn put_Subtitle(&self, value: HSTRING) -> HRESULT,
    fn get_Producers(&self, out: *mut *mut super::super::foundation::collections::IVector<HString>) -> HRESULT,
    fn get_Publisher(&self, out: *mut HSTRING) -> HRESULT,
    fn put_Publisher(&self, value: HSTRING) -> HRESULT,
    fn get_Writers(&self, out: *mut *mut super::super::foundation::collections::IVector<HString>) -> HRESULT,
    fn get_Year(&self, out: *mut u32) -> HRESULT,
    fn put_Year(&self, value: u32) -> HRESULT,
    fn get_Bitrate(&self, out: *mut u32) -> HRESULT,
    fn get_Directors(&self, out: *mut *mut super::super::foundation::collections::IVector<HString>) -> HRESULT,
    fn get_Orientation(&self, out: *mut VideoOrientation) -> HRESULT
}}
impl IVideoProperties {
    #[inline] pub unsafe fn get_rating(&self) -> Result<u32> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_Rating)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_rating(&self, value: u32) -> Result<()> {
        let hr = ((*self.lpVtbl).put_Rating)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_keywords(&self) -> Result<ComPtr<super::super::foundation::collections::IVector<HString>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Keywords)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_width(&self) -> Result<u32> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_Width)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_height(&self) -> Result<u32> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_Height)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_duration(&self) -> Result<super::super::foundation::TimeSpan> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_Duration)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_latitude(&self) -> Result<ComPtr<super::super::foundation::IReference<f64>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Latitude)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_longitude(&self) -> Result<ComPtr<super::super::foundation::IReference<f64>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Longitude)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
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
    #[inline] pub unsafe fn get_subtitle(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Subtitle)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_subtitle(&self, value: &HStringArg) -> Result<()> {
        let hr = ((*self.lpVtbl).put_Subtitle)(self as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_producers(&self) -> Result<ComPtr<super::super::foundation::collections::IVector<HString>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Producers)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_publisher(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Publisher)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_publisher(&self, value: &HStringArg) -> Result<()> {
        let hr = ((*self.lpVtbl).put_Publisher)(self as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_writers(&self) -> Result<ComPtr<super::super::foundation::collections::IVector<HString>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Writers)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_year(&self) -> Result<u32> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_Year)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_year(&self, value: u32) -> Result<()> {
        let hr = ((*self.lpVtbl).put_Year)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_bitrate(&self) -> Result<u32> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_Bitrate)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_directors(&self) -> Result<ComPtr<super::super::foundation::collections::IVector<HString>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Directors)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_orientation(&self) -> Result<VideoOrientation> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_Orientation)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
}
RT_CLASS!{class VideoProperties: IVideoProperties}
} // Windows.Storage.FileProperties
pub mod accesscache { // Windows.Storage.AccessCache
use ::prelude::*;
RT_ENUM! { enum AccessCacheOptions: u32 {
    None (AccessCacheOptions_None) = 0, DisallowUserInput (AccessCacheOptions_DisallowUserInput) = 1, FastLocationsOnly (AccessCacheOptions_FastLocationsOnly) = 2, UseReadOnlyCachedCopy (AccessCacheOptions_UseReadOnlyCachedCopy) = 4, SuppressAccessTimeUpdate (AccessCacheOptions_SuppressAccessTimeUpdate) = 8,
}}
RT_STRUCT! { struct AccessListEntry {
    Token: HSTRING, Metadata: HSTRING,
}}
RT_CLASS!{class AccessListEntryView: super::super::foundation::collections::IVectorView<AccessListEntry>}
DEFINE_IID!(IID_IItemRemovedEventArgs, 1499954780, 21950, 19558, 186, 102, 94, 174, 167, 157, 38, 49);
RT_INTERFACE!{interface IItemRemovedEventArgs(IItemRemovedEventArgsVtbl): IInspectable(IInspectableVtbl) [IID_IItemRemovedEventArgs] {
    fn get_RemovedEntry(&self, out: *mut AccessListEntry) -> HRESULT
}}
impl IItemRemovedEventArgs {
    #[inline] pub unsafe fn get_removed_entry(&self) -> Result<AccessListEntry> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_RemovedEntry)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
}
RT_CLASS!{class ItemRemovedEventArgs: IItemRemovedEventArgs}
RT_ENUM! { enum RecentStorageItemVisibility: i32 {
    AppOnly (RecentStorageItemVisibility_AppOnly) = 0, AppAndSystem (RecentStorageItemVisibility_AppAndSystem) = 1,
}}
RT_CLASS!{static class StorageApplicationPermissions}
impl RtActivatable<IStorageApplicationPermissionsStatics> for StorageApplicationPermissions {}
impl StorageApplicationPermissions {
    #[inline] pub fn get_future_access_list() -> Result<ComPtr<StorageItemAccessList>> { unsafe {
        <Self as RtActivatable<IStorageApplicationPermissionsStatics>>::get_activation_factory().get_future_access_list()
    }}
    #[inline] pub fn get_most_recently_used_list() -> Result<ComPtr<StorageItemMostRecentlyUsedList>> { unsafe {
        <Self as RtActivatable<IStorageApplicationPermissionsStatics>>::get_activation_factory().get_most_recently_used_list()
    }}
}
DEFINE_CLSID!(StorageApplicationPermissions(&[87,105,110,100,111,119,115,46,83,116,111,114,97,103,101,46,65,99,99,101,115,115,67,97,99,104,101,46,83,116,111,114,97,103,101,65,112,112,108,105,99,97,116,105,111,110,80,101,114,109,105,115,115,105,111,110,115,0]) [CLSID_StorageApplicationPermissions]);
DEFINE_IID!(IID_IStorageApplicationPermissionsStatics, 1133633450, 53299, 18681, 128, 96, 62, 200, 71, 210, 227, 241);
RT_INTERFACE!{static interface IStorageApplicationPermissionsStatics(IStorageApplicationPermissionsStaticsVtbl): IInspectable(IInspectableVtbl) [IID_IStorageApplicationPermissionsStatics] {
    fn get_FutureAccessList(&self, out: *mut *mut StorageItemAccessList) -> HRESULT,
    fn get_MostRecentlyUsedList(&self, out: *mut *mut StorageItemMostRecentlyUsedList) -> HRESULT
}}
impl IStorageApplicationPermissionsStatics {
    #[inline] pub unsafe fn get_future_access_list(&self) -> Result<ComPtr<StorageItemAccessList>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_FutureAccessList)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_most_recently_used_list(&self) -> Result<ComPtr<StorageItemMostRecentlyUsedList>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_MostRecentlyUsedList)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
DEFINE_IID!(IID_IStorageItemAccessList, 749729453, 56976, 18421, 178, 195, 221, 54, 201, 253, 212, 83);
RT_INTERFACE!{interface IStorageItemAccessList(IStorageItemAccessListVtbl): IInspectable(IInspectableVtbl) [IID_IStorageItemAccessList] {
    fn AddOverloadDefaultMetadata(&self, file: *mut super::IStorageItem, out: *mut HSTRING) -> HRESULT,
    fn Add(&self, file: *mut super::IStorageItem, metadata: HSTRING, out: *mut HSTRING) -> HRESULT,
    fn AddOrReplaceOverloadDefaultMetadata(&self, token: HSTRING, file: *mut super::IStorageItem) -> HRESULT,
    fn AddOrReplace(&self, token: HSTRING, file: *mut super::IStorageItem, metadata: HSTRING) -> HRESULT,
    fn GetItemAsync(&self, token: HSTRING, out: *mut *mut super::super::foundation::IAsyncOperation<super::IStorageItem>) -> HRESULT,
    fn GetFileAsync(&self, token: HSTRING, out: *mut *mut super::super::foundation::IAsyncOperation<super::StorageFile>) -> HRESULT,
    fn GetFolderAsync(&self, token: HSTRING, out: *mut *mut super::super::foundation::IAsyncOperation<super::StorageFolder>) -> HRESULT,
    fn GetItemWithOptionsAsync(&self, token: HSTRING, options: AccessCacheOptions, out: *mut *mut super::super::foundation::IAsyncOperation<super::IStorageItem>) -> HRESULT,
    fn GetFileWithOptionsAsync(&self, token: HSTRING, options: AccessCacheOptions, out: *mut *mut super::super::foundation::IAsyncOperation<super::StorageFile>) -> HRESULT,
    fn GetFolderWithOptionsAsync(&self, token: HSTRING, options: AccessCacheOptions, out: *mut *mut super::super::foundation::IAsyncOperation<super::StorageFolder>) -> HRESULT,
    fn Remove(&self, token: HSTRING) -> HRESULT,
    fn ContainsItem(&self, token: HSTRING, out: *mut bool) -> HRESULT,
    fn Clear(&self) -> HRESULT,
    fn CheckAccess(&self, file: *mut super::IStorageItem, out: *mut bool) -> HRESULT,
    fn get_Entries(&self, out: *mut *mut AccessListEntryView) -> HRESULT,
    fn get_MaximumItemsAllowed(&self, out: *mut u32) -> HRESULT
}}
impl IStorageItemAccessList {
    #[inline] pub unsafe fn add_overload_default_metadata(&self, file: &super::IStorageItem) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).AddOverloadDefaultMetadata)(self as *const _ as *mut _, file as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn add(&self, file: &super::IStorageItem, metadata: &HStringArg) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).Add)(self as *const _ as *mut _, file as *const _ as *mut _, metadata.get(), &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn add_or_replace_overload_default_metadata(&self, token: &HStringArg, file: &super::IStorageItem) -> Result<()> {
        let hr = ((*self.lpVtbl).AddOrReplaceOverloadDefaultMetadata)(self as *const _ as *mut _, token.get(), file as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn add_or_replace(&self, token: &HStringArg, file: &super::IStorageItem, metadata: &HStringArg) -> Result<()> {
        let hr = ((*self.lpVtbl).AddOrReplace)(self as *const _ as *mut _, token.get(), file as *const _ as *mut _, metadata.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_item_async(&self, token: &HStringArg) -> Result<ComPtr<super::super::foundation::IAsyncOperation<super::IStorageItem>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetItemAsync)(self as *const _ as *mut _, token.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_file_async(&self, token: &HStringArg) -> Result<ComPtr<super::super::foundation::IAsyncOperation<super::StorageFile>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetFileAsync)(self as *const _ as *mut _, token.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_folder_async(&self, token: &HStringArg) -> Result<ComPtr<super::super::foundation::IAsyncOperation<super::StorageFolder>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetFolderAsync)(self as *const _ as *mut _, token.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_item_with_options_async(&self, token: &HStringArg, options: AccessCacheOptions) -> Result<ComPtr<super::super::foundation::IAsyncOperation<super::IStorageItem>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetItemWithOptionsAsync)(self as *const _ as *mut _, token.get(), options, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_file_with_options_async(&self, token: &HStringArg, options: AccessCacheOptions) -> Result<ComPtr<super::super::foundation::IAsyncOperation<super::StorageFile>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetFileWithOptionsAsync)(self as *const _ as *mut _, token.get(), options, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_folder_with_options_async(&self, token: &HStringArg, options: AccessCacheOptions) -> Result<ComPtr<super::super::foundation::IAsyncOperation<super::StorageFolder>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetFolderWithOptionsAsync)(self as *const _ as *mut _, token.get(), options, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn remove(&self, token: &HStringArg) -> Result<()> {
        let hr = ((*self.lpVtbl).Remove)(self as *const _ as *mut _, token.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn contains_item(&self, token: &HStringArg) -> Result<bool> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).ContainsItem)(self as *const _ as *mut _, token.get(), &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn clear(&self) -> Result<()> {
        let hr = ((*self.lpVtbl).Clear)(self as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn check_access(&self, file: &super::IStorageItem) -> Result<bool> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).CheckAccess)(self as *const _ as *mut _, file as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_entries(&self) -> Result<ComPtr<AccessListEntryView>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Entries)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_maximum_items_allowed(&self) -> Result<u32> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_MaximumItemsAllowed)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
}
RT_CLASS!{class StorageItemAccessList: IStorageItemAccessList}
DEFINE_IID!(IID_IStorageItemMostRecentlyUsedList, 23214549, 20749, 16670, 140, 241, 195, 209, 239, 250, 76, 51);
RT_INTERFACE!{interface IStorageItemMostRecentlyUsedList(IStorageItemMostRecentlyUsedListVtbl): IInspectable(IInspectableVtbl) [IID_IStorageItemMostRecentlyUsedList] {
    fn add_ItemRemoved(&self, handler: *mut super::super::foundation::TypedEventHandler<StorageItemMostRecentlyUsedList, ItemRemovedEventArgs>, out: *mut super::super::foundation::EventRegistrationToken) -> HRESULT,
    fn remove_ItemRemoved(&self, eventCookie: super::super::foundation::EventRegistrationToken) -> HRESULT
}}
impl IStorageItemMostRecentlyUsedList {
    #[inline] pub unsafe fn add_item_removed(&self, handler: &super::super::foundation::TypedEventHandler<StorageItemMostRecentlyUsedList, ItemRemovedEventArgs>) -> Result<super::super::foundation::EventRegistrationToken> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).add_ItemRemoved)(self as *const _ as *mut _, handler as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn remove_item_removed(&self, eventCookie: super::super::foundation::EventRegistrationToken) -> Result<()> {
        let hr = ((*self.lpVtbl).remove_ItemRemoved)(self as *const _ as *mut _, eventCookie);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
}
RT_CLASS!{class StorageItemMostRecentlyUsedList: IStorageItemMostRecentlyUsedList}
DEFINE_IID!(IID_IStorageItemMostRecentlyUsedList2, 3662159520, 60813, 18225, 161, 219, 228, 78, 226, 32, 64, 147);
RT_INTERFACE!{interface IStorageItemMostRecentlyUsedList2(IStorageItemMostRecentlyUsedList2Vtbl): IInspectable(IInspectableVtbl) [IID_IStorageItemMostRecentlyUsedList2] {
    fn AddWithMetadataAndVisibility(&self, file: *mut super::IStorageItem, metadata: HSTRING, visibility: RecentStorageItemVisibility, out: *mut HSTRING) -> HRESULT,
    fn AddOrReplaceWithMetadataAndVisibility(&self, token: HSTRING, file: *mut super::IStorageItem, metadata: HSTRING, visibility: RecentStorageItemVisibility) -> HRESULT
}}
impl IStorageItemMostRecentlyUsedList2 {
    #[inline] pub unsafe fn add_with_metadata_and_visibility(&self, file: &super::IStorageItem, metadata: &HStringArg, visibility: RecentStorageItemVisibility) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).AddWithMetadataAndVisibility)(self as *const _ as *mut _, file as *const _ as *mut _, metadata.get(), visibility, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn add_or_replace_with_metadata_and_visibility(&self, token: &HStringArg, file: &super::IStorageItem, metadata: &HStringArg, visibility: RecentStorageItemVisibility) -> Result<()> {
        let hr = ((*self.lpVtbl).AddOrReplaceWithMetadataAndVisibility)(self as *const _ as *mut _, token.get(), file as *const _ as *mut _, metadata.get(), visibility);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
}
} // Windows.Storage.AccessCache
pub mod bulkaccess { // Windows.Storage.BulkAccess
use ::prelude::*;
RT_CLASS!{class FileInformation: IStorageItemInformation}
DEFINE_IID!(IID_IFileInformationFactory, 1075677374, 38415, 19821, 167, 208, 26, 56, 97, 231, 108, 131);
RT_INTERFACE!{interface IFileInformationFactory(IFileInformationFactoryVtbl): IInspectable(IInspectableVtbl) [IID_IFileInformationFactory] {
    fn GetItemsAsync(&self, startIndex: u32, maxItemsToRetrieve: u32, out: *mut *mut super::super::foundation::IAsyncOperation<super::super::foundation::collections::IVectorView<IStorageItemInformation>>) -> HRESULT,
    fn GetItemsAsyncDefaultStartAndCount(&self, out: *mut *mut super::super::foundation::IAsyncOperation<super::super::foundation::collections::IVectorView<IStorageItemInformation>>) -> HRESULT,
    fn GetFilesAsync(&self, startIndex: u32, maxItemsToRetrieve: u32, out: *mut *mut super::super::foundation::IAsyncOperation<super::super::foundation::collections::IVectorView<FileInformation>>) -> HRESULT,
    fn GetFilesAsyncDefaultStartAndCount(&self, out: *mut *mut super::super::foundation::IAsyncOperation<super::super::foundation::collections::IVectorView<FileInformation>>) -> HRESULT,
    fn GetFoldersAsync(&self, startIndex: u32, maxItemsToRetrieve: u32, out: *mut *mut super::super::foundation::IAsyncOperation<super::super::foundation::collections::IVectorView<FolderInformation>>) -> HRESULT,
    fn GetFoldersAsyncDefaultStartAndCount(&self, out: *mut *mut super::super::foundation::IAsyncOperation<super::super::foundation::collections::IVectorView<FolderInformation>>) -> HRESULT,
    fn GetVirtualizedItemsVector(&self, out: *mut *mut IInspectable) -> HRESULT,
    fn GetVirtualizedFilesVector(&self, out: *mut *mut IInspectable) -> HRESULT,
    fn GetVirtualizedFoldersVector(&self, out: *mut *mut IInspectable) -> HRESULT
}}
impl IFileInformationFactory {
    #[inline] pub unsafe fn get_items_async(&self, startIndex: u32, maxItemsToRetrieve: u32) -> Result<ComPtr<super::super::foundation::IAsyncOperation<super::super::foundation::collections::IVectorView<IStorageItemInformation>>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetItemsAsync)(self as *const _ as *mut _, startIndex, maxItemsToRetrieve, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_items_async_default_start_and_count(&self) -> Result<ComPtr<super::super::foundation::IAsyncOperation<super::super::foundation::collections::IVectorView<IStorageItemInformation>>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetItemsAsyncDefaultStartAndCount)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_files_async(&self, startIndex: u32, maxItemsToRetrieve: u32) -> Result<ComPtr<super::super::foundation::IAsyncOperation<super::super::foundation::collections::IVectorView<FileInformation>>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetFilesAsync)(self as *const _ as *mut _, startIndex, maxItemsToRetrieve, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_files_async_default_start_and_count(&self) -> Result<ComPtr<super::super::foundation::IAsyncOperation<super::super::foundation::collections::IVectorView<FileInformation>>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetFilesAsyncDefaultStartAndCount)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_folders_async(&self, startIndex: u32, maxItemsToRetrieve: u32) -> Result<ComPtr<super::super::foundation::IAsyncOperation<super::super::foundation::collections::IVectorView<FolderInformation>>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetFoldersAsync)(self as *const _ as *mut _, startIndex, maxItemsToRetrieve, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_folders_async_default_start_and_count(&self) -> Result<ComPtr<super::super::foundation::IAsyncOperation<super::super::foundation::collections::IVectorView<FolderInformation>>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetFoldersAsyncDefaultStartAndCount)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_virtualized_items_vector(&self) -> Result<ComPtr<IInspectable>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetVirtualizedItemsVector)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_virtualized_files_vector(&self) -> Result<ComPtr<IInspectable>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetVirtualizedFilesVector)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_virtualized_folders_vector(&self) -> Result<ComPtr<IInspectable>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetVirtualizedFoldersVector)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
RT_CLASS!{class FileInformationFactory: IFileInformationFactory}
impl RtActivatable<IFileInformationFactoryFactory> for FileInformationFactory {}
impl FileInformationFactory {
    #[inline] pub fn create_with_mode(queryResult: &super::search::IStorageQueryResultBase, mode: super::fileproperties::ThumbnailMode) -> Result<ComPtr<FileInformationFactory>> { unsafe {
        <Self as RtActivatable<IFileInformationFactoryFactory>>::get_activation_factory().create_with_mode(queryResult, mode)
    }}
    #[inline] pub fn create_with_mode_and_size(queryResult: &super::search::IStorageQueryResultBase, mode: super::fileproperties::ThumbnailMode, requestedThumbnailSize: u32) -> Result<ComPtr<FileInformationFactory>> { unsafe {
        <Self as RtActivatable<IFileInformationFactoryFactory>>::get_activation_factory().create_with_mode_and_size(queryResult, mode, requestedThumbnailSize)
    }}
    #[inline] pub fn create_with_mode_and_size_and_options(queryResult: &super::search::IStorageQueryResultBase, mode: super::fileproperties::ThumbnailMode, requestedThumbnailSize: u32, thumbnailOptions: super::fileproperties::ThumbnailOptions) -> Result<ComPtr<FileInformationFactory>> { unsafe {
        <Self as RtActivatable<IFileInformationFactoryFactory>>::get_activation_factory().create_with_mode_and_size_and_options(queryResult, mode, requestedThumbnailSize, thumbnailOptions)
    }}
    #[inline] pub fn create_with_mode_and_size_and_options_and_flags(queryResult: &super::search::IStorageQueryResultBase, mode: super::fileproperties::ThumbnailMode, requestedThumbnailSize: u32, thumbnailOptions: super::fileproperties::ThumbnailOptions, delayLoad: bool) -> Result<ComPtr<FileInformationFactory>> { unsafe {
        <Self as RtActivatable<IFileInformationFactoryFactory>>::get_activation_factory().create_with_mode_and_size_and_options_and_flags(queryResult, mode, requestedThumbnailSize, thumbnailOptions, delayLoad)
    }}
}
DEFINE_CLSID!(FileInformationFactory(&[87,105,110,100,111,119,115,46,83,116,111,114,97,103,101,46,66,117,108,107,65,99,99,101,115,115,46,70,105,108,101,73,110,102,111,114,109,97,116,105,111,110,70,97,99,116,111,114,121,0]) [CLSID_FileInformationFactory]);
DEFINE_IID!(IID_IFileInformationFactoryFactory, 2229931645, 58530, 20224, 138, 250, 175, 94, 15, 130, 107, 213);
RT_INTERFACE!{static interface IFileInformationFactoryFactory(IFileInformationFactoryFactoryVtbl): IInspectable(IInspectableVtbl) [IID_IFileInformationFactoryFactory] {
    fn CreateWithMode(&self, queryResult: *mut super::search::IStorageQueryResultBase, mode: super::fileproperties::ThumbnailMode, out: *mut *mut FileInformationFactory) -> HRESULT,
    fn CreateWithModeAndSize(&self, queryResult: *mut super::search::IStorageQueryResultBase, mode: super::fileproperties::ThumbnailMode, requestedThumbnailSize: u32, out: *mut *mut FileInformationFactory) -> HRESULT,
    fn CreateWithModeAndSizeAndOptions(&self, queryResult: *mut super::search::IStorageQueryResultBase, mode: super::fileproperties::ThumbnailMode, requestedThumbnailSize: u32, thumbnailOptions: super::fileproperties::ThumbnailOptions, out: *mut *mut FileInformationFactory) -> HRESULT,
    fn CreateWithModeAndSizeAndOptionsAndFlags(&self, queryResult: *mut super::search::IStorageQueryResultBase, mode: super::fileproperties::ThumbnailMode, requestedThumbnailSize: u32, thumbnailOptions: super::fileproperties::ThumbnailOptions, delayLoad: bool, out: *mut *mut FileInformationFactory) -> HRESULT
}}
impl IFileInformationFactoryFactory {
    #[inline] pub unsafe fn create_with_mode(&self, queryResult: &super::search::IStorageQueryResultBase, mode: super::fileproperties::ThumbnailMode) -> Result<ComPtr<FileInformationFactory>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).CreateWithMode)(self as *const _ as *mut _, queryResult as *const _ as *mut _, mode, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn create_with_mode_and_size(&self, queryResult: &super::search::IStorageQueryResultBase, mode: super::fileproperties::ThumbnailMode, requestedThumbnailSize: u32) -> Result<ComPtr<FileInformationFactory>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).CreateWithModeAndSize)(self as *const _ as *mut _, queryResult as *const _ as *mut _, mode, requestedThumbnailSize, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn create_with_mode_and_size_and_options(&self, queryResult: &super::search::IStorageQueryResultBase, mode: super::fileproperties::ThumbnailMode, requestedThumbnailSize: u32, thumbnailOptions: super::fileproperties::ThumbnailOptions) -> Result<ComPtr<FileInformationFactory>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).CreateWithModeAndSizeAndOptions)(self as *const _ as *mut _, queryResult as *const _ as *mut _, mode, requestedThumbnailSize, thumbnailOptions, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn create_with_mode_and_size_and_options_and_flags(&self, queryResult: &super::search::IStorageQueryResultBase, mode: super::fileproperties::ThumbnailMode, requestedThumbnailSize: u32, thumbnailOptions: super::fileproperties::ThumbnailOptions, delayLoad: bool) -> Result<ComPtr<FileInformationFactory>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).CreateWithModeAndSizeAndOptionsAndFlags)(self as *const _ as *mut _, queryResult as *const _ as *mut _, mode, requestedThumbnailSize, thumbnailOptions, delayLoad, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
RT_CLASS!{class FolderInformation: IStorageItemInformation}
DEFINE_IID!(IID_IStorageItemInformation, 2275789707, 35186, 20288, 141, 224, 216, 111, 177, 121, 216, 250);
RT_INTERFACE!{interface IStorageItemInformation(IStorageItemInformationVtbl): IInspectable(IInspectableVtbl) [IID_IStorageItemInformation] {
    fn get_MusicProperties(&self, out: *mut *mut super::fileproperties::MusicProperties) -> HRESULT,
    fn get_VideoProperties(&self, out: *mut *mut super::fileproperties::VideoProperties) -> HRESULT,
    fn get_ImageProperties(&self, out: *mut *mut super::fileproperties::ImageProperties) -> HRESULT,
    fn get_DocumentProperties(&self, out: *mut *mut super::fileproperties::DocumentProperties) -> HRESULT,
    fn get_BasicProperties(&self, out: *mut *mut super::fileproperties::BasicProperties) -> HRESULT,
    fn get_Thumbnail(&self, out: *mut *mut super::fileproperties::StorageItemThumbnail) -> HRESULT,
    fn add_ThumbnailUpdated(&self, changedHandler: *mut super::super::foundation::TypedEventHandler<IStorageItemInformation, IInspectable>, out: *mut super::super::foundation::EventRegistrationToken) -> HRESULT,
    fn remove_ThumbnailUpdated(&self, eventCookie: super::super::foundation::EventRegistrationToken) -> HRESULT,
    fn add_PropertiesUpdated(&self, changedHandler: *mut super::super::foundation::TypedEventHandler<IStorageItemInformation, IInspectable>, out: *mut super::super::foundation::EventRegistrationToken) -> HRESULT,
    fn remove_PropertiesUpdated(&self, eventCookie: super::super::foundation::EventRegistrationToken) -> HRESULT
}}
impl IStorageItemInformation {
    #[inline] pub unsafe fn get_music_properties(&self) -> Result<ComPtr<super::fileproperties::MusicProperties>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_MusicProperties)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_video_properties(&self) -> Result<ComPtr<super::fileproperties::VideoProperties>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_VideoProperties)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_image_properties(&self) -> Result<ComPtr<super::fileproperties::ImageProperties>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_ImageProperties)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_document_properties(&self) -> Result<ComPtr<super::fileproperties::DocumentProperties>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_DocumentProperties)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_basic_properties(&self) -> Result<ComPtr<super::fileproperties::BasicProperties>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_BasicProperties)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_thumbnail(&self) -> Result<ComPtr<super::fileproperties::StorageItemThumbnail>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Thumbnail)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn add_thumbnail_updated(&self, changedHandler: &super::super::foundation::TypedEventHandler<IStorageItemInformation, IInspectable>) -> Result<super::super::foundation::EventRegistrationToken> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).add_ThumbnailUpdated)(self as *const _ as *mut _, changedHandler as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn remove_thumbnail_updated(&self, eventCookie: super::super::foundation::EventRegistrationToken) -> Result<()> {
        let hr = ((*self.lpVtbl).remove_ThumbnailUpdated)(self as *const _ as *mut _, eventCookie);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn add_properties_updated(&self, changedHandler: &super::super::foundation::TypedEventHandler<IStorageItemInformation, IInspectable>) -> Result<super::super::foundation::EventRegistrationToken> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).add_PropertiesUpdated)(self as *const _ as *mut _, changedHandler as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn remove_properties_updated(&self, eventCookie: super::super::foundation::EventRegistrationToken) -> Result<()> {
        let hr = ((*self.lpVtbl).remove_PropertiesUpdated)(self as *const _ as *mut _, eventCookie);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
}
} // Windows.Storage.BulkAccess
