#[doc = "Register `FLSTOP` reader"]
pub struct R(crate::R<FLSTOP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FLSTOP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FLSTOP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FLSTOP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FLSTOP` writer"]
pub struct W(crate::W<FLSTOP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FLSTOP_SPEC>;
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
impl From<crate::W<FLSTOP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FLSTOP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FLSTOP` reader - Selecting ON/OFF of the Flash Memory Operation"]
pub type FLSTOP_R = crate::BitReader<FLSTOP_A>;
#[doc = "Selecting ON/OFF of the Flash Memory Operation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FLSTOP_A {
    #[doc = "0: Code flash and data flash memory operates"]
    _0 = 0,
    #[doc = "1: Code flash and data flash memory stops."]
    _1 = 1,
}
impl From<FLSTOP_A> for bool {
    #[inline(always)]
    fn from(variant: FLSTOP_A) -> Self {
        variant as u8 != 0
    }
}
impl FLSTOP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLSTOP_A {
        match self.bits {
            false => FLSTOP_A::_0,
            true => FLSTOP_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FLSTOP_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FLSTOP_A::_1
    }
}
#[doc = "Field `FLSTOP` writer - Selecting ON/OFF of the Flash Memory Operation"]
pub type FLSTOP_W<'a, const O: u8> = crate::BitWriter<'a, u8, FLSTOP_SPEC, FLSTOP_A, O>;
impl<'a, const O: u8> FLSTOP_W<'a, O> {
    #[doc = "Code flash and data flash memory operates"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FLSTOP_A::_0)
    }
    #[doc = "Code flash and data flash memory stops."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FLSTOP_A::_1)
    }
}
#[doc = "Field `FLSTPF` reader - Flash Memory Operation Status Flag"]
pub type FLSTPF_R = crate::BitReader<FLSTPF_A>;
#[doc = "Flash Memory Operation Status Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FLSTPF_A {
    #[doc = "0: Transition completed"]
    _0 = 0,
    #[doc = "1: During transition (from the flash-stop-status to flash-operating-status or vice versa)"]
    _1 = 1,
}
impl From<FLSTPF_A> for bool {
    #[inline(always)]
    fn from(variant: FLSTPF_A) -> Self {
        variant as u8 != 0
    }
}
impl FLSTPF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLSTPF_A {
        match self.bits {
            false => FLSTPF_A::_0,
            true => FLSTPF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FLSTPF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FLSTPF_A::_1
    }
}
#[doc = "Field `FLSTPF` writer - Flash Memory Operation Status Flag"]
pub type FLSTPF_W<'a, const O: u8> = crate::BitWriter<'a, u8, FLSTOP_SPEC, FLSTPF_A, O>;
impl<'a, const O: u8> FLSTPF_W<'a, O> {
    #[doc = "Transition completed"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FLSTPF_A::_0)
    }
    #[doc = "During transition (from the flash-stop-status to flash-operating-status or vice versa)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FLSTPF_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Selecting ON/OFF of the Flash Memory Operation"]
    #[inline(always)]
    pub fn flstop(&self) -> FLSTOP_R {
        FLSTOP_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - Flash Memory Operation Status Flag"]
    #[inline(always)]
    pub fn flstpf(&self) -> FLSTPF_R {
        FLSTPF_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Selecting ON/OFF of the Flash Memory Operation"]
    #[inline(always)]
    #[must_use]
    pub fn flstop(&mut self) -> FLSTOP_W<0> {
        FLSTOP_W::new(self)
    }
    #[doc = "Bit 4 - Flash Memory Operation Status Flag"]
    #[inline(always)]
    #[must_use]
    pub fn flstpf(&mut self) -> FLSTPF_W<4> {
        FLSTPF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Flash Operation Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flstop](index.html) module"]
pub struct FLSTOP_SPEC;
impl crate::RegisterSpec for FLSTOP_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [flstop::R](R) reader structure"]
impl crate::Readable for FLSTOP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [flstop::W](W) writer structure"]
impl crate::Writable for FLSTOP_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FLSTOP to value 0"]
impl crate::Resettable for FLSTOP_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
