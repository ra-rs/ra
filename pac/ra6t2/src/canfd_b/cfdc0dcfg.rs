#[doc = "Register `CFDC0DCFG` reader"]
pub struct R(crate::R<CFDC0DCFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFDC0DCFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFDC0DCFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFDC0DCFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CFDC0DCFG` writer"]
pub struct W(crate::W<CFDC0DCFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFDC0DCFG_SPEC>;
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
impl From<crate::W<CFDC0DCFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFDC0DCFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DBRP` reader - Channel Data Baud Rate Prescaler"]
pub type DBRP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DBRP` writer - Channel Data Baud Rate Prescaler"]
pub type DBRP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFDC0DCFG_SPEC, u8, u8, 8, O>;
#[doc = "Field `DTSEG1` reader - Timing Segment 1"]
pub type DTSEG1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DTSEG1` writer - Timing Segment 1"]
pub type DTSEG1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFDC0DCFG_SPEC, u8, u8, 5, O>;
#[doc = "Field `DTSEG2` reader - Timing Segment 2"]
pub type DTSEG2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DTSEG2` writer - Timing Segment 2"]
pub type DTSEG2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFDC0DCFG_SPEC, u8, u8, 4, O>;
#[doc = "Field `DSJW` reader - Resynchronization Jump Width"]
pub type DSJW_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DSJW` writer - Resynchronization Jump Width"]
pub type DSJW_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFDC0DCFG_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 0:7 - Channel Data Baud Rate Prescaler"]
    #[inline(always)]
    pub fn dbrp(&self) -> DBRP_R {
        DBRP_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:12 - Timing Segment 1"]
    #[inline(always)]
    pub fn dtseg1(&self) -> DTSEG1_R {
        DTSEG1_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:19 - Timing Segment 2"]
    #[inline(always)]
    pub fn dtseg2(&self) -> DTSEG2_R {
        DTSEG2_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - Resynchronization Jump Width"]
    #[inline(always)]
    pub fn dsjw(&self) -> DSJW_R {
        DSJW_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Channel Data Baud Rate Prescaler"]
    #[inline(always)]
    #[must_use]
    pub fn dbrp(&mut self) -> DBRP_W<0> {
        DBRP_W::new(self)
    }
    #[doc = "Bits 8:12 - Timing Segment 1"]
    #[inline(always)]
    #[must_use]
    pub fn dtseg1(&mut self) -> DTSEG1_W<8> {
        DTSEG1_W::new(self)
    }
    #[doc = "Bits 16:19 - Timing Segment 2"]
    #[inline(always)]
    #[must_use]
    pub fn dtseg2(&mut self) -> DTSEG2_W<16> {
        DTSEG2_W::new(self)
    }
    #[doc = "Bits 24:27 - Resynchronization Jump Width"]
    #[inline(always)]
    #[must_use]
    pub fn dsjw(&mut self) -> DSJW_W<24> {
        DSJW_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Channel 0 Data Bitrate Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfdc0dcfg](index.html) module"]
pub struct CFDC0DCFG_SPEC;
impl crate::RegisterSpec for CFDC0DCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cfdc0dcfg::R](R) reader structure"]
impl crate::Readable for CFDC0DCFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cfdc0dcfg::W](W) writer structure"]
impl crate::Writable for CFDC0DCFG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CFDC0DCFG to value 0"]
impl crate::Resettable for CFDC0DCFG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
