#[doc = "Register `PRSST` reader"]
pub struct R(crate::R<PRSST_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PRSST_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PRSST_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PRSST_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PRSST` writer"]
pub struct W(crate::W<PRSST_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PRSST_SPEC>;
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
impl From<crate::W<PRSST_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PRSST_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CRMS` reader - Current Master"]
pub type CRMS_R = crate::BitReader<CRMS_A>;
#[doc = "Current Master\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CRMS_A {
    #[doc = "0: The Master is not the Current Master, and must request and acquire bus ownership before initiating any transfer."]
    _0 = 0,
    #[doc = "1: The Master is the Current Master, and as a result can initiate transfers."]
    _1 = 1,
}
impl From<CRMS_A> for bool {
    #[inline(always)]
    fn from(variant: CRMS_A) -> Self {
        variant as u8 != 0
    }
}
impl CRMS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CRMS_A {
        match self.bits {
            false => CRMS_A::_0,
            true => CRMS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CRMS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CRMS_A::_1
    }
}
#[doc = "Field `CRMS` writer - Current Master"]
pub type CRMS_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRSST_SPEC, CRMS_A, O>;
impl<'a, const O: u8> CRMS_W<'a, O> {
    #[doc = "The Master is not the Current Master, and must request and acquire bus ownership before initiating any transfer."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CRMS_A::_0)
    }
    #[doc = "The Master is the Current Master, and as a result can initiate transfers."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CRMS_A::_1)
    }
}
#[doc = "Field `TRMD` reader - Transmit/Receive Mode"]
pub type TRMD_R = crate::BitReader<TRMD_A>;
#[doc = "Transmit/Receive Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TRMD_A {
    #[doc = "0: Receive mode"]
    _0 = 0,
    #[doc = "1: Transmit mode"]
    _1 = 1,
}
impl From<TRMD_A> for bool {
    #[inline(always)]
    fn from(variant: TRMD_A) -> Self {
        variant as u8 != 0
    }
}
impl TRMD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TRMD_A {
        match self.bits {
            false => TRMD_A::_0,
            true => TRMD_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TRMD_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TRMD_A::_1
    }
}
#[doc = "Present State Write Protect\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PRSSTWP_AW {
    #[doc = "0: CRMS bit is protected."]
    _0 = 0,
    #[doc = "1: CRMS bit can be written when writing simultaneously with the value of the target bit."]
    _1 = 1,
}
impl From<PRSSTWP_AW> for bool {
    #[inline(always)]
    fn from(variant: PRSSTWP_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PRSSTWP` writer - Present State Write Protect"]
pub type PRSSTWP_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRSST_SPEC, PRSSTWP_AW, O>;
impl<'a, const O: u8> PRSSTWP_W<'a, O> {
    #[doc = "CRMS bit is protected."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PRSSTWP_AW::_0)
    }
    #[doc = "CRMS bit can be written when writing simultaneously with the value of the target bit."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PRSSTWP_AW::_1)
    }
}
impl R {
    #[doc = "Bit 2 - Current Master"]
    #[inline(always)]
    pub fn crms(&self) -> CRMS_R {
        CRMS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - Transmit/Receive Mode"]
    #[inline(always)]
    pub fn trmd(&self) -> TRMD_R {
        TRMD_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - Current Master"]
    #[inline(always)]
    #[must_use]
    pub fn crms(&mut self) -> CRMS_W<2> {
        CRMS_W::new(self)
    }
    #[doc = "Bit 7 - Present State Write Protect"]
    #[inline(always)]
    #[must_use]
    pub fn prsstwp(&mut self) -> PRSSTWP_W<7> {
        PRSSTWP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Present State Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prsst](index.html) module"]
pub struct PRSST_SPEC;
impl crate::RegisterSpec for PRSST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [prsst::R](R) reader structure"]
impl crate::Readable for PRSST_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [prsst::W](W) writer structure"]
impl crate::Writable for PRSST_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PRSST to value 0"]
impl crate::Resettable for PRSST_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
