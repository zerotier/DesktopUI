pub mod ui { // Windows.Gaming.UI
use ::prelude::*;
RT_CLASS!{static class GameBar}
impl RtActivatable<IGameBarStatics> for GameBar {}
impl GameBar {
    #[inline] pub fn add_visibility_changed(handler: &super::super::foundation::EventHandler<IInspectable>) -> Result<super::super::foundation::EventRegistrationToken> { unsafe {
        <Self as RtActivatable<IGameBarStatics>>::get_activation_factory().add_visibility_changed(handler)
    }}
    #[inline] pub fn remove_visibility_changed(token: super::super::foundation::EventRegistrationToken) -> Result<()> { unsafe {
        <Self as RtActivatable<IGameBarStatics>>::get_activation_factory().remove_visibility_changed(token)
    }}
    #[inline] pub fn add_is_input_redirected_changed(handler: &super::super::foundation::EventHandler<IInspectable>) -> Result<super::super::foundation::EventRegistrationToken> { unsafe {
        <Self as RtActivatable<IGameBarStatics>>::get_activation_factory().add_is_input_redirected_changed(handler)
    }}
    #[inline] pub fn remove_is_input_redirected_changed(token: super::super::foundation::EventRegistrationToken) -> Result<()> { unsafe {
        <Self as RtActivatable<IGameBarStatics>>::get_activation_factory().remove_is_input_redirected_changed(token)
    }}
    #[inline] pub fn get_visible() -> Result<bool> { unsafe {
        <Self as RtActivatable<IGameBarStatics>>::get_activation_factory().get_visible()
    }}
    #[inline] pub fn get_is_input_redirected() -> Result<bool> { unsafe {
        <Self as RtActivatable<IGameBarStatics>>::get_activation_factory().get_is_input_redirected()
    }}
}
DEFINE_CLSID!(GameBar(&[87,105,110,100,111,119,115,46,71,97,109,105,110,103,46,85,73,46,71,97,109,101,66,97,114,0]) [CLSID_GameBar]);
DEFINE_IID!(IID_IGameBarStatics, 498705042, 52344, 16755, 190, 69, 182, 30, 103, 40, 62, 167);
RT_INTERFACE!{static interface IGameBarStatics(IGameBarStaticsVtbl): IInspectable(IInspectableVtbl) [IID_IGameBarStatics] {
    fn add_VisibilityChanged(&self, handler: *mut super::super::foundation::EventHandler<IInspectable>, out: *mut super::super::foundation::EventRegistrationToken) -> HRESULT,
    fn remove_VisibilityChanged(&self, token: super::super::foundation::EventRegistrationToken) -> HRESULT,
    fn add_IsInputRedirectedChanged(&self, handler: *mut super::super::foundation::EventHandler<IInspectable>, out: *mut super::super::foundation::EventRegistrationToken) -> HRESULT,
    fn remove_IsInputRedirectedChanged(&self, token: super::super::foundation::EventRegistrationToken) -> HRESULT,
    fn get_Visible(&self, out: *mut bool) -> HRESULT,
    fn get_IsInputRedirected(&self, out: *mut bool) -> HRESULT
}}
impl IGameBarStatics {
    #[inline] pub unsafe fn add_visibility_changed(&self, handler: &super::super::foundation::EventHandler<IInspectable>) -> Result<super::super::foundation::EventRegistrationToken> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).add_VisibilityChanged)(self as *const _ as *mut _, handler as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn remove_visibility_changed(&self, token: super::super::foundation::EventRegistrationToken) -> Result<()> {
        let hr = ((*self.lpVtbl).remove_VisibilityChanged)(self as *const _ as *mut _, token);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn add_is_input_redirected_changed(&self, handler: &super::super::foundation::EventHandler<IInspectable>) -> Result<super::super::foundation::EventRegistrationToken> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).add_IsInputRedirectedChanged)(self as *const _ as *mut _, handler as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn remove_is_input_redirected_changed(&self, token: super::super::foundation::EventRegistrationToken) -> Result<()> {
        let hr = ((*self.lpVtbl).remove_IsInputRedirectedChanged)(self as *const _ as *mut _, token);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_visible(&self) -> Result<bool> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_Visible)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_is_input_redirected(&self) -> Result<bool> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_IsInputRedirected)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
}
RT_ENUM! { enum GameChatMessageOrigin: i32 {
    Voice (GameChatMessageOrigin_Voice) = 0, Text (GameChatMessageOrigin_Text) = 1,
}}
DEFINE_IID!(IID_IGameChatMessageReceivedEventArgs, 2726429169, 16313, 20034, 164, 3, 122, 252, 226, 2, 59, 30);
RT_INTERFACE!{interface IGameChatMessageReceivedEventArgs(IGameChatMessageReceivedEventArgsVtbl): IInspectable(IInspectableVtbl) [IID_IGameChatMessageReceivedEventArgs] {
    fn get_AppId(&self, out: *mut HSTRING) -> HRESULT,
    fn get_AppDisplayName(&self, out: *mut HSTRING) -> HRESULT,
    fn get_SenderName(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Message(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Origin(&self, out: *mut GameChatMessageOrigin) -> HRESULT
}}
impl IGameChatMessageReceivedEventArgs {
    #[inline] pub unsafe fn get_app_id(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_AppId)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_app_display_name(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_AppDisplayName)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_sender_name(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_SenderName)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_message(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Message)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_origin(&self) -> Result<GameChatMessageOrigin> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_Origin)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
}
RT_CLASS!{class GameChatMessageReceivedEventArgs: IGameChatMessageReceivedEventArgs}
DEFINE_IID!(IID_IGameChatOverlay, 4224075877, 63228, 19016, 174, 7, 3, 172, 110, 212, 55, 4);
RT_INTERFACE!{interface IGameChatOverlay(IGameChatOverlayVtbl): IInspectable(IInspectableVtbl) [IID_IGameChatOverlay] {
    fn get_DesiredPosition(&self, out: *mut GameChatOverlayPosition) -> HRESULT,
    fn put_DesiredPosition(&self, value: GameChatOverlayPosition) -> HRESULT,
    fn AddMessage(&self, sender: HSTRING, message: HSTRING, origin: GameChatMessageOrigin) -> HRESULT
}}
impl IGameChatOverlay {
    #[inline] pub unsafe fn get_desired_position(&self) -> Result<GameChatOverlayPosition> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_DesiredPosition)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_desired_position(&self, value: GameChatOverlayPosition) -> Result<()> {
        let hr = ((*self.lpVtbl).put_DesiredPosition)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn add_message(&self, sender: &HStringArg, message: &HStringArg, origin: GameChatMessageOrigin) -> Result<()> {
        let hr = ((*self.lpVtbl).AddMessage)(self as *const _ as *mut _, sender.get(), message.get(), origin);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
}
RT_CLASS!{class GameChatOverlay: IGameChatOverlay}
impl RtActivatable<IGameChatOverlayStatics> for GameChatOverlay {}
impl GameChatOverlay {
    #[inline] pub fn get_default() -> Result<ComPtr<GameChatOverlay>> { unsafe {
        <Self as RtActivatable<IGameChatOverlayStatics>>::get_activation_factory().get_default()
    }}
}
DEFINE_CLSID!(GameChatOverlay(&[87,105,110,100,111,119,115,46,71,97,109,105,110,103,46,85,73,46,71,97,109,101,67,104,97,116,79,118,101,114,108,97,121,0]) [CLSID_GameChatOverlay]);
DEFINE_IID!(IID_IGameChatOverlayMessageSource, 504853399, 23035, 20303, 142, 154, 128, 172, 248, 23, 116, 60);
RT_INTERFACE!{interface IGameChatOverlayMessageSource(IGameChatOverlayMessageSourceVtbl): IInspectable(IInspectableVtbl) [IID_IGameChatOverlayMessageSource] {
    fn add_MessageReceived(&self, handler: *mut super::super::foundation::TypedEventHandler<GameChatOverlayMessageSource, GameChatMessageReceivedEventArgs>, out: *mut super::super::foundation::EventRegistrationToken) -> HRESULT,
    fn remove_MessageReceived(&self, token: super::super::foundation::EventRegistrationToken) -> HRESULT,
    fn SetDelayBeforeClosingAfterMessageReceived(&self, value: super::super::foundation::TimeSpan) -> HRESULT
}}
impl IGameChatOverlayMessageSource {
    #[inline] pub unsafe fn add_message_received(&self, handler: &super::super::foundation::TypedEventHandler<GameChatOverlayMessageSource, GameChatMessageReceivedEventArgs>) -> Result<super::super::foundation::EventRegistrationToken> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).add_MessageReceived)(self as *const _ as *mut _, handler as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn remove_message_received(&self, token: super::super::foundation::EventRegistrationToken) -> Result<()> {
        let hr = ((*self.lpVtbl).remove_MessageReceived)(self as *const _ as *mut _, token);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_delay_before_closing_after_message_received(&self, value: super::super::foundation::TimeSpan) -> Result<()> {
        let hr = ((*self.lpVtbl).SetDelayBeforeClosingAfterMessageReceived)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
}
RT_CLASS!{class GameChatOverlayMessageSource: IGameChatOverlayMessageSource}
impl RtActivatable<IActivationFactory> for GameChatOverlayMessageSource {}
DEFINE_CLSID!(GameChatOverlayMessageSource(&[87,105,110,100,111,119,115,46,71,97,109,105,110,103,46,85,73,46,71,97,109,101,67,104,97,116,79,118,101,114,108,97,121,77,101,115,115,97,103,101,83,111,117,114,99,101,0]) [CLSID_GameChatOverlayMessageSource]);
RT_ENUM! { enum GameChatOverlayPosition: i32 {
    BottomCenter (GameChatOverlayPosition_BottomCenter) = 0, BottomLeft (GameChatOverlayPosition_BottomLeft) = 1, BottomRight (GameChatOverlayPosition_BottomRight) = 2, MiddleRight (GameChatOverlayPosition_MiddleRight) = 3, MiddleLeft (GameChatOverlayPosition_MiddleLeft) = 4, TopCenter (GameChatOverlayPosition_TopCenter) = 5, TopLeft (GameChatOverlayPosition_TopLeft) = 6, TopRight (GameChatOverlayPosition_TopRight) = 7,
}}
DEFINE_IID!(IID_IGameChatOverlayStatics, 2309813780, 30823, 18935, 150, 135, 37, 217, 219, 244, 68, 209);
RT_INTERFACE!{static interface IGameChatOverlayStatics(IGameChatOverlayStaticsVtbl): IInspectable(IInspectableVtbl) [IID_IGameChatOverlayStatics] {
    fn GetDefault(&self, out: *mut *mut GameChatOverlay) -> HRESULT
}}
impl IGameChatOverlayStatics {
    #[inline] pub unsafe fn get_default(&self) -> Result<ComPtr<GameChatOverlay>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetDefault)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
DEFINE_IID!(IID_IGameMonitor, 304300888, 56585, 17681, 173, 205, 141, 89, 117, 216, 16, 40);
RT_INTERFACE!{interface IGameMonitor(IGameMonitorVtbl): IInspectable(IInspectableVtbl) [IID_IGameMonitor] {
    fn RequestPermissionAsync(&self, out: *mut *mut super::super::foundation::IAsyncOperation<GameMonitoringPermission>) -> HRESULT
}}
impl IGameMonitor {
    #[inline] pub unsafe fn request_permission_async(&self) -> Result<ComPtr<super::super::foundation::IAsyncOperation<GameMonitoringPermission>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).RequestPermissionAsync)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
RT_CLASS!{class GameMonitor: IGameMonitor}
impl RtActivatable<IGameMonitorStatics> for GameMonitor {}
impl GameMonitor {
    #[inline] pub fn get_default() -> Result<ComPtr<GameMonitor>> { unsafe {
        <Self as RtActivatable<IGameMonitorStatics>>::get_activation_factory().get_default()
    }}
}
DEFINE_CLSID!(GameMonitor(&[87,105,110,100,111,119,115,46,71,97,109,105,110,103,46,85,73,46,71,97,109,101,77,111,110,105,116,111,114,0]) [CLSID_GameMonitor]);
RT_ENUM! { enum GameMonitoringPermission: i32 {
    Allowed (GameMonitoringPermission_Allowed) = 0, DeniedByUser (GameMonitoringPermission_DeniedByUser) = 1, DeniedBySystem (GameMonitoringPermission_DeniedBySystem) = 2,
}}
DEFINE_IID!(IID_IGameMonitorStatics, 291982132, 23264, 19380, 185, 31, 138, 203, 72, 21, 154, 113);
RT_INTERFACE!{static interface IGameMonitorStatics(IGameMonitorStaticsVtbl): IInspectable(IInspectableVtbl) [IID_IGameMonitorStatics] {
    fn GetDefault(&self, out: *mut *mut GameMonitor) -> HRESULT
}}
impl IGameMonitorStatics {
    #[inline] pub unsafe fn get_default(&self) -> Result<ComPtr<GameMonitor>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetDefault)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
DEFINE_IID!(IID_IGameUIProviderActivatedEventArgs, 2813534270, 51959, 19949, 187, 210, 71, 222, 67, 187, 109, 213);
RT_INTERFACE!{interface IGameUIProviderActivatedEventArgs(IGameUIProviderActivatedEventArgsVtbl): IInspectable(IInspectableVtbl) [IID_IGameUIProviderActivatedEventArgs] {
    fn get_GameUIArgs(&self, out: *mut *mut super::super::foundation::collections::ValueSet) -> HRESULT,
    fn ReportCompleted(&self, results: *mut super::super::foundation::collections::ValueSet) -> HRESULT
}}
impl IGameUIProviderActivatedEventArgs {
    #[inline] pub unsafe fn get_game_uiargs(&self) -> Result<ComPtr<super::super::foundation::collections::ValueSet>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_GameUIArgs)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn report_completed(&self, results: &super::super::foundation::collections::ValueSet) -> Result<()> {
        let hr = ((*self.lpVtbl).ReportCompleted)(self as *const _ as *mut _, results as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
}
RT_CLASS!{class GameUIProviderActivatedEventArgs: IGameUIProviderActivatedEventArgs}
} // Windows.Gaming.UI
pub mod input { // Windows.Gaming.Input
use ::prelude::*;
DEFINE_IID!(IID_IArcadeStick, 2974438301, 48891, 19585, 128, 81, 21, 236, 243, 177, 48, 54);
RT_INTERFACE!{interface IArcadeStick(IArcadeStickVtbl): IInspectable(IInspectableVtbl) [IID_IArcadeStick] {
    fn GetButtonLabel(&self, button: ArcadeStickButtons, out: *mut GameControllerButtonLabel) -> HRESULT,
    fn GetCurrentReading(&self, out: *mut ArcadeStickReading) -> HRESULT
}}
impl IArcadeStick {
    #[inline] pub unsafe fn get_button_label(&self, button: ArcadeStickButtons) -> Result<GameControllerButtonLabel> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).GetButtonLabel)(self as *const _ as *mut _, button, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_current_reading(&self) -> Result<ArcadeStickReading> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).GetCurrentReading)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
}
RT_CLASS!{class ArcadeStick: IArcadeStick}
impl RtActivatable<IArcadeStickStatics> for ArcadeStick {}
impl RtActivatable<IArcadeStickStatics2> for ArcadeStick {}
impl ArcadeStick {
    #[inline] pub fn add_arcade_stick_added(value: &super::super::foundation::EventHandler<ArcadeStick>) -> Result<super::super::foundation::EventRegistrationToken> { unsafe {
        <Self as RtActivatable<IArcadeStickStatics>>::get_activation_factory().add_arcade_stick_added(value)
    }}
    #[inline] pub fn remove_arcade_stick_added(token: super::super::foundation::EventRegistrationToken) -> Result<()> { unsafe {
        <Self as RtActivatable<IArcadeStickStatics>>::get_activation_factory().remove_arcade_stick_added(token)
    }}
    #[inline] pub fn add_arcade_stick_removed(value: &super::super::foundation::EventHandler<ArcadeStick>) -> Result<super::super::foundation::EventRegistrationToken> { unsafe {
        <Self as RtActivatable<IArcadeStickStatics>>::get_activation_factory().add_arcade_stick_removed(value)
    }}
    #[inline] pub fn remove_arcade_stick_removed(token: super::super::foundation::EventRegistrationToken) -> Result<()> { unsafe {
        <Self as RtActivatable<IArcadeStickStatics>>::get_activation_factory().remove_arcade_stick_removed(token)
    }}
    #[inline] pub fn get_arcade_sticks() -> Result<ComPtr<super::super::foundation::collections::IVectorView<ArcadeStick>>> { unsafe {
        <Self as RtActivatable<IArcadeStickStatics>>::get_activation_factory().get_arcade_sticks()
    }}
    #[inline] pub fn from_game_controller(gameController: &IGameController) -> Result<ComPtr<ArcadeStick>> { unsafe {
        <Self as RtActivatable<IArcadeStickStatics2>>::get_activation_factory().from_game_controller(gameController)
    }}
}
DEFINE_CLSID!(ArcadeStick(&[87,105,110,100,111,119,115,46,71,97,109,105,110,103,46,73,110,112,117,116,46,65,114,99,97,100,101,83,116,105,99,107,0]) [CLSID_ArcadeStick]);
RT_ENUM! { enum ArcadeStickButtons: u32 {
    None (ArcadeStickButtons_None) = 0, StickUp (ArcadeStickButtons_StickUp) = 1, StickDown (ArcadeStickButtons_StickDown) = 2, StickLeft (ArcadeStickButtons_StickLeft) = 4, StickRight (ArcadeStickButtons_StickRight) = 8, Action1 (ArcadeStickButtons_Action1) = 16, Action2 (ArcadeStickButtons_Action2) = 32, Action3 (ArcadeStickButtons_Action3) = 64, Action4 (ArcadeStickButtons_Action4) = 128, Action5 (ArcadeStickButtons_Action5) = 256, Action6 (ArcadeStickButtons_Action6) = 512, Special1 (ArcadeStickButtons_Special1) = 1024, Special2 (ArcadeStickButtons_Special2) = 2048,
}}
RT_STRUCT! { struct ArcadeStickReading {
    Timestamp: u64, Buttons: ArcadeStickButtons,
}}
DEFINE_IID!(IID_IArcadeStickStatics, 1547155656, 14257, 19160, 148, 88, 32, 15, 26, 48, 1, 142);
RT_INTERFACE!{static interface IArcadeStickStatics(IArcadeStickStaticsVtbl): IInspectable(IInspectableVtbl) [IID_IArcadeStickStatics] {
    fn add_ArcadeStickAdded(&self, value: *mut super::super::foundation::EventHandler<ArcadeStick>, out: *mut super::super::foundation::EventRegistrationToken) -> HRESULT,
    fn remove_ArcadeStickAdded(&self, token: super::super::foundation::EventRegistrationToken) -> HRESULT,
    fn add_ArcadeStickRemoved(&self, value: *mut super::super::foundation::EventHandler<ArcadeStick>, out: *mut super::super::foundation::EventRegistrationToken) -> HRESULT,
    fn remove_ArcadeStickRemoved(&self, token: super::super::foundation::EventRegistrationToken) -> HRESULT,
    fn get_ArcadeSticks(&self, out: *mut *mut super::super::foundation::collections::IVectorView<ArcadeStick>) -> HRESULT
}}
impl IArcadeStickStatics {
    #[inline] pub unsafe fn add_arcade_stick_added(&self, value: &super::super::foundation::EventHandler<ArcadeStick>) -> Result<super::super::foundation::EventRegistrationToken> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).add_ArcadeStickAdded)(self as *const _ as *mut _, value as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn remove_arcade_stick_added(&self, token: super::super::foundation::EventRegistrationToken) -> Result<()> {
        let hr = ((*self.lpVtbl).remove_ArcadeStickAdded)(self as *const _ as *mut _, token);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn add_arcade_stick_removed(&self, value: &super::super::foundation::EventHandler<ArcadeStick>) -> Result<super::super::foundation::EventRegistrationToken> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).add_ArcadeStickRemoved)(self as *const _ as *mut _, value as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn remove_arcade_stick_removed(&self, token: super::super::foundation::EventRegistrationToken) -> Result<()> {
        let hr = ((*self.lpVtbl).remove_ArcadeStickRemoved)(self as *const _ as *mut _, token);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_arcade_sticks(&self) -> Result<ComPtr<super::super::foundation::collections::IVectorView<ArcadeStick>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_ArcadeSticks)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
DEFINE_IID!(IID_IArcadeStickStatics2, 1387648836, 48006, 17498, 181, 156, 89, 111, 14, 42, 73, 223);
RT_INTERFACE!{static interface IArcadeStickStatics2(IArcadeStickStatics2Vtbl): IInspectable(IInspectableVtbl) [IID_IArcadeStickStatics2] {
    fn FromGameController(&self, gameController: *mut IGameController, out: *mut *mut ArcadeStick) -> HRESULT
}}
impl IArcadeStickStatics2 {
    #[inline] pub unsafe fn from_game_controller(&self, gameController: &IGameController) -> Result<ComPtr<ArcadeStick>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).FromGameController)(self as *const _ as *mut _, gameController as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
DEFINE_IID!(IID_IFlightStick, 3030564892, 47163, 17497, 161, 169, 151, 176, 60, 51, 218, 124);
RT_INTERFACE!{interface IFlightStick(IFlightStickVtbl): IInspectable(IInspectableVtbl) [IID_IFlightStick] {
    fn get_HatSwitchKind(&self, out: *mut GameControllerSwitchKind) -> HRESULT,
    fn GetButtonLabel(&self, button: FlightStickButtons, out: *mut GameControllerButtonLabel) -> HRESULT,
    fn GetCurrentReading(&self, out: *mut FlightStickReading) -> HRESULT
}}
impl IFlightStick {
    #[inline] pub unsafe fn get_hat_switch_kind(&self) -> Result<GameControllerSwitchKind> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_HatSwitchKind)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_button_label(&self, button: FlightStickButtons) -> Result<GameControllerButtonLabel> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).GetButtonLabel)(self as *const _ as *mut _, button, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_current_reading(&self) -> Result<FlightStickReading> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).GetCurrentReading)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
}
RT_CLASS!{class FlightStick: IFlightStick}
impl RtActivatable<IFlightStickStatics> for FlightStick {}
impl FlightStick {
    #[inline] pub fn add_flight_stick_added(value: &super::super::foundation::EventHandler<FlightStick>) -> Result<super::super::foundation::EventRegistrationToken> { unsafe {
        <Self as RtActivatable<IFlightStickStatics>>::get_activation_factory().add_flight_stick_added(value)
    }}
    #[inline] pub fn remove_flight_stick_added(token: super::super::foundation::EventRegistrationToken) -> Result<()> { unsafe {
        <Self as RtActivatable<IFlightStickStatics>>::get_activation_factory().remove_flight_stick_added(token)
    }}
    #[inline] pub fn add_flight_stick_removed(value: &super::super::foundation::EventHandler<FlightStick>) -> Result<super::super::foundation::EventRegistrationToken> { unsafe {
        <Self as RtActivatable<IFlightStickStatics>>::get_activation_factory().add_flight_stick_removed(value)
    }}
    #[inline] pub fn remove_flight_stick_removed(token: super::super::foundation::EventRegistrationToken) -> Result<()> { unsafe {
        <Self as RtActivatable<IFlightStickStatics>>::get_activation_factory().remove_flight_stick_removed(token)
    }}
    #[inline] pub fn get_flight_sticks() -> Result<ComPtr<super::super::foundation::collections::IVectorView<FlightStick>>> { unsafe {
        <Self as RtActivatable<IFlightStickStatics>>::get_activation_factory().get_flight_sticks()
    }}
    #[inline] pub fn from_game_controller(gameController: &IGameController) -> Result<ComPtr<FlightStick>> { unsafe {
        <Self as RtActivatable<IFlightStickStatics>>::get_activation_factory().from_game_controller(gameController)
    }}
}
DEFINE_CLSID!(FlightStick(&[87,105,110,100,111,119,115,46,71,97,109,105,110,103,46,73,110,112,117,116,46,70,108,105,103,104,116,83,116,105,99,107,0]) [CLSID_FlightStick]);
RT_ENUM! { enum FlightStickButtons: u32 {
    None (FlightStickButtons_None) = 0, FirePrimary (FlightStickButtons_FirePrimary) = 1, FireSecondary (FlightStickButtons_FireSecondary) = 2,
}}
RT_STRUCT! { struct FlightStickReading {
    Timestamp: u64, Buttons: FlightStickButtons, HatSwitch: GameControllerSwitchPosition, Roll: f64, Pitch: f64, Yaw: f64, Throttle: f64,
}}
DEFINE_IID!(IID_IFlightStickStatics, 1427411530, 65228, 17246, 131, 220, 92, 236, 138, 24, 165, 32);
RT_INTERFACE!{static interface IFlightStickStatics(IFlightStickStaticsVtbl): IInspectable(IInspectableVtbl) [IID_IFlightStickStatics] {
    fn add_FlightStickAdded(&self, value: *mut super::super::foundation::EventHandler<FlightStick>, out: *mut super::super::foundation::EventRegistrationToken) -> HRESULT,
    fn remove_FlightStickAdded(&self, token: super::super::foundation::EventRegistrationToken) -> HRESULT,
    fn add_FlightStickRemoved(&self, value: *mut super::super::foundation::EventHandler<FlightStick>, out: *mut super::super::foundation::EventRegistrationToken) -> HRESULT,
    fn remove_FlightStickRemoved(&self, token: super::super::foundation::EventRegistrationToken) -> HRESULT,
    fn get_FlightSticks(&self, out: *mut *mut super::super::foundation::collections::IVectorView<FlightStick>) -> HRESULT,
    fn FromGameController(&self, gameController: *mut IGameController, out: *mut *mut FlightStick) -> HRESULT
}}
impl IFlightStickStatics {
    #[inline] pub unsafe fn add_flight_stick_added(&self, value: &super::super::foundation::EventHandler<FlightStick>) -> Result<super::super::foundation::EventRegistrationToken> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).add_FlightStickAdded)(self as *const _ as *mut _, value as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn remove_flight_stick_added(&self, token: super::super::foundation::EventRegistrationToken) -> Result<()> {
        let hr = ((*self.lpVtbl).remove_FlightStickAdded)(self as *const _ as *mut _, token);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn add_flight_stick_removed(&self, value: &super::super::foundation::EventHandler<FlightStick>) -> Result<super::super::foundation::EventRegistrationToken> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).add_FlightStickRemoved)(self as *const _ as *mut _, value as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn remove_flight_stick_removed(&self, token: super::super::foundation::EventRegistrationToken) -> Result<()> {
        let hr = ((*self.lpVtbl).remove_FlightStickRemoved)(self as *const _ as *mut _, token);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_flight_sticks(&self) -> Result<ComPtr<super::super::foundation::collections::IVectorView<FlightStick>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_FlightSticks)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn from_game_controller(&self, gameController: &IGameController) -> Result<ComPtr<FlightStick>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).FromGameController)(self as *const _ as *mut _, gameController as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
DEFINE_IID!(IID_IGameController, 464479522, 24420, 17093, 130, 103, 185, 254, 34, 21, 191, 189);
RT_INTERFACE!{interface IGameController(IGameControllerVtbl): IInspectable(IInspectableVtbl) [IID_IGameController] {
    fn add_HeadsetConnected(&self, value: *mut super::super::foundation::TypedEventHandler<IGameController, Headset>, out: *mut super::super::foundation::EventRegistrationToken) -> HRESULT,
    fn remove_HeadsetConnected(&self, token: super::super::foundation::EventRegistrationToken) -> HRESULT,
    fn add_HeadsetDisconnected(&self, value: *mut super::super::foundation::TypedEventHandler<IGameController, Headset>, out: *mut super::super::foundation::EventRegistrationToken) -> HRESULT,
    fn remove_HeadsetDisconnected(&self, token: super::super::foundation::EventRegistrationToken) -> HRESULT,
    #[cfg(feature="windows-system")] fn add_UserChanged(&self, value: *mut super::super::foundation::TypedEventHandler<IGameController, super::super::system::UserChangedEventArgs>, out: *mut super::super::foundation::EventRegistrationToken) -> HRESULT,
    fn remove_UserChanged(&self, token: super::super::foundation::EventRegistrationToken) -> HRESULT,
    fn get_Headset(&self, out: *mut *mut Headset) -> HRESULT,
    fn get_IsWireless(&self, out: *mut bool) -> HRESULT,
    #[cfg(feature="windows-system")] fn get_User(&self, out: *mut *mut super::super::system::User) -> HRESULT
}}
impl IGameController {
    #[inline] pub unsafe fn add_headset_connected(&self, value: &super::super::foundation::TypedEventHandler<IGameController, Headset>) -> Result<super::super::foundation::EventRegistrationToken> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).add_HeadsetConnected)(self as *const _ as *mut _, value as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn remove_headset_connected(&self, token: super::super::foundation::EventRegistrationToken) -> Result<()> {
        let hr = ((*self.lpVtbl).remove_HeadsetConnected)(self as *const _ as *mut _, token);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn add_headset_disconnected(&self, value: &super::super::foundation::TypedEventHandler<IGameController, Headset>) -> Result<super::super::foundation::EventRegistrationToken> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).add_HeadsetDisconnected)(self as *const _ as *mut _, value as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn remove_headset_disconnected(&self, token: super::super::foundation::EventRegistrationToken) -> Result<()> {
        let hr = ((*self.lpVtbl).remove_HeadsetDisconnected)(self as *const _ as *mut _, token);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[cfg(feature="windows-system")] #[inline] pub unsafe fn add_user_changed(&self, value: &super::super::foundation::TypedEventHandler<IGameController, super::super::system::UserChangedEventArgs>) -> Result<super::super::foundation::EventRegistrationToken> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).add_UserChanged)(self as *const _ as *mut _, value as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn remove_user_changed(&self, token: super::super::foundation::EventRegistrationToken) -> Result<()> {
        let hr = ((*self.lpVtbl).remove_UserChanged)(self as *const _ as *mut _, token);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_headset(&self) -> Result<ComPtr<Headset>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Headset)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_is_wireless(&self) -> Result<bool> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_IsWireless)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[cfg(feature="windows-system")] #[inline] pub unsafe fn get_user(&self) -> Result<ComPtr<super::super::system::User>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_User)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
