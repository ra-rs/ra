#[doc = "Register `BCNT%sAR` reader"]
pub struct R(crate::R<BCNTAR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BCNTAR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BCNTAR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BCNTAR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BCNT%sAR` writer"]
pub struct W(crate::W<BCNTAR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BCNTAR_SPEC>;
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
impl From<crate::W<BCNTAR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BCNTAR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BCNTAR` reader - Alarm register associated with the 32-bit binary counter"]
pub type BCNTAR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BCNTAR` writer - Alarm register associated with the 32-bit binary counter"]
pub type BCNTAR_W<'a, const O: u8> = crate::FieldWriter<'a, u8, BCNTAR_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Alarm register associated with the 32-bit binary counter"]
    #[inline(always)]
    pub fn bcntar(&self) -> BCNTAR_R {
        BCNTAR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Alarm register associated with the 32-bit binary counter"]
    #[inline(always)]
    #[must_use]
    pub fn bcntar(&mut self) -> BCNTAR_W<0> {
        BCNTAR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Binary Counter %s Alarm Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bcntar](index.html) module"]
pub struct BCNTAR_SPEC;
impl crate::RegisterSpec for BCNTAR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [bcntar::R](R) reader structure"]
impl crate::Readable for BCNTAR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bcntar::W](W) writer structure"]
impl crate::Writable for BCNTAR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BCNT%sAR to value 0"]
impl crate::Resettable for BCNTAR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
