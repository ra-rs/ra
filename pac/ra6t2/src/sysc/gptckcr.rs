#[doc = "Register `GPTCKCR` reader"]
pub struct R(crate::R<GPTCKCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPTCKCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GPTCKCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GPTCKCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GPTCKCR` writer"]
pub struct W(crate::W<GPTCKCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GPTCKCR_SPEC>;
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
impl From<crate::W<GPTCKCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GPTCKCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `GPTCKSEL` reader - GPT clock (GPTCLK) Source Select"]
pub type GPTCKSEL_R = crate::FieldReader<u8, GPTCKSEL_A>;
#[doc = "GPT clock (GPTCLK) Source Select\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum GPTCKSEL_A {
    #[doc = "0: HOCO"]
    _000 = 0,
    #[doc = "1: MOCO (value after reset)"]
    _001 = 1,
    #[doc = "2: LOCO"]
    _010 = 2,
    #[doc = "3: Main clock oscillator"]
    _011 = 3,
    #[doc = "5: PLL"]
    _101 = 5,
    #[doc = "6: PLL2"]
    _110 = 6,
}
impl From<GPTCKSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: GPTCKSEL_A) -> Self {
        variant as _
    }
}
impl GPTCKSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<GPTCKSEL_A> {
        match self.bits {
            0 => Some(GPTCKSEL_A::_000),
            1 => Some(GPTCKSEL_A::_001),
            2 => Some(GPTCKSEL_A::_010),
            3 => Some(GPTCKSEL_A::_011),
            5 => Some(GPTCKSEL_A::_101),
            6 => Some(GPTCKSEL_A::_110),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == GPTCKSEL_A::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == GPTCKSEL_A::_001
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == GPTCKSEL_A::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == GPTCKSEL_A::_011
    }
    #[doc = "Checks if the value of the field is `_101`"]
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == GPTCKSEL_A::_101
    }
    #[doc = "Checks if the value of the field is `_110`"]
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        *self == GPTCKSEL_A::_110
    }
}
#[doc = "Field `GPTCKSEL` writer - GPT clock (GPTCLK) Source Select"]
pub type GPTCKSEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u8, GPTCKCR_SPEC, u8, GPTCKSEL_A, 3, O>;
impl<'a, const O: u8> GPTCKSEL_W<'a, O> {
    #[doc = "HOCO"]
    #[inline(always)]
    pub fn _000(self) -> &'a mut W {
        self.variant(GPTCKSEL_A::_000)
    }
    #[doc = "MOCO (value after reset)"]
    #[inline(always)]
    pub fn _001(self) -> &'a mut W {
        self.variant(GPTCKSEL_A::_001)
    }
    #[doc = "LOCO"]
    #[inline(always)]
    pub fn _010(self) -> &'a mut W {
        self.variant(GPTCKSEL_A::_010)
    }
    #[doc = "Main clock oscillator"]
    #[inline(always)]
    pub fn _011(self) -> &'a mut W {
        self.variant(GPTCKSEL_A::_011)
    }
    #[doc = "PLL"]
    #[inline(always)]
    pub fn _101(self) -> &'a mut W {
        self.variant(GPTCKSEL_A::_101)
    }
    #[doc = "PLL2"]
    #[inline(always)]
    pub fn _110(self) -> &'a mut W {
        self.variant(GPTCKSEL_A::_110)
    }
}
#[doc = "Field `GPTCKSREQ` reader - GPT clock (GPTCLK) Switching Request"]
pub type GPTCKSREQ_R = crate::BitReader<GPTCKSREQ_A>;
#[doc = "GPT clock (GPTCLK) Switching Request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPTCKSREQ_A {
    #[doc = "0: No request"]
    _0 = 0,
    #[doc = "1: Request switching"]
    _1 = 1,
}
impl From<GPTCKSREQ_A> for bool {
    #[inline(always)]
    fn from(variant: GPTCKSREQ_A) -> Self {
        variant as u8 != 0
    }
}
impl GPTCKSREQ_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPTCKSREQ_A {
        match self.bits {
            false => GPTCKSREQ_A::_0,
            true => GPTCKSREQ_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == GPTCKSREQ_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == GPTCKSREQ_A::_1
    }
}
#[doc = "Field `GPTCKSREQ` writer - GPT clock (GPTCLK) Switching Request"]
pub type GPTCKSREQ_W<'a, const O: u8> = crate::BitWriter<'a, u8, GPTCKCR_SPEC, GPTCKSREQ_A, O>;
impl<'a, const O: u8> GPTCKSREQ_W<'a, O> {
    #[doc = "No request"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(GPTCKSREQ_A::_0)
    }
    #[doc = "Request switching"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(GPTCKSREQ_A::_1)
    }
}
#[doc = "Field `GPTCKSRDY` reader - GPT clock (GPTCLK) Switching Ready state flag"]
pub type GPTCKSRDY_R = crate::BitReader<GPTCKSRDY_A>;
#[doc = "GPT clock (GPTCLK) Switching Ready state flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPTCKSRDY_A {
    #[doc = "0: Impossible to Switch"]
    _0 = 0,
    #[doc = "1: Possible to Switch"]
    _1 = 1,
}
impl From<GPTCKSRDY_A> for bool {
    #[inline(always)]
    fn from(variant: GPTCKSRDY_A) -> Self {
        variant as u8 != 0
    }
}
impl GPTCKSRDY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPTCKSRDY_A {
        match self.bits {
            false => GPTCKSRDY_A::_0,
            true => GPTCKSRDY_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == GPTCKSRDY_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == GPTCKSRDY_A::_1
    }
}
impl R {
    #[doc = "Bits 0:2 - GPT clock (GPTCLK) Source Select"]
    #[inline(always)]
    pub fn gptcksel(&self) -> GPTCKSEL_R {
        GPTCKSEL_R::new(self.bits & 7)
    }
    #[doc = "Bit 6 - GPT clock (GPTCLK) Switching Request"]
    #[inline(always)]
    pub fn gptcksreq(&self) -> GPTCKSREQ_R {
        GPTCKSREQ_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - GPT clock (GPTCLK) Switching Ready state flag"]
    #[inline(always)]
    pub fn gptcksrdy(&self) -> GPTCKSRDY_R {
        GPTCKSRDY_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - GPT clock (GPTCLK) Source Select"]
    #[inline(always)]
    #[must_use]
    pub fn gptcksel(&mut self) -> GPTCKSEL_W<0> {
        GPTCKSEL_W::new(self)
    }
    #[doc = "Bit 6 - GPT clock (GPTCLK) Switching Request"]
    #[inline(always)]
    #[must_use]
    pub fn gptcksreq(&mut self) -> GPTCKSREQ_W<6> {
        GPTCKSREQ_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPT Clock Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gptckcr](index.html) module"]
pub struct GPTCKCR_SPEC;
impl crate::RegisterSpec for GPTCKCR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [gptckcr::R](R) reader structure"]
impl crate::Readable for GPTCKCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gptckcr::W](W) writer structure"]
impl crate::Writable for GPTCKCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GPTCKCR to value 0x01"]
impl crate::Resettable for GPTCKCR_SPEC {
    const RESET_VALUE: Self::Ux = 0x01;
}
