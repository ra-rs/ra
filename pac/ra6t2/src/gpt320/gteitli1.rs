#[doc = "Register `GTEITLI1` reader"]
pub struct R(crate::R<GTEITLI1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GTEITLI1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GTEITLI1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GTEITLI1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GTEITLI1` writer"]
pub struct W(crate::W<GTEITLI1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GTEITLI1_SPEC>;
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
impl From<crate::W<GTEITLI1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GTEITLI1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EITLA` reader - GTCCRA Register Compare Match/Input Capture Interrupt Extended Skipping Function Select"]
pub type EITLA_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EITLA` writer - GTCCRA Register Compare Match/Input Capture Interrupt Extended Skipping Function Select"]
pub type EITLA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GTEITLI1_SPEC, u8, u8, 3, O>;
#[doc = "Field `EITLB` reader - GTCCRB Register Compare Match/Input Capture Interrupt Extended Skipping Function Select"]
pub type EITLB_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EITLB` writer - GTCCRB Register Compare Match/Input Capture Interrupt Extended Skipping Function Select"]
pub type EITLB_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GTEITLI1_SPEC, u8, u8, 3, O>;
#[doc = "Field `EITLC` reader - GTCCRC Register Compare Match Interrupt Extended Skipping Function Select"]
pub type EITLC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EITLC` writer - GTCCRC Register Compare Match Interrupt Extended Skipping Function Select"]
pub type EITLC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GTEITLI1_SPEC, u8, u8, 3, O>;
#[doc = "Field `EITLD` reader - GTCCRD Register Compare Match Interrupt Extended Skipping Function Select"]
pub type EITLD_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EITLD` writer - GTCCRD Register Compare Match Interrupt Extended Skipping Function Select"]
pub type EITLD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GTEITLI1_SPEC, u8, u8, 3, O>;
#[doc = "Field `EITLE` reader - GTCCRE Register Compare Match Interrupt Extended Skipping Function Select"]
pub type EITLE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EITLE` writer - GTCCRE Register Compare Match Interrupt Extended Skipping Function Select"]
pub type EITLE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GTEITLI1_SPEC, u8, u8, 3, O>;
#[doc = "Field `EITLF` reader - GTCCRF Register Compare Match Interrupt Extended Skipping Function Select"]
pub type EITLF_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EITLF` writer - GTCCRF Register Compare Match Interrupt Extended Skipping Function Select"]
pub type EITLF_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GTEITLI1_SPEC, u8, u8, 3, O>;
#[doc = "Field `EITLV` reader - Overflow Interrupt Extended Skipping Function Select"]
pub type EITLV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EITLV` writer - Overflow Interrupt Extended Skipping Function Select"]
pub type EITLV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GTEITLI1_SPEC, u8, u8, 3, O>;
#[doc = "Field `EITLU` reader - Underflow Interrupt Extended Skipping Function Select"]
pub type EITLU_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EITLU` writer - Underflow Interrupt Extended Skipping Function Select"]
pub type EITLU_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GTEITLI1_SPEC, u8, u8, 3, O>;
impl R {
    #[doc = "Bits 0:2 - GTCCRA Register Compare Match/Input Capture Interrupt Extended Skipping Function Select"]
    #[inline(always)]
    pub fn eitla(&self) -> EITLA_R {
        EITLA_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:6 - GTCCRB Register Compare Match/Input Capture Interrupt Extended Skipping Function Select"]
    #[inline(always)]
    pub fn eitlb(&self) -> EITLB_R {
        EITLB_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 8:10 - GTCCRC Register Compare Match Interrupt Extended Skipping Function Select"]
    #[inline(always)]
    pub fn eitlc(&self) -> EITLC_R {
        EITLC_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 12:14 - GTCCRD Register Compare Match Interrupt Extended Skipping Function Select"]
    #[inline(always)]
    pub fn eitld(&self) -> EITLD_R {
        EITLD_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 16:18 - GTCCRE Register Compare Match Interrupt Extended Skipping Function Select"]
    #[inline(always)]
    pub fn eitle(&self) -> EITLE_R {
        EITLE_R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 20:22 - GTCCRF Register Compare Match Interrupt Extended Skipping Function Select"]
    #[inline(always)]
    pub fn eitlf(&self) -> EITLF_R {
        EITLF_R::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bits 24:26 - Overflow Interrupt Extended Skipping Function Select"]
    #[inline(always)]
    pub fn eitlv(&self) -> EITLV_R {
        EITLV_R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bits 28:30 - Underflow Interrupt Extended Skipping Function Select"]
    #[inline(always)]
    pub fn eitlu(&self) -> EITLU_R {
        EITLU_R::new(((self.bits >> 28) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - GTCCRA Register Compare Match/Input Capture Interrupt Extended Skipping Function Select"]
    #[inline(always)]
    #[must_use]
    pub fn eitla(&mut self) -> EITLA_W<0> {
        EITLA_W::new(self)
    }
    #[doc = "Bits 4:6 - GTCCRB Register Compare Match/Input Capture Interrupt Extended Skipping Function Select"]
    #[inline(always)]
    #[must_use]
    pub fn eitlb(&mut self) -> EITLB_W<4> {
        EITLB_W::new(self)
    }
    #[doc = "Bits 8:10 - GTCCRC Register Compare Match Interrupt Extended Skipping Function Select"]
    #[inline(always)]
    #[must_use]
    pub fn eitlc(&mut self) -> EITLC_W<8> {
        EITLC_W::new(self)
    }
    #[doc = "Bits 12:14 - GTCCRD Register Compare Match Interrupt Extended Skipping Function Select"]
    #[inline(always)]
    #[must_use]
    pub fn eitld(&mut self) -> EITLD_W<12> {
        EITLD_W::new(self)
    }
    #[doc = "Bits 16:18 - GTCCRE Register Compare Match Interrupt Extended Skipping Function Select"]
    #[inline(always)]
    #[must_use]
    pub fn eitle(&mut self) -> EITLE_W<16> {
        EITLE_W::new(self)
    }
    #[doc = "Bits 20:22 - GTCCRF Register Compare Match Interrupt Extended Skipping Function Select"]
    #[inline(always)]
    #[must_use]
    pub fn eitlf(&mut self) -> EITLF_W<20> {
        EITLF_W::new(self)
    }
    #[doc = "Bits 24:26 - Overflow Interrupt Extended Skipping Function Select"]
    #[inline(always)]
    #[must_use]
    pub fn eitlv(&mut self) -> EITLV_W<24> {
        EITLV_W::new(self)
    }
    #[doc = "Bits 28:30 - Underflow Interrupt Extended Skipping Function Select"]
    #[inline(always)]
    #[must_use]
    pub fn eitlu(&mut self) -> EITLU_W<28> {
        EITLU_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "General PWM Timer Extended Interrupt Skipping Setting Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gteitli1](index.html) module"]
pub struct GTEITLI1_SPEC;
impl crate::RegisterSpec for GTEITLI1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gteitli1::R](R) reader structure"]
impl crate::Readable for GTEITLI1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gteitli1::W](W) writer structure"]
impl crate::Writable for GTEITLI1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GTEITLI1 to value 0"]
impl crate::Resettable for GTEITLI1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
