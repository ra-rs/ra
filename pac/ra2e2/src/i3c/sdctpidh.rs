#[doc = "Register `SDCTPIDH` reader"]
pub struct R(crate::R<SDCTPIDH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SDCTPIDH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SDCTPIDH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SDCTPIDH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SDCTPIDH` writer"]
pub struct W(crate::W<SDCTPIDH_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SDCTPIDH_SPEC>;
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
impl From<crate::W<SDCTPIDH_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SDCTPIDH_SPEC>) -> Self {
        W(writer)
    }
}
impl W {
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Slave Device Characteristic Table Provisional ID High Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sdctpidh](index.html) module"]
pub struct SDCTPIDH_SPEC;
impl crate::RegisterSpec for SDCTPIDH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sdctpidh::R](R) reader structure"]
impl crate::Readable for SDCTPIDH_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sdctpidh::W](W) writer structure"]
impl crate::Writable for SDCTPIDH_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SDCTPIDH to value 0"]
impl crate::Resettable for SDCTPIDH_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
