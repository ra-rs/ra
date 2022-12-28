#[doc = "Register `ELSEGR%s` reader"]
pub struct R(crate::R<ELSEGR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ELSEGR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ELSEGR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ELSEGR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ELSEGR%s` writer"]
pub struct W(crate::W<ELSEGR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ELSEGR_SPEC>;
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
impl From<crate::W<ELSEGR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ELSEGR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Software Event Generation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SEG_AW {
    #[doc = "0: Normal operation"]
    _0 = 0,
    #[doc = "1: Software event is generated."]
    _1 = 1,
}
impl From<SEG_AW> for bool {
    #[inline(always)]
    fn from(variant: SEG_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SEG` writer - Software Event Generation"]
pub type SEG_W<'a, const O: u8> = crate::BitWriter<'a, u8, ELSEGR_SPEC, SEG_AW, O>;
impl<'a, const O: u8> SEG_W<'a, O> {
    #[doc = "Normal operation"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SEG_AW::_0)
    }
    #[doc = "Software event is generated."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SEG_AW::_1)
    }
}
#[doc = "Field `WE` reader - SEG Bit Write Enable"]
pub type WE_R = crate::BitReader<WE_A>;
#[doc = "SEG Bit Write Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WE_A {
    #[doc = "0: Write to SEG bit disabled."]
    _0 = 0,
    #[doc = "1: Write to SEG bit enabled."]
    _1 = 1,
}
impl From<WE_A> for bool {
    #[inline(always)]
    fn from(variant: WE_A) -> Self {
        variant as u8 != 0
    }
}
impl WE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WE_A {
        match self.bits {
            false => WE_A::_0,
            true => WE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == WE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == WE_A::_1
    }
}
#[doc = "Field `WE` writer - SEG Bit Write Enable"]
pub type WE_W<'a, const O: u8> = crate::BitWriter<'a, u8, ELSEGR_SPEC, WE_A, O>;
impl<'a, const O: u8> WE_W<'a, O> {
    #[doc = "Write to SEG bit disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(WE_A::_0)
    }
    #[doc = "Write to SEG bit enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(WE_A::_1)
    }
}
#[doc = "ELSEGR Register Write Disable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WI_AW {
    #[doc = "0: Write to ELSEGR register enabled."]
    _0 = 0,
    #[doc = "1: Write to ELSEGR register disabled."]
    _1 = 1,
}
impl From<WI_AW> for bool {
    #[inline(always)]
    fn from(variant: WI_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WI` writer - ELSEGR Register Write Disable"]
pub type WI_W<'a, const O: u8> = crate::BitWriter<'a, u8, ELSEGR_SPEC, WI_AW, O>;
impl<'a, const O: u8> WI_W<'a, O> {
    #[doc = "Write to ELSEGR register enabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(WI_AW::_0)
    }
    #[doc = "Write to ELSEGR register disabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(WI_AW::_1)
    }
}
impl R {
    #[doc = "Bit 6 - SEG Bit Write Enable"]
    #[inline(always)]
    pub fn we(&self) -> WE_R {
        WE_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Software Event Generation"]
    #[inline(always)]
    #[must_use]
    pub fn seg(&mut self) -> SEG_W<0> {
        SEG_W::new(self)
    }
    #[doc = "Bit 6 - SEG Bit Write Enable"]
    #[inline(always)]
    #[must_use]
    pub fn we(&mut self) -> WE_W<6> {
        WE_W::new(self)
    }
    #[doc = "Bit 7 - ELSEGR Register Write Disable"]
    #[inline(always)]
    #[must_use]
    pub fn wi(&mut self) -> WI_W<7> {
        WI_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Event Link Software Event Generation Register %s\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [elsegr](index.html) module"]
pub struct ELSEGR_SPEC;
impl crate::RegisterSpec for ELSEGR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [elsegr::R](R) reader structure"]
impl crate::Readable for ELSEGR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [elsegr::W](W) writer structure"]
impl crate::Writable for ELSEGR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ELSEGR%s to value 0x80"]
impl crate::Resettable for ELSEGR_SPEC {
    const RESET_VALUE: Self::Ux = 0x80;
}
