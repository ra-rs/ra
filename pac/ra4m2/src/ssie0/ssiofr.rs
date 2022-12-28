#[doc = "Register `SSIOFR` reader"]
pub struct R(crate::R<SSIOFR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SSIOFR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SSIOFR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SSIOFR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SSIOFR` writer"]
pub struct W(crate::W<SSIOFR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SSIOFR_SPEC>;
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
impl From<crate::W<SSIOFR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SSIOFR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OMOD` reader - Audio Format Select"]
pub type OMOD_R = crate::FieldReader<u8, OMOD_A>;
#[doc = "Audio Format Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum OMOD_A {
    #[doc = "0: I2S format"]
    _00 = 0,
    #[doc = "1: TDM format"]
    _01 = 1,
    #[doc = "2: Monaural format"]
    _10 = 2,
    #[doc = "3: Setting prohibited"]
    _11 = 3,
}
impl From<OMOD_A> for u8 {
    #[inline(always)]
    fn from(variant: OMOD_A) -> Self {
        variant as _
    }
}
impl OMOD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OMOD_A {
        match self.bits {
            0 => OMOD_A::_00,
            1 => OMOD_A::_01,
            2 => OMOD_A::_10,
            3 => OMOD_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == OMOD_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == OMOD_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == OMOD_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == OMOD_A::_11
    }
}
#[doc = "Field `OMOD` writer - Audio Format Select"]
pub type OMOD_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, SSIOFR_SPEC, u8, OMOD_A, 2, O>;
impl<'a, const O: u8> OMOD_W<'a, O> {
    #[doc = "I2S format"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(OMOD_A::_00)
    }
    #[doc = "TDM format"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(OMOD_A::_01)
    }
    #[doc = "Monaural format"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(OMOD_A::_10)
    }
    #[doc = "Setting prohibited"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(OMOD_A::_11)
    }
}
#[doc = "Field `LRCONT` reader - Whether to Enable LRCK/FS Continuation"]
pub type LRCONT_R = crate::BitReader<LRCONT_A>;
#[doc = "Whether to Enable LRCK/FS Continuation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LRCONT_A {
    #[doc = "0: Disables LRCK/FS continuation"]
    _0 = 0,
    #[doc = "1: Enables LRCK/FS continuation"]
    _1 = 1,
}
impl From<LRCONT_A> for bool {
    #[inline(always)]
    fn from(variant: LRCONT_A) -> Self {
        variant as u8 != 0
    }
}
impl LRCONT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LRCONT_A {
        match self.bits {
            false => LRCONT_A::_0,
            true => LRCONT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == LRCONT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == LRCONT_A::_1
    }
}
#[doc = "Field `LRCONT` writer - Whether to Enable LRCK/FS Continuation"]
pub type LRCONT_W<'a, const O: u8> = crate::BitWriter<'a, u32, SSIOFR_SPEC, LRCONT_A, O>;
impl<'a, const O: u8> LRCONT_W<'a, O> {
    #[doc = "Disables LRCK/FS continuation"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(LRCONT_A::_0)
    }
    #[doc = "Enables LRCK/FS continuation"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(LRCONT_A::_1)
    }
}
#[doc = "Field `BCKASTP` reader - Whether to Enable Stopping BCK Output When SSIE is in Idle Status"]
pub type BCKASTP_R = crate::BitReader<BCKASTP_A>;
#[doc = "Whether to Enable Stopping BCK Output When SSIE is in Idle Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BCKASTP_A {
    #[doc = "0: Always outputs BCK to the SSIBCK pin"]
    _0 = 0,
    #[doc = "1: Automatically controls output of BCK to the SSIBCK pin"]
    _1 = 1,
}
impl From<BCKASTP_A> for bool {
    #[inline(always)]
    fn from(variant: BCKASTP_A) -> Self {
        variant as u8 != 0
    }
}
impl BCKASTP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BCKASTP_A {
        match self.bits {
            false => BCKASTP_A::_0,
            true => BCKASTP_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BCKASTP_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BCKASTP_A::_1
    }
}
#[doc = "Field `BCKASTP` writer - Whether to Enable Stopping BCK Output When SSIE is in Idle Status"]
pub type BCKASTP_W<'a, const O: u8> = crate::BitWriter<'a, u32, SSIOFR_SPEC, BCKASTP_A, O>;
impl<'a, const O: u8> BCKASTP_W<'a, O> {
    #[doc = "Always outputs BCK to the SSIBCK pin"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BCKASTP_A::_0)
    }
    #[doc = "Automatically controls output of BCK to the SSIBCK pin"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BCKASTP_A::_1)
    }
}
impl R {
    #[doc = "Bits 0:1 - Audio Format Select"]
    #[inline(always)]
    pub fn omod(&self) -> OMOD_R {
        OMOD_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 8 - Whether to Enable LRCK/FS Continuation"]
    #[inline(always)]
    pub fn lrcont(&self) -> LRCONT_R {
        LRCONT_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Whether to Enable Stopping BCK Output When SSIE is in Idle Status"]
    #[inline(always)]
    pub fn bckastp(&self) -> BCKASTP_R {
        BCKASTP_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Audio Format Select"]
    #[inline(always)]
    #[must_use]
    pub fn omod(&mut self) -> OMOD_W<0> {
        OMOD_W::new(self)
    }
    #[doc = "Bit 8 - Whether to Enable LRCK/FS Continuation"]
    #[inline(always)]
    #[must_use]
    pub fn lrcont(&mut self) -> LRCONT_W<8> {
        LRCONT_W::new(self)
    }
    #[doc = "Bit 9 - Whether to Enable Stopping BCK Output When SSIE is in Idle Status"]
    #[inline(always)]
    #[must_use]
    pub fn bckastp(&mut self) -> BCKASTP_W<9> {
        BCKASTP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Audio Format Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ssiofr](index.html) module"]
pub struct SSIOFR_SPEC;
impl crate::RegisterSpec for SSIOFR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ssiofr::R](R) reader structure"]
impl crate::Readable for SSIOFR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ssiofr::W](W) writer structure"]
impl crate::Writable for SSIOFR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SSIOFR to value 0"]
impl crate::Resettable for SSIOFR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
