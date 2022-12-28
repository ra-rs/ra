#[doc = "Register `ADBUFEN` reader"]
pub struct R(crate::R<ADBUFEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADBUFEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADBUFEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADBUFEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADBUFEN` writer"]
pub struct W(crate::W<ADBUFEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADBUFEN_SPEC>;
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
impl From<crate::W<ADBUFEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADBUFEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BUFEN` reader - Data Buffer Enable"]
pub type BUFEN_R = crate::BitReader<BUFEN_A>;
#[doc = "Data Buffer Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BUFEN_A {
    #[doc = "0: The data buffer is not used."]
    _0 = 0,
    #[doc = "1: The data buffer is used."]
    _1 = 1,
}
impl From<BUFEN_A> for bool {
    #[inline(always)]
    fn from(variant: BUFEN_A) -> Self {
        variant as u8 != 0
    }
}
impl BUFEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BUFEN_A {
        match self.bits {
            false => BUFEN_A::_0,
            true => BUFEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BUFEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BUFEN_A::_1
    }
}
#[doc = "Field `BUFEN` writer - Data Buffer Enable"]
pub type BUFEN_W<'a, const O: u8> = crate::BitWriter<'a, u8, ADBUFEN_SPEC, BUFEN_A, O>;
impl<'a, const O: u8> BUFEN_W<'a, O> {
    #[doc = "The data buffer is not used."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BUFEN_A::_0)
    }
    #[doc = "The data buffer is used."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BUFEN_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Data Buffer Enable"]
    #[inline(always)]
    pub fn bufen(&self) -> BUFEN_R {
        BUFEN_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Data Buffer Enable"]
    #[inline(always)]
    #[must_use]
    pub fn bufen(&mut self) -> BUFEN_W<0> {
        BUFEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "A/D Data Buffer Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adbufen](index.html) module"]
pub struct ADBUFEN_SPEC;
impl crate::RegisterSpec for ADBUFEN_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [adbufen::R](R) reader structure"]
impl crate::Readable for ADBUFEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adbufen::W](W) writer structure"]
impl crate::Writable for ADBUFEN_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADBUFEN to value 0"]
impl crate::Resettable for ADBUFEN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
