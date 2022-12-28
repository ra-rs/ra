#[doc = "Register `BUSMCNT%s` reader"]
pub struct R(crate::R<BUSMCNT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BUSMCNT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BUSMCNT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BUSMCNT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BUSMCNT%s` writer"]
pub struct W(crate::W<BUSMCNT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BUSMCNT_SPEC>;
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
impl From<crate::W<BUSMCNT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BUSMCNT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IERES` reader - Ignore Error Responses"]
pub type IERES_R = crate::BitReader<IERES_A>;
#[doc = "Ignore Error Responses\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IERES_A {
    #[doc = "0: Bus error will be reported."]
    _0 = 0,
    #[doc = "1: Bus error will not be reported."]
    _1 = 1,
}
impl From<IERES_A> for bool {
    #[inline(always)]
    fn from(variant: IERES_A) -> Self {
        variant as u8 != 0
    }
}
impl IERES_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IERES_A {
        match self.bits {
            false => IERES_A::_0,
            true => IERES_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == IERES_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == IERES_A::_1
    }
}
#[doc = "Field `IERES` writer - Ignore Error Responses"]
pub type IERES_W<'a, const O: u8> = crate::BitWriter<'a, u16, BUSMCNT_SPEC, IERES_A, O>;
impl<'a, const O: u8> IERES_W<'a, O> {
    #[doc = "Bus error will be reported."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(IERES_A::_0)
    }
    #[doc = "Bus error will not be reported."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(IERES_A::_1)
    }
}
impl R {
    #[doc = "Bit 15 - Ignore Error Responses"]
    #[inline(always)]
    pub fn ieres(&self) -> IERES_R {
        IERES_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 15 - Ignore Error Responses"]
    #[inline(always)]
    #[must_use]
    pub fn ieres(&mut self) -> IERES_W<15> {
        IERES_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Master Bus Control Register %s\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [busmcnt](index.html) module"]
pub struct BUSMCNT_SPEC;
impl crate::RegisterSpec for BUSMCNT_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [busmcnt::R](R) reader structure"]
impl crate::Readable for BUSMCNT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [busmcnt::W](W) writer structure"]
impl crate::Writable for BUSMCNT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BUSMCNT%s to value 0"]
impl crate::Resettable for BUSMCNT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
