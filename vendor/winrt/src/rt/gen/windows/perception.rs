use ::prelude::*;
DEFINE_IID!(IID_IPerceptionTimestamp, 2277656580, 41518, 19163, 186, 38, 215, 142, 246, 57, 188, 244);
RT_INTERFACE!{interface IPerceptionTimestamp(IPerceptionTimestampVtbl): IInspectable(IInspectableVtbl) [IID_IPerceptionTimestamp] {
    fn get_TargetTime(&self, out: *mut super::foundation::DateTime) -> HRESULT,
    fn get_PredictionAmount(&self, out: *mut super::foundation::TimeSpan) -> HRESULT
}}
impl IPerceptionTimestamp {
    #[inline] pub unsafe fn get_target_time(&self) -> Result<super::foundation::DateTime> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_TargetTime)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_prediction_amount(&self) -> Result<super::foundation::TimeSpan> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_PredictionAmount)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
}
RT_CLASS!{class PerceptionTimestamp: IPerceptionTimestamp}
RT_CLASS!{static class PerceptionTimestampHelper}
impl RtActivatable<IPerceptionTimestampHelperStatics> for PerceptionTimestampHelper {}
impl PerceptionTimestampHelper {
    #[inline] pub fn from_historical_target_time(targetTime: super::foundation::DateTime) -> Result<ComPtr<PerceptionTimestamp>> { unsafe {
        <Self as RtActivatable<IPerceptionTimestampHelperStatics>>::get_activation_factory().from_historical_target_time(targetTime)
    }}
}
DEFINE_CLSID!(PerceptionTimestampHelper(&[87,105,110,100,111,119,115,46,80,101,114,99,101,112,116,105,111,110,46,80,101,114,99,101,112,116,105,111,110,84,105,109,101,115,116,97,109,112,72,101,108,112,101,114,0]) [CLSID_PerceptionTimestampHelper]);
DEFINE_IID!(IID_IPerceptionTimestampHelperStatics, 1202065876, 43487, 20188, 133, 93, 244, 211, 57, 217, 103, 172);
RT_INTERFACE!{static interface IPerceptionTimestampHelperStatics(IPerceptionTimestampHelperStaticsVtbl): IInspectable(IInspectableVtbl) [IID_IPerceptionTimestampHelperStatics] {
    fn FromHistoricalTargetTime(&self, targetTime: super::foundation::DateTime, out: *mut *mut PerceptionTimestamp) -> HRESULT
}}
impl IPerceptionTimestampHelperStatics {
    #[inline] pub unsafe fn from_historical_target_time(&self, targetTime: super::foundation::DateTime) -> Result<ComPtr<PerceptionTimestamp>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).FromHistoricalTargetTime)(self as *const _ as *mut _, targetTime, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
pub mod spatial { // Windows.Perception.Spatial
use ::prelude::*;
DEFINE_IID!(IID_ISpatialAnchor, 86631886, 7476, 14082, 188, 236, 234, 191, 245, 120, 168, 105);
RT_INTERFACE!{interface ISpatialAnchor(ISpatialAnchorVtbl): IInspectable(IInspectableVtbl) [IID_ISpatialAnchor] {
    fn get_CoordinateSystem(&self, out: *mut *mut SpatialCoordinateSystem) -> HRESULT,
    fn get_RawCoordinateSystem(&self, out: *mut *mut SpatialCoordinateSystem) -> HRESULT,
    fn add_RawCoordinateSystemAdjusted(&self, handler: *mut super::super::foundation::TypedEventHandler<SpatialAnchor, SpatialAnchorRawCoordinateSystemAdjustedEventArgs>, out: *mut super::super::foundation::EventRegistrationToken) -> HRESULT,
    fn remove_RawCoordinateSystemAdjusted(&self, cookie: super::super::foundation::EventRegistrationToken) -> HRESULT
}}
impl ISpatialAnchor {
    #[inline] pub unsafe fn get_coordinate_system(&self) -> Result<ComPtr<SpatialCoordinateSystem>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_CoordinateSystem)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_raw_coordinate_system(&self) -> Result<ComPtr<SpatialCoordinateSystem>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_RawCoordinateSystem)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn add_raw_coordinate_system_adjusted(&self, handler: &super::super::foundation::TypedEventHandler<SpatialAnchor, SpatialAnchorRawCoordinateSystemAdjustedEventArgs>) -> Result<super::super::foundation::EventRegistrationToken> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).add_RawCoordinateSystemAdjusted)(self as *const _ as *mut _, handler as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn remove_raw_coordinate_system_adjusted(&self, cookie: super::super::foundation::EventRegistrationToken) -> Result<()> {
        let hr = ((*self.lpVtbl).remove_RawCoordinateSystemAdjusted)(self as *const _ as *mut _, cookie);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
}
RT_CLASS!{class SpatialAnchor: ISpatialAnchor}
impl RtActivatable<ISpatialAnchorStatics> for SpatialAnchor {}
impl SpatialAnchor {
    #[inline] pub fn try_create_relative_to(coordinateSystem: &SpatialCoordinateSystem) -> Result<ComPtr<SpatialAnchor>> { unsafe {
        <Self as RtActivatable<ISpatialAnchorStatics>>::get_activation_factory().try_create_relative_to(coordinateSystem)
    }}
    #[inline] pub fn try_create_with_position_relative_to(coordinateSystem: &SpatialCoordinateSystem, position: super::super::foundation::numerics::Vector3) -> Result<ComPtr<SpatialAnchor>> { unsafe {
        <Self as RtActivatable<ISpatialAnchorStatics>>::get_activation_factory().try_create_with_position_relative_to(coordinateSystem, position)
    }}
    #[inline] pub fn try_create_with_position_and_orientation_relative_to(coordinateSystem: &SpatialCoordinateSystem, position: super::super::foundation::numerics::Vector3, orientation: super::super::foundation::numerics::Quaternion) -> Result<ComPtr<SpatialAnchor>> { unsafe {
        <Self as RtActivatable<ISpatialAnchorStatics>>::get_activation_factory().try_create_with_position_and_orientation_relative_to(coordinateSystem, position, orientation)
    }}
}
DEFINE_CLSID!(SpatialAnchor(&[87,105,110,100,111,119,115,46,80,101,114,99,101,112,116,105,111,110,46,83,112,97,116,105,97,108,46,83,112,97,116,105,97,108,65,110,99,104,111,114,0]) [CLSID_SpatialAnchor]);
DEFINE_IID!(IID_ISpatialAnchor2, 3977758984, 42645, 19702, 146, 253, 151, 38, 59, 167, 16, 71);
RT_INTERFACE!{interface ISpatialAnchor2(ISpatialAnchor2Vtbl): IInspectable(IInspectableVtbl) [IID_ISpatialAnchor2] {
    fn get_RemovedByUser(&self, out: *mut bool) -> HRESULT
}}
impl ISpatialAnchor2 {
    #[inline] pub unsafe fn get_removed_by_user(&self) -> Result<bool> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_RemovedByUser)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
}
RT_CLASS!{static class SpatialAnchorManager}
impl RtActivatable<ISpatialAnchorManagerStatics> for SpatialAnchorManager {}
impl SpatialAnchorManager {
    #[inline] pub fn request_store_async() -> Result<ComPtr<super::super::foundation::IAsyncOperation<SpatialAnchorStore>>> { unsafe {
        <Self as RtActivatable<ISpatialAnchorManagerStatics>>::get_activation_factory().request_store_async()
    }}
}
DEFINE_CLSID!(SpatialAnchorManager(&[87,105,110,100,111,119,115,46,80,101,114,99,101,112,116,105,111,110,46,83,112,97,116,105,97,108,46,83,112,97,116,105,97,108,65,110,99,104,111,114,77,97,110,97,103,101,114,0]) [CLSID_SpatialAnchorManager]);
DEFINE_IID!(IID_ISpatialAnchorManagerStatics, 2296581803, 62391, 16907, 176, 134, 138, 128, 192, 125, 145, 13);
RT_INTERFACE!{static interface ISpatialAnchorManagerStatics(ISpatialAnchorManagerStaticsVtbl): IInspectable(IInspectableVtbl) [IID_ISpatialAnchorManagerStatics] {
    fn RequestStoreAsync(&self, out: *mut *mut super::super::foundation::IAsyncOperation<SpatialAnchorStore>) -> HRESULT
}}
impl ISpatialAnchorManagerStatics {
    #[inline] pub unsafe fn request_store_async(&self) -> Result<ComPtr<super::super::foundation::IAsyncOperation<SpatialAnchorStore>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).RequestStoreAsync)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
DEFINE_IID!(IID_ISpatialAnchorRawCoordinateSystemAdjustedEventArgs, 2716343992, 22215, 12567, 162, 228, 129, 224, 252, 242, 142, 0);
RT_INTERFACE!{interface ISpatialAnchorRawCoordinateSystemAdjustedEventArgs(ISpatialAnchorRawCoordinateSystemAdjustedEventArgsVtbl): IInspectable(IInspectableVtbl) [IID_ISpatialAnchorRawCoordinateSystemAdjustedEventArgs] {
    fn get_OldRawCoordinateSystemToNewRawCoordinateSystemTransform(&self, out: *mut super::super::foundation::numerics::Matrix4x4) -> HRESULT
}}
impl ISpatialAnchorRawCoordinateSystemAdjustedEventArgs {
    #[inline] pub unsafe fn get_old_raw_coordinate_system_to_new_raw_coordinate_system_transform(&self) -> Result<super::super::foundation::numerics::Matrix4x4> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_OldRawCoordinateSystemToNewRawCoordinateSystemTransform)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
}
RT_CLASS!{class SpatialAnchorRawCoordinateSystemAdjustedEventArgs: ISpatialAnchorRawCoordinateSystemAdjustedEventArgs}
DEFINE_IID!(IID_ISpatialAnchorStatics, 2844952130, 372, 12572, 174, 121, 14, 81, 7, 102, 159, 22);
RT_INTERFACE!{static interface ISpatialAnchorStatics(ISpatialAnchorStaticsVtbl): IInspectable(IInspectableVtbl) [IID_ISpatialAnchorStatics] {
    fn TryCreateRelativeTo(&self, coordinateSystem: *mut SpatialCoordinateSystem, out: *mut *mut SpatialAnchor) -> HRESULT,
    fn TryCreateWithPositionRelativeTo(&self, coordinateSystem: *mut SpatialCoordinateSystem, position: super::super::foundation::numerics::Vector3, out: *mut *mut SpatialAnchor) -> HRESULT,
    fn TryCreateWithPositionAndOrientationRelativeTo(&self, coordinateSystem: *mut SpatialCoordinateSystem, position: super::super::foundation::numerics::Vector3, orientation: super::super::foundation::numerics::Quaternion, out: *mut *mut SpatialAnchor) -> HRESULT
}}
impl ISpatialAnchorStatics {
    #[inline] pub unsafe fn try_create_relative_to(&self, coordinateSystem: &SpatialCoordinateSystem) -> Result<ComPtr<SpatialAnchor>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).TryCreateRelativeTo)(self as *const _ as *mut _, coordinateSystem as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn try_create_with_position_relative_to(&self, coordinateSystem: &SpatialCoordinateSystem, position: super::super::foundation::numerics::Vector3) -> Result<ComPtr<SpatialAnchor>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).TryCreateWithPositionRelativeTo)(self as *const _ as *mut _, coordinateSystem as *const _ as *mut _, position, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn try_create_with_position_and_orientation_relative_to(&self, coordinateSystem: &SpatialCoordinateSystem, position: super::super::foundation::numerics::Vector3, orientation: super::super::foundation::numerics::Quaternion) -> Result<ComPtr<SpatialAnchor>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).TryCreateWithPositionAndOrientationRelativeTo)(self as *const _ as *mut _, coordinateSystem as *const _ as *mut _, position, orientation, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
DEFINE_IID!(IID_ISpatialAnchorStore, 2965124662, 18538, 15536, 158, 111, 18, 69, 22, 92, 77, 182);
RT_INTERFACE!{interface ISpatialAnchorStore(ISpatialAnchorStoreVtbl): IInspectable(IInspectableVtbl) [IID_ISpatialAnchorStore] {
    fn GetAllSavedAnchors(&self, out: *mut *mut super::super::foundation::collections::IMapView<HString, SpatialAnchor>) -> HRESULT,
    fn TrySave(&self, id: HSTRING, anchor: *mut SpatialAnchor, out: *mut bool) -> HRESULT,
    fn Remove(&self, id: HSTRING) -> HRESULT,
    fn Clear(&self) -> HRESULT
}}
impl ISpatialAnchorStore {
    #[inline] pub unsafe fn get_all_saved_anchors(&self) -> Result<ComPtr<super::super::foundation::collections::IMapView<HString, SpatialAnchor>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetAllSavedAnchors)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn try_save(&self, id: &HStringArg, anchor: &SpatialAnchor) -> Result<bool> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).TrySave)(self as *const _ as *mut _, id.get(), anchor as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn remove(&self, id: &HStringArg) -> Result<()> {
        let hr = ((*self.lpVtbl).Remove)(self as *const _ as *mut _, id.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn clear(&self) -> Result<()> {
        let hr = ((*self.lpVtbl).Clear)(self as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
}
RT_CLASS!{class SpatialAnchorStore: ISpatialAnchorStore}
RT_CLASS!{static class SpatialAnchorTransferManager}
impl RtActivatable<ISpatialAnchorTransferManagerStatics> for SpatialAnchorTransferManager {}
impl SpatialAnchorTransferManager {
    #[cfg(feature="windows-storage")] #[inline] pub fn try_import_anchors_async(stream: &super::super::storage::streams::IInputStream) -> Result<ComPtr<super::super::foundation::IAsyncOperation<super::super::foundation::collections::IMapView<HString, SpatialAnchor>>>> { unsafe {
        <Self as RtActivatable<ISpatialAnchorTransferManagerStatics>>::get_activation_factory().try_import_anchors_async(stream)
    }}
    #[cfg(feature="windows-storage")] #[inline] pub fn try_export_anchors_async(anchors: &super::super::foundation::collections::IIterable<super::super::foundation::collections::IKeyValuePair<HString, SpatialAnchor>>, stream: &super::super::storage::streams::IOutputStream) -> Result<ComPtr<super::super::foundation::IAsyncOperation<bool>>> { unsafe {
        <Self as RtActivatable<ISpatialAnchorTransferManagerStatics>>::get_activation_factory().try_export_anchors_async(anchors, stream)
    }}
    #[inline] pub fn request_access_async() -> Result<ComPtr<super::super::foundation::IAsyncOperation<SpatialPerceptionAccessStatus>>> { unsafe {
        <Self as RtActivatable<ISpatialAnchorTransferManagerStatics>>::get_activation_factory().request_access_async()
    }}
}
DEFINE_CLSID!(SpatialAnchorTransferManager(&[87,105,110,100,111,119,115,46,80,101,114,99,101,112,116,105,111,110,46,83,112,97,116,105,97,108,46,83,112,97,116,105,97,108,65,110,99,104,111,114,84,114,97,110,115,102,101,114,77,97,110,97,103,101,114,0]) [CLSID_SpatialAnchorTransferManager]);
DEFINE_IID!(IID_ISpatialAnchorTransferManagerStatics, 62650809, 4824, 19406, 136, 53, 197, 223, 58, 192, 173, 171);
RT_INTERFACE!{static interface ISpatialAnchorTransferManagerStatics(ISpatialAnchorTransferManagerStaticsVtbl): IInspectable(IInspectableVtbl) [IID_ISpatialAnchorTransferManagerStatics] {
    #[cfg(not(feature="windows-storage"))] fn __Dummy0(&self) -> (),
    #[cfg(feature="windows-storage")] fn TryImportAnchorsAsync(&self, stream: *mut super::super::storage::streams::IInputStream, out: *mut *mut super::super::foundation::IAsyncOperation<super::super::foundation::collections::IMapView<HString, SpatialAnchor>>) -> HRESULT,
    #[cfg(not(feature="windows-storage"))] fn __Dummy1(&self) -> (),
    #[cfg(feature="windows-storage")] fn TryExportAnchorsAsync(&self, anchors: *mut super::super::foundation::collections::IIterable<super::super::foundation::collections::IKeyValuePair<HString, SpatialAnchor>>, stream: *mut super::super::storage::streams::IOutputStream, out: *mut *mut super::super::foundation::IAsyncOperation<bool>) -> HRESULT,
    fn RequestAccessAsync(&self, out: *mut *mut super::super::foundation::IAsyncOperation<SpatialPerceptionAccessStatus>) -> HRESULT
}}
impl ISpatialAnchorTransferManagerStatics {
    #[cfg(feature="windows-storage")] #[inline] pub unsafe fn try_import_anchors_async(&self, stream: &super::super::storage::streams::IInputStream) -> Result<ComPtr<super::super::foundation::IAsyncOperation<super::super::foundation::collections::IMapView<HString, SpatialAnchor>>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).TryImportAnchorsAsync)(self as *const _ as *mut _, stream as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[cfg(feature="windows-storage")] #[inline] pub unsafe fn try_export_anchors_async(&self, anchors: &super::super::foundation::collections::IIterable<super::super::foundation::collections::IKeyValuePair<HString, SpatialAnchor>>, stream: &super::super::storage::streams::IOutputStream) -> Result<ComPtr<super::super::foundation::IAsyncOperation<bool>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).TryExportAnchorsAsync)(self as *const _ as *mut _, anchors as *const _ as *mut _, stream as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn request_access_async(&self) -> Result<ComPtr<super::super::foundation::IAsyncOperation<SpatialPerceptionAccessStatus>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).RequestAccessAsync)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
RT_STRUCT! { struct SpatialBoundingBox {
    Center: super::super::foundation::numerics::Vector3, Extents: super::super::foundation::numerics::Vector3,
}}
RT_STRUCT! { struct SpatialBoundingFrustum {
    Near: super::super::foundation::numerics::Plane, Far: super::super::foundation::numerics::Plane, Right: super::super::foundation::numerics::Plane, Left: super::super::foundation::numerics::Plane, Top: super::super::foundation::numerics::Plane, Bottom: super::super::foundation::numerics::Plane,
}}
RT_STRUCT! { struct SpatialBoundingOrientedBox {
    Center: super::super::foundation::numerics::Vector3, Extents: super::super::foundation::numerics::Vector3, Orientation: super::super::foundation::numerics::Quaternion,
}}
RT_STRUCT! { struct SpatialBoundingSphere {
    Center: super::super::foundation::numerics::Vector3, Radius: f32,
}}
DEFINE_IID!(IID_ISpatialBoundingVolume, 4213204442, 26819, 13279, 183, 175, 76, 120, 114, 7, 153, 156);
RT_INTERFACE!{interface ISpatialBoundingVolume(ISpatialBoundingVolumeVtbl): IInspectable(IInspectableVtbl) [IID_ISpatialBoundingVolume] {
    
}}
RT_CLASS!{class SpatialBoundingVolume: ISpatialBoundingVolume}
impl RtActivatable<ISpatialBoundingVolumeStatics> for SpatialBoundingVolume {}
impl SpatialBoundingVolume {
    #[inline] pub fn from_box(coordinateSystem: &SpatialCoordinateSystem, box_: SpatialBoundingBox) -> Result<ComPtr<SpatialBoundingVolume>> { unsafe {
        <Self as RtActivatable<ISpatialBoundingVolumeStatics>>::get_activation_factory().from_box(coordinateSystem, box_)
    }}
    #[inline] pub fn from_oriented_box(coordinateSystem: &SpatialCoordinateSystem, box_: SpatialBoundingOrientedBox) -> Result<ComPtr<SpatialBoundingVolume>> { unsafe {
        <Self as RtActivatable<ISpatialBoundingVolumeStatics>>::get_activation_factory().from_oriented_box(coordinateSystem, box_)
    }}
    #[inline] pub fn from_sphere(coordinateSystem: &SpatialCoordinateSystem, sphere: SpatialBoundingSphere) -> Result<ComPtr<SpatialBoundingVolume>> { unsafe {
        <Self as RtActivatable<ISpatialBoundingVolumeStatics>>::get_activation_factory().from_sphere(coordinateSystem, sphere)
    }}
    #[inline] pub fn from_frustum(coordinateSystem: &SpatialCoordinateSystem, frustum: SpatialBoundingFrustum) -> Result<ComPtr<SpatialBoundingVolume>> { unsafe {
        <Self as RtActivatable<ISpatialBoundingVolumeStatics>>::get_activation_factory().from_frustum(coordinateSystem, frustum)
    }}
}
DEFINE_CLSID!(SpatialBoundingVolume(&[87,105,110,100,111,119,115,46,80,101,114,99,101,112,116,105,111,110,46,83,112,97,116,105,97,108,46,83,112,97,116,105,97,108,66,111,117,110,100,105,110,103,86,111,108,117,109,101,0]) [CLSID_SpatialBoundingVolume]);
DEFINE_IID!(IID_ISpatialBoundingVolumeStatics, 92836119, 46049, 14040, 176, 23, 86, 97, 129, 165, 177, 150);
RT_INTERFACE!{static interface ISpatialBoundingVolumeStatics(ISpatialBoundingVolumeStaticsVtbl): IInspectable(IInspectableVtbl) [IID_ISpatialBoundingVolumeStatics] {
    fn FromBox(&self, coordinateSystem: *mut SpatialCoordinateSystem, box_: SpatialBoundingBox, out: *mut *mut SpatialBoundingVolume) -> HRESULT,
    fn FromOrientedBox(&self, coordinateSystem: *mut SpatialCoordinateSystem, box_: SpatialBoundingOrientedBox, out: *mut *mut SpatialBoundingVolume) -> HRESULT,
    fn FromSphere(&self, coordinateSystem: *mut SpatialCoordinateSystem, sphere: SpatialBoundingSphere, out: *mut *mut SpatialBoundingVolume) -> HRESULT,
    fn FromFrustum(&self, coordinateSystem: *mut SpatialCoordinateSystem, frustum: SpatialBoundingFrustum, out: *mut *mut SpatialBoundingVolume) -> HRESULT
}}
impl ISpatialBoundingVolumeStatics {
    #[inline] pub unsafe fn from_box(&self, coordinateSystem: &SpatialCoordinateSystem, box_: SpatialBoundingBox) -> Result<ComPtr<SpatialBoundingVolume>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).FromBox)(self as *const _ as *mut _, coordinateSystem as *const _ as *mut _, box_, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn from_oriented_box(&self, coordinateSystem: &SpatialCoordinateSystem, box_: SpatialBoundingOrientedBox) -> Result<ComPtr<SpatialBoundingVolume>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).FromOrientedBox)(self as *const _ as *mut _, coordinateSystem as *const _ as *mut _, box_, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn from_sphere(&self, coordinateSystem: &SpatialCoordinateSystem, sphere: SpatialBoundingSphere) -> Result<ComPtr<SpatialBoundingVolume>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).FromSphere)(self as *const _ as *mut _, coordinateSystem as *const _ as *mut _, sphere, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn from_frustum(&self, coordinateSystem: &SpatialCoordinateSystem, frustum: SpatialBoundingFrustum) -> Result<ComPtr<SpatialBoundingVolume>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).FromFrustum)(self as *const _ as *mut _, coordinateSystem as *const _ as *mut _, frustum, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
DEFINE_IID!(IID_ISpatialCoordinateSystem, 1777060427, 24739, 13702, 166, 83, 89, 167, 189, 103, 109, 7);
RT_INTERFACE!{interface ISpatialCoordinateSystem(ISpatialCoordinateSystemVtbl): IInspectable(IInspectableVtbl) [IID_ISpatialCoordinateSystem] {
    fn TryGetTransformTo(&self, target: *mut SpatialCoordinateSystem, out: *mut *mut super::super::foundation::IReference<super::super::foundation::numerics::Matrix4x4>) -> HRESULT
}}
impl ISpatialCoordinateSystem {
    #[inline] pub unsafe fn try_get_transform_to(&self, target: &SpatialCoordinateSystem) -> Result<ComPtr<super::super::foundation::IReference<super::super::foundation::numerics::Matrix4x4>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).TryGetTransformTo)(self as *const _ as *mut _, target as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
RT_CLASS!{class SpatialCoordinateSystem: ISpatialCoordinateSystem}
DEFINE_IID!(IID_ISpatialEntity, 376301909, 57835, 17740, 186, 8, 230, 192, 102, 141, 220, 101);
RT_INTERFACE!{interface ISpatialEntity(ISpatialEntityVtbl): IInspectable(IInspectableVtbl) [IID_ISpatialEntity] {
    fn get_Id(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Anchor(&self, out: *mut *mut SpatialAnchor) -> HRESULT,
    fn get_Properties(&self, out: *mut *mut super::super::foundation::collections::ValueSet) -> HRESULT
}}
impl ISpatialEntity {
    #[inline] pub unsafe fn get_id(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Id)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_anchor(&self) -> Result<ComPtr<SpatialAnchor>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Anchor)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_properties(&self) -> Result<ComPtr<super::super::foundation::collections::ValueSet>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Properties)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
RT_CLASS!{class SpatialEntity: ISpatialEntity}
impl RtActivatable<ISpatialEntityFactory> for SpatialEntity {}
impl SpatialEntity {
    #[inline] pub fn create_with_spatial_anchor(spatialAnchor: &SpatialAnchor) -> Result<ComPtr<SpatialEntity>> { unsafe {
        <Self as RtActivatable<ISpatialEntityFactory>>::get_activation_factory().create_with_spatial_anchor(spatialAnchor)
    }}
    #[inline] pub fn create_with_spatial_anchor_and_properties(spatialAnchor: &SpatialAnchor, propertySet: &super::super::foundation::collections::ValueSet) -> Result<ComPtr<SpatialEntity>> { unsafe {
        <Self as RtActivatable<ISpatialEntityFactory>>::get_activation_factory().create_with_spatial_anchor_and_properties(spatialAnchor, propertySet)
    }}
}
DEFINE_CLSID!(SpatialEntity(&[87,105,110,100,111,119,115,46,80,101,114,99,101,112,116,105,111,110,46,83,112,97,116,105,97,108,46,83,112,97,116,105,97,108,69,110,116,105,116,121,0]) [CLSID_SpatialEntity]);
DEFINE_IID!(IID_ISpatialEntityAddedEventArgs, 2744644763, 5482, 18183, 172, 44, 211, 29, 87, 14, 211, 153);
RT_INTERFACE!{interface ISpatialEntityAddedEventArgs(ISpatialEntityAddedEventArgsVtbl): IInspectable(IInspectableVtbl) [IID_ISpatialEntityAddedEventArgs] {
    fn get_Entity(&self, out: *mut *mut SpatialEntity) -> HRESULT
}}
impl ISpatialEntityAddedEventArgs {
    #[inline] pub unsafe fn get_entity(&self) -> Result<ComPtr<SpatialEntity>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Entity)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
RT_CLASS!{class SpatialEntityAddedEventArgs: ISpatialEntityAddedEventArgs}
DEFINE_IID!(IID_ISpatialEntityFactory, 3790725925, 13471, 16933, 162, 243, 75, 1, 193, 95, 224, 86);
RT_INTERFACE!{static interface ISpatialEntityFactory(ISpatialEntityFactoryVtbl): IInspectable(IInspectableVtbl) [IID_ISpatialEntityFactory] {
    fn CreateWithSpatialAnchor(&self, spatialAnchor: *mut SpatialAnchor, out: *mut *mut SpatialEntity) -> HRESULT,
    fn CreateWithSpatialAnchorAndProperties(&self, spatialAnchor: *mut SpatialAnchor, propertySet: *mut super::super::foundation::collections::ValueSet, out: *mut *mut SpatialEntity) -> HRESULT
}}
impl ISpatialEntityFactory {
    #[inline] pub unsafe fn create_with_spatial_anchor(&self, spatialAnchor: &SpatialAnchor) -> Result<ComPtr<SpatialEntity>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).CreateWithSpatialAnchor)(self as *const _ as *mut _, spatialAnchor as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn create_with_spatial_anchor_and_properties(&self, spatialAnchor: &SpatialAnchor, propertySet: &super::super::foundation::collections::ValueSet) -> Result<ComPtr<SpatialEntity>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).CreateWithSpatialAnchorAndProperties)(self as *const _ as *mut _, spatialAnchor as *const _ as *mut _, propertySet as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
DEFINE_IID!(IID_ISpatialEntityRemovedEventArgs, 2440304640, 21357, 20127, 171, 246, 65, 91, 84, 68, 214, 81);
RT_INTERFACE!{interface ISpatialEntityRemovedEventArgs(ISpatialEntityRemovedEventArgsVtbl): IInspectable(IInspectableVtbl) [IID_ISpatialEntityRemovedEventArgs] {
    fn get_Entity(&self, out: *mut *mut SpatialEntity) -> HRESULT
}}
impl ISpatialEntityRemovedEventArgs {
    #[inline] pub unsafe fn get_entity(&self) -> Result<ComPtr<SpatialEntity>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Entity)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
RT_CLASS!{class SpatialEntityRemovedEventArgs: ISpatialEntityRemovedEventArgs}
DEFINE_IID!(IID_ISpatialEntityStore, 848791738, 58643, 20230, 136, 157, 27, 227, 14, 207, 67, 230);
RT_INTERFACE!{interface ISpatialEntityStore(ISpatialEntityStoreVtbl): IInspectable(IInspectableVtbl) [IID_ISpatialEntityStore] {
    fn SaveAsync(&self, entity: *mut SpatialEntity, out: *mut *mut super::super::foundation::IAsyncAction) -> HRESULT,
    fn RemoveAsync(&self, entity: *mut SpatialEntity, out: *mut *mut super::super::foundation::IAsyncAction) -> HRESULT,
    fn CreateEntityWatcher(&self, out: *mut *mut SpatialEntityWatcher) -> HRESULT
}}
impl ISpatialEntityStore {
    #[inline] pub unsafe fn save_async(&self, entity: &SpatialEntity) -> Result<ComPtr<super::super::foundation::IAsyncAction>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).SaveAsync)(self as *const _ as *mut _, entity as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn remove_async(&self, entity: &SpatialEntity) -> Result<ComPtr<super::super::foundation::IAsyncAction>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).RemoveAsync)(self as *const _ as *mut _, entity as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn create_entity_watcher(&self) -> Result<ComPtr<SpatialEntityWatcher>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).CreateEntityWatcher)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
RT_CLASS!{class SpatialEntityStore: ISpatialEntityStore}
impl RtActivatable<ISpatialEntityStoreStatics> for SpatialEntityStore {}
impl SpatialEntityStore {
    #[inline] pub fn get_is_supported() -> Result<bool> { unsafe {
        <Self as RtActivatable<ISpatialEntityStoreStatics>>::get_activation_factory().get_is_supported()
    }}
    #[cfg(feature="windows-system")] #[inline] pub fn try_get_for_remote_system_session(session: &super::super::system::remotesystems::RemoteSystemSession) -> Result<ComPtr<SpatialEntityStore>> { unsafe {
        <Self as RtActivatable<ISpatialEntityStoreStatics>>::get_activation_factory().try_get_for_remote_system_session(session)
    }}
}
DEFINE_CLSID!(SpatialEntityStore(&[87,105,110,100,111,119,115,46,80,101,114,99,101,112,116,105,111,110,46,83,112,97,116,105,97,108,46,83,112,97,116,105,97,108,69,110,116,105,116,121,83,116,111,114,101,0]) [CLSID_SpatialEntityStore]);
DEFINE_IID!(IID_ISpatialEntityStoreStatics, 1800091806, 31824, 20114, 138, 98, 77, 29, 75, 124, 205, 62);
RT_INTERFACE!{static interface ISpatialEntityStoreStatics(ISpatialEntityStoreStaticsVtbl): IInspectable(IInspectableVtbl) [IID_ISpatialEntityStoreStatics] {
    fn get_IsSupported(&self, out: *mut bool) -> HRESULT,
    #[cfg(feature="windows-system")] fn TryGetForRemoteSystemSession(&self, session: *mut super::super::system::remotesystems::RemoteSystemSession, out: *mut *mut SpatialEntityStore) -> HRESULT
}}
impl ISpatialEntityStoreStatics {
    #[inline] pub unsafe fn get_is_supported(&self) -> Result<bool> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_IsSupported)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[cfg(feature="windows-system")] #[inline] pub unsafe fn try_get_for_remote_system_session(&self, session: &super::super::system::remotesystems::RemoteSystemSession) -> Result<ComPtr<SpatialEntityStore>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).TryGetForRemoteSystemSession)(self as *const _ as *mut _, session as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
DEFINE_IID!(IID_ISpatialEntityUpdatedEventArgs, 3848738662, 25211, 17355, 164, 159, 179, 190, 109, 71, 222, 237);
RT_INTERFACE!{interface ISpatialEntityUpdatedEventArgs(ISpatialEntityUpdatedEventArgsVtbl): IInspectable(IInspectableVtbl) [IID_ISpatialEntityUpdatedEventArgs] {
    fn get_Entity(&self, out: *mut *mut SpatialEntity) -> HRESULT
}}
impl ISpatialEntityUpdatedEventArgs {
    #[inline] pub unsafe fn get_entity(&self) -> Result<ComPtr<SpatialEntity>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Entity)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
RT_CLASS!{class SpatialEntityUpdatedEventArgs: ISpatialEntityUpdatedEventArgs}
DEFINE_IID!(IID_ISpatialEntityWatcher, 3015204768, 27998, 19388, 128, 93, 95, 229, 185, 186, 25, 89);
RT_INTERFACE!{interface ISpatialEntityWatcher(ISpatialEntityWatcherVtbl): IInspectable(IInspectableVtbl) [IID_ISpatialEntityWatcher] {
    fn get_Status(&self, out: *mut SpatialEntityWatcherStatus) -> HRESULT,
    fn add_Added(&self, handler: *mut super::super::foundation::TypedEventHandler<SpatialEntityWatcher, SpatialEntityAddedEventArgs>, out: *mut super::super::foundation::EventRegistrationToken) -> HRESULT,
    fn remove_Added(&self, token: super::super::foundation::EventRegistrationToken) -> HRESULT,
    fn add_Updated(&self, handler: *mut super::super::foundation::TypedEventHandler<SpatialEntityWatcher, SpatialEntityUpdatedEventArgs>, out: *mut super::super::foundation::EventRegistrationToken) -> HRESULT,
    fn remove_Updated(&self, token: super::super::foundation::EventRegistrationToken) -> HRESULT,
    fn add_Removed(&self, handler: *mut super::super::foundation::TypedEventHandler<SpatialEntityWatcher, SpatialEntityRemovedEventArgs>, out: *mut super::super::foundation::EventRegistrationToken) -> HRESULT,
    fn remove_Removed(&self, token: super::super::foundation::EventRegistrationToken) -> HRESULT,
    fn add_EnumerationCompleted(&self, handler: *mut super::super::foundation::TypedEventHandler<SpatialEntityWatcher, IInspectable>, out: *mut super::super::foundation::EventRegistrationToken) -> HRESULT,
    fn remove_EnumerationCompleted(&self, token: super::super::foundation::EventRegistrationToken) -> HRESULT,
    fn Start(&self) -> HRESULT,
    fn Stop(&self) -> HRESULT
}}
impl ISpatialEntityWatcher {
    #[inline] pub unsafe fn get_status(&self) -> Result<SpatialEntityWatcherStatus> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_Status)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn add_added(&self, handler: &super::super::foundation::TypedEventHandler<SpatialEntityWatcher, SpatialEntityAddedEventArgs>) -> Result<super::super::foundation::EventRegistrationToken> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).add_Added)(self as *const _ as *mut _, handler as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn remove_added(&self, token: super::super::foundation::EventRegistrationToken) -> Result<()> {
        let hr = ((*self.lpVtbl).remove_Added)(self as *const _ as *mut _, token);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn add_updated(&self, handler: &super::super::foundation::TypedEventHandler<SpatialEntityWatcher, SpatialEntityUpdatedEventArgs>) -> Result<super::super::foundation::EventRegistrationToken> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).add_Updated)(self as *const _ as *mut _, handler as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn remove_updated(&self, token: super::super::foundation::EventRegistrationToken) -> Result<()> {
        let hr = ((*self.lpVtbl).remove_Updated)(self as *const _ as *mut _, token);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn add_removed(&self, handler: &super::super::foundation::TypedEventHandler<SpatialEntityWatcher, SpatialEntityRemovedEventArgs>) -> Result<super::super::foundation::EventRegistrationToken> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).add_Removed)(self as *const _ as *mut _, handler as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn remove_removed(&self, token: super::super::foundation::EventRegistrationToken) -> Result<()> {
        let hr = ((*self.lpVtbl).remove_Removed)(self as *const _ as *mut _, token);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn add_enumeration_completed(&self, handler: &super::super::foundation::TypedEventHandler<SpatialEntityWatcher, IInspectable>) -> Result<super::super::foundation::EventRegistrationToken> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).add_EnumerationCompleted)(self as *const _ as *mut _, handler as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn remove_enumeration_completed(&self, token: super::super::foundation::EventRegistrationToken) -> Result<()> {
        let hr = ((*self.lpVtbl).remove_EnumerationCompleted)(self as *const _ as *mut _, token);
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
}
RT_CLASS!{class SpatialEntityWatcher: ISpatialEntityWatcher}
RT_ENUM! { enum SpatialEntityWatcherStatus: i32 {
    Created (SpatialEntityWatcherStatus_Created) = 0, Started (SpatialEntityWatcherStatus_Started) = 1, EnumerationCompleted (SpatialEntityWatcherStatus_EnumerationCompleted) = 2, Stopping (SpatialEntityWatcherStatus_Stopping) = 3, Stopped (SpatialEntityWatcherStatus_Stopped) = 4, Aborted (SpatialEntityWatcherStatus_Aborted) = 5,
}}
RT_ENUM! { enum SpatialLocatability: i32 {
    Unavailable (SpatialLocatability_Unavailable) = 0, OrientationOnly (SpatialLocatability_OrientationOnly) = 1, PositionalTrackingActivating (SpatialLocatability_PositionalTrackingActivating) = 2, PositionalTrackingActive (SpatialLocatability_PositionalTrackingActive) = 3, PositionalTrackingInhibited (SpatialLocatability_PositionalTrackingInhibited) = 4,
}}
DEFINE_IID!(IID_ISpatialLocation, 495047325, 9377, 14293, 143, 161, 57, 180, 249, 173, 103, 226);
RT_INTERFACE!{interface ISpatialLocation(ISpatialLocationVtbl): IInspectable(IInspectableVtbl) [IID_ISpatialLocation] {
    fn get_Position(&self, out: *mut super::super::foundation::numerics::Vector3) -> HRESULT,
    fn get_Orientation(&self, out: *mut super::super::foundation::numerics::Quaternion) -> HRESULT,
    fn get_AbsoluteLinearVelocity(&self, out: *mut super::super::foundation::numerics::Vector3) -> HRESULT,
    fn get_AbsoluteLinearAcceleration(&self, out: *mut super::super::foundation::numerics::Vector3) -> HRESULT,
    fn get_AbsoluteAngularVelocity(&self, out: *mut super::super::foundation::numerics::Quaternion) -> HRESULT,
    fn get_AbsoluteAngularAcceleration(&self, out: *mut super::super::foundation::numerics::Quaternion) -> HRESULT
}}
impl ISpatialLocation {
    #[inline] pub unsafe fn get_position(&self) -> Result<super::super::foundation::numerics::Vector3> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_Position)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_orientation(&self) -> Result<super::super::foundation::numerics::Quaternion> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_Orientation)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_absolute_linear_velocity(&self) -> Result<super::super::foundation::numerics::Vector3> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_AbsoluteLinearVelocity)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_absolute_linear_acceleration(&self) -> Result<super::super::foundation::numerics::Vector3> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_AbsoluteLinearAcceleration)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_absolute_angular_velocity(&self) -> Result<super::super::foundation::numerics::Quaternion> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_AbsoluteAngularVelocity)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_absolute_angular_acceleration(&self) -> Result<super::super::foundation::numerics::Quaternion> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_AbsoluteAngularAcceleration)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
}
RT_CLASS!{class SpatialLocation: ISpatialLocation}
DEFINE_IID!(IID_ISpatialLocator, 4131883301, 40460, 15286, 153, 126, 182, 78, 204, 162, 76, 244);
RT_INTERFACE!{interface ISpatialLocator(ISpatialLocatorVtbl): IInspectable(IInspectableVtbl) [IID_ISpatialLocator] {
    fn get_Locatability(&self, out: *mut SpatialLocatability) -> HRESULT,
    fn add_LocatabilityChanged(&self, handler: *mut super::super::foundation::TypedEventHandler<SpatialLocator, IInspectable>, out: *mut super::super::foundation::EventRegistrationToken) -> HRESULT,
    fn remove_LocatabilityChanged(&self, cookie: super::super::foundation::EventRegistrationToken) -> HRESULT,
    fn add_PositionalTrackingDeactivating(&self, handler: *mut super::super::foundation::TypedEventHandler<SpatialLocator, SpatialLocatorPositionalTrackingDeactivatingEventArgs>, out: *mut super::super::foundation::EventRegistrationToken) -> HRESULT,
    fn remove_PositionalTrackingDeactivating(&self, cookie: super::super::foundation::EventRegistrationToken) -> HRESULT,
    fn TryLocateAtTimestamp(&self, timestamp: *mut super::PerceptionTimestamp, coordinateSystem: *mut SpatialCoordinateSystem, out: *mut *mut SpatialLocation) -> HRESULT,
    fn CreateAttachedFrameOfReferenceAtCurrentHeading(&self, out: *mut *mut SpatialLocatorAttachedFrameOfReference) -> HRESULT,
    fn CreateAttachedFrameOfReferenceAtCurrentHeadingWithPosition(&self, relativePosition: super::super::foundation::numerics::Vector3, out: *mut *mut SpatialLocatorAttachedFrameOfReference) -> HRESULT,
    fn CreateAttachedFrameOfReferenceAtCurrentHeadingWithPositionAndOrientation(&self, relativePosition: super::super::foundation::numerics::Vector3, relativeOrientation: super::super::foundation::numerics::Quaternion, out: *mut *mut SpatialLocatorAttachedFrameOfReference) -> HRESULT,
    fn CreateAttachedFrameOfReferenceAtCurrentHeadingWithPositionAndOrientationAndRelativeHeading(&self, relativePosition: super::super::foundation::numerics::Vector3, relativeOrientation: super::super::foundation::numerics::Quaternion, relativeHeadingInRadians: f64, out: *mut *mut SpatialLocatorAttachedFrameOfReference) -> HRESULT,
    fn CreateStationaryFrameOfReferenceAtCurrentLocation(&self, out: *mut *mut SpatialStationaryFrameOfReference) -> HRESULT,
    fn CreateStationaryFrameOfReferenceAtCurrentLocationWithPosition(&self, relativePosition: super::super::foundation::numerics::Vector3, out: *mut *mut SpatialStationaryFrameOfReference) -> HRESULT,
    fn CreateStationaryFrameOfReferenceAtCurrentLocationWithPositionAndOrientation(&self, relativePosition: super::super::foundation::numerics::Vector3, relativeOrientation: super::super::foundation::numerics::Quaternion, out: *mut *mut SpatialStationaryFrameOfReference) -> HRESULT,
    fn CreateStationaryFrameOfReferenceAtCurrentLocationWithPositionAndOrientationAndRelativeHeading(&self, relativePosition: super::super::foundation::numerics::Vector3, relativeOrientation: super::super::foundation::numerics::Quaternion, relativeHeadingInRadians: f64, out: *mut *mut SpatialStationaryFrameOfReference) -> HRESULT
}}
impl ISpatialLocator {
    #[inline] pub unsafe fn get_locatability(&self) -> Result<SpatialLocatability> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_Locatability)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn add_locatability_changed(&self, handler: &super::super::foundation::TypedEventHandler<SpatialLocator, IInspectable>) -> Result<super::super::foundation::EventRegistrationToken> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).add_LocatabilityChanged)(self as *const _ as *mut _, handler as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn remove_locatability_changed(&self, cookie: super::super::foundation::EventRegistrationToken) -> Result<()> {
        let hr = ((*self.lpVtbl).remove_LocatabilityChanged)(self as *const _ as *mut _, cookie);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn add_positional_tracking_deactivating(&self, handler: &super::super::foundation::TypedEventHandler<SpatialLocator, SpatialLocatorPositionalTrackingDeactivatingEventArgs>) -> Result<super::super::foundation::EventRegistrationToken> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).add_PositionalTrackingDeactivating)(self as *const _ as *mut _, handler as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn remove_positional_tracking_deactivating(&self, cookie: super::super::foundation::EventRegistrationToken) -> Result<()> {
        let hr = ((*self.lpVtbl).remove_PositionalTrackingDeactivating)(self as *const _ as *mut _, cookie);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn try_locate_at_timestamp(&self, timestamp: &super::PerceptionTimestamp, coordinateSystem: &SpatialCoordinateSystem) -> Result<ComPtr<SpatialLocation>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).TryLocateAtTimestamp)(self as *const _ as *mut _, timestamp as *const _ as *mut _, coordinateSystem as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn create_attached_frame_of_reference_at_current_heading(&self) -> Result<ComPtr<SpatialLocatorAttachedFrameOfReference>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).CreateAttachedFrameOfReferenceAtCurrentHeading)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn create_attached_frame_of_reference_at_current_heading_with_position(&self, relativePosition: super::super::foundation::numerics::Vector3) -> Result<ComPtr<SpatialLocatorAttachedFrameOfReference>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).CreateAttachedFrameOfReferenceAtCurrentHeadingWithPosition)(self as *const _ as *mut _, relativePosition, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn create_attached_frame_of_reference_at_current_heading_with_position_and_orientation(&self, relativePosition: super::super::foundation::numerics::Vector3, relativeOrientation: super::super::foundation::numerics::Quaternion) -> Result<ComPtr<SpatialLocatorAttachedFrameOfReference>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).CreateAttachedFrameOfReferenceAtCurrentHeadingWithPositionAndOrientation)(self as *const _ as *mut _, relativePosition, relativeOrientation, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn create_attached_frame_of_reference_at_current_heading_with_position_and_orientation_and_relative_heading(&self, relativePosition: super::super::foundation::numerics::Vector3, relativeOrientation: super::super::foundation::numerics::Quaternion, relativeHeadingInRadians: f64) -> Result<ComPtr<SpatialLocatorAttachedFrameOfReference>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).CreateAttachedFrameOfReferenceAtCurrentHeadingWithPositionAndOrientationAndRelativeHeading)(self as *const _ as *mut _, relativePosition, relativeOrientation, relativeHeadingInRadians, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn create_stationary_frame_of_reference_at_current_location(&self) -> Result<ComPtr<SpatialStationaryFrameOfReference>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).CreateStationaryFrameOfReferenceAtCurrentLocation)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn create_stationary_frame_of_reference_at_current_location_with_position(&self, relativePosition: super::super::foundation::numerics::Vector3) -> Result<ComPtr<SpatialStationaryFrameOfReference>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).CreateStationaryFrameOfReferenceAtCurrentLocationWithPosition)(self as *const _ as *mut _, relativePosition, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn create_stationary_frame_of_reference_at_current_location_with_position_and_orientation(&self, relativePosition: super::super::foundation::numerics::Vector3, relativeOrientation: super::super::foundation::numerics::Quaternion) -> Result<ComPtr<SpatialStationaryFrameOfReference>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).CreateStationaryFrameOfReferenceAtCurrentLocationWithPositionAndOrientation)(self as *const _ as *mut _, relativePosition, relativeOrientation, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn create_stationary_frame_of_reference_at_current_location_with_position_and_orientation_and_relative_heading(&self, relativePosition: super::super::foundation::numerics::Vector3, relativeOrientation: super::super::foundation::numerics::Quaternion, relativeHeadingInRadians: f64) -> Result<ComPtr<SpatialStationaryFrameOfReference>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).CreateStationaryFrameOfReferenceAtCurrentLocationWithPositionAndOrientationAndRelativeHeading)(self as *const _ as *mut _, relativePosition, relativeOrientation, relativeHeadingInRadians, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
RT_CLASS!{class SpatialLocator: ISpatialLocator}
impl RtActivatable<ISpatialLocatorStatics> for SpatialLocator {}
impl SpatialLocator {
    #[inline] pub fn get_default() -> Result<ComPtr<SpatialLocator>> { unsafe {
        <Self as RtActivatable<ISpatialLocatorStatics>>::get_activation_factory().get_default()
    }}
}
DEFINE_CLSID!(SpatialLocator(&[87,105,110,100,111,119,115,46,80,101,114,99,101,112,116,105,111,110,46,83,112,97,116,105,97,108,46,83,112,97,116,105,97,108,76,111,99,97,116,111,114,0]) [CLSID_SpatialLocator]);
DEFINE_IID!(IID_ISpatialLocatorAttachedFrameOfReference, 3782692598, 8015, 18844, 150, 37, 239, 94, 110, 215, 160, 72);
RT_INTERFACE!{interface ISpatialLocatorAttachedFrameOfReference(ISpatialLocatorAttachedFrameOfReferenceVtbl): IInspectable(IInspectableVtbl) [IID_ISpatialLocatorAttachedFrameOfReference] {
    fn get_RelativePosition(&self, out: *mut super::super::foundation::numerics::Vector3) -> HRESULT,
    fn put_RelativePosition(&self, value: super::super::foundation::numerics::Vector3) -> HRESULT,
    fn get_RelativeOrientation(&self, out: *mut super::super::foundation::numerics::Quaternion) -> HRESULT,
    fn put_RelativeOrientation(&self, value: super::super::foundation::numerics::Quaternion) -> HRESULT,
    fn AdjustHeading(&self, headingOffsetInRadians: f64) -> HRESULT,
    fn GetStationaryCoordinateSystemAtTimestamp(&self, timestamp: *mut super::PerceptionTimestamp, out: *mut *mut SpatialCoordinateSystem) -> HRESULT,
    fn TryGetRelativeHeadingAtTimestamp(&self, timestamp: *mut super::PerceptionTimestamp, out: *mut *mut super::super::foundation::IReference<f64>) -> HRESULT
}}
impl ISpatialLocatorAttachedFrameOfReference {
    #[inline] pub unsafe fn get_relative_position(&self) -> Result<super::super::foundation::numerics::Vector3> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_RelativePosition)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_relative_position(&self, value: super::super::foundation::numerics::Vector3) -> Result<()> {
        let hr = ((*self.lpVtbl).put_RelativePosition)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_relative_orientation(&self) -> Result<super::super::foundation::numerics::Quaternion> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_RelativeOrientation)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_relative_orientation(&self, value: super::super::foundation::numerics::Quaternion) -> Result<()> {
        let hr = ((*self.lpVtbl).put_RelativeOrientation)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn adjust_heading(&self, headingOffsetInRadians: f64) -> Result<()> {
        let hr = ((*self.lpVtbl).AdjustHeading)(self as *const _ as *mut _, headingOffsetInRadians);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_stationary_coordinate_system_at_timestamp(&self, timestamp: &super::PerceptionTimestamp) -> Result<ComPtr<SpatialCoordinateSystem>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetStationaryCoordinateSystemAtTimestamp)(self as *const _ as *mut _, timestamp as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn try_get_relative_heading_at_timestamp(&self, timestamp: &super::PerceptionTimestamp) -> Result<ComPtr<super::super::foundation::IReference<f64>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).TryGetRelativeHeadingAtTimestamp)(self as *const _ as *mut _, timestamp as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
RT_CLASS!{class SpatialLocatorAttachedFrameOfReference: ISpatialLocatorAttachedFrameOfReference}
DEFINE_IID!(IID_ISpatialLocatorPositionalTrackingDeactivatingEventArgs, 3098034275, 58356, 13963, 144, 97, 158, 169, 209, 214, 204, 22);
RT_INTERFACE!{interface ISpatialLocatorPositionalTrackingDeactivatingEventArgs(ISpatialLocatorPositionalTrackingDeactivatingEventArgsVtbl): IInspectable(IInspectableVtbl) [IID_ISpatialLocatorPositionalTrackingDeactivatingEventArgs] {
    fn get_Canceled(&self, out: *mut bool) -> HRESULT,
    fn put_Canceled(&self, value: bool) -> HRESULT
}}
impl ISpatialLocatorPositionalTrackingDeactivatingEventArgs {
    #[inline] pub unsafe fn get_canceled(&self) -> Result<bool> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_Canceled)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_canceled(&self, value: bool) -> Result<()> {
        let hr = ((*self.lpVtbl).put_Canceled)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
}
RT_CLASS!{class SpatialLocatorPositionalTrackingDeactivatingEventArgs: ISpatialLocatorPositionalTrackingDeactivatingEventArgs}
DEFINE_IID!(IID_ISpatialLocatorStatics, 3077452608, 42946, 13851, 187, 130, 86, 233, 59, 137, 177, 187);
RT_INTERFACE!{static interface ISpatialLocatorStatics(ISpatialLocatorStaticsVtbl): IInspectable(IInspectableVtbl) [IID_ISpatialLocatorStatics] {
    fn GetDefault(&self, out: *mut *mut SpatialLocator) -> HRESULT
}}
impl ISpatialLocatorStatics {
    #[inline] pub unsafe fn get_default(&self) -> Result<ComPtr<SpatialLocator>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetDefault)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
RT_ENUM! { enum SpatialLookDirectionRange: i32 {
    ForwardOnly (SpatialLookDirectionRange_ForwardOnly) = 0, Omnidirectional (SpatialLookDirectionRange_Omnidirectional) = 1,
}}
RT_ENUM! { enum SpatialMovementRange: i32 {
    NoMovement (SpatialMovementRange_NoMovement) = 0, Bounded (SpatialMovementRange_Bounded) = 1,
}}
RT_ENUM! { enum SpatialPerceptionAccessStatus: i32 {
    Unspecified (SpatialPerceptionAccessStatus_Unspecified) = 0, Allowed (SpatialPerceptionAccessStatus_Allowed) = 1, DeniedByUser (SpatialPerceptionAccessStatus_DeniedByUser) = 2, DeniedBySystem (SpatialPerceptionAccessStatus_DeniedBySystem) = 3,
}}
DEFINE_IID!(IID_ISpatialStageFrameOfReference, 2055877732, 44301, 17808, 171, 134, 51, 6, 43, 103, 73, 38);
RT_INTERFACE!{interface ISpatialStageFrameOfReference(ISpatialStageFrameOfReferenceVtbl): IInspectable(IInspectableVtbl) [IID_ISpatialStageFrameOfReference] {
    fn get_CoordinateSystem(&self, out: *mut *mut SpatialCoordinateSystem) -> HRESULT,
    fn get_MovementRange(&self, out: *mut SpatialMovementRange) -> HRESULT,
    fn get_LookDirectionRange(&self, out: *mut SpatialLookDirectionRange) -> HRESULT,
    fn GetCoordinateSystemAtCurrentLocation(&self, locator: *mut SpatialLocator, out: *mut *mut SpatialCoordinateSystem) -> HRESULT,
    fn TryGetMovementBounds(&self, coordinateSystem: *mut SpatialCoordinateSystem, outSize: *mut u32, out: *mut *mut super::super::foundation::numerics::Vector3) -> HRESULT
}}
impl ISpatialStageFrameOfReference {
    #[inline] pub unsafe fn get_coordinate_system(&self) -> Result<ComPtr<SpatialCoordinateSystem>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_CoordinateSystem)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_movement_range(&self) -> Result<SpatialMovementRange> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_MovementRange)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_look_direction_range(&self) -> Result<SpatialLookDirectionRange> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_LookDirectionRange)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_coordinate_system_at_current_location(&self, locator: &SpatialLocator) -> Result<ComPtr<SpatialCoordinateSystem>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetCoordinateSystemAtCurrentLocation)(self as *const _ as *mut _, locator as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn try_get_movement_bounds(&self, coordinateSystem: &SpatialCoordinateSystem) -> Result<ComArray<super::super::foundation::numerics::Vector3>> {
        let mut outSize = 0; let mut out = null_mut();
        let hr = ((*self.lpVtbl).TryGetMovementBounds)(self as *const _ as *mut _, coordinateSystem as *const _ as *mut _, &mut outSize, &mut out);
        if hr == S_OK { Ok(ComArray::from_raw(outSize, out)) } else { err(hr) }
    }
}
RT_CLASS!{class SpatialStageFrameOfReference: ISpatialStageFrameOfReference}
impl RtActivatable<ISpatialStageFrameOfReferenceStatics> for SpatialStageFrameOfReference {}
impl SpatialStageFrameOfReference {
    #[inline] pub fn get_current() -> Result<ComPtr<SpatialStageFrameOfReference>> { unsafe {
        <Self as RtActivatable<ISpatialStageFrameOfReferenceStatics>>::get_activation_factory().get_current()
    }}
    #[inline] pub fn add_current_changed(handler: &super::super::foundation::EventHandler<IInspectable>) -> Result<super::super::foundation::EventRegistrationToken> { unsafe {
        <Self as RtActivatable<ISpatialStageFrameOfReferenceStatics>>::get_activation_factory().add_current_changed(handler)
    }}
    #[inline] pub fn remove_current_changed(cookie: super::super::foundation::EventRegistrationToken) -> Result<()> { unsafe {
        <Self as RtActivatable<ISpatialStageFrameOfReferenceStatics>>::get_activation_factory().remove_current_changed(cookie)
    }}
    #[inline] pub fn request_new_stage_async() -> Result<ComPtr<super::super::foundation::IAsyncOperation<SpatialStageFrameOfReference>>> { unsafe {
        <Self as RtActivatable<ISpatialStageFrameOfReferenceStatics>>::get_activation_factory().request_new_stage_async()
    }}
}
DEFINE_CLSID!(SpatialStageFrameOfReference(&[87,105,110,100,111,119,115,46,80,101,114,99,101,112,116,105,111,110,46,83,112,97,116,105,97,108,46,83,112,97,116,105,97,108,83,116,97,103,101,70,114,97,109,101,79,102,82,101,102,101,114,101,110,99,101,0]) [CLSID_SpatialStageFrameOfReference]);
DEFINE_IID!(IID_ISpatialStageFrameOfReferenceStatics, 4153236557, 41124, 18844, 141, 145, 168, 201, 101, 212, 6, 84);
RT_INTERFACE!{static interface ISpatialStageFrameOfReferenceStatics(ISpatialStageFrameOfReferenceStaticsVtbl): IInspectable(IInspectableVtbl) [IID_ISpatialStageFrameOfReferenceStatics] {
    fn get_Current(&self, out: *mut *mut SpatialStageFrameOfReference) -> HRESULT,
    fn add_CurrentChanged(&self, handler: *mut super::super::foundation::EventHandler<IInspectable>, out: *mut super::super::foundation::EventRegistrationToken) -> HRESULT,
    fn remove_CurrentChanged(&self, cookie: super::super::foundation::EventRegistrationToken) -> HRESULT,
    fn RequestNewStageAsync(&self, out: *mut *mut super::super::foundation::IAsyncOperation<SpatialStageFrameOfReference>) -> HRESULT
}}
impl ISpatialStageFrameOfReferenceStatics {
    #[inline] pub unsafe fn get_current(&self) -> Result<ComPtr<SpatialStageFrameOfReference>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Current)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn add_current_changed(&self, handler: &super::super::foundation::EventHandler<IInspectable>) -> Result<super::super::foundation::EventRegistrationToken> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).add_CurrentChanged)(self as *const _ as *mut _, handler as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn remove_current_changed(&self, cookie: super::super::foundation::EventRegistrationToken) -> Result<()> {
        let hr = ((*self.lpVtbl).remove_CurrentChanged)(self as *const _ as *mut _, cookie);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn request_new_stage_async(&self) -> Result<ComPtr<super::super::foundation::IAsyncOperation<SpatialStageFrameOfReference>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).RequestNewStageAsync)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
