use ::prelude::*;
RT_CLASS!{static class ApplicationLanguages}
impl RtActivatable<IApplicationLanguagesStatics> for ApplicationLanguages {}
impl ApplicationLanguages {
    #[inline] pub fn get_primary_language_override() -> Result<HString> { unsafe {
        <Self as RtActivatable<IApplicationLanguagesStatics>>::get_activation_factory().get_primary_language_override()
    }}
    #[inline] pub fn set_primary_language_override(value: &HStringArg) -> Result<()> { unsafe {
        <Self as RtActivatable<IApplicationLanguagesStatics>>::get_activation_factory().set_primary_language_override(value)
    }}
    #[inline] pub fn get_languages() -> Result<ComPtr<super::foundation::collections::IVectorView<HString>>> { unsafe {
        <Self as RtActivatable<IApplicationLanguagesStatics>>::get_activation_factory().get_languages()
    }}
    #[inline] pub fn get_manifest_languages() -> Result<ComPtr<super::foundation::collections::IVectorView<HString>>> { unsafe {
        <Self as RtActivatable<IApplicationLanguagesStatics>>::get_activation_factory().get_manifest_languages()
    }}
}
DEFINE_CLSID!(ApplicationLanguages(&[87,105,110,100,111,119,115,46,71,108,111,98,97,108,105,122,97,116,105,111,110,46,65,112,112,108,105,99,97,116,105,111,110,76,97,110,103,117,97,103,101,115,0]) [CLSID_ApplicationLanguages]);
DEFINE_IID!(IID_IApplicationLanguagesStatics, 1974732871, 2636, 19090, 149, 101, 253, 99, 201, 95, 122, 237);
RT_INTERFACE!{static interface IApplicationLanguagesStatics(IApplicationLanguagesStaticsVtbl): IInspectable(IInspectableVtbl) [IID_IApplicationLanguagesStatics] {
    fn get_PrimaryLanguageOverride(&self, out: *mut HSTRING) -> HRESULT,
    fn put_PrimaryLanguageOverride(&self, value: HSTRING) -> HRESULT,
    fn get_Languages(&self, out: *mut *mut super::foundation::collections::IVectorView<HString>) -> HRESULT,
    fn get_ManifestLanguages(&self, out: *mut *mut super::foundation::collections::IVectorView<HString>) -> HRESULT
}}
impl IApplicationLanguagesStatics {
    #[inline] pub unsafe fn get_primary_language_override(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_PrimaryLanguageOverride)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_primary_language_override(&self, value: &HStringArg) -> Result<()> {
        let hr = ((*self.lpVtbl).put_PrimaryLanguageOverride)(self as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_languages(&self) -> Result<ComPtr<super::foundation::collections::IVectorView<HString>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Languages)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_manifest_languages(&self) -> Result<ComPtr<super::foundation::collections::IVectorView<HString>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_ManifestLanguages)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
