#[doc = "Register `SYTRENR` reader"]
pub struct R(crate::R<SYTRENR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYTRENR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SYTRENR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SYTRENR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SYTRENR` writer"]
pub struct W(crate::W<SYTRENR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SYTRENR_SPEC>;
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
impl From<crate::W<SYTRENR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SYTRENR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ANCE` reader - Announce Message Transmission Enable"]
pub type ANCE_R = crate::BitReader<ANCE_A>;
#[doc = "Announce Message Transmission Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ANCE_A {
    #[doc = "0: Announce messages are not transmitted."]
    _0 = 0,
    #[doc = "1: Announce messages are transmitted."]
    _1 = 1,
}
impl From<ANCE_A> for bool {
    #[inline(always)]
    fn from(variant: ANCE_A) -> Self {
        variant as u8 != 0
    }
}
impl ANCE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ANCE_A {
        match self.bits {
            false => ANCE_A::_0,
            true => ANCE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ANCE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ANCE_A::_1
    }
}
#[doc = "Field `ANCE` writer - Announce Message Transmission Enable"]
pub type ANCE_W<'a, const O: u8> = crate::BitWriter<'a, u32, SYTRENR_SPEC, ANCE_A, O>;
impl<'a, const O: u8> ANCE_W<'a, O> {
    #[doc = "Announce messages are not transmitted."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ANCE_A::_0)
    }
    #[doc = "Announce messages are transmitted."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ANCE_A::_1)
    }
}
#[doc = "Field `SYNC` reader - Sync Message Transmission Enable"]
pub type SYNC_R = crate::BitReader<SYNC_A>;
#[doc = "Sync Message Transmission Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SYNC_A {
    #[doc = "0: Sync messages are not transmitted."]
    _0 = 0,
    #[doc = "1: Sync messages are transmitted."]
    _1 = 1,
}
impl From<SYNC_A> for bool {
    #[inline(always)]
    fn from(variant: SYNC_A) -> Self {
        variant as u8 != 0
    }
}
impl SYNC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SYNC_A {
        match self.bits {
            false => SYNC_A::_0,
            true => SYNC_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SYNC_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SYNC_A::_1
    }
}
#[doc = "Field `SYNC` writer - Sync Message Transmission Enable"]
pub type SYNC_W<'a, const O: u8> = crate::BitWriter<'a, u32, SYTRENR_SPEC, SYNC_A, O>;
impl<'a, const O: u8> SYNC_W<'a, O> {
    #[doc = "Sync messages are not transmitted."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SYNC_A::_0)
    }
    #[doc = "Sync messages are transmitted."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SYNC_A::_1)
    }
}
#[doc = "Field `DRQ` reader - Delay_Req Message Transmission Enable"]
pub type DRQ_R = crate::BitReader<DRQ_A>;
#[doc = "Delay_Req Message Transmission Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DRQ_A {
    #[doc = "0: Delay_Req messages are not transmitted."]
    _0 = 0,
    #[doc = "1: Delay_Req messages are transmitted."]
    _1 = 1,
}
impl From<DRQ_A> for bool {
    #[inline(always)]
    fn from(variant: DRQ_A) -> Self {
        variant as u8 != 0
    }
}
impl DRQ_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DRQ_A {
        match self.bits {
            false => DRQ_A::_0,
            true => DRQ_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DRQ_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DRQ_A::_1
    }
}
#[doc = "Field `DRQ` writer - Delay_Req Message Transmission Enable"]
pub type DRQ_W<'a, const O: u8> = crate::BitWriter<'a, u32, SYTRENR_SPEC, DRQ_A, O>;
impl<'a, const O: u8> DRQ_W<'a, O> {
    #[doc = "Delay_Req messages are not transmitted."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DRQ_A::_0)
    }
    #[doc = "Delay_Req messages are transmitted."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DRQ_A::_1)
    }
}
#[doc = "Field `PDRQ` reader - Pdelay_Req Message Transmission Enable"]
pub type PDRQ_R = crate::BitReader<PDRQ_A>;
#[doc = "Pdelay_Req Message Transmission Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PDRQ_A {
    #[doc = "0: Pdelay_Req messages are not transmitted."]
    _0 = 0,
    #[doc = "1: Pdelay_Req messages are transmitted."]
    _1 = 1,
}
impl From<PDRQ_A> for bool {
    #[inline(always)]
    fn from(variant: PDRQ_A) -> Self {
        variant as u8 != 0
    }
}
impl PDRQ_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDRQ_A {
        match self.bits {
            false => PDRQ_A::_0,
            true => PDRQ_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PDRQ_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PDRQ_A::_1
    }
}
#[doc = "Field `PDRQ` writer - Pdelay_Req Message Transmission Enable"]
pub type PDRQ_W<'a, const O: u8> = crate::BitWriter<'a, u32, SYTRENR_SPEC, PDRQ_A, O>;
impl<'a, const O: u8> PDRQ_W<'a, O> {
    #[doc = "Pdelay_Req messages are not transmitted."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PDRQ_A::_0)
    }
    #[doc = "Pdelay_Req messages are transmitted."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PDRQ_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Announce Message Transmission Enable"]
    #[inline(always)]
    pub fn ance(&self) -> ANCE_R {
        ANCE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - Sync Message Transmission Enable"]
    #[inline(always)]
    pub fn sync(&self) -> SYNC_R {
        SYNC_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - Delay_Req Message Transmission Enable"]
    #[inline(always)]
    pub fn drq(&self) -> DRQ_R {
        DRQ_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 12 - Pdelay_Req Message Transmission Enable"]
    #[inline(always)]
    pub fn pdrq(&self) -> PDRQ_R {
        PDRQ_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Announce Message Transmission Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ance(&mut self) -> ANCE_W<0> {
        ANCE_W::new(self)
    }
    #[doc = "Bit 4 - Sync Message Transmission Enable"]
    #[inline(always)]
    #[must_use]
    pub fn sync(&mut self) -> SYNC_W<4> {
        SYNC_W::new(self)
    }
    #[doc = "Bit 8 - Delay_Req Message Transmission Enable"]
    #[inline(always)]
    #[must_use]
    pub fn drq(&mut self) -> DRQ_W<8> {
        DRQ_W::new(self)
    }
    #[doc = "Bit 12 - Pdelay_Req Message Transmission Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pdrq(&mut self) -> PDRQ_W<12> {
        PDRQ_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SYNFP Transmission Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sytrenr](index.html) module"]
pub struct SYTRENR_SPEC;
impl crate::RegisterSpec for SYTRENR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sytrenr::R](R) reader structure"]
impl crate::Readable for SYTRENR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sytrenr::W](W) writer structure"]
impl crate::Writable for SYTRENR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SYTRENR to value 0"]
impl crate::Resettable for SYTRENR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
