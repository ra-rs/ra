#[doc = "Register `CTSUSO` reader"]
pub struct R(crate::R<CTSUSO_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTSUSO_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTSUSO_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTSUSO_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTSUSO` writer"]
pub struct W(crate::W<CTSUSO_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTSUSO_SPEC>;
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
impl From<crate::W<CTSUSO_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTSUSO_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SO` reader - CTSU Sensor Offset Adjustment"]
pub type SO_R = crate::FieldReader<u16, u16>;
#[doc = "Field `SO` writer - CTSU Sensor Offset Adjustment"]
pub type SO_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTSUSO_SPEC, u16, u16, 10, O>;
#[doc = "Field `SNUM` reader - CTSU Measurement Count Setting"]
pub type SNUM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SNUM` writer - CTSU Measurement Count Setting"]
pub type SNUM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTSUSO_SPEC, u8, u8, 8, O>;
#[doc = "Field `SSDIV` reader - Spread Spectrum Frequency"]
pub type SSDIV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SSDIV` writer - Spread Spectrum Frequency"]
pub type SSDIV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTSUSO_SPEC, u8, u8, 4, O>;
#[doc = "Field `SDPA` reader - CTSU Base Clock Setting"]
pub type SDPA_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SDPA` writer - CTSU Base Clock Setting"]
pub type SDPA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTSUSO_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:9 - CTSU Sensor Offset Adjustment"]
    #[inline(always)]
    pub fn so(&self) -> SO_R {
        SO_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 10:17 - CTSU Measurement Count Setting"]
    #[inline(always)]
    pub fn snum(&self) -> SNUM_R {
        SNUM_R::new(((self.bits >> 10) & 0xff) as u8)
    }
    #[doc = "Bits 20:23 - Spread Spectrum Frequency"]
    #[inline(always)]
    pub fn ssdiv(&self) -> SSDIV_R {
        SSDIV_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:31 - CTSU Base Clock Setting"]
    #[inline(always)]
    pub fn sdpa(&self) -> SDPA_R {
        SDPA_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:9 - CTSU Sensor Offset Adjustment"]
    #[inline(always)]
    #[must_use]
    pub fn so(&mut self) -> SO_W<0> {
        SO_W::new(self)
    }
    #[doc = "Bits 10:17 - CTSU Measurement Count Setting"]
    #[inline(always)]
    #[must_use]
    pub fn snum(&mut self) -> SNUM_W<10> {
        SNUM_W::new(self)
    }
    #[doc = "Bits 20:23 - Spread Spectrum Frequency"]
    #[inline(always)]
    #[must_use]
    pub fn ssdiv(&mut self) -> SSDIV_W<20> {
        SSDIV_W::new(self)
    }
    #[doc = "Bits 24:31 - CTSU Base Clock Setting"]
    #[inline(always)]
    #[must_use]
    pub fn sdpa(&mut self) -> SDPA_W<24> {
        SDPA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CTSU Sensor Offset Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctsuso](index.html) module"]
pub struct CTSUSO_SPEC;
impl crate::RegisterSpec for CTSUSO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctsuso::R](R) reader structure"]
impl crate::Readable for CTSUSO_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctsuso::W](W) writer structure"]
impl crate::Writable for CTSUSO_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTSUSO to value 0"]
impl crate::Resettable for CTSUSO_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