DEFINE_IID!(IID_ISpatialStationaryFrameOfReference, 165399737, 48376, 15999, 190, 126, 126, 220, 203, 177, 120, 168);
RT_INTERFACE!{interface ISpatialStationaryFrameOfReference(ISpatialStationaryFrameOfReferenceVtbl): IInspectable(IInspectableVtbl) [IID_ISpatialStationaryFrameOfReference] {
    fn get_CoordinateSystem(&self, out: *mut *mut SpatialCoordinateSystem) -> HRESULT
}}
impl ISpatialStationaryFrameOfReference {
    #[inline] pub unsafe fn get_coordinate_system(&self) -> Result<ComPtr<SpatialCoordinateSystem>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_CoordinateSystem)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
RT_CLASS!{class SpatialStationaryFrameOfReference: ISpatialStationaryFrameOfReference}
pub mod surfaces { // Windows.Perception.Spatial.Surfaces
use ::prelude::*;
DEFINE_IID!(IID_ISpatialSurfaceInfo, 4176079847, 14775, 14690, 187, 3, 87, 245, 110, 31, 176, 161);
RT_INTERFACE!{interface ISpatialSurfaceInfo(ISpatialSurfaceInfoVtbl): IInspectable(IInspectableVtbl) [IID_ISpatialSurfaceInfo] {
    fn get_Id(&self, out: *mut Guid) -> HRESULT,
    fn get_UpdateTime(&self, out: *mut ::rt::gen::windows::foundation::DateTime) -> HRESULT,
    fn TryGetBounds(&self, coordinateSystem: *mut super::SpatialCoordinateSystem, out: *mut *mut ::rt::gen::windows::foundation::IReference<super::SpatialBoundingOrientedBox>) -> HRESULT,
    fn TryComputeLatestMeshAsync(&self, maxTrianglesPerCubicMeter: f64, out: *mut *mut ::rt::gen::windows::foundation::IAsyncOperation<SpatialSurfaceMesh>) -> HRESULT,
    fn TryComputeLatestMeshWithOptionsAsync(&self, maxTrianglesPerCubicMeter: f64, options: *mut SpatialSurfaceMeshOptions, out: *mut *mut ::rt::gen::windows::foundation::IAsyncOperation<SpatialSurfaceMesh>) -> HRESULT
}}
impl ISpatialSurfaceInfo {
    #[inline] pub unsafe fn get_id(&self) -> Result<Guid> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_Id)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_update_time(&self) -> Result<::rt::gen::windows::foundation::DateTime> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_UpdateTime)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn try_get_bounds(&self, coordinateSystem: &super::SpatialCoordinateSystem) -> Result<ComPtr<::rt::gen::windows::foundation::IReference<super::SpatialBoundingOrientedBox>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).TryGetBounds)(self as *const _ as *mut _, coordinateSystem as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn try_compute_latest_mesh_async(&self, maxTrianglesPerCubicMeter: f64) -> Result<ComPtr<::rt::gen::windows::foundation::IAsyncOperation<SpatialSurfaceMesh>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).TryComputeLatestMeshAsync)(self as *const _ as *mut _, maxTrianglesPerCubicMeter, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn try_compute_latest_mesh_with_options_async(&self, maxTrianglesPerCubicMeter: f64, options: &SpatialSurfaceMeshOptions) -> Result<ComPtr<::rt::gen::windows::foundation::IAsyncOperation<SpatialSurfaceMesh>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).TryComputeLatestMeshWithOptionsAsync)(self as *const _ as *mut _, maxTrianglesPerCubicMeter, options as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
RT_CLASS!{class SpatialSurfaceInfo: ISpatialSurfaceInfo}
DEFINE_IID!(IID_ISpatialSurfaceMesh, 277829593, 57101, 14672, 160, 253, 249, 114, 199, 124, 39, 180);
RT_INTERFACE!{interface ISpatialSurfaceMesh(ISpatialSurfaceMeshVtbl): IInspectable(IInspectableVtbl) [IID_ISpatialSurfaceMesh] {
    fn get_SurfaceInfo(&self, out: *mut *mut SpatialSurfaceInfo) -> HRESULT,
    fn get_CoordinateSystem(&self, out: *mut *mut super::SpatialCoordinateSystem) -> HRESULT,
    fn get_TriangleIndices(&self, out: *mut *mut SpatialSurfaceMeshBuffer) -> HRESULT,
    fn get_VertexPositions(&self, out: *mut *mut SpatialSurfaceMeshBuffer) -> HRESULT,
    fn get_VertexPositionScale(&self, out: *mut ::rt::gen::windows::foundation::numerics::Vector3) -> HRESULT,
    fn get_VertexNormals(&self, out: *mut *mut SpatialSurfaceMeshBuffer) -> HRESULT
}}
impl ISpatialSurfaceMesh {
    #[inline] pub unsafe fn get_surface_info(&self) -> Result<ComPtr<SpatialSurfaceInfo>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_SurfaceInfo)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_coordinate_system(&self) -> Result<ComPtr<super::SpatialCoordinateSystem>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_CoordinateSystem)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_triangle_indices(&self) -> Result<ComPtr<SpatialSurfaceMeshBuffer>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_TriangleIndices)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_vertex_positions(&self) -> Result<ComPtr<SpatialSurfaceMeshBuffer>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_VertexPositions)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_vertex_position_scale(&self) -> Result<::rt::gen::windows::foundation::numerics::Vector3> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_VertexPositionScale)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_vertex_normals(&self) -> Result<ComPtr<SpatialSurfaceMeshBuffer>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_VertexNormals)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
RT_CLASS!{class SpatialSurfaceMesh: ISpatialSurfaceMesh}
DEFINE_IID!(IID_ISpatialSurfaceMeshBuffer, 2479839712, 34591, 13304, 152, 178, 3, 209, 1, 69, 143, 111);
RT_INTERFACE!{interface ISpatialSurfaceMeshBuffer(ISpatialSurfaceMeshBufferVtbl): IInspectable(IInspectableVtbl) [IID_ISpatialSurfaceMeshBuffer] {
    #[cfg(not(feature="windows-graphics"))] fn __Dummy0(&self) -> (),
    #[cfg(feature="windows-graphics")] fn get_Format(&self, out: *mut ::rt::gen::windows::graphics::directx::DirectXPixelFormat) -> HRESULT,
    fn get_Stride(&self, out: *mut u32) -> HRESULT,
    fn get_ElementCount(&self, out: *mut u32) -> HRESULT,
    #[cfg(feature="windows-storage")] fn get_Data(&self, out: *mut *mut ::rt::gen::windows::storage::streams::IBuffer) -> HRESULT
}}
impl ISpatialSurfaceMeshBuffer {
    #[cfg(feature="windows-graphics")] #[inline] pub unsafe fn get_format(&self) -> Result<::rt::gen::windows::graphics::directx::DirectXPixelFormat> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_Format)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_stride(&self) -> Result<u32> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_Stride)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_element_count(&self) -> Result<u32> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_ElementCount)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[cfg(feature="windows-storage")] #[inline] pub unsafe fn get_data(&self) -> Result<ComPtr<::rt::gen::windows::storage::streams::IBuffer>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Data)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
RT_CLASS!{class SpatialSurfaceMeshBuffer: ISpatialSurfaceMeshBuffer}
DEFINE_IID!(IID_ISpatialSurfaceMeshOptions, 3530923913, 13682, 15661, 161, 13, 95, 238, 147, 148, 170, 55);
RT_INTERFACE!{interface ISpatialSurfaceMeshOptions(ISpatialSurfaceMeshOptionsVtbl): IInspectable(IInspectableVtbl) [IID_ISpatialSurfaceMeshOptions] {
    #[cfg(not(feature="windows-graphics"))] fn __Dummy0(&self) -> (),
    #[cfg(feature="windows-graphics")] fn get_VertexPositionFormat(&self, out: *mut ::rt::gen::windows::graphics::directx::DirectXPixelFormat) -> HRESULT,
    #[cfg(not(feature="windows-graphics"))] fn __Dummy1(&self) -> (),
    #[cfg(feature="windows-graphics")] fn put_VertexPositionFormat(&self, value: ::rt::gen::windows::graphics::directx::DirectXPixelFormat) -> HRESULT,
    #[cfg(not(feature="windows-graphics"))] fn __Dummy2(&self) -> (),
    #[cfg(feature="windows-graphics")] fn get_TriangleIndexFormat(&self, out: *mut ::rt::gen::windows::graphics::directx::DirectXPixelFormat) -> HRESULT,
    #[cfg(not(feature="windows-graphics"))] fn __Dummy3(&self) -> (),
    #[cfg(feature="windows-graphics")] fn put_TriangleIndexFormat(&self, value: ::rt::gen::windows::graphics::directx::DirectXPixelFormat) -> HRESULT,
    #[cfg(not(feature="windows-graphics"))] fn __Dummy4(&self) -> (),
    #[cfg(feature="windows-graphics")] fn get_VertexNormalFormat(&self, out: *mut ::rt::gen::windows::graphics::directx::DirectXPixelFormat) -> HRESULT,
    #[cfg(not(feature="windows-graphics"))] fn __Dummy5(&self) -> (),
    #[cfg(feature="windows-graphics")] fn put_VertexNormalFormat(&self, value: ::rt::gen::windows::graphics::directx::DirectXPixelFormat) -> HRESULT,
    fn get_IncludeVertexNormals(&self, out: *mut bool) -> HRESULT,
    fn put_IncludeVertexNormals(&self, value: bool) -> HRESULT
}}
impl ISpatialSurfaceMeshOptions {
    #[cfg(feature="windows-graphics")] #[inline] pub unsafe fn get_vertex_position_format(&self) -> Result<::rt::gen::windows::graphics::directx::DirectXPixelFormat> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_VertexPositionFormat)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[cfg(feature="windows-graphics")] #[inline] pub unsafe fn set_vertex_position_format(&self, value: ::rt::gen::windows::graphics::directx::DirectXPixelFormat) -> Result<()> {
        let hr = ((*self.lpVtbl).put_VertexPositionFormat)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[cfg(feature="windows-graphics")] #[inline] pub unsafe fn get_triangle_index_format(&self) -> Result<::rt::gen::windows::graphics::directx::DirectXPixelFormat> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_TriangleIndexFormat)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[cfg(feature="windows-graphics")] #[inline] pub unsafe fn set_triangle_index_format(&self, value: ::rt::gen::windows::graphics::directx::DirectXPixelFormat) -> Result<()> {
        let hr = ((*self.lpVtbl).put_TriangleIndexFormat)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[cfg(feature="windows-graphics")] #[inline] pub unsafe fn get_vertex_normal_format(&self) -> Result<::rt::gen::windows::graphics::directx::DirectXPixelFormat> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_VertexNormalFormat)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[cfg(feature="windows-graphics")] #[inline] pub unsafe fn set_vertex_normal_format(&self, value: ::rt::gen::windows::graphics::directx::DirectXPixelFormat) -> Result<()> {
        let hr = ((*self.lpVtbl).put_VertexNormalFormat)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_include_vertex_normals(&self) -> Result<bool> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_IncludeVertexNormals)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_include_vertex_normals(&self, value: bool) -> Result<()> {
        let hr = ((*self.lpVtbl).put_IncludeVertexNormals)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
}
RT_CLASS!{class SpatialSurfaceMeshOptions: ISpatialSurfaceMeshOptions}
impl RtActivatable<ISpatialSurfaceMeshOptionsStatics> for SpatialSurfaceMeshOptions {}
impl RtActivatable<IActivationFactory> for SpatialSurfaceMeshOptions {}
impl SpatialSurfaceMeshOptions {
    #[cfg(feature="windows-graphics")] #[inline] pub fn get_supported_vertex_position_formats() -> Result<ComPtr<::rt::gen::windows::foundation::collections::IVectorView<::rt::gen::windows::graphics::directx::DirectXPixelFormat>>> { unsafe {
        <Self as RtActivatable<ISpatialSurfaceMeshOptionsStatics>>::get_activation_factory().get_supported_vertex_position_formats()
    }}
    #[cfg(feature="windows-graphics")] #[inline] pub fn get_supported_triangle_index_formats() -> Result<ComPtr<::rt::gen::windows::foundation::collections::IVectorView<::rt::gen::windows::graphics::directx::DirectXPixelFormat>>> { unsafe {
        <Self as RtActivatable<ISpatialSurfaceMeshOptionsStatics>>::get_activation_factory().get_supported_triangle_index_formats()
    }}
    #[cfg(feature="windows-graphics")] #[inline] pub fn get_supported_vertex_normal_formats() -> Result<ComPtr<::rt::gen::windows::foundation::collections::IVectorView<::rt::gen::windows::graphics::directx::DirectXPixelFormat>>> { unsafe {
        <Self as RtActivatable<ISpatialSurfaceMeshOptionsStatics>>::get_activation_factory().get_supported_vertex_normal_formats()
    }}
}
DEFINE_CLSID!(SpatialSurfaceMeshOptions(&[87,105,110,100,111,119,115,46,80,101,114,99,101,112,116,105,111,110,46,83,112,97,116,105,97,108,46,83,117,114,102,97,99,101,115,46,83,112,97,116,105,97,108,83,117,114,102,97,99,101,77,101,115,104,79,112,116,105,111,110,115,0]) [CLSID_SpatialSurfaceMeshOptions]);
DEFINE_IID!(IID_ISpatialSurfaceMeshOptionsStatics, 2603879103, 38785, 17669, 137, 53, 1, 53, 117, 202, 174, 94);
RT_INTERFACE!{static interface ISpatialSurfaceMeshOptionsStatics(ISpatialSurfaceMeshOptionsStaticsVtbl): IInspectable(IInspectableVtbl) [IID_ISpatialSurfaceMeshOptionsStatics] {
    #[cfg(feature="windows-graphics")] fn get_SupportedVertexPositionFormats(&self, out: *mut *mut ::rt::gen::windows::foundation::collections::IVectorView<::rt::gen::windows::graphics::directx::DirectXPixelFormat>) -> HRESULT,
    #[cfg(feature="windows-graphics")] fn get_SupportedTriangleIndexFormats(&self, out: *mut *mut ::rt::gen::windows::foundation::collections::IVectorView<::rt::gen::windows::graphics::directx::DirectXPixelFormat>) -> HRESULT,
    #[cfg(feature="windows-graphics")] fn get_SupportedVertexNormalFormats(&self, out: *mut *mut ::rt::gen::windows::foundation::collections::IVectorView<::rt::gen::windows::graphics::directx::DirectXPixelFormat>) -> HRESULT
}}
impl ISpatialSurfaceMeshOptionsStatics {
    #[cfg(feature="windows-graphics")] #[inline] pub unsafe fn get_supported_vertex_position_formats(&self) -> Result<ComPtr<::rt::gen::windows::foundation::collections::IVectorView<::rt::gen::windows::graphics::directx::DirectXPixelFormat>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_SupportedVertexPositionFormats)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[cfg(feature="windows-graphics")] #[inline] pub unsafe fn get_supported_triangle_index_formats(&self) -> Result<ComPtr<::rt::gen::windows::foundation::collections::IVectorView<::rt::gen::windows::graphics::directx::DirectXPixelFormat>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_SupportedTriangleIndexFormats)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[cfg(feature="windows-graphics")] #[inline] pub unsafe fn get_supported_vertex_normal_formats(&self) -> Result<ComPtr<::rt::gen::windows::foundation::collections::IVectorView<::rt::gen::windows::graphics::directx::DirectXPixelFormat>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_SupportedVertexNormalFormats)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
DEFINE_IID!(IID_ISpatialSurfaceObserver, 280401945, 56778, 13443, 172, 58, 116, 143, 232, 200, 109, 245);
RT_INTERFACE!{interface ISpatialSurfaceObserver(ISpatialSurfaceObserverVtbl): IInspectable(IInspectableVtbl) [IID_ISpatialSurfaceObserver] {
    fn GetObservedSurfaces(&self, out: *mut *mut ::rt::gen::windows::foundation::collections::IMapView<Guid, SpatialSurfaceInfo>) -> HRESULT,
    fn SetBoundingVolume(&self, bounds: *mut super::SpatialBoundingVolume) -> HRESULT,
    fn SetBoundingVolumes(&self, bounds: *mut ::rt::gen::windows::foundation::collections::IIterable<super::SpatialBoundingVolume>) -> HRESULT,
    fn add_ObservedSurfacesChanged(&self, handler: *mut ::rt::gen::windows::foundation::TypedEventHandler<SpatialSurfaceObserver, IInspectable>, out: *mut ::rt::gen::windows::foundation::EventRegistrationToken) -> HRESULT,
    fn remove_ObservedSurfacesChanged(&self, token: ::rt::gen::windows::foundation::EventRegistrationToken) -> HRESULT
}}
impl ISpatialSurfaceObserver {
    #[inline] pub unsafe fn get_observed_surfaces(&self) -> Result<ComPtr<::rt::gen::windows::foundation::collections::IMapView<Guid, SpatialSurfaceInfo>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetObservedSurfaces)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_bounding_volume(&self, bounds: &super::SpatialBoundingVolume) -> Result<()> {
        let hr = ((*self.lpVtbl).SetBoundingVolume)(self as *const _ as *mut _, bounds as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_bounding_volumes(&self, bounds: &::rt::gen::windows::foundation::collections::IIterable<super::SpatialBoundingVolume>) -> Result<()> {
        let hr = ((*self.lpVtbl).SetBoundingVolumes)(self as *const _ as *mut _, bounds as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn add_observed_surfaces_changed(&self, handler: &::rt::gen::windows::foundation::TypedEventHandler<SpatialSurfaceObserver, IInspectable>) -> Result<::rt::gen::windows::foundation::EventRegistrationToken> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).add_ObservedSurfacesChanged)(self as *const _ as *mut _, handler as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn remove_observed_surfaces_changed(&self, token: ::rt::gen::windows::foundation::EventRegistrationToken) -> Result<()> {
        let hr = ((*self.lpVtbl).remove_ObservedSurfacesChanged)(self as *const _ as *mut _, token);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
}
RT_CLASS!{class SpatialSurfaceObserver: ISpatialSurfaceObserver}
impl RtActivatable<ISpatialSurfaceObserverStatics> for SpatialSurfaceObserver {}
impl RtActivatable<ISpatialSurfaceObserverStatics2> for SpatialSurfaceObserver {}
impl RtActivatable<IActivationFactory> for SpatialSurfaceObserver {}
impl SpatialSurfaceObserver {
    #[inline] pub fn request_access_async() -> Result<ComPtr<::rt::gen::windows::foundation::IAsyncOperation<super::SpatialPerceptionAccessStatus>>> { unsafe {
        <Self as RtActivatable<ISpatialSurfaceObserverStatics>>::get_activation_factory().request_access_async()
    }}
    #[inline] pub fn is_supported() -> Result<bool> { unsafe {
        <Self as RtActivatable<ISpatialSurfaceObserverStatics2>>::get_activation_factory().is_supported()
    }}
}
DEFINE_CLSID!(SpatialSurfaceObserver(&[87,105,110,100,111,119,115,46,80,101,114,99,101,112,116,105,111,110,46,83,112,97,116,105,97,108,46,83,117,114,102,97,99,101,115,46,83,112,97,116,105,97,108,83,117,114,102,97,99,101,79,98,115,101,114,118,101,114,0]) [CLSID_SpatialSurfaceObserver]);
DEFINE_IID!(IID_ISpatialSurfaceObserverStatics, 374952429, 8456, 16744, 145, 117, 135, 224, 39, 188, 146, 133);
RT_INTERFACE!{static interface ISpatialSurfaceObserverStatics(ISpatialSurfaceObserverStaticsVtbl): IInspectable(IInspectableVtbl) [IID_ISpatialSurfaceObserverStatics] {
    fn RequestAccessAsync(&self, out: *mut *mut ::rt::gen::windows::foundation::IAsyncOperation<super::SpatialPerceptionAccessStatus>) -> HRESULT
}}
impl ISpatialSurfaceObserverStatics {
    #[inline] pub unsafe fn request_access_async(&self) -> Result<ComPtr<::rt::gen::windows::foundation::IAsyncOperation<super::SpatialPerceptionAccessStatus>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).RequestAccessAsync)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
DEFINE_IID!(IID_ISpatialSurfaceObserverStatics2, 257114721, 50525, 20075, 168, 149, 161, 157, 230, 154, 66, 227);
RT_INTERFACE!{static interface ISpatialSurfaceObserverStatics2(ISpatialSurfaceObserverStatics2Vtbl): IInspectable(IInspectableVtbl) [IID_ISpatialSurfaceObserverStatics2] {
    fn IsSupported(&self, out: *mut bool) -> HRESULT
}}
impl ISpatialSurfaceObserverStatics2 {
    #[inline] pub unsafe fn is_supported(&self) -> Result<bool> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).IsSupported)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
}
} // Windows.Perception.Spatial.Surfaces
} // Windows.Perception.Spatial
pub mod people { // Windows.Perception.People
use ::prelude::*;
DEFINE_IID!(IID_IHeadPose, 2136655269, 18907, 14239, 148, 41, 50, 162, 250, 243, 79, 166);
RT_INTERFACE!{interface IHeadPose(IHeadPoseVtbl): IInspectable(IInspectableVtbl) [IID_IHeadPose] {
    fn get_Position(&self, out: *mut super::super::foundation::numerics::Vector3) -> HRESULT,
    fn get_ForwardDirection(&self, out: *mut super::super::foundation::numerics::Vector3) -> HRESULT,
    fn get_UpDirection(&self, out: *mut super::super::foundation::numerics::Vector3) -> HRESULT
}}
impl IHeadPose {
    #[inline] pub unsafe fn get_position(&self) -> Result<super::super::foundation::numerics::Vector3> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_Position)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_forward_direction(&self) -> Result<super::super::foundation::numerics::Vector3> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_ForwardDirection)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_up_direction(&self) -> Result<super::super::foundation::numerics::Vector3> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_UpDirection)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
}
RT_CLASS!{class HeadPose: IHeadPose}
} // Windows.Perception.People
pub mod automation { // Windows.Perception.Automation
pub mod core { // Windows.Perception.Automation.Core
use ::prelude::*;
RT_CLASS!{static class CorePerceptionAutomation}
impl RtActivatable<ICorePerceptionAutomationStatics> for CorePerceptionAutomation {}
impl CorePerceptionAutomation {
    #[inline] pub fn set_activation_factory_provider(provider: &::rt::gen::windows::foundation::IGetActivationFactory) -> Result<()> { unsafe {
        <Self as RtActivatable<ICorePerceptionAutomationStatics>>::get_activation_factory().set_activation_factory_provider(provider)
    }}
}
DEFINE_CLSID!(CorePerceptionAutomation(&[87,105,110,100,111,119,115,46,80,101,114,99,101,112,116,105,111,110,46,65,117,116,111,109,97,116,105,111,110,46,67,111,114,101,46,67,111,114,101,80,101,114,99,101,112,116,105,111,110,65,117,116,111,109,97,116,105,111,110,0]) [CLSID_CorePerceptionAutomation]);
DEFINE_IID!(IID_ICorePerceptionAutomationStatics, 196101441, 19682, 18723, 154, 118, 129, 135, 236, 197, 145, 18);
RT_INTERFACE!{static interface ICorePerceptionAutomationStatics(ICorePerceptionAutomationStaticsVtbl): IInspectable(IInspectableVtbl) [IID_ICorePerceptionAutomationStatics] {
    fn SetActivationFactoryProvider(&self, provider: *mut ::rt::gen::windows::foundation::IGetActivationFactory) -> HRESULT
}}
impl ICorePerceptionAutomationStatics {
    #[inline] pub unsafe fn set_activation_factory_provider(&self, provider: &::rt::gen::windows::foundation::IGetActivationFactory) -> Result<()> {
        let hr = ((*self.lpVtbl).SetActivationFactoryProvider)(self as *const _ as *mut _, provider as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
}
} // Windows.Perception.Automation.Core
} // Windows.Perception.Automation
