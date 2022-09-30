#[doc = "Register `ELCR` reader"]
pub struct R(crate::R<ELCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ELCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ELCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ELCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ELCR` writer"]
pub struct W(crate::W<ELCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ELCR_SPEC>;
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
impl From<crate::W<ELCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ELCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ELCON` reader - All Event Link Enable"]
pub type ELCON_R = crate::BitReader<ELCON_A>;
#[doc = "All Event Link Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ELCON_A {
    #[doc = "0: ELC function is disabled."]
    _0 = 0,
    #[doc = "1: ELC function is enabled."]
    _1 = 1,
}
impl From<ELCON_A> for bool {
    #[inline(always)]
    fn from(variant: ELCON_A) -> Self {
        variant as u8 != 0
    }
}
impl ELCON_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ELCON_A {
        match self.bits {
            false => ELCON_A::_0,
            true => ELCON_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ELCON_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ELCON_A::_1
    }
}
#[doc = "Field `ELCON` writer - All Event Link Enable"]
pub type ELCON_W<'a, const O: u8> = crate::BitWriter<'a, u8, ELCR_SPEC, ELCON_A, O>;
impl<'a, const O: u8> ELCON_W<'a, O> {
    #[doc = "ELC function is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ELCON_A::_0)
    }
    #[doc = "ELC function is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ELCON_A::_1)
    }
}
impl R {
    #[doc = "Bit 7 - All Event Link Enable"]
    #[inline(always)]
    pub fn elcon(&self) -> ELCON_R {
        ELCON_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 7 - All Event Link Enable"]
    #[inline(always)]
    pub fn elcon(&mut self) -> ELCON_W<7> {
        ELCON_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Event Link Controller Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [elcr](index.html) module"]
pub struct ELCR_SPEC;
impl crate::RegisterSpec for ELCR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [elcr::R](R) reader structure"]
impl crate::Readable for ELCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [elcr::W](W) writer structure"]
impl crate::Writable for ELCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ELCR to value 0"]
impl crate::Resettable for ELCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
