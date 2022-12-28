#[doc = "Register `CACR0` reader"]
pub struct R(crate::R<CACR0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CACR0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CACR0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CACR0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CACR0` writer"]
pub struct W(crate::W<CACR0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CACR0_SPEC>;
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
impl From<crate::W<CACR0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CACR0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CFME` reader - Clock Frequency Measurement Enable"]
pub type CFME_R = crate::BitReader<CFME_A>;
#[doc = "Clock Frequency Measurement Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CFME_A {
    #[doc = "0: Disable"]
    _0 = 0,
    #[doc = "1: Enable"]
    _1 = 1,
}
impl From<CFME_A> for bool {
    #[inline(always)]
    fn from(variant: CFME_A) -> Self {
        variant as u8 != 0
    }
}
impl CFME_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CFME_A {
        match self.bits {
            false => CFME_A::_0,
            true => CFME_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CFME_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CFME_A::_1
    }
}
#[doc = "Field `CFME` writer - Clock Frequency Measurement Enable"]
pub type CFME_W<'a, const O: u8> = crate::BitWriter<'a, u8, CACR0_SPEC, CFME_A, O>;
impl<'a, const O: u8> CFME_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CFME_A::_0)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CFME_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Clock Frequency Measurement Enable"]
    #[inline(always)]
    pub fn cfme(&self) -> CFME_R {
        CFME_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Clock Frequency Measurement Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cfme(&mut self) -> CFME_W<0> {
        CFME_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CAC Control Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cacr0](index.html) module"]
pub struct CACR0_SPEC;
impl crate::RegisterSpec for CACR0_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [cacr0::R](R) reader structure"]
impl crate::Readable for CACR0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cacr0::W](W) writer structure"]
impl crate::Writable for CACR0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CACR0 to value 0"]
impl crate::Resettable for CACR0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
