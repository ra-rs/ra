#[doc = "Register `GTITC` reader"]
pub struct R(crate::R<GTITC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GTITC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GTITC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GTITC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GTITC` writer"]
pub struct W(crate::W<GTITC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GTITC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<GTITC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GTITC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ITLA` reader - GTCCRA Compare Match/Input Capture Interrupt Link"]
pub type ITLA_R = crate::BitReader<ITLA_A>;
#[doc = "GTCCRA Compare Match/Input Capture Interrupt Link\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ITLA_A {
    #[doc = "0: Do not link with GPTn_OVF/GPTn_UDF interrupt skipping function"]
    _0 = 0,
    #[doc = "1: Link with GPTn_OVF/GPTn_UDF interrupt skipping function."]
    _1 = 1,
}
impl From<ITLA_A> for bool {
    #[inline(always)]
    fn from(variant: ITLA_A) -> Self {
        variant as u8 != 0
    }
}
impl ITLA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ITLA_A {
        match self.bits {
            false => ITLA_A::_0,
            true => ITLA_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ITLA_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ITLA_A::_1
    }
}
#[doc = "Field `ITLA` writer - GTCCRA Compare Match/Input Capture Interrupt Link"]
pub type ITLA_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTITC_SPEC, ITLA_A, O>;
impl<'a, const O: u8> ITLA_W<'a, O> {
    #[doc = "Do not link with GPTn_OVF/GPTn_UDF interrupt skipping function"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ITLA_A::_0)
    }
    #[doc = "Link with GPTn_OVF/GPTn_UDF interrupt skipping function."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ITLA_A::_1)
    }
}
#[doc = "Field `ITLB` reader - GTCCRB Compare Match/Input Capture Interrupt Link"]
pub type ITLB_R = crate::BitReader<ITLB_A>;
#[doc = "GTCCRB Compare Match/Input Capture Interrupt Link\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ITLB_A {
    #[doc = "0: Do not link with GPTn_OVF/GPTn_UDF interrupt skipping function"]
    _0 = 0,
    #[doc = "1: Link with GPTn_OVF/GPTn_UDF interrupt skipping function."]
    _1 = 1,
}
impl From<ITLB_A> for bool {
    #[inline(always)]
    fn from(variant: ITLB_A) -> Self {
        variant as u8 != 0
    }
}
impl ITLB_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ITLB_A {
        match self.bits {
            false => ITLB_A::_0,
            true => ITLB_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ITLB_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ITLB_A::_1
    }
}
#[doc = "Field `ITLB` writer - GTCCRB Compare Match/Input Capture Interrupt Link"]
pub type ITLB_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTITC_SPEC, ITLB_A, O>;
impl<'a, const O: u8> ITLB_W<'a, O> {
    #[doc = "Do not link with GPTn_OVF/GPTn_UDF interrupt skipping function"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ITLB_A::_0)
    }
    #[doc = "Link with GPTn_OVF/GPTn_UDF interrupt skipping function."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ITLB_A::_1)
    }
}
#[doc = "Field `ITLC` reader - GTCCRC Compare Match Interrupt Link"]
pub type ITLC_R = crate::BitReader<ITLC_A>;
#[doc = "GTCCRC Compare Match Interrupt Link\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ITLC_A {
    #[doc = "0: Do not link with GPTn_OVF/GPTn_UDF interrupt skipping function"]
    _0 = 0,
    #[doc = "1: Link with GPTn_OVF/GPTn_UDF interrupt skipping function."]
    _1 = 1,
}
impl From<ITLC_A> for bool {
    #[inline(always)]
    fn from(variant: ITLC_A) -> Self {
        variant as u8 != 0
    }
}
impl ITLC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ITLC_A {
        match self.bits {
            false => ITLC_A::_0,
            true => ITLC_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ITLC_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ITLC_A::_1
    }
}
#[doc = "Field `ITLC` writer - GTCCRC Compare Match Interrupt Link"]
pub type ITLC_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTITC_SPEC, ITLC_A, O>;
impl<'a, const O: u8> ITLC_W<'a, O> {
    #[doc = "Do not link with GPTn_OVF/GPTn_UDF interrupt skipping function"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ITLC_A::_0)
    }
    #[doc = "Link with GPTn_OVF/GPTn_UDF interrupt skipping function."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ITLC_A::_1)
    }
}
#[doc = "Field `ITLD` reader - GTCCRD Compare Match Interrupt Link"]
pub type ITLD_R = crate::BitReader<ITLD_A>;
#[doc = "GTCCRD Compare Match Interrupt Link\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ITLD_A {
    #[doc = "0: Do not link with GPTn_OVF/GPTn_UDF interrupt skipping function"]
    _0 = 0,
    #[doc = "1: Link with GPTn_OVF/GPTn_UDF interrupt skipping function."]
    _1 = 1,
}
impl From<ITLD_A> for bool {
    #[inline(always)]
    fn from(variant: ITLD_A) -> Self {
        variant as u8 != 0
    }
}
impl ITLD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ITLD_A {
        match self.bits {
            false => ITLD_A::_0,
            true => ITLD_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ITLD_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ITLD_A::_1
    }
}
#[doc = "Field `ITLD` writer - GTCCRD Compare Match Interrupt Link"]
pub type ITLD_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTITC_SPEC, ITLD_A, O>;
impl<'a, const O: u8> ITLD_W<'a, O> {
    #[doc = "Do not link with GPTn_OVF/GPTn_UDF interrupt skipping function"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ITLD_A::_0)
    }
    #[doc = "Link with GPTn_OVF/GPTn_UDF interrupt skipping function."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ITLD_A::_1)
    }
}
#[doc = "Field `ITLE` reader - GTCCRE Compare Match Interrupt Link"]
pub type ITLE_R = crate::BitReader<ITLE_A>;
#[doc = "GTCCRE Compare Match Interrupt Link\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ITLE_A {
    #[doc = "0: Do not link with GPTn_OVF/GPTn_UDF interrupt skipping function"]
    _0 = 0,
    #[doc = "1: Link with GPTn_OVF/GPTn_UDF interrupt skipping function."]
    _1 = 1,
}
impl From<ITLE_A> for bool {
    #[inline(always)]
    fn from(variant: ITLE_A) -> Self {
        variant as u8 != 0
    }
}
impl ITLE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ITLE_A {
        match self.bits {
            false => ITLE_A::_0,
            true => ITLE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ITLE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ITLE_A::_1
    }
}
#[doc = "Field `ITLE` writer - GTCCRE Compare Match Interrupt Link"]
pub type ITLE_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTITC_SPEC, ITLE_A, O>;
impl<'a, const O: u8> ITLE_W<'a, O> {
    #[doc = "Do not link with GPTn_OVF/GPTn_UDF interrupt skipping function"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ITLE_A::_0)
    }
    #[doc = "Link with GPTn_OVF/GPTn_UDF interrupt skipping function."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ITLE_A::_1)
    }
}
#[doc = "Field `ITLF` reader - GTCCRF Compare Match Interrupt Link"]
pub type ITLF_R = crate::BitReader<ITLF_A>;
#[doc = "GTCCRF Compare Match Interrupt Link\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ITLF_A {
    #[doc = "0: Do not link with GPTn_OVF/GPTn_UDF interrupt skipping function"]
    _0 = 0,
    #[doc = "1: Link with GPTn_OVF/GPTn_UDF interrupt skipping function."]
    _1 = 1,
}
impl From<ITLF_A> for bool {
    #[inline(always)]
    fn from(variant: ITLF_A) -> Self {
        variant as u8 != 0
    }
}
impl ITLF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ITLF_A {
        match self.bits {
            false => ITLF_A::_0,
            true => ITLF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ITLF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ITLF_A::_1
    }
}
#[doc = "Field `ITLF` writer - GTCCRF Compare Match Interrupt Link"]
pub type ITLF_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTITC_SPEC, ITLF_A, O>;
impl<'a, const O: u8> ITLF_W<'a, O> {
    #[doc = "Do not link with GPTn_OVF/GPTn_UDF interrupt skipping function"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ITLF_A::_0)
    }
    #[doc = "Link with GPTn_OVF/GPTn_UDF interrupt skipping function."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ITLF_A::_1)
    }
}
#[doc = "Field `IVTC` reader - GPT_OVF/GPT_UDF Interrupt Skipping Function Select"]
pub type IVTC_R = crate::FieldReader<u8, IVTC_A>;
#[doc = "GPT_OVF/GPT_UDF Interrupt Skipping Function Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum IVTC_A {
    #[doc = "0: Do not perform skipping"]
    _00 = 0,
    #[doc = "1: Count and skip both overflow and underflow for saw waves and crest for triangle waves"]
    _01 = 1,
    #[doc = "2: Count and skip both overflow and underflow for saw waves and trough for triangle waves"]
    _10 = 2,
    #[doc = "3: Count and skip both overflow and underflow for saw waves and both crest and trough for triangle waves."]
    _11 = 3,
}
impl From<IVTC_A> for u8 {
    #[inline(always)]
    fn from(variant: IVTC_A) -> Self {
        variant as _
    }
}
impl IVTC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IVTC_A {
        match self.bits {
            0 => IVTC_A::_00,
            1 => IVTC_A::_01,
            2 => IVTC_A::_10,
            3 => IVTC_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == IVTC_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == IVTC_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == IVTC_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == IVTC_A::_11
    }
}
#[doc = "Field `IVTC` writer - GPT_OVF/GPT_UDF Interrupt Skipping Function Select"]
pub type IVTC_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, GTITC_SPEC, u8, IVTC_A, 2, O>;
impl<'a, const O: u8> IVTC_W<'a, O> {
    #[doc = "Do not perform skipping"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(IVTC_A::_00)
    }
    #[doc = "Count and skip both overflow and underflow for saw waves and crest for triangle waves"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(IVTC_A::_01)
    }
    #[doc = "Count and skip both overflow and underflow for saw waves and trough for triangle waves"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(IVTC_A::_10)
    }
    #[doc = "Count and skip both overflow and underflow for saw waves and both crest and trough for triangle waves."]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(IVTC_A::_11)
    }
}
#[doc = "Field `IVTT` reader - GPT_OVF/GPT_UDF Interrupt Skipping Count Select"]
pub type IVTT_R = crate::FieldReader<u8, IVTT_A>;
#[doc = "GPT_OVF/GPT_UDF Interrupt Skipping Count Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum IVTT_A {
    #[doc = "0: No skipping"]
    _000 = 0,
    #[doc = "1: Skipping count of 1"]
    _001 = 1,
    #[doc = "2: Skipping count of 2"]
    _010 = 2,
    #[doc = "3: Skipping count of 3"]
    _011 = 3,
    #[doc = "4: Skipping count of 4"]
    _100 = 4,
    #[doc = "5: Skipping count of 5"]
    _101 = 5,
    #[doc = "6: Skipping count of 6"]
    _110 = 6,
    #[doc = "7: Skipping count of 7."]
    _111 = 7,
}
impl From<IVTT_A> for u8 {
    #[inline(always)]
    fn from(variant: IVTT_A) -> Self {
        variant as _
    }
}
impl IVTT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IVTT_A {
        match self.bits {
            0 => IVTT_A::_000,
            1 => IVTT_A::_001,
            2 => IVTT_A::_010,
            3 => IVTT_A::_011,
            4 => IVTT_A::_100,
            5 => IVTT_A::_101,
            6 => IVTT_A::_110,
            7 => IVTT_A::_111,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == IVTT_A::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == IVTT_A::_001
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == IVTT_A::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == IVTT_A::_011
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == IVTT_A::_100
    }
    #[doc = "Checks if the value of the field is `_101`"]
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == IVTT_A::_101
    }
    #[doc = "Checks if the value of the field is `_110`"]
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        *self == IVTT_A::_110
    }
    #[doc = "Checks if the value of the field is `_111`"]
    #[inline(always)]
    pub fn is_111(&self) -> bool {
        *self == IVTT_A::_111
    }
}
#[doc = "Field `IVTT` writer - GPT_OVF/GPT_UDF Interrupt Skipping Count Select"]
pub type IVTT_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, GTITC_SPEC, u8, IVTT_A, 3, O>;
impl<'a, const O: u8> IVTT_W<'a, O> {
    #[doc = "No skipping"]
    #[inline(always)]
    pub fn _000(self) -> &'a mut W {
        self.variant(IVTT_A::_000)
    }
    #[doc = "Skipping count of 1"]
    #[inline(always)]
    pub fn _001(self) -> &'a mut W {
        self.variant(IVTT_A::_001)
    }
    #[doc = "Skipping count of 2"]
    #[inline(always)]
    pub fn _010(self) -> &'a mut W {
        self.variant(IVTT_A::_010)
    }
    #[doc = "Skipping count of 3"]
    #[inline(always)]
    pub fn _011(self) -> &'a mut W {
        self.variant(IVTT_A::_011)
    }
    #[doc = "Skipping count of 4"]
    #[inline(always)]
    pub fn _100(self) -> &'a mut W {
        self.variant(IVTT_A::_100)
    }
    #[doc = "Skipping count of 5"]
    #[inline(always)]
    pub fn _101(self) -> &'a mut W {
        self.variant(IVTT_A::_101)
    }
    #[doc = "Skipping count of 6"]
    #[inline(always)]
    pub fn _110(self) -> &'a mut W {
        self.variant(IVTT_A::_110)
    }
    #[doc = "Skipping count of 7."]
    #[inline(always)]
    pub fn _111(self) -> &'a mut W {
        self.variant(IVTT_A::_111)
    }
}
#[doc = "Field `ADTAL` reader - GTADTRA A/D Converter Start Request Link"]
pub type ADTAL_R = crate::BitReader<ADTAL_A>;
#[doc = "GTADTRA A/D Converter Start Request Link\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADTAL_A {
    #[doc = "0: Do not link with GPTn_OVF/GPTn_UDF interrupt skipping function"]
    _0 = 0,
    #[doc = "1: Link with GPTn_OVF/GPTn_UDF interrupt skipping function"]
    _1 = 1,
}
impl From<ADTAL_A> for bool {
    #[inline(always)]
    fn from(variant: ADTAL_A) -> Self {
        variant as u8 != 0
    }
}
impl ADTAL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADTAL_A {
        match self.bits {
            false => ADTAL_A::_0,
            true => ADTAL_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ADTAL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ADTAL_A::_1
    }
}
#[doc = "Field `ADTAL` writer - GTADTRA A/D Converter Start Request Link"]
pub type ADTAL_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTITC_SPEC, ADTAL_A, O>;
impl<'a, const O: u8> ADTAL_W<'a, O> {
    #[doc = "Do not link with GPTn_OVF/GPTn_UDF interrupt skipping function"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ADTAL_A::_0)
    }
    #[doc = "Link with GPTn_OVF/GPTn_UDF interrupt skipping function"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ADTAL_A::_1)
    }
}
#[doc = "Field `ADTBL` reader - GTADTRB A/D Converter Start Request Link"]
pub type ADTBL_R = crate::BitReader<ADTBL_A>;
#[doc = "GTADTRB A/D Converter Start Request Link\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADTBL_A {
    #[doc = "0: Do not link with GPTn_OVF/GPTn_UDF interrupt skipping function"]
    _0 = 0,
    #[doc = "1: Link with GPTn_OVF/GPTn_UDF interrupt skipping function."]
    _1 = 1,
}
impl From<ADTBL_A> for bool {
    #[inline(always)]
    fn from(variant: ADTBL_A) -> Self {
        variant as u8 != 0
    }
}
impl ADTBL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADTBL_A {
        match self.bits {
            false => ADTBL_A::_0,
            true => ADTBL_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ADTBL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ADTBL_A::_1
    }
}
#[doc = "Field `ADTBL` writer - GTADTRB A/D Converter Start Request Link"]
pub type ADTBL_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTITC_SPEC, ADTBL_A, O>;
impl<'a, const O: u8> ADTBL_W<'a, O> {
    #[doc = "Do not link with GPTn_OVF/GPTn_UDF interrupt skipping function"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ADTBL_A::_0)
    }
    #[doc = "Link with GPTn_OVF/GPTn_UDF interrupt skipping function."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ADTBL_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - GTCCRA Compare Match/Input Capture Interrupt Link"]
    #[inline(always)]
    pub fn itla(&self) -> ITLA_R {
        ITLA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - GTCCRB Compare Match/Input Capture Interrupt Link"]
    #[inline(always)]
    pub fn itlb(&self) -> ITLB_R {
        ITLB_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - GTCCRC Compare Match Interrupt Link"]
    #[inline(always)]
    pub fn itlc(&self) -> ITLC_R {
        ITLC_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - GTCCRD Compare Match Interrupt Link"]
    #[inline(always)]
    pub fn itld(&self) -> ITLD_R {
        ITLD_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - GTCCRE Compare Match Interrupt Link"]
    #[inline(always)]
    pub fn itle(&self) -> ITLE_R {
        ITLE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - GTCCRF Compare Match Interrupt Link"]
    #[inline(always)]
    pub fn itlf(&self) -> ITLF_R {
        ITLF_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - GPT_OVF/GPT_UDF Interrupt Skipping Function Select"]
    #[inline(always)]
    pub fn ivtc(&self) -> IVTC_R {
        IVTC_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:10 - GPT_OVF/GPT_UDF Interrupt Skipping Count Select"]
    #[inline(always)]
    pub fn ivtt(&self) -> IVTT_R {
        IVTT_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 12 - GTADTRA A/D Converter Start Request Link"]
    #[inline(always)]
    pub fn adtal(&self) -> ADTAL_R {
        ADTAL_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 14 - GTADTRB A/D Converter Start Request Link"]
    #[inline(always)]
    pub fn adtbl(&self) -> ADTBL_R {
        ADTBL_R::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - GTCCRA Compare Match/Input Capture Interrupt Link"]
    #[inline(always)]
    #[must_use]
    pub fn itla(&mut self) -> ITLA_W<0> {
        ITLA_W::new(self)
    }
    #[doc = "Bit 1 - GTCCRB Compare Match/Input Capture Interrupt Link"]
    #[inline(always)]
    #[must_use]
    pub fn itlb(&mut self) -> ITLB_W<1> {
        ITLB_W::new(self)
    }
    #[doc = "Bit 2 - GTCCRC Compare Match Interrupt Link"]
    #[inline(always)]
    #[must_use]
    pub fn itlc(&mut self) -> ITLC_W<2> {
        ITLC_W::new(self)
    }
    #[doc = "Bit 3 - GTCCRD Compare Match Interrupt Link"]
    #[inline(always)]
    #[must_use]
    pub fn itld(&mut self) -> ITLD_W<3> {
        ITLD_W::new(self)
    }
    #[doc = "Bit 4 - GTCCRE Compare Match Interrupt Link"]
    #[inline(always)]
    #[must_use]
    pub fn itle(&mut self) -> ITLE_W<4> {
        ITLE_W::new(self)
    }
    #[doc = "Bit 5 - GTCCRF Compare Match Interrupt Link"]
    #[inline(always)]
    #[must_use]
    pub fn itlf(&mut self) -> ITLF_W<5> {
        ITLF_W::new(self)
    }
    #[doc = "Bits 6:7 - GPT_OVF/GPT_UDF Interrupt Skipping Function Select"]
    #[inline(always)]
    #[must_use]
    pub fn ivtc(&mut self) -> IVTC_W<6> {
        IVTC_W::new(self)
    }
    #[doc = "Bits 8:10 - GPT_OVF/GPT_UDF Interrupt Skipping Count Select"]
    #[inline(always)]
    #[must_use]
    pub fn ivtt(&mut self) -> IVTT_W<8> {
        IVTT_W::new(self)
    }
    #[doc = "Bit 12 - GTADTRA A/D Converter Start Request Link"]
    #[inline(always)]
    #[must_use]
    pub fn adtal(&mut self) -> ADTAL_W<12> {
        ADTAL_W::new(self)
    }
    #[doc = "Bit 14 - GTADTRB A/D Converter Start Request Link"]
    #[inline(always)]
    #[must_use]
    pub fn adtbl(&mut self) -> ADTBL_W<14> {
        ADTBL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "General PWM Timer Interrupt and A/D Converter Start Request Skipping Setting Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gtitc](index.html) module"]
pub struct GTITC_SPEC;
impl crate::RegisterSpec for GTITC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gtitc::R](R) reader structure"]
impl crate::Readable for GTITC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gtitc::W](W) writer structure"]
impl crate::Writable for GTITC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GTITC to value 0"]
impl crate::Resettable for GTITC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
