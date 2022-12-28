#[doc = "Register `FEARL` reader"]
pub struct R(crate::R<FEARL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FEARL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FEARL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FEARL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FEARL` writer"]
pub struct W(crate::W<FEARL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FEARL_SPEC>;
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
impl From<crate::W<FEARL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FEARL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FEARL` reader - Flash Processing End Address L"]
pub type FEARL_R = crate::FieldReader<u16, u16>;
#[doc = "Field `FEARL` writer - Flash Processing End Address L"]
pub type FEARL_W<'a, const O: u8> = crate::FieldWriter<'a, u16, FEARL_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - Flash Processing End Address L"]
    #[inline(always)]
    pub fn fearl(&self) -> FEARL_R {
        FEARL_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - Flash Processing End Address L"]
    #[inline(always)]
    #[must_use]
    pub fn fearl(&mut self) -> FEARL_W<0> {
        FEARL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Flash Processing End Address Register L\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fearl](index.html) module"]
pub struct FEARL_SPEC;
impl crate::RegisterSpec for FEARL_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [fearl::R](R) reader structure"]
impl crate::Readable for FEARL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fearl::W](W) writer structure"]
impl crate::Writable for FEARL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FEARL to value 0"]
impl crate::Resettable for FEARL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
