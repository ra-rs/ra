#[doc = "Register `SPDCR` reader"]
pub struct R(crate::R<SPDCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPDCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SPDCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SPDCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SPDCR` writer"]
pub struct W(crate::W<SPDCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SPDCR_SPEC>;
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
impl From<crate::W<SPDCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SPDCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SPFC` reader - Number of Frames Specification"]
pub type SPFC_R = crate::FieldReader<u8, SPFC_A>;
#[doc = "Number of Frames Specification\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SPFC_A {
    #[doc = "0: 1 frame"]
    _00 = 0,
    #[doc = "1: 2 frames"]
    _01 = 1,
    #[doc = "2: 3 frames"]
    _10 = 2,
    #[doc = "3: 4 frames."]
    _11 = 3,
}
impl From<SPFC_A> for u8 {
    #[inline(always)]
    fn from(variant: SPFC_A) -> Self {
        variant as _
    }
}
impl SPFC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPFC_A {
        match self.bits {
            0 => SPFC_A::_00,
            1 => SPFC_A::_01,
            2 => SPFC_A::_10,
            3 => SPFC_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == SPFC_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == SPFC_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == SPFC_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == SPFC_A::_11
    }
}
#[doc = "Field `SPFC` writer - Number of Frames Specification"]
pub type SPFC_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u8, SPDCR_SPEC, u8, SPFC_A, 2, O>;
impl<'a, const O: u8> SPFC_W<'a, O> {
    #[doc = "1 frame"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(SPFC_A::_00)
    }
    #[doc = "2 frames"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(SPFC_A::_01)
    }
    #[doc = "3 frames"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(SPFC_A::_10)
    }
    #[doc = "4 frames."]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(SPFC_A::_11)
    }
}
#[doc = "Field `SPRDTD` reader - RSPI Receive/Transmit Data Selection"]
pub type SPRDTD_R = crate::BitReader<SPRDTD_A>;
#[doc = "RSPI Receive/Transmit Data Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SPRDTD_A {
    #[doc = "0: SPDR values are read from the receive buffer"]
    _0 = 0,
    #[doc = "1: SPDR values are read from the transmit buffer (but only if the transmit buffer is empty)"]
    _1 = 1,
}
impl From<SPRDTD_A> for bool {
    #[inline(always)]
    fn from(variant: SPRDTD_A) -> Self {
        variant as u8 != 0
    }
}
impl SPRDTD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPRDTD_A {
        match self.bits {
            false => SPRDTD_A::_0,
            true => SPRDTD_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SPRDTD_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SPRDTD_A::_1
    }
}
#[doc = "Field `SPRDTD` writer - RSPI Receive/Transmit Data Selection"]
pub type SPRDTD_W<'a, const O: u8> = crate::BitWriter<'a, u8, SPDCR_SPEC, SPRDTD_A, O>;
impl<'a, const O: u8> SPRDTD_W<'a, O> {
    #[doc = "SPDR values are read from the receive buffer"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SPRDTD_A::_0)
    }
    #[doc = "SPDR values are read from the transmit buffer (but only if the transmit buffer is empty)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SPRDTD_A::_1)
    }
}
#[doc = "Field `SPLW` reader - SPI Word Access/Halfword Access Specification"]
pub type SPLW_R = crate::BitReader<SPLW_A>;
#[doc = "SPI Word Access/Halfword Access Specification\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SPLW_A {
    #[doc = "0: SPDR_HA is valid to access in halfwords"]
    _0 = 0,
    #[doc = "1: SPDR is valid (to access in words)."]
    _1 = 1,
}
impl From<SPLW_A> for bool {
    #[inline(always)]
    fn from(variant: SPLW_A) -> Self {
        variant as u8 != 0
    }
}
impl SPLW_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPLW_A {
        match self.bits {
            false => SPLW_A::_0,
            true => SPLW_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SPLW_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SPLW_A::_1
    }
}
#[doc = "Field `SPLW` writer - SPI Word Access/Halfword Access Specification"]
pub type SPLW_W<'a, const O: u8> = crate::BitWriter<'a, u8, SPDCR_SPEC, SPLW_A, O>;
impl<'a, const O: u8> SPLW_W<'a, O> {
    #[doc = "SPDR_HA is valid to access in halfwords"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SPLW_A::_0)
    }
    #[doc = "SPDR is valid (to access in words)."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SPLW_A::_1)
    }
}
impl R {
    #[doc = "Bits 0:1 - Number of Frames Specification"]
    #[inline(always)]
    pub fn spfc(&self) -> SPFC_R {
        SPFC_R::new(self.bits & 3)
    }
    #[doc = "Bit 4 - RSPI Receive/Transmit Data Selection"]
    #[inline(always)]
    pub fn sprdtd(&self) -> SPRDTD_R {
        SPRDTD_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - SPI Word Access/Halfword Access Specification"]
    #[inline(always)]
    pub fn splw(&self) -> SPLW_R {
        SPLW_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Number of Frames Specification"]
    #[inline(always)]
    #[must_use]
    pub fn spfc(&mut self) -> SPFC_W<0> {
        SPFC_W::new(self)
    }
    #[doc = "Bit 4 - RSPI Receive/Transmit Data Selection"]
    #[inline(always)]
    #[must_use]
    pub fn sprdtd(&mut self) -> SPRDTD_W<4> {
        SPRDTD_W::new(self)
    }
    #[doc = "Bit 5 - SPI Word Access/Halfword Access Specification"]
    #[inline(always)]
    #[must_use]
    pub fn splw(&mut self) -> SPLW_W<5> {
        SPLW_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPI Data Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spdcr](index.html) module"]
pub struct SPDCR_SPEC;
impl crate::RegisterSpec for SPDCR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [spdcr::R](R) reader structure"]
impl crate::Readable for SPDCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [spdcr::W](W) writer structure"]
impl crate::Writable for SPDCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SPDCR to value 0"]
impl crate::Resettable for SPDCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
