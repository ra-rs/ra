#[doc = "Register `MEMWAIT` reader"]
pub struct R(crate::R<MEMWAIT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MEMWAIT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MEMWAIT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MEMWAIT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MEMWAIT` writer"]
pub struct W(crate::W<MEMWAIT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MEMWAIT_SPEC>;
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
impl From<crate::W<MEMWAIT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MEMWAIT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MEMWAIT` reader - Memory Wait Cycle Select Note: Writing 0 to the MEMWAIT is prohibited when SCKDIVCR.ICK selects division by 1 and SCKSCR.CKSEL\\[2:0\\]
bits select the system clock source that is faster than 32 MHz (ICLK > 32 MHz)."]
pub type MEMWAIT_R = crate::BitReader<MEMWAIT_A>;
#[doc = "Memory Wait Cycle Select Note: Writing 0 to the MEMWAIT is prohibited when SCKDIVCR.ICK selects division by 1 and SCKSCR.CKSEL\\[2:0\\]
bits select the system clock source that is faster than 32 MHz (ICLK > 32 MHz).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MEMWAIT_A {
    #[doc = "0: no wait"]
    _0 = 0,
    #[doc = "1: wait"]
    _1 = 1,
}
impl From<MEMWAIT_A> for bool {
    #[inline(always)]
    fn from(variant: MEMWAIT_A) -> Self {
        variant as u8 != 0
    }
}
impl MEMWAIT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MEMWAIT_A {
        match self.bits {
            false => MEMWAIT_A::_0,
            true => MEMWAIT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MEMWAIT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MEMWAIT_A::_1
    }
}
#[doc = "Field `MEMWAIT` writer - Memory Wait Cycle Select Note: Writing 0 to the MEMWAIT is prohibited when SCKDIVCR.ICK selects division by 1 and SCKSCR.CKSEL\\[2:0\\]
bits select the system clock source that is faster than 32 MHz (ICLK > 32 MHz)."]
pub type MEMWAIT_W<'a, const O: u8> = crate::BitWriter<'a, u8, MEMWAIT_SPEC, MEMWAIT_A, O>;
impl<'a, const O: u8> MEMWAIT_W<'a, O> {
    #[doc = "no wait"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MEMWAIT_A::_0)
    }
    #[doc = "wait"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MEMWAIT_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Memory Wait Cycle Select Note: Writing 0 to the MEMWAIT is prohibited when SCKDIVCR.ICK selects division by 1 and SCKSCR.CKSEL\\[2:0\\]
bits select the system clock source that is faster than 32 MHz (ICLK > 32 MHz)."]
    #[inline(always)]
    pub fn memwait(&self) -> MEMWAIT_R {
        MEMWAIT_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Memory Wait Cycle Select Note: Writing 0 to the MEMWAIT is prohibited when SCKDIVCR.ICK selects division by 1 and SCKSCR.CKSEL\\[2:0\\]
bits select the system clock source that is faster than 32 MHz (ICLK > 32 MHz)."]
    #[inline(always)]
    #[must_use]
    pub fn memwait(&mut self) -> MEMWAIT_W<0> {
        MEMWAIT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Memory Wait Cycle Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [memwait](index.html) module"]
pub struct MEMWAIT_SPEC;
impl crate::RegisterSpec for MEMWAIT_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [memwait::R](R) reader structure"]
impl crate::Readable for MEMWAIT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [memwait::W](W) writer structure"]
impl crate::Writable for MEMWAIT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MEMWAIT to value 0"]
impl crate::Resettable for MEMWAIT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
