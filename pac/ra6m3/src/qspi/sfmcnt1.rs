#[doc = "Register `SFMCNT1` reader"]
pub struct R(crate::R<SFMCNT1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SFMCNT1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SFMCNT1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SFMCNT1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SFMCNT1` writer"]
pub struct W(crate::W<SFMCNT1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SFMCNT1_SPEC>;
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
impl From<crate::W<SFMCNT1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SFMCNT1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `QSPI_EXT` reader - BANK Switching AddressWhen accessing from 0x6000_0000 to 0x63FF_FFFF, Addres bus is Set QSPI_EXT\\[5:0\\]
to high-order 6bits of SHADDR\\[31:0\\]NOTE: Setting 6'h3F is prihibited."]
pub type QSPI_EXT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `QSPI_EXT` writer - BANK Switching AddressWhen accessing from 0x6000_0000 to 0x63FF_FFFF, Addres bus is Set QSPI_EXT\\[5:0\\]
to high-order 6bits of SHADDR\\[31:0\\]NOTE: Setting 6'h3F is prihibited."]
pub type QSPI_EXT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SFMCNT1_SPEC, u8, u8, 6, O>;
impl R {
    #[doc = "Bits 26:31 - BANK Switching AddressWhen accessing from 0x6000_0000 to 0x63FF_FFFF, Addres bus is Set QSPI_EXT\\[5:0\\]
to high-order 6bits of SHADDR\\[31:0\\]NOTE: Setting 6'h3F is prihibited."]
    #[inline(always)]
    pub fn qspi_ext(&self) -> QSPI_EXT_R {
        QSPI_EXT_R::new(((self.bits >> 26) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 26:31 - BANK Switching AddressWhen accessing from 0x6000_0000 to 0x63FF_FFFF, Addres bus is Set QSPI_EXT\\[5:0\\]
to high-order 6bits of SHADDR\\[31:0\\]NOTE: Setting 6'h3F is prihibited."]
    #[inline(always)]
    #[must_use]
    pub fn qspi_ext(&mut self) -> QSPI_EXT_W<26> {
        QSPI_EXT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "External QSPI Address Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sfmcnt1](index.html) module"]
pub struct SFMCNT1_SPEC;
impl crate::RegisterSpec for SFMCNT1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sfmcnt1::R](R) reader structure"]
impl crate::Readable for SFMCNT1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sfmcnt1::W](W) writer structure"]
impl crate::Writable for SFMCNT1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SFMCNT1 to value 0"]
impl crate::Resettable for SFMCNT1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
