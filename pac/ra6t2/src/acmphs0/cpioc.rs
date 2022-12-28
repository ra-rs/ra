#[doc = "Register `CPIOC` reader"]
pub struct R(crate::R<CPIOC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CPIOC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CPIOC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CPIOC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CPIOC` writer"]
pub struct W(crate::W<CPIOC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CPIOC_SPEC>;
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
impl From<crate::W<CPIOC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CPIOC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CPOE` reader - Comparator Output Selection"]
pub type CPOE_R = crate::BitReader<CPOE_A>;
#[doc = "Comparator Output Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CPOE_A {
    #[doc = "0: Disable CMPOUTn pin output of the comparator (output signal is low fixed)"]
    _0 = 0,
    #[doc = "1: Enable CMPOUTn pin output of the comparator"]
    _1 = 1,
}
impl From<CPOE_A> for bool {
    #[inline(always)]
    fn from(variant: CPOE_A) -> Self {
        variant as u8 != 0
    }
}
impl CPOE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CPOE_A {
        match self.bits {
            false => CPOE_A::_0,
            true => CPOE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CPOE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CPOE_A::_1
    }
}
#[doc = "Field `CPOE` writer - Comparator Output Selection"]
pub type CPOE_W<'a, const O: u8> = crate::BitWriter<'a, u8, CPIOC_SPEC, CPOE_A, O>;
impl<'a, const O: u8> CPOE_W<'a, O> {
    #[doc = "Disable CMPOUTn pin output of the comparator (output signal is low fixed)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CPOE_A::_0)
    }
    #[doc = "Enable CMPOUTn pin output of the comparator"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CPOE_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Comparator Output Selection"]
    #[inline(always)]
    pub fn cpoe(&self) -> CPOE_R {
        CPOE_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Comparator Output Selection"]
    #[inline(always)]
    #[must_use]
    pub fn cpoe(&mut self) -> CPOE_W<0> {
        CPOE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Comparator Output Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cpioc](index.html) module"]
pub struct CPIOC_SPEC;
impl crate::RegisterSpec for CPIOC_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [cpioc::R](R) reader structure"]
impl crate::Readable for CPIOC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cpioc::W](W) writer structure"]
impl crate::Writable for CPIOC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CPIOC to value 0"]
impl crate::Resettable for CPIOC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
