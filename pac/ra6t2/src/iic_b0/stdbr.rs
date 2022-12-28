#[doc = "Register `STDBR` reader"]
pub struct R(crate::R<STDBR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STDBR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STDBR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STDBR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `STDBR` writer"]
pub struct W(crate::W<STDBR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<STDBR_SPEC>;
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
impl From<crate::W<STDBR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<STDBR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SBRLO` reader - Count value of the Low-level period of SCL clock"]
pub type SBRLO_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SBRLO` writer - Count value of the Low-level period of SCL clock"]
pub type SBRLO_W<'a, const O: u8> = crate::FieldWriter<'a, u32, STDBR_SPEC, u8, u8, 8, O>;
#[doc = "Field `SBRHO` reader - Count value of the High-level period of SCL clock"]
pub type SBRHO_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SBRHO` writer - Count value of the High-level period of SCL clock"]
pub type SBRHO_W<'a, const O: u8> = crate::FieldWriter<'a, u32, STDBR_SPEC, u8, u8, 8, O>;
#[doc = "Field `DSBRPO` reader - Double the Standard Bit Rate Period for Open-Drain"]
pub type DSBRPO_R = crate::BitReader<DSBRPO_A>;
#[doc = "Double the Standard Bit Rate Period for Open-Drain\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DSBRPO_A {
    #[doc = "0: The time period set for SBRHO\\[7:0\\]
and SBRLO\\[7:0\\]
is not doubled."]
    _0 = 0,
    #[doc = "1: The time period set for SBRHO\\[7:0\\]
and SBRLO\\[7:0\\]
is doubled."]
    _1 = 1,
}
impl From<DSBRPO_A> for bool {
    #[inline(always)]
    fn from(variant: DSBRPO_A) -> Self {
        variant as u8 != 0
    }
}
impl DSBRPO_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DSBRPO_A {
        match self.bits {
            false => DSBRPO_A::_0,
            true => DSBRPO_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DSBRPO_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DSBRPO_A::_1
    }
}
#[doc = "Field `DSBRPO` writer - Double the Standard Bit Rate Period for Open-Drain"]
pub type DSBRPO_W<'a, const O: u8> = crate::BitWriter<'a, u32, STDBR_SPEC, DSBRPO_A, O>;
impl<'a, const O: u8> DSBRPO_W<'a, O> {
    #[doc = "The time period set for SBRHO\\[7:0\\]
and SBRLO\\[7:0\\]
is not doubled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DSBRPO_A::_0)
    }
    #[doc = "The time period set for SBRHO\\[7:0\\]
and SBRLO\\[7:0\\]
is doubled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DSBRPO_A::_1)
    }
}
impl R {
    #[doc = "Bits 0:7 - Count value of the Low-level period of SCL clock"]
    #[inline(always)]
    pub fn sbrlo(&self) -> SBRLO_R {
        SBRLO_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Count value of the High-level period of SCL clock"]
    #[inline(always)]
    pub fn sbrho(&self) -> SBRHO_R {
        SBRHO_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bit 31 - Double the Standard Bit Rate Period for Open-Drain"]
    #[inline(always)]
    pub fn dsbrpo(&self) -> DSBRPO_R {
        DSBRPO_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Count value of the Low-level period of SCL clock"]
    #[inline(always)]
    #[must_use]
    pub fn sbrlo(&mut self) -> SBRLO_W<0> {
        SBRLO_W::new(self)
    }
    #[doc = "Bits 8:15 - Count value of the High-level period of SCL clock"]
    #[inline(always)]
    #[must_use]
    pub fn sbrho(&mut self) -> SBRHO_W<8> {
        SBRHO_W::new(self)
    }
    #[doc = "Bit 31 - Double the Standard Bit Rate Period for Open-Drain"]
    #[inline(always)]
    #[must_use]
    pub fn dsbrpo(&mut self) -> DSBRPO_W<31> {
        DSBRPO_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Standard Bit Rate Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stdbr](index.html) module"]
pub struct STDBR_SPEC;
impl crate::RegisterSpec for STDBR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [stdbr::R](R) reader structure"]
impl crate::Readable for STDBR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [stdbr::W](W) writer structure"]
impl crate::Writable for STDBR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets STDBR to value 0xffff"]
impl crate::Resettable for STDBR_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff;
}
