#[doc = "Register `JCMOD` reader"]
pub struct R(crate::R<JCMOD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<JCMOD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<JCMOD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<JCMOD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `JCMOD` writer"]
pub struct W(crate::W<JCMOD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<JCMOD_SPEC>;
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
impl From<crate::W<JCMOD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<JCMOD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `REDU` reader - Pixel FormatNOTE: Read-only in Decompression."]
pub type REDU_R = crate::FieldReader<u8, REDU_A>;
#[doc = "Pixel FormatNOTE: Read-only in Decompression.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum REDU_A {
    #[doc = "1: YCbCr422(Compression) / YCbCr422(Decompression)"]
    _001 = 1,
    #[doc = "0: Setting prohibited(Compression) / YCbCr444(Decompression)"]
    _000 = 0,
    #[doc = "6: Setting prohibited(Compression) / YCbCr411/\\[Decompression\\]"]
    _110 = 6,
    #[doc = "2: Setting prohibited(Compression) / YCbCr420/\\[Decompression\\]"]
    _010 = 2,
}
impl From<REDU_A> for u8 {
    #[inline(always)]
    fn from(variant: REDU_A) -> Self {
        variant as _
    }
}
impl REDU_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<REDU_A> {
        match self.bits {
            1 => Some(REDU_A::_001),
            0 => Some(REDU_A::_000),
            6 => Some(REDU_A::_110),
            2 => Some(REDU_A::_010),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == REDU_A::_001
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == REDU_A::_000
    }
    #[doc = "Checks if the value of the field is `_110`"]
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        *self == REDU_A::_110
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == REDU_A::_010
    }
}
#[doc = "Field `REDU` writer - Pixel FormatNOTE: Read-only in Decompression."]
pub type REDU_W<'a, const O: u8> = crate::FieldWriter<'a, u8, JCMOD_SPEC, u8, REDU_A, 3, O>;
impl<'a, const O: u8> REDU_W<'a, O> {
    #[doc = "YCbCr422(Compression) / YCbCr422(Decompression)"]
    #[inline(always)]
    pub fn _001(self) -> &'a mut W {
        self.variant(REDU_A::_001)
    }
    #[doc = "Setting prohibited(Compression) / YCbCr444(Decompression)"]
    #[inline(always)]
    pub fn _000(self) -> &'a mut W {
        self.variant(REDU_A::_000)
    }
    #[doc = "Setting prohibited(Compression) / YCbCr411/\\[Decompression\\]"]
    #[inline(always)]
    pub fn _110(self) -> &'a mut W {
        self.variant(REDU_A::_110)
    }
    #[doc = "Setting prohibited(Compression) / YCbCr420/\\[Decompression\\]"]
    #[inline(always)]
    pub fn _010(self) -> &'a mut W {
        self.variant(REDU_A::_010)
    }
}
#[doc = "Field `DSP` reader - Compression/Decompression Set Note: When changing between processing for compression and for decompression, be sure to reset this module in advance by setting the JCUSRST bit in the software reset control register 2 (SWRSTCR2) of the power-downmodes."]
pub type DSP_R = crate::BitReader<DSP_A>;
#[doc = "Compression/Decompression Set Note: When changing between processing for compression and for decompression, be sure to reset this module in advance by setting the JCUSRST bit in the software reset control register 2 (SWRSTCR2) of the power-downmodes.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DSP_A {
    #[doc = "0: Compression process"]
    _0 = 0,
    #[doc = "1: Decompression process"]
    _1 = 1,
}
impl From<DSP_A> for bool {
    #[inline(always)]
    fn from(variant: DSP_A) -> Self {
        variant as u8 != 0
    }
}
impl DSP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DSP_A {
        match self.bits {
            false => DSP_A::_0,
            true => DSP_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DSP_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DSP_A::_1
    }
}
#[doc = "Field `DSP` writer - Compression/Decompression Set Note: When changing between processing for compression and for decompression, be sure to reset this module in advance by setting the JCUSRST bit in the software reset control register 2 (SWRSTCR2) of the power-downmodes."]
pub type DSP_W<'a, const O: u8> = crate::BitWriter<'a, u8, JCMOD_SPEC, DSP_A, O>;
impl<'a, const O: u8> DSP_W<'a, O> {
    #[doc = "Compression process"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DSP_A::_0)
    }
    #[doc = "Decompression process"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DSP_A::_1)
    }
}
impl R {
    #[doc = "Bits 0:2 - Pixel FormatNOTE: Read-only in Decompression."]
    #[inline(always)]
    pub fn redu(&self) -> REDU_R {
        REDU_R::new(self.bits & 7)
    }
    #[doc = "Bit 3 - Compression/Decompression Set Note: When changing between processing for compression and for decompression, be sure to reset this module in advance by setting the JCUSRST bit in the software reset control register 2 (SWRSTCR2) of the power-downmodes."]
    #[inline(always)]
    pub fn dsp(&self) -> DSP_R {
        DSP_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Pixel FormatNOTE: Read-only in Decompression."]
    #[inline(always)]
    #[must_use]
    pub fn redu(&mut self) -> REDU_W<0> {
        REDU_W::new(self)
    }
    #[doc = "Bit 3 - Compression/Decompression Set Note: When changing between processing for compression and for decompression, be sure to reset this module in advance by setting the JCUSRST bit in the software reset control register 2 (SWRSTCR2) of the power-downmodes."]
    #[inline(always)]
    #[must_use]
    pub fn dsp(&mut self) -> DSP_W<3> {
        DSP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "JPEG Code Mode Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [jcmod](index.html) module"]
pub struct JCMOD_SPEC;
impl crate::RegisterSpec for JCMOD_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [jcmod::R](R) reader structure"]
impl crate::Readable for JCMOD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [jcmod::W](W) writer structure"]
impl crate::Writable for JCMOD_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets JCMOD to value 0"]
impl crate::Resettable for JCMOD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
