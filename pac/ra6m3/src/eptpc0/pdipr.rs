#[doc = "Register `PDIPR` reader"]
pub struct R(crate::R<PDIPR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PDIPR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PDIPR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PDIPR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PDIPR` writer"]
pub struct W(crate::W<PDIPR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PDIPR_SPEC>;
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
impl From<crate::W<PDIPR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PDIPR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PDIPR` reader - These bits hold the setting for the destination IP address for PTPpdelay messages."]
pub type PDIPR_R = crate::FieldReader<u32, u32>;
#[doc = "Field `PDIPR` writer - These bits hold the setting for the destination IP address for PTPpdelay messages."]
pub type PDIPR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PDIPR_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - These bits hold the setting for the destination IP address for PTPpdelay messages."]
    #[inline(always)]
    pub fn pdipr(&self) -> PDIPR_R {
        PDIPR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - These bits hold the setting for the destination IP address for PTPpdelay messages."]
    #[inline(always)]
    #[must_use]
    pub fn pdipr(&mut self) -> PDIPR_W<0> {
        PDIPR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PTP-pdelay Message Destination IP Address Setting Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdipr](index.html) module"]
pub struct PDIPR_SPEC;
impl crate::RegisterSpec for PDIPR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pdipr::R](R) reader structure"]
impl crate::Readable for PDIPR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pdipr::W](W) writer structure"]
impl crate::Writable for PDIPR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PDIPR to value 0xe000_006b"]
impl crate::Resettable for PDIPR_SPEC {
    const RESET_VALUE: Self::Ux = 0xe000_006b;
}
