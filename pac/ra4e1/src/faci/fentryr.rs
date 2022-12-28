#[doc = "Register `FENTRYR` reader"]
pub struct R(crate::R<FENTRYR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FENTRYR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FENTRYR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FENTRYR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FENTRYR` writer"]
pub struct W(crate::W<FENTRYR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FENTRYR_SPEC>;
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
impl From<crate::W<FENTRYR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FENTRYR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FENTRYC` reader - Code Flash P/E Mode Entry"]
pub type FENTRYC_R = crate::BitReader<FENTRYC_A>;
#[doc = "Code Flash P/E Mode Entry\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FENTRYC_A {
    #[doc = "0: Code flash is in read mode"]
    _0 = 0,
    #[doc = "1: Code flash is in P/E mode."]
    _1 = 1,
}
impl From<FENTRYC_A> for bool {
    #[inline(always)]
    fn from(variant: FENTRYC_A) -> Self {
        variant as u8 != 0
    }
}
impl FENTRYC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FENTRYC_A {
        match self.bits {
            false => FENTRYC_A::_0,
            true => FENTRYC_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FENTRYC_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FENTRYC_A::_1
    }
}
#[doc = "Field `FENTRYC` writer - Code Flash P/E Mode Entry"]
pub type FENTRYC_W<'a, const O: u8> = crate::BitWriter<'a, u16, FENTRYR_SPEC, FENTRYC_A, O>;
impl<'a, const O: u8> FENTRYC_W<'a, O> {
    #[doc = "Code flash is in read mode"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FENTRYC_A::_0)
    }
    #[doc = "Code flash is in P/E mode."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FENTRYC_A::_1)
    }
}
#[doc = "Field `FENTRYD` reader - Data Flash P/E Mode Entry"]
pub type FENTRYD_R = crate::BitReader<FENTRYD_A>;
#[doc = "Data Flash P/E Mode Entry\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FENTRYD_A {
    #[doc = "0: Data flash is in read mode"]
    _0 = 0,
    #[doc = "1: Data flash is in P/E mode."]
    _1 = 1,
}
impl From<FENTRYD_A> for bool {
    #[inline(always)]
    fn from(variant: FENTRYD_A) -> Self {
        variant as u8 != 0
    }
}
impl FENTRYD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FENTRYD_A {
        match self.bits {
            false => FENTRYD_A::_0,
            true => FENTRYD_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FENTRYD_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FENTRYD_A::_1
    }
}
#[doc = "Field `FENTRYD` writer - Data Flash P/E Mode Entry"]
pub type FENTRYD_W<'a, const O: u8> = crate::BitWriter<'a, u16, FENTRYR_SPEC, FENTRYD_A, O>;
impl<'a, const O: u8> FENTRYD_W<'a, O> {
    #[doc = "Data flash is in read mode"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FENTRYD_A::_0)
    }
    #[doc = "Data flash is in P/E mode."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FENTRYD_A::_1)
    }
}
#[doc = "Field `KEY` writer - Key Code"]
pub type KEY_W<'a, const O: u8> = crate::FieldWriter<'a, u16, FENTRYR_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bit 0 - Code Flash P/E Mode Entry"]
    #[inline(always)]
    pub fn fentryc(&self) -> FENTRYC_R {
        FENTRYC_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 7 - Data Flash P/E Mode Entry"]
    #[inline(always)]
    pub fn fentryd(&self) -> FENTRYD_R {
        FENTRYD_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Code Flash P/E Mode Entry"]
    #[inline(always)]
    #[must_use]
    pub fn fentryc(&mut self) -> FENTRYC_W<0> {
        FENTRYC_W::new(self)
    }
    #[doc = "Bit 7 - Data Flash P/E Mode Entry"]
    #[inline(always)]
    #[must_use]
    pub fn fentryd(&mut self) -> FENTRYD_W<7> {
        FENTRYD_W::new(self)
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
#[doc = "Flash P/E Mode Entry Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fentryr](index.html) module"]
pub struct FENTRYR_SPEC;
impl crate::RegisterSpec for FENTRYR_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [fentryr::R](R) reader structure"]
impl crate::Readable for FENTRYR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fentryr::W](W) writer structure"]
impl crate::Writable for FENTRYR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FENTRYR to value 0"]
impl crate::Resettable for FENTRYR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
