#[doc = "Register `GTSOS` reader"]
pub struct R(crate::R<GTSOS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GTSOS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GTSOS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GTSOS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SOS` reader - Output Protection Function Status"]
pub type SOS_R = crate::FieldReader<u8, SOS_A>;
#[doc = "Output Protection Function Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SOS_A {
    #[doc = "0: Normal operation"]
    _00 = 0,
    #[doc = "1: Protected state (GTCCRA = 0 is set during transfer at trough or crest)"]
    _01 = 1,
    #[doc = "2: Protected state (GTCCRA â\u{89}¥ GTPR is set during transfer at trough)"]
    _10 = 2,
    #[doc = "3: Protected state (GTCCRA â\u{89}¥ GTPR is set during transfer at crest)"]
    _11 = 3,
}
impl From<SOS_A> for u8 {
    #[inline(always)]
    fn from(variant: SOS_A) -> Self {
        variant as _
    }
}
impl SOS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SOS_A {
        match self.bits {
            0 => SOS_A::_00,
            1 => SOS_A::_01,
            2 => SOS_A::_10,
            3 => SOS_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == SOS_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == SOS_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == SOS_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == SOS_A::_11
    }
}
impl R {
    #[doc = "Bits 0:1 - Output Protection Function Status"]
    #[inline(always)]
    pub fn sos(&self) -> SOS_R {
        SOS_R::new((self.bits & 3) as u8)
    }
}
#[doc = "General PWM Timer Output Protection Function Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gtsos](index.html) module"]
pub struct GTSOS_SPEC;
impl crate::RegisterSpec for GTSOS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gtsos::R](R) reader structure"]
impl crate::Readable for GTSOS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets GTSOS to value 0"]
impl crate::Resettable for GTSOS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
