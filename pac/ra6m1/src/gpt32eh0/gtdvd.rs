#[doc = "Register `GTDVD` reader"]
pub struct R(crate::R<GTDVD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GTDVD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GTDVD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GTDVD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GTDVD` writer"]
pub struct W(crate::W<GTDVD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GTDVD_SPEC>;
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
impl From<crate::W<GTDVD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GTDVD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `GTDVD` reader - Dead Time Value Register D"]
pub type GTDVD_R = crate::FieldReader<u32, u32>;
#[doc = "Field `GTDVD` writer - Dead Time Value Register D"]
pub type GTDVD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GTDVD_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Dead Time Value Register D"]
    #[inline(always)]
    pub fn gtdvd(&self) -> GTDVD_R {
        GTDVD_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Dead Time Value Register D"]
    #[inline(always)]
    #[must_use]
    pub fn gtdvd(&mut self) -> GTDVD_W<0> {
        GTDVD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "General PWM Timer Dead Time Value Register D\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gtdvd](index.html) module"]
pub struct GTDVD_SPEC;
impl crate::RegisterSpec for GTDVD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gtdvd::R](R) reader structure"]
impl crate::Readable for GTDVD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gtdvd::W](W) writer structure"]
impl crate::Writable for GTDVD_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GTDVD to value 0xffff_ffff"]
impl crate::Resettable for GTDVD_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_ffff;
}
