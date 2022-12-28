#[doc = "Register `GTADCMSS` reader"]
pub struct R(crate::R<GTADCMSS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GTADCMSS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GTADCMSS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GTADCMSS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GTADCMSS` writer"]
pub struct W(crate::W<GTADCMSS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GTADCMSS_SPEC>;
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
impl From<crate::W<GTADCMSS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GTADCMSS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADCMSAL` reader - GTADTRA Register A/D Conversion Start Request Compare Match Skipping Function Select"]
pub type ADCMSAL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ADCMSAL` writer - GTADTRA Register A/D Conversion Start Request Compare Match Skipping Function Select"]
pub type ADCMSAL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GTADCMSS_SPEC, u8, u8, 3, O>;
#[doc = "Field `ADCMSBL` reader - GTADTRB Register A/D Conversion Start Request Compare Match Skipping Function Select"]
pub type ADCMSBL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ADCMSBL` writer - GTADTRB Register A/D Conversion Start Request Compare Match Skipping Function Select"]
pub type ADCMSBL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GTADCMSS_SPEC, u8, u8, 3, O>;
#[doc = "Field `ADCMBSA` reader - GTADTRA Register Buffer Transfer by A/D Conversion Start Request Compare Match Skipping Function Select"]
pub type ADCMBSA_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ADCMBSA` writer - GTADTRA Register Buffer Transfer by A/D Conversion Start Request Compare Match Skipping Function Select"]
pub type ADCMBSA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GTADCMSS_SPEC, u8, u8, 3, O>;
#[doc = "Field `ADCMBSB` reader - GTADTRB Register Buffer Transfer by A/D Conversion Start Request Compare Match Skipping Function Select"]
pub type ADCMBSB_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ADCMBSB` writer - GTADTRB Register Buffer Transfer by A/D Conversion Start Request Compare Match Skipping Function Select"]
pub type ADCMBSB_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GTADCMSS_SPEC, u8, u8, 3, O>;
impl R {
    #[doc = "Bits 0:2 - GTADTRA Register A/D Conversion Start Request Compare Match Skipping Function Select"]
    #[inline(always)]
    pub fn adcmsal(&self) -> ADCMSAL_R {
        ADCMSAL_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:6 - GTADTRB Register A/D Conversion Start Request Compare Match Skipping Function Select"]
    #[inline(always)]
    pub fn adcmsbl(&self) -> ADCMSBL_R {
        ADCMSBL_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 16:18 - GTADTRA Register Buffer Transfer by A/D Conversion Start Request Compare Match Skipping Function Select"]
    #[inline(always)]
    pub fn adcmbsa(&self) -> ADCMBSA_R {
        ADCMBSA_R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 20:22 - GTADTRB Register Buffer Transfer by A/D Conversion Start Request Compare Match Skipping Function Select"]
    #[inline(always)]
    pub fn adcmbsb(&self) -> ADCMBSB_R {
        ADCMBSB_R::new(((self.bits >> 20) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - GTADTRA Register A/D Conversion Start Request Compare Match Skipping Function Select"]
    #[inline(always)]
    #[must_use]
    pub fn adcmsal(&mut self) -> ADCMSAL_W<0> {
        ADCMSAL_W::new(self)
    }
    #[doc = "Bits 4:6 - GTADTRB Register A/D Conversion Start Request Compare Match Skipping Function Select"]
    #[inline(always)]
    #[must_use]
    pub fn adcmsbl(&mut self) -> ADCMSBL_W<4> {
        ADCMSBL_W::new(self)
    }
    #[doc = "Bits 16:18 - GTADTRA Register Buffer Transfer by A/D Conversion Start Request Compare Match Skipping Function Select"]
    #[inline(always)]
    #[must_use]
    pub fn adcmbsa(&mut self) -> ADCMBSA_W<16> {
        ADCMBSA_W::new(self)
    }
    #[doc = "Bits 20:22 - GTADTRB Register Buffer Transfer by A/D Conversion Start Request Compare Match Skipping Function Select"]
    #[inline(always)]
    #[must_use]
    pub fn adcmbsb(&mut self) -> ADCMBSB_W<20> {
        ADCMBSB_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "General PWM Timer A/D Conversion Start Request Compare Match Skipping Setting Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gtadcmss](index.html) module"]
pub struct GTADCMSS_SPEC;
impl crate::RegisterSpec for GTADCMSS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gtadcmss::R](R) reader structure"]
impl crate::Readable for GTADCMSS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gtadcmss::W](W) writer structure"]
impl crate::Writable for GTADCMSS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GTADCMSS to value 0"]
impl crate::Resettable for GTADCMSS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
