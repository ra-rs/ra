#[doc = "Register `VBTICTLR` reader"]
pub struct R(crate::R<VBTICTLR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<VBTICTLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<VBTICTLR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<VBTICTLR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `VBTICTLR` writer"]
pub struct W(crate::W<VBTICTLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<VBTICTLR_SPEC>;
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
impl From<crate::W<VBTICTLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<VBTICTLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `VCH0INEN` reader - VBATT CH0 Input Enable"]
pub type VCH0INEN_R = crate::BitReader<VCH0INEN_A>;
#[doc = "VBATT CH0 Input Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VCH0INEN_A {
    #[doc = "0: RTCIC0 input disable"]
    _0 = 0,
    #[doc = "1: RTCIC0 input enable"]
    _1 = 1,
}
impl From<VCH0INEN_A> for bool {
    #[inline(always)]
    fn from(variant: VCH0INEN_A) -> Self {
        variant as u8 != 0
    }
}
impl VCH0INEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VCH0INEN_A {
        match self.bits {
            false => VCH0INEN_A::_0,
            true => VCH0INEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == VCH0INEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == VCH0INEN_A::_1
    }
}
#[doc = "Field `VCH0INEN` writer - VBATT CH0 Input Enable"]
pub type VCH0INEN_W<'a, const O: u8> = crate::BitWriter<'a, u8, VBTICTLR_SPEC, VCH0INEN_A, O>;
impl<'a, const O: u8> VCH0INEN_W<'a, O> {
    #[doc = "RTCIC0 input disable"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(VCH0INEN_A::_0)
    }
    #[doc = "RTCIC0 input enable"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(VCH0INEN_A::_1)
    }
}
#[doc = "Field `VCH1INEN` reader - VBATT CH1 Input Enable"]
pub type VCH1INEN_R = crate::BitReader<VCH1INEN_A>;
#[doc = "VBATT CH1 Input Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VCH1INEN_A {
    #[doc = "0: RTCIC1 input disable"]
    _0 = 0,
    #[doc = "1: RTCIC1 input enable"]
    _1 = 1,
}
impl From<VCH1INEN_A> for bool {
    #[inline(always)]
    fn from(variant: VCH1INEN_A) -> Self {
        variant as u8 != 0
    }
}
impl VCH1INEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VCH1INEN_A {
        match self.bits {
            false => VCH1INEN_A::_0,
            true => VCH1INEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == VCH1INEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == VCH1INEN_A::_1
    }
}
#[doc = "Field `VCH1INEN` writer - VBATT CH1 Input Enable"]
pub type VCH1INEN_W<'a, const O: u8> = crate::BitWriter<'a, u8, VBTICTLR_SPEC, VCH1INEN_A, O>;
impl<'a, const O: u8> VCH1INEN_W<'a, O> {
    #[doc = "RTCIC1 input disable"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(VCH1INEN_A::_0)
    }
    #[doc = "RTCIC1 input enable"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(VCH1INEN_A::_1)
    }
}
#[doc = "Field `VCH2INEN` reader - VBATT CH2 Input Enable"]
pub type VCH2INEN_R = crate::BitReader<VCH2INEN_A>;
#[doc = "VBATT CH2 Input Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VCH2INEN_A {
    #[doc = "0: RTCIC2 input disable"]
    _0 = 0,
    #[doc = "1: RTCIC2 input enable"]
    _1 = 1,
}
impl From<VCH2INEN_A> for bool {
    #[inline(always)]
    fn from(variant: VCH2INEN_A) -> Self {
        variant as u8 != 0
    }
}
impl VCH2INEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VCH2INEN_A {
        match self.bits {
            false => VCH2INEN_A::_0,
            true => VCH2INEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == VCH2INEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == VCH2INEN_A::_1
    }
}
#[doc = "Field `VCH2INEN` writer - VBATT CH2 Input Enable"]
pub type VCH2INEN_W<'a, const O: u8> = crate::BitWriter<'a, u8, VBTICTLR_SPEC, VCH2INEN_A, O>;
impl<'a, const O: u8> VCH2INEN_W<'a, O> {
    #[doc = "RTCIC2 input disable"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(VCH2INEN_A::_0)
    }
    #[doc = "RTCIC2 input enable"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(VCH2INEN_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - VBATT CH0 Input Enable"]
    #[inline(always)]
    pub fn vch0inen(&self) -> VCH0INEN_R {
        VCH0INEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - VBATT CH1 Input Enable"]
    #[inline(always)]
    pub fn vch1inen(&self) -> VCH1INEN_R {
        VCH1INEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - VBATT CH2 Input Enable"]
    #[inline(always)]
    pub fn vch2inen(&self) -> VCH2INEN_R {
        VCH2INEN_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - VBATT CH0 Input Enable"]
    #[inline(always)]
    #[must_use]
    pub fn vch0inen(&mut self) -> VCH0INEN_W<0> {
        VCH0INEN_W::new(self)
    }
    #[doc = "Bit 1 - VBATT CH1 Input Enable"]
    #[inline(always)]
    #[must_use]
    pub fn vch1inen(&mut self) -> VCH1INEN_W<1> {
        VCH1INEN_W::new(self)
    }
    #[doc = "Bit 2 - VBATT CH2 Input Enable"]
    #[inline(always)]
    #[must_use]
    pub fn vch2inen(&mut self) -> VCH2INEN_W<2> {
        VCH2INEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "VBATT Input Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [vbtictlr](index.html) module"]
pub struct VBTICTLR_SPEC;
impl crate::RegisterSpec for VBTICTLR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [vbtictlr::R](R) reader structure"]
impl crate::Readable for VBTICTLR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [vbtictlr::W](W) writer structure"]
impl crate::Writable for VBTICTLR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets VBTICTLR to value 0"]
impl crate::Resettable for VBTICTLR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
