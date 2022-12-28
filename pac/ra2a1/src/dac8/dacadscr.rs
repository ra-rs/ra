#[doc = "Register `DACADSCR` reader"]
pub struct R(crate::R<DACADSCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DACADSCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DACADSCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DACADSCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DACADSCR` writer"]
pub struct W(crate::W<DACADSCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DACADSCR_SPEC>;
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
impl From<crate::W<DACADSCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DACADSCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DACADST` reader - D/A A/D Synchronous Conversion"]
pub type DACADST_R = crate::BitReader<DACADST_A>;
#[doc = "D/A A/D Synchronous Conversion\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DACADST_A {
    #[doc = "0: Do not synchronize DAC8 with ADC16 operation (disable interference reduction between D/A and A/D conversion)"]
    _0 = 0,
    #[doc = "1: Synchronize DAC8 with ADC16 operation (enable interference reduction between D/A and A/D conversion)."]
    _1 = 1,
}
impl From<DACADST_A> for bool {
    #[inline(always)]
    fn from(variant: DACADST_A) -> Self {
        variant as u8 != 0
    }
}
impl DACADST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DACADST_A {
        match self.bits {
            false => DACADST_A::_0,
            true => DACADST_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DACADST_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DACADST_A::_1
    }
}
#[doc = "Field `DACADST` writer - D/A A/D Synchronous Conversion"]
pub type DACADST_W<'a, const O: u8> = crate::BitWriter<'a, u8, DACADSCR_SPEC, DACADST_A, O>;
impl<'a, const O: u8> DACADST_W<'a, O> {
    #[doc = "Do not synchronize DAC8 with ADC16 operation (disable interference reduction between D/A and A/D conversion)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DACADST_A::_0)
    }
    #[doc = "Synchronize DAC8 with ADC16 operation (enable interference reduction between D/A and A/D conversion)."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DACADST_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - D/A A/D Synchronous Conversion"]
    #[inline(always)]
    pub fn dacadst(&self) -> DACADST_R {
        DACADST_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - D/A A/D Synchronous Conversion"]
    #[inline(always)]
    #[must_use]
    pub fn dacadst(&mut self) -> DACADST_W<0> {
        DACADST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "D/A A/D Synchronous Start Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dacadscr](index.html) module"]
pub struct DACADSCR_SPEC;
impl crate::RegisterSpec for DACADSCR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [dacadscr::R](R) reader structure"]
impl crate::Readable for DACADSCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dacadscr::W](W) writer structure"]
impl crate::Writable for DACADSCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DACADSCR to value 0"]
impl crate::Resettable for DACADSCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
