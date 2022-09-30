#[doc = "Register `DTCCR` reader"]
pub struct R(crate::R<DTCCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DTCCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DTCCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DTCCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DTCCR` writer"]
pub struct W(crate::W<DTCCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DTCCR_SPEC>;
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
impl From<crate::W<DTCCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DTCCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RRS` reader - DTC Transfer Information Read Skip Enable"]
pub type RRS_R = crate::BitReader<RRS_A>;
#[doc = "DTC Transfer Information Read Skip Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RRS_A {
    #[doc = "0: Transfer information read is not skipped"]
    _0 = 0,
    #[doc = "1: Transfer information read is skipped when vector numbers match"]
    _1 = 1,
}
impl From<RRS_A> for bool {
    #[inline(always)]
    fn from(variant: RRS_A) -> Self {
        variant as u8 != 0
    }
}
impl RRS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RRS_A {
        match self.bits {
            false => RRS_A::_0,
            true => RRS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RRS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RRS_A::_1
    }
}
#[doc = "Field `RRS` writer - DTC Transfer Information Read Skip Enable"]
pub type RRS_W<'a, const O: u8> = crate::BitWriter<'a, u8, DTCCR_SPEC, RRS_A, O>;
impl<'a, const O: u8> RRS_W<'a, O> {
    #[doc = "Transfer information read is not skipped"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RRS_A::_0)
    }
    #[doc = "Transfer information read is skipped when vector numbers match"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RRS_A::_1)
    }
}
impl R {
    #[doc = "Bit 4 - DTC Transfer Information Read Skip Enable"]
    #[inline(always)]
    pub fn rrs(&self) -> RRS_R {
        RRS_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 4 - DTC Transfer Information Read Skip Enable"]
    #[inline(always)]
    pub fn rrs(&mut self) -> RRS_W<4> {
        RRS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DTC Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dtccr](index.html) module"]
pub struct DTCCR_SPEC;
impl crate::RegisterSpec for DTCCR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [dtccr::R](R) reader structure"]
impl crate::Readable for DTCCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dtccr::W](W) writer structure"]
impl crate::Writable for DTCCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DTCCR to value 0x08"]
impl crate::Resettable for DTCCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x08
    }
}
