#[doc = "Register `ADUOFTR%s` reader"]
pub struct R(crate::R<ADUOFTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADUOFTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADUOFTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADUOFTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADUOFTR%s` writer"]
pub struct W(crate::W<ADUOFTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADUOFTR_SPEC>;
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
impl From<crate::W<ADUOFTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADUOFTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `UOFSET` reader - User Offset Table n"]
pub type UOFSET_R = crate::FieldReader<u16, u16>;
#[doc = "Field `UOFSET` writer - User Offset Table n"]
pub type UOFSET_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ADUOFTR_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - User Offset Table n"]
    #[inline(always)]
    pub fn uofset(&self) -> UOFSET_R {
        UOFSET_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - User Offset Table n"]
    #[inline(always)]
    #[must_use]
    pub fn uofset(&mut self) -> UOFSET_W<0> {
        UOFSET_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "User Offset Table Register %s\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [aduoftr](index.html) module"]
pub struct ADUOFTR_SPEC;
impl crate::RegisterSpec for ADUOFTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [aduoftr::R](R) reader structure"]
impl crate::Readable for ADUOFTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [aduoftr::W](W) writer structure"]
impl crate::Writable for ADUOFTR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADUOFTR%s to value 0"]
impl crate::Resettable for ADUOFTR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
