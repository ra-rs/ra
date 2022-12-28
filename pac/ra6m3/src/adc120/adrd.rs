#[doc = "Register `ADRD` reader"]
pub struct R(crate::R<ADRD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADRD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADRD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADRD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `AD` reader - A/D-converted value (right-justified)NOTE: Unused bits in the AD bit field are fixed \"0\""]
pub type AD_R = crate::FieldReader<u16, u16>;
#[doc = "Field `DIAGST` reader - Self-Diagnosis Status"]
pub type DIAGST_R = crate::FieldReader<u8, DIAGST_A>;
#[doc = "Self-Diagnosis Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DIAGST_A {
    #[doc = "0: Self-diagnosis has never been executed since power-on."]
    _00 = 0,
    #[doc = "1: Self-diagnosis using the voltage of 0 V has been executed."]
    _01 = 1,
    #[doc = "2: Self-diagnosis using the voltage of reference power supply(VREFH) x 1/2 has been executed."]
    _10 = 2,
    #[doc = "3: Self-diagnosis using the voltage of reference power supply(VREFH) has been executed."]
    _11 = 3,
}
impl From<DIAGST_A> for u8 {
    #[inline(always)]
    fn from(variant: DIAGST_A) -> Self {
        variant as _
    }
}
impl DIAGST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIAGST_A {
        match self.bits {
            0 => DIAGST_A::_00,
            1 => DIAGST_A::_01,
            2 => DIAGST_A::_10,
            3 => DIAGST_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == DIAGST_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == DIAGST_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == DIAGST_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == DIAGST_A::_11
    }
}
impl R {
    #[doc = "Bits 0:11 - A/D-converted value (right-justified)NOTE: Unused bits in the AD bit field are fixed \"0\""]
    #[inline(always)]
    pub fn ad(&self) -> AD_R {
        AD_R::new(self.bits & 0x0fff)
    }
    #[doc = "Bits 14:15 - Self-Diagnosis Status"]
    #[inline(always)]
    pub fn diagst(&self) -> DIAGST_R {
        DIAGST_R::new(((self.bits >> 14) & 3) as u8)
    }
}
#[doc = "A/D Self-Diagnosis Data Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adrd](index.html) module"]
pub struct ADRD_SPEC;
impl crate::RegisterSpec for ADRD_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [adrd::R](R) reader structure"]
impl crate::Readable for ADRD_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ADRD to value 0"]
impl crate::Resettable for ADRD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
