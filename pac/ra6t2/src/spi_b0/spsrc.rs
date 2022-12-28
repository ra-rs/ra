#[doc = "Register `SPSRC` reader"]
pub struct R(crate::R<SPSRC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPSRC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SPSRC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SPSRC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SPSRC` writer"]
pub struct W(crate::W<SPSRC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SPSRC_SPEC>;
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
impl From<crate::W<SPSRC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SPSRC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SPDRFC` writer - SPI Receive Data Ready Flag Clear"]
pub type SPDRFC_W<'a, const O: u8> = crate::BitWriter<'a, u32, SPSRC_SPEC, bool, O>;
#[doc = "Field `OVRFC` writer - Overrun Error Flag Clear"]
pub type OVRFC_W<'a, const O: u8> = crate::BitWriter<'a, u32, SPSRC_SPEC, bool, O>;
#[doc = "Field `MODFC` writer - Mode Fault Error Flag Clear"]
pub type MODFC_W<'a, const O: u8> = crate::BitWriter<'a, u32, SPSRC_SPEC, bool, O>;
#[doc = "Field `PERFC` writer - Parity Error Flag Clear"]
pub type PERFC_W<'a, const O: u8> = crate::BitWriter<'a, u32, SPSRC_SPEC, bool, O>;
#[doc = "Field `UDRFC` writer - Underrun Error Flag Clear"]
pub type UDRFC_W<'a, const O: u8> = crate::BitWriter<'a, u32, SPSRC_SPEC, bool, O>;
#[doc = "Field `SPTEFC` writer - SPI Transmit Buffer Empty Flag Clear"]
pub type SPTEFC_W<'a, const O: u8> = crate::BitWriter<'a, u32, SPSRC_SPEC, bool, O>;
#[doc = "Field `CENDFC` writer - Communication End Flag Clear"]
pub type CENDFC_W<'a, const O: u8> = crate::BitWriter<'a, u32, SPSRC_SPEC, bool, O>;
#[doc = "Field `SPRFC` writer - SPI Receive Buffer Full Flag Clear"]
pub type SPRFC_W<'a, const O: u8> = crate::BitWriter<'a, u32, SPSRC_SPEC, bool, O>;
impl W {
    #[doc = "Bit 23 - SPI Receive Data Ready Flag Clear"]
    #[inline(always)]
    #[must_use]
    pub fn spdrfc(&mut self) -> SPDRFC_W<23> {
        SPDRFC_W::new(self)
    }
    #[doc = "Bit 24 - Overrun Error Flag Clear"]
    #[inline(always)]
    #[must_use]
    pub fn ovrfc(&mut self) -> OVRFC_W<24> {
        OVRFC_W::new(self)
    }
    #[doc = "Bit 26 - Mode Fault Error Flag Clear"]
    #[inline(always)]
    #[must_use]
    pub fn modfc(&mut self) -> MODFC_W<26> {
        MODFC_W::new(self)
    }
    #[doc = "Bit 27 - Parity Error Flag Clear"]
    #[inline(always)]
    #[must_use]
    pub fn perfc(&mut self) -> PERFC_W<27> {
        PERFC_W::new(self)
    }
    #[doc = "Bit 28 - Underrun Error Flag Clear"]
    #[inline(always)]
    #[must_use]
    pub fn udrfc(&mut self) -> UDRFC_W<28> {
        UDRFC_W::new(self)
    }
    #[doc = "Bit 29 - SPI Transmit Buffer Empty Flag Clear"]
    #[inline(always)]
    #[must_use]
    pub fn sptefc(&mut self) -> SPTEFC_W<29> {
        SPTEFC_W::new(self)
    }
    #[doc = "Bit 30 - Communication End Flag Clear"]
    #[inline(always)]
    #[must_use]
    pub fn cendfc(&mut self) -> CENDFC_W<30> {
        CENDFC_W::new(self)
    }
    #[doc = "Bit 31 - SPI Receive Buffer Full Flag Clear"]
    #[inline(always)]
    #[must_use]
    pub fn sprfc(&mut self) -> SPRFC_W<31> {
        SPRFC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPI Status Clear Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spsrc](index.html) module"]
pub struct SPSRC_SPEC;
impl crate::RegisterSpec for SPSRC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [spsrc::R](R) reader structure"]
impl crate::Readable for SPSRC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [spsrc::W](W) writer structure"]
impl crate::Writable for SPSRC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SPSRC to value 0"]
impl crate::Resettable for SPSRC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
