#[doc = "Register `SPDECR` reader"]
pub struct R(crate::R<SPDECR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPDECR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SPDECR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SPDECR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SPDECR` writer"]
pub struct W(crate::W<SPDECR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SPDECR_SPEC>;
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
impl From<crate::W<SPDECR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SPDECR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SCKDL` reader - RSPCK Delay"]
pub type SCKDL_R = crate::FieldReader<u8, SCKDL_A>;
#[doc = "RSPCK Delay\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SCKDL_A {
    #[doc = "0: 1RSPCK"]
    _000 = 0,
    #[doc = "1: 2RSPCK"]
    _001 = 1,
    #[doc = "2: 3RSPCK"]
    _010 = 2,
    #[doc = "3: 4RSPCK"]
    _011 = 3,
    #[doc = "4: 5RSPCK"]
    _100 = 4,
    #[doc = "5: 6RSPCK"]
    _101 = 5,
    #[doc = "6: 7RSPCK"]
    _110 = 6,
    #[doc = "7: 8RSPCK"]
    _111 = 7,
}
impl From<SCKDL_A> for u8 {
    #[inline(always)]
    fn from(variant: SCKDL_A) -> Self {
        variant as _
    }
}
impl SCKDL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SCKDL_A {
        match self.bits {
            0 => SCKDL_A::_000,
            1 => SCKDL_A::_001,
            2 => SCKDL_A::_010,
            3 => SCKDL_A::_011,
            4 => SCKDL_A::_100,
            5 => SCKDL_A::_101,
            6 => SCKDL_A::_110,
            7 => SCKDL_A::_111,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == SCKDL_A::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == SCKDL_A::_001
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == SCKDL_A::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == SCKDL_A::_011
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == SCKDL_A::_100
    }
    #[doc = "Checks if the value of the field is `_101`"]
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == SCKDL_A::_101
    }
    #[doc = "Checks if the value of the field is `_110`"]
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        *self == SCKDL_A::_110
    }
    #[doc = "Checks if the value of the field is `_111`"]
    #[inline(always)]
    pub fn is_111(&self) -> bool {
        *self == SCKDL_A::_111
    }
}
#[doc = "Field `SCKDL` writer - RSPCK Delay"]
pub type SCKDL_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, SPDECR_SPEC, u8, SCKDL_A, 3, O>;
impl<'a, const O: u8> SCKDL_W<'a, O> {
    #[doc = "1RSPCK"]
    #[inline(always)]
    pub fn _000(self) -> &'a mut W {
        self.variant(SCKDL_A::_000)
    }
    #[doc = "2RSPCK"]
    #[inline(always)]
    pub fn _001(self) -> &'a mut W {
        self.variant(SCKDL_A::_001)
    }
    #[doc = "3RSPCK"]
    #[inline(always)]
    pub fn _010(self) -> &'a mut W {
        self.variant(SCKDL_A::_010)
    }
    #[doc = "4RSPCK"]
    #[inline(always)]
    pub fn _011(self) -> &'a mut W {
        self.variant(SCKDL_A::_011)
    }
    #[doc = "5RSPCK"]
    #[inline(always)]
    pub fn _100(self) -> &'a mut W {
        self.variant(SCKDL_A::_100)
    }
    #[doc = "6RSPCK"]
    #[inline(always)]
    pub fn _101(self) -> &'a mut W {
        self.variant(SCKDL_A::_101)
    }
    #[doc = "7RSPCK"]
    #[inline(always)]
    pub fn _110(self) -> &'a mut W {
        self.variant(SCKDL_A::_110)
    }
    #[doc = "8RSPCK"]
    #[inline(always)]
    pub fn _111(self) -> &'a mut W {
        self.variant(SCKDL_A::_111)
    }
}
#[doc = "Field `SLNDL` reader - SSL Negation Delay"]
pub type SLNDL_R = crate::FieldReader<u8, SLNDL_A>;
#[doc = "SSL Negation Delay\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SLNDL_A {
    #[doc = "0: 1RSPCK"]
    _000 = 0,
    #[doc = "1: 2RSPCK"]
    _001 = 1,
    #[doc = "2: 3RSPCK"]
    _010 = 2,
    #[doc = "3: 4RSPCK"]
    _011 = 3,
    #[doc = "4: 5RSPCK"]
    _100 = 4,
    #[doc = "5: 6RSPCK"]
    _101 = 5,
    #[doc = "6: 7RSPCK"]
    _110 = 6,
    #[doc = "7: 8RSPCK"]
    _111 = 7,
}
impl From<SLNDL_A> for u8 {
    #[inline(always)]
    fn from(variant: SLNDL_A) -> Self {
        variant as _
    }
}
impl SLNDL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SLNDL_A {
        match self.bits {
            0 => SLNDL_A::_000,
            1 => SLNDL_A::_001,
            2 => SLNDL_A::_010,
            3 => SLNDL_A::_011,
            4 => SLNDL_A::_100,
            5 => SLNDL_A::_101,
            6 => SLNDL_A::_110,
            7 => SLNDL_A::_111,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == SLNDL_A::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == SLNDL_A::_001
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == SLNDL_A::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == SLNDL_A::_011
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == SLNDL_A::_100
    }
    #[doc = "Checks if the value of the field is `_101`"]
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == SLNDL_A::_101
    }
    #[doc = "Checks if the value of the field is `_110`"]
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        *self == SLNDL_A::_110
    }
    #[doc = "Checks if the value of the field is `_111`"]
    #[inline(always)]
    pub fn is_111(&self) -> bool {
        *self == SLNDL_A::_111
    }
}
#[doc = "Field `SLNDL` writer - SSL Negation Delay"]
pub type SLNDL_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, SPDECR_SPEC, u8, SLNDL_A, 3, O>;
impl<'a, const O: u8> SLNDL_W<'a, O> {
    #[doc = "1RSPCK"]
    #[inline(always)]
    pub fn _000(self) -> &'a mut W {
        self.variant(SLNDL_A::_000)
    }
    #[doc = "2RSPCK"]
    #[inline(always)]
    pub fn _001(self) -> &'a mut W {
        self.variant(SLNDL_A::_001)
    }
    #[doc = "3RSPCK"]
    #[inline(always)]
    pub fn _010(self) -> &'a mut W {
        self.variant(SLNDL_A::_010)
    }
    #[doc = "4RSPCK"]
    #[inline(always)]
    pub fn _011(self) -> &'a mut W {
        self.variant(SLNDL_A::_011)
    }
    #[doc = "5RSPCK"]
    #[inline(always)]
    pub fn _100(self) -> &'a mut W {
        self.variant(SLNDL_A::_100)
    }
    #[doc = "6RSPCK"]
    #[inline(always)]
    pub fn _101(self) -> &'a mut W {
        self.variant(SLNDL_A::_101)
    }
    #[doc = "7RSPCK"]
    #[inline(always)]
    pub fn _110(self) -> &'a mut W {
        self.variant(SLNDL_A::_110)
    }
    #[doc = "8RSPCK"]
    #[inline(always)]
    pub fn _111(self) -> &'a mut W {
        self.variant(SLNDL_A::_111)
    }
}
#[doc = "Field `SPNDL` reader - SPI Next-Access Delay"]
pub type SPNDL_R = crate::FieldReader<u8, SPNDL_A>;
#[doc = "SPI Next-Access Delay\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SPNDL_A {
    #[doc = "0: 1RSPCK + 5TCLK"]
    _000 = 0,
    #[doc = "1: 2RSPCK + 5TCLK"]
    _001 = 1,
    #[doc = "2: 3RSPCK + 5TCLK"]
    _010 = 2,
    #[doc = "3: 4RSPCK + 5TCLK"]
    _011 = 3,
    #[doc = "4: 5RSPCK + 5TCLK"]
    _100 = 4,
    #[doc = "5: 6RSPCK + 5TCLK"]
    _101 = 5,
    #[doc = "6: 7RSPCK + 5TCLK"]
    _110 = 6,
    #[doc = "7: 8RSPCK + 5TCLK"]
    _111 = 7,
}
impl From<SPNDL_A> for u8 {
    #[inline(always)]
    fn from(variant: SPNDL_A) -> Self {
        variant as _
    }
}
impl SPNDL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPNDL_A {
        match self.bits {
            0 => SPNDL_A::_000,
            1 => SPNDL_A::_001,
            2 => SPNDL_A::_010,
            3 => SPNDL_A::_011,
            4 => SPNDL_A::_100,
            5 => SPNDL_A::_101,
            6 => SPNDL_A::_110,
            7 => SPNDL_A::_111,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == SPNDL_A::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == SPNDL_A::_001
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == SPNDL_A::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == SPNDL_A::_011
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == SPNDL_A::_100
    }
    #[doc = "Checks if the value of the field is `_101`"]
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == SPNDL_A::_101
    }
    #[doc = "Checks if the value of the field is `_110`"]
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        *self == SPNDL_A::_110
    }
    #[doc = "Checks if the value of the field is `_111`"]
    #[inline(always)]
    pub fn is_111(&self) -> bool {
        *self == SPNDL_A::_111
    }
}
#[doc = "Field `SPNDL` writer - SPI Next-Access Delay"]
pub type SPNDL_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, SPDECR_SPEC, u8, SPNDL_A, 3, O>;
impl<'a, const O: u8> SPNDL_W<'a, O> {
    #[doc = "1RSPCK + 5TCLK"]
    #[inline(always)]
    pub fn _000(self) -> &'a mut W {
        self.variant(SPNDL_A::_000)
    }
    #[doc = "2RSPCK + 5TCLK"]
    #[inline(always)]
    pub fn _001(self) -> &'a mut W {
        self.variant(SPNDL_A::_001)
    }
    #[doc = "3RSPCK + 5TCLK"]
    #[inline(always)]
    pub fn _010(self) -> &'a mut W {
        self.variant(SPNDL_A::_010)
    }
    #[doc = "4RSPCK + 5TCLK"]
    #[inline(always)]
    pub fn _011(self) -> &'a mut W {
        self.variant(SPNDL_A::_011)
    }
    #[doc = "5RSPCK + 5TCLK"]
    #[inline(always)]
    pub fn _100(self) -> &'a mut W {
        self.variant(SPNDL_A::_100)
    }
    #[doc = "6RSPCK + 5TCLK"]
    #[inline(always)]
    pub fn _101(self) -> &'a mut W {
        self.variant(SPNDL_A::_101)
    }
    #[doc = "7RSPCK + 5TCLK"]
    #[inline(always)]
    pub fn _110(self) -> &'a mut W {
        self.variant(SPNDL_A::_110)
    }
    #[doc = "8RSPCK + 5TCLK"]
    #[inline(always)]
    pub fn _111(self) -> &'a mut W {
        self.variant(SPNDL_A::_111)
    }
}
impl R {
    #[doc = "Bits 0:2 - RSPCK Delay"]
    #[inline(always)]
    pub fn sckdl(&self) -> SCKDL_R {
        SCKDL_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 8:10 - SSL Negation Delay"]
    #[inline(always)]
    pub fn slndl(&self) -> SLNDL_R {
        SLNDL_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 16:18 - SPI Next-Access Delay"]
    #[inline(always)]
    pub fn spndl(&self) -> SPNDL_R {
        SPNDL_R::new(((self.bits >> 16) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - RSPCK Delay"]
    #[inline(always)]
    #[must_use]
    pub fn sckdl(&mut self) -> SCKDL_W<0> {
        SCKDL_W::new(self)
    }
    #[doc = "Bits 8:10 - SSL Negation Delay"]
    #[inline(always)]
    #[must_use]
    pub fn slndl(&mut self) -> SLNDL_W<8> {
        SLNDL_W::new(self)
    }
    #[doc = "Bits 16:18 - SPI Next-Access Delay"]
    #[inline(always)]
    #[must_use]
    pub fn spndl(&mut self) -> SPNDL_W<16> {
        SPNDL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPI Delay Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spdecr](index.html) module"]
pub struct SPDECR_SPEC;
impl crate::RegisterSpec for SPDECR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [spdecr::R](R) reader structure"]
impl crate::Readable for SPDECR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [spdecr::W](W) writer structure"]
impl crate::Writable for SPDECR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SPDECR to value 0"]
impl crate::Resettable for SPDECR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
