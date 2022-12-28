#[doc = "Register `CFDC0FDCRC` reader"]
pub struct R(crate::R<CFDC0FDCRC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFDC0FDCRC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFDC0FDCRC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFDC0FDCRC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CFDC0FDCRC` writer"]
pub struct W(crate::W<CFDC0FDCRC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFDC0FDCRC_SPEC>;
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
impl From<crate::W<CFDC0FDCRC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFDC0FDCRC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CRCREG` reader - CRC Register value"]
pub type CRCREG_R = crate::FieldReader<u32, u32>;
#[doc = "Field `SCNT` reader - Stuff bit count"]
pub type SCNT_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:20 - CRC Register value"]
    #[inline(always)]
    pub fn crcreg(&self) -> CRCREG_R {
        CRCREG_R::new(self.bits & 0x001f_ffff)
    }
    #[doc = "Bits 24:27 - Stuff bit count"]
    #[inline(always)]
    pub fn scnt(&self) -> SCNT_R {
        SCNT_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Channel 0 CANFD CRC Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfdc0fdcrc](index.html) module"]
pub struct CFDC0FDCRC_SPEC;
impl crate::RegisterSpec for CFDC0FDCRC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cfdc0fdcrc::R](R) reader structure"]
impl crate::Readable for CFDC0FDCRC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cfdc0fdcrc::W](W) writer structure"]
impl crate::Writable for CFDC0FDCRC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CFDC0FDCRC to value 0"]
impl crate::Resettable for CFDC0FDCRC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
