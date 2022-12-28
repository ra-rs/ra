#[doc = "Register `SIMR1` reader"]
pub struct R(crate::R<SIMR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SIMR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SIMR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SIMR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SIMR1` writer"]
pub struct W(crate::W<SIMR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SIMR1_SPEC>;
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
impl From<crate::W<SIMR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SIMR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IICM` reader - Simple IIC Mode Select"]
pub type IICM_R = crate::BitReader<IICM_A>;
#[doc = "Simple IIC Mode Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IICM_A {
    #[doc = "0: SCMR.SMIF = 0: Asynchronous mode (including multi-processor mode), clock synchronous mode, or simple SPI mode SCMR.SMIF = 1: Smart card interface mode"]
    _0 = 0,
    #[doc = "1: SCMR.SMIF = 0: Simple IIC mode SCMR.SMIF = 1: Setting prohibited"]
    _1 = 1,
}
impl From<IICM_A> for bool {
    #[inline(always)]
    fn from(variant: IICM_A) -> Self {
        variant as u8 != 0
    }
}
impl IICM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IICM_A {
        match self.bits {
            false => IICM_A::_0,
            true => IICM_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == IICM_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == IICM_A::_1
    }
}
#[doc = "Field `IICM` writer - Simple IIC Mode Select"]
pub type IICM_W<'a, const O: u8> = crate::BitWriter<'a, u8, SIMR1_SPEC, IICM_A, O>;
impl<'a, const O: u8> IICM_W<'a, O> {
    #[doc = "SCMR.SMIF = 0: Asynchronous mode (including multi-processor mode), clock synchronous mode, or simple SPI mode SCMR.SMIF = 1: Smart card interface mode"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(IICM_A::_0)
    }
    #[doc = "SCMR.SMIF = 0: Simple IIC mode SCMR.SMIF = 1: Setting prohibited"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(IICM_A::_1)
    }
}
#[doc = "Field `IICDL` reader - SDAn Delay Output Select"]
pub type IICDL_R = crate::FieldReader<u8, IICDL_A>;
#[doc = "SDAn Delay Output Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum IICDL_A {
    #[doc = "0: No output delay"]
    _0X00 = 0,
}
impl From<IICDL_A> for u8 {
    #[inline(always)]
    fn from(variant: IICDL_A) -> Self {
        variant as _
    }
}
impl IICDL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<IICDL_A> {
        match self.bits {
            0 => Some(IICDL_A::_0X00),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0X00`"]
    #[inline(always)]
    pub fn is_0x00(&self) -> bool {
        *self == IICDL_A::_0X00
    }
}
#[doc = "Field `IICDL` writer - SDAn Delay Output Select"]
pub type IICDL_W<'a, const O: u8> = crate::FieldWriter<'a, u8, SIMR1_SPEC, u8, IICDL_A, 5, O>;
impl<'a, const O: u8> IICDL_W<'a, O> {
    #[doc = "No output delay"]
    #[inline(always)]
    pub fn _0x00(self) -> &'a mut W {
        self.variant(IICDL_A::_0X00)
    }
}
impl R {
    #[doc = "Bit 0 - Simple IIC Mode Select"]
    #[inline(always)]
    pub fn iicm(&self) -> IICM_R {
        IICM_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 3:7 - SDAn Delay Output Select"]
    #[inline(always)]
    pub fn iicdl(&self) -> IICDL_R {
        IICDL_R::new((self.bits >> 3) & 0x1f)
    }
}
impl W {
    #[doc = "Bit 0 - Simple IIC Mode Select"]
    #[inline(always)]
    #[must_use]
    pub fn iicm(&mut self) -> IICM_W<0> {
        IICM_W::new(self)
    }
    #[doc = "Bits 3:7 - SDAn Delay Output Select"]
    #[inline(always)]
    #[must_use]
    pub fn iicdl(&mut self) -> IICDL_W<3> {
        IICDL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "IIC Mode Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [simr1](index.html) module"]
pub struct SIMR1_SPEC;
impl crate::RegisterSpec for SIMR1_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [simr1::R](R) reader structure"]
impl crate::Readable for SIMR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [simr1::W](W) writer structure"]
impl crate::Writable for SIMR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SIMR1 to value 0"]
impl crate::Resettable for SIMR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
