#[doc = "Register `ADTRGDLR2` reader"]
pub struct R(crate::R<ADTRGDLR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADTRGDLR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADTRGDLR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADTRGDLR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADTRGDLR2` writer"]
pub struct W(crate::W<ADTRGDLR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADTRGDLR2_SPEC>;
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
impl From<crate::W<ADTRGDLR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADTRGDLR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TRGDLY4` reader - Scan Group 4 Trigger Input Delay Configuration"]
pub type TRGDLY4_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TRGDLY4` writer - Scan Group 4 Trigger Input Delay Configuration"]
pub type TRGDLY4_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ADTRGDLR2_SPEC, u8, u8, 8, O>;
#[doc = "Field `TRGDLY5` reader - Scan Group 5 Trigger Input Delay Configuration"]
pub type TRGDLY5_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TRGDLY5` writer - Scan Group 5 Trigger Input Delay Configuration"]
pub type TRGDLY5_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ADTRGDLR2_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Scan Group 4 Trigger Input Delay Configuration"]
    #[inline(always)]
    pub fn trgdly4(&self) -> TRGDLY4_R {
        TRGDLY4_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Scan Group 5 Trigger Input Delay Configuration"]
    #[inline(always)]
    pub fn trgdly5(&self) -> TRGDLY5_R {
        TRGDLY5_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Scan Group 4 Trigger Input Delay Configuration"]
    #[inline(always)]
    #[must_use]
    pub fn trgdly4(&mut self) -> TRGDLY4_W<0> {
        TRGDLY4_W::new(self)
    }
    #[doc = "Bits 16:23 - Scan Group 5 Trigger Input Delay Configuration"]
    #[inline(always)]
    #[must_use]
    pub fn trgdly5(&mut self) -> TRGDLY5_W<16> {
        TRGDLY5_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "A/D Conversion Start Trigger Delay Register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adtrgdlr2](index.html) module"]
pub struct ADTRGDLR2_SPEC;
impl crate::RegisterSpec for ADTRGDLR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adtrgdlr2::R](R) reader structure"]
impl crate::Readable for ADTRGDLR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adtrgdlr2::W](W) writer structure"]
impl crate::Writable for ADTRGDLR2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADTRGDLR2 to value 0"]
impl crate::Resettable for ADTRGDLR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