DEFINE_IID!(IID_IGameControllerBatteryInfo, 3706504833, 14691, 19878, 149, 93, 85, 63, 59, 111, 97, 97);
RT_INTERFACE!{interface IGameControllerBatteryInfo(IGameControllerBatteryInfoVtbl): IInspectable(IInspectableVtbl) [IID_IGameControllerBatteryInfo] {
    #[cfg(feature="windows-devices")] fn TryGetBatteryReport(&self, out: *mut *mut super::super::devices::power::BatteryReport) -> HRESULT
}}
impl IGameControllerBatteryInfo {
    #[cfg(feature="windows-devices")] #[inline] pub unsafe fn try_get_battery_report(&self) -> Result<ComPtr<super::super::devices::power::BatteryReport>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).TryGetBatteryReport)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
RT_ENUM! { enum GameControllerButtonLabel: i32 {
    None (GameControllerButtonLabel_None) = 0, XboxBack (GameControllerButtonLabel_XboxBack) = 1, XboxStart (GameControllerButtonLabel_XboxStart) = 2, XboxMenu (GameControllerButtonLabel_XboxMenu) = 3, XboxView (GameControllerButtonLabel_XboxView) = 4, XboxUp (GameControllerButtonLabel_XboxUp) = 5, XboxDown (GameControllerButtonLabel_XboxDown) = 6, XboxLeft (GameControllerButtonLabel_XboxLeft) = 7, XboxRight (GameControllerButtonLabel_XboxRight) = 8, XboxA (GameControllerButtonLabel_XboxA) = 9, XboxB (GameControllerButtonLabel_XboxB) = 10, XboxX (GameControllerButtonLabel_XboxX) = 11, XboxY (GameControllerButtonLabel_XboxY) = 12, XboxLeftBumper (GameControllerButtonLabel_XboxLeftBumper) = 13, XboxLeftTrigger (GameControllerButtonLabel_XboxLeftTrigger) = 14, XboxLeftStickButton (GameControllerButtonLabel_XboxLeftStickButton) = 15, XboxRightBumper (GameControllerButtonLabel_XboxRightBumper) = 16, XboxRightTrigger (GameControllerButtonLabel_XboxRightTrigger) = 17, XboxRightStickButton (GameControllerButtonLabel_XboxRightStickButton) = 18, XboxPaddle1 (GameControllerButtonLabel_XboxPaddle1) = 19, XboxPaddle2 (GameControllerButtonLabel_XboxPaddle2) = 20, XboxPaddle3 (GameControllerButtonLabel_XboxPaddle3) = 21, XboxPaddle4 (GameControllerButtonLabel_XboxPaddle4) = 22, Mode (GameControllerButtonLabel_Mode) = 23, Select (GameControllerButtonLabel_Select) = 24, Menu (GameControllerButtonLabel_Menu) = 25, View (GameControllerButtonLabel_View) = 26, Back (GameControllerButtonLabel_Back) = 27, Start (GameControllerButtonLabel_Start) = 28, Options (GameControllerButtonLabel_Options) = 29, Share (GameControllerButtonLabel_Share) = 30, Up (GameControllerButtonLabel_Up) = 31, Down (GameControllerButtonLabel_Down) = 32, Left (GameControllerButtonLabel_Left) = 33, Right (GameControllerButtonLabel_Right) = 34, LetterA (GameControllerButtonLabel_LetterA) = 35, LetterB (GameControllerButtonLabel_LetterB) = 36, LetterC (GameControllerButtonLabel_LetterC) = 37, LetterL (GameControllerButtonLabel_LetterL) = 38, LetterR (GameControllerButtonLabel_LetterR) = 39, LetterX (GameControllerButtonLabel_LetterX) = 40, LetterY (GameControllerButtonLabel_LetterY) = 41, LetterZ (GameControllerButtonLabel_LetterZ) = 42, Cross (GameControllerButtonLabel_Cross) = 43, Circle (GameControllerButtonLabel_Circle) = 44, Square (GameControllerButtonLabel_Square) = 45, Triangle (GameControllerButtonLabel_Triangle) = 46, LeftBumper (GameControllerButtonLabel_LeftBumper) = 47, LeftTrigger (GameControllerButtonLabel_LeftTrigger) = 48, LeftStickButton (GameControllerButtonLabel_LeftStickButton) = 49, Left1 (GameControllerButtonLabel_Left1) = 50, Left2 (GameControllerButtonLabel_Left2) = 51, Left3 (GameControllerButtonLabel_Left3) = 52, RightBumper (GameControllerButtonLabel_RightBumper) = 53, RightTrigger (GameControllerButtonLabel_RightTrigger) = 54, RightStickButton (GameControllerButtonLabel_RightStickButton) = 55, Right1 (GameControllerButtonLabel_Right1) = 56, Right2 (GameControllerButtonLabel_Right2) = 57, Right3 (GameControllerButtonLabel_Right3) = 58, Paddle1 (GameControllerButtonLabel_Paddle1) = 59, Paddle2 (GameControllerButtonLabel_Paddle2) = 60, Paddle3 (GameControllerButtonLabel_Paddle3) = 61, Paddle4 (GameControllerButtonLabel_Paddle4) = 62, Plus (GameControllerButtonLabel_Plus) = 63, Minus (GameControllerButtonLabel_Minus) = 64, DownLeftArrow (GameControllerButtonLabel_DownLeftArrow) = 65, DialLeft (GameControllerButtonLabel_DialLeft) = 66, DialRight (GameControllerButtonLabel_DialRight) = 67, Suspension (GameControllerButtonLabel_Suspension) = 68,
}}
RT_ENUM! { enum GameControllerSwitchKind: i32 {
    TwoWay (GameControllerSwitchKind_TwoWay) = 0, FourWay (GameControllerSwitchKind_FourWay) = 1, EightWay (GameControllerSwitchKind_EightWay) = 2,
}}
RT_ENUM! { enum GameControllerSwitchPosition: i32 {
    Center (GameControllerSwitchPosition_Center) = 0, Up (GameControllerSwitchPosition_Up) = 1, UpRight (GameControllerSwitchPosition_UpRight) = 2, Right (GameControllerSwitchPosition_Right) = 3, DownRight (GameControllerSwitchPosition_DownRight) = 4, Down (GameControllerSwitchPosition_Down) = 5, DownLeft (GameControllerSwitchPosition_DownLeft) = 6, Left (GameControllerSwitchPosition_Left) = 7, UpLeft (GameControllerSwitchPosition_UpLeft) = 8,
}}
DEFINE_IID!(IID_IGamepad, 3162223676, 2665, 14595, 158, 157, 165, 15, 134, 164, 93, 229);
RT_INTERFACE!{interface IGamepad(IGamepadVtbl): IInspectable(IInspectableVtbl) [IID_IGamepad] {
    fn get_Vibration(&self, out: *mut GamepadVibration) -> HRESULT,
    fn put_Vibration(&self, value: GamepadVibration) -> HRESULT,
    fn GetCurrentReading(&self, out: *mut GamepadReading) -> HRESULT
}}
impl IGamepad {
    #[inline] pub unsafe fn get_vibration(&self) -> Result<GamepadVibration> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_Vibration)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_vibration(&self, value: GamepadVibration) -> Result<()> {
        let hr = ((*self.lpVtbl).put_Vibration)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_current_reading(&self) -> Result<GamepadReading> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).GetCurrentReading)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
}
RT_CLASS!{class Gamepad: IGamepad}
impl RtActivatable<IGamepadStatics> for Gamepad {}
impl RtActivatable<IGamepadStatics2> for Gamepad {}
impl Gamepad {
    #[inline] pub fn add_gamepad_added(value: &super::super::foundation::EventHandler<Gamepad>) -> Result<super::super::foundation::EventRegistrationToken> { unsafe {
        <Self as RtActivatable<IGamepadStatics>>::get_activation_factory().add_gamepad_added(value)
    }}
    #[inline] pub fn remove_gamepad_added(token: super::super::foundation::EventRegistrationToken) -> Result<()> { unsafe {
        <Self as RtActivatable<IGamepadStatics>>::get_activation_factory().remove_gamepad_added(token)
    }}
    #[inline] pub fn add_gamepad_removed(value: &super::super::foundation::EventHandler<Gamepad>) -> Result<super::super::foundation::EventRegistrationToken> { unsafe {
        <Self as RtActivatable<IGamepadStatics>>::get_activation_factory().add_gamepad_removed(value)
    }}
    #[inline] pub fn remove_gamepad_removed(token: super::super::foundation::EventRegistrationToken) -> Result<()> { unsafe {
        <Self as RtActivatable<IGamepadStatics>>::get_activation_factory().remove_gamepad_removed(token)
    }}
    #[inline] pub fn get_gamepads() -> Result<ComPtr<super::super::foundation::collections::IVectorView<Gamepad>>> { unsafe {
        <Self as RtActivatable<IGamepadStatics>>::get_activation_factory().get_gamepads()
    }}
    #[inline] pub fn from_game_controller(gameController: &IGameController) -> Result<ComPtr<Gamepad>> { unsafe {
        <Self as RtActivatable<IGamepadStatics2>>::get_activation_factory().from_game_controller(gameController)
    }}
}
DEFINE_CLSID!(Gamepad(&[87,105,110,100,111,119,115,46,71,97,109,105,110,103,46,73,110,112,117,116,46,71,97,109,101,112,97,100,0]) [CLSID_Gamepad]);
DEFINE_IID!(IID_IGamepad2, 1008110013, 22805, 16965, 176, 192, 200, 159, 174, 3, 8, 255);
RT_INTERFACE!{interface IGamepad2(IGamepad2Vtbl): IInspectable(IInspectableVtbl) [IID_IGamepad2] {
    fn GetButtonLabel(&self, button: GamepadButtons, out: *mut GameControllerButtonLabel) -> HRESULT
}}
impl IGamepad2 {
    #[inline] pub unsafe fn get_button_label(&self, button: GamepadButtons) -> Result<GameControllerButtonLabel> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).GetButtonLabel)(self as *const _ as *mut _, button, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
}
RT_ENUM! { enum GamepadButtons: u32 {
    None (GamepadButtons_None) = 0, Menu (GamepadButtons_Menu) = 1, View (GamepadButtons_View) = 2, A (GamepadButtons_A) = 4, B (GamepadButtons_B) = 8, X (GamepadButtons_X) = 16, Y (GamepadButtons_Y) = 32, DPadUp (GamepadButtons_DPadUp) = 64, DPadDown (GamepadButtons_DPadDown) = 128, DPadLeft (GamepadButtons_DPadLeft) = 256, DPadRight (GamepadButtons_DPadRight) = 512, LeftShoulder (GamepadButtons_LeftShoulder) = 1024, RightShoulder (GamepadButtons_RightShoulder) = 2048, LeftThumbstick (GamepadButtons_LeftThumbstick) = 4096, RightThumbstick (GamepadButtons_RightThumbstick) = 8192, Paddle1 (GamepadButtons_Paddle1) = 16384, Paddle2 (GamepadButtons_Paddle2) = 32768, Paddle3 (GamepadButtons_Paddle3) = 65536, Paddle4 (GamepadButtons_Paddle4) = 131072,
}}
RT_STRUCT! { struct GamepadReading {
    Timestamp: u64, Buttons: GamepadButtons, LeftTrigger: f64, RightTrigger: f64, LeftThumbstickX: f64, LeftThumbstickY: f64, RightThumbstickX: f64, RightThumbstickY: f64,
}}
DEFINE_IID!(IID_IGamepadStatics, 2344412457, 54428, 14825, 149, 96, 228, 125, 222, 150, 183, 200);
RT_INTERFACE!{static interface IGamepadStatics(IGamepadStaticsVtbl): IInspectable(IInspectableVtbl) [IID_IGamepadStatics] {
    fn add_GamepadAdded(&self, value: *mut super::super::foundation::EventHandler<Gamepad>, out: *mut super::super::foundation::EventRegistrationToken) -> HRESULT,
    fn remove_GamepadAdded(&self, token: super::super::foundation::EventRegistrationToken) -> HRESULT,
    fn add_GamepadRemoved(&self, value: *mut super::super::foundation::EventHandler<Gamepad>, out: *mut super::super::foundation::EventRegistrationToken) -> HRESULT,
    fn remove_GamepadRemoved(&self, token: super::super::foundation::EventRegistrationToken) -> HRESULT,
    fn get_Gamepads(&self, out: *mut *mut super::super::foundation::collections::IVectorView<Gamepad>) -> HRESULT
}}
impl IGamepadStatics {
    #[inline] pub unsafe fn add_gamepad_added(&self, value: &super::super::foundation::EventHandler<Gamepad>) -> Result<super::super::foundation::EventRegistrationToken> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).add_GamepadAdded)(self as *const _ as *mut _, value as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn remove_gamepad_added(&self, token: super::super::foundation::EventRegistrationToken) -> Result<()> {
        let hr = ((*self.lpVtbl).remove_GamepadAdded)(self as *const _ as *mut _, token);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn add_gamepad_removed(&self, value: &super::super::foundation::EventHandler<Gamepad>) -> Result<super::super::foundation::EventRegistrationToken> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).add_GamepadRemoved)(self as *const _ as *mut _, value as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn remove_gamepad_removed(&self, token: super::super::foundation::EventRegistrationToken) -> Result<()> {
        let hr = ((*self.lpVtbl).remove_GamepadRemoved)(self as *const _ as *mut _, token);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_gamepads(&self) -> Result<ComPtr<super::super::foundation::collections::IVectorView<Gamepad>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Gamepads)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
DEFINE_IID!(IID_IGamepadStatics2, 1114074565, 2134, 18372, 146, 19, 179, 149, 80, 76, 58, 60);
RT_INTERFACE!{static interface IGamepadStatics2(IGamepadStatics2Vtbl): IInspectable(IInspectableVtbl) [IID_IGamepadStatics2] {
    fn FromGameController(&self, gameController: *mut IGameController, out: *mut *mut Gamepad) -> HRESULT
}}
impl IGamepadStatics2 {
    #[inline] pub unsafe fn from_game_controller(&self, gameController: &IGameController) -> Result<ComPtr<Gamepad>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).FromGameController)(self as *const _ as *mut _, gameController as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
RT_STRUCT! { struct GamepadVibration {
    LeftMotor: f64, RightMotor: f64, LeftTrigger: f64, RightTrigger: f64,
}}
DEFINE_IID!(IID_IHeadset, 1070683887, 26917, 16296, 145, 129, 2, 156, 82, 35, 174, 59);
RT_INTERFACE!{interface IHeadset(IHeadsetVtbl): IInspectable(IInspectableVtbl) [IID_IHeadset] {
    fn get_CaptureDeviceId(&self, out: *mut HSTRING) -> HRESULT,
    fn get_RenderDeviceId(&self, out: *mut HSTRING) -> HRESULT
}}
impl IHeadset {
    #[inline] pub unsafe fn get_capture_device_id(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_CaptureDeviceId)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_render_device_id(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_RenderDeviceId)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
}
RT_CLASS!{class Headset: IHeadset}
RT_ENUM! { enum OptionalUINavigationButtons: u32 {
    None (OptionalUINavigationButtons_None) = 0, Context1 (OptionalUINavigationButtons_Context1) = 1, Context2 (OptionalUINavigationButtons_Context2) = 2, Context3 (OptionalUINavigationButtons_Context3) = 4, Context4 (OptionalUINavigationButtons_Context4) = 8, PageUp (OptionalUINavigationButtons_PageUp) = 16, PageDown (OptionalUINavigationButtons_PageDown) = 32, PageLeft (OptionalUINavigationButtons_PageLeft) = 64, PageRight (OptionalUINavigationButtons_PageRight) = 128, ScrollUp (OptionalUINavigationButtons_ScrollUp) = 256, ScrollDown (OptionalUINavigationButtons_ScrollDown) = 512, ScrollLeft (OptionalUINavigationButtons_ScrollLeft) = 1024, ScrollRight (OptionalUINavigationButtons_ScrollRight) = 2048,
}}
DEFINE_IID!(IID_IRacingWheel, 4115031407, 57606, 19586, 169, 15, 85, 64, 18, 144, 75, 133);
RT_INTERFACE!{interface IRacingWheel(IRacingWheelVtbl): IInspectable(IInspectableVtbl) [IID_IRacingWheel] {
    fn get_HasClutch(&self, out: *mut bool) -> HRESULT,
    fn get_HasHandbrake(&self, out: *mut bool) -> HRESULT,
    fn get_HasPatternShifter(&self, out: *mut bool) -> HRESULT,
    fn get_MaxPatternShifterGear(&self, out: *mut i32) -> HRESULT,
    fn get_MaxWheelAngle(&self, out: *mut f64) -> HRESULT,
    fn get_WheelMotor(&self, out: *mut *mut forcefeedback::ForceFeedbackMotor) -> HRESULT,
    fn GetButtonLabel(&self, button: RacingWheelButtons, out: *mut GameControllerButtonLabel) -> HRESULT,
    fn GetCurrentReading(&self, out: *mut RacingWheelReading) -> HRESULT
}}
impl IRacingWheel {
    #[inline] pub unsafe fn get_has_clutch(&self) -> Result<bool> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_HasClutch)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_has_handbrake(&self) -> Result<bool> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_HasHandbrake)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_has_pattern_shifter(&self) -> Result<bool> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_HasPatternShifter)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_max_pattern_shifter_gear(&self) -> Result<i32> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_MaxPatternShifterGear)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_max_wheel_angle(&self) -> Result<f64> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_MaxWheelAngle)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_wheel_motor(&self) -> Result<ComPtr<forcefeedback::ForceFeedbackMotor>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_WheelMotor)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_button_label(&self, button: RacingWheelButtons) -> Result<GameControllerButtonLabel> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).GetButtonLabel)(self as *const _ as *mut _, button, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_current_reading(&self) -> Result<RacingWheelReading> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).GetCurrentReading)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
}
RT_CLASS!{class RacingWheel: IRacingWheel}
impl RtActivatable<IRacingWheelStatics> for RacingWheel {}
impl RtActivatable<IRacingWheelStatics2> for RacingWheel {}
impl RacingWheel {
    #[inline] pub fn add_racing_wheel_added(value: &super::super::foundation::EventHandler<RacingWheel>) -> Result<super::super::foundation::EventRegistrationToken> { unsafe {
        <Self as RtActivatable<IRacingWheelStatics>>::get_activation_factory().add_racing_wheel_added(value)
    }}
    #[inline] pub fn remove_racing_wheel_added(token: super::super::foundation::EventRegistrationToken) -> Result<()> { unsafe {
        <Self as RtActivatable<IRacingWheelStatics>>::get_activation_factory().remove_racing_wheel_added(token)
    }}
    #[inline] pub fn add_racing_wheel_removed(value: &super::super::foundation::EventHandler<RacingWheel>) -> Result<super::super::foundation::EventRegistrationToken> { unsafe {
        <Self as RtActivatable<IRacingWheelStatics>>::get_activation_factory().add_racing_wheel_removed(value)
    }}
    #[inline] pub fn remove_racing_wheel_removed(token: super::super::foundation::EventRegistrationToken) -> Result<()> { unsafe {
        <Self as RtActivatable<IRacingWheelStatics>>::get_activation_factory().remove_racing_wheel_removed(token)
    }}
    #[inline] pub fn get_racing_wheels() -> Result<ComPtr<super::super::foundation::collections::IVectorView<RacingWheel>>> { unsafe {
        <Self as RtActivatable<IRacingWheelStatics>>::get_activation_factory().get_racing_wheels()
    }}
    #[inline] pub fn from_game_controller(gameController: &IGameController) -> Result<ComPtr<RacingWheel>> { unsafe {
        <Self as RtActivatable<IRacingWheelStatics2>>::get_activation_factory().from_game_controller(gameController)
    }}
}
DEFINE_CLSID!(RacingWheel(&[87,105,110,100,111,119,115,46,71,97,109,105,110,103,46,73,110,112,117,116,46,82,97,99,105,110,103,87,104,101,101,108,0]) [CLSID_RacingWheel]);
RT_ENUM! { enum RacingWheelButtons: u32 {
    None (RacingWheelButtons_None) = 0, PreviousGear (RacingWheelButtons_PreviousGear) = 1, NextGear (RacingWheelButtons_NextGear) = 2, DPadUp (RacingWheelButtons_DPadUp) = 4, DPadDown (RacingWheelButtons_DPadDown) = 8, DPadLeft (RacingWheelButtons_DPadLeft) = 16, DPadRight (RacingWheelButtons_DPadRight) = 32, Button1 (RacingWheelButtons_Button1) = 64, Button2 (RacingWheelButtons_Button2) = 128, Button3 (RacingWheelButtons_Button3) = 256, Button4 (RacingWheelButtons_Button4) = 512, Button5 (RacingWheelButtons_Button5) = 1024, Button6 (RacingWheelButtons_Button6) = 2048, Button7 (RacingWheelButtons_Button7) = 4096, Button8 (RacingWheelButtons_Button8) = 8192, Button9 (RacingWheelButtons_Button9) = 16384, Button10 (RacingWheelButtons_Button10) = 32768, Button11 (RacingWheelButtons_Button11) = 65536, Button12 (RacingWheelButtons_Button12) = 131072, Button13 (RacingWheelButtons_Button13) = 262144, Button14 (RacingWheelButtons_Button14) = 524288, Button15 (RacingWheelButtons_Button15) = 1048576, Button16 (RacingWheelButtons_Button16) = 2097152,
}}
RT_STRUCT! { struct RacingWheelReading {
    Timestamp: u64, Buttons: RacingWheelButtons, PatternShifterGear: i32, Wheel: f64, Throttle: f64, Brake: f64, Clutch: f64, Handbrake: f64,
}}
DEFINE_IID!(IID_IRacingWheelStatics, 985738453, 22555, 18742, 159, 148, 105, 241, 230, 81, 76, 125);
RT_INTERFACE!{static interface IRacingWheelStatics(IRacingWheelStaticsVtbl): IInspectable(IInspectableVtbl) [IID_IRacingWheelStatics] {
    fn add_RacingWheelAdded(&self, value: *mut super::super::foundation::EventHandler<RacingWheel>, out: *mut super::super::foundation::EventRegistrationToken) -> HRESULT,
    fn remove_RacingWheelAdded(&self, token: super::super::foundation::EventRegistrationToken) -> HRESULT,
    fn add_RacingWheelRemoved(&self, value: *mut super::super::foundation::EventHandler<RacingWheel>, out: *mut super::super::foundation::EventRegistrationToken) -> HRESULT,
    fn remove_RacingWheelRemoved(&self, token: super::super::foundation::EventRegistrationToken) -> HRESULT,
    fn get_RacingWheels(&self, out: *mut *mut super::super::foundation::collections::IVectorView<RacingWheel>) -> HRESULT
}}
impl IRacingWheelStatics {
    #[inline] pub unsafe fn add_racing_wheel_added(&self, value: &super::super::foundation::EventHandler<RacingWheel>) -> Result<super::super::foundation::EventRegistrationToken> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).add_RacingWheelAdded)(self as *const _ as *mut _, value as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn remove_racing_wheel_added(&self, token: super::super::foundation::EventRegistrationToken) -> Result<()> {
        let hr = ((*self.lpVtbl).remove_RacingWheelAdded)(self as *const _ as *mut _, token);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn add_racing_wheel_removed(&self, value: &super::super::foundation::EventHandler<RacingWheel>) -> Result<super::super::foundation::EventRegistrationToken> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).add_RacingWheelRemoved)(self as *const _ as *mut _, value as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn remove_racing_wheel_removed(&self, token: super::super::foundation::EventRegistrationToken) -> Result<()> {
        let hr = ((*self.lpVtbl).remove_RacingWheelRemoved)(self as *const _ as *mut _, token);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_racing_wheels(&self) -> Result<ComPtr<super::super::foundation::collections::IVectorView<RacingWheel>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_RacingWheels)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
DEFINE_IID!(IID_IRacingWheelStatics2, 3865492650, 60925, 17187, 169, 246, 60, 56, 64, 72, 209, 237);
RT_INTERFACE!{static interface IRacingWheelStatics2(IRacingWheelStatics2Vtbl): IInspectable(IInspectableVtbl) [IID_IRacingWheelStatics2] {
    fn FromGameController(&self, gameController: *mut IGameController, out: *mut *mut RacingWheel) -> HRESULT
}}
impl IRacingWheelStatics2 {
    #[inline] pub unsafe fn from_game_controller(&self, gameController: &IGameController) -> Result<ComPtr<RacingWheel>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).FromGameController)(self as *const _ as *mut _, gameController as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
DEFINE_IID!(IID_IRawGameController, 2091740561, 42977, 20337, 154, 120, 51, 233, 197, 223, 234, 98);
RT_INTERFACE!{interface IRawGameController(IRawGameControllerVtbl): IInspectable(IInspectableVtbl) [IID_IRawGameController] {
    fn get_AxisCount(&self, out: *mut i32) -> HRESULT,
    fn get_ButtonCount(&self, out: *mut i32) -> HRESULT,
    fn get_ForceFeedbackMotors(&self, out: *mut *mut super::super::foundation::collections::IVectorView<forcefeedback::ForceFeedbackMotor>) -> HRESULT,
    fn get_HardwareProductId(&self, out: *mut u16) -> HRESULT,
    fn get_HardwareVendorId(&self, out: *mut u16) -> HRESULT,
    fn get_SwitchCount(&self, out: *mut i32) -> HRESULT,
    fn GetButtonLabel(&self, buttonIndex: i32, out: *mut GameControllerButtonLabel) -> HRESULT,
    fn GetCurrentReading(&self, buttonArraySize: u32, buttonArray: *mut bool, switchArraySize: u32, switchArray: *mut GameControllerSwitchPosition, axisArraySize: u32, axisArray: *mut f64, out: *mut u64) -> HRESULT,
    fn GetSwitchKind(&self, switchIndex: i32, out: *mut GameControllerSwitchKind) -> HRESULT
}}
impl IRawGameController {
    #[inline] pub unsafe fn get_axis_count(&self) -> Result<i32> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_AxisCount)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_button_count(&self) -> Result<i32> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_ButtonCount)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_force_feedback_motors(&self) -> Result<ComPtr<super::super::foundation::collections::IVectorView<forcefeedback::ForceFeedbackMotor>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_ForceFeedbackMotors)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_hardware_product_id(&self) -> Result<u16> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_HardwareProductId)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_hardware_vendor_id(&self) -> Result<u16> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_HardwareVendorId)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_switch_count(&self) -> Result<i32> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_SwitchCount)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_button_label(&self, buttonIndex: i32) -> Result<GameControllerButtonLabel> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).GetButtonLabel)(self as *const _ as *mut _, buttonIndex, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_current_reading(&self, buttonArray: &mut [bool], switchArray: &mut [GameControllerSwitchPosition], axisArray: &mut [f64]) -> Result<u64> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).GetCurrentReading)(self as *const _ as *mut _, buttonArray.len() as u32, buttonArray.as_mut_ptr() as *mut _, switchArray.len() as u32, switchArray.as_mut_ptr() as *mut _, axisArray.len() as u32, axisArray.as_mut_ptr() as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_switch_kind(&self, switchIndex: i32) -> Result<GameControllerSwitchKind> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).GetSwitchKind)(self as *const _ as *mut _, switchIndex, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
}
RT_CLASS!{class RawGameController: IRawGameController}
impl RtActivatable<IRawGameControllerStatics> for RawGameController {}
impl RawGameController {
    #[inline] pub fn add_raw_game_controller_added(value: &super::super::foundation::EventHandler<RawGameController>) -> Result<super::super::foundation::EventRegistrationToken> { unsafe {
        <Self as RtActivatable<IRawGameControllerStatics>>::get_activation_factory().add_raw_game_controller_added(value)
    }}
    #[inline] pub fn remove_raw_game_controller_added(token: super::super::foundation::EventRegistrationToken) -> Result<()> { unsafe {
        <Self as RtActivatable<IRawGameControllerStatics>>::get_activation_factory().remove_raw_game_controller_added(token)
    }}
    #[inline] pub fn add_raw_game_controller_removed(value: &super::super::foundation::EventHandler<RawGameController>) -> Result<super::super::foundation::EventRegistrationToken> { unsafe {
        <Self as RtActivatable<IRawGameControllerStatics>>::get_activation_factory().add_raw_game_controller_removed(value)
    }}
    #[inline] pub fn remove_raw_game_controller_removed(token: super::super::foundation::EventRegistrationToken) -> Result<()> { unsafe {
        <Self as RtActivatable<IRawGameControllerStatics>>::get_activation_factory().remove_raw_game_controller_removed(token)
    }}
    #[inline] pub fn get_raw_game_controllers() -> Result<ComPtr<super::super::foundation::collections::IVectorView<RawGameController>>> { unsafe {
        <Self as RtActivatable<IRawGameControllerStatics>>::get_activation_factory().get_raw_game_controllers()
    }}
    #[inline] pub fn from_game_controller(gameController: &IGameController) -> Result<ComPtr<RawGameController>> { unsafe {
        <Self as RtActivatable<IRawGameControllerStatics>>::get_activation_factory().from_game_controller(gameController)
    }}
}
DEFINE_CLSID!(RawGameController(&[87,105,110,100,111,119,115,46,71,97,109,105,110,103,46,73,110,112,117,116,46,82,97,119,71,97,109,101,67,111,110,116,114,111,108,108,101,114,0]) [CLSID_RawGameController]);
DEFINE_IID!(IID_IRawGameController2, 1136705589, 47987, 18262, 167, 135, 62, 214, 190, 166, 23, 189);
RT_INTERFACE!{interface IRawGameController2(IRawGameController2Vtbl): IInspectable(IInspectableVtbl) [IID_IRawGameController2] {
    #[cfg(not(feature="windows-devices"))] fn __Dummy0(&self) -> (),
    #[cfg(feature="windows-devices")] fn get_SimpleHapticsControllers(&self, out: *mut *mut super::super::foundation::collections::IVectorView<super::super::devices::haptics::SimpleHapticsController>) -> HRESULT,
    fn get_NonRoamableId(&self, out: *mut HSTRING) -> HRESULT,
    fn get_DisplayName(&self, out: *mut HSTRING) -> HRESULT
}}
impl IRawGameController2 {
    #[cfg(feature="windows-devices")] #[inline] pub unsafe fn get_simple_haptics_controllers(&self) -> Result<ComPtr<super::super::foundation::collections::IVectorView<super::super::devices::haptics::SimpleHapticsController>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_SimpleHapticsControllers)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_non_roamable_id(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_NonRoamableId)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_display_name(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_DisplayName)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
}
DEFINE_IID!(IID_IRawGameControllerStatics, 3951888274, 59738, 19225, 175, 199, 10, 89, 248, 191, 117, 158);
RT_INTERFACE!{static interface IRawGameControllerStatics(IRawGameControllerStaticsVtbl): IInspectable(IInspectableVtbl) [IID_IRawGameControllerStatics] {
    fn add_RawGameControllerAdded(&self, value: *mut super::super::foundation::EventHandler<RawGameController>, out: *mut super::super::foundation::EventRegistrationToken) -> HRESULT,
    fn remove_RawGameControllerAdded(&self, token: super::super::foundation::EventRegistrationToken) -> HRESULT,
    fn add_RawGameControllerRemoved(&self, value: *mut super::super::foundation::EventHandler<RawGameController>, out: *mut super::super::foundation::EventRegistrationToken) -> HRESULT,
    fn remove_RawGameControllerRemoved(&self, token: super::super::foundation::EventRegistrationToken) -> HRESULT,
    fn get_RawGameControllers(&self, out: *mut *mut super::super::foundation::collections::IVectorView<RawGameController>) -> HRESULT,
    fn FromGameController(&self, gameController: *mut IGameController, out: *mut *mut RawGameController) -> HRESULT
}}
impl IRawGameControllerStatics {
    #[inline] pub unsafe fn add_raw_game_controller_added(&self, value: &super::super::foundation::EventHandler<RawGameController>) -> Result<super::super::foundation::EventRegistrationToken> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).add_RawGameControllerAdded)(self as *const _ as *mut _, value as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn remove_raw_game_controller_added(&self, token: super::super::foundation::EventRegistrationToken) -> Result<()> {
        let hr = ((*self.lpVtbl).remove_RawGameControllerAdded)(self as *const _ as *mut _, token);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn add_raw_game_controller_removed(&self, value: &super::super::foundation::EventHandler<RawGameController>) -> Result<super::super::foundation::EventRegistrationToken> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).add_RawGameControllerRemoved)(self as *const _ as *mut _, value as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn remove_raw_game_controller_removed(&self, token: super::super::foundation::EventRegistrationToken) -> Result<()> {
        let hr = ((*self.lpVtbl).remove_RawGameControllerRemoved)(self as *const _ as *mut _, token);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_raw_game_controllers(&self) -> Result<ComPtr<super::super::foundation::collections::IVectorView<RawGameController>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_RawGameControllers)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn from_game_controller(&self, gameController: &IGameController) -> Result<ComPtr<RawGameController>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).FromGameController)(self as *const _ as *mut _, gameController as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
RT_ENUM! { enum RequiredUINavigationButtons: u32 {
    None (RequiredUINavigationButtons_None) = 0, Menu (RequiredUINavigationButtons_Menu) = 1, View (RequiredUINavigationButtons_View) = 2, Accept (RequiredUINavigationButtons_Accept) = 4, Cancel (RequiredUINavigationButtons_Cancel) = 8, Up (RequiredUINavigationButtons_Up) = 16, Down (RequiredUINavigationButtons_Down) = 32, Left (RequiredUINavigationButtons_Left) = 64, Right (RequiredUINavigationButtons_Right) = 128,
}}
DEFINE_IID!(IID_IUINavigationController, 3853447133, 62734, 19029, 140, 220, 211, 50, 41, 84, 129, 117);
RT_INTERFACE!{interface IUINavigationController(IUINavigationControllerVtbl): IInspectable(IInspectableVtbl) [IID_IUINavigationController] {
    fn GetCurrentReading(&self, out: *mut UINavigationReading) -> HRESULT,
    fn GetOptionalButtonLabel(&self, button: OptionalUINavigationButtons, out: *mut GameControllerButtonLabel) -> HRESULT,
    fn GetRequiredButtonLabel(&self, button: RequiredUINavigationButtons, out: *mut GameControllerButtonLabel) -> HRESULT
}}
impl IUINavigationController {
    #[inline] pub unsafe fn get_current_reading(&self) -> Result<UINavigationReading> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).GetCurrentReading)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_optional_button_label(&self, button: OptionalUINavigationButtons) -> Result<GameControllerButtonLabel> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).GetOptionalButtonLabel)(self as *const _ as *mut _, button, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_required_button_label(&self, button: RequiredUINavigationButtons) -> Result<GameControllerButtonLabel> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).GetRequiredButtonLabel)(self as *const _ as *mut _, button, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
}
RT_CLASS!{class UINavigationController: IUINavigationController}
impl RtActivatable<IUINavigationControllerStatics> for UINavigationController {}
impl RtActivatable<IUINavigationControllerStatics2> for UINavigationController {}
impl UINavigationController {
    #[inline] pub fn add_uinavigation_controller_added(value: &super::super::foundation::EventHandler<UINavigationController>) -> Result<super::super::foundation::EventRegistrationToken> { unsafe {
        <Self as RtActivatable<IUINavigationControllerStatics>>::get_activation_factory().add_uinavigation_controller_added(value)
    }}
    #[inline] pub fn remove_uinavigation_controller_added(token: super::super::foundation::EventRegistrationToken) -> Result<()> { unsafe {
        <Self as RtActivatable<IUINavigationControllerStatics>>::get_activation_factory().remove_uinavigation_controller_added(token)
    }}
    #[inline] pub fn add_uinavigation_controller_removed(value: &super::super::foundation::EventHandler<UINavigationController>) -> Result<super::super::foundation::EventRegistrationToken> { unsafe {
        <Self as RtActivatable<IUINavigationControllerStatics>>::get_activation_factory().add_uinavigation_controller_removed(value)
    }}
    #[inline] pub fn remove_uinavigation_controller_removed(token: super::super::foundation::EventRegistrationToken) -> Result<()> { unsafe {
        <Self as RtActivatable<IUINavigationControllerStatics>>::get_activation_factory().remove_uinavigation_controller_removed(token)
    }}
    #[inline] pub fn get_uinavigation_controllers() -> Result<ComPtr<super::super::foundation::collections::IVectorView<UINavigationController>>> { unsafe {
        <Self as RtActivatable<IUINavigationControllerStatics>>::get_activation_factory().get_uinavigation_controllers()
    }}
    #[inline] pub fn from_game_controller(gameController: &IGameController) -> Result<ComPtr<UINavigationController>> { unsafe {
        <Self as RtActivatable<IUINavigationControllerStatics2>>::get_activation_factory().from_game_controller(gameController)
    }}
}
DEFINE_CLSID!(UINavigationController(&[87,105,110,100,111,119,115,46,71,97,109,105,110,103,46,73,110,112,117,116,46,85,73,78,97,118,105,103,97,116,105,111,110,67,111,110,116,114,111,108,108,101,114,0]) [CLSID_UINavigationController]);
DEFINE_IID!(IID_IUINavigationControllerStatics, 789877514, 63224, 19016, 141, 137, 148, 120, 108, 202, 12, 46);
RT_INTERFACE!{static interface IUINavigationControllerStatics(IUINavigationControllerStaticsVtbl): IInspectable(IInspectableVtbl) [IID_IUINavigationControllerStatics] {
    fn add_UINavigationControllerAdded(&self, value: *mut super::super::foundation::EventHandler<UINavigationController>, out: *mut super::super::foundation::EventRegistrationToken) -> HRESULT,
    fn remove_UINavigationControllerAdded(&self, token: super::super::foundation::EventRegistrationToken) -> HRESULT,
    fn add_UINavigationControllerRemoved(&self, value: *mut super::super::foundation::EventHandler<UINavigationController>, out: *mut super::super::foundation::EventRegistrationToken) -> HRESULT,
    fn remove_UINavigationControllerRemoved(&self, token: super::super::foundation::EventRegistrationToken) -> HRESULT,
    fn get_UINavigationControllers(&self, out: *mut *mut super::super::foundation::collections::IVectorView<UINavigationController>) -> HRESULT
}}
impl IUINavigationControllerStatics {
    #[inline] pub unsafe fn add_uinavigation_controller_added(&self, value: &super::super::foundation::EventHandler<UINavigationController>) -> Result<super::super::foundation::EventRegistrationToken> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).add_UINavigationControllerAdded)(self as *const _ as *mut _, value as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn remove_uinavigation_controller_added(&self, token: super::super::foundation::EventRegistrationToken) -> Result<()> {
        let hr = ((*self.lpVtbl).remove_UINavigationControllerAdded)(self as *const _ as *mut _, token);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn add_uinavigation_controller_removed(&self, value: &super::super::foundation::EventHandler<UINavigationController>) -> Result<super::super::foundation::EventRegistrationToken> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).add_UINavigationControllerRemoved)(self as *const _ as *mut _, value as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn remove_uinavigation_controller_removed(&self, token: super::super::foundation::EventRegistrationToken) -> Result<()> {
        let hr = ((*self.lpVtbl).remove_UINavigationControllerRemoved)(self as *const _ as *mut _, token);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_uinavigation_controllers(&self) -> Result<ComPtr<super::super::foundation::collections::IVectorView<UINavigationController>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_UINavigationControllers)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
DEFINE_IID!(IID_IUINavigationControllerStatics2, 3771410659, 45579, 19211, 158, 212, 243, 213, 60, 236, 13, 228);
RT_INTERFACE!{static interface IUINavigationControllerStatics2(IUINavigationControllerStatics2Vtbl): IInspectable(IInspectableVtbl) [IID_IUINavigationControllerStatics2] {
    fn FromGameController(&self, gameController: *mut IGameController, out: *mut *mut UINavigationController) -> HRESULT
}}
impl IUINavigationControllerStatics2 {
    #[inline] pub unsafe fn from_game_controller(&self, gameController: &IGameController) -> Result<ComPtr<UINavigationController>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).FromGameController)(self as *const _ as *mut _, gameController as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
RT_STRUCT! { struct UINavigationReading {
    Timestamp: u64, RequiredButtons: RequiredUINavigationButtons, OptionalButtons: OptionalUINavigationButtons,
}}
pub mod custom { // Windows.Gaming.Input.Custom
use ::prelude::*;
DEFINE_IID!(IID_ICustomGameControllerFactory, 1772138078, 30094, 19646, 172, 230, 98, 21, 95, 233, 18, 111);
RT_INTERFACE!{interface ICustomGameControllerFactory(ICustomGameControllerFactoryVtbl): IInspectable(IInspectableVtbl) [IID_ICustomGameControllerFactory] {
    fn CreateGameController(&self, provider: *mut IGameControllerProvider, out: *mut *mut IInspectable) -> HRESULT,
    fn OnGameControllerAdded(&self, value: *mut super::IGameController) -> HRESULT,
    fn OnGameControllerRemoved(&self, value: *mut super::IGameController) -> HRESULT
}}
impl ICustomGameControllerFactory {
    #[inline] pub unsafe fn create_game_controller(&self, provider: &IGameControllerProvider) -> Result<ComPtr<IInspectable>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).CreateGameController)(self as *const _ as *mut _, provider as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn on_game_controller_added(&self, value: &super::IGameController) -> Result<()> {
        let hr = ((*self.lpVtbl).OnGameControllerAdded)(self as *const _ as *mut _, value as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn on_game_controller_removed(&self, value: &super::IGameController) -> Result<()> {
        let hr = ((*self.lpVtbl).OnGameControllerRemoved)(self as *const _ as *mut _, value as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
}
RT_CLASS!{static class GameControllerFactoryManager}
impl RtActivatable<IGameControllerFactoryManagerStatics> for GameControllerFactoryManager {}
impl RtActivatable<IGameControllerFactoryManagerStatics2> for GameControllerFactoryManager {}
impl GameControllerFactoryManager {
    #[inline] pub fn register_custom_factory_for_gip_interface(factory: &ICustomGameControllerFactory, interfaceId: Guid) -> Result<()> { unsafe {
        <Self as RtActivatable<IGameControllerFactoryManagerStatics>>::get_activation_factory().register_custom_factory_for_gip_interface(factory, interfaceId)
    }}
    #[inline] pub fn register_custom_factory_for_hardware_id(factory: &ICustomGameControllerFactory, hardwareVendorId: u16, hardwareProductId: u16) -> Result<()> { unsafe {
        <Self as RtActivatable<IGameControllerFactoryManagerStatics>>::get_activation_factory().register_custom_factory_for_hardware_id(factory, hardwareVendorId, hardwareProductId)
    }}
    #[inline] pub fn register_custom_factory_for_xusb_type(factory: &ICustomGameControllerFactory, xusbType: XusbDeviceType, xusbSubtype: XusbDeviceSubtype) -> Result<()> { unsafe {
        <Self as RtActivatable<IGameControllerFactoryManagerStatics>>::get_activation_factory().register_custom_factory_for_xusb_type(factory, xusbType, xusbSubtype)
    }}
    #[inline] pub fn try_get_factory_controller_from_game_controller(factory: &ICustomGameControllerFactory, gameController: &super::IGameController) -> Result<ComPtr<super::IGameController>> { unsafe {
        <Self as RtActivatable<IGameControllerFactoryManagerStatics2>>::get_activation_factory().try_get_factory_controller_from_game_controller(factory, gameController)
    }}
}
DEFINE_CLSID!(GameControllerFactoryManager(&[87,105,110,100,111,119,115,46,71,97,109,105,110,103,46,73,110,112,117,116,46,67,117,115,116,111,109,46,71,97,109,101,67,111,110,116,114,111,108,108,101,114,70,97,99,116,111,114,121,77,97,110,97,103,101,114,0]) [CLSID_GameControllerFactoryManager]);
DEFINE_IID!(IID_IGameControllerFactoryManagerStatics, 919299811, 53409, 18822, 162, 76, 64, 177, 55, 222, 186, 158);
RT_INTERFACE!{static interface IGameControllerFactoryManagerStatics(IGameControllerFactoryManagerStaticsVtbl): IInspectable(IInspectableVtbl) [IID_IGameControllerFactoryManagerStatics] {
    fn RegisterCustomFactoryForGipInterface(&self, factory: *mut ICustomGameControllerFactory, interfaceId: Guid) -> HRESULT,
    fn RegisterCustomFactoryForHardwareId(&self, factory: *mut ICustomGameControllerFactory, hardwareVendorId: u16, hardwareProductId: u16) -> HRESULT,
    fn RegisterCustomFactoryForXusbType(&self, factory: *mut ICustomGameControllerFactory, xusbType: XusbDeviceType, xusbSubtype: XusbDeviceSubtype) -> HRESULT
}}
impl IGameControllerFactoryManagerStatics {
    #[inline] pub unsafe fn register_custom_factory_for_gip_interface(&self, factory: &ICustomGameControllerFactory, interfaceId: Guid) -> Result<()> {
        let hr = ((*self.lpVtbl).RegisterCustomFactoryForGipInterface)(self as *const _ as *mut _, factory as *const _ as *mut _, interfaceId);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn register_custom_factory_for_hardware_id(&self, factory: &ICustomGameControllerFactory, hardwareVendorId: u16, hardwareProductId: u16) -> Result<()> {
        let hr = ((*self.lpVtbl).RegisterCustomFactoryForHardwareId)(self as *const _ as *mut _, factory as *const _ as *mut _, hardwareVendorId, hardwareProductId);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn register_custom_factory_for_xusb_type(&self, factory: &ICustomGameControllerFactory, xusbType: XusbDeviceType, xusbSubtype: XusbDeviceSubtype) -> Result<()> {
        let hr = ((*self.lpVtbl).RegisterCustomFactoryForXusbType)(self as *const _ as *mut _, factory as *const _ as *mut _, xusbType, xusbSubtype);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
}
DEFINE_IID!(IID_IGameControllerFactoryManagerStatics2, 3939391044, 6623, 16661, 179, 42, 39, 147, 226, 174, 163, 187);
RT_INTERFACE!{static interface IGameControllerFactoryManagerStatics2(IGameControllerFactoryManagerStatics2Vtbl): IInspectable(IInspectableVtbl) [IID_IGameControllerFactoryManagerStatics2] {
    fn TryGetFactoryControllerFromGameController(&self, factory: *mut ICustomGameControllerFactory, gameController: *mut super::IGameController, out: *mut *mut super::IGameController) -> HRESULT
}}
impl IGameControllerFactoryManagerStatics2 {
    #[inline] pub unsafe fn try_get_factory_controller_from_game_controller(&self, factory: &ICustomGameControllerFactory, gameController: &super::IGameController) -> Result<ComPtr<super::IGameController>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).TryGetFactoryControllerFromGameController)(self as *const _ as *mut _, factory as *const _ as *mut _, gameController as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
DEFINE_IID!(IID_IGameControllerInputSink, 536279330, 50752, 19576, 168, 32, 154, 113, 92, 85, 139, 203);
RT_INTERFACE!{interface IGameControllerInputSink(IGameControllerInputSinkVtbl): IInspectable(IInspectableVtbl) [IID_IGameControllerInputSink] {
    fn OnInputResumed(&self, timestamp: u64) -> HRESULT,
    fn OnInputSuspended(&self, timestamp: u64) -> HRESULT
}}
impl IGameControllerInputSink {
    #[inline] pub unsafe fn on_input_resumed(&self, timestamp: u64) -> Result<()> {
        let hr = ((*self.lpVtbl).OnInputResumed)(self as *const _ as *mut _, timestamp);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn on_input_suspended(&self, timestamp: u64) -> Result<()> {
        let hr = ((*self.lpVtbl).OnInputSuspended)(self as *const _ as *mut _, timestamp);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
}
DEFINE_IID!(IID_IGameControllerProvider, 3872864642, 10646, 17753, 177, 108, 62, 87, 212, 110, 88, 214);
RT_INTERFACE!{interface IGameControllerProvider(IGameControllerProviderVtbl): IInspectable(IInspectableVtbl) [IID_IGameControllerProvider] {
    fn get_FirmwareVersionInfo(&self, out: *mut GameControllerVersionInfo) -> HRESULT,
    fn get_HardwareProductId(&self, out: *mut u16) -> HRESULT,
    fn get_HardwareVendorId(&self, out: *mut u16) -> HRESULT,
    fn get_HardwareVersionInfo(&self, out: *mut GameControllerVersionInfo) -> HRESULT,
    fn get_IsConnected(&self, out: *mut bool) -> HRESULT
}}
impl IGameControllerProvider {
    #[inline] pub unsafe fn get_firmware_version_info(&self) -> Result<GameControllerVersionInfo> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_FirmwareVersionInfo)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_hardware_product_id(&self) -> Result<u16> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_HardwareProductId)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_hardware_vendor_id(&self) -> Result<u16> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_HardwareVendorId)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_hardware_version_info(&self) -> Result<GameControllerVersionInfo> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_HardwareVersionInfo)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_is_connected(&self) -> Result<bool> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_IsConnected)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
}
RT_STRUCT! { struct GameControllerVersionInfo {
    Major: u16, Minor: u16, Build: u16, Revision: u16,
}}
RT_STRUCT! { struct GipFirmwareUpdateProgress {
    PercentCompleted: f64, CurrentComponentId: u32,
}}
DEFINE_IID!(IID_IGipFirmwareUpdateResult, 1803111730, 34131, 17042, 142, 3, 225, 102, 81, 162, 248, 188);
RT_INTERFACE!{interface IGipFirmwareUpdateResult(IGipFirmwareUpdateResultVtbl): IInspectable(IInspectableVtbl) [IID_IGipFirmwareUpdateResult] {
    fn get_ExtendedErrorCode(&self, out: *mut u32) -> HRESULT,
    fn get_FinalComponentId(&self, out: *mut u32) -> HRESULT,
    fn get_Status(&self, out: *mut GipFirmwareUpdateStatus) -> HRESULT
}}
impl IGipFirmwareUpdateResult {
    #[inline] pub unsafe fn get_extended_error_code(&self) -> Result<u32> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_ExtendedErrorCode)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_final_component_id(&self) -> Result<u32> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_FinalComponentId)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_status(&self) -> Result<GipFirmwareUpdateStatus> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_Status)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
}
RT_CLASS!{class GipFirmwareUpdateResult: IGipFirmwareUpdateResult}
RT_ENUM! { enum GipFirmwareUpdateStatus: i32 {
    Completed (GipFirmwareUpdateStatus_Completed) = 0, UpToDate (GipFirmwareUpdateStatus_UpToDate) = 1, Failed (GipFirmwareUpdateStatus_Failed) = 2,
}}
DEFINE_IID!(IID_IGipGameControllerInputSink, 2718993087, 2545, 17340, 161, 64, 128, 248, 153, 236, 54, 251);
RT_INTERFACE!{interface IGipGameControllerInputSink(IGipGameControllerInputSinkVtbl): IInspectable(IInspectableVtbl) [IID_IGipGameControllerInputSink] {
    fn OnKeyReceived(&self, timestamp: u64, keyCode: u8, isPressed: bool) -> HRESULT,
    fn OnMessageReceived(&self, timestamp: u64, messageClass: GipMessageClass, messageId: u8, sequenceId: u8, messageBufferSize: u32, messageBuffer: *mut u8) -> HRESULT
}}
impl IGipGameControllerInputSink {
    #[inline] pub unsafe fn on_key_received(&self, timestamp: u64, keyCode: u8, isPressed: bool) -> Result<()> {
        let hr = ((*self.lpVtbl).OnKeyReceived)(self as *const _ as *mut _, timestamp, keyCode, isPressed);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn on_message_received(&self, timestamp: u64, messageClass: GipMessageClass, messageId: u8, sequenceId: u8, messageBuffer: &[u8]) -> Result<()> {
        let hr = ((*self.lpVtbl).OnMessageReceived)(self as *const _ as *mut _, timestamp, messageClass, messageId, sequenceId, messageBuffer.len() as u32, messageBuffer.as_ptr() as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
}
DEFINE_IID!(IID_IGipGameControllerProvider, 3687783961, 6901, 17832, 191, 2, 160, 238, 80, 200, 35, 252);
RT_INTERFACE!{interface IGipGameControllerProvider(IGipGameControllerProviderVtbl): IInspectable(IInspectableVtbl) [IID_IGipGameControllerProvider] {
    fn SendMessage(&self, messageClass: GipMessageClass, messageId: u8, messageBufferSize: u32, messageBuffer: *mut u8) -> HRESULT,
    fn SendReceiveMessage(&self, messageClass: GipMessageClass, messageId: u8, requestMessageBufferSize: u32, requestMessageBuffer: *mut u8, responseMessageBufferSize: u32, responseMessageBuffer: *mut u8) -> HRESULT,
    #[cfg(feature="windows-storage")] fn UpdateFirmwareAsync(&self, firmwareImage: *mut ::rt::gen::windows::storage::streams::IInputStream, out: *mut *mut ::rt::gen::windows::foundation::IAsyncOperationWithProgress<GipFirmwareUpdateResult, GipFirmwareUpdateProgress>) -> HRESULT
}}
impl IGipGameControllerProvider {
    #[inline] pub unsafe fn send_message(&self, messageClass: GipMessageClass, messageId: u8, messageBuffer: &[u8]) -> Result<()> {
        let hr = ((*self.lpVtbl).SendMessage)(self as *const _ as *mut _, messageClass, messageId, messageBuffer.len() as u32, messageBuffer.as_ptr() as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn send_receive_message(&self, messageClass: GipMessageClass, messageId: u8, requestMessageBuffer: &[u8], responseMessageBuffer: &mut [u8]) -> Result<()> {
        let hr = ((*self.lpVtbl).SendReceiveMessage)(self as *const _ as *mut _, messageClass, messageId, requestMessageBuffer.len() as u32, requestMessageBuffer.as_ptr() as *mut _, responseMessageBuffer.len() as u32, responseMessageBuffer.as_mut_ptr() as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[cfg(feature="windows-storage")] #[inline] pub unsafe fn update_firmware_async(&self, firmwareImage: &::rt::gen::windows::storage::streams::IInputStream) -> Result<ComPtr<::rt::gen::windows::foundation::IAsyncOperationWithProgress<GipFirmwareUpdateResult, GipFirmwareUpdateProgress>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).UpdateFirmwareAsync)(self as *const _ as *mut _, firmwareImage as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
RT_CLASS!{class GipGameControllerProvider: IGipGameControllerProvider}
RT_ENUM! { enum GipMessageClass: i32 {
    Command (GipMessageClass_Command) = 0, LowLatency (GipMessageClass_LowLatency) = 1, StandardLatency (GipMessageClass_StandardLatency) = 2,
}}
DEFINE_IID!(IID_IHidGameControllerInputSink, 4149527330, 6189, 16612, 161, 38, 252, 238, 79, 250, 30, 49);
RT_INTERFACE!{interface IHidGameControllerInputSink(IHidGameControllerInputSinkVtbl): IInspectable(IInspectableVtbl) [IID_IHidGameControllerInputSink] {
    fn OnInputReportReceived(&self, timestamp: u64, reportId: u8, reportBufferSize: u32, reportBuffer: *mut u8) -> HRESULT
}}
impl IHidGameControllerInputSink {
    #[inline] pub unsafe fn on_input_report_received(&self, timestamp: u64, reportId: u8, reportBuffer: &[u8]) -> Result<()> {
        let hr = ((*self.lpVtbl).OnInputReportReceived)(self as *const _ as *mut _, timestamp, reportId, reportBuffer.len() as u32, reportBuffer.as_ptr() as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
}
DEFINE_IID!(IID_IHidGameControllerProvider, 2513320692, 44016, 19304, 160, 129, 59, 125, 231, 63, 240, 231);
RT_INTERFACE!{interface IHidGameControllerProvider(IHidGameControllerProviderVtbl): IInspectable(IInspectableVtbl) [IID_IHidGameControllerProvider] {
    fn get_UsageId(&self, out: *mut u16) -> HRESULT,
    fn get_UsagePage(&self, out: *mut u16) -> HRESULT,
    fn GetFeatureReport(&self, reportId: u8, reportBufferSize: u32, reportBuffer: *mut u8) -> HRESULT,
    fn SendFeatureReport(&self, reportId: u8, reportBufferSize: u32, reportBuffer: *mut u8) -> HRESULT,
    fn SendOutputReport(&self, reportId: u8, reportBufferSize: u32, reportBuffer: *mut u8) -> HRESULT
}}
impl IHidGameControllerProvider {
    #[inline] pub unsafe fn get_usage_id(&self) -> Result<u16> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_UsageId)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_usage_page(&self) -> Result<u16> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_UsagePage)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_feature_report(&self, reportId: u8, reportBuffer: &mut [u8]) -> Result<()> {
        let hr = ((*self.lpVtbl).GetFeatureReport)(self as *const _ as *mut _, reportId, reportBuffer.len() as u32, reportBuffer.as_mut_ptr() as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn send_feature_report(&self, reportId: u8, reportBuffer: &[u8]) -> Result<()> {
        let hr = ((*self.lpVtbl).SendFeatureReport)(self as *const _ as *mut _, reportId, reportBuffer.len() as u32, reportBuffer.as_ptr() as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn send_output_report(&self, reportId: u8, reportBuffer: &[u8]) -> Result<()> {
        let hr = ((*self.lpVtbl).SendOutputReport)(self as *const _ as *mut _, reportId, reportBuffer.len() as u32, reportBuffer.as_ptr() as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
}
RT_CLASS!{class HidGameControllerProvider: IHidGameControllerProvider}
RT_ENUM! { enum XusbDeviceSubtype: i32 {
    Unknown (XusbDeviceSubtype_Unknown) = 0, Gamepad (XusbDeviceSubtype_Gamepad) = 1, ArcadePad (XusbDeviceSubtype_ArcadePad) = 2, ArcadeStick (XusbDeviceSubtype_ArcadeStick) = 3, FlightStick (XusbDeviceSubtype_FlightStick) = 4, Wheel (XusbDeviceSubtype_Wheel) = 5, Guitar (XusbDeviceSubtype_Guitar) = 6, GuitarAlternate (XusbDeviceSubtype_GuitarAlternate) = 7, GuitarBass (XusbDeviceSubtype_GuitarBass) = 8, DrumKit (XusbDeviceSubtype_DrumKit) = 9, DancePad (XusbDeviceSubtype_DancePad) = 10,
}}
RT_ENUM! { enum XusbDeviceType: i32 {
    Unknown (XusbDeviceType_Unknown) = 0, Gamepad (XusbDeviceType_Gamepad) = 1,
}}
DEFINE_IID!(IID_IXusbGameControllerInputSink, 2997624213, 28363, 17075, 138, 171, 2, 84, 1, 202, 71, 18);
RT_INTERFACE!{interface IXusbGameControllerInputSink(IXusbGameControllerInputSinkVtbl): IInspectable(IInspectableVtbl) [IID_IXusbGameControllerInputSink] {
    fn OnInputReceived(&self, timestamp: u64, reportId: u8, inputBufferSize: u32, inputBuffer: *mut u8) -> HRESULT
}}
impl IXusbGameControllerInputSink {
    #[inline] pub unsafe fn on_input_received(&self, timestamp: u64, reportId: u8, inputBuffer: &[u8]) -> Result<()> {
        let hr = ((*self.lpVtbl).OnInputReceived)(self as *const _ as *mut _, timestamp, reportId, inputBuffer.len() as u32, inputBuffer.as_ptr() as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
}
DEFINE_IID!(IID_IXusbGameControllerProvider, 1848209899, 3835, 18612, 128, 139, 131, 118, 67, 178, 242, 22);
RT_INTERFACE!{interface IXusbGameControllerProvider(IXusbGameControllerProviderVtbl): IInspectable(IInspectableVtbl) [IID_IXusbGameControllerProvider] {
    fn SetVibration(&self, lowFrequencyMotorSpeed: f64, highFrequencyMotorSpeed: f64) -> HRESULT
}}
impl IXusbGameControllerProvider {
    #[inline] pub unsafe fn set_vibration(&self, lowFrequencyMotorSpeed: f64, highFrequencyMotorSpeed: f64) -> Result<()> {
        let hr = ((*self.lpVtbl).SetVibration)(self as *const _ as *mut _, lowFrequencyMotorSpeed, highFrequencyMotorSpeed);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
}
RT_CLASS!{class XusbGameControllerProvider: IXusbGameControllerProvider}
} // Windows.Gaming.Input.Custom
pub mod forcefeedback { // Windows.Gaming.Input.ForceFeedback
use ::prelude::*;
DEFINE_IID!(IID_IConditionForceEffect, 852617832, 13973, 20073, 133, 192, 205, 25, 68, 24, 145, 64);
RT_INTERFACE!{interface IConditionForceEffect(IConditionForceEffectVtbl): IInspectable(IInspectableVtbl) [IID_IConditionForceEffect] {
    fn get_Kind(&self, out: *mut ConditionForceEffectKind) -> HRESULT,
    fn SetParameters(&self, direction: ::rt::gen::windows::foundation::numerics::Vector3, positiveCoefficient: f32, negativeCoefficient: f32, maxPositiveMagnitude: f32, maxNegativeMagnitude: f32, deadZone: f32, bias: f32) -> HRESULT
}}
impl IConditionForceEffect {
    #[inline] pub unsafe fn get_kind(&self) -> Result<ConditionForceEffectKind> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_Kind)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_parameters(&self, direction: ::rt::gen::windows::foundation::numerics::Vector3, positiveCoefficient: f32, negativeCoefficient: f32, maxPositiveMagnitude: f32, maxNegativeMagnitude: f32, deadZone: f32, bias: f32) -> Result<()> {
        let hr = ((*self.lpVtbl).SetParameters)(self as *const _ as *mut _, direction, positiveCoefficient, negativeCoefficient, maxPositiveMagnitude, maxNegativeMagnitude, deadZone, bias);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
}
RT_CLASS!{class ConditionForceEffect: IForceFeedbackEffect}
impl RtActivatable<IConditionForceEffectFactory> for ConditionForceEffect {}
impl ConditionForceEffect {
    #[inline] pub fn create_instance(effectKind: ConditionForceEffectKind) -> Result<ComPtr<ConditionForceEffect>> { unsafe {
        <Self as RtActivatable<IConditionForceEffectFactory>>::get_activation_factory().create_instance(effectKind)
    }}
}
DEFINE_CLSID!(ConditionForceEffect(&[87,105,110,100,111,119,115,46,71,97,109,105,110,103,46,73,110,112,117,116,46,70,111,114,99,101,70,101,101,100,98,97,99,107,46,67,111,110,100,105,116,105,111,110,70,111,114,99,101,69,102,102,101,99,116,0]) [CLSID_ConditionForceEffect]);
DEFINE_IID!(IID_IConditionForceEffectFactory, 2443809380, 6160, 20150, 167, 115, 191, 211, 184, 205, 219, 171);
RT_INTERFACE!{static interface IConditionForceEffectFactory(IConditionForceEffectFactoryVtbl): IInspectable(IInspectableVtbl) [IID_IConditionForceEffectFactory] {
    fn CreateInstance(&self, effectKind: ConditionForceEffectKind, out: *mut *mut ConditionForceEffect) -> HRESULT
}}
impl IConditionForceEffectFactory {
    #[inline] pub unsafe fn create_instance(&self, effectKind: ConditionForceEffectKind) -> Result<ComPtr<ConditionForceEffect>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).CreateInstance)(self as *const _ as *mut _, effectKind, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
RT_ENUM! { enum ConditionForceEffectKind: i32 {
    Spring (ConditionForceEffectKind_Spring) = 0, Damper (ConditionForceEffectKind_Damper) = 1, Inertia (ConditionForceEffectKind_Inertia) = 2, Friction (ConditionForceEffectKind_Friction) = 3,
}}
DEFINE_IID!(IID_IConstantForceEffect, 2616852800, 62407, 16732, 176, 104, 15, 6, 135, 52, 188, 224);
RT_INTERFACE!{interface IConstantForceEffect(IConstantForceEffectVtbl): IInspectable(IInspectableVtbl) [IID_IConstantForceEffect] {
    fn SetParameters(&self, vector: ::rt::gen::windows::foundation::numerics::Vector3, duration: ::rt::gen::windows::foundation::TimeSpan) -> HRESULT,
    fn SetParametersWithEnvelope(&self, vector: ::rt::gen::windows::foundation::numerics::Vector3, attackGain: f32, sustainGain: f32, releaseGain: f32, startDelay: ::rt::gen::windows::foundation::TimeSpan, attackDuration: ::rt::gen::windows::foundation::TimeSpan, sustainDuration: ::rt::gen::windows::foundation::TimeSpan, releaseDuration: ::rt::gen::windows::foundation::TimeSpan, repeatCount: u32) -> HRESULT
}}
impl IConstantForceEffect {
    #[inline] pub unsafe fn set_parameters(&self, vector: ::rt::gen::windows::foundation::numerics::Vector3, duration: ::rt::gen::windows::foundation::TimeSpan) -> Result<()> {
        let hr = ((*self.lpVtbl).SetParameters)(self as *const _ as *mut _, vector, duration);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_parameters_with_envelope(&self, vector: ::rt::gen::windows::foundation::numerics::Vector3, attackGain: f32, sustainGain: f32, releaseGain: f32, startDelay: ::rt::gen::windows::foundation::TimeSpan, attackDuration: ::rt::gen::windows::foundation::TimeSpan, sustainDuration: ::rt::gen::windows::foundation::TimeSpan, releaseDuration: ::rt::gen::windows::foundation::TimeSpan, repeatCount: u32) -> Result<()> {
        let hr = ((*self.lpVtbl).SetParametersWithEnvelope)(self as *const _ as *mut _, vector, attackGain, sustainGain, releaseGain, startDelay, attackDuration, sustainDuration, releaseDuration, repeatCount);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
}
RT_CLASS!{class ConstantForceEffect: IForceFeedbackEffect}
impl RtActivatable<IActivationFactory> for ConstantForceEffect {}
DEFINE_CLSID!(ConstantForceEffect(&[87,105,110,100,111,119,115,46,71,97,109,105,110,103,46,73,110,112,117,116,46,70,111,114,99,101,70,101,101,100,98,97,99,107,46,67,111,110,115,116,97,110,116,70,111,114,99,101,69,102,102,101,99,116,0]) [CLSID_ConstantForceEffect]);
DEFINE_IID!(IID_IForceFeedbackEffect, 2709502476, 10980, 18626, 128, 99, 234, 189, 7, 119, 203, 137);
RT_INTERFACE!{interface IForceFeedbackEffect(IForceFeedbackEffectVtbl): IInspectable(IInspectableVtbl) [IID_IForceFeedbackEffect] {
    fn get_Gain(&self, out: *mut f64) -> HRESULT,
    fn put_Gain(&self, value: f64) -> HRESULT,
    fn get_State(&self, out: *mut ForceFeedbackEffectState) -> HRESULT,
    fn Start(&self) -> HRESULT,
    fn Stop(&self) -> HRESULT
}}
impl IForceFeedbackEffect {
    #[inline] pub unsafe fn get_gain(&self) -> Result<f64> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_Gain)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_gain(&self, value: f64) -> Result<()> {
        let hr = ((*self.lpVtbl).put_Gain)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_state(&self) -> Result<ForceFeedbackEffectState> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_State)(self as *const _ as *mut _, &mut out);
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
RT_ENUM! { enum ForceFeedbackEffectAxes: u32 {
    None (ForceFeedbackEffectAxes_None) = 0, X (ForceFeedbackEffectAxes_X) = 1, Y (ForceFeedbackEffectAxes_Y) = 2, Z (ForceFeedbackEffectAxes_Z) = 4,
}}
RT_ENUM! { enum ForceFeedbackEffectState: i32 {
    Stopped (ForceFeedbackEffectState_Stopped) = 0, Running (ForceFeedbackEffectState_Running) = 1, Paused (ForceFeedbackEffectState_Paused) = 2, Faulted (ForceFeedbackEffectState_Faulted) = 3,
}}
RT_ENUM! { enum ForceFeedbackLoadEffectResult: i32 {
    Succeeded (ForceFeedbackLoadEffectResult_Succeeded) = 0, EffectStorageFull (ForceFeedbackLoadEffectResult_EffectStorageFull) = 1, EffectNotSupported (ForceFeedbackLoadEffectResult_EffectNotSupported) = 2,
}}
DEFINE_IID!(IID_IForceFeedbackMotor, 2369601916, 42474, 17686, 128, 38, 43, 0, 247, 78, 246, 229);
RT_INTERFACE!{interface IForceFeedbackMotor(IForceFeedbackMotorVtbl): IInspectable(IInspectableVtbl) [IID_IForceFeedbackMotor] {
    fn get_AreEffectsPaused(&self, out: *mut bool) -> HRESULT,
    fn get_MasterGain(&self, out: *mut f64) -> HRESULT,
    fn put_MasterGain(&self, value: f64) -> HRESULT,
    fn get_IsEnabled(&self, out: *mut bool) -> HRESULT,
    fn get_SupportedAxes(&self, out: *mut ForceFeedbackEffectAxes) -> HRESULT,
    fn LoadEffectAsync(&self, effect: *mut IForceFeedbackEffect, out: *mut *mut ::rt::gen::windows::foundation::IAsyncOperation<ForceFeedbackLoadEffectResult>) -> HRESULT,
    fn PauseAllEffects(&self) -> HRESULT,
    fn ResumeAllEffects(&self) -> HRESULT,
    fn StopAllEffects(&self) -> HRESULT,
    fn TryDisableAsync(&self, out: *mut *mut ::rt::gen::windows::foundation::IAsyncOperation<bool>) -> HRESULT,
    fn TryEnableAsync(&self, out: *mut *mut ::rt::gen::windows::foundation::IAsyncOperation<bool>) -> HRESULT,
    fn TryResetAsync(&self, out: *mut *mut ::rt::gen::windows::foundation::IAsyncOperation<bool>) -> HRESULT,
    fn TryUnloadEffectAsync(&self, effect: *mut IForceFeedbackEffect, out: *mut *mut ::rt::gen::windows::foundation::IAsyncOperation<bool>) -> HRESULT
}}
impl IForceFeedbackMotor {
    #[inline] pub unsafe fn get_are_effects_paused(&self) -> Result<bool> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_AreEffectsPaused)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_master_gain(&self) -> Result<f64> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_MasterGain)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_master_gain(&self, value: f64) -> Result<()> {
        let hr = ((*self.lpVtbl).put_MasterGain)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_is_enabled(&self) -> Result<bool> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_IsEnabled)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_supported_axes(&self) -> Result<ForceFeedbackEffectAxes> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_SupportedAxes)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn load_effect_async(&self, effect: &IForceFeedbackEffect) -> Result<ComPtr<::rt::gen::windows::foundation::IAsyncOperation<ForceFeedbackLoadEffectResult>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).LoadEffectAsync)(self as *const _ as *mut _, effect as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn pause_all_effects(&self) -> Result<()> {
        let hr = ((*self.lpVtbl).PauseAllEffects)(self as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn resume_all_effects(&self) -> Result<()> {
        let hr = ((*self.lpVtbl).ResumeAllEffects)(self as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn stop_all_effects(&self) -> Result<()> {
        let hr = ((*self.lpVtbl).StopAllEffects)(self as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn try_disable_async(&self) -> Result<ComPtr<::rt::gen::windows::foundation::IAsyncOperation<bool>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).TryDisableAsync)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn try_enable_async(&self) -> Result<ComPtr<::rt::gen::windows::foundation::IAsyncOperation<bool>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).TryEnableAsync)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn try_reset_async(&self) -> Result<ComPtr<::rt::gen::windows::foundation::IAsyncOperation<bool>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).TryResetAsync)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn try_unload_effect_async(&self, effect: &IForceFeedbackEffect) -> Result<ComPtr<::rt::gen::windows::foundation::IAsyncOperation<bool>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).TryUnloadEffectAsync)(self as *const _ as *mut _, effect as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
RT_CLASS!{class ForceFeedbackMotor: IForceFeedbackMotor}
DEFINE_IID!(IID_IPeriodicForceEffect, 1548826839, 64629, 19794, 154, 10, 239, 228, 202, 181, 254, 100);
RT_INTERFACE!{interface IPeriodicForceEffect(IPeriodicForceEffectVtbl): IInspectable(IInspectableVtbl) [IID_IPeriodicForceEffect] {
    fn get_Kind(&self, out: *mut PeriodicForceEffectKind) -> HRESULT,
    fn SetParameters(&self, vector: ::rt::gen::windows::foundation::numerics::Vector3, frequency: f32, phase: f32, bias: f32, duration: ::rt::gen::windows::foundation::TimeSpan) -> HRESULT,
    fn SetParametersWithEnvelope(&self, vector: ::rt::gen::windows::foundation::numerics::Vector3, frequency: f32, phase: f32, bias: f32, attackGain: f32, sustainGain: f32, releaseGain: f32, startDelay: ::rt::gen::windows::foundation::TimeSpan, attackDuration: ::rt::gen::windows::foundation::TimeSpan, sustainDuration: ::rt::gen::windows::foundation::TimeSpan, releaseDuration: ::rt::gen::windows::foundation::TimeSpan, repeatCount: u32) -> HRESULT
}}
impl IPeriodicForceEffect {
    #[inline] pub unsafe fn get_kind(&self) -> Result<PeriodicForceEffectKind> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_Kind)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_parameters(&self, vector: ::rt::gen::windows::foundation::numerics::Vector3, frequency: f32, phase: f32, bias: f32, duration: ::rt::gen::windows::foundation::TimeSpan) -> Result<()> {
        let hr = ((*self.lpVtbl).SetParameters)(self as *const _ as *mut _, vector, frequency, phase, bias, duration);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_parameters_with_envelope(&self, vector: ::rt::gen::windows::foundation::numerics::Vector3, frequency: f32, phase: f32, bias: f32, attackGain: f32, sustainGain: f32, releaseGain: f32, startDelay: ::rt::gen::windows::foundation::TimeSpan, attackDuration: ::rt::gen::windows::foundation::TimeSpan, sustainDuration: ::rt::gen::windows::foundation::TimeSpan, releaseDuration: ::rt::gen::windows::foundation::TimeSpan, repeatCount: u32) -> Result<()> {
        let hr = ((*self.lpVtbl).SetParametersWithEnvelope)(self as *const _ as *mut _, vector, frequency, phase, bias, attackGain, sustainGain, releaseGain, startDelay, attackDuration, sustainDuration, releaseDuration, repeatCount);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
}
RT_CLASS!{class PeriodicForceEffect: IForceFeedbackEffect}
impl RtActivatable<IPeriodicForceEffectFactory> for PeriodicForceEffect {}
impl PeriodicForceEffect {
    #[inline] pub fn create_instance(effectKind: PeriodicForceEffectKind) -> Result<ComPtr<PeriodicForceEffect>> { unsafe {
        <Self as RtActivatable<IPeriodicForceEffectFactory>>::get_activation_factory().create_instance(effectKind)
    }}
}
DEFINE_CLSID!(PeriodicForceEffect(&[87,105,110,100,111,119,115,46,71,97,109,105,110,103,46,73,110,112,117,116,46,70,111,114,99,101,70,101,101,100,98,97,99,107,46,80,101,114,105,111,100,105,99,70,111,114,99,101,69,102,102,101,99,116,0]) [CLSID_PeriodicForceEffect]);
DEFINE_IID!(IID_IPeriodicForceEffectFactory, 1868753690, 38993, 18299, 179, 24, 53, 236, 170, 21, 7, 15);
RT_INTERFACE!{static interface IPeriodicForceEffectFactory(IPeriodicForceEffectFactoryVtbl): IInspectable(IInspectableVtbl) [IID_IPeriodicForceEffectFactory] {
    fn CreateInstance(&self, effectKind: PeriodicForceEffectKind, out: *mut *mut PeriodicForceEffect) -> HRESULT
}}
impl IPeriodicForceEffectFactory {
    #[inline] pub unsafe fn create_instance(&self, effectKind: PeriodicForceEffectKind) -> Result<ComPtr<PeriodicForceEffect>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).CreateInstance)(self as *const _ as *mut _, effectKind, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
RT_ENUM! { enum PeriodicForceEffectKind: i32 {
    SquareWave (PeriodicForceEffectKind_SquareWave) = 0, SineWave (PeriodicForceEffectKind_SineWave) = 1, TriangleWave (PeriodicForceEffectKind_TriangleWave) = 2, SawtoothWaveUp (PeriodicForceEffectKind_SawtoothWaveUp) = 3, SawtoothWaveDown (PeriodicForceEffectKind_SawtoothWaveDown) = 4,
}}
DEFINE_IID!(IID_IRampForceEffect, 4059566681, 7334, 16512, 181, 109, 180, 63, 51, 84, 208, 82);
RT_INTERFACE!{interface IRampForceEffect(IRampForceEffectVtbl): IInspectable(IInspectableVtbl) [IID_IRampForceEffect] {
    fn SetParameters(&self, startVector: ::rt::gen::windows::foundation::numerics::Vector3, endVector: ::rt::gen::windows::foundation::numerics::Vector3, duration: ::rt::gen::windows::foundation::TimeSpan) -> HRESULT,
    fn SetParametersWithEnvelope(&self, startVector: ::rt::gen::windows::foundation::numerics::Vector3, endVector: ::rt::gen::windows::foundation::numerics::Vector3, attackGain: f32, sustainGain: f32, releaseGain: f32, startDelay: ::rt::gen::windows::foundation::TimeSpan, attackDuration: ::rt::gen::windows::foundation::TimeSpan, sustainDuration: ::rt::gen::windows::foundation::TimeSpan, releaseDuration: ::rt::gen::windows::foundation::TimeSpan, repeatCount: u32) -> HRESULT
}}
impl IRampForceEffect {
    #[inline] pub unsafe fn set_parameters(&self, startVector: ::rt::gen::windows::foundation::numerics::Vector3, endVector: ::rt::gen::windows::foundation::numerics::Vector3, duration: ::rt::gen::windows::foundation::TimeSpan) -> Result<()> {
        let hr = ((*self.lpVtbl).SetParameters)(self as *const _ as *mut _, startVector, endVector, duration);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_parameters_with_envelope(&self, startVector: ::rt::gen::windows::foundation::numerics::Vector3, endVector: ::rt::gen::windows::foundation::numerics::Vector3, attackGain: f32, sustainGain: f32, releaseGain: f32, startDelay: ::rt::gen::windows::foundation::TimeSpan, attackDuration: ::rt::gen::windows::foundation::TimeSpan, sustainDuration: ::rt::gen::windows::foundation::TimeSpan, releaseDuration: ::rt::gen::windows::foundation::TimeSpan, repeatCount: u32) -> Result<()> {
        let hr = ((*self.lpVtbl).SetParametersWithEnvelope)(self as *const _ as *mut _, startVector, endVector, attackGain, sustainGain, releaseGain, startDelay, attackDuration, sustainDuration, releaseDuration, repeatCount);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
}
RT_CLASS!{class RampForceEffect: IForceFeedbackEffect}
impl RtActivatable<IActivationFactory> for RampForceEffect {}
DEFINE_CLSID!(RampForceEffect(&[87,105,110,100,111,119,115,46,71,97,109,105,110,103,46,73,110,112,117,116,46,70,111,114,99,101,70,101,101,100,98,97,99,107,46,82,97,109,112,70,111,114,99,101,69,102,102,101,99,116,0]) [CLSID_RampForceEffect]);
} // Windows.Gaming.Input.ForceFeedback
pub mod preview { // Windows.Gaming.Input.Preview
use ::prelude::*;
RT_CLASS!{static class GameControllerProviderInfo}
impl RtActivatable<IGameControllerProviderInfoStatics> for GameControllerProviderInfo {}
impl GameControllerProviderInfo {
    #[inline] pub fn get_parent_provider_id(provider: &super::custom::IGameControllerProvider) -> Result<HString> { unsafe {
        <Self as RtActivatable<IGameControllerProviderInfoStatics>>::get_activation_factory().get_parent_provider_id(provider)
    }}
    #[inline] pub fn get_provider_id(provider: &super::custom::IGameControllerProvider) -> Result<HString> { unsafe {
        <Self as RtActivatable<IGameControllerProviderInfoStatics>>::get_activation_factory().get_provider_id(provider)
    }}
}
DEFINE_CLSID!(GameControllerProviderInfo(&[87,105,110,100,111,119,115,46,71,97,109,105,110,103,46,73,110,112,117,116,46,80,114,101,118,105,101,119,46,71,97,109,101,67,111,110,116,114,111,108,108,101,114,80,114,111,118,105,100,101,114,73,110,102,111,0]) [CLSID_GameControllerProviderInfo]);
DEFINE_IID!(IID_IGameControllerProviderInfoStatics, 199354053, 55741, 17646, 131, 98, 72, 139, 46, 70, 75, 251);
RT_INTERFACE!{static interface IGameControllerProviderInfoStatics(IGameControllerProviderInfoStaticsVtbl): IInspectable(IInspectableVtbl) [IID_IGameControllerProviderInfoStatics] {
    fn GetParentProviderId(&self, provider: *mut super::custom::IGameControllerProvider, out: *mut HSTRING) -> HRESULT,
    fn GetProviderId(&self, provider: *mut super::custom::IGameControllerProvider, out: *mut HSTRING) -> HRESULT
}}
impl IGameControllerProviderInfoStatics {
    #[inline] pub unsafe fn get_parent_provider_id(&self, provider: &super::custom::IGameControllerProvider) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetParentProviderId)(self as *const _ as *mut _, provider as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_provider_id(&self, provider: &super::custom::IGameControllerProvider) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetProviderId)(self as *const _ as *mut _, provider as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
}
} // Windows.Gaming.Input.Preview
} // Windows.Gaming.Input
pub mod xboxlive { // Windows.Gaming.XboxLive
pub mod storage { // Windows.Gaming.XboxLive.Storage
use ::prelude::*;
DEFINE_IID!(IID_IGameSaveBlobGetResult, 2440200672, 29185, 18771, 170, 44, 64, 8, 240, 58, 239, 69);
RT_INTERFACE!{interface IGameSaveBlobGetResult(IGameSaveBlobGetResultVtbl): IInspectable(IInspectableVtbl) [IID_IGameSaveBlobGetResult] {
    fn get_Status(&self, out: *mut GameSaveErrorStatus) -> HRESULT,
    #[cfg(feature="windows-storage")] fn get_Value(&self, out: *mut *mut ::rt::gen::windows::foundation::collections::IMapView<HString, ::rt::gen::windows::storage::streams::IBuffer>) -> HRESULT
}}
impl IGameSaveBlobGetResult {
    #[inline] pub unsafe fn get_status(&self) -> Result<GameSaveErrorStatus> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_Status)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[cfg(feature="windows-storage")] #[inline] pub unsafe fn get_value(&self) -> Result<ComPtr<::rt::gen::windows::foundation::collections::IMapView<HString, ::rt::gen::windows::storage::streams::IBuffer>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Value)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
RT_CLASS!{class GameSaveBlobGetResult: IGameSaveBlobGetResult}
DEFINE_IID!(IID_IGameSaveBlobInfo, 2916319284, 47856, 17989, 182, 208, 70, 237, 175, 251, 60, 43);
RT_INTERFACE!{interface IGameSaveBlobInfo(IGameSaveBlobInfoVtbl): IInspectable(IInspectableVtbl) [IID_IGameSaveBlobInfo] {
    fn get_Name(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Size(&self, out: *mut u32) -> HRESULT
}}
impl IGameSaveBlobInfo {
    #[inline] pub unsafe fn get_name(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Name)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_size(&self) -> Result<u32> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_Size)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
}
RT_CLASS!{class GameSaveBlobInfo: IGameSaveBlobInfo}
DEFINE_IID!(IID_IGameSaveBlobInfoGetResult, 3344401794, 13975, 17087, 152, 156, 102, 93, 146, 59, 82, 49);
RT_INTERFACE!{interface IGameSaveBlobInfoGetResult(IGameSaveBlobInfoGetResultVtbl): IInspectable(IInspectableVtbl) [IID_IGameSaveBlobInfoGetResult] {
    fn get_Status(&self, out: *mut GameSaveErrorStatus) -> HRESULT,
    fn get_Value(&self, out: *mut *mut ::rt::gen::windows::foundation::collections::IVectorView<GameSaveBlobInfo>) -> HRESULT
}}
impl IGameSaveBlobInfoGetResult {
    #[inline] pub unsafe fn get_status(&self) -> Result<GameSaveErrorStatus> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_Status)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_value(&self) -> Result<ComPtr<::rt::gen::windows::foundation::collections::IVectorView<GameSaveBlobInfo>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Value)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
RT_CLASS!{class GameSaveBlobInfoGetResult: IGameSaveBlobInfoGetResult}
DEFINE_IID!(IID_IGameSaveBlobInfoQuery, 2682090674, 61166, 17531, 169, 210, 127, 150, 192, 248, 50, 8);
RT_INTERFACE!{interface IGameSaveBlobInfoQuery(IGameSaveBlobInfoQueryVtbl): IInspectable(IInspectableVtbl) [IID_IGameSaveBlobInfoQuery] {
    fn GetBlobInfoAsync(&self, out: *mut *mut ::rt::gen::windows::foundation::IAsyncOperation<GameSaveBlobInfoGetResult>) -> HRESULT,
    fn GetBlobInfoWithIndexAndMaxAsync(&self, startIndex: u32, maxNumberOfItems: u32, out: *mut *mut ::rt::gen::windows::foundation::IAsyncOperation<GameSaveBlobInfoGetResult>) -> HRESULT,
    fn GetItemCountAsync(&self, out: *mut *mut ::rt::gen::windows::foundation::IAsyncOperation<u32>) -> HRESULT
}}
impl IGameSaveBlobInfoQuery {
    #[inline] pub unsafe fn get_blob_info_async(&self) -> Result<ComPtr<::rt::gen::windows::foundation::IAsyncOperation<GameSaveBlobInfoGetResult>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetBlobInfoAsync)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_blob_info_with_index_and_max_async(&self, startIndex: u32, maxNumberOfItems: u32) -> Result<ComPtr<::rt::gen::windows::foundation::IAsyncOperation<GameSaveBlobInfoGetResult>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetBlobInfoWithIndexAndMaxAsync)(self as *const _ as *mut _, startIndex, maxNumberOfItems, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_item_count_async(&self) -> Result<ComPtr<::rt::gen::windows::foundation::IAsyncOperation<u32>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetItemCountAsync)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
RT_CLASS!{class GameSaveBlobInfoQuery: IGameSaveBlobInfoQuery}
DEFINE_IID!(IID_IGameSaveContainer, 3284176777, 22079, 20173, 156, 111, 51, 253, 14, 50, 61, 16);
RT_INTERFACE!{interface IGameSaveContainer(IGameSaveContainerVtbl): IInspectable(IInspectableVtbl) [IID_IGameSaveContainer] {
    fn get_Name(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Provider(&self, out: *mut *mut GameSaveProvider) -> HRESULT,
    #[cfg(not(feature="windows-storage"))] fn __Dummy2(&self) -> (),
    #[cfg(feature="windows-storage")] fn SubmitUpdatesAsync(&self, blobsToWrite: *mut ::rt::gen::windows::foundation::collections::IMapView<HString, ::rt::gen::windows::storage::streams::IBuffer>, blobsToDelete: *mut ::rt::gen::windows::foundation::collections::IIterable<HString>, displayName: HSTRING, out: *mut *mut ::rt::gen::windows::foundation::IAsyncOperation<GameSaveOperationResult>) -> HRESULT,
    #[cfg(not(feature="windows-storage"))] fn __Dummy3(&self) -> (),
    #[cfg(feature="windows-storage")] fn ReadAsync(&self, blobsToRead: *mut ::rt::gen::windows::foundation::collections::IMapView<HString, ::rt::gen::windows::storage::streams::IBuffer>, out: *mut *mut ::rt::gen::windows::foundation::IAsyncOperation<GameSaveOperationResult>) -> HRESULT,
    fn GetAsync(&self, blobsToRead: *mut ::rt::gen::windows::foundation::collections::IIterable<HString>, out: *mut *mut ::rt::gen::windows::foundation::IAsyncOperation<GameSaveBlobGetResult>) -> HRESULT,
    fn SubmitPropertySetUpdatesAsync(&self, blobsToWrite: *mut ::rt::gen::windows::foundation::collections::IPropertySet, blobsToDelete: *mut ::rt::gen::windows::foundation::collections::IIterable<HString>, displayName: HSTRING, out: *mut *mut ::rt::gen::windows::foundation::IAsyncOperation<GameSaveOperationResult>) -> HRESULT,
    fn CreateBlobInfoQuery(&self, blobNamePrefix: HSTRING, out: *mut *mut GameSaveBlobInfoQuery) -> HRESULT
}}
impl IGameSaveContainer {
    #[inline] pub unsafe fn get_name(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Name)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_provider(&self) -> Result<ComPtr<GameSaveProvider>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Provider)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[cfg(feature="windows-storage")] #[inline] pub unsafe fn submit_updates_async(&self, blobsToWrite: &::rt::gen::windows::foundation::collections::IMapView<HString, ::rt::gen::windows::storage::streams::IBuffer>, blobsToDelete: &::rt::gen::windows::foundation::collections::IIterable<HString>, displayName: &HStringArg) -> Result<ComPtr<::rt::gen::windows::foundation::IAsyncOperation<GameSaveOperationResult>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).SubmitUpdatesAsync)(self as *const _ as *mut _, blobsToWrite as *const _ as *mut _, blobsToDelete as *const _ as *mut _, displayName.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[cfg(feature="windows-storage")] #[inline] pub unsafe fn read_async(&self, blobsToRead: &::rt::gen::windows::foundation::collections::IMapView<HString, ::rt::gen::windows::storage::streams::IBuffer>) -> Result<ComPtr<::rt::gen::windows::foundation::IAsyncOperation<GameSaveOperationResult>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).ReadAsync)(self as *const _ as *mut _, blobsToRead as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_async(&self, blobsToRead: &::rt::gen::windows::foundation::collections::IIterable<HString>) -> Result<ComPtr<::rt::gen::windows::foundation::IAsyncOperation<GameSaveBlobGetResult>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetAsync)(self as *const _ as *mut _, blobsToRead as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn submit_property_set_updates_async(&self, blobsToWrite: &::rt::gen::windows::foundation::collections::IPropertySet, blobsToDelete: &::rt::gen::windows::foundation::collections::IIterable<HString>, displayName: &HStringArg) -> Result<ComPtr<::rt::gen::windows::foundation::IAsyncOperation<GameSaveOperationResult>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).SubmitPropertySetUpdatesAsync)(self as *const _ as *mut _, blobsToWrite as *const _ as *mut _, blobsToDelete as *const _ as *mut _, displayName.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn create_blob_info_query(&self, blobNamePrefix: &HStringArg) -> Result<ComPtr<GameSaveBlobInfoQuery>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).CreateBlobInfoQuery)(self as *const _ as *mut _, blobNamePrefix.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
RT_CLASS!{class GameSaveContainer: IGameSaveContainer}
DEFINE_IID!(IID_IGameSaveContainerInfo, 3085071104, 5469, 19380, 178, 186, 147, 3, 6, 243, 145, 181);
RT_INTERFACE!{interface IGameSaveContainerInfo(IGameSaveContainerInfoVtbl): IInspectable(IInspectableVtbl) [IID_IGameSaveContainerInfo] {
    fn get_Name(&self, out: *mut HSTRING) -> HRESULT,
    fn get_TotalSize(&self, out: *mut u64) -> HRESULT,
    fn get_DisplayName(&self, out: *mut HSTRING) -> HRESULT,
    fn get_LastModifiedTime(&self, out: *mut ::rt::gen::windows::foundation::DateTime) -> HRESULT,
    fn get_NeedsSync(&self, out: *mut bool) -> HRESULT
}}
impl IGameSaveContainerInfo {
    #[inline] pub unsafe fn get_name(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Name)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_total_size(&self) -> Result<u64> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_TotalSize)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_display_name(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_DisplayName)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_last_modified_time(&self) -> Result<::rt::gen::windows::foundation::DateTime> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_LastModifiedTime)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_needs_sync(&self) -> Result<bool> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_NeedsSync)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
}
RT_CLASS!{class GameSaveContainerInfo: IGameSaveContainerInfo}
DEFINE_IID!(IID_IGameSaveContainerInfoGetResult, 4291104116, 50561, 20381, 158, 57, 48, 161, 12, 30, 76, 80);
RT_INTERFACE!{interface IGameSaveContainerInfoGetResult(IGameSaveContainerInfoGetResultVtbl): IInspectable(IInspectableVtbl) [IID_IGameSaveContainerInfoGetResult] {
    fn get_Status(&self, out: *mut GameSaveErrorStatus) -> HRESULT,
    fn get_Value(&self, out: *mut *mut ::rt::gen::windows::foundation::collections::IVectorView<GameSaveContainerInfo>) -> HRESULT
}}
impl IGameSaveContainerInfoGetResult {
    #[inline] pub unsafe fn get_status(&self) -> Result<GameSaveErrorStatus> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_Status)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_value(&self) -> Result<ComPtr<::rt::gen::windows::foundation::collections::IVectorView<GameSaveContainerInfo>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Value)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
RT_CLASS!{class GameSaveContainerInfoGetResult: IGameSaveContainerInfoGetResult}
DEFINE_IID!(IID_IGameSaveContainerInfoQuery, 1016391779, 28544, 17191, 147, 39, 255, 193, 26, 253, 66, 179);
RT_INTERFACE!{interface IGameSaveContainerInfoQuery(IGameSaveContainerInfoQueryVtbl): IInspectable(IInspectableVtbl) [IID_IGameSaveContainerInfoQuery] {
    fn GetContainerInfoAsync(&self, out: *mut *mut ::rt::gen::windows::foundation::IAsyncOperation<GameSaveContainerInfoGetResult>) -> HRESULT,
    fn GetContainerInfoWithIndexAndMaxAsync(&self, startIndex: u32, maxNumberOfItems: u32, out: *mut *mut ::rt::gen::windows::foundation::IAsyncOperation<GameSaveContainerInfoGetResult>) -> HRESULT,
    fn GetItemCountAsync(&self, out: *mut *mut ::rt::gen::windows::foundation::IAsyncOperation<u32>) -> HRESULT
}}
impl IGameSaveContainerInfoQuery {
    #[inline] pub unsafe fn get_container_info_async(&self) -> Result<ComPtr<::rt::gen::windows::foundation::IAsyncOperation<GameSaveContainerInfoGetResult>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetContainerInfoAsync)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_container_info_with_index_and_max_async(&self, startIndex: u32, maxNumberOfItems: u32) -> Result<ComPtr<::rt::gen::windows::foundation::IAsyncOperation<GameSaveContainerInfoGetResult>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetContainerInfoWithIndexAndMaxAsync)(self as *const _ as *mut _, startIndex, maxNumberOfItems, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_item_count_async(&self) -> Result<ComPtr<::rt::gen::windows::foundation::IAsyncOperation<u32>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetItemCountAsync)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
RT_CLASS!{class GameSaveContainerInfoQuery: IGameSaveContainerInfoQuery}
RT_ENUM! { enum GameSaveErrorStatus: i32 {
    Ok (GameSaveErrorStatus_Ok) = 0, Abort (GameSaveErrorStatus_Abort) = -2147467260, InvalidContainerName (GameSaveErrorStatus_InvalidContainerName) = -2138898431, NoAccess (GameSaveErrorStatus_NoAccess) = -2138898430, OutOfLocalStorage (GameSaveErrorStatus_OutOfLocalStorage) = -2138898429, UserCanceled (GameSaveErrorStatus_UserCanceled) = -2138898428, UpdateTooBig (GameSaveErrorStatus_UpdateTooBig) = -2138898427, QuotaExceeded (GameSaveErrorStatus_QuotaExceeded) = -2138898426, ProvidedBufferTooSmall (GameSaveErrorStatus_ProvidedBufferTooSmall) = -2138898425, BlobNotFound (GameSaveErrorStatus_BlobNotFound) = -2138898424, NoXboxLiveInfo (GameSaveErrorStatus_NoXboxLiveInfo) = -2138898423, ContainerNotInSync (GameSaveErrorStatus_ContainerNotInSync) = -2138898422, ContainerSyncFailed (GameSaveErrorStatus_ContainerSyncFailed) = -2138898421, UserHasNoXboxLiveInfo (GameSaveErrorStatus_UserHasNoXboxLiveInfo) = -2138898420, ObjectExpired (GameSaveErrorStatus_ObjectExpired) = -2138898419,
}}
DEFINE_IID!(IID_IGameSaveOperationResult, 3473873413, 9376, 17794, 154, 85, 177, 187, 187, 147, 136, 216);
RT_INTERFACE!{interface IGameSaveOperationResult(IGameSaveOperationResultVtbl): IInspectable(IInspectableVtbl) [IID_IGameSaveOperationResult] {
    fn get_Status(&self, out: *mut GameSaveErrorStatus) -> HRESULT
}}
impl IGameSaveOperationResult {
    #[inline] pub unsafe fn get_status(&self) -> Result<GameSaveErrorStatus> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_Status)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
}
RT_CLASS!{class GameSaveOperationResult: IGameSaveOperationResult}
DEFINE_IID!(IID_IGameSaveProvider, 2426798996, 33022, 16913, 151, 248, 165, 222, 20, 221, 149, 210);
RT_INTERFACE!{interface IGameSaveProvider(IGameSaveProviderVtbl): IInspectable(IInspectableVtbl) [IID_IGameSaveProvider] {
    #[cfg(not(feature="windows-system"))] fn __Dummy0(&self) -> (),
    #[cfg(feature="windows-system")] fn get_User(&self, out: *mut *mut ::rt::gen::windows::system::User) -> HRESULT,
    fn CreateContainer(&self, name: HSTRING, out: *mut *mut GameSaveContainer) -> HRESULT,
    fn DeleteContainerAsync(&self, name: HSTRING, out: *mut *mut ::rt::gen::windows::foundation::IAsyncOperation<GameSaveOperationResult>) -> HRESULT,
    fn CreateContainerInfoQuery(&self, out: *mut *mut GameSaveContainerInfoQuery) -> HRESULT,
    fn CreateContainerInfoQueryWithName(&self, containerNamePrefix: HSTRING, out: *mut *mut GameSaveContainerInfoQuery) -> HRESULT,
    fn GetRemainingBytesInQuotaAsync(&self, out: *mut *mut ::rt::gen::windows::foundation::IAsyncOperation<i64>) -> HRESULT,
    fn get_ContainersChangedSinceLastSync(&self, out: *mut *mut ::rt::gen::windows::foundation::collections::IVectorView<HString>) -> HRESULT
}}
impl IGameSaveProvider {
    #[cfg(feature="windows-system")] #[inline] pub unsafe fn get_user(&self) -> Result<ComPtr<::rt::gen::windows::system::User>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_User)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn create_container(&self, name: &HStringArg) -> Result<ComPtr<GameSaveContainer>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).CreateContainer)(self as *const _ as *mut _, name.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn delete_container_async(&self, name: &HStringArg) -> Result<ComPtr<::rt::gen::windows::foundation::IAsyncOperation<GameSaveOperationResult>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).DeleteContainerAsync)(self as *const _ as *mut _, name.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn create_container_info_query(&self) -> Result<ComPtr<GameSaveContainerInfoQuery>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).CreateContainerInfoQuery)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn create_container_info_query_with_name(&self, containerNamePrefix: &HStringArg) -> Result<ComPtr<GameSaveContainerInfoQuery>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).CreateContainerInfoQueryWithName)(self as *const _ as *mut _, containerNamePrefix.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_remaining_bytes_in_quota_async(&self) -> Result<ComPtr<::rt::gen::windows::foundation::IAsyncOperation<i64>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetRemainingBytesInQuotaAsync)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_containers_changed_since_last_sync(&self) -> Result<ComPtr<::rt::gen::windows::foundation::collections::IVectorView<HString>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_ContainersChangedSinceLastSync)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
RT_CLASS!{class GameSaveProvider: IGameSaveProvider}
impl RtActivatable<IGameSaveProviderStatics> for GameSaveProvider {}
impl GameSaveProvider {
    #[cfg(feature="windows-system")] #[inline] pub fn get_for_user_async(user: &::rt::gen::windows::system::User, serviceConfigId: &HStringArg) -> Result<ComPtr<::rt::gen::windows::foundation::IAsyncOperation<GameSaveProviderGetResult>>> { unsafe {
        <Self as RtActivatable<IGameSaveProviderStatics>>::get_activation_factory().get_for_user_async(user, serviceConfigId)
    }}
    #[cfg(feature="windows-system")] #[inline] pub fn get_sync_on_demand_for_user_async(user: &::rt::gen::windows::system::User, serviceConfigId: &HStringArg) -> Result<ComPtr<::rt::gen::windows::foundation::IAsyncOperation<GameSaveProviderGetResult>>> { unsafe {
        <Self as RtActivatable<IGameSaveProviderStatics>>::get_activation_factory().get_sync_on_demand_for_user_async(user, serviceConfigId)
    }}
}
DEFINE_CLSID!(GameSaveProvider(&[87,105,110,100,111,119,115,46,71,97,109,105,110,103,46,88,98,111,120,76,105,118,101,46,83,116,111,114,97,103,101,46,71,97,109,101,83,97,118,101,80,114,111,118,105,100,101,114,0]) [CLSID_GameSaveProvider]);
DEFINE_IID!(IID_IGameSaveProviderGetResult, 985204758, 54163, 19813, 172, 22, 65, 195, 230, 122, 185, 69);
RT_INTERFACE!{interface IGameSaveProviderGetResult(IGameSaveProviderGetResultVtbl): IInspectable(IInspectableVtbl) [IID_IGameSaveProviderGetResult] {
    fn get_Status(&self, out: *mut GameSaveErrorStatus) -> HRESULT,
    fn get_Value(&self, out: *mut *mut GameSaveProvider) -> HRESULT
}}
impl IGameSaveProviderGetResult {
    #[inline] pub unsafe fn get_status(&self) -> Result<GameSaveErrorStatus> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_Status)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_value(&self) -> Result<ComPtr<GameSaveProvider>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Value)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
RT_CLASS!{class GameSaveProviderGetResult: IGameSaveProviderGetResult}
DEFINE_IID!(IID_IGameSaveProviderStatics, 3491577552, 31491, 17565, 140, 189, 52, 2, 132, 42, 16, 72);
RT_INTERFACE!{static interface IGameSaveProviderStatics(IGameSaveProviderStaticsVtbl): IInspectable(IInspectableVtbl) [IID_IGameSaveProviderStatics] {
    #[cfg(feature="windows-system")] fn GetForUserAsync(&self, user: *mut ::rt::gen::windows::system::User, serviceConfigId: HSTRING, out: *mut *mut ::rt::gen::windows::foundation::IAsyncOperation<GameSaveProviderGetResult>) -> HRESULT,
    #[cfg(feature="windows-system")] fn GetSyncOnDemandForUserAsync(&self, user: *mut ::rt::gen::windows::system::User, serviceConfigId: HSTRING, out: *mut *mut ::rt::gen::windows::foundation::IAsyncOperation<GameSaveProviderGetResult>) -> HRESULT
}}
impl IGameSaveProviderStatics {
    #[cfg(feature="windows-system")] #[inline] pub unsafe fn get_for_user_async(&self, user: &::rt::gen::windows::system::User, serviceConfigId: &HStringArg) -> Result<ComPtr<::rt::gen::windows::foundation::IAsyncOperation<GameSaveProviderGetResult>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetForUserAsync)(self as *const _ as *mut _, user as *const _ as *mut _, serviceConfigId.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[cfg(feature="windows-system")] #[inline] pub unsafe fn get_sync_on_demand_for_user_async(&self, user: &::rt::gen::windows::system::User, serviceConfigId: &HStringArg) -> Result<ComPtr<::rt::gen::windows::foundation::IAsyncOperation<GameSaveProviderGetResult>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetSyncOnDemandForUserAsync)(self as *const _ as *mut _, user as *const _ as *mut _, serviceConfigId.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
} // Windows.Gaming.XboxLive.Storage
} // Windows.Gaming.XboxLive
pub mod preview { // Windows.Gaming.Preview
pub mod gamesenumeration { // Windows.Gaming.Preview.GamesEnumeration
use ::prelude::*;
RT_CLASS!{static class GameList}
impl RtActivatable<IGameListStatics> for GameList {}
impl RtActivatable<IGameListStatics2> for GameList {}
impl GameList {
    #[inline] pub fn find_all_async() -> Result<ComPtr<::rt::gen::windows::foundation::IAsyncOperation<::rt::gen::windows::foundation::collections::IVectorView<GameListEntry>>>> { unsafe {
        <Self as RtActivatable<IGameListStatics>>::get_activation_factory().find_all_async()
    }}
    #[inline] pub fn find_all_async_package_family_name(packageFamilyName: &HStringArg) -> Result<ComPtr<::rt::gen::windows::foundation::IAsyncOperation<::rt::gen::windows::foundation::collections::IVectorView<GameListEntry>>>> { unsafe {
        <Self as RtActivatable<IGameListStatics>>::get_activation_factory().find_all_async_package_family_name(packageFamilyName)
    }}
    #[inline] pub fn add_game_added(handler: &GameListChangedEventHandler) -> Result<::rt::gen::windows::foundation::EventRegistrationToken> { unsafe {
        <Self as RtActivatable<IGameListStatics>>::get_activation_factory().add_game_added(handler)
    }}
    #[inline] pub fn remove_game_added(token: ::rt::gen::windows::foundation::EventRegistrationToken) -> Result<()> { unsafe {
        <Self as RtActivatable<IGameListStatics>>::get_activation_factory().remove_game_added(token)
    }}
    #[inline] pub fn add_game_removed(handler: &GameListRemovedEventHandler) -> Result<::rt::gen::windows::foundation::EventRegistrationToken> { unsafe {
        <Self as RtActivatable<IGameListStatics>>::get_activation_factory().add_game_removed(handler)
    }}
    #[inline] pub fn remove_game_removed(token: ::rt::gen::windows::foundation::EventRegistrationToken) -> Result<()> { unsafe {
        <Self as RtActivatable<IGameListStatics>>::get_activation_factory().remove_game_removed(token)
    }}
    #[inline] pub fn add_game_updated(handler: &GameListChangedEventHandler) -> Result<::rt::gen::windows::foundation::EventRegistrationToken> { unsafe {
        <Self as RtActivatable<IGameListStatics>>::get_activation_factory().add_game_updated(handler)
    }}
    #[inline] pub fn remove_game_updated(token: ::rt::gen::windows::foundation::EventRegistrationToken) -> Result<()> { unsafe {
        <Self as RtActivatable<IGameListStatics>>::get_activation_factory().remove_game_updated(token)
    }}
    #[inline] pub fn merge_entries_async(left: &GameListEntry, right: &GameListEntry) -> Result<ComPtr<::rt::gen::windows::foundation::IAsyncOperation<GameListEntry>>> { unsafe {
        <Self as RtActivatable<IGameListStatics2>>::get_activation_factory().merge_entries_async(left, right)
    }}
    #[inline] pub fn unmerge_entry_async(mergedEntry: &GameListEntry) -> Result<ComPtr<::rt::gen::windows::foundation::IAsyncOperation<::rt::gen::windows::foundation::collections::IVectorView<GameListEntry>>>> { unsafe {
        <Self as RtActivatable<IGameListStatics2>>::get_activation_factory().unmerge_entry_async(mergedEntry)
    }}
}
DEFINE_CLSID!(GameList(&[87,105,110,100,111,119,115,46,71,97,109,105,110,103,46,80,114,101,118,105,101,119,46,71,97,109,101,115,69,110,117,109,101,114,97,116,105,111,110,46,71,97,109,101,76,105,115,116,0]) [CLSID_GameList]);
RT_ENUM! { enum GameListCategory: i32 {
    Candidate (GameListCategory_Candidate) = 0, ConfirmedBySystem (GameListCategory_ConfirmedBySystem) = 1, ConfirmedByUser (GameListCategory_ConfirmedByUser) = 2,
}}
DEFINE_IID!(IID_GameListChangedEventHandler, 636920865, 55541, 19857, 180, 14, 83, 213, 232, 111, 222, 100);
RT_DELEGATE!{delegate GameListChangedEventHandler(GameListChangedEventHandlerVtbl, GameListChangedEventHandlerImpl) [IID_GameListChangedEventHandler] {
    fn Invoke(&self, game: *mut GameListEntry) -> HRESULT
}}
impl GameListChangedEventHandler {
    #[inline] pub unsafe fn invoke(&self, game: &GameListEntry) -> Result<()> {
        let hr = ((*self.lpVtbl).Invoke)(self as *const _ as *mut _, game as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
}
DEFINE_IID!(IID_IGameListEntry, 1935221971, 33055, 17556, 182, 156, 198, 65, 160, 198, 21, 67);
RT_INTERFACE!{interface IGameListEntry(IGameListEntryVtbl): IInspectable(IInspectableVtbl) [IID_IGameListEntry] {
    #[cfg(not(feature="windows-applicationmodel"))] fn __Dummy0(&self) -> (),
    #[cfg(feature="windows-applicationmodel")] fn get_DisplayInfo(&self, out: *mut *mut ::rt::gen::windows::applicationmodel::AppDisplayInfo) -> HRESULT,
    fn LaunchAsync(&self, out: *mut *mut ::rt::gen::windows::foundation::IAsyncOperation<bool>) -> HRESULT,
    fn get_Category(&self, out: *mut GameListCategory) -> HRESULT,
    fn get_Properties(&self, out: *mut *mut ::rt::gen::windows::foundation::collections::IMapView<HString, IInspectable>) -> HRESULT,
    fn SetCategoryAsync(&self, value: GameListCategory, out: *mut *mut ::rt::gen::windows::foundation::IAsyncAction) -> HRESULT
}}
impl IGameListEntry {
    #[cfg(feature="windows-applicationmodel")] #[inline] pub unsafe fn get_display_info(&self) -> Result<ComPtr<::rt::gen::windows::applicationmodel::AppDisplayInfo>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_DisplayInfo)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn launch_async(&self) -> Result<ComPtr<::rt::gen::windows::foundation::IAsyncOperation<bool>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).LaunchAsync)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_category(&self) -> Result<GameListCategory> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_Category)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_properties(&self) -> Result<ComPtr<::rt::gen::windows::foundation::collections::IMapView<HString, IInspectable>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Properties)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_category_async(&self, value: GameListCategory) -> Result<ComPtr<::rt::gen::windows::foundation::IAsyncAction>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).SetCategoryAsync)(self as *const _ as *mut _, value, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
RT_CLASS!{class GameListEntry: IGameListEntry}
DEFINE_IID!(IID_IGameListEntry2, 3628765067, 34633, 18981, 144, 211, 246, 197, 164, 39, 136, 109);
RT_INTERFACE!{interface IGameListEntry2(IGameListEntry2Vtbl): IInspectable(IInspectableVtbl) [IID_IGameListEntry2] {
    fn get_LaunchableState(&self, out: *mut GameListEntryLaunchableState) -> HRESULT,
    #[cfg(not(feature="windows-storage"))] fn __Dummy1(&self) -> (),
    #[cfg(feature="windows-storage")] fn get_LauncherExecutable(&self, out: *mut *mut ::rt::gen::windows::storage::IStorageFile) -> HRESULT,
    fn get_LaunchParameters(&self, out: *mut HSTRING) -> HRESULT,
    #[cfg(not(feature="windows-storage"))] fn __Dummy3(&self) -> (),
    #[cfg(feature="windows-storage")] fn SetLauncherExecutableFileAsync(&self, executableFile: *mut ::rt::gen::windows::storage::IStorageFile, out: *mut *mut ::rt::gen::windows::foundation::IAsyncAction) -> HRESULT,
    #[cfg(not(feature="windows-storage"))] fn __Dummy4(&self) -> (),
    #[cfg(feature="windows-storage")] fn SetLauncherExecutableFileWithParamsAsync(&self, executableFile: *mut ::rt::gen::windows::storage::IStorageFile, launchParams: HSTRING, out: *mut *mut ::rt::gen::windows::foundation::IAsyncAction) -> HRESULT,
    fn get_TitleId(&self, out: *mut HSTRING) -> HRESULT,
    fn SetTitleIdAsync(&self, id: HSTRING, out: *mut *mut ::rt::gen::windows::foundation::IAsyncAction) -> HRESULT,
    fn get_GameModeConfiguration(&self, out: *mut *mut GameModeConfiguration) -> HRESULT
}}
impl IGameListEntry2 {
    #[inline] pub unsafe fn get_launchable_state(&self) -> Result<GameListEntryLaunchableState> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_LaunchableState)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[cfg(feature="windows-storage")] #[inline] pub unsafe fn get_launcher_executable(&self) -> Result<ComPtr<::rt::gen::windows::storage::IStorageFile>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_LauncherExecutable)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_launch_parameters(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_LaunchParameters)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[cfg(feature="windows-storage")] #[inline] pub unsafe fn set_launcher_executable_file_async(&self, executableFile: &::rt::gen::windows::storage::IStorageFile) -> Result<ComPtr<::rt::gen::windows::foundation::IAsyncAction>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).SetLauncherExecutableFileAsync)(self as *const _ as *mut _, executableFile as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[cfg(feature="windows-storage")] #[inline] pub unsafe fn set_launcher_executable_file_with_params_async(&self, executableFile: &::rt::gen::windows::storage::IStorageFile, launchParams: &HStringArg) -> Result<ComPtr<::rt::gen::windows::foundation::IAsyncAction>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).SetLauncherExecutableFileWithParamsAsync)(self as *const _ as *mut _, executableFile as *const _ as *mut _, launchParams.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_title_id(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_TitleId)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_title_id_async(&self, id: &HStringArg) -> Result<ComPtr<::rt::gen::windows::foundation::IAsyncAction>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).SetTitleIdAsync)(self as *const _ as *mut _, id.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_game_mode_configuration(&self) -> Result<ComPtr<GameModeConfiguration>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_GameModeConfiguration)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
RT_ENUM! { enum GameListEntryLaunchableState: i32 {
    NotLaunchable (GameListEntryLaunchableState_NotLaunchable) = 0, ByLastRunningFullPath (GameListEntryLaunchableState_ByLastRunningFullPath) = 1, ByUserProvidedPath (GameListEntryLaunchableState_ByUserProvidedPath) = 2, ByTile (GameListEntryLaunchableState_ByTile) = 3,
}}
DEFINE_IID!(IID_GameListRemovedEventHandler, 281371791, 27791, 18194, 155, 56, 71, 75, 194, 46, 118, 216);
RT_DELEGATE!{delegate GameListRemovedEventHandler(GameListRemovedEventHandlerVtbl, GameListRemovedEventHandlerImpl) [IID_GameListRemovedEventHandler] {
    fn Invoke(&self, identifier: HSTRING) -> HRESULT
}}
impl GameListRemovedEventHandler {
    #[inline] pub unsafe fn invoke(&self, identifier: &HStringArg) -> Result<()> {
        let hr = ((*self.lpVtbl).Invoke)(self as *const _ as *mut _, identifier.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
}
DEFINE_IID!(IID_IGameListStatics, 769462127, 40038, 19205, 148, 92, 214, 237, 120, 73, 27, 140);
RT_INTERFACE!{static interface IGameListStatics(IGameListStaticsVtbl): IInspectable(IInspectableVtbl) [IID_IGameListStatics] {
    fn FindAllAsync(&self, out: *mut *mut ::rt::gen::windows::foundation::IAsyncOperation<::rt::gen::windows::foundation::collections::IVectorView<GameListEntry>>) -> HRESULT,
    fn FindAllAsyncPackageFamilyName(&self, packageFamilyName: HSTRING, out: *mut *mut ::rt::gen::windows::foundation::IAsyncOperation<::rt::gen::windows::foundation::collections::IVectorView<GameListEntry>>) -> HRESULT,
    fn add_GameAdded(&self, handler: *mut GameListChangedEventHandler, out: *mut ::rt::gen::windows::foundation::EventRegistrationToken) -> HRESULT,
    fn remove_GameAdded(&self, token: ::rt::gen::windows::foundation::EventRegistrationToken) -> HRESULT,
    fn add_GameRemoved(&self, handler: *mut GameListRemovedEventHandler, out: *mut ::rt::gen::windows::foundation::EventRegistrationToken) -> HRESULT,
    fn remove_GameRemoved(&self, token: ::rt::gen::windows::foundation::EventRegistrationToken) -> HRESULT,
    fn add_GameUpdated(&self, handler: *mut GameListChangedEventHandler, out: *mut ::rt::gen::windows::foundation::EventRegistrationToken) -> HRESULT,
    fn remove_GameUpdated(&self, token: ::rt::gen::windows::foundation::EventRegistrationToken) -> HRESULT
}}
impl IGameListStatics {
    #[inline] pub unsafe fn find_all_async(&self) -> Result<ComPtr<::rt::gen::windows::foundation::IAsyncOperation<::rt::gen::windows::foundation::collections::IVectorView<GameListEntry>>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).FindAllAsync)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn find_all_async_package_family_name(&self, packageFamilyName: &HStringArg) -> Result<ComPtr<::rt::gen::windows::foundation::IAsyncOperation<::rt::gen::windows::foundation::collections::IVectorView<GameListEntry>>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).FindAllAsyncPackageFamilyName)(self as *const _ as *mut _, packageFamilyName.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn add_game_added(&self, handler: &GameListChangedEventHandler) -> Result<::rt::gen::windows::foundation::EventRegistrationToken> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).add_GameAdded)(self as *const _ as *mut _, handler as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn remove_game_added(&self, token: ::rt::gen::windows::foundation::EventRegistrationToken) -> Result<()> {
        let hr = ((*self.lpVtbl).remove_GameAdded)(self as *const _ as *mut _, token);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn add_game_removed(&self, handler: &GameListRemovedEventHandler) -> Result<::rt::gen::windows::foundation::EventRegistrationToken> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).add_GameRemoved)(self as *const _ as *mut _, handler as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn remove_game_removed(&self, token: ::rt::gen::windows::foundation::EventRegistrationToken) -> Result<()> {
        let hr = ((*self.lpVtbl).remove_GameRemoved)(self as *const _ as *mut _, token);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn add_game_updated(&self, handler: &GameListChangedEventHandler) -> Result<::rt::gen::windows::foundation::EventRegistrationToken> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).add_GameUpdated)(self as *const _ as *mut _, handler as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn remove_game_updated(&self, token: ::rt::gen::windows::foundation::EventRegistrationToken) -> Result<()> {
        let hr = ((*self.lpVtbl).remove_GameUpdated)(self as *const _ as *mut _, token);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
}
DEFINE_IID!(IID_IGameListStatics2, 962535576, 59930, 17834, 146, 104, 168, 57, 5, 104, 111, 39);
RT_INTERFACE!{static interface IGameListStatics2(IGameListStatics2Vtbl): IInspectable(IInspectableVtbl) [IID_IGameListStatics2] {
    fn MergeEntriesAsync(&self, left: *mut GameListEntry, right: *mut GameListEntry, out: *mut *mut ::rt::gen::windows::foundation::IAsyncOperation<GameListEntry>) -> HRESULT,
    fn UnmergeEntryAsync(&self, mergedEntry: *mut GameListEntry, out: *mut *mut ::rt::gen::windows::foundation::IAsyncOperation<::rt::gen::windows::foundation::collections::IVectorView<GameListEntry>>) -> HRESULT
}}
impl IGameListStatics2 {
    #[inline] pub unsafe fn merge_entries_async(&self, left: &GameListEntry, right: &GameListEntry) -> Result<ComPtr<::rt::gen::windows::foundation::IAsyncOperation<GameListEntry>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).MergeEntriesAsync)(self as *const _ as *mut _, left as *const _ as *mut _, right as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn unmerge_entry_async(&self, mergedEntry: &GameListEntry) -> Result<ComPtr<::rt::gen::windows::foundation::IAsyncOperation<::rt::gen::windows::foundation::collections::IVectorView<GameListEntry>>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).UnmergeEntryAsync)(self as *const _ as *mut _, mergedEntry as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
DEFINE_IID!(IID_IGameModeConfiguration, 2028310959, 45378, 20208, 136, 48, 85, 188, 43, 228, 245, 234);
RT_INTERFACE!{interface IGameModeConfiguration(IGameModeConfigurationVtbl): IInspectable(IInspectableVtbl) [IID_IGameModeConfiguration] {
    fn get_IsEnabled(&self, out: *mut bool) -> HRESULT,
    fn put_IsEnabled(&self, value: bool) -> HRESULT,
    fn get_RelatedProcessNames(&self, out: *mut *mut ::rt::gen::windows::foundation::collections::IVector<HString>) -> HRESULT,
    fn get_PercentGpuTimeAllocatedToGame(&self, out: *mut *mut ::rt::gen::windows::foundation::IReference<i32>) -> HRESULT,
    fn put_PercentGpuTimeAllocatedToGame(&self, value: *mut ::rt::gen::windows::foundation::IReference<i32>) -> HRESULT,
    fn get_PercentGpuMemoryAllocatedToGame(&self, out: *mut *mut ::rt::gen::windows::foundation::IReference<i32>) -> HRESULT,
    fn put_PercentGpuMemoryAllocatedToGame(&self, value: *mut ::rt::gen::windows::foundation::IReference<i32>) -> HRESULT,
    fn get_PercentGpuMemoryAllocatedToSystemCompositor(&self, out: *mut *mut ::rt::gen::windows::foundation::IReference<i32>) -> HRESULT,
    fn put_PercentGpuMemoryAllocatedToSystemCompositor(&self, value: *mut ::rt::gen::windows::foundation::IReference<i32>) -> HRESULT,
    fn get_MaxCpuCount(&self, out: *mut *mut ::rt::gen::windows::foundation::IReference<i32>) -> HRESULT,
    fn put_MaxCpuCount(&self, value: *mut ::rt::gen::windows::foundation::IReference<i32>) -> HRESULT,
    fn get_CpuExclusivityMaskLow(&self, out: *mut *mut ::rt::gen::windows::foundation::IReference<i32>) -> HRESULT,
    fn put_CpuExclusivityMaskLow(&self, value: *mut ::rt::gen::windows::foundation::IReference<i32>) -> HRESULT,
    fn get_CpuExclusivityMaskHigh(&self, out: *mut *mut ::rt::gen::windows::foundation::IReference<i32>) -> HRESULT,
    fn put_CpuExclusivityMaskHigh(&self, value: *mut ::rt::gen::windows::foundation::IReference<i32>) -> HRESULT,
    fn get_AffinitizeToExclusiveCpus(&self, out: *mut bool) -> HRESULT,
    fn put_AffinitizeToExclusiveCpus(&self, value: bool) -> HRESULT,
    fn SaveAsync(&self, out: *mut *mut ::rt::gen::windows::foundation::IAsyncAction) -> HRESULT
}}
impl IGameModeConfiguration {
    #[inline] pub unsafe fn get_is_enabled(&self) -> Result<bool> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_IsEnabled)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_is_enabled(&self, value: bool) -> Result<()> {
        let hr = ((*self.lpVtbl).put_IsEnabled)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_related_process_names(&self) -> Result<ComPtr<::rt::gen::windows::foundation::collections::IVector<HString>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_RelatedProcessNames)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_percent_gpu_time_allocated_to_game(&self) -> Result<ComPtr<::rt::gen::windows::foundation::IReference<i32>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_PercentGpuTimeAllocatedToGame)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_percent_gpu_time_allocated_to_game(&self, value: &::rt::gen::windows::foundation::IReference<i32>) -> Result<()> {
        let hr = ((*self.lpVtbl).put_PercentGpuTimeAllocatedToGame)(self as *const _ as *mut _, value as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_percent_gpu_memory_allocated_to_game(&self) -> Result<ComPtr<::rt::gen::windows::foundation::IReference<i32>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_PercentGpuMemoryAllocatedToGame)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_percent_gpu_memory_allocated_to_game(&self, value: &::rt::gen::windows::foundation::IReference<i32>) -> Result<()> {
        let hr = ((*self.lpVtbl).put_PercentGpuMemoryAllocatedToGame)(self as *const _ as *mut _, value as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_percent_gpu_memory_allocated_to_system_compositor(&self) -> Result<ComPtr<::rt::gen::windows::foundation::IReference<i32>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_PercentGpuMemoryAllocatedToSystemCompositor)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_percent_gpu_memory_allocated_to_system_compositor(&self, value: &::rt::gen::windows::foundation::IReference<i32>) -> Result<()> {
        let hr = ((*self.lpVtbl).put_PercentGpuMemoryAllocatedToSystemCompositor)(self as *const _ as *mut _, value as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_max_cpu_count(&self) -> Result<ComPtr<::rt::gen::windows::foundation::IReference<i32>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_MaxCpuCount)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_max_cpu_count(&self, value: &::rt::gen::windows::foundation::IReference<i32>) -> Result<()> {
        let hr = ((*self.lpVtbl).put_MaxCpuCount)(self as *const _ as *mut _, value as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_cpu_exclusivity_mask_low(&self) -> Result<ComPtr<::rt::gen::windows::foundation::IReference<i32>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_CpuExclusivityMaskLow)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_cpu_exclusivity_mask_low(&self, value: &::rt::gen::windows::foundation::IReference<i32>) -> Result<()> {
        let hr = ((*self.lpVtbl).put_CpuExclusivityMaskLow)(self as *const _ as *mut _, value as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_cpu_exclusivity_mask_high(&self) -> Result<ComPtr<::rt::gen::windows::foundation::IReference<i32>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_CpuExclusivityMaskHigh)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_cpu_exclusivity_mask_high(&self, value: &::rt::gen::windows::foundation::IReference<i32>) -> Result<()> {
        let hr = ((*self.lpVtbl).put_CpuExclusivityMaskHigh)(self as *const _ as *mut _, value as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_affinitize_to_exclusive_cpus(&self) -> Result<bool> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_AffinitizeToExclusiveCpus)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_affinitize_to_exclusive_cpus(&self, value: bool) -> Result<()> {
        let hr = ((*self.lpVtbl).put_AffinitizeToExclusiveCpus)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn save_async(&self) -> Result<ComPtr<::rt::gen::windows::foundation::IAsyncAction>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).SaveAsync)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
RT_CLASS!{class GameModeConfiguration: IGameModeConfiguration}
DEFINE_IID!(IID_IGameModeUserConfiguration, 1926449908, 30059, 18191, 160, 194, 186, 98, 169, 7, 149, 219);
RT_INTERFACE!{interface IGameModeUserConfiguration(IGameModeUserConfigurationVtbl): IInspectable(IInspectableVtbl) [IID_IGameModeUserConfiguration] {
    fn get_GamingRelatedProcessNames(&self, out: *mut *mut ::rt::gen::windows::foundation::collections::IVector<HString>) -> HRESULT,
    fn SaveAsync(&self, out: *mut *mut ::rt::gen::windows::foundation::IAsyncAction) -> HRESULT
}}
impl IGameModeUserConfiguration {
    #[inline] pub unsafe fn get_gaming_related_process_names(&self) -> Result<ComPtr<::rt::gen::windows::foundation::collections::IVector<HString>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_GamingRelatedProcessNames)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn save_async(&self) -> Result<ComPtr<::rt::gen::windows::foundation::IAsyncAction>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).SaveAsync)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
RT_CLASS!{class GameModeUserConfiguration: IGameModeUserConfiguration}
impl RtActivatable<IGameModeUserConfigurationStatics> for GameModeUserConfiguration {}
impl GameModeUserConfiguration {
    #[inline] pub fn get_default() -> Result<ComPtr<GameModeUserConfiguration>> { unsafe {
        <Self as RtActivatable<IGameModeUserConfigurationStatics>>::get_activation_factory().get_default()
    }}
}
DEFINE_CLSID!(GameModeUserConfiguration(&[87,105,110,100,111,119,115,46,71,97,109,105,110,103,46,80,114,101,118,105,101,119,46,71,97,109,101,115,69,110,117,109,101,114,97,116,105,111,110,46,71,97,109,101,77,111,100,101,85,115,101,114,67,111,110,102,105,103,117,114,97,116,105,111,110,0]) [CLSID_GameModeUserConfiguration]);
DEFINE_IID!(IID_IGameModeUserConfigurationStatics, 1850792316, 26346, 18318, 164, 161, 245, 124, 14, 141, 0, 231);
RT_INTERFACE!{static interface IGameModeUserConfigurationStatics(IGameModeUserConfigurationStaticsVtbl): IInspectable(IInspectableVtbl) [IID_IGameModeUserConfigurationStatics] {
    fn GetDefault(&self, out: *mut *mut GameModeUserConfiguration) -> HRESULT
}}
impl IGameModeUserConfigurationStatics {
    #[inline] pub unsafe fn get_default(&self) -> Result<ComPtr<GameModeUserConfiguration>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetDefault)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
} // Windows.Gaming.Preview.GamesEnumeration
} // Windows.Gaming.Preview
