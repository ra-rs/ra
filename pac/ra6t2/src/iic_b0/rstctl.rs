#[doc = "Register `RSTCTL` reader"]
pub struct R(crate::R<RSTCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RSTCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RSTCTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RSTCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RSTCTL` writer"]
pub struct W(crate::W<RSTCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RSTCTL_SPEC>;
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
impl From<crate::W<RSTCTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RSTCTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RI2CRST` reader - IIC Software Reset"]
pub type RI2CRST_R = crate::BitReader<RI2CRST_A>;
#[doc = "IIC Software Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RI2CRST_A {
    #[doc = "0: Reset of all registers and internal state."]
    _0 = 0,
    #[doc = "1: Releases of all registers and internal state."]
    _1 = 1,
}
impl From<RI2CRST_A> for bool {
    #[inline(always)]
    fn from(variant: RI2CRST_A) -> Self {
        variant as u8 != 0
    }
}
impl RI2CRST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RI2CRST_A {
        match self.bits {
            false => RI2CRST_A::_0,
            true => RI2CRST_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RI2CRST_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RI2CRST_A::_1
    }
}
#[doc = "Field `RI2CRST` writer - IIC Software Reset"]
pub type RI2CRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, RSTCTL_SPEC, RI2CRST_A, O>;
impl<'a, const O: u8> RI2CRST_W<'a, O> {
    #[doc = "Reset of all registers and internal state."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RI2CRST_A::_0)
    }
    #[doc = "Releases of all registers and internal state."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RI2CRST_A::_1)
    }
}
#[doc = "Field `INTLRST` reader - Internal Software Reset"]
pub type INTLRST_R = crate::BitReader<INTLRST_A>;
#[doc = "Internal Software Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INTLRST_A {
    #[doc = "0: Releases of some registers and internal state."]
    _0 = 0,
    #[doc = "1: Resets of some registers and internal state."]
    _1 = 1,
}
impl From<INTLRST_A> for bool {
    #[inline(always)]
    fn from(variant: INTLRST_A) -> Self {
        variant as u8 != 0
    }
}
impl INTLRST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INTLRST_A {
        match self.bits {
            false => INTLRST_A::_0,
            true => INTLRST_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == INTLRST_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == INTLRST_A::_1
    }
}
#[doc = "Field `INTLRST` writer - Internal Software Reset"]
pub type INTLRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, RSTCTL_SPEC, INTLRST_A, O>;
impl<'a, const O: u8> INTLRST_W<'a, O> {
    #[doc = "Releases of some registers and internal state."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(INTLRST_A::_0)
    }
    #[doc = "Resets of some registers and internal state."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(INTLRST_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - IIC Software Reset"]
    #[inline(always)]
    pub fn ri2crst(&self) -> RI2CRST_R {
        RI2CRST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 16 - Internal Software Reset"]
    #[inline(always)]
    pub fn intlrst(&self) -> INTLRST_R {
        INTLRST_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - IIC Software Reset"]
    #[inline(always)]
    #[must_use]
    pub fn ri2crst(&mut self) -> RI2CRST_W<0> {
        RI2CRST_W::new(self)
    }
    #[doc = "Bit 16 - Internal Software Reset"]
    #[inline(always)]
    #[must_use]
    pub fn intlrst(&mut self) -> INTLRST_W<16> {
        INTLRST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Reset Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rstctl](index.html) module"]
pub struct RSTCTL_SPEC;
impl crate::RegisterSpec for RSTCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rstctl::R](R) reader structure"]
impl crate::Readable for RSTCTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rstctl::W](W) writer structure"]
impl crate::Writable for RSTCTL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RSTCTL to value 0"]
impl crate::Resettable for RSTCTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
