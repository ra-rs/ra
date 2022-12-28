#[doc = "Register `ADCR` reader"]
pub struct R(crate::R<ADCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADCR` writer"]
pub struct W(crate::W<ADCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADCR_SPEC>;
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
impl From<crate::W<ADCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SDADCRD` reader - The 24-bit A/D conversion result"]
pub type SDADCRD_R = crate::FieldReader<u32, u32>;
#[doc = "Field `SDADCRS` reader - Status of an A/D conversion result"]
pub type SDADCRS_R = crate::BitReader<SDADCRS_A>;
#[doc = "Status of an A/D conversion result\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SDADCRS_A {
    #[doc = "0: Normal status (within the range)"]
    _0 = 0,
    #[doc = "1: Overflow occurred"]
    _1 = 1,
}
impl From<SDADCRS_A> for bool {
    #[inline(always)]
    fn from(variant: SDADCRS_A) -> Self {
        variant as u8 != 0
    }
}
impl SDADCRS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SDADCRS_A {
        match self.bits {
            false => SDADCRS_A::_0,
            true => SDADCRS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SDADCRS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SDADCRS_A::_1
    }
}
#[doc = "Field `SDADCRC` reader - Channel number for an A/D conversion result"]
pub type SDADCRC_R = crate::FieldReader<u8, SDADCRC_A>;
#[doc = "Channel number for an A/D conversion result\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SDADCRC_A {
    #[doc = "0: Reset value (Conversion result is invalid)"]
    _000 = 0,
    #[doc = "1: Input multiplexer 0 (ANSD0P / ANSD0N)"]
    _001 = 1,
    #[doc = "2: Input multiplexer 1 (ANSD1P / ANSD1N)"]
    _010 = 2,
    #[doc = "3: Input multiplexer 2 (ANSD2P / ANSD2N)"]
    _011 = 3,
    #[doc = "4: Input multiplexer 3 (ANSD3P / ANSD3N)"]
    _100 = 4,
    #[doc = "5: Input multiplexer 4 (AMP0O / AMP1O)"]
    _101 = 5,
}
impl From<SDADCRC_A> for u8 {
    #[inline(always)]
    fn from(variant: SDADCRC_A) -> Self {
        variant as _
    }
}
impl SDADCRC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SDADCRC_A> {
        match self.bits {
            0 => Some(SDADCRC_A::_000),
            1 => Some(SDADCRC_A::_001),
            2 => Some(SDADCRC_A::_010),
            3 => Some(SDADCRC_A::_011),
            4 => Some(SDADCRC_A::_100),
            5 => Some(SDADCRC_A::_101),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == SDADCRC_A::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == SDADCRC_A::_001
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == SDADCRC_A::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == SDADCRC_A::_011
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == SDADCRC_A::_100
    }
    #[doc = "Checks if the value of the field is `_101`"]
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == SDADCRC_A::_101
    }
}
impl R {
    #[doc = "Bits 0:23 - The 24-bit A/D conversion result"]
    #[inline(always)]
    pub fn sdadcrd(&self) -> SDADCRD_R {
        SDADCRD_R::new(self.bits & 0x00ff_ffff)
    }
    #[doc = "Bit 24 - Status of an A/D conversion result"]
    #[inline(always)]
    pub fn sdadcrs(&self) -> SDADCRS_R {
        SDADCRS_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bits 25:27 - Channel number for an A/D conversion result"]
    #[inline(always)]
    pub fn sdadcrc(&self) -> SDADCRC_R {
        SDADCRC_R::new(((self.bits >> 25) & 7) as u8)
    }
}
impl W {
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Sigma-delta A/D Converter Conversion Result Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adcr](index.html) module"]
pub struct ADCR_SPEC;
impl crate::RegisterSpec for ADCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adcr::R](R) reader structure"]
impl crate::Readable for ADCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adcr::W](W) writer structure"]
impl crate::Writable for ADCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADCR to value 0"]
impl crate::Resettable for ADCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
