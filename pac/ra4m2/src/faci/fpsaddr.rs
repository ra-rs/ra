#[doc = "Register `FPSADDR` reader"]
pub struct R(crate::R<FPSADDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FPSADDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FPSADDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FPSADDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FPSADDR` writer"]
pub struct W(crate::W<FPSADDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FPSADDR_SPEC>;
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
impl From<crate::W<FPSADDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FPSADDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PSADR` reader - Programmed Area Start Address"]
pub type PSADR_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:16 - Programmed Area Start Address"]
    #[inline(always)]
    pub fn psadr(&self) -> PSADR_R {
        PSADR_R::new(self.bits & 0x0001_ffff)
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
#[doc = "Data Flash Programming Start Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fpsaddr](index.html) module"]
pub struct FPSADDR_SPEC;
impl crate::RegisterSpec for FPSADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fpsaddr::R](R) reader structure"]
impl crate::Readable for FPSADDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fpsaddr::W](W) writer structure"]
impl crate::Writable for FPSADDR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FPSADDR to value 0"]
impl crate::Resettable for FPSADDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
