#[doc = "Register `ADTRGDLR4` reader"]
pub struct R(crate::R<ADTRGDLR4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADTRGDLR4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADTRGDLR4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADTRGDLR4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADTRGDLR4` writer"]
pub struct W(crate::W<ADTRGDLR4_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADTRGDLR4_SPEC>;
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
impl From<crate::W<ADTRGDLR4_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADTRGDLR4_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TRGDLY8` reader - Scan Group 8 Trigger Input Delay Configuration"]
pub type TRGDLY8_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TRGDLY8` writer - Scan Group 8 Trigger Input Delay Configuration"]
pub type TRGDLY8_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ADTRGDLR4_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Scan Group 8 Trigger Input Delay Configuration"]
    #[inline(always)]
    pub fn trgdly8(&self) -> TRGDLY8_R {
        TRGDLY8_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Scan Group 8 Trigger Input Delay Configuration"]
    #[inline(always)]
    #[must_use]
    pub fn trgdly8(&mut self) -> TRGDLY8_W<0> {
        TRGDLY8_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "A/D Conversion Start Trigger Delay Register 4\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adtrgdlr4](index.html) module"]
pub struct ADTRGDLR4_SPEC;
impl crate::RegisterSpec for ADTRGDLR4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adtrgdlr4::R](R) reader structure"]
impl crate::Readable for ADTRGDLR4_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adtrgdlr4::W](W) writer structure"]
impl crate::Writable for ADTRGDLR4_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADTRGDLR4 to value 0"]
impl crate::Resettable for ADTRGDLR4_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
