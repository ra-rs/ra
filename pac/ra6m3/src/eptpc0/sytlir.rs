#[doc = "Register `SYTLIR` reader"]
pub struct R(crate::R<SYTLIR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYTLIR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SYTLIR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SYTLIR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SYTLIR` writer"]
pub struct W(crate::W<SYTLIR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SYTLIR_SPEC>;
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
impl From<crate::W<SYTLIR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SYTLIR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ANCE` reader - Announce Message Transmission Interval SettingThese bits set the interval for the transmission of Announce messages."]
pub type ANCE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ANCE` writer - Announce Message Transmission Interval SettingThese bits set the interval for the transmission of Announce messages."]
pub type ANCE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SYTLIR_SPEC, u8, u8, 8, O>;
#[doc = "Field `SYNC` reader - Sync Message Transmission Interval SettingThese bits set the interval for the transmission of Sync messages. The setting is also placed in the logMessageInterval field of transmitted Sync messages."]
pub type SYNC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SYNC` writer - Sync Message Transmission Interval SettingThese bits set the interval for the transmission of Sync messages. The setting is also placed in the logMessageInterval field of transmitted Sync messages."]
pub type SYNC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SYTLIR_SPEC, u8, u8, 8, O>;
#[doc = "Field `DREQ` reader - Delay_Req Transmission Interval Average Value/ Pdelay_Req Transmission Interval SettingThe bits set the average interval for the transmission of Delay_Req messages and the interval for the transmission of Pdelay_Req messages.The setting is also placed in the logMessageInterval field of Delay_Resp messages."]
pub type DREQ_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DREQ` writer - Delay_Req Transmission Interval Average Value/ Pdelay_Req Transmission Interval SettingThe bits set the average interval for the transmission of Delay_Req messages and the interval for the transmission of Pdelay_Req messages.The setting is also placed in the logMessageInterval field of Delay_Resp messages."]
pub type DREQ_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SYTLIR_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Announce Message Transmission Interval SettingThese bits set the interval for the transmission of Announce messages."]
    #[inline(always)]
    pub fn ance(&self) -> ANCE_R {
        ANCE_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Sync Message Transmission Interval SettingThese bits set the interval for the transmission of Sync messages. The setting is also placed in the logMessageInterval field of transmitted Sync messages."]
    #[inline(always)]
    pub fn sync(&self) -> SYNC_R {
        SYNC_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Delay_Req Transmission Interval Average Value/ Pdelay_Req Transmission Interval SettingThe bits set the average interval for the transmission of Delay_Req messages and the interval for the transmission of Pdelay_Req messages.The setting is also placed in the logMessageInterval field of Delay_Resp messages."]
    #[inline(always)]
    pub fn dreq(&self) -> DREQ_R {
        DREQ_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Announce Message Transmission Interval SettingThese bits set the interval for the transmission of Announce messages."]
    #[inline(always)]
    #[must_use]
    pub fn ance(&mut self) -> ANCE_W<0> {
        ANCE_W::new(self)
    }
    #[doc = "Bits 8:15 - Sync Message Transmission Interval SettingThese bits set the interval for the transmission of Sync messages. The setting is also placed in the logMessageInterval field of transmitted Sync messages."]
    #[inline(always)]
    #[must_use]
    pub fn sync(&mut self) -> SYNC_W<8> {
        SYNC_W::new(self)
    }
    #[doc = "Bits 16:23 - Delay_Req Transmission Interval Average Value/ Pdelay_Req Transmission Interval SettingThe bits set the average interval for the transmission of Delay_Req messages and the interval for the transmission of Pdelay_Req messages.The setting is also placed in the logMessageInterval field of Delay_Resp messages."]
    #[inline(always)]
    #[must_use]
    pub fn dreq(&mut self) -> DREQ_W<16> {
        DREQ_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SYNFP Transmission Interval Setting Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sytlir](index.html) module"]
pub struct SYTLIR_SPEC;
impl crate::RegisterSpec for SYTLIR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sytlir::R](R) reader structure"]
impl crate::Readable for SYTLIR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sytlir::W](W) writer structure"]
impl crate::Writable for SYTLIR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SYTLIR to value 0x01"]
impl crate::Resettable for SYTLIR_SPEC {
    const RESET_VALUE: Self::Ux = 0x01;
}
