#[doc = "Register `BCNT%s` reader"]
pub struct R(crate::R<BCNT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BCNT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BCNT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BCNT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BCNT%s` writer"]
pub struct W(crate::W<BCNT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BCNT_SPEC>;
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
impl From<crate::W<BCNT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BCNT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BCNT` reader - Binary Counter"]
pub type BCNT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BCNT` writer - Binary Counter"]
pub type BCNT_W<'a, const O: u8> = crate::FieldWriter<'a, u8, BCNT_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Binary Counter"]
    #[inline(always)]
    pub fn bcnt(&self) -> BCNT_R {
        BCNT_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Binary Counter"]
    #[inline(always)]
    pub fn bcnt(&mut self) -> BCNT_W<0> {
        BCNT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Binary Counter %s\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bcnt](index.html) module"]
pub struct BCNT_SPEC;
impl crate::RegisterSpec for BCNT_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [bcnt::R](R) reader structure"]
impl crate::Readable for BCNT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bcnt::W](W) writer structure"]
impl crate::Writable for BCNT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BCNT%s to value 0"]
impl crate::Resettable for BCNT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
