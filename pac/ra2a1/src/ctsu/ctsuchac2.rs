#[doc = "Register `CTSUCHAC2` reader"]
pub struct R(crate::R<CTSUCHAC2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTSUCHAC2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTSUCHAC2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTSUCHAC2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTSUCHAC2` writer"]
pub struct W(crate::W<CTSUCHAC2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTSUCHAC2_SPEC>;
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
impl From<crate::W<CTSUCHAC2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTSUCHAC2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CTSUCHAC2` reader - CTSU Channel Enable Control 2.0: Not measurement target1: Measurement targetNote: CTSUCHAC2\\[0\\]
corresponds to TS16 and CTSUCHAC2\\[7\\]
corresponds to TS23."]
pub type CTSUCHAC2_R = crate::FieldReader<u8, CTSUCHAC2_A>;
#[doc = "CTSU Channel Enable Control 2.0: Not measurement target1: Measurement targetNote: CTSUCHAC2\\[0\\]
corresponds to TS16 and CTSUCHAC2\\[7\\]
corresponds to TS23.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CTSUCHAC2_A(u8);
impl From<CTSUCHAC2_A> for u8 {
    #[inline(always)]
    fn from(val: CTSUCHAC2_A) -> Self {
        val.0 as _
    }
}
#[doc = "Field `CTSUCHAC2` writer - CTSU Channel Enable Control 2.0: Not measurement target1: Measurement targetNote: CTSUCHAC2\\[0\\]
corresponds to TS16 and CTSUCHAC2\\[7\\]
corresponds to TS23."]
pub type CTSUCHAC2_W<'a, const O: u8> =
    crate::FieldWriter<'a, u8, CTSUCHAC2_SPEC, u8, CTSUCHAC2_A, 8, O>;
impl R {
    #[doc = "Bits 0:7 - CTSU Channel Enable Control 2.0: Not measurement target1: Measurement targetNote: CTSUCHAC2\\[0\\]
corresponds to TS16 and CTSUCHAC2\\[7\\]
corresponds to TS23."]
    #[inline(always)]
    pub fn ctsuchac2(&self) -> CTSUCHAC2_R {
        CTSUCHAC2_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - CTSU Channel Enable Control 2.0: Not measurement target1: Measurement targetNote: CTSUCHAC2\\[0\\]
corresponds to TS16 and CTSUCHAC2\\[7\\]
corresponds to TS23."]
    #[inline(always)]
    #[must_use]
    pub fn ctsuchac2(&mut self) -> CTSUCHAC2_W<0> {
        CTSUCHAC2_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CTSU Channel Enable Control Register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctsuchac2](index.html) module"]
pub struct CTSUCHAC2_SPEC;
impl crate::RegisterSpec for CTSUCHAC2_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [ctsuchac2::R](R) reader structure"]
impl crate::Readable for CTSUCHAC2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctsuchac2::W](W) writer structure"]
impl crate::Writable for CTSUCHAC2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTSUCHAC2 to value 0"]
impl crate::Resettable for CTSUCHAC2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
