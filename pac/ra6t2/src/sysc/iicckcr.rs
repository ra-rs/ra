#[doc = "Register `IICCKCR` reader"]
pub struct R(crate::R<IICCKCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IICCKCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IICCKCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IICCKCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IICCKCR` writer"]
pub struct W(crate::W<IICCKCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IICCKCR_SPEC>;
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
impl From<crate::W<IICCKCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IICCKCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IICCKSEL` reader - IIC clock (IICCLK) Source Select"]
pub type IICCKSEL_R = crate::FieldReader<u8, IICCKSEL_A>;
#[doc = "IIC clock (IICCLK) Source Select\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum IICCKSEL_A {
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
impl From<IICCKSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: IICCKSEL_A) -> Self {
        variant as _
    }
}
impl IICCKSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<IICCKSEL_A> {
        match self.bits {
            0 => Some(IICCKSEL_A::_000),
            1 => Some(IICCKSEL_A::_001),
            2 => Some(IICCKSEL_A::_010),
            3 => Some(IICCKSEL_A::_011),
            5 => Some(IICCKSEL_A::_101),
            6 => Some(IICCKSEL_A::_110),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == IICCKSEL_A::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == IICCKSEL_A::_001
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == IICCKSEL_A::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == IICCKSEL_A::_011
    }
    #[doc = "Checks if the value of the field is `_101`"]
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == IICCKSEL_A::_101
    }
    #[doc = "Checks if the value of the field is `_110`"]
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        *self == IICCKSEL_A::_110
    }
}
#[doc = "Field `IICCKSEL` writer - IIC clock (IICCLK) Source Select"]
pub type IICCKSEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u8, IICCKCR_SPEC, u8, IICCKSEL_A, 3, O>;
impl<'a, const O: u8> IICCKSEL_W<'a, O> {
    #[doc = "HOCO"]
    #[inline(always)]
    pub fn _000(self) -> &'a mut W {
        self.variant(IICCKSEL_A::_000)
    }
    #[doc = "MOCO (value after reset)"]
    #[inline(always)]
    pub fn _001(self) -> &'a mut W {
        self.variant(IICCKSEL_A::_001)
    }
    #[doc = "LOCO"]
    #[inline(always)]
    pub fn _010(self) -> &'a mut W {
        self.variant(IICCKSEL_A::_010)
    }
    #[doc = "Main clock oscillator"]
    #[inline(always)]
    pub fn _011(self) -> &'a mut W {
        self.variant(IICCKSEL_A::_011)
    }
    #[doc = "PLL"]
    #[inline(always)]
    pub fn _101(self) -> &'a mut W {
        self.variant(IICCKSEL_A::_101)
    }
    #[doc = "PLL2"]
    #[inline(always)]
    pub fn _110(self) -> &'a mut W {
        self.variant(IICCKSEL_A::_110)
    }
}
#[doc = "Field `IICCKSREQ` reader - IIC clock (IICCLK) Switching Request"]
pub type IICCKSREQ_R = crate::BitReader<IICCKSREQ_A>;
#[doc = "IIC clock (IICCLK) Switching Request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IICCKSREQ_A {
    #[doc = "0: No request"]
    _0 = 0,
    #[doc = "1: Request switching"]
    _1 = 1,
}
impl From<IICCKSREQ_A> for bool {
    #[inline(always)]
    fn from(variant: IICCKSREQ_A) -> Self {
        variant as u8 != 0
    }
}
impl IICCKSREQ_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IICCKSREQ_A {
        match self.bits {
            false => IICCKSREQ_A::_0,
            true => IICCKSREQ_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == IICCKSREQ_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == IICCKSREQ_A::_1
    }
}
#[doc = "Field `IICCKSREQ` writer - IIC clock (IICCLK) Switching Request"]
pub type IICCKSREQ_W<'a, const O: u8> = crate::BitWriter<'a, u8, IICCKCR_SPEC, IICCKSREQ_A, O>;
impl<'a, const O: u8> IICCKSREQ_W<'a, O> {
    #[doc = "No request"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(IICCKSREQ_A::_0)
    }
    #[doc = "Request switching"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(IICCKSREQ_A::_1)
    }
}
#[doc = "Field `IICCKSRDY` reader - IIC clock (IICCLK) Switching Ready state flag"]
pub type IICCKSRDY_R = crate::BitReader<IICCKSRDY_A>;
#[doc = "IIC clock (IICCLK) Switching Ready state flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IICCKSRDY_A {
    #[doc = "0: Impossible to Switch"]
    _0 = 0,
    #[doc = "1: Possible to Switch"]
    _1 = 1,
}
impl From<IICCKSRDY_A> for bool {
    #[inline(always)]
    fn from(variant: IICCKSRDY_A) -> Self {
        variant as u8 != 0
    }
}
impl IICCKSRDY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IICCKSRDY_A {
        match self.bits {
            false => IICCKSRDY_A::_0,
            true => IICCKSRDY_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == IICCKSRDY_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == IICCKSRDY_A::_1
    }
}
impl R {
    #[doc = "Bits 0:2 - IIC clock (IICCLK) Source Select"]
    #[inline(always)]
    pub fn iiccksel(&self) -> IICCKSEL_R {
        IICCKSEL_R::new(self.bits & 7)
    }
    #[doc = "Bit 6 - IIC clock (IICCLK) Switching Request"]
    #[inline(always)]
    pub fn iiccksreq(&self) -> IICCKSREQ_R {
        IICCKSREQ_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - IIC clock (IICCLK) Switching Ready state flag"]
    #[inline(always)]
    pub fn iiccksrdy(&self) -> IICCKSRDY_R {
        IICCKSRDY_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - IIC clock (IICCLK) Source Select"]
    #[inline(always)]
    #[must_use]
    pub fn iiccksel(&mut self) -> IICCKSEL_W<0> {
        IICCKSEL_W::new(self)
    }
    #[doc = "Bit 6 - IIC clock (IICCLK) Switching Request"]
    #[inline(always)]
    #[must_use]
    pub fn iiccksreq(&mut self) -> IICCKSREQ_W<6> {
        IICCKSREQ_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "IIC Clock Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iicckcr](index.html) module"]
pub struct IICCKCR_SPEC;
impl crate::RegisterSpec for IICCKCR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [iicckcr::R](R) reader structure"]
impl crate::Readable for IICCKCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [iicckcr::W](W) writer structure"]
impl crate::Writable for IICCKCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IICCKCR to value 0x01"]
impl crate::Resettable for IICCKCR_SPEC {
    const RESET_VALUE: Self::Ux = 0x01;
}
