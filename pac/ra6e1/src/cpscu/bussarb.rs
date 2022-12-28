#[doc = "Register `BUSSARB` reader"]
pub struct R(crate::R<BUSSARB_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BUSSARB_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BUSSARB_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BUSSARB_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BUSSARB` writer"]
pub struct W(crate::W<BUSSARB_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BUSSARB_SPEC>;
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
impl From<crate::W<BUSSARB_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BUSSARB_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BUSSB0` reader - BUS Security Attribution B0"]
pub type BUSSB0_R = crate::BitReader<BUSSB0_A>;
#[doc = "BUS Security Attribution B0\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BUSSB0_A {
    #[doc = "0: Secure"]
    _0 = 0,
    #[doc = "1: Non-Secure"]
    _1 = 1,
}
impl From<BUSSB0_A> for bool {
    #[inline(always)]
    fn from(variant: BUSSB0_A) -> Self {
        variant as u8 != 0
    }
}
impl BUSSB0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BUSSB0_A {
        match self.bits {
            false => BUSSB0_A::_0,
            true => BUSSB0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BUSSB0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BUSSB0_A::_1
    }
}
#[doc = "Field `BUSSB0` writer - BUS Security Attribution B0"]
pub type BUSSB0_W<'a, const O: u8> = crate::BitWriter<'a, u32, BUSSARB_SPEC, BUSSB0_A, O>;
impl<'a, const O: u8> BUSSB0_W<'a, O> {
    #[doc = "Secure"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BUSSB0_A::_0)
    }
    #[doc = "Non-Secure"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BUSSB0_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - BUS Security Attribution B0"]
    #[inline(always)]
    pub fn bussb0(&self) -> BUSSB0_R {
        BUSSB0_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - BUS Security Attribution B0"]
    #[inline(always)]
    #[must_use]
    pub fn bussb0(&mut self) -> BUSSB0_W<0> {
        BUSSB0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "BUS Security Attribution Register B\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bussarb](index.html) module"]
pub struct BUSSARB_SPEC;
impl crate::RegisterSpec for BUSSARB_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bussarb::R](R) reader structure"]
impl crate::Readable for BUSSARB_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bussarb::W](W) writer structure"]
impl crate::Writable for BUSSARB_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BUSSARB to value 0xffff_ffff"]
impl crate::Resettable for BUSSARB_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_ffff;
}
