#[doc = "Register `TRSCER` reader"]
pub struct R(crate::R<TRSCER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TRSCER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TRSCER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TRSCER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TRSCER` writer"]
pub struct W(crate::W<TRSCER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TRSCER_SPEC>;
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
impl From<crate::W<TRSCER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TRSCER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RRFCE` reader - RRF Flag Copy Enable"]
pub type RRFCE_R = crate::BitReader<RRFCE_A>;
#[doc = "RRF Flag Copy Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RRFCE_A {
    #[doc = "0: Reflect the EESR.RRF flag status in the RD0.RFE bit of the receive descriptor"]
    _0 = 0,
    #[doc = "1: Do not reflect the EESR.RRF flag status in the RD0.RFE bit of the receive descriptor."]
    _1 = 1,
}
impl From<RRFCE_A> for bool {
    #[inline(always)]
    fn from(variant: RRFCE_A) -> Self {
        variant as u8 != 0
    }
}
impl RRFCE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RRFCE_A {
        match self.bits {
            false => RRFCE_A::_0,
            true => RRFCE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RRFCE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RRFCE_A::_1
    }
}
#[doc = "Field `RRFCE` writer - RRF Flag Copy Enable"]
pub type RRFCE_W<'a, const O: u8> = crate::BitWriter<'a, u32, TRSCER_SPEC, RRFCE_A, O>;
impl<'a, const O: u8> RRFCE_W<'a, O> {
    #[doc = "Reflect the EESR.RRF flag status in the RD0.RFE bit of the receive descriptor"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RRFCE_A::_0)
    }
    #[doc = "Do not reflect the EESR.RRF flag status in the RD0.RFE bit of the receive descriptor."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RRFCE_A::_1)
    }
}
#[doc = "Field `RMAFCE` reader - RMAF Flag Copy Enable"]
pub type RMAFCE_R = crate::BitReader<RMAFCE_A>;
#[doc = "RMAF Flag Copy Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RMAFCE_A {
    #[doc = "0: Reflect the EESR.RMAF flag status in the RD0.RFE bit of the receive descriptor"]
    _0 = 0,
    #[doc = "1: Do not reflect the EESR.RMAF flag status in the RD0.RFE bit of the receive descriptor."]
    _1 = 1,
}
impl From<RMAFCE_A> for bool {
    #[inline(always)]
    fn from(variant: RMAFCE_A) -> Self {
        variant as u8 != 0
    }
}
impl RMAFCE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RMAFCE_A {
        match self.bits {
            false => RMAFCE_A::_0,
            true => RMAFCE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RMAFCE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RMAFCE_A::_1
    }
}
#[doc = "Field `RMAFCE` writer - RMAF Flag Copy Enable"]
pub type RMAFCE_W<'a, const O: u8> = crate::BitWriter<'a, u32, TRSCER_SPEC, RMAFCE_A, O>;
impl<'a, const O: u8> RMAFCE_W<'a, O> {
    #[doc = "Reflect the EESR.RMAF flag status in the RD0.RFE bit of the receive descriptor"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RMAFCE_A::_0)
    }
    #[doc = "Do not reflect the EESR.RMAF flag status in the RD0.RFE bit of the receive descriptor."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RMAFCE_A::_1)
    }
}
impl R {
    #[doc = "Bit 4 - RRF Flag Copy Enable"]
    #[inline(always)]
    pub fn rrfce(&self) -> RRFCE_R {
        RRFCE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 7 - RMAF Flag Copy Enable"]
    #[inline(always)]
    pub fn rmafce(&self) -> RMAFCE_R {
        RMAFCE_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 4 - RRF Flag Copy Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rrfce(&mut self) -> RRFCE_W<4> {
        RRFCE_W::new(self)
    }
    #[doc = "Bit 7 - RMAF Flag Copy Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rmafce(&mut self) -> RMAFCE_W<7> {
        RMAFCE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ETHERC/EDMAC Transmit/Receive Status Copy Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [trscer](index.html) module"]
pub struct TRSCER_SPEC;
impl crate::RegisterSpec for TRSCER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [trscer::R](R) reader structure"]
impl crate::Readable for TRSCER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [trscer::W](W) writer structure"]
impl crate::Writable for TRSCER_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TRSCER to value 0"]
impl crate::Resettable for TRSCER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
