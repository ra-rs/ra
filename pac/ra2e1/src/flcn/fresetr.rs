#[doc = "Register `FRESETR` reader"]
pub struct R(crate::R<FRESETR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FRESETR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FRESETR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FRESETR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FRESETR` writer"]
pub struct W(crate::W<FRESETR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FRESETR_SPEC>;
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
impl From<crate::W<FRESETR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FRESETR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FRESET` reader - Software reset of the registers"]
pub type FRESET_R = crate::BitReader<FRESET_A>;
#[doc = "Software reset of the registers\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FRESET_A {
    #[doc = "0: The registers related to the flash programming are not reset"]
    _0 = 0,
    #[doc = "1: The registers related to the flash programming are reset."]
    _1 = 1,
}
impl From<FRESET_A> for bool {
    #[inline(always)]
    fn from(variant: FRESET_A) -> Self {
        variant as u8 != 0
    }
}
impl FRESET_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FRESET_A {
        match self.bits {
            false => FRESET_A::_0,
            true => FRESET_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FRESET_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FRESET_A::_1
    }
}
#[doc = "Field `FRESET` writer - Software reset of the registers"]
pub type FRESET_W<'a, const O: u8> = crate::BitWriter<'a, u8, FRESETR_SPEC, FRESET_A, O>;
impl<'a, const O: u8> FRESET_W<'a, O> {
    #[doc = "The registers related to the flash programming are not reset"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FRESET_A::_0)
    }
    #[doc = "The registers related to the flash programming are reset."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FRESET_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Software reset of the registers"]
    #[inline(always)]
    pub fn freset(&self) -> FRESET_R {
        FRESET_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Software reset of the registers"]
    #[inline(always)]
    pub fn freset(&mut self) -> FRESET_W<0> {
        FRESET_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Flash Reset Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fresetr](index.html) module"]
pub struct FRESETR_SPEC;
impl crate::RegisterSpec for FRESETR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [fresetr::R](R) reader structure"]
impl crate::Readable for FRESETR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fresetr::W](W) writer structure"]
impl crate::Writable for FRESETR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FRESETR to value 0"]
impl crate::Resettable for FRESETR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
