#[doc = "Register `IIRCPRCFF` reader"]
pub struct R(crate::R<IIRCPRCFF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IIRCPRCFF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IIRCPRCFF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IIRCPRCFF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CPRCFF` reader - Channel processing completion flag"]
pub type CPRCFF_R = crate::FieldReader<u16, CPRCFF_A>;
#[doc = "Channel processing completion flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum CPRCFF_A {
    #[doc = "0: The channel processing of the corresponding channel is not completed."]
    _0 = 0,
    #[doc = "1: The channel processing of the corresponding channel is completed."]
    _1 = 1,
}
impl From<CPRCFF_A> for u16 {
    #[inline(always)]
    fn from(variant: CPRCFF_A) -> Self {
        variant as _
    }
}
impl CPRCFF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CPRCFF_A> {
        match self.bits {
            0 => Some(CPRCFF_A::_0),
            1 => Some(CPRCFF_A::_1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CPRCFF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CPRCFF_A::_1
    }
}
impl R {
    #[doc = "Bits 0:15 - Channel processing completion flag"]
    #[inline(always)]
    pub fn cprcff(&self) -> CPRCFF_R {
        CPRCFF_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Channel Processing Completion Flag Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iircprcff](index.html) module"]
pub struct IIRCPRCFF_SPEC;
impl crate::RegisterSpec for IIRCPRCFF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [iircprcff::R](R) reader structure"]
impl crate::Readable for IIRCPRCFF_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets IIRCPRCFF to value 0"]
impl crate::Resettable for IIRCPRCFF_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