DEFINE_IID!(IID_ICalendar, 3392152093, 34521, 16635, 162, 107, 212, 78, 183, 207, 8, 234);
RT_INTERFACE!{interface ICalendar(ICalendarVtbl): IInspectable(IInspectableVtbl) [IID_ICalendar] {
    fn Clone(&self, out: *mut *mut Calendar) -> HRESULT,
    fn SetToMin(&self) -> HRESULT,
    fn SetToMax(&self) -> HRESULT,
    fn get_Languages(&self, out: *mut *mut super::foundation::collections::IVectorView<HString>) -> HRESULT,
    fn get_NumeralSystem(&self, out: *mut HSTRING) -> HRESULT,
    fn put_NumeralSystem(&self, value: HSTRING) -> HRESULT,
    fn GetCalendarSystem(&self, out: *mut HSTRING) -> HRESULT,
    fn ChangeCalendarSystem(&self, value: HSTRING) -> HRESULT,
    fn GetClock(&self, out: *mut HSTRING) -> HRESULT,
    fn ChangeClock(&self, value: HSTRING) -> HRESULT,
    fn GetDateTime(&self, out: *mut super::foundation::DateTime) -> HRESULT,
    fn SetDateTime(&self, value: super::foundation::DateTime) -> HRESULT,
    fn SetToNow(&self) -> HRESULT,
    fn get_FirstEra(&self, out: *mut i32) -> HRESULT,
    fn get_LastEra(&self, out: *mut i32) -> HRESULT,
    fn get_NumberOfEras(&self, out: *mut i32) -> HRESULT,
    fn get_Era(&self, out: *mut i32) -> HRESULT,
    fn put_Era(&self, value: i32) -> HRESULT,
    fn AddEras(&self, eras: i32) -> HRESULT,
    fn EraAsFullString(&self, out: *mut HSTRING) -> HRESULT,
    fn EraAsString(&self, idealLength: i32, out: *mut HSTRING) -> HRESULT,
    fn get_FirstYearInThisEra(&self, out: *mut i32) -> HRESULT,
    fn get_LastYearInThisEra(&self, out: *mut i32) -> HRESULT,
    fn get_NumberOfYearsInThisEra(&self, out: *mut i32) -> HRESULT,
    fn get_Year(&self, out: *mut i32) -> HRESULT,
    fn put_Year(&self, value: i32) -> HRESULT,
    fn AddYears(&self, years: i32) -> HRESULT,
    fn YearAsString(&self, out: *mut HSTRING) -> HRESULT,
    fn YearAsTruncatedString(&self, remainingDigits: i32, out: *mut HSTRING) -> HRESULT,
    fn YearAsPaddedString(&self, minDigits: i32, out: *mut HSTRING) -> HRESULT,
    fn get_FirstMonthInThisYear(&self, out: *mut i32) -> HRESULT,
    fn get_LastMonthInThisYear(&self, out: *mut i32) -> HRESULT,
    fn get_NumberOfMonthsInThisYear(&self, out: *mut i32) -> HRESULT,
    fn get_Month(&self, out: *mut i32) -> HRESULT,
    fn put_Month(&self, value: i32) -> HRESULT,
    fn AddMonths(&self, months: i32) -> HRESULT,
    fn MonthAsFullString(&self, out: *mut HSTRING) -> HRESULT,
    fn MonthAsString(&self, idealLength: i32, out: *mut HSTRING) -> HRESULT,
    fn MonthAsFullSoloString(&self, out: *mut HSTRING) -> HRESULT,
    fn MonthAsSoloString(&self, idealLength: i32, out: *mut HSTRING) -> HRESULT,
    fn MonthAsNumericString(&self, out: *mut HSTRING) -> HRESULT,
    fn MonthAsPaddedNumericString(&self, minDigits: i32, out: *mut HSTRING) -> HRESULT,
    fn AddWeeks(&self, weeks: i32) -> HRESULT,
    fn get_FirstDayInThisMonth(&self, out: *mut i32) -> HRESULT,
    fn get_LastDayInThisMonth(&self, out: *mut i32) -> HRESULT,
    fn get_NumberOfDaysInThisMonth(&self, out: *mut i32) -> HRESULT,
    fn get_Day(&self, out: *mut i32) -> HRESULT,
    fn put_Day(&self, value: i32) -> HRESULT,
    fn AddDays(&self, days: i32) -> HRESULT,
    fn DayAsString(&self, out: *mut HSTRING) -> HRESULT,
    fn DayAsPaddedString(&self, minDigits: i32, out: *mut HSTRING) -> HRESULT,
    fn get_DayOfWeek(&self, out: *mut DayOfWeek) -> HRESULT,
    fn DayOfWeekAsFullString(&self, out: *mut HSTRING) -> HRESULT,
    fn DayOfWeekAsString(&self, idealLength: i32, out: *mut HSTRING) -> HRESULT,
    fn DayOfWeekAsFullSoloString(&self, out: *mut HSTRING) -> HRESULT,
    fn DayOfWeekAsSoloString(&self, idealLength: i32, out: *mut HSTRING) -> HRESULT,
    fn get_FirstPeriodInThisDay(&self, out: *mut i32) -> HRESULT,
    fn get_LastPeriodInThisDay(&self, out: *mut i32) -> HRESULT,
    fn get_NumberOfPeriodsInThisDay(&self, out: *mut i32) -> HRESULT,
    fn get_Period(&self, out: *mut i32) -> HRESULT,
    fn put_Period(&self, value: i32) -> HRESULT,
    fn AddPeriods(&self, periods: i32) -> HRESULT,
    fn PeriodAsFullString(&self, out: *mut HSTRING) -> HRESULT,
    fn PeriodAsString(&self, idealLength: i32, out: *mut HSTRING) -> HRESULT,
    fn get_FirstHourInThisPeriod(&self, out: *mut i32) -> HRESULT,
    fn get_LastHourInThisPeriod(&self, out: *mut i32) -> HRESULT,
    fn get_NumberOfHoursInThisPeriod(&self, out: *mut i32) -> HRESULT,
    fn get_Hour(&self, out: *mut i32) -> HRESULT,
    fn put_Hour(&self, value: i32) -> HRESULT,
    fn AddHours(&self, hours: i32) -> HRESULT,
    fn HourAsString(&self, out: *mut HSTRING) -> HRESULT,
    fn HourAsPaddedString(&self, minDigits: i32, out: *mut HSTRING) -> HRESULT,
    fn get_Minute(&self, out: *mut i32) -> HRESULT,
    fn put_Minute(&self, value: i32) -> HRESULT,
    fn AddMinutes(&self, minutes: i32) -> HRESULT,
    fn MinuteAsString(&self, out: *mut HSTRING) -> HRESULT,
    fn MinuteAsPaddedString(&self, minDigits: i32, out: *mut HSTRING) -> HRESULT,
    fn get_Second(&self, out: *mut i32) -> HRESULT,
    fn put_Second(&self, value: i32) -> HRESULT,
    fn AddSeconds(&self, seconds: i32) -> HRESULT,
    fn SecondAsString(&self, out: *mut HSTRING) -> HRESULT,
    fn SecondAsPaddedString(&self, minDigits: i32, out: *mut HSTRING) -> HRESULT,
    fn get_Nanosecond(&self, out: *mut i32) -> HRESULT,
    fn put_Nanosecond(&self, value: i32) -> HRESULT,
    fn AddNanoseconds(&self, nanoseconds: i32) -> HRESULT,
    fn NanosecondAsString(&self, out: *mut HSTRING) -> HRESULT,
    fn NanosecondAsPaddedString(&self, minDigits: i32, out: *mut HSTRING) -> HRESULT,
    fn Compare(&self, other: *mut Calendar, out: *mut i32) -> HRESULT,
    fn CompareDateTime(&self, other: super::foundation::DateTime, out: *mut i32) -> HRESULT,
    fn CopyTo(&self, other: *mut Calendar) -> HRESULT,
    fn get_FirstMinuteInThisHour(&self, out: *mut i32) -> HRESULT,
    fn get_LastMinuteInThisHour(&self, out: *mut i32) -> HRESULT,
    fn get_NumberOfMinutesInThisHour(&self, out: *mut i32) -> HRESULT,
    fn get_FirstSecondInThisMinute(&self, out: *mut i32) -> HRESULT,
    fn get_LastSecondInThisMinute(&self, out: *mut i32) -> HRESULT,
    fn get_NumberOfSecondsInThisMinute(&self, out: *mut i32) -> HRESULT,
    fn get_ResolvedLanguage(&self, out: *mut HSTRING) -> HRESULT,
    fn get_IsDaylightSavingTime(&self, out: *mut bool) -> HRESULT
}}
impl ICalendar {
    #[inline] pub unsafe fn clone(&self) -> Result<ComPtr<Calendar>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).Clone)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_to_min(&self) -> Result<()> {
        let hr = ((*self.lpVtbl).SetToMin)(self as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_to_max(&self) -> Result<()> {
        let hr = ((*self.lpVtbl).SetToMax)(self as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_languages(&self) -> Result<ComPtr<super::foundation::collections::IVectorView<HString>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Languages)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_numeral_system(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_NumeralSystem)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_numeral_system(&self, value: &HStringArg) -> Result<()> {
        let hr = ((*self.lpVtbl).put_NumeralSystem)(self as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_calendar_system(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetCalendarSystem)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn change_calendar_system(&self, value: &HStringArg) -> Result<()> {
        let hr = ((*self.lpVtbl).ChangeCalendarSystem)(self as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_clock(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetClock)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn change_clock(&self, value: &HStringArg) -> Result<()> {
        let hr = ((*self.lpVtbl).ChangeClock)(self as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_date_time(&self) -> Result<super::foundation::DateTime> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).GetDateTime)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_date_time(&self, value: super::foundation::DateTime) -> Result<()> {
        let hr = ((*self.lpVtbl).SetDateTime)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_to_now(&self) -> Result<()> {
        let hr = ((*self.lpVtbl).SetToNow)(self as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_first_era(&self) -> Result<i32> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_FirstEra)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_last_era(&self) -> Result<i32> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_LastEra)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_number_of_eras(&self) -> Result<i32> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_NumberOfEras)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_era(&self) -> Result<i32> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_Era)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_era(&self, value: i32) -> Result<()> {
        let hr = ((*self.lpVtbl).put_Era)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn add_eras(&self, eras: i32) -> Result<()> {
        let hr = ((*self.lpVtbl).AddEras)(self as *const _ as *mut _, eras);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn era_as_full_string(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).EraAsFullString)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn era_as_string(&self, idealLength: i32) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).EraAsString)(self as *const _ as *mut _, idealLength, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_first_year_in_this_era(&self) -> Result<i32> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_FirstYearInThisEra)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_last_year_in_this_era(&self) -> Result<i32> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_LastYearInThisEra)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_number_of_years_in_this_era(&self) -> Result<i32> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_NumberOfYearsInThisEra)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_year(&self) -> Result<i32> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_Year)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_year(&self, value: i32) -> Result<()> {
        let hr = ((*self.lpVtbl).put_Year)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn add_years(&self, years: i32) -> Result<()> {
        let hr = ((*self.lpVtbl).AddYears)(self as *const _ as *mut _, years);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn year_as_string(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).YearAsString)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn year_as_truncated_string(&self, remainingDigits: i32) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).YearAsTruncatedString)(self as *const _ as *mut _, remainingDigits, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn year_as_padded_string(&self, minDigits: i32) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).YearAsPaddedString)(self as *const _ as *mut _, minDigits, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_first_month_in_this_year(&self) -> Result<i32> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_FirstMonthInThisYear)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_last_month_in_this_year(&self) -> Result<i32> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_LastMonthInThisYear)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_number_of_months_in_this_year(&self) -> Result<i32> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_NumberOfMonthsInThisYear)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_month(&self) -> Result<i32> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_Month)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_month(&self, value: i32) -> Result<()> {
        let hr = ((*self.lpVtbl).put_Month)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn add_months(&self, months: i32) -> Result<()> {
        let hr = ((*self.lpVtbl).AddMonths)(self as *const _ as *mut _, months);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn month_as_full_string(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).MonthAsFullString)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn month_as_string(&self, idealLength: i32) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).MonthAsString)(self as *const _ as *mut _, idealLength, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn month_as_full_solo_string(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).MonthAsFullSoloString)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn month_as_solo_string(&self, idealLength: i32) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).MonthAsSoloString)(self as *const _ as *mut _, idealLength, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn month_as_numeric_string(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).MonthAsNumericString)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn month_as_padded_numeric_string(&self, minDigits: i32) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).MonthAsPaddedNumericString)(self as *const _ as *mut _, minDigits, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn add_weeks(&self, weeks: i32) -> Result<()> {
        let hr = ((*self.lpVtbl).AddWeeks)(self as *const _ as *mut _, weeks);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_first_day_in_this_month(&self) -> Result<i32> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_FirstDayInThisMonth)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_last_day_in_this_month(&self) -> Result<i32> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_LastDayInThisMonth)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_number_of_days_in_this_month(&self) -> Result<i32> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_NumberOfDaysInThisMonth)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_day(&self) -> Result<i32> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_Day)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_day(&self, value: i32) -> Result<()> {
        let hr = ((*self.lpVtbl).put_Day)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn add_days(&self, days: i32) -> Result<()> {
        let hr = ((*self.lpVtbl).AddDays)(self as *const _ as *mut _, days);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn day_as_string(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).DayAsString)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn day_as_padded_string(&self, minDigits: i32) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).DayAsPaddedString)(self as *const _ as *mut _, minDigits, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_day_of_week(&self) -> Result<DayOfWeek> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_DayOfWeek)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn day_of_week_as_full_string(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).DayOfWeekAsFullString)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn day_of_week_as_string(&self, idealLength: i32) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).DayOfWeekAsString)(self as *const _ as *mut _, idealLength, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn day_of_week_as_full_solo_string(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).DayOfWeekAsFullSoloString)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn day_of_week_as_solo_string(&self, idealLength: i32) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).DayOfWeekAsSoloString)(self as *const _ as *mut _, idealLength, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_first_period_in_this_day(&self) -> Result<i32> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_FirstPeriodInThisDay)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_last_period_in_this_day(&self) -> Result<i32> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_LastPeriodInThisDay)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_number_of_periods_in_this_day(&self) -> Result<i32> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_NumberOfPeriodsInThisDay)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_period(&self) -> Result<i32> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_Period)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_period(&self, value: i32) -> Result<()> {
        let hr = ((*self.lpVtbl).put_Period)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn add_periods(&self, periods: i32) -> Result<()> {
        let hr = ((*self.lpVtbl).AddPeriods)(self as *const _ as *mut _, periods);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn period_as_full_string(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).PeriodAsFullString)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn period_as_string(&self, idealLength: i32) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).PeriodAsString)(self as *const _ as *mut _, idealLength, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_first_hour_in_this_period(&self) -> Result<i32> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_FirstHourInThisPeriod)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_last_hour_in_this_period(&self) -> Result<i32> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_LastHourInThisPeriod)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_number_of_hours_in_this_period(&self) -> Result<i32> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_NumberOfHoursInThisPeriod)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_hour(&self) -> Result<i32> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_Hour)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_hour(&self, value: i32) -> Result<()> {
        let hr = ((*self.lpVtbl).put_Hour)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn add_hours(&self, hours: i32) -> Result<()> {
        let hr = ((*self.lpVtbl).AddHours)(self as *const _ as *mut _, hours);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn hour_as_string(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).HourAsString)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn hour_as_padded_string(&self, minDigits: i32) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).HourAsPaddedString)(self as *const _ as *mut _, minDigits, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_minute(&self) -> Result<i32> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_Minute)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_minute(&self, value: i32) -> Result<()> {
        let hr = ((*self.lpVtbl).put_Minute)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn add_minutes(&self, minutes: i32) -> Result<()> {
        let hr = ((*self.lpVtbl).AddMinutes)(self as *const _ as *mut _, minutes);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn minute_as_string(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).MinuteAsString)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn minute_as_padded_string(&self, minDigits: i32) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).MinuteAsPaddedString)(self as *const _ as *mut _, minDigits, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_second(&self) -> Result<i32> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_Second)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_second(&self, value: i32) -> Result<()> {
        let hr = ((*self.lpVtbl).put_Second)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn add_seconds(&self, seconds: i32) -> Result<()> {
        let hr = ((*self.lpVtbl).AddSeconds)(self as *const _ as *mut _, seconds);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn second_as_string(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).SecondAsString)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn second_as_padded_string(&self, minDigits: i32) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).SecondAsPaddedString)(self as *const _ as *mut _, minDigits, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_nanosecond(&self) -> Result<i32> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_Nanosecond)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_nanosecond(&self, value: i32) -> Result<()> {
        let hr = ((*self.lpVtbl).put_Nanosecond)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn add_nanoseconds(&self, nanoseconds: i32) -> Result<()> {
        let hr = ((*self.lpVtbl).AddNanoseconds)(self as *const _ as *mut _, nanoseconds);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn nanosecond_as_string(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).NanosecondAsString)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn nanosecond_as_padded_string(&self, minDigits: i32) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).NanosecondAsPaddedString)(self as *const _ as *mut _, minDigits, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn compare(&self, other: &Calendar) -> Result<i32> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).Compare)(self as *const _ as *mut _, other as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn compare_date_time(&self, other: super::foundation::DateTime) -> Result<i32> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).CompareDateTime)(self as *const _ as *mut _, other, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn copy_to(&self, other: &Calendar) -> Result<()> {
        let hr = ((*self.lpVtbl).CopyTo)(self as *const _ as *mut _, other as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_first_minute_in_this_hour(&self) -> Result<i32> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_FirstMinuteInThisHour)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_last_minute_in_this_hour(&self) -> Result<i32> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_LastMinuteInThisHour)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_number_of_minutes_in_this_hour(&self) -> Result<i32> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_NumberOfMinutesInThisHour)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_first_second_in_this_minute(&self) -> Result<i32> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_FirstSecondInThisMinute)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_last_second_in_this_minute(&self) -> Result<i32> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_LastSecondInThisMinute)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_number_of_seconds_in_this_minute(&self) -> Result<i32> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_NumberOfSecondsInThisMinute)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_resolved_language(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_ResolvedLanguage)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_is_daylight_saving_time(&self) -> Result<bool> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_IsDaylightSavingTime)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
}
RT_CLASS!{class Calendar: ICalendar}
impl RtActivatable<ICalendarFactory> for Calendar {}
impl RtActivatable<ICalendarFactory2> for Calendar {}
impl RtActivatable<IActivationFactory> for Calendar {}
impl Calendar {
    #[inline] pub fn create_calendar_default_calendar_and_clock(languages: &super::foundation::collections::IIterable<HString>) -> Result<ComPtr<Calendar>> { unsafe {
        <Self as RtActivatable<ICalendarFactory>>::get_activation_factory().create_calendar_default_calendar_and_clock(languages)
    }}
    #[inline] pub fn create_calendar(languages: &super::foundation::collections::IIterable<HString>, calendar: &HStringArg, clock: &HStringArg) -> Result<ComPtr<Calendar>> { unsafe {
        <Self as RtActivatable<ICalendarFactory>>::get_activation_factory().create_calendar(languages, calendar, clock)
    }}
    #[inline] pub fn create_calendar_with_time_zone(languages: &super::foundation::collections::IIterable<HString>, calendar: &HStringArg, clock: &HStringArg, timeZoneId: &HStringArg) -> Result<ComPtr<Calendar>> { unsafe {
        <Self as RtActivatable<ICalendarFactory2>>::get_activation_factory().create_calendar_with_time_zone(languages, calendar, clock, timeZoneId)
    }}
}
DEFINE_CLSID!(Calendar(&[87,105,110,100,111,119,115,46,71,108,111,98,97,108,105,122,97,116,105,111,110,46,67,97,108,101,110,100,97,114,0]) [CLSID_Calendar]);
DEFINE_IID!(IID_ICalendarFactory, 2213905426, 58731, 19573, 166, 110, 15, 99, 213, 119, 88, 166);
RT_INTERFACE!{static interface ICalendarFactory(ICalendarFactoryVtbl): IInspectable(IInspectableVtbl) [IID_ICalendarFactory] {
    fn CreateCalendarDefaultCalendarAndClock(&self, languages: *mut super::foundation::collections::IIterable<HString>, out: *mut *mut Calendar) -> HRESULT,
    fn CreateCalendar(&self, languages: *mut super::foundation::collections::IIterable<HString>, calendar: HSTRING, clock: HSTRING, out: *mut *mut Calendar) -> HRESULT
}}
impl ICalendarFactory {
    #[inline] pub unsafe fn create_calendar_default_calendar_and_clock(&self, languages: &super::foundation::collections::IIterable<HString>) -> Result<ComPtr<Calendar>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).CreateCalendarDefaultCalendarAndClock)(self as *const _ as *mut _, languages as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn create_calendar(&self, languages: &super::foundation::collections::IIterable<HString>, calendar: &HStringArg, clock: &HStringArg) -> Result<ComPtr<Calendar>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).CreateCalendar)(self as *const _ as *mut _, languages as *const _ as *mut _, calendar.get(), clock.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
DEFINE_IID!(IID_ICalendarFactory2, 3024828300, 51838, 17808, 158, 114, 234, 43, 236, 26, 81, 21);
RT_INTERFACE!{static interface ICalendarFactory2(ICalendarFactory2Vtbl): IInspectable(IInspectableVtbl) [IID_ICalendarFactory2] {
    fn CreateCalendarWithTimeZone(&self, languages: *mut super::foundation::collections::IIterable<HString>, calendar: HSTRING, clock: HSTRING, timeZoneId: HSTRING, out: *mut *mut Calendar) -> HRESULT
}}
impl ICalendarFactory2 {
    #[inline] pub unsafe fn create_calendar_with_time_zone(&self, languages: &super::foundation::collections::IIterable<HString>, calendar: &HStringArg, clock: &HStringArg, timeZoneId: &HStringArg) -> Result<ComPtr<Calendar>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).CreateCalendarWithTimeZone)(self as *const _ as *mut _, languages as *const _ as *mut _, calendar.get(), clock.get(), timeZoneId.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
RT_CLASS!{static class CalendarIdentifiers}
impl RtActivatable<ICalendarIdentifiersStatics> for CalendarIdentifiers {}
impl RtActivatable<ICalendarIdentifiersStatics2> for CalendarIdentifiers {}
impl RtActivatable<ICalendarIdentifiersStatics3> for CalendarIdentifiers {}
impl CalendarIdentifiers {
    #[inline] pub fn get_gregorian() -> Result<HString> { unsafe {
        <Self as RtActivatable<ICalendarIdentifiersStatics>>::get_activation_factory().get_gregorian()
    }}
    #[inline] pub fn get_hebrew() -> Result<HString> { unsafe {
        <Self as RtActivatable<ICalendarIdentifiersStatics>>::get_activation_factory().get_hebrew()
    }}
    #[inline] pub fn get_hijri() -> Result<HString> { unsafe {
        <Self as RtActivatable<ICalendarIdentifiersStatics>>::get_activation_factory().get_hijri()
    }}
    #[inline] pub fn get_japanese() -> Result<HString> { unsafe {
        <Self as RtActivatable<ICalendarIdentifiersStatics>>::get_activation_factory().get_japanese()
    }}
    #[inline] pub fn get_julian() -> Result<HString> { unsafe {
        <Self as RtActivatable<ICalendarIdentifiersStatics>>::get_activation_factory().get_julian()
    }}
    #[inline] pub fn get_korean() -> Result<HString> { unsafe {
        <Self as RtActivatable<ICalendarIdentifiersStatics>>::get_activation_factory().get_korean()
    }}
    #[inline] pub fn get_taiwan() -> Result<HString> { unsafe {
        <Self as RtActivatable<ICalendarIdentifiersStatics>>::get_activation_factory().get_taiwan()
    }}
    #[inline] pub fn get_thai() -> Result<HString> { unsafe {
        <Self as RtActivatable<ICalendarIdentifiersStatics>>::get_activation_factory().get_thai()
    }}
    #[inline] pub fn get_um_al_qura() -> Result<HString> { unsafe {
        <Self as RtActivatable<ICalendarIdentifiersStatics>>::get_activation_factory().get_um_al_qura()
    }}
    #[inline] pub fn get_persian() -> Result<HString> { unsafe {
        <Self as RtActivatable<ICalendarIdentifiersStatics2>>::get_activation_factory().get_persian()
    }}
    #[inline] pub fn get_chinese_lunar() -> Result<HString> { unsafe {
        <Self as RtActivatable<ICalendarIdentifiersStatics3>>::get_activation_factory().get_chinese_lunar()
    }}
    #[inline] pub fn get_japanese_lunar() -> Result<HString> { unsafe {
        <Self as RtActivatable<ICalendarIdentifiersStatics3>>::get_activation_factory().get_japanese_lunar()
    }}
    #[inline] pub fn get_korean_lunar() -> Result<HString> { unsafe {
        <Self as RtActivatable<ICalendarIdentifiersStatics3>>::get_activation_factory().get_korean_lunar()
    }}
    #[inline] pub fn get_taiwan_lunar() -> Result<HString> { unsafe {
        <Self as RtActivatable<ICalendarIdentifiersStatics3>>::get_activation_factory().get_taiwan_lunar()
    }}
    #[inline] pub fn get_vietnamese_lunar() -> Result<HString> { unsafe {
        <Self as RtActivatable<ICalendarIdentifiersStatics3>>::get_activation_factory().get_vietnamese_lunar()
    }}
}
DEFINE_CLSID!(CalendarIdentifiers(&[87,105,110,100,111,119,115,46,71,108,111,98,97,108,105,122,97,116,105,111,110,46,67,97,108,101,110,100,97,114,73,100,101,110,116,105,102,105,101,114,115,0]) [CLSID_CalendarIdentifiers]);
DEFINE_IID!(IID_ICalendarIdentifiersStatics, 2154119016, 11442, 19487, 181, 144, 240, 245, 43, 244, 253, 26);
RT_INTERFACE!{static interface ICalendarIdentifiersStatics(ICalendarIdentifiersStaticsVtbl): IInspectable(IInspectableVtbl) [IID_ICalendarIdentifiersStatics] {
    fn get_Gregorian(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Hebrew(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Hijri(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Japanese(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Julian(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Korean(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Taiwan(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Thai(&self, out: *mut HSTRING) -> HRESULT,
    fn get_UmAlQura(&self, out: *mut HSTRING) -> HRESULT
}}
impl ICalendarIdentifiersStatics {
    #[inline] pub unsafe fn get_gregorian(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Gregorian)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_hebrew(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Hebrew)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_hijri(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Hijri)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_japanese(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Japanese)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_julian(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Julian)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_korean(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Korean)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_taiwan(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Taiwan)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_thai(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Thai)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_um_al_qura(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_UmAlQura)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
}
DEFINE_IID!(IID_ICalendarIdentifiersStatics2, 2113197192, 24528, 17063, 149, 181, 125, 152, 216, 35, 7, 95);
RT_INTERFACE!{static interface ICalendarIdentifiersStatics2(ICalendarIdentifiersStatics2Vtbl): IInspectable(IInspectableVtbl) [IID_ICalendarIdentifiersStatics2] {
    fn get_Persian(&self, out: *mut HSTRING) -> HRESULT
}}
impl ICalendarIdentifiersStatics2 {
    #[inline] pub unsafe fn get_persian(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Persian)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
}
DEFINE_IID!(IID_ICalendarIdentifiersStatics3, 740447267, 8109, 16576, 147, 52, 168, 235, 144, 219, 4, 245);
RT_INTERFACE!{static interface ICalendarIdentifiersStatics3(ICalendarIdentifiersStatics3Vtbl): IInspectable(IInspectableVtbl) [IID_ICalendarIdentifiersStatics3] {
    fn get_ChineseLunar(&self, out: *mut HSTRING) -> HRESULT,
    fn get_JapaneseLunar(&self, out: *mut HSTRING) -> HRESULT,
    fn get_KoreanLunar(&self, out: *mut HSTRING) -> HRESULT,
    fn get_TaiwanLunar(&self, out: *mut HSTRING) -> HRESULT,
    fn get_VietnameseLunar(&self, out: *mut HSTRING) -> HRESULT
}}
impl ICalendarIdentifiersStatics3 {
    #[inline] pub unsafe fn get_chinese_lunar(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_ChineseLunar)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_japanese_lunar(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_JapaneseLunar)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_korean_lunar(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_KoreanLunar)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_taiwan_lunar(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_TaiwanLunar)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_vietnamese_lunar(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_VietnameseLunar)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
}
RT_CLASS!{static class ClockIdentifiers}
impl RtActivatable<IClockIdentifiersStatics> for ClockIdentifiers {}
impl ClockIdentifiers {
    #[inline] pub fn get_twelve_hour() -> Result<HString> { unsafe {
        <Self as RtActivatable<IClockIdentifiersStatics>>::get_activation_factory().get_twelve_hour()
    }}
    #[inline] pub fn get_twenty_four_hour() -> Result<HString> { unsafe {
        <Self as RtActivatable<IClockIdentifiersStatics>>::get_activation_factory().get_twenty_four_hour()
    }}
}
DEFINE_CLSID!(ClockIdentifiers(&[87,105,110,100,111,119,115,46,71,108,111,98,97,108,105,122,97,116,105,111,110,46,67,108,111,99,107,73,100,101,110,116,105,102,105,101,114,115,0]) [CLSID_ClockIdentifiers]);
DEFINE_IID!(IID_IClockIdentifiersStatics, 1379403195, 4844, 20355, 188, 49, 177, 180, 55, 107, 8, 8);
RT_INTERFACE!{static interface IClockIdentifiersStatics(IClockIdentifiersStaticsVtbl): IInspectable(IInspectableVtbl) [IID_IClockIdentifiersStatics] {
    fn get_TwelveHour(&self, out: *mut HSTRING) -> HRESULT,
    fn get_TwentyFourHour(&self, out: *mut HSTRING) -> HRESULT
}}
impl IClockIdentifiersStatics {
    #[inline] pub unsafe fn get_twelve_hour(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_TwelveHour)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_twenty_four_hour(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_TwentyFourHour)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
}
RT_CLASS!{static class CurrencyIdentifiers}
impl RtActivatable<ICurrencyIdentifiersStatics> for CurrencyIdentifiers {}
impl RtActivatable<ICurrencyIdentifiersStatics2> for CurrencyIdentifiers {}
impl CurrencyIdentifiers {
    #[inline] pub fn get_aed() -> Result<HString> { unsafe {
        <Self as RtActivatable<ICurrencyIdentifiersStatics>>::get_activation_factory().get_aed()
    }}
    #[inline] pub fn get_afn() -> Result<HString> { unsafe {
        <Self as RtActivatable<ICurrencyIdentifiersStatics>>::get_activation_factory().get_afn()
    }}
    #[inline] pub fn get_all() -> Result<HString> { unsafe {
        <Self as RtActivatable<ICurrencyIdentifiersStatics>>::get_activation_factory().get_all()
    }}
    #[inline] pub fn get_amd() -> Result<HString> { unsafe {
        <Self as RtActivatable<ICurrencyIdentifiersStatics>>::get_activation_factory().get_amd()
    }}
    #[inline] pub fn get_ang() -> Result<HString> { unsafe {
        <Self as RtActivatable<ICurrencyIdentifiersStatics>>::get_activation_factory().get_ang()
    }}
    #[inline] pub fn get_aoa() -> Result<HString> { unsafe {
        <Self as RtActivatable<ICurrencyIdentifiersStatics>>::get_activation_factory().get_aoa()
    }}
    #[inline] pub fn get_ars() -> Result<HString> { unsafe {
        <Self as RtActivatable<ICurrencyIdentifiersStatics>>::get_activation_factory().get_ars()
    }}
    #[inline] pub fn get_aud() -> Result<HString> { unsafe {
        <Self as RtActivatable<ICurrencyIdentifiersStatics>>::get_activation_factory().get_aud()
    }}
    #[inline] pub fn get_awg() -> Result<HString> { unsafe {
        <Self as RtActivatable<ICurrencyIdentifiersStatics>>::get_activation_factory().get_awg()
    }}
    #[inline] pub fn get_azn() -> Result<HString> { unsafe {
        <Self as RtActivatable<ICurrencyIdentifiersStatics>>::get_activation_factory().get_azn()
    }}
    #[inline] pub fn get_bam() -> Result<HString> { unsafe {
        <Self as RtActivatable<ICurrencyIdentifiersStatics>>::get_activation_factory().get_bam()
    }}
    #[inline] pub fn get_bbd() -> Result<HString> { unsafe {
        <Self as RtActivatable<ICurrencyIdentifiersStatics>>::get_activation_factory().get_bbd()
    }}
    #[inline] pub fn get_bdt() -> Result<HString> { unsafe {
        <Self as RtActivatable<ICurrencyIdentifiersStatics>>::get_activation_factory().get_bdt()
    }}
    #[inline] pub fn get_bgn() -> Result<HString> { unsafe {
        <Self as RtActivatable<ICurrencyIdentifiersStatics>>::get_activation_factory().get_bgn()
    }}
    #[inline] pub fn get_bhd() -> Result<HString> { unsafe {
        <Self as RtActivatable<ICurrencyIdentifiersStatics>>::get_activation_factory().get_bhd()
    }}
    #[inline] pub fn get_bif() -> Result<HString> { unsafe {
        <Self as RtActivatable<ICurrencyIdentifiersStatics>>::get_activation_factory().get_bif()
    }}
    #[inline] pub fn get_bmd() -> Result<HString> { unsafe {
        <Self as RtActivatable<ICurrencyIdentifiersStatics>>::get_activation_factory().get_bmd()
    }}
    #[inline] pub fn get_bnd() -> Result<HString> { unsafe {
        <Self as RtActivatable<ICurrencyIdentifiersStatics>>::get_activation_factory().get_bnd()
    }}
    #[inline] pub fn get_bob() -> Result<HString> { unsafe {
        <Self as RtActivatable<ICurrencyIdentifiersStatics>>::get_activation_factory().get_bob()
    }}
    #[inline] pub fn get_brl() -> Result<HString> { unsafe {
        <Self as RtActivatable<ICurrencyIdentifiersStatics>>::get_activation_factory().get_brl()
    }}
    #[inline] pub fn get_bsd() -> Result<HString> { unsafe {
        <Self as RtActivatable<ICurrencyIdentifiersStatics>>::get_activation_factory().get_bsd()
    }}
    #[inline] pub fn get_btn() -> Result<HString> { unsafe {
        <Self as RtActivatable<ICurrencyIdentifiersStatics>>::get_activation_factory().get_btn()
    }}
    #[inline] pub fn get_bwp() -> Result<HString> { unsafe {
        <Self as RtActivatable<ICurrencyIdentifiersStatics>>::get_activation_factory().get_bwp()
    }}
    #[inline] pub fn get_byr() -> Result<HString> { unsafe {
        <Self as RtActivatable<ICurrencyIdentifiersStatics>>::get_activation_factory().get_byr()
    }}
    #[inline] pub fn get_bzd() -> Result<HString> { unsafe {
        <Self as RtActivatable<ICurrencyIdentifiersStatics>>::get_activation_factory().get_bzd()
    }}
    #[inline] pub fn get_cad() -> Result<HString> { unsafe {
        <Self as RtActivatable<ICurrencyIdentifiersStatics>>::get_activation_factory().get_cad()
    }}
    #[inline] pub fn get_cdf() -> Result<HString> { unsafe {
        <Self as RtActivatable<ICurrencyIdentifiersStatics>>::get_activation_factory().get_cdf()
    }}
    #[inline] pub fn get_chf() -> Result<HString> { unsafe {
        <Self as RtActivatable<ICurrencyIdentifiersStatics>>::get_activation_factory().get_chf()
    }}
    #[inline] pub fn get_clp() -> Result<HString> { unsafe {
        <Self as RtActivatable<ICurrencyIdentifiersStatics>>::get_activation_factory().get_clp()
    }}
    #[inline] pub fn get_cny() -> Result<HString> { unsafe {
        <Self as RtActivatable<ICurrencyIdentifiersStatics>>::get_activation_factory().get_cny()
    }}
    #[inline] pub fn get_cop() -> Result<HString> { unsafe {
        <Self as RtActivatable<ICurrencyIdentifiersStatics>>::get_activation_factory().get_cop()
    }}
    #[inline] pub fn get_crc() -> Result<HString> { unsafe {
        <Self as RtActivatable<ICurrencyIdentifiersStatics>>::get_activation_factory().get_crc()
    }}
    #[inline] pub fn get_cup() -> Result<HString> { unsafe {
        <Self as RtActivatable<ICurrencyIdentifiersStatics>>::get_activation_factory().get_cup()
    }}
    #[inline] pub fn get_cve() -> Result<HString> { unsafe {
        <Self as RtActivatable<ICurrencyIdentifiersStatics>>::get_activation_factory().get_cve()
    }}
    #[inline] pub fn get_czk() -> Result<HString> { unsafe {
        <Self as RtActivatable<ICurrencyIdentifiersStatics>>::get_activation_factory().get_czk()
    }}
    #[inline] pub fn get_djf() -> Result<HString> { unsafe {
        <Self as RtActivatable<ICurrencyIdentifiersStatics>>::get_activation_factory().get_djf()
    }}
    #[inline] pub fn get_dkk() -> Result<HString> { unsafe {
        <Self as RtActivatable<ICurrencyIdentifiersStatics>>::get_activation_factory().get_dkk()
    }}
    #[inline] pub fn get_dop() -> Result<HString> { unsafe {
        <Self as RtActivatable<ICurrencyIdentifiersStatics>>::get_activation_factory().get_dop()
    }}
    #[inline] pub fn get_dzd() -> Result<HString> { unsafe {
        <Self as RtActivatable<ICurrencyIdentifiersStatics>>::get_activation_factory().get_dzd()
    }}
    #[inline] pub fn get_egp() -> Result<HString> { unsafe {
        <Self as RtActivatable<ICurrencyIdentifiersStatics>>::get_activation_factory().get_egp()
    }}
    #[inline] pub fn get_ern() -> Result<HString> { unsafe {
        <Self as RtActivatable<ICurrencyIdentifiersStatics>>::get_activation_factory().get_ern()
    }}
    #[inline] pub fn get_etb() -> Result<HString> { unsafe {
        <Self as RtActivatable<ICurrencyIdentifiersStatics>>::get_activation_factory().get_etb()
    }}
    #[inline] pub fn get_eur() -> Result<HString> { unsafe {
        <Self as RtActivatable<ICurrencyIdentifiersStatics>>::get_activation_factory().get_eur()
    }}
    #[inline] pub fn get_fjd() -> Result<HString> { unsafe {
        <Self as RtActivatable<ICurrencyIdentifiersStatics>>::get_activation_factory().get_fjd()
    }}
    #[inline] pub fn get_fkp() -> Result<HString> { unsafe {
        <Self as RtActivatable<ICurrencyIdentifiersStatics>>::get_activation_factory().get_fkp()
    }}
    #[inline] pub fn get_gbp() -> Result<HString> { unsafe {
        <Self as RtActivatable<ICurrencyIdentifiersStatics>>::get_activation_factory().get_gbp()
    }}
    #[inline] pub fn get_gel() -> Result<HString> { unsafe {
        <Self as RtActivatable<ICurrencyIdentifiersStatics>>::get_activation_factory().get_gel()
    }}
    #[inline] pub fn get_ghs() -> Result<HString> { unsafe {
        <Self as RtActivatable<ICurrencyIdentifiersStatics>>::get_activation_factory().get_ghs()
    }}
    #[inline] pub fn get_gip() -> Result<HString> { unsafe {
        <Self as RtActivatable<ICurrencyIdentifiersStatics>>::get_activation_factory().get_gip()
    }}
    #[inline] pub fn get_gmd() -> Result<HString> { unsafe {
        <Self as RtActivatable<ICurrencyIdentifiersStatics>>::get_activation_factory().get_gmd()
    }}
    #[inline] pub fn get_gnf() -> Result<HString> { unsafe {
        <Self as RtActivatable<ICurrencyIdentifiersStatics>>::get_activation_factory().get_gnf()
    }}
    #[inline] pub fn get_gtq() -> Result<HString> { unsafe {
        <Self as RtActivatable<ICurrencyIdentifiersStatics>>::get_activation_factory().get_gtq()
    }}
    #[inline] pub fn get_gyd() -> Result<HString> { unsafe {
        <Self as RtActivatable<ICurrencyIdentifiersStatics>>::get_activation_factory().get_gyd()
    }}
    #[inline] pub fn get_hkd() -> Result<HString> { unsafe {
        <Self as RtActivatable<ICurrencyIdentifiersStatics>>::get_activation_factory().get_hkd()
    }}
    #[inline] pub fn get_hnl() -> Result<HString> { unsafe {
        <Self as RtActivatable<ICurrencyIdentifiersStatics>>::get_activation_factory().get_hnl()
    }}
    #[inline] pub fn get_hrk() -> Result<HString> { unsafe {
        <Self as RtActivatable<ICurrencyIdentifiersStatics>>::get_activation_factory().get_hrk()
    }}
    #[inline] pub fn get_htg() -> Result<HString> { unsafe {
        <Self as RtActivatable<ICurrencyIdentifiersStatics>>::get_activation_factory().get_htg()
    }}
    #[inline] pub fn get_huf() -> Result<HString> { unsafe {
        <Self as RtActivatable<ICurrencyIdentifiersStatics>>::get_activation_factory().get_huf()
    }}
    #[inline] pub fn get_idr() -> Result<HString> { unsafe {
        <Self as RtActivatable<ICurrencyIdentifiersStatics>>::get_activation_factory().get_idr()
    }}
    #[inline] pub fn get_ils() -> Result<HString> { unsafe {
        <Self as RtActivatable<ICurrencyIdentifiersStatics>>::get_activation_factory().get_ils()
    }}
    #[inline] pub fn get_inr() -> Result<HString> { unsafe {
        <Self as RtActivatable<ICurrencyIdentifiersStatics>>::get_activation_factory().get_inr()
    }}
    #[inline] pub fn get_iqd() -> Result<HString> { unsafe {
        <Self as RtActivatable<ICurrencyIdentifiersStatics>>::get_activation_factory().get_iqd()
    }}
    #[inline] pub fn get_irr() -> Result<HString> { unsafe {
        <Self as RtActivatable<ICurrencyIdentifiersStatics>>::get_activation_factory().get_irr()
    }}
    #[inline] pub fn get_isk() -> Result<HString> { unsafe {
        <Self as RtActivatable<ICurrencyIdentifiersStatics>>::get_activation_factory().get_isk()
    }}
    #[inline] pub fn get_jmd() -> Result<HString> { unsafe {
        <Self as RtActivatable<ICurrencyIdentifiersStatics>>::get_activation_factory().get_jmd()
    }}
    #[inline] pub fn get_jod() -> Result<HString> { unsafe {
        <Self as RtActivatable<ICurrencyIdentifiersStatics>>::get_activation_factory().get_jod()
    }}
    #[inline] pub fn get_jpy() -> Result<HString> { unsafe {
        <Self as RtActivatable<ICurrencyIdentifiersStatics>>::get_activation_factory().get_jpy()
    }}
    #[inline] pub fn get_kes() -> Result<HString> { unsafe {
        <Self as RtActivatable<ICurrencyIdentifiersStatics>>::get_activation_factory().get_kes()
    }}
    #[inline] pub fn get_kgs() -> Result<HString> { unsafe {
        <Self as RtActivatable<ICurrencyIdentifiersStatics>>::get_activation_factory().get_kgs()
    }}
    #[inline] pub fn get_khr() -> Result<HString> { unsafe {
        <Self as RtActivatable<ICurrencyIdentifiersStatics>>::get_activation_factory().get_khr()
    }}
    #[inline] pub fn get_kmf() -> Result<HString> { unsafe {
        <Self as RtActivatable<ICurrencyIdentifiersStatics>>::get_activation_factory().get_kmf()
    }}
    #[inline] pub fn get_kpw() -> Result<HString> { unsafe {
        <Self as RtActivatable<ICurrencyIdentifiersStatics>>::get_activation_factory().get_kpw()
    }}
    #[inline] pub fn get_krw() -> Result<HString> { unsafe {
        <Self as RtActivatable<ICurrencyIdentifiersStatics>>::get_activation_factory().get_krw()
    }}
    #[inline] pub fn get_kwd() -> Result<HString> { unsafe {
        <Self as RtActivatable<ICurrencyIdentifiersStatics>>::get_activation_factory().get_kwd()
    }}
    #[inline] pub fn get_kyd() -> Result<HString> { unsafe {
        <Self as RtActivatable<ICurrencyIdentifiersStatics>>::get_activation_factory().get_kyd()
    }}
    #[inline] pub fn get_kzt() -> Result<HString> { unsafe {
        <Self as RtActivatable<ICurrencyIdentifiersStatics>>::get_activation_factory().get_kzt()
    }}
    #[inline] pub fn get_lak() -> Result<HString> { unsafe {
        <Self as RtActivatable<ICurrencyIdentifiersStatics>>::get_activation_factory().get_lak()
    }}
    #[inline] pub fn get_lbp() -> Result<HString> { unsafe {
        <Self as RtActivatable<ICurrencyIdentifiersStatics>>::get_activation_factory().get_lbp()
    }}
    #[inline] pub fn get_lkr() -> Result<HString> { unsafe {
        <Self as RtActivatable<ICurrencyIdentifiersStatics>>::get_activation_factory().get_lkr()
    }}
    #[inline] pub fn get_lrd() -> Result<HString> { unsafe {
        <Self as RtActivatable<ICurrencyIdentifiersStatics>>::get_activation_factory().get_lrd()
    }}
    #[inline] pub fn get_lsl() -> Result<HString> { unsafe {
        <Self as RtActivatable<ICurrencyIdentifiersStatics>>::get_activation_factory().get_lsl()
    }}
    #[inline] pub fn get_ltl() -> Result<HString> { unsafe {
        <Self as RtActivatable<ICurrencyIdentifiersStatics>>::get_activation_factory().get_ltl()
    }}
    #[inline] pub fn get_lvl() -> Result<HString> { unsafe {
        <Self as RtActivatable<ICurrencyIdentifiersStatics>>::get_activation_factory().get_lvl()
    }}
    #[inline] pub fn get_lyd() -> Result<HString> { unsafe {
        <Self as RtActivatable<ICurrencyIdentifiersStatics>>::get_activation_factory().get_lyd()
    }}
    #[inline] pub fn get_mad() -> Result<HString> { unsafe {
        <Self as RtActivatable<ICurrencyIdentifiersStatics>>::get_activation_factory().get_mad()
    }}
    #[inline] pub fn get_mdl() -> Result<HString> { unsafe {
        <Self as RtActivatable<ICurrencyIdentifiersStatics>>::get_activation_factory().get_mdl()
    }}
    #[inline] pub fn get_mga() -> Result<HString> { unsafe {
        <Self as RtActivatable<ICurrencyIdentifiersStatics>>::get_activation_factory().get_mga()
    }}
    #[inline] pub fn get_mkd() -> Result<HString> { unsafe {
        <Self as RtActivatable<ICurrencyIdentifiersStatics>>::get_activation_factory().get_mkd()
    }}
    #[inline] pub fn get_mmk() -> Result<HString> { unsafe {
        <Self as RtActivatable<ICurrencyIdentifiersStatics>>::get_activation_factory().get_mmk()
    }}
    #[inline] pub fn get_mnt() -> Result<HString> { unsafe {
        <Self as RtActivatable<ICurrencyIdentifiersStatics>>::get_activation_factory().get_mnt()
    }}
    #[inline] pub fn get_mop() -> Result<HString> { unsafe {
        <Self as RtActivatable<ICurrencyIdentifiersStatics>>::get_activation_factory().get_mop()
    }}
    #[inline] pub fn get_mro() -> Result<HString> { unsafe {
        <Self as RtActivatable<ICurrencyIdentifiersStatics>>::get_activation_factory().get_mro()
    }}
    #[inline] pub fn get_mur() -> Result<HString> { unsafe {
        <Self as RtActivatable<ICurrencyIdentifiersStatics>>::get_activation_factory().get_mur()
    }}
    #[inline] pub fn get_mvr() -> Result<HString> { unsafe {
        <Self as RtActivatable<ICurrencyIdentifiersStatics>>::get_activation_factory().get_mvr()
    }}
    #[inline] pub fn get_mwk() -> Result<HString> { unsafe {
        <Self as RtActivatable<ICurrencyIdentifiersStatics>>::get_activation_factory().get_mwk()
    }}
    #[inline] pub fn get_mxn() -> Result<HString> { unsafe {
        <Self as RtActivatable<ICurrencyIdentifiersStatics>>::get_activation_factory().get_mxn()
    }}
    #[inline] pub fn get_myr() -> Result<HString> { unsafe {
        <Self as RtActivatable<ICurrencyIdentifiersStatics>>::get_activation_factory().get_myr()
    }}
    #[inline] pub fn get_mzn() -> Result<HString> { unsafe {
        <Self as RtActivatable<ICurrencyIdentifiersStatics>>::get_activation_factory().get_mzn()
    }}
    #[inline] pub fn get_nad() -> Result<HString> { unsafe {
        <Self as RtActivatable<ICurrencyIdentifiersStatics>>::get_activation_factory().get_nad()
    }}
    #[inline] pub fn get_ngn() -> Result<HString> { unsafe {
        <Self as RtActivatable<ICurrencyIdentifiersStatics>>::get_activation_factory().get_ngn()
    }}
    #[inline] pub fn get_nio() -> Result<HString> { unsafe {
        <Self as RtActivatable<ICurrencyIdentifiersStatics>>::get_activation_factory().get_nio()
    }}
    #[inline] pub fn get_nok() -> Result<HString> { unsafe {
        <Self as RtActivatable<ICurrencyIdentifiersStatics>>::get_activation_factory().get_nok()
    }}
    #[inline] pub fn get_npr() -> Result<HString> { unsafe {
        <Self as RtActivatable<ICurrencyIdentifiersStatics>>::get_activation_factory().get_npr()
    }}
    #[inline] pub fn get_nzd() -> Result<HString> { unsafe {
        <Self as RtActivatable<ICurrencyIdentifiersStatics>>::get_activation_factory().get_nzd()
    }}
    #[inline] pub fn get_omr() -> Result<HString> { unsafe {
        <Self as RtActivatable<ICurrencyIdentifiersStatics>>::get_activation_factory().get_omr()
    }}
    #[inline] pub fn get_pab() -> Result<HString> { unsafe {
        <Self as RtActivatable<ICurrencyIdentifiersStatics>>::get_activation_factory().get_pab()
    }}
    #[inline] pub fn get_pen() -> Result<HString> { unsafe {
        <Self as RtActivatable<ICurrencyIdentifiersStatics>>::get_activation_factory().get_pen()
    }}
    #[inline] pub fn get_pgk() -> Result<HString> { unsafe {
        <Self as RtActivatable<ICurrencyIdentifiersStatics>>::get_activation_factory().get_pgk()
    }}
    #[inline] pub fn get_php() -> Result<HString> { unsafe {
        <Self as RtActivatable<ICurrencyIdentifiersStatics>>::get_activation_factory().get_php()
    }}
    #[inline] pub fn get_pkr() -> Result<HString> { unsafe {
        <Self as RtActivatable<ICurrencyIdentifiersStatics>>::get_activation_factory().get_pkr()
    }}
    #[inline] pub fn get_pln() -> Result<HString> { unsafe {
        <Self as RtActivatable<ICurrencyIdentifiersStatics>>::get_activation_factory().get_pln()
    }}
    #[inline] pub fn get_pyg() -> Result<HString> { unsafe {
        <Self as RtActivatable<ICurrencyIdentifiersStatics>>::get_activation_factory().get_pyg()
    }}
    #[inline] pub fn get_qar() -> Result<HString> { unsafe {
        <Self as RtActivatable<ICurrencyIdentifiersStatics>>::get_activation_factory().get_qar()
    }}
    #[inline] pub fn get_ron() -> Result<HString> { unsafe {
        <Self as RtActivatable<ICurrencyIdentifiersStatics>>::get_activation_factory().get_ron()
    }}
    #[inline] pub fn get_rsd() -> Result<HString> { unsafe {
        <Self as RtActivatable<ICurrencyIdentifiersStatics>>::get_activation_factory().get_rsd()
    }}
    #[inline] pub fn get_rub() -> Result<HString> { unsafe {
        <Self as RtActivatable<ICurrencyIdentifiersStatics>>::get_activation_factory().get_rub()
    }}
    #[inline] pub fn get_rwf() -> Result<HString> { unsafe {
        <Self as RtActivatable<ICurrencyIdentifiersStatics>>::get_activation_factory().get_rwf()
    }}
    #[inline] pub fn get_sar() -> Result<HString> { unsafe {
        <Self as RtActivatable<ICurrencyIdentifiersStatics>>::get_activation_factory().get_sar()
    }}
    #[inline] pub fn get_sbd() -> Result<HString> { unsafe {
        <Self as RtActivatable<ICurrencyIdentifiersStatics>>::get_activation_factory().get_sbd()
    }}
    #[inline] pub fn get_scr() -> Result<HString> { unsafe {
        <Self as RtActivatable<ICurrencyIdentifiersStatics>>::get_activation_factory().get_scr()
    }}
    #[inline] pub fn get_sdg() -> Result<HString> { unsafe {
        <Self as RtActivatable<ICurrencyIdentifiersStatics>>::get_activation_factory().get_sdg()
    }}
    #[inline] pub fn get_sek() -> Result<HString> { unsafe {
        <Self as RtActivatable<ICurrencyIdentifiersStatics>>::get_activation_factory().get_sek()
    }}
    #[inline] pub fn get_sgd() -> Result<HString> { unsafe {
        <Self as RtActivatable<ICurrencyIdentifiersStatics>>::get_activation_factory().get_sgd()
    }}
    #[inline] pub fn get_shp() -> Result<HString> { unsafe {
        <Self as RtActivatable<ICurrencyIdentifiersStatics>>::get_activation_factory().get_shp()
    }}
    #[inline] pub fn get_sll() -> Result<HString> { unsafe {
        <Self as RtActivatable<ICurrencyIdentifiersStatics>>::get_activation_factory().get_sll()
    }}
    #[inline] pub fn get_sos() -> Result<HString> { unsafe {
        <Self as RtActivatable<ICurrencyIdentifiersStatics>>::get_activation_factory().get_sos()
    }}
    #[inline] pub fn get_srd() -> Result<HString> { unsafe {
        <Self as RtActivatable<ICurrencyIdentifiersStatics>>::get_activation_factory().get_srd()
    }}
    #[inline] pub fn get_std() -> Result<HString> { unsafe {
        <Self as RtActivatable<ICurrencyIdentifiersStatics>>::get_activation_factory().get_std()
    }}
    #[inline] pub fn get_syp() -> Result<HString> { unsafe {
        <Self as RtActivatable<ICurrencyIdentifiersStatics>>::get_activation_factory().get_syp()
    }}
    #[inline] pub fn get_szl() -> Result<HString> { unsafe {
        <Self as RtActivatable<ICurrencyIdentifiersStatics>>::get_activation_factory().get_szl()
    }}
    #[inline] pub fn get_thb() -> Result<HString> { unsafe {
        <Self as RtActivatable<ICurrencyIdentifiersStatics>>::get_activation_factory().get_thb()
    }}
    #[inline] pub fn get_tjs() -> Result<HString> { unsafe {
        <Self as RtActivatable<ICurrencyIdentifiersStatics>>::get_activation_factory().get_tjs()
    }}
    #[inline] pub fn get_tmt() -> Result<HString> { unsafe {
        <Self as RtActivatable<ICurrencyIdentifiersStatics>>::get_activation_factory().get_tmt()
    }}
    #[inline] pub fn get_tnd() -> Result<HString> { unsafe {
        <Self as RtActivatable<ICurrencyIdentifiersStatics>>::get_activation_factory().get_tnd()
    }}
    #[inline] pub fn get_top() -> Result<HString> { unsafe {
        <Self as RtActivatable<ICurrencyIdentifiersStatics>>::get_activation_factory().get_top()
    }}
    #[inline] pub fn get_try() -> Result<HString> { unsafe {
        <Self as RtActivatable<ICurrencyIdentifiersStatics>>::get_activation_factory().get_try()
    }}
    #[inline] pub fn get_ttd() -> Result<HString> { unsafe {
        <Self as RtActivatable<ICurrencyIdentifiersStatics>>::get_activation_factory().get_ttd()
    }}
    #[inline] pub fn get_twd() -> Result<HString> { unsafe {
        <Self as RtActivatable<ICurrencyIdentifiersStatics>>::get_activation_factory().get_twd()
    }}
    #[inline] pub fn get_tzs() -> Result<HString> { unsafe {
        <Self as RtActivatable<ICurrencyIdentifiersStatics>>::get_activation_factory().get_tzs()
    }}
    #[inline] pub fn get_uah() -> Result<HString> { unsafe {
        <Self as RtActivatable<ICurrencyIdentifiersStatics>>::get_activation_factory().get_uah()
    }}
    #[inline] pub fn get_ugx() -> Result<HString> { unsafe {
        <Self as RtActivatable<ICurrencyIdentifiersStatics>>::get_activation_factory().get_ugx()
    }}
    #[inline] pub fn get_usd() -> Result<HString> { unsafe {
        <Self as RtActivatable<ICurrencyIdentifiersStatics>>::get_activation_factory().get_usd()
    }}
    #[inline] pub fn get_uyu() -> Result<HString> { unsafe {
        <Self as RtActivatable<ICurrencyIdentifiersStatics>>::get_activation_factory().get_uyu()
    }}
    #[inline] pub fn get_uzs() -> Result<HString> { unsafe {
        <Self as RtActivatable<ICurrencyIdentifiersStatics>>::get_activation_factory().get_uzs()
    }}
    #[inline] pub fn get_vef() -> Result<HString> { unsafe {
        <Self as RtActivatable<ICurrencyIdentifiersStatics>>::get_activation_factory().get_vef()
    }}
    #[inline] pub fn get_vnd() -> Result<HString> { unsafe {
        <Self as RtActivatable<ICurrencyIdentifiersStatics>>::get_activation_factory().get_vnd()
    }}
    #[inline] pub fn get_vuv() -> Result<HString> { unsafe {
        <Self as RtActivatable<ICurrencyIdentifiersStatics>>::get_activation_factory().get_vuv()
    }}
    #[inline] pub fn get_wst() -> Result<HString> { unsafe {
        <Self as RtActivatable<ICurrencyIdentifiersStatics>>::get_activation_factory().get_wst()
    }}
    #[inline] pub fn get_xaf() -> Result<HString> { unsafe {
        <Self as RtActivatable<ICurrencyIdentifiersStatics>>::get_activation_factory().get_xaf()
    }}
    #[inline] pub fn get_xcd() -> Result<HString> { unsafe {
        <Self as RtActivatable<ICurrencyIdentifiersStatics>>::get_activation_factory().get_xcd()
    }}
    #[inline] pub fn get_xof() -> Result<HString> { unsafe {
        <Self as RtActivatable<ICurrencyIdentifiersStatics>>::get_activation_factory().get_xof()
    }}
    #[inline] pub fn get_xpf() -> Result<HString> { unsafe {
        <Self as RtActivatable<ICurrencyIdentifiersStatics>>::get_activation_factory().get_xpf()
    }}
    #[inline] pub fn get_xxx() -> Result<HString> { unsafe {
        <Self as RtActivatable<ICurrencyIdentifiersStatics>>::get_activation_factory().get_xxx()
    }}
    #[inline] pub fn get_yer() -> Result<HString> { unsafe {
        <Self as RtActivatable<ICurrencyIdentifiersStatics>>::get_activation_factory().get_yer()
    }}
    #[inline] pub fn get_zar() -> Result<HString> { unsafe {
        <Self as RtActivatable<ICurrencyIdentifiersStatics>>::get_activation_factory().get_zar()
    }}
    #[inline] pub fn get_zmw() -> Result<HString> { unsafe {
        <Self as RtActivatable<ICurrencyIdentifiersStatics>>::get_activation_factory().get_zmw()
    }}
    #[inline] pub fn get_zwl() -> Result<HString> { unsafe {
        <Self as RtActivatable<ICurrencyIdentifiersStatics>>::get_activation_factory().get_zwl()
    }}
    #[inline] pub fn get_byn() -> Result<HString> { unsafe {
        <Self as RtActivatable<ICurrencyIdentifiersStatics2>>::get_activation_factory().get_byn()
    }}
}
DEFINE_CLSID!(CurrencyIdentifiers(&[87,105,110,100,111,119,115,46,71,108,111,98,97,108,105,122,97,116,105,111,110,46,67,117,114,114,101,110,99,121,73,100,101,110,116,105,102,105,101,114,115,0]) [CLSID_CurrencyIdentifiers]);
DEFINE_IID!(IID_ICurrencyIdentifiersStatics, 2669480219, 54662, 18707, 155, 106, 169, 189, 45, 193, 40, 116);
RT_INTERFACE!{static interface ICurrencyIdentifiersStatics(ICurrencyIdentifiersStaticsVtbl): IInspectable(IInspectableVtbl) [IID_ICurrencyIdentifiersStatics] {
    fn get_AED(&self, out: *mut HSTRING) -> HRESULT,
    fn get_AFN(&self, out: *mut HSTRING) -> HRESULT,
    fn get_ALL(&self, out: *mut HSTRING) -> HRESULT,
    fn get_AMD(&self, out: *mut HSTRING) -> HRESULT,
    fn get_ANG(&self, out: *mut HSTRING) -> HRESULT,
    fn get_AOA(&self, out: *mut HSTRING) -> HRESULT,
    fn get_ARS(&self, out: *mut HSTRING) -> HRESULT,
    fn get_AUD(&self, out: *mut HSTRING) -> HRESULT,
    fn get_AWG(&self, out: *mut HSTRING) -> HRESULT,
    fn get_AZN(&self, out: *mut HSTRING) -> HRESULT,
    fn get_BAM(&self, out: *mut HSTRING) -> HRESULT,
    fn get_BBD(&self, out: *mut HSTRING) -> HRESULT,
    fn get_BDT(&self, out: *mut HSTRING) -> HRESULT,
    fn get_BGN(&self, out: *mut HSTRING) -> HRESULT,
    fn get_BHD(&self, out: *mut HSTRING) -> HRESULT,
    fn get_BIF(&self, out: *mut HSTRING) -> HRESULT,
    fn get_BMD(&self, out: *mut HSTRING) -> HRESULT,
    fn get_BND(&self, out: *mut HSTRING) -> HRESULT,
    fn get_BOB(&self, out: *mut HSTRING) -> HRESULT,
    fn get_BRL(&self, out: *mut HSTRING) -> HRESULT,
    fn get_BSD(&self, out: *mut HSTRING) -> HRESULT,
    fn get_BTN(&self, out: *mut HSTRING) -> HRESULT,
    fn get_BWP(&self, out: *mut HSTRING) -> HRESULT,
    fn get_BYR(&self, out: *mut HSTRING) -> HRESULT,
    fn get_BZD(&self, out: *mut HSTRING) -> HRESULT,
    fn get_CAD(&self, out: *mut HSTRING) -> HRESULT,
    fn get_CDF(&self, out: *mut HSTRING) -> HRESULT,
    fn get_CHF(&self, out: *mut HSTRING) -> HRESULT,
    fn get_CLP(&self, out: *mut HSTRING) -> HRESULT,
    fn get_CNY(&self, out: *mut HSTRING) -> HRESULT,
    fn get_COP(&self, out: *mut HSTRING) -> HRESULT,
    fn get_CRC(&self, out: *mut HSTRING) -> HRESULT,
    fn get_CUP(&self, out: *mut HSTRING) -> HRESULT,
    fn get_CVE(&self, out: *mut HSTRING) -> HRESULT,
    fn get_CZK(&self, out: *mut HSTRING) -> HRESULT,
    fn get_DJF(&self, out: *mut HSTRING) -> HRESULT,
    fn get_DKK(&self, out: *mut HSTRING) -> HRESULT,
    fn get_DOP(&self, out: *mut HSTRING) -> HRESULT,
    fn get_DZD(&self, out: *mut HSTRING) -> HRESULT,
    fn get_EGP(&self, out: *mut HSTRING) -> HRESULT,
    fn get_ERN(&self, out: *mut HSTRING) -> HRESULT,
    fn get_ETB(&self, out: *mut HSTRING) -> HRESULT,
    fn get_EUR(&self, out: *mut HSTRING) -> HRESULT,
    fn get_FJD(&self, out: *mut HSTRING) -> HRESULT,
    fn get_FKP(&self, out: *mut HSTRING) -> HRESULT,
    fn get_GBP(&self, out: *mut HSTRING) -> HRESULT,
    fn get_GEL(&self, out: *mut HSTRING) -> HRESULT,
    fn get_GHS(&self, out: *mut HSTRING) -> HRESULT,
    fn get_GIP(&self, out: *mut HSTRING) -> HRESULT,
    fn get_GMD(&self, out: *mut HSTRING) -> HRESULT,
    fn get_GNF(&self, out: *mut HSTRING) -> HRESULT,
    fn get_GTQ(&self, out: *mut HSTRING) -> HRESULT,
    fn get_GYD(&self, out: *mut HSTRING) -> HRESULT,
    fn get_HKD(&self, out: *mut HSTRING) -> HRESULT,
    fn get_HNL(&self, out: *mut HSTRING) -> HRESULT,
    fn get_HRK(&self, out: *mut HSTRING) -> HRESULT,
    fn get_HTG(&self, out: *mut HSTRING) -> HRESULT,
    fn get_HUF(&self, out: *mut HSTRING) -> HRESULT,
    fn get_IDR(&self, out: *mut HSTRING) -> HRESULT,
    fn get_ILS(&self, out: *mut HSTRING) -> HRESULT,
    fn get_INR(&self, out: *mut HSTRING) -> HRESULT,
    fn get_IQD(&self, out: *mut HSTRING) -> HRESULT,
    fn get_IRR(&self, out: *mut HSTRING) -> HRESULT,
    fn get_ISK(&self, out: *mut HSTRING) -> HRESULT,
    fn get_JMD(&self, out: *mut HSTRING) -> HRESULT,
    fn get_JOD(&self, out: *mut HSTRING) -> HRESULT,
    fn get_JPY(&self, out: *mut HSTRING) -> HRESULT,
    fn get_KES(&self, out: *mut HSTRING) -> HRESULT,
    fn get_KGS(&self, out: *mut HSTRING) -> HRESULT,
    fn get_KHR(&self, out: *mut HSTRING) -> HRESULT,
    fn get_KMF(&self, out: *mut HSTRING) -> HRESULT,
    fn get_KPW(&self, out: *mut HSTRING) -> HRESULT,
    fn get_KRW(&self, out: *mut HSTRING) -> HRESULT,
    fn get_KWD(&self, out: *mut HSTRING) -> HRESULT,
    fn get_KYD(&self, out: *mut HSTRING) -> HRESULT,
    fn get_KZT(&self, out: *mut HSTRING) -> HRESULT,
    fn get_LAK(&self, out: *mut HSTRING) -> HRESULT,
    fn get_LBP(&self, out: *mut HSTRING) -> HRESULT,
    fn get_LKR(&self, out: *mut HSTRING) -> HRESULT,
    fn get_LRD(&self, out: *mut HSTRING) -> HRESULT,
    fn get_LSL(&self, out: *mut HSTRING) -> HRESULT,
    fn get_LTL(&self, out: *mut HSTRING) -> HRESULT,
    fn get_LVL(&self, out: *mut HSTRING) -> HRESULT,
    fn get_LYD(&self, out: *mut HSTRING) -> HRESULT,
    fn get_MAD(&self, out: *mut HSTRING) -> HRESULT,
    fn get_MDL(&self, out: *mut HSTRING) -> HRESULT,
    fn get_MGA(&self, out: *mut HSTRING) -> HRESULT,
    fn get_MKD(&self, out: *mut HSTRING) -> HRESULT,
    fn get_MMK(&self, out: *mut HSTRING) -> HRESULT,
    fn get_MNT(&self, out: *mut HSTRING) -> HRESULT,
    fn get_MOP(&self, out: *mut HSTRING) -> HRESULT,
    fn get_MRO(&self, out: *mut HSTRING) -> HRESULT,
    fn get_MUR(&self, out: *mut HSTRING) -> HRESULT,
    fn get_MVR(&self, out: *mut HSTRING) -> HRESULT,
    fn get_MWK(&self, out: *mut HSTRING) -> HRESULT,
    fn get_MXN(&self, out: *mut HSTRING) -> HRESULT,
    fn get_MYR(&self, out: *mut HSTRING) -> HRESULT,
    fn get_MZN(&self, out: *mut HSTRING) -> HRESULT,
    fn get_NAD(&self, out: *mut HSTRING) -> HRESULT,
    fn get_NGN(&self, out: *mut HSTRING) -> HRESULT,
    fn get_NIO(&self, out: *mut HSTRING) -> HRESULT,
    fn get_NOK(&self, out: *mut HSTRING) -> HRESULT,
    fn get_NPR(&self, out: *mut HSTRING) -> HRESULT,
    fn get_NZD(&self, out: *mut HSTRING) -> HRESULT,
    fn get_OMR(&self, out: *mut HSTRING) -> HRESULT,
    fn get_PAB(&self, out: *mut HSTRING) -> HRESULT,
    fn get_PEN(&self, out: *mut HSTRING) -> HRESULT,
    fn get_PGK(&self, out: *mut HSTRING) -> HRESULT,
    fn get_PHP(&self, out: *mut HSTRING) -> HRESULT,
    fn get_PKR(&self, out: *mut HSTRING) -> HRESULT,
    fn get_PLN(&self, out: *mut HSTRING) -> HRESULT,
    fn get_PYG(&self, out: *mut HSTRING) -> HRESULT,
    fn get_QAR(&self, out: *mut HSTRING) -> HRESULT,
    fn get_RON(&self, out: *mut HSTRING) -> HRESULT,
    fn get_RSD(&self, out: *mut HSTRING) -> HRESULT,
    fn get_RUB(&self, out: *mut HSTRING) -> HRESULT,
    fn get_RWF(&self, out: *mut HSTRING) -> HRESULT,
    fn get_SAR(&self, out: *mut HSTRING) -> HRESULT,
    fn get_SBD(&self, out: *mut HSTRING) -> HRESULT,
    fn get_SCR(&self, out: *mut HSTRING) -> HRESULT,
    fn get_SDG(&self, out: *mut HSTRING) -> HRESULT,
    fn get_SEK(&self, out: *mut HSTRING) -> HRESULT,
    fn get_SGD(&self, out: *mut HSTRING) -> HRESULT,
    fn get_SHP(&self, out: *mut HSTRING) -> HRESULT,
    fn get_SLL(&self, out: *mut HSTRING) -> HRESULT,
    fn get_SOS(&self, out: *mut HSTRING) -> HRESULT,
    fn get_SRD(&self, out: *mut HSTRING) -> HRESULT,
    fn get_STD(&self, out: *mut HSTRING) -> HRESULT,
    fn get_SYP(&self, out: *mut HSTRING) -> HRESULT,
    fn get_SZL(&self, out: *mut HSTRING) -> HRESULT,
    fn get_THB(&self, out: *mut HSTRING) -> HRESULT,
    fn get_TJS(&self, out: *mut HSTRING) -> HRESULT,
    fn get_TMT(&self, out: *mut HSTRING) -> HRESULT,
    fn get_TND(&self, out: *mut HSTRING) -> HRESULT,
    fn get_TOP(&self, out: *mut HSTRING) -> HRESULT,
    fn get_TRY(&self, out: *mut HSTRING) -> HRESULT,
    fn get_TTD(&self, out: *mut HSTRING) -> HRESULT,
    fn get_TWD(&self, out: *mut HSTRING) -> HRESULT,
    fn get_TZS(&self, out: *mut HSTRING) -> HRESULT,
    fn get_UAH(&self, out: *mut HSTRING) -> HRESULT,
    fn get_UGX(&self, out: *mut HSTRING) -> HRESULT,
    fn get_USD(&self, out: *mut HSTRING) -> HRESULT,
    fn get_UYU(&self, out: *mut HSTRING) -> HRESULT,
    fn get_UZS(&self, out: *mut HSTRING) -> HRESULT,
    fn get_VEF(&self, out: *mut HSTRING) -> HRESULT,
    fn get_VND(&self, out: *mut HSTRING) -> HRESULT,
    fn get_VUV(&self, out: *mut HSTRING) -> HRESULT,
    fn get_WST(&self, out: *mut HSTRING) -> HRESULT,
    fn get_XAF(&self, out: *mut HSTRING) -> HRESULT,
    fn get_XCD(&self, out: *mut HSTRING) -> HRESULT,
    fn get_XOF(&self, out: *mut HSTRING) -> HRESULT,
    fn get_XPF(&self, out: *mut HSTRING) -> HRESULT,
    fn get_XXX(&self, out: *mut HSTRING) -> HRESULT,
    fn get_YER(&self, out: *mut HSTRING) -> HRESULT,
    fn get_ZAR(&self, out: *mut HSTRING) -> HRESULT,
    fn get_ZMW(&self, out: *mut HSTRING) -> HRESULT,
    fn get_ZWL(&self, out: *mut HSTRING) -> HRESULT
}}
impl ICurrencyIdentifiersStatics {
    #[inline] pub unsafe fn get_aed(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_AED)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_afn(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_AFN)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_all(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_ALL)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_amd(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_AMD)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_ang(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_ANG)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_aoa(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_AOA)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_ars(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_ARS)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_aud(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_AUD)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_awg(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_AWG)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_azn(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_AZN)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_bam(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_BAM)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_bbd(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_BBD)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_bdt(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_BDT)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_bgn(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_BGN)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_bhd(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_BHD)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_bif(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_BIF)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_bmd(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_BMD)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_bnd(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_BND)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_bob(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_BOB)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_brl(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_BRL)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_bsd(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_BSD)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_btn(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_BTN)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_bwp(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_BWP)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_byr(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_BYR)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_bzd(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_BZD)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_cad(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_CAD)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_cdf(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_CDF)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_chf(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_CHF)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_clp(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_CLP)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_cny(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_CNY)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_cop(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_COP)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_crc(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_CRC)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_cup(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_CUP)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_cve(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_CVE)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_czk(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_CZK)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_djf(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_DJF)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_dkk(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_DKK)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_dop(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_DOP)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_dzd(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_DZD)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_egp(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_EGP)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_ern(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_ERN)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_etb(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_ETB)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_eur(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_EUR)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_fjd(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_FJD)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_fkp(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_FKP)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_gbp(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_GBP)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_gel(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_GEL)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_ghs(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_GHS)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_gip(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_GIP)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_gmd(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_GMD)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_gnf(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_GNF)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_gtq(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_GTQ)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_gyd(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_GYD)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_hkd(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_HKD)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_hnl(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_HNL)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_hrk(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_HRK)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_htg(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_HTG)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_huf(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_HUF)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_idr(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_IDR)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_ils(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_ILS)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_inr(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_INR)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_iqd(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_IQD)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_irr(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_IRR)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_isk(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_ISK)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_jmd(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_JMD)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_jod(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_JOD)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_jpy(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_JPY)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_kes(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_KES)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_kgs(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_KGS)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_khr(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_KHR)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_kmf(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_KMF)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_kpw(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_KPW)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_krw(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_KRW)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_kwd(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_KWD)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_kyd(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_KYD)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_kzt(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_KZT)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_lak(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_LAK)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_lbp(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_LBP)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_lkr(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_LKR)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_lrd(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_LRD)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_lsl(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_LSL)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_ltl(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_LTL)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_lvl(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_LVL)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_lyd(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_LYD)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_mad(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_MAD)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_mdl(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_MDL)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_mga(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_MGA)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_mkd(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_MKD)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_mmk(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_MMK)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_mnt(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_MNT)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_mop(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_MOP)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_mro(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_MRO)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_mur(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_MUR)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_mvr(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_MVR)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_mwk(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_MWK)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_mxn(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_MXN)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_myr(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_MYR)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_mzn(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_MZN)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_nad(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_NAD)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_ngn(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_NGN)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_nio(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_NIO)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_nok(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_NOK)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_npr(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_NPR)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_nzd(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_NZD)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_omr(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_OMR)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_pab(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_PAB)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_pen(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_PEN)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_pgk(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_PGK)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_php(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_PHP)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_pkr(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_PKR)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_pln(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_PLN)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_pyg(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_PYG)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_qar(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_QAR)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_ron(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_RON)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_rsd(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_RSD)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_rub(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_RUB)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_rwf(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_RWF)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_sar(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_SAR)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_sbd(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_SBD)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_scr(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_SCR)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_sdg(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_SDG)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_sek(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_SEK)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_sgd(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_SGD)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_shp(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_SHP)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_sll(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_SLL)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_sos(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_SOS)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_srd(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_SRD)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_std(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_STD)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_syp(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_SYP)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_szl(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_SZL)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_thb(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_THB)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_tjs(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_TJS)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_tmt(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_TMT)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_tnd(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_TND)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_top(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_TOP)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_try(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_TRY)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_ttd(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_TTD)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_twd(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_TWD)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_tzs(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_TZS)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_uah(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_UAH)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_ugx(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_UGX)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_usd(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_USD)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_uyu(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_UYU)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_uzs(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_UZS)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_vef(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_VEF)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_vnd(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_VND)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_vuv(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_VUV)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_wst(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_WST)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_xaf(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_XAF)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_xcd(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_XCD)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_xof(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_XOF)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_xpf(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_XPF)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_xxx(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_XXX)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_yer(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_YER)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_zar(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_ZAR)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_zmw(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_ZMW)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_zwl(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_ZWL)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
}
DEFINE_IID!(IID_ICurrencyIdentifiersStatics2, 403995007, 50098, 19507, 149, 145, 152, 0, 17, 149, 13, 55);
RT_INTERFACE!{static interface ICurrencyIdentifiersStatics2(ICurrencyIdentifiersStatics2Vtbl): IInspectable(IInspectableVtbl) [IID_ICurrencyIdentifiersStatics2] {
    fn get_BYN(&self, out: *mut HSTRING) -> HRESULT
}}
impl ICurrencyIdentifiersStatics2 {
    #[inline] pub unsafe fn get_byn(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_BYN)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
}
RT_ENUM! { enum DayOfWeek: i32 {
    Sunday (DayOfWeek_Sunday) = 0, Monday (DayOfWeek_Monday) = 1, Tuesday (DayOfWeek_Tuesday) = 2, Wednesday (DayOfWeek_Wednesday) = 3, Thursday (DayOfWeek_Thursday) = 4, Friday (DayOfWeek_Friday) = 5, Saturday (DayOfWeek_Saturday) = 6,
}}
DEFINE_IID!(IID_IGeographicRegion, 32089633, 19044, 20185, 149, 79, 158, 222, 176, 123, 217, 3);
RT_INTERFACE!{interface IGeographicRegion(IGeographicRegionVtbl): IInspectable(IInspectableVtbl) [IID_IGeographicRegion] {
    fn get_Code(&self, out: *mut HSTRING) -> HRESULT,
    fn get_CodeTwoLetter(&self, out: *mut HSTRING) -> HRESULT,
    fn get_CodeThreeLetter(&self, out: *mut HSTRING) -> HRESULT,
    fn get_CodeThreeDigit(&self, out: *mut HSTRING) -> HRESULT,
    fn get_DisplayName(&self, out: *mut HSTRING) -> HRESULT,
    fn get_NativeName(&self, out: *mut HSTRING) -> HRESULT,
    fn get_CurrenciesInUse(&self, out: *mut *mut super::foundation::collections::IVectorView<HString>) -> HRESULT
}}
impl IGeographicRegion {
    #[inline] pub unsafe fn get_code(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Code)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_code_two_letter(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_CodeTwoLetter)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_code_three_letter(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_CodeThreeLetter)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_code_three_digit(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_CodeThreeDigit)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_display_name(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_DisplayName)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_native_name(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_NativeName)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_currencies_in_use(&self) -> Result<ComPtr<super::foundation::collections::IVectorView<HString>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_CurrenciesInUse)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
RT_CLASS!{class GeographicRegion: IGeographicRegion}
impl RtActivatable<IGeographicRegionFactory> for GeographicRegion {}
impl RtActivatable<IGeographicRegionStatics> for GeographicRegion {}
impl RtActivatable<IActivationFactory> for GeographicRegion {}
impl GeographicRegion {
    #[inline] pub fn create_geographic_region(geographicRegionCode: &HStringArg) -> Result<ComPtr<GeographicRegion>> { unsafe {
        <Self as RtActivatable<IGeographicRegionFactory>>::get_activation_factory().create_geographic_region(geographicRegionCode)
    }}
    #[inline] pub fn is_supported(geographicRegionCode: &HStringArg) -> Result<bool> { unsafe {
        <Self as RtActivatable<IGeographicRegionStatics>>::get_activation_factory().is_supported(geographicRegionCode)
    }}
}
DEFINE_CLSID!(GeographicRegion(&[87,105,110,100,111,119,115,46,71,108,111,98,97,108,105,122,97,116,105,111,110,46,71,101,111,103,114,97,112,104,105,99,82,101,103,105,111,110,0]) [CLSID_GeographicRegion]);
DEFINE_IID!(IID_IGeographicRegionFactory, 1396855408, 30644, 17003, 133, 159, 129, 225, 157, 81, 37, 70);
RT_INTERFACE!{static interface IGeographicRegionFactory(IGeographicRegionFactoryVtbl): IInspectable(IInspectableVtbl) [IID_IGeographicRegionFactory] {
    fn CreateGeographicRegion(&self, geographicRegionCode: HSTRING, out: *mut *mut GeographicRegion) -> HRESULT
}}
impl IGeographicRegionFactory {
    #[inline] pub unsafe fn create_geographic_region(&self, geographicRegionCode: &HStringArg) -> Result<ComPtr<GeographicRegion>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).CreateGeographicRegion)(self as *const _ as *mut _, geographicRegionCode.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
DEFINE_IID!(IID_IGeographicRegionStatics, 702712180, 31449, 20212, 135, 153, 179, 180, 79, 173, 236, 8);
RT_INTERFACE!{static interface IGeographicRegionStatics(IGeographicRegionStaticsVtbl): IInspectable(IInspectableVtbl) [IID_IGeographicRegionStatics] {
    fn IsSupported(&self, geographicRegionCode: HSTRING, out: *mut bool) -> HRESULT
}}
impl IGeographicRegionStatics {
    #[inline] pub unsafe fn is_supported(&self, geographicRegionCode: &HStringArg) -> Result<bool> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).IsSupported)(self as *const _ as *mut _, geographicRegionCode.get(), &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
}
DEFINE_IID!(IID_IJapanesePhoneme, 795513600, 59483, 17382, 137, 125, 93, 130, 248, 98, 223, 33);
RT_INTERFACE!{interface IJapanesePhoneme(IJapanesePhonemeVtbl): IInspectable(IInspectableVtbl) [IID_IJapanesePhoneme] {
    fn get_DisplayText(&self, out: *mut HSTRING) -> HRESULT,
    fn get_YomiText(&self, out: *mut HSTRING) -> HRESULT,
    fn get_IsPhraseStart(&self, out: *mut bool) -> HRESULT
}}
impl IJapanesePhoneme {
    #[inline] pub unsafe fn get_display_text(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_DisplayText)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_yomi_text(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_YomiText)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_is_phrase_start(&self) -> Result<bool> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_IsPhraseStart)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
}
RT_CLASS!{class JapanesePhoneme: IJapanesePhoneme}
RT_CLASS!{static class JapanesePhoneticAnalyzer}
impl RtActivatable<IJapanesePhoneticAnalyzerStatics> for JapanesePhoneticAnalyzer {}
impl JapanesePhoneticAnalyzer {
    #[inline] pub fn get_words(input: &HStringArg) -> Result<ComPtr<super::foundation::collections::IVectorView<JapanesePhoneme>>> { unsafe {
        <Self as RtActivatable<IJapanesePhoneticAnalyzerStatics>>::get_activation_factory().get_words(input)
    }}
    #[inline] pub fn get_words_with_mono_ruby_option(input: &HStringArg, monoRuby: bool) -> Result<ComPtr<super::foundation::collections::IVectorView<JapanesePhoneme>>> { unsafe {
        <Self as RtActivatable<IJapanesePhoneticAnalyzerStatics>>::get_activation_factory().get_words_with_mono_ruby_option(input, monoRuby)
    }}
}
DEFINE_CLSID!(JapanesePhoneticAnalyzer(&[87,105,110,100,111,119,115,46,71,108,111,98,97,108,105,122,97,116,105,111,110,46,74,97,112,97,110,101,115,101,80,104,111,110,101,116,105,99,65,110,97,108,121,122,101,114,0]) [CLSID_JapanesePhoneticAnalyzer]);
DEFINE_IID!(IID_IJapanesePhoneticAnalyzerStatics, 2292948624, 37854, 16818, 180, 213, 142, 219, 34, 127, 209, 194);
RT_INTERFACE!{static interface IJapanesePhoneticAnalyzerStatics(IJapanesePhoneticAnalyzerStaticsVtbl): IInspectable(IInspectableVtbl) [IID_IJapanesePhoneticAnalyzerStatics] {
    fn GetWords(&self, input: HSTRING, out: *mut *mut super::foundation::collections::IVectorView<JapanesePhoneme>) -> HRESULT,
    fn GetWordsWithMonoRubyOption(&self, input: HSTRING, monoRuby: bool, out: *mut *mut super::foundation::collections::IVectorView<JapanesePhoneme>) -> HRESULT
}}
impl IJapanesePhoneticAnalyzerStatics {
    #[inline] pub unsafe fn get_words(&self, input: &HStringArg) -> Result<ComPtr<super::foundation::collections::IVectorView<JapanesePhoneme>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetWords)(self as *const _ as *mut _, input.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_words_with_mono_ruby_option(&self, input: &HStringArg, monoRuby: bool) -> Result<ComPtr<super::foundation::collections::IVectorView<JapanesePhoneme>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetWordsWithMonoRubyOption)(self as *const _ as *mut _, input.get(), monoRuby, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
DEFINE_IID!(IID_ILanguage, 3933841234, 63426, 16997, 177, 189, 196, 222, 196, 228, 240, 128);
RT_INTERFACE!{interface ILanguage(ILanguageVtbl): IInspectable(IInspectableVtbl) [IID_ILanguage] {
    fn get_LanguageTag(&self, out: *mut HSTRING) -> HRESULT,
    fn get_DisplayName(&self, out: *mut HSTRING) -> HRESULT,
    fn get_NativeName(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Script(&self, out: *mut HSTRING) -> HRESULT
}}
impl ILanguage {
    #[inline] pub unsafe fn get_language_tag(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_LanguageTag)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_display_name(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_DisplayName)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_native_name(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_NativeName)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_script(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Script)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
}
RT_CLASS!{class Language: ILanguage}
impl RtActivatable<ILanguageFactory> for Language {}
impl RtActivatable<ILanguageStatics> for Language {}
impl RtActivatable<ILanguageStatics2> for Language {}
impl Language {
    #[inline] pub fn create_language(languageTag: &HStringArg) -> Result<ComPtr<Language>> { unsafe {
        <Self as RtActivatable<ILanguageFactory>>::get_activation_factory().create_language(languageTag)
    }}
    #[inline] pub fn is_well_formed(languageTag: &HStringArg) -> Result<bool> { unsafe {
        <Self as RtActivatable<ILanguageStatics>>::get_activation_factory().is_well_formed(languageTag)
    }}
    #[inline] pub fn get_current_input_method_language_tag() -> Result<HString> { unsafe {
        <Self as RtActivatable<ILanguageStatics>>::get_activation_factory().get_current_input_method_language_tag()
    }}
    #[inline] pub fn try_set_input_method_language_tag(languageTag: &HStringArg) -> Result<bool> { unsafe {
        <Self as RtActivatable<ILanguageStatics2>>::get_activation_factory().try_set_input_method_language_tag(languageTag)
    }}
}
DEFINE_CLSID!(Language(&[87,105,110,100,111,119,115,46,71,108,111,98,97,108,105,122,97,116,105,111,110,46,76,97,110,103,117,97,103,101,0]) [CLSID_Language]);
DEFINE_IID!(IID_ILanguageExtensionSubtags, 2105388869, 13965, 17252, 133, 43, 222, 201, 39, 3, 123, 133);
RT_INTERFACE!{interface ILanguageExtensionSubtags(ILanguageExtensionSubtagsVtbl): IInspectable(IInspectableVtbl) [IID_ILanguageExtensionSubtags] {
    fn GetExtensionSubtags(&self, singleton: HSTRING, out: *mut *mut super::foundation::collections::IVectorView<HString>) -> HRESULT
}}
impl ILanguageExtensionSubtags {
    #[inline] pub unsafe fn get_extension_subtags(&self, singleton: &HStringArg) -> Result<ComPtr<super::foundation::collections::IVectorView<HString>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetExtensionSubtags)(self as *const _ as *mut _, singleton.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
DEFINE_IID!(IID_ILanguageFactory, 2600620716, 3111, 17656, 183, 146, 151, 147, 251, 102, 198, 62);
RT_INTERFACE!{static interface ILanguageFactory(ILanguageFactoryVtbl): IInspectable(IInspectableVtbl) [IID_ILanguageFactory] {
    fn CreateLanguage(&self, languageTag: HSTRING, out: *mut *mut Language) -> HRESULT
}}
impl ILanguageFactory {
    #[inline] pub unsafe fn create_language(&self, languageTag: &HStringArg) -> Result<ComPtr<Language>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).CreateLanguage)(self as *const _ as *mut _, languageTag.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
DEFINE_IID!(IID_ILanguageStatics, 2990331223, 2149, 18132, 137, 184, 213, 155, 232, 153, 15, 13);
RT_INTERFACE!{static interface ILanguageStatics(ILanguageStaticsVtbl): IInspectable(IInspectableVtbl) [IID_ILanguageStatics] {
    fn IsWellFormed(&self, languageTag: HSTRING, out: *mut bool) -> HRESULT,
    fn get_CurrentInputMethodLanguageTag(&self, out: *mut HSTRING) -> HRESULT
}}
impl ILanguageStatics {
    #[inline] pub unsafe fn is_well_formed(&self, languageTag: &HStringArg) -> Result<bool> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).IsWellFormed)(self as *const _ as *mut _, languageTag.get(), &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_current_input_method_language_tag(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_CurrentInputMethodLanguageTag)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
}
DEFINE_IID!(IID_ILanguageStatics2, 806985582, 37195, 19242, 157, 110, 227, 176, 226, 125, 190, 79);
RT_INTERFACE!{static interface ILanguageStatics2(ILanguageStatics2Vtbl): IInspectable(IInspectableVtbl) [IID_ILanguageStatics2] {
    fn TrySetInputMethodLanguageTag(&self, languageTag: HSTRING, out: *mut bool) -> HRESULT
}}
impl ILanguageStatics2 {
    #[inline] pub unsafe fn try_set_input_method_language_tag(&self, languageTag: &HStringArg) -> Result<bool> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).TrySetInputMethodLanguageTag)(self as *const _ as *mut _, languageTag.get(), &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
}
RT_CLASS!{static class NumeralSystemIdentifiers}
impl RtActivatable<INumeralSystemIdentifiersStatics> for NumeralSystemIdentifiers {}
impl RtActivatable<INumeralSystemIdentifiersStatics2> for NumeralSystemIdentifiers {}
impl NumeralSystemIdentifiers {
    #[inline] pub fn get_arab() -> Result<HString> { unsafe {
        <Self as RtActivatable<INumeralSystemIdentifiersStatics>>::get_activation_factory().get_arab()
    }}
    #[inline] pub fn get_arab_ext() -> Result<HString> { unsafe {
        <Self as RtActivatable<INumeralSystemIdentifiersStatics>>::get_activation_factory().get_arab_ext()
    }}
    #[inline] pub fn get_bali() -> Result<HString> { unsafe {
        <Self as RtActivatable<INumeralSystemIdentifiersStatics>>::get_activation_factory().get_bali()
    }}
    #[inline] pub fn get_beng() -> Result<HString> { unsafe {
        <Self as RtActivatable<INumeralSystemIdentifiersStatics>>::get_activation_factory().get_beng()
    }}
    #[inline] pub fn get_cham() -> Result<HString> { unsafe {
        <Self as RtActivatable<INumeralSystemIdentifiersStatics>>::get_activation_factory().get_cham()
    }}
    #[inline] pub fn get_deva() -> Result<HString> { unsafe {
        <Self as RtActivatable<INumeralSystemIdentifiersStatics>>::get_activation_factory().get_deva()
    }}
    #[inline] pub fn get_full_wide() -> Result<HString> { unsafe {
        <Self as RtActivatable<INumeralSystemIdentifiersStatics>>::get_activation_factory().get_full_wide()
    }}
    #[inline] pub fn get_gujr() -> Result<HString> { unsafe {
        <Self as RtActivatable<INumeralSystemIdentifiersStatics>>::get_activation_factory().get_gujr()
    }}
    #[inline] pub fn get_guru() -> Result<HString> { unsafe {
        <Self as RtActivatable<INumeralSystemIdentifiersStatics>>::get_activation_factory().get_guru()
    }}
    #[inline] pub fn get_hani_dec() -> Result<HString> { unsafe {
        <Self as RtActivatable<INumeralSystemIdentifiersStatics>>::get_activation_factory().get_hani_dec()
    }}
    #[inline] pub fn get_java() -> Result<HString> { unsafe {
        <Self as RtActivatable<INumeralSystemIdentifiersStatics>>::get_activation_factory().get_java()
    }}
    #[inline] pub fn get_kali() -> Result<HString> { unsafe {
        <Self as RtActivatable<INumeralSystemIdentifiersStatics>>::get_activation_factory().get_kali()
    }}
    #[inline] pub fn get_khmr() -> Result<HString> { unsafe {
        <Self as RtActivatable<INumeralSystemIdentifiersStatics>>::get_activation_factory().get_khmr()
    }}
    #[inline] pub fn get_knda() -> Result<HString> { unsafe {
        <Self as RtActivatable<INumeralSystemIdentifiersStatics>>::get_activation_factory().get_knda()
    }}
    #[inline] pub fn get_lana() -> Result<HString> { unsafe {
        <Self as RtActivatable<INumeralSystemIdentifiersStatics>>::get_activation_factory().get_lana()
    }}
    #[inline] pub fn get_lana_tham() -> Result<HString> { unsafe {
        <Self as RtActivatable<INumeralSystemIdentifiersStatics>>::get_activation_factory().get_lana_tham()
    }}
    #[inline] pub fn get_laoo() -> Result<HString> { unsafe {
        <Self as RtActivatable<INumeralSystemIdentifiersStatics>>::get_activation_factory().get_laoo()
    }}
    #[inline] pub fn get_latn() -> Result<HString> { unsafe {
        <Self as RtActivatable<INumeralSystemIdentifiersStatics>>::get_activation_factory().get_latn()
    }}
    #[inline] pub fn get_lepc() -> Result<HString> { unsafe {
        <Self as RtActivatable<INumeralSystemIdentifiersStatics>>::get_activation_factory().get_lepc()
    }}
    #[inline] pub fn get_limb() -> Result<HString> { unsafe {
        <Self as RtActivatable<INumeralSystemIdentifiersStatics>>::get_activation_factory().get_limb()
    }}
    #[inline] pub fn get_mlym() -> Result<HString> { unsafe {
        <Self as RtActivatable<INumeralSystemIdentifiersStatics>>::get_activation_factory().get_mlym()
    }}
    #[inline] pub fn get_mong() -> Result<HString> { unsafe {
        <Self as RtActivatable<INumeralSystemIdentifiersStatics>>::get_activation_factory().get_mong()
    }}
    #[inline] pub fn get_mtei() -> Result<HString> { unsafe {
        <Self as RtActivatable<INumeralSystemIdentifiersStatics>>::get_activation_factory().get_mtei()
    }}
    #[inline] pub fn get_mymr() -> Result<HString> { unsafe {
        <Self as RtActivatable<INumeralSystemIdentifiersStatics>>::get_activation_factory().get_mymr()
    }}
    #[inline] pub fn get_mymr_shan() -> Result<HString> { unsafe {
        <Self as RtActivatable<INumeralSystemIdentifiersStatics>>::get_activation_factory().get_mymr_shan()
    }}
    #[inline] pub fn get_nkoo() -> Result<HString> { unsafe {
        <Self as RtActivatable<INumeralSystemIdentifiersStatics>>::get_activation_factory().get_nkoo()
    }}
    #[inline] pub fn get_olck() -> Result<HString> { unsafe {
        <Self as RtActivatable<INumeralSystemIdentifiersStatics>>::get_activation_factory().get_olck()
    }}
    #[inline] pub fn get_orya() -> Result<HString> { unsafe {
        <Self as RtActivatable<INumeralSystemIdentifiersStatics>>::get_activation_factory().get_orya()
    }}
    #[inline] pub fn get_saur() -> Result<HString> { unsafe {
        <Self as RtActivatable<INumeralSystemIdentifiersStatics>>::get_activation_factory().get_saur()
    }}
    #[inline] pub fn get_sund() -> Result<HString> { unsafe {
        <Self as RtActivatable<INumeralSystemIdentifiersStatics>>::get_activation_factory().get_sund()
    }}
    #[inline] pub fn get_talu() -> Result<HString> { unsafe {
        <Self as RtActivatable<INumeralSystemIdentifiersStatics>>::get_activation_factory().get_talu()
    }}
    #[inline] pub fn get_taml_dec() -> Result<HString> { unsafe {
        <Self as RtActivatable<INumeralSystemIdentifiersStatics>>::get_activation_factory().get_taml_dec()
    }}
    #[inline] pub fn get_telu() -> Result<HString> { unsafe {
        <Self as RtActivatable<INumeralSystemIdentifiersStatics>>::get_activation_factory().get_telu()
    }}
    #[inline] pub fn get_thai() -> Result<HString> { unsafe {
        <Self as RtActivatable<INumeralSystemIdentifiersStatics>>::get_activation_factory().get_thai()
    }}
    #[inline] pub fn get_tibt() -> Result<HString> { unsafe {
        <Self as RtActivatable<INumeralSystemIdentifiersStatics>>::get_activation_factory().get_tibt()
    }}
    #[inline] pub fn get_vaii() -> Result<HString> { unsafe {
        <Self as RtActivatable<INumeralSystemIdentifiersStatics>>::get_activation_factory().get_vaii()
    }}
    #[inline] pub fn get_brah() -> Result<HString> { unsafe {
        <Self as RtActivatable<INumeralSystemIdentifiersStatics2>>::get_activation_factory().get_brah()
    }}
    #[inline] pub fn get_osma() -> Result<HString> { unsafe {
        <Self as RtActivatable<INumeralSystemIdentifiersStatics2>>::get_activation_factory().get_osma()
    }}
    #[inline] pub fn get_math_bold() -> Result<HString> { unsafe {
        <Self as RtActivatable<INumeralSystemIdentifiersStatics2>>::get_activation_factory().get_math_bold()
    }}
    #[inline] pub fn get_math_dbl() -> Result<HString> { unsafe {
        <Self as RtActivatable<INumeralSystemIdentifiersStatics2>>::get_activation_factory().get_math_dbl()
    }}
    #[inline] pub fn get_math_sans() -> Result<HString> { unsafe {
        <Self as RtActivatable<INumeralSystemIdentifiersStatics2>>::get_activation_factory().get_math_sans()
    }}
    #[inline] pub fn get_math_sanb() -> Result<HString> { unsafe {
        <Self as RtActivatable<INumeralSystemIdentifiersStatics2>>::get_activation_factory().get_math_sanb()
    }}
    #[inline] pub fn get_math_mono() -> Result<HString> { unsafe {
        <Self as RtActivatable<INumeralSystemIdentifiersStatics2>>::get_activation_factory().get_math_mono()
    }}
    #[inline] pub fn get_zmth_bold() -> Result<HString> { unsafe {
        <Self as RtActivatable<INumeralSystemIdentifiersStatics2>>::get_activation_factory().get_zmth_bold()
    }}
    #[inline] pub fn get_zmth_dbl() -> Result<HString> { unsafe {
        <Self as RtActivatable<INumeralSystemIdentifiersStatics2>>::get_activation_factory().get_zmth_dbl()
    }}
    #[inline] pub fn get_zmth_sans() -> Result<HString> { unsafe {
        <Self as RtActivatable<INumeralSystemIdentifiersStatics2>>::get_activation_factory().get_zmth_sans()
    }}
    #[inline] pub fn get_zmth_sanb() -> Result<HString> { unsafe {
        <Self as RtActivatable<INumeralSystemIdentifiersStatics2>>::get_activation_factory().get_zmth_sanb()
    }}
    #[inline] pub fn get_zmth_mono() -> Result<HString> { unsafe {
        <Self as RtActivatable<INumeralSystemIdentifiersStatics2>>::get_activation_factory().get_zmth_mono()
    }}
}
DEFINE_CLSID!(NumeralSystemIdentifiers(&[87,105,110,100,111,119,115,46,71,108,111,98,97,108,105,122,97,116,105,111,110,46,78,117,109,101,114,97,108,83,121,115,116,101,109,73,100,101,110,116,105,102,105,101,114,115,0]) [CLSID_NumeralSystemIdentifiers]);
DEFINE_IID!(IID_INumeralSystemIdentifiersStatics, 2781242051, 26825, 19773, 183, 101, 151, 32, 41, 226, 29, 236);
RT_INTERFACE!{static interface INumeralSystemIdentifiersStatics(INumeralSystemIdentifiersStaticsVtbl): IInspectable(IInspectableVtbl) [IID_INumeralSystemIdentifiersStatics] {
    fn get_Arab(&self, out: *mut HSTRING) -> HRESULT,
    fn get_ArabExt(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Bali(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Beng(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Cham(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Deva(&self, out: *mut HSTRING) -> HRESULT,
    fn get_FullWide(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Gujr(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Guru(&self, out: *mut HSTRING) -> HRESULT,
    fn get_HaniDec(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Java(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Kali(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Khmr(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Knda(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Lana(&self, out: *mut HSTRING) -> HRESULT,
    fn get_LanaTham(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Laoo(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Latn(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Lepc(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Limb(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Mlym(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Mong(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Mtei(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Mymr(&self, out: *mut HSTRING) -> HRESULT,
    fn get_MymrShan(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Nkoo(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Olck(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Orya(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Saur(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Sund(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Talu(&self, out: *mut HSTRING) -> HRESULT,
    fn get_TamlDec(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Telu(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Thai(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Tibt(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Vaii(&self, out: *mut HSTRING) -> HRESULT
}}
impl INumeralSystemIdentifiersStatics {
    #[inline] pub unsafe fn get_arab(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Arab)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_arab_ext(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_ArabExt)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_bali(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Bali)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_beng(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Beng)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_cham(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Cham)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_deva(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Deva)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_full_wide(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_FullWide)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_gujr(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Gujr)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_guru(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Guru)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_hani_dec(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_HaniDec)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_java(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Java)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_kali(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Kali)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_khmr(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Khmr)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_knda(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Knda)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_lana(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Lana)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_lana_tham(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_LanaTham)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_laoo(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Laoo)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_latn(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Latn)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_lepc(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Lepc)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_limb(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Limb)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_mlym(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Mlym)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_mong(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Mong)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_mtei(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Mtei)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_mymr(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Mymr)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_mymr_shan(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_MymrShan)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_nkoo(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Nkoo)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_olck(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Olck)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_orya(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Orya)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_saur(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Saur)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_sund(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Sund)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_talu(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Talu)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_taml_dec(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_TamlDec)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_telu(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Telu)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_thai(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Thai)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_tibt(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Tibt)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_vaii(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Vaii)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
}
DEFINE_IID!(IID_INumeralSystemIdentifiersStatics2, 2130719272, 40411, 18996, 145, 4, 2, 96, 192, 145, 167, 199);
RT_INTERFACE!{static interface INumeralSystemIdentifiersStatics2(INumeralSystemIdentifiersStatics2Vtbl): IInspectable(IInspectableVtbl) [IID_INumeralSystemIdentifiersStatics2] {
    fn get_Brah(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Osma(&self, out: *mut HSTRING) -> HRESULT,
    fn get_MathBold(&self, out: *mut HSTRING) -> HRESULT,
    fn get_MathDbl(&self, out: *mut HSTRING) -> HRESULT,
    fn get_MathSans(&self, out: *mut HSTRING) -> HRESULT,
    fn get_MathSanb(&self, out: *mut HSTRING) -> HRESULT,
    fn get_MathMono(&self, out: *mut HSTRING) -> HRESULT,
    fn get_ZmthBold(&self, out: *mut HSTRING) -> HRESULT,
    fn get_ZmthDbl(&self, out: *mut HSTRING) -> HRESULT,
    fn get_ZmthSans(&self, out: *mut HSTRING) -> HRESULT,
    fn get_ZmthSanb(&self, out: *mut HSTRING) -> HRESULT,
    fn get_ZmthMono(&self, out: *mut HSTRING) -> HRESULT
}}
impl INumeralSystemIdentifiersStatics2 {
    #[inline] pub unsafe fn get_brah(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Brah)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_osma(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Osma)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_math_bold(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_MathBold)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_math_dbl(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_MathDbl)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_math_sans(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_MathSans)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_math_sanb(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_MathSanb)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_math_mono(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_MathMono)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_zmth_bold(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_ZmthBold)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_zmth_dbl(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_ZmthDbl)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_zmth_sans(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_ZmthSans)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_zmth_sanb(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_ZmthSanb)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_zmth_mono(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_ZmthMono)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
}
DEFINE_IID!(IID_ITimeZoneOnCalendar, 3141281253, 18127, 17175, 163, 245, 2, 98, 26, 213, 68, 120);
RT_INTERFACE!{interface ITimeZoneOnCalendar(ITimeZoneOnCalendarVtbl): IInspectable(IInspectableVtbl) [IID_ITimeZoneOnCalendar] {
    fn GetTimeZone(&self, out: *mut HSTRING) -> HRESULT,
    fn ChangeTimeZone(&self, timeZoneId: HSTRING) -> HRESULT,
    fn TimeZoneAsFullString(&self, out: *mut HSTRING) -> HRESULT,
    fn TimeZoneAsString(&self, idealLength: i32, out: *mut HSTRING) -> HRESULT
}}
impl ITimeZoneOnCalendar {
    #[inline] pub unsafe fn get_time_zone(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetTimeZone)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn change_time_zone(&self, timeZoneId: &HStringArg) -> Result<()> {
        let hr = ((*self.lpVtbl).ChangeTimeZone)(self as *const _ as *mut _, timeZoneId.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn time_zone_as_full_string(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).TimeZoneAsFullString)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn time_zone_as_string(&self, idealLength: i32) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).TimeZoneAsString)(self as *const _ as *mut _, idealLength, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
}
pub mod fonts { // Windows.Globalization.Fonts
use ::prelude::*;
DEFINE_IID!(IID_ILanguageFont, 2972605498, 46957, 17819, 190, 235, 144, 17, 81, 205, 119, 209);
RT_INTERFACE!{interface ILanguageFont(ILanguageFontVtbl): IInspectable(IInspectableVtbl) [IID_ILanguageFont] {
    fn get_FontFamily(&self, out: *mut HSTRING) -> HRESULT,
    #[cfg(not(feature="windows-ui"))] fn __Dummy1(&self) -> (),
    #[cfg(feature="windows-ui")] fn get_FontWeight(&self, out: *mut super::super::ui::text::FontWeight) -> HRESULT,
    #[cfg(not(feature="windows-ui"))] fn __Dummy2(&self) -> (),
    #[cfg(feature="windows-ui")] fn get_FontStretch(&self, out: *mut super::super::ui::text::FontStretch) -> HRESULT,
    #[cfg(not(feature="windows-ui"))] fn __Dummy3(&self) -> (),
    #[cfg(feature="windows-ui")] fn get_FontStyle(&self, out: *mut super::super::ui::text::FontStyle) -> HRESULT,
    fn get_ScaleFactor(&self, out: *mut f64) -> HRESULT
}}
impl ILanguageFont {
    #[inline] pub unsafe fn get_font_family(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_FontFamily)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[cfg(feature="windows-ui")] #[inline] pub unsafe fn get_font_weight(&self) -> Result<super::super::ui::text::FontWeight> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_FontWeight)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[cfg(feature="windows-ui")] #[inline] pub unsafe fn get_font_stretch(&self) -> Result<super::super::ui::text::FontStretch> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_FontStretch)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[cfg(feature="windows-ui")] #[inline] pub unsafe fn get_font_style(&self) -> Result<super::super::ui::text::FontStyle> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_FontStyle)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_scale_factor(&self) -> Result<f64> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_ScaleFactor)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
}
RT_CLASS!{class LanguageFont: ILanguageFont}
DEFINE_IID!(IID_ILanguageFontGroup, 4080697283, 14940, 19178, 185, 255, 179, 159, 178, 66, 247, 246);
RT_INTERFACE!{interface ILanguageFontGroup(ILanguageFontGroupVtbl): IInspectable(IInspectableVtbl) [IID_ILanguageFontGroup] {
    fn get_UITextFont(&self, out: *mut *mut LanguageFont) -> HRESULT,
    fn get_UIHeadingFont(&self, out: *mut *mut LanguageFont) -> HRESULT,
    fn get_UITitleFont(&self, out: *mut *mut LanguageFont) -> HRESULT,
    fn get_UICaptionFont(&self, out: *mut *mut LanguageFont) -> HRESULT,
    fn get_UINotificationHeadingFont(&self, out: *mut *mut LanguageFont) -> HRESULT,
    fn get_TraditionalDocumentFont(&self, out: *mut *mut LanguageFont) -> HRESULT,
    fn get_ModernDocumentFont(&self, out: *mut *mut LanguageFont) -> HRESULT,
    fn get_DocumentHeadingFont(&self, out: *mut *mut LanguageFont) -> HRESULT,
    fn get_FixedWidthTextFont(&self, out: *mut *mut LanguageFont) -> HRESULT,
    fn get_DocumentAlternate1Font(&self, out: *mut *mut LanguageFont) -> HRESULT,
    fn get_DocumentAlternate2Font(&self, out: *mut *mut LanguageFont) -> HRESULT
}}
impl ILanguageFontGroup {
    #[inline] pub unsafe fn get_uitext_font(&self) -> Result<ComPtr<LanguageFont>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_UITextFont)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_uiheading_font(&self) -> Result<ComPtr<LanguageFont>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_UIHeadingFont)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_uititle_font(&self) -> Result<ComPtr<LanguageFont>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_UITitleFont)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_uicaption_font(&self) -> Result<ComPtr<LanguageFont>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_UICaptionFont)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_uinotification_heading_font(&self) -> Result<ComPtr<LanguageFont>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_UINotificationHeadingFont)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_traditional_document_font(&self) -> Result<ComPtr<LanguageFont>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_TraditionalDocumentFont)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_modern_document_font(&self) -> Result<ComPtr<LanguageFont>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_ModernDocumentFont)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_document_heading_font(&self) -> Result<ComPtr<LanguageFont>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_DocumentHeadingFont)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_fixed_width_text_font(&self) -> Result<ComPtr<LanguageFont>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_FixedWidthTextFont)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_document_alternate1_font(&self) -> Result<ComPtr<LanguageFont>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_DocumentAlternate1Font)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_document_alternate2_font(&self) -> Result<ComPtr<LanguageFont>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_DocumentAlternate2Font)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
RT_CLASS!{class LanguageFontGroup: ILanguageFontGroup}
impl RtActivatable<ILanguageFontGroupFactory> for LanguageFontGroup {}
impl LanguageFontGroup {
    #[inline] pub fn create_language_font_group(languageTag: &HStringArg) -> Result<ComPtr<LanguageFontGroup>> { unsafe {
        <Self as RtActivatable<ILanguageFontGroupFactory>>::get_activation_factory().create_language_font_group(languageTag)
    }}
}
DEFINE_CLSID!(LanguageFontGroup(&[87,105,110,100,111,119,115,46,71,108,111,98,97,108,105,122,97,116,105,111,110,46,70,111,110,116,115,46,76,97,110,103,117,97,103,101,70,111,110,116,71,114,111,117,112,0]) [CLSID_LanguageFontGroup]);
DEFINE_IID!(IID_ILanguageFontGroupFactory, 4239305831, 20087, 18887, 184, 86, 221, 233, 52, 252, 115, 91);
RT_INTERFACE!{static interface ILanguageFontGroupFactory(ILanguageFontGroupFactoryVtbl): IInspectable(IInspectableVtbl) [IID_ILanguageFontGroupFactory] {
    fn CreateLanguageFontGroup(&self, languageTag: HSTRING, out: *mut *mut LanguageFontGroup) -> HRESULT
}}
impl ILanguageFontGroupFactory {
    #[inline] pub unsafe fn create_language_font_group(&self, languageTag: &HStringArg) -> Result<ComPtr<LanguageFontGroup>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).CreateLanguageFontGroup)(self as *const _ as *mut _, languageTag.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
} // Windows.Globalization.Fonts
pub mod phonenumberformatting { // Windows.Globalization.PhoneNumberFormatting
use ::prelude::*;
RT_ENUM! { enum PhoneNumberFormat: i32 {
    E164 (PhoneNumberFormat_E164) = 0, International (PhoneNumberFormat_International) = 1, National (PhoneNumberFormat_National) = 2, Rfc3966 (PhoneNumberFormat_Rfc3966) = 3,
}}
DEFINE_IID!(IID_IPhoneNumberFormatter, 358003870, 47828, 19274, 144, 13, 68, 7, 173, 183, 201, 129);
RT_INTERFACE!{interface IPhoneNumberFormatter(IPhoneNumberFormatterVtbl): IInspectable(IInspectableVtbl) [IID_IPhoneNumberFormatter] {
    fn Format(&self, number: *mut PhoneNumberInfo, out: *mut HSTRING) -> HRESULT,
    fn FormatWithOutputFormat(&self, number: *mut PhoneNumberInfo, numberFormat: PhoneNumberFormat, out: *mut HSTRING) -> HRESULT,
    fn FormatPartialString(&self, number: HSTRING, out: *mut HSTRING) -> HRESULT,
    fn FormatString(&self, number: HSTRING, out: *mut HSTRING) -> HRESULT,
    fn FormatStringWithLeftToRightMarkers(&self, number: HSTRING, out: *mut HSTRING) -> HRESULT
}}
impl IPhoneNumberFormatter {
    #[inline] pub unsafe fn format(&self, number: &PhoneNumberInfo) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).Format)(self as *const _ as *mut _, number as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn format_with_output_format(&self, number: &PhoneNumberInfo, numberFormat: PhoneNumberFormat) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).FormatWithOutputFormat)(self as *const _ as *mut _, number as *const _ as *mut _, numberFormat, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn format_partial_string(&self, number: &HStringArg) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).FormatPartialString)(self as *const _ as *mut _, number.get(), &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn format_string(&self, number: &HStringArg) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).FormatString)(self as *const _ as *mut _, number.get(), &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn format_string_with_left_to_right_markers(&self, number: &HStringArg) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).FormatStringWithLeftToRightMarkers)(self as *const _ as *mut _, number.get(), &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
}
RT_CLASS!{class PhoneNumberFormatter: IPhoneNumberFormatter}
impl RtActivatable<IPhoneNumberFormatterStatics> for PhoneNumberFormatter {}
impl RtActivatable<IActivationFactory> for PhoneNumberFormatter {}
impl PhoneNumberFormatter {
    #[inline] pub fn try_create(regionCode: &HStringArg) -> Result<ComPtr<PhoneNumberFormatter>> { unsafe {
        <Self as RtActivatable<IPhoneNumberFormatterStatics>>::get_activation_factory().try_create(regionCode)
    }}
    #[inline] pub fn get_country_code_for_region(regionCode: &HStringArg) -> Result<i32> { unsafe {
        <Self as RtActivatable<IPhoneNumberFormatterStatics>>::get_activation_factory().get_country_code_for_region(regionCode)
    }}
    #[inline] pub fn get_national_direct_dialing_prefix_for_region(regionCode: &HStringArg, stripNonDigit: bool) -> Result<HString> { unsafe {
        <Self as RtActivatable<IPhoneNumberFormatterStatics>>::get_activation_factory().get_national_direct_dialing_prefix_for_region(regionCode, stripNonDigit)
    }}
    #[inline] pub fn wrap_with_left_to_right_markers(number: &HStringArg) -> Result<HString> { unsafe {
        <Self as RtActivatable<IPhoneNumberFormatterStatics>>::get_activation_factory().wrap_with_left_to_right_markers(number)
    }}
}
DEFINE_CLSID!(PhoneNumberFormatter(&[87,105,110,100,111,119,115,46,71,108,111,98,97,108,105,122,97,116,105,111,110,46,80,104,111,110,101,78,117,109,98,101,114,70,111,114,109,97,116,116,105,110,103,46,80,104,111,110,101,78,117,109,98,101,114,70,111,114,109,97,116,116,101,114,0]) [CLSID_PhoneNumberFormatter]);
DEFINE_IID!(IID_IPhoneNumberFormatterStatics, 1554446641, 34009, 16715, 171, 78, 160, 85, 44, 135, 134, 2);
RT_INTERFACE!{static interface IPhoneNumberFormatterStatics(IPhoneNumberFormatterStaticsVtbl): IInspectable(IInspectableVtbl) [IID_IPhoneNumberFormatterStatics] {
    fn TryCreate(&self, regionCode: HSTRING, phoneNumber: *mut *mut PhoneNumberFormatter) -> HRESULT,
    fn GetCountryCodeForRegion(&self, regionCode: HSTRING, out: *mut i32) -> HRESULT,
    fn GetNationalDirectDialingPrefixForRegion(&self, regionCode: HSTRING, stripNonDigit: bool, out: *mut HSTRING) -> HRESULT,
    fn WrapWithLeftToRightMarkers(&self, number: HSTRING, out: *mut HSTRING) -> HRESULT
}}
impl IPhoneNumberFormatterStatics {
    #[inline] pub unsafe fn try_create(&self, regionCode: &HStringArg) -> Result<ComPtr<PhoneNumberFormatter>> {
        let mut phoneNumber = null_mut();
        let hr = ((*self.lpVtbl).TryCreate)(self as *const _ as *mut _, regionCode.get(), &mut phoneNumber);
        if hr == S_OK { Ok(ComPtr::wrap(phoneNumber)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_country_code_for_region(&self, regionCode: &HStringArg) -> Result<i32> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).GetCountryCodeForRegion)(self as *const _ as *mut _, regionCode.get(), &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_national_direct_dialing_prefix_for_region(&self, regionCode: &HStringArg, stripNonDigit: bool) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetNationalDirectDialingPrefixForRegion)(self as *const _ as *mut _, regionCode.get(), stripNonDigit, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn wrap_with_left_to_right_markers(&self, number: &HStringArg) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).WrapWithLeftToRightMarkers)(self as *const _ as *mut _, number.get(), &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
}
DEFINE_IID!(IID_IPhoneNumberInfo, 477947101, 51380, 20131, 154, 239, 179, 66, 226, 197, 180, 23);
RT_INTERFACE!{interface IPhoneNumberInfo(IPhoneNumberInfoVtbl): IInspectable(IInspectableVtbl) [IID_IPhoneNumberInfo] {
    fn get_CountryCode(&self, out: *mut i32) -> HRESULT,
    fn get_PhoneNumber(&self, out: *mut HSTRING) -> HRESULT,
    fn GetLengthOfGeographicalAreaCode(&self, out: *mut i32) -> HRESULT,
    fn GetNationalSignificantNumber(&self, out: *mut HSTRING) -> HRESULT,
    fn GetLengthOfNationalDestinationCode(&self, out: *mut i32) -> HRESULT,
    fn PredictNumberKind(&self, out: *mut PredictedPhoneNumberKind) -> HRESULT,
    fn GetGeographicRegionCode(&self, out: *mut HSTRING) -> HRESULT,
    fn CheckNumberMatch(&self, otherNumber: *mut PhoneNumberInfo, out: *mut PhoneNumberMatchResult) -> HRESULT
}}
impl IPhoneNumberInfo {
    #[inline] pub unsafe fn get_country_code(&self) -> Result<i32> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_CountryCode)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_phone_number(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_PhoneNumber)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_length_of_geographical_area_code(&self) -> Result<i32> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).GetLengthOfGeographicalAreaCode)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_national_significant_number(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetNationalSignificantNumber)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_length_of_national_destination_code(&self) -> Result<i32> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).GetLengthOfNationalDestinationCode)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn predict_number_kind(&self) -> Result<PredictedPhoneNumberKind> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).PredictNumberKind)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_geographic_region_code(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetGeographicRegionCode)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn check_number_match(&self, otherNumber: &PhoneNumberInfo) -> Result<PhoneNumberMatchResult> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).CheckNumberMatch)(self as *const _ as *mut _, otherNumber as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
}
RT_CLASS!{class PhoneNumberInfo: IPhoneNumberInfo}
impl RtActivatable<IPhoneNumberInfoFactory> for PhoneNumberInfo {}
impl RtActivatable<IPhoneNumberInfoStatics> for PhoneNumberInfo {}
impl PhoneNumberInfo {
    #[inline] pub fn create(number: &HStringArg) -> Result<ComPtr<PhoneNumberInfo>> { unsafe {
        <Self as RtActivatable<IPhoneNumberInfoFactory>>::get_activation_factory().create(number)
    }}
    #[inline] pub fn try_parse(input: &HStringArg) -> Result<(ComPtr<PhoneNumberInfo>, PhoneNumberParseResult)> { unsafe {
        <Self as RtActivatable<IPhoneNumberInfoStatics>>::get_activation_factory().try_parse(input)
    }}
    #[inline] pub fn try_parse_with_region(input: &HStringArg, regionCode: &HStringArg) -> Result<(ComPtr<PhoneNumberInfo>, PhoneNumberParseResult)> { unsafe {
        <Self as RtActivatable<IPhoneNumberInfoStatics>>::get_activation_factory().try_parse_with_region(input, regionCode)
    }}
}
DEFINE_CLSID!(PhoneNumberInfo(&[87,105,110,100,111,119,115,46,71,108,111,98,97,108,105,122,97,116,105,111,110,46,80,104,111,110,101,78,117,109,98,101,114,70,111,114,109,97,116,116,105,110,103,46,80,104,111,110,101,78,117,109,98,101,114,73,110,102,111,0]) [CLSID_PhoneNumberInfo]);
DEFINE_IID!(IID_IPhoneNumberInfoFactory, 2181216612, 44458, 19711, 143, 207, 23, 231, 81, 106, 40, 255);
RT_INTERFACE!{static interface IPhoneNumberInfoFactory(IPhoneNumberInfoFactoryVtbl): IInspectable(IInspectableVtbl) [IID_IPhoneNumberInfoFactory] {
    fn Create(&self, number: HSTRING, out: *mut *mut PhoneNumberInfo) -> HRESULT
}}
impl IPhoneNumberInfoFactory {
    #[inline] pub unsafe fn create(&self, number: &HStringArg) -> Result<ComPtr<PhoneNumberInfo>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).Create)(self as *const _ as *mut _, number.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
DEFINE_IID!(IID_IPhoneNumberInfoStatics, 1530875754, 34473, 16617, 134, 73, 109, 97, 22, 25, 40, 212);
RT_INTERFACE!{static interface IPhoneNumberInfoStatics(IPhoneNumberInfoStaticsVtbl): IInspectable(IInspectableVtbl) [IID_IPhoneNumberInfoStatics] {
    fn TryParse(&self, input: HSTRING, phoneNumber: *mut *mut PhoneNumberInfo, out: *mut PhoneNumberParseResult) -> HRESULT,
    fn TryParseWithRegion(&self, input: HSTRING, regionCode: HSTRING, phoneNumber: *mut *mut PhoneNumberInfo, out: *mut PhoneNumberParseResult) -> HRESULT
}}
impl IPhoneNumberInfoStatics {
    #[inline] pub unsafe fn try_parse(&self, input: &HStringArg) -> Result<(ComPtr<PhoneNumberInfo>, PhoneNumberParseResult)> {
        let mut phoneNumber = null_mut(); let mut out = zeroed();
        let hr = ((*self.lpVtbl).TryParse)(self as *const _ as *mut _, input.get(), &mut phoneNumber, &mut out);
        if hr == S_OK { Ok((ComPtr::wrap(phoneNumber), out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn try_parse_with_region(&self, input: &HStringArg, regionCode: &HStringArg) -> Result<(ComPtr<PhoneNumberInfo>, PhoneNumberParseResult)> {
        let mut phoneNumber = null_mut(); let mut out = zeroed();
        let hr = ((*self.lpVtbl).TryParseWithRegion)(self as *const _ as *mut _, input.get(), regionCode.get(), &mut phoneNumber, &mut out);
        if hr == S_OK { Ok((ComPtr::wrap(phoneNumber), out)) } else { err(hr) }
    }
}
RT_ENUM! { enum PhoneNumberMatchResult: i32 {
    NoMatch (PhoneNumberMatchResult_NoMatch) = 0, ShortNationalSignificantNumberMatch (PhoneNumberMatchResult_ShortNationalSignificantNumberMatch) = 1, NationalSignificantNumberMatch (PhoneNumberMatchResult_NationalSignificantNumberMatch) = 2, ExactMatch (PhoneNumberMatchResult_ExactMatch) = 3,
}}
RT_ENUM! { enum PhoneNumberParseResult: i32 {
    Valid (PhoneNumberParseResult_Valid) = 0, NotANumber (PhoneNumberParseResult_NotANumber) = 1, InvalidCountryCode (PhoneNumberParseResult_InvalidCountryCode) = 2, TooShort (PhoneNumberParseResult_TooShort) = 3, TooLong (PhoneNumberParseResult_TooLong) = 4,
}}
RT_ENUM! { enum PredictedPhoneNumberKind: i32 {
    FixedLine (PredictedPhoneNumberKind_FixedLine) = 0, Mobile (PredictedPhoneNumberKind_Mobile) = 1, FixedLineOrMobile (PredictedPhoneNumberKind_FixedLineOrMobile) = 2, TollFree (PredictedPhoneNumberKind_TollFree) = 3, PremiumRate (PredictedPhoneNumberKind_PremiumRate) = 4, SharedCost (PredictedPhoneNumberKind_SharedCost) = 5, Voip (PredictedPhoneNumberKind_Voip) = 6, PersonalNumber (PredictedPhoneNumberKind_PersonalNumber) = 7, Pager (PredictedPhoneNumberKind_Pager) = 8, UniversalAccountNumber (PredictedPhoneNumberKind_UniversalAccountNumber) = 9, Voicemail (PredictedPhoneNumberKind_Voicemail) = 10, Unknown (PredictedPhoneNumberKind_Unknown) = 11,
}}
} // Windows.Globalization.PhoneNumberFormatting
pub mod datetimeformatting { // Windows.Globalization.DateTimeFormatting
use ::prelude::*;
DEFINE_IID!(IID_IDateTimeFormatter, 2515454480, 29664, 20043, 161, 131, 61, 106, 208, 186, 53, 236);
RT_INTERFACE!{interface IDateTimeFormatter(IDateTimeFormatterVtbl): IInspectable(IInspectableVtbl) [IID_IDateTimeFormatter] {
    fn get_Languages(&self, out: *mut *mut super::super::foundation::collections::IVectorView<HString>) -> HRESULT,
    fn get_GeographicRegion(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Calendar(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Clock(&self, out: *mut HSTRING) -> HRESULT,
    fn get_NumeralSystem(&self, out: *mut HSTRING) -> HRESULT,
    fn put_NumeralSystem(&self, value: HSTRING) -> HRESULT,
    fn get_Patterns(&self, out: *mut *mut super::super::foundation::collections::IVectorView<HString>) -> HRESULT,
    fn get_Template(&self, out: *mut HSTRING) -> HRESULT,
    fn Format(&self, value: super::super::foundation::DateTime, out: *mut HSTRING) -> HRESULT,
    fn get_IncludeYear(&self, out: *mut YearFormat) -> HRESULT,
    fn get_IncludeMonth(&self, out: *mut MonthFormat) -> HRESULT,
    fn get_IncludeDayOfWeek(&self, out: *mut DayOfWeekFormat) -> HRESULT,
    fn get_IncludeDay(&self, out: *mut DayFormat) -> HRESULT,
    fn get_IncludeHour(&self, out: *mut HourFormat) -> HRESULT,
    fn get_IncludeMinute(&self, out: *mut MinuteFormat) -> HRESULT,
    fn get_IncludeSecond(&self, out: *mut SecondFormat) -> HRESULT,
    fn get_ResolvedLanguage(&self, out: *mut HSTRING) -> HRESULT,
    fn get_ResolvedGeographicRegion(&self, out: *mut HSTRING) -> HRESULT
}}
impl IDateTimeFormatter {
    #[inline] pub unsafe fn get_languages(&self) -> Result<ComPtr<super::super::foundation::collections::IVectorView<HString>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Languages)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_geographic_region(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_GeographicRegion)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_calendar(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Calendar)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_clock(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Clock)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_numeral_system(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_NumeralSystem)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_numeral_system(&self, value: &HStringArg) -> Result<()> {
        let hr = ((*self.lpVtbl).put_NumeralSystem)(self as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_patterns(&self) -> Result<ComPtr<super::super::foundation::collections::IVectorView<HString>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Patterns)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_template(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Template)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn format(&self, value: super::super::foundation::DateTime) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).Format)(self as *const _ as *mut _, value, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_include_year(&self) -> Result<YearFormat> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_IncludeYear)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_include_month(&self) -> Result<MonthFormat> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_IncludeMonth)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_include_day_of_week(&self) -> Result<DayOfWeekFormat> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_IncludeDayOfWeek)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_include_day(&self) -> Result<DayFormat> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_IncludeDay)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_include_hour(&self) -> Result<HourFormat> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_IncludeHour)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_include_minute(&self) -> Result<MinuteFormat> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_IncludeMinute)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_include_second(&self) -> Result<SecondFormat> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_IncludeSecond)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_resolved_language(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_ResolvedLanguage)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_resolved_geographic_region(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_ResolvedGeographicRegion)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
}
RT_CLASS!{class DateTimeFormatter: IDateTimeFormatter}
impl RtActivatable<IDateTimeFormatterFactory> for DateTimeFormatter {}
impl RtActivatable<IDateTimeFormatterStatics> for DateTimeFormatter {}
impl DateTimeFormatter {
    #[inline] pub fn create_date_time_formatter(formatTemplate: &HStringArg) -> Result<ComPtr<DateTimeFormatter>> { unsafe {
        <Self as RtActivatable<IDateTimeFormatterFactory>>::get_activation_factory().create_date_time_formatter(formatTemplate)
    }}
    #[inline] pub fn create_date_time_formatter_languages(formatTemplate: &HStringArg, languages: &super::super::foundation::collections::IIterable<HString>) -> Result<ComPtr<DateTimeFormatter>> { unsafe {
        <Self as RtActivatable<IDateTimeFormatterFactory>>::get_activation_factory().create_date_time_formatter_languages(formatTemplate, languages)
    }}
    #[inline] pub fn create_date_time_formatter_context(formatTemplate: &HStringArg, languages: &super::super::foundation::collections::IIterable<HString>, geographicRegion: &HStringArg, calendar: &HStringArg, clock: &HStringArg) -> Result<ComPtr<DateTimeFormatter>> { unsafe {
        <Self as RtActivatable<IDateTimeFormatterFactory>>::get_activation_factory().create_date_time_formatter_context(formatTemplate, languages, geographicRegion, calendar, clock)
    }}
    #[inline] pub fn create_date_time_formatter_date(yearFormat: YearFormat, monthFormat: MonthFormat, dayFormat: DayFormat, dayOfWeekFormat: DayOfWeekFormat) -> Result<ComPtr<DateTimeFormatter>> { unsafe {
        <Self as RtActivatable<IDateTimeFormatterFactory>>::get_activation_factory().create_date_time_formatter_date(yearFormat, monthFormat, dayFormat, dayOfWeekFormat)
    }}
    #[inline] pub fn create_date_time_formatter_time(hourFormat: HourFormat, minuteFormat: MinuteFormat, secondFormat: SecondFormat) -> Result<ComPtr<DateTimeFormatter>> { unsafe {
        <Self as RtActivatable<IDateTimeFormatterFactory>>::get_activation_factory().create_date_time_formatter_time(hourFormat, minuteFormat, secondFormat)
    }}
    #[inline] pub fn create_date_time_formatter_date_time_languages(yearFormat: YearFormat, monthFormat: MonthFormat, dayFormat: DayFormat, dayOfWeekFormat: DayOfWeekFormat, hourFormat: HourFormat, minuteFormat: MinuteFormat, secondFormat: SecondFormat, languages: &super::super::foundation::collections::IIterable<HString>) -> Result<ComPtr<DateTimeFormatter>> { unsafe {
        <Self as RtActivatable<IDateTimeFormatterFactory>>::get_activation_factory().create_date_time_formatter_date_time_languages(yearFormat, monthFormat, dayFormat, dayOfWeekFormat, hourFormat, minuteFormat, secondFormat, languages)
    }}
    #[inline] pub fn create_date_time_formatter_date_time_context(yearFormat: YearFormat, monthFormat: MonthFormat, dayFormat: DayFormat, dayOfWeekFormat: DayOfWeekFormat, hourFormat: HourFormat, minuteFormat: MinuteFormat, secondFormat: SecondFormat, languages: &super::super::foundation::collections::IIterable<HString>, geographicRegion: &HStringArg, calendar: &HStringArg, clock: &HStringArg) -> Result<ComPtr<DateTimeFormatter>> { unsafe {
        <Self as RtActivatable<IDateTimeFormatterFactory>>::get_activation_factory().create_date_time_formatter_date_time_context(yearFormat, monthFormat, dayFormat, dayOfWeekFormat, hourFormat, minuteFormat, secondFormat, languages, geographicRegion, calendar, clock)
    }}
    #[inline] pub fn get_long_date() -> Result<ComPtr<DateTimeFormatter>> { unsafe {
        <Self as RtActivatable<IDateTimeFormatterStatics>>::get_activation_factory().get_long_date()
    }}
    #[inline] pub fn get_long_time() -> Result<ComPtr<DateTimeFormatter>> { unsafe {
        <Self as RtActivatable<IDateTimeFormatterStatics>>::get_activation_factory().get_long_time()
    }}
    #[inline] pub fn get_short_date() -> Result<ComPtr<DateTimeFormatter>> { unsafe {
        <Self as RtActivatable<IDateTimeFormatterStatics>>::get_activation_factory().get_short_date()
    }}
    #[inline] pub fn get_short_time() -> Result<ComPtr<DateTimeFormatter>> { unsafe {
        <Self as RtActivatable<IDateTimeFormatterStatics>>::get_activation_factory().get_short_time()
    }}
}
DEFINE_CLSID!(DateTimeFormatter(&[87,105,110,100,111,119,115,46,71,108,111,98,97,108,105,122,97,116,105,111,110,46,68,97,116,101,84,105,109,101,70,111,114,109,97,116,116,105,110,103,46,68,97,116,101,84,105,109,101,70,111,114,109,97,116,116,101,114,0]) [CLSID_DateTimeFormatter]);
DEFINE_IID!(IID_IDateTimeFormatter2, 667490950, 48554, 20432, 158, 54, 103, 29, 90, 165, 238, 3);
RT_INTERFACE!{interface IDateTimeFormatter2(IDateTimeFormatter2Vtbl): IInspectable(IInspectableVtbl) [IID_IDateTimeFormatter2] {
    fn FormatUsingTimeZone(&self, datetime: super::super::foundation::DateTime, timeZoneId: HSTRING, out: *mut HSTRING) -> HRESULT
}}
impl IDateTimeFormatter2 {
    #[inline] pub unsafe fn format_using_time_zone(&self, datetime: super::super::foundation::DateTime, timeZoneId: &HStringArg) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).FormatUsingTimeZone)(self as *const _ as *mut _, datetime, timeZoneId.get(), &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
}
DEFINE_IID!(IID_IDateTimeFormatterFactory, 3968698963, 6702, 16685, 136, 21, 59, 116, 95, 177, 162, 160);
RT_INTERFACE!{static interface IDateTimeFormatterFactory(IDateTimeFormatterFactoryVtbl): IInspectable(IInspectableVtbl) [IID_IDateTimeFormatterFactory] {
    fn CreateDateTimeFormatter(&self, formatTemplate: HSTRING, out: *mut *mut DateTimeFormatter) -> HRESULT,
    fn CreateDateTimeFormatterLanguages(&self, formatTemplate: HSTRING, languages: *mut super::super::foundation::collections::IIterable<HString>, out: *mut *mut DateTimeFormatter) -> HRESULT,
    fn CreateDateTimeFormatterContext(&self, formatTemplate: HSTRING, languages: *mut super::super::foundation::collections::IIterable<HString>, geographicRegion: HSTRING, calendar: HSTRING, clock: HSTRING, out: *mut *mut DateTimeFormatter) -> HRESULT,
    fn CreateDateTimeFormatterDate(&self, yearFormat: YearFormat, monthFormat: MonthFormat, dayFormat: DayFormat, dayOfWeekFormat: DayOfWeekFormat, out: *mut *mut DateTimeFormatter) -> HRESULT,
    fn CreateDateTimeFormatterTime(&self, hourFormat: HourFormat, minuteFormat: MinuteFormat, secondFormat: SecondFormat, out: *mut *mut DateTimeFormatter) -> HRESULT,
    fn CreateDateTimeFormatterDateTimeLanguages(&self, yearFormat: YearFormat, monthFormat: MonthFormat, dayFormat: DayFormat, dayOfWeekFormat: DayOfWeekFormat, hourFormat: HourFormat, minuteFormat: MinuteFormat, secondFormat: SecondFormat, languages: *mut super::super::foundation::collections::IIterable<HString>, out: *mut *mut DateTimeFormatter) -> HRESULT,
    fn CreateDateTimeFormatterDateTimeContext(&self, yearFormat: YearFormat, monthFormat: MonthFormat, dayFormat: DayFormat, dayOfWeekFormat: DayOfWeekFormat, hourFormat: HourFormat, minuteFormat: MinuteFormat, secondFormat: SecondFormat, languages: *mut super::super::foundation::collections::IIterable<HString>, geographicRegion: HSTRING, calendar: HSTRING, clock: HSTRING, out: *mut *mut DateTimeFormatter) -> HRESULT
}}
impl IDateTimeFormatterFactory {
    #[inline] pub unsafe fn create_date_time_formatter(&self, formatTemplate: &HStringArg) -> Result<ComPtr<DateTimeFormatter>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).CreateDateTimeFormatter)(self as *const _ as *mut _, formatTemplate.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn create_date_time_formatter_languages(&self, formatTemplate: &HStringArg, languages: &super::super::foundation::collections::IIterable<HString>) -> Result<ComPtr<DateTimeFormatter>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).CreateDateTimeFormatterLanguages)(self as *const _ as *mut _, formatTemplate.get(), languages as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn create_date_time_formatter_context(&self, formatTemplate: &HStringArg, languages: &super::super::foundation::collections::IIterable<HString>, geographicRegion: &HStringArg, calendar: &HStringArg, clock: &HStringArg) -> Result<ComPtr<DateTimeFormatter>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).CreateDateTimeFormatterContext)(self as *const _ as *mut _, formatTemplate.get(), languages as *const _ as *mut _, geographicRegion.get(), calendar.get(), clock.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn create_date_time_formatter_date(&self, yearFormat: YearFormat, monthFormat: MonthFormat, dayFormat: DayFormat, dayOfWeekFormat: DayOfWeekFormat) -> Result<ComPtr<DateTimeFormatter>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).CreateDateTimeFormatterDate)(self as *const _ as *mut _, yearFormat, monthFormat, dayFormat, dayOfWeekFormat, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn create_date_time_formatter_time(&self, hourFormat: HourFormat, minuteFormat: MinuteFormat, secondFormat: SecondFormat) -> Result<ComPtr<DateTimeFormatter>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).CreateDateTimeFormatterTime)(self as *const _ as *mut _, hourFormat, minuteFormat, secondFormat, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn create_date_time_formatter_date_time_languages(&self, yearFormat: YearFormat, monthFormat: MonthFormat, dayFormat: DayFormat, dayOfWeekFormat: DayOfWeekFormat, hourFormat: HourFormat, minuteFormat: MinuteFormat, secondFormat: SecondFormat, languages: &super::super::foundation::collections::IIterable<HString>) -> Result<ComPtr<DateTimeFormatter>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).CreateDateTimeFormatterDateTimeLanguages)(self as *const _ as *mut _, yearFormat, monthFormat, dayFormat, dayOfWeekFormat, hourFormat, minuteFormat, secondFormat, languages as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn create_date_time_formatter_date_time_context(&self, yearFormat: YearFormat, monthFormat: MonthFormat, dayFormat: DayFormat, dayOfWeekFormat: DayOfWeekFormat, hourFormat: HourFormat, minuteFormat: MinuteFormat, secondFormat: SecondFormat, languages: &super::super::foundation::collections::IIterable<HString>, geographicRegion: &HStringArg, calendar: &HStringArg, clock: &HStringArg) -> Result<ComPtr<DateTimeFormatter>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).CreateDateTimeFormatterDateTimeContext)(self as *const _ as *mut _, yearFormat, monthFormat, dayFormat, dayOfWeekFormat, hourFormat, minuteFormat, secondFormat, languages as *const _ as *mut _, geographicRegion.get(), calendar.get(), clock.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
DEFINE_IID!(IID_IDateTimeFormatterStatics, 3217942464, 57164, 18990, 144, 18, 244, 125, 175, 63, 18, 18);
RT_INTERFACE!{static interface IDateTimeFormatterStatics(IDateTimeFormatterStaticsVtbl): IInspectable(IInspectableVtbl) [IID_IDateTimeFormatterStatics] {
    fn get_LongDate(&self, out: *mut *mut DateTimeFormatter) -> HRESULT,
    fn get_LongTime(&self, out: *mut *mut DateTimeFormatter) -> HRESULT,
    fn get_ShortDate(&self, out: *mut *mut DateTimeFormatter) -> HRESULT,
    fn get_ShortTime(&self, out: *mut *mut DateTimeFormatter) -> HRESULT
}}
impl IDateTimeFormatterStatics {
    #[inline] pub unsafe fn get_long_date(&self) -> Result<ComPtr<DateTimeFormatter>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_LongDate)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_long_time(&self) -> Result<ComPtr<DateTimeFormatter>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_LongTime)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_short_date(&self) -> Result<ComPtr<DateTimeFormatter>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_ShortDate)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_short_time(&self) -> Result<ComPtr<DateTimeFormatter>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_ShortTime)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
RT_ENUM! { enum DayFormat: i32 {
    None (DayFormat_None) = 0, Default (DayFormat_Default) = 1,
}}
RT_ENUM! { enum DayOfWeekFormat: i32 {
    None (DayOfWeekFormat_None) = 0, Default (DayOfWeekFormat_Default) = 1, Abbreviated (DayOfWeekFormat_Abbreviated) = 2, Full (DayOfWeekFormat_Full) = 3,
}}
RT_ENUM! { enum HourFormat: i32 {
    None (HourFormat_None) = 0, Default (HourFormat_Default) = 1,
}}
RT_ENUM! { enum MinuteFormat: i32 {
    None (MinuteFormat_None) = 0, Default (MinuteFormat_Default) = 1,
}}
RT_ENUM! { enum MonthFormat: i32 {
    None (MonthFormat_None) = 0, Default (MonthFormat_Default) = 1, Abbreviated (MonthFormat_Abbreviated) = 2, Full (MonthFormat_Full) = 3, Numeric (MonthFormat_Numeric) = 4,
}}
RT_ENUM! { enum SecondFormat: i32 {
    None (SecondFormat_None) = 0, Default (SecondFormat_Default) = 1,
}}
RT_ENUM! { enum YearFormat: i32 {
    None (YearFormat_None) = 0, Default (YearFormat_Default) = 1, Abbreviated (YearFormat_Abbreviated) = 2, Full (YearFormat_Full) = 3,
}}
} // Windows.Globalization.DateTimeFormatting
pub mod numberformatting { // Windows.Globalization.NumberFormatting
use ::prelude::*;
DEFINE_IID!(IID_ICurrencyFormatter, 292752549, 19200, 16818, 179, 50, 115, 177, 42, 73, 125, 84);
RT_INTERFACE!{interface ICurrencyFormatter(ICurrencyFormatterVtbl): IInspectable(IInspectableVtbl) [IID_ICurrencyFormatter] {
    fn get_Currency(&self, out: *mut HSTRING) -> HRESULT,
    fn put_Currency(&self, value: HSTRING) -> HRESULT
}}
impl ICurrencyFormatter {
    #[inline] pub unsafe fn get_currency(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Currency)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_currency(&self, value: &HStringArg) -> Result<()> {
        let hr = ((*self.lpVtbl).put_Currency)(self as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
}
RT_CLASS!{class CurrencyFormatter: ICurrencyFormatter}
impl RtActivatable<ICurrencyFormatterFactory> for CurrencyFormatter {}
impl CurrencyFormatter {
    #[inline] pub fn create_currency_formatter_code(currencyCode: &HStringArg) -> Result<ComPtr<CurrencyFormatter>> { unsafe {
        <Self as RtActivatable<ICurrencyFormatterFactory>>::get_activation_factory().create_currency_formatter_code(currencyCode)
    }}
    #[inline] pub fn create_currency_formatter_code_context(currencyCode: &HStringArg, languages: &super::super::foundation::collections::IIterable<HString>, geographicRegion: &HStringArg) -> Result<ComPtr<CurrencyFormatter>> { unsafe {
        <Self as RtActivatable<ICurrencyFormatterFactory>>::get_activation_factory().create_currency_formatter_code_context(currencyCode, languages, geographicRegion)
    }}
}
DEFINE_CLSID!(CurrencyFormatter(&[87,105,110,100,111,119,115,46,71,108,111,98,97,108,105,122,97,116,105,111,110,46,78,117,109,98,101,114,70,111,114,109,97,116,116,105,110,103,46,67,117,114,114,101,110,99,121,70,111,114,109,97,116,116,101,114,0]) [CLSID_CurrencyFormatter]);
DEFINE_IID!(IID_ICurrencyFormatter2, 120336157, 59322, 16791, 146, 14, 36, 124, 146, 247, 222, 166);
RT_INTERFACE!{interface ICurrencyFormatter2(ICurrencyFormatter2Vtbl): IInspectable(IInspectableVtbl) [IID_ICurrencyFormatter2] {
    fn get_Mode(&self, out: *mut CurrencyFormatterMode) -> HRESULT,
    fn put_Mode(&self, value: CurrencyFormatterMode) -> HRESULT,
    fn ApplyRoundingForCurrency(&self, roundingAlgorithm: RoundingAlgorithm) -> HRESULT
}}
impl ICurrencyFormatter2 {
    #[inline] pub unsafe fn get_mode(&self) -> Result<CurrencyFormatterMode> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_Mode)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_mode(&self, value: CurrencyFormatterMode) -> Result<()> {
        let hr = ((*self.lpVtbl).put_Mode)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn apply_rounding_for_currency(&self, roundingAlgorithm: RoundingAlgorithm) -> Result<()> {
        let hr = ((*self.lpVtbl).ApplyRoundingForCurrency)(self as *const _ as *mut _, roundingAlgorithm);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
}
DEFINE_IID!(IID_ICurrencyFormatterFactory, 2261209982, 47416, 19106, 132, 176, 44, 51, 220, 91, 20, 80);
RT_INTERFACE!{static interface ICurrencyFormatterFactory(ICurrencyFormatterFactoryVtbl): IInspectable(IInspectableVtbl) [IID_ICurrencyFormatterFactory] {
    fn CreateCurrencyFormatterCode(&self, currencyCode: HSTRING, out: *mut *mut CurrencyFormatter) -> HRESULT,
    fn CreateCurrencyFormatterCodeContext(&self, currencyCode: HSTRING, languages: *mut super::super::foundation::collections::IIterable<HString>, geographicRegion: HSTRING, out: *mut *mut CurrencyFormatter) -> HRESULT
}}
impl ICurrencyFormatterFactory {
    #[inline] pub unsafe fn create_currency_formatter_code(&self, currencyCode: &HStringArg) -> Result<ComPtr<CurrencyFormatter>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).CreateCurrencyFormatterCode)(self as *const _ as *mut _, currencyCode.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn create_currency_formatter_code_context(&self, currencyCode: &HStringArg, languages: &super::super::foundation::collections::IIterable<HString>, geographicRegion: &HStringArg) -> Result<ComPtr<CurrencyFormatter>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).CreateCurrencyFormatterCodeContext)(self as *const _ as *mut _, currencyCode.get(), languages as *const _ as *mut _, geographicRegion.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
RT_ENUM! { enum CurrencyFormatterMode: i32 {
    UseSymbol (CurrencyFormatterMode_UseSymbol) = 0, UseCurrencyCode (CurrencyFormatterMode_UseCurrencyCode) = 1,
}}
RT_CLASS!{class DecimalFormatter: INumberFormatter}
impl RtActivatable<IDecimalFormatterFactory> for DecimalFormatter {}
impl RtActivatable<IActivationFactory> for DecimalFormatter {}
impl DecimalFormatter {
    #[inline] pub fn create_decimal_formatter(languages: &super::super::foundation::collections::IIterable<HString>, geographicRegion: &HStringArg) -> Result<ComPtr<DecimalFormatter>> { unsafe {
        <Self as RtActivatable<IDecimalFormatterFactory>>::get_activation_factory().create_decimal_formatter(languages, geographicRegion)
    }}
}
DEFINE_CLSID!(DecimalFormatter(&[87,105,110,100,111,119,115,46,71,108,111,98,97,108,105,122,97,116,105,111,110,46,78,117,109,98,101,114,70,111,114,109,97,116,116,105,110,103,46,68,101,99,105,109,97,108,70,111,114,109,97,116,116,101,114,0]) [CLSID_DecimalFormatter]);
DEFINE_IID!(IID_IDecimalFormatterFactory, 218205338, 58259, 18104, 184, 48, 122, 105, 200, 248, 159, 187);
RT_INTERFACE!{static interface IDecimalFormatterFactory(IDecimalFormatterFactoryVtbl): IInspectable(IInspectableVtbl) [IID_IDecimalFormatterFactory] {
    fn CreateDecimalFormatter(&self, languages: *mut super::super::foundation::collections::IIterable<HString>, geographicRegion: HSTRING, out: *mut *mut DecimalFormatter) -> HRESULT
}}
impl IDecimalFormatterFactory {
    #[inline] pub unsafe fn create_decimal_formatter(&self, languages: &super::super::foundation::collections::IIterable<HString>, geographicRegion: &HStringArg) -> Result<ComPtr<DecimalFormatter>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).CreateDecimalFormatter)(self as *const _ as *mut _, languages as *const _ as *mut _, geographicRegion.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
DEFINE_IID!(IID_IIncrementNumberRounder, 1889947640, 26283, 16725, 157, 161, 115, 158, 70, 118, 69, 67);
RT_INTERFACE!{interface IIncrementNumberRounder(IIncrementNumberRounderVtbl): IInspectable(IInspectableVtbl) [IID_IIncrementNumberRounder] {
    fn get_RoundingAlgorithm(&self, out: *mut RoundingAlgorithm) -> HRESULT,
    fn put_RoundingAlgorithm(&self, value: RoundingAlgorithm) -> HRESULT,
    fn get_Increment(&self, out: *mut f64) -> HRESULT,
    fn put_Increment(&self, value: f64) -> HRESULT
}}
impl IIncrementNumberRounder {
    #[inline] pub unsafe fn get_rounding_algorithm(&self) -> Result<RoundingAlgorithm> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_RoundingAlgorithm)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_rounding_algorithm(&self, value: RoundingAlgorithm) -> Result<()> {
        let hr = ((*self.lpVtbl).put_RoundingAlgorithm)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_increment(&self) -> Result<f64> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_Increment)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_increment(&self, value: f64) -> Result<()> {
        let hr = ((*self.lpVtbl).put_Increment)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
}
RT_CLASS!{class IncrementNumberRounder: INumberRounder}
impl RtActivatable<IActivationFactory> for IncrementNumberRounder {}
DEFINE_CLSID!(IncrementNumberRounder(&[87,105,110,100,111,119,115,46,71,108,111,98,97,108,105,122,97,116,105,111,110,46,78,117,109,98,101,114,70,111,114,109,97,116,116,105,110,103,46,73,110,99,114,101,109,101,110,116,78,117,109,98,101,114,82,111,117,110,100,101,114,0]) [CLSID_IncrementNumberRounder]);
DEFINE_IID!(IID_INumberFormatter, 2768272457, 30326, 19895, 134, 49, 27, 111, 242, 101, 202, 169);
RT_INTERFACE!{interface INumberFormatter(INumberFormatterVtbl): IInspectable(IInspectableVtbl) [IID_INumberFormatter] {
    fn FormatInt(&self, value: i64, out: *mut HSTRING) -> HRESULT,
    fn FormatUInt(&self, value: u64, out: *mut HSTRING) -> HRESULT,
    fn FormatDouble(&self, value: f64, out: *mut HSTRING) -> HRESULT
}}
impl INumberFormatter {
    #[inline] pub unsafe fn format_int(&self, value: i64) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).FormatInt)(self as *const _ as *mut _, value, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn format_uint(&self, value: u64) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).FormatUInt)(self as *const _ as *mut _, value, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn format_double(&self, value: f64) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).FormatDouble)(self as *const _ as *mut _, value, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
}
DEFINE_IID!(IID_INumberFormatter2, 3567829488, 32976, 19213, 168, 158, 136, 44, 30, 143, 131, 16);
RT_INTERFACE!{interface INumberFormatter2(INumberFormatter2Vtbl): IInspectable(IInspectableVtbl) [IID_INumberFormatter2] {
    fn FormatInt(&self, value: i64, out: *mut HSTRING) -> HRESULT,
    fn FormatUInt(&self, value: u64, out: *mut HSTRING) -> HRESULT,
    fn FormatDouble(&self, value: f64, out: *mut HSTRING) -> HRESULT
}}
impl INumberFormatter2 {
    #[inline] pub unsafe fn format_int(&self, value: i64) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).FormatInt)(self as *const _ as *mut _, value, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn format_uint(&self, value: u64) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).FormatUInt)(self as *const _ as *mut _, value, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn format_double(&self, value: f64) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).FormatDouble)(self as *const _ as *mut _, value, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
}
DEFINE_IID!(IID_INumberFormatterOptions, 2150837537, 44769, 19001, 186, 162, 7, 237, 140, 150, 218, 246);
RT_INTERFACE!{interface INumberFormatterOptions(INumberFormatterOptionsVtbl): IInspectable(IInspectableVtbl) [IID_INumberFormatterOptions] {
    fn get_Languages(&self, out: *mut *mut super::super::foundation::collections::IVectorView<HString>) -> HRESULT,
    fn get_GeographicRegion(&self, out: *mut HSTRING) -> HRESULT,
    fn get_IntegerDigits(&self, out: *mut i32) -> HRESULT,
    fn put_IntegerDigits(&self, value: i32) -> HRESULT,
    fn get_FractionDigits(&self, out: *mut i32) -> HRESULT,
    fn put_FractionDigits(&self, value: i32) -> HRESULT,
    fn get_IsGrouped(&self, out: *mut bool) -> HRESULT,
    fn put_IsGrouped(&self, value: bool) -> HRESULT,
    fn get_IsDecimalPointAlwaysDisplayed(&self, out: *mut bool) -> HRESULT,
    fn put_IsDecimalPointAlwaysDisplayed(&self, value: bool) -> HRESULT,
    fn get_NumeralSystem(&self, out: *mut HSTRING) -> HRESULT,
    fn put_NumeralSystem(&self, value: HSTRING) -> HRESULT,
    fn get_ResolvedLanguage(&self, out: *mut HSTRING) -> HRESULT,
    fn get_ResolvedGeographicRegion(&self, out: *mut HSTRING) -> HRESULT
}}
impl INumberFormatterOptions {
    #[inline] pub unsafe fn get_languages(&self) -> Result<ComPtr<super::super::foundation::collections::IVectorView<HString>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Languages)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_geographic_region(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_GeographicRegion)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_integer_digits(&self) -> Result<i32> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_IntegerDigits)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_integer_digits(&self, value: i32) -> Result<()> {
        let hr = ((*self.lpVtbl).put_IntegerDigits)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_fraction_digits(&self) -> Result<i32> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_FractionDigits)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_fraction_digits(&self, value: i32) -> Result<()> {
        let hr = ((*self.lpVtbl).put_FractionDigits)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_is_grouped(&self) -> Result<bool> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_IsGrouped)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_is_grouped(&self, value: bool) -> Result<()> {
        let hr = ((*self.lpVtbl).put_IsGrouped)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_is_decimal_point_always_displayed(&self) -> Result<bool> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_IsDecimalPointAlwaysDisplayed)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_is_decimal_point_always_displayed(&self, value: bool) -> Result<()> {
        let hr = ((*self.lpVtbl).put_IsDecimalPointAlwaysDisplayed)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_numeral_system(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_NumeralSystem)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_numeral_system(&self, value: &HStringArg) -> Result<()> {
        let hr = ((*self.lpVtbl).put_NumeralSystem)(self as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_resolved_language(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_ResolvedLanguage)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_resolved_geographic_region(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_ResolvedGeographicRegion)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
}
DEFINE_IID!(IID_INumberParser, 3865416722, 18963, 19027, 131, 161, 57, 47, 190, 76, 255, 159);
RT_INTERFACE!{interface INumberParser(INumberParserVtbl): IInspectable(IInspectableVtbl) [IID_INumberParser] {
    fn ParseInt(&self, text: HSTRING, out: *mut *mut super::super::foundation::IReference<i64>) -> HRESULT,
    fn ParseUInt(&self, text: HSTRING, out: *mut *mut super::super::foundation::IReference<u64>) -> HRESULT,
    fn ParseDouble(&self, text: HSTRING, out: *mut *mut super::super::foundation::IReference<f64>) -> HRESULT
}}
impl INumberParser {
    #[inline] pub unsafe fn parse_int(&self, text: &HStringArg) -> Result<ComPtr<super::super::foundation::IReference<i64>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).ParseInt)(self as *const _ as *mut _, text.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn parse_uint(&self, text: &HStringArg) -> Result<ComPtr<super::super::foundation::IReference<u64>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).ParseUInt)(self as *const _ as *mut _, text.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn parse_double(&self, text: &HStringArg) -> Result<ComPtr<super::super::foundation::IReference<f64>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).ParseDouble)(self as *const _ as *mut _, text.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
DEFINE_IID!(IID_INumberRounder, 1416872821, 14573, 17969, 184, 12, 239, 52, 252, 72, 183, 245);
RT_INTERFACE!{interface INumberRounder(INumberRounderVtbl): IInspectable(IInspectableVtbl) [IID_INumberRounder] {
    fn RoundInt32(&self, value: i32, out: *mut i32) -> HRESULT,
    fn RoundUInt32(&self, value: u32, out: *mut u32) -> HRESULT,
    fn RoundInt64(&self, value: i64, out: *mut i64) -> HRESULT,
    fn RoundUInt64(&self, value: u64, out: *mut u64) -> HRESULT,
    fn RoundSingle(&self, value: f32, out: *mut f32) -> HRESULT,
    fn RoundDouble(&self, value: f64, out: *mut f64) -> HRESULT
}}
impl INumberRounder {
    #[inline] pub unsafe fn round_int32(&self, value: i32) -> Result<i32> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).RoundInt32)(self as *const _ as *mut _, value, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn round_uint32(&self, value: u32) -> Result<u32> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).RoundUInt32)(self as *const _ as *mut _, value, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn round_int64(&self, value: i64) -> Result<i64> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).RoundInt64)(self as *const _ as *mut _, value, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn round_uint64(&self, value: u64) -> Result<u64> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).RoundUInt64)(self as *const _ as *mut _, value, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn round_single(&self, value: f32) -> Result<f32> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).RoundSingle)(self as *const _ as *mut _, value, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn round_double(&self, value: f64) -> Result<f64> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).RoundDouble)(self as *const _ as *mut _, value, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
}
DEFINE_IID!(IID_INumberRounderOption, 990413875, 25711, 20222, 141, 72, 102, 235, 46, 73, 231, 54);
RT_INTERFACE!{interface INumberRounderOption(INumberRounderOptionVtbl): IInspectable(IInspectableVtbl) [IID_INumberRounderOption] {
    fn get_NumberRounder(&self, out: *mut *mut INumberRounder) -> HRESULT,
    fn put_NumberRounder(&self, value: *mut INumberRounder) -> HRESULT
}}
impl INumberRounderOption {
    #[inline] pub unsafe fn get_number_rounder(&self) -> Result<ComPtr<INumberRounder>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_NumberRounder)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_number_rounder(&self, value: &INumberRounder) -> Result<()> {
        let hr = ((*self.lpVtbl).put_NumberRounder)(self as *const _ as *mut _, value as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
}
DEFINE_IID!(IID_INumeralSystemTranslator, 687193132, 35875, 16948, 173, 46, 250, 90, 58, 66, 110, 155);
RT_INTERFACE!{interface INumeralSystemTranslator(INumeralSystemTranslatorVtbl): IInspectable(IInspectableVtbl) [IID_INumeralSystemTranslator] {
    fn get_Languages(&self, out: *mut *mut super::super::foundation::collections::IVectorView<HString>) -> HRESULT,
    fn get_ResolvedLanguage(&self, out: *mut HSTRING) -> HRESULT,
    fn get_NumeralSystem(&self, out: *mut HSTRING) -> HRESULT,
    fn put_NumeralSystem(&self, value: HSTRING) -> HRESULT,
    fn TranslateNumerals(&self, value: HSTRING, out: *mut HSTRING) -> HRESULT
}}
impl INumeralSystemTranslator {
    #[inline] pub unsafe fn get_languages(&self) -> Result<ComPtr<super::super::foundation::collections::IVectorView<HString>>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Languages)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_resolved_language(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_ResolvedLanguage)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_numeral_system(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_NumeralSystem)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_numeral_system(&self, value: &HStringArg) -> Result<()> {
        let hr = ((*self.lpVtbl).put_NumeralSystem)(self as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn translate_numerals(&self, value: &HStringArg) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).TranslateNumerals)(self as *const _ as *mut _, value.get(), &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
}
RT_CLASS!{class NumeralSystemTranslator: INumeralSystemTranslator}
impl RtActivatable<INumeralSystemTranslatorFactory> for NumeralSystemTranslator {}
impl RtActivatable<IActivationFactory> for NumeralSystemTranslator {}
impl NumeralSystemTranslator {
    #[inline] pub fn create(languages: &super::super::foundation::collections::IIterable<HString>) -> Result<ComPtr<NumeralSystemTranslator>> { unsafe {
        <Self as RtActivatable<INumeralSystemTranslatorFactory>>::get_activation_factory().create(languages)
    }}
}
DEFINE_CLSID!(NumeralSystemTranslator(&[87,105,110,100,111,119,115,46,71,108,111,98,97,108,105,122,97,116,105,111,110,46,78,117,109,98,101,114,70,111,114,109,97,116,116,105,110,103,46,78,117,109,101,114,97,108,83,121,115,116,101,109,84,114,97,110,115,108,97,116,111,114,0]) [CLSID_NumeralSystemTranslator]);
DEFINE_IID!(IID_INumeralSystemTranslatorFactory, 2519779546, 14063, 19848, 168, 92, 111, 13, 152, 214, 32, 166);
RT_INTERFACE!{static interface INumeralSystemTranslatorFactory(INumeralSystemTranslatorFactoryVtbl): IInspectable(IInspectableVtbl) [IID_INumeralSystemTranslatorFactory] {
    fn Create(&self, languages: *mut super::super::foundation::collections::IIterable<HString>, out: *mut *mut NumeralSystemTranslator) -> HRESULT
}}
impl INumeralSystemTranslatorFactory {
    #[inline] pub unsafe fn create(&self, languages: &super::super::foundation::collections::IIterable<HString>) -> Result<ComPtr<NumeralSystemTranslator>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).Create)(self as *const _ as *mut _, languages as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
RT_CLASS!{class PercentFormatter: INumberFormatter}
impl RtActivatable<IPercentFormatterFactory> for PercentFormatter {}
impl RtActivatable<IActivationFactory> for PercentFormatter {}
impl PercentFormatter {
    #[inline] pub fn create_percent_formatter(languages: &super::super::foundation::collections::IIterable<HString>, geographicRegion: &HStringArg) -> Result<ComPtr<PercentFormatter>> { unsafe {
        <Self as RtActivatable<IPercentFormatterFactory>>::get_activation_factory().create_percent_formatter(languages, geographicRegion)
    }}
}
DEFINE_CLSID!(PercentFormatter(&[87,105,110,100,111,119,115,46,71,108,111,98,97,108,105,122,97,116,105,111,110,46,78,117,109,98,101,114,70,111,114,109,97,116,116,105,110,103,46,80,101,114,99,101,110,116,70,111,114,109,97,116,116,101,114,0]) [CLSID_PercentFormatter]);
DEFINE_IID!(IID_IPercentFormatterFactory, 3078785775, 65236, 16408, 166, 226, 224, 153, 97, 224, 55, 101);
RT_INTERFACE!{static interface IPercentFormatterFactory(IPercentFormatterFactoryVtbl): IInspectable(IInspectableVtbl) [IID_IPercentFormatterFactory] {
    fn CreatePercentFormatter(&self, languages: *mut super::super::foundation::collections::IIterable<HString>, geographicRegion: HSTRING, out: *mut *mut PercentFormatter) -> HRESULT
}}
impl IPercentFormatterFactory {
    #[inline] pub unsafe fn create_percent_formatter(&self, languages: &super::super::foundation::collections::IIterable<HString>, geographicRegion: &HStringArg) -> Result<ComPtr<PercentFormatter>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).CreatePercentFormatter)(self as *const _ as *mut _, languages as *const _ as *mut _, geographicRegion.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
RT_CLASS!{class PermilleFormatter: INumberFormatter}
impl RtActivatable<IPermilleFormatterFactory> for PermilleFormatter {}
impl RtActivatable<IActivationFactory> for PermilleFormatter {}
impl PermilleFormatter {
    #[inline] pub fn create_permille_formatter(languages: &super::super::foundation::collections::IIterable<HString>, geographicRegion: &HStringArg) -> Result<ComPtr<PermilleFormatter>> { unsafe {
        <Self as RtActivatable<IPermilleFormatterFactory>>::get_activation_factory().create_permille_formatter(languages, geographicRegion)
    }}
}
DEFINE_CLSID!(PermilleFormatter(&[87,105,110,100,111,119,115,46,71,108,111,98,97,108,105,122,97,116,105,111,110,46,78,117,109,98,101,114,70,111,114,109,97,116,116,105,110,103,46,80,101,114,109,105,108,108,101,70,111,114,109,97,116,116,101,114,0]) [CLSID_PermilleFormatter]);
DEFINE_IID!(IID_IPermilleFormatterFactory, 725071020, 58936, 20181, 169, 152, 98, 246, 176, 106, 73, 174);
RT_INTERFACE!{static interface IPermilleFormatterFactory(IPermilleFormatterFactoryVtbl): IInspectable(IInspectableVtbl) [IID_IPermilleFormatterFactory] {
    fn CreatePermilleFormatter(&self, languages: *mut super::super::foundation::collections::IIterable<HString>, geographicRegion: HSTRING, out: *mut *mut PermilleFormatter) -> HRESULT
}}
impl IPermilleFormatterFactory {
    #[inline] pub unsafe fn create_permille_formatter(&self, languages: &super::super::foundation::collections::IIterable<HString>, geographicRegion: &HStringArg) -> Result<ComPtr<PermilleFormatter>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).CreatePermilleFormatter)(self as *const _ as *mut _, languages as *const _ as *mut _, geographicRegion.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
RT_ENUM! { enum RoundingAlgorithm: i32 {
    None (RoundingAlgorithm_None) = 0, RoundDown (RoundingAlgorithm_RoundDown) = 1, RoundUp (RoundingAlgorithm_RoundUp) = 2, RoundTowardsZero (RoundingAlgorithm_RoundTowardsZero) = 3, RoundAwayFromZero (RoundingAlgorithm_RoundAwayFromZero) = 4, RoundHalfDown (RoundingAlgorithm_RoundHalfDown) = 5, RoundHalfUp (RoundingAlgorithm_RoundHalfUp) = 6, RoundHalfTowardsZero (RoundingAlgorithm_RoundHalfTowardsZero) = 7, RoundHalfAwayFromZero (RoundingAlgorithm_RoundHalfAwayFromZero) = 8, RoundHalfToEven (RoundingAlgorithm_RoundHalfToEven) = 9, RoundHalfToOdd (RoundingAlgorithm_RoundHalfToOdd) = 10,
}}
DEFINE_IID!(IID_ISignedZeroOption, 4246527281, 2620, 18884, 166, 66, 150, 161, 86, 79, 79, 48);
RT_INTERFACE!{interface ISignedZeroOption(ISignedZeroOptionVtbl): IInspectable(IInspectableVtbl) [IID_ISignedZeroOption] {
    fn get_IsZeroSigned(&self, out: *mut bool) -> HRESULT,
    fn put_IsZeroSigned(&self, value: bool) -> HRESULT
}}
impl ISignedZeroOption {
    #[inline] pub unsafe fn get_is_zero_signed(&self) -> Result<bool> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_IsZeroSigned)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_is_zero_signed(&self, value: bool) -> Result<()> {
        let hr = ((*self.lpVtbl).put_IsZeroSigned)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
}
DEFINE_IID!(IID_ISignificantDigitsNumberRounder, 4120124362, 26182, 18707, 140, 118, 27, 25, 31, 249, 77, 253);
RT_INTERFACE!{interface ISignificantDigitsNumberRounder(ISignificantDigitsNumberRounderVtbl): IInspectable(IInspectableVtbl) [IID_ISignificantDigitsNumberRounder] {
    fn get_RoundingAlgorithm(&self, out: *mut RoundingAlgorithm) -> HRESULT,
    fn put_RoundingAlgorithm(&self, value: RoundingAlgorithm) -> HRESULT,
    fn get_SignificantDigits(&self, out: *mut u32) -> HRESULT,
    fn put_SignificantDigits(&self, value: u32) -> HRESULT
}}
impl ISignificantDigitsNumberRounder {
    #[inline] pub unsafe fn get_rounding_algorithm(&self) -> Result<RoundingAlgorithm> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_RoundingAlgorithm)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_rounding_algorithm(&self, value: RoundingAlgorithm) -> Result<()> {
        let hr = ((*self.lpVtbl).put_RoundingAlgorithm)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_significant_digits(&self) -> Result<u32> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_SignificantDigits)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_significant_digits(&self, value: u32) -> Result<()> {
        let hr = ((*self.lpVtbl).put_SignificantDigits)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
}
RT_CLASS!{class SignificantDigitsNumberRounder: INumberRounder}
impl RtActivatable<IActivationFactory> for SignificantDigitsNumberRounder {}
DEFINE_CLSID!(SignificantDigitsNumberRounder(&[87,105,110,100,111,119,115,46,71,108,111,98,97,108,105,122,97,116,105,111,110,46,78,117,109,98,101,114,70,111,114,109,97,116,116,105,110,103,46,83,105,103,110,105,102,105,99,97,110,116,68,105,103,105,116,115,78,117,109,98,101,114,82,111,117,110,100,101,114,0]) [CLSID_SignificantDigitsNumberRounder]);
DEFINE_IID!(IID_ISignificantDigitsOption, 491650269, 11587, 20200, 187, 241, 193, 178, 106, 113, 26, 88);
RT_INTERFACE!{interface ISignificantDigitsOption(ISignificantDigitsOptionVtbl): IInspectable(IInspectableVtbl) [IID_ISignificantDigitsOption] {
    fn get_SignificantDigits(&self, out: *mut i32) -> HRESULT,
    fn put_SignificantDigits(&self, value: i32) -> HRESULT
}}
impl ISignificantDigitsOption {
    #[inline] pub unsafe fn get_significant_digits(&self) -> Result<i32> {
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_SignificantDigits)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }
    #[inline] pub unsafe fn set_significant_digits(&self, value: i32) -> Result<()> {
        let hr = ((*self.lpVtbl).put_SignificantDigits)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }
}
} // Windows.Globalization.NumberFormatting
pub mod collation { // Windows.Globalization.Collation
use ::prelude::*;
DEFINE_IID!(IID_ICharacterGrouping, 4209467835, 32861, 19376, 149, 187, 193, 247, 195, 232, 235, 142);
RT_INTERFACE!{interface ICharacterGrouping(ICharacterGroupingVtbl): IInspectable(IInspectableVtbl) [IID_ICharacterGrouping] {
    fn get_First(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Label(&self, out: *mut HSTRING) -> HRESULT
}}
impl ICharacterGrouping {
    #[inline] pub unsafe fn get_first(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_First)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
    #[inline] pub unsafe fn get_label(&self) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Label)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
}
RT_CLASS!{class CharacterGrouping: ICharacterGrouping}
DEFINE_IID!(IID_ICharacterGroupings, 3100772981, 54479, 16469, 128, 229, 206, 22, 156, 34, 100, 150);
RT_INTERFACE!{interface ICharacterGroupings(ICharacterGroupingsVtbl): IInspectable(IInspectableVtbl) [IID_ICharacterGroupings] {
    fn Lookup(&self, text: HSTRING, out: *mut HSTRING) -> HRESULT
}}
impl ICharacterGroupings {
    #[inline] pub unsafe fn lookup(&self, text: &HStringArg) -> Result<HString> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).Lookup)(self as *const _ as *mut _, text.get(), &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }
}
RT_CLASS!{class CharacterGroupings: ICharacterGroupings}
impl RtActivatable<ICharacterGroupingsFactory> for CharacterGroupings {}
impl RtActivatable<IActivationFactory> for CharacterGroupings {}
impl CharacterGroupings {
    #[inline] pub fn create(language: &HStringArg) -> Result<ComPtr<CharacterGroupings>> { unsafe {
        <Self as RtActivatable<ICharacterGroupingsFactory>>::get_activation_factory().create(language)
    }}
}
DEFINE_CLSID!(CharacterGroupings(&[87,105,110,100,111,119,115,46,71,108,111,98,97,108,105,122,97,116,105,111,110,46,67,111,108,108,97,116,105,111,110,46,67,104,97,114,97,99,116,101,114,71,114,111,117,112,105,110,103,115,0]) [CLSID_CharacterGroupings]);
DEFINE_IID!(IID_ICharacterGroupingsFactory, 2582290393, 34925, 17409, 159, 152, 105, 200, 45, 76, 47, 120);
RT_INTERFACE!{static interface ICharacterGroupingsFactory(ICharacterGroupingsFactoryVtbl): IInspectable(IInspectableVtbl) [IID_ICharacterGroupingsFactory] {
    fn Create(&self, language: HSTRING, out: *mut *mut CharacterGroupings) -> HRESULT
}}
impl ICharacterGroupingsFactory {
    #[inline] pub unsafe fn create(&self, language: &HStringArg) -> Result<ComPtr<CharacterGroupings>> {
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).Create)(self as *const _ as *mut _, language.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }
}
} // Windows.Globalization.Collation
