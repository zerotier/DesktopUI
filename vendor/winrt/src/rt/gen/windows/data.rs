pub mod json { // Windows.Data.Json
use ::prelude::*;
DEFINE_IID!(IID_IJsonArray, 146922934, 3261, 19098, 181, 211, 47, 133, 45, 195, 126, 129);
RT_INTERFACE!{interface IJsonArray(IJsonArrayVtbl): IInspectable(IInspectableVtbl) [IID_IJsonArray] {
    fn GetObjectAt(&self, index: u32, out: *mut *mut JsonObject) -> HRESULT,
    fn GetArrayAt(&self, index: u32, out: *mut *mut JsonArray) -> HRESULT,
    fn GetStringAt(&self, index: u32, out: *mut HSTRING) -> HRESULT,
    fn GetNumberAt(&self, index: u32, out: *mut f64) -> HRESULT,
    fn GetBooleanAt(&self, index: u32, out: *mut bool) -> HRESULT
}}
impl IJsonArray {
    #[inline] pub unsafe fn get_object_at(&self, index: u32) -> Result<ComPtr<JsonObject>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetObjectAt)(self as *const _ as *mut _, index, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_array_at(&self, index: u32) -> Result<ComPtr<JsonArray>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetArrayAt)(self as *const _ as *mut _, index, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_string_at(&self, index: u32) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetStringAt)(self as *const _ as *mut _, index, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_number_at(&self, index: u32) -> Result<f64> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).GetNumberAt)(self as *const _ as *mut _, index, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_boolean_at(&self, index: u32) -> Result<bool> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).GetBooleanAt)(self as *const _ as *mut _, index, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
}
RT_CLASS!{class JsonArray: IJsonArray}
impl RtActivatable<IJsonArrayStatics> for JsonArray {}
impl RtActivatable<IActivationFactory> for JsonArray {}
impl JsonArray {
    #[inline] pub fn parse(input: &HStringArg) -> Result<ComPtr<JsonArray>> { unsafe {
        <Self as RtActivatable<IJsonArrayStatics>>::get_activation_factory().parse(input)
    }}
    #[inline] pub fn try_parse(input: &HStringArg) -> Result<(ComPtr<JsonArray>, bool)> { unsafe {
        <Self as RtActivatable<IJsonArrayStatics>>::get_activation_factory().try_parse(input)
    }}
}
DEFINE_CLSID!(JsonArray(&[87,105,110,100,111,119,115,46,68,97,116,97,46,74,115,111,110,46,74,115,111,110,65,114,114,97,121,0]) [CLSID_JsonArray]);
DEFINE_IID!(IID_IJsonArrayStatics, 3675534505, 57700, 18847, 147, 226, 138, 143, 73, 187, 144, 186);
RT_INTERFACE!{static interface IJsonArrayStatics(IJsonArrayStaticsVtbl): IInspectable(IInspectableVtbl) [IID_IJsonArrayStatics] {
    fn Parse(&self, input: HSTRING, out: *mut *mut JsonArray) -> HRESULT,
    fn TryParse(&self, input: HSTRING, result: *mut *mut JsonArray, out: *mut bool) -> HRESULT
}}
impl IJsonArrayStatics {
    #[inline] pub unsafe fn parse(&self, input: &HStringArg) -> Result<ComPtr<JsonArray>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).Parse)(self as *const _ as *mut _, input.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn try_parse(&self, input: &HStringArg) -> Result<(ComPtr<JsonArray>, bool)> {
        let mut result = null_mut(); let mut out = zeroed();
        let hr = ((*self.lpVtbl).TryParse)(self as *const _ as *mut _, input.get(), &mut result, &mut out);
        if hr == S_OK { Ok((ComPtr::wrap(result), out)) } else { err(hr) }
    }
}
RT_CLASS!{static class JsonError}
impl RtActivatable<IJsonErrorStatics2> for JsonError {}
impl JsonError {
    #[inline] pub fn get_json_status(hresult: i32) -> Result<JsonErrorStatus> { unsafe {
        <Self as RtActivatable<IJsonErrorStatics2>>::get_activation_factory().get_json_status(hresult)
    }}
}
DEFINE_CLSID!(JsonError(&[87,105,110,100,111,119,115,46,68,97,116,97,46,74,115,111,110,46,74,115,111,110,69,114,114,111,114,0]) [CLSID_JsonError]);
DEFINE_IID!(IID_IJsonErrorStatics2, 1077948634, 34768, 17260, 131, 171, 252, 123, 18, 192, 204, 38);
RT_INTERFACE!{static interface IJsonErrorStatics2(IJsonErrorStatics2Vtbl): IInspectable(IInspectableVtbl) [IID_IJsonErrorStatics2] {
    fn GetJsonStatus(&self, hresult: i32, out: *mut JsonErrorStatus) -> HRESULT
}}
impl IJsonErrorStatics2 {
    #[inline] pub unsafe fn get_json_status(&self, hresult: i32) -> Result<JsonErrorStatus> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).GetJsonStatus)(self as *const _ as *mut _, hresult, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
}
RT_ENUM! { enum JsonErrorStatus: i32 {
    Unknown (JsonErrorStatus_Unknown) = 0, InvalidJsonString (JsonErrorStatus_InvalidJsonString) = 1, InvalidJsonNumber (JsonErrorStatus_InvalidJsonNumber) = 2, JsonValueNotFound (JsonErrorStatus_JsonValueNotFound) = 3, ImplementationLimit (JsonErrorStatus_ImplementationLimit) = 4,
}}
DEFINE_IID!(IID_IJsonObject, 105784541, 10690, 20355, 154, 193, 158, 225, 21, 120, 190, 179);
RT_INTERFACE!{interface IJsonObject(IJsonObjectVtbl): IInspectable(IInspectableVtbl) [IID_IJsonObject] {
    fn GetNamedValue(&self, name: HSTRING, out: *mut *mut JsonValue) -> HRESULT,
    fn SetNamedValue(&self, name: HSTRING, value: *mut IJsonValue) -> HRESULT,
    fn GetNamedObject(&self, name: HSTRING, out: *mut *mut JsonObject) -> HRESULT,
    fn GetNamedArray(&self, name: HSTRING, out: *mut *mut JsonArray) -> HRESULT,
    fn GetNamedString(&self, name: HSTRING, out: *mut HSTRING) -> HRESULT,
    fn GetNamedNumber(&self, name: HSTRING, out: *mut f64) -> HRESULT,
    fn GetNamedBoolean(&self, name: HSTRING, out: *mut bool) -> HRESULT
}}
impl IJsonObject {
    #[inline] pub unsafe fn get_named_value(&self, name: &HStringArg) -> Result<ComPtr<JsonValue>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetNamedValue)(self as *const _ as *mut _, name.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_named_value(&self, name: &HStringArg, value: &IJsonValue) -> Result<()> {
        let hr = ((*self.lpVtbl).SetNamedValue)(self as *const _ as *mut _, name.get(), value as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_named_object(&self, name: &HStringArg) -> Result<ComPtr<JsonObject>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetNamedObject)(self as *const _ as *mut _, name.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_named_array(&self, name: &HStringArg) -> Result<ComPtr<JsonArray>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetNamedArray)(self as *const _ as *mut _, name.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_named_string(&self, name: &HStringArg) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetNamedString)(self as *const _ as *mut _, name.get(), &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_named_number(&self, name: &HStringArg) -> Result<f64> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).GetNamedNumber)(self as *const _ as *mut _, name.get(), &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_named_boolean(&self, name: &HStringArg) -> Result<bool> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).GetNamedBoolean)(self as *const _ as *mut _, name.get(), &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
}
RT_CLASS!{class JsonObject: IJsonObject}
impl RtActivatable<IJsonObjectStatics> for JsonObject {}
impl RtActivatable<IActivationFactory> for JsonObject {}
impl JsonObject {
    #[inline] pub fn parse(input: &HStringArg) -> Result<ComPtr<JsonObject>> { unsafe {
        <Self as RtActivatable<IJsonObjectStatics>>::get_activation_factory().parse(input)
    }}
    #[inline] pub fn try_parse(input: &HStringArg) -> Result<(ComPtr<JsonObject>, bool)> { unsafe {
        <Self as RtActivatable<IJsonObjectStatics>>::get_activation_factory().try_parse(input)
    }}
}
DEFINE_CLSID!(JsonObject(&[87,105,110,100,111,119,115,46,68,97,116,97,46,74,115,111,110,46,74,115,111,110,79,98,106,101,99,116,0]) [CLSID_JsonObject]);
DEFINE_IID!(IID_IJsonObjectStatics, 579465561, 21726, 17880, 171, 204, 34, 96, 63, 160, 102, 160);
RT_INTERFACE!{static interface IJsonObjectStatics(IJsonObjectStaticsVtbl): IInspectable(IInspectableVtbl) [IID_IJsonObjectStatics] {
    fn Parse(&self, input: HSTRING, out: *mut *mut JsonObject) -> HRESULT,
    fn TryParse(&self, input: HSTRING, result: *mut *mut JsonObject, out: *mut bool) -> HRESULT
}}
impl IJsonObjectStatics {
    #[inline] pub unsafe fn parse(&self, input: &HStringArg) -> Result<ComPtr<JsonObject>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).Parse)(self as *const _ as *mut _, input.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn try_parse(&self, input: &HStringArg) -> Result<(ComPtr<JsonObject>, bool)> {
        let mut result = null_mut(); let mut out = zeroed();
        let hr = ((*self.lpVtbl).TryParse)(self as *const _ as *mut _, input.get(), &mut result, &mut out);
        if hr == S_OK { Ok((ComPtr::wrap(result), out)) } else { err(hr) }
    }
}
DEFINE_IID!(IID_IJsonObjectWithDefaultValues, 3647001250, 47088, 20224, 142, 68, 216, 44, 244, 21, 234, 19);
RT_INTERFACE!{interface IJsonObjectWithDefaultValues(IJsonObjectWithDefaultValuesVtbl): IInspectable(IInspectableVtbl) [IID_IJsonObjectWithDefaultValues] {
    fn GetNamedValueOrDefault(&self, name: HSTRING, defaultValue: *mut JsonValue, out: *mut *mut JsonValue) -> HRESULT,
    fn GetNamedObjectOrDefault(&self, name: HSTRING, defaultValue: *mut JsonObject, out: *mut *mut JsonObject) -> HRESULT,
    fn GetNamedStringOrDefault(&self, name: HSTRING, defaultValue: HSTRING, out: *mut HSTRING) -> HRESULT,
    fn GetNamedArrayOrDefault(&self, name: HSTRING, defaultValue: *mut JsonArray, out: *mut *mut JsonArray) -> HRESULT,
    fn GetNamedNumberOrDefault(&self, name: HSTRING, defaultValue: f64, out: *mut f64) -> HRESULT,
    fn GetNamedBooleanOrDefault(&self, name: HSTRING, defaultValue: bool, out: *mut bool) -> HRESULT
}}
impl IJsonObjectWithDefaultValues {
    #[inline] pub unsafe fn get_named_value_or_default(&self, name: &HStringArg, defaultValue: &JsonValue) -> Result<ComPtr<JsonValue>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetNamedValueOrDefault)(self as *const _ as *mut _, name.get(), defaultValue as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_named_object_or_default(&self, name: &HStringArg, defaultValue: &JsonObject) -> Result<ComPtr<JsonObject>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetNamedObjectOrDefault)(self as *const _ as *mut _, name.get(), defaultValue as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_named_string_or_default(&self, name: &HStringArg, defaultValue: &HStringArg) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetNamedStringOrDefault)(self as *const _ as *mut _, name.get(), defaultValue.get(), &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_named_array_or_default(&self, name: &HStringArg, defaultValue: &JsonArray) -> Result<ComPtr<JsonArray>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetNamedArrayOrDefault)(self as *const _ as *mut _, name.get(), defaultValue as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_named_number_or_default(&self, name: &HStringArg, defaultValue: f64) -> Result<f64> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).GetNamedNumberOrDefault)(self as *const _ as *mut _, name.get(), defaultValue, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_named_boolean_or_default(&self, name: &HStringArg, defaultValue: bool) -> Result<bool> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).GetNamedBooleanOrDefault)(self as *const _ as *mut _, name.get(), defaultValue, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
}
DEFINE_IID!(IID_IJsonValue, 2736889547, 61619, 19917, 190, 238, 25, 212, 140, 211, 237, 30);
RT_INTERFACE!{interface IJsonValue(IJsonValueVtbl): IInspectable(IInspectableVtbl) [IID_IJsonValue] {
    fn get_ValueType(&self, out: *mut JsonValueType) -> HRESULT,
    fn Stringify(&self, out: *mut HSTRING) -> HRESULT,
    fn GetString(&self, out: *mut HSTRING) -> HRESULT,
    fn GetNumber(&self, out: *mut f64) -> HRESULT,
    fn GetBoolean(&self, out: *mut bool) -> HRESULT,
    fn GetArray(&self, out: *mut *mut JsonArray) -> HRESULT,
    fn GetObject(&self, out: *mut *mut JsonObject) -> HRESULT
}}
impl IJsonValue {
    #[inline] pub unsafe fn get_value_type(&self) -> Result<JsonValueType> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_ValueType)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn stringify(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).Stringify)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_string(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetString)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_number(&self) -> Result<f64> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).GetNumber)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_boolean(&self) -> Result<bool> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).GetBoolean)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_array(&self) -> Result<ComPtr<JsonArray>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetArray)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_object(&self) -> Result<ComPtr<JsonObject>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetObject)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
RT_CLASS!{class JsonValue: IJsonValue}
impl RtActivatable<IJsonValueStatics> for JsonValue {}
impl RtActivatable<IJsonValueStatics2> for JsonValue {}
impl JsonValue {
    #[inline] pub fn parse(input: &HStringArg) -> Result<ComPtr<JsonValue>> { unsafe {
        <Self as RtActivatable<IJsonValueStatics>>::get_activation_factory().parse(input)
    }}
    #[inline] pub fn try_parse(input: &HStringArg) -> Result<(ComPtr<JsonValue>, bool)> { unsafe {
        <Self as RtActivatable<IJsonValueStatics>>::get_activation_factory().try_parse(input)
    }}
    #[inline] pub fn create_boolean_value(input: bool) -> Result<ComPtr<JsonValue>> { unsafe {
        <Self as RtActivatable<IJsonValueStatics>>::get_activation_factory().create_boolean_value(input)
    }}
    #[inline] pub fn create_number_value(input: f64) -> Result<ComPtr<JsonValue>> { unsafe {
        <Self as RtActivatable<IJsonValueStatics>>::get_activation_factory().create_number_value(input)
    }}
    #[inline] pub fn create_string_value(input: &HStringArg) -> Result<ComPtr<JsonValue>> { unsafe {
        <Self as RtActivatable<IJsonValueStatics>>::get_activation_factory().create_string_value(input)
    }}
    #[inline] pub fn create_null_value() -> Result<ComPtr<JsonValue>> { unsafe {
        <Self as RtActivatable<IJsonValueStatics2>>::get_activation_factory().create_null_value()
    }}
}
DEFINE_CLSID!(JsonValue(&[87,105,110,100,111,119,115,46,68,97,116,97,46,74,115,111,110,46,74,115,111,110,86,97,108,117,101,0]) [CLSID_JsonValue]);
DEFINE_IID!(IID_IJsonValueStatics, 1600869450, 12115, 18657, 145, 163, 247, 139, 80, 166, 52, 92);
RT_INTERFACE!{static interface IJsonValueStatics(IJsonValueStaticsVtbl): IInspectable(IInspectableVtbl) [IID_IJsonValueStatics] {
    fn Parse(&self, input: HSTRING, out: *mut *mut JsonValue) -> HRESULT,
    fn TryParse(&self, input: HSTRING, result: *mut *mut JsonValue, out: *mut bool) -> HRESULT,
    fn CreateBooleanValue(&self, input: bool, out: *mut *mut JsonValue) -> HRESULT,
    fn CreateNumberValue(&self, input: f64, out: *mut *mut JsonValue) -> HRESULT,
    fn CreateStringValue(&self, input: HSTRING, out: *mut *mut JsonValue) -> HRESULT
}}
impl IJsonValueStatics {
    #[inline] pub unsafe fn parse(&self, input: &HStringArg) -> Result<ComPtr<JsonValue>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).Parse)(self as *const _ as *mut _, input.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn try_parse(&self, input: &HStringArg) -> Result<(ComPtr<JsonValue>, bool)> {
        let mut result = null_mut(); let mut out = zeroed();
        let hr = ((*self.lpVtbl).TryParse)(self as *const _ as *mut _, input.get(), &mut result, &mut out);
        if hr == S_OK { Ok((ComPtr::wrap(result), out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn create_boolean_value(&self, input: bool) -> Result<ComPtr<JsonValue>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).CreateBooleanValue)(self as *const _ as *mut _, input, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn create_number_value(&self, input: f64) -> Result<ComPtr<JsonValue>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).CreateNumberValue)(self as *const _ as *mut _, input, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn create_string_value(&self, input: &HStringArg) -> Result<ComPtr<JsonValue>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).CreateStringValue)(self as *const _ as *mut _, input.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
DEFINE_IID!(IID_IJsonValueStatics2, 496946148, 16360, 17205, 131, 146, 147, 216, 227, 104, 101, 240);
RT_INTERFACE!{static interface IJsonValueStatics2(IJsonValueStatics2Vtbl): IInspectable(IInspectableVtbl) [IID_IJsonValueStatics2] {
    fn CreateNullValue(&self, out: *mut *mut JsonValue) -> HRESULT
}}
impl IJsonValueStatics2 {
    #[inline] pub unsafe fn create_null_value(&self) -> Result<ComPtr<JsonValue>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).CreateNullValue)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
RT_ENUM! { enum JsonValueType: i32 {
    Null (JsonValueType_Null) = 0, Boolean (JsonValueType_Boolean) = 1, Number (JsonValueType_Number) = 2, String (JsonValueType_String) = 3, Array (JsonValueType_Array) = 4, Object (JsonValueType_Object) = 5,
}}
} // Windows.Data.Json
pub mod xml { // Windows.Data.Xml
pub mod dom { // Windows.Data.Xml.Dom
use ::prelude::*;
DEFINE_IID!(IID_IDtdEntity, 1779130364, 25524, 18447, 158, 106, 138, 146, 129, 106, 173, 228);
RT_INTERFACE!{interface IDtdEntity(IDtdEntityVtbl): IInspectable(IInspectableVtbl) [IID_IDtdEntity] {
    fn get_PublicId(&self, out: *mut *mut IInspectable) -> HRESULT,
    fn get_SystemId(&self, out: *mut *mut IInspectable) -> HRESULT,
    fn get_NotationName(&self, out: *mut *mut IInspectable) -> HRESULT
}}
impl IDtdEntity {
    #[inline] pub unsafe fn get_public_id(&self) -> Result<ComPtr<IInspectable>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_PublicId)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_system_id(&self) -> Result<ComPtr<IInspectable>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_SystemId)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_notation_name(&self) -> Result<ComPtr<IInspectable>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_NotationName)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
RT_CLASS!{class DtdEntity: IDtdEntity}
DEFINE_IID!(IID_IDtdNotation, 2360664141, 27974, 20187, 171, 115, 223, 131, 197, 26, 211, 151);
RT_INTERFACE!{interface IDtdNotation(IDtdNotationVtbl): IInspectable(IInspectableVtbl) [IID_IDtdNotation] {
    fn get_PublicId(&self, out: *mut *mut IInspectable) -> HRESULT,
    fn get_SystemId(&self, out: *mut *mut IInspectable) -> HRESULT
}}
impl IDtdNotation {
    #[inline] pub unsafe fn get_public_id(&self) -> Result<ComPtr<IInspectable>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_PublicId)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_system_id(&self) -> Result<ComPtr<IInspectable>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_SystemId)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
RT_CLASS!{class DtdNotation: IDtdNotation}
RT_ENUM! { enum NodeType: i32 {
    Invalid (NodeType_Invalid) = 0, ElementNode (NodeType_ElementNode) = 1, AttributeNode (NodeType_AttributeNode) = 2, TextNode (NodeType_TextNode) = 3, DataSectionNode (NodeType_DataSectionNode) = 4, EntityReferenceNode (NodeType_EntityReferenceNode) = 5, EntityNode (NodeType_EntityNode) = 6, ProcessingInstructionNode (NodeType_ProcessingInstructionNode) = 7, CommentNode (NodeType_CommentNode) = 8, DocumentNode (NodeType_DocumentNode) = 9, DocumentTypeNode (NodeType_DocumentTypeNode) = 10, DocumentFragmentNode (NodeType_DocumentFragmentNode) = 11, NotationNode (NodeType_NotationNode) = 12,
}}
DEFINE_IID!(IID_IXmlAttribute, 2887010980, 46321, 19894, 178, 6, 138, 34, 195, 8, 219, 10);
RT_INTERFACE!{interface IXmlAttribute(IXmlAttributeVtbl): IInspectable(IInspectableVtbl) [IID_IXmlAttribute] {
    fn get_Name(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Specified(&self, out: *mut bool) -> HRESULT,
    fn get_Value(&self, out: *mut HSTRING) -> HRESULT,
    fn put_Value(&self, value: HSTRING) -> HRESULT
}}
impl IXmlAttribute {
    #[inline] pub unsafe fn get_name(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Name)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_specified(&self) -> Result<bool> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_Specified)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
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
RT_CLASS!{class XmlAttribute: IXmlAttribute}
DEFINE_IID!(IID_IXmlCDataSection, 1292153967, 51389, 17844, 136, 153, 4, 0, 215, 194, 198, 15);
RT_INTERFACE!{interface IXmlCDataSection(IXmlCDataSectionVtbl): IInspectable(IInspectableVtbl) [IID_IXmlCDataSection] {
    
}}
RT_CLASS!{class XmlCDataSection: IXmlCDataSection}
DEFINE_IID!(IID_IXmlCharacterData, 321798827, 20022, 19958, 177, 200, 12, 230, 47, 216, 139, 38);
RT_INTERFACE!{interface IXmlCharacterData(IXmlCharacterDataVtbl): IInspectable(IInspectableVtbl) [IID_IXmlCharacterData] {
    fn get_Data(&self, out: *mut HSTRING) -> HRESULT,
    fn put_Data(&self, value: HSTRING) -> HRESULT,
    fn get_Length(&self, out: *mut u32) -> HRESULT,
    fn SubstringData(&self, offset: u32, count: u32, out: *mut HSTRING) -> HRESULT,
    fn AppendData(&self, data: HSTRING) -> HRESULT,
    fn InsertData(&self, offset: u32, data: HSTRING) -> HRESULT,
    fn DeleteData(&self, offset: u32, count: u32) -> HRESULT,
    fn ReplaceData(&self, offset: u32, count: u32, data: HSTRING) -> HRESULT
}}
impl IXmlCharacterData {
    #[inline] pub unsafe fn get_data(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Data)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_data(&self, value: &HStringArg) -> Result<()> {
        let hr = ((*self.lpVtbl).put_Data)(self as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_length(&self) -> Result<u32> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_Length)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn substring_data(&self, offset: u32, count: u32) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).SubstringData)(self as *const _ as *mut _, offset, count, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn append_data(&self, data: &HStringArg) -> Result<()> {
        let hr = ((*self.lpVtbl).AppendData)(self as *const _ as *mut _, data.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn insert_data(&self, offset: u32, data: &HStringArg) -> Result<()> {
        let hr = ((*self.lpVtbl).InsertData)(self as *const _ as *mut _, offset, data.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn delete_data(&self, offset: u32, count: u32) -> Result<()> {
        let hr = ((*self.lpVtbl).DeleteData)(self as *const _ as *mut _, offset, count);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn replace_data(&self, offset: u32, count: u32, data: &HStringArg) -> Result<()> {
        let hr = ((*self.lpVtbl).ReplaceData)(self as *const _ as *mut _, offset, count, data.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
}
DEFINE_IID!(IID_IXmlComment, 3164894421, 46623, 17937, 156, 172, 46, 146, 227, 71, 109, 71);
RT_INTERFACE!{interface IXmlComment(IXmlCommentVtbl): IInspectable(IInspectableVtbl) [IID_IXmlComment] {
    
}}
RT_CLASS!{class XmlComment: IXmlComment}
DEFINE_IID!(IID_IXmlDocument, 4159939846, 7815, 17110, 188, 251, 184, 200, 9, 250, 84, 148);
RT_INTERFACE!{interface IXmlDocument(IXmlDocumentVtbl): IInspectable(IInspectableVtbl) [IID_IXmlDocument] {
    fn get_Doctype(&self, out: *mut *mut XmlDocumentType) -> HRESULT,
    fn get_Implementation(&self, out: *mut *mut XmlDomImplementation) -> HRESULT,
    fn get_DocumentElement(&self, out: *mut *mut XmlElement) -> HRESULT,
    fn CreateElement(&self, tagName: HSTRING, out: *mut *mut XmlElement) -> HRESULT,
    fn CreateDocumentFragment(&self, out: *mut *mut XmlDocumentFragment) -> HRESULT,
    fn CreateTextNode(&self, data: HSTRING, out: *mut *mut XmlText) -> HRESULT,
    fn CreateComment(&self, data: HSTRING, out: *mut *mut XmlComment) -> HRESULT,
    fn CreateProcessingInstruction(&self, target: HSTRING, data: HSTRING, out: *mut *mut XmlProcessingInstruction) -> HRESULT,
    fn CreateAttribute(&self, name: HSTRING, out: *mut *mut XmlAttribute) -> HRESULT,
    fn CreateEntityReference(&self, name: HSTRING, out: *mut *mut XmlEntityReference) -> HRESULT,
    fn GetElementsByTagName(&self, tagName: HSTRING, out: *mut *mut XmlNodeList) -> HRESULT,
    fn CreateCDataSection(&self, data: HSTRING, out: *mut *mut XmlCDataSection) -> HRESULT,
    fn get_DocumentUri(&self, out: *mut HSTRING) -> HRESULT,
    fn CreateAttributeNS(&self, namespaceUri: *mut IInspectable, qualifiedName: HSTRING, out: *mut *mut XmlAttribute) -> HRESULT,
    fn CreateElementNS(&self, namespaceUri: *mut IInspectable, qualifiedName: HSTRING, out: *mut *mut XmlElement) -> HRESULT,
    fn GetElementById(&self, elementId: HSTRING, out: *mut *mut XmlElement) -> HRESULT,
    fn ImportNode(&self, node: *mut IXmlNode, deep: bool, out: *mut *mut IXmlNode) -> HRESULT
}}
impl IXmlDocument {
    #[inline] pub unsafe fn get_doctype(&self) -> Result<ComPtr<XmlDocumentType>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Doctype)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_implementation(&self) -> Result<ComPtr<XmlDomImplementation>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Implementation)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_document_element(&self) -> Result<ComPtr<XmlElement>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_DocumentElement)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn create_element(&self, tagName: &HStringArg) -> Result<ComPtr<XmlElement>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).CreateElement)(self as *const _ as *mut _, tagName.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn create_document_fragment(&self) -> Result<ComPtr<XmlDocumentFragment>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).CreateDocumentFragment)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn create_text_node(&self, data: &HStringArg) -> Result<ComPtr<XmlText>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).CreateTextNode)(self as *const _ as *mut _, data.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn create_comment(&self, data: &HStringArg) -> Result<ComPtr<XmlComment>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).CreateComment)(self as *const _ as *mut _, data.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn create_processing_instruction(&self, target: &HStringArg, data: &HStringArg) -> Result<ComPtr<XmlProcessingInstruction>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).CreateProcessingInstruction)(self as *const _ as *mut _, target.get(), data.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn create_attribute(&self, name: &HStringArg) -> Result<ComPtr<XmlAttribute>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).CreateAttribute)(self as *const _ as *mut _, name.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn create_entity_reference(&self, name: &HStringArg) -> Result<ComPtr<XmlEntityReference>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).CreateEntityReference)(self as *const _ as *mut _, name.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_elements_by_tag_name(&self, tagName: &HStringArg) -> Result<ComPtr<XmlNodeList>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetElementsByTagName)(self as *const _ as *mut _, tagName.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn create_cdata_section(&self, data: &HStringArg) -> Result<ComPtr<XmlCDataSection>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).CreateCDataSection)(self as *const _ as *mut _, data.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_document_uri(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_DocumentUri)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn create_attribute_ns(&self, namespaceUri: &IInspectable, qualifiedName: &HStringArg) -> Result<ComPtr<XmlAttribute>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).CreateAttributeNS)(self as *const _ as *mut _, namespaceUri as *const _ as *mut _, qualifiedName.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn create_element_ns(&self, namespaceUri: &IInspectable, qualifiedName: &HStringArg) -> Result<ComPtr<XmlElement>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).CreateElementNS)(self as *const _ as *mut _, namespaceUri as *const _ as *mut _, qualifiedName.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_element_by_id(&self, elementId: &HStringArg) -> Result<ComPtr<XmlElement>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetElementById)(self as *const _ as *mut _, elementId.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn import_node(&self, node: &IXmlNode, deep: bool) -> Result<ComPtr<IXmlNode>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).ImportNode)(self as *const _ as *mut _, node as *const _ as *mut _, deep, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
RT_CLASS!{class XmlDocument: IXmlDocument}
impl RtActivatable<IXmlDocumentStatics> for XmlDocument {}
impl RtActivatable<IActivationFactory> for XmlDocument {}
impl XmlDocument {
    #[inline] pub fn load_from_uri_async(uri: &::rt::gen::windows::foundation::Uri) -> Result<ComPtr<::rt::gen::windows::foundation::IAsyncOperation<XmlDocument>>> { unsafe {
        <Self as RtActivatable<IXmlDocumentStatics>>::get_activation_factory().load_from_uri_async(uri)
    }}
    #[inline] pub fn load_from_uri_with_settings_async(uri: &::rt::gen::windows::foundation::Uri, loadSettings: &XmlLoadSettings) -> Result<ComPtr<::rt::gen::windows::foundation::IAsyncOperation<XmlDocument>>> { unsafe {
        <Self as RtActivatable<IXmlDocumentStatics>>::get_activation_factory().load_from_uri_with_settings_async(uri, loadSettings)
    }}
    #[cfg(feature="windows-storage")] #[inline] pub fn load_from_file_async(file: &::rt::gen::windows::storage::IStorageFile) -> Result<ComPtr<::rt::gen::windows::foundation::IAsyncOperation<XmlDocument>>> { unsafe {
        <Self as RtActivatable<IXmlDocumentStatics>>::get_activation_factory().load_from_file_async(file)
    }}
    #[cfg(feature="windows-storage")] #[inline] pub fn load_from_file_with_settings_async(file: &::rt::gen::windows::storage::IStorageFile, loadSettings: &XmlLoadSettings) -> Result<ComPtr<::rt::gen::windows::foundation::IAsyncOperation<XmlDocument>>> { unsafe {
        <Self as RtActivatable<IXmlDocumentStatics>>::get_activation_factory().load_from_file_with_settings_async(file, loadSettings)
    }}
}
DEFINE_CLSID!(XmlDocument(&[87,105,110,100,111,119,115,46,68,97,116,97,46,88,109,108,46,68,111,109,46,88,109,108,68,111,99,117,109,101,110,116,0]) [CLSID_XmlDocument]);
DEFINE_IID!(IID_IXmlDocumentFragment, 3807013526, 3105, 17573, 139, 201, 158, 74, 38, 39, 8, 236);
RT_INTERFACE!{interface IXmlDocumentFragment(IXmlDocumentFragmentVtbl): IInspectable(IInspectableVtbl) [IID_IXmlDocumentFragment] {
    
}}
RT_CLASS!{class XmlDocumentFragment: IXmlDocumentFragment}
DEFINE_IID!(IID_IXmlDocumentIO, 1825630030, 61029, 17545, 158, 191, 202, 67, 232, 123, 166, 55);
RT_INTERFACE!{interface IXmlDocumentIO(IXmlDocumentIOVtbl): IInspectable(IInspectableVtbl) [IID_IXmlDocumentIO] {
    fn LoadXml(&self, xml: HSTRING) -> HRESULT,
    fn LoadXmlWithSettings(&self, xml: HSTRING, loadSettings: *mut XmlLoadSettings) -> HRESULT,
    #[cfg(feature="windows-storage")] fn SaveToFileAsync(&self, file: *mut ::rt::gen::windows::storage::IStorageFile, out: *mut *mut ::rt::gen::windows::foundation::IAsyncAction) -> HRESULT
}}
impl IXmlDocumentIO {
    #[inline] pub unsafe fn load_xml(&self, xml: &HStringArg) -> Result<()> {
        let hr = ((*self.lpVtbl).LoadXml)(self as *const _ as *mut _, xml.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn load_xml_with_settings(&self, xml: &HStringArg, loadSettings: &XmlLoadSettings) -> Result<()> {
        let hr = ((*self.lpVtbl).LoadXmlWithSettings)(self as *const _ as *mut _, xml.get(), loadSettings as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[cfg(feature="windows-storage")] #[inline] pub unsafe fn save_to_file_async(&self, file: &::rt::gen::windows::storage::IStorageFile) -> Result<ComPtr<::rt::gen::windows::foundation::IAsyncAction>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).SaveToFileAsync)(self as *const _ as *mut _, file as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
DEFINE_IID!(IID_IXmlDocumentIO2, 1560495713, 31704, 19157, 158, 191, 129, 230, 52, 114, 99, 177);
RT_INTERFACE!{interface IXmlDocumentIO2(IXmlDocumentIO2Vtbl): IInspectable(IInspectableVtbl) [IID_IXmlDocumentIO2] {
    #[cfg(feature="windows-storage")] fn LoadXmlFromBuffer(&self, buffer: *mut ::rt::gen::windows::storage::streams::IBuffer) -> HRESULT,
    #[cfg(feature="windows-storage")] fn LoadXmlFromBufferWithSettings(&self, buffer: *mut ::rt::gen::windows::storage::streams::IBuffer, loadSettings: *mut XmlLoadSettings) -> HRESULT
}}
impl IXmlDocumentIO2 {
    #[cfg(feature="windows-storage")] #[inline] pub unsafe fn load_xml_from_buffer(&self, buffer: &::rt::gen::windows::storage::streams::IBuffer) -> Result<()> {
        let hr = ((*self.lpVtbl).LoadXmlFromBuffer)(self as *const _ as *mut _, buffer as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[cfg(feature="windows-storage")] #[inline] pub unsafe fn load_xml_from_buffer_with_settings(&self, buffer: &::rt::gen::windows::storage::streams::IBuffer, loadSettings: &XmlLoadSettings) -> Result<()> {
        let hr = ((*self.lpVtbl).LoadXmlFromBufferWithSettings)(self as *const _ as *mut _, buffer as *const _ as *mut _, loadSettings as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
}
DEFINE_IID!(IID_IXmlDocumentStatics, 1430508116, 55127, 19321, 149, 57, 35, 43, 24, 245, 11, 241);
RT_INTERFACE!{static interface IXmlDocumentStatics(IXmlDocumentStaticsVtbl): IInspectable(IInspectableVtbl) [IID_IXmlDocumentStatics] {
    fn LoadFromUriAsync(&self, uri: *mut ::rt::gen::windows::foundation::Uri, out: *mut *mut ::rt::gen::windows::foundation::IAsyncOperation<XmlDocument>) -> HRESULT,
    fn LoadFromUriWithSettingsAsync(&self, uri: *mut ::rt::gen::windows::foundation::Uri, loadSettings: *mut XmlLoadSettings, out: *mut *mut ::rt::gen::windows::foundation::IAsyncOperation<XmlDocument>) -> HRESULT,
    #[cfg(feature="windows-storage")] fn LoadFromFileAsync(&self, file: *mut ::rt::gen::windows::storage::IStorageFile, out: *mut *mut ::rt::gen::windows::foundation::IAsyncOperation<XmlDocument>) -> HRESULT,
    #[cfg(feature="windows-storage")] fn LoadFromFileWithSettingsAsync(&self, file: *mut ::rt::gen::windows::storage::IStorageFile, loadSettings: *mut XmlLoadSettings, out: *mut *mut ::rt::gen::windows::foundation::IAsyncOperation<XmlDocument>) -> HRESULT
}}
impl IXmlDocumentStatics {
    #[inline] pub unsafe fn load_from_uri_async(&self, uri: &::rt::gen::windows::foundation::Uri) -> Result<ComPtr<::rt::gen::windows::foundation::IAsyncOperation<XmlDocument>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).LoadFromUriAsync)(self as *const _ as *mut _, uri as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn load_from_uri_with_settings_async(&self, uri: &::rt::gen::windows::foundation::Uri, loadSettings: &XmlLoadSettings) -> Result<ComPtr<::rt::gen::windows::foundation::IAsyncOperation<XmlDocument>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).LoadFromUriWithSettingsAsync)(self as *const _ as *mut _, uri as *const _ as *mut _, loadSettings as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[cfg(feature="windows-storage")] #[inline] pub unsafe fn load_from_file_async(&self, file: &::rt::gen::windows::storage::IStorageFile) -> Result<ComPtr<::rt::gen::windows::foundation::IAsyncOperation<XmlDocument>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).LoadFromFileAsync)(self as *const _ as *mut _, file as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[cfg(feature="windows-storage")] #[inline] pub unsafe fn load_from_file_with_settings_async(&self, file: &::rt::gen::windows::storage::IStorageFile, loadSettings: &XmlLoadSettings) -> Result<ComPtr<::rt::gen::windows::foundation::IAsyncOperation<XmlDocument>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).LoadFromFileWithSettingsAsync)(self as *const _ as *mut _, file as *const _ as *mut _, loadSettings as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
DEFINE_IID!(IID_IXmlDocumentType, 4147389477, 38785, 18788, 142, 148, 155, 28, 109, 252, 155, 199);
RT_INTERFACE!{interface IXmlDocumentType(IXmlDocumentTypeVtbl): IInspectable(IInspectableVtbl) [IID_IXmlDocumentType] {
    fn get_Name(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Entities(&self, out: *mut *mut XmlNamedNodeMap) -> HRESULT,
    fn get_Notations(&self, out: *mut *mut XmlNamedNodeMap) -> HRESULT
}}
impl IXmlDocumentType {
    #[inline] pub unsafe fn get_name(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Name)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_entities(&self) -> Result<ComPtr<XmlNamedNodeMap>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Entities)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_notations(&self) -> Result<ComPtr<XmlNamedNodeMap>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Notations)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
RT_CLASS!{class XmlDocumentType: IXmlDocumentType}
DEFINE_IID!(IID_IXmlDomImplementation, 1843757362, 61725, 20411, 140, 198, 88, 60, 186, 147, 17, 47);
RT_INTERFACE!{interface IXmlDomImplementation(IXmlDomImplementationVtbl): IInspectable(IInspectableVtbl) [IID_IXmlDomImplementation] {
    fn HasFeature(&self, feature: HSTRING, version: *mut IInspectable, out: *mut bool) -> HRESULT
}}
impl IXmlDomImplementation {
    #[inline] pub unsafe fn has_feature(&self, feature: &HStringArg, version: &IInspectable) -> Result<bool> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).HasFeature)(self as *const _ as *mut _, feature.get(), version as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
}
RT_CLASS!{class XmlDomImplementation: IXmlDomImplementation}
DEFINE_IID!(IID_IXmlElement, 771459615, 27408, 20216, 159, 131, 239, 204, 232, 250, 236, 55);
RT_INTERFACE!{interface IXmlElement(IXmlElementVtbl): IInspectable(IInspectableVtbl) [IID_IXmlElement] {
    fn get_TagName(&self, out: *mut HSTRING) -> HRESULT,
    fn GetAttribute(&self, attributeName: HSTRING, out: *mut HSTRING) -> HRESULT,
    fn SetAttribute(&self, attributeName: HSTRING, attributeValue: HSTRING) -> HRESULT,
    fn RemoveAttribute(&self, attributeName: HSTRING) -> HRESULT,
    fn GetAttributeNode(&self, attributeName: HSTRING, out: *mut *mut XmlAttribute) -> HRESULT,
    fn SetAttributeNode(&self, newAttribute: *mut XmlAttribute, out: *mut *mut XmlAttribute) -> HRESULT,
    fn RemoveAttributeNode(&self, attributeNode: *mut XmlAttribute, out: *mut *mut XmlAttribute) -> HRESULT,
    fn GetElementsByTagName(&self, tagName: HSTRING, out: *mut *mut XmlNodeList) -> HRESULT,
    fn SetAttributeNS(&self, namespaceUri: *mut IInspectable, qualifiedName: HSTRING, value: HSTRING) -> HRESULT,
    fn GetAttributeNS(&self, namespaceUri: *mut IInspectable, localName: HSTRING, out: *mut HSTRING) -> HRESULT,
    fn RemoveAttributeNS(&self, namespaceUri: *mut IInspectable, localName: HSTRING) -> HRESULT,
    fn SetAttributeNodeNS(&self, newAttribute: *mut XmlAttribute, out: *mut *mut XmlAttribute) -> HRESULT,
    fn GetAttributeNodeNS(&self, namespaceUri: *mut IInspectable, localName: HSTRING, out: *mut *mut XmlAttribute) -> HRESULT
}}
impl IXmlElement {
    #[inline] pub unsafe fn get_tag_name(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_TagName)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_attribute(&self, attributeName: &HStringArg) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetAttribute)(self as *const _ as *mut _, attributeName.get(), &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_attribute(&self, attributeName: &HStringArg, attributeValue: &HStringArg) -> Result<()> {
        let hr = ((*self.lpVtbl).SetAttribute)(self as *const _ as *mut _, attributeName.get(), attributeValue.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn remove_attribute(&self, attributeName: &HStringArg) -> Result<()> {
        let hr = ((*self.lpVtbl).RemoveAttribute)(self as *const _ as *mut _, attributeName.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_attribute_node(&self, attributeName: &HStringArg) -> Result<ComPtr<XmlAttribute>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetAttributeNode)(self as *const _ as *mut _, attributeName.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_attribute_node(&self, newAttribute: &XmlAttribute) -> Result<ComPtr<XmlAttribute>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).SetAttributeNode)(self as *const _ as *mut _, newAttribute as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn remove_attribute_node(&self, attributeNode: &XmlAttribute) -> Result<ComPtr<XmlAttribute>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).RemoveAttributeNode)(self as *const _ as *mut _, attributeNode as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_elements_by_tag_name(&self, tagName: &HStringArg) -> Result<ComPtr<XmlNodeList>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetElementsByTagName)(self as *const _ as *mut _, tagName.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_attribute_ns(&self, namespaceUri: &IInspectable, qualifiedName: &HStringArg, value: &HStringArg) -> Result<()> {
        let hr = ((*self.lpVtbl).SetAttributeNS)(self as *const _ as *mut _, namespaceUri as *const _ as *mut _, qualifiedName.get(), value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_attribute_ns(&self, namespaceUri: &IInspectable, localName: &HStringArg) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetAttributeNS)(self as *const _ as *mut _, namespaceUri as *const _ as *mut _, localName.get(), &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn remove_attribute_ns(&self, namespaceUri: &IInspectable, localName: &HStringArg) -> Result<()> {
        let hr = ((*self.lpVtbl).RemoveAttributeNS)(self as *const _ as *mut _, namespaceUri as *const _ as *mut _, localName.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_attribute_node_ns(&self, newAttribute: &XmlAttribute) -> Result<ComPtr<XmlAttribute>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).SetAttributeNodeNS)(self as *const _ as *mut _, newAttribute as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_attribute_node_ns(&self, namespaceUri: &IInspectable, localName: &HStringArg) -> Result<ComPtr<XmlAttribute>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetAttributeNodeNS)(self as *const _ as *mut _, namespaceUri as *const _ as *mut _, localName.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
RT_CLASS!{class XmlElement: IXmlElement}
DEFINE_IID!(IID_IXmlEntityReference, 774850492, 50128, 19663, 187, 134, 10, 184, 195, 106, 97, 207);
RT_INTERFACE!{interface IXmlEntityReference(IXmlEntityReferenceVtbl): IInspectable(IInspectableVtbl) [IID_IXmlEntityReference] {
    
}}
RT_CLASS!{class XmlEntityReference: IXmlEntityReference}
DEFINE_IID!(IID_IXmlLoadSettings, 1487538088, 65238, 18167, 180, 197, 251, 27, 167, 33, 8, 214);
RT_INTERFACE!{interface IXmlLoadSettings(IXmlLoadSettingsVtbl): IInspectable(IInspectableVtbl) [IID_IXmlLoadSettings] {
    fn get_MaxElementDepth(&self, out: *mut u32) -> HRESULT,
    fn put_MaxElementDepth(&self, value: u32) -> HRESULT,
    fn get_ProhibitDtd(&self, out: *mut bool) -> HRESULT,
    fn put_ProhibitDtd(&self, value: bool) -> HRESULT,
    fn get_ResolveExternals(&self, out: *mut bool) -> HRESULT,
    fn put_ResolveExternals(&self, value: bool) -> HRESULT,
    fn get_ValidateOnParse(&self, out: *mut bool) -> HRESULT,
    fn put_ValidateOnParse(&self, value: bool) -> HRESULT,
    fn get_ElementContentWhiteSpace(&self, out: *mut bool) -> HRESULT,
    fn put_ElementContentWhiteSpace(&self, value: bool) -> HRESULT
}}
impl IXmlLoadSettings {
    #[inline] pub unsafe fn get_max_element_depth(&self) -> Result<u32> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_MaxElementDepth)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_max_element_depth(&self, value: u32) -> Result<()> {
        let hr = ((*self.lpVtbl).put_MaxElementDepth)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_prohibit_dtd(&self) -> Result<bool> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_ProhibitDtd)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_prohibit_dtd(&self, value: bool) -> Result<()> {
        let hr = ((*self.lpVtbl).put_ProhibitDtd)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_resolve_externals(&self) -> Result<bool> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_ResolveExternals)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_resolve_externals(&self, value: bool) -> Result<()> {
        let hr = ((*self.lpVtbl).put_ResolveExternals)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_validate_on_parse(&self) -> Result<bool> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_ValidateOnParse)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_validate_on_parse(&self, value: bool) -> Result<()> {
        let hr = ((*self.lpVtbl).put_ValidateOnParse)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_element_content_white_space(&self) -> Result<bool> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_ElementContentWhiteSpace)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_element_content_white_space(&self, value: bool) -> Result<()> {
        let hr = ((*self.lpVtbl).put_ElementContentWhiteSpace)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
}
RT_CLASS!{class XmlLoadSettings: IXmlLoadSettings}
impl RtActivatable<IActivationFactory> for XmlLoadSettings {}
DEFINE_CLSID!(XmlLoadSettings(&[87,105,110,100,111,119,115,46,68,97,116,97,46,88,109,108,46,68,111,109,46,88,109,108,76,111,97,100,83,101,116,116,105,110,103,115,0]) [CLSID_XmlLoadSettings]);
DEFINE_IID!(IID_IXmlNamedNodeMap, 3014041264, 43696, 19330, 166, 250, 177, 69, 63, 124, 2, 27);
RT_INTERFACE!{interface IXmlNamedNodeMap(IXmlNamedNodeMapVtbl): IInspectable(IInspectableVtbl) [IID_IXmlNamedNodeMap] {
    fn get_Length(&self, out: *mut u32) -> HRESULT,
    fn Item(&self, index: u32, out: *mut *mut IXmlNode) -> HRESULT,
    fn GetNamedItem(&self, name: HSTRING, out: *mut *mut IXmlNode) -> HRESULT,
    fn SetNamedItem(&self, node: *mut IXmlNode, out: *mut *mut IXmlNode) -> HRESULT,
    fn RemoveNamedItem(&self, name: HSTRING, out: *mut *mut IXmlNode) -> HRESULT,
    fn GetNamedItemNS(&self, namespaceUri: *mut IInspectable, name: HSTRING, out: *mut *mut IXmlNode) -> HRESULT,
    fn RemoveNamedItemNS(&self, namespaceUri: *mut IInspectable, name: HSTRING, out: *mut *mut IXmlNode) -> HRESULT,
    fn SetNamedItemNS(&self, node: *mut IXmlNode, out: *mut *mut IXmlNode) -> HRESULT
}}
impl IXmlNamedNodeMap {
    #[inline] pub unsafe fn get_length(&self) -> Result<u32> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_Length)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn item(&self, index: u32) -> Result<ComPtr<IXmlNode>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).Item)(self as *const _ as *mut _, index, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_named_item(&self, name: &HStringArg) -> Result<ComPtr<IXmlNode>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetNamedItem)(self as *const _ as *mut _, name.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_named_item(&self, node: &IXmlNode) -> Result<ComPtr<IXmlNode>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).SetNamedItem)(self as *const _ as *mut _, node as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn remove_named_item(&self, name: &HStringArg) -> Result<ComPtr<IXmlNode>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).RemoveNamedItem)(self as *const _ as *mut _, name.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_named_item_ns(&self, namespaceUri: &IInspectable, name: &HStringArg) -> Result<ComPtr<IXmlNode>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetNamedItemNS)(self as *const _ as *mut _, namespaceUri as *const _ as *mut _, name.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn remove_named_item_ns(&self, namespaceUri: &IInspectable, name: &HStringArg) -> Result<ComPtr<IXmlNode>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).RemoveNamedItemNS)(self as *const _ as *mut _, namespaceUri as *const _ as *mut _, name.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_named_item_ns(&self, node: &IXmlNode) -> Result<ComPtr<IXmlNode>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).SetNamedItemNS)(self as *const _ as *mut _, node as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
RT_CLASS!{class XmlNamedNodeMap: IXmlNamedNodeMap}
DEFINE_IID!(IID_IXmlNode, 477371737, 8482, 18389, 168, 86, 131, 243, 212, 33, 72, 117);
RT_INTERFACE!{interface IXmlNode(IXmlNodeVtbl): IInspectable(IInspectableVtbl) [IID_IXmlNode] {
    fn get_NodeValue(&self, out: *mut *mut IInspectable) -> HRESULT,
    fn put_NodeValue(&self, value: *mut IInspectable) -> HRESULT,
    fn get_NodeType(&self, out: *mut NodeType) -> HRESULT,
    fn get_NodeName(&self, out: *mut HSTRING) -> HRESULT,
    fn get_ParentNode(&self, out: *mut *mut IXmlNode) -> HRESULT,
    fn get_ChildNodes(&self, out: *mut *mut XmlNodeList) -> HRESULT,
    fn get_FirstChild(&self, out: *mut *mut IXmlNode) -> HRESULT,
    fn get_LastChild(&self, out: *mut *mut IXmlNode) -> HRESULT,
    fn get_PreviousSibling(&self, out: *mut *mut IXmlNode) -> HRESULT,
    fn get_NextSibling(&self, out: *mut *mut IXmlNode) -> HRESULT,
    fn get_Attributes(&self, out: *mut *mut XmlNamedNodeMap) -> HRESULT,
    fn HasChildNodes(&self, out: *mut bool) -> HRESULT,
    fn get_OwnerDocument(&self, out: *mut *mut XmlDocument) -> HRESULT,
    fn InsertBefore(&self, newChild: *mut IXmlNode, referenceChild: *mut IXmlNode, out: *mut *mut IXmlNode) -> HRESULT,
    fn ReplaceChild(&self, newChild: *mut IXmlNode, referenceChild: *mut IXmlNode, out: *mut *mut IXmlNode) -> HRESULT,
    fn RemoveChild(&self, childNode: *mut IXmlNode, out: *mut *mut IXmlNode) -> HRESULT,
    fn AppendChild(&self, newChild: *mut IXmlNode, out: *mut *mut IXmlNode) -> HRESULT,
    fn CloneNode(&self, deep: bool, out: *mut *mut IXmlNode) -> HRESULT,
    fn get_NamespaceUri(&self, out: *mut *mut IInspectable) -> HRESULT,
    fn get_LocalName(&self, out: *mut *mut IInspectable) -> HRESULT,
    fn get_Prefix(&self, out: *mut *mut IInspectable) -> HRESULT,
    fn Normalize(&self) -> HRESULT,
    fn put_Prefix(&self, value: *mut IInspectable) -> HRESULT
}}
impl IXmlNode {
    #[inline] pub unsafe fn get_node_value(&self) -> Result<ComPtr<IInspectable>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_NodeValue)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_node_value(&self, value: &IInspectable) -> Result<()> {
        let hr = ((*self.lpVtbl).put_NodeValue)(self as *const _ as *mut _, value as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_node_type(&self) -> Result<NodeType> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_NodeType)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_node_name(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_NodeName)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_parent_node(&self) -> Result<ComPtr<IXmlNode>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_ParentNode)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_child_nodes(&self) -> Result<ComPtr<XmlNodeList>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_ChildNodes)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_first_child(&self) -> Result<ComPtr<IXmlNode>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_FirstChild)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_last_child(&self) -> Result<ComPtr<IXmlNode>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_LastChild)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_previous_sibling(&self) -> Result<ComPtr<IXmlNode>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_PreviousSibling)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_next_sibling(&self) -> Result<ComPtr<IXmlNode>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_NextSibling)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_attributes(&self) -> Result<ComPtr<XmlNamedNodeMap>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Attributes)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn has_child_nodes(&self) -> Result<bool> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).HasChildNodes)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_owner_document(&self) -> Result<ComPtr<XmlDocument>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_OwnerDocument)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn insert_before(&self, newChild: &IXmlNode, referenceChild: &IXmlNode) -> Result<ComPtr<IXmlNode>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).InsertBefore)(self as *const _ as *mut _, newChild as *const _ as *mut _, referenceChild as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn replace_child(&self, newChild: &IXmlNode, referenceChild: &IXmlNode) -> Result<ComPtr<IXmlNode>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).ReplaceChild)(self as *const _ as *mut _, newChild as *const _ as *mut _, referenceChild as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn remove_child(&self, childNode: &IXmlNode) -> Result<ComPtr<IXmlNode>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).RemoveChild)(self as *const _ as *mut _, childNode as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn append_child(&self, newChild: &IXmlNode) -> Result<ComPtr<IXmlNode>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).AppendChild)(self as *const _ as *mut _, newChild as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn clone_node(&self, deep: bool) -> Result<ComPtr<IXmlNode>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).CloneNode)(self as *const _ as *mut _, deep, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_namespace_uri(&self) -> Result<ComPtr<IInspectable>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_NamespaceUri)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_local_name(&self) -> Result<ComPtr<IInspectable>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_LocalName)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_prefix(&self) -> Result<ComPtr<IInspectable>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Prefix)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn normalize(&self) -> Result<()> {
        let hr = ((*self.lpVtbl).Normalize)(self as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_prefix(&self, value: &IInspectable) -> Result<()> {
        let hr = ((*self.lpVtbl).put_Prefix)(self as *const _ as *mut _, value as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
}
DEFINE_IID!(IID_IXmlNodeList, 2355146103, 33700, 20161, 156, 84, 123, 164, 41, 225, 61, 166);
RT_INTERFACE!{interface IXmlNodeList(IXmlNodeListVtbl): IInspectable(IInspectableVtbl) [IID_IXmlNodeList] {
    fn get_Length(&self, out: *mut u32) -> HRESULT,
    fn Item(&self, index: u32, out: *mut *mut IXmlNode) -> HRESULT
}}
impl IXmlNodeList {
    #[inline] pub unsafe fn get_length(&self) -> Result<u32> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_Length)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn item(&self, index: u32) -> Result<ComPtr<IXmlNode>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).Item)(self as *const _ as *mut _, index, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
RT_CLASS!{class XmlNodeList: IXmlNodeList}
DEFINE_IID!(IID_IXmlNodeSelector, 1675344523, 53467, 20449, 183, 69, 249, 67, 58, 253, 194, 91);
RT_INTERFACE!{interface IXmlNodeSelector(IXmlNodeSelectorVtbl): IInspectable(IInspectableVtbl) [IID_IXmlNodeSelector] {
    fn SelectSingleNode(&self, xpath: HSTRING, out: *mut *mut IXmlNode) -> HRESULT,
    fn SelectNodes(&self, xpath: HSTRING, out: *mut *mut XmlNodeList) -> HRESULT,
    fn SelectSingleNodeNS(&self, xpath: HSTRING, namespaces: *mut IInspectable, out: *mut *mut IXmlNode) -> HRESULT,
    fn SelectNodesNS(&self, xpath: HSTRING, namespaces: *mut IInspectable, out: *mut *mut XmlNodeList) -> HRESULT
}}
impl IXmlNodeSelector {
    #[inline] pub unsafe fn select_single_node(&self, xpath: &HStringArg) -> Result<ComPtr<IXmlNode>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).SelectSingleNode)(self as *const _ as *mut _, xpath.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn select_nodes(&self, xpath: &HStringArg) -> Result<ComPtr<XmlNodeList>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).SelectNodes)(self as *const _ as *mut _, xpath.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn select_single_node_ns(&self, xpath: &HStringArg, namespaces: &IInspectable) -> Result<ComPtr<IXmlNode>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).SelectSingleNodeNS)(self as *const _ as *mut _, xpath.get(), namespaces as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn select_nodes_ns(&self, xpath: &HStringArg, namespaces: &IInspectable) -> Result<ComPtr<XmlNodeList>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).SelectNodesNS)(self as *const _ as *mut _, xpath.get(), namespaces as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
DEFINE_IID!(IID_IXmlNodeSerializer, 1556460418, 59101, 18833, 171, 239, 6, 216, 210, 231, 189, 12);
RT_INTERFACE!{interface IXmlNodeSerializer(IXmlNodeSerializerVtbl): IInspectable(IInspectableVtbl) [IID_IXmlNodeSerializer] {
    fn GetXml(&self, out: *mut HSTRING) -> HRESULT,
    fn get_InnerText(&self, out: *mut HSTRING) -> HRESULT,
    fn put_InnerText(&self, value: HSTRING) -> HRESULT
}}
impl IXmlNodeSerializer {
    #[inline] pub unsafe fn get_xml(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetXml)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_inner_text(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_InnerText)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_inner_text(&self, value: &HStringArg) -> Result<()> {
        let hr = ((*self.lpVtbl).put_InnerText)(self as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
}
DEFINE_IID!(IID_IXmlProcessingInstruction, 654834974, 7826, 20174, 182, 244, 38, 240, 105, 7, 141, 220);
RT_INTERFACE!{interface IXmlProcessingInstruction(IXmlProcessingInstructionVtbl): IInspectable(IInspectableVtbl) [IID_IXmlProcessingInstruction] {
    fn get_Target(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Data(&self, out: *mut HSTRING) -> HRESULT,
    fn put_Data(&self, value: HSTRING) -> HRESULT
}}
impl IXmlProcessingInstruction {
    #[inline] pub unsafe fn get_target(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Target)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_data(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Data)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_data(&self, value: &HStringArg) -> Result<()> {
        let hr = ((*self.lpVtbl).put_Data)(self as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
}
RT_CLASS!{class XmlProcessingInstruction: IXmlProcessingInstruction}
DEFINE_IID!(IID_IXmlText, 4180780235, 12429, 18272, 161, 213, 67, 182, 116, 80, 172, 126);
RT_INTERFACE!{interface IXmlText(IXmlTextVtbl): IInspectable(IInspectableVtbl) [IID_IXmlText] {
    fn SplitText(&self, offset: u32, out: *mut *mut IXmlText) -> HRESULT
}}
impl IXmlText {
    #[inline] pub unsafe fn split_text(&self, offset: u32) -> Result<ComPtr<IXmlText>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).SplitText)(self as *const _ as *mut _, offset, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
RT_CLASS!{class XmlText: IXmlText}
} // Windows.Data.Xml.Dom
pub mod xsl { // Windows.Data.Xml.Xsl
use ::prelude::*;
DEFINE_IID!(IID_IXsltProcessor, 2070179903, 21772, 18630, 169, 15, 147, 165, 185, 100, 81, 143);
RT_INTERFACE!{interface IXsltProcessor(IXsltProcessorVtbl): IInspectable(IInspectableVtbl) [IID_IXsltProcessor] {
    fn TransformToString(&self, inputNode: *mut super::dom::IXmlNode, out: *mut HSTRING) -> HRESULT
}}
impl IXsltProcessor {
    #[inline] pub unsafe fn transform_to_string(&self, inputNode: &super::dom::IXmlNode) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).TransformToString)(self as *const _ as *mut _, inputNode as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
}
RT_CLASS!{class XsltProcessor: IXsltProcessor}
impl RtActivatable<IXsltProcessorFactory> for XsltProcessor {}
impl XsltProcessor {
    #[inline] pub fn create_instance(document: &super::dom::XmlDocument) -> Result<ComPtr<XsltProcessor>> { unsafe {
        <Self as RtActivatable<IXsltProcessorFactory>>::get_activation_factory().create_instance(document)
    }}
}
DEFINE_CLSID!(XsltProcessor(&[87,105,110,100,111,119,115,46,68,97,116,97,46,88,109,108,46,88,115,108,46,88,115,108,116,80,114,111,99,101,115,115,111,114,0]) [CLSID_XsltProcessor]);
DEFINE_IID!(IID_IXsltProcessor2, 2376358998, 38821, 17611, 168, 190, 39, 216, 98, 128, 199, 10);
RT_INTERFACE!{interface IXsltProcessor2(IXsltProcessor2Vtbl): IInspectable(IInspectableVtbl) [IID_IXsltProcessor2] {
    fn TransformToDocument(&self, inputNode: *mut super::dom::IXmlNode, out: *mut *mut super::dom::XmlDocument) -> HRESULT
}}
impl IXsltProcessor2 {
    #[inline] pub unsafe fn transform_to_document(&self, inputNode: &super::dom::IXmlNode) -> Result<ComPtr<super::dom::XmlDocument>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).TransformToDocument)(self as *const _ as *mut _, inputNode as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
DEFINE_IID!(IID_IXsltProcessorFactory, 658589376, 39505, 18019, 191, 48, 14, 247, 66, 20, 111, 32);
RT_INTERFACE!{static interface IXsltProcessorFactory(IXsltProcessorFactoryVtbl): IInspectable(IInspectableVtbl) [IID_IXsltProcessorFactory] {
    fn CreateInstance(&self, document: *mut super::dom::XmlDocument, out: *mut *mut XsltProcessor) -> HRESULT
}}
impl IXsltProcessorFactory {
    #[inline] pub unsafe fn create_instance(&self, document: &super::dom::XmlDocument) -> Result<ComPtr<XsltProcessor>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).CreateInstance)(self as *const _ as *mut _, document as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
} // Windows.Data.Xml.Xsl
} // Windows.Data.Xml
pub mod html { // Windows.Data.Html
use ::prelude::*;
DEFINE_IID!(IID_IHtmlUtilities, 4273998557, 9113, 20396, 181, 167, 5, 233, 172, 215, 24, 29);
RT_INTERFACE!{static interface IHtmlUtilities(IHtmlUtilitiesVtbl): IInspectable(IInspectableVtbl) [IID_IHtmlUtilities] {
    fn ConvertToText(&self, html: HSTRING, out: *mut HSTRING) -> HRESULT
}}
impl IHtmlUtilities {
    #[inline] pub unsafe fn convert_to_text(&self, html: &HStringArg) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).ConvertToText)(self as *const _ as *mut _, html.get(), &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
}
RT_CLASS!{static class HtmlUtilities}
impl RtActivatable<IHtmlUtilities> for HtmlUtilities {}
impl HtmlUtilities {
    #[inline] pub fn convert_to_text(html: &HStringArg) -> Result<HString> { unsafe {
        <Self as RtActivatable<IHtmlUtilities>>::get_activation_factory().convert_to_text(html)
    }}
}
DEFINE_CLSID!(HtmlUtilities(&[87,105,110,100,111,119,115,46,68,97,116,97,46,72,116,109,108,46,72,116,109,108,85,116,105,108,105,116,105,101,115,0]) [CLSID_HtmlUtilities]);
} // Windows.Data.Html
pub mod pdf { // Windows.Data.Pdf
use ::prelude::*;
DEFINE_IID!(IID_IPdfDocument, 2893987549, 33018, 16521, 132, 110, 129, 183, 127, 245, 168, 108);
RT_INTERFACE!{interface IPdfDocument(IPdfDocumentVtbl): IInspectable(IInspectableVtbl) [IID_IPdfDocument] {
    fn GetPage(&self, pageIndex: u32, out: *mut *mut PdfPage) -> HRESULT,
    fn get_PageCount(&self, out: *mut u32) -> HRESULT,
    fn get_IsPasswordProtected(&self, out: *mut bool) -> HRESULT
}}
impl IPdfDocument {
    #[inline] pub unsafe fn get_page(&self, pageIndex: u32) -> Result<ComPtr<PdfPage>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetPage)(self as *const _ as *mut _, pageIndex, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_page_count(&self) -> Result<u32> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_PageCount)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_is_password_protected(&self) -> Result<bool> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_IsPasswordProtected)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
}
RT_CLASS!{class PdfDocument: IPdfDocument}
impl RtActivatable<IPdfDocumentStatics> for PdfDocument {}
impl PdfDocument {
    #[cfg(feature="windows-storage")] #[inline] pub fn load_from_file_async(file: &super::super::storage::IStorageFile) -> Result<ComPtr<super::super::foundation::IAsyncOperation<PdfDocument>>> { unsafe {
        <Self as RtActivatable<IPdfDocumentStatics>>::get_activation_factory().load_from_file_async(file)
    }}
    #[cfg(feature="windows-storage")] #[inline] pub fn load_from_file_with_password_async(file: &super::super::storage::IStorageFile, password: &HStringArg) -> Result<ComPtr<super::super::foundation::IAsyncOperation<PdfDocument>>> { unsafe {
        <Self as RtActivatable<IPdfDocumentStatics>>::get_activation_factory().load_from_file_with_password_async(file, password)
    }}
    #[cfg(feature="windows-storage")] #[inline] pub fn load_from_stream_async(inputStream: &super::super::storage::streams::IRandomAccessStream) -> Result<ComPtr<super::super::foundation::IAsyncOperation<PdfDocument>>> { unsafe {
        <Self as RtActivatable<IPdfDocumentStatics>>::get_activation_factory().load_from_stream_async(inputStream)
    }}
    #[cfg(feature="windows-storage")] #[inline] pub fn load_from_stream_with_password_async(inputStream: &super::super::storage::streams::IRandomAccessStream, password: &HStringArg) -> Result<ComPtr<super::super::foundation::IAsyncOperation<PdfDocument>>> { unsafe {
        <Self as RtActivatable<IPdfDocumentStatics>>::get_activation_factory().load_from_stream_with_password_async(inputStream, password)
    }}
}
DEFINE_CLSID!(PdfDocument(&[87,105,110,100,111,119,115,46,68,97,116,97,46,80,100,102,46,80,100,102,68,111,99,117,109,101,110,116,0]) [CLSID_PdfDocument]);
DEFINE_IID!(IID_IPdfDocumentStatics, 1127877471, 49159, 18312, 144, 242, 8, 20, 61, 146, 37, 153);
RT_INTERFACE!{static interface IPdfDocumentStatics(IPdfDocumentStaticsVtbl): IInspectable(IInspectableVtbl) [IID_IPdfDocumentStatics] {
    #[cfg(feature="windows-storage")] fn LoadFromFileAsync(&self, file: *mut super::super::storage::IStorageFile, out: *mut *mut super::super::foundation::IAsyncOperation<PdfDocument>) -> HRESULT,
    #[cfg(feature="windows-storage")] fn LoadFromFileWithPasswordAsync(&self, file: *mut super::super::storage::IStorageFile, password: HSTRING, out: *mut *mut super::super::foundation::IAsyncOperation<PdfDocument>) -> HRESULT,
    #[cfg(feature="windows-storage")] fn LoadFromStreamAsync(&self, inputStream: *mut super::super::storage::streams::IRandomAccessStream, out: *mut *mut super::super::foundation::IAsyncOperation<PdfDocument>) -> HRESULT,
    #[cfg(feature="windows-storage")] fn LoadFromStreamWithPasswordAsync(&self, inputStream: *mut super::super::storage::streams::IRandomAccessStream, password: HSTRING, out: *mut *mut super::super::foundation::IAsyncOperation<PdfDocument>) -> HRESULT
}}
impl IPdfDocumentStatics {
    #[cfg(feature="windows-storage")] #[inline] pub unsafe fn load_from_file_async(&self, file: &super::super::storage::IStorageFile) -> Result<ComPtr<super::super::foundation::IAsyncOperation<PdfDocument>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).LoadFromFileAsync)(self as *const _ as *mut _, file as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[cfg(feature="windows-storage")] #[inline] pub unsafe fn load_from_file_with_password_async(&self, file: &super::super::storage::IStorageFile, password: &HStringArg) -> Result<ComPtr<super::super::foundation::IAsyncOperation<PdfDocument>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).LoadFromFileWithPasswordAsync)(self as *const _ as *mut _, file as *const _ as *mut _, password.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[cfg(feature="windows-storage")] #[inline] pub unsafe fn load_from_stream_async(&self, inputStream: &super::super::storage::streams::IRandomAccessStream) -> Result<ComPtr<super::super::foundation::IAsyncOperation<PdfDocument>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).LoadFromStreamAsync)(self as *const _ as *mut _, inputStream as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[cfg(feature="windows-storage")] #[inline] pub unsafe fn load_from_stream_with_password_async(&self, inputStream: &super::super::storage::streams::IRandomAccessStream, password: &HStringArg) -> Result<ComPtr<super::super::foundation::IAsyncOperation<PdfDocument>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).LoadFromStreamWithPasswordAsync)(self as *const _ as *mut _, inputStream as *const _ as *mut _, password.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
DEFINE_IID!(IID_IPdfPage, 2645864648, 21280, 19708, 173, 118, 73, 63, 218, 208, 229, 148);
RT_INTERFACE!{interface IPdfPage(IPdfPageVtbl): IInspectable(IInspectableVtbl) [IID_IPdfPage] {
    #[cfg(not(feature="windows-storage"))] fn __Dummy0(&self) -> (),
    #[cfg(feature="windows-storage")] fn RenderToStreamAsync(&self, outputStream: *mut super::super::storage::streams::IRandomAccessStream, out: *mut *mut super::super::foundation::IAsyncAction) -> HRESULT,
    #[cfg(not(feature="windows-storage"))] fn __Dummy1(&self) -> (),
    #[cfg(feature="windows-storage")] fn RenderWithOptionsToStreamAsync(&self, outputStream: *mut super::super::storage::streams::IRandomAccessStream, options: *mut PdfPageRenderOptions, out: *mut *mut super::super::foundation::IAsyncAction) -> HRESULT,
    fn PreparePageAsync(&self, out: *mut *mut super::super::foundation::IAsyncAction) -> HRESULT,
    fn get_Index(&self, out: *mut u32) -> HRESULT,
    fn get_Size(&self, out: *mut super::super::foundation::Size) -> HRESULT,
    fn get_Dimensions(&self, out: *mut *mut PdfPageDimensions) -> HRESULT,
    fn get_Rotation(&self, out: *mut PdfPageRotation) -> HRESULT,
    fn get_PreferredZoom(&self, out: *mut f32) -> HRESULT
}}
impl IPdfPage {
    #[cfg(feature="windows-storage")] #[inline] pub unsafe fn render_to_stream_async(&self, outputStream: &super::super::storage::streams::IRandomAccessStream) -> Result<ComPtr<super::super::foundation::IAsyncAction>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).RenderToStreamAsync)(self as *const _ as *mut _, outputStream as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[cfg(feature="windows-storage")] #[inline] pub unsafe fn render_with_options_to_stream_async(&self, outputStream: &super::super::storage::streams::IRandomAccessStream, options: &PdfPageRenderOptions) -> Result<ComPtr<super::super::foundation::IAsyncAction>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).RenderWithOptionsToStreamAsync)(self as *const _ as *mut _, outputStream as *const _ as *mut _, options as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn prepare_page_async(&self) -> Result<ComPtr<super::super::foundation::IAsyncAction>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).PreparePageAsync)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_index(&self) -> Result<u32> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_Index)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_size(&self) -> Result<super::super::foundation::Size> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_Size)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_dimensions(&self) -> Result<ComPtr<PdfPageDimensions>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Dimensions)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_rotation(&self) -> Result<PdfPageRotation> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_Rotation)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_preferred_zoom(&self) -> Result<f32> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_PreferredZoom)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
}
RT_CLASS!{class PdfPage: IPdfPage}
DEFINE_IID!(IID_IPdfPageDimensions, 571933809, 12606, 17640, 131, 93, 99, 163, 231, 98, 74, 16);
RT_INTERFACE!{interface IPdfPageDimensions(IPdfPageDimensionsVtbl): IInspectable(IInspectableVtbl) [IID_IPdfPageDimensions] {
    fn get_MediaBox(&self, out: *mut super::super::foundation::Rect) -> HRESULT,
    fn get_CropBox(&self, out: *mut super::super::foundation::Rect) -> HRESULT,
    fn get_BleedBox(&self, out: *mut super::super::foundation::Rect) -> HRESULT,
    fn get_TrimBox(&self, out: *mut super::super::foundation::Rect) -> HRESULT,
    fn get_ArtBox(&self, out: *mut super::super::foundation::Rect) -> HRESULT
}}
impl IPdfPageDimensions {
    #[inline] pub unsafe fn get_media_box(&self) -> Result<super::super::foundation::Rect> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_MediaBox)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_crop_box(&self) -> Result<super::super::foundation::Rect> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_CropBox)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_bleed_box(&self) -> Result<super::super::foundation::Rect> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_BleedBox)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_trim_box(&self) -> Result<super::super::foundation::Rect> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_TrimBox)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_art_box(&self) -> Result<super::super::foundation::Rect> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_ArtBox)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
}
RT_CLASS!{class PdfPageDimensions: IPdfPageDimensions}
DEFINE_IID!(IID_IPdfPageRenderOptions, 1016595823, 47055, 19497, 154, 4, 82, 217, 2, 103, 244, 37);
RT_INTERFACE!{interface IPdfPageRenderOptions(IPdfPageRenderOptionsVtbl): IInspectable(IInspectableVtbl) [IID_IPdfPageRenderOptions] {
    fn get_SourceRect(&self, out: *mut super::super::foundation::Rect) -> HRESULT,
    fn put_SourceRect(&self, value: super::super::foundation::Rect) -> HRESULT,
    fn get_DestinationWidth(&self, out: *mut u32) -> HRESULT,
    fn put_DestinationWidth(&self, value: u32) -> HRESULT,
    fn get_DestinationHeight(&self, out: *mut u32) -> HRESULT,
    fn put_DestinationHeight(&self, value: u32) -> HRESULT,
    #[cfg(not(feature="windows-ui"))] fn __Dummy6(&self) -> (),
    #[cfg(feature="windows-ui")] fn get_BackgroundColor(&self, out: *mut super::super::ui::Color) -> HRESULT,
    #[cfg(not(feature="windows-ui"))] fn __Dummy7(&self) -> (),
    #[cfg(feature="windows-ui")] fn put_BackgroundColor(&self, value: super::super::ui::Color) -> HRESULT,
    fn get_IsIgnoringHighContrast(&self, out: *mut bool) -> HRESULT,
    fn put_IsIgnoringHighContrast(&self, value: bool) -> HRESULT,
    fn get_BitmapEncoderId(&self, out: *mut Guid) -> HRESULT,
    fn put_BitmapEncoderId(&self, value: Guid) -> HRESULT
}}
impl IPdfPageRenderOptions {
    #[inline] pub unsafe fn get_source_rect(&self) -> Result<super::super::foundation::Rect> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_SourceRect)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_source_rect(&self, value: super::super::foundation::Rect) -> Result<()> {
        let hr = ((*self.lpVtbl).put_SourceRect)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_destination_width(&self) -> Result<u32> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_DestinationWidth)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_destination_width(&self, value: u32) -> Result<()> {
        let hr = ((*self.lpVtbl).put_DestinationWidth)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_destination_height(&self) -> Result<u32> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_DestinationHeight)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_destination_height(&self, value: u32) -> Result<()> {
        let hr = ((*self.lpVtbl).put_DestinationHeight)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[cfg(feature="windows-ui")] #[inline] pub unsafe fn get_background_color(&self) -> Result<super::super::ui::Color> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_BackgroundColor)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[cfg(feature="windows-ui")] #[inline] pub unsafe fn set_background_color(&self, value: super::super::ui::Color) -> Result<()> {
        let hr = ((*self.lpVtbl).put_BackgroundColor)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_is_ignoring_high_contrast(&self) -> Result<bool> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_IsIgnoringHighContrast)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_is_ignoring_high_contrast(&self, value: bool) -> Result<()> {
        let hr = ((*self.lpVtbl).put_IsIgnoringHighContrast)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_bitmap_encoder_id(&self) -> Result<Guid> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_BitmapEncoderId)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_bitmap_encoder_id(&self, value: Guid) -> Result<()> {
        let hr = ((*self.lpVtbl).put_BitmapEncoderId)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
}
RT_CLASS!{class PdfPageRenderOptions: IPdfPageRenderOptions}
impl RtActivatable<IActivationFactory> for PdfPageRenderOptions {}
DEFINE_CLSID!(PdfPageRenderOptions(&[87,105,110,100,111,119,115,46,68,97,116,97,46,80,100,102,46,80,100,102,80,97,103,101,82,101,110,100,101,114,79,112,116,105,111,110,115,0]) [CLSID_PdfPageRenderOptions]);
RT_ENUM! { enum PdfPageRotation: i32 {
    Normal (PdfPageRotation_Normal) = 0, Rotate90 (PdfPageRotation_Rotate90) = 1, Rotate180 (PdfPageRotation_Rotate180) = 2, Rotate270 (PdfPageRotation_Rotate270) = 3,
}}
} // Windows.Data.Pdf
pub mod text { // Windows.Data.Text
use ::prelude::*;
RT_ENUM! { enum AlternateNormalizationFormat: i32 {
    NotNormalized (AlternateNormalizationFormat_NotNormalized) = 0, Number (AlternateNormalizationFormat_Number) = 1, Currency (AlternateNormalizationFormat_Currency) = 3, Date (AlternateNormalizationFormat_Date) = 4, Time (AlternateNormalizationFormat_Time) = 5,
}}
DEFINE_IID!(IID_IAlternateWordForm, 1194945566, 20921, 16903, 145, 70, 36, 142, 99, 106, 29, 29);
RT_INTERFACE!{interface IAlternateWordForm(IAlternateWordFormVtbl): IInspectable(IInspectableVtbl) [IID_IAlternateWordForm] {
    fn get_SourceTextSegment(&self, out: *mut TextSegment) -> HRESULT,
    fn get_AlternateText(&self, out: *mut HSTRING) -> HRESULT,
    fn get_NormalizationFormat(&self, out: *mut AlternateNormalizationFormat) -> HRESULT
}}
impl IAlternateWordForm {
    #[inline] pub unsafe fn get_source_text_segment(&self) -> Result<TextSegment> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_SourceTextSegment)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_alternate_text(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_AlternateText)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_normalization_format(&self) -> Result<AlternateNormalizationFormat> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_NormalizationFormat)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
}
RT_CLASS!{class AlternateWordForm: IAlternateWordForm}
DEFINE_IID!(IID_ISelectableWordSegment, 2439662775, 35495, 19576, 179, 116, 93, 237, 183, 82, 230, 11);
RT_INTERFACE!{interface ISelectableWordSegment(ISelectableWordSegmentVtbl): IInspectable(IInspectableVtbl) [IID_ISelectableWordSegment] {
    fn get_Text(&self, out: *mut HSTRING) -> HRESULT,
    fn get_SourceTextSegment(&self, out: *mut TextSegment) -> HRESULT
}}
impl ISelectableWordSegment {
    #[inline] pub unsafe fn get_text(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Text)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_source_text_segment(&self) -> Result<TextSegment> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_SourceTextSegment)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
}
RT_CLASS!{class SelectableWordSegment: ISelectableWordSegment}
DEFINE_IID!(IID_SelectableWordSegmentsTokenizingHandler, 977140892, 44766, 19911, 158, 108, 65, 192, 68, 189, 53, 146);
RT_DELEGATE!{delegate SelectableWordSegmentsTokenizingHandler(SelectableWordSegmentsTokenizingHandlerVtbl, SelectableWordSegmentsTokenizingHandlerImpl) [IID_SelectableWordSegmentsTokenizingHandler] {
    fn Invoke(&self, precedingWords: *mut super::super::foundation::collections::IIterable<SelectableWordSegment>, words: *mut super::super::foundation::collections::IIterable<SelectableWordSegment>) -> HRESULT
}}
impl SelectableWordSegmentsTokenizingHandler {
    #[inline] pub unsafe fn invoke(&self, precedingWords: &super::super::foundation::collections::IIterable<SelectableWordSegment>, words: &super::super::foundation::collections::IIterable<SelectableWordSegment>) -> Result<()> {
        let hr = ((*self.lpVtbl).Invoke)(self as *const _ as *mut _, precedingWords as *const _ as *mut _, words as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
}
DEFINE_IID!(IID_ISelectableWordsSegmenter, 4141625831, 19219, 17861, 136, 151, 125, 113, 38, 158, 8, 93);
RT_INTERFACE!{interface ISelectableWordsSegmenter(ISelectableWordsSegmenterVtbl): IInspectable(IInspectableVtbl) [IID_ISelectableWordsSegmenter] {
    fn get_ResolvedLanguage(&self, out: *mut HSTRING) -> HRESULT,
    fn GetTokenAt(&self, text: HSTRING, startIndex: u32, out: *mut *mut SelectableWordSegment) -> HRESULT,
    fn GetTokens(&self, text: HSTRING, out: *mut *mut super::super::foundation::collections::IVectorView<SelectableWordSegment>) -> HRESULT,
    fn Tokenize(&self, text: HSTRING, startIndex: u32, handler: *mut SelectableWordSegmentsTokenizingHandler) -> HRESULT
}}
impl ISelectableWordsSegmenter {
    #[inline] pub unsafe fn get_resolved_language(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_ResolvedLanguage)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_token_at(&self, text: &HStringArg, startIndex: u32) -> Result<ComPtr<SelectableWordSegment>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetTokenAt)(self as *const _ as *mut _, text.get(), startIndex, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_tokens(&self, text: &HStringArg) -> Result<ComPtr<super::super::foundation::collections::IVectorView<SelectableWordSegment>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetTokens)(self as *const _ as *mut _, text.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn tokenize(&self, text: &HStringArg, startIndex: u32, handler: &SelectableWordSegmentsTokenizingHandler) -> Result<()> {
        let hr = ((*self.lpVtbl).Tokenize)(self as *const _ as *mut _, text.get(), startIndex, handler as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
}
RT_CLASS!{class SelectableWordsSegmenter: ISelectableWordsSegmenter}
impl RtActivatable<ISelectableWordsSegmenterFactory> for SelectableWordsSegmenter {}
impl SelectableWordsSegmenter {
    #[inline] pub fn create_with_language(language: &HStringArg) -> Result<ComPtr<SelectableWordsSegmenter>> { unsafe {
        <Self as RtActivatable<ISelectableWordsSegmenterFactory>>::get_activation_factory().create_with_language(language)
    }}
}
DEFINE_CLSID!(SelectableWordsSegmenter(&[87,105,110,100,111,119,115,46,68,97,116,97,46,84,101,120,116,46,83,101,108,101,99,116,97,98,108,101,87,111,114,100,115,83,101,103,109,101,110,116,101,114,0]) [CLSID_SelectableWordsSegmenter]);
DEFINE_IID!(IID_ISelectableWordsSegmenterFactory, 2356835912, 24663, 17209, 188, 112, 242, 16, 1, 10, 65, 80);
RT_INTERFACE!{static interface ISelectableWordsSegmenterFactory(ISelectableWordsSegmenterFactoryVtbl): IInspectable(IInspectableVtbl) [IID_ISelectableWordsSegmenterFactory] {
    fn CreateWithLanguage(&self, language: HSTRING, out: *mut *mut SelectableWordsSegmenter) -> HRESULT
}}
impl ISelectableWordsSegmenterFactory {
    #[inline] pub unsafe fn create_with_language(&self, language: &HStringArg) -> Result<ComPtr<SelectableWordsSegmenter>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).CreateWithLanguage)(self as *const _ as *mut _, language.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
DEFINE_IID!(IID_ISemanticTextQuery, 1780263761, 8114, 18697, 128, 184, 53, 115, 26, 43, 62, 127);
RT_INTERFACE!{interface ISemanticTextQuery(ISemanticTextQueryVtbl): IInspectable(IInspectableVtbl) [IID_ISemanticTextQuery] {
    fn Find(&self, content: HSTRING, out: *mut *mut super::super::foundation::collections::IVectorView<TextSegment>) -> HRESULT,
    fn FindInProperty(&self, propertyContent: HSTRING, propertyName: HSTRING, out: *mut *mut super::super::foundation::collections::IVectorView<TextSegment>) -> HRESULT
}}
impl ISemanticTextQuery {
    #[inline] pub unsafe fn find(&self, content: &HStringArg) -> Result<ComPtr<super::super::foundation::collections::IVectorView<TextSegment>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).Find)(self as *const _ as *mut _, content.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn find_in_property(&self, propertyContent: &HStringArg, propertyName: &HStringArg) -> Result<ComPtr<super::super::foundation::collections::IVectorView<TextSegment>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).FindInProperty)(self as *const _ as *mut _, propertyContent.get(), propertyName.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
RT_CLASS!{class SemanticTextQuery: ISemanticTextQuery}
impl RtActivatable<ISemanticTextQueryFactory> for SemanticTextQuery {}
impl SemanticTextQuery {
    #[inline] pub fn create(aqsFilter: &HStringArg) -> Result<ComPtr<SemanticTextQuery>> { unsafe {
        <Self as RtActivatable<ISemanticTextQueryFactory>>::get_activation_factory().create(aqsFilter)
    }}
    #[inline] pub fn create_with_language(aqsFilter: &HStringArg, filterLanguage: &HStringArg) -> Result<ComPtr<SemanticTextQuery>> { unsafe {
        <Self as RtActivatable<ISemanticTextQueryFactory>>::get_activation_factory().create_with_language(aqsFilter, filterLanguage)
    }}
}
DEFINE_CLSID!(SemanticTextQuery(&[87,105,110,100,111,119,115,46,68,97,116,97,46,84,101,120,116,46,83,101,109,97,110,116,105,99,84,101,120,116,81,117,101,114,121,0]) [CLSID_SemanticTextQuery]);
DEFINE_IID!(IID_ISemanticTextQueryFactory, 596378883, 63893, 17799, 135, 119, 162, 183, 216, 10, 207, 239);
RT_INTERFACE!{static interface ISemanticTextQueryFactory(ISemanticTextQueryFactoryVtbl): IInspectable(IInspectableVtbl) [IID_ISemanticTextQueryFactory] {
    fn Create(&self, aqsFilter: HSTRING, out: *mut *mut SemanticTextQuery) -> HRESULT,
    fn CreateWithLanguage(&self, aqsFilter: HSTRING, filterLanguage: HSTRING, out: *mut *mut SemanticTextQuery) -> HRESULT
}}
impl ISemanticTextQueryFactory {
    #[inline] pub unsafe fn create(&self, aqsFilter: &HStringArg) -> Result<ComPtr<SemanticTextQuery>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).Create)(self as *const _ as *mut _, aqsFilter.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn create_with_language(&self, aqsFilter: &HStringArg, filterLanguage: &HStringArg) -> Result<ComPtr<SemanticTextQuery>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).CreateWithLanguage)(self as *const _ as *mut _, aqsFilter.get(), filterLanguage.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
DEFINE_IID!(IID_ITextConversionGenerator, 56650334, 10921, 19126, 175, 139, 165, 98, 182, 58, 137, 146);
RT_INTERFACE!{interface ITextConversionGenerator(ITextConversionGeneratorVtbl): IInspectable(IInspectableVtbl) [IID_ITextConversionGenerator] {
    fn get_ResolvedLanguage(&self, out: *mut HSTRING) -> HRESULT,
    fn get_LanguageAvailableButNotInstalled(&self, out: *mut bool) -> HRESULT,
    fn GetCandidatesAsync(&self, input: HSTRING, out: *mut *mut super::super::foundation::IAsyncOperation<super::super::foundation::collections::IVectorView<HString>>) -> HRESULT,
    fn GetCandidatesWithMaxCountAsync(&self, input: HSTRING, maxCandidates: u32, out: *mut *mut super::super::foundation::IAsyncOperation<super::super::foundation::collections::IVectorView<HString>>) -> HRESULT
}}
impl ITextConversionGenerator {
    #[inline] pub unsafe fn get_resolved_language(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_ResolvedLanguage)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_language_available_but_not_installed(&self) -> Result<bool> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_LanguageAvailableButNotInstalled)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_candidates_async(&self, input: &HStringArg) -> Result<ComPtr<super::super::foundation::IAsyncOperation<super::super::foundation::collections::IVectorView<HString>>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetCandidatesAsync)(self as *const _ as *mut _, input.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_candidates_with_max_count_async(&self, input: &HStringArg, maxCandidates: u32) -> Result<ComPtr<super::super::foundation::IAsyncOperation<super::super::foundation::collections::IVectorView<HString>>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetCandidatesWithMaxCountAsync)(self as *const _ as *mut _, input.get(), maxCandidates, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
RT_CLASS!{class TextConversionGenerator: ITextConversionGenerator}
impl RtActivatable<ITextConversionGeneratorFactory> for TextConversionGenerator {}
impl TextConversionGenerator {
    #[inline] pub fn create(languageTag: &HStringArg) -> Result<ComPtr<TextConversionGenerator>> { unsafe {
        <Self as RtActivatable<ITextConversionGeneratorFactory>>::get_activation_factory().create(languageTag)
    }}
}
DEFINE_CLSID!(TextConversionGenerator(&[87,105,110,100,111,119,115,46,68,97,116,97,46,84,101,120,116,46,84,101,120,116,67,111,110,118,101,114,115,105,111,110,71,101,110,101,114,97,116,111,114,0]) [CLSID_TextConversionGenerator]);
DEFINE_IID!(IID_ITextConversionGeneratorFactory, 4239013761, 12419, 18859, 190, 21, 86, 223, 187, 183, 77, 111);
RT_INTERFACE!{static interface ITextConversionGeneratorFactory(ITextConversionGeneratorFactoryVtbl): IInspectable(IInspectableVtbl) [IID_ITextConversionGeneratorFactory] {
    fn Create(&self, languageTag: HSTRING, out: *mut *mut TextConversionGenerator) -> HRESULT
}}
impl ITextConversionGeneratorFactory {
    #[inline] pub unsafe fn create(&self, languageTag: &HStringArg) -> Result<ComPtr<TextConversionGenerator>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).Create)(self as *const _ as *mut _, languageTag.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
DEFINE_IID!(IID_ITextPhoneme, 2472715274, 39802, 17769, 148, 207, 216, 79, 47, 56, 207, 155);
RT_INTERFACE!{interface ITextPhoneme(ITextPhonemeVtbl): IInspectable(IInspectableVtbl) [IID_ITextPhoneme] {
    fn get_DisplayText(&self, out: *mut HSTRING) -> HRESULT,
    fn get_ReadingText(&self, out: *mut HSTRING) -> HRESULT
}}
impl ITextPhoneme {
    #[inline] pub unsafe fn get_display_text(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_DisplayText)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_reading_text(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_ReadingText)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
}
RT_CLASS!{class TextPhoneme: ITextPhoneme}
DEFINE_IID!(IID_ITextPredictionGenerator, 1588374279, 44017, 19638, 157, 158, 50, 111, 43, 70, 135, 86);
RT_INTERFACE!{interface ITextPredictionGenerator(ITextPredictionGeneratorVtbl): IInspectable(IInspectableVtbl) [IID_ITextPredictionGenerator] {
    fn get_ResolvedLanguage(&self, out: *mut HSTRING) -> HRESULT,
    fn get_LanguageAvailableButNotInstalled(&self, out: *mut bool) -> HRESULT,
    fn GetCandidatesAsync(&self, input: HSTRING, out: *mut *mut super::super::foundation::IAsyncOperation<super::super::foundation::collections::IVectorView<HString>>) -> HRESULT,
    fn GetCandidatesWithMaxCountAsync(&self, input: HSTRING, maxCandidates: u32, out: *mut *mut super::super::foundation::IAsyncOperation<super::super::foundation::collections::IVectorView<HString>>) -> HRESULT
}}
impl ITextPredictionGenerator {
    #[inline] pub unsafe fn get_resolved_language(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_ResolvedLanguage)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_language_available_but_not_installed(&self) -> Result<bool> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_LanguageAvailableButNotInstalled)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_candidates_async(&self, input: &HStringArg) -> Result<ComPtr<super::super::foundation::IAsyncOperation<super::super::foundation::collections::IVectorView<HString>>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetCandidatesAsync)(self as *const _ as *mut _, input.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_candidates_with_max_count_async(&self, input: &HStringArg, maxCandidates: u32) -> Result<ComPtr<super::super::foundation::IAsyncOperation<super::super::foundation::collections::IVectorView<HString>>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetCandidatesWithMaxCountAsync)(self as *const _ as *mut _, input.get(), maxCandidates, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
RT_CLASS!{class TextPredictionGenerator: ITextPredictionGenerator}
impl RtActivatable<ITextPredictionGeneratorFactory> for TextPredictionGenerator {}
impl TextPredictionGenerator {
    #[inline] pub fn create(languageTag: &HStringArg) -> Result<ComPtr<TextPredictionGenerator>> { unsafe {
        <Self as RtActivatable<ITextPredictionGeneratorFactory>>::get_activation_factory().create(languageTag)
    }}
}
DEFINE_CLSID!(TextPredictionGenerator(&[87,105,110,100,111,119,115,46,68,97,116,97,46,84,101,120,116,46,84,101,120,116,80,114,101,100,105,99,116,105,111,110,71,101,110,101,114,97,116,111,114,0]) [CLSID_TextPredictionGenerator]);
DEFINE_IID!(IID_ITextPredictionGeneratorFactory, 1918350358, 35746, 18257, 157, 48, 157, 133, 67, 86, 83, 162);
RT_INTERFACE!{static interface ITextPredictionGeneratorFactory(ITextPredictionGeneratorFactoryVtbl): IInspectable(IInspectableVtbl) [IID_ITextPredictionGeneratorFactory] {
    fn Create(&self, languageTag: HSTRING, out: *mut *mut TextPredictionGenerator) -> HRESULT
}}
impl ITextPredictionGeneratorFactory {
    #[inline] pub unsafe fn create(&self, languageTag: &HStringArg) -> Result<ComPtr<TextPredictionGenerator>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).Create)(self as *const _ as *mut _, languageTag.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
DEFINE_IID!(IID_ITextReverseConversionGenerator, 1374156052, 40017, 19846, 174, 27, 180, 152, 251, 173, 131, 19);
RT_INTERFACE!{interface ITextReverseConversionGenerator(ITextReverseConversionGeneratorVtbl): IInspectable(IInspectableVtbl) [IID_ITextReverseConversionGenerator] {
    fn get_ResolvedLanguage(&self, out: *mut HSTRING) -> HRESULT,
    fn get_LanguageAvailableButNotInstalled(&self, out: *mut bool) -> HRESULT,
    fn ConvertBackAsync(&self, input: HSTRING, out: *mut *mut super::super::foundation::IAsyncOperation<HString>) -> HRESULT
}}
impl ITextReverseConversionGenerator {
    #[inline] pub unsafe fn get_resolved_language(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_ResolvedLanguage)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_language_available_but_not_installed(&self) -> Result<bool> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_LanguageAvailableButNotInstalled)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn convert_back_async(&self, input: &HStringArg) -> Result<ComPtr<super::super::foundation::IAsyncOperation<HString>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).ConvertBackAsync)(self as *const _ as *mut _, input.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
RT_CLASS!{class TextReverseConversionGenerator: ITextReverseConversionGenerator}
impl RtActivatable<ITextReverseConversionGeneratorFactory> for TextReverseConversionGenerator {}
impl TextReverseConversionGenerator {
    #[inline] pub fn create(languageTag: &HStringArg) -> Result<ComPtr<TextReverseConversionGenerator>> { unsafe {
        <Self as RtActivatable<ITextReverseConversionGeneratorFactory>>::get_activation_factory().create(languageTag)
    }}
}
DEFINE_CLSID!(TextReverseConversionGenerator(&[87,105,110,100,111,119,115,46,68,97,116,97,46,84,101,120,116,46,84,101,120,116,82,101,118,101,114,115,101,67,111,110,118,101,114,115,105,111,110,71,101,110,101,114,97,116,111,114,0]) [CLSID_TextReverseConversionGenerator]);
DEFINE_IID!(IID_ITextReverseConversionGenerator2, 447730412, 34262, 18173, 130, 138, 58, 72, 48, 250, 110, 24);
RT_INTERFACE!{interface ITextReverseConversionGenerator2(ITextReverseConversionGenerator2Vtbl): IInspectable(IInspectableVtbl) [IID_ITextReverseConversionGenerator2] {
    fn GetPhonemesAsync(&self, input: HSTRING, out: *mut *mut super::super::foundation::IAsyncOperation<super::super::foundation::collections::IVectorView<TextPhoneme>>) -> HRESULT
}}
impl ITextReverseConversionGenerator2 {
    #[inline] pub unsafe fn get_phonemes_async(&self, input: &HStringArg) -> Result<ComPtr<super::super::foundation::IAsyncOperation<super::super::foundation::collections::IVectorView<TextPhoneme>>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetPhonemesAsync)(self as *const _ as *mut _, input.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
DEFINE_IID!(IID_ITextReverseConversionGeneratorFactory, 1673450278, 8154, 16886, 137, 213, 35, 221, 234, 60, 114, 154);
RT_INTERFACE!{static interface ITextReverseConversionGeneratorFactory(ITextReverseConversionGeneratorFactoryVtbl): IInspectable(IInspectableVtbl) [IID_ITextReverseConversionGeneratorFactory] {
    fn Create(&self, languageTag: HSTRING, out: *mut *mut TextReverseConversionGenerator) -> HRESULT
}}
impl ITextReverseConversionGeneratorFactory {
    #[inline] pub unsafe fn create(&self, languageTag: &HStringArg) -> Result<ComPtr<TextReverseConversionGenerator>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).Create)(self as *const _ as *mut _, languageTag.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
RT_STRUCT! { struct TextSegment {
    StartPosition: u32, Length: u32,
}}
RT_CLASS!{static class UnicodeCharacters}
impl RtActivatable<IUnicodeCharactersStatics> for UnicodeCharacters {}
impl UnicodeCharacters {
    #[inline] pub fn get_codepoint_from_surrogate_pair(highSurrogate: u32, lowSurrogate: u32) -> Result<u32> { unsafe {
        <Self as RtActivatable<IUnicodeCharactersStatics>>::get_activation_factory().get_codepoint_from_surrogate_pair(highSurrogate, lowSurrogate)
    }}
    #[inline] pub fn get_surrogate_pair_from_codepoint(codepoint: u32) -> Result<(Char, Char)> { unsafe {
        <Self as RtActivatable<IUnicodeCharactersStatics>>::get_activation_factory().get_surrogate_pair_from_codepoint(codepoint)
    }}
    #[inline] pub fn is_high_surrogate(codepoint: u32) -> Result<bool> { unsafe {
        <Self as RtActivatable<IUnicodeCharactersStatics>>::get_activation_factory().is_high_surrogate(codepoint)
    }}
    #[inline] pub fn is_low_surrogate(codepoint: u32) -> Result<bool> { unsafe {
        <Self as RtActivatable<IUnicodeCharactersStatics>>::get_activation_factory().is_low_surrogate(codepoint)
    }}
    #[inline] pub fn is_supplementary(codepoint: u32) -> Result<bool> { unsafe {
        <Self as RtActivatable<IUnicodeCharactersStatics>>::get_activation_factory().is_supplementary(codepoint)
    }}
    #[inline] pub fn is_noncharacter(codepoint: u32) -> Result<bool> { unsafe {
        <Self as RtActivatable<IUnicodeCharactersStatics>>::get_activation_factory().is_noncharacter(codepoint)
    }}
    #[inline] pub fn is_whitespace(codepoint: u32) -> Result<bool> { unsafe {
        <Self as RtActivatable<IUnicodeCharactersStatics>>::get_activation_factory().is_whitespace(codepoint)
    }}
    #[inline] pub fn is_alphabetic(codepoint: u32) -> Result<bool> { unsafe {
        <Self as RtActivatable<IUnicodeCharactersStatics>>::get_activation_factory().is_alphabetic(codepoint)
    }}
    #[inline] pub fn is_cased(codepoint: u32) -> Result<bool> { unsafe {
        <Self as RtActivatable<IUnicodeCharactersStatics>>::get_activation_factory().is_cased(codepoint)
    }}
    #[inline] pub fn is_uppercase(codepoint: u32) -> Result<bool> { unsafe {
        <Self as RtActivatable<IUnicodeCharactersStatics>>::get_activation_factory().is_uppercase(codepoint)
    }}
    #[inline] pub fn is_lowercase(codepoint: u32) -> Result<bool> { unsafe {
        <Self as RtActivatable<IUnicodeCharactersStatics>>::get_activation_factory().is_lowercase(codepoint)
    }}
    #[inline] pub fn is_id_start(codepoint: u32) -> Result<bool> { unsafe {
        <Self as RtActivatable<IUnicodeCharactersStatics>>::get_activation_factory().is_id_start(codepoint)
    }}
    #[inline] pub fn is_id_continue(codepoint: u32) -> Result<bool> { unsafe {
        <Self as RtActivatable<IUnicodeCharactersStatics>>::get_activation_factory().is_id_continue(codepoint)
    }}
    #[inline] pub fn is_grapheme_base(codepoint: u32) -> Result<bool> { unsafe {
        <Self as RtActivatable<IUnicodeCharactersStatics>>::get_activation_factory().is_grapheme_base(codepoint)
    }}
    #[inline] pub fn is_grapheme_extend(codepoint: u32) -> Result<bool> { unsafe {
        <Self as RtActivatable<IUnicodeCharactersStatics>>::get_activation_factory().is_grapheme_extend(codepoint)
    }}
    #[inline] pub fn get_numeric_type(codepoint: u32) -> Result<UnicodeNumericType> { unsafe {
        <Self as RtActivatable<IUnicodeCharactersStatics>>::get_activation_factory().get_numeric_type(codepoint)
    }}
    #[inline] pub fn get_general_category(codepoint: u32) -> Result<UnicodeGeneralCategory> { unsafe {
        <Self as RtActivatable<IUnicodeCharactersStatics>>::get_activation_factory().get_general_category(codepoint)
    }}
}
DEFINE_CLSID!(UnicodeCharacters(&[87,105,110,100,111,119,115,46,68,97,116,97,46,84,101,120,116,46,85,110,105,99,111,100,101,67,104,97,114,97,99,116,101,114,115,0]) [CLSID_UnicodeCharacters]);
DEFINE_IID!(IID_IUnicodeCharactersStatics, 2542837383, 37521, 20369, 182, 200, 182, 227, 89, 215, 167, 251);
RT_INTERFACE!{static interface IUnicodeCharactersStatics(IUnicodeCharactersStaticsVtbl): IInspectable(IInspectableVtbl) [IID_IUnicodeCharactersStatics] {
    fn GetCodepointFromSurrogatePair(&self, highSurrogate: u32, lowSurrogate: u32, out: *mut u32) -> HRESULT,
    fn GetSurrogatePairFromCodepoint(&self, codepoint: u32, highSurrogate: *mut Char, lowSurrogate: *mut Char) -> HRESULT,
    fn IsHighSurrogate(&self, codepoint: u32, out: *mut bool) -> HRESULT,
    fn IsLowSurrogate(&self, codepoint: u32, out: *mut bool) -> HRESULT,
    fn IsSupplementary(&self, codepoint: u32, out: *mut bool) -> HRESULT,
    fn IsNoncharacter(&self, codepoint: u32, out: *mut bool) -> HRESULT,
    fn IsWhitespace(&self, codepoint: u32, out: *mut bool) -> HRESULT,
    fn IsAlphabetic(&self, codepoint: u32, out: *mut bool) -> HRESULT,
    fn IsCased(&self, codepoint: u32, out: *mut bool) -> HRESULT,
    fn IsUppercase(&self, codepoint: u32, out: *mut bool) -> HRESULT,
    fn IsLowercase(&self, codepoint: u32, out: *mut bool) -> HRESULT,
    fn IsIdStart(&self, codepoint: u32, out: *mut bool) -> HRESULT,
    fn IsIdContinue(&self, codepoint: u32, out: *mut bool) -> HRESULT,
    fn IsGraphemeBase(&self, codepoint: u32, out: *mut bool) -> HRESULT,
    fn IsGraphemeExtend(&self, codepoint: u32, out: *mut bool) -> HRESULT,
    fn GetNumericType(&self, codepoint: u32, out: *mut UnicodeNumericType) -> HRESULT,
    fn GetGeneralCategory(&self, codepoint: u32, out: *mut UnicodeGeneralCategory) -> HRESULT
}}
impl IUnicodeCharactersStatics {
    #[inline] pub unsafe fn get_codepoint_from_surrogate_pair(&self, highSurrogate: u32, lowSurrogate: u32) -> Result<u32> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).GetCodepointFromSurrogatePair)(self as *const _ as *mut _, highSurrogate, lowSurrogate, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_surrogate_pair_from_codepoint(&self, codepoint: u32) -> Result<(Char, Char)> {
        let mut highSurrogate = zeroed(); let mut lowSurrogate = zeroed();
        let hr = ((*self.lpVtbl).GetSurrogatePairFromCodepoint)(self as *const _ as *mut _, codepoint, &mut highSurrogate, &mut lowSurrogate);
        if hr == S_OK { Ok((highSurrogate, lowSurrogate)) } else { err(hr) }
    }
    #[inline] pub unsafe fn is_high_surrogate(&self, codepoint: u32) -> Result<bool> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).IsHighSurrogate)(self as *const _ as *mut _, codepoint, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn is_low_surrogate(&self, codepoint: u32) -> Result<bool> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).IsLowSurrogate)(self as *const _ as *mut _, codepoint, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn is_supplementary(&self, codepoint: u32) -> Result<bool> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).IsSupplementary)(self as *const _ as *mut _, codepoint, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn is_noncharacter(&self, codepoint: u32) -> Result<bool> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).IsNoncharacter)(self as *const _ as *mut _, codepoint, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn is_whitespace(&self, codepoint: u32) -> Result<bool> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).IsWhitespace)(self as *const _ as *mut _, codepoint, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn is_alphabetic(&self, codepoint: u32) -> Result<bool> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).IsAlphabetic)(self as *const _ as *mut _, codepoint, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn is_cased(&self, codepoint: u32) -> Result<bool> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).IsCased)(self as *const _ as *mut _, codepoint, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn is_uppercase(&self, codepoint: u32) -> Result<bool> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).IsUppercase)(self as *const _ as *mut _, codepoint, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn is_lowercase(&self, codepoint: u32) -> Result<bool> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).IsLowercase)(self as *const _ as *mut _, codepoint, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn is_id_start(&self, codepoint: u32) -> Result<bool> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).IsIdStart)(self as *const _ as *mut _, codepoint, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn is_id_continue(&self, codepoint: u32) -> Result<bool> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).IsIdContinue)(self as *const _ as *mut _, codepoint, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn is_grapheme_base(&self, codepoint: u32) -> Result<bool> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).IsGraphemeBase)(self as *const _ as *mut _, codepoint, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn is_grapheme_extend(&self, codepoint: u32) -> Result<bool> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).IsGraphemeExtend)(self as *const _ as *mut _, codepoint, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_numeric_type(&self, codepoint: u32) -> Result<UnicodeNumericType> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).GetNumericType)(self as *const _ as *mut _, codepoint, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_general_category(&self, codepoint: u32) -> Result<UnicodeGeneralCategory> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).GetGeneralCategory)(self as *const _ as *mut _, codepoint, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
}
RT_ENUM! { enum UnicodeGeneralCategory: i32 {
    UppercaseLetter (UnicodeGeneralCategory_UppercaseLetter) = 0, LowercaseLetter (UnicodeGeneralCategory_LowercaseLetter) = 1, TitlecaseLetter (UnicodeGeneralCategory_TitlecaseLetter) = 2, ModifierLetter (UnicodeGeneralCategory_ModifierLetter) = 3, OtherLetter (UnicodeGeneralCategory_OtherLetter) = 4, NonspacingMark (UnicodeGeneralCategory_NonspacingMark) = 5, SpacingCombiningMark (UnicodeGeneralCategory_SpacingCombiningMark) = 6, EnclosingMark (UnicodeGeneralCategory_EnclosingMark) = 7, DecimalDigitNumber (UnicodeGeneralCategory_DecimalDigitNumber) = 8, LetterNumber (UnicodeGeneralCategory_LetterNumber) = 9, OtherNumber (UnicodeGeneralCategory_OtherNumber) = 10, SpaceSeparator (UnicodeGeneralCategory_SpaceSeparator) = 11, LineSeparator (UnicodeGeneralCategory_LineSeparator) = 12, ParagraphSeparator (UnicodeGeneralCategory_ParagraphSeparator) = 13, Control (UnicodeGeneralCategory_Control) = 14, Format (UnicodeGeneralCategory_Format) = 15, Surrogate (UnicodeGeneralCategory_Surrogate) = 16, PrivateUse (UnicodeGeneralCategory_PrivateUse) = 17, ConnectorPunctuation (UnicodeGeneralCategory_ConnectorPunctuation) = 18, DashPunctuation (UnicodeGeneralCategory_DashPunctuation) = 19, OpenPunctuation (UnicodeGeneralCategory_OpenPunctuation) = 20, ClosePunctuation (UnicodeGeneralCategory_ClosePunctuation) = 21, InitialQuotePunctuation (UnicodeGeneralCategory_InitialQuotePunctuation) = 22, FinalQuotePunctuation (UnicodeGeneralCategory_FinalQuotePunctuation) = 23, OtherPunctuation (UnicodeGeneralCategory_OtherPunctuation) = 24, MathSymbol (UnicodeGeneralCategory_MathSymbol) = 25, CurrencySymbol (UnicodeGeneralCategory_CurrencySymbol) = 26, ModifierSymbol (UnicodeGeneralCategory_ModifierSymbol) = 27, OtherSymbol (UnicodeGeneralCategory_OtherSymbol) = 28, NotAssigned (UnicodeGeneralCategory_NotAssigned) = 29,
}}
RT_ENUM! { enum UnicodeNumericType: i32 {
    None (UnicodeNumericType_None) = 0, Decimal (UnicodeNumericType_Decimal) = 1, Digit (UnicodeNumericType_Digit) = 2, Numeric (UnicodeNumericType_Numeric) = 3,
}}
DEFINE_IID!(IID_IWordSegment, 3537156717, 39036, 19648, 182, 189, 212, 154, 17, 179, 143, 154);
RT_INTERFACE!{interface IWordSegment(IWordSegmentVtbl): IInspectable(IInspectableVtbl) [IID_IWordSegment] {
    fn get_Text(&self, out: *mut HSTRING) -> HRESULT,
    fn get_SourceTextSegment(&self, out: *mut TextSegment) -> HRESULT,
    fn get_AlternateForms(&self, out: *mut *mut super::super::foundation::collections::IVectorView<AlternateWordForm>) -> HRESULT
}}
impl IWordSegment {
    #[inline] pub unsafe fn get_text(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Text)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_source_text_segment(&self) -> Result<TextSegment> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_SourceTextSegment)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_alternate_forms(&self) -> Result<ComPtr<super::super::foundation::collections::IVectorView<AlternateWordForm>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_AlternateForms)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
RT_CLASS!{class WordSegment: IWordSegment}
DEFINE_IID!(IID_WordSegmentsTokenizingHandler, 2782749527, 48938, 19535, 163, 31, 41, 231, 28, 111, 139, 53);
RT_DELEGATE!{delegate WordSegmentsTokenizingHandler(WordSegmentsTokenizingHandlerVtbl, WordSegmentsTokenizingHandlerImpl) [IID_WordSegmentsTokenizingHandler] {
    fn Invoke(&self, precedingWords: *mut super::super::foundation::collections::IIterable<WordSegment>, words: *mut super::super::foundation::collections::IIterable<WordSegment>) -> HRESULT
}}
impl WordSegmentsTokenizingHandler {
    #[inline] pub unsafe fn invoke(&self, precedingWords: &super::super::foundation::collections::IIterable<WordSegment>, words: &super::super::foundation::collections::IIterable<WordSegment>) -> Result<()> {
        let hr = ((*self.lpVtbl).Invoke)(self as *const _ as *mut _, precedingWords as *const _ as *mut _, words as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
}
DEFINE_IID!(IID_IWordsSegmenter, 2259997905, 45822, 20020, 168, 29, 102, 100, 3, 0, 69, 79);
RT_INTERFACE!{interface IWordsSegmenter(IWordsSegmenterVtbl): IInspectable(IInspectableVtbl) [IID_IWordsSegmenter] {
    fn get_ResolvedLanguage(&self, out: *mut HSTRING) -> HRESULT,
    fn GetTokenAt(&self, text: HSTRING, startIndex: u32, out: *mut *mut WordSegment) -> HRESULT,
    fn GetTokens(&self, text: HSTRING, out: *mut *mut super::super::foundation::collections::IVectorView<WordSegment>) -> HRESULT,
    fn Tokenize(&self, text: HSTRING, startIndex: u32, handler: *mut WordSegmentsTokenizingHandler) -> HRESULT
}}
impl IWordsSegmenter {
    #[inline] pub unsafe fn get_resolved_language(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_ResolvedLanguage)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_token_at(&self, text: &HStringArg, startIndex: u32) -> Result<ComPtr<WordSegment>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetTokenAt)(self as *const _ as *mut _, text.get(), startIndex, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_tokens(&self, text: &HStringArg) -> Result<ComPtr<super::super::foundation::collections::IVectorView<WordSegment>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetTokens)(self as *const _ as *mut _, text.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn tokenize(&self, text: &HStringArg, startIndex: u32, handler: &WordSegmentsTokenizingHandler) -> Result<()> {
        let hr = ((*self.lpVtbl).Tokenize)(self as *const _ as *mut _, text.get(), startIndex, handler as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
}
RT_CLASS!{class WordsSegmenter: IWordsSegmenter}
impl RtActivatable<IWordsSegmenterFactory> for WordsSegmenter {}
impl WordsSegmenter {
    #[inline] pub fn create_with_language(language: &HStringArg) -> Result<ComPtr<WordsSegmenter>> { unsafe {
        <Self as RtActivatable<IWordsSegmenterFactory>>::get_activation_factory().create_with_language(language)
    }}
}
DEFINE_CLSID!(WordsSegmenter(&[87,105,110,100,111,119,115,46,68,97,116,97,46,84,101,120,116,46,87,111,114,100,115,83,101,103,109,101,110,116,101,114,0]) [CLSID_WordsSegmenter]);
DEFINE_IID!(IID_IWordsSegmenterFactory, 3868684916, 64565, 17756, 139, 251, 109, 127, 70, 83, 202, 151);
RT_INTERFACE!{static interface IWordsSegmenterFactory(IWordsSegmenterFactoryVtbl): IInspectable(IInspectableVtbl) [IID_IWordsSegmenterFactory] {
    fn CreateWithLanguage(&self, language: HSTRING, out: *mut *mut WordsSegmenter) -> HRESULT
}}
impl IWordsSegmenterFactory {
    #[inline] pub unsafe fn create_with_language(&self, language: &HStringArg) -> Result<ComPtr<WordsSegmenter>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).CreateWithLanguage)(self as *const _ as *mut _, language.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
} // Windows.Data.Text
