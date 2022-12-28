#[doc = "Register `IIRECCCNT` reader"]
pub struct R(crate::R<IIRECCCNT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IIRECCCNT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IIRECCCNT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IIRECCCNT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IIRECCCNT` writer"]
pub struct W(crate::W<IIRECCCNT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IIRECCCNT_SPEC>;
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
impl From<crate::W<IIRECCCNT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IIRECCCNT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ECCMD` reader - ECC setting bit"]
pub type ECCMD_R = crate::BitReader<ECCMD_A>;
#[doc = "ECC setting bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ECCMD_A {
    #[doc = "0: The ECC error detection/correction function is disabled."]
    _0 = 0,
    #[doc = "1: The ECC error detection/correction function is enabled."]
    _1 = 1,
}
impl From<ECCMD_A> for bool {
    #[inline(always)]
    fn from(variant: ECCMD_A) -> Self {
        variant as u8 != 0
    }
}
impl ECCMD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ECCMD_A {
        match self.bits {
            false => ECCMD_A::_0,
            true => ECCMD_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ECCMD_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ECCMD_A::_1
    }
}
#[doc = "Field `ECCMD` writer - ECC setting bit"]
pub type ECCMD_W<'a, const O: u8> = crate::BitWriter<'a, u32, IIRECCCNT_SPEC, ECCMD_A, O>;
impl<'a, const O: u8> ECCMD_W<'a, O> {
    #[doc = "The ECC error detection/correction function is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ECCMD_A::_0)
    }
    #[doc = "The ECC error detection/correction function is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ECCMD_A::_1)
    }
}
#[doc = "Field `ECCWBDIS` reader - ECC-corrected data write-back disable bit"]
pub type ECCWBDIS_R = crate::BitReader<ECCWBDIS_A>;
#[doc = "ECC-corrected data write-back disable bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ECCWBDIS_A {
    #[doc = "0: The error-corrected data write-back is enabled."]
    _0 = 0,
    #[doc = "1: The error-corrected data write-back is disabled."]
    _1 = 1,
}
impl From<ECCWBDIS_A> for bool {
    #[inline(always)]
    fn from(variant: ECCWBDIS_A) -> Self {
        variant as u8 != 0
    }
}
impl ECCWBDIS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ECCWBDIS_A {
        match self.bits {
            false => ECCWBDIS_A::_0,
            true => ECCWBDIS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ECCWBDIS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ECCWBDIS_A::_1
    }
}
#[doc = "Field `ECCWBDIS` writer - ECC-corrected data write-back disable bit"]
pub type ECCWBDIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, IIRECCCNT_SPEC, ECCWBDIS_A, O>;
impl<'a, const O: u8> ECCWBDIS_W<'a, O> {
    #[doc = "The error-corrected data write-back is enabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ECCWBDIS_A::_0)
    }
    #[doc = "The error-corrected data write-back is disabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ECCWBDIS_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - ECC setting bit"]
    #[inline(always)]
    pub fn eccmd(&self) -> ECCMD_R {
        ECCMD_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ECC-corrected data write-back disable bit"]
    #[inline(always)]
    pub fn eccwbdis(&self) -> ECCWBDIS_R {
        ECCWBDIS_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ECC setting bit"]
    #[inline(always)]
    #[must_use]
    pub fn eccmd(&mut self) -> ECCMD_W<0> {
        ECCMD_W::new(self)
    }
    #[doc = "Bit 1 - ECC-corrected data write-back disable bit"]
    #[inline(always)]
    #[must_use]
    pub fn eccwbdis(&mut self) -> ECCWBDIS_W<1> {
        ECCWBDIS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ECC Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iirecccnt](index.html) module"]
pub struct IIRECCCNT_SPEC;
impl crate::RegisterSpec for IIRECCCNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [iirecccnt::R](R) reader structure"]
impl crate::Readable for IIRECCCNT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [iirecccnt::W](W) writer structure"]
impl crate::Writable for IIRECCCNT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IIRECCCNT to value 0"]
impl crate::Resettable for IIRECCCNT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
