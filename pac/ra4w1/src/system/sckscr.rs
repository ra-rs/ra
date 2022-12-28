#[doc = "Register `SCKSCR` reader"]
pub struct R(crate::R<SCKSCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SCKSCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SCKSCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SCKSCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SCKSCR` writer"]
pub struct W(crate::W<SCKSCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SCKSCR_SPEC>;
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
impl From<crate::W<SCKSCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SCKSCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CKSEL` reader - Clock Source SelectSelecting the system clock source faster than 32MHz(system clock source > 32MHz ) is prohibit when SCKDIVCR.ICK\\[2:0\\]
bits select the division-by-1 and MEMWAIT.MEMWAIT =0."]
pub type CKSEL_R = crate::FieldReader<u8, CKSEL_A>;
#[doc = "Clock Source SelectSelecting the system clock source faster than 32MHz(system clock source > 32MHz ) is prohibit when SCKDIVCR.ICK\\[2:0\\]
bits select the division-by-1 and MEMWAIT.MEMWAIT =0.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CKSEL_A {
    #[doc = "0: HOCO"]
    _000 = 0,
    #[doc = "1: MOCO"]
    _001 = 1,
    #[doc = "2: LOCO"]
    _010 = 2,
    #[doc = "3: Main clock oscillator"]
    _011 = 3,
    #[doc = "4: Sub-clock oscillator"]
    _100 = 4,
    #[doc = "5: PLL"]
    _101 = 5,
}
impl From<CKSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: CKSEL_A) -> Self {
        variant as _
    }
}
impl CKSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CKSEL_A> {
        match self.bits {
            0 => Some(CKSEL_A::_000),
            1 => Some(CKSEL_A::_001),
            2 => Some(CKSEL_A::_010),
            3 => Some(CKSEL_A::_011),
            4 => Some(CKSEL_A::_100),
            5 => Some(CKSEL_A::_101),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == CKSEL_A::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == CKSEL_A::_001
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == CKSEL_A::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == CKSEL_A::_011
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == CKSEL_A::_100
    }
    #[doc = "Checks if the value of the field is `_101`"]
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == CKSEL_A::_101
    }
}
#[doc = "Field `CKSEL` writer - Clock Source SelectSelecting the system clock source faster than 32MHz(system clock source > 32MHz ) is prohibit when SCKDIVCR.ICK\\[2:0\\]
bits select the division-by-1 and MEMWAIT.MEMWAIT =0."]
pub type CKSEL_W<'a, const O: u8> = crate::FieldWriter<'a, u8, SCKSCR_SPEC, u8, CKSEL_A, 3, O>;
impl<'a, const O: u8> CKSEL_W<'a, O> {
    #[doc = "HOCO"]
    #[inline(always)]
    pub fn _000(self) -> &'a mut W {
        self.variant(CKSEL_A::_000)
    }
    #[doc = "MOCO"]
    #[inline(always)]
    pub fn _001(self) -> &'a mut W {
        self.variant(CKSEL_A::_001)
    }
    #[doc = "LOCO"]
    #[inline(always)]
    pub fn _010(self) -> &'a mut W {
        self.variant(CKSEL_A::_010)
    }
    #[doc = "Main clock oscillator"]
    #[inline(always)]
    pub fn _011(self) -> &'a mut W {
        self.variant(CKSEL_A::_011)
    }
    #[doc = "Sub-clock oscillator"]
    #[inline(always)]
    pub fn _100(self) -> &'a mut W {
        self.variant(CKSEL_A::_100)
    }
    #[doc = "PLL"]
    #[inline(always)]
    pub fn _101(self) -> &'a mut W {
        self.variant(CKSEL_A::_101)
    }
}
impl R {
    #[doc = "Bits 0:2 - Clock Source SelectSelecting the system clock source faster than 32MHz(system clock source > 32MHz ) is prohibit when SCKDIVCR.ICK\\[2:0\\]
bits select the division-by-1 and MEMWAIT.MEMWAIT =0."]
    #[inline(always)]
    pub fn cksel(&self) -> CKSEL_R {
        CKSEL_R::new(self.bits & 7)
    }
}
impl W {
    #[doc = "Bits 0:2 - Clock Source SelectSelecting the system clock source faster than 32MHz(system clock source > 32MHz ) is prohibit when SCKDIVCR.ICK\\[2:0\\]
bits select the division-by-1 and MEMWAIT.MEMWAIT =0."]
    #[inline(always)]
    #[must_use]
    pub fn cksel(&mut self) -> CKSEL_W<0> {
        CKSEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "System Clock Source Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sckscr](index.html) module"]
pub struct SCKSCR_SPEC;
impl crate::RegisterSpec for SCKSCR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [sckscr::R](R) reader structure"]
impl crate::Readable for SCKSCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sckscr::W](W) writer structure"]
impl crate::Writable for SCKSCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SCKSCR to value 0x01"]
impl crate::Resettable for SCKSCR_SPEC {
    const RESET_VALUE: Self::Ux = 0x01;
}
