#[doc = "Register `VBTWEGR` reader"]
pub struct R(crate::R<VBTWEGR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<VBTWEGR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<VBTWEGR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<VBTWEGR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `VBTWEGR` writer"]
pub struct W(crate::W<VBTWEGR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<VBTWEGR_SPEC>;
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
impl From<crate::W<VBTWEGR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<VBTWEGR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `VCH0EG` reader - VBATWIO0 Wakeup Trigger Source Edge Select"]
pub type VCH0EG_R = crate::BitReader<VCH0EG_A>;
#[doc = "VBATWIO0 Wakeup Trigger Source Edge Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VCH0EG_A {
    #[doc = "0: Wakeup trigger is generated at a falling edge"]
    _0 = 0,
    #[doc = "1: Wakeup trigger is generated at a rising edge."]
    _1 = 1,
}
impl From<VCH0EG_A> for bool {
    #[inline(always)]
    fn from(variant: VCH0EG_A) -> Self {
        variant as u8 != 0
    }
}
impl VCH0EG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VCH0EG_A {
        match self.bits {
            false => VCH0EG_A::_0,
            true => VCH0EG_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == VCH0EG_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == VCH0EG_A::_1
    }
}
#[doc = "Field `VCH0EG` writer - VBATWIO0 Wakeup Trigger Source Edge Select"]
pub type VCH0EG_W<'a, const O: u8> = crate::BitWriter<'a, u8, VBTWEGR_SPEC, VCH0EG_A, O>;
impl<'a, const O: u8> VCH0EG_W<'a, O> {
    #[doc = "Wakeup trigger is generated at a falling edge"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(VCH0EG_A::_0)
    }
    #[doc = "Wakeup trigger is generated at a rising edge."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(VCH0EG_A::_1)
    }
}
#[doc = "Field `VCH1EG` reader - VBATWIO1 Wakeup Trigger Source Edge Select"]
pub type VCH1EG_R = crate::BitReader<VCH1EG_A>;
#[doc = "VBATWIO1 Wakeup Trigger Source Edge Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VCH1EG_A {
    #[doc = "0: Wakeup trigger is generated at a falling edge"]
    _0 = 0,
    #[doc = "1: Wakeup trigger is generated at a rising edge."]
    _1 = 1,
}
impl From<VCH1EG_A> for bool {
    #[inline(always)]
    fn from(variant: VCH1EG_A) -> Self {
        variant as u8 != 0
    }
}
impl VCH1EG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VCH1EG_A {
        match self.bits {
            false => VCH1EG_A::_0,
            true => VCH1EG_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == VCH1EG_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == VCH1EG_A::_1
    }
}
#[doc = "Field `VCH1EG` writer - VBATWIO1 Wakeup Trigger Source Edge Select"]
pub type VCH1EG_W<'a, const O: u8> = crate::BitWriter<'a, u8, VBTWEGR_SPEC, VCH1EG_A, O>;
impl<'a, const O: u8> VCH1EG_W<'a, O> {
    #[doc = "Wakeup trigger is generated at a falling edge"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(VCH1EG_A::_0)
    }
    #[doc = "Wakeup trigger is generated at a rising edge."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(VCH1EG_A::_1)
    }
}
#[doc = "Field `VCH2EG` reader - VBATWIO2 Wakeup Trigger Source Edge Select"]
pub type VCH2EG_R = crate::BitReader<VCH2EG_A>;
#[doc = "VBATWIO2 Wakeup Trigger Source Edge Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VCH2EG_A {
    #[doc = "0: Wakeup trigger is generated at a falling edge"]
    _0 = 0,
    #[doc = "1: Wakeup trigger is generated at a rising edge."]
    _1 = 1,
}
impl From<VCH2EG_A> for bool {
    #[inline(always)]
    fn from(variant: VCH2EG_A) -> Self {
        variant as u8 != 0
    }
}
impl VCH2EG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VCH2EG_A {
        match self.bits {
            false => VCH2EG_A::_0,
            true => VCH2EG_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == VCH2EG_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == VCH2EG_A::_1
    }
}
#[doc = "Field `VCH2EG` writer - VBATWIO2 Wakeup Trigger Source Edge Select"]
pub type VCH2EG_W<'a, const O: u8> = crate::BitWriter<'a, u8, VBTWEGR_SPEC, VCH2EG_A, O>;
impl<'a, const O: u8> VCH2EG_W<'a, O> {
    #[doc = "Wakeup trigger is generated at a falling edge"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(VCH2EG_A::_0)
    }
    #[doc = "Wakeup trigger is generated at a rising edge."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(VCH2EG_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - VBATWIO0 Wakeup Trigger Source Edge Select"]
    #[inline(always)]
    pub fn vch0eg(&self) -> VCH0EG_R {
        VCH0EG_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - VBATWIO1 Wakeup Trigger Source Edge Select"]
    #[inline(always)]
    pub fn vch1eg(&self) -> VCH1EG_R {
        VCH1EG_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - VBATWIO2 Wakeup Trigger Source Edge Select"]
    #[inline(always)]
    pub fn vch2eg(&self) -> VCH2EG_R {
        VCH2EG_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - VBATWIO0 Wakeup Trigger Source Edge Select"]
    #[inline(always)]
    #[must_use]
    pub fn vch0eg(&mut self) -> VCH0EG_W<0> {
        VCH0EG_W::new(self)
    }
    #[doc = "Bit 1 - VBATWIO1 Wakeup Trigger Source Edge Select"]
    #[inline(always)]
    #[must_use]
    pub fn vch1eg(&mut self) -> VCH1EG_W<1> {
        VCH1EG_W::new(self)
    }
    #[doc = "Bit 2 - VBATWIO2 Wakeup Trigger Source Edge Select"]
    #[inline(always)]
    #[must_use]
    pub fn vch2eg(&mut self) -> VCH2EG_W<2> {
        VCH2EG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "VBATT Wakeup Trigger source Edge Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [vbtwegr](index.html) module"]
pub struct VBTWEGR_SPEC;
impl crate::RegisterSpec for VBTWEGR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [vbtwegr::R](R) reader structure"]
impl crate::Readable for VBTWEGR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [vbtwegr::W](W) writer structure"]
impl crate::Writable for VBTWEGR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets VBTWEGR to value 0"]
impl crate::Resettable for VBTWEGR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
