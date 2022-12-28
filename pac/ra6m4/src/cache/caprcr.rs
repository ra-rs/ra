#[doc = "Register `CAPRCR` reader"]
pub struct R(crate::R<CAPRCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CAPRCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CAPRCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CAPRCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CAPRCR` writer"]
pub struct W(crate::W<CAPRCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CAPRCR_SPEC>;
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
impl From<crate::W<CAPRCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CAPRCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PRCR` reader - Register Write Control"]
pub type PRCR_R = crate::BitReader<PRCR_A>;
#[doc = "Register Write Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PRCR_A {
    #[doc = "0: Disable writes to protected registers"]
    _0 = 0,
    #[doc = "1: Enable writes to protected registers"]
    _1 = 1,
}
impl From<PRCR_A> for bool {
    #[inline(always)]
    fn from(variant: PRCR_A) -> Self {
        variant as u8 != 0
    }
}
impl PRCR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PRCR_A {
        match self.bits {
            false => PRCR_A::_0,
            true => PRCR_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PRCR_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PRCR_A::_1
    }
}
#[doc = "Field `PRCR` writer - Register Write Control"]
pub type PRCR_W<'a, const O: u8> = crate::BitWriter<'a, u32, CAPRCR_SPEC, PRCR_A, O>;
impl<'a, const O: u8> PRCR_W<'a, O> {
    #[doc = "Disable writes to protected registers"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PRCR_A::_0)
    }
    #[doc = "Enable writes to protected registers"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PRCR_A::_1)
    }
}
#[doc = "Field `KW` reader - Write key code"]
pub type KW_R = crate::FieldReader<u8, u8>;
#[doc = "Field `KW` writer - Write key code"]
pub type KW_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CAPRCR_SPEC, u8, u8, 7, O>;
impl R {
    #[doc = "Bit 0 - Register Write Control"]
    #[inline(always)]
    pub fn prcr(&self) -> PRCR_R {
        PRCR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:7 - Write key code"]
    #[inline(always)]
    pub fn kw(&self) -> KW_R {
        KW_R::new(((self.bits >> 1) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Register Write Control"]
    #[inline(always)]
    #[must_use]
    pub fn prcr(&mut self) -> PRCR_W<0> {
        PRCR_W::new(self)
    }
    #[doc = "Bits 1:7 - Write key code"]
    #[inline(always)]
    #[must_use]
    pub fn kw(&mut self) -> KW_W<1> {
        KW_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Cache Protection Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [caprcr](index.html) module"]
pub struct CAPRCR_SPEC;
impl crate::RegisterSpec for CAPRCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [caprcr::R](R) reader structure"]
impl crate::Readable for CAPRCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [caprcr::W](W) writer structure"]
impl crate::Writable for CAPRCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CAPRCR to value 0"]
impl crate::Resettable for CAPRCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
