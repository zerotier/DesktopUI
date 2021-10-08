pub mod cortana { // Windows.Services.Cortana
use ::prelude::*;
RT_ENUM! { enum CortanaPermission: i32 {
    BrowsingHistory (CortanaPermission_BrowsingHistory) = 0, Calendar (CortanaPermission_Calendar) = 1, CallHistory (CortanaPermission_CallHistory) = 2, Contacts (CortanaPermission_Contacts) = 3, Email (CortanaPermission_Email) = 4, InputPersonalization (CortanaPermission_InputPersonalization) = 5, Location (CortanaPermission_Location) = 6, Messaging (CortanaPermission_Messaging) = 7, Microphone (CortanaPermission_Microphone) = 8, Personalization (CortanaPermission_Personalization) = 9, PhoneCall (CortanaPermission_PhoneCall) = 10,
}}
RT_ENUM! { enum CortanaPermissionsChangeResult: i32 {
    Success (CortanaPermissionsChangeResult_Success) = 0, Unavailable (CortanaPermissionsChangeResult_Unavailable) = 1, DisabledByPolicy (CortanaPermissionsChangeResult_DisabledByPolicy) = 2,
}}
DEFINE_IID!(IID_ICortanaPermissionsManager, 420688096, 34453, 17290, 149, 69, 61, 164, 232, 34, 221, 180);
RT_INTERFACE!{interface ICortanaPermissionsManager(ICortanaPermissionsManagerVtbl): IInspectable(IInspectableVtbl) [IID_ICortanaPermissionsManager] {
    fn IsSupported(&self, out: *mut bool) -> HRESULT,
    fn ArePermissionsGrantedAsync(&self, permissions: *mut super::super::foundation::collections::IIterable<CortanaPermission>, out: *mut *mut super::super::foundation::IAsyncOperation<bool>) -> HRESULT,
    fn GrantPermissionsAsync(&self, permissions: *mut super::super::foundation::collections::IIterable<CortanaPermission>, out: *mut *mut super::super::foundation::IAsyncOperation<CortanaPermissionsChangeResult>) -> HRESULT,
    fn RevokePermissionsAsync(&self, permissions: *mut super::super::foundation::collections::IIterable<CortanaPermission>, out: *mut *mut super::super::foundation::IAsyncOperation<CortanaPermissionsChangeResult>) -> HRESULT
}}
impl ICortanaPermissionsManager {
    #[inline] pub unsafe fn is_supported(&self) -> Result<bool> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).IsSupported)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn are_permissions_granted_async(&self, permissions: &super::super::foundation::collections::IIterable<CortanaPermission>) -> Result<ComPtr<super::super::foundation::IAsyncOperation<bool>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).ArePermissionsGrantedAsync)(self as *const _ as *mut _, permissions as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn grant_permissions_async(&self, permissions: &super::super::foundation::collections::IIterable<CortanaPermission>) -> Result<ComPtr<super::super::foundation::IAsyncOperation<CortanaPermissionsChangeResult>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GrantPermissionsAsync)(self as *const _ as *mut _, permissions as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn revoke_permissions_async(&self, permissions: &super::super::foundation::collections::IIterable<CortanaPermission>) -> Result<ComPtr<super::super::foundation::IAsyncOperation<CortanaPermissionsChangeResult>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).RevokePermissionsAsync)(self as *const _ as *mut _, permissions as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
RT_CLASS!{class CortanaPermissionsManager: ICortanaPermissionsManager}
impl RtActivatable<ICortanaPermissionsManagerStatics> for CortanaPermissionsManager {}
impl CortanaPermissionsManager {
    #[inline] pub fn get_default() -> Result<ComPtr<CortanaPermissionsManager>> { unsafe {
        <Self as RtActivatable<ICortanaPermissionsManagerStatics>>::get_activation_factory().get_default()
    }}
}
DEFINE_CLSID!(CortanaPermissionsManager(&[87,105,110,100,111,119,115,46,83,101,114,118,105,99,101,115,46,67,111,114,116,97,110,97,46,67,111,114,116,97,110,97,80,101,114,109,105,115,115,105,111,110,115,77,97,110,97,103,101,114,0]) [CLSID_CortanaPermissionsManager]);
DEFINE_IID!(IID_ICortanaPermissionsManagerStatics, 1991370362, 45125, 17428, 157, 109, 42, 211, 165, 254, 58, 126);
RT_INTERFACE!{static interface ICortanaPermissionsManagerStatics(ICortanaPermissionsManagerStaticsVtbl): IInspectable(IInspectableVtbl) [IID_ICortanaPermissionsManagerStatics] {
    fn GetDefault(&self, out: *mut *mut CortanaPermissionsManager) -> HRESULT
}}
impl ICortanaPermissionsManagerStatics {
    #[inline] pub unsafe fn get_default(&self) -> Result<ComPtr<CortanaPermissionsManager>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetDefault)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
DEFINE_IID!(IID_ICortanaSettings, 1423274407, 32866, 16628, 171, 231, 222, 223, 214, 151, 176, 25);
RT_INTERFACE!{interface ICortanaSettings(ICortanaSettingsVtbl): IInspectable(IInspectableVtbl) [IID_ICortanaSettings] {
    fn get_HasUserConsentToVoiceActivation(&self, out: *mut bool) -> HRESULT,
    fn get_IsVoiceActivationEnabled(&self, out: *mut bool) -> HRESULT,
    fn put_IsVoiceActivationEnabled(&self, value: bool) -> HRESULT
}}
impl ICortanaSettings {
    #[inline] pub unsafe fn get_has_user_consent_to_voice_activation(&self) -> Result<bool> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_HasUserConsentToVoiceActivation)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_is_voice_activation_enabled(&self) -> Result<bool> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_IsVoiceActivationEnabled)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_is_voice_activation_enabled(&self, value: bool) -> Result<()> {
        let hr = ((*self.lpVtbl).put_IsVoiceActivationEnabled)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
}
RT_CLASS!{class CortanaSettings: ICortanaSettings}
impl RtActivatable<ICortanaSettingsStatics> for CortanaSettings {}
impl CortanaSettings {
    #[inline] pub fn is_supported() -> Result<bool> { unsafe {
        <Self as RtActivatable<ICortanaSettingsStatics>>::get_activation_factory().is_supported()
    }}
    #[inline] pub fn get_default() -> Result<ComPtr<CortanaSettings>> { unsafe {
        <Self as RtActivatable<ICortanaSettingsStatics>>::get_activation_factory().get_default()
    }}
}
DEFINE_CLSID!(CortanaSettings(&[87,105,110,100,111,119,115,46,83,101,114,118,105,99,101,115,46,67,111,114,116,97,110,97,46,67,111,114,116,97,110,97,83,101,116,116,105,110,103,115,0]) [CLSID_CortanaSettings]);
DEFINE_IID!(IID_ICortanaSettingsStatics, 2334969214, 11968, 17517, 146, 133, 51, 240, 124, 232, 172, 4);
RT_INTERFACE!{static interface ICortanaSettingsStatics(ICortanaSettingsStaticsVtbl): IInspectable(IInspectableVtbl) [IID_ICortanaSettingsStatics] {
    fn IsSupported(&self, out: *mut bool) -> HRESULT,
    fn GetDefault(&self, out: *mut *mut CortanaSettings) -> HRESULT
}}
impl ICortanaSettingsStatics {
    #[inline] pub unsafe fn is_supported(&self) -> Result<bool> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).IsSupported)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_default(&self) -> Result<ComPtr<CortanaSettings>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetDefault)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
} // Windows.Services.Cortana
pub mod maps { // Windows.Services.Maps
use ::prelude::*;
DEFINE_IID!(IID_IEnhancedWaypoint, 3978726516, 22803, 4582, 139, 119, 134, 243, 12, 168, 147, 211);
RT_INTERFACE!{interface IEnhancedWaypoint(IEnhancedWaypointVtbl): IInspectable(IInspectableVtbl) [IID_IEnhancedWaypoint] {
    #[cfg(not(feature="windows-devices"))] fn __Dummy0(&self) -> (),
    #[cfg(feature="windows-devices")] fn get_Point(&self, out: *mut *mut super::super::devices::geolocation::Geopoint) -> HRESULT,
    fn get_Kind(&self, out: *mut WaypointKind) -> HRESULT
}}
impl IEnhancedWaypoint {
    #[cfg(feature="windows-devices")] #[inline] pub unsafe fn get_point(&self) -> Result<ComPtr<super::super::devices::geolocation::Geopoint>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Point)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_kind(&self) -> Result<WaypointKind> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_Kind)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
}
RT_CLASS!{class EnhancedWaypoint: IEnhancedWaypoint}
impl RtActivatable<IEnhancedWaypointFactory> for EnhancedWaypoint {}
impl EnhancedWaypoint {
    #[cfg(feature="windows-devices")] #[inline] pub fn create(point: &super::super::devices::geolocation::Geopoint, kind: WaypointKind) -> Result<ComPtr<EnhancedWaypoint>> { unsafe {
        <Self as RtActivatable<IEnhancedWaypointFactory>>::get_activation_factory().create(point, kind)
    }}
}
DEFINE_CLSID!(EnhancedWaypoint(&[87,105,110,100,111,119,115,46,83,101,114,118,105,99,101,115,46,77,97,112,115,46,69,110,104,97,110,99,101,100,87,97,121,112,111,105,110,116,0]) [CLSID_EnhancedWaypoint]);
DEFINE_IID!(IID_IEnhancedWaypointFactory, 2944828535, 41642, 18141, 182, 69, 35, 179, 27, 138, 166, 199);
RT_INTERFACE!{static interface IEnhancedWaypointFactory(IEnhancedWaypointFactoryVtbl): IInspectable(IInspectableVtbl) [IID_IEnhancedWaypointFactory] {
    #[cfg(feature="windows-devices")] fn Create(&self, point: *mut super::super::devices::geolocation::Geopoint, kind: WaypointKind, out: *mut *mut EnhancedWaypoint) -> HRESULT
}}
impl IEnhancedWaypointFactory {
    #[cfg(feature="windows-devices")] #[inline] pub unsafe fn create(&self, point: &super::super::devices::geolocation::Geopoint, kind: WaypointKind) -> Result<ComPtr<EnhancedWaypoint>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).Create)(self as *const _ as *mut _, point as *const _ as *mut _, kind, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
DEFINE_IID!(IID_IManeuverWarning, 3248713098, 9776, 17272, 158, 74, 110, 68, 37, 61, 206, 186);
RT_INTERFACE!{interface IManeuverWarning(IManeuverWarningVtbl): IInspectable(IInspectableVtbl) [IID_IManeuverWarning] {
    fn get_Kind(&self, out: *mut ManeuverWarningKind) -> HRESULT,
    fn get_Severity(&self, out: *mut ManeuverWarningSeverity) -> HRESULT
}}
impl IManeuverWarning {
    #[inline] pub unsafe fn get_kind(&self) -> Result<ManeuverWarningKind> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_Kind)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_severity(&self) -> Result<ManeuverWarningSeverity> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_Severity)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
}
RT_CLASS!{class ManeuverWarning: IManeuverWarning}
RT_ENUM! { enum ManeuverWarningKind: i32 {
    None (ManeuverWarningKind_None) = 0, Accident (ManeuverWarningKind_Accident) = 1, AdministrativeDivisionChange (ManeuverWarningKind_AdministrativeDivisionChange) = 2, Alert (ManeuverWarningKind_Alert) = 3, BlockedRoad (ManeuverWarningKind_BlockedRoad) = 4, CheckTimetable (ManeuverWarningKind_CheckTimetable) = 5, Congestion (ManeuverWarningKind_Congestion) = 6, Construction (ManeuverWarningKind_Construction) = 7, CountryChange (ManeuverWarningKind_CountryChange) = 8, DisabledVehicle (ManeuverWarningKind_DisabledVehicle) = 9, GateAccess (ManeuverWarningKind_GateAccess) = 10, GetOffTransit (ManeuverWarningKind_GetOffTransit) = 11, GetOnTransit (ManeuverWarningKind_GetOnTransit) = 12, IllegalUTurn (ManeuverWarningKind_IllegalUTurn) = 13, MassTransit (ManeuverWarningKind_MassTransit) = 14, Miscellaneous (ManeuverWarningKind_Miscellaneous) = 15, NoIncident (ManeuverWarningKind_NoIncident) = 16, Other (ManeuverWarningKind_Other) = 17, OtherNews (ManeuverWarningKind_OtherNews) = 18, OtherTrafficIncidents (ManeuverWarningKind_OtherTrafficIncidents) = 19, PlannedEvent (ManeuverWarningKind_PlannedEvent) = 20, PrivateRoad (ManeuverWarningKind_PrivateRoad) = 21, RestrictedTurn (ManeuverWarningKind_RestrictedTurn) = 22, RoadClosures (ManeuverWarningKind_RoadClosures) = 23, RoadHazard (ManeuverWarningKind_RoadHazard) = 24, ScheduledConstruction (ManeuverWarningKind_ScheduledConstruction) = 25, SeasonalClosures (ManeuverWarningKind_SeasonalClosures) = 26, Tollbooth (ManeuverWarningKind_Tollbooth) = 27, TollRoad (ManeuverWarningKind_TollRoad) = 28, TollZoneEnter (ManeuverWarningKind_TollZoneEnter) = 29, TollZoneExit (ManeuverWarningKind_TollZoneExit) = 30, TrafficFlow (ManeuverWarningKind_TrafficFlow) = 31, TransitLineChange (ManeuverWarningKind_TransitLineChange) = 32, UnpavedRoad (ManeuverWarningKind_UnpavedRoad) = 33, UnscheduledConstruction (ManeuverWarningKind_UnscheduledConstruction) = 34, Weather (ManeuverWarningKind_Weather) = 35,
}}
RT_ENUM! { enum ManeuverWarningSeverity: i32 {
    None (ManeuverWarningSeverity_None) = 0, LowImpact (ManeuverWarningSeverity_LowImpact) = 1, Minor (ManeuverWarningSeverity_Minor) = 2, Moderate (ManeuverWarningSeverity_Moderate) = 3, Serious (ManeuverWarningSeverity_Serious) = 4,
}}
DEFINE_IID!(IID_IMapAddress, 3483871603, 41908, 17556, 179, 255, 203, 169, 77, 182, 150, 153);
RT_INTERFACE!{interface IMapAddress(IMapAddressVtbl): IInspectable(IInspectableVtbl) [IID_IMapAddress] {
    fn get_BuildingName(&self, out: *mut HSTRING) -> HRESULT,
    fn get_BuildingFloor(&self, out: *mut HSTRING) -> HRESULT,
    fn get_BuildingRoom(&self, out: *mut HSTRING) -> HRESULT,
    fn get_BuildingWing(&self, out: *mut HSTRING) -> HRESULT,
    fn get_StreetNumber(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Street(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Neighborhood(&self, out: *mut HSTRING) -> HRESULT,
    fn get_District(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Town(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Region(&self, out: *mut HSTRING) -> HRESULT,
    fn get_RegionCode(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Country(&self, out: *mut HSTRING) -> HRESULT,
    fn get_CountryCode(&self, out: *mut HSTRING) -> HRESULT,
    fn get_PostCode(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Continent(&self, out: *mut HSTRING) -> HRESULT
}}
impl IMapAddress {
    #[inline] pub unsafe fn get_building_name(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_BuildingName)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_building_floor(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_BuildingFloor)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_building_room(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_BuildingRoom)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_building_wing(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_BuildingWing)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_street_number(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_StreetNumber)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_street(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Street)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_neighborhood(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Neighborhood)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_district(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_District)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_town(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Town)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_region(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Region)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_region_code(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_RegionCode)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_country(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Country)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_country_code(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_CountryCode)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_post_code(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_PostCode)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_continent(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Continent)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
}
RT_CLASS!{class MapAddress: IMapAddress}
DEFINE_IID!(IID_IMapAddress2, 1976397297, 58797, 17833, 191, 64, 108, 242, 86, 193, 221, 19);
RT_INTERFACE!{interface IMapAddress2(IMapAddress2Vtbl): IInspectable(IInspectableVtbl) [IID_IMapAddress2] {
    fn get_FormattedAddress(&self, out: *mut HSTRING) -> HRESULT
}}
impl IMapAddress2 {
    #[inline] pub unsafe fn get_formatted_address(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_FormattedAddress)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
}
DEFINE_IID!(IID_IMapLocation, 1007107927, 3492, 17128, 158, 226, 169, 111, 207, 35, 113, 220);
RT_INTERFACE!{interface IMapLocation(IMapLocationVtbl): IInspectable(IInspectableVtbl) [IID_IMapLocation] {
    #[cfg(not(feature="windows-devices"))] fn __Dummy0(&self) -> (),
    #[cfg(feature="windows-devices")] fn get_Point(&self, out: *mut *mut super::super::devices::geolocation::Geopoint) -> HRESULT,
    fn get_DisplayName(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Description(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Address(&self, out: *mut *mut MapAddress) -> HRESULT
}}
impl IMapLocation {
    #[cfg(feature="windows-devices")] #[inline] pub unsafe fn get_point(&self) -> Result<ComPtr<super::super::devices::geolocation::Geopoint>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Point)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_display_name(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_DisplayName)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_description(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Description)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_address(&self) -> Result<ComPtr<MapAddress>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Address)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
RT_CLASS!{class MapLocation: IMapLocation}
RT_ENUM! { enum MapLocationDesiredAccuracy: i32 {
    High (MapLocationDesiredAccuracy_High) = 0, Low (MapLocationDesiredAccuracy_Low) = 1,
}}
RT_CLASS!{static class MapLocationFinder}
impl RtActivatable<IMapLocationFinderStatics> for MapLocationFinder {}
impl RtActivatable<IMapLocationFinderStatics2> for MapLocationFinder {}
impl MapLocationFinder {
    #[cfg(feature="windows-devices")] #[inline] pub fn find_locations_at_async(queryPoint: &super::super::devices::geolocation::Geopoint) -> Result<ComPtr<super::super::foundation::IAsyncOperation<MapLocationFinderResult>>> { unsafe {
        <Self as RtActivatable<IMapLocationFinderStatics>>::get_activation_factory().find_locations_at_async(queryPoint)
    }}
    #[cfg(feature="windows-devices")] #[inline] pub fn find_locations_async(searchText: &HStringArg, referencePoint: &super::super::devices::geolocation::Geopoint) -> Result<ComPtr<super::super::foundation::IAsyncOperation<MapLocationFinderResult>>> { unsafe {
        <Self as RtActivatable<IMapLocationFinderStatics>>::get_activation_factory().find_locations_async(searchText, referencePoint)
    }}
    #[cfg(feature="windows-devices")] #[inline] pub fn find_locations_with_max_count_async(searchText: &HStringArg, referencePoint: &super::super::devices::geolocation::Geopoint, maxCount: u32) -> Result<ComPtr<super::super::foundation::IAsyncOperation<MapLocationFinderResult>>> { unsafe {
        <Self as RtActivatable<IMapLocationFinderStatics>>::get_activation_factory().find_locations_with_max_count_async(searchText, referencePoint, maxCount)
    }}
    #[cfg(feature="windows-devices")] #[inline] pub fn find_locations_at_with_accuracy_async(queryPoint: &super::super::devices::geolocation::Geopoint, accuracy: MapLocationDesiredAccuracy) -> Result<ComPtr<super::super::foundation::IAsyncOperation<MapLocationFinderResult>>> { unsafe {
        <Self as RtActivatable<IMapLocationFinderStatics2>>::get_activation_factory().find_locations_at_with_accuracy_async(queryPoint, accuracy)
    }}
}
DEFINE_CLSID!(MapLocationFinder(&[87,105,110,100,111,119,115,46,83,101,114,118,105,99,101,115,46,77,97,112,115,46,77,97,112,76,111,99,97,116,105,111,110,70,105,110,100,101,114,0]) [CLSID_MapLocationFinder]);
DEFINE_IID!(IID_IMapLocationFinderResult, 1139929465, 59596, 17910, 190, 210, 84, 204, 191, 150, 93, 154);
RT_INTERFACE!{interface IMapLocationFinderResult(IMapLocationFinderResultVtbl): IInspectable(IInspectableVtbl) [IID_IMapLocationFinderResult] {
    fn get_Locations(&self, out: *mut *mut super::super::foundation::collections::IVectorView<MapLocation>) -> HRESULT,
    fn get_Status(&self, out: *mut MapLocationFinderStatus) -> HRESULT
}}
impl IMapLocationFinderResult {
    #[inline] pub unsafe fn get_locations(&self) -> Result<ComPtr<super::super::foundation::collections::IVectorView<MapLocation>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Locations)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_status(&self) -> Result<MapLocationFinderStatus> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_Status)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
}
RT_CLASS!{class MapLocationFinderResult: IMapLocationFinderResult}
DEFINE_IID!(IID_IMapLocationFinderStatics, 831183709, 7261, 20277, 162, 223, 170, 202, 148, 149, 149, 23);
RT_INTERFACE!{static interface IMapLocationFinderStatics(IMapLocationFinderStaticsVtbl): IInspectable(IInspectableVtbl) [IID_IMapLocationFinderStatics] {
    #[cfg(feature="windows-devices")] fn FindLocationsAtAsync(&self, queryPoint: *mut super::super::devices::geolocation::Geopoint, out: *mut *mut super::super::foundation::IAsyncOperation<MapLocationFinderResult>) -> HRESULT,
    #[cfg(feature="windows-devices")] fn FindLocationsAsync(&self, searchText: HSTRING, referencePoint: *mut super::super::devices::geolocation::Geopoint, out: *mut *mut super::super::foundation::IAsyncOperation<MapLocationFinderResult>) -> HRESULT,
    #[cfg(feature="windows-devices")] fn FindLocationsWithMaxCountAsync(&self, searchText: HSTRING, referencePoint: *mut super::super::devices::geolocation::Geopoint, maxCount: u32, out: *mut *mut super::super::foundation::IAsyncOperation<MapLocationFinderResult>) -> HRESULT
}}
impl IMapLocationFinderStatics {
    #[cfg(feature="windows-devices")] #[inline] pub unsafe fn find_locations_at_async(&self, queryPoint: &super::super::devices::geolocation::Geopoint) -> Result<ComPtr<super::super::foundation::IAsyncOperation<MapLocationFinderResult>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).FindLocationsAtAsync)(self as *const _ as *mut _, queryPoint as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[cfg(feature="windows-devices")] #[inline] pub unsafe fn find_locations_async(&self, searchText: &HStringArg, referencePoint: &super::super::devices::geolocation::Geopoint) -> Result<ComPtr<super::super::foundation::IAsyncOperation<MapLocationFinderResult>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).FindLocationsAsync)(self as *const _ as *mut _, searchText.get(), referencePoint as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[cfg(feature="windows-devices")] #[inline] pub unsafe fn find_locations_with_max_count_async(&self, searchText: &HStringArg, referencePoint: &super::super::devices::geolocation::Geopoint, maxCount: u32) -> Result<ComPtr<super::super::foundation::IAsyncOperation<MapLocationFinderResult>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).FindLocationsWithMaxCountAsync)(self as *const _ as *mut _, searchText.get(), referencePoint as *const _ as *mut _, maxCount, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
DEFINE_IID!(IID_IMapLocationFinderStatics2, 2509933462, 25733, 19965, 133, 26, 51, 172, 49, 126, 58, 246);
RT_INTERFACE!{static interface IMapLocationFinderStatics2(IMapLocationFinderStatics2Vtbl): IInspectable(IInspectableVtbl) [IID_IMapLocationFinderStatics2] {
    #[cfg(feature="windows-devices")] fn FindLocationsAtWithAccuracyAsync(&self, queryPoint: *mut super::super::devices::geolocation::Geopoint, accuracy: MapLocationDesiredAccuracy, out: *mut *mut super::super::foundation::IAsyncOperation<MapLocationFinderResult>) -> HRESULT
}}
impl IMapLocationFinderStatics2 {
    #[cfg(feature="windows-devices")] #[inline] pub unsafe fn find_locations_at_with_accuracy_async(&self, queryPoint: &super::super::devices::geolocation::Geopoint, accuracy: MapLocationDesiredAccuracy) -> Result<ComPtr<super::super::foundation::IAsyncOperation<MapLocationFinderResult>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).FindLocationsAtWithAccuracyAsync)(self as *const _ as *mut _, queryPoint as *const _ as *mut _, accuracy, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
RT_ENUM! { enum MapLocationFinderStatus: i32 {
    Success (MapLocationFinderStatus_Success) = 0, UnknownError (MapLocationFinderStatus_UnknownError) = 1, InvalidCredentials (MapLocationFinderStatus_InvalidCredentials) = 2, BadLocation (MapLocationFinderStatus_BadLocation) = 3, IndexFailure (MapLocationFinderStatus_IndexFailure) = 4, NetworkFailure (MapLocationFinderStatus_NetworkFailure) = 5, NotSupported (MapLocationFinderStatus_NotSupported) = 6,
}}
RT_CLASS!{static class MapManager}
impl RtActivatable<IMapManagerStatics> for MapManager {}
impl MapManager {
    #[inline] pub fn show_downloaded_maps_ui() -> Result<()> { unsafe {
        <Self as RtActivatable<IMapManagerStatics>>::get_activation_factory().show_downloaded_maps_ui()
    }}
    #[inline] pub fn show_maps_update_ui() -> Result<()> { unsafe {
        <Self as RtActivatable<IMapManagerStatics>>::get_activation_factory().show_maps_update_ui()
    }}
}
DEFINE_CLSID!(MapManager(&[87,105,110,100,111,119,115,46,83,101,114,118,105,99,101,115,46,77,97,112,115,46,77,97,112,77,97,110,97,103,101,114,0]) [CLSID_MapManager]);
DEFINE_IID!(IID_IMapManagerStatics, 937682197, 33460, 19796, 143, 217, 175, 38, 36, 179, 1, 28);
RT_INTERFACE!{static interface IMapManagerStatics(IMapManagerStaticsVtbl): IInspectable(IInspectableVtbl) [IID_IMapManagerStatics] {
    fn ShowDownloadedMapsUI(&self) -> HRESULT,
    fn ShowMapsUpdateUI(&self) -> HRESULT
}}
impl IMapManagerStatics {
    #[inline] pub unsafe fn show_downloaded_maps_ui(&self) -> Result<()> {
        let hr = ((*self.lpVtbl).ShowDownloadedMapsUI)(self as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn show_maps_update_ui(&self) -> Result<()> {
        let hr = ((*self.lpVtbl).ShowMapsUpdateUI)(self as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
}
RT_ENUM! { enum MapManeuverNotices: u32 {
    None (MapManeuverNotices_None) = 0, Toll (MapManeuverNotices_Toll) = 1, Unpaved (MapManeuverNotices_Unpaved) = 2,
}}
DEFINE_IID!(IID_IMapRoute, 4211586866, 22605, 17795, 156, 96, 100, 31, 234, 39, 67, 73);
RT_INTERFACE!{interface IMapRoute(IMapRouteVtbl): IInspectable(IInspectableVtbl) [IID_IMapRoute] {
    #[cfg(not(feature="windows-devices"))] fn __Dummy0(&self) -> (),
    #[cfg(feature="windows-devices")] fn get_BoundingBox(&self, out: *mut *mut super::super::devices::geolocation::GeoboundingBox) -> HRESULT,
    fn get_LengthInMeters(&self, out: *mut f64) -> HRESULT,
    fn get_EstimatedDuration(&self, out: *mut super::super::foundation::TimeSpan) -> HRESULT,
    #[cfg(not(feature="windows-devices"))] fn __Dummy3(&self) -> (),
    #[cfg(feature="windows-devices")] fn get_Path(&self, out: *mut *mut super::super::devices::geolocation::Geopath) -> HRESULT,
    fn get_Legs(&self, out: *mut *mut super::super::foundation::collections::IVectorView<MapRouteLeg>) -> HRESULT,
    fn get_IsTrafficBased(&self, out: *mut bool) -> HRESULT
}}
impl IMapRoute {
    #[cfg(feature="windows-devices")] #[inline] pub unsafe fn get_bounding_box(&self) -> Result<ComPtr<super::super::devices::geolocation::GeoboundingBox>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_BoundingBox)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_length_in_meters(&self) -> Result<f64> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_LengthInMeters)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_estimated_duration(&self) -> Result<super::super::foundation::TimeSpan> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_EstimatedDuration)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[cfg(feature="windows-devices")] #[inline] pub unsafe fn get_path(&self) -> Result<ComPtr<super::super::devices::geolocation::Geopath>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Path)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_legs(&self) -> Result<ComPtr<super::super::foundation::collections::IVectorView<MapRouteLeg>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Legs)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_is_traffic_based(&self) -> Result<bool> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_IsTrafficBased)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
}
RT_CLASS!{class MapRoute: IMapRoute}
DEFINE_IID!(IID_IMapRoute2, 3519403020, 8723, 19120, 162, 96, 70, 179, 129, 105, 190, 172);
RT_INTERFACE!{interface IMapRoute2(IMapRoute2Vtbl): IInspectable(IInspectableVtbl) [IID_IMapRoute2] {
    fn get_ViolatedRestrictions(&self, out: *mut MapRouteRestrictions) -> HRESULT,
    fn get_HasBlockedRoads(&self, out: *mut bool) -> HRESULT
}}
impl IMapRoute2 {
    #[inline] pub unsafe fn get_violated_restrictions(&self) -> Result<MapRouteRestrictions> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_ViolatedRestrictions)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_has_blocked_roads(&self) -> Result<bool> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_HasBlockedRoads)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
}
DEFINE_IID!(IID_IMapRoute3, 2240618158, 62125, 17055, 187, 55, 205, 33, 9, 79, 252, 146);
RT_INTERFACE!{interface IMapRoute3(IMapRoute3Vtbl): IInspectable(IInspectableVtbl) [IID_IMapRoute3] {
    fn get_DurationWithoutTraffic(&self, out: *mut super::super::foundation::TimeSpan) -> HRESULT,
    fn get_TrafficCongestion(&self, out: *mut TrafficCongestion) -> HRESULT
}}
impl IMapRoute3 {
    #[inline] pub unsafe fn get_duration_without_traffic(&self) -> Result<super::super::foundation::TimeSpan> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_DurationWithoutTraffic)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_traffic_congestion(&self) -> Result<TrafficCongestion> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_TrafficCongestion)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
}
DEFINE_IID!(IID_IMapRoute4, 913083557, 12371, 20385, 128, 255, 212, 117, 243, 237, 30, 110);
RT_INTERFACE!{interface IMapRoute4(IMapRoute4Vtbl): IInspectable(IInspectableVtbl) [IID_IMapRoute4] {
    fn get_IsScenic(&self, out: *mut bool) -> HRESULT
}}
impl IMapRoute4 {
    #[inline] pub unsafe fn get_is_scenic(&self) -> Result<bool> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_IsScenic)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
}
DEFINE_IID!(IID_IMapRouteDrivingOptions, 1746220621, 50908, 18071, 164, 82, 177, 143, 143, 11, 103, 161);
RT_INTERFACE!{interface IMapRouteDrivingOptions(IMapRouteDrivingOptionsVtbl): IInspectable(IInspectableVtbl) [IID_IMapRouteDrivingOptions] {
    fn get_MaxAlternateRouteCount(&self, out: *mut u32) -> HRESULT,
    fn put_MaxAlternateRouteCount(&self, value: u32) -> HRESULT,
    fn get_InitialHeading(&self, out: *mut *mut super::super::foundation::IReference<f64>) -> HRESULT,
    fn put_InitialHeading(&self, value: *mut super::super::foundation::IReference<f64>) -> HRESULT,
    fn get_RouteOptimization(&self, out: *mut MapRouteOptimization) -> HRESULT,
    fn put_RouteOptimization(&self, value: MapRouteOptimization) -> HRESULT,
    fn get_RouteRestrictions(&self, out: *mut MapRouteRestrictions) -> HRESULT,
    fn put_RouteRestrictions(&self, value: MapRouteRestrictions) -> HRESULT
}}
impl IMapRouteDrivingOptions {
    #[inline] pub unsafe fn get_max_alternate_route_count(&self) -> Result<u32> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_MaxAlternateRouteCount)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_max_alternate_route_count(&self, value: u32) -> Result<()> {
        let hr = ((*self.lpVtbl).put_MaxAlternateRouteCount)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_initial_heading(&self) -> Result<ComPtr<super::super::foundation::IReference<f64>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_InitialHeading)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_initial_heading(&self, value: &super::super::foundation::IReference<f64>) -> Result<()> {
        let hr = ((*self.lpVtbl).put_InitialHeading)(self as *const _ as *mut _, value as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_route_optimization(&self) -> Result<MapRouteOptimization> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_RouteOptimization)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_route_optimization(&self, value: MapRouteOptimization) -> Result<()> {
        let hr = ((*self.lpVtbl).put_RouteOptimization)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_route_restrictions(&self) -> Result<MapRouteRestrictions> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_RouteRestrictions)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_route_restrictions(&self, value: MapRouteRestrictions) -> Result<()> {
        let hr = ((*self.lpVtbl).put_RouteRestrictions)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
}
RT_CLASS!{class MapRouteDrivingOptions: IMapRouteDrivingOptions}
impl RtActivatable<IActivationFactory> for MapRouteDrivingOptions {}
DEFINE_CLSID!(MapRouteDrivingOptions(&[87,105,110,100,111,119,115,46,83,101,114,118,105,99,101,115,46,77,97,112,115,46,77,97,112,82,111,117,116,101,68,114,105,118,105,110,103,79,112,116,105,111,110,115,0]) [CLSID_MapRouteDrivingOptions]);
RT_CLASS!{static class MapRouteFinder}
impl RtActivatable<IMapRouteFinderStatics> for MapRouteFinder {}
impl RtActivatable<IMapRouteFinderStatics2> for MapRouteFinder {}
impl RtActivatable<IMapRouteFinderStatics3> for MapRouteFinder {}
impl MapRouteFinder {
    #[cfg(feature="windows-devices")] #[inline] pub fn get_driving_route_async(startPoint: &super::super::devices::geolocation::Geopoint, endPoint: &super::super::devices::geolocation::Geopoint) -> Result<ComPtr<super::super::foundation::IAsyncOperation<MapRouteFinderResult>>> { unsafe {
        <Self as RtActivatable<IMapRouteFinderStatics>>::get_activation_factory().get_driving_route_async(startPoint, endPoint)
    }}
    #[cfg(feature="windows-devices")] #[inline] pub fn get_driving_route_with_optimization_async(startPoint: &super::super::devices::geolocation::Geopoint, endPoint: &super::super::devices::geolocation::Geopoint, optimization: MapRouteOptimization) -> Result<ComPtr<super::super::foundation::IAsyncOperation<MapRouteFinderResult>>> { unsafe {
        <Self as RtActivatable<IMapRouteFinderStatics>>::get_activation_factory().get_driving_route_with_optimization_async(startPoint, endPoint, optimization)
    }}
    #[cfg(feature="windows-devices")] #[inline] pub fn get_driving_route_with_optimization_and_restrictions_async(startPoint: &super::super::devices::geolocation::Geopoint, endPoint: &super::super::devices::geolocation::Geopoint, optimization: MapRouteOptimization, restrictions: MapRouteRestrictions) -> Result<ComPtr<super::super::foundation::IAsyncOperation<MapRouteFinderResult>>> { unsafe {
        <Self as RtActivatable<IMapRouteFinderStatics>>::get_activation_factory().get_driving_route_with_optimization_and_restrictions_async(startPoint, endPoint, optimization, restrictions)
    }}
    #[cfg(feature="windows-devices")] #[inline] pub fn get_driving_route_with_optimization_restrictions_and_heading_async(startPoint: &super::super::devices::geolocation::Geopoint, endPoint: &super::super::devices::geolocation::Geopoint, optimization: MapRouteOptimization, restrictions: MapRouteRestrictions, headingInDegrees: f64) -> Result<ComPtr<super::super::foundation::IAsyncOperation<MapRouteFinderResult>>> { unsafe {
        <Self as RtActivatable<IMapRouteFinderStatics>>::get_activation_factory().get_driving_route_with_optimization_restrictions_and_heading_async(startPoint, endPoint, optimization, restrictions, headingInDegrees)
    }}
    #[cfg(feature="windows-devices")] #[inline] pub fn get_driving_route_from_waypoints_async(wayPoints: &super::super::foundation::collections::IIterable<super::super::devices::geolocation::Geopoint>) -> Result<ComPtr<super::super::foundation::IAsyncOperation<MapRouteFinderResult>>> { unsafe {
        <Self as RtActivatable<IMapRouteFinderStatics>>::get_activation_factory().get_driving_route_from_waypoints_async(wayPoints)
    }}
    #[cfg(feature="windows-devices")] #[inline] pub fn get_driving_route_from_waypoints_and_optimization_async(wayPoints: &super::super::foundation::collections::IIterable<super::super::devices::geolocation::Geopoint>, optimization: MapRouteOptimization) -> Result<ComPtr<super::super::foundation::IAsyncOperation<MapRouteFinderResult>>> { unsafe {
        <Self as RtActivatable<IMapRouteFinderStatics>>::get_activation_factory().get_driving_route_from_waypoints_and_optimization_async(wayPoints, optimization)
    }}
    #[cfg(feature="windows-devices")] #[inline] pub fn get_driving_route_from_waypoints_optimization_and_restrictions_async(wayPoints: &super::super::foundation::collections::IIterable<super::super::devices::geolocation::Geopoint>, optimization: MapRouteOptimization, restrictions: MapRouteRestrictions) -> Result<ComPtr<super::super::foundation::IAsyncOperation<MapRouteFinderResult>>> { unsafe {
        <Self as RtActivatable<IMapRouteFinderStatics>>::get_activation_factory().get_driving_route_from_waypoints_optimization_and_restrictions_async(wayPoints, optimization, restrictions)
    }}
    #[cfg(feature="windows-devices")] #[inline] pub fn get_driving_route_from_waypoints_optimization_restrictions_and_heading_async(wayPoints: &super::super::foundation::collections::IIterable<super::super::devices::geolocation::Geopoint>, optimization: MapRouteOptimization, restrictions: MapRouteRestrictions, headingInDegrees: f64) -> Result<ComPtr<super::super::foundation::IAsyncOperation<MapRouteFinderResult>>> { unsafe {
        <Self as RtActivatable<IMapRouteFinderStatics>>::get_activation_factory().get_driving_route_from_waypoints_optimization_restrictions_and_heading_async(wayPoints, optimization, restrictions, headingInDegrees)
    }}
    #[cfg(feature="windows-devices")] #[inline] pub fn get_walking_route_async(startPoint: &super::super::devices::geolocation::Geopoint, endPoint: &super::super::devices::geolocation::Geopoint) -> Result<ComPtr<super::super::foundation::IAsyncOperation<MapRouteFinderResult>>> { unsafe {
        <Self as RtActivatable<IMapRouteFinderStatics>>::get_activation_factory().get_walking_route_async(startPoint, endPoint)
    }}
    #[cfg(feature="windows-devices")] #[inline] pub fn get_walking_route_from_waypoints_async(wayPoints: &super::super::foundation::collections::IIterable<super::super::devices::geolocation::Geopoint>) -> Result<ComPtr<super::super::foundation::IAsyncOperation<MapRouteFinderResult>>> { unsafe {
        <Self as RtActivatable<IMapRouteFinderStatics>>::get_activation_factory().get_walking_route_from_waypoints_async(wayPoints)
    }}
    #[cfg(feature="windows-devices")] #[inline] pub fn get_driving_route_with_options_async(startPoint: &super::super::devices::geolocation::Geopoint, endPoint: &super::super::devices::geolocation::Geopoint, options: &MapRouteDrivingOptions) -> Result<ComPtr<super::super::foundation::IAsyncOperation<MapRouteFinderResult>>> { unsafe {
        <Self as RtActivatable<IMapRouteFinderStatics2>>::get_activation_factory().get_driving_route_with_options_async(startPoint, endPoint, options)
    }}
    #[inline] pub fn get_driving_route_from_enhanced_waypoints_async(waypoints: &super::super::foundation::collections::IIterable<EnhancedWaypoint>) -> Result<ComPtr<super::super::foundation::IAsyncOperation<MapRouteFinderResult>>> { unsafe {
        <Self as RtActivatable<IMapRouteFinderStatics3>>::get_activation_factory().get_driving_route_from_enhanced_waypoints_async(waypoints)
    }}
    #[inline] pub fn get_driving_route_from_enhanced_waypoints_with_options_async(waypoints: &super::super::foundation::collections::IIterable<EnhancedWaypoint>, options: &MapRouteDrivingOptions) -> Result<ComPtr<super::super::foundation::IAsyncOperation<MapRouteFinderResult>>> { unsafe {
        <Self as RtActivatable<IMapRouteFinderStatics3>>::get_activation_factory().get_driving_route_from_enhanced_waypoints_with_options_async(waypoints, options)
    }}
}
DEFINE_CLSID!(MapRouteFinder(&[87,105,110,100,111,119,115,46,83,101,114,118,105,99,101,115,46,77,97,112,115,46,77,97,112,82,111,117,116,101,70,105,110,100,101,114,0]) [CLSID_MapRouteFinder]);
DEFINE_IID!(IID_IMapRouteFinderResult, 2825429786, 37922, 18092, 140, 161, 177, 97, 77, 75, 251, 226);
RT_INTERFACE!{interface IMapRouteFinderResult(IMapRouteFinderResultVtbl): IInspectable(IInspectableVtbl) [IID_IMapRouteFinderResult] {
    fn get_Route(&self, out: *mut *mut MapRoute) -> HRESULT,
    fn get_Status(&self, out: *mut MapRouteFinderStatus) -> HRESULT
}}
impl IMapRouteFinderResult {
    #[inline] pub unsafe fn get_route(&self) -> Result<ComPtr<MapRoute>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Route)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_status(&self) -> Result<MapRouteFinderStatus> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_Status)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
}
RT_CLASS!{class MapRouteFinderResult: IMapRouteFinderResult}
DEFINE_IID!(IID_IMapRouteFinderResult2, 544250989, 55564, 18120, 145, 198, 125, 75, 228, 239, 178, 21);
RT_INTERFACE!{interface IMapRouteFinderResult2(IMapRouteFinderResult2Vtbl): IInspectable(IInspectableVtbl) [IID_IMapRouteFinderResult2] {
    fn get_AlternateRoutes(&self, out: *mut *mut super::super::foundation::collections::IVectorView<MapRoute>) -> HRESULT
}}
impl IMapRouteFinderResult2 {
    #[inline] pub unsafe fn get_alternate_routes(&self) -> Result<ComPtr<super::super::foundation::collections::IVectorView<MapRoute>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_AlternateRoutes)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
DEFINE_IID!(IID_IMapRouteFinderStatics, 3097871631, 7268, 19514, 129, 235, 31, 124, 21, 42, 251, 187);
RT_INTERFACE!{static interface IMapRouteFinderStatics(IMapRouteFinderStaticsVtbl): IInspectable(IInspectableVtbl) [IID_IMapRouteFinderStatics] {
    #[cfg(feature="windows-devices")] fn GetDrivingRouteAsync(&self, startPoint: *mut super::super::devices::geolocation::Geopoint, endPoint: *mut super::super::devices::geolocation::Geopoint, out: *mut *mut super::super::foundation::IAsyncOperation<MapRouteFinderResult>) -> HRESULT,
    #[cfg(feature="windows-devices")] fn GetDrivingRouteWithOptimizationAsync(&self, startPoint: *mut super::super::devices::geolocation::Geopoint, endPoint: *mut super::super::devices::geolocation::Geopoint, optimization: MapRouteOptimization, out: *mut *mut super::super::foundation::IAsyncOperation<MapRouteFinderResult>) -> HRESULT,
    #[cfg(feature="windows-devices")] fn GetDrivingRouteWithOptimizationAndRestrictionsAsync(&self, startPoint: *mut super::super::devices::geolocation::Geopoint, endPoint: *mut super::super::devices::geolocation::Geopoint, optimization: MapRouteOptimization, restrictions: MapRouteRestrictions, out: *mut *mut super::super::foundation::IAsyncOperation<MapRouteFinderResult>) -> HRESULT,
    #[cfg(feature="windows-devices")] fn GetDrivingRouteWithOptimizationRestrictionsAndHeadingAsync(&self, startPoint: *mut super::super::devices::geolocation::Geopoint, endPoint: *mut super::super::devices::geolocation::Geopoint, optimization: MapRouteOptimization, restrictions: MapRouteRestrictions, headingInDegrees: f64, out: *mut *mut super::super::foundation::IAsyncOperation<MapRouteFinderResult>) -> HRESULT,
    #[cfg(feature="windows-devices")] fn GetDrivingRouteFromWaypointsAsync(&self, wayPoints: *mut super::super::foundation::collections::IIterable<super::super::devices::geolocation::Geopoint>, out: *mut *mut super::super::foundation::IAsyncOperation<MapRouteFinderResult>) -> HRESULT,
    #[cfg(feature="windows-devices")] fn GetDrivingRouteFromWaypointsAndOptimizationAsync(&self, wayPoints: *mut super::super::foundation::collections::IIterable<super::super::devices::geolocation::Geopoint>, optimization: MapRouteOptimization, out: *mut *mut super::super::foundation::IAsyncOperation<MapRouteFinderResult>) -> HRESULT,
    #[cfg(feature="windows-devices")] fn GetDrivingRouteFromWaypointsOptimizationAndRestrictionsAsync(&self, wayPoints: *mut super::super::foundation::collections::IIterable<super::super::devices::geolocation::Geopoint>, optimization: MapRouteOptimization, restrictions: MapRouteRestrictions, out: *mut *mut super::super::foundation::IAsyncOperation<MapRouteFinderResult>) -> HRESULT,
    #[cfg(feature="windows-devices")] fn GetDrivingRouteFromWaypointsOptimizationRestrictionsAndHeadingAsync(&self, wayPoints: *mut super::super::foundation::collections::IIterable<super::super::devices::geolocation::Geopoint>, optimization: MapRouteOptimization, restrictions: MapRouteRestrictions, headingInDegrees: f64, out: *mut *mut super::super::foundation::IAsyncOperation<MapRouteFinderResult>) -> HRESULT,
    #[cfg(feature="windows-devices")] fn GetWalkingRouteAsync(&self, startPoint: *mut super::super::devices::geolocation::Geopoint, endPoint: *mut super::super::devices::geolocation::Geopoint, out: *mut *mut super::super::foundation::IAsyncOperation<MapRouteFinderResult>) -> HRESULT,
    #[cfg(feature="windows-devices")] fn GetWalkingRouteFromWaypointsAsync(&self, wayPoints: *mut super::super::foundation::collections::IIterable<super::super::devices::geolocation::Geopoint>, out: *mut *mut super::super::foundation::IAsyncOperation<MapRouteFinderResult>) -> HRESULT
}}
impl IMapRouteFinderStatics {
    #[cfg(feature="windows-devices")] #[inline] pub unsafe fn get_driving_route_async(&self, startPoint: &super::super::devices::geolocation::Geopoint, endPoint: &super::super::devices::geolocation::Geopoint) -> Result<ComPtr<super::super::foundation::IAsyncOperation<MapRouteFinderResult>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetDrivingRouteAsync)(self as *const _ as *mut _, startPoint as *const _ as *mut _, endPoint as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[cfg(feature="windows-devices")] #[inline] pub unsafe fn get_driving_route_with_optimization_async(&self, startPoint: &super::super::devices::geolocation::Geopoint, endPoint: &super::super::devices::geolocation::Geopoint, optimization: MapRouteOptimization) -> Result<ComPtr<super::super::foundation::IAsyncOperation<MapRouteFinderResult>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetDrivingRouteWithOptimizationAsync)(self as *const _ as *mut _, startPoint as *const _ as *mut _, endPoint as *const _ as *mut _, optimization, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[cfg(feature="windows-devices")] #[inline] pub unsafe fn get_driving_route_with_optimization_and_restrictions_async(&self, startPoint: &super::super::devices::geolocation::Geopoint, endPoint: &super::super::devices::geolocation::Geopoint, optimization: MapRouteOptimization, restrictions: MapRouteRestrictions) -> Result<ComPtr<super::super::foundation::IAsyncOperation<MapRouteFinderResult>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetDrivingRouteWithOptimizationAndRestrictionsAsync)(self as *const _ as *mut _, startPoint as *const _ as *mut _, endPoint as *const _ as *mut _, optimization, restrictions, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[cfg(feature="windows-devices")] #[inline] pub unsafe fn get_driving_route_with_optimization_restrictions_and_heading_async(&self, startPoint: &super::super::devices::geolocation::Geopoint, endPoint: &super::super::devices::geolocation::Geopoint, optimization: MapRouteOptimization, restrictions: MapRouteRestrictions, headingInDegrees: f64) -> Result<ComPtr<super::super::foundation::IAsyncOperation<MapRouteFinderResult>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetDrivingRouteWithOptimizationRestrictionsAndHeadingAsync)(self as *const _ as *mut _, startPoint as *const _ as *mut _, endPoint as *const _ as *mut _, optimization, restrictions, headingInDegrees, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[cfg(feature="windows-devices")] #[inline] pub unsafe fn get_driving_route_from_waypoints_async(&self, wayPoints: &super::super::foundation::collections::IIterable<super::super::devices::geolocation::Geopoint>) -> Result<ComPtr<super::super::foundation::IAsyncOperation<MapRouteFinderResult>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetDrivingRouteFromWaypointsAsync)(self as *const _ as *mut _, wayPoints as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[cfg(feature="windows-devices")] #[inline] pub unsafe fn get_driving_route_from_waypoints_and_optimization_async(&self, wayPoints: &super::super::foundation::collections::IIterable<super::super::devices::geolocation::Geopoint>, optimization: MapRouteOptimization) -> Result<ComPtr<super::super::foundation::IAsyncOperation<MapRouteFinderResult>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetDrivingRouteFromWaypointsAndOptimizationAsync)(self as *const _ as *mut _, wayPoints as *const _ as *mut _, optimization, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[cfg(feature="windows-devices")] #[inline] pub unsafe fn get_driving_route_from_waypoints_optimization_and_restrictions_async(&self, wayPoints: &super::super::foundation::collections::IIterable<super::super::devices::geolocation::Geopoint>, optimization: MapRouteOptimization, restrictions: MapRouteRestrictions) -> Result<ComPtr<super::super::foundation::IAsyncOperation<MapRouteFinderResult>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetDrivingRouteFromWaypointsOptimizationAndRestrictionsAsync)(self as *const _ as *mut _, wayPoints as *const _ as *mut _, optimization, restrictions, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[cfg(feature="windows-devices")] #[inline] pub unsafe fn get_driving_route_from_waypoints_optimization_restrictions_and_heading_async(&self, wayPoints: &super::super::foundation::collections::IIterable<super::super::devices::geolocation::Geopoint>, optimization: MapRouteOptimization, restrictions: MapRouteRestrictions, headingInDegrees: f64) -> Result<ComPtr<super::super::foundation::IAsyncOperation<MapRouteFinderResult>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetDrivingRouteFromWaypointsOptimizationRestrictionsAndHeadingAsync)(self as *const _ as *mut _, wayPoints as *const _ as *mut _, optimization, restrictions, headingInDegrees, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[cfg(feature="windows-devices")] #[inline] pub unsafe fn get_walking_route_async(&self, startPoint: &super::super::devices::geolocation::Geopoint, endPoint: &super::super::devices::geolocation::Geopoint) -> Result<ComPtr<super::super::foundation::IAsyncOperation<MapRouteFinderResult>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetWalkingRouteAsync)(self as *const _ as *mut _, startPoint as *const _ as *mut _, endPoint as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[cfg(feature="windows-devices")] #[inline] pub unsafe fn get_walking_route_from_waypoints_async(&self, wayPoints: &super::super::foundation::collections::IIterable<super::super::devices::geolocation::Geopoint>) -> Result<ComPtr<super::super::foundation::IAsyncOperation<MapRouteFinderResult>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetWalkingRouteFromWaypointsAsync)(self as *const _ as *mut _, wayPoints as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
DEFINE_IID!(IID_IMapRouteFinderStatics2, 2949393523, 30560, 18863, 179, 189, 186, 241, 53, 183, 3, 225);
RT_INTERFACE!{static interface IMapRouteFinderStatics2(IMapRouteFinderStatics2Vtbl): IInspectable(IInspectableVtbl) [IID_IMapRouteFinderStatics2] {
    #[cfg(feature="windows-devices")] fn GetDrivingRouteWithOptionsAsync(&self, startPoint: *mut super::super::devices::geolocation::Geopoint, endPoint: *mut super::super::devices::geolocation::Geopoint, options: *mut MapRouteDrivingOptions, out: *mut *mut super::super::foundation::IAsyncOperation<MapRouteFinderResult>) -> HRESULT
}}
impl IMapRouteFinderStatics2 {
    #[cfg(feature="windows-devices")] #[inline] pub unsafe fn get_driving_route_with_options_async(&self, startPoint: &super::super::devices::geolocation::Geopoint, endPoint: &super::super::devices::geolocation::Geopoint, options: &MapRouteDrivingOptions) -> Result<ComPtr<super::super::foundation::IAsyncOperation<MapRouteFinderResult>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetDrivingRouteWithOptionsAsync)(self as *const _ as *mut _, startPoint as *const _ as *mut _, endPoint as *const _ as *mut _, options as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
DEFINE_IID!(IID_IMapRouteFinderStatics3, 4127818036, 22803, 4582, 139, 119, 134, 243, 12, 168, 147, 211);
RT_INTERFACE!{static interface IMapRouteFinderStatics3(IMapRouteFinderStatics3Vtbl): IInspectable(IInspectableVtbl) [IID_IMapRouteFinderStatics3] {
    fn GetDrivingRouteFromEnhancedWaypointsAsync(&self, waypoints: *mut super::super::foundation::collections::IIterable<EnhancedWaypoint>, out: *mut *mut super::super::foundation::IAsyncOperation<MapRouteFinderResult>) -> HRESULT,
    fn GetDrivingRouteFromEnhancedWaypointsWithOptionsAsync(&self, waypoints: *mut super::super::foundation::collections::IIterable<EnhancedWaypoint>, options: *mut MapRouteDrivingOptions, out: *mut *mut super::super::foundation::IAsyncOperation<MapRouteFinderResult>) -> HRESULT
}}
impl IMapRouteFinderStatics3 {
    #[inline] pub unsafe fn get_driving_route_from_enhanced_waypoints_async(&self, waypoints: &super::super::foundation::collections::IIterable<EnhancedWaypoint>) -> Result<ComPtr<super::super::foundation::IAsyncOperation<MapRouteFinderResult>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetDrivingRouteFromEnhancedWaypointsAsync)(self as *const _ as *mut _, waypoints as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_driving_route_from_enhanced_waypoints_with_options_async(&self, waypoints: &super::super::foundation::collections::IIterable<EnhancedWaypoint>, options: &MapRouteDrivingOptions) -> Result<ComPtr<super::super::foundation::IAsyncOperation<MapRouteFinderResult>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetDrivingRouteFromEnhancedWaypointsWithOptionsAsync)(self as *const _ as *mut _, waypoints as *const _ as *mut _, options as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
RT_ENUM! { enum MapRouteFinderStatus: i32 {
    Success (MapRouteFinderStatus_Success) = 0, UnknownError (MapRouteFinderStatus_UnknownError) = 1, InvalidCredentials (MapRouteFinderStatus_InvalidCredentials) = 2, NoRouteFound (MapRouteFinderStatus_NoRouteFound) = 3, NoRouteFoundWithGivenOptions (MapRouteFinderStatus_NoRouteFoundWithGivenOptions) = 4, StartPointNotFound (MapRouteFinderStatus_StartPointNotFound) = 5, EndPointNotFound (MapRouteFinderStatus_EndPointNotFound) = 6, NoPedestrianRouteFound (MapRouteFinderStatus_NoPedestrianRouteFound) = 7, NetworkFailure (MapRouteFinderStatus_NetworkFailure) = 8, NotSupported (MapRouteFinderStatus_NotSupported) = 9,
}}
DEFINE_IID!(IID_IMapRouteLeg, 2532881142, 23482, 19735, 157, 182, 26, 38, 63, 236, 116, 113);
RT_INTERFACE!{interface IMapRouteLeg(IMapRouteLegVtbl): IInspectable(IInspectableVtbl) [IID_IMapRouteLeg] {
    #[cfg(not(feature="windows-devices"))] fn __Dummy0(&self) -> (),
    #[cfg(feature="windows-devices")] fn get_BoundingBox(&self, out: *mut *mut super::super::devices::geolocation::GeoboundingBox) -> HRESULT,
    #[cfg(not(feature="windows-devices"))] fn __Dummy1(&self) -> (),
    #[cfg(feature="windows-devices")] fn get_Path(&self, out: *mut *mut super::super::devices::geolocation::Geopath) -> HRESULT,
    fn get_LengthInMeters(&self, out: *mut f64) -> HRESULT,
    fn get_EstimatedDuration(&self, out: *mut super::super::foundation::TimeSpan) -> HRESULT,
    fn get_Maneuvers(&self, out: *mut *mut super::super::foundation::collections::IVectorView<MapRouteManeuver>) -> HRESULT
}}
impl IMapRouteLeg {
    #[cfg(feature="windows-devices")] #[inline] pub unsafe fn get_bounding_box(&self) -> Result<ComPtr<super::super::devices::geolocation::GeoboundingBox>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_BoundingBox)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[cfg(feature="windows-devices")] #[inline] pub unsafe fn get_path(&self) -> Result<ComPtr<super::super::devices::geolocation::Geopath>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Path)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_length_in_meters(&self) -> Result<f64> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_LengthInMeters)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_estimated_duration(&self) -> Result<super::super::foundation::TimeSpan> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_EstimatedDuration)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_maneuvers(&self) -> Result<ComPtr<super::super::foundation::collections::IVectorView<MapRouteManeuver>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Maneuvers)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
RT_CLASS!{class MapRouteLeg: IMapRouteLeg}
DEFINE_IID!(IID_IMapRouteLeg2, 48367149, 51654, 17848, 142, 84, 26, 16, 181, 122, 23, 232);
RT_INTERFACE!{interface IMapRouteLeg2(IMapRouteLeg2Vtbl): IInspectable(IInspectableVtbl) [IID_IMapRouteLeg2] {
    fn get_DurationWithoutTraffic(&self, out: *mut super::super::foundation::TimeSpan) -> HRESULT,
    fn get_TrafficCongestion(&self, out: *mut TrafficCongestion) -> HRESULT
}}
impl IMapRouteLeg2 {
    #[inline] pub unsafe fn get_duration_without_traffic(&self) -> Result<super::super::foundation::TimeSpan> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_DurationWithoutTraffic)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_traffic_congestion(&self) -> Result<TrafficCongestion> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_TrafficCongestion)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
}
DEFINE_IID!(IID_IMapRouteManeuver, 3982235632, 42667, 19813, 160, 134, 250, 138, 126, 52, 13, 242);
RT_INTERFACE!{interface IMapRouteManeuver(IMapRouteManeuverVtbl): IInspectable(IInspectableVtbl) [IID_IMapRouteManeuver] {
    #[cfg(not(feature="windows-devices"))] fn __Dummy0(&self) -> (),
    #[cfg(feature="windows-devices")] fn get_StartingPoint(&self, out: *mut *mut super::super::devices::geolocation::Geopoint) -> HRESULT,
    fn get_LengthInMeters(&self, out: *mut f64) -> HRESULT,
    fn get_InstructionText(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Kind(&self, out: *mut MapRouteManeuverKind) -> HRESULT,
    fn get_ExitNumber(&self, out: *mut HSTRING) -> HRESULT,
    fn get_ManeuverNotices(&self, out: *mut MapManeuverNotices) -> HRESULT
}}
impl IMapRouteManeuver {
    #[cfg(feature="windows-devices")] #[inline] pub unsafe fn get_starting_point(&self) -> Result<ComPtr<super::super::devices::geolocation::Geopoint>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_StartingPoint)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_length_in_meters(&self) -> Result<f64> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_LengthInMeters)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_instruction_text(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_InstructionText)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_kind(&self) -> Result<MapRouteManeuverKind> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_Kind)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_exit_number(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_ExitNumber)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_maneuver_notices(&self) -> Result<MapManeuverNotices> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_ManeuverNotices)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
}
RT_CLASS!{class MapRouteManeuver: IMapRouteManeuver}
DEFINE_IID!(IID_IMapRouteManeuver2, 1568394652, 31899, 16863, 131, 139, 234, 226, 30, 75, 5, 169);
RT_INTERFACE!{interface IMapRouteManeuver2(IMapRouteManeuver2Vtbl): IInspectable(IInspectableVtbl) [IID_IMapRouteManeuver2] {
    fn get_StartHeading(&self, out: *mut f64) -> HRESULT,
    fn get_EndHeading(&self, out: *mut f64) -> HRESULT,
    fn get_StreetName(&self, out: *mut HSTRING) -> HRESULT
}}
impl IMapRouteManeuver2 {
    #[inline] pub unsafe fn get_start_heading(&self) -> Result<f64> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_StartHeading)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_end_heading(&self) -> Result<f64> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_EndHeading)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_street_name(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_StreetName)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
}
DEFINE_IID!(IID_IMapRouteManeuver3, 2795583711, 1155, 16742, 133, 190, 185, 147, 54, 193, 24, 117);
RT_INTERFACE!{interface IMapRouteManeuver3(IMapRouteManeuver3Vtbl): IInspectable(IInspectableVtbl) [IID_IMapRouteManeuver3] {
    fn get_Warnings(&self, out: *mut *mut super::super::foundation::collections::IVectorView<ManeuverWarning>) -> HRESULT
}}
impl IMapRouteManeuver3 {
    #[inline] pub unsafe fn get_warnings(&self) -> Result<ComPtr<super::super::foundation::collections::IVectorView<ManeuverWarning>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Warnings)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
RT_ENUM! { enum MapRouteManeuverKind: i32 {
    None (MapRouteManeuverKind_None) = 0, Start (MapRouteManeuverKind_Start) = 1, Stopover (MapRouteManeuverKind_Stopover) = 2, StopoverResume (MapRouteManeuverKind_StopoverResume) = 3, End (MapRouteManeuverKind_End) = 4, GoStraight (MapRouteManeuverKind_GoStraight) = 5, UTurnLeft (MapRouteManeuverKind_UTurnLeft) = 6, UTurnRight (MapRouteManeuverKind_UTurnRight) = 7, TurnKeepLeft (MapRouteManeuverKind_TurnKeepLeft) = 8, TurnKeepRight (MapRouteManeuverKind_TurnKeepRight) = 9, TurnLightLeft (MapRouteManeuverKind_TurnLightLeft) = 10, TurnLightRight (MapRouteManeuverKind_TurnLightRight) = 11, TurnLeft (MapRouteManeuverKind_TurnLeft) = 12, TurnRight (MapRouteManeuverKind_TurnRight) = 13, TurnHardLeft (MapRouteManeuverKind_TurnHardLeft) = 14, TurnHardRight (MapRouteManeuverKind_TurnHardRight) = 15, FreewayEnterLeft (MapRouteManeuverKind_FreewayEnterLeft) = 16, FreewayEnterRight (MapRouteManeuverKind_FreewayEnterRight) = 17, FreewayLeaveLeft (MapRouteManeuverKind_FreewayLeaveLeft) = 18, FreewayLeaveRight (MapRouteManeuverKind_FreewayLeaveRight) = 19, FreewayContinueLeft (MapRouteManeuverKind_FreewayContinueLeft) = 20, FreewayContinueRight (MapRouteManeuverKind_FreewayContinueRight) = 21, TrafficCircleLeft (MapRouteManeuverKind_TrafficCircleLeft) = 22, TrafficCircleRight (MapRouteManeuverKind_TrafficCircleRight) = 23, TakeFerry (MapRouteManeuverKind_TakeFerry) = 24,
}}
RT_ENUM! { enum MapRouteOptimization: i32 {
    Time (MapRouteOptimization_Time) = 0, Distance (MapRouteOptimization_Distance) = 1, TimeWithTraffic (MapRouteOptimization_TimeWithTraffic) = 2, Scenic (MapRouteOptimization_Scenic) = 3,
}}
RT_ENUM! { enum MapRouteRestrictions: u32 {
    None (MapRouteRestrictions_None) = 0, Highways (MapRouteRestrictions_Highways) = 1, TollRoads (MapRouteRestrictions_TollRoads) = 2, Ferries (MapRouteRestrictions_Ferries) = 4, Tunnels (MapRouteRestrictions_Tunnels) = 8, DirtRoads (MapRouteRestrictions_DirtRoads) = 16, Motorail (MapRouteRestrictions_Motorail) = 32,
}}
RT_CLASS!{static class MapService}
impl RtActivatable<IMapServiceStatics> for MapService {}
impl RtActivatable<IMapServiceStatics2> for MapService {}
impl RtActivatable<IMapServiceStatics3> for MapService {}
impl RtActivatable<IMapServiceStatics4> for MapService {}
impl MapService {
    #[inline] pub fn set_service_token(value: &HStringArg) -> Result<()> { unsafe {
        <Self as RtActivatable<IMapServiceStatics>>::get_activation_factory().set_service_token(value)
    }}
    #[inline] pub fn get_service_token() -> Result<HString> { unsafe {
        <Self as RtActivatable<IMapServiceStatics>>::get_activation_factory().get_service_token()
    }}
    #[inline] pub fn get_world_view_region_code() -> Result<HString> { unsafe {
        <Self as RtActivatable<IMapServiceStatics2>>::get_activation_factory().get_world_view_region_code()
    }}
    #[inline] pub fn get_data_attributions() -> Result<HString> { unsafe {
        <Self as RtActivatable<IMapServiceStatics3>>::get_activation_factory().get_data_attributions()
    }}
    #[inline] pub fn set_data_usage_preference(value: MapServiceDataUsagePreference) -> Result<()> { unsafe {
        <Self as RtActivatable<IMapServiceStatics4>>::get_activation_factory().set_data_usage_preference(value)
    }}
    #[inline] pub fn get_data_usage_preference() -> Result<MapServiceDataUsagePreference> { unsafe {
        <Self as RtActivatable<IMapServiceStatics4>>::get_activation_factory().get_data_usage_preference()
    }}
}
DEFINE_CLSID!(MapService(&[87,105,110,100,111,119,115,46,83,101,114,118,105,99,101,115,46,77,97,112,115,46,77,97,112,83,101,114,118,105,99,101,0]) [CLSID_MapService]);
RT_ENUM! { enum MapServiceDataUsagePreference: i32 {
    Default (MapServiceDataUsagePreference_Default) = 0, OfflineMapDataOnly (MapServiceDataUsagePreference_OfflineMapDataOnly) = 1,
}}
DEFINE_IID!(IID_IMapServiceStatics, 21278085, 49228, 19677, 135, 26, 160, 114, 109, 9, 124, 212);
RT_INTERFACE!{static interface IMapServiceStatics(IMapServiceStaticsVtbl): IInspectable(IInspectableVtbl) [IID_IMapServiceStatics] {
    fn put_ServiceToken(&self, value: HSTRING) -> HRESULT,
    fn get_ServiceToken(&self, out: *mut HSTRING) -> HRESULT
}}
impl IMapServiceStatics {
    #[inline] pub unsafe fn set_service_token(&self, value: &HStringArg) -> Result<()> {
        let hr = ((*self.lpVtbl).put_ServiceToken)(self as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_service_token(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_ServiceToken)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
}
DEFINE_IID!(IID_IMapServiceStatics2, 4162404077, 40069, 16553, 136, 150, 15, 195, 253, 43, 124, 42);
RT_INTERFACE!{static interface IMapServiceStatics2(IMapServiceStatics2Vtbl): IInspectable(IInspectableVtbl) [IID_IMapServiceStatics2] {
    fn get_WorldViewRegionCode(&self, out: *mut HSTRING) -> HRESULT
}}
impl IMapServiceStatics2 {
    #[inline] pub unsafe fn get_world_view_region_code(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_WorldViewRegionCode)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
}
DEFINE_IID!(IID_IMapServiceStatics3, 168939040, 25511, 18516, 179, 85, 214, 220, 218, 34, 61, 27);
RT_INTERFACE!{static interface IMapServiceStatics3(IMapServiceStatics3Vtbl): IInspectable(IInspectableVtbl) [IID_IMapServiceStatics3] {
    fn get_DataAttributions(&self, out: *mut HSTRING) -> HRESULT
}}
impl IMapServiceStatics3 {
    #[inline] pub unsafe fn get_data_attributions(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_DataAttributions)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
}
DEFINE_IID!(IID_IMapServiceStatics4, 143272034, 27324, 16910, 148, 95, 76, 253, 137, 198, 115, 86);
RT_INTERFACE!{static interface IMapServiceStatics4(IMapServiceStatics4Vtbl): IInspectable(IInspectableVtbl) [IID_IMapServiceStatics4] {
    fn put_DataUsagePreference(&self, value: MapServiceDataUsagePreference) -> HRESULT,
    fn get_DataUsagePreference(&self, out: *mut MapServiceDataUsagePreference) -> HRESULT
}}
impl IMapServiceStatics4 {
    #[inline] pub unsafe fn set_data_usage_preference(&self, value: MapServiceDataUsagePreference) -> Result<()> {
        let hr = ((*self.lpVtbl).put_DataUsagePreference)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_data_usage_preference(&self) -> Result<MapServiceDataUsagePreference> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_DataUsagePreference)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
}
DEFINE_IID!(IID_IPlaceInfo, 2584219830, 12744, 20330, 159, 24, 149, 11, 76, 56, 149, 26);
RT_INTERFACE!{interface IPlaceInfo(IPlaceInfoVtbl): IInspectable(IInspectableVtbl) [IID_IPlaceInfo] {
    fn Show(&self, selection: super::super::foundation::Rect) -> HRESULT,
    #[cfg(not(feature="windows-ui"))] fn __Dummy1(&self) -> (),
    #[cfg(feature="windows-ui")] fn ShowWithPreferredPlacement(&self, selection: super::super::foundation::Rect, preferredPlacement: super::super::ui::popups::Placement) -> HRESULT,
    fn get_Identifier(&self, out: *mut HSTRING) -> HRESULT,
    fn get_DisplayName(&self, out: *mut HSTRING) -> HRESULT,
    fn get_DisplayAddress(&self, out: *mut HSTRING) -> HRESULT,
    #[cfg(feature="windows-devices")] fn get_Geoshape(&self, out: *mut *mut super::super::devices::geolocation::IGeoshape) -> HRESULT
}}
impl IPlaceInfo {
    #[inline] pub unsafe fn show(&self, selection: super::super::foundation::Rect) -> Result<()> {
        let hr = ((*self.lpVtbl).Show)(self as *const _ as *mut _, selection);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[cfg(feature="windows-ui")] #[inline] pub unsafe fn show_with_preferred_placement(&self, selection: super::super::foundation::Rect, preferredPlacement: super::super::ui::popups::Placement) -> Result<()> {
        let hr = ((*self.lpVtbl).ShowWithPreferredPlacement)(self as *const _ as *mut _, selection, preferredPlacement);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_identifier(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Identifier)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_display_name(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_DisplayName)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_display_address(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_DisplayAddress)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[cfg(feature="windows-devices")] #[inline] pub unsafe fn get_geoshape(&self) -> Result<ComPtr<super::super::devices::geolocation::IGeoshape>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Geoshape)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
RT_CLASS!{class PlaceInfo: IPlaceInfo}
impl RtActivatable<IPlaceInfoStatics> for PlaceInfo {}
impl PlaceInfo {
    #[cfg(feature="windows-devices")] #[inline] pub fn create(referencePoint: &super::super::devices::geolocation::Geopoint) -> Result<ComPtr<PlaceInfo>> { unsafe {
        <Self as RtActivatable<IPlaceInfoStatics>>::get_activation_factory().create(referencePoint)
    }}
    #[cfg(feature="windows-devices")] #[inline] pub fn create_with_geopoint_and_options(referencePoint: &super::super::devices::geolocation::Geopoint, options: &PlaceInfoCreateOptions) -> Result<ComPtr<PlaceInfo>> { unsafe {
        <Self as RtActivatable<IPlaceInfoStatics>>::get_activation_factory().create_with_geopoint_and_options(referencePoint, options)
    }}
    #[inline] pub fn create_from_identifier(identifier: &HStringArg) -> Result<ComPtr<PlaceInfo>> { unsafe {
        <Self as RtActivatable<IPlaceInfoStatics>>::get_activation_factory().create_from_identifier(identifier)
    }}
    #[cfg(feature="windows-devices")] #[inline] pub fn create_from_identifier_with_options(identifier: &HStringArg, defaultPoint: &super::super::devices::geolocation::Geopoint, options: &PlaceInfoCreateOptions) -> Result<ComPtr<PlaceInfo>> { unsafe {
        <Self as RtActivatable<IPlaceInfoStatics>>::get_activation_factory().create_from_identifier_with_options(identifier, defaultPoint, options)
    }}
    #[inline] pub fn create_from_map_location(location: &MapLocation) -> Result<ComPtr<PlaceInfo>> { unsafe {
        <Self as RtActivatable<IPlaceInfoStatics>>::get_activation_factory().create_from_map_location(location)
    }}
    #[inline] pub fn get_is_show_supported() -> Result<bool> { unsafe {
        <Self as RtActivatable<IPlaceInfoStatics>>::get_activation_factory().get_is_show_supported()
    }}
}
DEFINE_CLSID!(PlaceInfo(&[87,105,110,100,111,119,115,46,83,101,114,118,105,99,101,115,46,77,97,112,115,46,80,108,97,99,101,73,110,102,111,0]) [CLSID_PlaceInfo]);
DEFINE_IID!(IID_IPlaceInfoCreateOptions, 3442721061, 26609, 19379, 153, 7, 236, 206, 147, 155, 3, 153);
RT_INTERFACE!{interface IPlaceInfoCreateOptions(IPlaceInfoCreateOptionsVtbl): IInspectable(IInspectableVtbl) [IID_IPlaceInfoCreateOptions] {
    fn put_DisplayName(&self, value: HSTRING) -> HRESULT,
    fn get_DisplayName(&self, out: *mut HSTRING) -> HRESULT,
    fn put_DisplayAddress(&self, value: HSTRING) -> HRESULT,
    fn get_DisplayAddress(&self, out: *mut HSTRING) -> HRESULT
}}
impl IPlaceInfoCreateOptions {
    #[inline] pub unsafe fn set_display_name(&self, value: &HStringArg) -> Result<()> {
        let hr = ((*self.lpVtbl).put_DisplayName)(self as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_display_name(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_DisplayName)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_display_address(&self, value: &HStringArg) -> Result<()> {
        let hr = ((*self.lpVtbl).put_DisplayAddress)(self as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_display_address(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_DisplayAddress)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
}
RT_CLASS!{class PlaceInfoCreateOptions: IPlaceInfoCreateOptions}
impl RtActivatable<IActivationFactory> for PlaceInfoCreateOptions {}
DEFINE_CLSID!(PlaceInfoCreateOptions(&[87,105,110,100,111,119,115,46,83,101,114,118,105,99,101,115,46,77,97,112,115,46,80,108,97,99,101,73,110,102,111,67,114,101,97,116,101,79,112,116,105,111,110,115,0]) [CLSID_PlaceInfoCreateOptions]);
DEFINE_IID!(IID_IPlaceInfoStatics, 2193227633, 27856, 18596, 175, 217, 94, 216, 32, 151, 147, 107);
RT_INTERFACE!{static interface IPlaceInfoStatics(IPlaceInfoStaticsVtbl): IInspectable(IInspectableVtbl) [IID_IPlaceInfoStatics] {
    #[cfg(not(feature="windows-devices"))] fn __Dummy0(&self) -> (),
    #[cfg(feature="windows-devices")] fn Create(&self, referencePoint: *mut super::super::devices::geolocation::Geopoint, out: *mut *mut PlaceInfo) -> HRESULT,
    #[cfg(not(feature="windows-devices"))] fn __Dummy1(&self) -> (),
    #[cfg(feature="windows-devices")] fn CreateWithGeopointAndOptions(&self, referencePoint: *mut super::super::devices::geolocation::Geopoint, options: *mut PlaceInfoCreateOptions, out: *mut *mut PlaceInfo) -> HRESULT,
    fn CreateFromIdentifier(&self, identifier: HSTRING, out: *mut *mut PlaceInfo) -> HRESULT,
    #[cfg(not(feature="windows-devices"))] fn __Dummy3(&self) -> (),
    #[cfg(feature="windows-devices")] fn CreateFromIdentifierWithOptions(&self, identifier: HSTRING, defaultPoint: *mut super::super::devices::geolocation::Geopoint, options: *mut PlaceInfoCreateOptions, out: *mut *mut PlaceInfo) -> HRESULT,
    fn CreateFromMapLocation(&self, location: *mut MapLocation, out: *mut *mut PlaceInfo) -> HRESULT,
    fn get_IsShowSupported(&self, out: *mut bool) -> HRESULT
}}
impl IPlaceInfoStatics {
    #[cfg(feature="windows-devices")] #[inline] pub unsafe fn create(&self, referencePoint: &super::super::devices::geolocation::Geopoint) -> Result<ComPtr<PlaceInfo>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).Create)(self as *const _ as *mut _, referencePoint as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[cfg(feature="windows-devices")] #[inline] pub unsafe fn create_with_geopoint_and_options(&self, referencePoint: &super::super::devices::geolocation::Geopoint, options: &PlaceInfoCreateOptions) -> Result<ComPtr<PlaceInfo>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).CreateWithGeopointAndOptions)(self as *const _ as *mut _, referencePoint as *const _ as *mut _, options as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn create_from_identifier(&self, identifier: &HStringArg) -> Result<ComPtr<PlaceInfo>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).CreateFromIdentifier)(self as *const _ as *mut _, identifier.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[cfg(feature="windows-devices")] #[inline] pub unsafe fn create_from_identifier_with_options(&self, identifier: &HStringArg, defaultPoint: &super::super::devices::geolocation::Geopoint, options: &PlaceInfoCreateOptions) -> Result<ComPtr<PlaceInfo>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).CreateFromIdentifierWithOptions)(self as *const _ as *mut _, identifier.get(), defaultPoint as *const _ as *mut _, options as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn create_from_map_location(&self, location: &MapLocation) -> Result<ComPtr<PlaceInfo>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).CreateFromMapLocation)(self as *const _ as *mut _, location as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_is_show_supported(&self) -> Result<bool> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_IsShowSupported)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
}
RT_ENUM! { enum TrafficCongestion: i32 {
    Unknown (TrafficCongestion_Unknown) = 0, Light (TrafficCongestion_Light) = 1, Mild (TrafficCongestion_Mild) = 2, Medium (TrafficCongestion_Medium) = 3, Heavy (TrafficCongestion_Heavy) = 4,
}}
RT_ENUM! { enum WaypointKind: i32 {
    Stop (WaypointKind_Stop) = 0, Via (WaypointKind_Via) = 1,
}}
pub mod offlinemaps { // Windows.Services.Maps.OfflineMaps
use ::prelude::*;
DEFINE_IID!(IID_IOfflineMapPackage, 2811717435, 42421, 16708, 181, 37, 230, 140, 136, 98, 102, 75);
RT_INTERFACE!{interface IOfflineMapPackage(IOfflineMapPackageVtbl): IInspectable(IInspectableVtbl) [IID_IOfflineMapPackage] {
    fn get_Status(&self, out: *mut OfflineMapPackageStatus) -> HRESULT,
    fn get_DisplayName(&self, out: *mut HSTRING) -> HRESULT,
    fn get_EnclosingRegionName(&self, out: *mut HSTRING) -> HRESULT,
    fn get_EstimatedSizeInBytes(&self, out: *mut u64) -> HRESULT,
    fn remove_StatusChanged(&self, token: ::rt::gen::windows::foundation::EventRegistrationToken) -> HRESULT,
    fn add_StatusChanged(&self, value: *mut ::rt::gen::windows::foundation::TypedEventHandler<OfflineMapPackage, IInspectable>, out: *mut ::rt::gen::windows::foundation::EventRegistrationToken) -> HRESULT,
    fn RequestStartDownloadAsync(&self, out: *mut *mut ::rt::gen::windows::foundation::IAsyncOperation<OfflineMapPackageStartDownloadResult>) -> HRESULT
}}
impl IOfflineMapPackage {
    #[inline] pub unsafe fn get_status(&self) -> Result<OfflineMapPackageStatus> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_Status)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_display_name(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_DisplayName)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_enclosing_region_name(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_EnclosingRegionName)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_estimated_size_in_bytes(&self) -> Result<u64> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_EstimatedSizeInBytes)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn remove_status_changed(&self, token: ::rt::gen::windows::foundation::EventRegistrationToken) -> Result<()> {
        let hr = ((*self.lpVtbl).remove_StatusChanged)(self as *const _ as *mut _, token);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn add_status_changed(&self, value: &::rt::gen::windows::foundation::TypedEventHandler<OfflineMapPackage, IInspectable>) -> Result<::rt::gen::windows::foundation::EventRegistrationToken> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).add_StatusChanged)(self as *const _ as *mut _, value as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn request_start_download_async(&self) -> Result<ComPtr<::rt::gen::windows::foundation::IAsyncOperation<OfflineMapPackageStartDownloadResult>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).RequestStartDownloadAsync)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
RT_CLASS!{class OfflineMapPackage: IOfflineMapPackage}
impl RtActivatable<IOfflineMapPackageStatics> for OfflineMapPackage {}
impl OfflineMapPackage {
    #[cfg(feature="windows-devices")] #[inline] pub fn find_packages_async(queryPoint: &::rt::gen::windows::devices::geolocation::Geopoint) -> Result<ComPtr<::rt::gen::windows::foundation::IAsyncOperation<OfflineMapPackageQueryResult>>> { unsafe {
        <Self as RtActivatable<IOfflineMapPackageStatics>>::get_activation_factory().find_packages_async(queryPoint)
    }}
    #[cfg(feature="windows-devices")] #[inline] pub fn find_packages_in_bounding_box_async(queryBoundingBox: &::rt::gen::windows::devices::geolocation::GeoboundingBox) -> Result<ComPtr<::rt::gen::windows::foundation::IAsyncOperation<OfflineMapPackageQueryResult>>> { unsafe {
        <Self as RtActivatable<IOfflineMapPackageStatics>>::get_activation_factory().find_packages_in_bounding_box_async(queryBoundingBox)
    }}
    #[cfg(feature="windows-devices")] #[inline] pub fn find_packages_in_geocircle_async(queryCircle: &::rt::gen::windows::devices::geolocation::Geocircle) -> Result<ComPtr<::rt::gen::windows::foundation::IAsyncOperation<OfflineMapPackageQueryResult>>> { unsafe {
        <Self as RtActivatable<IOfflineMapPackageStatics>>::get_activation_factory().find_packages_in_geocircle_async(queryCircle)
    }}
}
DEFINE_CLSID!(OfflineMapPackage(&[87,105,110,100,111,119,115,46,83,101,114,118,105,99,101,115,46,77,97,112,115,46,79,102,102,108,105,110,101,77,97,112,115,46,79,102,102,108,105,110,101,77,97,112,80,97,99,107,97,103,101,0]) [CLSID_OfflineMapPackage]);
DEFINE_IID!(IID_IOfflineMapPackageQueryResult, 1431852049, 14817, 20033, 164, 225, 95, 72, 114, 190, 225, 153);
RT_INTERFACE!{interface IOfflineMapPackageQueryResult(IOfflineMapPackageQueryResultVtbl): IInspectable(IInspectableVtbl) [IID_IOfflineMapPackageQueryResult] {
    fn get_Status(&self, out: *mut OfflineMapPackageQueryStatus) -> HRESULT,
    fn get_Packages(&self, out: *mut *mut ::rt::gen::windows::foundation::collections::IVectorView<OfflineMapPackage>) -> HRESULT
}}
impl IOfflineMapPackageQueryResult {
    #[inline] pub unsafe fn get_status(&self) -> Result<OfflineMapPackageQueryStatus> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_Status)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_packages(&self) -> Result<ComPtr<::rt::gen::windows::foundation::collections::IVectorView<OfflineMapPackage>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Packages)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
RT_CLASS!{class OfflineMapPackageQueryResult: IOfflineMapPackageQueryResult}
RT_ENUM! { enum OfflineMapPackageQueryStatus: i32 {
    Success (OfflineMapPackageQueryStatus_Success) = 0, UnknownError (OfflineMapPackageQueryStatus_UnknownError) = 1, InvalidCredentials (OfflineMapPackageQueryStatus_InvalidCredentials) = 2, NetworkFailure (OfflineMapPackageQueryStatus_NetworkFailure) = 3,
}}
DEFINE_IID!(IID_IOfflineMapPackageStartDownloadResult, 3647322392, 54486, 19198, 147, 120, 62, 199, 30, 241, 28, 61);
RT_INTERFACE!{interface IOfflineMapPackageStartDownloadResult(IOfflineMapPackageStartDownloadResultVtbl): IInspectable(IInspectableVtbl) [IID_IOfflineMapPackageStartDownloadResult] {
    fn get_Status(&self, out: *mut OfflineMapPackageStartDownloadStatus) -> HRESULT
}}
impl IOfflineMapPackageStartDownloadResult {
    #[inline] pub unsafe fn get_status(&self) -> Result<OfflineMapPackageStartDownloadStatus> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_Status)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
}
RT_CLASS!{class OfflineMapPackageStartDownloadResult: IOfflineMapPackageStartDownloadResult}
RT_ENUM! { enum OfflineMapPackageStartDownloadStatus: i32 {
    Success (OfflineMapPackageStartDownloadStatus_Success) = 0, UnknownError (OfflineMapPackageStartDownloadStatus_UnknownError) = 1, InvalidCredentials (OfflineMapPackageStartDownloadStatus_InvalidCredentials) = 2, DeniedWithoutCapability (OfflineMapPackageStartDownloadStatus_DeniedWithoutCapability) = 3,
}}
DEFINE_IID!(IID_IOfflineMapPackageStatics, 408844578, 43057, 19120, 148, 31, 105, 152, 250, 146, 146, 133);
RT_INTERFACE!{static interface IOfflineMapPackageStatics(IOfflineMapPackageStaticsVtbl): IInspectable(IInspectableVtbl) [IID_IOfflineMapPackageStatics] {
    #[cfg(feature="windows-devices")] fn FindPackagesAsync(&self, queryPoint: *mut ::rt::gen::windows::devices::geolocation::Geopoint, out: *mut *mut ::rt::gen::windows::foundation::IAsyncOperation<OfflineMapPackageQueryResult>) -> HRESULT,
    #[cfg(feature="windows-devices")] fn FindPackagesInBoundingBoxAsync(&self, queryBoundingBox: *mut ::rt::gen::windows::devices::geolocation::GeoboundingBox, out: *mut *mut ::rt::gen::windows::foundation::IAsyncOperation<OfflineMapPackageQueryResult>) -> HRESULT,
    #[cfg(feature="windows-devices")] fn FindPackagesInGeocircleAsync(&self, queryCircle: *mut ::rt::gen::windows::devices::geolocation::Geocircle, out: *mut *mut ::rt::gen::windows::foundation::IAsyncOperation<OfflineMapPackageQueryResult>) -> HRESULT
}}
impl IOfflineMapPackageStatics {
    #[cfg(feature="windows-devices")] #[inline] pub unsafe fn find_packages_async(&self, queryPoint: &::rt::gen::windows::devices::geolocation::Geopoint) -> Result<ComPtr<::rt::gen::windows::foundation::IAsyncOperation<OfflineMapPackageQueryResult>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).FindPackagesAsync)(self as *const _ as *mut _, queryPoint as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[cfg(feature="windows-devices")] #[inline] pub unsafe fn find_packages_in_bounding_box_async(&self, queryBoundingBox: &::rt::gen::windows::devices::geolocation::GeoboundingBox) -> Result<ComPtr<::rt::gen::windows::foundation::IAsyncOperation<OfflineMapPackageQueryResult>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).FindPackagesInBoundingBoxAsync)(self as *const _ as *mut _, queryBoundingBox as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[cfg(feature="windows-devices")] #[inline] pub unsafe fn find_packages_in_geocircle_async(&self, queryCircle: &::rt::gen::windows::devices::geolocation::Geocircle) -> Result<ComPtr<::rt::gen::windows::foundation::IAsyncOperation<OfflineMapPackageQueryResult>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).FindPackagesInGeocircleAsync)(self as *const _ as *mut _, queryCircle as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
RT_ENUM! { enum OfflineMapPackageStatus: i32 {
    NotDownloaded (OfflineMapPackageStatus_NotDownloaded) = 0, Downloading (OfflineMapPackageStatus_Downloading) = 1, Downloaded (OfflineMapPackageStatus_Downloaded) = 2, Deleting (OfflineMapPackageStatus_Deleting) = 3,
}}
} // Windows.Services.Maps.OfflineMaps
pub mod localsearch { // Windows.Services.Maps.LocalSearch
use ::prelude::*;
RT_CLASS!{static class LocalCategories}
impl RtActivatable<ILocalCategoriesStatics> for LocalCategories {}
impl LocalCategories {
    #[inline] pub fn get_bank_and_credit_unions() -> Result<HString> { unsafe {
        <Self as RtActivatable<ILocalCategoriesStatics>>::get_activation_factory().get_bank_and_credit_unions()
    }}
    #[inline] pub fn get_eat_drink() -> Result<HString> { unsafe {
        <Self as RtActivatable<ILocalCategoriesStatics>>::get_activation_factory().get_eat_drink()
    }}
    #[inline] pub fn get_hospitals() -> Result<HString> { unsafe {
        <Self as RtActivatable<ILocalCategoriesStatics>>::get_activation_factory().get_hospitals()
    }}
    #[inline] pub fn get_hotels_and_motels() -> Result<HString> { unsafe {
        <Self as RtActivatable<ILocalCategoriesStatics>>::get_activation_factory().get_hotels_and_motels()
    }}
    #[inline] pub fn get_all() -> Result<HString> { unsafe {
        <Self as RtActivatable<ILocalCategoriesStatics>>::get_activation_factory().get_all()
    }}
    #[inline] pub fn get_parking() -> Result<HString> { unsafe {
        <Self as RtActivatable<ILocalCategoriesStatics>>::get_activation_factory().get_parking()
    }}
    #[inline] pub fn get_see_do() -> Result<HString> { unsafe {
        <Self as RtActivatable<ILocalCategoriesStatics>>::get_activation_factory().get_see_do()
    }}
    #[inline] pub fn get_shop() -> Result<HString> { unsafe {
        <Self as RtActivatable<ILocalCategoriesStatics>>::get_activation_factory().get_shop()
    }}
}
DEFINE_CLSID!(LocalCategories(&[87,105,110,100,111,119,115,46,83,101,114,118,105,99,101,115,46,77,97,112,115,46,76,111,99,97,108,83,101,97,114,99,104,46,76,111,99,97,108,67,97,116,101,103,111,114,105,101,115,0]) [CLSID_LocalCategories]);
DEFINE_IID!(IID_ILocalCategoriesStatics, 4103313909, 33377, 17185, 153, 116, 239, 146, 212, 154, 141, 202);
RT_INTERFACE!{static interface ILocalCategoriesStatics(ILocalCategoriesStaticsVtbl): IInspectable(IInspectableVtbl) [IID_ILocalCategoriesStatics] {
    fn get_BankAndCreditUnions(&self, out: *mut HSTRING) -> HRESULT,
    fn get_EatDrink(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Hospitals(&self, out: *mut HSTRING) -> HRESULT,
    fn get_HotelsAndMotels(&self, out: *mut HSTRING) -> HRESULT,
    fn get_All(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Parking(&self, out: *mut HSTRING) -> HRESULT,
    fn get_SeeDo(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Shop(&self, out: *mut HSTRING) -> HRESULT
}}
impl ILocalCategoriesStatics {
    #[inline] pub unsafe fn get_bank_and_credit_unions(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_BankAndCreditUnions)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_eat_drink(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_EatDrink)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_hospitals(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Hospitals)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_hotels_and_motels(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_HotelsAndMotels)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_all(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_All)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_parking(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Parking)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_see_do(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_SeeDo)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_shop(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Shop)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
}
DEFINE_IID!(IID_ILocalLocation, 3138382251, 17666, 20268, 148, 169, 13, 96, 222, 14, 33, 99);
RT_INTERFACE!{interface ILocalLocation(ILocalLocationVtbl): IInspectable(IInspectableVtbl) [IID_ILocalLocation] {
    fn get_Address(&self, out: *mut *mut super::MapAddress) -> HRESULT,
    fn get_Identifier(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Description(&self, out: *mut HSTRING) -> HRESULT,
    fn get_DisplayName(&self, out: *mut HSTRING) -> HRESULT,
    #[cfg(not(feature="windows-devices"))] fn __Dummy4(&self) -> (),
    #[cfg(feature="windows-devices")] fn get_Point(&self, out: *mut *mut ::rt::gen::windows::devices::geolocation::Geopoint) -> HRESULT,
    fn get_PhoneNumber(&self, out: *mut HSTRING) -> HRESULT,
    fn get_DataAttribution(&self, out: *mut HSTRING) -> HRESULT
}}
impl ILocalLocation {
    #[inline] pub unsafe fn get_address(&self) -> Result<ComPtr<super::MapAddress>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Address)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_identifier(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Identifier)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_description(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Description)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_display_name(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_DisplayName)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[cfg(feature="windows-devices")] #[inline] pub unsafe fn get_point(&self) -> Result<ComPtr<::rt::gen::windows::devices::geolocation::Geopoint>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Point)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_phone_number(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_PhoneNumber)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_data_attribution(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_DataAttribution)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
}
RT_CLASS!{class LocalLocation: ILocalLocation}
DEFINE_IID!(IID_ILocalLocation2, 1855860860, 60597, 20476, 187, 140, 186, 80, 186, 140, 45, 198);
RT_INTERFACE!{interface ILocalLocation2(ILocalLocation2Vtbl): IInspectable(IInspectableVtbl) [IID_ILocalLocation2] {
    fn get_Category(&self, out: *mut HSTRING) -> HRESULT,
    fn get_RatingInfo(&self, out: *mut *mut LocalLocationRatingInfo) -> HRESULT,
    fn get_HoursOfOperation(&self, out: *mut *mut ::rt::gen::windows::foundation::collections::IVectorView<LocalLocationHoursOfOperationItem>) -> HRESULT
}}
impl ILocalLocation2 {
    #[inline] pub unsafe fn get_category(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Category)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_rating_info(&self) -> Result<ComPtr<LocalLocationRatingInfo>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_RatingInfo)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_hours_of_operation(&self) -> Result<ComPtr<::rt::gen::windows::foundation::collections::IVectorView<LocalLocationHoursOfOperationItem>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_HoursOfOperation)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
RT_CLASS!{static class LocalLocationFinder}
impl RtActivatable<ILocalLocationFinderStatics> for LocalLocationFinder {}
impl LocalLocationFinder {
    #[cfg(feature="windows-devices")] #[inline] pub fn find_local_locations_async(searchTerm: &HStringArg, searchArea: &::rt::gen::windows::devices::geolocation::Geocircle, localCategory: &HStringArg, maxResults: u32) -> Result<ComPtr<::rt::gen::windows::foundation::IAsyncOperation<LocalLocationFinderResult>>> { unsafe {
        <Self as RtActivatable<ILocalLocationFinderStatics>>::get_activation_factory().find_local_locations_async(searchTerm, searchArea, localCategory, maxResults)
    }}
}
DEFINE_CLSID!(LocalLocationFinder(&[87,105,110,100,111,119,115,46,83,101,114,118,105,99,101,115,46,77,97,112,115,46,76,111,99,97,108,83,101,97,114,99,104,46,76,111,99,97,108,76,111,99,97,116,105,111,110,70,105,110,100,101,114,0]) [CLSID_LocalLocationFinder]);
DEFINE_IID!(IID_ILocalLocationFinderResult, 3499846854, 62264, 16785, 159, 216, 84, 64, 185, 166, 143, 82);
RT_INTERFACE!{interface ILocalLocationFinderResult(ILocalLocationFinderResultVtbl): IInspectable(IInspectableVtbl) [IID_ILocalLocationFinderResult] {
    fn get_LocalLocations(&self, out: *mut *mut ::rt::gen::windows::foundation::collections::IVectorView<LocalLocation>) -> HRESULT,
    fn get_Status(&self, out: *mut LocalLocationFinderStatus) -> HRESULT
}}
impl ILocalLocationFinderResult {
    #[inline] pub unsafe fn get_local_locations(&self) -> Result<ComPtr<::rt::gen::windows::foundation::collections::IVectorView<LocalLocation>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_LocalLocations)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_status(&self) -> Result<LocalLocationFinderStatus> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_Status)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
}
RT_CLASS!{class LocalLocationFinderResult: ILocalLocationFinderResult}
DEFINE_IID!(IID_ILocalLocationFinderStatics, 3538907972, 41182, 18634, 129, 168, 7, 199, 220, 253, 55, 171);
RT_INTERFACE!{static interface ILocalLocationFinderStatics(ILocalLocationFinderStaticsVtbl): IInspectable(IInspectableVtbl) [IID_ILocalLocationFinderStatics] {
    #[cfg(feature="windows-devices")] fn FindLocalLocationsAsync(&self, searchTerm: HSTRING, searchArea: *mut ::rt::gen::windows::devices::geolocation::Geocircle, localCategory: HSTRING, maxResults: u32, out: *mut *mut ::rt::gen::windows::foundation::IAsyncOperation<LocalLocationFinderResult>) -> HRESULT
}}
impl ILocalLocationFinderStatics {
    #[cfg(feature="windows-devices")] #[inline] pub unsafe fn find_local_locations_async(&self, searchTerm: &HStringArg, searchArea: &::rt::gen::windows::devices::geolocation::Geocircle, localCategory: &HStringArg, maxResults: u32) -> Result<ComPtr<::rt::gen::windows::foundation::IAsyncOperation<LocalLocationFinderResult>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).FindLocalLocationsAsync)(self as *const _ as *mut _, searchTerm.get(), searchArea as *const _ as *mut _, localCategory.get(), maxResults, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
RT_ENUM! { enum LocalLocationFinderStatus: i32 {
    Success (LocalLocationFinderStatus_Success) = 0, UnknownError (LocalLocationFinderStatus_UnknownError) = 1, InvalidCredentials (LocalLocationFinderStatus_InvalidCredentials) = 2, InvalidCategory (LocalLocationFinderStatus_InvalidCategory) = 3, InvalidSearchTerm (LocalLocationFinderStatus_InvalidSearchTerm) = 4, InvalidSearchArea (LocalLocationFinderStatus_InvalidSearchArea) = 5, NetworkFailure (LocalLocationFinderStatus_NetworkFailure) = 6, NotSupported (LocalLocationFinderStatus_NotSupported) = 7,
}}
DEFINE_IID!(IID_ILocalLocationHoursOfOperationItem, 592743538, 41415, 17393, 164, 240, 16, 145, 195, 158, 198, 64);
RT_INTERFACE!{interface ILocalLocationHoursOfOperationItem(ILocalLocationHoursOfOperationItemVtbl): IInspectable(IInspectableVtbl) [IID_ILocalLocationHoursOfOperationItem] {
    #[cfg(not(feature="windows-globalization"))] fn __Dummy0(&self) -> (),
    #[cfg(feature="windows-globalization")] fn get_Day(&self, out: *mut ::rt::gen::windows::globalization::DayOfWeek) -> HRESULT,
    fn get_Start(&self, out: *mut ::rt::gen::windows::foundation::TimeSpan) -> HRESULT,
    fn get_Span(&self, out: *mut ::rt::gen::windows::foundation::TimeSpan) -> HRESULT
}}
impl ILocalLocationHoursOfOperationItem {
    #[cfg(feature="windows-globalization")] #[inline] pub unsafe fn get_day(&self) -> Result<::rt::gen::windows::globalization::DayOfWeek> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_Day)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_start(&self) -> Result<::rt::gen::windows::foundation::TimeSpan> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_Start)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_span(&self) -> Result<::rt::gen::windows::foundation::TimeSpan> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_Span)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
}
RT_CLASS!{class LocalLocationHoursOfOperationItem: ILocalLocationHoursOfOperationItem}
DEFINE_IID!(IID_ILocalLocationRatingInfo, 3407719254, 13140, 17169, 139, 192, 162, 212, 213, 235, 128, 110);
RT_INTERFACE!{interface ILocalLocationRatingInfo(ILocalLocationRatingInfoVtbl): IInspectable(IInspectableVtbl) [IID_ILocalLocationRatingInfo] {
    fn get_AggregateRating(&self, out: *mut *mut ::rt::gen::windows::foundation::IReference<f64>) -> HRESULT,
    fn get_RatingCount(&self, out: *mut *mut ::rt::gen::windows::foundation::IReference<i32>) -> HRESULT,
    fn get_ProviderIdentifier(&self, out: *mut HSTRING) -> HRESULT
}}
impl ILocalLocationRatingInfo {
    #[inline] pub unsafe fn get_aggregate_rating(&self) -> Result<ComPtr<::rt::gen::windows::foundation::IReference<f64>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_AggregateRating)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_rating_count(&self) -> Result<ComPtr<::rt::gen::windows::foundation::IReference<i32>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_RatingCount)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_provider_identifier(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_ProviderIdentifier)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
}
RT_CLASS!{class LocalLocationRatingInfo: ILocalLocationRatingInfo}
RT_CLASS!{static class PlaceInfoHelper}
impl RtActivatable<IPlaceInfoHelperStatics> for PlaceInfoHelper {}
impl PlaceInfoHelper {
    #[inline] pub fn create_from_local_location(location: &LocalLocation) -> Result<ComPtr<super::PlaceInfo>> { unsafe {
        <Self as RtActivatable<IPlaceInfoHelperStatics>>::get_activation_factory().create_from_local_location(location)
    }}
}
DEFINE_CLSID!(PlaceInfoHelper(&[87,105,110,100,111,119,115,46,83,101,114,118,105,99,101,115,46,77,97,112,115,46,76,111,99,97,108,83,101,97,114,99,104,46,80,108,97,99,101,73,110,102,111,72,101,108,112,101,114,0]) [CLSID_PlaceInfoHelper]);
DEFINE_IID!(IID_IPlaceInfoHelperStatics, 3709643175, 43462, 18715, 188, 9, 232, 15, 206, 164, 142, 230);
RT_INTERFACE!{static interface IPlaceInfoHelperStatics(IPlaceInfoHelperStaticsVtbl): IInspectable(IInspectableVtbl) [IID_IPlaceInfoHelperStatics] {
    fn CreateFromLocalLocation(&self, location: *mut LocalLocation, out: *mut *mut super::PlaceInfo) -> HRESULT
}}
impl IPlaceInfoHelperStatics {
    #[inline] pub unsafe fn create_from_local_location(&self, location: &LocalLocation) -> Result<ComPtr<super::PlaceInfo>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).CreateFromLocalLocation)(self as *const _ as *mut _, location as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
} // Windows.Services.Maps.LocalSearch
pub mod guidance { // Windows.Services.Maps.Guidance
use ::prelude::*;
RT_ENUM! { enum GuidanceAudioMeasurementSystem: i32 {
    Meters (GuidanceAudioMeasurementSystem_Meters) = 0, MilesAndYards (GuidanceAudioMeasurementSystem_MilesAndYards) = 1, MilesAndFeet (GuidanceAudioMeasurementSystem_MilesAndFeet) = 2,
}}
RT_ENUM! { enum GuidanceAudioNotificationKind: i32 {
    Maneuver (GuidanceAudioNotificationKind_Maneuver) = 0, Route (GuidanceAudioNotificationKind_Route) = 1, Gps (GuidanceAudioNotificationKind_Gps) = 2, SpeedLimit (GuidanceAudioNotificationKind_SpeedLimit) = 3, Traffic (GuidanceAudioNotificationKind_Traffic) = 4, TrafficCamera (GuidanceAudioNotificationKind_TrafficCamera) = 5,
}}
DEFINE_IID!(IID_IGuidanceAudioNotificationRequestedEventArgs, 3391791690, 51138, 19788, 157, 124, 73, 149, 118, 188, 237, 219);
RT_INTERFACE!{interface IGuidanceAudioNotificationRequestedEventArgs(IGuidanceAudioNotificationRequestedEventArgsVtbl): IInspectable(IInspectableVtbl) [IID_IGuidanceAudioNotificationRequestedEventArgs] {
    fn get_AudioNotification(&self, out: *mut GuidanceAudioNotificationKind) -> HRESULT,
    fn get_AudioFilePaths(&self, out: *mut *mut ::rt::gen::windows::foundation::collections::IVectorView<HString>) -> HRESULT,
    fn get_AudioText(&self, out: *mut HSTRING) -> HRESULT
}}
impl IGuidanceAudioNotificationRequestedEventArgs {
    #[inline] pub unsafe fn get_audio_notification(&self) -> Result<GuidanceAudioNotificationKind> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_AudioNotification)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_audio_file_paths(&self) -> Result<ComPtr<::rt::gen::windows::foundation::collections::IVectorView<HString>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_AudioFilePaths)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_audio_text(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_AudioText)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
}
RT_CLASS!{class GuidanceAudioNotificationRequestedEventArgs: IGuidanceAudioNotificationRequestedEventArgs}
RT_ENUM! { enum GuidanceAudioNotifications: u32 {
    None (GuidanceAudioNotifications_None) = 0, Maneuver (GuidanceAudioNotifications_Maneuver) = 1, Route (GuidanceAudioNotifications_Route) = 2, Gps (GuidanceAudioNotifications_Gps) = 4, SpeedLimit (GuidanceAudioNotifications_SpeedLimit) = 8, Traffic (GuidanceAudioNotifications_Traffic) = 16, TrafficCamera (GuidanceAudioNotifications_TrafficCamera) = 32,
}}
DEFINE_IID!(IID_IGuidanceLaneInfo, 2214908180, 25985, 17335, 172, 21, 201, 7, 155, 249, 13, 241);
RT_INTERFACE!{interface IGuidanceLaneInfo(IGuidanceLaneInfoVtbl): IInspectable(IInspectableVtbl) [IID_IGuidanceLaneInfo] {
    fn get_LaneMarkers(&self, out: *mut GuidanceLaneMarkers) -> HRESULT,
    fn get_IsOnRoute(&self, out: *mut bool) -> HRESULT
}}
impl IGuidanceLaneInfo {
    #[inline] pub unsafe fn get_lane_markers(&self) -> Result<GuidanceLaneMarkers> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_LaneMarkers)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_is_on_route(&self) -> Result<bool> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_IsOnRoute)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
}
RT_CLASS!{class GuidanceLaneInfo: IGuidanceLaneInfo}
RT_ENUM! { enum GuidanceLaneMarkers: u32 {
    None (GuidanceLaneMarkers_None) = 0, LightRight (GuidanceLaneMarkers_LightRight) = 1, Right (GuidanceLaneMarkers_Right) = 2, HardRight (GuidanceLaneMarkers_HardRight) = 4, Straight (GuidanceLaneMarkers_Straight) = 8, UTurnLeft (GuidanceLaneMarkers_UTurnLeft) = 16, HardLeft (GuidanceLaneMarkers_HardLeft) = 32, Left (GuidanceLaneMarkers_Left) = 64, LightLeft (GuidanceLaneMarkers_LightLeft) = 128, UTurnRight (GuidanceLaneMarkers_UTurnRight) = 256, Unknown (GuidanceLaneMarkers_Unknown) = 4294967295,
}}
DEFINE_IID!(IID_IGuidanceManeuver, 4228461164, 60617, 18728, 162, 161, 114, 50, 185, 155, 148, 161);
RT_INTERFACE!{interface IGuidanceManeuver(IGuidanceManeuverVtbl): IInspectable(IInspectableVtbl) [IID_IGuidanceManeuver] {
    #[cfg(not(feature="windows-devices"))] fn __Dummy0(&self) -> (),
    #[cfg(feature="windows-devices")] fn get_StartLocation(&self, out: *mut *mut ::rt::gen::windows::devices::geolocation::Geopoint) -> HRESULT,
    fn get_DistanceFromRouteStart(&self, out: *mut i32) -> HRESULT,
    fn get_DistanceFromPreviousManeuver(&self, out: *mut i32) -> HRESULT,
    fn get_DepartureRoadName(&self, out: *mut HSTRING) -> HRESULT,
    fn get_NextRoadName(&self, out: *mut HSTRING) -> HRESULT,
    fn get_DepartureShortRoadName(&self, out: *mut HSTRING) -> HRESULT,
    fn get_NextShortRoadName(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Kind(&self, out: *mut GuidanceManeuverKind) -> HRESULT,
    fn get_StartAngle(&self, out: *mut i32) -> HRESULT,
    fn get_EndAngle(&self, out: *mut i32) -> HRESULT,
    fn get_RoadSignpost(&self, out: *mut *mut GuidanceRoadSignpost) -> HRESULT,
    fn get_InstructionText(&self, out: *mut HSTRING) -> HRESULT
}}
impl IGuidanceManeuver {
    #[cfg(feature="windows-devices")] #[inline] pub unsafe fn get_start_location(&self) -> Result<ComPtr<::rt::gen::windows::devices::geolocation::Geopoint>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_StartLocation)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_distance_from_route_start(&self) -> Result<i32> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_DistanceFromRouteStart)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_distance_from_previous_maneuver(&self) -> Result<i32> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_DistanceFromPreviousManeuver)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_departure_road_name(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_DepartureRoadName)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_next_road_name(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_NextRoadName)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_departure_short_road_name(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_DepartureShortRoadName)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_next_short_road_name(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_NextShortRoadName)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_kind(&self) -> Result<GuidanceManeuverKind> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_Kind)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_start_angle(&self) -> Result<i32> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_StartAngle)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_end_angle(&self) -> Result<i32> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_EndAngle)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_road_signpost(&self) -> Result<ComPtr<GuidanceRoadSignpost>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_RoadSignpost)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_instruction_text(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_InstructionText)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
}
RT_CLASS!{class GuidanceManeuver: IGuidanceManeuver}
RT_ENUM! { enum GuidanceManeuverKind: i32 {
    None (GuidanceManeuverKind_None) = 0, GoStraight (GuidanceManeuverKind_GoStraight) = 1, UTurnRight (GuidanceManeuverKind_UTurnRight) = 2, UTurnLeft (GuidanceManeuverKind_UTurnLeft) = 3, TurnKeepRight (GuidanceManeuverKind_TurnKeepRight) = 4, TurnLightRight (GuidanceManeuverKind_TurnLightRight) = 5, TurnRight (GuidanceManeuverKind_TurnRight) = 6, TurnHardRight (GuidanceManeuverKind_TurnHardRight) = 7, KeepMiddle (GuidanceManeuverKind_KeepMiddle) = 8, TurnKeepLeft (GuidanceManeuverKind_TurnKeepLeft) = 9, TurnLightLeft (GuidanceManeuverKind_TurnLightLeft) = 10, TurnLeft (GuidanceManeuverKind_TurnLeft) = 11, TurnHardLeft (GuidanceManeuverKind_TurnHardLeft) = 12, FreewayEnterRight (GuidanceManeuverKind_FreewayEnterRight) = 13, FreewayEnterLeft (GuidanceManeuverKind_FreewayEnterLeft) = 14, FreewayLeaveRight (GuidanceManeuverKind_FreewayLeaveRight) = 15, FreewayLeaveLeft (GuidanceManeuverKind_FreewayLeaveLeft) = 16, FreewayKeepRight (GuidanceManeuverKind_FreewayKeepRight) = 17, FreewayKeepLeft (GuidanceManeuverKind_FreewayKeepLeft) = 18, TrafficCircleRight1 (GuidanceManeuverKind_TrafficCircleRight1) = 19, TrafficCircleRight2 (GuidanceManeuverKind_TrafficCircleRight2) = 20, TrafficCircleRight3 (GuidanceManeuverKind_TrafficCircleRight3) = 21, TrafficCircleRight4 (GuidanceManeuverKind_TrafficCircleRight4) = 22, TrafficCircleRight5 (GuidanceManeuverKind_TrafficCircleRight5) = 23, TrafficCircleRight6 (GuidanceManeuverKind_TrafficCircleRight6) = 24, TrafficCircleRight7 (GuidanceManeuverKind_TrafficCircleRight7) = 25, TrafficCircleRight8 (GuidanceManeuverKind_TrafficCircleRight8) = 26, TrafficCircleRight9 (GuidanceManeuverKind_TrafficCircleRight9) = 27, TrafficCircleRight10 (GuidanceManeuverKind_TrafficCircleRight10) = 28, TrafficCircleRight11 (GuidanceManeuverKind_TrafficCircleRight11) = 29, TrafficCircleRight12 (GuidanceManeuverKind_TrafficCircleRight12) = 30, TrafficCircleLeft1 (GuidanceManeuverKind_TrafficCircleLeft1) = 31, TrafficCircleLeft2 (GuidanceManeuverKind_TrafficCircleLeft2) = 32, TrafficCircleLeft3 (GuidanceManeuverKind_TrafficCircleLeft3) = 33, TrafficCircleLeft4 (GuidanceManeuverKind_TrafficCircleLeft4) = 34, TrafficCircleLeft5 (GuidanceManeuverKind_TrafficCircleLeft5) = 35, TrafficCircleLeft6 (GuidanceManeuverKind_TrafficCircleLeft6) = 36, TrafficCircleLeft7 (GuidanceManeuverKind_TrafficCircleLeft7) = 37, TrafficCircleLeft8 (GuidanceManeuverKind_TrafficCircleLeft8) = 38, TrafficCircleLeft9 (GuidanceManeuverKind_TrafficCircleLeft9) = 39, TrafficCircleLeft10 (GuidanceManeuverKind_TrafficCircleLeft10) = 40, TrafficCircleLeft11 (GuidanceManeuverKind_TrafficCircleLeft11) = 41, TrafficCircleLeft12 (GuidanceManeuverKind_TrafficCircleLeft12) = 42, Start (GuidanceManeuverKind_Start) = 43, End (GuidanceManeuverKind_End) = 44, TakeFerry (GuidanceManeuverKind_TakeFerry) = 45, PassTransitStation (GuidanceManeuverKind_PassTransitStation) = 46, LeaveTransitStation (GuidanceManeuverKind_LeaveTransitStation) = 47,
}}
DEFINE_IID!(IID_IGuidanceMapMatchedCoordinate, 3081548136, 10514, 19097, 175, 241, 121, 134, 9, 185, 129, 254);
RT_INTERFACE!{interface IGuidanceMapMatchedCoordinate(IGuidanceMapMatchedCoordinateVtbl): IInspectable(IInspectableVtbl) [IID_IGuidanceMapMatchedCoordinate] {
    #[cfg(not(feature="windows-devices"))] fn __Dummy0(&self) -> (),
    #[cfg(feature="windows-devices")] fn get_Location(&self, out: *mut *mut ::rt::gen::windows::devices::geolocation::Geopoint) -> HRESULT,
    fn get_CurrentHeading(&self, out: *mut f64) -> HRESULT,
    fn get_CurrentSpeed(&self, out: *mut f64) -> HRESULT,
    fn get_IsOnStreet(&self, out: *mut bool) -> HRESULT,
    fn get_Road(&self, out: *mut *mut GuidanceRoadSegment) -> HRESULT
}}
impl IGuidanceMapMatchedCoordinate {
    #[cfg(feature="windows-devices")] #[inline] pub unsafe fn get_location(&self) -> Result<ComPtr<::rt::gen::windows::devices::geolocation::Geopoint>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Location)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_current_heading(&self) -> Result<f64> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_CurrentHeading)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_current_speed(&self) -> Result<f64> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_CurrentSpeed)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_is_on_street(&self) -> Result<bool> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_IsOnStreet)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_road(&self) -> Result<ComPtr<GuidanceRoadSegment>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Road)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
RT_CLASS!{class GuidanceMapMatchedCoordinate: IGuidanceMapMatchedCoordinate}
RT_ENUM! { enum GuidanceMode: i32 {
    None (GuidanceMode_None) = 0, Simulation (GuidanceMode_Simulation) = 1, Navigation (GuidanceMode_Navigation) = 2, Tracking (GuidanceMode_Tracking) = 3,
}}
DEFINE_IID!(IID_IGuidanceNavigator, 150044407, 36415, 19866, 190, 138, 16, 143, 154, 1, 44, 103);
RT_INTERFACE!{interface IGuidanceNavigator(IGuidanceNavigatorVtbl): IInspectable(IInspectableVtbl) [IID_IGuidanceNavigator] {
    fn StartNavigating(&self, route: *mut GuidanceRoute) -> HRESULT,
    fn StartSimulating(&self, route: *mut GuidanceRoute, speedInMetersPerSecond: i32) -> HRESULT,
    fn StartTracking(&self) -> HRESULT,
    fn Pause(&self) -> HRESULT,
    fn Resume(&self) -> HRESULT,
    fn Stop(&self) -> HRESULT,
    fn RepeatLastAudioNotification(&self) -> HRESULT,
    fn get_AudioMeasurementSystem(&self, out: *mut GuidanceAudioMeasurementSystem) -> HRESULT,
    fn put_AudioMeasurementSystem(&self, value: GuidanceAudioMeasurementSystem) -> HRESULT,
    fn get_AudioNotifications(&self, out: *mut GuidanceAudioNotifications) -> HRESULT,
    fn put_AudioNotifications(&self, value: GuidanceAudioNotifications) -> HRESULT,
    fn add_GuidanceUpdated(&self, handler: *mut ::rt::gen::windows::foundation::TypedEventHandler<GuidanceNavigator, GuidanceUpdatedEventArgs>, out: *mut ::rt::gen::windows::foundation::EventRegistrationToken) -> HRESULT,
    fn remove_GuidanceUpdated(&self, token: ::rt::gen::windows::foundation::EventRegistrationToken) -> HRESULT,
    fn add_DestinationReached(&self, handler: *mut ::rt::gen::windows::foundation::TypedEventHandler<GuidanceNavigator, IInspectable>, out: *mut ::rt::gen::windows::foundation::EventRegistrationToken) -> HRESULT,
    fn remove_DestinationReached(&self, token: ::rt::gen::windows::foundation::EventRegistrationToken) -> HRESULT,
    fn add_Rerouting(&self, handler: *mut ::rt::gen::windows::foundation::TypedEventHandler<GuidanceNavigator, IInspectable>, out: *mut ::rt::gen::windows::foundation::EventRegistrationToken) -> HRESULT,
    fn remove_Rerouting(&self, token: ::rt::gen::windows::foundation::EventRegistrationToken) -> HRESULT,
    fn add_Rerouted(&self, handler: *mut ::rt::gen::windows::foundation::TypedEventHandler<GuidanceNavigator, GuidanceReroutedEventArgs>, out: *mut ::rt::gen::windows::foundation::EventRegistrationToken) -> HRESULT,
    fn remove_Rerouted(&self, token: ::rt::gen::windows::foundation::EventRegistrationToken) -> HRESULT,
    fn add_RerouteFailed(&self, handler: *mut ::rt::gen::windows::foundation::TypedEventHandler<GuidanceNavigator, IInspectable>, out: *mut ::rt::gen::windows::foundation::EventRegistrationToken) -> HRESULT,
    fn remove_RerouteFailed(&self, token: ::rt::gen::windows::foundation::EventRegistrationToken) -> HRESULT,
    fn add_UserLocationLost(&self, handler: *mut ::rt::gen::windows::foundation::TypedEventHandler<GuidanceNavigator, IInspectable>, out: *mut ::rt::gen::windows::foundation::EventRegistrationToken) -> HRESULT,
    fn remove_UserLocationLost(&self, token: ::rt::gen::windows::foundation::EventRegistrationToken) -> HRESULT,
    fn add_UserLocationRestored(&self, handler: *mut ::rt::gen::windows::foundation::TypedEventHandler<GuidanceNavigator, IInspectable>, out: *mut ::rt::gen::windows::foundation::EventRegistrationToken) -> HRESULT,
    fn remove_UserLocationRestored(&self, token: ::rt::gen::windows::foundation::EventRegistrationToken) -> HRESULT,
    fn SetGuidanceVoice(&self, voiceId: i32, voiceFolder: HSTRING) -> HRESULT,
    #[cfg(feature="windows-devices")] fn UpdateUserLocation(&self, userLocation: *mut ::rt::gen::windows::devices::geolocation::Geocoordinate) -> HRESULT,
    #[cfg(feature="windows-devices")] fn UpdateUserLocationWithPositionOverride(&self, userLocation: *mut ::rt::gen::windows::devices::geolocation::Geocoordinate, positionOverride: ::rt::gen::windows::devices::geolocation::BasicGeoposition) -> HRESULT
}}
impl IGuidanceNavigator {
    #[inline] pub unsafe fn start_navigating(&self, route: &GuidanceRoute) -> Result<()> {
        let hr = ((*self.lpVtbl).StartNavigating)(self as *const _ as *mut _, route as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn start_simulating(&self, route: &GuidanceRoute, speedInMetersPerSecond: i32) -> Result<()> {
        let hr = ((*self.lpVtbl).StartSimulating)(self as *const _ as *mut _, route as *const _ as *mut _, speedInMetersPerSecond);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn start_tracking(&self) -> Result<()> {
        let hr = ((*self.lpVtbl).StartTracking)(self as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn pause(&self) -> Result<()> {
        let hr = ((*self.lpVtbl).Pause)(self as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn resume(&self) -> Result<()> {
        let hr = ((*self.lpVtbl).Resume)(self as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn stop(&self) -> Result<()> {
        let hr = ((*self.lpVtbl).Stop)(self as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn repeat_last_audio_notification(&self) -> Result<()> {
        let hr = ((*self.lpVtbl).RepeatLastAudioNotification)(self as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_audio_measurement_system(&self) -> Result<GuidanceAudioMeasurementSystem> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_AudioMeasurementSystem)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_audio_measurement_system(&self, value: GuidanceAudioMeasurementSystem) -> Result<()> {
        let hr = ((*self.lpVtbl).put_AudioMeasurementSystem)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_audio_notifications(&self) -> Result<GuidanceAudioNotifications> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_AudioNotifications)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_audio_notifications(&self, value: GuidanceAudioNotifications) -> Result<()> {
        let hr = ((*self.lpVtbl).put_AudioNotifications)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn add_guidance_updated(&self, handler: &::rt::gen::windows::foundation::TypedEventHandler<GuidanceNavigator, GuidanceUpdatedEventArgs>) -> Result<::rt::gen::windows::foundation::EventRegistrationToken> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).add_GuidanceUpdated)(self as *const _ as *mut _, handler as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn remove_guidance_updated(&self, token: ::rt::gen::windows::foundation::EventRegistrationToken) -> Result<()> {
        let hr = ((*self.lpVtbl).remove_GuidanceUpdated)(self as *const _ as *mut _, token);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn add_destination_reached(&self, handler: &::rt::gen::windows::foundation::TypedEventHandler<GuidanceNavigator, IInspectable>) -> Result<::rt::gen::windows::foundation::EventRegistrationToken> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).add_DestinationReached)(self as *const _ as *mut _, handler as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn remove_destination_reached(&self, token: ::rt::gen::windows::foundation::EventRegistrationToken) -> Result<()> {
        let hr = ((*self.lpVtbl).remove_DestinationReached)(self as *const _ as *mut _, token);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn add_rerouting(&self, handler: &::rt::gen::windows::foundation::TypedEventHandler<GuidanceNavigator, IInspectable>) -> Result<::rt::gen::windows::foundation::EventRegistrationToken> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).add_Rerouting)(self as *const _ as *mut _, handler as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn remove_rerouting(&self, token: ::rt::gen::windows::foundation::EventRegistrationToken) -> Result<()> {
        let hr = ((*self.lpVtbl).remove_Rerouting)(self as *const _ as *mut _, token);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn add_rerouted(&self, handler: &::rt::gen::windows::foundation::TypedEventHandler<GuidanceNavigator, GuidanceReroutedEventArgs>) -> Result<::rt::gen::windows::foundation::EventRegistrationToken> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).add_Rerouted)(self as *const _ as *mut _, handler as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn remove_rerouted(&self, token: ::rt::gen::windows::foundation::EventRegistrationToken) -> Result<()> {
        let hr = ((*self.lpVtbl).remove_Rerouted)(self as *const _ as *mut _, token);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn add_reroute_failed(&self, handler: &::rt::gen::windows::foundation::TypedEventHandler<GuidanceNavigator, IInspectable>) -> Result<::rt::gen::windows::foundation::EventRegistrationToken> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).add_RerouteFailed)(self as *const _ as *mut _, handler as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn remove_reroute_failed(&self, token: ::rt::gen::windows::foundation::EventRegistrationToken) -> Result<()> {
        let hr = ((*self.lpVtbl).remove_RerouteFailed)(self as *const _ as *mut _, token);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn add_user_location_lost(&self, handler: &::rt::gen::windows::foundation::TypedEventHandler<GuidanceNavigator, IInspectable>) -> Result<::rt::gen::windows::foundation::EventRegistrationToken> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).add_UserLocationLost)(self as *const _ as *mut _, handler as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn remove_user_location_lost(&self, token: ::rt::gen::windows::foundation::EventRegistrationToken) -> Result<()> {
        let hr = ((*self.lpVtbl).remove_UserLocationLost)(self as *const _ as *mut _, token);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn add_user_location_restored(&self, handler: &::rt::gen::windows::foundation::TypedEventHandler<GuidanceNavigator, IInspectable>) -> Result<::rt::gen::windows::foundation::EventRegistrationToken> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).add_UserLocationRestored)(self as *const _ as *mut _, handler as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn remove_user_location_restored(&self, token: ::rt::gen::windows::foundation::EventRegistrationToken) -> Result<()> {
        let hr = ((*self.lpVtbl).remove_UserLocationRestored)(self as *const _ as *mut _, token);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_guidance_voice(&self, voiceId: i32, voiceFolder: &HStringArg) -> Result<()> {
        let hr = ((*self.lpVtbl).SetGuidanceVoice)(self as *const _ as *mut _, voiceId, voiceFolder.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[cfg(feature="windows-devices")] #[inline] pub unsafe fn update_user_location(&self, userLocation: &::rt::gen::windows::devices::geolocation::Geocoordinate) -> Result<()> {
        let hr = ((*self.lpVtbl).UpdateUserLocation)(self as *const _ as *mut _, userLocation as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[cfg(feature="windows-devices")] #[inline] pub unsafe fn update_user_location_with_position_override(&self, userLocation: &::rt::gen::windows::devices::geolocation::Geocoordinate, positionOverride: ::rt::gen::windows::devices::geolocation::BasicGeoposition) -> Result<()> {
        let hr = ((*self.lpVtbl).UpdateUserLocationWithPositionOverride)(self as *const _ as *mut _, userLocation as *const _ as *mut _, positionOverride);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
}
RT_CLASS!{class GuidanceNavigator: IGuidanceNavigator}
impl RtActivatable<IGuidanceNavigatorStatics> for GuidanceNavigator {}
impl RtActivatable<IGuidanceNavigatorStatics2> for GuidanceNavigator {}
impl GuidanceNavigator {
    #[inline] pub fn get_current() -> Result<ComPtr<GuidanceNavigator>> { unsafe {
        <Self as RtActivatable<IGuidanceNavigatorStatics>>::get_activation_factory().get_current()
    }}
    #[inline] pub fn get_use_app_provided_voice() -> Result<bool> { unsafe {
        <Self as RtActivatable<IGuidanceNavigatorStatics2>>::get_activation_factory().get_use_app_provided_voice()
    }}
}
DEFINE_CLSID!(GuidanceNavigator(&[87,105,110,100,111,119,115,46,83,101,114,118,105,99,101,115,46,77,97,112,115,46,71,117,105,100,97,110,99,101,46,71,117,105,100,97,110,99,101,78,97,118,105,103,97,116,111,114,0]) [CLSID_GuidanceNavigator]);
DEFINE_IID!(IID_IGuidanceNavigator2, 1826377937, 1052, 19443, 182, 51, 161, 1, 252, 47, 107, 87);
RT_INTERFACE!{interface IGuidanceNavigator2(IGuidanceNavigator2Vtbl): IInspectable(IInspectableVtbl) [IID_IGuidanceNavigator2] {
    fn add_AudioNotificationRequested(&self, value: *mut ::rt::gen::windows::foundation::TypedEventHandler<GuidanceNavigator, GuidanceAudioNotificationRequestedEventArgs>, out: *mut ::rt::gen::windows::foundation::EventRegistrationToken) -> HRESULT,
    fn remove_AudioNotificationRequested(&self, token: ::rt::gen::windows::foundation::EventRegistrationToken) -> HRESULT,
    fn get_IsGuidanceAudioMuted(&self, out: *mut bool) -> HRESULT,
    fn put_IsGuidanceAudioMuted(&self, value: bool) -> HRESULT
}}
impl IGuidanceNavigator2 {
    #[inline] pub unsafe fn add_audio_notification_requested(&self, value: &::rt::gen::windows::foundation::TypedEventHandler<GuidanceNavigator, GuidanceAudioNotificationRequestedEventArgs>) -> Result<::rt::gen::windows::foundation::EventRegistrationToken> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).add_AudioNotificationRequested)(self as *const _ as *mut _, value as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn remove_audio_notification_requested(&self, token: ::rt::gen::windows::foundation::EventRegistrationToken) -> Result<()> {
        let hr = ((*self.lpVtbl).remove_AudioNotificationRequested)(self as *const _ as *mut _, token);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_is_guidance_audio_muted(&self) -> Result<bool> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_IsGuidanceAudioMuted)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_is_guidance_audio_muted(&self, value: bool) -> Result<()> {
        let hr = ((*self.lpVtbl).put_IsGuidanceAudioMuted)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
}
DEFINE_IID!(IID_IGuidanceNavigatorStatics, 16618771, 17494, 20070, 161, 67, 58, 221, 107, 224, 132, 38);
RT_INTERFACE!{static interface IGuidanceNavigatorStatics(IGuidanceNavigatorStaticsVtbl): IInspectable(IInspectableVtbl) [IID_IGuidanceNavigatorStatics] {
    fn GetCurrent(&self, out: *mut *mut GuidanceNavigator) -> HRESULT
}}
impl IGuidanceNavigatorStatics {
    #[inline] pub unsafe fn get_current(&self) -> Result<ComPtr<GuidanceNavigator>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetCurrent)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
DEFINE_IID!(IID_IGuidanceNavigatorStatics2, 1422246882, 30596, 19589, 140, 149, 208, 198, 239, 180, 57, 101);
RT_INTERFACE!{static interface IGuidanceNavigatorStatics2(IGuidanceNavigatorStatics2Vtbl): IInspectable(IInspectableVtbl) [IID_IGuidanceNavigatorStatics2] {
    fn get_UseAppProvidedVoice(&self, out: *mut bool) -> HRESULT
}}
impl IGuidanceNavigatorStatics2 {
    #[inline] pub unsafe fn get_use_app_provided_voice(&self) -> Result<bool> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_UseAppProvidedVoice)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
}
DEFINE_IID!(IID_IGuidanceReroutedEventArgs, 291323912, 54568, 17742, 187, 148, 165, 3, 65, 210, 201, 241);
RT_INTERFACE!{interface IGuidanceReroutedEventArgs(IGuidanceReroutedEventArgsVtbl): IInspectable(IInspectableVtbl) [IID_IGuidanceReroutedEventArgs] {
    fn get_Route(&self, out: *mut *mut GuidanceRoute) -> HRESULT
}}
impl IGuidanceReroutedEventArgs {
    #[inline] pub unsafe fn get_route(&self) -> Result<ComPtr<GuidanceRoute>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Route)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
RT_CLASS!{class GuidanceReroutedEventArgs: IGuidanceReroutedEventArgs}
DEFINE_IID!(IID_IGuidanceRoadSegment, 3005700262, 48760, 19555, 175, 231, 108, 41, 87, 71, 155, 62);
RT_INTERFACE!{interface IGuidanceRoadSegment(IGuidanceRoadSegmentVtbl): IInspectable(IInspectableVtbl) [IID_IGuidanceRoadSegment] {
    fn get_RoadName(&self, out: *mut HSTRING) -> HRESULT,
    fn get_ShortRoadName(&self, out: *mut HSTRING) -> HRESULT,
    fn get_SpeedLimit(&self, out: *mut f64) -> HRESULT,
    fn get_TravelTime(&self, out: *mut ::rt::gen::windows::foundation::TimeSpan) -> HRESULT,
    #[cfg(not(feature="windows-devices"))] fn __Dummy4(&self) -> (),
    #[cfg(feature="windows-devices")] fn get_Path(&self, out: *mut *mut ::rt::gen::windows::devices::geolocation::Geopath) -> HRESULT,
    fn get_Id(&self, out: *mut HSTRING) -> HRESULT,
    fn get_IsHighway(&self, out: *mut bool) -> HRESULT,
    fn get_IsTunnel(&self, out: *mut bool) -> HRESULT,
    fn get_IsTollRoad(&self, out: *mut bool) -> HRESULT
}}
impl IGuidanceRoadSegment {
    #[inline] pub unsafe fn get_road_name(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_RoadName)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_short_road_name(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_ShortRoadName)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_speed_limit(&self) -> Result<f64> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_SpeedLimit)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_travel_time(&self) -> Result<::rt::gen::windows::foundation::TimeSpan> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_TravelTime)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[cfg(feature="windows-devices")] #[inline] pub unsafe fn get_path(&self) -> Result<ComPtr<::rt::gen::windows::devices::geolocation::Geopath>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Path)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_id(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Id)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_is_highway(&self) -> Result<bool> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_IsHighway)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_is_tunnel(&self) -> Result<bool> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_IsTunnel)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_is_toll_road(&self) -> Result<bool> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_IsTollRoad)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
}
RT_CLASS!{class GuidanceRoadSegment: IGuidanceRoadSegment}
DEFINE_IID!(IID_IGuidanceRoadSegment2, 611624477, 5923, 18929, 137, 91, 71, 162, 196, 170, 156, 85);
RT_INTERFACE!{interface IGuidanceRoadSegment2(IGuidanceRoadSegment2Vtbl): IInspectable(IInspectableVtbl) [IID_IGuidanceRoadSegment2] {
    fn get_IsScenic(&self, out: *mut bool) -> HRESULT
}}
impl IGuidanceRoadSegment2 {
    #[inline] pub unsafe fn get_is_scenic(&self) -> Result<bool> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_IsScenic)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
}
DEFINE_IID!(IID_IGuidanceRoadSignpost, 4054263990, 63354, 18242, 131, 18, 83, 48, 15, 152, 69, 240);
RT_INTERFACE!{interface IGuidanceRoadSignpost(IGuidanceRoadSignpostVtbl): IInspectable(IInspectableVtbl) [IID_IGuidanceRoadSignpost] {
    fn get_ExitNumber(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Exit(&self, out: *mut HSTRING) -> HRESULT,
    #[cfg(not(feature="windows-ui"))] fn __Dummy2(&self) -> (),
    #[cfg(feature="windows-ui")] fn get_BackgroundColor(&self, out: *mut ::rt::gen::windows::ui::Color) -> HRESULT,
    #[cfg(not(feature="windows-ui"))] fn __Dummy3(&self) -> (),
    #[cfg(feature="windows-ui")] fn get_ForegroundColor(&self, out: *mut ::rt::gen::windows::ui::Color) -> HRESULT,
    fn get_ExitDirections(&self, out: *mut *mut ::rt::gen::windows::foundation::collections::IVectorView<HString>) -> HRESULT
}}
impl IGuidanceRoadSignpost {
    #[inline] pub unsafe fn get_exit_number(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_ExitNumber)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_exit(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Exit)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[cfg(feature="windows-ui")] #[inline] pub unsafe fn get_background_color(&self) -> Result<::rt::gen::windows::ui::Color> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_BackgroundColor)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[cfg(feature="windows-ui")] #[inline] pub unsafe fn get_foreground_color(&self) -> Result<::rt::gen::windows::ui::Color> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_ForegroundColor)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_exit_directions(&self) -> Result<ComPtr<::rt::gen::windows::foundation::collections::IVectorView<HString>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_ExitDirections)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
RT_CLASS!{class GuidanceRoadSignpost: IGuidanceRoadSignpost}
DEFINE_IID!(IID_IGuidanceRoute, 974410845, 32794, 16573, 162, 134, 175, 178, 1, 12, 206, 108);
RT_INTERFACE!{interface IGuidanceRoute(IGuidanceRouteVtbl): IInspectable(IInspectableVtbl) [IID_IGuidanceRoute] {
    fn get_Duration(&self, out: *mut ::rt::gen::windows::foundation::TimeSpan) -> HRESULT,
    fn get_Distance(&self, out: *mut i32) -> HRESULT,
    fn get_Maneuvers(&self, out: *mut *mut ::rt::gen::windows::foundation::collections::IVectorView<GuidanceManeuver>) -> HRESULT,
    #[cfg(not(feature="windows-devices"))] fn __Dummy3(&self) -> (),
    #[cfg(feature="windows-devices")] fn get_BoundingBox(&self, out: *mut *mut ::rt::gen::windows::devices::geolocation::GeoboundingBox) -> HRESULT,
    #[cfg(not(feature="windows-devices"))] fn __Dummy4(&self) -> (),
    #[cfg(feature="windows-devices")] fn get_Path(&self, out: *mut *mut ::rt::gen::windows::devices::geolocation::Geopath) -> HRESULT,
    fn get_RoadSegments(&self, out: *mut *mut ::rt::gen::windows::foundation::collections::IVectorView<GuidanceRoadSegment>) -> HRESULT,
    fn ConvertToMapRoute(&self, out: *mut *mut super::MapRoute) -> HRESULT
}}
impl IGuidanceRoute {
    #[inline] pub unsafe fn get_duration(&self) -> Result<::rt::gen::windows::foundation::TimeSpan> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_Duration)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_distance(&self) -> Result<i32> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_Distance)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_maneuvers(&self) -> Result<ComPtr<::rt::gen::windows::foundation::collections::IVectorView<GuidanceManeuver>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Maneuvers)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[cfg(feature="windows-devices")] #[inline] pub unsafe fn get_bounding_box(&self) -> Result<ComPtr<::rt::gen::windows::devices::geolocation::GeoboundingBox>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_BoundingBox)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[cfg(feature="windows-devices")] #[inline] pub unsafe fn get_path(&self) -> Result<ComPtr<::rt::gen::windows::devices::geolocation::Geopath>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Path)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_road_segments(&self) -> Result<ComPtr<::rt::gen::windows::foundation::collections::IVectorView<GuidanceRoadSegment>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_RoadSegments)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn convert_to_map_route(&self) -> Result<ComPtr<super::MapRoute>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).ConvertToMapRoute)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
RT_CLASS!{class GuidanceRoute: IGuidanceRoute}
impl RtActivatable<IGuidanceRouteStatics> for GuidanceRoute {}
impl GuidanceRoute {
    #[inline] pub fn can_create_from_map_route(mapRoute: &super::MapRoute) -> Result<bool> { unsafe {
        <Self as RtActivatable<IGuidanceRouteStatics>>::get_activation_factory().can_create_from_map_route(mapRoute)
    }}
    #[inline] pub fn try_create_from_map_route(mapRoute: &super::MapRoute) -> Result<ComPtr<GuidanceRoute>> { unsafe {
        <Self as RtActivatable<IGuidanceRouteStatics>>::get_activation_factory().try_create_from_map_route(mapRoute)
    }}
}
DEFINE_CLSID!(GuidanceRoute(&[87,105,110,100,111,119,115,46,83,101,114,118,105,99,101,115,46,77,97,112,115,46,71,117,105,100,97,110,99,101,46,71,117,105,100,97,110,99,101,82,111,117,116,101,0]) [CLSID_GuidanceRoute]);
DEFINE_IID!(IID_IGuidanceRouteStatics, 4117598826, 21997, 18881, 176, 156, 75, 130, 35, 181, 13, 179);
RT_INTERFACE!{static interface IGuidanceRouteStatics(IGuidanceRouteStaticsVtbl): IInspectable(IInspectableVtbl) [IID_IGuidanceRouteStatics] {
    fn CanCreateFromMapRoute(&self, mapRoute: *mut super::MapRoute, out: *mut bool) -> HRESULT,
    fn TryCreateFromMapRoute(&self, mapRoute: *mut super::MapRoute, out: *mut *mut GuidanceRoute) -> HRESULT
}}
impl IGuidanceRouteStatics {
    #[inline] pub unsafe fn can_create_from_map_route(&self, mapRoute: &super::MapRoute) -> Result<bool> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).CanCreateFromMapRoute)(self as *const _ as *mut _, mapRoute as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn try_create_from_map_route(&self, mapRoute: &super::MapRoute) -> Result<ComPtr<GuidanceRoute>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).TryCreateFromMapRoute)(self as *const _ as *mut _, mapRoute as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
DEFINE_IID!(IID_IGuidanceTelemetryCollector, 3676278181, 47224, 19858, 152, 221, 52, 125, 35, 211, 130, 98);
RT_INTERFACE!{interface IGuidanceTelemetryCollector(IGuidanceTelemetryCollectorVtbl): IInspectable(IInspectableVtbl) [IID_IGuidanceTelemetryCollector] {
    fn get_Enabled(&self, out: *mut bool) -> HRESULT,
    fn put_Enabled(&self, value: bool) -> HRESULT,
    fn ClearLocalData(&self) -> HRESULT,
    fn get_SpeedTrigger(&self, out: *mut f64) -> HRESULT,
    fn put_SpeedTrigger(&self, value: f64) -> HRESULT,
    fn get_UploadFrequency(&self, out: *mut i32) -> HRESULT,
    fn put_UploadFrequency(&self, value: i32) -> HRESULT
}}
impl IGuidanceTelemetryCollector {
    #[inline] pub unsafe fn get_enabled(&self) -> Result<bool> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_Enabled)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_enabled(&self, value: bool) -> Result<()> {
        let hr = ((*self.lpVtbl).put_Enabled)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn clear_local_data(&self) -> Result<()> {
        let hr = ((*self.lpVtbl).ClearLocalData)(self as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_speed_trigger(&self) -> Result<f64> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_SpeedTrigger)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_speed_trigger(&self, value: f64) -> Result<()> {
        let hr = ((*self.lpVtbl).put_SpeedTrigger)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_upload_frequency(&self) -> Result<i32> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_UploadFrequency)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_upload_frequency(&self, value: i32) -> Result<()> {
        let hr = ((*self.lpVtbl).put_UploadFrequency)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
}
RT_CLASS!{class GuidanceTelemetryCollector: IGuidanceTelemetryCollector}
impl RtActivatable<IGuidanceTelemetryCollectorStatics> for GuidanceTelemetryCollector {}
impl GuidanceTelemetryCollector {
    #[inline] pub fn get_current() -> Result<ComPtr<GuidanceTelemetryCollector>> { unsafe {
        <Self as RtActivatable<IGuidanceTelemetryCollectorStatics>>::get_activation_factory().get_current()
    }}
}
DEFINE_CLSID!(GuidanceTelemetryCollector(&[87,105,110,100,111,119,115,46,83,101,114,118,105,99,101,115,46,77,97,112,115,46,71,117,105,100,97,110,99,101,46,71,117,105,100,97,110,99,101,84,101,108,101,109,101,116,114,121,67,111,108,108,101,99,116,111,114,0]) [CLSID_GuidanceTelemetryCollector]);
DEFINE_IID!(IID_IGuidanceTelemetryCollectorStatics, 911417415, 61792, 17659, 181, 120, 148, 87, 124, 160, 89, 144);
RT_INTERFACE!{static interface IGuidanceTelemetryCollectorStatics(IGuidanceTelemetryCollectorStaticsVtbl): IInspectable(IInspectableVtbl) [IID_IGuidanceTelemetryCollectorStatics] {
    fn GetCurrent(&self, out: *mut *mut GuidanceTelemetryCollector) -> HRESULT
}}
impl IGuidanceTelemetryCollectorStatics {
    #[inline] pub unsafe fn get_current(&self) -> Result<ComPtr<GuidanceTelemetryCollector>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetCurrent)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
DEFINE_IID!(IID_IGuidanceUpdatedEventArgs, 4255913483, 40589, 19939, 169, 250, 176, 99, 33, 209, 141, 185);
RT_INTERFACE!{interface IGuidanceUpdatedEventArgs(IGuidanceUpdatedEventArgsVtbl): IInspectable(IInspectableVtbl) [IID_IGuidanceUpdatedEventArgs] {
    fn get_Mode(&self, out: *mut GuidanceMode) -> HRESULT,
    fn get_NextManeuver(&self, out: *mut *mut GuidanceManeuver) -> HRESULT,
    fn get_NextManeuverDistance(&self, out: *mut i32) -> HRESULT,
    fn get_AfterNextManeuver(&self, out: *mut *mut GuidanceManeuver) -> HRESULT,
    fn get_AfterNextManeuverDistance(&self, out: *mut i32) -> HRESULT,
    fn get_DistanceToDestination(&self, out: *mut i32) -> HRESULT,
    fn get_ElapsedDistance(&self, out: *mut i32) -> HRESULT,
    fn get_ElapsedTime(&self, out: *mut ::rt::gen::windows::foundation::TimeSpan) -> HRESULT,
    fn get_TimeToDestination(&self, out: *mut ::rt::gen::windows::foundation::TimeSpan) -> HRESULT,
    fn get_RoadName(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Route(&self, out: *mut *mut GuidanceRoute) -> HRESULT,
    fn get_CurrentLocation(&self, out: *mut *mut GuidanceMapMatchedCoordinate) -> HRESULT,
    fn get_IsNewManeuver(&self, out: *mut bool) -> HRESULT,
    fn get_LaneInfo(&self, out: *mut *mut ::rt::gen::windows::foundation::collections::IVectorView<GuidanceLaneInfo>) -> HRESULT
}}
impl IGuidanceUpdatedEventArgs {
    #[inline] pub unsafe fn get_mode(&self) -> Result<GuidanceMode> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_Mode)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_next_maneuver(&self) -> Result<ComPtr<GuidanceManeuver>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_NextManeuver)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_next_maneuver_distance(&self) -> Result<i32> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_NextManeuverDistance)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_after_next_maneuver(&self) -> Result<ComPtr<GuidanceManeuver>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_AfterNextManeuver)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_after_next_maneuver_distance(&self) -> Result<i32> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_AfterNextManeuverDistance)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_distance_to_destination(&self) -> Result<i32> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_DistanceToDestination)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_elapsed_distance(&self) -> Result<i32> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_ElapsedDistance)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_elapsed_time(&self) -> Result<::rt::gen::windows::foundation::TimeSpan> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_ElapsedTime)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_time_to_destination(&self) -> Result<::rt::gen::windows::foundation::TimeSpan> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_TimeToDestination)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_road_name(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_RoadName)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_route(&self) -> Result<ComPtr<GuidanceRoute>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Route)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_current_location(&self) -> Result<ComPtr<GuidanceMapMatchedCoordinate>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_CurrentLocation)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_is_new_maneuver(&self) -> Result<bool> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_IsNewManeuver)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_lane_info(&self) -> Result<ComPtr<::rt::gen::windows::foundation::collections::IVectorView<GuidanceLaneInfo>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_LaneInfo)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
RT_CLASS!{class GuidanceUpdatedEventArgs: IGuidanceUpdatedEventArgs}
} // Windows.Services.Maps.Guidance
} // Windows.Services.Maps
pub mod store { // Windows.Services.Store
use ::prelude::*;
DEFINE_IID!(IID_IStoreAcquireLicenseResult, 4225209453, 61504, 19635, 154, 57, 41, 188, 236, 219, 226, 45);
RT_INTERFACE!{interface IStoreAcquireLicenseResult(IStoreAcquireLicenseResultVtbl): IInspectable(IInspectableVtbl) [IID_IStoreAcquireLicenseResult] {
    fn get_StorePackageLicense(&self, out: *mut *mut StorePackageLicense) -> HRESULT,
    fn get_ExtendedError(&self, out: *mut super::super::foundation::HResult) -> HRESULT
}}
impl IStoreAcquireLicenseResult {
    #[inline] pub unsafe fn get_store_package_license(&self) -> Result<ComPtr<StorePackageLicense>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_StorePackageLicense)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_extended_error(&self) -> Result<super::super::foundation::HResult> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_ExtendedError)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
}
RT_CLASS!{class StoreAcquireLicenseResult: IStoreAcquireLicenseResult}
DEFINE_IID!(IID_IStoreAppLicense, 4085905886, 29632, 17870, 155, 171, 178, 254, 62, 94, 175, 211);
RT_INTERFACE!{interface IStoreAppLicense(IStoreAppLicenseVtbl): IInspectable(IInspectableVtbl) [IID_IStoreAppLicense] {
    fn get_SkuStoreId(&self, out: *mut HSTRING) -> HRESULT,
    fn get_IsActive(&self, out: *mut bool) -> HRESULT,
    fn get_IsTrial(&self, out: *mut bool) -> HRESULT,
    fn get_ExpirationDate(&self, out: *mut super::super::foundation::DateTime) -> HRESULT,
    fn get_ExtendedJsonData(&self, out: *mut HSTRING) -> HRESULT,
    fn get_AddOnLicenses(&self, out: *mut *mut super::super::foundation::collections::IMapView<HString, StoreLicense>) -> HRESULT,
    fn get_TrialTimeRemaining(&self, out: *mut super::super::foundation::TimeSpan) -> HRESULT,
    fn get_IsTrialOwnedByThisUser(&self, out: *mut bool) -> HRESULT,
    fn get_TrialUniqueId(&self, out: *mut HSTRING) -> HRESULT
}}
impl IStoreAppLicense {
    #[inline] pub unsafe fn get_sku_store_id(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_SkuStoreId)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_is_active(&self) -> Result<bool> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_IsActive)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_is_trial(&self) -> Result<bool> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_IsTrial)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_expiration_date(&self) -> Result<super::super::foundation::DateTime> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_ExpirationDate)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_extended_json_data(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_ExtendedJsonData)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_add_on_licenses(&self) -> Result<ComPtr<super::super::foundation::collections::IMapView<HString, StoreLicense>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_AddOnLicenses)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_trial_time_remaining(&self) -> Result<super::super::foundation::TimeSpan> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_TrialTimeRemaining)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_is_trial_owned_by_this_user(&self) -> Result<bool> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_IsTrialOwnedByThisUser)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_trial_unique_id(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_TrialUniqueId)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
}
RT_CLASS!{class StoreAppLicense: IStoreAppLicense}
DEFINE_IID!(IID_IStoreAvailability, 4194698021, 4093, 17555, 173, 67, 241, 249, 145, 143, 105, 250);
RT_INTERFACE!{interface IStoreAvailability(IStoreAvailabilityVtbl): IInspectable(IInspectableVtbl) [IID_IStoreAvailability] {
    fn get_StoreId(&self, out: *mut HSTRING) -> HRESULT,
    fn get_EndDate(&self, out: *mut super::super::foundation::DateTime) -> HRESULT,
    fn get_Price(&self, out: *mut *mut StorePrice) -> HRESULT,
    fn get_ExtendedJsonData(&self, out: *mut HSTRING) -> HRESULT,
    fn RequestPurchaseAsync(&self, out: *mut *mut super::super::foundation::IAsyncOperation<StorePurchaseResult>) -> HRESULT,
    fn RequestPurchaseWithPurchasePropertiesAsync(&self, storePurchaseProperties: *mut StorePurchaseProperties, out: *mut *mut super::super::foundation::IAsyncOperation<StorePurchaseResult>) -> HRESULT
}}
impl IStoreAvailability {
    #[inline] pub unsafe fn get_store_id(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_StoreId)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_end_date(&self) -> Result<super::super::foundation::DateTime> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_EndDate)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_price(&self) -> Result<ComPtr<StorePrice>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Price)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_extended_json_data(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_ExtendedJsonData)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn request_purchase_async(&self) -> Result<ComPtr<super::super::foundation::IAsyncOperation<StorePurchaseResult>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).RequestPurchaseAsync)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn request_purchase_with_purchase_properties_async(&self, storePurchaseProperties: &StorePurchaseProperties) -> Result<ComPtr<super::super::foundation::IAsyncOperation<StorePurchaseResult>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).RequestPurchaseWithPurchasePropertiesAsync)(self as *const _ as *mut _, storePurchaseProperties as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
RT_CLASS!{class StoreAvailability: IStoreAvailability}
DEFINE_IID!(IID_IStoreCollectionData, 2326053811, 23475, 17434, 42, 180, 77, 171, 115, 213, 206, 103);
RT_INTERFACE!{interface IStoreCollectionData(IStoreCollectionDataVtbl): IInspectable(IInspectableVtbl) [IID_IStoreCollectionData] {
    fn get_IsTrial(&self, out: *mut bool) -> HRESULT,
    fn get_CampaignId(&self, out: *mut HSTRING) -> HRESULT,
    fn get_DeveloperOfferId(&self, out: *mut HSTRING) -> HRESULT,
    fn get_AcquiredDate(&self, out: *mut super::super::foundation::DateTime) -> HRESULT,
    fn get_StartDate(&self, out: *mut super::super::foundation::DateTime) -> HRESULT,
    fn get_EndDate(&self, out: *mut super::super::foundation::DateTime) -> HRESULT,
    fn get_TrialTimeRemaining(&self, out: *mut super::super::foundation::TimeSpan) -> HRESULT,
    fn get_ExtendedJsonData(&self, out: *mut HSTRING) -> HRESULT
}}
impl IStoreCollectionData {
    #[inline] pub unsafe fn get_is_trial(&self) -> Result<bool> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_IsTrial)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_campaign_id(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_CampaignId)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_developer_offer_id(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_DeveloperOfferId)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_acquired_date(&self) -> Result<super::super::foundation::DateTime> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_AcquiredDate)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_start_date(&self) -> Result<super::super::foundation::DateTime> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_StartDate)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_end_date(&self) -> Result<super::super::foundation::DateTime> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_EndDate)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_trial_time_remaining(&self) -> Result<super::super::foundation::TimeSpan> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_TrialTimeRemaining)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_extended_json_data(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_ExtendedJsonData)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
}
RT_CLASS!{class StoreCollectionData: IStoreCollectionData}
DEFINE_IID!(IID_IStoreConsumableResult, 3932007282, 27136, 16466, 190, 91, 191, 218, 180, 67, 51, 82);
RT_INTERFACE!{interface IStoreConsumableResult(IStoreConsumableResultVtbl): IInspectable(IInspectableVtbl) [IID_IStoreConsumableResult] {
    fn get_Status(&self, out: *mut StoreConsumableStatus) -> HRESULT,
    fn get_TrackingId(&self, out: *mut Guid) -> HRESULT,
    fn get_BalanceRemaining(&self, out: *mut u32) -> HRESULT,
    fn get_ExtendedError(&self, out: *mut super::super::foundation::HResult) -> HRESULT
}}
impl IStoreConsumableResult {
    #[inline] pub unsafe fn get_status(&self) -> Result<StoreConsumableStatus> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_Status)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_tracking_id(&self) -> Result<Guid> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_TrackingId)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_balance_remaining(&self) -> Result<u32> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_BalanceRemaining)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_extended_error(&self) -> Result<super::super::foundation::HResult> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_ExtendedError)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
}
RT_CLASS!{class StoreConsumableResult: IStoreConsumableResult}
RT_ENUM! { enum StoreConsumableStatus: i32 {
    Succeeded (StoreConsumableStatus_Succeeded) = 0, InsufficentQuantity (StoreConsumableStatus_InsufficentQuantity) = 1, NetworkError (StoreConsumableStatus_NetworkError) = 2, ServerError (StoreConsumableStatus_ServerError) = 3,
}}
DEFINE_IID!(IID_IStoreContext, 2895689406, 62717, 18706, 186, 189, 80, 53, 229, 232, 188, 171);
RT_INTERFACE!{interface IStoreContext(IStoreContextVtbl): IInspectable(IInspectableVtbl) [IID_IStoreContext] {
    #[cfg(not(feature="windows-system"))] fn __Dummy0(&self) -> (),
    #[cfg(feature="windows-system")] fn get_User(&self, out: *mut *mut super::super::system::User) -> HRESULT,
    fn add_OfflineLicensesChanged(&self, handler: *mut super::super::foundation::TypedEventHandler<StoreContext, IInspectable>, out: *mut super::super::foundation::EventRegistrationToken) -> HRESULT,
    fn remove_OfflineLicensesChanged(&self, token: super::super::foundation::EventRegistrationToken) -> HRESULT,
    fn GetCustomerPurchaseIdAsync(&self, serviceTicket: HSTRING, publisherUserId: HSTRING, out: *mut *mut super::super::foundation::IAsyncOperation<HString>) -> HRESULT,
    fn GetCustomerCollectionsIdAsync(&self, serviceTicket: HSTRING, publisherUserId: HSTRING, out: *mut *mut super::super::foundation::IAsyncOperation<HString>) -> HRESULT,
    fn GetAppLicenseAsync(&self, out: *mut *mut super::super::foundation::IAsyncOperation<StoreAppLicense>) -> HRESULT,
    fn GetStoreProductForCurrentAppAsync(&self, out: *mut *mut super::super::foundation::IAsyncOperation<StoreProductResult>) -> HRESULT,
    fn GetStoreProductsAsync(&self, productKinds: *mut super::super::foundation::collections::IIterable<HString>, storeIds: *mut super::super::foundation::collections::IIterable<HString>, out: *mut *mut super::super::foundation::IAsyncOperation<StoreProductQueryResult>) -> HRESULT,
    fn GetAssociatedStoreProductsAsync(&self, productKinds: *mut super::super::foundation::collections::IIterable<HString>, out: *mut *mut super::super::foundation::IAsyncOperation<StoreProductQueryResult>) -> HRESULT,
    fn GetAssociatedStoreProductsWithPagingAsync(&self, productKinds: *mut super::super::foundation::collections::IIterable<HString>, maxItemsToRetrievePerPage: u32, out: *mut *mut super::super::foundation::IAsyncOperation<StoreProductPagedQueryResult>) -> HRESULT,
    fn GetUserCollectionAsync(&self, productKinds: *mut super::super::foundation::collections::IIterable<HString>, out: *mut *mut super::super::foundation::IAsyncOperation<StoreProductQueryResult>) -> HRESULT,
    fn GetUserCollectionWithPagingAsync(&self, productKinds: *mut super::super::foundation::collections::IIterable<HString>, maxItemsToRetrievePerPage: u32, out: *mut *mut super::super::foundation::IAsyncOperation<StoreProductPagedQueryResult>) -> HRESULT,
    fn ReportConsumableFulfillmentAsync(&self, productStoreId: HSTRING, quantity: u32, trackingId: Guid, out: *mut *mut super::super::foundation::IAsyncOperation<StoreConsumableResult>) -> HRESULT,
    fn GetConsumableBalanceRemainingAsync(&self, productStoreId: HSTRING, out: *mut *mut super::super::foundation::IAsyncOperation<StoreConsumableResult>) -> HRESULT,
    #[cfg(not(feature="windows-applicationmodel"))] fn __Dummy14(&self) -> (),
    #[cfg(feature="windows-applicationmodel")] fn AcquireStoreLicenseForOptionalPackageAsync(&self, optionalPackage: *mut super::super::applicationmodel::Package, out: *mut *mut super::super::foundation::IAsyncOperation<StoreAcquireLicenseResult>) -> HRESULT,
    fn RequestPurchaseAsync(&self, storeId: HSTRING, out: *mut *mut super::super::foundation::IAsyncOperation<StorePurchaseResult>) -> HRESULT,
    fn RequestPurchaseWithPurchasePropertiesAsync(&self, storeId: HSTRING, storePurchaseProperties: *mut StorePurchaseProperties, out: *mut *mut super::super::foundation::IAsyncOperation<StorePurchaseResult>) -> HRESULT,
    fn GetAppAndOptionalStorePackageUpdatesAsync(&self, out: *mut *mut super::super::foundation::IAsyncOperation<super::super::foundation::collections::IVectorView<StorePackageUpdate>>) -> HRESULT,
    fn RequestDownloadStorePackageUpdatesAsync(&self, storePackageUpdates: *mut super::super::foundation::collections::IIterable<StorePackageUpdate>, out: *mut *mut super::super::foundation::IAsyncOperationWithProgress<StorePackageUpdateResult, StorePackageUpdateStatus>) -> HRESULT,
    fn RequestDownloadAndInstallStorePackageUpdatesAsync(&self, storePackageUpdates: *mut super::super::foundation::collections::IIterable<StorePackageUpdate>, out: *mut *mut super::super::foundation::IAsyncOperationWithProgress<StorePackageUpdateResult, StorePackageUpdateStatus>) -> HRESULT,
    fn RequestDownloadAndInstallStorePackagesAsync(&self, storeIds: *mut super::super::foundation::collections::IIterable<HString>, out: *mut *mut super::super::foundation::IAsyncOperationWithProgress<StorePackageUpdateResult, StorePackageUpdateStatus>) -> HRESULT
}}
impl IStoreContext {
    #[cfg(feature="windows-system")] #[inline] pub unsafe fn get_user(&self) -> Result<ComPtr<super::super::system::User>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_User)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn add_offline_licenses_changed(&self, handler: &super::super::foundation::TypedEventHandler<StoreContext, IInspectable>) -> Result<super::super::foundation::EventRegistrationToken> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).add_OfflineLicensesChanged)(self as *const _ as *mut _, handler as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn remove_offline_licenses_changed(&self, token: super::super::foundation::EventRegistrationToken) -> Result<()> {
        let hr = ((*self.lpVtbl).remove_OfflineLicensesChanged)(self as *const _ as *mut _, token);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_customer_purchase_id_async(&self, serviceTicket: &HStringArg, publisherUserId: &HStringArg) -> Result<ComPtr<super::super::foundation::IAsyncOperation<HString>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetCustomerPurchaseIdAsync)(self as *const _ as *mut _, serviceTicket.get(), publisherUserId.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_customer_collections_id_async(&self, serviceTicket: &HStringArg, publisherUserId: &HStringArg) -> Result<ComPtr<super::super::foundation::IAsyncOperation<HString>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetCustomerCollectionsIdAsync)(self as *const _ as *mut _, serviceTicket.get(), publisherUserId.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_app_license_async(&self) -> Result<ComPtr<super::super::foundation::IAsyncOperation<StoreAppLicense>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetAppLicenseAsync)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_store_product_for_current_app_async(&self) -> Result<ComPtr<super::super::foundation::IAsyncOperation<StoreProductResult>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetStoreProductForCurrentAppAsync)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_store_products_async(&self, productKinds: &super::super::foundation::collections::IIterable<HString>, storeIds: &super::super::foundation::collections::IIterable<HString>) -> Result<ComPtr<super::super::foundation::IAsyncOperation<StoreProductQueryResult>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetStoreProductsAsync)(self as *const _ as *mut _, productKinds as *const _ as *mut _, storeIds as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_associated_store_products_async(&self, productKinds: &super::super::foundation::collections::IIterable<HString>) -> Result<ComPtr<super::super::foundation::IAsyncOperation<StoreProductQueryResult>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetAssociatedStoreProductsAsync)(self as *const _ as *mut _, productKinds as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_associated_store_products_with_paging_async(&self, productKinds: &super::super::foundation::collections::IIterable<HString>, maxItemsToRetrievePerPage: u32) -> Result<ComPtr<super::super::foundation::IAsyncOperation<StoreProductPagedQueryResult>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetAssociatedStoreProductsWithPagingAsync)(self as *const _ as *mut _, productKinds as *const _ as *mut _, maxItemsToRetrievePerPage, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_user_collection_async(&self, productKinds: &super::super::foundation::collections::IIterable<HString>) -> Result<ComPtr<super::super::foundation::IAsyncOperation<StoreProductQueryResult>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetUserCollectionAsync)(self as *const _ as *mut _, productKinds as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_user_collection_with_paging_async(&self, productKinds: &super::super::foundation::collections::IIterable<HString>, maxItemsToRetrievePerPage: u32) -> Result<ComPtr<super::super::foundation::IAsyncOperation<StoreProductPagedQueryResult>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetUserCollectionWithPagingAsync)(self as *const _ as *mut _, productKinds as *const _ as *mut _, maxItemsToRetrievePerPage, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn report_consumable_fulfillment_async(&self, productStoreId: &HStringArg, quantity: u32, trackingId: Guid) -> Result<ComPtr<super::super::foundation::IAsyncOperation<StoreConsumableResult>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).ReportConsumableFulfillmentAsync)(self as *const _ as *mut _, productStoreId.get(), quantity, trackingId, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_consumable_balance_remaining_async(&self, productStoreId: &HStringArg) -> Result<ComPtr<super::super::foundation::IAsyncOperation<StoreConsumableResult>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetConsumableBalanceRemainingAsync)(self as *const _ as *mut _, productStoreId.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[cfg(feature="windows-applicationmodel")] #[inline] pub unsafe fn acquire_store_license_for_optional_package_async(&self, optionalPackage: &super::super::applicationmodel::Package) -> Result<ComPtr<super::super::foundation::IAsyncOperation<StoreAcquireLicenseResult>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).AcquireStoreLicenseForOptionalPackageAsync)(self as *const _ as *mut _, optionalPackage as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn request_purchase_async(&self, storeId: &HStringArg) -> Result<ComPtr<super::super::foundation::IAsyncOperation<StorePurchaseResult>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).RequestPurchaseAsync)(self as *const _ as *mut _, storeId.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn request_purchase_with_purchase_properties_async(&self, storeId: &HStringArg, storePurchaseProperties: &StorePurchaseProperties) -> Result<ComPtr<super::super::foundation::IAsyncOperation<StorePurchaseResult>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).RequestPurchaseWithPurchasePropertiesAsync)(self as *const _ as *mut _, storeId.get(), storePurchaseProperties as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_app_and_optional_store_package_updates_async(&self) -> Result<ComPtr<super::super::foundation::IAsyncOperation<super::super::foundation::collections::IVectorView<StorePackageUpdate>>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetAppAndOptionalStorePackageUpdatesAsync)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn request_download_store_package_updates_async(&self, storePackageUpdates: &super::super::foundation::collections::IIterable<StorePackageUpdate>) -> Result<ComPtr<super::super::foundation::IAsyncOperationWithProgress<StorePackageUpdateResult, StorePackageUpdateStatus>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).RequestDownloadStorePackageUpdatesAsync)(self as *const _ as *mut _, storePackageUpdates as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn request_download_and_install_store_package_updates_async(&self, storePackageUpdates: &super::super::foundation::collections::IIterable<StorePackageUpdate>) -> Result<ComPtr<super::super::foundation::IAsyncOperationWithProgress<StorePackageUpdateResult, StorePackageUpdateStatus>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).RequestDownloadAndInstallStorePackageUpdatesAsync)(self as *const _ as *mut _, storePackageUpdates as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn request_download_and_install_store_packages_async(&self, storeIds: &super::super::foundation::collections::IIterable<HString>) -> Result<ComPtr<super::super::foundation::IAsyncOperationWithProgress<StorePackageUpdateResult, StorePackageUpdateStatus>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).RequestDownloadAndInstallStorePackagesAsync)(self as *const _ as *mut _, storeIds as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
RT_CLASS!{class StoreContext: IStoreContext}
impl RtActivatable<IStoreContextStatics> for StoreContext {}
impl StoreContext {
    #[inline] pub fn get_default() -> Result<ComPtr<StoreContext>> { unsafe {
        <Self as RtActivatable<IStoreContextStatics>>::get_activation_factory().get_default()
    }}
    #[cfg(feature="windows-system")] #[inline] pub fn get_for_user(user: &super::super::system::User) -> Result<ComPtr<StoreContext>> { unsafe {
        <Self as RtActivatable<IStoreContextStatics>>::get_activation_factory().get_for_user(user)
    }}
}
DEFINE_CLSID!(StoreContext(&[87,105,110,100,111,119,115,46,83,101,114,118,105,99,101,115,46,83,116,111,114,101,46,83,116,111,114,101,67,111,110,116,101,120,116,0]) [CLSID_StoreContext]);
DEFINE_IID!(IID_IStoreContext2, 414995674, 31705, 17708, 145, 22, 59, 189, 6, 255, 198, 58);
RT_INTERFACE!{interface IStoreContext2(IStoreContext2Vtbl): IInspectable(IInspectableVtbl) [IID_IStoreContext2] {
    #[cfg(feature="windows-applicationmodel")] fn FindStoreProductForPackageAsync(&self, productKinds: *mut super::super::foundation::collections::IIterable<HString>, package: *mut super::super::applicationmodel::Package, out: *mut *mut super::super::foundation::IAsyncOperation<StoreProductResult>) -> HRESULT
}}
impl IStoreContext2 {
    #[cfg(feature="windows-applicationmodel")] #[inline] pub unsafe fn find_store_product_for_package_async(&self, productKinds: &super::super::foundation::collections::IIterable<HString>, package: &super::super::applicationmodel::Package) -> Result<ComPtr<super::super::foundation::IAsyncOperation<StoreProductResult>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).FindStoreProductForPackageAsync)(self as *const _ as *mut _, productKinds as *const _ as *mut _, package as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
DEFINE_IID!(IID_IStoreContextStatics, 2617699935, 5568, 20082, 147, 48, 214, 25, 28, 235, 209, 156);
RT_INTERFACE!{static interface IStoreContextStatics(IStoreContextStaticsVtbl): IInspectable(IInspectableVtbl) [IID_IStoreContextStatics] {
    fn GetDefault(&self, out: *mut *mut StoreContext) -> HRESULT,
    #[cfg(feature="windows-system")] fn GetForUser(&self, user: *mut super::super::system::User, out: *mut *mut StoreContext) -> HRESULT
}}
impl IStoreContextStatics {
    #[inline] pub unsafe fn get_default(&self) -> Result<ComPtr<StoreContext>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetDefault)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[cfg(feature="windows-system")] #[inline] pub unsafe fn get_for_user(&self, user: &super::super::system::User) -> Result<ComPtr<StoreContext>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetForUser)(self as *const _ as *mut _, user as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
RT_ENUM! { enum StoreDurationUnit: i32 {
    Minute (StoreDurationUnit_Minute) = 0, Hour (StoreDurationUnit_Hour) = 1, Day (StoreDurationUnit_Day) = 2, Week (StoreDurationUnit_Week) = 3, Month (StoreDurationUnit_Month) = 4, Year (StoreDurationUnit_Year) = 5,
}}
DEFINE_IID!(IID_IStoreImage, 136303176, 44468, 19300, 169, 147, 120, 71, 137, 146, 110, 213);
RT_INTERFACE!{interface IStoreImage(IStoreImageVtbl): IInspectable(IInspectableVtbl) [IID_IStoreImage] {
    fn get_Uri(&self, out: *mut *mut super::super::foundation::Uri) -> HRESULT,
    fn get_ImagePurposeTag(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Width(&self, out: *mut u32) -> HRESULT,
    fn get_Height(&self, out: *mut u32) -> HRESULT,
    fn get_Caption(&self, out: *mut HSTRING) -> HRESULT
}}
impl IStoreImage {
    #[inline] pub unsafe fn get_uri(&self) -> Result<ComPtr<super::super::foundation::Uri>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Uri)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_image_purpose_tag(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_ImagePurposeTag)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
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
    #[inline] pub unsafe fn get_caption(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Caption)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
}
RT_CLASS!{class StoreImage: IStoreImage}
DEFINE_IID!(IID_IStoreLicense, 651990393, 19535, 20272, 188, 137, 100, 159, 96, 227, 96, 85);
RT_INTERFACE!{interface IStoreLicense(IStoreLicenseVtbl): IInspectable(IInspectableVtbl) [IID_IStoreLicense] {
    fn get_SkuStoreId(&self, out: *mut HSTRING) -> HRESULT,
    fn get_IsActive(&self, out: *mut bool) -> HRESULT,
    fn get_ExpirationDate(&self, out: *mut super::super::foundation::DateTime) -> HRESULT,
    fn get_ExtendedJsonData(&self, out: *mut HSTRING) -> HRESULT,
    fn get_InAppOfferToken(&self, out: *mut HSTRING) -> HRESULT
}}
impl IStoreLicense {
    #[inline] pub unsafe fn get_sku_store_id(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_SkuStoreId)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_is_active(&self) -> Result<bool> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_IsActive)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_expiration_date(&self) -> Result<super::super::foundation::DateTime> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_ExpirationDate)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_extended_json_data(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_ExtendedJsonData)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_in_app_offer_token(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_InAppOfferToken)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
}
RT_CLASS!{class StoreLicense: IStoreLicense}
DEFINE_IID!(IID_IStorePackageLicense, 205936404, 5345, 18803, 189, 20, 247, 119, 36, 39, 30, 153);
RT_INTERFACE!{interface IStorePackageLicense(IStorePackageLicenseVtbl): IInspectable(IInspectableVtbl) [IID_IStorePackageLicense] {
    fn add_LicenseLost(&self, handler: *mut super::super::foundation::TypedEventHandler<StorePackageLicense, IInspectable>, out: *mut super::super::foundation::EventRegistrationToken) -> HRESULT,
    fn remove_LicenseLost(&self, token: super::super::foundation::EventRegistrationToken) -> HRESULT,
    #[cfg(not(feature="windows-applicationmodel"))] fn __Dummy2(&self) -> (),
    #[cfg(feature="windows-applicationmodel")] fn get_Package(&self, out: *mut *mut super::super::applicationmodel::Package) -> HRESULT,
    fn get_IsValid(&self, out: *mut bool) -> HRESULT,
    fn ReleaseLicense(&self) -> HRESULT
}}
impl IStorePackageLicense {
    #[inline] pub unsafe fn add_license_lost(&self, handler: &super::super::foundation::TypedEventHandler<StorePackageLicense, IInspectable>) -> Result<super::super::foundation::EventRegistrationToken> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).add_LicenseLost)(self as *const _ as *mut _, handler as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn remove_license_lost(&self, token: super::super::foundation::EventRegistrationToken) -> Result<()> {
        let hr = ((*self.lpVtbl).remove_LicenseLost)(self as *const _ as *mut _, token);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[cfg(feature="windows-applicationmodel")] #[inline] pub unsafe fn get_package(&self) -> Result<ComPtr<super::super::applicationmodel::Package>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Package)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_is_valid(&self) -> Result<bool> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_IsValid)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn release_license(&self) -> Result<()> {
        let hr = ((*self.lpVtbl).ReleaseLicense)(self as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
}
RT_CLASS!{class StorePackageLicense: IStorePackageLicense}
DEFINE_IID!(IID_IStorePackageUpdate, 336568656, 15551, 18997, 185, 31, 72, 39, 28, 49, 176, 114);
RT_INTERFACE!{interface IStorePackageUpdate(IStorePackageUpdateVtbl): IInspectable(IInspectableVtbl) [IID_IStorePackageUpdate] {
    #[cfg(not(feature="windows-applicationmodel"))] fn __Dummy0(&self) -> (),
    #[cfg(feature="windows-applicationmodel")] fn get_Package(&self, out: *mut *mut super::super::applicationmodel::Package) -> HRESULT,
    fn get_Mandatory(&self, out: *mut bool) -> HRESULT
}}
impl IStorePackageUpdate {
    #[cfg(feature="windows-applicationmodel")] #[inline] pub unsafe fn get_package(&self) -> Result<ComPtr<super::super::applicationmodel::Package>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Package)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_mandatory(&self) -> Result<bool> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_Mandatory)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
}
RT_CLASS!{class StorePackageUpdate: IStorePackageUpdate}
DEFINE_IID!(IID_IStorePackageUpdateResult, 3885056749, 25081, 18579, 180, 254, 207, 25, 22, 3, 175, 123);
RT_INTERFACE!{interface IStorePackageUpdateResult(IStorePackageUpdateResultVtbl): IInspectable(IInspectableVtbl) [IID_IStorePackageUpdateResult] {
    fn get_OverallState(&self, out: *mut StorePackageUpdateState) -> HRESULT,
    fn get_StorePackageUpdateStatuses(&self, out: *mut *mut super::super::foundation::collections::IVectorView<StorePackageUpdateStatus>) -> HRESULT
}}
impl IStorePackageUpdateResult {
    #[inline] pub unsafe fn get_overall_state(&self) -> Result<StorePackageUpdateState> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_OverallState)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_store_package_update_statuses(&self) -> Result<ComPtr<super::super::foundation::collections::IVectorView<StorePackageUpdateStatus>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_StorePackageUpdateStatuses)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
RT_CLASS!{class StorePackageUpdateResult: IStorePackageUpdateResult}
RT_ENUM! { enum StorePackageUpdateState: i32 {
    Pending (StorePackageUpdateState_Pending) = 0, Downloading (StorePackageUpdateState_Downloading) = 1, Deploying (StorePackageUpdateState_Deploying) = 2, Completed (StorePackageUpdateState_Completed) = 3, Canceled (StorePackageUpdateState_Canceled) = 4, OtherError (StorePackageUpdateState_OtherError) = 5, ErrorLowBattery (StorePackageUpdateState_ErrorLowBattery) = 6, ErrorWiFiRecommended (StorePackageUpdateState_ErrorWiFiRecommended) = 7, ErrorWiFiRequired (StorePackageUpdateState_ErrorWiFiRequired) = 8,
}}
RT_STRUCT! { struct StorePackageUpdateStatus {
    PackageFamilyName: HSTRING, PackageDownloadSizeInBytes: u64, PackageBytesDownloaded: u64, PackageDownloadProgress: f64, TotalDownloadProgress: f64, PackageUpdateState: StorePackageUpdateState,
}}
DEFINE_IID!(IID_IStorePrice, 1438291140, 5617, 16508, 143, 6, 0, 99, 128, 244, 223, 11);
RT_INTERFACE!{interface IStorePrice(IStorePriceVtbl): IInspectable(IInspectableVtbl) [IID_IStorePrice] {
    fn get_FormattedBasePrice(&self, out: *mut HSTRING) -> HRESULT,
    fn get_FormattedPrice(&self, out: *mut HSTRING) -> HRESULT,
    fn get_IsOnSale(&self, out: *mut bool) -> HRESULT,
    fn get_SaleEndDate(&self, out: *mut super::super::foundation::DateTime) -> HRESULT,
    fn get_CurrencyCode(&self, out: *mut HSTRING) -> HRESULT,
    fn get_FormattedRecurrencePrice(&self, out: *mut HSTRING) -> HRESULT
}}
impl IStorePrice {
    #[inline] pub unsafe fn get_formatted_base_price(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_FormattedBasePrice)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_formatted_price(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_FormattedPrice)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_is_on_sale(&self) -> Result<bool> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_IsOnSale)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_sale_end_date(&self) -> Result<super::super::foundation::DateTime> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_SaleEndDate)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_currency_code(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_CurrencyCode)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_formatted_recurrence_price(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_FormattedRecurrencePrice)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
}
RT_CLASS!{class StorePrice: IStorePrice}
DEFINE_IID!(IID_IStoreProduct, 839789650, 55136, 17674, 164, 43, 103, 209, 233, 1, 172, 144);
RT_INTERFACE!{interface IStoreProduct(IStoreProductVtbl): IInspectable(IInspectableVtbl) [IID_IStoreProduct] {
    fn get_StoreId(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Language(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Title(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Description(&self, out: *mut HSTRING) -> HRESULT,
    fn get_ProductKind(&self, out: *mut HSTRING) -> HRESULT,
    fn get_HasDigitalDownload(&self, out: *mut bool) -> HRESULT,
    fn get_Keywords(&self, out: *mut *mut super::super::foundation::collections::IVectorView<HString>) -> HRESULT,
    fn get_Images(&self, out: *mut *mut super::super::foundation::collections::IVectorView<StoreImage>) -> HRESULT,
    fn get_Videos(&self, out: *mut *mut super::super::foundation::collections::IVectorView<StoreVideo>) -> HRESULT,
    fn get_Skus(&self, out: *mut *mut super::super::foundation::collections::IVectorView<StoreSku>) -> HRESULT,
    fn get_IsInUserCollection(&self, out: *mut bool) -> HRESULT,
    fn get_Price(&self, out: *mut *mut StorePrice) -> HRESULT,
    fn get_ExtendedJsonData(&self, out: *mut HSTRING) -> HRESULT,
    fn get_LinkUri(&self, out: *mut *mut super::super::foundation::Uri) -> HRESULT,
    fn GetIsAnySkuInstalledAsync(&self, out: *mut *mut super::super::foundation::IAsyncOperation<bool>) -> HRESULT,
    fn RequestPurchaseAsync(&self, out: *mut *mut super::super::foundation::IAsyncOperation<StorePurchaseResult>) -> HRESULT,
    fn RequestPurchaseWithPurchasePropertiesAsync(&self, storePurchaseProperties: *mut StorePurchaseProperties, out: *mut *mut super::super::foundation::IAsyncOperation<StorePurchaseResult>) -> HRESULT,
    fn get_InAppOfferToken(&self, out: *mut HSTRING) -> HRESULT
}}
impl IStoreProduct {
    #[inline] pub unsafe fn get_store_id(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_StoreId)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_language(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Language)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_title(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Title)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_description(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Description)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_product_kind(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_ProductKind)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_has_digital_download(&self) -> Result<bool> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_HasDigitalDownload)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_keywords(&self) -> Result<ComPtr<super::super::foundation::collections::IVectorView<HString>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Keywords)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_images(&self) -> Result<ComPtr<super::super::foundation::collections::IVectorView<StoreImage>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Images)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_videos(&self) -> Result<ComPtr<super::super::foundation::collections::IVectorView<StoreVideo>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Videos)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_skus(&self) -> Result<ComPtr<super::super::foundation::collections::IVectorView<StoreSku>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Skus)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_is_in_user_collection(&self) -> Result<bool> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_IsInUserCollection)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_price(&self) -> Result<ComPtr<StorePrice>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Price)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_extended_json_data(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_ExtendedJsonData)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_link_uri(&self) -> Result<ComPtr<super::super::foundation::Uri>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_LinkUri)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_is_any_sku_installed_async(&self) -> Result<ComPtr<super::super::foundation::IAsyncOperation<bool>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetIsAnySkuInstalledAsync)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn request_purchase_async(&self) -> Result<ComPtr<super::super::foundation::IAsyncOperation<StorePurchaseResult>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).RequestPurchaseAsync)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn request_purchase_with_purchase_properties_async(&self, storePurchaseProperties: &StorePurchaseProperties) -> Result<ComPtr<super::super::foundation::IAsyncOperation<StorePurchaseResult>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).RequestPurchaseWithPurchasePropertiesAsync)(self as *const _ as *mut _, storePurchaseProperties as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_in_app_offer_token(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_InAppOfferToken)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
}
RT_CLASS!{class StoreProduct: IStoreProduct}
DEFINE_IID!(IID_IStoreProductPagedQueryResult, 3374782661, 19925, 18537, 164, 98, 236, 198, 135, 46, 67, 197);
RT_INTERFACE!{interface IStoreProductPagedQueryResult(IStoreProductPagedQueryResultVtbl): IInspectable(IInspectableVtbl) [IID_IStoreProductPagedQueryResult] {
    fn get_Products(&self, out: *mut *mut super::super::foundation::collections::IMapView<HString, StoreProduct>) -> HRESULT,
    fn get_HasMoreResults(&self, out: *mut bool) -> HRESULT,
    fn get_ExtendedError(&self, out: *mut super::super::foundation::HResult) -> HRESULT,
    fn GetNextAsync(&self, out: *mut *mut super::super::foundation::IAsyncOperation<StoreProductPagedQueryResult>) -> HRESULT
}}
impl IStoreProductPagedQueryResult {
    #[inline] pub unsafe fn get_products(&self) -> Result<ComPtr<super::super::foundation::collections::IMapView<HString, StoreProduct>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Products)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_has_more_results(&self) -> Result<bool> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_HasMoreResults)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_extended_error(&self) -> Result<super::super::foundation::HResult> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_ExtendedError)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_next_async(&self) -> Result<ComPtr<super::super::foundation::IAsyncOperation<StoreProductPagedQueryResult>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetNextAsync)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
RT_CLASS!{class StoreProductPagedQueryResult: IStoreProductPagedQueryResult}
DEFINE_IID!(IID_IStoreProductQueryResult, 3624265413, 54358, 20470, 128, 73, 144, 118, 213, 22, 95, 115);
RT_INTERFACE!{interface IStoreProductQueryResult(IStoreProductQueryResultVtbl): IInspectable(IInspectableVtbl) [IID_IStoreProductQueryResult] {
    fn get_Products(&self, out: *mut *mut super::super::foundation::collections::IMapView<HString, StoreProduct>) -> HRESULT,
    fn get_ExtendedError(&self, out: *mut super::super::foundation::HResult) -> HRESULT
}}
impl IStoreProductQueryResult {
    #[inline] pub unsafe fn get_products(&self) -> Result<ComPtr<super::super::foundation::collections::IMapView<HString, StoreProduct>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Products)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_extended_error(&self) -> Result<super::super::foundation::HResult> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_ExtendedError)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
}
RT_CLASS!{class StoreProductQueryResult: IStoreProductQueryResult}
DEFINE_IID!(IID_IStoreProductResult, 3077001075, 15495, 20193, 130, 1, 244, 40, 53, 155, 211, 175);
RT_INTERFACE!{interface IStoreProductResult(IStoreProductResultVtbl): IInspectable(IInspectableVtbl) [IID_IStoreProductResult] {
    fn get_Product(&self, out: *mut *mut StoreProduct) -> HRESULT,
    fn get_ExtendedError(&self, out: *mut super::super::foundation::HResult) -> HRESULT
}}
impl IStoreProductResult {
    #[inline] pub unsafe fn get_product(&self) -> Result<ComPtr<StoreProduct>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Product)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_extended_error(&self) -> Result<super::super::foundation::HResult> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_ExtendedError)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
}
RT_CLASS!{class StoreProductResult: IStoreProductResult}
DEFINE_IID!(IID_IStorePurchaseProperties, 2204268787, 65415, 17252, 165, 180, 253, 33, 83, 235, 228, 59);
RT_INTERFACE!{interface IStorePurchaseProperties(IStorePurchasePropertiesVtbl): IInspectable(IInspectableVtbl) [IID_IStorePurchaseProperties] {
    fn get_Name(&self, out: *mut HSTRING) -> HRESULT,
    fn put_Name(&self, value: HSTRING) -> HRESULT,
    fn get_ExtendedJsonData(&self, out: *mut HSTRING) -> HRESULT,
    fn put_ExtendedJsonData(&self, value: HSTRING) -> HRESULT
}}
impl IStorePurchaseProperties {
    #[inline] pub unsafe fn get_name(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Name)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_name(&self, value: &HStringArg) -> Result<()> {
        let hr = ((*self.lpVtbl).put_Name)(self as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_extended_json_data(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_ExtendedJsonData)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_extended_json_data(&self, value: &HStringArg) -> Result<()> {
        let hr = ((*self.lpVtbl).put_ExtendedJsonData)(self as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
}
RT_CLASS!{class StorePurchaseProperties: IStorePurchaseProperties}
impl RtActivatable<IStorePurchasePropertiesFactory> for StorePurchaseProperties {}
impl RtActivatable<IActivationFactory> for StorePurchaseProperties {}
impl StorePurchaseProperties {
    #[inline] pub fn create(name: &HStringArg) -> Result<ComPtr<StorePurchaseProperties>> { unsafe {
        <Self as RtActivatable<IStorePurchasePropertiesFactory>>::get_activation_factory().create(name)
    }}
}
DEFINE_CLSID!(StorePurchaseProperties(&[87,105,110,100,111,119,115,46,83,101,114,118,105,99,101,115,46,83,116,111,114,101,46,83,116,111,114,101,80,117,114,99,104,97,115,101,80,114,111,112,101,114,116,105,101,115,0]) [CLSID_StorePurchaseProperties]);
DEFINE_IID!(IID_IStorePurchasePropertiesFactory, 2808673694, 65277, 18591, 154, 23, 34, 165, 147, 230, 139, 157);
RT_INTERFACE!{static interface IStorePurchasePropertiesFactory(IStorePurchasePropertiesFactoryVtbl): IInspectable(IInspectableVtbl) [IID_IStorePurchasePropertiesFactory] {
    fn Create(&self, name: HSTRING, out: *mut *mut StorePurchaseProperties) -> HRESULT
}}
impl IStorePurchasePropertiesFactory {
    #[inline] pub unsafe fn create(&self, name: &HStringArg) -> Result<ComPtr<StorePurchaseProperties>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).Create)(self as *const _ as *mut _, name.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
DEFINE_IID!(IID_IStorePurchaseResult, 2916255058, 63850, 17981, 167, 187, 194, 11, 79, 202, 105, 82);
RT_INTERFACE!{interface IStorePurchaseResult(IStorePurchaseResultVtbl): IInspectable(IInspectableVtbl) [IID_IStorePurchaseResult] {
    fn get_Status(&self, out: *mut StorePurchaseStatus) -> HRESULT,
    fn get_ExtendedError(&self, out: *mut super::super::foundation::HResult) -> HRESULT
}}
impl IStorePurchaseResult {
    #[inline] pub unsafe fn get_status(&self) -> Result<StorePurchaseStatus> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_Status)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_extended_error(&self) -> Result<super::super::foundation::HResult> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_ExtendedError)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
}
RT_CLASS!{class StorePurchaseResult: IStorePurchaseResult}
RT_ENUM! { enum StorePurchaseStatus: i32 {
    Succeeded (StorePurchaseStatus_Succeeded) = 0, AlreadyPurchased (StorePurchaseStatus_AlreadyPurchased) = 1, NotPurchased (StorePurchaseStatus_NotPurchased) = 2, NetworkError (StorePurchaseStatus_NetworkError) = 3, ServerError (StorePurchaseStatus_ServerError) = 4,
}}
RT_CLASS!{static class StoreRequestHelper}
impl RtActivatable<IStoreRequestHelperStatics> for StoreRequestHelper {}
impl StoreRequestHelper {
    #[inline] pub fn send_request_async(context: &StoreContext, requestKind: u32, parametersAsJson: &HStringArg) -> Result<ComPtr<super::super::foundation::IAsyncOperation<StoreSendRequestResult>>> { unsafe {
        <Self as RtActivatable<IStoreRequestHelperStatics>>::get_activation_factory().send_request_async(context, requestKind, parametersAsJson)
    }}
}
DEFINE_CLSID!(StoreRequestHelper(&[87,105,110,100,111,119,115,46,83,101,114,118,105,99,101,115,46,83,116,111,114,101,46,83,116,111,114,101,82,101,113,117,101,115,116,72,101,108,112,101,114,0]) [CLSID_StoreRequestHelper]);
DEFINE_IID!(IID_IStoreRequestHelperStatics, 1827005945, 41161, 19244, 150, 166, 161, 113, 198, 48, 3, 141);
RT_INTERFACE!{static interface IStoreRequestHelperStatics(IStoreRequestHelperStaticsVtbl): IInspectable(IInspectableVtbl) [IID_IStoreRequestHelperStatics] {
    fn SendRequestAsync(&self, context: *mut StoreContext, requestKind: u32, parametersAsJson: HSTRING, out: *mut *mut super::super::foundation::IAsyncOperation<StoreSendRequestResult>) -> HRESULT
}}
impl IStoreRequestHelperStatics {
    #[inline] pub unsafe fn send_request_async(&self, context: &StoreContext, requestKind: u32, parametersAsJson: &HStringArg) -> Result<ComPtr<super::super::foundation::IAsyncOperation<StoreSendRequestResult>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).SendRequestAsync)(self as *const _ as *mut _, context as *const _ as *mut _, requestKind, parametersAsJson.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
DEFINE_IID!(IID_IStoreSendRequestResult, 3342515808, 33394, 17666, 138, 105, 110, 117, 21, 58, 66, 153);
RT_INTERFACE!{interface IStoreSendRequestResult(IStoreSendRequestResultVtbl): IInspectable(IInspectableVtbl) [IID_IStoreSendRequestResult] {
    fn get_Response(&self, out: *mut HSTRING) -> HRESULT,
    fn get_ExtendedError(&self, out: *mut super::super::foundation::HResult) -> HRESULT
}}
impl IStoreSendRequestResult {
    #[inline] pub unsafe fn get_response(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Response)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_extended_error(&self) -> Result<super::super::foundation::HResult> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_ExtendedError)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
}
RT_CLASS!{class StoreSendRequestResult: IStoreSendRequestResult}
DEFINE_IID!(IID_IStoreSendRequestResult2, 687941999, 49328, 18896, 142, 141, 170, 148, 10, 249, 193, 11);
RT_INTERFACE!{interface IStoreSendRequestResult2(IStoreSendRequestResult2Vtbl): IInspectable(IInspectableVtbl) [IID_IStoreSendRequestResult2] {
    #[cfg(feature="windows-web")] fn get_HttpStatusCode(&self, out: *mut super::super::web::http::HttpStatusCode) -> HRESULT
}}
impl IStoreSendRequestResult2 {
    #[cfg(feature="windows-web")] #[inline] pub unsafe fn get_http_status_code(&self) -> Result<super::super::web::http::HttpStatusCode> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_HttpStatusCode)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
}
DEFINE_IID!(IID_IStoreSku, 964587349, 17472, 20227, 134, 60, 145, 243, 254, 200, 61, 121);
RT_INTERFACE!{interface IStoreSku(IStoreSkuVtbl): IInspectable(IInspectableVtbl) [IID_IStoreSku] {
    fn get_StoreId(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Language(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Title(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Description(&self, out: *mut HSTRING) -> HRESULT,
    fn get_IsTrial(&self, out: *mut bool) -> HRESULT,
    fn get_CustomDeveloperData(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Images(&self, out: *mut *mut super::super::foundation::collections::IVectorView<StoreImage>) -> HRESULT,
    fn get_Videos(&self, out: *mut *mut super::super::foundation::collections::IVectorView<StoreVideo>) -> HRESULT,
    fn get_Availabilities(&self, out: *mut *mut super::super::foundation::collections::IVectorView<StoreAvailability>) -> HRESULT,
    fn get_Price(&self, out: *mut *mut StorePrice) -> HRESULT,
    fn get_ExtendedJsonData(&self, out: *mut HSTRING) -> HRESULT,
    fn get_IsInUserCollection(&self, out: *mut bool) -> HRESULT,
    fn get_BundledSkus(&self, out: *mut *mut super::super::foundation::collections::IVectorView<HString>) -> HRESULT,
    fn get_CollectionData(&self, out: *mut *mut StoreCollectionData) -> HRESULT,
    fn GetIsInstalledAsync(&self, out: *mut *mut super::super::foundation::IAsyncOperation<bool>) -> HRESULT,
    fn RequestPurchaseAsync(&self, out: *mut *mut super::super::foundation::IAsyncOperation<StorePurchaseResult>) -> HRESULT,
    fn RequestPurchaseWithPurchasePropertiesAsync(&self, storePurchaseProperties: *mut StorePurchaseProperties, out: *mut *mut super::super::foundation::IAsyncOperation<StorePurchaseResult>) -> HRESULT,
    fn get_IsSubscription(&self, out: *mut bool) -> HRESULT,
    fn get_SubscriptionInfo(&self, out: *mut *mut StoreSubscriptionInfo) -> HRESULT
}}
impl IStoreSku {
    #[inline] pub unsafe fn get_store_id(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_StoreId)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_language(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Language)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_title(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Title)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_description(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Description)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_is_trial(&self) -> Result<bool> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_IsTrial)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_custom_developer_data(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_CustomDeveloperData)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_images(&self) -> Result<ComPtr<super::super::foundation::collections::IVectorView<StoreImage>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Images)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_videos(&self) -> Result<ComPtr<super::super::foundation::collections::IVectorView<StoreVideo>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Videos)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_availabilities(&self) -> Result<ComPtr<super::super::foundation::collections::IVectorView<StoreAvailability>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Availabilities)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_price(&self) -> Result<ComPtr<StorePrice>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Price)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_extended_json_data(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_ExtendedJsonData)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_is_in_user_collection(&self) -> Result<bool> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_IsInUserCollection)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_bundled_skus(&self) -> Result<ComPtr<super::super::foundation::collections::IVectorView<HString>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_BundledSkus)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_collection_data(&self) -> Result<ComPtr<StoreCollectionData>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_CollectionData)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_is_installed_async(&self) -> Result<ComPtr<super::super::foundation::IAsyncOperation<bool>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetIsInstalledAsync)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn request_purchase_async(&self) -> Result<ComPtr<super::super::foundation::IAsyncOperation<StorePurchaseResult>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).RequestPurchaseAsync)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn request_purchase_with_purchase_properties_async(&self, storePurchaseProperties: &StorePurchaseProperties) -> Result<ComPtr<super::super::foundation::IAsyncOperation<StorePurchaseResult>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).RequestPurchaseWithPurchasePropertiesAsync)(self as *const _ as *mut _, storePurchaseProperties as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_is_subscription(&self) -> Result<bool> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_IsSubscription)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_subscription_info(&self) -> Result<ComPtr<StoreSubscriptionInfo>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_SubscriptionInfo)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
RT_CLASS!{class StoreSku: IStoreSku}
DEFINE_IID!(IID_IStoreSubscriptionInfo, 1099528042, 1369, 17324, 169, 198, 58, 176, 1, 31, 184, 235);
RT_INTERFACE!{interface IStoreSubscriptionInfo(IStoreSubscriptionInfoVtbl): IInspectable(IInspectableVtbl) [IID_IStoreSubscriptionInfo] {
    fn get_BillingPeriod(&self, out: *mut u32) -> HRESULT,
    fn get_BillingPeriodUnit(&self, out: *mut StoreDurationUnit) -> HRESULT,
    fn get_HasTrialPeriod(&self, out: *mut bool) -> HRESULT,
    fn get_TrialPeriod(&self, out: *mut u32) -> HRESULT,
    fn get_TrialPeriodUnit(&self, out: *mut StoreDurationUnit) -> HRESULT
}}
impl IStoreSubscriptionInfo {
    #[inline] pub unsafe fn get_billing_period(&self) -> Result<u32> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_BillingPeriod)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_billing_period_unit(&self) -> Result<StoreDurationUnit> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_BillingPeriodUnit)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_has_trial_period(&self) -> Result<bool> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_HasTrialPeriod)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_trial_period(&self) -> Result<u32> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_TrialPeriod)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_trial_period_unit(&self) -> Result<StoreDurationUnit> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_TrialPeriodUnit)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
}
RT_CLASS!{class StoreSubscriptionInfo: IStoreSubscriptionInfo}
DEFINE_IID!(IID_IStoreVideo, 4067209604, 28510, 19906, 136, 108, 60, 99, 8, 60, 47, 148);
RT_INTERFACE!{interface IStoreVideo(IStoreVideoVtbl): IInspectable(IInspectableVtbl) [IID_IStoreVideo] {
    fn get_Uri(&self, out: *mut *mut super::super::foundation::Uri) -> HRESULT,
    fn get_VideoPurposeTag(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Width(&self, out: *mut u32) -> HRESULT,
    fn get_Height(&self, out: *mut u32) -> HRESULT,
    fn get_Caption(&self, out: *mut HSTRING) -> HRESULT,
    fn get_PreviewImage(&self, out: *mut *mut StoreImage) -> HRESULT
}}
impl IStoreVideo {
    #[inline] pub unsafe fn get_uri(&self) -> Result<ComPtr<super::super::foundation::Uri>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Uri)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_video_purpose_tag(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_VideoPurposeTag)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
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
    #[inline] pub unsafe fn get_caption(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Caption)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_preview_image(&self) -> Result<ComPtr<StoreImage>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_PreviewImage)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
RT_CLASS!{class StoreVideo: IStoreVideo}
} // Windows.Services.Store
pub mod targetedcontent { // Windows.Services.TargetedContent
use ::prelude::*;
DEFINE_IID!(IID_ITargetedContentAction, 3613092126, 27862, 19616, 157, 143, 71, 40, 176, 183, 230, 182);
RT_INTERFACE!{interface ITargetedContentAction(ITargetedContentActionVtbl): IInspectable(IInspectableVtbl) [IID_ITargetedContentAction] {
    fn InvokeAsync(&self, out: *mut *mut super::super::foundation::IAsyncAction) -> HRESULT
}}
impl ITargetedContentAction {
    #[inline] pub unsafe fn invoke_async(&self) -> Result<ComPtr<super::super::foundation::IAsyncAction>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).InvokeAsync)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
RT_CLASS!{class TargetedContentAction: ITargetedContentAction}
RT_ENUM! { enum TargetedContentAppInstallationState: i32 {
    NotApplicable (TargetedContentAppInstallationState_NotApplicable) = 0, NotInstalled (TargetedContentAppInstallationState_NotInstalled) = 1, Installed (TargetedContentAppInstallationState_Installed) = 2,
}}
RT_ENUM! { enum TargetedContentAvailability: i32 {
    None (TargetedContentAvailability_None) = 0, Partial (TargetedContentAvailability_Partial) = 1, All (TargetedContentAvailability_All) = 2,
}}
DEFINE_IID!(IID_ITargetedContentAvailabilityChangedEventArgs, 3774192934, 22823, 17488, 150, 92, 28, 235, 123, 236, 222, 101);
RT_INTERFACE!{interface ITargetedContentAvailabilityChangedEventArgs(ITargetedContentAvailabilityChangedEventArgsVtbl): IInspectable(IInspectableVtbl) [IID_ITargetedContentAvailabilityChangedEventArgs] {
    fn GetDeferral(&self, out: *mut *mut super::super::foundation::Deferral) -> HRESULT
}}
impl ITargetedContentAvailabilityChangedEventArgs {
    #[inline] pub unsafe fn get_deferral(&self) -> Result<ComPtr<super::super::foundation::Deferral>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetDeferral)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
RT_CLASS!{class TargetedContentAvailabilityChangedEventArgs: ITargetedContentAvailabilityChangedEventArgs}
DEFINE_IID!(IID_ITargetedContentChangedEventArgs, 2580842697, 22654, 17798, 142, 247, 181, 76, 169, 69, 58, 22);
RT_INTERFACE!{interface ITargetedContentChangedEventArgs(ITargetedContentChangedEventArgsVtbl): IInspectable(IInspectableVtbl) [IID_ITargetedContentChangedEventArgs] {
    fn GetDeferral(&self, out: *mut *mut super::super::foundation::Deferral) -> HRESULT,
    fn get_HasPreviousContentExpired(&self, out: *mut bool) -> HRESULT
}}
impl ITargetedContentChangedEventArgs {
    #[inline] pub unsafe fn get_deferral(&self) -> Result<ComPtr<super::super::foundation::Deferral>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetDeferral)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_has_previous_content_expired(&self) -> Result<bool> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_HasPreviousContentExpired)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
}
RT_CLASS!{class TargetedContentChangedEventArgs: ITargetedContentChangedEventArgs}
DEFINE_IID!(IID_ITargetedContentCollection, 759916229, 61795, 17594, 159, 110, 225, 164, 194, 187, 85, 157);
RT_INTERFACE!{interface ITargetedContentCollection(ITargetedContentCollectionVtbl): IInspectable(IInspectableVtbl) [IID_ITargetedContentCollection] {
    fn get_Id(&self, out: *mut HSTRING) -> HRESULT,
    fn ReportInteraction(&self, interaction: TargetedContentInteraction) -> HRESULT,
    fn ReportCustomInteraction(&self, customInteractionName: HSTRING) -> HRESULT,
    fn get_Path(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Properties(&self, out: *mut *mut super::super::foundation::collections::IMapView<HString, TargetedContentValue>) -> HRESULT,
    fn get_Collections(&self, out: *mut *mut super::super::foundation::collections::IVectorView<TargetedContentCollection>) -> HRESULT,
    fn get_Items(&self, out: *mut *mut super::super::foundation::collections::IVectorView<TargetedContentItem>) -> HRESULT
}}
impl ITargetedContentCollection {
    #[inline] pub unsafe fn get_id(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Id)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn report_interaction(&self, interaction: TargetedContentInteraction) -> Result<()> {
        let hr = ((*self.lpVtbl).ReportInteraction)(self as *const _ as *mut _, interaction);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn report_custom_interaction(&self, customInteractionName: &HStringArg) -> Result<()> {
        let hr = ((*self.lpVtbl).ReportCustomInteraction)(self as *const _ as *mut _, customInteractionName.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_path(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Path)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_properties(&self) -> Result<ComPtr<super::super::foundation::collections::IMapView<HString, TargetedContentValue>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Properties)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_collections(&self) -> Result<ComPtr<super::super::foundation::collections::IVectorView<TargetedContentCollection>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Collections)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_items(&self) -> Result<ComPtr<super::super::foundation::collections::IVectorView<TargetedContentItem>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Items)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
RT_CLASS!{class TargetedContentCollection: ITargetedContentCollection}
DEFINE_IID!(IID_ITargetedContentContainer, 3156513993, 34871, 18370, 133, 15, 215, 157, 100, 89, 89, 38);
RT_INTERFACE!{interface ITargetedContentContainer(ITargetedContentContainerVtbl): IInspectable(IInspectableVtbl) [IID_ITargetedContentContainer] {
    fn get_Id(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Timestamp(&self, out: *mut super::super::foundation::DateTime) -> HRESULT,
    fn get_Availability(&self, out: *mut TargetedContentAvailability) -> HRESULT,
    fn get_Content(&self, out: *mut *mut TargetedContentCollection) -> HRESULT,
    fn SelectSingleObject(&self, path: HSTRING, out: *mut *mut TargetedContentObject) -> HRESULT
}}
impl ITargetedContentContainer {
    #[inline] pub unsafe fn get_id(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Id)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_timestamp(&self) -> Result<super::super::foundation::DateTime> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_Timestamp)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_availability(&self) -> Result<TargetedContentAvailability> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_Availability)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_content(&self) -> Result<ComPtr<TargetedContentCollection>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Content)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn select_single_object(&self, path: &HStringArg) -> Result<ComPtr<TargetedContentObject>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).SelectSingleObject)(self as *const _ as *mut _, path.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
RT_CLASS!{class TargetedContentContainer: ITargetedContentContainer}
impl RtActivatable<ITargetedContentContainerStatics> for TargetedContentContainer {}
impl TargetedContentContainer {
    #[inline] pub fn get_async(contentId: &HStringArg) -> Result<ComPtr<super::super::foundation::IAsyncOperation<TargetedContentContainer>>> { unsafe {
        <Self as RtActivatable<ITargetedContentContainerStatics>>::get_activation_factory().get_async(contentId)
    }}
}
DEFINE_CLSID!(TargetedContentContainer(&[87,105,110,100,111,119,115,46,83,101,114,118,105,99,101,115,46,84,97,114,103,101,116,101,100,67,111,110,116,101,110,116,46,84,97,114,103,101,116,101,100,67,111,110,116,101,110,116,67,111,110,116,97,105,110,101,114,0]) [CLSID_TargetedContentContainer]);
DEFINE_IID!(IID_ITargetedContentContainerStatics, 1531439099, 8512, 19487, 167, 54, 197, 149, 131, 242, 39, 216);
RT_INTERFACE!{static interface ITargetedContentContainerStatics(ITargetedContentContainerStaticsVtbl): IInspectable(IInspectableVtbl) [IID_ITargetedContentContainerStatics] {
    fn GetAsync(&self, contentId: HSTRING, out: *mut *mut super::super::foundation::IAsyncOperation<TargetedContentContainer>) -> HRESULT
}}
impl ITargetedContentContainerStatics {
    #[inline] pub unsafe fn get_async(&self, contentId: &HStringArg) -> Result<ComPtr<super::super::foundation::IAsyncOperation<TargetedContentContainer>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetAsync)(self as *const _ as *mut _, contentId.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
#[cfg(feature="windows-storage")] RT_CLASS!{class TargetedContentFile: super::super::storage::streams::IRandomAccessStreamReference}
#[cfg(not(feature="windows-storage"))] RT_CLASS!{class TargetedContentFile: IInspectable}
DEFINE_IID!(IID_ITargetedContentImage, 2812642777, 30623, 19230, 187, 177, 142, 175, 83, 251, 234, 178);
RT_INTERFACE!{interface ITargetedContentImage(ITargetedContentImageVtbl): IInspectable(IInspectableVtbl) [IID_ITargetedContentImage] {
    fn get_Height(&self, out: *mut u32) -> HRESULT,
    fn get_Width(&self, out: *mut u32) -> HRESULT
}}
impl ITargetedContentImage {
    #[inline] pub unsafe fn get_height(&self) -> Result<u32> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_Height)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_width(&self) -> Result<u32> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_Width)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
}
RT_CLASS!{class TargetedContentImage: ITargetedContentImage}
RT_ENUM! { enum TargetedContentInteraction: i32 {
    Impression (TargetedContentInteraction_Impression) = 0, ClickThrough (TargetedContentInteraction_ClickThrough) = 1, Hover (TargetedContentInteraction_Hover) = 2, Like (TargetedContentInteraction_Like) = 3, Dislike (TargetedContentInteraction_Dislike) = 4, Dismiss (TargetedContentInteraction_Dismiss) = 5, Ineligible (TargetedContentInteraction_Ineligible) = 6, Accept (TargetedContentInteraction_Accept) = 7, Decline (TargetedContentInteraction_Decline) = 8, Defer (TargetedContentInteraction_Defer) = 9, Canceled (TargetedContentInteraction_Canceled) = 10, Conversion (TargetedContentInteraction_Conversion) = 11, Opportunity (TargetedContentInteraction_Opportunity) = 12,
}}
DEFINE_IID!(IID_ITargetedContentItem, 941002180, 10092, 19506, 150, 186, 86, 92, 110, 64, 110, 116);
RT_INTERFACE!{interface ITargetedContentItem(ITargetedContentItemVtbl): IInspectable(IInspectableVtbl) [IID_ITargetedContentItem] {
    fn get_Path(&self, out: *mut HSTRING) -> HRESULT,
    fn ReportInteraction(&self, interaction: TargetedContentInteraction) -> HRESULT,
    fn ReportCustomInteraction(&self, customInteractionName: HSTRING) -> HRESULT,
    fn get_State(&self, out: *mut *mut TargetedContentItemState) -> HRESULT,
    fn get_Properties(&self, out: *mut *mut super::super::foundation::collections::IMapView<HString, TargetedContentValue>) -> HRESULT,
    fn get_Collections(&self, out: *mut *mut super::super::foundation::collections::IVectorView<TargetedContentCollection>) -> HRESULT
}}
impl ITargetedContentItem {
    #[inline] pub unsafe fn get_path(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Path)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn report_interaction(&self, interaction: TargetedContentInteraction) -> Result<()> {
        let hr = ((*self.lpVtbl).ReportInteraction)(self as *const _ as *mut _, interaction);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn report_custom_interaction(&self, customInteractionName: &HStringArg) -> Result<()> {
        let hr = ((*self.lpVtbl).ReportCustomInteraction)(self as *const _ as *mut _, customInteractionName.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_state(&self) -> Result<ComPtr<TargetedContentItemState>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_State)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_properties(&self) -> Result<ComPtr<super::super::foundation::collections::IMapView<HString, TargetedContentValue>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Properties)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_collections(&self) -> Result<ComPtr<super::super::foundation::collections::IVectorView<TargetedContentCollection>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Collections)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
RT_CLASS!{class TargetedContentItem: ITargetedContentItem}
DEFINE_IID!(IID_ITargetedContentItemState, 1939035220, 19557, 19271, 164, 65, 71, 45, 229, 60, 121, 182);
RT_INTERFACE!{interface ITargetedContentItemState(ITargetedContentItemStateVtbl): IInspectable(IInspectableVtbl) [IID_ITargetedContentItemState] {
    fn get_ShouldDisplay(&self, out: *mut bool) -> HRESULT,
    fn get_AppInstallationState(&self, out: *mut TargetedContentAppInstallationState) -> HRESULT
}}
impl ITargetedContentItemState {
    #[inline] pub unsafe fn get_should_display(&self) -> Result<bool> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_ShouldDisplay)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_app_installation_state(&self) -> Result<TargetedContentAppInstallationState> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_AppInstallationState)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
}
RT_CLASS!{class TargetedContentItemState: ITargetedContentItemState}
DEFINE_IID!(IID_ITargetedContentObject, 69040489, 8722, 17105, 157, 250, 136, 168, 227, 3, 58, 163);
RT_INTERFACE!{interface ITargetedContentObject(ITargetedContentObjectVtbl): IInspectable(IInspectableVtbl) [IID_ITargetedContentObject] {
    fn get_ObjectKind(&self, out: *mut TargetedContentObjectKind) -> HRESULT,
    fn get_Collection(&self, out: *mut *mut TargetedContentCollection) -> HRESULT,
    fn get_Item(&self, out: *mut *mut TargetedContentItem) -> HRESULT,
    fn get_Value(&self, out: *mut *mut TargetedContentValue) -> HRESULT
}}
impl ITargetedContentObject {
    #[inline] pub unsafe fn get_object_kind(&self) -> Result<TargetedContentObjectKind> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_ObjectKind)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_collection(&self) -> Result<ComPtr<TargetedContentCollection>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Collection)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_item(&self) -> Result<ComPtr<TargetedContentItem>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Item)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_value(&self) -> Result<ComPtr<TargetedContentValue>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Value)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
RT_CLASS!{class TargetedContentObject: ITargetedContentObject}
RT_ENUM! { enum TargetedContentObjectKind: i32 {
    Collection (TargetedContentObjectKind_Collection) = 0, Item (TargetedContentObjectKind_Item) = 1, Value (TargetedContentObjectKind_Value) = 2,
}}
DEFINE_IID!(IID_ITargetedContentStateChangedEventArgs, 2585587517, 32883, 17430, 141, 242, 84, 104, 53, 166, 65, 79);
RT_INTERFACE!{interface ITargetedContentStateChangedEventArgs(ITargetedContentStateChangedEventArgsVtbl): IInspectable(IInspectableVtbl) [IID_ITargetedContentStateChangedEventArgs] {
    fn GetDeferral(&self, out: *mut *mut super::super::foundation::Deferral) -> HRESULT
}}
impl ITargetedContentStateChangedEventArgs {
    #[inline] pub unsafe fn get_deferral(&self) -> Result<ComPtr<super::super::foundation::Deferral>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetDeferral)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
RT_CLASS!{class TargetedContentStateChangedEventArgs: ITargetedContentStateChangedEventArgs}
DEFINE_IID!(IID_ITargetedContentSubscription, 2284596297, 50770, 19578, 172, 173, 31, 127, 162, 152, 108, 115);
RT_INTERFACE!{interface ITargetedContentSubscription(ITargetedContentSubscriptionVtbl): IInspectable(IInspectableVtbl) [IID_ITargetedContentSubscription] {
    fn get_Id(&self, out: *mut HSTRING) -> HRESULT,
    fn GetContentContainerAsync(&self, out: *mut *mut super::super::foundation::IAsyncOperation<TargetedContentContainer>) -> HRESULT,
    fn add_ContentChanged(&self, handler: *mut super::super::foundation::TypedEventHandler<TargetedContentSubscription, TargetedContentChangedEventArgs>, out: *mut super::super::foundation::EventRegistrationToken) -> HRESULT,
    fn remove_ContentChanged(&self, cookie: super::super::foundation::EventRegistrationToken) -> HRESULT,
    fn add_AvailabilityChanged(&self, handler: *mut super::super::foundation::TypedEventHandler<TargetedContentSubscription, TargetedContentAvailabilityChangedEventArgs>, out: *mut super::super::foundation::EventRegistrationToken) -> HRESULT,
    fn remove_AvailabilityChanged(&self, cookie: super::super::foundation::EventRegistrationToken) -> HRESULT,
    fn add_StateChanged(&self, handler: *mut super::super::foundation::TypedEventHandler<TargetedContentSubscription, TargetedContentStateChangedEventArgs>, out: *mut super::super::foundation::EventRegistrationToken) -> HRESULT,
    fn remove_StateChanged(&self, cookie: super::super::foundation::EventRegistrationToken) -> HRESULT
}}
impl ITargetedContentSubscription {
    #[inline] pub unsafe fn get_id(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Id)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_content_container_async(&self) -> Result<ComPtr<super::super::foundation::IAsyncOperation<TargetedContentContainer>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetContentContainerAsync)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn add_content_changed(&self, handler: &super::super::foundation::TypedEventHandler<TargetedContentSubscription, TargetedContentChangedEventArgs>) -> Result<super::super::foundation::EventRegistrationToken> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).add_ContentChanged)(self as *const _ as *mut _, handler as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn remove_content_changed(&self, cookie: super::super::foundation::EventRegistrationToken) -> Result<()> {
        let hr = ((*self.lpVtbl).remove_ContentChanged)(self as *const _ as *mut _, cookie);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn add_availability_changed(&self, handler: &super::super::foundation::TypedEventHandler<TargetedContentSubscription, TargetedContentAvailabilityChangedEventArgs>) -> Result<super::super::foundation::EventRegistrationToken> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).add_AvailabilityChanged)(self as *const _ as *mut _, handler as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn remove_availability_changed(&self, cookie: super::super::foundation::EventRegistrationToken) -> Result<()> {
        let hr = ((*self.lpVtbl).remove_AvailabilityChanged)(self as *const _ as *mut _, cookie);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn add_state_changed(&self, handler: &super::super::foundation::TypedEventHandler<TargetedContentSubscription, TargetedContentStateChangedEventArgs>) -> Result<super::super::foundation::EventRegistrationToken> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).add_StateChanged)(self as *const _ as *mut _, handler as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn remove_state_changed(&self, cookie: super::super::foundation::EventRegistrationToken) -> Result<()> {
        let hr = ((*self.lpVtbl).remove_StateChanged)(self as *const _ as *mut _, cookie);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
}
RT_CLASS!{class TargetedContentSubscription: ITargetedContentSubscription}
impl RtActivatable<ITargetedContentSubscriptionStatics> for TargetedContentSubscription {}
impl TargetedContentSubscription {
    #[inline] pub fn get_async(subscriptionId: &HStringArg) -> Result<ComPtr<super::super::foundation::IAsyncOperation<TargetedContentSubscription>>> { unsafe {
        <Self as RtActivatable<ITargetedContentSubscriptionStatics>>::get_activation_factory().get_async(subscriptionId)
    }}
    #[inline] pub fn get_options(subscriptionId: &HStringArg) -> Result<ComPtr<TargetedContentSubscriptionOptions>> { unsafe {
        <Self as RtActivatable<ITargetedContentSubscriptionStatics>>::get_activation_factory().get_options(subscriptionId)
    }}
}
DEFINE_CLSID!(TargetedContentSubscription(&[87,105,110,100,111,119,115,46,83,101,114,118,105,99,101,115,46,84,97,114,103,101,116,101,100,67,111,110,116,101,110,116,46,84,97,114,103,101,116,101,100,67,111,110,116,101,110,116,83,117,98,115,99,114,105,112,116,105,111,110,0]) [CLSID_TargetedContentSubscription]);
DEFINE_IID!(IID_ITargetedContentSubscriptionOptions, 1643014864, 11395, 16923, 132, 103, 65, 62, 175, 26, 235, 151);
RT_INTERFACE!{interface ITargetedContentSubscriptionOptions(ITargetedContentSubscriptionOptionsVtbl): IInspectable(IInspectableVtbl) [IID_ITargetedContentSubscriptionOptions] {
    fn get_SubscriptionId(&self, out: *mut HSTRING) -> HRESULT,
    fn get_AllowPartialContentAvailability(&self, out: *mut bool) -> HRESULT,
    fn put_AllowPartialContentAvailability(&self, value: bool) -> HRESULT,
    fn get_CloudQueryParameters(&self, out: *mut *mut super::super::foundation::collections::IMap<HString, HString>) -> HRESULT,
    fn get_LocalFilters(&self, out: *mut *mut super::super::foundation::collections::IVector<HString>) -> HRESULT,
    fn Update(&self) -> HRESULT
}}
impl ITargetedContentSubscriptionOptions {
    #[inline] pub unsafe fn get_subscription_id(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_SubscriptionId)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_allow_partial_content_availability(&self) -> Result<bool> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_AllowPartialContentAvailability)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_allow_partial_content_availability(&self, value: bool) -> Result<()> {
        let hr = ((*self.lpVtbl).put_AllowPartialContentAvailability)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_cloud_query_parameters(&self) -> Result<ComPtr<super::super::foundation::collections::IMap<HString, HString>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_CloudQueryParameters)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_local_filters(&self) -> Result<ComPtr<super::super::foundation::collections::IVector<HString>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_LocalFilters)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn update(&self) -> Result<()> {
        let hr = ((*self.lpVtbl).Update)(self as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
}
RT_CLASS!{class TargetedContentSubscriptionOptions: ITargetedContentSubscriptionOptions}
DEFINE_IID!(IID_ITargetedContentSubscriptionStatics, 4208852608, 13837, 18710, 181, 60, 126, 162, 112, 144, 208, 42);
RT_INTERFACE!{static interface ITargetedContentSubscriptionStatics(ITargetedContentSubscriptionStaticsVtbl): IInspectable(IInspectableVtbl) [IID_ITargetedContentSubscriptionStatics] {
    fn GetAsync(&self, subscriptionId: HSTRING, out: *mut *mut super::super::foundation::IAsyncOperation<TargetedContentSubscription>) -> HRESULT,
    fn GetOptions(&self, subscriptionId: HSTRING, out: *mut *mut TargetedContentSubscriptionOptions) -> HRESULT
}}
impl ITargetedContentSubscriptionStatics {
    #[inline] pub unsafe fn get_async(&self, subscriptionId: &HStringArg) -> Result<ComPtr<super::super::foundation::IAsyncOperation<TargetedContentSubscription>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetAsync)(self as *const _ as *mut _, subscriptionId.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_options(&self, subscriptionId: &HStringArg) -> Result<ComPtr<TargetedContentSubscriptionOptions>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetOptions)(self as *const _ as *mut _, subscriptionId.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
DEFINE_IID!(IID_ITargetedContentValue, 2868765875, 16917, 19448, 134, 127, 67, 240, 72, 101, 249, 191);
RT_INTERFACE!{interface ITargetedContentValue(ITargetedContentValueVtbl): IInspectable(IInspectableVtbl) [IID_ITargetedContentValue] {
    fn get_ValueKind(&self, out: *mut TargetedContentValueKind) -> HRESULT,
    fn get_Path(&self, out: *mut HSTRING) -> HRESULT,
    fn get_String(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Uri(&self, out: *mut *mut super::super::foundation::Uri) -> HRESULT,
    fn get_Number(&self, out: *mut f64) -> HRESULT,
    fn get_Boolean(&self, out: *mut bool) -> HRESULT,
    fn get_File(&self, out: *mut *mut TargetedContentFile) -> HRESULT,
    fn get_ImageFile(&self, out: *mut *mut TargetedContentImage) -> HRESULT,
    fn get_Action(&self, out: *mut *mut TargetedContentAction) -> HRESULT,
    fn get_Strings(&self, out: *mut *mut super::super::foundation::collections::IVectorView<HString>) -> HRESULT,
    fn get_Uris(&self, out: *mut *mut super::super::foundation::collections::IVectorView<super::super::foundation::Uri>) -> HRESULT,
    fn get_Numbers(&self, out: *mut *mut super::super::foundation::collections::IVectorView<f64>) -> HRESULT,
    fn get_Booleans(&self, out: *mut *mut super::super::foundation::collections::IVectorView<bool>) -> HRESULT,
    fn get_Files(&self, out: *mut *mut super::super::foundation::collections::IVectorView<TargetedContentFile>) -> HRESULT,
    fn get_ImageFiles(&self, out: *mut *mut super::super::foundation::collections::IVectorView<TargetedContentImage>) -> HRESULT,
    fn get_Actions(&self, out: *mut *mut super::super::foundation::collections::IVectorView<TargetedContentAction>) -> HRESULT
}}
impl ITargetedContentValue {
    #[inline] pub unsafe fn get_value_kind(&self) -> Result<TargetedContentValueKind> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_ValueKind)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_path(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Path)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_string(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_String)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_uri(&self) -> Result<ComPtr<super::super::foundation::Uri>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Uri)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_number(&self) -> Result<f64> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_Number)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_boolean(&self) -> Result<bool> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_Boolean)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_file(&self) -> Result<ComPtr<TargetedContentFile>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_File)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_image_file(&self) -> Result<ComPtr<TargetedContentImage>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_ImageFile)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_action(&self) -> Result<ComPtr<TargetedContentAction>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Action)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_strings(&self) -> Result<ComPtr<super::super::foundation::collections::IVectorView<HString>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Strings)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_uris(&self) -> Result<ComPtr<super::super::foundation::collections::IVectorView<super::super::foundation::Uri>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Uris)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_numbers(&self) -> Result<ComPtr<super::super::foundation::collections::IVectorView<f64>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Numbers)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_booleans(&self) -> Result<ComPtr<super::super::foundation::collections::IVectorView<bool>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Booleans)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_files(&self) -> Result<ComPtr<super::super::foundation::collections::IVectorView<TargetedContentFile>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Files)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_image_files(&self) -> Result<ComPtr<super::super::foundation::collections::IVectorView<TargetedContentImage>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_ImageFiles)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_actions(&self) -> Result<ComPtr<super::super::foundation::collections::IVectorView<TargetedContentAction>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Actions)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
RT_CLASS!{class TargetedContentValue: ITargetedContentValue}
RT_ENUM! { enum TargetedContentValueKind: i32 {
    String (TargetedContentValueKind_String) = 0, Uri (TargetedContentValueKind_Uri) = 1, Number (TargetedContentValueKind_Number) = 2, Boolean (TargetedContentValueKind_Boolean) = 3, File (TargetedContentValueKind_File) = 4, ImageFile (TargetedContentValueKind_ImageFile) = 5, Action (TargetedContentValueKind_Action) = 6, Strings (TargetedContentValueKind_Strings) = 7, Uris (TargetedContentValueKind_Uris) = 8, Numbers (TargetedContentValueKind_Numbers) = 9, Booleans (TargetedContentValueKind_Booleans) = 10, Files (TargetedContentValueKind_Files) = 11, ImageFiles (TargetedContentValueKind_ImageFiles) = 12, Actions (TargetedContentValueKind_Actions) = 13,
}}
} // Windows.Services.TargetedContent
