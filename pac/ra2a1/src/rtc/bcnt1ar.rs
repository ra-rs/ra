#[doc = "Register `BCNT1AR` reader"]
pub struct R(crate::R<BCNT1AR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BCNT1AR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BCNT1AR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BCNT1AR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BCNT1AR` writer"]
pub struct W(crate::W<BCNT1AR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BCNT1AR_SPEC>;
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
impl From<crate::W<BCNT1AR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BCNT1AR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BCNT1AR` reader - he BCNT1AR counter is a readable/writable alarm register corresponding to 32-bit binary counter b15 to b8."]
pub type BCNT1AR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BCNT1AR` writer - he BCNT1AR counter is a readable/writable alarm register corresponding to 32-bit binary counter b15 to b8."]
pub type BCNT1AR_W<'a, const O: u8> = crate::FieldWriter<'a, u8, BCNT1AR_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - he BCNT1AR counter is a readable/writable alarm register corresponding to 32-bit binary counter b15 to b8."]
    #[inline(always)]
    pub fn bcnt1ar(&self) -> BCNT1AR_R {
        BCNT1AR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - he BCNT1AR counter is a readable/writable alarm register corresponding to 32-bit binary counter b15 to b8."]
    #[inline(always)]
    #[must_use]
    pub fn bcnt1ar(&mut self) -> BCNT1AR_W<0> {
        BCNT1AR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Binary Counter 1 Alarm Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bcnt1ar](index.html) module"]
pub struct BCNT1AR_SPEC;
impl crate::RegisterSpec for BCNT1AR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [bcnt1ar::R](R) reader structure"]
impl crate::Readable for BCNT1AR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bcnt1ar::W](W) writer structure"]
impl crate::Writable for BCNT1AR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BCNT1AR to value 0"]
impl crate::Resettable for BCNT1AR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
