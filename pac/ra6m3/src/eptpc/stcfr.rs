#[doc = "Register `STCFR` reader"]
pub struct R(crate::R<STCFR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STCFR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STCFR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STCFR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `STCFR` writer"]
pub struct W(crate::W<STCFR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<STCFR_SPEC>;
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
impl From<crate::W<STCFR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<STCFR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `STCF` reader - STCA Clock Frequency"]
pub type STCF_R = crate::FieldReader<u8, STCF_A>;
#[doc = "STCA Clock Frequency\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum STCF_A {
    #[doc = "0: 20MHz"]
    _00 = 0,
    #[doc = "1: 25MHz"]
    _01 = 1,
    #[doc = "2: 50MHz"]
    _10 = 2,
    #[doc = "3: 100 MHz"]
    _11 = 3,
}
impl From<STCF_A> for u8 {
    #[inline(always)]
    fn from(variant: STCF_A) -> Self {
        variant as _
    }
}
impl STCF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STCF_A {
        match self.bits {
            0 => STCF_A::_00,
            1 => STCF_A::_01,
            2 => STCF_A::_10,
            3 => STCF_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == STCF_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == STCF_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == STCF_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == STCF_A::_11
    }
}
#[doc = "Field `STCF` writer - STCA Clock Frequency"]
pub type STCF_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, STCFR_SPEC, u8, STCF_A, 2, O>;
impl<'a, const O: u8> STCF_W<'a, O> {
    #[doc = "20MHz"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(STCF_A::_00)
    }
    #[doc = "25MHz"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(STCF_A::_01)
    }
    #[doc = "50MHz"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(STCF_A::_10)
    }
    #[doc = "100 MHz"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(STCF_A::_11)
    }
}
impl R {
    #[doc = "Bits 0:1 - STCA Clock Frequency"]
    #[inline(always)]
    pub fn stcf(&self) -> STCF_R {
        STCF_R::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - STCA Clock Frequency"]
    #[inline(always)]
    #[must_use]
    pub fn stcf(&mut self) -> STCF_W<0> {
        STCF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "STCA Clock Frequency Setting Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stcfr](index.html) module"]
pub struct STCFR_SPEC;
impl crate::RegisterSpec for STCFR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [stcfr::R](R) reader structure"]
impl crate::Readable for STCFR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [stcfr::W](W) writer structure"]
impl crate::Writable for STCFR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets STCFR to value 0"]
impl crate::Resettable for STCFR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
