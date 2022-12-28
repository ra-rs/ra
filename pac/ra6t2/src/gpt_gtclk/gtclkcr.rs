#[doc = "Register `GTCLKCR` reader"]
pub struct R(crate::R<GTCLKCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GTCLKCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GTCLKCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GTCLKCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GTCLKCR` writer"]
pub struct W(crate::W<GTCLKCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GTCLKCR_SPEC>;
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
impl From<crate::W<GTCLKCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GTCLKCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BPEN` reader - Synchronization Circuit Bypass Enable"]
pub type BPEN_R = crate::BitReader<BPEN_A>;
#[doc = "Synchronization Circuit Bypass Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BPEN_A {
    #[doc = "0: In case of using Bus Clock and GPT Core Clock asynchronously"]
    _0 = 0,
    #[doc = "1: In case of using Bus Clock and GPT Core Clock synchronously"]
    _1 = 1,
}
impl From<BPEN_A> for bool {
    #[inline(always)]
    fn from(variant: BPEN_A) -> Self {
        variant as u8 != 0
    }
}
impl BPEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BPEN_A {
        match self.bits {
            false => BPEN_A::_0,
            true => BPEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BPEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BPEN_A::_1
    }
}
#[doc = "Field `BPEN` writer - Synchronization Circuit Bypass Enable"]
pub type BPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTCLKCR_SPEC, BPEN_A, O>;
impl<'a, const O: u8> BPEN_W<'a, O> {
    #[doc = "In case of using Bus Clock and GPT Core Clock asynchronously"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BPEN_A::_0)
    }
    #[doc = "In case of using Bus Clock and GPT Core Clock synchronously"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BPEN_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Synchronization Circuit Bypass Enable"]
    #[inline(always)]
    pub fn bpen(&self) -> BPEN_R {
        BPEN_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Synchronization Circuit Bypass Enable"]
    #[inline(always)]
    #[must_use]
    pub fn bpen(&mut self) -> BPEN_W<0> {
        BPEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "General PWM Timer Clock Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gtclkcr](index.html) module"]
pub struct GTCLKCR_SPEC;
impl crate::RegisterSpec for GTCLKCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gtclkcr::R](R) reader structure"]
impl crate::Readable for GTCLKCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gtclkcr::W](W) writer structure"]
impl crate::Writable for GTCLKCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GTCLKCR to value 0"]
impl crate::Resettable for GTCLKCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
