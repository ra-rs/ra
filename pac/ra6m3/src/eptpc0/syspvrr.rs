#[doc = "Register `SYSPVRR` reader"]
pub struct R(crate::R<SYSPVRR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYSPVRR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SYSPVRR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SYSPVRR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SYSPVRR` writer"]
pub struct W(crate::W<SYSPVRR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SYSPVRR_SPEC>;
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
impl From<crate::W<SYSPVRR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SYSPVRR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `VER` reader - versionPTP Field ValueThese bits are used to set the versionPTP field value of the PTP v2 header.When a message is received, this value is compared with the versionPTP field of the received frame.In generating messages, the value is used for the versionPTP field of the frame for transmission.Set these bits to 0010b (PTP v2)."]
pub type VER_R = crate::FieldReader<u8, u8>;
#[doc = "Field `VER` writer - versionPTP Field ValueThese bits are used to set the versionPTP field value of the PTP v2 header.When a message is received, this value is compared with the versionPTP field of the received frame.In generating messages, the value is used for the versionPTP field of the frame for transmission.Set these bits to 0010b (PTP v2)."]
pub type VER_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SYSPVRR_SPEC, u8, u8, 4, O>;
#[doc = "Field `TRSP` reader - transportSpecific Field ValueThese bits are used to set the transportSpecific field value of the PTP v2 header.When a message is received, this value is compared with the transportSpecific field of the received frame.In generating messages, the value is used for the transportSpecific field of the frame for transmission.Set these bits to 0000b (IEEE 1588)."]
pub type TRSP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TRSP` writer - transportSpecific Field ValueThese bits are used to set the transportSpecific field value of the PTP v2 header.When a message is received, this value is compared with the transportSpecific field of the received frame.In generating messages, the value is used for the transportSpecific field of the frame for transmission.Set these bits to 0000b (IEEE 1588)."]
pub type TRSP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SYSPVRR_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 0:3 - versionPTP Field ValueThese bits are used to set the versionPTP field value of the PTP v2 header.When a message is received, this value is compared with the versionPTP field of the received frame.In generating messages, the value is used for the versionPTP field of the frame for transmission.Set these bits to 0010b (PTP v2)."]
    #[inline(always)]
    pub fn ver(&self) -> VER_R {
        VER_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - transportSpecific Field ValueThese bits are used to set the transportSpecific field value of the PTP v2 header.When a message is received, this value is compared with the transportSpecific field of the received frame.In generating messages, the value is used for the transportSpecific field of the frame for transmission.Set these bits to 0000b (IEEE 1588)."]
    #[inline(always)]
    pub fn trsp(&self) -> TRSP_R {
        TRSP_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - versionPTP Field ValueThese bits are used to set the versionPTP field value of the PTP v2 header.When a message is received, this value is compared with the versionPTP field of the received frame.In generating messages, the value is used for the versionPTP field of the frame for transmission.Set these bits to 0010b (PTP v2)."]
    #[inline(always)]
    #[must_use]
    pub fn ver(&mut self) -> VER_W<0> {
        VER_W::new(self)
    }
    #[doc = "Bits 4:7 - transportSpecific Field ValueThese bits are used to set the transportSpecific field value of the PTP v2 header.When a message is received, this value is compared with the transportSpecific field of the received frame.In generating messages, the value is used for the transportSpecific field of the frame for transmission.Set these bits to 0000b (IEEE 1588)."]
    #[inline(always)]
    #[must_use]
    pub fn trsp(&mut self) -> TRSP_W<4> {
        TRSP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SYNFP Specification Version Setting Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [syspvrr](index.html) module"]
pub struct SYSPVRR_SPEC;
impl crate::RegisterSpec for SYSPVRR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [syspvrr::R](R) reader structure"]
impl crate::Readable for SYSPVRR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [syspvrr::W](W) writer structure"]
impl crate::Writable for SYSPVRR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SYSPVRR to value 0x02"]
impl crate::Resettable for SYSPVRR_SPEC {
    const RESET_VALUE: Self::Ux = 0x02;
}
