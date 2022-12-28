#[doc = "Register `CDR` reader"]
pub struct R(crate::R<CDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CDR` writer"]
pub struct W(crate::W<CDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CDR_SPEC>;
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
impl From<crate::W<CDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CMPD` reader - Compare Match Data"]
pub type CMPD_R = crate::FieldReader<u16, u16>;
#[doc = "Field `CMPD` writer - Compare Match Data"]
pub type CMPD_W<'a, const O: u8> = crate::FieldWriter<'a, u16, CDR_SPEC, u16, u16, 9, O>;
impl R {
    #[doc = "Bits 0:8 - Compare Match Data"]
    #[inline(always)]
    pub fn cmpd(&self) -> CMPD_R {
        CMPD_R::new(self.bits & 0x01ff)
    }
}
impl W {
    #[doc = "Bits 0:8 - Compare Match Data"]
    #[inline(always)]
    #[must_use]
    pub fn cmpd(&mut self) -> CMPD_W<0> {
        CMPD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Compare Match Data Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cdr](index.html) module"]
pub struct CDR_SPEC;
impl crate::RegisterSpec for CDR_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [cdr::R](R) reader structure"]
impl crate::Readable for CDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cdr::W](W) writer structure"]
impl crate::Writable for CDR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CDR to value 0"]
impl crate::Resettable for CDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
