#[doc = "Register `BYPASS` reader"]
pub struct R(crate::R<BYPASS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BYPASS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BYPASS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BYPASS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BYPASS` writer"]
pub struct W(crate::W<BYPASS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BYPASS_SPEC>;
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
impl From<crate::W<BYPASS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BYPASS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BYPASS0` reader - Bypass 1588 module for Ether 0ch"]
pub type BYPASS0_R = crate::BitReader<BYPASS0_A>;
#[doc = "Bypass 1588 module for Ether 0ch\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BYPASS0_A {
    #[doc = "0: to use 1588 module for Ether 0ch"]
    _0 = 0,
    #[doc = "1: to bypass 1588 module for Ether 0ch"]
    _1 = 1,
}
impl From<BYPASS0_A> for bool {
    #[inline(always)]
    fn from(variant: BYPASS0_A) -> Self {
        variant as u8 != 0
    }
}
impl BYPASS0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BYPASS0_A {
        match self.bits {
            false => BYPASS0_A::_0,
            true => BYPASS0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BYPASS0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BYPASS0_A::_1
    }
}
#[doc = "Field `BYPASS0` writer - Bypass 1588 module for Ether 0ch"]
pub type BYPASS0_W<'a, const O: u8> = crate::BitWriter<'a, u32, BYPASS_SPEC, BYPASS0_A, O>;
impl<'a, const O: u8> BYPASS0_W<'a, O> {
    #[doc = "to use 1588 module for Ether 0ch"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BYPASS0_A::_0)
    }
    #[doc = "to bypass 1588 module for Ether 0ch"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BYPASS0_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Bypass 1588 module for Ether 0ch"]
    #[inline(always)]
    pub fn bypass0(&self) -> BYPASS0_R {
        BYPASS0_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Bypass 1588 module for Ether 0ch"]
    #[inline(always)]
    #[must_use]
    pub fn bypass0(&mut self) -> BYPASS0_W<0> {
        BYPASS0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Bypass 1588 module Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bypass](index.html) module"]
pub struct BYPASS_SPEC;
impl crate::RegisterSpec for BYPASS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bypass::R](R) reader structure"]
impl crate::Readable for BYPASS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bypass::W](W) writer structure"]
impl crate::Writable for BYPASS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BYPASS to value 0"]
impl crate::Resettable for BYPASS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
