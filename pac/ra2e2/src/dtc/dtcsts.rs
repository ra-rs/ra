#[doc = "Register `DTCSTS` reader"]
pub struct R(crate::R<DTCSTS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DTCSTS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DTCSTS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DTCSTS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `VECN` reader - DTC-Activating Vector Number Monitoring"]
pub type VECN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ACT` reader - DTC Active Flag"]
pub type ACT_R = crate::BitReader<ACT_A>;
#[doc = "DTC Active Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ACT_A {
    #[doc = "0: DTC transfer operation is not in progress."]
    _0 = 0,
    #[doc = "1: DTC transfer operation is in progress."]
    _1 = 1,
}
impl From<ACT_A> for bool {
    #[inline(always)]
    fn from(variant: ACT_A) -> Self {
        variant as u8 != 0
    }
}
impl ACT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ACT_A {
        match self.bits {
            false => ACT_A::_0,
            true => ACT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ACT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ACT_A::_1
    }
}
impl R {
    #[doc = "Bits 0:7 - DTC-Activating Vector Number Monitoring"]
    #[inline(always)]
    pub fn vecn(&self) -> VECN_R {
        VECN_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 15 - DTC Active Flag"]
    #[inline(always)]
    pub fn act(&self) -> ACT_R {
        ACT_R::new(((self.bits >> 15) & 1) != 0)
    }
}
#[doc = "DTC Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dtcsts](index.html) module"]
pub struct DTCSTS_SPEC;
impl crate::RegisterSpec for DTCSTS_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [dtcsts::R](R) reader structure"]
impl crate::Readable for DTCSTS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DTCSTS to value 0"]
impl crate::Resettable for DTCSTS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
