#[doc = "Register `JCCMD` writer"]
pub struct W(crate::W<JCCMD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<JCCMD_SPEC>;
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
impl From<crate::W<JCCMD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<JCCMD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "JPEG Core Process Start CommandTo start JPEG core processing, set this bit to 1. Do not write this bit to 1 again while this module is in operation.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JSRT_AW {
    #[doc = "0: No effect."]
    _0 = 0,
    #[doc = "1: Start JPEG core processing"]
    _1 = 1,
}
impl From<JSRT_AW> for bool {
    #[inline(always)]
    fn from(variant: JSRT_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `JSRT` writer - JPEG Core Process Start CommandTo start JPEG core processing, set this bit to 1. Do not write this bit to 1 again while this module is in operation."]
pub type JSRT_W<'a, const O: u8> = crate::BitWriter<'a, u8, JCCMD_SPEC, JSRT_AW, O>;
impl<'a, const O: u8> JSRT_W<'a, O> {
    #[doc = "No effect."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(JSRT_AW::_0)
    }
    #[doc = "Start JPEG core processing"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(JSRT_AW::_1)
    }
}
#[doc = "JPEG Core Process Stop Clear CommandTo clear the process-stopped state caused by requests to read the image size and pixel format (enabled by the INT3 bit in JINTE0), set this bit to 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JRST_AW {
    #[doc = "0: No effect."]
    _0 = 0,
    #[doc = "1: Clear the process-stopped state caused by requests to read the image size and pixel format(enabled by the INT3 bit in JINTE0)."]
    _1 = 1,
}
impl From<JRST_AW> for bool {
    #[inline(always)]
    fn from(variant: JRST_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `JRST` writer - JPEG Core Process Stop Clear CommandTo clear the process-stopped state caused by requests to read the image size and pixel format (enabled by the INT3 bit in JINTE0), set this bit to 1."]
pub type JRST_W<'a, const O: u8> = crate::BitWriter<'a, u8, JCCMD_SPEC, JRST_AW, O>;
impl<'a, const O: u8> JRST_W<'a, O> {
    #[doc = "No effect."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(JRST_AW::_0)
    }
    #[doc = "Clear the process-stopped state caused by requests to read the image size and pixel format(enabled by the INT3 bit in JINTE0)."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(JRST_AW::_1)
    }
}
#[doc = "Interrupt Request Clear Command This bit is valid only for the interrupt sources corresponding to bits INS6, INS5, and INS3 in JINTS0. To clear an interrupt request, set this bit to 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JEND_AW {
    #[doc = "0: No effect."]
    _0 = 0,
    #[doc = "1: Clear all bits in JINTE0."]
    _1 = 1,
}
impl From<JEND_AW> for bool {
    #[inline(always)]
    fn from(variant: JEND_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `JEND` writer - Interrupt Request Clear Command This bit is valid only for the interrupt sources corresponding to bits INS6, INS5, and INS3 in JINTS0. To clear an interrupt request, set this bit to 1"]
pub type JEND_W<'a, const O: u8> = crate::BitWriter<'a, u8, JCCMD_SPEC, JEND_AW, O>;
impl<'a, const O: u8> JEND_W<'a, O> {
    #[doc = "No effect."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(JEND_AW::_0)
    }
    #[doc = "Clear all bits in JINTE0."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(JEND_AW::_1)
    }
}
#[doc = "Bus Reset. NOTE: When this module is in operation, the bus reset command should not be issued.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BRST_AW {
    #[doc = "0: No effect."]
    _0 = 0,
    #[doc = "1: Resets the JCDTCU, JCDTCM, JCDTCD, JCDERR and JCRST registers."]
    _1 = 1,
}
impl From<BRST_AW> for bool {
    #[inline(always)]
    fn from(variant: BRST_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BRST` writer - Bus Reset. NOTE: When this module is in operation, the bus reset command should not be issued."]
pub type BRST_W<'a, const O: u8> = crate::BitWriter<'a, u8, JCCMD_SPEC, BRST_AW, O>;
impl<'a, const O: u8> BRST_W<'a, O> {
    #[doc = "No effect."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BRST_AW::_0)
    }
    #[doc = "Resets the JCDTCU, JCDTCM, JCDTCD, JCDERR and JCRST registers."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BRST_AW::_1)
    }
}
impl W {
    #[doc = "Bit 0 - JPEG Core Process Start CommandTo start JPEG core processing, set this bit to 1. Do not write this bit to 1 again while this module is in operation."]
    #[inline(always)]
    #[must_use]
    pub fn jsrt(&mut self) -> JSRT_W<0> {
        JSRT_W::new(self)
    }
    #[doc = "Bit 1 - JPEG Core Process Stop Clear CommandTo clear the process-stopped state caused by requests to read the image size and pixel format (enabled by the INT3 bit in JINTE0), set this bit to 1."]
    #[inline(always)]
    #[must_use]
    pub fn jrst(&mut self) -> JRST_W<1> {
        JRST_W::new(self)
    }
    #[doc = "Bit 2 - Interrupt Request Clear Command This bit is valid only for the interrupt sources corresponding to bits INS6, INS5, and INS3 in JINTS0. To clear an interrupt request, set this bit to 1"]
    #[inline(always)]
    #[must_use]
    pub fn jend(&mut self) -> JEND_W<2> {
        JEND_W::new(self)
    }
    #[doc = "Bit 7 - Bus Reset. NOTE: When this module is in operation, the bus reset command should not be issued."]
    #[inline(always)]
    #[must_use]
    pub fn brst(&mut self) -> BRST_W<7> {
        BRST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "JPEG Code Command Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [jccmd](index.html) module"]
pub struct JCCMD_SPEC;
impl crate::RegisterSpec for JCCMD_SPEC {
    type Ux = u8;
}
#[doc = "`write(|w| ..)` method takes [jccmd::W](W) writer structure"]
impl crate::Writable for JCCMD_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets JCCMD to value 0"]
impl crate::Resettable for JCCMD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
