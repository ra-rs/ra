#[doc = "Register `CFDGAFLIGNCTR` reader"]
pub struct R(crate::R<CFDGAFLIGNCTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFDGAFLIGNCTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFDGAFLIGNCTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFDGAFLIGNCTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CFDGAFLIGNCTR` writer"]
pub struct W(crate::W<CFDGAFLIGNCTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFDGAFLIGNCTR_SPEC>;
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
impl From<crate::W<CFDGAFLIGNCTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFDGAFLIGNCTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IREN` reader - Ignore Rule Enable"]
pub type IREN_R = crate::BitReader<IREN_A>;
#[doc = "Ignore Rule Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IREN_A {
    #[doc = "0: AFL entry number is not ignored"]
    _0 = 0,
    #[doc = "1: AFL entry number is ignored"]
    _1 = 1,
}
impl From<IREN_A> for bool {
    #[inline(always)]
    fn from(variant: IREN_A) -> Self {
        variant as u8 != 0
    }
}
impl IREN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IREN_A {
        match self.bits {
            false => IREN_A::_0,
            true => IREN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == IREN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == IREN_A::_1
    }
}
#[doc = "Field `IREN` writer - Ignore Rule Enable"]
pub type IREN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFDGAFLIGNCTR_SPEC, IREN_A, O>;
impl<'a, const O: u8> IREN_W<'a, O> {
    #[doc = "AFL entry number is not ignored"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(IREN_A::_0)
    }
    #[doc = "AFL entry number is ignored"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(IREN_A::_1)
    }
}
#[doc = "Field `KEY` writer - Key Code"]
pub type KEY_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFDGAFLIGNCTR_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bit 0 - Ignore Rule Enable"]
    #[inline(always)]
    pub fn iren(&self) -> IREN_R {
        IREN_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Ignore Rule Enable"]
    #[inline(always)]
    #[must_use]
    pub fn iren(&mut self) -> IREN_W<0> {
        IREN_W::new(self)
    }
    #[doc = "Bits 8:15 - Key Code"]
    #[inline(always)]
    #[must_use]
    pub fn key(&mut self) -> KEY_W<8> {
        KEY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Global AFL Ignore Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfdgaflignctr](index.html) module"]
pub struct CFDGAFLIGNCTR_SPEC;
impl crate::RegisterSpec for CFDGAFLIGNCTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cfdgaflignctr::R](R) reader structure"]
impl crate::Readable for CFDGAFLIGNCTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cfdgaflignctr::W](W) writer structure"]
impl crate::Writable for CFDGAFLIGNCTR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CFDGAFLIGNCTR to value 0"]
impl crate::Resettable for CFDGAFLIGNCTR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
