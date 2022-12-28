#[doc = "Register `PCCR1` reader"]
pub struct R(crate::R<PCCR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PCCR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PCCR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PCCR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PCCR1` writer"]
pub struct W(crate::W<PCCR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PCCR1_SPEC>;
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
impl From<crate::W<PCCR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PCCR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PCE` reader - PDC Operation Enable"]
pub type PCE_R = crate::BitReader<PCE_A>;
#[doc = "PDC Operation Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PCE_A {
    #[doc = "0: Operations for reception are disabled."]
    _0 = 0,
    #[doc = "1: Operations for reception are enabled."]
    _1 = 1,
}
impl From<PCE_A> for bool {
    #[inline(always)]
    fn from(variant: PCE_A) -> Self {
        variant as u8 != 0
    }
}
impl PCE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PCE_A {
        match self.bits {
            false => PCE_A::_0,
            true => PCE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PCE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PCE_A::_1
    }
}
#[doc = "Field `PCE` writer - PDC Operation Enable"]
pub type PCE_W<'a, const O: u8> = crate::BitWriter<'a, u32, PCCR1_SPEC, PCE_A, O>;
impl<'a, const O: u8> PCE_W<'a, O> {
    #[doc = "Operations for reception are disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PCE_A::_0)
    }
    #[doc = "Operations for reception are enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PCE_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - PDC Operation Enable"]
    #[inline(always)]
    pub fn pce(&self) -> PCE_R {
        PCE_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PDC Operation Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pce(&mut self) -> PCE_W<0> {
        PCE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PDC Control Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pccr1](index.html) module"]
pub struct PCCR1_SPEC;
impl crate::RegisterSpec for PCCR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pccr1::R](R) reader structure"]
impl crate::Readable for PCCR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pccr1::W](W) writer structure"]
impl crate::Writable for PCCR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PCCR1 to value 0"]
impl crate::Resettable for PCCR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
