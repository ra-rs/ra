#[doc = "Register `SDAMOD` reader"]
pub struct R(crate::R<SDAMOD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SDAMOD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SDAMOD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SDAMOD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SDAMOD` writer"]
pub struct W(crate::W<SDAMOD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SDAMOD_SPEC>;
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
impl From<crate::W<SDAMOD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SDAMOD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BE` reader - Continuous Access Enable"]
pub type BE_R = crate::BitReader<BE_A>;
#[doc = "Continuous Access Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BE_A {
    #[doc = "0: Disable"]
    _0 = 0,
    #[doc = "1: Enable."]
    _1 = 1,
}
impl From<BE_A> for bool {
    #[inline(always)]
    fn from(variant: BE_A) -> Self {
        variant as u8 != 0
    }
}
impl BE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BE_A {
        match self.bits {
            false => BE_A::_0,
            true => BE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BE_A::_1
    }
}
#[doc = "Field `BE` writer - Continuous Access Enable"]
pub type BE_W<'a, const O: u8> = crate::BitWriter<'a, u8, SDAMOD_SPEC, BE_A, O>;
impl<'a, const O: u8> BE_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BE_A::_0)
    }
    #[doc = "Enable."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BE_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Continuous Access Enable"]
    #[inline(always)]
    pub fn be(&self) -> BE_R {
        BE_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Continuous Access Enable"]
    #[inline(always)]
    #[must_use]
    pub fn be(&mut self) -> BE_W<0> {
        BE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SDRAM Access Mode Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sdamod](index.html) module"]
pub struct SDAMOD_SPEC;
impl crate::RegisterSpec for SDAMOD_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [sdamod::R](R) reader structure"]
impl crate::Readable for SDAMOD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sdamod::W](W) writer structure"]
impl crate::Writable for SDAMOD_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SDAMOD to value 0"]
impl crate::Resettable for SDAMOD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
