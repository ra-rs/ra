#[doc = "Register `CFDC0NCFG` reader"]
pub struct R(crate::R<CFDC0NCFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFDC0NCFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFDC0NCFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFDC0NCFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CFDC0NCFG` writer"]
pub struct W(crate::W<CFDC0NCFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFDC0NCFG_SPEC>;
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
impl From<crate::W<CFDC0NCFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFDC0NCFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `NBRP` reader - Channel Nominal Baud Rate Prescaler"]
pub type NBRP_R = crate::FieldReader<u16, u16>;
#[doc = "Field `NBRP` writer - Channel Nominal Baud Rate Prescaler"]
pub type NBRP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFDC0NCFG_SPEC, u16, u16, 10, O>;
#[doc = "Field `NSJW` reader - Resynchronization Jump Width"]
pub type NSJW_R = crate::FieldReader<u8, u8>;
#[doc = "Field `NSJW` writer - Resynchronization Jump Width"]
pub type NSJW_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFDC0NCFG_SPEC, u8, u8, 7, O>;
#[doc = "Field `NTSEG1` reader - Timing Segment 1"]
pub type NTSEG1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `NTSEG1` writer - Timing Segment 1"]
pub type NTSEG1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFDC0NCFG_SPEC, u8, u8, 8, O>;
#[doc = "Field `NTSEG2` reader - Timing Segment 2"]
pub type NTSEG2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `NTSEG2` writer - Timing Segment 2"]
pub type NTSEG2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFDC0NCFG_SPEC, u8, u8, 7, O>;
impl R {
    #[doc = "Bits 0:9 - Channel Nominal Baud Rate Prescaler"]
    #[inline(always)]
    pub fn nbrp(&self) -> NBRP_R {
        NBRP_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 10:16 - Resynchronization Jump Width"]
    #[inline(always)]
    pub fn nsjw(&self) -> NSJW_R {
        NSJW_R::new(((self.bits >> 10) & 0x7f) as u8)
    }
    #[doc = "Bits 17:24 - Timing Segment 1"]
    #[inline(always)]
    pub fn ntseg1(&self) -> NTSEG1_R {
        NTSEG1_R::new(((self.bits >> 17) & 0xff) as u8)
    }
    #[doc = "Bits 25:31 - Timing Segment 2"]
    #[inline(always)]
    pub fn ntseg2(&self) -> NTSEG2_R {
        NTSEG2_R::new(((self.bits >> 25) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:9 - Channel Nominal Baud Rate Prescaler"]
    #[inline(always)]
    #[must_use]
    pub fn nbrp(&mut self) -> NBRP_W<0> {
        NBRP_W::new(self)
    }
    #[doc = "Bits 10:16 - Resynchronization Jump Width"]
    #[inline(always)]
    #[must_use]
    pub fn nsjw(&mut self) -> NSJW_W<10> {
        NSJW_W::new(self)
    }
    #[doc = "Bits 17:24 - Timing Segment 1"]
    #[inline(always)]
    #[must_use]
    pub fn ntseg1(&mut self) -> NTSEG1_W<17> {
        NTSEG1_W::new(self)
    }
    #[doc = "Bits 25:31 - Timing Segment 2"]
    #[inline(always)]
    #[must_use]
    pub fn ntseg2(&mut self) -> NTSEG2_W<25> {
        NTSEG2_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Channel 0 Nominal Bitrate Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfdc0ncfg](index.html) module"]
pub struct CFDC0NCFG_SPEC;
impl crate::RegisterSpec for CFDC0NCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cfdc0ncfg::R](R) reader structure"]
impl crate::Readable for CFDC0NCFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cfdc0ncfg::W](W) writer structure"]
impl crate::Writable for CFDC0NCFG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CFDC0NCFG to value 0"]
impl crate::Resettable for CFDC0NCFG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
