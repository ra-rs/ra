#[doc = "Register `DTCST` reader"]
pub struct R(crate::R<DTCST_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DTCST_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DTCST_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DTCST_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DTCST` writer"]
pub struct W(crate::W<DTCST_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DTCST_SPEC>;
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
impl From<crate::W<DTCST_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DTCST_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DTCST` reader - DTC Module Start"]
pub type DTCST_R = crate::BitReader<DTCST_A>;
#[doc = "DTC Module Start\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DTCST_A {
    #[doc = "0: DTC module stopped."]
    _0 = 0,
    #[doc = "1: DTC module started."]
    _1 = 1,
}
impl From<DTCST_A> for bool {
    #[inline(always)]
    fn from(variant: DTCST_A) -> Self {
        variant as u8 != 0
    }
}
impl DTCST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DTCST_A {
        match self.bits {
            false => DTCST_A::_0,
            true => DTCST_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DTCST_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DTCST_A::_1
    }
}
#[doc = "Field `DTCST` writer - DTC Module Start"]
pub type DTCST_W<'a, const O: u8> = crate::BitWriter<'a, u8, DTCST_SPEC, DTCST_A, O>;
impl<'a, const O: u8> DTCST_W<'a, O> {
    #[doc = "DTC module stopped."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DTCST_A::_0)
    }
    #[doc = "DTC module started."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DTCST_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - DTC Module Start"]
    #[inline(always)]
    pub fn dtcst(&self) -> DTCST_R {
        DTCST_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DTC Module Start"]
    #[inline(always)]
    #[must_use]
    pub fn dtcst(&mut self) -> DTCST_W<0> {
        DTCST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DTC Module Start Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dtcst](index.html) module"]
pub struct DTCST_SPEC;
impl crate::RegisterSpec for DTCST_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [dtcst::R](R) reader structure"]
impl crate::Readable for DTCST_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dtcst::W](W) writer structure"]
impl crate::Writable for DTCST_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DTCST to value 0"]
impl crate::Resettable for DTCST_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
