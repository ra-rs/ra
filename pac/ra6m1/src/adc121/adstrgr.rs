#[doc = "Register `ADSTRGR` reader"]
pub struct R(crate::R<ADSTRGR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADSTRGR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADSTRGR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADSTRGR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADSTRGR` writer"]
pub struct W(crate::W<ADSTRGR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADSTRGR_SPEC>;
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
impl From<crate::W<ADSTRGR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADSTRGR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TRSB` reader - A/D Conversion Start Trigger Select for Group B Select the A/D conversion start trigger for group B in group scan mode."]
pub type TRSB_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TRSB` writer - A/D Conversion Start Trigger Select for Group B Select the A/D conversion start trigger for group B in group scan mode."]
pub type TRSB_W<'a, const O: u8> = crate::FieldWriter<'a, u16, ADSTRGR_SPEC, u8, u8, 6, O>;
#[doc = "Field `TRSA` reader - A/D Conversion Start Trigger Select Select the A/D conversion start trigger in single scan mode and continuous mode. In group scan mode, the A/D conversion start trigger for group A is selected."]
pub type TRSA_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TRSA` writer - A/D Conversion Start Trigger Select Select the A/D conversion start trigger in single scan mode and continuous mode. In group scan mode, the A/D conversion start trigger for group A is selected."]
pub type TRSA_W<'a, const O: u8> = crate::FieldWriter<'a, u16, ADSTRGR_SPEC, u8, u8, 6, O>;
impl R {
    #[doc = "Bits 0:5 - A/D Conversion Start Trigger Select for Group B Select the A/D conversion start trigger for group B in group scan mode."]
    #[inline(always)]
    pub fn trsb(&self) -> TRSB_R {
        TRSB_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:13 - A/D Conversion Start Trigger Select Select the A/D conversion start trigger in single scan mode and continuous mode. In group scan mode, the A/D conversion start trigger for group A is selected."]
    #[inline(always)]
    pub fn trsa(&self) -> TRSA_R {
        TRSA_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - A/D Conversion Start Trigger Select for Group B Select the A/D conversion start trigger for group B in group scan mode."]
    #[inline(always)]
    #[must_use]
    pub fn trsb(&mut self) -> TRSB_W<0> {
        TRSB_W::new(self)
    }
    #[doc = "Bits 8:13 - A/D Conversion Start Trigger Select Select the A/D conversion start trigger in single scan mode and continuous mode. In group scan mode, the A/D conversion start trigger for group A is selected."]
    #[inline(always)]
    #[must_use]
    pub fn trsa(&mut self) -> TRSA_W<8> {
        TRSA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "A/D Conversion Start Trigger Select Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adstrgr](index.html) module"]
pub struct ADSTRGR_SPEC;
impl crate::RegisterSpec for ADSTRGR_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [adstrgr::R](R) reader structure"]
impl crate::Readable for ADSTRGR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adstrgr::W](W) writer structure"]
impl crate::Writable for ADSTRGR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADSTRGR to value 0"]
impl crate::Resettable for ADSTRGR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
