#[doc = "Register `GTDBU` reader"]
pub struct R(crate::R<GTDBU_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GTDBU_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GTDBU_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GTDBU_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GTDBU` writer"]
pub struct W(crate::W<GTDBU_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GTDBU_SPEC>;
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
impl From<crate::W<GTDBU_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GTDBU_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `GTDVU` reader - Dead Time Buffer Register U"]
pub type GTDVU_R = crate::FieldReader<u32, u32>;
#[doc = "Field `GTDVU` writer - Dead Time Buffer Register U"]
pub type GTDVU_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GTDBU_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Dead Time Buffer Register U"]
    #[inline(always)]
    pub fn gtdvu(&self) -> GTDVU_R {
        GTDVU_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Dead Time Buffer Register U"]
    #[inline(always)]
    #[must_use]
    pub fn gtdvu(&mut self) -> GTDVU_W<0> {
        GTDVU_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "General PWM Timer Dead Time Buffer Register U\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gtdbu](index.html) module"]
pub struct GTDBU_SPEC;
impl crate::RegisterSpec for GTDBU_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gtdbu::R](R) reader structure"]
impl crate::Readable for GTDBU_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gtdbu::W](W) writer structure"]
impl crate::Writable for GTDBU_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GTDBU to value 0xffff_ffff"]
impl crate::Resettable for GTDBU_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_ffff;
}
