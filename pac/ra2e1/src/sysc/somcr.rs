#[doc = "Register `SOMCR` reader"]
pub struct R(crate::R<SOMCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SOMCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SOMCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SOMCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SOMCR` writer"]
pub struct W(crate::W<SOMCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SOMCR_SPEC>;
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
impl From<crate::W<SOMCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SOMCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SODRV` reader - Sub-Clock Oscillator Drive Capability Switching"]
pub type SODRV_R = crate::FieldReader<u8, SODRV_A>;
#[doc = "Sub-Clock Oscillator Drive Capability Switching\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SODRV_A {
    #[doc = "0: Normal Mode"]
    _00 = 0,
    #[doc = "1: Low Power Mode 1"]
    _01 = 1,
    #[doc = "2: Low Power Mode 2"]
    _10 = 2,
    #[doc = "3: Low Power Mode 3"]
    _11 = 3,
}
impl From<SODRV_A> for u8 {
    #[inline(always)]
    fn from(variant: SODRV_A) -> Self {
        variant as _
    }
}
impl SODRV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SODRV_A {
        match self.bits {
            0 => SODRV_A::_00,
            1 => SODRV_A::_01,
            2 => SODRV_A::_10,
            3 => SODRV_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == SODRV_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == SODRV_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == SODRV_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == SODRV_A::_11
    }
}
#[doc = "Field `SODRV` writer - Sub-Clock Oscillator Drive Capability Switching"]
pub type SODRV_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u8, SOMCR_SPEC, u8, SODRV_A, 2, O>;
impl<'a, const O: u8> SODRV_W<'a, O> {
    #[doc = "Normal Mode"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(SODRV_A::_00)
    }
    #[doc = "Low Power Mode 1"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(SODRV_A::_01)
    }
    #[doc = "Low Power Mode 2"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(SODRV_A::_10)
    }
    #[doc = "Low Power Mode 3"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(SODRV_A::_11)
    }
}
impl R {
    #[doc = "Bits 0:1 - Sub-Clock Oscillator Drive Capability Switching"]
    #[inline(always)]
    pub fn sodrv(&self) -> SODRV_R {
        SODRV_R::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Sub-Clock Oscillator Drive Capability Switching"]
    #[inline(always)]
    pub fn sodrv(&mut self) -> SODRV_W<0> {
        SODRV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Sub-Clock Oscillator Mode Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [somcr](index.html) module"]
pub struct SOMCR_SPEC;
impl crate::RegisterSpec for SOMCR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [somcr::R](R) reader structure"]
impl crate::Readable for SOMCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [somcr::W](W) writer structure"]
impl crate::Writable for SOMCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SOMCR to value 0"]
impl crate::Resettable for SOMCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
