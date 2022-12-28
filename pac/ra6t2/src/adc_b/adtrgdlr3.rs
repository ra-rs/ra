#[doc = "Register `ADTRGDLR3` reader"]
pub struct R(crate::R<ADTRGDLR3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADTRGDLR3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADTRGDLR3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADTRGDLR3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADTRGDLR3` writer"]
pub struct W(crate::W<ADTRGDLR3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADTRGDLR3_SPEC>;
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
impl From<crate::W<ADTRGDLR3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADTRGDLR3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TRGDLY6` reader - Scan Group 6 Trigger Input Delay Configuration"]
pub type TRGDLY6_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TRGDLY6` writer - Scan Group 6 Trigger Input Delay Configuration"]
pub type TRGDLY6_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ADTRGDLR3_SPEC, u8, u8, 8, O>;
#[doc = "Field `TRGDLY7` reader - Scan Group 7 Trigger Input Delay Configuration"]
pub type TRGDLY7_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TRGDLY7` writer - Scan Group 7 Trigger Input Delay Configuration"]
pub type TRGDLY7_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ADTRGDLR3_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Scan Group 6 Trigger Input Delay Configuration"]
    #[inline(always)]
    pub fn trgdly6(&self) -> TRGDLY6_R {
        TRGDLY6_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Scan Group 7 Trigger Input Delay Configuration"]
    #[inline(always)]
    pub fn trgdly7(&self) -> TRGDLY7_R {
        TRGDLY7_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Scan Group 6 Trigger Input Delay Configuration"]
    #[inline(always)]
    #[must_use]
    pub fn trgdly6(&mut self) -> TRGDLY6_W<0> {
        TRGDLY6_W::new(self)
    }
    #[doc = "Bits 16:23 - Scan Group 7 Trigger Input Delay Configuration"]
    #[inline(always)]
    #[must_use]
    pub fn trgdly7(&mut self) -> TRGDLY7_W<16> {
        TRGDLY7_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "A/D Conversion Start Trigger Delay Register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adtrgdlr3](index.html) module"]
pub struct ADTRGDLR3_SPEC;
impl crate::RegisterSpec for ADTRGDLR3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adtrgdlr3::R](R) reader structure"]
impl crate::Readable for ADTRGDLR3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adtrgdlr3::W](W) writer structure"]
impl crate::Writable for ADTRGDLR3_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADTRGDLR3 to value 0"]
impl crate::Resettable for ADTRGDLR3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
