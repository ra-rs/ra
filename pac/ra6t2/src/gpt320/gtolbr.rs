#[doc = "Register `GTOLBR` reader"]
pub struct R(crate::R<GTOLBR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GTOLBR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GTOLBR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GTOLBR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GTOLBR` writer"]
pub struct W(crate::W<GTOLBR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GTOLBR_SPEC>;
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
impl From<crate::W<GTOLBR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GTOLBR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `GTIOAB` reader - GTIOA buffer bits"]
pub type GTIOAB_R = crate::FieldReader<u8, u8>;
#[doc = "Field `GTIOAB` writer - GTIOA buffer bits"]
pub type GTIOAB_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GTOLBR_SPEC, u8, u8, 5, O>;
#[doc = "Field `GTIOBB` reader - GTIOB buffer bits"]
pub type GTIOBB_R = crate::FieldReader<u8, u8>;
#[doc = "Field `GTIOBB` writer - GTIOB buffer bits"]
pub type GTIOBB_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GTOLBR_SPEC, u8, u8, 5, O>;
impl R {
    #[doc = "Bits 0:4 - GTIOA buffer bits"]
    #[inline(always)]
    pub fn gtioab(&self) -> GTIOAB_R {
        GTIOAB_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - GTIOB buffer bits"]
    #[inline(always)]
    pub fn gtiobb(&self) -> GTIOBB_R {
        GTIOBB_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - GTIOA buffer bits"]
    #[inline(always)]
    #[must_use]
    pub fn gtioab(&mut self) -> GTIOAB_W<0> {
        GTIOAB_W::new(self)
    }
    #[doc = "Bits 16:20 - GTIOB buffer bits"]
    #[inline(always)]
    #[must_use]
    pub fn gtiobb(&mut self) -> GTIOBB_W<16> {
        GTIOBB_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "General PWM Timer Output Level Buffer Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gtolbr](index.html) module"]
pub struct GTOLBR_SPEC;
impl crate::RegisterSpec for GTOLBR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gtolbr::R](R) reader structure"]
impl crate::Readable for GTOLBR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gtolbr::W](W) writer structure"]
impl crate::Writable for GTOLBR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GTOLBR to value 0"]
impl crate::Resettable for GTOLBR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
