#[doc = "Register `SOPCCR` reader"]
pub struct R(crate::R<SOPCCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SOPCCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SOPCCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SOPCCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SOPCCR` writer"]
pub struct W(crate::W<SOPCCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SOPCCR_SPEC>;
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
impl From<crate::W<SOPCCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SOPCCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SOPCM` reader - Sub Operating Power Control Mode Select"]
pub type SOPCM_R = crate::BitReader<SOPCM_A>;
#[doc = "Sub Operating Power Control Mode Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SOPCM_A {
    #[doc = "0: Other than Subosc-speed mode"]
    _0 = 0,
    #[doc = "1: Subosc-speed mode"]
    _1 = 1,
}
impl From<SOPCM_A> for bool {
    #[inline(always)]
    fn from(variant: SOPCM_A) -> Self {
        variant as u8 != 0
    }
}
impl SOPCM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SOPCM_A {
        match self.bits {
            false => SOPCM_A::_0,
            true => SOPCM_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SOPCM_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SOPCM_A::_1
    }
}
#[doc = "Field `SOPCM` writer - Sub Operating Power Control Mode Select"]
pub type SOPCM_W<'a, const O: u8> = crate::BitWriter<'a, u8, SOPCCR_SPEC, SOPCM_A, O>;
impl<'a, const O: u8> SOPCM_W<'a, O> {
    #[doc = "Other than Subosc-speed mode"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SOPCM_A::_0)
    }
    #[doc = "Subosc-speed mode"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SOPCM_A::_1)
    }
}
#[doc = "Field `SOPCMTSF` reader - Sub Operating Power Control Mode Transition Status Flag"]
pub type SOPCMTSF_R = crate::BitReader<SOPCMTSF_A>;
#[doc = "Sub Operating Power Control Mode Transition Status Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SOPCMTSF_A {
    #[doc = "0: Transition completed"]
    _0 = 0,
    #[doc = "1: During transition"]
    _1 = 1,
}
impl From<SOPCMTSF_A> for bool {
    #[inline(always)]
    fn from(variant: SOPCMTSF_A) -> Self {
        variant as u8 != 0
    }
}
impl SOPCMTSF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SOPCMTSF_A {
        match self.bits {
            false => SOPCMTSF_A::_0,
            true => SOPCMTSF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SOPCMTSF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SOPCMTSF_A::_1
    }
}
impl R {
    #[doc = "Bit 0 - Sub Operating Power Control Mode Select"]
    #[inline(always)]
    pub fn sopcm(&self) -> SOPCM_R {
        SOPCM_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - Sub Operating Power Control Mode Transition Status Flag"]
    #[inline(always)]
    pub fn sopcmtsf(&self) -> SOPCMTSF_R {
        SOPCMTSF_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Sub Operating Power Control Mode Select"]
    #[inline(always)]
    #[must_use]
    pub fn sopcm(&mut self) -> SOPCM_W<0> {
        SOPCM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Sub Operating Power Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sopccr](index.html) module"]
pub struct SOPCCR_SPEC;
impl crate::RegisterSpec for SOPCCR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [sopccr::R](R) reader structure"]
impl crate::Readable for SOPCCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sopccr::W](W) writer structure"]
impl crate::Writable for SOPCCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SOPCCR to value 0"]
impl crate::Resettable for SOPCCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
