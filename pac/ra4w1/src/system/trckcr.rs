#[doc = "Register `TRCKCR` reader"]
pub struct R(crate::R<TRCKCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TRCKCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TRCKCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TRCKCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TRCKCR` writer"]
pub struct W(crate::W<TRCKCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TRCKCR_SPEC>;
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
impl From<crate::W<TRCKCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TRCKCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TRCK` reader - Trace Clock operating frequency select"]
pub type TRCK_R = crate::FieldReader<u8, TRCK_A>;
#[doc = "Trace Clock operating frequency select\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TRCK_A {
    #[doc = "0: /1"]
    _0000 = 0,
    #[doc = "1: /2(value after reset)"]
    _0001 = 1,
    #[doc = "2: /4"]
    _0010 = 2,
}
impl From<TRCK_A> for u8 {
    #[inline(always)]
    fn from(variant: TRCK_A) -> Self {
        variant as _
    }
}
impl TRCK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<TRCK_A> {
        match self.bits {
            0 => Some(TRCK_A::_0000),
            1 => Some(TRCK_A::_0001),
            2 => Some(TRCK_A::_0010),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0000`"]
    #[inline(always)]
    pub fn is_0000(&self) -> bool {
        *self == TRCK_A::_0000
    }
    #[doc = "Checks if the value of the field is `_0001`"]
    #[inline(always)]
    pub fn is_0001(&self) -> bool {
        *self == TRCK_A::_0001
    }
    #[doc = "Checks if the value of the field is `_0010`"]
    #[inline(always)]
    pub fn is_0010(&self) -> bool {
        *self == TRCK_A::_0010
    }
}
#[doc = "Field `TRCK` writer - Trace Clock operating frequency select"]
pub type TRCK_W<'a, const O: u8> = crate::FieldWriter<'a, u8, TRCKCR_SPEC, u8, TRCK_A, 4, O>;
impl<'a, const O: u8> TRCK_W<'a, O> {
    #[doc = "/1"]
    #[inline(always)]
    pub fn _0000(self) -> &'a mut W {
        self.variant(TRCK_A::_0000)
    }
    #[doc = "/2(value after reset)"]
    #[inline(always)]
    pub fn _0001(self) -> &'a mut W {
        self.variant(TRCK_A::_0001)
    }
    #[doc = "/4"]
    #[inline(always)]
    pub fn _0010(self) -> &'a mut W {
        self.variant(TRCK_A::_0010)
    }
}
#[doc = "Field `TRCKEN` reader - Trace Clock operating enable"]
pub type TRCKEN_R = crate::BitReader<TRCKEN_A>;
#[doc = "Trace Clock operating enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TRCKEN_A {
    #[doc = "0: Operation disabled"]
    _0 = 0,
    #[doc = "1: Operation enabled."]
    _1 = 1,
}
impl From<TRCKEN_A> for bool {
    #[inline(always)]
    fn from(variant: TRCKEN_A) -> Self {
        variant as u8 != 0
    }
}
impl TRCKEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TRCKEN_A {
        match self.bits {
            false => TRCKEN_A::_0,
            true => TRCKEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TRCKEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TRCKEN_A::_1
    }
}
#[doc = "Field `TRCKEN` writer - Trace Clock operating enable"]
pub type TRCKEN_W<'a, const O: u8> = crate::BitWriter<'a, u8, TRCKCR_SPEC, TRCKEN_A, O>;
impl<'a, const O: u8> TRCKEN_W<'a, O> {
    #[doc = "Operation disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TRCKEN_A::_0)
    }
    #[doc = "Operation enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TRCKEN_A::_1)
    }
}
impl R {
    #[doc = "Bits 0:3 - Trace Clock operating frequency select"]
    #[inline(always)]
    pub fn trck(&self) -> TRCK_R {
        TRCK_R::new(self.bits & 0x0f)
    }
    #[doc = "Bit 7 - Trace Clock operating enable"]
    #[inline(always)]
    pub fn trcken(&self) -> TRCKEN_R {
        TRCKEN_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Trace Clock operating frequency select"]
    #[inline(always)]
    #[must_use]
    pub fn trck(&mut self) -> TRCK_W<0> {
        TRCK_W::new(self)
    }
    #[doc = "Bit 7 - Trace Clock operating enable"]
    #[inline(always)]
    #[must_use]
    pub fn trcken(&mut self) -> TRCKEN_W<7> {
        TRCKEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Trace Clock Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [trckcr](index.html) module"]
pub struct TRCKCR_SPEC;
impl crate::RegisterSpec for TRCKCR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [trckcr::R](R) reader structure"]
impl crate::Readable for TRCKCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [trckcr::W](W) writer structure"]
impl crate::Writable for TRCKCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TRCKCR to value 0x01"]
impl crate::Resettable for TRCKCR_SPEC {
    const RESET_VALUE: Self::Ux = 0x01;
}
