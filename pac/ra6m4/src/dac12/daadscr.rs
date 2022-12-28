#[doc = "Register `DAADSCR` reader"]
pub struct R(crate::R<DAADSCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DAADSCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DAADSCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DAADSCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DAADSCR` writer"]
pub struct W(crate::W<DAADSCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DAADSCR_SPEC>;
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
impl From<crate::W<DAADSCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DAADSCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DAADST` reader - D/A A/D Synchronous Conversion"]
pub type DAADST_R = crate::BitReader<DAADST_A>;
#[doc = "D/A A/D Synchronous Conversion\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DAADST_A {
    #[doc = "0: Do not synchronize DAC12 with ADC12 (unit 1) operation (disable interference reduction between D/A and A/D conversion)."]
    _0 = 0,
    #[doc = "1: Synchronize DAC12 with ADC12 (unit 1) operation (enable interference reduction between D/A and A/D conversion)."]
    _1 = 1,
}
impl From<DAADST_A> for bool {
    #[inline(always)]
    fn from(variant: DAADST_A) -> Self {
        variant as u8 != 0
    }
}
impl DAADST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DAADST_A {
        match self.bits {
            false => DAADST_A::_0,
            true => DAADST_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DAADST_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DAADST_A::_1
    }
}
#[doc = "Field `DAADST` writer - D/A A/D Synchronous Conversion"]
pub type DAADST_W<'a, const O: u8> = crate::BitWriter<'a, u8, DAADSCR_SPEC, DAADST_A, O>;
impl<'a, const O: u8> DAADST_W<'a, O> {
    #[doc = "Do not synchronize DAC12 with ADC12 (unit 1) operation (disable interference reduction between D/A and A/D conversion)."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DAADST_A::_0)
    }
    #[doc = "Synchronize DAC12 with ADC12 (unit 1) operation (enable interference reduction between D/A and A/D conversion)."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DAADST_A::_1)
    }
}
impl R {
    #[doc = "Bit 7 - D/A A/D Synchronous Conversion"]
    #[inline(always)]
    pub fn daadst(&self) -> DAADST_R {
        DAADST_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 7 - D/A A/D Synchronous Conversion"]
    #[inline(always)]
    #[must_use]
    pub fn daadst(&mut self) -> DAADST_W<7> {
        DAADST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "D/A A/D Synchronous Start Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [daadscr](index.html) module"]
pub struct DAADSCR_SPEC;
impl crate::RegisterSpec for DAADSCR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [daadscr::R](R) reader structure"]
impl crate::Readable for DAADSCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [daadscr::W](W) writer structure"]
impl crate::Writable for DAADSCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DAADSCR to value 0"]
impl crate::Resettable for DAADSCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
