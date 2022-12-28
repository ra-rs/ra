#[doc = "Register `ADTRGDLR0` reader"]
pub struct R(crate::R<ADTRGDLR0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADTRGDLR0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADTRGDLR0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADTRGDLR0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADTRGDLR0` writer"]
pub struct W(crate::W<ADTRGDLR0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADTRGDLR0_SPEC>;
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
impl From<crate::W<ADTRGDLR0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADTRGDLR0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TRGDLY0` reader - Scan Group 0 Trigger Input Delay Configuration"]
pub type TRGDLY0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TRGDLY0` writer - Scan Group 0 Trigger Input Delay Configuration"]
pub type TRGDLY0_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ADTRGDLR0_SPEC, u8, u8, 8, O>;
#[doc = "Field `TRGDLY1` reader - Scan Group 1 Trigger Input Delay Configuration"]
pub type TRGDLY1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TRGDLY1` writer - Scan Group 1 Trigger Input Delay Configuration"]
pub type TRGDLY1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ADTRGDLR0_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Scan Group 0 Trigger Input Delay Configuration"]
    #[inline(always)]
    pub fn trgdly0(&self) -> TRGDLY0_R {
        TRGDLY0_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Scan Group 1 Trigger Input Delay Configuration"]
    #[inline(always)]
    pub fn trgdly1(&self) -> TRGDLY1_R {
        TRGDLY1_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Scan Group 0 Trigger Input Delay Configuration"]
    #[inline(always)]
    #[must_use]
    pub fn trgdly0(&mut self) -> TRGDLY0_W<0> {
        TRGDLY0_W::new(self)
    }
    #[doc = "Bits 16:23 - Scan Group 1 Trigger Input Delay Configuration"]
    #[inline(always)]
    #[must_use]
    pub fn trgdly1(&mut self) -> TRGDLY1_W<16> {
        TRGDLY1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "A/D Conversion Start Trigger Delay Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adtrgdlr0](index.html) module"]
pub struct ADTRGDLR0_SPEC;
impl crate::RegisterSpec for ADTRGDLR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adtrgdlr0::R](R) reader structure"]
impl crate::Readable for ADTRGDLR0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adtrgdlr0::W](W) writer structure"]
impl crate::Writable for ADTRGDLR0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADTRGDLR0 to value 0"]
impl crate::Resettable for ADTRGDLR0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
