#[doc = "Register `SYRLIR` reader"]
pub struct R(crate::R<SYRLIR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYRLIR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SYRLIR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SYRLIR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `ANCE` reader - Announce Message logMessageInterval Field IndicationThese bits indicate the logMessageInterval field value of a received Announce message."]
pub type ANCE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SYNC` reader - Sync Message logMessageInterval Field IndicationThese bits indicate the logMessageInterval field value of a received Sync message."]
pub type SYNC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DRESP` reader - Delay_Resp Message logMessageInterval Field IndicationThese bits indicate the logMessageInterval field value of a received Delay_Resp message."]
pub type DRESP_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - Announce Message logMessageInterval Field IndicationThese bits indicate the logMessageInterval field value of a received Announce message."]
    #[inline(always)]
    pub fn ance(&self) -> ANCE_R {
        ANCE_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Sync Message logMessageInterval Field IndicationThese bits indicate the logMessageInterval field value of a received Sync message."]
    #[inline(always)]
    pub fn sync(&self) -> SYNC_R {
        SYNC_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Delay_Resp Message logMessageInterval Field IndicationThese bits indicate the logMessageInterval field value of a received Delay_Resp message."]
    #[inline(always)]
    pub fn dresp(&self) -> DRESP_R {
        DRESP_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
#[doc = "SYNFP Received logMessageInterval Value Indication Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [syrlir](index.html) module"]
pub struct SYRLIR_SPEC;
impl crate::RegisterSpec for SYRLIR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [syrlir::R](R) reader structure"]
impl crate::Readable for SYRLIR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SYRLIR to value 0"]
impl crate::Resettable for SYRLIR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
