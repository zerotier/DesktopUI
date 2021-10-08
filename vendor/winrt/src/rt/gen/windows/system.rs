use ::prelude::*;
DEFINE_IID!(IID_IAppDiagnosticInfo, 3813189274, 34953, 19619, 190, 7, 213, 255, 255, 95, 8, 4);
RT_INTERFACE!{interface IAppDiagnosticInfo(IAppDiagnosticInfoVtbl): IInspectable(IInspectableVtbl) [IID_IAppDiagnosticInfo] {
    #[cfg(feature="windows-applicationmodel")] fn get_AppInfo(&self, out: *mut *mut super::applicationmodel::AppInfo) -> HRESULT
}}
impl IAppDiagnosticInfo {
    #[cfg(feature="windows-applicationmodel")] #[inline] pub unsafe fn get_app_info(&self) -> Result<ComPtr<super::applicationmodel::AppInfo>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_AppInfo)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
RT_CLASS!{class AppDiagnosticInfo: IAppDiagnosticInfo}
impl RtActivatable<IAppDiagnosticInfoStatics> for AppDiagnosticInfo {}
impl RtActivatable<IAppDiagnosticInfoStatics2> for AppDiagnosticInfo {}
impl AppDiagnosticInfo {
    #[inline] pub fn request_info_async() -> Result<ComPtr<super::foundation::IAsyncOperation<super::foundation::collections::IVector<AppDiagnosticInfo>>>> { unsafe {
        <Self as RtActivatable<IAppDiagnosticInfoStatics>>::get_activation_factory().request_info_async()
    }}
    #[inline] pub fn create_watcher() -> Result<ComPtr<AppDiagnosticInfoWatcher>> { unsafe {
        <Self as RtActivatable<IAppDiagnosticInfoStatics2>>::get_activation_factory().create_watcher()
    }}
    #[inline] pub fn request_access_async() -> Result<ComPtr<super::foundation::IAsyncOperation<DiagnosticAccessStatus>>> { unsafe {
        <Self as RtActivatable<IAppDiagnosticInfoStatics2>>::get_activation_factory().request_access_async()
    }}
    #[inline] pub fn request_info_for_package_async(packageFamilyName: &HStringArg) -> Result<ComPtr<super::foundation::IAsyncOperation<super::foundation::collections::IVector<AppDiagnosticInfo>>>> { unsafe {
        <Self as RtActivatable<IAppDiagnosticInfoStatics2>>::get_activation_factory().request_info_for_package_async(packageFamilyName)
    }}
    #[inline] pub fn request_info_for_app_async() -> Result<ComPtr<super::foundation::IAsyncOperation<super::foundation::collections::IVector<AppDiagnosticInfo>>>> { unsafe {
        <Self as RtActivatable<IAppDiagnosticInfoStatics2>>::get_activation_factory().request_info_for_app_async()
    }}
    #[inline] pub fn request_info_for_app_user_model_id(appUserModelId: &HStringArg) -> Result<ComPtr<super::foundation::IAsyncOperation<super::foundation::collections::IVector<AppDiagnosticInfo>>>> { unsafe {
        <Self as RtActivatable<IAppDiagnosticInfoStatics2>>::get_activation_factory().request_info_for_app_user_model_id(appUserModelId)
    }}
}
DEFINE_CLSID!(AppDiagnosticInfo(&[87,105,110,100,111,119,115,46,83,121,115,116,101,109,46,65,112,112,68,105,97,103,110,111,115,116,105,99,73,110,102,111,0]) [CLSID_AppDiagnosticInfo]);
DEFINE_IID!(IID_IAppDiagnosticInfo2, 3745971159, 6426, 17516, 148, 115, 143, 188, 35, 116, 163, 84);
RT_INTERFACE!{interface IAppDiagnosticInfo2(IAppDiagnosticInfo2Vtbl): IInspectable(IInspectableVtbl) [IID_IAppDiagnosticInfo2] {
    fn GetResourceGroups(&self, out: *mut *mut super::foundation::collections::IVector<AppResourceGroupInfo>) -> HRESULT,
    fn CreateResourceGroupWatcher(&self, out: *mut *mut AppResourceGroupInfoWatcher) -> HRESULT
}}
impl IAppDiagnosticInfo2 {
    #[inline] pub unsafe fn get_resource_groups(&self) -> Result<ComPtr<super::foundation::collections::IVector<AppResourceGroupInfo>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetResourceGroups)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn create_resource_group_watcher(&self) -> Result<ComPtr<AppResourceGroupInfoWatcher>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).CreateResourceGroupWatcher)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
DEFINE_IID!(IID_IAppDiagnosticInfoStatics, 3462997439, 4298, 16584, 169, 202, 197, 201, 101, 1, 134, 110);
RT_INTERFACE!{static interface IAppDiagnosticInfoStatics(IAppDiagnosticInfoStaticsVtbl): IInspectable(IInspectableVtbl) [IID_IAppDiagnosticInfoStatics] {
    fn RequestInfoAsync(&self, out: *mut *mut super::foundation::IAsyncOperation<super::foundation::collections::IVector<AppDiagnosticInfo>>) -> HRESULT
}}
impl IAppDiagnosticInfoStatics {
    #[inline] pub unsafe fn request_info_async(&self) -> Result<ComPtr<super::foundation::IAsyncOperation<super::foundation::collections::IVector<AppDiagnosticInfo>>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).RequestInfoAsync)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
DEFINE_IID!(IID_IAppDiagnosticInfoStatics2, 95570822, 4096, 19600, 187, 159, 114, 53, 7, 28, 80, 254);
RT_INTERFACE!{static interface IAppDiagnosticInfoStatics2(IAppDiagnosticInfoStatics2Vtbl): IInspectable(IInspectableVtbl) [IID_IAppDiagnosticInfoStatics2] {
    fn CreateWatcher(&self, out: *mut *mut AppDiagnosticInfoWatcher) -> HRESULT,
    fn RequestAccessAsync(&self, out: *mut *mut super::foundation::IAsyncOperation<DiagnosticAccessStatus>) -> HRESULT,
    fn RequestInfoForPackageAsync(&self, packageFamilyName: HSTRING, out: *mut *mut super::foundation::IAsyncOperation<super::foundation::collections::IVector<AppDiagnosticInfo>>) -> HRESULT,
    fn RequestInfoForAppAsync(&self, out: *mut *mut super::foundation::IAsyncOperation<super::foundation::collections::IVector<AppDiagnosticInfo>>) -> HRESULT,
    fn RequestInfoForAppUserModelId(&self, appUserModelId: HSTRING, out: *mut *mut super::foundation::IAsyncOperation<super::foundation::collections::IVector<AppDiagnosticInfo>>) -> HRESULT
}}
impl IAppDiagnosticInfoStatics2 {
    #[inline] pub unsafe fn create_watcher(&self) -> Result<ComPtr<AppDiagnosticInfoWatcher>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).CreateWatcher)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn request_access_async(&self) -> Result<ComPtr<super::foundation::IAsyncOperation<DiagnosticAccessStatus>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).RequestAccessAsync)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn request_info_for_package_async(&self, packageFamilyName: &HStringArg) -> Result<ComPtr<super::foundation::IAsyncOperation<super::foundation::collections::IVector<AppDiagnosticInfo>>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).RequestInfoForPackageAsync)(self as *const _ as *mut _, packageFamilyName.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn request_info_for_app_async(&self) -> Result<ComPtr<super::foundation::IAsyncOperation<super::foundation::collections::IVector<AppDiagnosticInfo>>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).RequestInfoForAppAsync)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn request_info_for_app_user_model_id(&self, appUserModelId: &HStringArg) -> Result<ComPtr<super::foundation::IAsyncOperation<super::foundation::collections::IVector<AppDiagnosticInfo>>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).RequestInfoForAppUserModelId)(self as *const _ as *mut _, appUserModelId.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
DEFINE_IID!(IID_IAppDiagnosticInfoWatcher, 1968656496, 467, 18586, 147, 37, 82, 249, 204, 110, 222, 10);
RT_INTERFACE!{interface IAppDiagnosticInfoWatcher(IAppDiagnosticInfoWatcherVtbl): IInspectable(IInspectableVtbl) [IID_IAppDiagnosticInfoWatcher] {
    fn add_Added(&self, handler: *mut super::foundation::TypedEventHandler<AppDiagnosticInfoWatcher, AppDiagnosticInfoWatcherEventArgs>, out: *mut super::foundation::EventRegistrationToken) -> HRESULT,
    fn remove_Added(&self, token: super::foundation::EventRegistrationToken) -> HRESULT,
    fn add_Removed(&self, handler: *mut super::foundation::TypedEventHandler<AppDiagnosticInfoWatcher, AppDiagnosticInfoWatcherEventArgs>, out: *mut super::foundation::EventRegistrationToken) -> HRESULT,
    fn remove_Removed(&self, token: super::foundation::EventRegistrationToken) -> HRESULT,
    fn add_EnumerationCompleted(&self, handler: *mut super::foundation::TypedEventHandler<AppDiagnosticInfoWatcher, IInspectable>, out: *mut super::foundation::EventRegistrationToken) -> HRESULT,
    fn remove_EnumerationCompleted(&self, token: super::foundation::EventRegistrationToken) -> HRESULT,
    fn add_Stopped(&self, handler: *mut super::foundation::TypedEventHandler<AppDiagnosticInfoWatcher, IInspectable>, out: *mut super::foundation::EventRegistrationToken) -> HRESULT,
    fn remove_Stopped(&self, token: super::foundation::EventRegistrationToken) -> HRESULT,
    fn get_Status(&self, out: *mut AppDiagnosticInfoWatcherStatus) -> HRESULT,
    fn Start(&self) -> HRESULT,
    fn Stop(&self) -> HRESULT
}}
impl IAppDiagnosticInfoWatcher {
    #[inline] pub unsafe fn add_added(&self, handler: &super::foundation::TypedEventHandler<AppDiagnosticInfoWatcher, AppDiagnosticInfoWatcherEventArgs>) -> Result<super::foundation::EventRegistrationToken> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).add_Added)(self as *const _ as *mut _, handler as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn remove_added(&self, token: super::foundation::EventRegistrationToken) -> Result<()> {
        let hr = ((*self.lpVtbl).remove_Added)(self as *const _ as *mut _, token);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn add_removed(&self, handler: &super::foundation::TypedEventHandler<AppDiagnosticInfoWatcher, AppDiagnosticInfoWatcherEventArgs>) -> Result<super::foundation::EventRegistrationToken> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).add_Removed)(self as *const _ as *mut _, handler as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn remove_removed(&self, token: super::foundation::EventRegistrationToken) -> Result<()> {
        let hr = ((*self.lpVtbl).remove_Removed)(self as *const _ as *mut _, token);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn add_enumeration_completed(&self, handler: &super::foundation::TypedEventHandler<AppDiagnosticInfoWatcher, IInspectable>) -> Result<super::foundation::EventRegistrationToken> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).add_EnumerationCompleted)(self as *const _ as *mut _, handler as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn remove_enumeration_completed(&self, token: super::foundation::EventRegistrationToken) -> Result<()> {
        let hr = ((*self.lpVtbl).remove_EnumerationCompleted)(self as *const _ as *mut _, token);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn add_stopped(&self, handler: &super::foundation::TypedEventHandler<AppDiagnosticInfoWatcher, IInspectable>) -> Result<super::foundation::EventRegistrationToken> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).add_Stopped)(self as *const _ as *mut _, handler as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn remove_stopped(&self, token: super::foundation::EventRegistrationToken) -> Result<()> {
        let hr = ((*self.lpVtbl).remove_Stopped)(self as *const _ as *mut _, token);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_status(&self) -> Result<AppDiagnosticInfoWatcherStatus> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_Status)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn start(&self) -> Result<()> {
        let hr = ((*self.lpVtbl).Start)(self as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn stop(&self) -> Result<()> {
        let hr = ((*self.lpVtbl).Stop)(self as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
}
RT_CLASS!{class AppDiagnosticInfoWatcher: IAppDiagnosticInfoWatcher}
DEFINE_IID!(IID_IAppDiagnosticInfoWatcherEventArgs, 1880606486, 57818, 19557, 153, 223, 4, 109, 255, 91, 231, 26);
RT_INTERFACE!{interface IAppDiagnosticInfoWatcherEventArgs(IAppDiagnosticInfoWatcherEventArgsVtbl): IInspectable(IInspectableVtbl) [IID_IAppDiagnosticInfoWatcherEventArgs] {
    fn get_AppDiagnosticInfo(&self, out: *mut *mut AppDiagnosticInfo) -> HRESULT
}}
impl IAppDiagnosticInfoWatcherEventArgs {
    #[inline] pub unsafe fn get_app_diagnostic_info(&self) -> Result<ComPtr<AppDiagnosticInfo>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_AppDiagnosticInfo)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
RT_CLASS!{class AppDiagnosticInfoWatcherEventArgs: IAppDiagnosticInfoWatcherEventArgs}
RT_ENUM! { enum AppDiagnosticInfoWatcherStatus: i32 {
    Created (AppDiagnosticInfoWatcherStatus_Created) = 0, Started (AppDiagnosticInfoWatcherStatus_Started) = 1, EnumerationCompleted (AppDiagnosticInfoWatcherStatus_EnumerationCompleted) = 2, Stopping (AppDiagnosticInfoWatcherStatus_Stopping) = 3, Stopped (AppDiagnosticInfoWatcherStatus_Stopped) = 4, Aborted (AppDiagnosticInfoWatcherStatus_Aborted) = 5,
}}
DEFINE_IID!(IID_IAppMemoryReport, 1835348891, 19823, 17852, 156, 94, 228, 155, 63, 242, 117, 141);
RT_INTERFACE!{interface IAppMemoryReport(IAppMemoryReportVtbl): IInspectable(IInspectableVtbl) [IID_IAppMemoryReport] {
    fn get_PrivateCommitUsage(&self, out: *mut u64) -> HRESULT,
    fn get_PeakPrivateCommitUsage(&self, out: *mut u64) -> HRESULT,
    fn get_TotalCommitUsage(&self, out: *mut u64) -> HRESULT,
    fn get_TotalCommitLimit(&self, out: *mut u64) -> HRESULT
}}
impl IAppMemoryReport {
    #[inline] pub unsafe fn get_private_commit_usage(&self) -> Result<u64> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_PrivateCommitUsage)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_peak_private_commit_usage(&self) -> Result<u64> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_PeakPrivateCommitUsage)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_total_commit_usage(&self) -> Result<u64> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_TotalCommitUsage)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_total_commit_limit(&self) -> Result<u64> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_TotalCommitLimit)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
}
RT_CLASS!{class AppMemoryReport: IAppMemoryReport}
DEFINE_IID!(IID_IAppMemoryReport2, 1602172728, 20919, 17116, 183, 237, 121, 186, 70, 210, 136, 87);
RT_INTERFACE!{interface IAppMemoryReport2(IAppMemoryReport2Vtbl): IInspectable(IInspectableVtbl) [IID_IAppMemoryReport2] {
    fn get_ExpectedTotalCommitLimit(&self, out: *mut u64) -> HRESULT
}}
impl IAppMemoryReport2 {
    #[inline] pub unsafe fn get_expected_total_commit_limit(&self) -> Result<u64> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_ExpectedTotalCommitLimit)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
}
RT_ENUM! { enum AppMemoryUsageLevel: i32 {
    Low (AppMemoryUsageLevel_Low) = 0, Medium (AppMemoryUsageLevel_Medium) = 1, High (AppMemoryUsageLevel_High) = 2, OverLimit (AppMemoryUsageLevel_OverLimit) = 3,
}}
DEFINE_IID!(IID_IAppMemoryUsageLimitChangingEventArgs, 2046322276, 65226, 19877, 158, 64, 43, 198, 62, 253, 201, 121);
RT_INTERFACE!{interface IAppMemoryUsageLimitChangingEventArgs(IAppMemoryUsageLimitChangingEventArgsVtbl): IInspectable(IInspectableVtbl) [IID_IAppMemoryUsageLimitChangingEventArgs] {
    fn get_OldLimit(&self, out: *mut u64) -> HRESULT,
    fn get_NewLimit(&self, out: *mut u64) -> HRESULT
}}
impl IAppMemoryUsageLimitChangingEventArgs {
    #[inline] pub unsafe fn get_old_limit(&self) -> Result<u64> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_OldLimit)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_new_limit(&self) -> Result<u64> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_NewLimit)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
}
RT_CLASS!{class AppMemoryUsageLimitChangingEventArgs: IAppMemoryUsageLimitChangingEventArgs}
DEFINE_IID!(IID_IAppResourceGroupBackgroundTaskReport, 627500878, 45149, 16578, 157, 193, 26, 79, 3, 158, 161, 32);
RT_INTERFACE!{interface IAppResourceGroupBackgroundTaskReport(IAppResourceGroupBackgroundTaskReportVtbl): IInspectable(IInspectableVtbl) [IID_IAppResourceGroupBackgroundTaskReport] {
    fn get_TaskId(&self, out: *mut Guid) -> HRESULT,
    fn get_Name(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Trigger(&self, out: *mut HSTRING) -> HRESULT,
    fn get_EntryPoint(&self, out: *mut HSTRING) -> HRESULT
}}
impl IAppResourceGroupBackgroundTaskReport {
    #[inline] pub unsafe fn get_task_id(&self) -> Result<Guid> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_TaskId)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_name(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Name)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_trigger(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Trigger)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_entry_point(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_EntryPoint)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
}
RT_CLASS!{class AppResourceGroupBackgroundTaskReport: IAppResourceGroupBackgroundTaskReport}
RT_ENUM! { enum AppResourceGroupEnergyQuotaState: i32 {
    Unknown (AppResourceGroupEnergyQuotaState_Unknown) = 0, Over (AppResourceGroupEnergyQuotaState_Over) = 1, Under (AppResourceGroupEnergyQuotaState_Under) = 2,
}}
RT_ENUM! { enum AppResourceGroupExecutionState: i32 {
    Unknown (AppResourceGroupExecutionState_Unknown) = 0, Running (AppResourceGroupExecutionState_Running) = 1, Suspending (AppResourceGroupExecutionState_Suspending) = 2, Suspended (AppResourceGroupExecutionState_Suspended) = 3, NotRunning (AppResourceGroupExecutionState_NotRunning) = 4,
}}
DEFINE_IID!(IID_IAppResourceGroupInfo, 3105093498, 59399, 18932, 132, 94, 123, 139, 220, 254, 142, 231);
RT_INTERFACE!{interface IAppResourceGroupInfo(IAppResourceGroupInfoVtbl): IInspectable(IInspectableVtbl) [IID_IAppResourceGroupInfo] {
    fn get_InstanceId(&self, out: *mut Guid) -> HRESULT,
    fn get_IsShared(&self, out: *mut bool) -> HRESULT,
    fn GetBackgroundTaskReports(&self, out: *mut *mut super::foundation::collections::IVector<AppResourceGroupBackgroundTaskReport>) -> HRESULT,
    fn GetMemoryReport(&self, out: *mut *mut AppResourceGroupMemoryReport) -> HRESULT,
    fn GetProcessDiagnosticInfos(&self, out: *mut *mut super::foundation::collections::IVector<diagnostics::ProcessDiagnosticInfo>) -> HRESULT,
    fn GetStateReport(&self, out: *mut *mut AppResourceGroupStateReport) -> HRESULT
}}
impl IAppResourceGroupInfo {
    #[inline] pub unsafe fn get_instance_id(&self) -> Result<Guid> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_InstanceId)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_is_shared(&self) -> Result<bool> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_IsShared)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_background_task_reports(&self) -> Result<ComPtr<super::foundation::collections::IVector<AppResourceGroupBackgroundTaskReport>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetBackgroundTaskReports)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_memory_report(&self) -> Result<ComPtr<AppResourceGroupMemoryReport>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetMemoryReport)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_process_diagnostic_infos(&self) -> Result<ComPtr<super::foundation::collections::IVector<diagnostics::ProcessDiagnosticInfo>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetProcessDiagnosticInfos)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_state_report(&self) -> Result<ComPtr<AppResourceGroupStateReport>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetStateReport)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
RT_CLASS!{class AppResourceGroupInfo: IAppResourceGroupInfo}
DEFINE_IID!(IID_IAppResourceGroupInfoWatcher, 3652231421, 28250, 19570, 139, 23, 9, 254, 196, 162, 18, 189);
RT_INTERFACE!{interface IAppResourceGroupInfoWatcher(IAppResourceGroupInfoWatcherVtbl): IInspectable(IInspectableVtbl) [IID_IAppResourceGroupInfoWatcher] {
    fn add_Added(&self, handler: *mut super::foundation::TypedEventHandler<AppResourceGroupInfoWatcher, AppResourceGroupInfoWatcherEventArgs>, out: *mut super::foundation::EventRegistrationToken) -> HRESULT,
    fn remove_Added(&self, token: super::foundation::EventRegistrationToken) -> HRESULT,
    fn add_Removed(&self, handler: *mut super::foundation::TypedEventHandler<AppResourceGroupInfoWatcher, AppResourceGroupInfoWatcherEventArgs>, out: *mut super::foundation::EventRegistrationToken) -> HRESULT,
    fn remove_Removed(&self, token: super::foundation::EventRegistrationToken) -> HRESULT,
    fn add_EnumerationCompleted(&self, handler: *mut super::foundation::TypedEventHandler<AppResourceGroupInfoWatcher, IInspectable>, out: *mut super::foundation::EventRegistrationToken) -> HRESULT,
    fn remove_EnumerationCompleted(&self, token: super::foundation::EventRegistrationToken) -> HRESULT,
    fn add_Stopped(&self, handler: *mut super::foundation::TypedEventHandler<AppResourceGroupInfoWatcher, IInspectable>, out: *mut super::foundation::EventRegistrationToken) -> HRESULT,
    fn remove_Stopped(&self, token: super::foundation::EventRegistrationToken) -> HRESULT,
    fn add_ExecutionStateChanged(&self, handler: *mut super::foundation::TypedEventHandler<AppResourceGroupInfoWatcher, AppResourceGroupInfoWatcherExecutionStateChangedEventArgs>, out: *mut super::foundation::EventRegistrationToken) -> HRESULT,
    fn remove_ExecutionStateChanged(&self, token: super::foundation::EventRegistrationToken) -> HRESULT,
    fn get_Status(&self, out: *mut AppResourceGroupInfoWatcherStatus) -> HRESULT,
    fn Start(&self) -> HRESULT,
    fn Stop(&self) -> HRESULT
}}
impl IAppResourceGroupInfoWatcher {
    #[inline] pub unsafe fn add_added(&self, handler: &super::foundation::TypedEventHandler<AppResourceGroupInfoWatcher, AppResourceGroupInfoWatcherEventArgs>) -> Result<super::foundation::EventRegistrationToken> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).add_Added)(self as *const _ as *mut _, handler as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn remove_added(&self, token: super::foundation::EventRegistrationToken) -> Result<()> {
        let hr = ((*self.lpVtbl).remove_Added)(self as *const _ as *mut _, token);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn add_removed(&self, handler: &super::foundation::TypedEventHandler<AppResourceGroupInfoWatcher, AppResourceGroupInfoWatcherEventArgs>) -> Result<super::foundation::EventRegistrationToken> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).add_Removed)(self as *const _ as *mut _, handler as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn remove_removed(&self, token: super::foundation::EventRegistrationToken) -> Result<()> {
        let hr = ((*self.lpVtbl).remove_Removed)(self as *const _ as *mut _, token);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn add_enumeration_completed(&self, handler: &super::foundation::TypedEventHandler<AppResourceGroupInfoWatcher, IInspectable>) -> Result<super::foundation::EventRegistrationToken> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).add_EnumerationCompleted)(self as *const _ as *mut _, handler as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn remove_enumeration_completed(&self, token: super::foundation::EventRegistrationToken) -> Result<()> {
        let hr = ((*self.lpVtbl).remove_EnumerationCompleted)(self as *const _ as *mut _, token);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn add_stopped(&self, handler: &super::foundation::TypedEventHandler<AppResourceGroupInfoWatcher, IInspectable>) -> Result<super::foundation::EventRegistrationToken> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).add_Stopped)(self as *const _ as *mut _, handler as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn remove_stopped(&self, token: super::foundation::EventRegistrationToken) -> Result<()> {
        let hr = ((*self.lpVtbl).remove_Stopped)(self as *const _ as *mut _, token);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn add_execution_state_changed(&self, handler: &super::foundation::TypedEventHandler<AppResourceGroupInfoWatcher, AppResourceGroupInfoWatcherExecutionStateChangedEventArgs>) -> Result<super::foundation::EventRegistrationToken> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).add_ExecutionStateChanged)(self as *const _ as *mut _, handler as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn remove_execution_state_changed(&self, token: super::foundation::EventRegistrationToken) -> Result<()> {
        let hr = ((*self.lpVtbl).remove_ExecutionStateChanged)(self as *const _ as *mut _, token);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_status(&self) -> Result<AppResourceGroupInfoWatcherStatus> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_Status)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn start(&self) -> Result<()> {
        let hr = ((*self.lpVtbl).Start)(self as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn stop(&self) -> Result<()> {
        let hr = ((*self.lpVtbl).Stop)(self as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
}
RT_CLASS!{class AppResourceGroupInfoWatcher: IAppResourceGroupInfoWatcher}
DEFINE_IID!(IID_IAppResourceGroupInfoWatcherEventArgs, 2054714935, 25346, 19759, 191, 137, 28, 18, 208, 178, 166, 185);
RT_INTERFACE!{interface IAppResourceGroupInfoWatcherEventArgs(IAppResourceGroupInfoWatcherEventArgsVtbl): IInspectable(IInspectableVtbl) [IID_IAppResourceGroupInfoWatcherEventArgs] {
    fn get_AppDiagnosticInfos(&self, out: *mut *mut super::foundation::collections::IVectorView<AppDiagnosticInfo>) -> HRESULT,
    fn get_AppResourceGroupInfo(&self, out: *mut *mut AppResourceGroupInfo) -> HRESULT
}}
impl IAppResourceGroupInfoWatcherEventArgs {
    #[inline] pub unsafe fn get_app_diagnostic_infos(&self) -> Result<ComPtr<super::foundation::collections::IVectorView<AppDiagnosticInfo>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_AppDiagnosticInfos)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_app_resource_group_info(&self) -> Result<ComPtr<AppResourceGroupInfo>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_AppResourceGroupInfo)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
RT_CLASS!{class AppResourceGroupInfoWatcherEventArgs: IAppResourceGroupInfoWatcherEventArgs}
DEFINE_IID!(IID_IAppResourceGroupInfoWatcherExecutionStateChangedEventArgs, 467398103, 65254, 20436, 152, 221, 233, 42, 44, 194, 153, 243);
RT_INTERFACE!{interface IAppResourceGroupInfoWatcherExecutionStateChangedEventArgs(IAppResourceGroupInfoWatcherExecutionStateChangedEventArgsVtbl): IInspectable(IInspectableVtbl) [IID_IAppResourceGroupInfoWatcherExecutionStateChangedEventArgs] {
    fn get_AppDiagnosticInfos(&self, out: *mut *mut super::foundation::collections::IVectorView<AppDiagnosticInfo>) -> HRESULT,
    fn get_AppResourceGroupInfo(&self, out: *mut *mut AppResourceGroupInfo) -> HRESULT
}}
impl IAppResourceGroupInfoWatcherExecutionStateChangedEventArgs {
    #[inline] pub unsafe fn get_app_diagnostic_infos(&self) -> Result<ComPtr<super::foundation::collections::IVectorView<AppDiagnosticInfo>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_AppDiagnosticInfos)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_app_resource_group_info(&self) -> Result<ComPtr<AppResourceGroupInfo>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_AppResourceGroupInfo)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
RT_CLASS!{class AppResourceGroupInfoWatcherExecutionStateChangedEventArgs: IAppResourceGroupInfoWatcherExecutionStateChangedEventArgs}
RT_ENUM! { enum AppResourceGroupInfoWatcherStatus: i32 {
    Created (AppResourceGroupInfoWatcherStatus_Created) = 0, Started (AppResourceGroupInfoWatcherStatus_Started) = 1, EnumerationCompleted (AppResourceGroupInfoWatcherStatus_EnumerationCompleted) = 2, Stopping (AppResourceGroupInfoWatcherStatus_Stopping) = 3, Stopped (AppResourceGroupInfoWatcherStatus_Stopped) = 4, Aborted (AppResourceGroupInfoWatcherStatus_Aborted) = 5,
}}
DEFINE_IID!(IID_IAppResourceGroupMemoryReport, 747374257, 32177, 19537, 162, 37, 127, 174, 45, 73, 228, 49);
RT_INTERFACE!{interface IAppResourceGroupMemoryReport(IAppResourceGroupMemoryReportVtbl): IInspectable(IInspectableVtbl) [IID_IAppResourceGroupMemoryReport] {
    fn get_CommitUsageLimit(&self, out: *mut u64) -> HRESULT,
    fn get_CommitUsageLevel(&self, out: *mut AppMemoryUsageLevel) -> HRESULT,
    fn get_PrivateCommitUsage(&self, out: *mut u64) -> HRESULT,
    fn get_TotalCommitUsage(&self, out: *mut u64) -> HRESULT
}}
impl IAppResourceGroupMemoryReport {
    #[inline] pub unsafe fn get_commit_usage_limit(&self) -> Result<u64> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_CommitUsageLimit)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_commit_usage_level(&self) -> Result<AppMemoryUsageLevel> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_CommitUsageLevel)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_private_commit_usage(&self) -> Result<u64> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_PrivateCommitUsage)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_total_commit_usage(&self) -> Result<u64> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_TotalCommitUsage)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
}
RT_CLASS!{class AppResourceGroupMemoryReport: IAppResourceGroupMemoryReport}
DEFINE_IID!(IID_IAppResourceGroupStateReport, 1384423192, 12144, 16950, 171, 64, 208, 77, 176, 199, 185, 49);
RT_INTERFACE!{interface IAppResourceGroupStateReport(IAppResourceGroupStateReportVtbl): IInspectable(IInspectableVtbl) [IID_IAppResourceGroupStateReport] {
    fn get_ExecutionState(&self, out: *mut AppResourceGroupExecutionState) -> HRESULT,
    fn get_EnergyQuotaState(&self, out: *mut AppResourceGroupEnergyQuotaState) -> HRESULT
}}
impl IAppResourceGroupStateReport {
    #[inline] pub unsafe fn get_execution_state(&self) -> Result<AppResourceGroupExecutionState> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_ExecutionState)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_energy_quota_state(&self) -> Result<AppResourceGroupEnergyQuotaState> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_EnergyQuotaState)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
}
RT_CLASS!{class AppResourceGroupStateReport: IAppResourceGroupStateReport}
RT_CLASS!{static class DateTimeSettings}
impl RtActivatable<IDateTimeSettingsStatics> for DateTimeSettings {}
impl DateTimeSettings {
    #[inline] pub fn set_system_date_time(utcDateTime: super::foundation::DateTime) -> Result<()> { unsafe {
        <Self as RtActivatable<IDateTimeSettingsStatics>>::get_activation_factory().set_system_date_time(utcDateTime)
    }}
}
DEFINE_CLSID!(DateTimeSettings(&[87,105,110,100,111,119,115,46,83,121,115,116,101,109,46,68,97,116,101,84,105,109,101,83,101,116,116,105,110,103,115,0]) [CLSID_DateTimeSettings]);
DEFINE_IID!(IID_IDateTimeSettingsStatics, 1562464465, 18414, 18603, 165, 43, 159, 25, 84, 39, 141, 130);
RT_INTERFACE!{static interface IDateTimeSettingsStatics(IDateTimeSettingsStaticsVtbl): IInspectable(IInspectableVtbl) [IID_IDateTimeSettingsStatics] {
    fn SetSystemDateTime(&self, utcDateTime: super::foundation::DateTime) -> HRESULT
}}
impl IDateTimeSettingsStatics {
    #[inline] pub unsafe fn set_system_date_time(&self, utcDateTime: super::foundation::DateTime) -> Result<()> {
        let hr = ((*self.lpVtbl).SetSystemDateTime)(self as *const _ as *mut _, utcDateTime);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
}
RT_ENUM! { enum DiagnosticAccessStatus: i32 {
    Unspecified (DiagnosticAccessStatus_Unspecified) = 0, Denied (DiagnosticAccessStatus_Denied) = 1, Limited (DiagnosticAccessStatus_Limited) = 2, Allowed (DiagnosticAccessStatus_Allowed) = 3,
}}
DEFINE_IID!(IID_IDispatcherQueue, 1614711012, 41784, 20478, 164, 87, 165, 207, 185, 206, 184, 153);
RT_INTERFACE!{interface IDispatcherQueue(IDispatcherQueueVtbl): IInspectable(IInspectableVtbl) [IID_IDispatcherQueue] {
    fn CreateTimer(&self, out: *mut *mut DispatcherQueueTimer) -> HRESULT,
    fn TryEnqueue(&self, callback: *mut DispatcherQueueHandler, out: *mut bool) -> HRESULT,
    fn TryEnqueueWithPriority(&self, priority: DispatcherQueuePriority, callback: *mut DispatcherQueueHandler, out: *mut bool) -> HRESULT,
    fn add_ShutdownStarting(&self, handler: *mut super::foundation::TypedEventHandler<DispatcherQueue, DispatcherQueueShutdownStartingEventArgs>, out: *mut super::foundation::EventRegistrationToken) -> HRESULT,
    fn remove_ShutdownStarting(&self, token: super::foundation::EventRegistrationToken) -> HRESULT,
    fn add_ShutdownCompleted(&self, handler: *mut super::foundation::TypedEventHandler<DispatcherQueue, IInspectable>, out: *mut super::foundation::EventRegistrationToken) -> HRESULT,
    fn remove_ShutdownCompleted(&self, token: super::foundation::EventRegistrationToken) -> HRESULT
}}
impl IDispatcherQueue {
    #[inline] pub unsafe fn create_timer(&self) -> Result<ComPtr<DispatcherQueueTimer>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).CreateTimer)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn try_enqueue(&self, callback: &DispatcherQueueHandler) -> Result<bool> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).TryEnqueue)(self as *const _ as *mut _, callback as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn try_enqueue_with_priority(&self, priority: DispatcherQueuePriority, callback: &DispatcherQueueHandler) -> Result<bool> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).TryEnqueueWithPriority)(self as *const _ as *mut _, priority, callback as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn add_shutdown_starting(&self, handler: &super::foundation::TypedEventHandler<DispatcherQueue, DispatcherQueueShutdownStartingEventArgs>) -> Result<super::foundation::EventRegistrationToken> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).add_ShutdownStarting)(self as *const _ as *mut _, handler as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn remove_shutdown_starting(&self, token: super::foundation::EventRegistrationToken) -> Result<()> {
        let hr = ((*self.lpVtbl).remove_ShutdownStarting)(self as *const _ as *mut _, token);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn add_shutdown_completed(&self, handler: &super::foundation::TypedEventHandler<DispatcherQueue, IInspectable>) -> Result<super::foundation::EventRegistrationToken> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).add_ShutdownCompleted)(self as *const _ as *mut _, handler as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn remove_shutdown_completed(&self, token: super::foundation::EventRegistrationToken) -> Result<()> {
        let hr = ((*self.lpVtbl).remove_ShutdownCompleted)(self as *const _ as *mut _, token);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
}
RT_CLASS!{class DispatcherQueue: IDispatcherQueue}
impl RtActivatable<IDispatcherQueueStatics> for DispatcherQueue {}
impl DispatcherQueue {
    #[inline] pub fn get_for_current_thread() -> Result<ComPtr<DispatcherQueue>> { unsafe {
        <Self as RtActivatable<IDispatcherQueueStatics>>::get_activation_factory().get_for_current_thread()
    }}
}
DEFINE_CLSID!(DispatcherQueue(&[87,105,110,100,111,119,115,46,83,121,115,116,101,109,46,68,105,115,112,97,116,99,104,101,114,81,117,101,117,101,0]) [CLSID_DispatcherQueue]);
DEFINE_IID!(IID_IDispatcherQueueController, 586370662, 20699, 20022, 169, 141, 97, 192, 27, 56, 77, 32);
RT_INTERFACE!{interface IDispatcherQueueController(IDispatcherQueueControllerVtbl): IInspectable(IInspectableVtbl) [IID_IDispatcherQueueController] {
    fn get_DispatcherQueue(&self, out: *mut *mut DispatcherQueue) -> HRESULT,
    fn ShutdownQueueAsync(&self, out: *mut *mut super::foundation::IAsyncAction) -> HRESULT
}}
impl IDispatcherQueueController {
    #[inline] pub unsafe fn get_dispatcher_queue(&self) -> Result<ComPtr<DispatcherQueue>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_DispatcherQueue)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn shutdown_queue_async(&self) -> Result<ComPtr<super::foundation::IAsyncAction>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).ShutdownQueueAsync)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
RT_CLASS!{class DispatcherQueueController: IDispatcherQueueController}
impl RtActivatable<IDispatcherQueueControllerStatics> for DispatcherQueueController {}
impl DispatcherQueueController {
    #[inline] pub fn create_on_dedicated_thread() -> Result<ComPtr<DispatcherQueueController>> { unsafe {
        <Self as RtActivatable<IDispatcherQueueControllerStatics>>::get_activation_factory().create_on_dedicated_thread()
    }}
}
DEFINE_CLSID!(DispatcherQueueController(&[87,105,110,100,111,119,115,46,83,121,115,116,101,109,46,68,105,115,112,97,116,99,104,101,114,81,117,101,117,101,67,111,110,116,114,111,108,108,101,114,0]) [CLSID_DispatcherQueueController]);
DEFINE_IID!(IID_IDispatcherQueueControllerStatics, 174889184, 20888, 18850, 163, 19, 63, 112, 209, 241, 60, 39);
RT_INTERFACE!{static interface IDispatcherQueueControllerStatics(IDispatcherQueueControllerStaticsVtbl): IInspectable(IInspectableVtbl) [IID_IDispatcherQueueControllerStatics] {
    fn CreateOnDedicatedThread(&self, out: *mut *mut DispatcherQueueController) -> HRESULT
}}
impl IDispatcherQueueControllerStatics {
    #[inline] pub unsafe fn create_on_dedicated_thread(&self) -> Result<ComPtr<DispatcherQueueController>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).CreateOnDedicatedThread)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
DEFINE_IID!(IID_DispatcherQueueHandler, 3751992476, 6701, 18711, 152, 242, 147, 154, 241, 214, 224, 200);
RT_DELEGATE!{delegate DispatcherQueueHandler(DispatcherQueueHandlerVtbl, DispatcherQueueHandlerImpl) [IID_DispatcherQueueHandler] {
    fn Invoke(&self) -> HRESULT
}}
impl DispatcherQueueHandler {
    #[inline] pub unsafe fn invoke(&self) -> Result<()> {
        let hr = ((*self.lpVtbl).Invoke)(self as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
}
RT_ENUM! { enum DispatcherQueuePriority: i32 {
    Low (DispatcherQueuePriority_Low) = -10, Normal (DispatcherQueuePriority_Normal) = 0, High (DispatcherQueuePriority_High) = 10,
}}
DEFINE_IID!(IID_IDispatcherQueueShutdownStartingEventArgs, 3295824972, 65431, 16576, 162, 38, 204, 10, 170, 84, 94, 137);
RT_INTERFACE!{interface IDispatcherQueueShutdownStartingEventArgs(IDispatcherQueueShutdownStartingEventArgsVtbl): IInspectable(IInspectableVtbl) [IID_IDispatcherQueueShutdownStartingEventArgs] {
    fn GetDeferral(&self, out: *mut *mut super::foundation::Deferral) -> HRESULT
}}
impl IDispatcherQueueShutdownStartingEventArgs {
    #[inline] pub unsafe fn get_deferral(&self) -> Result<ComPtr<super::foundation::Deferral>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetDeferral)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
RT_CLASS!{class DispatcherQueueShutdownStartingEventArgs: IDispatcherQueueShutdownStartingEventArgs}
DEFINE_IID!(IID_IDispatcherQueueStatics, 2842526679, 37745, 17687, 146, 69, 208, 130, 74, 193, 44, 116);
RT_INTERFACE!{static interface IDispatcherQueueStatics(IDispatcherQueueStaticsVtbl): IInspectable(IInspectableVtbl) [IID_IDispatcherQueueStatics] {
    fn GetForCurrentThread(&self, out: *mut *mut DispatcherQueue) -> HRESULT
}}
impl IDispatcherQueueStatics {
    #[inline] pub unsafe fn get_for_current_thread(&self) -> Result<ComPtr<DispatcherQueue>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetForCurrentThread)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
DEFINE_IID!(IID_IDispatcherQueueTimer, 1609218845, 41756, 18215, 177, 172, 55, 69, 70, 73, 213, 106);
RT_INTERFACE!{interface IDispatcherQueueTimer(IDispatcherQueueTimerVtbl): IInspectable(IInspectableVtbl) [IID_IDispatcherQueueTimer] {
    fn get_Interval(&self, out: *mut super::foundation::TimeSpan) -> HRESULT,
    fn put_Interval(&self, value: super::foundation::TimeSpan) -> HRESULT,
    fn get_IsRunning(&self, out: *mut bool) -> HRESULT,
    fn get_IsRepeating(&self, out: *mut bool) -> HRESULT,
    fn put_IsRepeating(&self, value: bool) -> HRESULT,
    fn Start(&self) -> HRESULT,
    fn Stop(&self) -> HRESULT,
    fn add_Tick(&self, handler: *mut super::foundation::TypedEventHandler<DispatcherQueueTimer, IInspectable>, out: *mut super::foundation::EventRegistrationToken) -> HRESULT,
    fn remove_Tick(&self, token: super::foundation::EventRegistrationToken) -> HRESULT
}}
impl IDispatcherQueueTimer {
    #[inline] pub unsafe fn get_interval(&self) -> Result<super::foundation::TimeSpan> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_Interval)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_interval(&self, value: super::foundation::TimeSpan) -> Result<()> {
        let hr = ((*self.lpVtbl).put_Interval)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_is_running(&self) -> Result<bool> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_IsRunning)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_is_repeating(&self) -> Result<bool> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_IsRepeating)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_is_repeating(&self, value: bool) -> Result<()> {
        let hr = ((*self.lpVtbl).put_IsRepeating)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn start(&self) -> Result<()> {
        let hr = ((*self.lpVtbl).Start)(self as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn stop(&self) -> Result<()> {
        let hr = ((*self.lpVtbl).Stop)(self as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn add_tick(&self, handler: &super::foundation::TypedEventHandler<DispatcherQueueTimer, IInspectable>) -> Result<super::foundation::EventRegistrationToken> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).add_Tick)(self as *const _ as *mut _, handler as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn remove_tick(&self, token: super::foundation::EventRegistrationToken) -> Result<()> {
        let hr = ((*self.lpVtbl).remove_Tick)(self as *const _ as *mut _, token);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
}
RT_CLASS!{class DispatcherQueueTimer: IDispatcherQueueTimer}
DEFINE_IID!(IID_IFolderLauncherOptions, 3146891901, 27527, 17194, 189, 4, 119, 108, 111, 95, 178, 171);
RT_INTERFACE!{interface IFolderLauncherOptions(IFolderLauncherOptionsVtbl): IInspectable(IInspectableVtbl) [IID_IFolderLauncherOptions] {
    #[cfg(feature="windows-storage")] fn get_ItemsToSelect(&self, out: *mut *mut super::foundation::collections::IVector<super::storage::IStorageItem>) -> HRESULT
}}
impl IFolderLauncherOptions {
    #[cfg(feature="windows-storage")] #[inline] pub unsafe fn get_items_to_select(&self) -> Result<ComPtr<super::foundation::collections::IVector<super::storage::IStorageItem>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_ItemsToSelect)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
RT_CLASS!{class FolderLauncherOptions: IFolderLauncherOptions}
impl RtActivatable<IActivationFactory> for FolderLauncherOptions {}
DEFINE_CLSID!(FolderLauncherOptions(&[87,105,110,100,111,119,115,46,83,121,115,116,101,109,46,70,111,108,100,101,114,76,97,117,110,99,104,101,114,79,112,116,105,111,110,115,0]) [CLSID_FolderLauncherOptions]);
RT_CLASS!{static class KnownUserProperties}
impl RtActivatable<IKnownUserPropertiesStatics> for KnownUserProperties {}
impl KnownUserProperties {
    #[inline] pub fn get_display_name() -> Result<HString> { unsafe {
        <Self as RtActivatable<IKnownUserPropertiesStatics>>::get_activation_factory().get_display_name()
    }}
    #[inline] pub fn get_first_name() -> Result<HString> { unsafe {
        <Self as RtActivatable<IKnownUserPropertiesStatics>>::get_activation_factory().get_first_name()
    }}
    #[inline] pub fn get_last_name() -> Result<HString> { unsafe {
        <Self as RtActivatable<IKnownUserPropertiesStatics>>::get_activation_factory().get_last_name()
    }}
    #[inline] pub fn get_provider_name() -> Result<HString> { unsafe {
        <Self as RtActivatable<IKnownUserPropertiesStatics>>::get_activation_factory().get_provider_name()
    }}
    #[inline] pub fn get_account_name() -> Result<HString> { unsafe {
        <Self as RtActivatable<IKnownUserPropertiesStatics>>::get_activation_factory().get_account_name()
    }}
    #[inline] pub fn get_guest_host() -> Result<HString> { unsafe {
        <Self as RtActivatable<IKnownUserPropertiesStatics>>::get_activation_factory().get_guest_host()
    }}
    #[inline] pub fn get_principal_name() -> Result<HString> { unsafe {
        <Self as RtActivatable<IKnownUserPropertiesStatics>>::get_activation_factory().get_principal_name()
    }}
    #[inline] pub fn get_domain_name() -> Result<HString> { unsafe {
        <Self as RtActivatable<IKnownUserPropertiesStatics>>::get_activation_factory().get_domain_name()
    }}
    #[inline] pub fn get_session_initiation_protocol_uri() -> Result<HString> { unsafe {
        <Self as RtActivatable<IKnownUserPropertiesStatics>>::get_activation_factory().get_session_initiation_protocol_uri()
    }}
}
DEFINE_CLSID!(KnownUserProperties(&[87,105,110,100,111,119,115,46,83,121,115,116,101,109,46,75,110,111,119,110,85,115,101,114,80,114,111,112,101,114,116,105,101,115,0]) [CLSID_KnownUserProperties]);
DEFINE_IID!(IID_IKnownUserPropertiesStatics, 2002096410, 28869, 18661, 182, 55, 91, 163, 68, 30, 78, 228);
RT_INTERFACE!{static interface IKnownUserPropertiesStatics(IKnownUserPropertiesStaticsVtbl): IInspectable(IInspectableVtbl) [IID_IKnownUserPropertiesStatics] {
    fn get_DisplayName(&self, out: *mut HSTRING) -> HRESULT,
    fn get_FirstName(&self, out: *mut HSTRING) -> HRESULT,
    fn get_LastName(&self, out: *mut HSTRING) -> HRESULT,
    fn get_ProviderName(&self, out: *mut HSTRING) -> HRESULT,
    fn get_AccountName(&self, out: *mut HSTRING) -> HRESULT,
    fn get_GuestHost(&self, out: *mut HSTRING) -> HRESULT,
    fn get_PrincipalName(&self, out: *mut HSTRING) -> HRESULT,
    fn get_DomainName(&self, out: *mut HSTRING) -> HRESULT,
    fn get_SessionInitiationProtocolUri(&self, out: *mut HSTRING) -> HRESULT
}}
impl IKnownUserPropertiesStatics {
    #[inline] pub unsafe fn get_display_name(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_DisplayName)(self as *const _ as *mut _, &mut out);
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
    #[inline] pub unsafe fn get_provider_name(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_ProviderName)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_account_name(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_AccountName)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_guest_host(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_GuestHost)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_principal_name(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_PrincipalName)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_domain_name(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_DomainName)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_session_initiation_protocol_uri(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_SessionInitiationProtocolUri)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
}
RT_CLASS!{static class Launcher}
impl RtActivatable<ILauncherStatics> for Launcher {}
impl RtActivatable<ILauncherStatics2> for Launcher {}
impl RtActivatable<ILauncherStatics3> for Launcher {}
impl RtActivatable<ILauncherStatics4> for Launcher {}
impl Launcher {
    #[cfg(feature="windows-storage")] #[inline] pub fn launch_file_async(file: &super::storage::IStorageFile) -> Result<ComPtr<super::foundation::IAsyncOperation<bool>>> { unsafe {
        <Self as RtActivatable<ILauncherStatics>>::get_activation_factory().launch_file_async(file)
    }}
    #[cfg(feature="windows-storage")] #[inline] pub fn launch_file_with_options_async(file: &super::storage::IStorageFile, options: &LauncherOptions) -> Result<ComPtr<super::foundation::IAsyncOperation<bool>>> { unsafe {
        <Self as RtActivatable<ILauncherStatics>>::get_activation_factory().launch_file_with_options_async(file, options)
    }}
    #[inline] pub fn launch_uri_async(uri: &super::foundation::Uri) -> Result<ComPtr<super::foundation::IAsyncOperation<bool>>> { unsafe {
        <Self as RtActivatable<ILauncherStatics>>::get_activation_factory().launch_uri_async(uri)
    }}
    #[inline] pub fn launch_uri_with_options_async(uri: &super::foundation::Uri, options: &LauncherOptions) -> Result<ComPtr<super::foundation::IAsyncOperation<bool>>> { unsafe {
        <Self as RtActivatable<ILauncherStatics>>::get_activation_factory().launch_uri_with_options_async(uri, options)
    }}
    #[inline] pub fn launch_uri_for_results_async(uri: &super::foundation::Uri, options: &LauncherOptions) -> Result<ComPtr<super::foundation::IAsyncOperation<LaunchUriResult>>> { unsafe {
        <Self as RtActivatable<ILauncherStatics2>>::get_activation_factory().launch_uri_for_results_async(uri, options)
    }}
    #[inline] pub fn launch_uri_for_results_with_data_async(uri: &super::foundation::Uri, options: &LauncherOptions, inputData: &super::foundation::collections::ValueSet) -> Result<ComPtr<super::foundation::IAsyncOperation<LaunchUriResult>>> { unsafe {
        <Self as RtActivatable<ILauncherStatics2>>::get_activation_factory().launch_uri_for_results_with_data_async(uri, options, inputData)
    }}
    #[inline] pub fn launch_uri_with_data_async(uri: &super::foundation::Uri, options: &LauncherOptions, inputData: &super::foundation::collections::ValueSet) -> Result<ComPtr<super::foundation::IAsyncOperation<bool>>> { unsafe {
        <Self as RtActivatable<ILauncherStatics2>>::get_activation_factory().launch_uri_with_data_async(uri, options, inputData)
    }}
    #[inline] pub fn query_uri_support_async(uri: &super::foundation::Uri, launchQuerySupportType: LaunchQuerySupportType) -> Result<ComPtr<super::foundation::IAsyncOperation<LaunchQuerySupportStatus>>> { unsafe {
        <Self as RtActivatable<ILauncherStatics2>>::get_activation_factory().query_uri_support_async(uri, launchQuerySupportType)
    }}
    #[inline] pub fn query_uri_support_with_package_family_name_async(uri: &super::foundation::Uri, launchQuerySupportType: LaunchQuerySupportType, packageFamilyName: &HStringArg) -> Result<ComPtr<super::foundation::IAsyncOperation<LaunchQuerySupportStatus>>> { unsafe {
        <Self as RtActivatable<ILauncherStatics2>>::get_activation_factory().query_uri_support_with_package_family_name_async(uri, launchQuerySupportType, packageFamilyName)
    }}
    #[cfg(feature="windows-storage")] #[inline] pub fn query_file_support_async(file: &super::storage::StorageFile) -> Result<ComPtr<super::foundation::IAsyncOperation<LaunchQuerySupportStatus>>> { unsafe {
        <Self as RtActivatable<ILauncherStatics2>>::get_activation_factory().query_file_support_async(file)
    }}
    #[cfg(feature="windows-storage")] #[inline] pub fn query_file_support_with_package_family_name_async(file: &super::storage::StorageFile, packageFamilyName: &HStringArg) -> Result<ComPtr<super::foundation::IAsyncOperation<LaunchQuerySupportStatus>>> { unsafe {
        <Self as RtActivatable<ILauncherStatics2>>::get_activation_factory().query_file_support_with_package_family_name_async(file, packageFamilyName)
    }}
    #[cfg(feature="windows-applicationmodel")] #[inline] pub fn find_uri_scheme_handlers_async(scheme: &HStringArg) -> Result<ComPtr<super::foundation::IAsyncOperation<super::foundation::collections::IVectorView<super::applicationmodel::AppInfo>>>> { unsafe {
        <Self as RtActivatable<ILauncherStatics2>>::get_activation_factory().find_uri_scheme_handlers_async(scheme)
    }}
    #[cfg(feature="windows-applicationmodel")] #[inline] pub fn find_uri_scheme_handlers_with_launch_uri_type_async(scheme: &HStringArg, launchQuerySupportType: LaunchQuerySupportType) -> Result<ComPtr<super::foundation::IAsyncOperation<super::foundation::collections::IVectorView<super::applicationmodel::AppInfo>>>> { unsafe {
        <Self as RtActivatable<ILauncherStatics2>>::get_activation_factory().find_uri_scheme_handlers_with_launch_uri_type_async(scheme, launchQuerySupportType)
    }}
    #[cfg(feature="windows-applicationmodel")] #[inline] pub fn find_file_handlers_async(extension: &HStringArg) -> Result<ComPtr<super::foundation::IAsyncOperation<super::foundation::collections::IVectorView<super::applicationmodel::AppInfo>>>> { unsafe {
        <Self as RtActivatable<ILauncherStatics2>>::get_activation_factory().find_file_handlers_async(extension)
    }}
    #[cfg(feature="windows-storage")] #[inline] pub fn launch_folder_async(folder: &super::storage::IStorageFolder) -> Result<ComPtr<super::foundation::IAsyncOperation<bool>>> { unsafe {
        <Self as RtActivatable<ILauncherStatics3>>::get_activation_factory().launch_folder_async(folder)
    }}
    #[cfg(feature="windows-storage")] #[inline] pub fn launch_folder_with_options_async(folder: &super::storage::IStorageFolder, options: &FolderLauncherOptions) -> Result<ComPtr<super::foundation::IAsyncOperation<bool>>> { unsafe {
        <Self as RtActivatable<ILauncherStatics3>>::get_activation_factory().launch_folder_with_options_async(folder, options)
    }}
    #[inline] pub fn query_app_uri_support_async(uri: &super::foundation::Uri) -> Result<ComPtr<super::foundation::IAsyncOperation<LaunchQuerySupportStatus>>> { unsafe {
        <Self as RtActivatable<ILauncherStatics4>>::get_activation_factory().query_app_uri_support_async(uri)
    }}
    #[inline] pub fn query_app_uri_support_with_package_family_name_async(uri: &super::foundation::Uri, packageFamilyName: &HStringArg) -> Result<ComPtr<super::foundation::IAsyncOperation<LaunchQuerySupportStatus>>> { unsafe {
        <Self as RtActivatable<ILauncherStatics4>>::get_activation_factory().query_app_uri_support_with_package_family_name_async(uri, packageFamilyName)
    }}
    #[cfg(feature="windows-applicationmodel")] #[inline] pub fn find_app_uri_handlers_async(uri: &super::foundation::Uri) -> Result<ComPtr<super::foundation::IAsyncOperation<super::foundation::collections::IVectorView<super::applicationmodel::AppInfo>>>> { unsafe {
        <Self as RtActivatable<ILauncherStatics4>>::get_activation_factory().find_app_uri_handlers_async(uri)
    }}
    #[inline] pub fn launch_uri_for_user_async(user: &User, uri: &super::foundation::Uri) -> Result<ComPtr<super::foundation::IAsyncOperation<LaunchUriStatus>>> { unsafe {
        <Self as RtActivatable<ILauncherStatics4>>::get_activation_factory().launch_uri_for_user_async(user, uri)
    }}
    #[inline] pub fn launch_uri_with_options_for_user_async(user: &User, uri: &super::foundation::Uri, options: &LauncherOptions) -> Result<ComPtr<super::foundation::IAsyncOperation<LaunchUriStatus>>> { unsafe {
        <Self as RtActivatable<ILauncherStatics4>>::get_activation_factory().launch_uri_with_options_for_user_async(user, uri, options)
    }}
    #[inline] pub fn launch_uri_with_data_for_user_async(user: &User, uri: &super::foundation::Uri, options: &LauncherOptions, inputData: &super::foundation::collections::ValueSet) -> Result<ComPtr<super::foundation::IAsyncOperation<LaunchUriStatus>>> { unsafe {
        <Self as RtActivatable<ILauncherStatics4>>::get_activation_factory().launch_uri_with_data_for_user_async(user, uri, options, inputData)
    }}
    #[inline] pub fn launch_uri_for_results_for_user_async(user: &User, uri: &super::foundation::Uri, options: &LauncherOptions) -> Result<ComPtr<super::foundation::IAsyncOperation<LaunchUriResult>>> { unsafe {
        <Self as RtActivatable<ILauncherStatics4>>::get_activation_factory().launch_uri_for_results_for_user_async(user, uri, options)
    }}
    #[inline] pub fn launch_uri_for_results_with_data_for_user_async(user: &User, uri: &super::foundation::Uri, options: &LauncherOptions, inputData: &super::foundation::collections::ValueSet) -> Result<ComPtr<super::foundation::IAsyncOperation<LaunchUriResult>>> { unsafe {
        <Self as RtActivatable<ILauncherStatics4>>::get_activation_factory().launch_uri_for_results_with_data_for_user_async(user, uri, options, inputData)
    }}
}
DEFINE_CLSID!(Launcher(&[87,105,110,100,111,119,115,46,83,121,115,116,101,109,46,76,97,117,110,99,104,101,114,0]) [CLSID_Launcher]);
DEFINE_IID!(IID_ILauncherOptions, 3136954840, 45169, 19672, 133, 62, 52, 18, 3, 229, 87, 211);
RT_INTERFACE!{interface ILauncherOptions(ILauncherOptionsVtbl): IInspectable(IInspectableVtbl) [IID_ILauncherOptions] {
    fn get_TreatAsUntrusted(&self, out: *mut bool) -> HRESULT,
    fn put_TreatAsUntrusted(&self, value: bool) -> HRESULT,
    fn get_DisplayApplicationPicker(&self, out: *mut bool) -> HRESULT,
    fn put_DisplayApplicationPicker(&self, value: bool) -> HRESULT,
    fn get_UI(&self, out: *mut *mut LauncherUIOptions) -> HRESULT,
    fn get_PreferredApplicationPackageFamilyName(&self, out: *mut HSTRING) -> HRESULT,
    fn put_PreferredApplicationPackageFamilyName(&self, value: HSTRING) -> HRESULT,
    fn get_PreferredApplicationDisplayName(&self, out: *mut HSTRING) -> HRESULT,
    fn put_PreferredApplicationDisplayName(&self, value: HSTRING) -> HRESULT,
    fn get_FallbackUri(&self, out: *mut *mut super::foundation::Uri) -> HRESULT,
    fn put_FallbackUri(&self, value: *mut super::foundation::Uri) -> HRESULT,
    fn get_ContentType(&self, out: *mut HSTRING) -> HRESULT,
    fn put_ContentType(&self, value: HSTRING) -> HRESULT
}}
impl ILauncherOptions {
    #[inline] pub unsafe fn get_treat_as_untrusted(&self) -> Result<bool> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_TreatAsUntrusted)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_treat_as_untrusted(&self, value: bool) -> Result<()> {
        let hr = ((*self.lpVtbl).put_TreatAsUntrusted)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_display_application_picker(&self) -> Result<bool> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_DisplayApplicationPicker)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_display_application_picker(&self, value: bool) -> Result<()> {
        let hr = ((*self.lpVtbl).put_DisplayApplicationPicker)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_ui(&self) -> Result<ComPtr<LauncherUIOptions>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_UI)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_preferred_application_package_family_name(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_PreferredApplicationPackageFamilyName)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_preferred_application_package_family_name(&self, value: &HStringArg) -> Result<()> {
        let hr = ((*self.lpVtbl).put_PreferredApplicationPackageFamilyName)(self as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_preferred_application_display_name(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_PreferredApplicationDisplayName)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_preferred_application_display_name(&self, value: &HStringArg) -> Result<()> {
        let hr = ((*self.lpVtbl).put_PreferredApplicationDisplayName)(self as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_fallback_uri(&self) -> Result<ComPtr<super::foundation::Uri>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_FallbackUri)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_fallback_uri(&self, value: &super::foundation::Uri) -> Result<()> {
        let hr = ((*self.lpVtbl).put_FallbackUri)(self as *const _ as *mut _, value as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_content_type(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_ContentType)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_content_type(&self, value: &HStringArg) -> Result<()> {
        let hr = ((*self.lpVtbl).put_ContentType)(self as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
}
RT_CLASS!{class LauncherOptions: ILauncherOptions}
impl RtActivatable<IActivationFactory> for LauncherOptions {}
DEFINE_CLSID!(LauncherOptions(&[87,105,110,100,111,119,115,46,83,121,115,116,101,109,46,76,97,117,110,99,104,101,114,79,112,116,105,111,110,115,0]) [CLSID_LauncherOptions]);
DEFINE_IID!(IID_ILauncherOptions2, 1000378036, 28224, 19918, 161, 163, 47, 83, 149, 10, 251, 73);
RT_INTERFACE!{interface ILauncherOptions2(ILauncherOptions2Vtbl): IInspectable(IInspectableVtbl) [IID_ILauncherOptions2] {
    fn get_TargetApplicationPackageFamilyName(&self, out: *mut HSTRING) -> HRESULT,
    fn put_TargetApplicationPackageFamilyName(&self, value: HSTRING) -> HRESULT,
    #[cfg(feature="windows-storage")] fn get_NeighboringFilesQuery(&self, out: *mut *mut super::storage::search::StorageFileQueryResult) -> HRESULT,
    #[cfg(feature="windows-storage")] fn put_NeighboringFilesQuery(&self, value: *mut super::storage::search::StorageFileQueryResult) -> HRESULT
}}
impl ILauncherOptions2 {
    #[inline] pub unsafe fn get_target_application_package_family_name(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_TargetApplicationPackageFamilyName)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_target_application_package_family_name(&self, value: &HStringArg) -> Result<()> {
        let hr = ((*self.lpVtbl).put_TargetApplicationPackageFamilyName)(self as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[cfg(feature="windows-storage")] #[inline] pub unsafe fn get_neighboring_files_query(&self) -> Result<ComPtr<super::storage::search::StorageFileQueryResult>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_NeighboringFilesQuery)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[cfg(feature="windows-storage")] #[inline] pub unsafe fn set_neighboring_files_query(&self, value: &super::storage::search::StorageFileQueryResult) -> Result<()> {
        let hr = ((*self.lpVtbl).put_NeighboringFilesQuery)(self as *const _ as *mut _, value as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
}
DEFINE_IID!(IID_ILauncherOptions3, 4034332245, 19299, 20026, 145, 7, 78, 104, 120, 65, 146, 58);
RT_INTERFACE!{interface ILauncherOptions3(ILauncherOptions3Vtbl): IInspectable(IInspectableVtbl) [IID_ILauncherOptions3] {
    fn get_IgnoreAppUriHandlers(&self, out: *mut bool) -> HRESULT,
    fn put_IgnoreAppUriHandlers(&self, value: bool) -> HRESULT
}}
impl ILauncherOptions3 {
    #[inline] pub unsafe fn get_ignore_app_uri_handlers(&self) -> Result<bool> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_IgnoreAppUriHandlers)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_ignore_app_uri_handlers(&self, value: bool) -> Result<()> {
        let hr = ((*self.lpVtbl).put_IgnoreAppUriHandlers)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
}
DEFINE_IID!(IID_ILauncherOptions4, 4017082638, 59131, 18452, 164, 78, 87, 232, 185, 217, 160, 27);
RT_INTERFACE!{interface ILauncherOptions4(ILauncherOptions4Vtbl): IInspectable(IInspectableVtbl) [IID_ILauncherOptions4] {
    fn get_LimitPickerToCurrentAppAndAppUriHandlers(&self, out: *mut bool) -> HRESULT,
    fn put_LimitPickerToCurrentAppAndAppUriHandlers(&self, value: bool) -> HRESULT
}}
impl ILauncherOptions4 {
    #[inline] pub unsafe fn get_limit_picker_to_current_app_and_app_uri_handlers(&self) -> Result<bool> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_LimitPickerToCurrentAppAndAppUriHandlers)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_limit_picker_to_current_app_and_app_uri_handlers(&self, value: bool) -> Result<()> {
        let hr = ((*self.lpVtbl).put_LimitPickerToCurrentAppAndAppUriHandlers)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
}
DEFINE_IID!(IID_ILauncherStatics, 661737923, 40510, 17142, 145, 164, 93, 253, 235, 35, 36, 81);
RT_INTERFACE!{static interface ILauncherStatics(ILauncherStaticsVtbl): IInspectable(IInspectableVtbl) [IID_ILauncherStatics] {
    #[cfg(not(feature="windows-storage"))] fn __Dummy0(&self) -> (),
    #[cfg(feature="windows-storage")] fn LaunchFileAsync(&self, file: *mut super::storage::IStorageFile, out: *mut *mut super::foundation::IAsyncOperation<bool>) -> HRESULT,
    #[cfg(not(feature="windows-storage"))] fn __Dummy1(&self) -> (),
    #[cfg(feature="windows-storage")] fn LaunchFileWithOptionsAsync(&self, file: *mut super::storage::IStorageFile, options: *mut LauncherOptions, out: *mut *mut super::foundation::IAsyncOperation<bool>) -> HRESULT,
    fn LaunchUriAsync(&self, uri: *mut super::foundation::Uri, out: *mut *mut super::foundation::IAsyncOperation<bool>) -> HRESULT,
    fn LaunchUriWithOptionsAsync(&self, uri: *mut super::foundation::Uri, options: *mut LauncherOptions, out: *mut *mut super::foundation::IAsyncOperation<bool>) -> HRESULT
}}
impl ILauncherStatics {
    #[cfg(feature="windows-storage")] #[inline] pub unsafe fn launch_file_async(&self, file: &super::storage::IStorageFile) -> Result<ComPtr<super::foundation::IAsyncOperation<bool>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).LaunchFileAsync)(self as *const _ as *mut _, file as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[cfg(feature="windows-storage")] #[inline] pub unsafe fn launch_file_with_options_async(&self, file: &super::storage::IStorageFile, options: &LauncherOptions) -> Result<ComPtr<super::foundation::IAsyncOperation<bool>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).LaunchFileWithOptionsAsync)(self as *const _ as *mut _, file as *const _ as *mut _, options as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn launch_uri_async(&self, uri: &super::foundation::Uri) -> Result<ComPtr<super::foundation::IAsyncOperation<bool>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).LaunchUriAsync)(self as *const _ as *mut _, uri as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn launch_uri_with_options_async(&self, uri: &super::foundation::Uri, options: &LauncherOptions) -> Result<ComPtr<super::foundation::IAsyncOperation<bool>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).LaunchUriWithOptionsAsync)(self as *const _ as *mut _, uri as *const _ as *mut _, options as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
DEFINE_IID!(IID_ILauncherStatics2, 1505374139, 9419, 19458, 164, 196, 130, 148, 86, 157, 84, 241);
RT_INTERFACE!{static interface ILauncherStatics2(ILauncherStatics2Vtbl): IInspectable(IInspectableVtbl) [IID_ILauncherStatics2] {
    fn LaunchUriForResultsAsync(&self, uri: *mut super::foundation::Uri, options: *mut LauncherOptions, out: *mut *mut super::foundation::IAsyncOperation<LaunchUriResult>) -> HRESULT,
    fn LaunchUriForResultsWithDataAsync(&self, uri: *mut super::foundation::Uri, options: *mut LauncherOptions, inputData: *mut super::foundation::collections::ValueSet, out: *mut *mut super::foundation::IAsyncOperation<LaunchUriResult>) -> HRESULT,
    fn LaunchUriWithDataAsync(&self, uri: *mut super::foundation::Uri, options: *mut LauncherOptions, inputData: *mut super::foundation::collections::ValueSet, out: *mut *mut super::foundation::IAsyncOperation<bool>) -> HRESULT,
    fn QueryUriSupportAsync(&self, uri: *mut super::foundation::Uri, launchQuerySupportType: LaunchQuerySupportType, out: *mut *mut super::foundation::IAsyncOperation<LaunchQuerySupportStatus>) -> HRESULT,
    fn QueryUriSupportWithPackageFamilyNameAsync(&self, uri: *mut super::foundation::Uri, launchQuerySupportType: LaunchQuerySupportType, packageFamilyName: HSTRING, out: *mut *mut super::foundation::IAsyncOperation<LaunchQuerySupportStatus>) -> HRESULT,
    #[cfg(not(feature="windows-storage"))] fn __Dummy5(&self) -> (),
    #[cfg(feature="windows-storage")] fn QueryFileSupportAsync(&self, file: *mut super::storage::StorageFile, out: *mut *mut super::foundation::IAsyncOperation<LaunchQuerySupportStatus>) -> HRESULT,
    #[cfg(not(feature="windows-storage"))] fn __Dummy6(&self) -> (),
    #[cfg(feature="windows-storage")] fn QueryFileSupportWithPackageFamilyNameAsync(&self, file: *mut super::storage::StorageFile, packageFamilyName: HSTRING, out: *mut *mut super::foundation::IAsyncOperation<LaunchQuerySupportStatus>) -> HRESULT,
    #[cfg(feature="windows-applicationmodel")] fn FindUriSchemeHandlersAsync(&self, scheme: HSTRING, out: *mut *mut super::foundation::IAsyncOperation<super::foundation::collections::IVectorView<super::applicationmodel::AppInfo>>) -> HRESULT,
    #[cfg(feature="windows-applicationmodel")] fn FindUriSchemeHandlersWithLaunchUriTypeAsync(&self, scheme: HSTRING, launchQuerySupportType: LaunchQuerySupportType, out: *mut *mut super::foundation::IAsyncOperation<super::foundation::collections::IVectorView<super::applicationmodel::AppInfo>>) -> HRESULT,
    #[cfg(feature="windows-applicationmodel")] fn FindFileHandlersAsync(&self, extension: HSTRING, out: *mut *mut super::foundation::IAsyncOperation<super::foundation::collections::IVectorView<super::applicationmodel::AppInfo>>) -> HRESULT
}}
impl ILauncherStatics2 {
    #[inline] pub unsafe fn launch_uri_for_results_async(&self, uri: &super::foundation::Uri, options: &LauncherOptions) -> Result<ComPtr<super::foundation::IAsyncOperation<LaunchUriResult>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).LaunchUriForResultsAsync)(self as *const _ as *mut _, uri as *const _ as *mut _, options as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn launch_uri_for_results_with_data_async(&self, uri: &super::foundation::Uri, options: &LauncherOptions, inputData: &super::foundation::collections::ValueSet) -> Result<ComPtr<super::foundation::IAsyncOperation<LaunchUriResult>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).LaunchUriForResultsWithDataAsync)(self as *const _ as *mut _, uri as *const _ as *mut _, options as *const _ as *mut _, inputData as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn launch_uri_with_data_async(&self, uri: &super::foundation::Uri, options: &LauncherOptions, inputData: &super::foundation::collections::ValueSet) -> Result<ComPtr<super::foundation::IAsyncOperation<bool>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).LaunchUriWithDataAsync)(self as *const _ as *mut _, uri as *const _ as *mut _, options as *const _ as *mut _, inputData as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn query_uri_support_async(&self, uri: &super::foundation::Uri, launchQuerySupportType: LaunchQuerySupportType) -> Result<ComPtr<super::foundation::IAsyncOperation<LaunchQuerySupportStatus>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).QueryUriSupportAsync)(self as *const _ as *mut _, uri as *const _ as *mut _, launchQuerySupportType, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn query_uri_support_with_package_family_name_async(&self, uri: &super::foundation::Uri, launchQuerySupportType: LaunchQuerySupportType, packageFamilyName: &HStringArg) -> Result<ComPtr<super::foundation::IAsyncOperation<LaunchQuerySupportStatus>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).QueryUriSupportWithPackageFamilyNameAsync)(self as *const _ as *mut _, uri as *const _ as *mut _, launchQuerySupportType, packageFamilyName.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[cfg(feature="windows-storage")] #[inline] pub unsafe fn query_file_support_async(&self, file: &super::storage::StorageFile) -> Result<ComPtr<super::foundation::IAsyncOperation<LaunchQuerySupportStatus>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).QueryFileSupportAsync)(self as *const _ as *mut _, file as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[cfg(feature="windows-storage")] #[inline] pub unsafe fn query_file_support_with_package_family_name_async(&self, file: &super::storage::StorageFile, packageFamilyName: &HStringArg) -> Result<ComPtr<super::foundation::IAsyncOperation<LaunchQuerySupportStatus>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).QueryFileSupportWithPackageFamilyNameAsync)(self as *const _ as *mut _, file as *const _ as *mut _, packageFamilyName.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[cfg(feature="windows-applicationmodel")] #[inline] pub unsafe fn find_uri_scheme_handlers_async(&self, scheme: &HStringArg) -> Result<ComPtr<super::foundation::IAsyncOperation<super::foundation::collections::IVectorView<super::applicationmodel::AppInfo>>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).FindUriSchemeHandlersAsync)(self as *const _ as *mut _, scheme.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[cfg(feature="windows-applicationmodel")] #[inline] pub unsafe fn find_uri_scheme_handlers_with_launch_uri_type_async(&self, scheme: &HStringArg, launchQuerySupportType: LaunchQuerySupportType) -> Result<ComPtr<super::foundation::IAsyncOperation<super::foundation::collections::IVectorView<super::applicationmodel::AppInfo>>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).FindUriSchemeHandlersWithLaunchUriTypeAsync)(self as *const _ as *mut _, scheme.get(), launchQuerySupportType, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[cfg(feature="windows-applicationmodel")] #[inline] pub unsafe fn find_file_handlers_async(&self, extension: &HStringArg) -> Result<ComPtr<super::foundation::IAsyncOperation<super::foundation::collections::IVectorView<super::applicationmodel::AppInfo>>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).FindFileHandlersAsync)(self as *const _ as *mut _, extension.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
DEFINE_IID!(IID_ILauncherStatics3, 591552936, 40371, 18051, 170, 66, 220, 111, 81, 211, 56, 71);
RT_INTERFACE!{static interface ILauncherStatics3(ILauncherStatics3Vtbl): IInspectable(IInspectableVtbl) [IID_ILauncherStatics3] {
    #[cfg(feature="windows-storage")] fn LaunchFolderAsync(&self, folder: *mut super::storage::IStorageFolder, out: *mut *mut super::foundation::IAsyncOperation<bool>) -> HRESULT,
    #[cfg(feature="windows-storage")] fn LaunchFolderWithOptionsAsync(&self, folder: *mut super::storage::IStorageFolder, options: *mut FolderLauncherOptions, out: *mut *mut super::foundation::IAsyncOperation<bool>) -> HRESULT
}}
impl ILauncherStatics3 {
    #[cfg(feature="windows-storage")] #[inline] pub unsafe fn launch_folder_async(&self, folder: &super::storage::IStorageFolder) -> Result<ComPtr<super::foundation::IAsyncOperation<bool>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).LaunchFolderAsync)(self as *const _ as *mut _, folder as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[cfg(feature="windows-storage")] #[inline] pub unsafe fn launch_folder_with_options_async(&self, folder: &super::storage::IStorageFolder, options: &FolderLauncherOptions) -> Result<ComPtr<super::foundation::IAsyncOperation<bool>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).LaunchFolderWithOptionsAsync)(self as *const _ as *mut _, folder as *const _ as *mut _, options as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
DEFINE_IID!(IID_ILauncherStatics4, 3119284639, 46501, 16838, 179, 179, 221, 27, 49, 120, 188, 242);
RT_INTERFACE!{static interface ILauncherStatics4(ILauncherStatics4Vtbl): IInspectable(IInspectableVtbl) [IID_ILauncherStatics4] {
    fn QueryAppUriSupportAsync(&self, uri: *mut super::foundation::Uri, out: *mut *mut super::foundation::IAsyncOperation<LaunchQuerySupportStatus>) -> HRESULT,
    fn QueryAppUriSupportWithPackageFamilyNameAsync(&self, uri: *mut super::foundation::Uri, packageFamilyName: HSTRING, out: *mut *mut super::foundation::IAsyncOperation<LaunchQuerySupportStatus>) -> HRESULT,
    #[cfg(not(feature="windows-applicationmodel"))] fn __Dummy2(&self) -> (),
    #[cfg(feature="windows-applicationmodel")] fn FindAppUriHandlersAsync(&self, uri: *mut super::foundation::Uri, out: *mut *mut super::foundation::IAsyncOperation<super::foundation::collections::IVectorView<super::applicationmodel::AppInfo>>) -> HRESULT,
    fn LaunchUriForUserAsync(&self, user: *mut User, uri: *mut super::foundation::Uri, out: *mut *mut super::foundation::IAsyncOperation<LaunchUriStatus>) -> HRESULT,
    fn LaunchUriWithOptionsForUserAsync(&self, user: *mut User, uri: *mut super::foundation::Uri, options: *mut LauncherOptions, out: *mut *mut super::foundation::IAsyncOperation<LaunchUriStatus>) -> HRESULT,
    fn LaunchUriWithDataForUserAsync(&self, user: *mut User, uri: *mut super::foundation::Uri, options: *mut LauncherOptions, inputData: *mut super::foundation::collections::ValueSet, out: *mut *mut super::foundation::IAsyncOperation<LaunchUriStatus>) -> HRESULT,
    fn LaunchUriForResultsForUserAsync(&self, user: *mut User, uri: *mut super::foundation::Uri, options: *mut LauncherOptions, out: *mut *mut super::foundation::IAsyncOperation<LaunchUriResult>) -> HRESULT,
    fn LaunchUriForResultsWithDataForUserAsync(&self, user: *mut User, uri: *mut super::foundation::Uri, options: *mut LauncherOptions, inputData: *mut super::foundation::collections::ValueSet, out: *mut *mut super::foundation::IAsyncOperation<LaunchUriResult>) -> HRESULT
}}
impl ILauncherStatics4 {
    #[inline] pub unsafe fn query_app_uri_support_async(&self, uri: &super::foundation::Uri) -> Result<ComPtr<super::foundation::IAsyncOperation<LaunchQuerySupportStatus>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).QueryAppUriSupportAsync)(self as *const _ as *mut _, uri as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn query_app_uri_support_with_package_family_name_async(&self, uri: &super::foundation::Uri, packageFamilyName: &HStringArg) -> Result<ComPtr<super::foundation::IAsyncOperation<LaunchQuerySupportStatus>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).QueryAppUriSupportWithPackageFamilyNameAsync)(self as *const _ as *mut _, uri as *const _ as *mut _, packageFamilyName.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[cfg(feature="windows-applicationmodel")] #[inline] pub unsafe fn find_app_uri_handlers_async(&self, uri: &super::foundation::Uri) -> Result<ComPtr<super::foundation::IAsyncOperation<super::foundation::collections::IVectorView<super::applicationmodel::AppInfo>>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).FindAppUriHandlersAsync)(self as *const _ as *mut _, uri as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn launch_uri_for_user_async(&self, user: &User, uri: &super::foundation::Uri) -> Result<ComPtr<super::foundation::IAsyncOperation<LaunchUriStatus>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).LaunchUriForUserAsync)(self as *const _ as *mut _, user as *const _ as *mut _, uri as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn launch_uri_with_options_for_user_async(&self, user: &User, uri: &super::foundation::Uri, options: &LauncherOptions) -> Result<ComPtr<super::foundation::IAsyncOperation<LaunchUriStatus>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).LaunchUriWithOptionsForUserAsync)(self as *const _ as *mut _, user as *const _ as *mut _, uri as *const _ as *mut _, options as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn launch_uri_with_data_for_user_async(&self, user: &User, uri: &super::foundation::Uri, options: &LauncherOptions, inputData: &super::foundation::collections::ValueSet) -> Result<ComPtr<super::foundation::IAsyncOperation<LaunchUriStatus>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).LaunchUriWithDataForUserAsync)(self as *const _ as *mut _, user as *const _ as *mut _, uri as *const _ as *mut _, options as *const _ as *mut _, inputData as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn launch_uri_for_results_for_user_async(&self, user: &User, uri: &super::foundation::Uri, options: &LauncherOptions) -> Result<ComPtr<super::foundation::IAsyncOperation<LaunchUriResult>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).LaunchUriForResultsForUserAsync)(self as *const _ as *mut _, user as *const _ as *mut _, uri as *const _ as *mut _, options as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn launch_uri_for_results_with_data_for_user_async(&self, user: &User, uri: &super::foundation::Uri, options: &LauncherOptions, inputData: &super::foundation::collections::ValueSet) -> Result<ComPtr<super::foundation::IAsyncOperation<LaunchUriResult>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).LaunchUriForResultsWithDataForUserAsync)(self as *const _ as *mut _, user as *const _ as *mut _, uri as *const _ as *mut _, options as *const _ as *mut _, inputData as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
DEFINE_IID!(IID_ILauncherUIOptions, 455465582, 35494, 16873, 130, 81, 65, 101, 245, 152, 95, 73);
RT_INTERFACE!{interface ILauncherUIOptions(ILauncherUIOptionsVtbl): IInspectable(IInspectableVtbl) [IID_ILauncherUIOptions] {
    fn get_InvocationPoint(&self, out: *mut *mut super::foundation::IReference<super::foundation::Point>) -> HRESULT,
    fn put_InvocationPoint(&self, value: *mut super::foundation::IReference<super::foundation::Point>) -> HRESULT,
    fn get_SelectionRect(&self, out: *mut *mut super::foundation::IReference<super::foundation::Rect>) -> HRESULT,
    fn put_SelectionRect(&self, value: *mut super::foundation::IReference<super::foundation::Rect>) -> HRESULT,
    #[cfg(feature="windows-ui")] fn get_PreferredPlacement(&self, out: *mut super::ui::popups::Placement) -> HRESULT,
    #[cfg(feature="windows-ui")] fn put_PreferredPlacement(&self, value: super::ui::popups::Placement) -> HRESULT
}}
impl ILauncherUIOptions {
    #[inline] pub unsafe fn get_invocation_point(&self) -> Result<ComPtr<super::foundation::IReference<super::foundation::Point>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_InvocationPoint)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_invocation_point(&self, value: &super::foundation::IReference<super::foundation::Point>) -> Result<()> {
        let hr = ((*self.lpVtbl).put_InvocationPoint)(self as *const _ as *mut _, value as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_selection_rect(&self) -> Result<ComPtr<super::foundation::IReference<super::foundation::Rect>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_SelectionRect)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_selection_rect(&self, value: &super::foundation::IReference<super::foundation::Rect>) -> Result<()> {
        let hr = ((*self.lpVtbl).put_SelectionRect)(self as *const _ as *mut _, value as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[cfg(feature="windows-ui")] #[inline] pub unsafe fn get_preferred_placement(&self) -> Result<super::ui::popups::Placement> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_PreferredPlacement)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[cfg(feature="windows-ui")] #[inline] pub unsafe fn set_preferred_placement(&self, value: super::ui::popups::Placement) -> Result<()> {
        let hr = ((*self.lpVtbl).put_PreferredPlacement)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
}
RT_CLASS!{class LauncherUIOptions: ILauncherUIOptions}
DEFINE_IID!(IID_ILauncherViewOptions, 2325424625, 31911, 18910, 155, 211, 60, 91, 113, 132, 246, 22);
RT_INTERFACE!{interface ILauncherViewOptions(ILauncherViewOptionsVtbl): IInspectable(IInspectableVtbl) [IID_ILauncherViewOptions] {
    #[cfg(feature="windows-ui")] fn get_DesiredRemainingView(&self, out: *mut super::ui::viewmanagement::ViewSizePreference) -> HRESULT,
    #[cfg(feature="windows-ui")] fn put_DesiredRemainingView(&self, value: super::ui::viewmanagement::ViewSizePreference) -> HRESULT
}}
impl ILauncherViewOptions {
    #[cfg(feature="windows-ui")] #[inline] pub unsafe fn get_desired_remaining_view(&self) -> Result<super::ui::viewmanagement::ViewSizePreference> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_DesiredRemainingView)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[cfg(feature="windows-ui")] #[inline] pub unsafe fn set_desired_remaining_view(&self, value: super::ui::viewmanagement::ViewSizePreference) -> Result<()> {
        let hr = ((*self.lpVtbl).put_DesiredRemainingView)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
}
RT_ENUM! { enum LaunchFileStatus: i32 {
    Success (LaunchFileStatus_Success) = 0, AppUnavailable (LaunchFileStatus_AppUnavailable) = 1, DeniedByPolicy (LaunchFileStatus_DeniedByPolicy) = 2, FileTypeNotSupported (LaunchFileStatus_FileTypeNotSupported) = 3, Unknown (LaunchFileStatus_Unknown) = 4,
}}
RT_ENUM! { enum LaunchQuerySupportStatus: i32 {
    Available (LaunchQuerySupportStatus_Available) = 0, AppNotInstalled (LaunchQuerySupportStatus_AppNotInstalled) = 1, AppUnavailable (LaunchQuerySupportStatus_AppUnavailable) = 2, NotSupported (LaunchQuerySupportStatus_NotSupported) = 3, Unknown (LaunchQuerySupportStatus_Unknown) = 4,
}}
RT_ENUM! { enum LaunchQuerySupportType: i32 {
    Uri (LaunchQuerySupportType_Uri) = 0, UriForResults (LaunchQuerySupportType_UriForResults) = 1,
}}
DEFINE_IID!(IID_ILaunchUriResult, 3962022111, 63189, 17866, 145, 58, 112, 164, 12, 92, 130, 33);
RT_INTERFACE!{interface ILaunchUriResult(ILaunchUriResultVtbl): IInspectable(IInspectableVtbl) [IID_ILaunchUriResult] {
    fn get_Status(&self, out: *mut LaunchUriStatus) -> HRESULT,
    fn get_Result(&self, out: *mut *mut super::foundation::collections::ValueSet) -> HRESULT
}}
impl ILaunchUriResult {
    #[inline] pub unsafe fn get_status(&self) -> Result<LaunchUriStatus> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_Status)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_result(&self) -> Result<ComPtr<super::foundation::collections::ValueSet>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Result)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
RT_CLASS!{class LaunchUriResult: ILaunchUriResult}
RT_ENUM! { enum LaunchUriStatus: i32 {
    Success (LaunchUriStatus_Success) = 0, AppUnavailable (LaunchUriStatus_AppUnavailable) = 1, ProtocolUnavailable (LaunchUriStatus_ProtocolUnavailable) = 2, Unknown (LaunchUriStatus_Unknown) = 3,
}}
RT_CLASS!{static class MemoryManager}
impl RtActivatable<IMemoryManagerStatics> for MemoryManager {}
impl RtActivatable<IMemoryManagerStatics2> for MemoryManager {}
impl RtActivatable<IMemoryManagerStatics3> for MemoryManager {}
impl RtActivatable<IMemoryManagerStatics4> for MemoryManager {}
impl MemoryManager {
    #[inline] pub fn get_app_memory_usage() -> Result<u64> { unsafe {
        <Self as RtActivatable<IMemoryManagerStatics>>::get_activation_factory().get_app_memory_usage()
    }}
    #[inline] pub fn get_app_memory_usage_limit() -> Result<u64> { unsafe {
        <Self as RtActivatable<IMemoryManagerStatics>>::get_activation_factory().get_app_memory_usage_limit()
    }}
    #[inline] pub fn get_app_memory_usage_level() -> Result<AppMemoryUsageLevel> { unsafe {
        <Self as RtActivatable<IMemoryManagerStatics>>::get_activation_factory().get_app_memory_usage_level()
    }}
    #[inline] pub fn add_app_memory_usage_increased(handler: &super::foundation::EventHandler<IInspectable>) -> Result<super::foundation::EventRegistrationToken> { unsafe {
        <Self as RtActivatable<IMemoryManagerStatics>>::get_activation_factory().add_app_memory_usage_increased(handler)
    }}
    #[inline] pub fn remove_app_memory_usage_increased(token: super::foundation::EventRegistrationToken) -> Result<()> { unsafe {
        <Self as RtActivatable<IMemoryManagerStatics>>::get_activation_factory().remove_app_memory_usage_increased(token)
    }}
    #[inline] pub fn add_app_memory_usage_decreased(handler: &super::foundation::EventHandler<IInspectable>) -> Result<super::foundation::EventRegistrationToken> { unsafe {
        <Self as RtActivatable<IMemoryManagerStatics>>::get_activation_factory().add_app_memory_usage_decreased(handler)
    }}
    #[inline] pub fn remove_app_memory_usage_decreased(token: super::foundation::EventRegistrationToken) -> Result<()> { unsafe {
        <Self as RtActivatable<IMemoryManagerStatics>>::get_activation_factory().remove_app_memory_usage_decreased(token)
    }}
    #[inline] pub fn add_app_memory_usage_limit_changing(handler: &super::foundation::EventHandler<AppMemoryUsageLimitChangingEventArgs>) -> Result<super::foundation::EventRegistrationToken> { unsafe {
        <Self as RtActivatable<IMemoryManagerStatics>>::get_activation_factory().add_app_memory_usage_limit_changing(handler)
    }}
    #[inline] pub fn remove_app_memory_usage_limit_changing(token: super::foundation::EventRegistrationToken) -> Result<()> { unsafe {
        <Self as RtActivatable<IMemoryManagerStatics>>::get_activation_factory().remove_app_memory_usage_limit_changing(token)
    }}
    #[inline] pub fn get_app_memory_report() -> Result<ComPtr<AppMemoryReport>> { unsafe {
        <Self as RtActivatable<IMemoryManagerStatics2>>::get_activation_factory().get_app_memory_report()
    }}
    #[inline] pub fn get_process_memory_report() -> Result<ComPtr<ProcessMemoryReport>> { unsafe {
        <Self as RtActivatable<IMemoryManagerStatics2>>::get_activation_factory().get_process_memory_report()
    }}
    #[inline] pub fn try_set_app_memory_usage_limit(value: u64) -> Result<bool> { unsafe {
        <Self as RtActivatable<IMemoryManagerStatics3>>::get_activation_factory().try_set_app_memory_usage_limit(value)
    }}
    #[inline] pub fn get_expected_app_memory_usage_limit() -> Result<u64> { unsafe {
        <Self as RtActivatable<IMemoryManagerStatics4>>::get_activation_factory().get_expected_app_memory_usage_limit()
    }}
}
DEFINE_CLSID!(MemoryManager(&[87,105,110,100,111,119,115,46,83,121,115,116,101,109,46,77,101,109,111,114,121,77,97,110,97,103,101,114,0]) [CLSID_MemoryManager]);
DEFINE_IID!(IID_IMemoryManagerStatics, 1550591900, 55242, 18297, 145, 136, 64, 87, 33, 156, 230, 76);
RT_INTERFACE!{static interface IMemoryManagerStatics(IMemoryManagerStaticsVtbl): IInspectable(IInspectableVtbl) [IID_IMemoryManagerStatics] {
    fn get_AppMemoryUsage(&self, out: *mut u64) -> HRESULT,
    fn get_AppMemoryUsageLimit(&self, out: *mut u64) -> HRESULT,
    fn get_AppMemoryUsageLevel(&self, out: *mut AppMemoryUsageLevel) -> HRESULT,
    fn add_AppMemoryUsageIncreased(&self, handler: *mut super::foundation::EventHandler<IInspectable>, out: *mut super::foundation::EventRegistrationToken) -> HRESULT,
    fn remove_AppMemoryUsageIncreased(&self, token: super::foundation::EventRegistrationToken) -> HRESULT,
    fn add_AppMemoryUsageDecreased(&self, handler: *mut super::foundation::EventHandler<IInspectable>, out: *mut super::foundation::EventRegistrationToken) -> HRESULT,
    fn remove_AppMemoryUsageDecreased(&self, token: super::foundation::EventRegistrationToken) -> HRESULT,
    fn add_AppMemoryUsageLimitChanging(&self, handler: *mut super::foundation::EventHandler<AppMemoryUsageLimitChangingEventArgs>, out: *mut super::foundation::EventRegistrationToken) -> HRESULT,
    fn remove_AppMemoryUsageLimitChanging(&self, token: super::foundation::EventRegistrationToken) -> HRESULT
}}
impl IMemoryManagerStatics {
    #[inline] pub unsafe fn get_app_memory_usage(&self) -> Result<u64> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_AppMemoryUsage)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_app_memory_usage_limit(&self) -> Result<u64> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_AppMemoryUsageLimit)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_app_memory_usage_level(&self) -> Result<AppMemoryUsageLevel> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_AppMemoryUsageLevel)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn add_app_memory_usage_increased(&self, handler: &super::foundation::EventHandler<IInspectable>) -> Result<super::foundation::EventRegistrationToken> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).add_AppMemoryUsageIncreased)(self as *const _ as *mut _, handler as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn remove_app_memory_usage_increased(&self, token: super::foundation::EventRegistrationToken) -> Result<()> {
        let hr = ((*self.lpVtbl).remove_AppMemoryUsageIncreased)(self as *const _ as *mut _, token);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn add_app_memory_usage_decreased(&self, handler: &super::foundation::EventHandler<IInspectable>) -> Result<super::foundation::EventRegistrationToken> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).add_AppMemoryUsageDecreased)(self as *const _ as *mut _, handler as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn remove_app_memory_usage_decreased(&self, token: super::foundation::EventRegistrationToken) -> Result<()> {
        let hr = ((*self.lpVtbl).remove_AppMemoryUsageDecreased)(self as *const _ as *mut _, token);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn add_app_memory_usage_limit_changing(&self, handler: &super::foundation::EventHandler<AppMemoryUsageLimitChangingEventArgs>) -> Result<super::foundation::EventRegistrationToken> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).add_AppMemoryUsageLimitChanging)(self as *const _ as *mut _, handler as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn remove_app_memory_usage_limit_changing(&self, token: super::foundation::EventRegistrationToken) -> Result<()> {
        let hr = ((*self.lpVtbl).remove_AppMemoryUsageLimitChanging)(self as *const _ as *mut _, token);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
}
DEFINE_IID!(IID_IMemoryManagerStatics2, 1861104927, 28002, 16959, 148, 121, 176, 31, 156, 159, 118, 105);
RT_INTERFACE!{static interface IMemoryManagerStatics2(IMemoryManagerStatics2Vtbl): IInspectable(IInspectableVtbl) [IID_IMemoryManagerStatics2] {
    fn GetAppMemoryReport(&self, out: *mut *mut AppMemoryReport) -> HRESULT,
    fn GetProcessMemoryReport(&self, out: *mut *mut ProcessMemoryReport) -> HRESULT
}}
impl IMemoryManagerStatics2 {
    #[inline] pub unsafe fn get_app_memory_report(&self) -> Result<ComPtr<AppMemoryReport>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetAppMemoryReport)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_process_memory_report(&self) -> Result<ComPtr<ProcessMemoryReport>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetProcessMemoryReport)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
DEFINE_IID!(IID_IMemoryManagerStatics3, 345725390, 37549, 20021, 137, 235, 80, 223, 180, 192, 217, 28);
RT_INTERFACE!{static interface IMemoryManagerStatics3(IMemoryManagerStatics3Vtbl): IInspectable(IInspectableVtbl) [IID_IMemoryManagerStatics3] {
    fn TrySetAppMemoryUsageLimit(&self, value: u64, out: *mut bool) -> HRESULT
}}
impl IMemoryManagerStatics3 {
    #[inline] pub unsafe fn try_set_app_memory_usage_limit(&self, value: u64) -> Result<bool> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).TrySetAppMemoryUsageLimit)(self as *const _ as *mut _, value, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
}
DEFINE_IID!(IID_IMemoryManagerStatics4, 3316205608, 59470, 18566, 138, 13, 68, 179, 25, 14, 59, 114);
RT_INTERFACE!{static interface IMemoryManagerStatics4(IMemoryManagerStatics4Vtbl): IInspectable(IInspectableVtbl) [IID_IMemoryManagerStatics4] {
    fn get_ExpectedAppMemoryUsageLimit(&self, out: *mut u64) -> HRESULT
}}
impl IMemoryManagerStatics4 {
    #[inline] pub unsafe fn get_expected_app_memory_usage_limit(&self) -> Result<u64> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_ExpectedAppMemoryUsageLimit)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
}
RT_ENUM! { enum PowerState: i32 {
    ConnectedStandby (PowerState_ConnectedStandby) = 0, SleepS3 (PowerState_SleepS3) = 1,
}}
RT_CLASS!{static class ProcessLauncher}
impl RtActivatable<IProcessLauncherStatics> for ProcessLauncher {}
impl ProcessLauncher {
    #[inline] pub fn run_to_completion_async(fileName: &HStringArg, args: &HStringArg) -> Result<ComPtr<super::foundation::IAsyncOperation<ProcessLauncherResult>>> { unsafe {
        <Self as RtActivatable<IProcessLauncherStatics>>::get_activation_factory().run_to_completion_async(fileName, args)
    }}
    #[inline] pub fn run_to_completion_async_with_options(fileName: &HStringArg, args: &HStringArg, options: &ProcessLauncherOptions) -> Result<ComPtr<super::foundation::IAsyncOperation<ProcessLauncherResult>>> { unsafe {
        <Self as RtActivatable<IProcessLauncherStatics>>::get_activation_factory().run_to_completion_async_with_options(fileName, args, options)
    }}
}
DEFINE_CLSID!(ProcessLauncher(&[87,105,110,100,111,119,115,46,83,121,115,116,101,109,46,80,114,111,99,101,115,115,76,97,117,110,99,104,101,114,0]) [CLSID_ProcessLauncher]);
DEFINE_IID!(IID_IProcessLauncherOptions, 813742543, 62532, 19075, 190, 175, 165, 73, 160, 243, 34, 156);
RT_INTERFACE!{interface IProcessLauncherOptions(IProcessLauncherOptionsVtbl): IInspectable(IInspectableVtbl) [IID_IProcessLauncherOptions] {
    #[cfg(not(feature="windows-storage"))] fn __Dummy0(&self) -> (),
    #[cfg(feature="windows-storage")] fn get_StandardInput(&self, out: *mut *mut super::storage::streams::IInputStream) -> HRESULT,
    #[cfg(not(feature="windows-storage"))] fn __Dummy1(&self) -> (),
    #[cfg(feature="windows-storage")] fn put_StandardInput(&self, value: *mut super::storage::streams::IInputStream) -> HRESULT,
    #[cfg(not(feature="windows-storage"))] fn __Dummy2(&self) -> (),
    #[cfg(feature="windows-storage")] fn get_StandardOutput(&self, out: *mut *mut super::storage::streams::IOutputStream) -> HRESULT,
    #[cfg(not(feature="windows-storage"))] fn __Dummy3(&self) -> (),
    #[cfg(feature="windows-storage")] fn put_StandardOutput(&self, value: *mut super::storage::streams::IOutputStream) -> HRESULT,
    #[cfg(not(feature="windows-storage"))] fn __Dummy4(&self) -> (),
    #[cfg(feature="windows-storage")] fn get_StandardError(&self, out: *mut *mut super::storage::streams::IOutputStream) -> HRESULT,
    #[cfg(not(feature="windows-storage"))] fn __Dummy5(&self) -> (),
    #[cfg(feature="windows-storage")] fn put_StandardError(&self, value: *mut super::storage::streams::IOutputStream) -> HRESULT,
    fn get_WorkingDirectory(&self, out: *mut HSTRING) -> HRESULT,
    fn put_WorkingDirectory(&self, value: HSTRING) -> HRESULT
}}
impl IProcessLauncherOptions {
    #[cfg(feature="windows-storage")] #[inline] pub unsafe fn get_standard_input(&self) -> Result<ComPtr<super::storage::streams::IInputStream>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_StandardInput)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[cfg(feature="windows-storage")] #[inline] pub unsafe fn set_standard_input(&self, value: &super::storage::streams::IInputStream) -> Result<()> {
        let hr = ((*self.lpVtbl).put_StandardInput)(self as *const _ as *mut _, value as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[cfg(feature="windows-storage")] #[inline] pub unsafe fn get_standard_output(&self) -> Result<ComPtr<super::storage::streams::IOutputStream>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_StandardOutput)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[cfg(feature="windows-storage")] #[inline] pub unsafe fn set_standard_output(&self, value: &super::storage::streams::IOutputStream) -> Result<()> {
        let hr = ((*self.lpVtbl).put_StandardOutput)(self as *const _ as *mut _, value as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[cfg(feature="windows-storage")] #[inline] pub unsafe fn get_standard_error(&self) -> Result<ComPtr<super::storage::streams::IOutputStream>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_StandardError)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[cfg(feature="windows-storage")] #[inline] pub unsafe fn set_standard_error(&self, value: &super::storage::streams::IOutputStream) -> Result<()> {
        let hr = ((*self.lpVtbl).put_StandardError)(self as *const _ as *mut _, value as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_working_directory(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_WorkingDirectory)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_working_directory(&self, value: &HStringArg) -> Result<()> {
        let hr = ((*self.lpVtbl).put_WorkingDirectory)(self as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
}
RT_CLASS!{class ProcessLauncherOptions: IProcessLauncherOptions}
impl RtActivatable<IActivationFactory> for ProcessLauncherOptions {}
DEFINE_CLSID!(ProcessLauncherOptions(&[87,105,110,100,111,119,115,46,83,121,115,116,101,109,46,80,114,111,99,101,115,115,76,97,117,110,99,104,101,114,79,112,116,105,111,110,115,0]) [CLSID_ProcessLauncherOptions]);
DEFINE_IID!(IID_IProcessLauncherResult, 1414302004, 34520, 18833, 142, 117, 236, 232, 164, 59, 107, 109);
RT_INTERFACE!{interface IProcessLauncherResult(IProcessLauncherResultVtbl): IInspectable(IInspectableVtbl) [IID_IProcessLauncherResult] {
    fn get_ExitCode(&self, out: *mut u32) -> HRESULT
}}
impl IProcessLauncherResult {
    #[inline] pub unsafe fn get_exit_code(&self) -> Result<u32> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_ExitCode)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
}
RT_CLASS!{class ProcessLauncherResult: IProcessLauncherResult}
DEFINE_IID!(IID_IProcessLauncherStatics, 866871015, 11534, 17547, 166, 160, 193, 60, 56, 54, 208, 156);
RT_INTERFACE!{static interface IProcessLauncherStatics(IProcessLauncherStaticsVtbl): IInspectable(IInspectableVtbl) [IID_IProcessLauncherStatics] {
    fn RunToCompletionAsync(&self, fileName: HSTRING, args: HSTRING, out: *mut *mut super::foundation::IAsyncOperation<ProcessLauncherResult>) -> HRESULT,
    fn RunToCompletionAsyncWithOptions(&self, fileName: HSTRING, args: HSTRING, options: *mut ProcessLauncherOptions, out: *mut *mut super::foundation::IAsyncOperation<ProcessLauncherResult>) -> HRESULT
}}
impl IProcessLauncherStatics {
    #[inline] pub unsafe fn run_to_completion_async(&self, fileName: &HStringArg, args: &HStringArg) -> Result<ComPtr<super::foundation::IAsyncOperation<ProcessLauncherResult>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).RunToCompletionAsync)(self as *const _ as *mut _, fileName.get(), args.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn run_to_completion_async_with_options(&self, fileName: &HStringArg, args: &HStringArg, options: &ProcessLauncherOptions) -> Result<ComPtr<super::foundation::IAsyncOperation<ProcessLauncherResult>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).RunToCompletionAsyncWithOptions)(self as *const _ as *mut _, fileName.get(), args.get(), options as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
DEFINE_IID!(IID_IProcessMemoryReport, 141755816, 39792, 18306, 135, 65, 58, 152, 43, 108, 229, 228);
RT_INTERFACE!{interface IProcessMemoryReport(IProcessMemoryReportVtbl): IInspectable(IInspectableVtbl) [IID_IProcessMemoryReport] {
    fn get_PrivateWorkingSetUsage(&self, out: *mut u64) -> HRESULT,
    fn get_TotalWorkingSetUsage(&self, out: *mut u64) -> HRESULT
}}
impl IProcessMemoryReport {
    #[inline] pub unsafe fn get_private_working_set_usage(&self) -> Result<u64> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_PrivateWorkingSetUsage)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_total_working_set_usage(&self) -> Result<u64> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_TotalWorkingSetUsage)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
}
RT_CLASS!{class ProcessMemoryReport: IProcessMemoryReport}
RT_ENUM! { enum ProcessorArchitecture: i32 {
    X86 (ProcessorArchitecture_X86) = 0, Arm (ProcessorArchitecture_Arm) = 5, X64 (ProcessorArchitecture_X64) = 9, Neutral (ProcessorArchitecture_Neutral) = 11, Unknown (ProcessorArchitecture_Unknown) = 65535,
}}
DEFINE_IID!(IID_IProtocolForResultsOperation, 3582011706, 28137, 19752, 147, 120, 248, 103, 130, 225, 130, 187);
RT_INTERFACE!{interface IProtocolForResultsOperation(IProtocolForResultsOperationVtbl): IInspectable(IInspectableVtbl) [IID_IProtocolForResultsOperation] {
    fn ReportCompleted(&self, data: *mut super::foundation::collections::ValueSet) -> HRESULT
}}
impl IProtocolForResultsOperation {
    #[inline] pub unsafe fn report_completed(&self, data: &super::foundation::collections::ValueSet) -> Result<()> {
        let hr = ((*self.lpVtbl).ReportCompleted)(self as *const _ as *mut _, data as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
}
RT_CLASS!{class ProtocolForResultsOperation: IProtocolForResultsOperation}
RT_CLASS!{static class RemoteLauncher}
impl RtActivatable<IRemoteLauncherStatics> for RemoteLauncher {}
impl RemoteLauncher {
    #[inline] pub fn launch_uri_async(remoteSystemConnectionRequest: &remotesystems::RemoteSystemConnectionRequest, uri: &super::foundation::Uri) -> Result<ComPtr<super::foundation::IAsyncOperation<RemoteLaunchUriStatus>>> { unsafe {
        <Self as RtActivatable<IRemoteLauncherStatics>>::get_activation_factory().launch_uri_async(remoteSystemConnectionRequest, uri)
    }}
    #[inline] pub fn launch_uri_with_options_async(remoteSystemConnectionRequest: &remotesystems::RemoteSystemConnectionRequest, uri: &super::foundation::Uri, options: &RemoteLauncherOptions) -> Result<ComPtr<super::foundation::IAsyncOperation<RemoteLaunchUriStatus>>> { unsafe {
        <Self as RtActivatable<IRemoteLauncherStatics>>::get_activation_factory().launch_uri_with_options_async(remoteSystemConnectionRequest, uri, options)
    }}
    #[inline] pub fn launch_uri_with_data_async(remoteSystemConnectionRequest: &remotesystems::RemoteSystemConnectionRequest, uri: &super::foundation::Uri, options: &RemoteLauncherOptions, inputData: &super::foundation::collections::ValueSet) -> Result<ComPtr<super::foundation::IAsyncOperation<RemoteLaunchUriStatus>>> { unsafe {
        <Self as RtActivatable<IRemoteLauncherStatics>>::get_activation_factory().launch_uri_with_data_async(remoteSystemConnectionRequest, uri, options, inputData)
    }}
}
DEFINE_CLSID!(RemoteLauncher(&[87,105,110,100,111,119,115,46,83,121,115,116,101,109,46,82,101,109,111,116,101,76,97,117,110,99,104,101,114,0]) [CLSID_RemoteLauncher]);
DEFINE_IID!(IID_IRemoteLauncherOptions, 2654611336, 10385, 19679, 162, 214, 157, 255, 125, 2, 230, 147);
RT_INTERFACE!{interface IRemoteLauncherOptions(IRemoteLauncherOptionsVtbl): IInspectable(IInspectableVtbl) [IID_IRemoteLauncherOptions] {
    fn get_FallbackUri(&self, out: *mut *mut super::foundation::Uri) -> HRESULT,
    fn put_FallbackUri(&self, value: *mut super::foundation::Uri) -> HRESULT,
    fn get_PreferredAppIds(&self, out: *mut *mut super::foundation::collections::IVector<HString>) -> HRESULT
}}
impl IRemoteLauncherOptions {
    #[inline] pub unsafe fn get_fallback_uri(&self) -> Result<ComPtr<super::foundation::Uri>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_FallbackUri)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_fallback_uri(&self, value: &super::foundation::Uri) -> Result<()> {
        let hr = ((*self.lpVtbl).put_FallbackUri)(self as *const _ as *mut _, value as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_preferred_app_ids(&self) -> Result<ComPtr<super::foundation::collections::IVector<HString>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_PreferredAppIds)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
RT_CLASS!{class RemoteLauncherOptions: IRemoteLauncherOptions}
impl RtActivatable<IActivationFactory> for RemoteLauncherOptions {}
DEFINE_CLSID!(RemoteLauncherOptions(&[87,105,110,100,111,119,115,46,83,121,115,116,101,109,46,82,101,109,111,116,101,76,97,117,110,99,104,101,114,79,112,116,105,111,110,115,0]) [CLSID_RemoteLauncherOptions]);
DEFINE_IID!(IID_IRemoteLauncherStatics, 3621485203, 41740, 18615, 159, 33, 5, 16, 38, 164, 229, 23);
RT_INTERFACE!{static interface IRemoteLauncherStatics(IRemoteLauncherStaticsVtbl): IInspectable(IInspectableVtbl) [IID_IRemoteLauncherStatics] {
    fn LaunchUriAsync(&self, remoteSystemConnectionRequest: *mut remotesystems::RemoteSystemConnectionRequest, uri: *mut super::foundation::Uri, out: *mut *mut super::foundation::IAsyncOperation<RemoteLaunchUriStatus>) -> HRESULT,
    fn LaunchUriWithOptionsAsync(&self, remoteSystemConnectionRequest: *mut remotesystems::RemoteSystemConnectionRequest, uri: *mut super::foundation::Uri, options: *mut RemoteLauncherOptions, out: *mut *mut super::foundation::IAsyncOperation<RemoteLaunchUriStatus>) -> HRESULT,
    fn LaunchUriWithDataAsync(&self, remoteSystemConnectionRequest: *mut remotesystems::RemoteSystemConnectionRequest, uri: *mut super::foundation::Uri, options: *mut RemoteLauncherOptions, inputData: *mut super::foundation::collections::ValueSet, out: *mut *mut super::foundation::IAsyncOperation<RemoteLaunchUriStatus>) -> HRESULT
}}
impl IRemoteLauncherStatics {
    #[inline] pub unsafe fn launch_uri_async(&self, remoteSystemConnectionRequest: &remotesystems::RemoteSystemConnectionRequest, uri: &super::foundation::Uri) -> Result<ComPtr<super::foundation::IAsyncOperation<RemoteLaunchUriStatus>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).LaunchUriAsync)(self as *const _ as *mut _, remoteSystemConnectionRequest as *const _ as *mut _, uri as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn launch_uri_with_options_async(&self, remoteSystemConnectionRequest: &remotesystems::RemoteSystemConnectionRequest, uri: &super::foundation::Uri, options: &RemoteLauncherOptions) -> Result<ComPtr<super::foundation::IAsyncOperation<RemoteLaunchUriStatus>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).LaunchUriWithOptionsAsync)(self as *const _ as *mut _, remoteSystemConnectionRequest as *const _ as *mut _, uri as *const _ as *mut _, options as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn launch_uri_with_data_async(&self, remoteSystemConnectionRequest: &remotesystems::RemoteSystemConnectionRequest, uri: &super::foundation::Uri, options: &RemoteLauncherOptions, inputData: &super::foundation::collections::ValueSet) -> Result<ComPtr<super::foundation::IAsyncOperation<RemoteLaunchUriStatus>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).LaunchUriWithDataAsync)(self as *const _ as *mut _, remoteSystemConnectionRequest as *const _ as *mut _, uri as *const _ as *mut _, options as *const _ as *mut _, inputData as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
RT_ENUM! { enum RemoteLaunchUriStatus: i32 {
    Unknown (RemoteLaunchUriStatus_Unknown) = 0, Success (RemoteLaunchUriStatus_Success) = 1, AppUnavailable (RemoteLaunchUriStatus_AppUnavailable) = 2, ProtocolUnavailable (RemoteLaunchUriStatus_ProtocolUnavailable) = 3, RemoteSystemUnavailable (RemoteLaunchUriStatus_RemoteSystemUnavailable) = 4, ValueSetTooLarge (RemoteLaunchUriStatus_ValueSetTooLarge) = 5, DeniedByLocalSystem (RemoteLaunchUriStatus_DeniedByLocalSystem) = 6, DeniedByRemoteSystem (RemoteLaunchUriStatus_DeniedByRemoteSystem) = 7,
}}
RT_ENUM! { enum ShutdownKind: i32 {
    Shutdown (ShutdownKind_Shutdown) = 0, Restart (ShutdownKind_Restart) = 1,
}}
RT_CLASS!{static class ShutdownManager}
impl RtActivatable<IShutdownManagerStatics> for ShutdownManager {}
impl RtActivatable<IShutdownManagerStatics2> for ShutdownManager {}
impl ShutdownManager {
    #[inline] pub fn begin_shutdown(shutdownKind: ShutdownKind, timeout: super::foundation::TimeSpan) -> Result<()> { unsafe {
        <Self as RtActivatable<IShutdownManagerStatics>>::get_activation_factory().begin_shutdown(shutdownKind, timeout)
    }}
    #[inline] pub fn cancel_shutdown() -> Result<()> { unsafe {
        <Self as RtActivatable<IShutdownManagerStatics>>::get_activation_factory().cancel_shutdown()
    }}
    #[inline] pub fn is_power_state_supported(powerState: PowerState) -> Result<bool> { unsafe {
        <Self as RtActivatable<IShutdownManagerStatics2>>::get_activation_factory().is_power_state_supported(powerState)
    }}
    #[inline] pub fn enter_power_state(powerState: PowerState) -> Result<()> { unsafe {
        <Self as RtActivatable<IShutdownManagerStatics2>>::get_activation_factory().enter_power_state(powerState)
    }}
    #[inline] pub fn enter_power_state_with_time_span(powerState: PowerState, wakeUpAfter: super::foundation::TimeSpan) -> Result<()> { unsafe {
        <Self as RtActivatable<IShutdownManagerStatics2>>::get_activation_factory().enter_power_state_with_time_span(powerState, wakeUpAfter)
    }}
}
DEFINE_CLSID!(ShutdownManager(&[87,105,110,100,111,119,115,46,83,121,115,116,101,109,46,83,104,117,116,100,111,119,110,77,97,110,97,103,101,114,0]) [CLSID_ShutdownManager]);
DEFINE_IID!(IID_IShutdownManagerStatics, 1927432173, 56667, 19820, 177, 208, 197, 122, 123, 187, 95, 148);
RT_INTERFACE!{static interface IShutdownManagerStatics(IShutdownManagerStaticsVtbl): IInspectable(IInspectableVtbl) [IID_IShutdownManagerStatics] {
    fn BeginShutdown(&self, shutdownKind: ShutdownKind, timeout: super::foundation::TimeSpan) -> HRESULT,
    fn CancelShutdown(&self) -> HRESULT
}}
impl IShutdownManagerStatics {
    #[inline] pub unsafe fn begin_shutdown(&self, shutdownKind: ShutdownKind, timeout: super::foundation::TimeSpan) -> Result<()> {
        let hr = ((*self.lpVtbl).BeginShutdown)(self as *const _ as *mut _, shutdownKind, timeout);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn cancel_shutdown(&self) -> Result<()> {
        let hr = ((*self.lpVtbl).CancelShutdown)(self as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
}
DEFINE_IID!(IID_IShutdownManagerStatics2, 258580527, 39988, 17351, 168, 195, 112, 179, 10, 127, 117, 4);
RT_INTERFACE!{static interface IShutdownManagerStatics2(IShutdownManagerStatics2Vtbl): IInspectable(IInspectableVtbl) [IID_IShutdownManagerStatics2] {
    fn IsPowerStateSupported(&self, powerState: PowerState, out: *mut bool) -> HRESULT,
    fn EnterPowerState(&self, powerState: PowerState) -> HRESULT,
    fn EnterPowerStateWithTimeSpan(&self, powerState: PowerState, wakeUpAfter: super::foundation::TimeSpan) -> HRESULT
}}
impl IShutdownManagerStatics2 {
    #[inline] pub unsafe fn is_power_state_supported(&self, powerState: PowerState) -> Result<bool> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).IsPowerStateSupported)(self as *const _ as *mut _, powerState, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn enter_power_state(&self, powerState: PowerState) -> Result<()> {
        let hr = ((*self.lpVtbl).EnterPowerState)(self as *const _ as *mut _, powerState);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn enter_power_state_with_time_span(&self, powerState: PowerState, wakeUpAfter: super::foundation::TimeSpan) -> Result<()> {
        let hr = ((*self.lpVtbl).EnterPowerStateWithTimeSpan)(self as *const _ as *mut _, powerState, wakeUpAfter);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
}
RT_CLASS!{static class TimeZoneSettings}
impl RtActivatable<ITimeZoneSettingsStatics> for TimeZoneSettings {}
impl TimeZoneSettings {
    #[inline] pub fn get_current_time_zone_display_name() -> Result<HString> { unsafe {
        <Self as RtActivatable<ITimeZoneSettingsStatics>>::get_activation_factory().get_current_time_zone_display_name()
    }}
    #[inline] pub fn get_supported_time_zone_display_names() -> Result<ComPtr<super::foundation::collections::IVectorView<HString>>> { unsafe {
        <Self as RtActivatable<ITimeZoneSettingsStatics>>::get_activation_factory().get_supported_time_zone_display_names()
    }}
    #[inline] pub fn get_can_change_time_zone() -> Result<bool> { unsafe {
        <Self as RtActivatable<ITimeZoneSettingsStatics>>::get_activation_factory().get_can_change_time_zone()
    }}
    #[inline] pub fn change_time_zone_by_display_name(timeZoneDisplayName: &HStringArg) -> Result<()> { unsafe {
        <Self as RtActivatable<ITimeZoneSettingsStatics>>::get_activation_factory().change_time_zone_by_display_name(timeZoneDisplayName)
    }}
}
DEFINE_CLSID!(TimeZoneSettings(&[87,105,110,100,111,119,115,46,83,121,115,116,101,109,46,84,105,109,101,90,111,110,101,83,101,116,116,105,110,103,115,0]) [CLSID_TimeZoneSettings]);
DEFINE_IID!(IID_ITimeZoneSettingsStatics, 2604346346, 41217, 16814, 159, 189, 2, 135, 40, 186, 183, 61);
RT_INTERFACE!{static interface ITimeZoneSettingsStatics(ITimeZoneSettingsStaticsVtbl): IInspectable(IInspectableVtbl) [IID_ITimeZoneSettingsStatics] {
    fn get_CurrentTimeZoneDisplayName(&self, out: *mut HSTRING) -> HRESULT,
    fn get_SupportedTimeZoneDisplayNames(&self, out: *mut *mut super::foundation::collections::IVectorView<HString>) -> HRESULT,
    fn get_CanChangeTimeZone(&self, out: *mut bool) -> HRESULT,
    fn ChangeTimeZoneByDisplayName(&self, timeZoneDisplayName: HSTRING) -> HRESULT
}}
impl ITimeZoneSettingsStatics {
    #[inline] pub unsafe fn get_current_time_zone_display_name(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_CurrentTimeZoneDisplayName)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_supported_time_zone_display_names(&self) -> Result<ComPtr<super::foundation::collections::IVectorView<HString>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_SupportedTimeZoneDisplayNames)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_can_change_time_zone(&self) -> Result<bool> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_CanChangeTimeZone)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn change_time_zone_by_display_name(&self, timeZoneDisplayName: &HStringArg) -> Result<()> {
        let hr = ((*self.lpVtbl).ChangeTimeZoneByDisplayName)(self as *const _ as *mut _, timeZoneDisplayName.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
}
DEFINE_IID!(IID_IUser, 3751421638, 59206, 19405, 181, 212, 18, 1, 3, 196, 32, 155);
RT_INTERFACE!{interface IUser(IUserVtbl): IInspectable(IInspectableVtbl) [IID_IUser] {
    fn get_NonRoamableId(&self, out: *mut HSTRING) -> HRESULT,
    fn get_AuthenticationStatus(&self, out: *mut UserAuthenticationStatus) -> HRESULT,
    fn get_Type(&self, out: *mut UserType) -> HRESULT,
    fn GetPropertyAsync(&self, value: HSTRING, out: *mut *mut super::foundation::IAsyncOperation<IInspectable>) -> HRESULT,
    fn GetPropertiesAsync(&self, values: *mut super::foundation::collections::IVectorView<HString>, out: *mut *mut super::foundation::IAsyncOperation<super::foundation::collections::IPropertySet>) -> HRESULT,
    #[cfg(feature="windows-storage")] fn GetPictureAsync(&self, desiredSize: UserPictureSize, out: *mut *mut super::foundation::IAsyncOperation<super::storage::streams::IRandomAccessStreamReference>) -> HRESULT
}}
impl IUser {
    #[inline] pub unsafe fn get_non_roamable_id(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_NonRoamableId)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_authentication_status(&self) -> Result<UserAuthenticationStatus> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_AuthenticationStatus)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_type(&self) -> Result<UserType> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_Type)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_property_async(&self, value: &HStringArg) -> Result<ComPtr<super::foundation::IAsyncOperation<IInspectable>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetPropertyAsync)(self as *const _ as *mut _, value.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_properties_async(&self, values: &super::foundation::collections::IVectorView<HString>) -> Result<ComPtr<super::foundation::IAsyncOperation<super::foundation::collections::IPropertySet>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetPropertiesAsync)(self as *const _ as *mut _, values as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[cfg(feature="windows-storage")] #[inline] pub unsafe fn get_picture_async(&self, desiredSize: UserPictureSize) -> Result<ComPtr<super::foundation::IAsyncOperation<super::storage::streams::IRandomAccessStreamReference>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetPictureAsync)(self as *const _ as *mut _, desiredSize, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
RT_CLASS!{class User: IUser}
impl RtActivatable<IUserStatics> for User {}
impl User {
    #[inline] pub fn create_watcher() -> Result<ComPtr<UserWatcher>> { unsafe {
        <Self as RtActivatable<IUserStatics>>::get_activation_factory().create_watcher()
    }}
    #[inline] pub fn find_all_async() -> Result<ComPtr<super::foundation::IAsyncOperation<super::foundation::collections::IVectorView<User>>>> { unsafe {
        <Self as RtActivatable<IUserStatics>>::get_activation_factory().find_all_async()
    }}
    #[inline] pub fn find_all_async_by_type(type_: UserType) -> Result<ComPtr<super::foundation::IAsyncOperation<super::foundation::collections::IVectorView<User>>>> { unsafe {
        <Self as RtActivatable<IUserStatics>>::get_activation_factory().find_all_async_by_type(type_)
    }}
    #[inline] pub fn find_all_async_by_type_and_status(type_: UserType, status: UserAuthenticationStatus) -> Result<ComPtr<super::foundation::IAsyncOperation<super::foundation::collections::IVectorView<User>>>> { unsafe {
        <Self as RtActivatable<IUserStatics>>::get_activation_factory().find_all_async_by_type_and_status(type_, status)
    }}
    #[inline] pub fn get_from_id(nonRoamableId: &HStringArg) -> Result<ComPtr<User>> { unsafe {
        <Self as RtActivatable<IUserStatics>>::get_activation_factory().get_from_id(nonRoamableId)
    }}
}
DEFINE_CLSID!(User(&[87,105,110,100,111,119,115,46,83,121,115,116,101,109,46,85,115,101,114,0]) [CLSID_User]);
RT_ENUM! { enum UserAuthenticationStatus: i32 {
    Unauthenticated (UserAuthenticationStatus_Unauthenticated) = 0, LocallyAuthenticated (UserAuthenticationStatus_LocallyAuthenticated) = 1, RemotelyAuthenticated (UserAuthenticationStatus_RemotelyAuthenticated) = 2,
}}
DEFINE_IID!(IID_IUserAuthenticationStatusChangeDeferral, 2293601640, 47920, 17147, 162, 112, 233, 144, 46, 64, 239, 167);
RT_INTERFACE!{interface IUserAuthenticationStatusChangeDeferral(IUserAuthenticationStatusChangeDeferralVtbl): IInspectable(IInspectableVtbl) [IID_IUserAuthenticationStatusChangeDeferral] {
    fn Complete(&self) -> HRESULT
}}
impl IUserAuthenticationStatusChangeDeferral {
    #[inline] pub unsafe fn complete(&self) -> Result<()> {
        let hr = ((*self.lpVtbl).Complete)(self as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
}
RT_CLASS!{class UserAuthenticationStatusChangeDeferral: IUserAuthenticationStatusChangeDeferral}
DEFINE_IID!(IID_IUserAuthenticationStatusChangingEventArgs, 2349010728, 42769, 19486, 171, 72, 4, 23, 156, 21, 147, 143);
RT_INTERFACE!{interface IUserAuthenticationStatusChangingEventArgs(IUserAuthenticationStatusChangingEventArgsVtbl): IInspectable(IInspectableVtbl) [IID_IUserAuthenticationStatusChangingEventArgs] {
    fn GetDeferral(&self, out: *mut *mut UserAuthenticationStatusChangeDeferral) -> HRESULT,
    fn get_User(&self, out: *mut *mut User) -> HRESULT,
    fn get_NewStatus(&self, out: *mut UserAuthenticationStatus) -> HRESULT,
    fn get_CurrentStatus(&self, out: *mut UserAuthenticationStatus) -> HRESULT
}}
impl IUserAuthenticationStatusChangingEventArgs {
    #[inline] pub unsafe fn get_deferral(&self) -> Result<ComPtr<UserAuthenticationStatusChangeDeferral>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetDeferral)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_user(&self) -> Result<ComPtr<User>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_User)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_new_status(&self) -> Result<UserAuthenticationStatus> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_NewStatus)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_current_status(&self) -> Result<UserAuthenticationStatus> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_CurrentStatus)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
}
RT_CLASS!{class UserAuthenticationStatusChangingEventArgs: IUserAuthenticationStatusChangingEventArgs}
DEFINE_IID!(IID_IUserChangedEventArgs, 140794332, 6342, 18651, 188, 153, 114, 79, 185, 32, 60, 204);
RT_INTERFACE!{interface IUserChangedEventArgs(IUserChangedEventArgsVtbl): IInspectable(IInspectableVtbl) [IID_IUserChangedEventArgs] {
    fn get_User(&self, out: *mut *mut User) -> HRESULT
}}
impl IUserChangedEventArgs {
    #[inline] pub unsafe fn get_user(&self) -> Result<ComPtr<User>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_User)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
RT_CLASS!{class UserChangedEventArgs: IUserChangedEventArgs}
RT_CLASS!{static class UserDeviceAssociation}
impl RtActivatable<IUserDeviceAssociationStatics> for UserDeviceAssociation {}
impl UserDeviceAssociation {
    #[inline] pub fn find_user_from_device_id(deviceId: &HStringArg) -> Result<ComPtr<User>> { unsafe {
        <Self as RtActivatable<IUserDeviceAssociationStatics>>::get_activation_factory().find_user_from_device_id(deviceId)
    }}
    #[inline] pub fn add_user_device_association_changed(handler: &super::foundation::EventHandler<UserDeviceAssociationChangedEventArgs>) -> Result<super::foundation::EventRegistrationToken> { unsafe {
        <Self as RtActivatable<IUserDeviceAssociationStatics>>::get_activation_factory().add_user_device_association_changed(handler)
    }}
    #[inline] pub fn remove_user_device_association_changed(token: super::foundation::EventRegistrationToken) -> Result<()> { unsafe {
        <Self as RtActivatable<IUserDeviceAssociationStatics>>::get_activation_factory().remove_user_device_association_changed(token)
    }}
}
DEFINE_CLSID!(UserDeviceAssociation(&[87,105,110,100,111,119,115,46,83,121,115,116,101,109,46,85,115,101,114,68,101,118,105,99,101,65,115,115,111,99,105,97,116,105,111,110,0]) [CLSID_UserDeviceAssociation]);
DEFINE_IID!(IID_IUserDeviceAssociationChangedEventArgs, 3172953964, 47965, 19835, 165, 240, 200, 205, 17, 163, 141, 66);
RT_INTERFACE!{interface IUserDeviceAssociationChangedEventArgs(IUserDeviceAssociationChangedEventArgsVtbl): IInspectable(IInspectableVtbl) [IID_IUserDeviceAssociationChangedEventArgs] {
    fn get_DeviceId(&self, out: *mut HSTRING) -> HRESULT,
    fn get_NewUser(&self, out: *mut *mut User) -> HRESULT,
    fn get_OldUser(&self, out: *mut *mut User) -> HRESULT
}}
impl IUserDeviceAssociationChangedEventArgs {
    #[inline] pub unsafe fn get_device_id(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_DeviceId)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_new_user(&self) -> Result<ComPtr<User>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_NewUser)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_old_user(&self) -> Result<ComPtr<User>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_OldUser)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
RT_CLASS!{class UserDeviceAssociationChangedEventArgs: IUserDeviceAssociationChangedEventArgs}
DEFINE_IID!(IID_IUserDeviceAssociationStatics, 2118721044, 63578, 19463, 141, 169, 127, 227, 208, 84, 35, 67);
RT_INTERFACE!{static interface IUserDeviceAssociationStatics(IUserDeviceAssociationStaticsVtbl): IInspectable(IInspectableVtbl) [IID_IUserDeviceAssociationStatics] {
    fn FindUserFromDeviceId(&self, deviceId: HSTRING, out: *mut *mut User) -> HRESULT,
    fn add_UserDeviceAssociationChanged(&self, handler: *mut super::foundation::EventHandler<UserDeviceAssociationChangedEventArgs>, out: *mut super::foundation::EventRegistrationToken) -> HRESULT,
    fn remove_UserDeviceAssociationChanged(&self, token: super::foundation::EventRegistrationToken) -> HRESULT
}}
impl IUserDeviceAssociationStatics {
    #[inline] pub unsafe fn find_user_from_device_id(&self, deviceId: &HStringArg) -> Result<ComPtr<User>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).FindUserFromDeviceId)(self as *const _ as *mut _, deviceId.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn add_user_device_association_changed(&self, handler: &super::foundation::EventHandler<UserDeviceAssociationChangedEventArgs>) -> Result<super::foundation::EventRegistrationToken> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).add_UserDeviceAssociationChanged)(self as *const _ as *mut _, handler as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn remove_user_device_association_changed(&self, token: super::foundation::EventRegistrationToken) -> Result<()> {
        let hr = ((*self.lpVtbl).remove_UserDeviceAssociationChanged)(self as *const _ as *mut _, token);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
}
DEFINE_IID!(IID_IUserPicker, 2102689800, 61923, 19052, 141, 220, 169, 187, 15, 72, 138, 237);
RT_INTERFACE!{interface IUserPicker(IUserPickerVtbl): IInspectable(IInspectableVtbl) [IID_IUserPicker] {
    fn get_AllowGuestAccounts(&self, out: *mut bool) -> HRESULT,
    fn put_AllowGuestAccounts(&self, value: bool) -> HRESULT,
    fn get_SuggestedSelectedUser(&self, out: *mut *mut User) -> HRESULT,
    fn put_SuggestedSelectedUser(&self, value: *mut User) -> HRESULT,
    fn PickSingleUserAsync(&self, out: *mut *mut super::foundation::IAsyncOperation<User>) -> HRESULT
}}
impl IUserPicker {
    #[inline] pub unsafe fn get_allow_guest_accounts(&self) -> Result<bool> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_AllowGuestAccounts)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_allow_guest_accounts(&self, value: bool) -> Result<()> {
        let hr = ((*self.lpVtbl).put_AllowGuestAccounts)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_suggested_selected_user(&self) -> Result<ComPtr<User>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_SuggestedSelectedUser)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_suggested_selected_user(&self, value: &User) -> Result<()> {
        let hr = ((*self.lpVtbl).put_SuggestedSelectedUser)(self as *const _ as *mut _, value as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn pick_single_user_async(&self) -> Result<ComPtr<super::foundation::IAsyncOperation<User>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).PickSingleUserAsync)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
RT_CLASS!{class UserPicker: IUserPicker}
impl RtActivatable<IUserPickerStatics> for UserPicker {}
impl RtActivatable<IActivationFactory> for UserPicker {}
impl UserPicker {
    #[inline] pub fn is_supported() -> Result<bool> { unsafe {
        <Self as RtActivatable<IUserPickerStatics>>::get_activation_factory().is_supported()
    }}
}
DEFINE_CLSID!(UserPicker(&[87,105,110,100,111,119,115,46,83,121,115,116,101,109,46,85,115,101,114,80,105,99,107,101,114,0]) [CLSID_UserPicker]);
DEFINE_IID!(IID_IUserPickerStatics, 3727855836, 32371, 19958, 161, 174, 77, 126, 202, 130, 180, 13);
RT_INTERFACE!{static interface IUserPickerStatics(IUserPickerStaticsVtbl): IInspectable(IInspectableVtbl) [IID_IUserPickerStatics] {
    fn IsSupported(&self, out: *mut bool) -> HRESULT
}}
impl IUserPickerStatics {
    #[inline] pub unsafe fn is_supported(&self) -> Result<bool> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).IsSupported)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
}
RT_ENUM! { enum UserPictureSize: i32 {
    Size64x64 (UserPictureSize_Size64x64) = 0, Size208x208 (UserPictureSize_Size208x208) = 1, Size424x424 (UserPictureSize_Size424x424) = 2, Size1080x1080 (UserPictureSize_Size1080x1080) = 3,
}}
DEFINE_IID!(IID_IUserStatics, 358527547, 9258, 17888, 162, 233, 49, 113, 252, 106, 127, 221);
RT_INTERFACE!{static interface IUserStatics(IUserStaticsVtbl): IInspectable(IInspectableVtbl) [IID_IUserStatics] {
    fn CreateWatcher(&self, out: *mut *mut UserWatcher) -> HRESULT,
    fn FindAllAsync(&self, out: *mut *mut super::foundation::IAsyncOperation<super::foundation::collections::IVectorView<User>>) -> HRESULT,
    fn FindAllAsyncByType(&self, type_: UserType, out: *mut *mut super::foundation::IAsyncOperation<super::foundation::collections::IVectorView<User>>) -> HRESULT,
    fn FindAllAsyncByTypeAndStatus(&self, type_: UserType, status: UserAuthenticationStatus, out: *mut *mut super::foundation::IAsyncOperation<super::foundation::collections::IVectorView<User>>) -> HRESULT,
    fn GetFromId(&self, nonRoamableId: HSTRING, out: *mut *mut User) -> HRESULT
}}
impl IUserStatics {
    #[inline] pub unsafe fn create_watcher(&self) -> Result<ComPtr<UserWatcher>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).CreateWatcher)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn find_all_async(&self) -> Result<ComPtr<super::foundation::IAsyncOperation<super::foundation::collections::IVectorView<User>>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).FindAllAsync)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn find_all_async_by_type(&self, type_: UserType) -> Result<ComPtr<super::foundation::IAsyncOperation<super::foundation::collections::IVectorView<User>>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).FindAllAsyncByType)(self as *const _ as *mut _, type_, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn find_all_async_by_type_and_status(&self, type_: UserType, status: UserAuthenticationStatus) -> Result<ComPtr<super::foundation::IAsyncOperation<super::foundation::collections::IVectorView<User>>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).FindAllAsyncByTypeAndStatus)(self as *const _ as *mut _, type_, status, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_from_id(&self, nonRoamableId: &HStringArg) -> Result<ComPtr<User>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetFromId)(self as *const _ as *mut _, nonRoamableId.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
RT_ENUM! { enum UserType: i32 {
    LocalUser (UserType_LocalUser) = 0, RemoteUser (UserType_RemoteUser) = 1, LocalGuest (UserType_LocalGuest) = 2, RemoteGuest (UserType_RemoteGuest) = 3,
}}
DEFINE_IID!(IID_IUserWatcher, 358527547, 9258, 17888, 162, 233, 49, 113, 252, 106, 127, 187);
RT_INTERFACE!{interface IUserWatcher(IUserWatcherVtbl): IInspectable(IInspectableVtbl) [IID_IUserWatcher] {
    fn get_Status(&self, out: *mut UserWatcherStatus) -> HRESULT,
    fn Start(&self) -> HRESULT,
    fn Stop(&self) -> HRESULT,
    fn add_Added(&self, handler: *mut super::foundation::TypedEventHandler<UserWatcher, UserChangedEventArgs>, out: *mut super::foundation::EventRegistrationToken) -> HRESULT,
    fn remove_Added(&self, token: super::foundation::EventRegistrationToken) -> HRESULT,
    fn add_Removed(&self, handler: *mut super::foundation::TypedEventHandler<UserWatcher, UserChangedEventArgs>, out: *mut super::foundation::EventRegistrationToken) -> HRESULT,
    fn remove_Removed(&self, token: super::foundation::EventRegistrationToken) -> HRESULT,
    fn add_Updated(&self, handler: *mut super::foundation::TypedEventHandler<UserWatcher, UserChangedEventArgs>, out: *mut super::foundation::EventRegistrationToken) -> HRESULT,
    fn remove_Updated(&self, token: super::foundation::EventRegistrationToken) -> HRESULT,
    fn add_AuthenticationStatusChanged(&self, handler: *mut super::foundation::TypedEventHandler<UserWatcher, UserChangedEventArgs>, out: *mut super::foundation::EventRegistrationToken) -> HRESULT,
    fn remove_AuthenticationStatusChanged(&self, token: super::foundation::EventRegistrationToken) -> HRESULT,
    fn add_AuthenticationStatusChanging(&self, handler: *mut super::foundation::TypedEventHandler<UserWatcher, UserAuthenticationStatusChangingEventArgs>, out: *mut super::foundation::EventRegistrationToken) -> HRESULT,
    fn remove_AuthenticationStatusChanging(&self, token: super::foundation::EventRegistrationToken) -> HRESULT,
    fn add_EnumerationCompleted(&self, handler: *mut super::foundation::TypedEventHandler<UserWatcher, IInspectable>, out: *mut super::foundation::EventRegistrationToken) -> HRESULT,
    fn remove_EnumerationCompleted(&self, token: super::foundation::EventRegistrationToken) -> HRESULT,
    fn add_Stopped(&self, handler: *mut super::foundation::TypedEventHandler<UserWatcher, IInspectable>, out: *mut super::foundation::EventRegistrationToken) -> HRESULT,
    fn remove_Stopped(&self, token: super::foundation::EventRegistrationToken) -> HRESULT
}}
impl IUserWatcher {
    #[inline] pub unsafe fn get_status(&self) -> Result<UserWatcherStatus> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_Status)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn start(&self) -> Result<()> {
        let hr = ((*self.lpVtbl).Start)(self as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn stop(&self) -> Result<()> {
        let hr = ((*self.lpVtbl).Stop)(self as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn add_added(&self, handler: &super::foundation::TypedEventHandler<UserWatcher, UserChangedEventArgs>) -> Result<super::foundation::EventRegistrationToken> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).add_Added)(self as *const _ as *mut _, handler as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn remove_added(&self, token: super::foundation::EventRegistrationToken) -> Result<()> {
        let hr = ((*self.lpVtbl).remove_Added)(self as *const _ as *mut _, token);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn add_removed(&self, handler: &super::foundation::TypedEventHandler<UserWatcher, UserChangedEventArgs>) -> Result<super::foundation::EventRegistrationToken> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).add_Removed)(self as *const _ as *mut _, handler as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn remove_removed(&self, token: super::foundation::EventRegistrationToken) -> Result<()> {
        let hr = ((*self.lpVtbl).remove_Removed)(self as *const _ as *mut _, token);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn add_updated(&self, handler: &super::foundation::TypedEventHandler<UserWatcher, UserChangedEventArgs>) -> Result<super::foundation::EventRegistrationToken> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).add_Updated)(self as *const _ as *mut _, handler as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn remove_updated(&self, token: super::foundation::EventRegistrationToken) -> Result<()> {
        let hr = ((*self.lpVtbl).remove_Updated)(self as *const _ as *mut _, token);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn add_authentication_status_changed(&self, handler: &super::foundation::TypedEventHandler<UserWatcher, UserChangedEventArgs>) -> Result<super::foundation::EventRegistrationToken> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).add_AuthenticationStatusChanged)(self as *const _ as *mut _, handler as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn remove_authentication_status_changed(&self, token: super::foundation::EventRegistrationToken) -> Result<()> {
        let hr = ((*self.lpVtbl).remove_AuthenticationStatusChanged)(self as *const _ as *mut _, token);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn add_authentication_status_changing(&self, handler: &super::foundation::TypedEventHandler<UserWatcher, UserAuthenticationStatusChangingEventArgs>) -> Result<super::foundation::EventRegistrationToken> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).add_AuthenticationStatusChanging)(self as *const _ as *mut _, handler as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn remove_authentication_status_changing(&self, token: super::foundation::EventRegistrationToken) -> Result<()> {
        let hr = ((*self.lpVtbl).remove_AuthenticationStatusChanging)(self as *const _ as *mut _, token);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn add_enumeration_completed(&self, handler: &super::foundation::TypedEventHandler<UserWatcher, IInspectable>) -> Result<super::foundation::EventRegistrationToken> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).add_EnumerationCompleted)(self as *const _ as *mut _, handler as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn remove_enumeration_completed(&self, token: super::foundation::EventRegistrationToken) -> Result<()> {
        let hr = ((*self.lpVtbl).remove_EnumerationCompleted)(self as *const _ as *mut _, token);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn add_stopped(&self, handler: &super::foundation::TypedEventHandler<UserWatcher, IInspectable>) -> Result<super::foundation::EventRegistrationToken> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).add_Stopped)(self as *const _ as *mut _, handler as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn remove_stopped(&self, token: super::foundation::EventRegistrationToken) -> Result<()> {
        let hr = ((*self.lpVtbl).remove_Stopped)(self as *const _ as *mut _, token);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
}
RT_CLASS!{class UserWatcher: IUserWatcher}
RT_ENUM! { enum UserWatcherStatus: i32 {
    Created (UserWatcherStatus_Created) = 0, Started (UserWatcherStatus_Started) = 1, EnumerationCompleted (UserWatcherStatus_EnumerationCompleted) = 2, Stopping (UserWatcherStatus_Stopping) = 3, Stopped (UserWatcherStatus_Stopped) = 4, Aborted (UserWatcherStatus_Aborted) = 5,
}}
RT_ENUM! { enum VirtualKey: i32 {
    None (VirtualKey_None) = 0, LeftButton (VirtualKey_LeftButton) = 1, RightButton (VirtualKey_RightButton) = 2, Cancel (VirtualKey_Cancel) = 3, MiddleButton (VirtualKey_MiddleButton) = 4, XButton1 (VirtualKey_XButton1) = 5, XButton2 (VirtualKey_XButton2) = 6, Back (VirtualKey_Back) = 8, Tab (VirtualKey_Tab) = 9, Clear (VirtualKey_Clear) = 12, Enter (VirtualKey_Enter) = 13, Shift (VirtualKey_Shift) = 16, Control (VirtualKey_Control) = 17, Menu (VirtualKey_Menu) = 18, Pause (VirtualKey_Pause) = 19, CapitalLock (VirtualKey_CapitalLock) = 20, Kana (VirtualKey_Kana) = 21, Hangul (VirtualKey_Hangul) = 21, Junja (VirtualKey_Junja) = 23, Final (VirtualKey_Final) = 24, Hanja (VirtualKey_Hanja) = 25, Kanji (VirtualKey_Kanji) = 25, Escape (VirtualKey_Escape) = 27, Convert (VirtualKey_Convert) = 28, NonConvert (VirtualKey_NonConvert) = 29, Accept (VirtualKey_Accept) = 30, ModeChange (VirtualKey_ModeChange) = 31, Space (VirtualKey_Space) = 32, PageUp (VirtualKey_PageUp) = 33, PageDown (VirtualKey_PageDown) = 34, End (VirtualKey_End) = 35, Home (VirtualKey_Home) = 36, Left (VirtualKey_Left) = 37, Up (VirtualKey_Up) = 38, Right (VirtualKey_Right) = 39, Down (VirtualKey_Down) = 40, Select (VirtualKey_Select) = 41, Print (VirtualKey_Print) = 42, Execute (VirtualKey_Execute) = 43, Snapshot (VirtualKey_Snapshot) = 44, Insert (VirtualKey_Insert) = 45, Delete (VirtualKey_Delete) = 46, Help (VirtualKey_Help) = 47, Number0 (VirtualKey_Number0) = 48, Number1 (VirtualKey_Number1) = 49, Number2 (VirtualKey_Number2) = 50, Number3 (VirtualKey_Number3) = 51, Number4 (VirtualKey_Number4) = 52, Number5 (VirtualKey_Number5) = 53, Number6 (VirtualKey_Number6) = 54, Number7 (VirtualKey_Number7) = 55, Number8 (VirtualKey_Number8) = 56, Number9 (VirtualKey_Number9) = 57, A (VirtualKey_A) = 65, B (VirtualKey_B) = 66, C (VirtualKey_C) = 67, D (VirtualKey_D) = 68, E (VirtualKey_E) = 69, F (VirtualKey_F) = 70, G (VirtualKey_G) = 71, H (VirtualKey_H) = 72, I (VirtualKey_I) = 73, J (VirtualKey_J) = 74, K (VirtualKey_K) = 75, L (VirtualKey_L) = 76, M (VirtualKey_M) = 77, N (VirtualKey_N) = 78, O (VirtualKey_O) = 79, P (VirtualKey_P) = 80, Q (VirtualKey_Q) = 81, R (VirtualKey_R) = 82, S (VirtualKey_S) = 83, T (VirtualKey_T) = 84, U (VirtualKey_U) = 85, V (VirtualKey_V) = 86, W (VirtualKey_W) = 87, X (VirtualKey_X) = 88, Y (VirtualKey_Y) = 89, Z (VirtualKey_Z) = 90, LeftWindows (VirtualKey_LeftWindows) = 91, RightWindows (VirtualKey_RightWindows) = 92, Application (VirtualKey_Application) = 93, Sleep (VirtualKey_Sleep) = 95, NumberPad0 (VirtualKey_NumberPad0) = 96, NumberPad1 (VirtualKey_NumberPad1) = 97, NumberPad2 (VirtualKey_NumberPad2) = 98, NumberPad3 (VirtualKey_NumberPad3) = 99, NumberPad4 (VirtualKey_NumberPad4) = 100, NumberPad5 (VirtualKey_NumberPad5) = 101, NumberPad6 (VirtualKey_NumberPad6) = 102, NumberPad7 (VirtualKey_NumberPad7) = 103, NumberPad8 (VirtualKey_NumberPad8) = 104, NumberPad9 (VirtualKey_NumberPad9) = 105, Multiply (VirtualKey_Multiply) = 106, Add (VirtualKey_Add) = 107, Separator (VirtualKey_Separator) = 108, Subtract (VirtualKey_Subtract) = 109, Decimal (VirtualKey_Decimal) = 110, Divide (VirtualKey_Divide) = 111, F1 (VirtualKey_F1) = 112, F2 (VirtualKey_F2) = 113, F3 (VirtualKey_F3) = 114, F4 (VirtualKey_F4) = 115, F5 (VirtualKey_F5) = 116, F6 (VirtualKey_F6) = 117, F7 (VirtualKey_F7) = 118, F8 (VirtualKey_F8) = 119, F9 (VirtualKey_F9) = 120, F10 (VirtualKey_F10) = 121, F11 (VirtualKey_F11) = 122, F12 (VirtualKey_F12) = 123, F13 (VirtualKey_F13) = 124, F14 (VirtualKey_F14) = 125, F15 (VirtualKey_F15) = 126, F16 (VirtualKey_F16) = 127, F17 (VirtualKey_F17) = 128, F18 (VirtualKey_F18) = 129, F19 (VirtualKey_F19) = 130, F20 (VirtualKey_F20) = 131, F21 (VirtualKey_F21) = 132, F22 (VirtualKey_F22) = 133, F23 (VirtualKey_F23) = 134, F24 (VirtualKey_F24) = 135, NavigationView (VirtualKey_NavigationView) = 136, NavigationMenu (VirtualKey_NavigationMenu) = 137, NavigationUp (VirtualKey_NavigationUp) = 138, NavigationDown (VirtualKey_NavigationDown) = 139, NavigationLeft (VirtualKey_NavigationLeft) = 140, NavigationRight (VirtualKey_NavigationRight) = 141, NavigationAccept (VirtualKey_NavigationAccept) = 142, NavigationCancel (VirtualKey_NavigationCancel) = 143, NumberKeyLock (VirtualKey_NumberKeyLock) = 144, Scroll (VirtualKey_Scroll) = 145, LeftShift (VirtualKey_LeftShift) = 160, RightShift (VirtualKey_RightShift) = 161, LeftControl (VirtualKey_LeftControl) = 162, RightControl (VirtualKey_RightControl) = 163, LeftMenu (VirtualKey_LeftMenu) = 164, RightMenu (VirtualKey_RightMenu) = 165, GoBack (VirtualKey_GoBack) = 166, GoForward (VirtualKey_GoForward) = 167, Refresh (VirtualKey_Refresh) = 168, Stop (VirtualKey_Stop) = 169, Search (VirtualKey_Search) = 170, Favorites (VirtualKey_Favorites) = 171, GoHome (VirtualKey_GoHome) = 172, GamepadA (VirtualKey_GamepadA) = 195, GamepadB (VirtualKey_GamepadB) = 196, GamepadX (VirtualKey_GamepadX) = 197, GamepadY (VirtualKey_GamepadY) = 198, GamepadRightShoulder (VirtualKey_GamepadRightShoulder) = 199, GamepadLeftShoulder (VirtualKey_GamepadLeftShoulder) = 200, GamepadLeftTrigger (VirtualKey_GamepadLeftTrigger) = 201, GamepadRightTrigger (VirtualKey_GamepadRightTrigger) = 202, GamepadDPadUp (VirtualKey_GamepadDPadUp) = 203, GamepadDPadDown (VirtualKey_GamepadDPadDown) = 204, GamepadDPadLeft (VirtualKey_GamepadDPadLeft) = 205, GamepadDPadRight (VirtualKey_GamepadDPadRight) = 206, GamepadMenu (VirtualKey_GamepadMenu) = 207, GamepadView (VirtualKey_GamepadView) = 208, GamepadLeftThumbstickButton (VirtualKey_GamepadLeftThumbstickButton) = 209, GamepadRightThumbstickButton (VirtualKey_GamepadRightThumbstickButton) = 210, GamepadLeftThumbstickUp (VirtualKey_GamepadLeftThumbstickUp) = 211, GamepadLeftThumbstickDown (VirtualKey_GamepadLeftThumbstickDown) = 212, GamepadLeftThumbstickRight (VirtualKey_GamepadLeftThumbstickRight) = 213, GamepadLeftThumbstickLeft (VirtualKey_GamepadLeftThumbstickLeft) = 214, GamepadRightThumbstickUp (VirtualKey_GamepadRightThumbstickUp) = 215, GamepadRightThumbstickDown (VirtualKey_GamepadRightThumbstickDown) = 216, GamepadRightThumbstickRight (VirtualKey_GamepadRightThumbstickRight) = 217, GamepadRightThumbstickLeft (VirtualKey_GamepadRightThumbstickLeft) = 218,
}}
RT_ENUM! { enum VirtualKeyModifiers: u32 {
    None (VirtualKeyModifiers_None) = 0, Control (VirtualKeyModifiers_Control) = 1, Menu (VirtualKeyModifiers_Menu) = 2, Shift (VirtualKeyModifiers_Shift) = 4, Windows (VirtualKeyModifiers_Windows) = 8,
}}
pub mod userprofile { // Windows.System.UserProfile
use ::prelude::*;
RT_ENUM! { enum AccountPictureKind: i32 {
    SmallImage (AccountPictureKind_SmallImage) = 0, LargeImage (AccountPictureKind_LargeImage) = 1, Video (AccountPictureKind_Video) = 2,
}}
RT_CLASS!{static class AdvertisingManager}
impl RtActivatable<IAdvertisingManagerStatics> for AdvertisingManager {}
impl RtActivatable<IAdvertisingManagerStatics2> for AdvertisingManager {}
impl AdvertisingManager {
    #[inline] pub fn get_advertising_id() -> Result<HString> { unsafe {
        <Self as RtActivatable<IAdvertisingManagerStatics>>::get_activation_factory().get_advertising_id()
    }}
    #[inline] pub fn get_for_user(user: &super::User) -> Result<ComPtr<AdvertisingManagerForUser>> { unsafe {
        <Self as RtActivatable<IAdvertisingManagerStatics2>>::get_activation_factory().get_for_user(user)
    }}
}
DEFINE_CLSID!(AdvertisingManager(&[87,105,110,100,111,119,115,46,83,121,115,116,101,109,46,85,115,101,114,80,114,111,102,105,108,101,46,65,100,118,101,114,116,105,115,105,110,103,77,97,110,97,103,101,114,0]) [CLSID_AdvertisingManager]);
DEFINE_IID!(IID_IAdvertisingManagerForUser, 2458645456, 53116, 19120, 167, 220, 109, 197, 188, 212, 66, 82);
RT_INTERFACE!{interface IAdvertisingManagerForUser(IAdvertisingManagerForUserVtbl): IInspectable(IInspectableVtbl) [IID_IAdvertisingManagerForUser] {
    fn get_AdvertisingId(&self, out: *mut HSTRING) -> HRESULT,
    fn get_User(&self, out: *mut *mut super::User) -> HRESULT
}}
impl IAdvertisingManagerForUser {
    #[inline] pub unsafe fn get_advertising_id(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_AdvertisingId)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_user(&self) -> Result<ComPtr<super::User>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_User)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
RT_CLASS!{class AdvertisingManagerForUser: IAdvertisingManagerForUser}
DEFINE_IID!(IID_IAdvertisingManagerStatics, 2916304524, 41587, 18635, 179, 70, 53, 68, 82, 45, 85, 129);
RT_INTERFACE!{static interface IAdvertisingManagerStatics(IAdvertisingManagerStaticsVtbl): IInspectable(IInspectableVtbl) [IID_IAdvertisingManagerStatics] {
    fn get_AdvertisingId(&self, out: *mut HSTRING) -> HRESULT
}}
impl IAdvertisingManagerStatics {
    #[inline] pub unsafe fn get_advertising_id(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_AdvertisingId)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
}
DEFINE_IID!(IID_IAdvertisingManagerStatics2, 3708372911, 6765, 18096, 149, 188, 243, 249, 214, 190, 185, 251);
RT_INTERFACE!{static interface IAdvertisingManagerStatics2(IAdvertisingManagerStatics2Vtbl): IInspectable(IInspectableVtbl) [IID_IAdvertisingManagerStatics2] {
    fn GetForUser(&self, user: *mut super::User, out: *mut *mut AdvertisingManagerForUser) -> HRESULT
}}
impl IAdvertisingManagerStatics2 {
    #[inline] pub unsafe fn get_for_user(&self, user: &super::User) -> Result<ComPtr<AdvertisingManagerForUser>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetForUser)(self as *const _ as *mut _, user as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
DEFINE_IID!(IID_IDiagnosticsSettings, 3857312973, 10001, 17632, 151, 60, 73, 29, 120, 4, 141, 36);
RT_INTERFACE!{interface IDiagnosticsSettings(IDiagnosticsSettingsVtbl): IInspectable(IInspectableVtbl) [IID_IDiagnosticsSettings] {
    fn get_CanUseDiagnosticsToTailorExperiences(&self, out: *mut bool) -> HRESULT,
    fn get_User(&self, out: *mut *mut super::User) -> HRESULT
}}
impl IDiagnosticsSettings {
    #[inline] pub unsafe fn get_can_use_diagnostics_to_tailor_experiences(&self) -> Result<bool> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_CanUseDiagnosticsToTailorExperiences)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_user(&self) -> Result<ComPtr<super::User>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_User)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
RT_CLASS!{class DiagnosticsSettings: IDiagnosticsSettings}
impl RtActivatable<IDiagnosticsSettingsStatics> for DiagnosticsSettings {}
impl DiagnosticsSettings {
    #[inline] pub fn get_default() -> Result<ComPtr<DiagnosticsSettings>> { unsafe {
        <Self as RtActivatable<IDiagnosticsSettingsStatics>>::get_activation_factory().get_default()
    }}
    #[inline] pub fn get_for_user(user: &super::User) -> Result<ComPtr<DiagnosticsSettings>> { unsafe {
        <Self as RtActivatable<IDiagnosticsSettingsStatics>>::get_activation_factory().get_for_user(user)
    }}
}
DEFINE_CLSID!(DiagnosticsSettings(&[87,105,110,100,111,119,115,46,83,121,115,116,101,109,46,85,115,101,114,80,114,111,102,105,108,101,46,68,105,97,103,110,111,115,116,105,99,115,83,101,116,116,105,110,103,115,0]) [CLSID_DiagnosticsSettings]);
DEFINE_IID!(IID_IDiagnosticsSettingsStatics, 1926424591, 21392, 18323, 153, 11, 60, 204, 125, 106, 201, 200);
RT_INTERFACE!{static interface IDiagnosticsSettingsStatics(IDiagnosticsSettingsStaticsVtbl): IInspectable(IInspectableVtbl) [IID_IDiagnosticsSettingsStatics] {
    fn GetDefault(&self, out: *mut *mut DiagnosticsSettings) -> HRESULT,
    fn GetForUser(&self, user: *mut super::User, out: *mut *mut DiagnosticsSettings) -> HRESULT
}}
impl IDiagnosticsSettingsStatics {
    #[inline] pub unsafe fn get_default(&self) -> Result<ComPtr<DiagnosticsSettings>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetDefault)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_for_user(&self, user: &super::User) -> Result<ComPtr<DiagnosticsSettings>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetForUser)(self as *const _ as *mut _, user as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
DEFINE_IID!(IID_IFirstSignInSettings, 1049907539, 14942, 17710, 166, 1, 245, 186, 173, 42, 72, 112);
RT_INTERFACE!{interface IFirstSignInSettings(IFirstSignInSettingsVtbl): IInspectable(IInspectableVtbl) [IID_IFirstSignInSettings] {
    
}}
RT_CLASS!{class FirstSignInSettings: IFirstSignInSettings}
impl RtActivatable<IFirstSignInSettingsStatics> for FirstSignInSettings {}
impl FirstSignInSettings {
    #[inline] pub fn get_default() -> Result<ComPtr<FirstSignInSettings>> { unsafe {
        <Self as RtActivatable<IFirstSignInSettingsStatics>>::get_activation_factory().get_default()
    }}
}
DEFINE_CLSID!(FirstSignInSettings(&[87,105,110,100,111,119,115,46,83,121,115,116,101,109,46,85,115,101,114,80,114,111,102,105,108,101,46,70,105,114,115,116,83,105,103,110,73,110,83,101,116,116,105,110,103,115,0]) [CLSID_FirstSignInSettings]);
DEFINE_IID!(IID_IFirstSignInSettingsStatics, 484544271, 7233, 20128, 183, 162, 111, 12, 28, 126, 132, 56);
RT_INTERFACE!{static interface IFirstSignInSettingsStatics(IFirstSignInSettingsStaticsVtbl): IInspectable(IInspectableVtbl) [IID_IFirstSignInSettingsStatics] {
    fn GetDefault(&self, out: *mut *mut FirstSignInSettings) -> HRESULT
}}
impl IFirstSignInSettingsStatics {
    #[inline] pub unsafe fn get_default(&self) -> Result<ComPtr<FirstSignInSettings>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetDefault)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
RT_CLASS!{static class GlobalizationPreferences}
impl RtActivatable<IGlobalizationPreferencesStatics> for GlobalizationPreferences {}
impl RtActivatable<IGlobalizationPreferencesStatics2> for GlobalizationPreferences {}
impl GlobalizationPreferences {
    #[inline] pub fn get_calendars() -> Result<ComPtr<super::super::foundation::collections::IVectorView<HString>>> { unsafe {
        <Self as RtActivatable<IGlobalizationPreferencesStatics>>::get_activation_factory().get_calendars()
    }}
    #[inline] pub fn get_clocks() -> Result<ComPtr<super::super::foundation::collections::IVectorView<HString>>> { unsafe {
        <Self as RtActivatable<IGlobalizationPreferencesStatics>>::get_activation_factory().get_clocks()
    }}
    #[inline] pub fn get_currencies() -> Result<ComPtr<super::super::foundation::collections::IVectorView<HString>>> { unsafe {
        <Self as RtActivatable<IGlobalizationPreferencesStatics>>::get_activation_factory().get_currencies()
    }}
    #[inline] pub fn get_languages() -> Result<ComPtr<super::super::foundation::collections::IVectorView<HString>>> { unsafe {
        <Self as RtActivatable<IGlobalizationPreferencesStatics>>::get_activation_factory().get_languages()
    }}
    #[inline] pub fn get_home_geographic_region() -> Result<HString> { unsafe {
        <Self as RtActivatable<IGlobalizationPreferencesStatics>>::get_activation_factory().get_home_geographic_region()
    }}
    #[cfg(feature="windows-globalization")] #[inline] pub fn get_week_starts_on() -> Result<super::super::globalization::DayOfWeek> { unsafe {
        <Self as RtActivatable<IGlobalizationPreferencesStatics>>::get_activation_factory().get_week_starts_on()
    }}
    #[inline] pub fn try_set_home_geographic_region(region: &HStringArg) -> Result<bool> { unsafe {
        <Self as RtActivatable<IGlobalizationPreferencesStatics2>>::get_activation_factory().try_set_home_geographic_region(region)
    }}
    #[inline] pub fn try_set_languages(languageTags: &super::super::foundation::collections::IIterable<HString>) -> Result<bool> { unsafe {
        <Self as RtActivatable<IGlobalizationPreferencesStatics2>>::get_activation_factory().try_set_languages(languageTags)
    }}
}
DEFINE_CLSID!(GlobalizationPreferences(&[87,105,110,100,111,119,115,46,83,121,115,116,101,109,46,85,115,101,114,80,114,111,102,105,108,101,46,71,108,111,98,97,108,105,122,97,116,105,111,110,80,114,101,102,101,114,101,110,99,101,115,0]) [CLSID_GlobalizationPreferences]);
DEFINE_IID!(IID_IGlobalizationPreferencesStatics, 29311782, 60727, 20118, 176, 233, 193, 52, 13, 30, 161, 88);
RT_INTERFACE!{static interface IGlobalizationPreferencesStatics(IGlobalizationPreferencesStaticsVtbl): IInspectable(IInspectableVtbl) [IID_IGlobalizationPreferencesStatics] {
    fn get_Calendars(&self, out: *mut *mut super::super::foundation::collections::IVectorView<HString>) -> HRESULT,
    fn get_Clocks(&self, out: *mut *mut super::super::foundation::collections::IVectorView<HString>) -> HRESULT,
    fn get_Currencies(&self, out: *mut *mut super::super::foundation::collections::IVectorView<HString>) -> HRESULT,
    fn get_Languages(&self, out: *mut *mut super::super::foundation::collections::IVectorView<HString>) -> HRESULT,
    fn get_HomeGeographicRegion(&self, out: *mut HSTRING) -> HRESULT,
    #[cfg(feature="windows-globalization")] fn get_WeekStartsOn(&self, out: *mut super::super::globalization::DayOfWeek) -> HRESULT
}}
impl IGlobalizationPreferencesStatics {
    #[inline] pub unsafe fn get_calendars(&self) -> Result<ComPtr<super::super::foundation::collections::IVectorView<HString>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Calendars)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_clocks(&self) -> Result<ComPtr<super::super::foundation::collections::IVectorView<HString>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Clocks)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_currencies(&self) -> Result<ComPtr<super::super::foundation::collections::IVectorView<HString>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Currencies)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_languages(&self) -> Result<ComPtr<super::super::foundation::collections::IVectorView<HString>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Languages)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_home_geographic_region(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_HomeGeographicRegion)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[cfg(feature="windows-globalization")] #[inline] pub unsafe fn get_week_starts_on(&self) -> Result<super::super::globalization::DayOfWeek> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_WeekStartsOn)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
}
DEFINE_IID!(IID_IGlobalizationPreferencesStatics2, 4241393137, 17152, 19664, 156, 172, 26, 142, 123, 126, 24, 244);
RT_INTERFACE!{static interface IGlobalizationPreferencesStatics2(IGlobalizationPreferencesStatics2Vtbl): IInspectable(IInspectableVtbl) [IID_IGlobalizationPreferencesStatics2] {
    fn TrySetHomeGeographicRegion(&self, region: HSTRING, out: *mut bool) -> HRESULT,
    fn TrySetLanguages(&self, languageTags: *mut super::super::foundation::collections::IIterable<HString>, out: *mut bool) -> HRESULT
}}
impl IGlobalizationPreferencesStatics2 {
    #[inline] pub unsafe fn try_set_home_geographic_region(&self, region: &HStringArg) -> Result<bool> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).TrySetHomeGeographicRegion)(self as *const _ as *mut _, region.get(), &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn try_set_languages(&self, languageTags: &super::super::foundation::collections::IIterable<HString>) -> Result<bool> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).TrySetLanguages)(self as *const _ as *mut _, languageTags as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
}
RT_CLASS!{static class LockScreen}
impl RtActivatable<ILockScreenImageFeedStatics> for LockScreen {}
impl RtActivatable<ILockScreenStatics> for LockScreen {}
impl LockScreen {
    #[inline] pub fn request_set_image_feed_async(syndicationFeedUri: &super::super::foundation::Uri) -> Result<ComPtr<super::super::foundation::IAsyncOperation<SetImageFeedResult>>> { unsafe {
        <Self as RtActivatable<ILockScreenImageFeedStatics>>::get_activation_factory().request_set_image_feed_async(syndicationFeedUri)
    }}
    #[inline] pub fn try_remove_image_feed() -> Result<bool> { unsafe {
        <Self as RtActivatable<ILockScreenImageFeedStatics>>::get_activation_factory().try_remove_image_feed()
    }}
    #[inline] pub fn get_original_image_file() -> Result<ComPtr<super::super::foundation::Uri>> { unsafe {
        <Self as RtActivatable<ILockScreenStatics>>::get_activation_factory().get_original_image_file()
    }}
    #[cfg(feature="windows-storage")] #[inline] pub fn get_image_stream() -> Result<ComPtr<super::super::storage::streams::IRandomAccessStream>> { unsafe {
        <Self as RtActivatable<ILockScreenStatics>>::get_activation_factory().get_image_stream()
    }}
    #[cfg(feature="windows-storage")] #[inline] pub fn set_image_file_async(value: &super::super::storage::IStorageFile) -> Result<ComPtr<super::super::foundation::IAsyncAction>> { unsafe {
        <Self as RtActivatable<ILockScreenStatics>>::get_activation_factory().set_image_file_async(value)
    }}
    #[cfg(feature="windows-storage")] #[inline] pub fn set_image_stream_async(value: &super::super::storage::streams::IRandomAccessStream) -> Result<ComPtr<super::super::foundation::IAsyncAction>> { unsafe {
        <Self as RtActivatable<ILockScreenStatics>>::get_activation_factory().set_image_stream_async(value)
    }}
}
DEFINE_CLSID!(LockScreen(&[87,105,110,100,111,119,115,46,83,121,115,116,101,109,46,85,115,101,114,80,114,111,102,105,108,101,46,76,111,99,107,83,99,114,101,101,110,0]) [CLSID_LockScreen]);
DEFINE_IID!(IID_ILockScreenImageFeedStatics, 739079158, 937, 16806, 155, 1, 73, 82, 81, 255, 81, 213);
RT_INTERFACE!{static interface ILockScreenImageFeedStatics(ILockScreenImageFeedStaticsVtbl): IInspectable(IInspectableVtbl) [IID_ILockScreenImageFeedStatics] {
    fn RequestSetImageFeedAsync(&self, syndicationFeedUri: *mut super::super::foundation::Uri, out: *mut *mut super::super::foundation::IAsyncOperation<SetImageFeedResult>) -> HRESULT,
    fn TryRemoveImageFeed(&self, out: *mut bool) -> HRESULT
}}
impl ILockScreenImageFeedStatics {
    #[inline] pub unsafe fn request_set_image_feed_async(&self, syndicationFeedUri: &super::super::foundation::Uri) -> Result<ComPtr<super::super::foundation::IAsyncOperation<SetImageFeedResult>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).RequestSetImageFeedAsync)(self as *const _ as *mut _, syndicationFeedUri as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn try_remove_image_feed(&self) -> Result<bool> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).TryRemoveImageFeed)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
}
DEFINE_IID!(IID_ILockScreenStatics, 1055511469, 46599, 16558, 180, 38, 118, 49, 217, 130, 18, 105);
RT_INTERFACE!{static interface ILockScreenStatics(ILockScreenStaticsVtbl): IInspectable(IInspectableVtbl) [IID_ILockScreenStatics] {
    fn get_OriginalImageFile(&self, out: *mut *mut super::super::foundation::Uri) -> HRESULT,
    #[cfg(feature="windows-storage")] fn GetImageStream(&self, out: *mut *mut super::super::storage::streams::IRandomAccessStream) -> HRESULT,
    #[cfg(feature="windows-storage")] fn SetImageFileAsync(&self, value: *mut super::super::storage::IStorageFile, out: *mut *mut super::super::foundation::IAsyncAction) -> HRESULT,
    #[cfg(feature="windows-storage")] fn SetImageStreamAsync(&self, value: *mut super::super::storage::streams::IRandomAccessStream, out: *mut *mut super::super::foundation::IAsyncAction) -> HRESULT
}}
impl ILockScreenStatics {
    #[inline] pub unsafe fn get_original_image_file(&self) -> Result<ComPtr<super::super::foundation::Uri>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_OriginalImageFile)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[cfg(feature="windows-storage")] #[inline] pub unsafe fn get_image_stream(&self) -> Result<ComPtr<super::super::storage::streams::IRandomAccessStream>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetImageStream)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[cfg(feature="windows-storage")] #[inline] pub unsafe fn set_image_file_async(&self, value: &super::super::storage::IStorageFile) -> Result<ComPtr<super::super::foundation::IAsyncAction>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).SetImageFileAsync)(self as *const _ as *mut _, value as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[cfg(feature="windows-storage")] #[inline] pub unsafe fn set_image_stream_async(&self, value: &super::super::storage::streams::IRandomAccessStream) -> Result<ComPtr<super::super::foundation::IAsyncAction>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).SetImageStreamAsync)(self as *const _ as *mut _, value as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
RT_ENUM! { enum SetAccountPictureResult: i32 {
    Success (SetAccountPictureResult_Success) = 0, ChangeDisabled (SetAccountPictureResult_ChangeDisabled) = 1, LargeOrDynamicError (SetAccountPictureResult_LargeOrDynamicError) = 2, VideoFrameSizeError (SetAccountPictureResult_VideoFrameSizeError) = 3, FileSizeError (SetAccountPictureResult_FileSizeError) = 4, Failure (SetAccountPictureResult_Failure) = 5,
}}
RT_ENUM! { enum SetImageFeedResult: i32 {
    Success (SetImageFeedResult_Success) = 0, ChangeDisabled (SetImageFeedResult_ChangeDisabled) = 1, UserCanceled (SetImageFeedResult_UserCanceled) = 2,
}}
RT_CLASS!{static class UserInformation}
impl RtActivatable<IUserInformationStatics> for UserInformation {}
impl UserInformation {
    #[inline] pub fn get_account_picture_change_enabled() -> Result<bool> { unsafe {
        <Self as RtActivatable<IUserInformationStatics>>::get_activation_factory().get_account_picture_change_enabled()
    }}
    #[inline] pub fn get_name_access_allowed() -> Result<bool> { unsafe {
        <Self as RtActivatable<IUserInformationStatics>>::get_activation_factory().get_name_access_allowed()
    }}
    #[cfg(feature="windows-storage")] #[inline] pub fn get_account_picture(kind: AccountPictureKind) -> Result<ComPtr<super::super::storage::IStorageFile>> { unsafe {
        <Self as RtActivatable<IUserInformationStatics>>::get_activation_factory().get_account_picture(kind)
    }}
    #[cfg(feature="windows-storage")] #[inline] pub fn set_account_picture_async(image: &super::super::storage::IStorageFile) -> Result<ComPtr<super::super::foundation::IAsyncOperation<SetAccountPictureResult>>> { unsafe {
        <Self as RtActivatable<IUserInformationStatics>>::get_activation_factory().set_account_picture_async(image)
    }}
    #[cfg(feature="windows-storage")] #[inline] pub fn set_account_pictures_async(smallImage: &super::super::storage::IStorageFile, largeImage: &super::super::storage::IStorageFile, video: &super::super::storage::IStorageFile) -> Result<ComPtr<super::super::foundation::IAsyncOperation<SetAccountPictureResult>>> { unsafe {
        <Self as RtActivatable<IUserInformationStatics>>::get_activation_factory().set_account_pictures_async(smallImage, largeImage, video)
    }}
    #[cfg(feature="windows-storage")] #[inline] pub fn set_account_picture_from_stream_async(image: &super::super::storage::streams::IRandomAccessStream) -> Result<ComPtr<super::super::foundation::IAsyncOperation<SetAccountPictureResult>>> { unsafe {
        <Self as RtActivatable<IUserInformationStatics>>::get_activation_factory().set_account_picture_from_stream_async(image)
    }}
    #[cfg(feature="windows-storage")] #[inline] pub fn set_account_pictures_from_streams_async(smallImage: &super::super::storage::streams::IRandomAccessStream, largeImage: &super::super::storage::streams::IRandomAccessStream, video: &super::super::storage::streams::IRandomAccessStream) -> Result<ComPtr<super::super::foundation::IAsyncOperation<SetAccountPictureResult>>> { unsafe {
        <Self as RtActivatable<IUserInformationStatics>>::get_activation_factory().set_account_pictures_from_streams_async(smallImage, largeImage, video)
    }}
    #[inline] pub fn add_account_picture_changed(changeHandler: &super::super::foundation::EventHandler<IInspectable>) -> Result<super::super::foundation::EventRegistrationToken> { unsafe {
        <Self as RtActivatable<IUserInformationStatics>>::get_activation_factory().add_account_picture_changed(changeHandler)
    }}
    #[inline] pub fn remove_account_picture_changed(token: super::super::foundation::EventRegistrationToken) -> Result<()> { unsafe {
        <Self as RtActivatable<IUserInformationStatics>>::get_activation_factory().remove_account_picture_changed(token)
    }}
    #[inline] pub fn get_display_name_async() -> Result<ComPtr<super::super::foundation::IAsyncOperation<HString>>> { unsafe {
        <Self as RtActivatable<IUserInformationStatics>>::get_activation_factory().get_display_name_async()
    }}
    #[inline] pub fn get_first_name_async() -> Result<ComPtr<super::super::foundation::IAsyncOperation<HString>>> { unsafe {
        <Self as RtActivatable<IUserInformationStatics>>::get_activation_factory().get_first_name_async()
    }}
    #[inline] pub fn get_last_name_async() -> Result<ComPtr<super::super::foundation::IAsyncOperation<HString>>> { unsafe {
        <Self as RtActivatable<IUserInformationStatics>>::get_activation_factory().get_last_name_async()
    }}
    #[inline] pub fn get_principal_name_async() -> Result<ComPtr<super::super::foundation::IAsyncOperation<HString>>> { unsafe {
        <Self as RtActivatable<IUserInformationStatics>>::get_activation_factory().get_principal_name_async()
    }}
    #[inline] pub fn get_session_initiation_protocol_uri_async() -> Result<ComPtr<super::super::foundation::IAsyncOperation<super::super::foundation::Uri>>> { unsafe {
        <Self as RtActivatable<IUserInformationStatics>>::get_activation_factory().get_session_initiation_protocol_uri_async()
    }}
    #[inline] pub fn get_domain_name_async() -> Result<ComPtr<super::super::foundation::IAsyncOperation<HString>>> { unsafe {
        <Self as RtActivatable<IUserInformationStatics>>::get_activation_factory().get_domain_name_async()
    }}
}
DEFINE_CLSID!(UserInformation(&[87,105,110,100,111,119,115,46,83,121,115,116,101,109,46,85,115,101,114,80,114,111,102,105,108,101,46,85,115,101,114,73,110,102,111,114,109,97,116,105,111,110,0]) [CLSID_UserInformation]);
DEFINE_IID!(IID_IUserInformationStatics, 2012457232, 18682, 18588, 147, 78, 42, 232, 91, 168, 247, 114);
RT_INTERFACE!{static interface IUserInformationStatics(IUserInformationStaticsVtbl): IInspectable(IInspectableVtbl) [IID_IUserInformationStatics] {
    fn get_AccountPictureChangeEnabled(&self, out: *mut bool) -> HRESULT,
    fn get_NameAccessAllowed(&self, out: *mut bool) -> HRESULT,
    #[cfg(not(feature="windows-storage"))] fn __Dummy2(&self) -> (),
    #[cfg(feature="windows-storage")] fn GetAccountPicture(&self, kind: AccountPictureKind, out: *mut *mut super::super::storage::IStorageFile) -> HRESULT,
    #[cfg(not(feature="windows-storage"))] fn __Dummy3(&self) -> (),
    #[cfg(feature="windows-storage")] fn SetAccountPictureAsync(&self, image: *mut super::super::storage::IStorageFile, out: *mut *mut super::super::foundation::IAsyncOperation<SetAccountPictureResult>) -> HRESULT,
    #[cfg(not(feature="windows-storage"))] fn __Dummy4(&self) -> (),
    #[cfg(feature="windows-storage")] fn SetAccountPicturesAsync(&self, smallImage: *mut super::super::storage::IStorageFile, largeImage: *mut super::super::storage::IStorageFile, video: *mut super::super::storage::IStorageFile, out: *mut *mut super::super::foundation::IAsyncOperation<SetAccountPictureResult>) -> HRESULT,
    #[cfg(not(feature="windows-storage"))] fn __Dummy5(&self) -> (),
    #[cfg(feature="windows-storage")] fn SetAccountPictureFromStreamAsync(&self, image: *mut super::super::storage::streams::IRandomAccessStream, out: *mut *mut super::super::foundation::IAsyncOperation<SetAccountPictureResult>) -> HRESULT,
    #[cfg(not(feature="windows-storage"))] fn __Dummy6(&self) -> (),
    #[cfg(feature="windows-storage")] fn SetAccountPicturesFromStreamsAsync(&self, smallImage: *mut super::super::storage::streams::IRandomAccessStream, largeImage: *mut super::super::storage::streams::IRandomAccessStream, video: *mut super::super::storage::streams::IRandomAccessStream, out: *mut *mut super::super::foundation::IAsyncOperation<SetAccountPictureResult>) -> HRESULT,
    fn add_AccountPictureChanged(&self, changeHandler: *mut super::super::foundation::EventHandler<IInspectable>, out: *mut super::super::foundation::EventRegistrationToken) -> HRESULT,
    fn remove_AccountPictureChanged(&self, token: super::super::foundation::EventRegistrationToken) -> HRESULT,
    fn GetDisplayNameAsync(&self, out: *mut *mut super::super::foundation::IAsyncOperation<HString>) -> HRESULT,
    fn GetFirstNameAsync(&self, out: *mut *mut super::super::foundation::IAsyncOperation<HString>) -> HRESULT,
    fn GetLastNameAsync(&self, out: *mut *mut super::super::foundation::IAsyncOperation<HString>) -> HRESULT,
    fn GetPrincipalNameAsync(&self, out: *mut *mut super::super::foundation::IAsyncOperation<HString>) -> HRESULT,
    fn GetSessionInitiationProtocolUriAsync(&self, out: *mut *mut super::super::foundation::IAsyncOperation<super::super::foundation::Uri>) -> HRESULT,
    fn GetDomainNameAsync(&self, out: *mut *mut super::super::foundation::IAsyncOperation<HString>) -> HRESULT
}}
impl IUserInformationStatics {
    #[inline] pub unsafe fn get_account_picture_change_enabled(&self) -> Result<bool> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_AccountPictureChangeEnabled)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_name_access_allowed(&self) -> Result<bool> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_NameAccessAllowed)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[cfg(feature="windows-storage")] #[inline] pub unsafe fn get_account_picture(&self, kind: AccountPictureKind) -> Result<ComPtr<super::super::storage::IStorageFile>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetAccountPicture)(self as *const _ as *mut _, kind, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[cfg(feature="windows-storage")] #[inline] pub unsafe fn set_account_picture_async(&self, image: &super::super::storage::IStorageFile) -> Result<ComPtr<super::super::foundation::IAsyncOperation<SetAccountPictureResult>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).SetAccountPictureAsync)(self as *const _ as *mut _, image as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[cfg(feature="windows-storage")] #[inline] pub unsafe fn set_account_pictures_async(&self, smallImage: &super::super::storage::IStorageFile, largeImage: &super::super::storage::IStorageFile, video: &super::super::storage::IStorageFile) -> Result<ComPtr<super::super::foundation::IAsyncOperation<SetAccountPictureResult>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).SetAccountPicturesAsync)(self as *const _ as *mut _, smallImage as *const _ as *mut _, largeImage as *const _ as *mut _, video as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[cfg(feature="windows-storage")] #[inline] pub unsafe fn set_account_picture_from_stream_async(&self, image: &super::super::storage::streams::IRandomAccessStream) -> Result<ComPtr<super::super::foundation::IAsyncOperation<SetAccountPictureResult>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).SetAccountPictureFromStreamAsync)(self as *const _ as *mut _, image as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[cfg(feature="windows-storage")] #[inline] pub unsafe fn set_account_pictures_from_streams_async(&self, smallImage: &super::super::storage::streams::IRandomAccessStream, largeImage: &super::super::storage::streams::IRandomAccessStream, video: &super::super::storage::streams::IRandomAccessStream) -> Result<ComPtr<super::super::foundation::IAsyncOperation<SetAccountPictureResult>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).SetAccountPicturesFromStreamsAsync)(self as *const _ as *mut _, smallImage as *const _ as *mut _, largeImage as *const _ as *mut _, video as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn add_account_picture_changed(&self, changeHandler: &super::super::foundation::EventHandler<IInspectable>) -> Result<super::super::foundation::EventRegistrationToken> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).add_AccountPictureChanged)(self as *const _ as *mut _, changeHandler as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn remove_account_picture_changed(&self, token: super::super::foundation::EventRegistrationToken) -> Result<()> {
        let hr = ((*self.lpVtbl).remove_AccountPictureChanged)(self as *const _ as *mut _, token);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_display_name_async(&self) -> Result<ComPtr<super::super::foundation::IAsyncOperation<HString>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetDisplayNameAsync)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_first_name_async(&self) -> Result<ComPtr<super::super::foundation::IAsyncOperation<HString>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetFirstNameAsync)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_last_name_async(&self) -> Result<ComPtr<super::super::foundation::IAsyncOperation<HString>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetLastNameAsync)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_principal_name_async(&self) -> Result<ComPtr<super::super::foundation::IAsyncOperation<HString>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetPrincipalNameAsync)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_session_initiation_protocol_uri_async(&self) -> Result<ComPtr<super::super::foundation::IAsyncOperation<super::super::foundation::Uri>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetSessionInitiationProtocolUriAsync)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_domain_name_async(&self) -> Result<ComPtr<super::super::foundation::IAsyncOperation<HString>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetDomainNameAsync)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
DEFINE_IID!(IID_IUserProfilePersonalizationSettings, 2364398260, 31128, 18133, 141, 211, 24, 79, 28, 95, 154, 185);
RT_INTERFACE!{interface IUserProfilePersonalizationSettings(IUserProfilePersonalizationSettingsVtbl): IInspectable(IInspectableVtbl) [IID_IUserProfilePersonalizationSettings] {
    #[cfg(feature="windows-storage")] fn TrySetLockScreenImageAsync(&self, imageFile: *mut super::super::storage::StorageFile, out: *mut *mut super::super::foundation::IAsyncOperation<bool>) -> HRESULT,
    #[cfg(feature="windows-storage")] fn TrySetWallpaperImageAsync(&self, imageFile: *mut super::super::storage::StorageFile, out: *mut *mut super::super::foundation::IAsyncOperation<bool>) -> HRESULT
}}
impl IUserProfilePersonalizationSettings {
    #[cfg(feature="windows-storage")] #[inline] pub unsafe fn try_set_lock_screen_image_async(&self, imageFile: &super::super::storage::StorageFile) -> Result<ComPtr<super::super::foundation::IAsyncOperation<bool>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).TrySetLockScreenImageAsync)(self as *const _ as *mut _, imageFile as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[cfg(feature="windows-storage")] #[inline] pub unsafe fn try_set_wallpaper_image_async(&self, imageFile: &super::super::storage::StorageFile) -> Result<ComPtr<super::super::foundation::IAsyncOperation<bool>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).TrySetWallpaperImageAsync)(self as *const _ as *mut _, imageFile as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
RT_CLASS!{class UserProfilePersonalizationSettings: IUserProfilePersonalizationSettings}
impl RtActivatable<IUserProfilePersonalizationSettingsStatics> for UserProfilePersonalizationSettings {}
impl UserProfilePersonalizationSettings {
    #[inline] pub fn get_current() -> Result<ComPtr<UserProfilePersonalizationSettings>> { unsafe {
        <Self as RtActivatable<IUserProfilePersonalizationSettingsStatics>>::get_activation_factory().get_current()
    }}
    #[inline] pub fn is_supported() -> Result<bool> { unsafe {
        <Self as RtActivatable<IUserProfilePersonalizationSettingsStatics>>::get_activation_factory().is_supported()
    }}
}
DEFINE_CLSID!(UserProfilePersonalizationSettings(&[87,105,110,100,111,119,115,46,83,121,115,116,101,109,46,85,115,101,114,80,114,111,102,105,108,101,46,85,115,101,114,80,114,111,102,105,108,101,80,101,114,115,111,110,97,108,105,122,97,116,105,111,110,83,101,116,116,105,110,103,115,0]) [CLSID_UserProfilePersonalizationSettings]);
DEFINE_IID!(IID_IUserProfilePersonalizationSettingsStatics, 2444015681, 20535, 17739, 152, 131, 187, 119, 45, 8, 221, 22);
RT_INTERFACE!{static interface IUserProfilePersonalizationSettingsStatics(IUserProfilePersonalizationSettingsStaticsVtbl): IInspectable(IInspectableVtbl) [IID_IUserProfilePersonalizationSettingsStatics] {
    fn get_Current(&self, out: *mut *mut UserProfilePersonalizationSettings) -> HRESULT,
    fn IsSupported(&self, out: *mut bool) -> HRESULT
}}
impl IUserProfilePersonalizationSettingsStatics {
    #[inline] pub unsafe fn get_current(&self) -> Result<ComPtr<UserProfilePersonalizationSettings>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Current)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn is_supported(&self) -> Result<bool> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).IsSupported)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
}
} // Windows.System.UserProfile
pub mod profile { // Windows.System.Profile
use ::prelude::*;
RT_CLASS!{static class AnalyticsInfo}
impl RtActivatable<IAnalyticsInfoStatics> for AnalyticsInfo {}
impl AnalyticsInfo {
    #[inline] pub fn get_version_info() -> Result<ComPtr<AnalyticsVersionInfo>> { unsafe {
        <Self as RtActivatable<IAnalyticsInfoStatics>>::get_activation_factory().get_version_info()
    }}
    #[inline] pub fn get_device_form() -> Result<HString> { unsafe {
        <Self as RtActivatable<IAnalyticsInfoStatics>>::get_activation_factory().get_device_form()
    }}
}
DEFINE_CLSID!(AnalyticsInfo(&[87,105,110,100,111,119,115,46,83,121,115,116,101,109,46,80,114,111,102,105,108,101,46,65,110,97,108,121,116,105,99,115,73,110,102,111,0]) [CLSID_AnalyticsInfo]);
DEFINE_IID!(IID_IAnalyticsInfoStatics, 492757094, 6285, 23465, 67, 135, 172, 174, 176, 231, 227, 5);
RT_INTERFACE!{static interface IAnalyticsInfoStatics(IAnalyticsInfoStaticsVtbl): IInspectable(IInspectableVtbl) [IID_IAnalyticsInfoStatics] {
    fn get_VersionInfo(&self, out: *mut *mut AnalyticsVersionInfo) -> HRESULT,
    fn get_DeviceForm(&self, out: *mut HSTRING) -> HRESULT
}}
impl IAnalyticsInfoStatics {
    #[inline] pub unsafe fn get_version_info(&self) -> Result<ComPtr<AnalyticsVersionInfo>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_VersionInfo)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_device_form(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_DeviceForm)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
}
DEFINE_IID!(IID_IAnalyticsVersionInfo, 2455843000, 39253, 19572, 189, 193, 124, 208, 222, 207, 155, 3);
RT_INTERFACE!{interface IAnalyticsVersionInfo(IAnalyticsVersionInfoVtbl): IInspectable(IInspectableVtbl) [IID_IAnalyticsVersionInfo] {
    fn get_DeviceFamily(&self, out: *mut HSTRING) -> HRESULT,
    fn get_DeviceFamilyVersion(&self, out: *mut HSTRING) -> HRESULT
}}
impl IAnalyticsVersionInfo {
    #[inline] pub unsafe fn get_device_family(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_DeviceFamily)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_device_family_version(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_DeviceFamilyVersion)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
}
RT_CLASS!{class AnalyticsVersionInfo: IAnalyticsVersionInfo}
RT_CLASS!{static class EducationSettings}
impl RtActivatable<IEducationSettingsStatics> for EducationSettings {}
impl EducationSettings {
    #[inline] pub fn get_is_education_environment() -> Result<bool> { unsafe {
        <Self as RtActivatable<IEducationSettingsStatics>>::get_activation_factory().get_is_education_environment()
    }}
}
DEFINE_CLSID!(EducationSettings(&[87,105,110,100,111,119,115,46,83,121,115,116,101,109,46,80,114,111,102,105,108,101,46,69,100,117,99,97,116,105,111,110,83,101,116,116,105,110,103,115,0]) [CLSID_EducationSettings]);
DEFINE_IID!(IID_IEducationSettingsStatics, 4233359599, 19774, 19987, 155, 35, 80, 95, 77, 9, 30, 146);
RT_INTERFACE!{static interface IEducationSettingsStatics(IEducationSettingsStaticsVtbl): IInspectable(IInspectableVtbl) [IID_IEducationSettingsStatics] {
    fn get_IsEducationEnvironment(&self, out: *mut bool) -> HRESULT
}}
impl IEducationSettingsStatics {
    #[inline] pub unsafe fn get_is_education_environment(&self) -> Result<bool> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_IsEducationEnvironment)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
}
RT_CLASS!{static class HardwareIdentification}
impl RtActivatable<IHardwareIdentificationStatics> for HardwareIdentification {}
impl HardwareIdentification {
    #[cfg(feature="windows-storage")] #[inline] pub fn get_package_specific_token(nonce: &super::super::storage::streams::IBuffer) -> Result<ComPtr<HardwareToken>> { unsafe {
        <Self as RtActivatable<IHardwareIdentificationStatics>>::get_activation_factory().get_package_specific_token(nonce)
    }}
}
DEFINE_CLSID!(HardwareIdentification(&[87,105,110,100,111,119,115,46,83,121,115,116,101,109,46,80,114,111,102,105,108,101,46,72,97,114,100,119,97,114,101,73,100,101,110,116,105,102,105,99,97,116,105,111,110,0]) [CLSID_HardwareIdentification]);
DEFINE_IID!(IID_IHardwareIdentificationStatics, 2534564064, 61808, 19010, 189, 85, 169, 0, 178, 18, 218, 226);
RT_INTERFACE!{static interface IHardwareIdentificationStatics(IHardwareIdentificationStaticsVtbl): IInspectable(IInspectableVtbl) [IID_IHardwareIdentificationStatics] {
    #[cfg(feature="windows-storage")] fn GetPackageSpecificToken(&self, nonce: *mut super::super::storage::streams::IBuffer, out: *mut *mut HardwareToken) -> HRESULT
}}
impl IHardwareIdentificationStatics {
    #[cfg(feature="windows-storage")] #[inline] pub unsafe fn get_package_specific_token(&self, nonce: &super::super::storage::streams::IBuffer) -> Result<ComPtr<HardwareToken>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetPackageSpecificToken)(self as *const _ as *mut _, nonce as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
DEFINE_IID!(IID_IHardwareToken, 687264960, 64274, 16548, 129, 103, 127, 78, 3, 210, 114, 76);
RT_INTERFACE!{interface IHardwareToken(IHardwareTokenVtbl): IInspectable(IInspectableVtbl) [IID_IHardwareToken] {
    #[cfg(feature="windows-storage")] fn get_Id(&self, out: *mut *mut super::super::storage::streams::IBuffer) -> HRESULT,
    #[cfg(feature="windows-storage")] fn get_Signature(&self, out: *mut *mut super::super::storage::streams::IBuffer) -> HRESULT,
    #[cfg(feature="windows-storage")] fn get_Certificate(&self, out: *mut *mut super::super::storage::streams::IBuffer) -> HRESULT
}}
impl IHardwareToken {
    #[cfg(feature="windows-storage")] #[inline] pub unsafe fn get_id(&self) -> Result<ComPtr<super::super::storage::streams::IBuffer>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Id)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[cfg(feature="windows-storage")] #[inline] pub unsafe fn get_signature(&self) -> Result<ComPtr<super::super::storage::streams::IBuffer>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Signature)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[cfg(feature="windows-storage")] #[inline] pub unsafe fn get_certificate(&self) -> Result<ComPtr<super::super::storage::streams::IBuffer>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Certificate)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
RT_CLASS!{class HardwareToken: IHardwareToken}
RT_CLASS!{static class KnownRetailInfoProperties}
impl RtActivatable<IKnownRetailInfoPropertiesStatics> for KnownRetailInfoProperties {}
impl KnownRetailInfoProperties {
    #[inline] pub fn get_retail_access_code() -> Result<HString> { unsafe {
        <Self as RtActivatable<IKnownRetailInfoPropertiesStatics>>::get_activation_factory().get_retail_access_code()
    }}
    #[inline] pub fn get_manufacturer_name() -> Result<HString> { unsafe {
        <Self as RtActivatable<IKnownRetailInfoPropertiesStatics>>::get_activation_factory().get_manufacturer_name()
    }}
    #[inline] pub fn get_model_name() -> Result<HString> { unsafe {
        <Self as RtActivatable<IKnownRetailInfoPropertiesStatics>>::get_activation_factory().get_model_name()
    }}
    #[inline] pub fn get_display_model_name() -> Result<HString> { unsafe {
        <Self as RtActivatable<IKnownRetailInfoPropertiesStatics>>::get_activation_factory().get_display_model_name()
    }}
    #[inline] pub fn get_price() -> Result<HString> { unsafe {
        <Self as RtActivatable<IKnownRetailInfoPropertiesStatics>>::get_activation_factory().get_price()
    }}
    #[inline] pub fn get_is_featured() -> Result<HString> { unsafe {
        <Self as RtActivatable<IKnownRetailInfoPropertiesStatics>>::get_activation_factory().get_is_featured()
    }}
    #[inline] pub fn get_form_factor() -> Result<HString> { unsafe {
        <Self as RtActivatable<IKnownRetailInfoPropertiesStatics>>::get_activation_factory().get_form_factor()
    }}
    #[inline] pub fn get_screen_size() -> Result<HString> { unsafe {
        <Self as RtActivatable<IKnownRetailInfoPropertiesStatics>>::get_activation_factory().get_screen_size()
    }}
    #[inline] pub fn get_weight() -> Result<HString> { unsafe {
        <Self as RtActivatable<IKnownRetailInfoPropertiesStatics>>::get_activation_factory().get_weight()
    }}
    #[inline] pub fn get_display_description() -> Result<HString> { unsafe {
        <Self as RtActivatable<IKnownRetailInfoPropertiesStatics>>::get_activation_factory().get_display_description()
    }}
    #[inline] pub fn get_battery_life_description() -> Result<HString> { unsafe {
        <Self as RtActivatable<IKnownRetailInfoPropertiesStatics>>::get_activation_factory().get_battery_life_description()
    }}
    #[inline] pub fn get_processor_description() -> Result<HString> { unsafe {
        <Self as RtActivatable<IKnownRetailInfoPropertiesStatics>>::get_activation_factory().get_processor_description()
    }}
    #[inline] pub fn get_memory() -> Result<HString> { unsafe {
        <Self as RtActivatable<IKnownRetailInfoPropertiesStatics>>::get_activation_factory().get_memory()
    }}
    #[inline] pub fn get_storage_description() -> Result<HString> { unsafe {
        <Self as RtActivatable<IKnownRetailInfoPropertiesStatics>>::get_activation_factory().get_storage_description()
    }}
    #[inline] pub fn get_graphics_description() -> Result<HString> { unsafe {
        <Self as RtActivatable<IKnownRetailInfoPropertiesStatics>>::get_activation_factory().get_graphics_description()
    }}
    #[inline] pub fn get_front_camera_description() -> Result<HString> { unsafe {
        <Self as RtActivatable<IKnownRetailInfoPropertiesStatics>>::get_activation_factory().get_front_camera_description()
    }}
    #[inline] pub fn get_rear_camera_description() -> Result<HString> { unsafe {
        <Self as RtActivatable<IKnownRetailInfoPropertiesStatics>>::get_activation_factory().get_rear_camera_description()
    }}
    #[inline] pub fn get_has_nfc() -> Result<HString> { unsafe {
        <Self as RtActivatable<IKnownRetailInfoPropertiesStatics>>::get_activation_factory().get_has_nfc()
    }}
    #[inline] pub fn get_has_sd_slot() -> Result<HString> { unsafe {
        <Self as RtActivatable<IKnownRetailInfoPropertiesStatics>>::get_activation_factory().get_has_sd_slot()
    }}
    #[inline] pub fn get_has_optical_drive() -> Result<HString> { unsafe {
        <Self as RtActivatable<IKnownRetailInfoPropertiesStatics>>::get_activation_factory().get_has_optical_drive()
    }}
    #[inline] pub fn get_is_office_installed() -> Result<HString> { unsafe {
        <Self as RtActivatable<IKnownRetailInfoPropertiesStatics>>::get_activation_factory().get_is_office_installed()
    }}
    #[inline] pub fn get_windows_edition() -> Result<HString> { unsafe {
        <Self as RtActivatable<IKnownRetailInfoPropertiesStatics>>::get_activation_factory().get_windows_edition()
    }}
}
DEFINE_CLSID!(KnownRetailInfoProperties(&[87,105,110,100,111,119,115,46,83,121,115,116,101,109,46,80,114,111,102,105,108,101,46,75,110,111,119,110,82,101,116,97,105,108,73,110,102,111,80,114,111,112,101,114,116,105,101,115,0]) [CLSID_KnownRetailInfoProperties]);
DEFINE_IID!(IID_IKnownRetailInfoPropertiesStatics, 2572620152, 20495, 18558, 142, 117, 41, 229, 81, 114, 135, 18);
RT_INTERFACE!{static interface IKnownRetailInfoPropertiesStatics(IKnownRetailInfoPropertiesStaticsVtbl): IInspectable(IInspectableVtbl) [IID_IKnownRetailInfoPropertiesStatics] {
    fn get_RetailAccessCode(&self, out: *mut HSTRING) -> HRESULT,
    fn get_ManufacturerName(&self, out: *mut HSTRING) -> HRESULT,
    fn get_ModelName(&self, out: *mut HSTRING) -> HRESULT,
    fn get_DisplayModelName(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Price(&self, out: *mut HSTRING) -> HRESULT,
    fn get_IsFeatured(&self, out: *mut HSTRING) -> HRESULT,
    fn get_FormFactor(&self, out: *mut HSTRING) -> HRESULT,
    fn get_ScreenSize(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Weight(&self, out: *mut HSTRING) -> HRESULT,
    fn get_DisplayDescription(&self, out: *mut HSTRING) -> HRESULT,
    fn get_BatteryLifeDescription(&self, out: *mut HSTRING) -> HRESULT,
    fn get_ProcessorDescription(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Memory(&self, out: *mut HSTRING) -> HRESULT,
    fn get_StorageDescription(&self, out: *mut HSTRING) -> HRESULT,
    fn get_GraphicsDescription(&self, out: *mut HSTRING) -> HRESULT,
    fn get_FrontCameraDescription(&self, out: *mut HSTRING) -> HRESULT,
    fn get_RearCameraDescription(&self, out: *mut HSTRING) -> HRESULT,
    fn get_HasNfc(&self, out: *mut HSTRING) -> HRESULT,
    fn get_HasSdSlot(&self, out: *mut HSTRING) -> HRESULT,
    fn get_HasOpticalDrive(&self, out: *mut HSTRING) -> HRESULT,
    fn get_IsOfficeInstalled(&self, out: *mut HSTRING) -> HRESULT,
    fn get_WindowsEdition(&self, out: *mut HSTRING) -> HRESULT
}}
impl IKnownRetailInfoPropertiesStatics {
    #[inline] pub unsafe fn get_retail_access_code(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_RetailAccessCode)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_manufacturer_name(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_ManufacturerName)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_model_name(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_ModelName)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_display_model_name(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_DisplayModelName)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_price(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Price)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_is_featured(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_IsFeatured)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_form_factor(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_FormFactor)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_screen_size(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_ScreenSize)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_weight(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Weight)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_display_description(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_DisplayDescription)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_battery_life_description(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_BatteryLifeDescription)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_processor_description(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_ProcessorDescription)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_memory(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Memory)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_storage_description(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_StorageDescription)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_graphics_description(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_GraphicsDescription)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_front_camera_description(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_FrontCameraDescription)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_rear_camera_description(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_RearCameraDescription)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_has_nfc(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_HasNfc)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_has_sd_slot(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_HasSdSlot)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_has_optical_drive(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_HasOpticalDrive)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_is_office_installed(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_IsOfficeInstalled)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_windows_edition(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_WindowsEdition)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
}
RT_ENUM! { enum PlatformDataCollectionLevel: i32 {
    Security (PlatformDataCollectionLevel_Security) = 0, Basic (PlatformDataCollectionLevel_Basic) = 1, Enhanced (PlatformDataCollectionLevel_Enhanced) = 2, Full (PlatformDataCollectionLevel_Full) = 3,
}}
RT_CLASS!{static class PlatformDiagnosticsAndUsageDataSettings}
impl RtActivatable<IPlatformDiagnosticsAndUsageDataSettingsStatics> for PlatformDiagnosticsAndUsageDataSettings {}
impl PlatformDiagnosticsAndUsageDataSettings {
    #[inline] pub fn get_collection_level() -> Result<PlatformDataCollectionLevel> { unsafe {
        <Self as RtActivatable<IPlatformDiagnosticsAndUsageDataSettingsStatics>>::get_activation_factory().get_collection_level()
    }}
    #[inline] pub fn add_collection_level_changed(handler: &super::super::foundation::EventHandler<IInspectable>) -> Result<super::super::foundation::EventRegistrationToken> { unsafe {
        <Self as RtActivatable<IPlatformDiagnosticsAndUsageDataSettingsStatics>>::get_activation_factory().add_collection_level_changed(handler)
    }}
    #[inline] pub fn remove_collection_level_changed(token: super::super::foundation::EventRegistrationToken) -> Result<()> { unsafe {
        <Self as RtActivatable<IPlatformDiagnosticsAndUsageDataSettingsStatics>>::get_activation_factory().remove_collection_level_changed(token)
    }}
    #[inline] pub fn can_collect_diagnostics(level: PlatformDataCollectionLevel) -> Result<bool> { unsafe {
        <Self as RtActivatable<IPlatformDiagnosticsAndUsageDataSettingsStatics>>::get_activation_factory().can_collect_diagnostics(level)
    }}
}
DEFINE_CLSID!(PlatformDiagnosticsAndUsageDataSettings(&[87,105,110,100,111,119,115,46,83,121,115,116,101,109,46,80,114,111,102,105,108,101,46,80,108,97,116,102,111,114,109,68,105,97,103,110,111,115,116,105,99,115,65,110,100,85,115,97,103,101,68,97,116,97,83,101,116,116,105,110,103,115,0]) [CLSID_PlatformDiagnosticsAndUsageDataSettings]);
DEFINE_IID!(IID_IPlatformDiagnosticsAndUsageDataSettingsStatics, 3068283931, 31516, 19250, 140, 98, 166, 101, 151, 206, 114, 58);
RT_INTERFACE!{static interface IPlatformDiagnosticsAndUsageDataSettingsStatics(IPlatformDiagnosticsAndUsageDataSettingsStaticsVtbl): IInspectable(IInspectableVtbl) [IID_IPlatformDiagnosticsAndUsageDataSettingsStatics] {
    fn get_CollectionLevel(&self, out: *mut PlatformDataCollectionLevel) -> HRESULT,
    fn add_CollectionLevelChanged(&self, handler: *mut super::super::foundation::EventHandler<IInspectable>, out: *mut super::super::foundation::EventRegistrationToken) -> HRESULT,
    fn remove_CollectionLevelChanged(&self, token: super::super::foundation::EventRegistrationToken) -> HRESULT,
    fn CanCollectDiagnostics(&self, level: PlatformDataCollectionLevel, out: *mut bool) -> HRESULT
}}
impl IPlatformDiagnosticsAndUsageDataSettingsStatics {
    #[inline] pub unsafe fn get_collection_level(&self) -> Result<PlatformDataCollectionLevel> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_CollectionLevel)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn add_collection_level_changed(&self, handler: &super::super::foundation::EventHandler<IInspectable>) -> Result<super::super::foundation::EventRegistrationToken> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).add_CollectionLevelChanged)(self as *const _ as *mut _, handler as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn remove_collection_level_changed(&self, token: super::super::foundation::EventRegistrationToken) -> Result<()> {
        let hr = ((*self.lpVtbl).remove_CollectionLevelChanged)(self as *const _ as *mut _, token);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn can_collect_diagnostics(&self, level: PlatformDataCollectionLevel) -> Result<bool> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).CanCollectDiagnostics)(self as *const _ as *mut _, level, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
}
RT_CLASS!{static class RetailInfo}
impl RtActivatable<IRetailInfoStatics> for RetailInfo {}
impl RetailInfo {
    #[inline] pub fn get_is_demo_mode_enabled() -> Result<bool> { unsafe {
        <Self as RtActivatable<IRetailInfoStatics>>::get_activation_factory().get_is_demo_mode_enabled()
    }}
    #[inline] pub fn get_properties() -> Result<ComPtr<super::super::foundation::collections::IMapView<HString, IInspectable>>> { unsafe {
        <Self as RtActivatable<IRetailInfoStatics>>::get_activation_factory().get_properties()
    }}
}
DEFINE_CLSID!(RetailInfo(&[87,105,110,100,111,119,115,46,83,121,115,116,101,109,46,80,114,111,102,105,108,101,46,82,101,116,97,105,108,73,110,102,111,0]) [CLSID_RetailInfo]);
DEFINE_IID!(IID_IRetailInfoStatics, 118671032, 35730, 20266, 132, 153, 3, 31, 23, 152, 214, 239);
RT_INTERFACE!{static interface IRetailInfoStatics(IRetailInfoStaticsVtbl): IInspectable(IInspectableVtbl) [IID_IRetailInfoStatics] {
    fn get_IsDemoModeEnabled(&self, out: *mut bool) -> HRESULT,
    fn get_Properties(&self, out: *mut *mut super::super::foundation::collections::IMapView<HString, IInspectable>) -> HRESULT
}}
impl IRetailInfoStatics {
    #[inline] pub unsafe fn get_is_demo_mode_enabled(&self) -> Result<bool> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_IsDemoModeEnabled)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_properties(&self) -> Result<ComPtr<super::super::foundation::collections::IMapView<HString, IInspectable>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Properties)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
RT_CLASS!{static class SharedModeSettings}
impl RtActivatable<ISharedModeSettingsStatics> for SharedModeSettings {}
impl RtActivatable<ISharedModeSettingsStatics2> for SharedModeSettings {}
impl SharedModeSettings {
    #[inline] pub fn get_is_enabled() -> Result<bool> { unsafe {
        <Self as RtActivatable<ISharedModeSettingsStatics>>::get_activation_factory().get_is_enabled()
    }}
    #[inline] pub fn get_should_avoid_local_storage() -> Result<bool> { unsafe {
        <Self as RtActivatable<ISharedModeSettingsStatics2>>::get_activation_factory().get_should_avoid_local_storage()
    }}
}
DEFINE_CLSID!(SharedModeSettings(&[87,105,110,100,111,119,115,46,83,121,115,116,101,109,46,80,114,111,102,105,108,101,46,83,104,97,114,101,100,77,111,100,101,83,101,116,116,105,110,103,115,0]) [CLSID_SharedModeSettings]);
DEFINE_IID!(IID_ISharedModeSettingsStatics, 2302538766, 51926, 19792, 140, 73, 111, 207, 192, 62, 219, 41);
RT_INTERFACE!{static interface ISharedModeSettingsStatics(ISharedModeSettingsStaticsVtbl): IInspectable(IInspectableVtbl) [IID_ISharedModeSettingsStatics] {
    fn get_IsEnabled(&self, out: *mut bool) -> HRESULT
}}
impl ISharedModeSettingsStatics {
    #[inline] pub unsafe fn get_is_enabled(&self) -> Result<bool> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_IsEnabled)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
}
DEFINE_IID!(IID_ISharedModeSettingsStatics2, 1619626148, 52465, 20200, 165, 226, 253, 106, 29, 12, 250, 200);
RT_INTERFACE!{static interface ISharedModeSettingsStatics2(ISharedModeSettingsStatics2Vtbl): IInspectable(IInspectableVtbl) [IID_ISharedModeSettingsStatics2] {
    fn get_ShouldAvoidLocalStorage(&self, out: *mut bool) -> HRESULT
}}
impl ISharedModeSettingsStatics2 {
    #[inline] pub unsafe fn get_should_avoid_local_storage(&self) -> Result<bool> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_ShouldAvoidLocalStorage)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
}
RT_CLASS!{static class SystemIdentification}
impl RtActivatable<ISystemIdentificationStatics> for SystemIdentification {}
impl SystemIdentification {
    #[inline] pub fn get_system_id_for_publisher() -> Result<ComPtr<SystemIdentificationInfo>> { unsafe {
        <Self as RtActivatable<ISystemIdentificationStatics>>::get_activation_factory().get_system_id_for_publisher()
    }}
    #[inline] pub fn get_system_id_for_user(user: &super::User) -> Result<ComPtr<SystemIdentificationInfo>> { unsafe {
        <Self as RtActivatable<ISystemIdentificationStatics>>::get_activation_factory().get_system_id_for_user(user)
    }}
}
DEFINE_CLSID!(SystemIdentification(&[87,105,110,100,111,119,115,46,83,121,115,116,101,109,46,80,114,111,102,105,108,101,46,83,121,115,116,101,109,73,100,101,110,116,105,102,105,99,97,116,105,111,110,0]) [CLSID_SystemIdentification]);
DEFINE_IID!(IID_ISystemIdentificationInfo, 207986301, 50114, 19763, 162, 223, 33, 188, 65, 145, 110, 179);
RT_INTERFACE!{interface ISystemIdentificationInfo(ISystemIdentificationInfoVtbl): IInspectable(IInspectableVtbl) [IID_ISystemIdentificationInfo] {
    #[cfg(not(feature="windows-storage"))] fn __Dummy0(&self) -> (),
    #[cfg(feature="windows-storage")] fn get_Id(&self, out: *mut *mut super::super::storage::streams::IBuffer) -> HRESULT,
    fn get_Source(&self, out: *mut SystemIdentificationSource) -> HRESULT
}}
impl ISystemIdentificationInfo {
    #[cfg(feature="windows-storage")] #[inline] pub unsafe fn get_id(&self) -> Result<ComPtr<super::super::storage::streams::IBuffer>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Id)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_source(&self) -> Result<SystemIdentificationSource> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_Source)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
}
RT_CLASS!{class SystemIdentificationInfo: ISystemIdentificationInfo}
RT_ENUM! { enum SystemIdentificationSource: i32 {
    None (SystemIdentificationSource_None) = 0, Tpm (SystemIdentificationSource_Tpm) = 1, Uefi (SystemIdentificationSource_Uefi) = 2, Registry (SystemIdentificationSource_Registry) = 3,
}}
DEFINE_IID!(IID_ISystemIdentificationStatics, 1434580010, 54239, 19859, 163, 125, 196, 26, 97, 108, 109, 1);
RT_INTERFACE!{static interface ISystemIdentificationStatics(ISystemIdentificationStaticsVtbl): IInspectable(IInspectableVtbl) [IID_ISystemIdentificationStatics] {
    fn GetSystemIdForPublisher(&self, out: *mut *mut SystemIdentificationInfo) -> HRESULT,
    fn GetSystemIdForUser(&self, user: *mut super::User, out: *mut *mut SystemIdentificationInfo) -> HRESULT
}}
impl ISystemIdentificationStatics {
    #[inline] pub unsafe fn get_system_id_for_publisher(&self) -> Result<ComPtr<SystemIdentificationInfo>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetSystemIdForPublisher)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_system_id_for_user(&self, user: &super::User) -> Result<ComPtr<SystemIdentificationInfo>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetSystemIdForUser)(self as *const _ as *mut _, user as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
pub mod systemmanufacturers { // Windows.System.Profile.SystemManufacturers
use ::prelude::*;
DEFINE_IID!(IID_IOemSupportInfo, 2368646741, 34799, 16998, 134, 208, 196, 175, 190, 178, 155, 185);
RT_INTERFACE!{interface IOemSupportInfo(IOemSupportInfoVtbl): IInspectable(IInspectableVtbl) [IID_IOemSupportInfo] {
    fn get_SupportLink(&self, out: *mut *mut ::rt::gen::windows::foundation::Uri) -> HRESULT,
    fn get_SupportAppLink(&self, out: *mut *mut ::rt::gen::windows::foundation::Uri) -> HRESULT,
    fn get_SupportProvider(&self, out: *mut HSTRING) -> HRESULT
}}
impl IOemSupportInfo {
    #[inline] pub unsafe fn get_support_link(&self) -> Result<ComPtr<::rt::gen::windows::foundation::Uri>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_SupportLink)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_support_app_link(&self) -> Result<ComPtr<::rt::gen::windows::foundation::Uri>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_SupportAppLink)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_support_provider(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_SupportProvider)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
}
RT_CLASS!{class OemSupportInfo: IOemSupportInfo}
RT_CLASS!{static class SmbiosInformation}
impl RtActivatable<ISmbiosInformationStatics> for SmbiosInformation {}
impl SmbiosInformation {
    #[inline] pub fn get_serial_number() -> Result<HString> { unsafe {
        <Self as RtActivatable<ISmbiosInformationStatics>>::get_activation_factory().get_serial_number()
    }}
}
DEFINE_CLSID!(SmbiosInformation(&[87,105,110,100,111,119,115,46,83,121,115,116,101,109,46,80,114,111,102,105,108,101,46,83,121,115,116,101,109,77,97,110,117,102,97,99,116,117,114,101,114,115,46,83,109,98,105,111,115,73,110,102,111,114,109,97,116,105,111,110,0]) [CLSID_SmbiosInformation]);
DEFINE_IID!(IID_ISmbiosInformationStatics, 135055996, 25468, 18628, 183, 40, 249, 39, 56, 18, 219, 142);
RT_INTERFACE!{static interface ISmbiosInformationStatics(ISmbiosInformationStaticsVtbl): IInspectable(IInspectableVtbl) [IID_ISmbiosInformationStatics] {
    fn get_SerialNumber(&self, out: *mut HSTRING) -> HRESULT
}}
impl ISmbiosInformationStatics {
    #[inline] pub unsafe fn get_serial_number(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_SerialNumber)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
}
RT_CLASS!{static class SystemSupportInfo}
impl RtActivatable<ISystemSupportInfoStatics> for SystemSupportInfo {}
impl SystemSupportInfo {
    #[inline] pub fn get_local_system_edition() -> Result<HString> { unsafe {
        <Self as RtActivatable<ISystemSupportInfoStatics>>::get_activation_factory().get_local_system_edition()
    }}
    #[inline] pub fn get_oem_support_info() -> Result<ComPtr<OemSupportInfo>> { unsafe {
        <Self as RtActivatable<ISystemSupportInfoStatics>>::get_activation_factory().get_oem_support_info()
    }}
}
DEFINE_CLSID!(SystemSupportInfo(&[87,105,110,100,111,119,115,46,83,121,115,116,101,109,46,80,114,111,102,105,108,101,46,83,121,115,116,101,109,77,97,110,117,102,97,99,116,117,114,101,114,115,46,83,121,115,116,101,109,83,117,112,112,111,114,116,73,110,102,111,0]) [CLSID_SystemSupportInfo]);
DEFINE_IID!(IID_ISystemSupportInfoStatics, 4017424756, 50210, 17879, 164, 77, 92, 28, 0, 67, 162, 179);
RT_INTERFACE!{static interface ISystemSupportInfoStatics(ISystemSupportInfoStaticsVtbl): IInspectable(IInspectableVtbl) [IID_ISystemSupportInfoStatics] {
    fn get_LocalSystemEdition(&self, out: *mut HSTRING) -> HRESULT,
    fn get_OemSupportInfo(&self, out: *mut *mut OemSupportInfo) -> HRESULT
}}
impl ISystemSupportInfoStatics {
    #[inline] pub unsafe fn get_local_system_edition(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_LocalSystemEdition)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_oem_support_info(&self) -> Result<ComPtr<OemSupportInfo>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_OemSupportInfo)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
} // Windows.System.Profile.SystemManufacturers
} // Windows.System.Profile
pub mod remotedesktop { // Windows.System.RemoteDesktop
use ::prelude::*;
RT_CLASS!{static class InteractiveSession}
impl RtActivatable<IInteractiveSessionStatics> for InteractiveSession {}
impl InteractiveSession {
    #[inline] pub fn get_is_remote() -> Result<bool> { unsafe {
        <Self as RtActivatable<IInteractiveSessionStatics>>::get_activation_factory().get_is_remote()
    }}
}
DEFINE_CLSID!(InteractiveSession(&[87,105,110,100,111,119,115,46,83,121,115,116,101,109,46,82,101,109,111,116,101,68,101,115,107,116,111,112,46,73,110,116,101,114,97,99,116,105,118,101,83,101,115,115,105,111,110,0]) [CLSID_InteractiveSession]);
DEFINE_IID!(IID_IInteractiveSessionStatics, 1619543601, 56634, 17782, 156, 141, 232, 2, 118, 24, 189, 206);
RT_INTERFACE!{static interface IInteractiveSessionStatics(IInteractiveSessionStaticsVtbl): IInspectable(IInspectableVtbl) [IID_IInteractiveSessionStatics] {
    fn get_IsRemote(&self, out: *mut bool) -> HRESULT
}}
impl IInteractiveSessionStatics {
    #[inline] pub unsafe fn get_is_remote(&self) -> Result<bool> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_IsRemote)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
}
} // Windows.System.RemoteDesktop
pub mod power { // Windows.System.Power
use ::prelude::*;
RT_CLASS!{static class BackgroundEnergyManager}
impl RtActivatable<IBackgroundEnergyManagerStatics> for BackgroundEnergyManager {}
impl BackgroundEnergyManager {
    #[inline] pub fn get_low_usage_level() -> Result<u32> { unsafe {
        <Self as RtActivatable<IBackgroundEnergyManagerStatics>>::get_activation_factory().get_low_usage_level()
    }}
    #[inline] pub fn get_near_max_acceptable_usage_level() -> Result<u32> { unsafe {
        <Self as RtActivatable<IBackgroundEnergyManagerStatics>>::get_activation_factory().get_near_max_acceptable_usage_level()
    }}
    #[inline] pub fn get_max_acceptable_usage_level() -> Result<u32> { unsafe {
        <Self as RtActivatable<IBackgroundEnergyManagerStatics>>::get_activation_factory().get_max_acceptable_usage_level()
    }}
    #[inline] pub fn get_excessive_usage_level() -> Result<u32> { unsafe {
        <Self as RtActivatable<IBackgroundEnergyManagerStatics>>::get_activation_factory().get_excessive_usage_level()
    }}
    #[inline] pub fn get_near_termination_usage_level() -> Result<u32> { unsafe {
        <Self as RtActivatable<IBackgroundEnergyManagerStatics>>::get_activation_factory().get_near_termination_usage_level()
    }}
    #[inline] pub fn get_termination_usage_level() -> Result<u32> { unsafe {
        <Self as RtActivatable<IBackgroundEnergyManagerStatics>>::get_activation_factory().get_termination_usage_level()
    }}
    #[inline] pub fn get_recent_energy_usage() -> Result<u32> { unsafe {
        <Self as RtActivatable<IBackgroundEnergyManagerStatics>>::get_activation_factory().get_recent_energy_usage()
    }}
    #[inline] pub fn get_recent_energy_usage_level() -> Result<u32> { unsafe {
        <Self as RtActivatable<IBackgroundEnergyManagerStatics>>::get_activation_factory().get_recent_energy_usage_level()
    }}
    #[inline] pub fn add_recent_energy_usage_increased(handler: &super::super::foundation::EventHandler<IInspectable>) -> Result<super::super::foundation::EventRegistrationToken> { unsafe {
        <Self as RtActivatable<IBackgroundEnergyManagerStatics>>::get_activation_factory().add_recent_energy_usage_increased(handler)
    }}
    #[inline] pub fn remove_recent_energy_usage_increased(token: super::super::foundation::EventRegistrationToken) -> Result<()> { unsafe {
        <Self as RtActivatable<IBackgroundEnergyManagerStatics>>::get_activation_factory().remove_recent_energy_usage_increased(token)
    }}
    #[inline] pub fn add_recent_energy_usage_returned_to_low(handler: &super::super::foundation::EventHandler<IInspectable>) -> Result<super::super::foundation::EventRegistrationToken> { unsafe {
        <Self as RtActivatable<IBackgroundEnergyManagerStatics>>::get_activation_factory().add_recent_energy_usage_returned_to_low(handler)
    }}
    #[inline] pub fn remove_recent_energy_usage_returned_to_low(token: super::super::foundation::EventRegistrationToken) -> Result<()> { unsafe {
        <Self as RtActivatable<IBackgroundEnergyManagerStatics>>::get_activation_factory().remove_recent_energy_usage_returned_to_low(token)
    }}
}
DEFINE_CLSID!(BackgroundEnergyManager(&[87,105,110,100,111,119,115,46,83,121,115,116,101,109,46,80,111,119,101,114,46,66,97,99,107,103,114,111,117,110,100,69,110,101,114,103,121,77,97,110,97,103,101,114,0]) [CLSID_BackgroundEnergyManager]);
DEFINE_IID!(IID_IBackgroundEnergyManagerStatics, 3004571029, 4480, 17270, 150, 225, 64, 149, 86, 129, 71, 206);
RT_INTERFACE!{static interface IBackgroundEnergyManagerStatics(IBackgroundEnergyManagerStaticsVtbl): IInspectable(IInspectableVtbl) [IID_IBackgroundEnergyManagerStatics] {
    fn get_LowUsageLevel(&self, out: *mut u32) -> HRESULT,
    fn get_NearMaxAcceptableUsageLevel(&self, out: *mut u32) -> HRESULT,
    fn get_MaxAcceptableUsageLevel(&self, out: *mut u32) -> HRESULT,
    fn get_ExcessiveUsageLevel(&self, out: *mut u32) -> HRESULT,
    fn get_NearTerminationUsageLevel(&self, out: *mut u32) -> HRESULT,
    fn get_TerminationUsageLevel(&self, out: *mut u32) -> HRESULT,
    fn get_RecentEnergyUsage(&self, out: *mut u32) -> HRESULT,
    fn get_RecentEnergyUsageLevel(&self, out: *mut u32) -> HRESULT,
    fn add_RecentEnergyUsageIncreased(&self, handler: *mut super::super::foundation::EventHandler<IInspectable>, out: *mut super::super::foundation::EventRegistrationToken) -> HRESULT,
    fn remove_RecentEnergyUsageIncreased(&self, token: super::super::foundation::EventRegistrationToken) -> HRESULT,
    fn add_RecentEnergyUsageReturnedToLow(&self, handler: *mut super::super::foundation::EventHandler<IInspectable>, out: *mut super::super::foundation::EventRegistrationToken) -> HRESULT,
    fn remove_RecentEnergyUsageReturnedToLow(&self, token: super::super::foundation::EventRegistrationToken) -> HRESULT
}}
impl IBackgroundEnergyManagerStatics {
    #[inline] pub unsafe fn get_low_usage_level(&self) -> Result<u32> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_LowUsageLevel)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_near_max_acceptable_usage_level(&self) -> Result<u32> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_NearMaxAcceptableUsageLevel)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_max_acceptable_usage_level(&self) -> Result<u32> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_MaxAcceptableUsageLevel)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_excessive_usage_level(&self) -> Result<u32> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_ExcessiveUsageLevel)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_near_termination_usage_level(&self) -> Result<u32> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_NearTerminationUsageLevel)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_termination_usage_level(&self) -> Result<u32> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_TerminationUsageLevel)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_recent_energy_usage(&self) -> Result<u32> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_RecentEnergyUsage)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_recent_energy_usage_level(&self) -> Result<u32> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_RecentEnergyUsageLevel)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn add_recent_energy_usage_increased(&self, handler: &super::super::foundation::EventHandler<IInspectable>) -> Result<super::super::foundation::EventRegistrationToken> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).add_RecentEnergyUsageIncreased)(self as *const _ as *mut _, handler as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn remove_recent_energy_usage_increased(&self, token: super::super::foundation::EventRegistrationToken) -> Result<()> {
        let hr = ((*self.lpVtbl).remove_RecentEnergyUsageIncreased)(self as *const _ as *mut _, token);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn add_recent_energy_usage_returned_to_low(&self, handler: &super::super::foundation::EventHandler<IInspectable>) -> Result<super::super::foundation::EventRegistrationToken> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).add_RecentEnergyUsageReturnedToLow)(self as *const _ as *mut _, handler as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn remove_recent_energy_usage_returned_to_low(&self, token: super::super::foundation::EventRegistrationToken) -> Result<()> {
        let hr = ((*self.lpVtbl).remove_RecentEnergyUsageReturnedToLow)(self as *const _ as *mut _, token);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
}
RT_ENUM! { enum BatteryStatus: i32 {
    NotPresent (BatteryStatus_NotPresent) = 0, Discharging (BatteryStatus_Discharging) = 1, Idle (BatteryStatus_Idle) = 2, Charging (BatteryStatus_Charging) = 3,
}}
RT_ENUM! { enum EnergySaverStatus: i32 {
    Disabled (EnergySaverStatus_Disabled) = 0, Off (EnergySaverStatus_Off) = 1, On (EnergySaverStatus_On) = 2,
}}
RT_CLASS!{static class ForegroundEnergyManager}
impl RtActivatable<IForegroundEnergyManagerStatics> for ForegroundEnergyManager {}
impl ForegroundEnergyManager {
    #[inline] pub fn get_low_usage_level() -> Result<u32> { unsafe {
        <Self as RtActivatable<IForegroundEnergyManagerStatics>>::get_activation_factory().get_low_usage_level()
    }}
    #[inline] pub fn get_near_max_acceptable_usage_level() -> Result<u32> { unsafe {
        <Self as RtActivatable<IForegroundEnergyManagerStatics>>::get_activation_factory().get_near_max_acceptable_usage_level()
    }}
    #[inline] pub fn get_max_acceptable_usage_level() -> Result<u32> { unsafe {
        <Self as RtActivatable<IForegroundEnergyManagerStatics>>::get_activation_factory().get_max_acceptable_usage_level()
    }}
    #[inline] pub fn get_excessive_usage_level() -> Result<u32> { unsafe {
        <Self as RtActivatable<IForegroundEnergyManagerStatics>>::get_activation_factory().get_excessive_usage_level()
    }}
    #[inline] pub fn get_recent_energy_usage() -> Result<u32> { unsafe {
        <Self as RtActivatable<IForegroundEnergyManagerStatics>>::get_activation_factory().get_recent_energy_usage()
    }}
    #[inline] pub fn get_recent_energy_usage_level() -> Result<u32> { unsafe {
        <Self as RtActivatable<IForegroundEnergyManagerStatics>>::get_activation_factory().get_recent_energy_usage_level()
    }}
    #[inline] pub fn add_recent_energy_usage_increased(handler: &super::super::foundation::EventHandler<IInspectable>) -> Result<super::super::foundation::EventRegistrationToken> { unsafe {
        <Self as RtActivatable<IForegroundEnergyManagerStatics>>::get_activation_factory().add_recent_energy_usage_increased(handler)
    }}
    #[inline] pub fn remove_recent_energy_usage_increased(token: super::super::foundation::EventRegistrationToken) -> Result<()> { unsafe {
        <Self as RtActivatable<IForegroundEnergyManagerStatics>>::get_activation_factory().remove_recent_energy_usage_increased(token)
    }}
    #[inline] pub fn add_recent_energy_usage_returned_to_low(handler: &super::super::foundation::EventHandler<IInspectable>) -> Result<super::super::foundation::EventRegistrationToken> { unsafe {
        <Self as RtActivatable<IForegroundEnergyManagerStatics>>::get_activation_factory().add_recent_energy_usage_returned_to_low(handler)
    }}
    #[inline] pub fn remove_recent_energy_usage_returned_to_low(token: super::super::foundation::EventRegistrationToken) -> Result<()> { unsafe {
        <Self as RtActivatable<IForegroundEnergyManagerStatics>>::get_activation_factory().remove_recent_energy_usage_returned_to_low(token)
    }}
}
DEFINE_CLSID!(ForegroundEnergyManager(&[87,105,110,100,111,119,115,46,83,121,115,116,101,109,46,80,111,119,101,114,46,70,111,114,101,103,114,111,117,110,100,69,110,101,114,103,121,77,97,110,97,103,101,114,0]) [CLSID_ForegroundEnergyManager]);
DEFINE_IID!(IID_IForegroundEnergyManagerStatics, 2683857010, 58999, 18452, 154, 32, 83, 55, 202, 115, 43, 152);
RT_INTERFACE!{static interface IForegroundEnergyManagerStatics(IForegroundEnergyManagerStaticsVtbl): IInspectable(IInspectableVtbl) [IID_IForegroundEnergyManagerStatics] {
    fn get_LowUsageLevel(&self, out: *mut u32) -> HRESULT,
    fn get_NearMaxAcceptableUsageLevel(&self, out: *mut u32) -> HRESULT,
    fn get_MaxAcceptableUsageLevel(&self, out: *mut u32) -> HRESULT,
    fn get_ExcessiveUsageLevel(&self, out: *mut u32) -> HRESULT,
    fn get_RecentEnergyUsage(&self, out: *mut u32) -> HRESULT,
    fn get_RecentEnergyUsageLevel(&self, out: *mut u32) -> HRESULT,
    fn add_RecentEnergyUsageIncreased(&self, handler: *mut super::super::foundation::EventHandler<IInspectable>, out: *mut super::super::foundation::EventRegistrationToken) -> HRESULT,
    fn remove_RecentEnergyUsageIncreased(&self, token: super::super::foundation::EventRegistrationToken) -> HRESULT,
    fn add_RecentEnergyUsageReturnedToLow(&self, handler: *mut super::super::foundation::EventHandler<IInspectable>, out: *mut super::super::foundation::EventRegistrationToken) -> HRESULT,
    fn remove_RecentEnergyUsageReturnedToLow(&self, token: super::super::foundation::EventRegistrationToken) -> HRESULT
}}
impl IForegroundEnergyManagerStatics {
    #[inline] pub unsafe fn get_low_usage_level(&self) -> Result<u32> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_LowUsageLevel)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_near_max_acceptable_usage_level(&self) -> Result<u32> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_NearMaxAcceptableUsageLevel)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_max_acceptable_usage_level(&self) -> Result<u32> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_MaxAcceptableUsageLevel)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_excessive_usage_level(&self) -> Result<u32> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_ExcessiveUsageLevel)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_recent_energy_usage(&self) -> Result<u32> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_RecentEnergyUsage)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_recent_energy_usage_level(&self) -> Result<u32> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_RecentEnergyUsageLevel)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn add_recent_energy_usage_increased(&self, handler: &super::super::foundation::EventHandler<IInspectable>) -> Result<super::super::foundation::EventRegistrationToken> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).add_RecentEnergyUsageIncreased)(self as *const _ as *mut _, handler as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn remove_recent_energy_usage_increased(&self, token: super::super::foundation::EventRegistrationToken) -> Result<()> {
        let hr = ((*self.lpVtbl).remove_RecentEnergyUsageIncreased)(self as *const _ as *mut _, token);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn add_recent_energy_usage_returned_to_low(&self, handler: &super::super::foundation::EventHandler<IInspectable>) -> Result<super::super::foundation::EventRegistrationToken> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).add_RecentEnergyUsageReturnedToLow)(self as *const _ as *mut _, handler as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn remove_recent_energy_usage_returned_to_low(&self, token: super::super::foundation::EventRegistrationToken) -> Result<()> {
        let hr = ((*self.lpVtbl).remove_RecentEnergyUsageReturnedToLow)(self as *const _ as *mut _, token);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
}
RT_CLASS!{static class PowerManager}
impl RtActivatable<IPowerManagerStatics> for PowerManager {}
impl PowerManager {
    #[inline] pub fn get_energy_saver_status() -> Result<EnergySaverStatus> { unsafe {
        <Self as RtActivatable<IPowerManagerStatics>>::get_activation_factory().get_energy_saver_status()
    }}
    #[inline] pub fn add_energy_saver_status_changed(handler: &super::super::foundation::EventHandler<IInspectable>) -> Result<super::super::foundation::EventRegistrationToken> { unsafe {
        <Self as RtActivatable<IPowerManagerStatics>>::get_activation_factory().add_energy_saver_status_changed(handler)
    }}
    #[inline] pub fn remove_energy_saver_status_changed(token: super::super::foundation::EventRegistrationToken) -> Result<()> { unsafe {
        <Self as RtActivatable<IPowerManagerStatics>>::get_activation_factory().remove_energy_saver_status_changed(token)
    }}
    #[inline] pub fn get_battery_status() -> Result<BatteryStatus> { unsafe {
        <Self as RtActivatable<IPowerManagerStatics>>::get_activation_factory().get_battery_status()
    }}
    #[inline] pub fn add_battery_status_changed(handler: &super::super::foundation::EventHandler<IInspectable>) -> Result<super::super::foundation::EventRegistrationToken> { unsafe {
        <Self as RtActivatable<IPowerManagerStatics>>::get_activation_factory().add_battery_status_changed(handler)
    }}
    #[inline] pub fn remove_battery_status_changed(token: super::super::foundation::EventRegistrationToken) -> Result<()> { unsafe {
        <Self as RtActivatable<IPowerManagerStatics>>::get_activation_factory().remove_battery_status_changed(token)
    }}
    #[inline] pub fn get_power_supply_status() -> Result<PowerSupplyStatus> { unsafe {
        <Self as RtActivatable<IPowerManagerStatics>>::get_activation_factory().get_power_supply_status()
    }}
    #[inline] pub fn add_power_supply_status_changed(handler: &super::super::foundation::EventHandler<IInspectable>) -> Result<super::super::foundation::EventRegistrationToken> { unsafe {
        <Self as RtActivatable<IPowerManagerStatics>>::get_activation_factory().add_power_supply_status_changed(handler)
    }}
    #[inline] pub fn remove_power_supply_status_changed(token: super::super::foundation::EventRegistrationToken) -> Result<()> { unsafe {
        <Self as RtActivatable<IPowerManagerStatics>>::get_activation_factory().remove_power_supply_status_changed(token)
    }}
    #[inline] pub fn get_remaining_charge_percent() -> Result<i32> { unsafe {
        <Self as RtActivatable<IPowerManagerStatics>>::get_activation_factory().get_remaining_charge_percent()
    }}
    #[inline] pub fn add_remaining_charge_percent_changed(handler: &super::super::foundation::EventHandler<IInspectable>) -> Result<super::super::foundation::EventRegistrationToken> { unsafe {
        <Self as RtActivatable<IPowerManagerStatics>>::get_activation_factory().add_remaining_charge_percent_changed(handler)
    }}
    #[inline] pub fn remove_remaining_charge_percent_changed(token: super::super::foundation::EventRegistrationToken) -> Result<()> { unsafe {
        <Self as RtActivatable<IPowerManagerStatics>>::get_activation_factory().remove_remaining_charge_percent_changed(token)
    }}
    #[inline] pub fn get_remaining_discharge_time() -> Result<super::super::foundation::TimeSpan> { unsafe {
        <Self as RtActivatable<IPowerManagerStatics>>::get_activation_factory().get_remaining_discharge_time()
    }}
    #[inline] pub fn add_remaining_discharge_time_changed(handler: &super::super::foundation::EventHandler<IInspectable>) -> Result<super::super::foundation::EventRegistrationToken> { unsafe {
        <Self as RtActivatable<IPowerManagerStatics>>::get_activation_factory().add_remaining_discharge_time_changed(handler)
    }}
    #[inline] pub fn remove_remaining_discharge_time_changed(token: super::super::foundation::EventRegistrationToken) -> Result<()> { unsafe {
        <Self as RtActivatable<IPowerManagerStatics>>::get_activation_factory().remove_remaining_discharge_time_changed(token)
    }}
}
DEFINE_CLSID!(PowerManager(&[87,105,110,100,111,119,115,46,83,121,115,116,101,109,46,80,111,119,101,114,46,80,111,119,101,114,77,97,110,97,103,101,114,0]) [CLSID_PowerManager]);
DEFINE_IID!(IID_IPowerManagerStatics, 328499805, 25294, 17252, 152, 213, 170, 40, 199, 251, 209, 91);
RT_INTERFACE!{static interface IPowerManagerStatics(IPowerManagerStaticsVtbl): IInspectable(IInspectableVtbl) [IID_IPowerManagerStatics] {
    fn get_EnergySaverStatus(&self, out: *mut EnergySaverStatus) -> HRESULT,
    fn add_EnergySaverStatusChanged(&self, handler: *mut super::super::foundation::EventHandler<IInspectable>, out: *mut super::super::foundation::EventRegistrationToken) -> HRESULT,
    fn remove_EnergySaverStatusChanged(&self, token: super::super::foundation::EventRegistrationToken) -> HRESULT,
    fn get_BatteryStatus(&self, out: *mut BatteryStatus) -> HRESULT,
    fn add_BatteryStatusChanged(&self, handler: *mut super::super::foundation::EventHandler<IInspectable>, out: *mut super::super::foundation::EventRegistrationToken) -> HRESULT,
    fn remove_BatteryStatusChanged(&self, token: super::super::foundation::EventRegistrationToken) -> HRESULT,
    fn get_PowerSupplyStatus(&self, out: *mut PowerSupplyStatus) -> HRESULT,
    fn add_PowerSupplyStatusChanged(&self, handler: *mut super::super::foundation::EventHandler<IInspectable>, out: *mut super::super::foundation::EventRegistrationToken) -> HRESULT,
    fn remove_PowerSupplyStatusChanged(&self, token: super::super::foundation::EventRegistrationToken) -> HRESULT,
    fn get_RemainingChargePercent(&self, out: *mut i32) -> HRESULT,
    fn add_RemainingChargePercentChanged(&self, handler: *mut super::super::foundation::EventHandler<IInspectable>, out: *mut super::super::foundation::EventRegistrationToken) -> HRESULT,
    fn remove_RemainingChargePercentChanged(&self, token: super::super::foundation::EventRegistrationToken) -> HRESULT,
    fn get_RemainingDischargeTime(&self, out: *mut super::super::foundation::TimeSpan) -> HRESULT,
    fn add_RemainingDischargeTimeChanged(&self, handler: *mut super::super::foundation::EventHandler<IInspectable>, out: *mut super::super::foundation::EventRegistrationToken) -> HRESULT,
    fn remove_RemainingDischargeTimeChanged(&self, token: super::super::foundation::EventRegistrationToken) -> HRESULT
}}
impl IPowerManagerStatics {
    #[inline] pub unsafe fn get_energy_saver_status(&self) -> Result<EnergySaverStatus> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_EnergySaverStatus)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn add_energy_saver_status_changed(&self, handler: &super::super::foundation::EventHandler<IInspectable>) -> Result<super::super::foundation::EventRegistrationToken> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).add_EnergySaverStatusChanged)(self as *const _ as *mut _, handler as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn remove_energy_saver_status_changed(&self, token: super::super::foundation::EventRegistrationToken) -> Result<()> {
        let hr = ((*self.lpVtbl).remove_EnergySaverStatusChanged)(self as *const _ as *mut _, token);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_battery_status(&self) -> Result<BatteryStatus> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_BatteryStatus)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn add_battery_status_changed(&self, handler: &super::super::foundation::EventHandler<IInspectable>) -> Result<super::super::foundation::EventRegistrationToken> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).add_BatteryStatusChanged)(self as *const _ as *mut _, handler as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn remove_battery_status_changed(&self, token: super::super::foundation::EventRegistrationToken) -> Result<()> {
        let hr = ((*self.lpVtbl).remove_BatteryStatusChanged)(self as *const _ as *mut _, token);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_power_supply_status(&self) -> Result<PowerSupplyStatus> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_PowerSupplyStatus)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn add_power_supply_status_changed(&self, handler: &super::super::foundation::EventHandler<IInspectable>) -> Result<super::super::foundation::EventRegistrationToken> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).add_PowerSupplyStatusChanged)(self as *const _ as *mut _, handler as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn remove_power_supply_status_changed(&self, token: super::super::foundation::EventRegistrationToken) -> Result<()> {
        let hr = ((*self.lpVtbl).remove_PowerSupplyStatusChanged)(self as *const _ as *mut _, token);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_remaining_charge_percent(&self) -> Result<i32> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_RemainingChargePercent)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn add_remaining_charge_percent_changed(&self, handler: &super::super::foundation::EventHandler<IInspectable>) -> Result<super::super::foundation::EventRegistrationToken> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).add_RemainingChargePercentChanged)(self as *const _ as *mut _, handler as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn remove_remaining_charge_percent_changed(&self, token: super::super::foundation::EventRegistrationToken) -> Result<()> {
        let hr = ((*self.lpVtbl).remove_RemainingChargePercentChanged)(self as *const _ as *mut _, token);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_remaining_discharge_time(&self) -> Result<super::super::foundation::TimeSpan> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_RemainingDischargeTime)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn add_remaining_discharge_time_changed(&self, handler: &super::super::foundation::EventHandler<IInspectable>) -> Result<super::super::foundation::EventRegistrationToken> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).add_RemainingDischargeTimeChanged)(self as *const _ as *mut _, handler as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn remove_remaining_discharge_time_changed(&self, token: super::super::foundation::EventRegistrationToken) -> Result<()> {
        let hr = ((*self.lpVtbl).remove_RemainingDischargeTimeChanged)(self as *const _ as *mut _, token);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
}
RT_ENUM! { enum PowerSupplyStatus: i32 {
    NotPresent (PowerSupplyStatus_NotPresent) = 0, Inadequate (PowerSupplyStatus_Inadequate) = 1, Adequate (PowerSupplyStatus_Adequate) = 2,
}}
pub mod diagnostics { // Windows.System.Power.Diagnostics
use ::prelude::*;
RT_CLASS!{static class BackgroundEnergyDiagnostics}
impl RtActivatable<IBackgroundEnergyDiagnosticsStatics> for BackgroundEnergyDiagnostics {}
impl BackgroundEnergyDiagnostics {
    #[inline] pub fn get_device_specific_conversion_factor() -> Result<f64> { unsafe {
        <Self as RtActivatable<IBackgroundEnergyDiagnosticsStatics>>::get_activation_factory().get_device_specific_conversion_factor()
    }}
    #[inline] pub fn compute_total_energy_usage() -> Result<u64> { unsafe {
        <Self as RtActivatable<IBackgroundEnergyDiagnosticsStatics>>::get_activation_factory().compute_total_energy_usage()
    }}
    #[inline] pub fn reset_total_energy_usage() -> Result<()> { unsafe {
        <Self as RtActivatable<IBackgroundEnergyDiagnosticsStatics>>::get_activation_factory().reset_total_energy_usage()
    }}
}
DEFINE_CLSID!(BackgroundEnergyDiagnostics(&[87,105,110,100,111,119,115,46,83,121,115,116,101,109,46,80,111,119,101,114,46,68,105,97,103,110,111,115,116,105,99,115,46,66,97,99,107,103,114,111,117,110,100,69,110,101,114,103,121,68,105,97,103,110,111,115,116,105,99,115,0]) [CLSID_BackgroundEnergyDiagnostics]);
DEFINE_IID!(IID_IBackgroundEnergyDiagnosticsStatics, 3613800194, 54182, 18144, 143, 155, 80, 185, 91, 180, 249, 197);
RT_INTERFACE!{static interface IBackgroundEnergyDiagnosticsStatics(IBackgroundEnergyDiagnosticsStaticsVtbl): IInspectable(IInspectableVtbl) [IID_IBackgroundEnergyDiagnosticsStatics] {
    fn get_DeviceSpecificConversionFactor(&self, out: *mut f64) -> HRESULT,
    fn ComputeTotalEnergyUsage(&self, out: *mut u64) -> HRESULT,
    fn ResetTotalEnergyUsage(&self) -> HRESULT
}}
impl IBackgroundEnergyDiagnosticsStatics {
    #[inline] pub unsafe fn get_device_specific_conversion_factor(&self) -> Result<f64> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_DeviceSpecificConversionFactor)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn compute_total_energy_usage(&self) -> Result<u64> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).ComputeTotalEnergyUsage)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn reset_total_energy_usage(&self) -> Result<()> {
        let hr = ((*self.lpVtbl).ResetTotalEnergyUsage)(self as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
}
RT_CLASS!{static class ForegroundEnergyDiagnostics}
impl RtActivatable<IForegroundEnergyDiagnosticsStatics> for ForegroundEnergyDiagnostics {}
impl ForegroundEnergyDiagnostics {
    #[inline] pub fn get_device_specific_conversion_factor() -> Result<f64> { unsafe {
        <Self as RtActivatable<IForegroundEnergyDiagnosticsStatics>>::get_activation_factory().get_device_specific_conversion_factor()
    }}
    #[inline] pub fn compute_total_energy_usage() -> Result<u64> { unsafe {
        <Self as RtActivatable<IForegroundEnergyDiagnosticsStatics>>::get_activation_factory().compute_total_energy_usage()
    }}
    #[inline] pub fn reset_total_energy_usage() -> Result<()> { unsafe {
        <Self as RtActivatable<IForegroundEnergyDiagnosticsStatics>>::get_activation_factory().reset_total_energy_usage()
    }}
}
DEFINE_CLSID!(ForegroundEnergyDiagnostics(&[87,105,110,100,111,119,115,46,83,121,115,116,101,109,46,80,111,119,101,114,46,68,105,97,103,110,111,115,116,105,99,115,46,70,111,114,101,103,114,111,117,110,100,69,110,101,114,103,121,68,105,97,103,110,111,115,116,105,99,115,0]) [CLSID_ForegroundEnergyDiagnostics]);
DEFINE_IID!(IID_IForegroundEnergyDiagnosticsStatics, 600443159, 52487, 17929, 190, 21, 143, 232, 148, 197, 228, 30);
RT_INTERFACE!{static interface IForegroundEnergyDiagnosticsStatics(IForegroundEnergyDiagnosticsStaticsVtbl): IInspectable(IInspectableVtbl) [IID_IForegroundEnergyDiagnosticsStatics] {
    fn get_DeviceSpecificConversionFactor(&self, out: *mut f64) -> HRESULT,
    fn ComputeTotalEnergyUsage(&self, out: *mut u64) -> HRESULT,
    fn ResetTotalEnergyUsage(&self) -> HRESULT
}}
impl IForegroundEnergyDiagnosticsStatics {
    #[inline] pub unsafe fn get_device_specific_conversion_factor(&self) -> Result<f64> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_DeviceSpecificConversionFactor)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn compute_total_energy_usage(&self) -> Result<u64> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).ComputeTotalEnergyUsage)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn reset_total_energy_usage(&self) -> Result<()> {
        let hr = ((*self.lpVtbl).ResetTotalEnergyUsage)(self as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
}
} // Windows.System.Power.Diagnostics
} // Windows.System.Power
pub mod diagnostics { // Windows.System.Diagnostics
use ::prelude::*;
DEFINE_IID!(IID_IDiagnosticActionResult, 3261440662, 59195, 16535, 178, 143, 52, 66, 240, 61, 216, 49);
RT_INTERFACE!{interface IDiagnosticActionResult(IDiagnosticActionResultVtbl): IInspectable(IInspectableVtbl) [IID_IDiagnosticActionResult] {
    fn get_ExtendedError(&self, out: *mut super::super::foundation::HResult) -> HRESULT,
    fn get_Results(&self, out: *mut *mut super::super::foundation::collections::ValueSet) -> HRESULT
}}
impl IDiagnosticActionResult {
    #[inline] pub unsafe fn get_extended_error(&self) -> Result<super::super::foundation::HResult> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_ExtendedError)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_results(&self) -> Result<ComPtr<super::super::foundation::collections::ValueSet>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Results)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
RT_CLASS!{class DiagnosticActionResult: IDiagnosticActionResult}
RT_ENUM! { enum DiagnosticActionState: i32 {
    Initializing (DiagnosticActionState_Initializing) = 0, Downloading (DiagnosticActionState_Downloading) = 1, VerifyingTrust (DiagnosticActionState_VerifyingTrust) = 2, Detecting (DiagnosticActionState_Detecting) = 3, Resolving (DiagnosticActionState_Resolving) = 4, VerifyingResolution (DiagnosticActionState_VerifyingResolution) = 5,
}}
DEFINE_IID!(IID_IDiagnosticInvoker, 410724106, 739, 20358, 132, 252, 253, 216, 146, 181, 148, 15);
RT_INTERFACE!{interface IDiagnosticInvoker(IDiagnosticInvokerVtbl): IInspectable(IInspectableVtbl) [IID_IDiagnosticInvoker] {
    #[cfg(feature="windows-data")] fn RunDiagnosticActionAsync(&self, context: *mut super::super::data::json::JsonObject, out: *mut *mut super::super::foundation::IAsyncOperationWithProgress<DiagnosticActionResult, DiagnosticActionState>) -> HRESULT
}}
impl IDiagnosticInvoker {
    #[cfg(feature="windows-data")] #[inline] pub unsafe fn run_diagnostic_action_async(&self, context: &super::super::data::json::JsonObject) -> Result<ComPtr<super::super::foundation::IAsyncOperationWithProgress<DiagnosticActionResult, DiagnosticActionState>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).RunDiagnosticActionAsync)(self as *const _ as *mut _, context as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
RT_CLASS!{class DiagnosticInvoker: IDiagnosticInvoker}
impl RtActivatable<IDiagnosticInvokerStatics> for DiagnosticInvoker {}
impl DiagnosticInvoker {
    #[inline] pub fn get_default() -> Result<ComPtr<DiagnosticInvoker>> { unsafe {
        <Self as RtActivatable<IDiagnosticInvokerStatics>>::get_activation_factory().get_default()
    }}
    #[inline] pub fn get_for_user(user: &super::User) -> Result<ComPtr<DiagnosticInvoker>> { unsafe {
        <Self as RtActivatable<IDiagnosticInvokerStatics>>::get_activation_factory().get_for_user(user)
    }}
    #[inline] pub fn get_is_supported() -> Result<bool> { unsafe {
        <Self as RtActivatable<IDiagnosticInvokerStatics>>::get_activation_factory().get_is_supported()
    }}
}
DEFINE_CLSID!(DiagnosticInvoker(&[87,105,110,100,111,119,115,46,83,121,115,116,101,109,46,68,105,97,103,110,111,115,116,105,99,115,46,68,105,97,103,110,111,115,116,105,99,73,110,118,111,107,101,114,0]) [CLSID_DiagnosticInvoker]);
DEFINE_IID!(IID_IDiagnosticInvokerStatics, 1559943390, 61788, 17748, 168, 19, 193, 19, 195, 136, 27, 9);
RT_INTERFACE!{static interface IDiagnosticInvokerStatics(IDiagnosticInvokerStaticsVtbl): IInspectable(IInspectableVtbl) [IID_IDiagnosticInvokerStatics] {
    fn GetDefault(&self, out: *mut *mut DiagnosticInvoker) -> HRESULT,
    fn GetForUser(&self, user: *mut super::User, out: *mut *mut DiagnosticInvoker) -> HRESULT,
    fn get_IsSupported(&self, out: *mut bool) -> HRESULT
}}
impl IDiagnosticInvokerStatics {
    #[inline] pub unsafe fn get_default(&self) -> Result<ComPtr<DiagnosticInvoker>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetDefault)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_for_user(&self, user: &super::User) -> Result<ComPtr<DiagnosticInvoker>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetForUser)(self as *const _ as *mut _, user as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_is_supported(&self) -> Result<bool> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_IsSupported)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
}
DEFINE_IID!(IID_IProcessCpuUsage, 196813938, 51391, 16954, 168, 16, 181, 89, 174, 67, 84, 226);
RT_INTERFACE!{interface IProcessCpuUsage(IProcessCpuUsageVtbl): IInspectable(IInspectableVtbl) [IID_IProcessCpuUsage] {
    fn GetReport(&self, out: *mut *mut ProcessCpuUsageReport) -> HRESULT
}}
impl IProcessCpuUsage {
    #[inline] pub unsafe fn get_report(&self) -> Result<ComPtr<ProcessCpuUsageReport>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetReport)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
RT_CLASS!{class ProcessCpuUsage: IProcessCpuUsage}
DEFINE_IID!(IID_IProcessCpuUsageReport, 2322439340, 14727, 20015, 161, 25, 107, 95, 162, 20, 241, 180);
RT_INTERFACE!{interface IProcessCpuUsageReport(IProcessCpuUsageReportVtbl): IInspectable(IInspectableVtbl) [IID_IProcessCpuUsageReport] {
    fn get_KernelTime(&self, out: *mut super::super::foundation::TimeSpan) -> HRESULT,
    fn get_UserTime(&self, out: *mut super::super::foundation::TimeSpan) -> HRESULT
}}
impl IProcessCpuUsageReport {
    #[inline] pub unsafe fn get_kernel_time(&self) -> Result<super::super::foundation::TimeSpan> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_KernelTime)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_user_time(&self) -> Result<super::super::foundation::TimeSpan> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_UserTime)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
}
RT_CLASS!{class ProcessCpuUsageReport: IProcessCpuUsageReport}
DEFINE_IID!(IID_IProcessDiagnosticInfo, 3895504971, 12302, 20198, 160, 171, 91, 95, 82, 49, 180, 52);
RT_INTERFACE!{interface IProcessDiagnosticInfo(IProcessDiagnosticInfoVtbl): IInspectable(IInspectableVtbl) [IID_IProcessDiagnosticInfo] {
    fn get_ProcessId(&self, out: *mut u32) -> HRESULT,
    fn get_ExecutableFileName(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Parent(&self, out: *mut *mut ProcessDiagnosticInfo) -> HRESULT,
    fn get_ProcessStartTime(&self, out: *mut super::super::foundation::DateTime) -> HRESULT,
    fn get_DiskUsage(&self, out: *mut *mut ProcessDiskUsage) -> HRESULT,
    fn get_MemoryUsage(&self, out: *mut *mut ProcessMemoryUsage) -> HRESULT,
    fn get_CpuUsage(&self, out: *mut *mut ProcessCpuUsage) -> HRESULT
}}
impl IProcessDiagnosticInfo {
    #[inline] pub unsafe fn get_process_id(&self) -> Result<u32> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_ProcessId)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_executable_file_name(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_ExecutableFileName)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_parent(&self) -> Result<ComPtr<ProcessDiagnosticInfo>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Parent)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_process_start_time(&self) -> Result<super::super::foundation::DateTime> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_ProcessStartTime)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_disk_usage(&self) -> Result<ComPtr<ProcessDiskUsage>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_DiskUsage)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_memory_usage(&self) -> Result<ComPtr<ProcessMemoryUsage>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_MemoryUsage)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_cpu_usage(&self) -> Result<ComPtr<ProcessCpuUsage>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_CpuUsage)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
RT_CLASS!{class ProcessDiagnosticInfo: IProcessDiagnosticInfo}
impl RtActivatable<IProcessDiagnosticInfoStatics> for ProcessDiagnosticInfo {}
impl RtActivatable<IProcessDiagnosticInfoStatics2> for ProcessDiagnosticInfo {}
impl ProcessDiagnosticInfo {
    #[inline] pub fn get_for_processes() -> Result<ComPtr<super::super::foundation::collections::IVectorView<ProcessDiagnosticInfo>>> { unsafe {
        <Self as RtActivatable<IProcessDiagnosticInfoStatics>>::get_activation_factory().get_for_processes()
    }}
    #[inline] pub fn get_for_current_process() -> Result<ComPtr<ProcessDiagnosticInfo>> { unsafe {
        <Self as RtActivatable<IProcessDiagnosticInfoStatics>>::get_activation_factory().get_for_current_process()
    }}
    #[inline] pub fn try_get_for_process_id(processId: u32) -> Result<ComPtr<ProcessDiagnosticInfo>> { unsafe {
        <Self as RtActivatable<IProcessDiagnosticInfoStatics2>>::get_activation_factory().try_get_for_process_id(processId)
    }}
}
DEFINE_CLSID!(ProcessDiagnosticInfo(&[87,105,110,100,111,119,115,46,83,121,115,116,101,109,46,68,105,97,103,110,111,115,116,105,99,115,46,80,114,111,99,101,115,115,68,105,97,103,110,111,115,116,105,99,73,110,102,111,0]) [CLSID_ProcessDiagnosticInfo]);
DEFINE_IID!(IID_IProcessDiagnosticInfo2, 2505624346, 15627, 18924, 171, 112, 79, 122, 17, 40, 5, 222);
RT_INTERFACE!{interface IProcessDiagnosticInfo2(IProcessDiagnosticInfo2Vtbl): IInspectable(IInspectableVtbl) [IID_IProcessDiagnosticInfo2] {
    fn GetAppDiagnosticInfos(&self, out: *mut *mut super::super::foundation::collections::IVector<super::AppDiagnosticInfo>) -> HRESULT,
    fn get_IsPackaged(&self, out: *mut bool) -> HRESULT
}}
impl IProcessDiagnosticInfo2 {
    #[inline] pub unsafe fn get_app_diagnostic_infos(&self) -> Result<ComPtr<super::super::foundation::collections::IVector<super::AppDiagnosticInfo>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetAppDiagnosticInfos)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_is_packaged(&self) -> Result<bool> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_IsPackaged)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
}
DEFINE_IID!(IID_IProcessDiagnosticInfoStatics, 792834656, 46239, 17036, 170, 14, 132, 116, 79, 73, 202, 149);
RT_INTERFACE!{static interface IProcessDiagnosticInfoStatics(IProcessDiagnosticInfoStaticsVtbl): IInspectable(IInspectableVtbl) [IID_IProcessDiagnosticInfoStatics] {
    fn GetForProcesses(&self, out: *mut *mut super::super::foundation::collections::IVectorView<ProcessDiagnosticInfo>) -> HRESULT,
    fn GetForCurrentProcess(&self, out: *mut *mut ProcessDiagnosticInfo) -> HRESULT
}}
impl IProcessDiagnosticInfoStatics {
    #[inline] pub unsafe fn get_for_processes(&self) -> Result<ComPtr<super::super::foundation::collections::IVectorView<ProcessDiagnosticInfo>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetForProcesses)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_for_current_process(&self) -> Result<ComPtr<ProcessDiagnosticInfo>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetForCurrentProcess)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
DEFINE_IID!(IID_IProcessDiagnosticInfoStatics2, 1250334871, 39065, 19012, 162, 155, 9, 22, 99, 190, 9, 182);
RT_INTERFACE!{static interface IProcessDiagnosticInfoStatics2(IProcessDiagnosticInfoStatics2Vtbl): IInspectable(IInspectableVtbl) [IID_IProcessDiagnosticInfoStatics2] {
    fn TryGetForProcessId(&self, processId: u32, out: *mut *mut ProcessDiagnosticInfo) -> HRESULT
}}
impl IProcessDiagnosticInfoStatics2 {
    #[inline] pub unsafe fn try_get_for_process_id(&self, processId: u32) -> Result<ComPtr<ProcessDiagnosticInfo>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).TryGetForProcessId)(self as *const _ as *mut _, processId, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
DEFINE_IID!(IID_IProcessDiskUsage, 1524075517, 32337, 20051, 191, 170, 90, 110, 225, 170, 187, 248);
RT_INTERFACE!{interface IProcessDiskUsage(IProcessDiskUsageVtbl): IInspectable(IInspectableVtbl) [IID_IProcessDiskUsage] {
    fn GetReport(&self, out: *mut *mut ProcessDiskUsageReport) -> HRESULT
}}
impl IProcessDiskUsage {
    #[inline] pub unsafe fn get_report(&self) -> Result<ComPtr<ProcessDiskUsageReport>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetReport)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
RT_CLASS!{class ProcessDiskUsage: IProcessDiskUsage}
DEFINE_IID!(IID_IProcessDiskUsageReport, 1075193853, 21341, 19487, 129, 184, 218, 84, 225, 190, 99, 94);
RT_INTERFACE!{interface IProcessDiskUsageReport(IProcessDiskUsageReportVtbl): IInspectable(IInspectableVtbl) [IID_IProcessDiskUsageReport] {
    fn get_ReadOperationCount(&self, out: *mut i64) -> HRESULT,
    fn get_WriteOperationCount(&self, out: *mut i64) -> HRESULT,
    fn get_OtherOperationCount(&self, out: *mut i64) -> HRESULT,
    fn get_BytesReadCount(&self, out: *mut i64) -> HRESULT,
    fn get_BytesWrittenCount(&self, out: *mut i64) -> HRESULT,
    fn get_OtherBytesCount(&self, out: *mut i64) -> HRESULT
}}
impl IProcessDiskUsageReport {
    #[inline] pub unsafe fn get_read_operation_count(&self) -> Result<i64> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_ReadOperationCount)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_write_operation_count(&self) -> Result<i64> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_WriteOperationCount)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_other_operation_count(&self) -> Result<i64> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_OtherOperationCount)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_bytes_read_count(&self) -> Result<i64> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_BytesReadCount)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_bytes_written_count(&self) -> Result<i64> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_BytesWrittenCount)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_other_bytes_count(&self) -> Result<i64> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_OtherBytesCount)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
}
RT_CLASS!{class ProcessDiskUsageReport: IProcessDiskUsageReport}
DEFINE_IID!(IID_IProcessMemoryUsage, 4111147675, 33404, 17079, 176, 124, 14, 50, 98, 126, 107, 62);
RT_INTERFACE!{interface IProcessMemoryUsage(IProcessMemoryUsageVtbl): IInspectable(IInspectableVtbl) [IID_IProcessMemoryUsage] {
    fn GetReport(&self, out: *mut *mut ProcessMemoryUsageReport) -> HRESULT
}}
impl IProcessMemoryUsage {
    #[inline] pub unsafe fn get_report(&self) -> Result<ComPtr<ProcessMemoryUsageReport>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetReport)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
RT_CLASS!{class ProcessMemoryUsage: IProcessMemoryUsage}
DEFINE_IID!(IID_IProcessMemoryUsageReport, 3267853498, 6481, 18053, 133, 50, 126, 116, 158, 207, 142, 235);
RT_INTERFACE!{interface IProcessMemoryUsageReport(IProcessMemoryUsageReportVtbl): IInspectable(IInspectableVtbl) [IID_IProcessMemoryUsageReport] {
    fn get_NonPagedPoolSizeInBytes(&self, out: *mut u64) -> HRESULT,
    fn get_PageFaultCount(&self, out: *mut u32) -> HRESULT,
    fn get_PageFileSizeInBytes(&self, out: *mut u64) -> HRESULT,
    fn get_PagedPoolSizeInBytes(&self, out: *mut u64) -> HRESULT,
    fn get_PeakNonPagedPoolSizeInBytes(&self, out: *mut u64) -> HRESULT,
    fn get_PeakPageFileSizeInBytes(&self, out: *mut u64) -> HRESULT,
    fn get_PeakPagedPoolSizeInBytes(&self, out: *mut u64) -> HRESULT,
    fn get_PeakVirtualMemorySizeInBytes(&self, out: *mut u64) -> HRESULT,
    fn get_PeakWorkingSetSizeInBytes(&self, out: *mut u64) -> HRESULT,
    fn get_PrivatePageCount(&self, out: *mut u64) -> HRESULT,
    fn get_VirtualMemorySizeInBytes(&self, out: *mut u64) -> HRESULT,
    fn get_WorkingSetSizeInBytes(&self, out: *mut u64) -> HRESULT
}}
impl IProcessMemoryUsageReport {
    #[inline] pub unsafe fn get_non_paged_pool_size_in_bytes(&self) -> Result<u64> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_NonPagedPoolSizeInBytes)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_page_fault_count(&self) -> Result<u32> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_PageFaultCount)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_page_file_size_in_bytes(&self) -> Result<u64> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_PageFileSizeInBytes)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_paged_pool_size_in_bytes(&self) -> Result<u64> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_PagedPoolSizeInBytes)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_peak_non_paged_pool_size_in_bytes(&self) -> Result<u64> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_PeakNonPagedPoolSizeInBytes)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_peak_page_file_size_in_bytes(&self) -> Result<u64> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_PeakPageFileSizeInBytes)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_peak_paged_pool_size_in_bytes(&self) -> Result<u64> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_PeakPagedPoolSizeInBytes)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_peak_virtual_memory_size_in_bytes(&self) -> Result<u64> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_PeakVirtualMemorySizeInBytes)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_peak_working_set_size_in_bytes(&self) -> Result<u64> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_PeakWorkingSetSizeInBytes)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_private_page_count(&self) -> Result<u64> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_PrivatePageCount)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_virtual_memory_size_in_bytes(&self) -> Result<u64> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_VirtualMemorySizeInBytes)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_working_set_size_in_bytes(&self) -> Result<u64> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_WorkingSetSizeInBytes)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
}
RT_CLASS!{class ProcessMemoryUsageReport: IProcessMemoryUsageReport}
DEFINE_IID!(IID_ISystemCpuUsage, 1614263212, 726, 16948, 131, 98, 127, 227, 173, 200, 31, 95);
RT_INTERFACE!{interface ISystemCpuUsage(ISystemCpuUsageVtbl): IInspectable(IInspectableVtbl) [IID_ISystemCpuUsage] {
    fn GetReport(&self, out: *mut *mut SystemCpuUsageReport) -> HRESULT
}}
impl ISystemCpuUsage {
    #[inline] pub unsafe fn get_report(&self) -> Result<ComPtr<SystemCpuUsageReport>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetReport)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
RT_CLASS!{class SystemCpuUsage: ISystemCpuUsage}
DEFINE_IID!(IID_ISystemCpuUsageReport, 740741298, 38019, 20322, 171, 87, 130, 178, 157, 151, 25, 184);
RT_INTERFACE!{interface ISystemCpuUsageReport(ISystemCpuUsageReportVtbl): IInspectable(IInspectableVtbl) [IID_ISystemCpuUsageReport] {
    fn get_KernelTime(&self, out: *mut super::super::foundation::TimeSpan) -> HRESULT,
    fn get_UserTime(&self, out: *mut super::super::foundation::TimeSpan) -> HRESULT,
    fn get_IdleTime(&self, out: *mut super::super::foundation::TimeSpan) -> HRESULT
}}
impl ISystemCpuUsageReport {
    #[inline] pub unsafe fn get_kernel_time(&self) -> Result<super::super::foundation::TimeSpan> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_KernelTime)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_user_time(&self) -> Result<super::super::foundation::TimeSpan> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_UserTime)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_idle_time(&self) -> Result<super::super::foundation::TimeSpan> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_IdleTime)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
}
RT_CLASS!{class SystemCpuUsageReport: ISystemCpuUsageReport}
DEFINE_IID!(IID_ISystemDiagnosticInfo, 2727411205, 57331, 16511, 154, 27, 11, 43, 49, 124, 168, 0);
RT_INTERFACE!{interface ISystemDiagnosticInfo(ISystemDiagnosticInfoVtbl): IInspectable(IInspectableVtbl) [IID_ISystemDiagnosticInfo] {
    fn get_MemoryUsage(&self, out: *mut *mut SystemMemoryUsage) -> HRESULT,
    fn get_CpuUsage(&self, out: *mut *mut SystemCpuUsage) -> HRESULT
}}
impl ISystemDiagnosticInfo {
    #[inline] pub unsafe fn get_memory_usage(&self) -> Result<ComPtr<SystemMemoryUsage>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_MemoryUsage)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_cpu_usage(&self) -> Result<ComPtr<SystemCpuUsage>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_CpuUsage)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
RT_CLASS!{class SystemDiagnosticInfo: ISystemDiagnosticInfo}
impl RtActivatable<ISystemDiagnosticInfoStatics> for SystemDiagnosticInfo {}
impl SystemDiagnosticInfo {
    #[inline] pub fn get_for_current_system() -> Result<ComPtr<SystemDiagnosticInfo>> { unsafe {
        <Self as RtActivatable<ISystemDiagnosticInfoStatics>>::get_activation_factory().get_for_current_system()
    }}
}
DEFINE_CLSID!(SystemDiagnosticInfo(&[87,105,110,100,111,119,115,46,83,121,115,116,101,109,46,68,105,97,103,110,111,115,116,105,99,115,46,83,121,115,116,101,109,68,105,97,103,110,111,115,116,105,99,73,110,102,111,0]) [CLSID_SystemDiagnosticInfo]);
DEFINE_IID!(IID_ISystemDiagnosticInfoStatics, 3557076001, 64637, 17904, 154, 63, 57, 32, 58, 237, 159, 126);
RT_INTERFACE!{static interface ISystemDiagnosticInfoStatics(ISystemDiagnosticInfoStaticsVtbl): IInspectable(IInspectableVtbl) [IID_ISystemDiagnosticInfoStatics] {
    fn GetForCurrentSystem(&self, out: *mut *mut SystemDiagnosticInfo) -> HRESULT
}}
impl ISystemDiagnosticInfoStatics {
    #[inline] pub unsafe fn get_for_current_system(&self) -> Result<ComPtr<SystemDiagnosticInfo>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetForCurrentSystem)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
DEFINE_IID!(IID_ISystemMemoryUsage, 402638229, 5890, 18895, 170, 39, 47, 10, 50, 89, 20, 4);
RT_INTERFACE!{interface ISystemMemoryUsage(ISystemMemoryUsageVtbl): IInspectable(IInspectableVtbl) [IID_ISystemMemoryUsage] {
    fn GetReport(&self, out: *mut *mut SystemMemoryUsageReport) -> HRESULT
}}
impl ISystemMemoryUsage {
    #[inline] pub unsafe fn get_report(&self) -> Result<ComPtr<SystemMemoryUsageReport>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetReport)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
RT_CLASS!{class SystemMemoryUsage: ISystemMemoryUsage}
DEFINE_IID!(IID_ISystemMemoryUsageReport, 946224263, 10911, 16442, 189, 25, 44, 243, 232, 22, 149, 0);
RT_INTERFACE!{interface ISystemMemoryUsageReport(ISystemMemoryUsageReportVtbl): IInspectable(IInspectableVtbl) [IID_ISystemMemoryUsageReport] {
    fn get_TotalPhysicalSizeInBytes(&self, out: *mut u64) -> HRESULT,
    fn get_AvailableSizeInBytes(&self, out: *mut u64) -> HRESULT,
    fn get_CommittedSizeInBytes(&self, out: *mut u64) -> HRESULT
}}
impl ISystemMemoryUsageReport {
    #[inline] pub unsafe fn get_total_physical_size_in_bytes(&self) -> Result<u64> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_TotalPhysicalSizeInBytes)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_available_size_in_bytes(&self) -> Result<u64> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_AvailableSizeInBytes)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_committed_size_in_bytes(&self) -> Result<u64> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_CommittedSizeInBytes)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
}
RT_CLASS!{class SystemMemoryUsageReport: ISystemMemoryUsageReport}
pub mod tracereporting { // Windows.System.Diagnostics.TraceReporting
use ::prelude::*;
RT_CLASS!{static class PlatformDiagnosticActions}
impl RtActivatable<IPlatformDiagnosticActionsStatics> for PlatformDiagnosticActions {}
impl PlatformDiagnosticActions {
    #[inline] pub fn is_scenario_enabled(scenarioId: Guid) -> Result<bool> { unsafe {
        <Self as RtActivatable<IPlatformDiagnosticActionsStatics>>::get_activation_factory().is_scenario_enabled(scenarioId)
    }}
    #[inline] pub fn try_escalate_scenario(scenarioId: Guid, escalationType: PlatformDiagnosticEscalationType, outputDirectory: &HStringArg, timestampOutputDirectory: bool, forceEscalationUpload: bool, triggers: &::rt::gen::windows::foundation::collections::IMapView<HString, HString>) -> Result<bool> { unsafe {
        <Self as RtActivatable<IPlatformDiagnosticActionsStatics>>::get_activation_factory().try_escalate_scenario(scenarioId, escalationType, outputDirectory, timestampOutputDirectory, forceEscalationUpload, triggers)
    }}
    #[inline] pub fn download_latest_settings_for_namespace(partner: &HStringArg, feature: &HStringArg, isScenarioNamespace: bool, downloadOverCostedNetwork: bool, downloadOverBattery: bool) -> Result<PlatformDiagnosticActionState> { unsafe {
        <Self as RtActivatable<IPlatformDiagnosticActionsStatics>>::get_activation_factory().download_latest_settings_for_namespace(partner, feature, isScenarioNamespace, downloadOverCostedNetwork, downloadOverBattery)
    }}
    #[inline] pub fn get_active_scenario_list() -> Result<ComPtr<::rt::gen::windows::foundation::collections::IVectorView<Guid>>> { unsafe {
        <Self as RtActivatable<IPlatformDiagnosticActionsStatics>>::get_activation_factory().get_active_scenario_list()
    }}
    #[inline] pub fn force_upload(latency: PlatformDiagnosticEventBufferLatencies, uploadOverCostedNetwork: bool, uploadOverBattery: bool) -> Result<PlatformDiagnosticActionState> { unsafe {
        <Self as RtActivatable<IPlatformDiagnosticActionsStatics>>::get_activation_factory().force_upload(latency, uploadOverCostedNetwork, uploadOverBattery)
    }}
    #[inline] pub fn is_trace_running(slotType: PlatformDiagnosticTraceSlotType, scenarioId: Guid, traceProfileHash: u64) -> Result<PlatformDiagnosticTraceSlotState> { unsafe {
        <Self as RtActivatable<IPlatformDiagnosticActionsStatics>>::get_activation_factory().is_trace_running(slotType, scenarioId, traceProfileHash)
    }}
    #[inline] pub fn get_active_trace_runtime(slotType: PlatformDiagnosticTraceSlotType) -> Result<ComPtr<PlatformDiagnosticTraceRuntimeInfo>> { unsafe {
        <Self as RtActivatable<IPlatformDiagnosticActionsStatics>>::get_activation_factory().get_active_trace_runtime(slotType)
    }}
    #[inline] pub fn get_known_trace_list(slotType: PlatformDiagnosticTraceSlotType) -> Result<ComPtr<::rt::gen::windows::foundation::collections::IVectorView<PlatformDiagnosticTraceInfo>>> { unsafe {
        <Self as RtActivatable<IPlatformDiagnosticActionsStatics>>::get_activation_factory().get_known_trace_list(slotType)
    }}
}
DEFINE_CLSID!(PlatformDiagnosticActions(&[87,105,110,100,111,119,115,46,83,121,115,116,101,109,46,68,105,97,103,110,111,115,116,105,99,115,46,84,114,97,99,101,82,101,112,111,114,116,105,110,103,46,80,108,97,116,102,111,114,109,68,105,97,103,110,111,115,116,105,99,65,99,116,105,111,110,115,0]) [CLSID_PlatformDiagnosticActions]);
DEFINE_IID!(IID_IPlatformDiagnosticActionsStatics, 3239337210, 37522, 16999, 137, 10, 158, 163, 237, 7, 35, 18);
RT_INTERFACE!{static interface IPlatformDiagnosticActionsStatics(IPlatformDiagnosticActionsStaticsVtbl): IInspectable(IInspectableVtbl) [IID_IPlatformDiagnosticActionsStatics] {
    fn IsScenarioEnabled(&self, scenarioId: Guid, out: *mut bool) -> HRESULT,
    fn TryEscalateScenario(&self, scenarioId: Guid, escalationType: PlatformDiagnosticEscalationType, outputDirectory: HSTRING, timestampOutputDirectory: bool, forceEscalationUpload: bool, triggers: *mut ::rt::gen::windows::foundation::collections::IMapView<HString, HString>, out: *mut bool) -> HRESULT,
    fn DownloadLatestSettingsForNamespace(&self, partner: HSTRING, feature: HSTRING, isScenarioNamespace: bool, downloadOverCostedNetwork: bool, downloadOverBattery: bool, out: *mut PlatformDiagnosticActionState) -> HRESULT,
    fn GetActiveScenarioList(&self, out: *mut *mut ::rt::gen::windows::foundation::collections::IVectorView<Guid>) -> HRESULT,
    fn ForceUpload(&self, latency: PlatformDiagnosticEventBufferLatencies, uploadOverCostedNetwork: bool, uploadOverBattery: bool, out: *mut PlatformDiagnosticActionState) -> HRESULT,
    fn IsTraceRunning(&self, slotType: PlatformDiagnosticTraceSlotType, scenarioId: Guid, traceProfileHash: u64, out: *mut PlatformDiagnosticTraceSlotState) -> HRESULT,
    fn GetActiveTraceRuntime(&self, slotType: PlatformDiagnosticTraceSlotType, out: *mut *mut PlatformDiagnosticTraceRuntimeInfo) -> HRESULT,
    fn GetKnownTraceList(&self, slotType: PlatformDiagnosticTraceSlotType, out: *mut *mut ::rt::gen::windows::foundation::collections::IVectorView<PlatformDiagnosticTraceInfo>) -> HRESULT
}}
impl IPlatformDiagnosticActionsStatics {
    #[inline] pub unsafe fn is_scenario_enabled(&self, scenarioId: Guid) -> Result<bool> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).IsScenarioEnabled)(self as *const _ as *mut _, scenarioId, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn try_escalate_scenario(&self, scenarioId: Guid, escalationType: PlatformDiagnosticEscalationType, outputDirectory: &HStringArg, timestampOutputDirectory: bool, forceEscalationUpload: bool, triggers: &::rt::gen::windows::foundation::collections::IMapView<HString, HString>) -> Result<bool> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).TryEscalateScenario)(self as *const _ as *mut _, scenarioId, escalationType, outputDirectory.get(), timestampOutputDirectory, forceEscalationUpload, triggers as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn download_latest_settings_for_namespace(&self, partner: &HStringArg, feature: &HStringArg, isScenarioNamespace: bool, downloadOverCostedNetwork: bool, downloadOverBattery: bool) -> Result<PlatformDiagnosticActionState> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).DownloadLatestSettingsForNamespace)(self as *const _ as *mut _, partner.get(), feature.get(), isScenarioNamespace, downloadOverCostedNetwork, downloadOverBattery, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_active_scenario_list(&self) -> Result<ComPtr<::rt::gen::windows::foundation::collections::IVectorView<Guid>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetActiveScenarioList)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn force_upload(&self, latency: PlatformDiagnosticEventBufferLatencies, uploadOverCostedNetwork: bool, uploadOverBattery: bool) -> Result<PlatformDiagnosticActionState> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).ForceUpload)(self as *const _ as *mut _, latency, uploadOverCostedNetwork, uploadOverBattery, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn is_trace_running(&self, slotType: PlatformDiagnosticTraceSlotType, scenarioId: Guid, traceProfileHash: u64) -> Result<PlatformDiagnosticTraceSlotState> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).IsTraceRunning)(self as *const _ as *mut _, slotType, scenarioId, traceProfileHash, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_active_trace_runtime(&self, slotType: PlatformDiagnosticTraceSlotType) -> Result<ComPtr<PlatformDiagnosticTraceRuntimeInfo>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetActiveTraceRuntime)(self as *const _ as *mut _, slotType, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_known_trace_list(&self, slotType: PlatformDiagnosticTraceSlotType) -> Result<ComPtr<::rt::gen::windows::foundation::collections::IVectorView<PlatformDiagnosticTraceInfo>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetKnownTraceList)(self as *const _ as *mut _, slotType, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
RT_ENUM! { enum PlatformDiagnosticActionState: i32 {
    Success (PlatformDiagnosticActionState_Success) = 0, FreeNetworkNotAvailable (PlatformDiagnosticActionState_FreeNetworkNotAvailable) = 1, ACPowerNotAvailable (PlatformDiagnosticActionState_ACPowerNotAvailable) = 2,
}}
RT_ENUM! { enum PlatformDiagnosticEscalationType: i32 {
    OnCompletion (PlatformDiagnosticEscalationType_OnCompletion) = 0, OnFailure (PlatformDiagnosticEscalationType_OnFailure) = 1,
}}
RT_ENUM! { enum PlatformDiagnosticEventBufferLatencies: u32 {
    Normal (PlatformDiagnosticEventBufferLatencies_Normal) = 1, CostDeferred (PlatformDiagnosticEventBufferLatencies_CostDeferred) = 2, Realtime (PlatformDiagnosticEventBufferLatencies_Realtime) = 4,
}}
DEFINE_IID!(IID_IPlatformDiagnosticTraceInfo, 4168150423, 54679, 19447, 136, 220, 207, 92, 125, 194, 161, 210);
RT_INTERFACE!{interface IPlatformDiagnosticTraceInfo(IPlatformDiagnosticTraceInfoVtbl): IInspectable(IInspectableVtbl) [IID_IPlatformDiagnosticTraceInfo] {
    fn get_ScenarioId(&self, out: *mut Guid) -> HRESULT,
    fn get_ProfileHash(&self, out: *mut u64) -> HRESULT,
    fn get_IsExclusive(&self, out: *mut bool) -> HRESULT,
    fn get_IsAutoLogger(&self, out: *mut bool) -> HRESULT,
    fn get_MaxTraceDurationFileTime(&self, out: *mut i64) -> HRESULT,
    fn get_Priority(&self, out: *mut PlatformDiagnosticTracePriority) -> HRESULT
}}
impl IPlatformDiagnosticTraceInfo {
    #[inline] pub unsafe fn get_scenario_id(&self) -> Result<Guid> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_ScenarioId)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_profile_hash(&self) -> Result<u64> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_ProfileHash)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_is_exclusive(&self) -> Result<bool> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_IsExclusive)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_is_auto_logger(&self) -> Result<bool> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_IsAutoLogger)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_max_trace_duration_file_time(&self) -> Result<i64> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_MaxTraceDurationFileTime)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_priority(&self) -> Result<PlatformDiagnosticTracePriority> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_Priority)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
}
RT_CLASS!{class PlatformDiagnosticTraceInfo: IPlatformDiagnosticTraceInfo}
RT_ENUM! { enum PlatformDiagnosticTracePriority: i32 {
    Normal (PlatformDiagnosticTracePriority_Normal) = 0, UserElevated (PlatformDiagnosticTracePriority_UserElevated) = 1,
}}
DEFINE_IID!(IID_IPlatformDiagnosticTraceRuntimeInfo, 1028480557, 472, 18280, 133, 84, 30, 177, 202, 97, 9, 134);
RT_INTERFACE!{interface IPlatformDiagnosticTraceRuntimeInfo(IPlatformDiagnosticTraceRuntimeInfoVtbl): IInspectable(IInspectableVtbl) [IID_IPlatformDiagnosticTraceRuntimeInfo] {
    fn get_RuntimeFileTime(&self, out: *mut i64) -> HRESULT,
    fn get_EtwRuntimeFileTime(&self, out: *mut i64) -> HRESULT
}}
impl IPlatformDiagnosticTraceRuntimeInfo {
    #[inline] pub unsafe fn get_runtime_file_time(&self) -> Result<i64> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_RuntimeFileTime)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_etw_runtime_file_time(&self) -> Result<i64> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_EtwRuntimeFileTime)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
}
RT_CLASS!{class PlatformDiagnosticTraceRuntimeInfo: IPlatformDiagnosticTraceRuntimeInfo}
RT_ENUM! { enum PlatformDiagnosticTraceSlotState: i32 {
    NotRunning (PlatformDiagnosticTraceSlotState_NotRunning) = 0, Running (PlatformDiagnosticTraceSlotState_Running) = 1, Throttled (PlatformDiagnosticTraceSlotState_Throttled) = 2,
}}
RT_ENUM! { enum PlatformDiagnosticTraceSlotType: i32 {
    Alternative (PlatformDiagnosticTraceSlotType_Alternative) = 0, AlwaysOn (PlatformDiagnosticTraceSlotType_AlwaysOn) = 1, Mini (PlatformDiagnosticTraceSlotType_Mini) = 2,
}}
} // Windows.System.Diagnostics.TraceReporting
pub mod telemetry { // Windows.System.Diagnostics.Telemetry
use ::prelude::*;
RT_CLASS!{static class PlatformTelemetryClient}
impl RtActivatable<IPlatformTelemetryClientStatics> for PlatformTelemetryClient {}
impl PlatformTelemetryClient {
    #[inline] pub fn register(id: &HStringArg) -> Result<ComPtr<PlatformTelemetryRegistrationResult>> { unsafe {
        <Self as RtActivatable<IPlatformTelemetryClientStatics>>::get_activation_factory().register(id)
    }}
    #[inline] pub fn register_with_settings(id: &HStringArg, settings: &PlatformTelemetryRegistrationSettings) -> Result<ComPtr<PlatformTelemetryRegistrationResult>> { unsafe {
        <Self as RtActivatable<IPlatformTelemetryClientStatics>>::get_activation_factory().register_with_settings(id, settings)
    }}
}
DEFINE_CLSID!(PlatformTelemetryClient(&[87,105,110,100,111,119,115,46,83,121,115,116,101,109,46,68,105,97,103,110,111,115,116,105,99,115,46,84,101,108,101,109,101,116,114,121,46,80,108,97,116,102,111,114,109,84,101,108,101,109,101,116,114,121,67,108,105,101,110,116,0]) [CLSID_PlatformTelemetryClient]);
DEFINE_IID!(IID_IPlatformTelemetryClientStatics, 2616455773, 54723, 20202, 141, 190, 156, 141, 187, 13, 157, 143);
RT_INTERFACE!{static interface IPlatformTelemetryClientStatics(IPlatformTelemetryClientStaticsVtbl): IInspectable(IInspectableVtbl) [IID_IPlatformTelemetryClientStatics] {
    fn Register(&self, id: HSTRING, out: *mut *mut PlatformTelemetryRegistrationResult) -> HRESULT,
    fn RegisterWithSettings(&self, id: HSTRING, settings: *mut PlatformTelemetryRegistrationSettings, out: *mut *mut PlatformTelemetryRegistrationResult) -> HRESULT
}}
impl IPlatformTelemetryClientStatics {
    #[inline] pub unsafe fn register(&self, id: &HStringArg) -> Result<ComPtr<PlatformTelemetryRegistrationResult>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).Register)(self as *const _ as *mut _, id.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn register_with_settings(&self, id: &HStringArg, settings: &PlatformTelemetryRegistrationSettings) -> Result<ComPtr<PlatformTelemetryRegistrationResult>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).RegisterWithSettings)(self as *const _ as *mut _, id.get(), settings as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
DEFINE_IID!(IID_IPlatformTelemetryRegistrationResult, 1300568235, 8850, 18877, 161, 90, 61, 113, 210, 20, 81, 18);
RT_INTERFACE!{interface IPlatformTelemetryRegistrationResult(IPlatformTelemetryRegistrationResultVtbl): IInspectable(IInspectableVtbl) [IID_IPlatformTelemetryRegistrationResult] {
    fn get_Status(&self, out: *mut PlatformTelemetryRegistrationStatus) -> HRESULT
}}
impl IPlatformTelemetryRegistrationResult {
    #[inline] pub unsafe fn get_status(&self) -> Result<PlatformTelemetryRegistrationStatus> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_Status)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
}
RT_CLASS!{class PlatformTelemetryRegistrationResult: IPlatformTelemetryRegistrationResult}
DEFINE_IID!(IID_IPlatformTelemetryRegistrationSettings, 2174387586, 51737, 16734, 187, 121, 156, 34, 75, 250, 58, 115);
RT_INTERFACE!{interface IPlatformTelemetryRegistrationSettings(IPlatformTelemetryRegistrationSettingsVtbl): IInspectable(IInspectableVtbl) [IID_IPlatformTelemetryRegistrationSettings] {
    fn get_StorageSize(&self, out: *mut u32) -> HRESULT,
    fn put_StorageSize(&self, value: u32) -> HRESULT,
    fn get_UploadQuotaSize(&self, out: *mut u32) -> HRESULT,
    fn put_UploadQuotaSize(&self, value: u32) -> HRESULT
}}
impl IPlatformTelemetryRegistrationSettings {
    #[inline] pub unsafe fn get_storage_size(&self) -> Result<u32> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_StorageSize)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_storage_size(&self, value: u32) -> Result<()> {
        let hr = ((*self.lpVtbl).put_StorageSize)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_upload_quota_size(&self) -> Result<u32> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_UploadQuotaSize)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_upload_quota_size(&self, value: u32) -> Result<()> {
        let hr = ((*self.lpVtbl).put_UploadQuotaSize)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
}
RT_CLASS!{class PlatformTelemetryRegistrationSettings: IPlatformTelemetryRegistrationSettings}
impl RtActivatable<IActivationFactory> for PlatformTelemetryRegistrationSettings {}
DEFINE_CLSID!(PlatformTelemetryRegistrationSettings(&[87,105,110,100,111,119,115,46,83,121,115,116,101,109,46,68,105,97,103,110,111,115,116,105,99,115,46,84,101,108,101,109,101,116,114,121,46,80,108,97,116,102,111,114,109,84,101,108,101,109,101,116,114,121,82,101,103,105,115,116,114,97,116,105,111,110,83,101,116,116,105,110,103,115,0]) [CLSID_PlatformTelemetryRegistrationSettings]);
RT_ENUM! { enum PlatformTelemetryRegistrationStatus: i32 {
    Success (PlatformTelemetryRegistrationStatus_Success) = 0, SettingsOutOfRange (PlatformTelemetryRegistrationStatus_SettingsOutOfRange) = 1, UnknownFailure (PlatformTelemetryRegistrationStatus_UnknownFailure) = 2,
}}
} // Windows.System.Diagnostics.Telemetry
pub mod deviceportal { // Windows.System.Diagnostics.DevicePortal
use ::prelude::*;
DEFINE_IID!(IID_IDevicePortalConnection, 256147281, 4504, 19873, 141, 84, 189, 239, 57, 62, 9, 182);
RT_INTERFACE!{interface IDevicePortalConnection(IDevicePortalConnectionVtbl): IInspectable(IInspectableVtbl) [IID_IDevicePortalConnection] {
    fn add_Closed(&self, handler: *mut ::rt::gen::windows::foundation::TypedEventHandler<DevicePortalConnection, DevicePortalConnectionClosedEventArgs>, out: *mut ::rt::gen::windows::foundation::EventRegistrationToken) -> HRESULT,
    fn remove_Closed(&self, token: ::rt::gen::windows::foundation::EventRegistrationToken) -> HRESULT,
    fn add_RequestReceived(&self, handler: *mut ::rt::gen::windows::foundation::TypedEventHandler<DevicePortalConnection, DevicePortalConnectionRequestReceivedEventArgs>, out: *mut ::rt::gen::windows::foundation::EventRegistrationToken) -> HRESULT,
    fn remove_RequestReceived(&self, token: ::rt::gen::windows::foundation::EventRegistrationToken) -> HRESULT
}}
impl IDevicePortalConnection {
    #[inline] pub unsafe fn add_closed(&self, handler: &::rt::gen::windows::foundation::TypedEventHandler<DevicePortalConnection, DevicePortalConnectionClosedEventArgs>) -> Result<::rt::gen::windows::foundation::EventRegistrationToken> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).add_Closed)(self as *const _ as *mut _, handler as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn remove_closed(&self, token: ::rt::gen::windows::foundation::EventRegistrationToken) -> Result<()> {
        let hr = ((*self.lpVtbl).remove_Closed)(self as *const _ as *mut _, token);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn add_request_received(&self, handler: &::rt::gen::windows::foundation::TypedEventHandler<DevicePortalConnection, DevicePortalConnectionRequestReceivedEventArgs>) -> Result<::rt::gen::windows::foundation::EventRegistrationToken> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).add_RequestReceived)(self as *const _ as *mut _, handler as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn remove_request_received(&self, token: ::rt::gen::windows::foundation::EventRegistrationToken) -> Result<()> {
        let hr = ((*self.lpVtbl).remove_RequestReceived)(self as *const _ as *mut _, token);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
}
RT_CLASS!{class DevicePortalConnection: IDevicePortalConnection}
impl RtActivatable<IDevicePortalConnectionStatics> for DevicePortalConnection {}
impl DevicePortalConnection {
    #[cfg(feature="windows-applicationmodel")] #[inline] pub fn get_for_app_service_connection(appServiceConnection: &::rt::gen::windows::applicationmodel::appservice::AppServiceConnection) -> Result<ComPtr<DevicePortalConnection>> { unsafe {
        <Self as RtActivatable<IDevicePortalConnectionStatics>>::get_activation_factory().get_for_app_service_connection(appServiceConnection)
    }}
}
DEFINE_CLSID!(DevicePortalConnection(&[87,105,110,100,111,119,115,46,83,121,115,116,101,109,46,68,105,97,103,110,111,115,116,105,99,115,46,68,101,118,105,99,101,80,111,114,116,97,108,46,68,101,118,105,99,101,80,111,114,116,97,108,67,111,110,110,101,99,116,105,111,110,0]) [CLSID_DevicePortalConnection]);
DEFINE_IID!(IID_IDevicePortalConnectionClosedEventArgs, 4244049464, 28722, 17036, 159, 80, 148, 92, 21, 169, 240, 203);
RT_INTERFACE!{interface IDevicePortalConnectionClosedEventArgs(IDevicePortalConnectionClosedEventArgsVtbl): IInspectable(IInspectableVtbl) [IID_IDevicePortalConnectionClosedEventArgs] {
    fn get_Reason(&self, out: *mut DevicePortalConnectionClosedReason) -> HRESULT
}}
impl IDevicePortalConnectionClosedEventArgs {
    #[inline] pub unsafe fn get_reason(&self) -> Result<DevicePortalConnectionClosedReason> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_Reason)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
}
RT_CLASS!{class DevicePortalConnectionClosedEventArgs: IDevicePortalConnectionClosedEventArgs}
RT_ENUM! { enum DevicePortalConnectionClosedReason: i32 {
    Unknown (DevicePortalConnectionClosedReason_Unknown) = 0, ResourceLimitsExceeded (DevicePortalConnectionClosedReason_ResourceLimitsExceeded) = 1, ProtocolError (DevicePortalConnectionClosedReason_ProtocolError) = 2, NotAuthorized (DevicePortalConnectionClosedReason_NotAuthorized) = 3, UserNotPresent (DevicePortalConnectionClosedReason_UserNotPresent) = 4, ServiceTerminated (DevicePortalConnectionClosedReason_ServiceTerminated) = 5,
}}
DEFINE_IID!(IID_IDevicePortalConnectionRequestReceivedEventArgs, 1692065861, 28634, 17497, 158, 189, 236, 206, 34, 227, 133, 89);
RT_INTERFACE!{interface IDevicePortalConnectionRequestReceivedEventArgs(IDevicePortalConnectionRequestReceivedEventArgsVtbl): IInspectable(IInspectableVtbl) [IID_IDevicePortalConnectionRequestReceivedEventArgs] {
    #[cfg(feature="windows-web")] fn get_RequestMessage(&self, out: *mut *mut ::rt::gen::windows::web::http::HttpRequestMessage) -> HRESULT,
    #[cfg(feature="windows-web")] fn get_ResponseMessage(&self, out: *mut *mut ::rt::gen::windows::web::http::HttpResponseMessage) -> HRESULT
}}
impl IDevicePortalConnectionRequestReceivedEventArgs {
    #[cfg(feature="windows-web")] #[inline] pub unsafe fn get_request_message(&self) -> Result<ComPtr<::rt::gen::windows::web::http::HttpRequestMessage>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_RequestMessage)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[cfg(feature="windows-web")] #[inline] pub unsafe fn get_response_message(&self) -> Result<ComPtr<::rt::gen::windows::web::http::HttpResponseMessage>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_ResponseMessage)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
RT_CLASS!{class DevicePortalConnectionRequestReceivedEventArgs: IDevicePortalConnectionRequestReceivedEventArgs}
DEFINE_IID!(IID_IDevicePortalConnectionStatics, 1270755815, 59833, 17989, 143, 237, 165, 62, 234, 14, 219, 214);
RT_INTERFACE!{static interface IDevicePortalConnectionStatics(IDevicePortalConnectionStaticsVtbl): IInspectable(IInspectableVtbl) [IID_IDevicePortalConnectionStatics] {
    #[cfg(feature="windows-applicationmodel")] fn GetForAppServiceConnection(&self, appServiceConnection: *mut ::rt::gen::windows::applicationmodel::appservice::AppServiceConnection, out: *mut *mut DevicePortalConnection) -> HRESULT
}}
impl IDevicePortalConnectionStatics {
    #[cfg(feature="windows-applicationmodel")] #[inline] pub unsafe fn get_for_app_service_connection(&self, appServiceConnection: &::rt::gen::windows::applicationmodel::appservice::AppServiceConnection) -> Result<ComPtr<DevicePortalConnection>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetForAppServiceConnection)(self as *const _ as *mut _, appServiceConnection as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
} // Windows.System.Diagnostics.DevicePortal
} // Windows.System.Diagnostics
pub mod remotesystems { // Windows.System.RemoteSystems
use ::prelude::*;
RT_CLASS!{static class KnownRemoteSystemCapabilities}
impl RtActivatable<IKnownRemoteSystemCapabilitiesStatics> for KnownRemoteSystemCapabilities {}
impl KnownRemoteSystemCapabilities {
    #[inline] pub fn get_app_service() -> Result<HString> { unsafe {
        <Self as RtActivatable<IKnownRemoteSystemCapabilitiesStatics>>::get_activation_factory().get_app_service()
    }}
    #[inline] pub fn get_launch_uri() -> Result<HString> { unsafe {
        <Self as RtActivatable<IKnownRemoteSystemCapabilitiesStatics>>::get_activation_factory().get_launch_uri()
    }}
    #[inline] pub fn get_remote_session() -> Result<HString> { unsafe {
        <Self as RtActivatable<IKnownRemoteSystemCapabilitiesStatics>>::get_activation_factory().get_remote_session()
    }}
    #[inline] pub fn get_spatial_entity() -> Result<HString> { unsafe {
        <Self as RtActivatable<IKnownRemoteSystemCapabilitiesStatics>>::get_activation_factory().get_spatial_entity()
    }}
}
DEFINE_CLSID!(KnownRemoteSystemCapabilities(&[87,105,110,100,111,119,115,46,83,121,115,116,101,109,46,82,101,109,111,116,101,83,121,115,116,101,109,115,46,75,110,111,119,110,82,101,109,111,116,101,83,121,115,116,101,109,67,97,112,97,98,105,108,105,116,105,101,115,0]) [CLSID_KnownRemoteSystemCapabilities]);
DEFINE_IID!(IID_IKnownRemoteSystemCapabilitiesStatics, 2164843392, 32650, 17636, 146, 205, 3, 182, 70, 155, 148, 163);
RT_INTERFACE!{static interface IKnownRemoteSystemCapabilitiesStatics(IKnownRemoteSystemCapabilitiesStaticsVtbl): IInspectable(IInspectableVtbl) [IID_IKnownRemoteSystemCapabilitiesStatics] {
    fn get_AppService(&self, out: *mut HSTRING) -> HRESULT,
    fn get_LaunchUri(&self, out: *mut HSTRING) -> HRESULT,
    fn get_RemoteSession(&self, out: *mut HSTRING) -> HRESULT,
    fn get_SpatialEntity(&self, out: *mut HSTRING) -> HRESULT
}}
impl IKnownRemoteSystemCapabilitiesStatics {
    #[inline] pub unsafe fn get_app_service(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_AppService)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_launch_uri(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_LaunchUri)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_remote_session(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_RemoteSession)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_spatial_entity(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_SpatialEntity)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
}
DEFINE_IID!(IID_IRemoteSystem, 3981981901, 7696, 19084, 180, 166, 78, 95, 214, 249, 119, 33);
RT_INTERFACE!{interface IRemoteSystem(IRemoteSystemVtbl): IInspectable(IInspectableVtbl) [IID_IRemoteSystem] {
    fn get_DisplayName(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Id(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Kind(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Status(&self, out: *mut RemoteSystemStatus) -> HRESULT,
    fn get_IsAvailableByProximity(&self, out: *mut bool) -> HRESULT
}}
impl IRemoteSystem {
    #[inline] pub unsafe fn get_display_name(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_DisplayName)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_id(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Id)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_kind(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Kind)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_status(&self) -> Result<RemoteSystemStatus> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_Status)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_is_available_by_proximity(&self) -> Result<bool> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_IsAvailableByProximity)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
}
RT_CLASS!{class RemoteSystem: IRemoteSystem}
impl RtActivatable<IRemoteSystemStatics> for RemoteSystem {}
impl RtActivatable<IRemoteSystemStatics2> for RemoteSystem {}
impl RemoteSystem {
    #[cfg(feature="windows-networking")] #[inline] pub fn find_by_host_name_async(hostName: &super::super::networking::HostName) -> Result<ComPtr<super::super::foundation::IAsyncOperation<RemoteSystem>>> { unsafe {
        <Self as RtActivatable<IRemoteSystemStatics>>::get_activation_factory().find_by_host_name_async(hostName)
    }}
    #[inline] pub fn create_watcher() -> Result<ComPtr<RemoteSystemWatcher>> { unsafe {
        <Self as RtActivatable<IRemoteSystemStatics>>::get_activation_factory().create_watcher()
    }}
    #[inline] pub fn create_watcher_with_filters(filters: &super::super::foundation::collections::IIterable<IRemoteSystemFilter>) -> Result<ComPtr<RemoteSystemWatcher>> { unsafe {
        <Self as RtActivatable<IRemoteSystemStatics>>::get_activation_factory().create_watcher_with_filters(filters)
    }}
    #[inline] pub fn request_access_async() -> Result<ComPtr<super::super::foundation::IAsyncOperation<RemoteSystemAccessStatus>>> { unsafe {
        <Self as RtActivatable<IRemoteSystemStatics>>::get_activation_factory().request_access_async()
    }}
    #[inline] pub fn is_authorization_kind_enabled(kind: RemoteSystemAuthorizationKind) -> Result<bool> { unsafe {
        <Self as RtActivatable<IRemoteSystemStatics2>>::get_activation_factory().is_authorization_kind_enabled(kind)
    }}
}
DEFINE_CLSID!(RemoteSystem(&[87,105,110,100,111,119,115,46,83,121,115,116,101,109,46,82,101,109,111,116,101,83,121,115,116,101,109,115,46,82,101,109,111,116,101,83,121,115,116,101,109,0]) [CLSID_RemoteSystem]);
DEFINE_IID!(IID_IRemoteSystem2, 165668076, 64395, 18952, 167, 88, 104, 118, 67, 93, 118, 158);
RT_INTERFACE!{interface IRemoteSystem2(IRemoteSystem2Vtbl): IInspectable(IInspectableVtbl) [IID_IRemoteSystem2] {
    fn get_IsAvailableBySpatialProximity(&self, out: *mut bool) -> HRESULT,
    fn GetCapabilitySupportedAsync(&self, capabilityName: HSTRING, out: *mut *mut super::super::foundation::IAsyncOperation<bool>) -> HRESULT
}}
impl IRemoteSystem2 {
    #[inline] pub unsafe fn get_is_available_by_spatial_proximity(&self) -> Result<bool> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_IsAvailableBySpatialProximity)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_capability_supported_async(&self, capabilityName: &HStringArg) -> Result<ComPtr<super::super::foundation::IAsyncOperation<bool>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetCapabilitySupportedAsync)(self as *const _ as *mut _, capabilityName.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
DEFINE_IID!(IID_IRemoteSystem3, 1924445333, 47046, 16574, 131, 27, 115, 86, 47, 18, 255, 168);
RT_INTERFACE!{interface IRemoteSystem3(IRemoteSystem3Vtbl): IInspectable(IInspectableVtbl) [IID_IRemoteSystem3] {
    fn get_ManufacturerDisplayName(&self, out: *mut HSTRING) -> HRESULT,
    fn get_ModelDisplayName(&self, out: *mut HSTRING) -> HRESULT
}}
impl IRemoteSystem3 {
    #[inline] pub unsafe fn get_manufacturer_display_name(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_ManufacturerDisplayName)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_model_display_name(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_ModelDisplayName)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
}
RT_ENUM! { enum RemoteSystemAccessStatus: i32 {
    Unspecified (RemoteSystemAccessStatus_Unspecified) = 0, Allowed (RemoteSystemAccessStatus_Allowed) = 1, DeniedByUser (RemoteSystemAccessStatus_DeniedByUser) = 2, DeniedBySystem (RemoteSystemAccessStatus_DeniedBySystem) = 3,
}}
DEFINE_IID!(IID_IRemoteSystemAddedEventArgs, 2402899471, 58676, 18071, 136, 54, 122, 190, 161, 81, 81, 110);
RT_INTERFACE!{interface IRemoteSystemAddedEventArgs(IRemoteSystemAddedEventArgsVtbl): IInspectable(IInspectableVtbl) [IID_IRemoteSystemAddedEventArgs] {
    fn get_RemoteSystem(&self, out: *mut *mut RemoteSystem) -> HRESULT
}}
impl IRemoteSystemAddedEventArgs {
    #[inline] pub unsafe fn get_remote_system(&self) -> Result<ComPtr<RemoteSystem>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_RemoteSystem)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
RT_CLASS!{class RemoteSystemAddedEventArgs: IRemoteSystemAddedEventArgs}
RT_ENUM! { enum RemoteSystemAuthorizationKind: i32 {
    SameUser (RemoteSystemAuthorizationKind_SameUser) = 0, Anonymous (RemoteSystemAuthorizationKind_Anonymous) = 1,
}}
DEFINE_IID!(IID_IRemoteSystemAuthorizationKindFilter, 1796071054, 1232, 16628, 162, 127, 194, 172, 187, 214, 183, 52);
RT_INTERFACE!{interface IRemoteSystemAuthorizationKindFilter(IRemoteSystemAuthorizationKindFilterVtbl): IInspectable(IInspectableVtbl) [IID_IRemoteSystemAuthorizationKindFilter] {
    fn get_RemoteSystemAuthorizationKind(&self, out: *mut RemoteSystemAuthorizationKind) -> HRESULT
}}
impl IRemoteSystemAuthorizationKindFilter {
    #[inline] pub unsafe fn get_remote_system_authorization_kind(&self) -> Result<RemoteSystemAuthorizationKind> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_RemoteSystemAuthorizationKind)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
}
RT_CLASS!{class RemoteSystemAuthorizationKindFilter: IRemoteSystemAuthorizationKindFilter}
impl RtActivatable<IRemoteSystemAuthorizationKindFilterFactory> for RemoteSystemAuthorizationKindFilter {}
impl RemoteSystemAuthorizationKindFilter {
    #[inline] pub fn create(remoteSystemAuthorizationKind: RemoteSystemAuthorizationKind) -> Result<ComPtr<RemoteSystemAuthorizationKindFilter>> { unsafe {
        <Self as RtActivatable<IRemoteSystemAuthorizationKindFilterFactory>>::get_activation_factory().create(remoteSystemAuthorizationKind)
    }}
}
DEFINE_CLSID!(RemoteSystemAuthorizationKindFilter(&[87,105,110,100,111,119,115,46,83,121,115,116,101,109,46,82,101,109,111,116,101,83,121,115,116,101,109,115,46,82,101,109,111,116,101,83,121,115,116,101,109,65,117,116,104,111,114,105,122,97,116,105,111,110,75,105,110,100,70,105,108,116,101,114,0]) [CLSID_RemoteSystemAuthorizationKindFilter]);
DEFINE_IID!(IID_IRemoteSystemAuthorizationKindFilterFactory, 2909134669, 46698, 17828, 129, 119, 140, 174, 215, 93, 158, 90);
RT_INTERFACE!{static interface IRemoteSystemAuthorizationKindFilterFactory(IRemoteSystemAuthorizationKindFilterFactoryVtbl): IInspectable(IInspectableVtbl) [IID_IRemoteSystemAuthorizationKindFilterFactory] {
    fn Create(&self, remoteSystemAuthorizationKind: RemoteSystemAuthorizationKind, out: *mut *mut RemoteSystemAuthorizationKindFilter) -> HRESULT
}}
impl IRemoteSystemAuthorizationKindFilterFactory {
    #[inline] pub unsafe fn create(&self, remoteSystemAuthorizationKind: RemoteSystemAuthorizationKind) -> Result<ComPtr<RemoteSystemAuthorizationKindFilter>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).Create)(self as *const _ as *mut _, remoteSystemAuthorizationKind, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
DEFINE_IID!(IID_IRemoteSystemConnectionRequest, 2230141188, 36190, 19826, 130, 56, 118, 33, 87, 108, 122, 103);
RT_INTERFACE!{interface IRemoteSystemConnectionRequest(IRemoteSystemConnectionRequestVtbl): IInspectable(IInspectableVtbl) [IID_IRemoteSystemConnectionRequest] {
    fn get_RemoteSystem(&self, out: *mut *mut RemoteSystem) -> HRESULT
}}
impl IRemoteSystemConnectionRequest {
    #[inline] pub unsafe fn get_remote_system(&self) -> Result<ComPtr<RemoteSystem>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_RemoteSystem)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
RT_CLASS!{class RemoteSystemConnectionRequest: IRemoteSystemConnectionRequest}
impl RtActivatable<IRemoteSystemConnectionRequestFactory> for RemoteSystemConnectionRequest {}
impl RemoteSystemConnectionRequest {
    #[inline] pub fn create(remoteSystem: &RemoteSystem) -> Result<ComPtr<RemoteSystemConnectionRequest>> { unsafe {
        <Self as RtActivatable<IRemoteSystemConnectionRequestFactory>>::get_activation_factory().create(remoteSystem)
    }}
}
DEFINE_CLSID!(RemoteSystemConnectionRequest(&[87,105,110,100,111,119,115,46,83,121,115,116,101,109,46,82,101,109,111,116,101,83,121,115,116,101,109,115,46,82,101,109,111,116,101,83,121,115,116,101,109,67,111,110,110,101,99,116,105,111,110,82,101,113,117,101,115,116,0]) [CLSID_RemoteSystemConnectionRequest]);
DEFINE_IID!(IID_IRemoteSystemConnectionRequestFactory, 2852784672, 47851, 17781, 181, 48, 129, 11, 185, 120, 99, 52);
RT_INTERFACE!{static interface IRemoteSystemConnectionRequestFactory(IRemoteSystemConnectionRequestFactoryVtbl): IInspectable(IInspectableVtbl) [IID_IRemoteSystemConnectionRequestFactory] {
    fn Create(&self, remoteSystem: *mut RemoteSystem, out: *mut *mut RemoteSystemConnectionRequest) -> HRESULT
}}
impl IRemoteSystemConnectionRequestFactory {
    #[inline] pub unsafe fn create(&self, remoteSystem: &RemoteSystem) -> Result<ComPtr<RemoteSystemConnectionRequest>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).Create)(self as *const _ as *mut _, remoteSystem as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
RT_ENUM! { enum RemoteSystemDiscoveryType: i32 {
    Any (RemoteSystemDiscoveryType_Any) = 0, Proximal (RemoteSystemDiscoveryType_Proximal) = 1, Cloud (RemoteSystemDiscoveryType_Cloud) = 2, SpatiallyProximal (RemoteSystemDiscoveryType_SpatiallyProximal) = 3,
}}
DEFINE_IID!(IID_IRemoteSystemDiscoveryTypeFilter, 1121518623, 61018, 17370, 172, 106, 111, 238, 37, 70, 7, 65);
RT_INTERFACE!{interface IRemoteSystemDiscoveryTypeFilter(IRemoteSystemDiscoveryTypeFilterVtbl): IInspectable(IInspectableVtbl) [IID_IRemoteSystemDiscoveryTypeFilter] {
    fn get_RemoteSystemDiscoveryType(&self, out: *mut RemoteSystemDiscoveryType) -> HRESULT
}}
impl IRemoteSystemDiscoveryTypeFilter {
    #[inline] pub unsafe fn get_remote_system_discovery_type(&self) -> Result<RemoteSystemDiscoveryType> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_RemoteSystemDiscoveryType)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
}
RT_CLASS!{class RemoteSystemDiscoveryTypeFilter: IRemoteSystemDiscoveryTypeFilter}
impl RtActivatable<IRemoteSystemDiscoveryTypeFilterFactory> for RemoteSystemDiscoveryTypeFilter {}
impl RemoteSystemDiscoveryTypeFilter {
    #[inline] pub fn create(discoveryType: RemoteSystemDiscoveryType) -> Result<ComPtr<RemoteSystemDiscoveryTypeFilter>> { unsafe {
        <Self as RtActivatable<IRemoteSystemDiscoveryTypeFilterFactory>>::get_activation_factory().create(discoveryType)
    }}
}
DEFINE_CLSID!(RemoteSystemDiscoveryTypeFilter(&[87,105,110,100,111,119,115,46,83,121,115,116,101,109,46,82,101,109,111,116,101,83,121,115,116,101,109,115,46,82,101,109,111,116,101,83,121,115,116,101,109,68,105,115,99,111,118,101,114,121,84,121,112,101,70,105,108,116,101,114,0]) [CLSID_RemoteSystemDiscoveryTypeFilter]);
DEFINE_IID!(IID_IRemoteSystemDiscoveryTypeFilterFactory, 2677979539, 49760, 16737, 146, 242, 156, 2, 31, 35, 254, 93);
RT_INTERFACE!{static interface IRemoteSystemDiscoveryTypeFilterFactory(IRemoteSystemDiscoveryTypeFilterFactoryVtbl): IInspectable(IInspectableVtbl) [IID_IRemoteSystemDiscoveryTypeFilterFactory] {
    fn Create(&self, discoveryType: RemoteSystemDiscoveryType, out: *mut *mut RemoteSystemDiscoveryTypeFilter) -> HRESULT
}}
impl IRemoteSystemDiscoveryTypeFilterFactory {
    #[inline] pub unsafe fn create(&self, discoveryType: RemoteSystemDiscoveryType) -> Result<ComPtr<RemoteSystemDiscoveryTypeFilter>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).Create)(self as *const _ as *mut _, discoveryType, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
DEFINE_IID!(IID_IRemoteSystemFilter, 1245424100, 39403, 17899, 186, 22, 3, 103, 114, 143, 243, 116);
RT_INTERFACE!{interface IRemoteSystemFilter(IRemoteSystemFilterVtbl): IInspectable(IInspectableVtbl) [IID_IRemoteSystemFilter] {
    
}}
DEFINE_IID!(IID_IRemoteSystemKindFilter, 954321388, 8899, 20214, 144, 26, 187, 177, 199, 170, 212, 237);
RT_INTERFACE!{interface IRemoteSystemKindFilter(IRemoteSystemKindFilterVtbl): IInspectable(IInspectableVtbl) [IID_IRemoteSystemKindFilter] {
    fn get_RemoteSystemKinds(&self, out: *mut *mut super::super::foundation::collections::IVectorView<HString>) -> HRESULT
}}
impl IRemoteSystemKindFilter {
    #[inline] pub unsafe fn get_remote_system_kinds(&self) -> Result<ComPtr<super::super::foundation::collections::IVectorView<HString>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_RemoteSystemKinds)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
RT_CLASS!{class RemoteSystemKindFilter: IRemoteSystemKindFilter}
impl RtActivatable<IRemoteSystemKindFilterFactory> for RemoteSystemKindFilter {}
impl RemoteSystemKindFilter {
    #[inline] pub fn create(remoteSystemKinds: &super::super::foundation::collections::IIterable<HString>) -> Result<ComPtr<RemoteSystemKindFilter>> { unsafe {
        <Self as RtActivatable<IRemoteSystemKindFilterFactory>>::get_activation_factory().create(remoteSystemKinds)
    }}
}
DEFINE_CLSID!(RemoteSystemKindFilter(&[87,105,110,100,111,119,115,46,83,121,115,116,101,109,46,82,101,109,111,116,101,83,121,115,116,101,109,115,46,82,101,109,111,116,101,83,121,115,116,101,109,75,105,110,100,70,105,108,116,101,114,0]) [CLSID_RemoteSystemKindFilter]);
DEFINE_IID!(IID_IRemoteSystemKindFilterFactory, 2717587694, 39402, 16572, 154, 57, 198, 112, 170, 128, 74, 40);
RT_INTERFACE!{static interface IRemoteSystemKindFilterFactory(IRemoteSystemKindFilterFactoryVtbl): IInspectable(IInspectableVtbl) [IID_IRemoteSystemKindFilterFactory] {
    fn Create(&self, remoteSystemKinds: *mut super::super::foundation::collections::IIterable<HString>, out: *mut *mut RemoteSystemKindFilter) -> HRESULT
}}
impl IRemoteSystemKindFilterFactory {
    #[inline] pub unsafe fn create(&self, remoteSystemKinds: &super::super::foundation::collections::IIterable<HString>) -> Result<ComPtr<RemoteSystemKindFilter>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).Create)(self as *const _ as *mut _, remoteSystemKinds as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
RT_CLASS!{static class RemoteSystemKinds}
impl RtActivatable<IRemoteSystemKindStatics> for RemoteSystemKinds {}
impl RtActivatable<IRemoteSystemKindStatics2> for RemoteSystemKinds {}
impl RemoteSystemKinds {
    #[inline] pub fn get_phone() -> Result<HString> { unsafe {
        <Self as RtActivatable<IRemoteSystemKindStatics>>::get_activation_factory().get_phone()
    }}
    #[inline] pub fn get_hub() -> Result<HString> { unsafe {
        <Self as RtActivatable<IRemoteSystemKindStatics>>::get_activation_factory().get_hub()
    }}
    #[inline] pub fn get_holographic() -> Result<HString> { unsafe {
        <Self as RtActivatable<IRemoteSystemKindStatics>>::get_activation_factory().get_holographic()
    }}
    #[inline] pub fn get_desktop() -> Result<HString> { unsafe {
        <Self as RtActivatable<IRemoteSystemKindStatics>>::get_activation_factory().get_desktop()
    }}
    #[inline] pub fn get_xbox() -> Result<HString> { unsafe {
        <Self as RtActivatable<IRemoteSystemKindStatics>>::get_activation_factory().get_xbox()
    }}
    #[inline] pub fn get_iot() -> Result<HString> { unsafe {
        <Self as RtActivatable<IRemoteSystemKindStatics2>>::get_activation_factory().get_iot()
    }}
    #[inline] pub fn get_tablet() -> Result<HString> { unsafe {
        <Self as RtActivatable<IRemoteSystemKindStatics2>>::get_activation_factory().get_tablet()
    }}
    #[inline] pub fn get_laptop() -> Result<HString> { unsafe {
        <Self as RtActivatable<IRemoteSystemKindStatics2>>::get_activation_factory().get_laptop()
    }}
}
DEFINE_CLSID!(RemoteSystemKinds(&[87,105,110,100,111,119,115,46,83,121,115,116,101,109,46,82,101,109,111,116,101,83,121,115,116,101,109,115,46,82,101,109,111,116,101,83,121,115,116,101,109,75,105,110,100,115,0]) [CLSID_RemoteSystemKinds]);
DEFINE_IID!(IID_IRemoteSystemKindStatics, 4130436659, 43796, 16848, 149, 83, 121, 106, 173, 184, 130, 219);
RT_INTERFACE!{static interface IRemoteSystemKindStatics(IRemoteSystemKindStaticsVtbl): IInspectable(IInspectableVtbl) [IID_IRemoteSystemKindStatics] {
    fn get_Phone(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Hub(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Holographic(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Desktop(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Xbox(&self, out: *mut HSTRING) -> HRESULT
}}
impl IRemoteSystemKindStatics {
    #[inline] pub unsafe fn get_phone(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Phone)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_hub(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Hub)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_holographic(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Holographic)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_desktop(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Desktop)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_xbox(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Xbox)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
}
DEFINE_IID!(IID_IRemoteSystemKindStatics2, 3118703568, 1126, 18249, 145, 232, 101, 249, 209, 154, 150, 165);
RT_INTERFACE!{static interface IRemoteSystemKindStatics2(IRemoteSystemKindStatics2Vtbl): IInspectable(IInspectableVtbl) [IID_IRemoteSystemKindStatics2] {
    fn get_Iot(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Tablet(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Laptop(&self, out: *mut HSTRING) -> HRESULT
}}
impl IRemoteSystemKindStatics2 {
    #[inline] pub unsafe fn get_iot(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Iot)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_tablet(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Tablet)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_laptop(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Laptop)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
}
DEFINE_IID!(IID_IRemoteSystemRemovedEventArgs, 2336036539, 29446, 18922, 183, 223, 103, 213, 113, 76, 176, 19);
RT_INTERFACE!{interface IRemoteSystemRemovedEventArgs(IRemoteSystemRemovedEventArgsVtbl): IInspectable(IInspectableVtbl) [IID_IRemoteSystemRemovedEventArgs] {
    fn get_RemoteSystemId(&self, out: *mut HSTRING) -> HRESULT
}}
impl IRemoteSystemRemovedEventArgs {
    #[inline] pub unsafe fn get_remote_system_id(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_RemoteSystemId)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
}
RT_CLASS!{class RemoteSystemRemovedEventArgs: IRemoteSystemRemovedEventArgs}
DEFINE_IID!(IID_IRemoteSystemSession, 1766287873, 39642, 18703, 149, 73, 211, 28, 177, 76, 158, 149);
RT_INTERFACE!{interface IRemoteSystemSession(IRemoteSystemSessionVtbl): IInspectable(IInspectableVtbl) [IID_IRemoteSystemSession] {
    fn get_Id(&self, out: *mut HSTRING) -> HRESULT,
    fn get_DisplayName(&self, out: *mut HSTRING) -> HRESULT,
    fn get_ControllerDisplayName(&self, out: *mut HSTRING) -> HRESULT,
    fn add_Disconnected(&self, handler: *mut super::super::foundation::TypedEventHandler<RemoteSystemSession, RemoteSystemSessionDisconnectedEventArgs>, out: *mut super::super::foundation::EventRegistrationToken) -> HRESULT,
    fn remove_Disconnected(&self, token: super::super::foundation::EventRegistrationToken) -> HRESULT,
    fn CreateParticipantWatcher(&self, out: *mut *mut RemoteSystemSessionParticipantWatcher) -> HRESULT,
    fn SendInvitationAsync(&self, invitee: *mut RemoteSystem, out: *mut *mut super::super::foundation::IAsyncOperation<bool>) -> HRESULT
}}
impl IRemoteSystemSession {
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
    #[inline] pub unsafe fn get_controller_display_name(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_ControllerDisplayName)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn add_disconnected(&self, handler: &super::super::foundation::TypedEventHandler<RemoteSystemSession, RemoteSystemSessionDisconnectedEventArgs>) -> Result<super::super::foundation::EventRegistrationToken> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).add_Disconnected)(self as *const _ as *mut _, handler as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn remove_disconnected(&self, token: super::super::foundation::EventRegistrationToken) -> Result<()> {
        let hr = ((*self.lpVtbl).remove_Disconnected)(self as *const _ as *mut _, token);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn create_participant_watcher(&self) -> Result<ComPtr<RemoteSystemSessionParticipantWatcher>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).CreateParticipantWatcher)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn send_invitation_async(&self, invitee: &RemoteSystem) -> Result<ComPtr<super::super::foundation::IAsyncOperation<bool>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).SendInvitationAsync)(self as *const _ as *mut _, invitee as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
RT_CLASS!{class RemoteSystemSession: IRemoteSystemSession}
impl RtActivatable<IRemoteSystemSessionStatics> for RemoteSystemSession {}
impl RemoteSystemSession {
    #[inline] pub fn create_watcher() -> Result<ComPtr<RemoteSystemSessionWatcher>> { unsafe {
        <Self as RtActivatable<IRemoteSystemSessionStatics>>::get_activation_factory().create_watcher()
    }}
}
DEFINE_CLSID!(RemoteSystemSession(&[87,105,110,100,111,119,115,46,83,121,115,116,101,109,46,82,101,109,111,116,101,83,121,115,116,101,109,115,46,82,101,109,111,116,101,83,121,115,116,101,109,83,101,115,115,105,111,110,0]) [CLSID_RemoteSystemSession]);
DEFINE_IID!(IID_IRemoteSystemSessionAddedEventArgs, 3582318420, 48279, 19513, 153, 180, 190, 202, 118, 224, 76, 63);
RT_INTERFACE!{interface IRemoteSystemSessionAddedEventArgs(IRemoteSystemSessionAddedEventArgsVtbl): IInspectable(IInspectableVtbl) [IID_IRemoteSystemSessionAddedEventArgs] {
    fn get_SessionInfo(&self, out: *mut *mut RemoteSystemSessionInfo) -> HRESULT
}}
impl IRemoteSystemSessionAddedEventArgs {
    #[inline] pub unsafe fn get_session_info(&self) -> Result<ComPtr<RemoteSystemSessionInfo>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_SessionInfo)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
RT_CLASS!{class RemoteSystemSessionAddedEventArgs: IRemoteSystemSessionAddedEventArgs}
DEFINE_IID!(IID_IRemoteSystemSessionController, 3834326482, 26656, 18535, 180, 37, 216, 156, 10, 62, 247, 186);
RT_INTERFACE!{interface IRemoteSystemSessionController(IRemoteSystemSessionControllerVtbl): IInspectable(IInspectableVtbl) [IID_IRemoteSystemSessionController] {
    fn add_JoinRequested(&self, handler: *mut super::super::foundation::TypedEventHandler<RemoteSystemSessionController, RemoteSystemSessionJoinRequestedEventArgs>, out: *mut super::super::foundation::EventRegistrationToken) -> HRESULT,
    fn remove_JoinRequested(&self, token: super::super::foundation::EventRegistrationToken) -> HRESULT,
    fn RemoveParticipantAsync(&self, pParticipant: *mut RemoteSystemSessionParticipant, out: *mut *mut super::super::foundation::IAsyncOperation<bool>) -> HRESULT,
    fn CreateSessionAsync(&self, out: *mut *mut super::super::foundation::IAsyncOperation<RemoteSystemSessionCreationResult>) -> HRESULT
}}
impl IRemoteSystemSessionController {
    #[inline] pub unsafe fn add_join_requested(&self, handler: &super::super::foundation::TypedEventHandler<RemoteSystemSessionController, RemoteSystemSessionJoinRequestedEventArgs>) -> Result<super::super::foundation::EventRegistrationToken> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).add_JoinRequested)(self as *const _ as *mut _, handler as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn remove_join_requested(&self, token: super::super::foundation::EventRegistrationToken) -> Result<()> {
        let hr = ((*self.lpVtbl).remove_JoinRequested)(self as *const _ as *mut _, token);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn remove_participant_async(&self, pParticipant: &RemoteSystemSessionParticipant) -> Result<ComPtr<super::super::foundation::IAsyncOperation<bool>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).RemoveParticipantAsync)(self as *const _ as *mut _, pParticipant as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn create_session_async(&self) -> Result<ComPtr<super::super::foundation::IAsyncOperation<RemoteSystemSessionCreationResult>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).CreateSessionAsync)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
RT_CLASS!{class RemoteSystemSessionController: IRemoteSystemSessionController}
impl RtActivatable<IRemoteSystemSessionControllerFactory> for RemoteSystemSessionController {}
impl RemoteSystemSessionController {
    #[inline] pub fn create_controller(displayName: &HStringArg) -> Result<ComPtr<RemoteSystemSessionController>> { unsafe {
        <Self as RtActivatable<IRemoteSystemSessionControllerFactory>>::get_activation_factory().create_controller(displayName)
    }}
    #[inline] pub fn create_controller_with_session_options(displayName: &HStringArg, options: &RemoteSystemSessionOptions) -> Result<ComPtr<RemoteSystemSessionController>> { unsafe {
        <Self as RtActivatable<IRemoteSystemSessionControllerFactory>>::get_activation_factory().create_controller_with_session_options(displayName, options)
    }}
}
DEFINE_CLSID!(RemoteSystemSessionController(&[87,105,110,100,111,119,115,46,83,121,115,116,101,109,46,82,101,109,111,116,101,83,121,115,116,101,109,115,46,82,101,109,111,116,101,83,121,115,116,101,109,83,101,115,115,105,111,110,67,111,110,116,114,111,108,108,101,114,0]) [CLSID_RemoteSystemSessionController]);
DEFINE_IID!(IID_IRemoteSystemSessionControllerFactory, 3217829739, 44093, 16793, 130, 205, 102, 112, 167, 115, 239, 46);
RT_INTERFACE!{static interface IRemoteSystemSessionControllerFactory(IRemoteSystemSessionControllerFactoryVtbl): IInspectable(IInspectableVtbl) [IID_IRemoteSystemSessionControllerFactory] {
    fn CreateController(&self, displayName: HSTRING, out: *mut *mut RemoteSystemSessionController) -> HRESULT,
    fn CreateControllerWithSessionOptions(&self, displayName: HSTRING, options: *mut RemoteSystemSessionOptions, out: *mut *mut RemoteSystemSessionController) -> HRESULT
}}
impl IRemoteSystemSessionControllerFactory {
    #[inline] pub unsafe fn create_controller(&self, displayName: &HStringArg) -> Result<ComPtr<RemoteSystemSessionController>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).CreateController)(self as *const _ as *mut _, displayName.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn create_controller_with_session_options(&self, displayName: &HStringArg, options: &RemoteSystemSessionOptions) -> Result<ComPtr<RemoteSystemSessionController>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).CreateControllerWithSessionOptions)(self as *const _ as *mut _, displayName.get(), options as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
DEFINE_IID!(IID_IRemoteSystemSessionCreationResult, 2811761346, 14302, 17548, 139, 131, 163, 10, 163, 196, 234, 214);
RT_INTERFACE!{interface IRemoteSystemSessionCreationResult(IRemoteSystemSessionCreationResultVtbl): IInspectable(IInspectableVtbl) [IID_IRemoteSystemSessionCreationResult] {
    fn get_Status(&self, out: *mut RemoteSystemSessionCreationStatus) -> HRESULT,
    fn get_Session(&self, out: *mut *mut RemoteSystemSession) -> HRESULT
}}
impl IRemoteSystemSessionCreationResult {
    #[inline] pub unsafe fn get_status(&self) -> Result<RemoteSystemSessionCreationStatus> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_Status)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_session(&self) -> Result<ComPtr<RemoteSystemSession>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Session)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
RT_CLASS!{class RemoteSystemSessionCreationResult: IRemoteSystemSessionCreationResult}
RT_ENUM! { enum RemoteSystemSessionCreationStatus: i32 {
    Success (RemoteSystemSessionCreationStatus_Success) = 0, SessionLimitsExceeded (RemoteSystemSessionCreationStatus_SessionLimitsExceeded) = 1, OperationAborted (RemoteSystemSessionCreationStatus_OperationAborted) = 2,
}}
DEFINE_IID!(IID_IRemoteSystemSessionDisconnectedEventArgs, 3725313691, 30661, 17948, 130, 9, 124, 108, 93, 49, 17, 171);
RT_INTERFACE!{interface IRemoteSystemSessionDisconnectedEventArgs(IRemoteSystemSessionDisconnectedEventArgsVtbl): IInspectable(IInspectableVtbl) [IID_IRemoteSystemSessionDisconnectedEventArgs] {
    fn get_Reason(&self, out: *mut RemoteSystemSessionDisconnectedReason) -> HRESULT
}}
impl IRemoteSystemSessionDisconnectedEventArgs {
    #[inline] pub unsafe fn get_reason(&self) -> Result<RemoteSystemSessionDisconnectedReason> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_Reason)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
}
RT_CLASS!{class RemoteSystemSessionDisconnectedEventArgs: IRemoteSystemSessionDisconnectedEventArgs}
RT_ENUM! { enum RemoteSystemSessionDisconnectedReason: i32 {
    SessionUnavailable (RemoteSystemSessionDisconnectedReason_SessionUnavailable) = 0, RemovedByController (RemoteSystemSessionDisconnectedReason_RemovedByController) = 1, SessionClosed (RemoteSystemSessionDisconnectedReason_SessionClosed) = 2,
}}
DEFINE_IID!(IID_IRemoteSystemSessionInfo, 4283299400, 35594, 20122, 153, 5, 105, 228, 184, 65, 197, 136);
RT_INTERFACE!{interface IRemoteSystemSessionInfo(IRemoteSystemSessionInfoVtbl): IInspectable(IInspectableVtbl) [IID_IRemoteSystemSessionInfo] {
    fn get_DisplayName(&self, out: *mut HSTRING) -> HRESULT,
    fn get_ControllerDisplayName(&self, out: *mut HSTRING) -> HRESULT,
    fn JoinAsync(&self, out: *mut *mut super::super::foundation::IAsyncOperation<RemoteSystemSessionJoinResult>) -> HRESULT
}}
impl IRemoteSystemSessionInfo {
    #[inline] pub unsafe fn get_display_name(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_DisplayName)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_controller_display_name(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_ControllerDisplayName)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn join_async(&self) -> Result<ComPtr<super::super::foundation::IAsyncOperation<RemoteSystemSessionJoinResult>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).JoinAsync)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
RT_CLASS!{class RemoteSystemSessionInfo: IRemoteSystemSessionInfo}
DEFINE_IID!(IID_IRemoteSystemSessionInvitation, 1043516561, 20951, 18278, 161, 33, 37, 81, 108, 59, 130, 148);
RT_INTERFACE!{interface IRemoteSystemSessionInvitation(IRemoteSystemSessionInvitationVtbl): IInspectable(IInspectableVtbl) [IID_IRemoteSystemSessionInvitation] {
    fn get_Sender(&self, out: *mut *mut RemoteSystem) -> HRESULT,
    fn get_SessionInfo(&self, out: *mut *mut RemoteSystemSessionInfo) -> HRESULT
}}
impl IRemoteSystemSessionInvitation {
    #[inline] pub unsafe fn get_sender(&self) -> Result<ComPtr<RemoteSystem>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Sender)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_session_info(&self) -> Result<ComPtr<RemoteSystemSessionInfo>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_SessionInfo)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
RT_CLASS!{class RemoteSystemSessionInvitation: IRemoteSystemSessionInvitation}
DEFINE_IID!(IID_IRemoteSystemSessionInvitationListener, 150208575, 48241, 18913, 135, 74, 49, 221, 255, 154, 39, 185);
RT_INTERFACE!{interface IRemoteSystemSessionInvitationListener(IRemoteSystemSessionInvitationListenerVtbl): IInspectable(IInspectableVtbl) [IID_IRemoteSystemSessionInvitationListener] {
    fn add_InvitationReceived(&self, handler: *mut super::super::foundation::TypedEventHandler<RemoteSystemSessionInvitationListener, RemoteSystemSessionInvitationReceivedEventArgs>, out: *mut super::super::foundation::EventRegistrationToken) -> HRESULT,
    fn remove_InvitationReceived(&self, token: super::super::foundation::EventRegistrationToken) -> HRESULT
}}
impl IRemoteSystemSessionInvitationListener {
    #[inline] pub unsafe fn add_invitation_received(&self, handler: &super::super::foundation::TypedEventHandler<RemoteSystemSessionInvitationListener, RemoteSystemSessionInvitationReceivedEventArgs>) -> Result<super::super::foundation::EventRegistrationToken> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).add_InvitationReceived)(self as *const _ as *mut _, handler as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn remove_invitation_received(&self, token: super::super::foundation::EventRegistrationToken) -> Result<()> {
        let hr = ((*self.lpVtbl).remove_InvitationReceived)(self as *const _ as *mut _, token);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
}
RT_CLASS!{class RemoteSystemSessionInvitationListener: IRemoteSystemSessionInvitationListener}
impl RtActivatable<IActivationFactory> for RemoteSystemSessionInvitationListener {}
DEFINE_CLSID!(RemoteSystemSessionInvitationListener(&[87,105,110,100,111,119,115,46,83,121,115,116,101,109,46,82,101,109,111,116,101,83,121,115,116,101,109,115,46,82,101,109,111,116,101,83,121,115,116,101,109,83,101,115,115,105,111,110,73,110,118,105,116,97,116,105,111,110,76,105,115,116,101,110,101,114,0]) [CLSID_RemoteSystemSessionInvitationListener]);
DEFINE_IID!(IID_IRemoteSystemSessionInvitationReceivedEventArgs, 1586907693, 41229, 20187, 141, 234, 84, 210, 10, 193, 149, 67);
RT_INTERFACE!{interface IRemoteSystemSessionInvitationReceivedEventArgs(IRemoteSystemSessionInvitationReceivedEventArgsVtbl): IInspectable(IInspectableVtbl) [IID_IRemoteSystemSessionInvitationReceivedEventArgs] {
    fn get_Invitation(&self, out: *mut *mut RemoteSystemSessionInvitation) -> HRESULT
}}
impl IRemoteSystemSessionInvitationReceivedEventArgs {
    #[inline] pub unsafe fn get_invitation(&self) -> Result<ComPtr<RemoteSystemSessionInvitation>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Invitation)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
RT_CLASS!{class RemoteSystemSessionInvitationReceivedEventArgs: IRemoteSystemSessionInvitationReceivedEventArgs}
DEFINE_IID!(IID_IRemoteSystemSessionJoinRequest, 543162472, 31124, 17201, 134, 209, 216, 157, 136, 37, 133, 238);
RT_INTERFACE!{interface IRemoteSystemSessionJoinRequest(IRemoteSystemSessionJoinRequestVtbl): IInspectable(IInspectableVtbl) [IID_IRemoteSystemSessionJoinRequest] {
    fn get_Participant(&self, out: *mut *mut RemoteSystemSessionParticipant) -> HRESULT,
    fn Accept(&self) -> HRESULT
}}
impl IRemoteSystemSessionJoinRequest {
    #[inline] pub unsafe fn get_participant(&self) -> Result<ComPtr<RemoteSystemSessionParticipant>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Participant)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn accept(&self) -> Result<()> {
        let hr = ((*self.lpVtbl).Accept)(self as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
}
RT_CLASS!{class RemoteSystemSessionJoinRequest: IRemoteSystemSessionJoinRequest}
DEFINE_IID!(IID_IRemoteSystemSessionJoinRequestedEventArgs, 3687468995, 33465, 18454, 156, 36, 228, 14, 97, 119, 75, 216);
RT_INTERFACE!{interface IRemoteSystemSessionJoinRequestedEventArgs(IRemoteSystemSessionJoinRequestedEventArgsVtbl): IInspectable(IInspectableVtbl) [IID_IRemoteSystemSessionJoinRequestedEventArgs] {
    fn get_JoinRequest(&self, out: *mut *mut RemoteSystemSessionJoinRequest) -> HRESULT,
    fn GetDeferral(&self, out: *mut *mut super::super::foundation::Deferral) -> HRESULT
}}
impl IRemoteSystemSessionJoinRequestedEventArgs {
    #[inline] pub unsafe fn get_join_request(&self) -> Result<ComPtr<RemoteSystemSessionJoinRequest>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_JoinRequest)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_deferral(&self) -> Result<ComPtr<super::super::foundation::Deferral>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetDeferral)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
RT_CLASS!{class RemoteSystemSessionJoinRequestedEventArgs: IRemoteSystemSessionJoinRequestedEventArgs}
DEFINE_IID!(IID_IRemoteSystemSessionJoinResult, 3464175364, 41022, 16804, 144, 11, 30, 121, 50, 140, 18, 103);
RT_INTERFACE!{interface IRemoteSystemSessionJoinResult(IRemoteSystemSessionJoinResultVtbl): IInspectable(IInspectableVtbl) [IID_IRemoteSystemSessionJoinResult] {
    fn get_Status(&self, out: *mut RemoteSystemSessionJoinStatus) -> HRESULT,
    fn get_Session(&self, out: *mut *mut RemoteSystemSession) -> HRESULT
}}
impl IRemoteSystemSessionJoinResult {
    #[inline] pub unsafe fn get_status(&self) -> Result<RemoteSystemSessionJoinStatus> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_Status)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_session(&self) -> Result<ComPtr<RemoteSystemSession>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Session)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
RT_CLASS!{class RemoteSystemSessionJoinResult: IRemoteSystemSessionJoinResult}
RT_ENUM! { enum RemoteSystemSessionJoinStatus: i32 {
    Success (RemoteSystemSessionJoinStatus_Success) = 0, SessionLimitsExceeded (RemoteSystemSessionJoinStatus_SessionLimitsExceeded) = 1, OperationAborted (RemoteSystemSessionJoinStatus_OperationAborted) = 2, SessionUnavailable (RemoteSystemSessionJoinStatus_SessionUnavailable) = 3, RejectedByController (RemoteSystemSessionJoinStatus_RejectedByController) = 4,
}}
DEFINE_IID!(IID_IRemoteSystemSessionMessageChannel, 2502218026, 29657, 19472, 183, 81, 194, 103, 132, 67, 113, 39);
RT_INTERFACE!{interface IRemoteSystemSessionMessageChannel(IRemoteSystemSessionMessageChannelVtbl): IInspectable(IInspectableVtbl) [IID_IRemoteSystemSessionMessageChannel] {
    fn get_Session(&self, out: *mut *mut RemoteSystemSession) -> HRESULT,
    fn BroadcastValueSetAsync(&self, messageData: *mut super::super::foundation::collections::ValueSet, out: *mut *mut super::super::foundation::IAsyncOperation<bool>) -> HRESULT,
    fn SendValueSetAsync(&self, messageData: *mut super::super::foundation::collections::ValueSet, participant: *mut RemoteSystemSessionParticipant, out: *mut *mut super::super::foundation::IAsyncOperation<bool>) -> HRESULT,
    fn SendValueSetToParticipantsAsync(&self, messageData: *mut super::super::foundation::collections::ValueSet, participants: *mut super::super::foundation::collections::IIterable<RemoteSystemSessionParticipant>, out: *mut *mut super::super::foundation::IAsyncOperation<bool>) -> HRESULT,
    fn add_ValueSetReceived(&self, handler: *mut super::super::foundation::TypedEventHandler<RemoteSystemSessionMessageChannel, RemoteSystemSessionValueSetReceivedEventArgs>, out: *mut super::super::foundation::EventRegistrationToken) -> HRESULT,
    fn remove_ValueSetReceived(&self, token: super::super::foundation::EventRegistrationToken) -> HRESULT
}}
impl IRemoteSystemSessionMessageChannel {
    #[inline] pub unsafe fn get_session(&self) -> Result<ComPtr<RemoteSystemSession>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Session)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn broadcast_value_set_async(&self, messageData: &super::super::foundation::collections::ValueSet) -> Result<ComPtr<super::super::foundation::IAsyncOperation<bool>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).BroadcastValueSetAsync)(self as *const _ as *mut _, messageData as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn send_value_set_async(&self, messageData: &super::super::foundation::collections::ValueSet, participant: &RemoteSystemSessionParticipant) -> Result<ComPtr<super::super::foundation::IAsyncOperation<bool>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).SendValueSetAsync)(self as *const _ as *mut _, messageData as *const _ as *mut _, participant as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn send_value_set_to_participants_async(&self, messageData: &super::super::foundation::collections::ValueSet, participants: &super::super::foundation::collections::IIterable<RemoteSystemSessionParticipant>) -> Result<ComPtr<super::super::foundation::IAsyncOperation<bool>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).SendValueSetToParticipantsAsync)(self as *const _ as *mut _, messageData as *const _ as *mut _, participants as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn add_value_set_received(&self, handler: &super::super::foundation::TypedEventHandler<RemoteSystemSessionMessageChannel, RemoteSystemSessionValueSetReceivedEventArgs>) -> Result<super::super::foundation::EventRegistrationToken> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).add_ValueSetReceived)(self as *const _ as *mut _, handler as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn remove_value_set_received(&self, token: super::super::foundation::EventRegistrationToken) -> Result<()> {
        let hr = ((*self.lpVtbl).remove_ValueSetReceived)(self as *const _ as *mut _, token);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
}
RT_CLASS!{class RemoteSystemSessionMessageChannel: IRemoteSystemSessionMessageChannel}
impl RtActivatable<IRemoteSystemSessionMessageChannelFactory> for RemoteSystemSessionMessageChannel {}
impl RemoteSystemSessionMessageChannel {
    #[inline] pub fn create(session: &RemoteSystemSession, channelName: &HStringArg) -> Result<ComPtr<RemoteSystemSessionMessageChannel>> { unsafe {
        <Self as RtActivatable<IRemoteSystemSessionMessageChannelFactory>>::get_activation_factory().create(session, channelName)
    }}
    #[inline] pub fn create_with_reliability(session: &RemoteSystemSession, channelName: &HStringArg, reliability: RemoteSystemSessionMessageChannelReliability) -> Result<ComPtr<RemoteSystemSessionMessageChannel>> { unsafe {
        <Self as RtActivatable<IRemoteSystemSessionMessageChannelFactory>>::get_activation_factory().create_with_reliability(session, channelName, reliability)
    }}
}
DEFINE_CLSID!(RemoteSystemSessionMessageChannel(&[87,105,110,100,111,119,115,46,83,121,115,116,101,109,46,82,101,109,111,116,101,83,121,115,116,101,109,115,46,82,101,109,111,116,101,83,121,115,116,101,109,83,101,115,115,105,111,110,77,101,115,115,97,103,101,67,104,97,110,110,101,108,0]) [CLSID_RemoteSystemSessionMessageChannel]);
DEFINE_IID!(IID_IRemoteSystemSessionMessageChannelFactory, 694033482, 48406, 17048, 183, 206, 65, 84, 130, 176, 225, 29);
RT_INTERFACE!{static interface IRemoteSystemSessionMessageChannelFactory(IRemoteSystemSessionMessageChannelFactoryVtbl): IInspectable(IInspectableVtbl) [IID_IRemoteSystemSessionMessageChannelFactory] {
    fn Create(&self, session: *mut RemoteSystemSession, channelName: HSTRING, out: *mut *mut RemoteSystemSessionMessageChannel) -> HRESULT,
    fn CreateWithReliability(&self, session: *mut RemoteSystemSession, channelName: HSTRING, reliability: RemoteSystemSessionMessageChannelReliability, out: *mut *mut RemoteSystemSessionMessageChannel) -> HRESULT
}}
impl IRemoteSystemSessionMessageChannelFactory {
    #[inline] pub unsafe fn create(&self, session: &RemoteSystemSession, channelName: &HStringArg) -> Result<ComPtr<RemoteSystemSessionMessageChannel>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).Create)(self as *const _ as *mut _, session as *const _ as *mut _, channelName.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn create_with_reliability(&self, session: &RemoteSystemSession, channelName: &HStringArg, reliability: RemoteSystemSessionMessageChannelReliability) -> Result<ComPtr<RemoteSystemSessionMessageChannel>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).CreateWithReliability)(self as *const _ as *mut _, session as *const _ as *mut _, channelName.get(), reliability, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
RT_ENUM! { enum RemoteSystemSessionMessageChannelReliability: i32 {
    Reliable (RemoteSystemSessionMessageChannelReliability_Reliable) = 0, Unreliable (RemoteSystemSessionMessageChannelReliability_Unreliable) = 1,
}}
DEFINE_IID!(IID_IRemoteSystemSessionOptions, 1947129685, 33816, 20225, 147, 83, 226, 28, 158, 204, 108, 252);
RT_INTERFACE!{interface IRemoteSystemSessionOptions(IRemoteSystemSessionOptionsVtbl): IInspectable(IInspectableVtbl) [IID_IRemoteSystemSessionOptions] {
    fn get_IsInviteOnly(&self, out: *mut bool) -> HRESULT,
    fn put_IsInviteOnly(&self, value: bool) -> HRESULT
}}
impl IRemoteSystemSessionOptions {
    #[inline] pub unsafe fn get_is_invite_only(&self) -> Result<bool> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_IsInviteOnly)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_is_invite_only(&self, value: bool) -> Result<()> {
        let hr = ((*self.lpVtbl).put_IsInviteOnly)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
}
RT_CLASS!{class RemoteSystemSessionOptions: IRemoteSystemSessionOptions}
impl RtActivatable<IActivationFactory> for RemoteSystemSessionOptions {}
DEFINE_CLSID!(RemoteSystemSessionOptions(&[87,105,110,100,111,119,115,46,83,121,115,116,101,109,46,82,101,109,111,116,101,83,121,115,116,101,109,115,46,82,101,109,111,116,101,83,121,115,116,101,109,83,101,115,115,105,111,110,79,112,116,105,111,110,115,0]) [CLSID_RemoteSystemSessionOptions]);
DEFINE_IID!(IID_IRemoteSystemSessionParticipant, 2123367820, 44281, 18217, 138, 23, 68, 231, 186, 237, 93, 204);
RT_INTERFACE!{interface IRemoteSystemSessionParticipant(IRemoteSystemSessionParticipantVtbl): IInspectable(IInspectableVtbl) [IID_IRemoteSystemSessionParticipant] {
    fn get_RemoteSystem(&self, out: *mut *mut RemoteSystem) -> HRESULT,
    #[cfg(feature="windows-networking")] fn GetHostNames(&self, out: *mut *mut super::super::foundation::collections::IVectorView<super::super::networking::HostName>) -> HRESULT
}}
impl IRemoteSystemSessionParticipant {
    #[inline] pub unsafe fn get_remote_system(&self) -> Result<ComPtr<RemoteSystem>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_RemoteSystem)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[cfg(feature="windows-networking")] #[inline] pub unsafe fn get_host_names(&self) -> Result<ComPtr<super::super::foundation::collections::IVectorView<super::super::networking::HostName>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetHostNames)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
RT_CLASS!{class RemoteSystemSessionParticipant: IRemoteSystemSessionParticipant}
DEFINE_IID!(IID_IRemoteSystemSessionParticipantAddedEventArgs, 3545913304, 51617, 19383, 182, 176, 121, 187, 145, 173, 249, 61);
RT_INTERFACE!{interface IRemoteSystemSessionParticipantAddedEventArgs(IRemoteSystemSessionParticipantAddedEventArgsVtbl): IInspectable(IInspectableVtbl) [IID_IRemoteSystemSessionParticipantAddedEventArgs] {
    fn get_Participant(&self, out: *mut *mut RemoteSystemSessionParticipant) -> HRESULT
}}
impl IRemoteSystemSessionParticipantAddedEventArgs {
    #[inline] pub unsafe fn get_participant(&self) -> Result<ComPtr<RemoteSystemSessionParticipant>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Participant)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
RT_CLASS!{class RemoteSystemSessionParticipantAddedEventArgs: IRemoteSystemSessionParticipantAddedEventArgs}
DEFINE_IID!(IID_IRemoteSystemSessionParticipantRemovedEventArgs, 2255417480, 56936, 19135, 136, 161, 249, 13, 22, 39, 65, 146);
RT_INTERFACE!{interface IRemoteSystemSessionParticipantRemovedEventArgs(IRemoteSystemSessionParticipantRemovedEventArgsVtbl): IInspectable(IInspectableVtbl) [IID_IRemoteSystemSessionParticipantRemovedEventArgs] {
    fn get_Participant(&self, out: *mut *mut RemoteSystemSessionParticipant) -> HRESULT
}}
impl IRemoteSystemSessionParticipantRemovedEventArgs {
    #[inline] pub unsafe fn get_participant(&self) -> Result<ComPtr<RemoteSystemSessionParticipant>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Participant)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
RT_CLASS!{class RemoteSystemSessionParticipantRemovedEventArgs: IRemoteSystemSessionParticipantRemovedEventArgs}
DEFINE_IID!(IID_IRemoteSystemSessionParticipantWatcher, 3705471692, 43655, 19833, 182, 204, 68, 89, 179, 233, 32, 117);
RT_INTERFACE!{interface IRemoteSystemSessionParticipantWatcher(IRemoteSystemSessionParticipantWatcherVtbl): IInspectable(IInspectableVtbl) [IID_IRemoteSystemSessionParticipantWatcher] {
    fn Start(&self) -> HRESULT,
    fn Stop(&self) -> HRESULT,
    fn get_Status(&self, out: *mut RemoteSystemSessionParticipantWatcherStatus) -> HRESULT,
    fn add_Added(&self, handler: *mut super::super::foundation::TypedEventHandler<RemoteSystemSessionParticipantWatcher, RemoteSystemSessionParticipantAddedEventArgs>, out: *mut super::super::foundation::EventRegistrationToken) -> HRESULT,
    fn remove_Added(&self, token: super::super::foundation::EventRegistrationToken) -> HRESULT,
    fn add_Removed(&self, handler: *mut super::super::foundation::TypedEventHandler<RemoteSystemSessionParticipantWatcher, RemoteSystemSessionParticipantRemovedEventArgs>, out: *mut super::super::foundation::EventRegistrationToken) -> HRESULT,
    fn remove_Removed(&self, token: super::super::foundation::EventRegistrationToken) -> HRESULT,
    fn add_EnumerationCompleted(&self, handler: *mut super::super::foundation::TypedEventHandler<RemoteSystemSessionParticipantWatcher, IInspectable>, out: *mut super::super::foundation::EventRegistrationToken) -> HRESULT,
    fn remove_EnumerationCompleted(&self, token: super::super::foundation::EventRegistrationToken) -> HRESULT
}}
impl IRemoteSystemSessionParticipantWatcher {
    #[inline] pub unsafe fn start(&self) -> Result<()> {
        let hr = ((*self.lpVtbl).Start)(self as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn stop(&self) -> Result<()> {
        let hr = ((*self.lpVtbl).Stop)(self as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_status(&self) -> Result<RemoteSystemSessionParticipantWatcherStatus> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_Status)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn add_added(&self, handler: &super::super::foundation::TypedEventHandler<RemoteSystemSessionParticipantWatcher, RemoteSystemSessionParticipantAddedEventArgs>) -> Result<super::super::foundation::EventRegistrationToken> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).add_Added)(self as *const _ as *mut _, handler as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn remove_added(&self, token: super::super::foundation::EventRegistrationToken) -> Result<()> {
        let hr = ((*self.lpVtbl).remove_Added)(self as *const _ as *mut _, token);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn add_removed(&self, handler: &super::super::foundation::TypedEventHandler<RemoteSystemSessionParticipantWatcher, RemoteSystemSessionParticipantRemovedEventArgs>) -> Result<super::super::foundation::EventRegistrationToken> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).add_Removed)(self as *const _ as *mut _, handler as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn remove_removed(&self, token: super::super::foundation::EventRegistrationToken) -> Result<()> {
        let hr = ((*self.lpVtbl).remove_Removed)(self as *const _ as *mut _, token);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn add_enumeration_completed(&self, handler: &super::super::foundation::TypedEventHandler<RemoteSystemSessionParticipantWatcher, IInspectable>) -> Result<super::super::foundation::EventRegistrationToken> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).add_EnumerationCompleted)(self as *const _ as *mut _, handler as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn remove_enumeration_completed(&self, token: super::super::foundation::EventRegistrationToken) -> Result<()> {
        let hr = ((*self.lpVtbl).remove_EnumerationCompleted)(self as *const _ as *mut _, token);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
}
RT_CLASS!{class RemoteSystemSessionParticipantWatcher: IRemoteSystemSessionParticipantWatcher}
RT_ENUM! { enum RemoteSystemSessionParticipantWatcherStatus: i32 {
    Created (RemoteSystemSessionParticipantWatcherStatus_Created) = 0, Started (RemoteSystemSessionParticipantWatcherStatus_Started) = 1, EnumerationCompleted (RemoteSystemSessionParticipantWatcherStatus_EnumerationCompleted) = 2, Stopping (RemoteSystemSessionParticipantWatcherStatus_Stopping) = 3, Stopped (RemoteSystemSessionParticipantWatcherStatus_Stopped) = 4, Aborted (RemoteSystemSessionParticipantWatcherStatus_Aborted) = 5,
}}
DEFINE_IID!(IID_IRemoteSystemSessionRemovedEventArgs, 2944569678, 14753, 19946, 157, 99, 67, 121, 141, 91, 187, 208);
RT_INTERFACE!{interface IRemoteSystemSessionRemovedEventArgs(IRemoteSystemSessionRemovedEventArgsVtbl): IInspectable(IInspectableVtbl) [IID_IRemoteSystemSessionRemovedEventArgs] {
    fn get_SessionInfo(&self, out: *mut *mut RemoteSystemSessionInfo) -> HRESULT
}}
impl IRemoteSystemSessionRemovedEventArgs {
    #[inline] pub unsafe fn get_session_info(&self) -> Result<ComPtr<RemoteSystemSessionInfo>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_SessionInfo)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
RT_CLASS!{class RemoteSystemSessionRemovedEventArgs: IRemoteSystemSessionRemovedEventArgs}
DEFINE_IID!(IID_IRemoteSystemSessionStatics, 2233764255, 64800, 17635, 149, 101, 231, 90, 59, 20, 198, 110);
RT_INTERFACE!{static interface IRemoteSystemSessionStatics(IRemoteSystemSessionStaticsVtbl): IInspectable(IInspectableVtbl) [IID_IRemoteSystemSessionStatics] {
    fn CreateWatcher(&self, out: *mut *mut RemoteSystemSessionWatcher) -> HRESULT
}}
impl IRemoteSystemSessionStatics {
    #[inline] pub unsafe fn create_watcher(&self) -> Result<ComPtr<RemoteSystemSessionWatcher>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).CreateWatcher)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
DEFINE_IID!(IID_IRemoteSystemSessionUpdatedEventArgs, 377966697, 8990, 19601, 142, 200, 179, 163, 157, 158, 85, 163);
RT_INTERFACE!{interface IRemoteSystemSessionUpdatedEventArgs(IRemoteSystemSessionUpdatedEventArgsVtbl): IInspectable(IInspectableVtbl) [IID_IRemoteSystemSessionUpdatedEventArgs] {
    fn get_SessionInfo(&self, out: *mut *mut RemoteSystemSessionInfo) -> HRESULT
}}
impl IRemoteSystemSessionUpdatedEventArgs {
    #[inline] pub unsafe fn get_session_info(&self) -> Result<ComPtr<RemoteSystemSessionInfo>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_SessionInfo)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
RT_CLASS!{class RemoteSystemSessionUpdatedEventArgs: IRemoteSystemSessionUpdatedEventArgs}
DEFINE_IID!(IID_IRemoteSystemSessionValueSetReceivedEventArgs, 116594565, 11685, 20056, 167, 143, 158, 141, 7, 132, 238, 37);
RT_INTERFACE!{interface IRemoteSystemSessionValueSetReceivedEventArgs(IRemoteSystemSessionValueSetReceivedEventArgsVtbl): IInspectable(IInspectableVtbl) [IID_IRemoteSystemSessionValueSetReceivedEventArgs] {
    fn get_Sender(&self, out: *mut *mut RemoteSystemSessionParticipant) -> HRESULT,
    fn get_Message(&self, out: *mut *mut super::super::foundation::collections::ValueSet) -> HRESULT
}}
impl IRemoteSystemSessionValueSetReceivedEventArgs {
    #[inline] pub unsafe fn get_sender(&self) -> Result<ComPtr<RemoteSystemSessionParticipant>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Sender)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_message(&self) -> Result<ComPtr<super::super::foundation::collections::ValueSet>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Message)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
RT_CLASS!{class RemoteSystemSessionValueSetReceivedEventArgs: IRemoteSystemSessionValueSetReceivedEventArgs}
DEFINE_IID!(IID_IRemoteSystemSessionWatcher, 2147738432, 3137, 19042, 182, 215, 189, 190, 43, 25, 190, 45);
RT_INTERFACE!{interface IRemoteSystemSessionWatcher(IRemoteSystemSessionWatcherVtbl): IInspectable(IInspectableVtbl) [IID_IRemoteSystemSessionWatcher] {
    fn Start(&self) -> HRESULT,
    fn Stop(&self) -> HRESULT,
    fn get_Status(&self, out: *mut RemoteSystemSessionWatcherStatus) -> HRESULT,
    fn add_Added(&self, handler: *mut super::super::foundation::TypedEventHandler<RemoteSystemSessionWatcher, RemoteSystemSessionAddedEventArgs>, out: *mut super::super::foundation::EventRegistrationToken) -> HRESULT,
    fn remove_Added(&self, token: super::super::foundation::EventRegistrationToken) -> HRESULT,
    fn add_Updated(&self, handler: *mut super::super::foundation::TypedEventHandler<RemoteSystemSessionWatcher, RemoteSystemSessionUpdatedEventArgs>, out: *mut super::super::foundation::EventRegistrationToken) -> HRESULT,
    fn remove_Updated(&self, token: super::super::foundation::EventRegistrationToken) -> HRESULT,
    fn add_Removed(&self, handler: *mut super::super::foundation::TypedEventHandler<RemoteSystemSessionWatcher, RemoteSystemSessionRemovedEventArgs>, out: *mut super::super::foundation::EventRegistrationToken) -> HRESULT,
    fn remove_Removed(&self, token: super::super::foundation::EventRegistrationToken) -> HRESULT
}}
impl IRemoteSystemSessionWatcher {
    #[inline] pub unsafe fn start(&self) -> Result<()> {
        let hr = ((*self.lpVtbl).Start)(self as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn stop(&self) -> Result<()> {
        let hr = ((*self.lpVtbl).Stop)(self as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_status(&self) -> Result<RemoteSystemSessionWatcherStatus> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_Status)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn add_added(&self, handler: &super::super::foundation::TypedEventHandler<RemoteSystemSessionWatcher, RemoteSystemSessionAddedEventArgs>) -> Result<super::super::foundation::EventRegistrationToken> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).add_Added)(self as *const _ as *mut _, handler as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn remove_added(&self, token: super::super::foundation::EventRegistrationToken) -> Result<()> {
        let hr = ((*self.lpVtbl).remove_Added)(self as *const _ as *mut _, token);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn add_updated(&self, handler: &super::super::foundation::TypedEventHandler<RemoteSystemSessionWatcher, RemoteSystemSessionUpdatedEventArgs>) -> Result<super::super::foundation::EventRegistrationToken> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).add_Updated)(self as *const _ as *mut _, handler as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn remove_updated(&self, token: super::super::foundation::EventRegistrationToken) -> Result<()> {
        let hr = ((*self.lpVtbl).remove_Updated)(self as *const _ as *mut _, token);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn add_removed(&self, handler: &super::super::foundation::TypedEventHandler<RemoteSystemSessionWatcher, RemoteSystemSessionRemovedEventArgs>) -> Result<super::super::foundation::EventRegistrationToken> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).add_Removed)(self as *const _ as *mut _, handler as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn remove_removed(&self, token: super::super::foundation::EventRegistrationToken) -> Result<()> {
        let hr = ((*self.lpVtbl).remove_Removed)(self as *const _ as *mut _, token);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
}
RT_CLASS!{class RemoteSystemSessionWatcher: IRemoteSystemSessionWatcher}
RT_ENUM! { enum RemoteSystemSessionWatcherStatus: i32 {
    Created (RemoteSystemSessionWatcherStatus_Created) = 0, Started (RemoteSystemSessionWatcherStatus_Started) = 1, EnumerationCompleted (RemoteSystemSessionWatcherStatus_EnumerationCompleted) = 2, Stopping (RemoteSystemSessionWatcherStatus_Stopping) = 3, Stopped (RemoteSystemSessionWatcherStatus_Stopped) = 4, Aborted (RemoteSystemSessionWatcherStatus_Aborted) = 5,
}}
DEFINE_IID!(IID_IRemoteSystemStatics, 2760225682, 65323, 19271, 190, 98, 116, 63, 47, 20, 15, 48);
RT_INTERFACE!{static interface IRemoteSystemStatics(IRemoteSystemStaticsVtbl): IInspectable(IInspectableVtbl) [IID_IRemoteSystemStatics] {
    #[cfg(not(feature="windows-networking"))] fn __Dummy0(&self) -> (),
    #[cfg(feature="windows-networking")] fn FindByHostNameAsync(&self, hostName: *mut super::super::networking::HostName, out: *mut *mut super::super::foundation::IAsyncOperation<RemoteSystem>) -> HRESULT,
    fn CreateWatcher(&self, out: *mut *mut RemoteSystemWatcher) -> HRESULT,
    fn CreateWatcherWithFilters(&self, filters: *mut super::super::foundation::collections::IIterable<IRemoteSystemFilter>, out: *mut *mut RemoteSystemWatcher) -> HRESULT,
    fn RequestAccessAsync(&self, out: *mut *mut super::super::foundation::IAsyncOperation<RemoteSystemAccessStatus>) -> HRESULT
}}
impl IRemoteSystemStatics {
    #[cfg(feature="windows-networking")] #[inline] pub unsafe fn find_by_host_name_async(&self, hostName: &super::super::networking::HostName) -> Result<ComPtr<super::super::foundation::IAsyncOperation<RemoteSystem>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).FindByHostNameAsync)(self as *const _ as *mut _, hostName as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn create_watcher(&self) -> Result<ComPtr<RemoteSystemWatcher>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).CreateWatcher)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn create_watcher_with_filters(&self, filters: &super::super::foundation::collections::IIterable<IRemoteSystemFilter>) -> Result<ComPtr<RemoteSystemWatcher>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).CreateWatcherWithFilters)(self as *const _ as *mut _, filters as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn request_access_async(&self) -> Result<ComPtr<super::super::foundation::IAsyncOperation<RemoteSystemAccessStatus>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).RequestAccessAsync)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
DEFINE_IID!(IID_IRemoteSystemStatics2, 211348938, 28569, 19538, 162, 114, 234, 79, 54, 71, 23, 68);
RT_INTERFACE!{static interface IRemoteSystemStatics2(IRemoteSystemStatics2Vtbl): IInspectable(IInspectableVtbl) [IID_IRemoteSystemStatics2] {
    fn IsAuthorizationKindEnabled(&self, kind: RemoteSystemAuthorizationKind, out: *mut bool) -> HRESULT
}}
impl IRemoteSystemStatics2 {
    #[inline] pub unsafe fn is_authorization_kind_enabled(&self, kind: RemoteSystemAuthorizationKind) -> Result<bool> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).IsAuthorizationKindEnabled)(self as *const _ as *mut _, kind, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
}
RT_ENUM! { enum RemoteSystemStatus: i32 {
    Unavailable (RemoteSystemStatus_Unavailable) = 0, DiscoveringAvailability (RemoteSystemStatus_DiscoveringAvailability) = 1, Available (RemoteSystemStatus_Available) = 2, Unknown (RemoteSystemStatus_Unknown) = 3,
}}
RT_ENUM! { enum RemoteSystemStatusType: i32 {
    Any (RemoteSystemStatusType_Any) = 0, Available (RemoteSystemStatusType_Available) = 1,
}}
DEFINE_IID!(IID_IRemoteSystemStatusTypeFilter, 205082958, 52150, 18295, 133, 52, 46, 12, 82, 26, 255, 162);
RT_INTERFACE!{interface IRemoteSystemStatusTypeFilter(IRemoteSystemStatusTypeFilterVtbl): IInspectable(IInspectableVtbl) [IID_IRemoteSystemStatusTypeFilter] {
    fn get_RemoteSystemStatusType(&self, out: *mut RemoteSystemStatusType) -> HRESULT
}}
impl IRemoteSystemStatusTypeFilter {
    #[inline] pub unsafe fn get_remote_system_status_type(&self) -> Result<RemoteSystemStatusType> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_RemoteSystemStatusType)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
}
RT_CLASS!{class RemoteSystemStatusTypeFilter: IRemoteSystemStatusTypeFilter}
impl RtActivatable<IRemoteSystemStatusTypeFilterFactory> for RemoteSystemStatusTypeFilter {}
impl RemoteSystemStatusTypeFilter {
    #[inline] pub fn create(remoteSystemStatusType: RemoteSystemStatusType) -> Result<ComPtr<RemoteSystemStatusTypeFilter>> { unsafe {
        <Self as RtActivatable<IRemoteSystemStatusTypeFilterFactory>>::get_activation_factory().create(remoteSystemStatusType)
    }}
}
DEFINE_CLSID!(RemoteSystemStatusTypeFilter(&[87,105,110,100,111,119,115,46,83,121,115,116,101,109,46,82,101,109,111,116,101,83,121,115,116,101,109,115,46,82,101,109,111,116,101,83,121,115,116,101,109,83,116,97,116,117,115,84,121,112,101,70,105,108,116,101,114,0]) [CLSID_RemoteSystemStatusTypeFilter]);
DEFINE_IID!(IID_IRemoteSystemStatusTypeFilterFactory, 869234938, 55076, 16677, 172, 122, 141, 40, 30, 68, 201, 73);
RT_INTERFACE!{static interface IRemoteSystemStatusTypeFilterFactory(IRemoteSystemStatusTypeFilterFactoryVtbl): IInspectable(IInspectableVtbl) [IID_IRemoteSystemStatusTypeFilterFactory] {
    fn Create(&self, remoteSystemStatusType: RemoteSystemStatusType, out: *mut *mut RemoteSystemStatusTypeFilter) -> HRESULT
}}
impl IRemoteSystemStatusTypeFilterFactory {
    #[inline] pub unsafe fn create(&self, remoteSystemStatusType: RemoteSystemStatusType) -> Result<ComPtr<RemoteSystemStatusTypeFilter>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).Create)(self as *const _ as *mut _, remoteSystemStatusType, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
DEFINE_IID!(IID_IRemoteSystemUpdatedEventArgs, 1963130638, 56267, 16725, 180, 202, 179, 10, 4, 242, 118, 39);
RT_INTERFACE!{interface IRemoteSystemUpdatedEventArgs(IRemoteSystemUpdatedEventArgsVtbl): IInspectable(IInspectableVtbl) [IID_IRemoteSystemUpdatedEventArgs] {
    fn get_RemoteSystem(&self, out: *mut *mut RemoteSystem) -> HRESULT
}}
impl IRemoteSystemUpdatedEventArgs {
    #[inline] pub unsafe fn get_remote_system(&self) -> Result<ComPtr<RemoteSystem>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_RemoteSystem)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
RT_CLASS!{class RemoteSystemUpdatedEventArgs: IRemoteSystemUpdatedEventArgs}
DEFINE_IID!(IID_IRemoteSystemWatcher, 1566575742, 11271, 18629, 136, 156, 69, 93, 43, 9, 151, 113);
RT_INTERFACE!{interface IRemoteSystemWatcher(IRemoteSystemWatcherVtbl): IInspectable(IInspectableVtbl) [IID_IRemoteSystemWatcher] {
    fn Start(&self) -> HRESULT,
    fn Stop(&self) -> HRESULT,
    fn add_RemoteSystemAdded(&self, handler: *mut super::super::foundation::TypedEventHandler<RemoteSystemWatcher, RemoteSystemAddedEventArgs>, out: *mut super::super::foundation::EventRegistrationToken) -> HRESULT,
    fn remove_RemoteSystemAdded(&self, token: super::super::foundation::EventRegistrationToken) -> HRESULT,
    fn add_RemoteSystemUpdated(&self, handler: *mut super::super::foundation::TypedEventHandler<RemoteSystemWatcher, RemoteSystemUpdatedEventArgs>, out: *mut super::super::foundation::EventRegistrationToken) -> HRESULT,
    fn remove_RemoteSystemUpdated(&self, token: super::super::foundation::EventRegistrationToken) -> HRESULT,
    fn add_RemoteSystemRemoved(&self, handler: *mut super::super::foundation::TypedEventHandler<RemoteSystemWatcher, RemoteSystemRemovedEventArgs>, out: *mut super::super::foundation::EventRegistrationToken) -> HRESULT,
    fn remove_RemoteSystemRemoved(&self, token: super::super::foundation::EventRegistrationToken) -> HRESULT
}}
impl IRemoteSystemWatcher {
    #[inline] pub unsafe fn start(&self) -> Result<()> {
        let hr = ((*self.lpVtbl).Start)(self as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn stop(&self) -> Result<()> {
        let hr = ((*self.lpVtbl).Stop)(self as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn add_remote_system_added(&self, handler: &super::super::foundation::TypedEventHandler<RemoteSystemWatcher, RemoteSystemAddedEventArgs>) -> Result<super::super::foundation::EventRegistrationToken> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).add_RemoteSystemAdded)(self as *const _ as *mut _, handler as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn remove_remote_system_added(&self, token: super::super::foundation::EventRegistrationToken) -> Result<()> {
        let hr = ((*self.lpVtbl).remove_RemoteSystemAdded)(self as *const _ as *mut _, token);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn add_remote_system_updated(&self, handler: &super::super::foundation::TypedEventHandler<RemoteSystemWatcher, RemoteSystemUpdatedEventArgs>) -> Result<super::super::foundation::EventRegistrationToken> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).add_RemoteSystemUpdated)(self as *const _ as *mut _, handler as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn remove_remote_system_updated(&self, token: super::super::foundation::EventRegistrationToken) -> Result<()> {
        let hr = ((*self.lpVtbl).remove_RemoteSystemUpdated)(self as *const _ as *mut _, token);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn add_remote_system_removed(&self, handler: &super::super::foundation::TypedEventHandler<RemoteSystemWatcher, RemoteSystemRemovedEventArgs>) -> Result<super::super::foundation::EventRegistrationToken> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).add_RemoteSystemRemoved)(self as *const _ as *mut _, handler as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn remove_remote_system_removed(&self, token: super::super::foundation::EventRegistrationToken) -> Result<()> {
        let hr = ((*self.lpVtbl).remove_RemoteSystemRemoved)(self as *const _ as *mut _, token);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
}
RT_CLASS!{class RemoteSystemWatcher: IRemoteSystemWatcher}
} // Windows.System.RemoteSystems
pub mod threading { // Windows.System.Threading
use ::prelude::*;
RT_CLASS!{static class ThreadPool}
impl RtActivatable<IThreadPoolStatics> for ThreadPool {}
impl ThreadPool {
    #[inline] pub fn run_async(handler: &WorkItemHandler) -> Result<ComPtr<super::super::foundation::IAsyncAction>> { unsafe {
        <Self as RtActivatable<IThreadPoolStatics>>::get_activation_factory().run_async(handler)
    }}
    #[inline] pub fn run_with_priority_async(handler: &WorkItemHandler, priority: WorkItemPriority) -> Result<ComPtr<super::super::foundation::IAsyncAction>> { unsafe {
        <Self as RtActivatable<IThreadPoolStatics>>::get_activation_factory().run_with_priority_async(handler, priority)
    }}
    #[inline] pub fn run_with_priority_and_options_async(handler: &WorkItemHandler, priority: WorkItemPriority, options: WorkItemOptions) -> Result<ComPtr<super::super::foundation::IAsyncAction>> { unsafe {
        <Self as RtActivatable<IThreadPoolStatics>>::get_activation_factory().run_with_priority_and_options_async(handler, priority, options)
    }}
}
DEFINE_CLSID!(ThreadPool(&[87,105,110,100,111,119,115,46,83,121,115,116,101,109,46,84,104,114,101,97,100,105,110,103,46,84,104,114,101,97,100,80,111,111,108,0]) [CLSID_ThreadPool]);
DEFINE_IID!(IID_IThreadPoolStatics, 3065997277, 33981, 17656, 172, 28, 147, 235, 203, 157, 186, 145);
RT_INTERFACE!{static interface IThreadPoolStatics(IThreadPoolStaticsVtbl): IInspectable(IInspectableVtbl) [IID_IThreadPoolStatics] {
    fn RunAsync(&self, handler: *mut WorkItemHandler, out: *mut *mut super::super::foundation::IAsyncAction) -> HRESULT,
    fn RunWithPriorityAsync(&self, handler: *mut WorkItemHandler, priority: WorkItemPriority, out: *mut *mut super::super::foundation::IAsyncAction) -> HRESULT,
    fn RunWithPriorityAndOptionsAsync(&self, handler: *mut WorkItemHandler, priority: WorkItemPriority, options: WorkItemOptions, out: *mut *mut super::super::foundation::IAsyncAction) -> HRESULT
}}
impl IThreadPoolStatics {
    #[inline] pub unsafe fn run_async(&self, handler: &WorkItemHandler) -> Result<ComPtr<super::super::foundation::IAsyncAction>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).RunAsync)(self as *const _ as *mut _, handler as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn run_with_priority_async(&self, handler: &WorkItemHandler, priority: WorkItemPriority) -> Result<ComPtr<super::super::foundation::IAsyncAction>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).RunWithPriorityAsync)(self as *const _ as *mut _, handler as *const _ as *mut _, priority, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn run_with_priority_and_options_async(&self, handler: &WorkItemHandler, priority: WorkItemPriority, options: WorkItemOptions) -> Result<ComPtr<super::super::foundation::IAsyncAction>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).RunWithPriorityAndOptionsAsync)(self as *const _ as *mut _, handler as *const _ as *mut _, priority, options, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
DEFINE_IID!(IID_IThreadPoolTimer, 1498332792, 21994, 19080, 165, 13, 52, 2, 174, 31, 156, 242);
RT_INTERFACE!{interface IThreadPoolTimer(IThreadPoolTimerVtbl): IInspectable(IInspectableVtbl) [IID_IThreadPoolTimer] {
    fn get_Period(&self, out: *mut super::super::foundation::TimeSpan) -> HRESULT,
    fn get_Delay(&self, out: *mut super::super::foundation::TimeSpan) -> HRESULT,
    fn Cancel(&self) -> HRESULT
}}
impl IThreadPoolTimer {
    #[inline] pub unsafe fn get_period(&self) -> Result<super::super::foundation::TimeSpan> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_Period)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_delay(&self) -> Result<super::super::foundation::TimeSpan> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_Delay)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn cancel(&self) -> Result<()> {
        let hr = ((*self.lpVtbl).Cancel)(self as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
}
RT_CLASS!{class ThreadPoolTimer: IThreadPoolTimer}
impl RtActivatable<IThreadPoolTimerStatics> for ThreadPoolTimer {}
impl ThreadPoolTimer {
    #[inline] pub fn create_periodic_timer(handler: &TimerElapsedHandler, period: super::super::foundation::TimeSpan) -> Result<ComPtr<ThreadPoolTimer>> { unsafe {
        <Self as RtActivatable<IThreadPoolTimerStatics>>::get_activation_factory().create_periodic_timer(handler, period)
    }}
    #[inline] pub fn create_timer(handler: &TimerElapsedHandler, delay: super::super::foundation::TimeSpan) -> Result<ComPtr<ThreadPoolTimer>> { unsafe {
        <Self as RtActivatable<IThreadPoolTimerStatics>>::get_activation_factory().create_timer(handler, delay)
    }}
    #[inline] pub fn create_periodic_timer_with_completion(handler: &TimerElapsedHandler, period: super::super::foundation::TimeSpan, destroyed: &TimerDestroyedHandler) -> Result<ComPtr<ThreadPoolTimer>> { unsafe {
        <Self as RtActivatable<IThreadPoolTimerStatics>>::get_activation_factory().create_periodic_timer_with_completion(handler, period, destroyed)
    }}
    #[inline] pub fn create_timer_with_completion(handler: &TimerElapsedHandler, delay: super::super::foundation::TimeSpan, destroyed: &TimerDestroyedHandler) -> Result<ComPtr<ThreadPoolTimer>> { unsafe {
        <Self as RtActivatable<IThreadPoolTimerStatics>>::get_activation_factory().create_timer_with_completion(handler, delay, destroyed)
    }}
}
DEFINE_CLSID!(ThreadPoolTimer(&[87,105,110,100,111,119,115,46,83,121,115,116,101,109,46,84,104,114,101,97,100,105,110,103,46,84,104,114,101,97,100,80,111,111,108,84,105,109,101,114,0]) [CLSID_ThreadPoolTimer]);
DEFINE_IID!(IID_IThreadPoolTimerStatics, 445291778, 58498, 17947, 184, 199, 142, 250, 209, 204, 229, 144);
RT_INTERFACE!{static interface IThreadPoolTimerStatics(IThreadPoolTimerStaticsVtbl): IInspectable(IInspectableVtbl) [IID_IThreadPoolTimerStatics] {
    fn CreatePeriodicTimer(&self, handler: *mut TimerElapsedHandler, period: super::super::foundation::TimeSpan, out: *mut *mut ThreadPoolTimer) -> HRESULT,
    fn CreateTimer(&self, handler: *mut TimerElapsedHandler, delay: super::super::foundation::TimeSpan, out: *mut *mut ThreadPoolTimer) -> HRESULT,
    fn CreatePeriodicTimerWithCompletion(&self, handler: *mut TimerElapsedHandler, period: super::super::foundation::TimeSpan, destroyed: *mut TimerDestroyedHandler, out: *mut *mut ThreadPoolTimer) -> HRESULT,
    fn CreateTimerWithCompletion(&self, handler: *mut TimerElapsedHandler, delay: super::super::foundation::TimeSpan, destroyed: *mut TimerDestroyedHandler, out: *mut *mut ThreadPoolTimer) -> HRESULT
}}
impl IThreadPoolTimerStatics {
    #[inline] pub unsafe fn create_periodic_timer(&self, handler: &TimerElapsedHandler, period: super::super::foundation::TimeSpan) -> Result<ComPtr<ThreadPoolTimer>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).CreatePeriodicTimer)(self as *const _ as *mut _, handler as *const _ as *mut _, period, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn create_timer(&self, handler: &TimerElapsedHandler, delay: super::super::foundation::TimeSpan) -> Result<ComPtr<ThreadPoolTimer>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).CreateTimer)(self as *const _ as *mut _, handler as *const _ as *mut _, delay, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn create_periodic_timer_with_completion(&self, handler: &TimerElapsedHandler, period: super::super::foundation::TimeSpan, destroyed: &TimerDestroyedHandler) -> Result<ComPtr<ThreadPoolTimer>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).CreatePeriodicTimerWithCompletion)(self as *const _ as *mut _, handler as *const _ as *mut _, period, destroyed as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn create_timer_with_completion(&self, handler: &TimerElapsedHandler, delay: super::super::foundation::TimeSpan, destroyed: &TimerDestroyedHandler) -> Result<ComPtr<ThreadPoolTimer>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).CreateTimerWithCompletion)(self as *const _ as *mut _, handler as *const _ as *mut _, delay, destroyed as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
DEFINE_IID!(IID_TimerDestroyedHandler, 887953914, 33668, 20153, 130, 9, 251, 80, 148, 238, 236, 53);
RT_DELEGATE!{delegate TimerDestroyedHandler(TimerDestroyedHandlerVtbl, TimerDestroyedHandlerImpl) [IID_TimerDestroyedHandler] {
    fn Invoke(&self, timer: *mut ThreadPoolTimer) -> HRESULT
}}
impl TimerDestroyedHandler {
    #[inline] pub unsafe fn invoke(&self, timer: &ThreadPoolTimer) -> Result<()> {
        let hr = ((*self.lpVtbl).Invoke)(self as *const _ as *mut _, timer as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
}
DEFINE_IID!(IID_TimerElapsedHandler, 4205749863, 64491, 18891, 173, 178, 113, 24, 76, 85, 110, 67);
RT_DELEGATE!{delegate TimerElapsedHandler(TimerElapsedHandlerVtbl, TimerElapsedHandlerImpl) [IID_TimerElapsedHandler] {
    fn Invoke(&self, timer: *mut ThreadPoolTimer) -> HRESULT
}}
impl TimerElapsedHandler {
    #[inline] pub unsafe fn invoke(&self, timer: &ThreadPoolTimer) -> Result<()> {
        let hr = ((*self.lpVtbl).Invoke)(self as *const _ as *mut _, timer as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
}
DEFINE_IID!(IID_WorkItemHandler, 488278923, 64102, 16719, 156, 189, 182, 95, 201, 157, 23, 250);
RT_DELEGATE!{delegate WorkItemHandler(WorkItemHandlerVtbl, WorkItemHandlerImpl) [IID_WorkItemHandler] {
    fn Invoke(&self, operation: *mut super::super::foundation::IAsyncAction) -> HRESULT
}}
impl WorkItemHandler {
    #[inline] pub unsafe fn invoke(&self, operation: &super::super::foundation::IAsyncAction) -> Result<()> {
        let hr = ((*self.lpVtbl).Invoke)(self as *const _ as *mut _, operation as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
}
RT_ENUM! { enum WorkItemOptions: u32 {
    None (WorkItemOptions_None) = 0, TimeSliced (WorkItemOptions_TimeSliced) = 1,
}}
RT_ENUM! { enum WorkItemPriority: i32 {
    Low (WorkItemPriority_Low) = -1, Normal (WorkItemPriority_Normal) = 0, High (WorkItemPriority_High) = 1,
}}
pub mod core { // Windows.System.Threading.Core
use ::prelude::*;
DEFINE_IID!(IID_IPreallocatedWorkItem, 3067783676, 48219, 16410, 168, 178, 110, 117, 77, 20, 218, 166);
RT_INTERFACE!{interface IPreallocatedWorkItem(IPreallocatedWorkItemVtbl): IInspectable(IInspectableVtbl) [IID_IPreallocatedWorkItem] {
    fn RunAsync(&self, out: *mut *mut ::rt::gen::windows::foundation::IAsyncAction) -> HRESULT
}}
impl IPreallocatedWorkItem {
    #[inline] pub unsafe fn run_async(&self) -> Result<ComPtr<::rt::gen::windows::foundation::IAsyncAction>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).RunAsync)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
RT_CLASS!{class PreallocatedWorkItem: IPreallocatedWorkItem}
impl RtActivatable<IPreallocatedWorkItemFactory> for PreallocatedWorkItem {}
impl PreallocatedWorkItem {
    #[inline] pub fn create_work_item(handler: &super::WorkItemHandler) -> Result<ComPtr<PreallocatedWorkItem>> { unsafe {
        <Self as RtActivatable<IPreallocatedWorkItemFactory>>::get_activation_factory().create_work_item(handler)
    }}
    #[inline] pub fn create_work_item_with_priority(handler: &super::WorkItemHandler, priority: super::WorkItemPriority) -> Result<ComPtr<PreallocatedWorkItem>> { unsafe {
        <Self as RtActivatable<IPreallocatedWorkItemFactory>>::get_activation_factory().create_work_item_with_priority(handler, priority)
    }}
    #[inline] pub fn create_work_item_with_priority_and_options(handler: &super::WorkItemHandler, priority: super::WorkItemPriority, options: super::WorkItemOptions) -> Result<ComPtr<PreallocatedWorkItem>> { unsafe {
        <Self as RtActivatable<IPreallocatedWorkItemFactory>>::get_activation_factory().create_work_item_with_priority_and_options(handler, priority, options)
    }}
}
DEFINE_CLSID!(PreallocatedWorkItem(&[87,105,110,100,111,119,115,46,83,121,115,116,101,109,46,84,104,114,101,97,100,105,110,103,46,67,111,114,101,46,80,114,101,97,108,108,111,99,97,116,101,100,87,111,114,107,73,116,101,109,0]) [CLSID_PreallocatedWorkItem]);
DEFINE_IID!(IID_IPreallocatedWorkItemFactory, 3822267205, 57322, 18075, 130, 197, 246, 227, 206, 253, 234, 251);
RT_INTERFACE!{static interface IPreallocatedWorkItemFactory(IPreallocatedWorkItemFactoryVtbl): IInspectable(IInspectableVtbl) [IID_IPreallocatedWorkItemFactory] {
    fn CreateWorkItem(&self, handler: *mut super::WorkItemHandler, out: *mut *mut PreallocatedWorkItem) -> HRESULT,
    fn CreateWorkItemWithPriority(&self, handler: *mut super::WorkItemHandler, priority: super::WorkItemPriority, out: *mut *mut PreallocatedWorkItem) -> HRESULT,
    fn CreateWorkItemWithPriorityAndOptions(&self, handler: *mut super::WorkItemHandler, priority: super::WorkItemPriority, options: super::WorkItemOptions, out: *mut *mut PreallocatedWorkItem) -> HRESULT
}}
impl IPreallocatedWorkItemFactory {
    #[inline] pub unsafe fn create_work_item(&self, handler: &super::WorkItemHandler) -> Result<ComPtr<PreallocatedWorkItem>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).CreateWorkItem)(self as *const _ as *mut _, handler as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn create_work_item_with_priority(&self, handler: &super::WorkItemHandler, priority: super::WorkItemPriority) -> Result<ComPtr<PreallocatedWorkItem>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).CreateWorkItemWithPriority)(self as *const _ as *mut _, handler as *const _ as *mut _, priority, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn create_work_item_with_priority_and_options(&self, handler: &super::WorkItemHandler, priority: super::WorkItemPriority, options: super::WorkItemOptions) -> Result<ComPtr<PreallocatedWorkItem>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).CreateWorkItemWithPriorityAndOptions)(self as *const _ as *mut _, handler as *const _ as *mut _, priority, options, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
DEFINE_IID!(IID_SignalHandler, 2453422126, 18209, 17422, 157, 218, 85, 182, 242, 224, 119, 16);
RT_DELEGATE!{delegate SignalHandler(SignalHandlerVtbl, SignalHandlerImpl) [IID_SignalHandler] {
    fn Invoke(&self, signalNotifier: *mut SignalNotifier, timedOut: bool) -> HRESULT
}}
impl SignalHandler {
    #[inline] pub unsafe fn invoke(&self, signalNotifier: &SignalNotifier, timedOut: bool) -> Result<()> {
        let hr = ((*self.lpVtbl).Invoke)(self as *const _ as *mut _, signalNotifier as *const _ as *mut _, timedOut);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
}
DEFINE_IID!(IID_ISignalNotifier, 338189830, 25511, 18195, 182, 217, 98, 246, 75, 86, 251, 139);
RT_INTERFACE!{interface ISignalNotifier(ISignalNotifierVtbl): IInspectable(IInspectableVtbl) [IID_ISignalNotifier] {
    fn Enable(&self) -> HRESULT,
    fn Terminate(&self) -> HRESULT
}}
impl ISignalNotifier {
    #[inline] pub unsafe fn enable(&self) -> Result<()> {
        let hr = ((*self.lpVtbl).Enable)(self as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn terminate(&self) -> Result<()> {
        let hr = ((*self.lpVtbl).Terminate)(self as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
}
RT_CLASS!{class SignalNotifier: ISignalNotifier}
impl RtActivatable<ISignalNotifierStatics> for SignalNotifier {}
impl SignalNotifier {
    #[inline] pub fn attach_to_event(name: &HStringArg, handler: &SignalHandler) -> Result<ComPtr<SignalNotifier>> { unsafe {
        <Self as RtActivatable<ISignalNotifierStatics>>::get_activation_factory().attach_to_event(name, handler)
    }}
    #[inline] pub fn attach_to_event_with_timeout(name: &HStringArg, handler: &SignalHandler, timeout: ::rt::gen::windows::foundation::TimeSpan) -> Result<ComPtr<SignalNotifier>> { unsafe {
        <Self as RtActivatable<ISignalNotifierStatics>>::get_activation_factory().attach_to_event_with_timeout(name, handler, timeout)
    }}
    #[inline] pub fn attach_to_semaphore(name: &HStringArg, handler: &SignalHandler) -> Result<ComPtr<SignalNotifier>> { unsafe {
        <Self as RtActivatable<ISignalNotifierStatics>>::get_activation_factory().attach_to_semaphore(name, handler)
    }}
    #[inline] pub fn attach_to_semaphore_with_timeout(name: &HStringArg, handler: &SignalHandler, timeout: ::rt::gen::windows::foundation::TimeSpan) -> Result<ComPtr<SignalNotifier>> { unsafe {
        <Self as RtActivatable<ISignalNotifierStatics>>::get_activation_factory().attach_to_semaphore_with_timeout(name, handler, timeout)
    }}
}
DEFINE_CLSID!(SignalNotifier(&[87,105,110,100,111,119,115,46,83,121,115,116,101,109,46,84,104,114,101,97,100,105,110,103,46,67,111,114,101,46,83,105,103,110,97,108,78,111,116,105,102,105,101,114,0]) [CLSID_SignalNotifier]);
DEFINE_IID!(IID_ISignalNotifierStatics, 474891622, 33792, 18131, 161, 21, 125, 12, 13, 252, 159, 98);
RT_INTERFACE!{static interface ISignalNotifierStatics(ISignalNotifierStaticsVtbl): IInspectable(IInspectableVtbl) [IID_ISignalNotifierStatics] {
    fn AttachToEvent(&self, name: HSTRING, handler: *mut SignalHandler, out: *mut *mut SignalNotifier) -> HRESULT,
    fn AttachToEventWithTimeout(&self, name: HSTRING, handler: *mut SignalHandler, timeout: ::rt::gen::windows::foundation::TimeSpan, out: *mut *mut SignalNotifier) -> HRESULT,
    fn AttachToSemaphore(&self, name: HSTRING, handler: *mut SignalHandler, out: *mut *mut SignalNotifier) -> HRESULT,
    fn AttachToSemaphoreWithTimeout(&self, name: HSTRING, handler: *mut SignalHandler, timeout: ::rt::gen::windows::foundation::TimeSpan, out: *mut *mut SignalNotifier) -> HRESULT
}}
impl ISignalNotifierStatics {
    #[inline] pub unsafe fn attach_to_event(&self, name: &HStringArg, handler: &SignalHandler) -> Result<ComPtr<SignalNotifier>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).AttachToEvent)(self as *const _ as *mut _, name.get(), handler as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn attach_to_event_with_timeout(&self, name: &HStringArg, handler: &SignalHandler, timeout: ::rt::gen::windows::foundation::TimeSpan) -> Result<ComPtr<SignalNotifier>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).AttachToEventWithTimeout)(self as *const _ as *mut _, name.get(), handler as *const _ as *mut _, timeout, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn attach_to_semaphore(&self, name: &HStringArg, handler: &SignalHandler) -> Result<ComPtr<SignalNotifier>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).AttachToSemaphore)(self as *const _ as *mut _, name.get(), handler as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn attach_to_semaphore_with_timeout(&self, name: &HStringArg, handler: &SignalHandler, timeout: ::rt::gen::windows::foundation::TimeSpan) -> Result<ComPtr<SignalNotifier>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).AttachToSemaphoreWithTimeout)(self as *const _ as *mut _, name.get(), handler as *const _ as *mut _, timeout, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
} // Windows.System.Threading.Core
} // Windows.System.Threading
pub mod display { // Windows.System.Display
use ::prelude::*;
DEFINE_IID!(IID_IDisplayRequest, 3849527364, 62623, 19296, 141, 212, 94, 126, 58, 99, 42, 192);
RT_INTERFACE!{interface IDisplayRequest(IDisplayRequestVtbl): IInspectable(IInspectableVtbl) [IID_IDisplayRequest] {
    fn RequestActive(&self) -> HRESULT,
    fn RequestRelease(&self) -> HRESULT
}}
impl IDisplayRequest {
    #[inline] pub unsafe fn request_active(&self) -> Result<()> {
        let hr = ((*self.lpVtbl).RequestActive)(self as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn request_release(&self) -> Result<()> {
        let hr = ((*self.lpVtbl).RequestRelease)(self as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
}
RT_CLASS!{class DisplayRequest: IDisplayRequest}
impl RtActivatable<IActivationFactory> for DisplayRequest {}
DEFINE_CLSID!(DisplayRequest(&[87,105,110,100,111,119,115,46,83,121,115,116,101,109,46,68,105,115,112,108,97,121,46,68,105,115,112,108,97,121,82,101,113,117,101,115,116,0]) [CLSID_DisplayRequest]);
} // Windows.System.Display
