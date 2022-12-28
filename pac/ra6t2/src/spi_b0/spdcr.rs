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
#[doc = "Field `BYSW` reader - Byte Swap Operating Mode Select"]
pub type BYSW_R = crate::BitReader<BYSW_A>;
#[doc = "Byte Swap Operating Mode Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BYSW_A {
    #[doc = "0: Byte Swap OFF"]
    _0 = 0,
    #[doc = "1: Byte Swap ON"]
    _1 = 1,
}
impl From<BYSW_A> for bool {
    #[inline(always)]
    fn from(variant: BYSW_A) -> Self {
        variant as u8 != 0
    }
}
impl BYSW_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BYSW_A {
        match self.bits {
            false => BYSW_A::_0,
            true => BYSW_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BYSW_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BYSW_A::_1
    }
}
#[doc = "Field `BYSW` writer - Byte Swap Operating Mode Select"]
pub type BYSW_W<'a, const O: u8> = crate::BitWriter<'a, u32, SPDCR_SPEC, BYSW_A, O>;
impl<'a, const O: u8> BYSW_W<'a, O> {
    #[doc = "Byte Swap OFF"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BYSW_A::_0)
    }
    #[doc = "Byte Swap ON"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BYSW_A::_1)
    }
}
#[doc = "Field `SPRDTD` reader - SPI Receive Data or Transmit Data Select"]
pub type SPRDTD_R = crate::BitReader<SPRDTD_A>;
#[doc = "SPI Receive Data or Transmit Data Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SPRDTD_A {
    #[doc = "0: The SPDR reads the receive buffer."]
    _0 = 0,
    #[doc = "1: The SPDR reads the transmit buffer"]
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
#[doc = "Field `SPRDTD` writer - SPI Receive Data or Transmit Data Select"]
pub type SPRDTD_W<'a, const O: u8> = crate::BitWriter<'a, u32, SPDCR_SPEC, SPRDTD_A, O>;
impl<'a, const O: u8> SPRDTD_W<'a, O> {
    #[doc = "The SPDR reads the receive buffer."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SPRDTD_A::_0)
    }
    #[doc = "The SPDR reads the transmit buffer"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SPRDTD_A::_1)
    }
}
#[doc = "Field `SINV` reader - Serial data invert bit"]
pub type SINV_R = crate::BitReader<SINV_A>;
#[doc = "Serial data invert bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SINV_A {
    #[doc = "0: Not invert serial data"]
    _0 = 0,
    #[doc = "1: Invert serial data."]
    _1 = 1,
}
impl From<SINV_A> for bool {
    #[inline(always)]
    fn from(variant: SINV_A) -> Self {
        variant as u8 != 0
    }
}
impl SINV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SINV_A {
        match self.bits {
            false => SINV_A::_0,
            true => SINV_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SINV_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SINV_A::_1
    }
}
#[doc = "Field `SINV` writer - Serial data invert bit"]
pub type SINV_W<'a, const O: u8> = crate::BitWriter<'a, u32, SPDCR_SPEC, SINV_A, O>;
impl<'a, const O: u8> SINV_W<'a, O> {
    #[doc = "Not invert serial data"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SINV_A::_0)
    }
    #[doc = "Invert serial data."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SINV_A::_1)
    }
}
#[doc = "Field `SPFC` reader - Frame Count"]
pub type SPFC_R = crate::FieldReader<u8, SPFC_A>;
#[doc = "Frame Count\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SPFC_A {
    #[doc = "0: 1 frame"]
    _00 = 0,
    #[doc = "1: 2 frame"]
    _01 = 1,
    #[doc = "2: 3 frame"]
    _10 = 2,
    #[doc = "3: 4 frame"]
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
#[doc = "Field `SPFC` writer - Frame Count"]
pub type SPFC_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, SPDCR_SPEC, u8, SPFC_A, 2, O>;
impl<'a, const O: u8> SPFC_W<'a, O> {
    #[doc = "1 frame"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(SPFC_A::_00)
    }
    #[doc = "2 frame"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(SPFC_A::_01)
    }
    #[doc = "3 frame"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(SPFC_A::_10)
    }
    #[doc = "4 frame"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(SPFC_A::_11)
    }
}
impl R {
    #[doc = "Bit 0 - Byte Swap Operating Mode Select"]
    #[inline(always)]
    pub fn bysw(&self) -> BYSW_R {
        BYSW_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 3 - SPI Receive Data or Transmit Data Select"]
    #[inline(always)]
    pub fn sprdtd(&self) -> SPRDTD_R {
        SPRDTD_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Serial data invert bit"]
    #[inline(always)]
    pub fn sinv(&self) -> SINV_R {
        SINV_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 8:9 - Frame Count"]
    #[inline(always)]
    pub fn spfc(&self) -> SPFC_R {
        SPFC_R::new(((self.bits >> 8) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Byte Swap Operating Mode Select"]
    #[inline(always)]
    #[must_use]
    pub fn bysw(&mut self) -> BYSW_W<0> {
        BYSW_W::new(self)
    }
    #[doc = "Bit 3 - SPI Receive Data or Transmit Data Select"]
    #[inline(always)]
    #[must_use]
    pub fn sprdtd(&mut self) -> SPRDTD_W<3> {
        SPRDTD_W::new(self)
    }
    #[doc = "Bit 4 - Serial data invert bit"]
    #[inline(always)]
    #[must_use]
    pub fn sinv(&mut self) -> SINV_W<4> {
        SINV_W::new(self)
    }
    #[doc = "Bits 8:9 - Frame Count"]
    #[inline(always)]
    #[must_use]
    pub fn spfc(&mut self) -> SPFC_W<8> {
        SPFC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPI Data Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spdcr](index.html) module"]
pub struct SPDCR_SPEC;
impl crate::RegisterSpec for SPDCR_SPEC {
    type Ux = u32;
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
