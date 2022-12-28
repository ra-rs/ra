#[doc = "Register `ICUSARB` reader"]
pub struct R(crate::R<ICUSARB_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ICUSARB_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ICUSARB_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ICUSARB_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ICUSARB` writer"]
pub struct W(crate::W<ICUSARB_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ICUSARB_SPEC>;
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
impl From<crate::W<ICUSARB_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ICUSARB_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SANMI` reader - Security attributes of registers for nonmaskable interrupt"]
pub type SANMI_R = crate::BitReader<SANMI_A>;
#[doc = "Security attributes of registers for nonmaskable interrupt\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SANMI_A {
    #[doc = "0: Secure"]
    _0 = 0,
    #[doc = "1: Non-secure"]
    _1 = 1,
}
impl From<SANMI_A> for bool {
    #[inline(always)]
    fn from(variant: SANMI_A) -> Self {
        variant as u8 != 0
    }
}
impl SANMI_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SANMI_A {
        match self.bits {
            false => SANMI_A::_0,
            true => SANMI_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SANMI_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SANMI_A::_1
    }
}
#[doc = "Field `SANMI` writer - Security attributes of registers for nonmaskable interrupt"]
pub type SANMI_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICUSARB_SPEC, SANMI_A, O>;
impl<'a, const O: u8> SANMI_W<'a, O> {
    #[doc = "Secure"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SANMI_A::_0)
    }
    #[doc = "Non-secure"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SANMI_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Security attributes of registers for nonmaskable interrupt"]
    #[inline(always)]
    pub fn sanmi(&self) -> SANMI_R {
        SANMI_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Security attributes of registers for nonmaskable interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn sanmi(&mut self) -> SANMI_W<0> {
        SANMI_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Controller Unit Security Attribution Register B\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [icusarb](index.html) module"]
pub struct ICUSARB_SPEC;
impl crate::RegisterSpec for ICUSARB_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [icusarb::R](R) reader structure"]
impl crate::Readable for ICUSARB_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [icusarb::W](W) writer structure"]
impl crate::Writable for ICUSARB_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ICUSARB to value 0xffff_ffff"]
impl crate::Resettable for ICUSARB_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_ffff;
}
