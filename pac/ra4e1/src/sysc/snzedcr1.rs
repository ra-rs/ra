#[doc = "Register `SNZEDCR1` reader"]
pub struct R(crate::R<SNZEDCR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SNZEDCR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SNZEDCR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SNZEDCR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SNZEDCR1` writer"]
pub struct W(crate::W<SNZEDCR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SNZEDCR1_SPEC>;
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
impl From<crate::W<SNZEDCR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SNZEDCR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `AGT3UNFED` reader - AGT3 underflow Snooze End Enable"]
pub type AGT3UNFED_R = crate::BitReader<AGT3UNFED_A>;
#[doc = "AGT3 underflow Snooze End Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AGT3UNFED_A {
    #[doc = "0: Disable the Snooze End request"]
    _0 = 0,
    #[doc = "1: Enable the Snooze End request"]
    _1 = 1,
}
impl From<AGT3UNFED_A> for bool {
    #[inline(always)]
    fn from(variant: AGT3UNFED_A) -> Self {
        variant as u8 != 0
    }
}
impl AGT3UNFED_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AGT3UNFED_A {
        match self.bits {
            false => AGT3UNFED_A::_0,
            true => AGT3UNFED_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == AGT3UNFED_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == AGT3UNFED_A::_1
    }
}
#[doc = "Field `AGT3UNFED` writer - AGT3 underflow Snooze End Enable"]
pub type AGT3UNFED_W<'a, const O: u8> = crate::BitWriter<'a, u8, SNZEDCR1_SPEC, AGT3UNFED_A, O>;
impl<'a, const O: u8> AGT3UNFED_W<'a, O> {
    #[doc = "Disable the Snooze End request"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(AGT3UNFED_A::_0)
    }
    #[doc = "Enable the Snooze End request"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(AGT3UNFED_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - AGT3 underflow Snooze End Enable"]
    #[inline(always)]
    pub fn agt3unfed(&self) -> AGT3UNFED_R {
        AGT3UNFED_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - AGT3 underflow Snooze End Enable"]
    #[inline(always)]
    #[must_use]
    pub fn agt3unfed(&mut self) -> AGT3UNFED_W<0> {
        AGT3UNFED_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Snooze End Control Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [snzedcr1](index.html) module"]
pub struct SNZEDCR1_SPEC;
impl crate::RegisterSpec for SNZEDCR1_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [snzedcr1::R](R) reader structure"]
impl crate::Readable for SNZEDCR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [snzedcr1::W](W) writer structure"]
impl crate::Writable for SNZEDCR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SNZEDCR1 to value 0"]
impl crate::Resettable for SNZEDCR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
