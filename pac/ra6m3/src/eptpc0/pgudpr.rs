#[doc = "Register `PGUDPR` reader"]
pub struct R(crate::R<PGUDPR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PGUDPR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PGUDPR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PGUDPR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PGUDPR` writer"]
pub struct W(crate::W<PGUDPR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PGUDPR_SPEC>;
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
impl From<crate::W<PGUDPR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PGUDPR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `GEUPT` reader - PTP general Message Destination Port NumberThese bits hold the setting for the value of the destination port number field within the UDP headers of PTP general messages."]
pub type GEUPT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `GEUPT` writer - PTP general Message Destination Port NumberThese bits hold the setting for the value of the destination port number field within the UDP headers of PTP general messages."]
pub type GEUPT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PGUDPR_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - PTP general Message Destination Port NumberThese bits hold the setting for the value of the destination port number field within the UDP headers of PTP general messages."]
    #[inline(always)]
    pub fn geupt(&self) -> GEUPT_R {
        GEUPT_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - PTP general Message Destination Port NumberThese bits hold the setting for the value of the destination port number field within the UDP headers of PTP general messages."]
    #[inline(always)]
    #[must_use]
    pub fn geupt(&mut self) -> GEUPT_W<0> {
        GEUPT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PTP general Message UDP Destination Port Number Setting Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pgudpr](index.html) module"]
pub struct PGUDPR_SPEC;
impl crate::RegisterSpec for PGUDPR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pgudpr::R](R) reader structure"]
impl crate::Readable for PGUDPR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pgudpr::W](W) writer structure"]
impl crate::Writable for PGUDPR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PGUDPR to value 0x0140"]
impl crate::Resettable for PGUDPR_SPEC {
    const RESET_VALUE: Self::Ux = 0x0140;
}
