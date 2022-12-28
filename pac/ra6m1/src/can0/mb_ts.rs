#[doc = "Register `MB%s_TS` reader"]
pub struct R(crate::R<MB_TS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MB_TS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MB_TS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MB_TS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MB%s_TS` writer"]
pub struct W(crate::W<MB_TS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MB_TS_SPEC>;
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
impl From<crate::W<MB_TS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MB_TS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TSL` reader - Time Stamp Higher ByteBits TSL\\[7:0\\]
store the counter value of the time stamp when received messages are stored in the mailbox."]
pub type TSL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TSL` writer - Time Stamp Higher ByteBits TSL\\[7:0\\]
store the counter value of the time stamp when received messages are stored in the mailbox."]
pub type TSL_W<'a, const O: u8> = crate::FieldWriter<'a, u16, MB_TS_SPEC, u8, u8, 8, O>;
#[doc = "Field `TSH` reader - Time Stamp Lower ByteBits TSH\\[7:0\\]
store the counter value of the time stamp when received messages are stored in the mailbox."]
pub type TSH_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TSH` writer - Time Stamp Lower ByteBits TSH\\[7:0\\]
store the counter value of the time stamp when received messages are stored in the mailbox."]
pub type TSH_W<'a, const O: u8> = crate::FieldWriter<'a, u16, MB_TS_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Time Stamp Higher ByteBits TSL\\[7:0\\]
store the counter value of the time stamp when received messages are stored in the mailbox."]
    #[inline(always)]
    pub fn tsl(&self) -> TSL_R {
        TSL_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Time Stamp Lower ByteBits TSH\\[7:0\\]
store the counter value of the time stamp when received messages are stored in the mailbox."]
    #[inline(always)]
    pub fn tsh(&self) -> TSH_R {
        TSH_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Time Stamp Higher ByteBits TSL\\[7:0\\]
store the counter value of the time stamp when received messages are stored in the mailbox."]
    #[inline(always)]
    #[must_use]
    pub fn tsl(&mut self) -> TSL_W<0> {
        TSL_W::new(self)
    }
    #[doc = "Bits 8:15 - Time Stamp Lower ByteBits TSH\\[7:0\\]
store the counter value of the time stamp when received messages are stored in the mailbox."]
    #[inline(always)]
    #[must_use]
    pub fn tsh(&mut self) -> TSH_W<8> {
        TSH_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Mailbox Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mb_ts](index.html) module"]
pub struct MB_TS_SPEC;
impl crate::RegisterSpec for MB_TS_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [mb_ts::R](R) reader structure"]
impl crate::Readable for MB_TS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mb_ts::W](W) writer structure"]
impl crate::Writable for MB_TS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MB%s_TS to value 0"]
impl crate::Resettable for MB_TS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
