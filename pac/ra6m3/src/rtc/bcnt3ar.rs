#[doc = "Register `BCNT3AR` reader"]
pub struct R(crate::R<BCNT3AR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BCNT3AR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BCNT3AR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BCNT3AR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BCNT3AR` writer"]
pub struct W(crate::W<BCNT3AR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BCNT3AR_SPEC>;
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
impl From<crate::W<BCNT3AR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BCNT3AR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BCNT3AR` reader - The BCNT3AR counter is a readable/writable 32-bit binary counter b31 to b24."]
pub type BCNT3AR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BCNT3AR` writer - The BCNT3AR counter is a readable/writable 32-bit binary counter b31 to b24."]
pub type BCNT3AR_W<'a, const O: u8> = crate::FieldWriter<'a, u8, BCNT3AR_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - The BCNT3AR counter is a readable/writable 32-bit binary counter b31 to b24."]
    #[inline(always)]
    pub fn bcnt3ar(&self) -> BCNT3AR_R {
        BCNT3AR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - The BCNT3AR counter is a readable/writable 32-bit binary counter b31 to b24."]
    #[inline(always)]
    #[must_use]
    pub fn bcnt3ar(&mut self) -> BCNT3AR_W<0> {
        BCNT3AR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Binary Counter 3 Alarm Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bcnt3ar](index.html) module"]
pub struct BCNT3AR_SPEC;
impl crate::RegisterSpec for BCNT3AR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [bcnt3ar::R](R) reader structure"]
impl crate::Readable for BCNT3AR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bcnt3ar::W](W) writer structure"]
impl crate::Writable for BCNT3AR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BCNT3AR to value 0"]
impl crate::Resettable for BCNT3AR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
