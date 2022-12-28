#[doc = "Register `MMPUSARB` reader"]
pub struct R(crate::R<MMPUSARB_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MMPUSARB_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MMPUSARB_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MMPUSARB_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MMPUSARB` writer"]
pub struct W(crate::W<MMPUSARB_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MMPUSARB_SPEC>;
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
impl From<crate::W<MMPUSARB_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MMPUSARB_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MMPUBSA0` reader - MMPUB Security Attribution"]
pub type MMPUBSA0_R = crate::BitReader<MMPUBSA0_A>;
#[doc = "MMPUB Security Attribution\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MMPUBSA0_A {
    #[doc = "0: Secure"]
    _0 = 0,
    #[doc = "1: Non-Secure"]
    _1 = 1,
}
impl From<MMPUBSA0_A> for bool {
    #[inline(always)]
    fn from(variant: MMPUBSA0_A) -> Self {
        variant as u8 != 0
    }
}
impl MMPUBSA0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MMPUBSA0_A {
        match self.bits {
            false => MMPUBSA0_A::_0,
            true => MMPUBSA0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MMPUBSA0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MMPUBSA0_A::_1
    }
}
#[doc = "Field `MMPUBSA0` writer - MMPUB Security Attribution"]
pub type MMPUBSA0_W<'a, const O: u8> = crate::BitWriter<'a, u32, MMPUSARB_SPEC, MMPUBSA0_A, O>;
impl<'a, const O: u8> MMPUBSA0_W<'a, O> {
    #[doc = "Secure"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MMPUBSA0_A::_0)
    }
    #[doc = "Non-Secure"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MMPUBSA0_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - MMPUB Security Attribution"]
    #[inline(always)]
    pub fn mmpubsa0(&self) -> MMPUBSA0_R {
        MMPUBSA0_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - MMPUB Security Attribution"]
    #[inline(always)]
    #[must_use]
    pub fn mmpubsa0(&mut self) -> MMPUBSA0_W<0> {
        MMPUBSA0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Master Memory Protection Unit Security Attribution Register B\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mmpusarb](index.html) module"]
pub struct MMPUSARB_SPEC;
impl crate::RegisterSpec for MMPUSARB_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mmpusarb::R](R) reader structure"]
impl crate::Readable for MMPUSARB_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mmpusarb::W](W) writer structure"]
impl crate::Writable for MMPUSARB_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MMPUSARB to value 0xffff_ffff"]
impl crate::Resettable for MMPUSARB_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_ffff;
}
