#[doc = "Register `FMEPROT` reader"]
pub struct R(crate::R<FMEPROT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FMEPROT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FMEPROT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FMEPROT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FMEPROT` writer"]
pub struct W(crate::W<FMEPROT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FMEPROT_SPEC>;
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
impl From<crate::W<FMEPROT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FMEPROT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CEPROT` reader - Code Flash P/E Mode Entry Protection"]
pub type CEPROT_R = crate::BitReader<CEPROT_A>;
#[doc = "Code Flash P/E Mode Entry Protection\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CEPROT_A {
    #[doc = "0: FENTRYC bit is not protected"]
    _0 = 0,
    #[doc = "1: FENTRYC bit is protected."]
    _1 = 1,
}
impl From<CEPROT_A> for bool {
    #[inline(always)]
    fn from(variant: CEPROT_A) -> Self {
        variant as u8 != 0
    }
}
impl CEPROT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CEPROT_A {
        match self.bits {
            false => CEPROT_A::_0,
            true => CEPROT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CEPROT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CEPROT_A::_1
    }
}
#[doc = "Field `CEPROT` writer - Code Flash P/E Mode Entry Protection"]
pub type CEPROT_W<'a, const O: u8> = crate::BitWriter<'a, u16, FMEPROT_SPEC, CEPROT_A, O>;
impl<'a, const O: u8> CEPROT_W<'a, O> {
    #[doc = "FENTRYC bit is not protected"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CEPROT_A::_0)
    }
    #[doc = "FENTRYC bit is protected."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CEPROT_A::_1)
    }
}
#[doc = "Field `KEY` writer - Key Code"]
pub type KEY_W<'a, const O: u8> = crate::FieldWriter<'a, u16, FMEPROT_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bit 0 - Code Flash P/E Mode Entry Protection"]
    #[inline(always)]
    pub fn ceprot(&self) -> CEPROT_R {
        CEPROT_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Code Flash P/E Mode Entry Protection"]
    #[inline(always)]
    #[must_use]
    pub fn ceprot(&mut self) -> CEPROT_W<0> {
        CEPROT_W::new(self)
    }
    #[doc = "Bits 8:15 - Key Code"]
    #[inline(always)]
    #[must_use]
    pub fn key(&mut self) -> KEY_W<8> {
        KEY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Flash P/E Mode Entry Protection Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fmeprot](index.html) module"]
pub struct FMEPROT_SPEC;
impl crate::RegisterSpec for FMEPROT_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [fmeprot::R](R) reader structure"]
impl crate::Readable for FMEPROT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fmeprot::W](W) writer structure"]
impl crate::Writable for FMEPROT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FMEPROT to value 0x01"]
impl crate::Resettable for FMEPROT_SPEC {
    const RESET_VALUE: Self::Ux = 0x01;
}
