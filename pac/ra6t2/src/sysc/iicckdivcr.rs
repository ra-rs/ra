#[doc = "Register `IICCKDIVCR` reader"]
pub struct R(crate::R<IICCKDIVCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IICCKDIVCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IICCKDIVCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IICCKDIVCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IICCKDIVCR` writer"]
pub struct W(crate::W<IICCKDIVCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IICCKDIVCR_SPEC>;
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
impl From<crate::W<IICCKDIVCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IICCKDIVCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IICCKDIV` reader - IIC clock (IICCLK) Division Select"]
pub type IICCKDIV_R = crate::FieldReader<u8, IICCKDIV_A>;
#[doc = "IIC clock (IICCLK) Division Select\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum IICCKDIV_A {
    #[doc = "0: /1 (value after reset)"]
    _000 = 0,
    #[doc = "1: /2"]
    _001 = 1,
    #[doc = "2: /4"]
    _010 = 2,
    #[doc = "3: /6"]
    _011 = 3,
    #[doc = "4: /8"]
    _100 = 4,
}
impl From<IICCKDIV_A> for u8 {
    #[inline(always)]
    fn from(variant: IICCKDIV_A) -> Self {
        variant as _
    }
}
impl IICCKDIV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<IICCKDIV_A> {
        match self.bits {
            0 => Some(IICCKDIV_A::_000),
            1 => Some(IICCKDIV_A::_001),
            2 => Some(IICCKDIV_A::_010),
            3 => Some(IICCKDIV_A::_011),
            4 => Some(IICCKDIV_A::_100),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == IICCKDIV_A::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == IICCKDIV_A::_001
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == IICCKDIV_A::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == IICCKDIV_A::_011
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == IICCKDIV_A::_100
    }
}
#[doc = "Field `IICCKDIV` writer - IIC clock (IICCLK) Division Select"]
pub type IICCKDIV_W<'a, const O: u8> =
    crate::FieldWriter<'a, u8, IICCKDIVCR_SPEC, u8, IICCKDIV_A, 3, O>;
impl<'a, const O: u8> IICCKDIV_W<'a, O> {
    #[doc = "/1 (value after reset)"]
    #[inline(always)]
    pub fn _000(self) -> &'a mut W {
        self.variant(IICCKDIV_A::_000)
    }
    #[doc = "/2"]
    #[inline(always)]
    pub fn _001(self) -> &'a mut W {
        self.variant(IICCKDIV_A::_001)
    }
    #[doc = "/4"]
    #[inline(always)]
    pub fn _010(self) -> &'a mut W {
        self.variant(IICCKDIV_A::_010)
    }
    #[doc = "/6"]
    #[inline(always)]
    pub fn _011(self) -> &'a mut W {
        self.variant(IICCKDIV_A::_011)
    }
    #[doc = "/8"]
    #[inline(always)]
    pub fn _100(self) -> &'a mut W {
        self.variant(IICCKDIV_A::_100)
    }
}
impl R {
    #[doc = "Bits 0:2 - IIC clock (IICCLK) Division Select"]
    #[inline(always)]
    pub fn iicckdiv(&self) -> IICCKDIV_R {
        IICCKDIV_R::new(self.bits & 7)
    }
}
impl W {
    #[doc = "Bits 0:2 - IIC clock (IICCLK) Division Select"]
    #[inline(always)]
    #[must_use]
    pub fn iicckdiv(&mut self) -> IICCKDIV_W<0> {
        IICCKDIV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "IIC Clock Division Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iicckdivcr](index.html) module"]
pub struct IICCKDIVCR_SPEC;
impl crate::RegisterSpec for IICCKDIVCR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [iicckdivcr::R](R) reader structure"]
impl crate::Readable for IICCKDIVCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [iicckdivcr::W](W) writer structure"]
impl crate::Writable for IICCKDIVCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IICCKDIVCR to value 0x01"]
impl crate::Resettable for IICCKDIVCR_SPEC {
    const RESET_VALUE: Self::Ux = 0x01;
}
