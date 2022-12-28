#[doc = "Register `INSTE` reader"]
pub struct R(crate::R<INSTE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INSTE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INSTE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INSTE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INSTE` writer"]
pub struct W(crate::W<INSTE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INSTE_SPEC>;
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
impl From<crate::W<INSTE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INSTE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INEE` reader - Internal Error Enable"]
pub type INEE_R = crate::BitReader<INEE_A>;
#[doc = "Internal Error Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INEE_A {
    #[doc = "0: Disable INST.INEF"]
    _0 = 0,
    #[doc = "1: Enable INST.INEF"]
    _1 = 1,
}
impl From<INEE_A> for bool {
    #[inline(always)]
    fn from(variant: INEE_A) -> Self {
        variant as u8 != 0
    }
}
impl INEE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INEE_A {
        match self.bits {
            false => INEE_A::_0,
            true => INEE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == INEE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == INEE_A::_1
    }
}
#[doc = "Field `INEE` writer - Internal Error Enable"]
pub type INEE_W<'a, const O: u8> = crate::BitWriter<'a, u32, INSTE_SPEC, INEE_A, O>;
impl<'a, const O: u8> INEE_W<'a, O> {
    #[doc = "Disable INST.INEF"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(INEE_A::_0)
    }
    #[doc = "Enable INST.INEF"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(INEE_A::_1)
    }
}
impl R {
    #[doc = "Bit 10 - Internal Error Enable"]
    #[inline(always)]
    pub fn inee(&self) -> INEE_R {
        INEE_R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 10 - Internal Error Enable"]
    #[inline(always)]
    #[must_use]
    pub fn inee(&mut self) -> INEE_W<10> {
        INEE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Internal Status Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [inste](index.html) module"]
pub struct INSTE_SPEC;
impl crate::RegisterSpec for INSTE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [inste::R](R) reader structure"]
impl crate::Readable for INSTE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [inste::W](W) writer structure"]
impl crate::Writable for INSTE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INSTE to value 0"]
impl crate::Resettable for INSTE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
