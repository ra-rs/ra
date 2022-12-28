#[doc = "Register `PPIPR` reader"]
pub struct R(crate::R<PPIPR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PPIPR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PPIPR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PPIPR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PPIPR` writer"]
pub struct W(crate::W<PPIPR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PPIPR_SPEC>;
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
impl From<crate::W<PPIPR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PPIPR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PPIPR` reader - These bits hold the setting for the destination IP address for PTPprimary messages."]
pub type PPIPR_R = crate::FieldReader<u32, u32>;
#[doc = "Field `PPIPR` writer - These bits hold the setting for the destination IP address for PTPprimary messages."]
pub type PPIPR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PPIPR_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - These bits hold the setting for the destination IP address for PTPprimary messages."]
    #[inline(always)]
    pub fn ppipr(&self) -> PPIPR_R {
        PPIPR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - These bits hold the setting for the destination IP address for PTPprimary messages."]
    #[inline(always)]
    #[must_use]
    pub fn ppipr(&mut self) -> PPIPR_W<0> {
        PPIPR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PTP-primary Message Destination IP Address Setting Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ppipr](index.html) module"]
pub struct PPIPR_SPEC;
impl crate::RegisterSpec for PPIPR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ppipr::R](R) reader structure"]
impl crate::Readable for PPIPR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ppipr::W](W) writer structure"]
impl crate::Writable for PPIPR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PPIPR to value 0xe000_0181"]
impl crate::Resettable for PPIPR_SPEC {
    const RESET_VALUE: Self::Ux = 0xe000_0181;
}
