#[doc = "Register `ADTRGDLR1` reader"]
pub struct R(crate::R<ADTRGDLR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADTRGDLR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADTRGDLR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADTRGDLR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADTRGDLR1` writer"]
pub struct W(crate::W<ADTRGDLR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADTRGDLR1_SPEC>;
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
impl From<crate::W<ADTRGDLR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADTRGDLR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TRGDLY2` reader - Scan Group 2 Trigger Input Delay Configuration"]
pub type TRGDLY2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TRGDLY2` writer - Scan Group 2 Trigger Input Delay Configuration"]
pub type TRGDLY2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ADTRGDLR1_SPEC, u8, u8, 8, O>;
#[doc = "Field `TRGDLY3` reader - Scan Group 3 Trigger Input Delay Configuration"]
pub type TRGDLY3_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TRGDLY3` writer - Scan Group 3 Trigger Input Delay Configuration"]
pub type TRGDLY3_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ADTRGDLR1_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Scan Group 2 Trigger Input Delay Configuration"]
    #[inline(always)]
    pub fn trgdly2(&self) -> TRGDLY2_R {
        TRGDLY2_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Scan Group 3 Trigger Input Delay Configuration"]
    #[inline(always)]
    pub fn trgdly3(&self) -> TRGDLY3_R {
        TRGDLY3_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Scan Group 2 Trigger Input Delay Configuration"]
    #[inline(always)]
    #[must_use]
    pub fn trgdly2(&mut self) -> TRGDLY2_W<0> {
        TRGDLY2_W::new(self)
    }
    #[doc = "Bits 16:23 - Scan Group 3 Trigger Input Delay Configuration"]
    #[inline(always)]
    #[must_use]
    pub fn trgdly3(&mut self) -> TRGDLY3_W<16> {
        TRGDLY3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "A/D Conversion Start Trigger Delay Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adtrgdlr1](index.html) module"]
pub struct ADTRGDLR1_SPEC;
impl crate::RegisterSpec for ADTRGDLR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adtrgdlr1::R](R) reader structure"]
impl crate::Readable for ADTRGDLR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adtrgdlr1::W](W) writer structure"]
impl crate::Writable for ADTRGDLR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADTRGDLR1 to value 0"]
impl crate::Resettable for ADTRGDLR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
