#[doc = "Register `CTSUCHAC0` reader"]
pub struct R(crate::R<CTSUCHAC0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTSUCHAC0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTSUCHAC0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTSUCHAC0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTSUCHAC0` writer"]
pub struct W(crate::W<CTSUCHAC0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTSUCHAC0_SPEC>;
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
impl From<crate::W<CTSUCHAC0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTSUCHAC0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CTSUCHAC0` reader - CTSU Channel Enable Control 0. 0: Not measurement target 1: Measurement target Note: CTSUCHAC0\\[0\\]
corresponds to TS00 and CTSUCHAC0\\[7\\]
corresponds to TS07. but the write value of CTSUCHAC0\\[2\\]
should be 0."]
pub type CTSUCHAC0_R = crate::FieldReader<u8, CTSUCHAC0_A>;
#[doc = "CTSU Channel Enable Control 0. 0: Not measurement target 1: Measurement target Note: CTSUCHAC0\\[0\\]
corresponds to TS00 and CTSUCHAC0\\[7\\]
corresponds to TS07. but the write value of CTSUCHAC0\\[2\\]
should be 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CTSUCHAC0_A(u8);
impl From<CTSUCHAC0_A> for u8 {
    #[inline(always)]
    fn from(val: CTSUCHAC0_A) -> Self {
        val.0 as _
    }
}
#[doc = "Field `CTSUCHAC0` writer - CTSU Channel Enable Control 0. 0: Not measurement target 1: Measurement target Note: CTSUCHAC0\\[0\\]
corresponds to TS00 and CTSUCHAC0\\[7\\]
corresponds to TS07. but the write value of CTSUCHAC0\\[2\\]
should be 0."]
pub type CTSUCHAC0_W<'a, const O: u8> =
    crate::FieldWriter<'a, u8, CTSUCHAC0_SPEC, u8, CTSUCHAC0_A, 8, O>;
impl R {
    #[doc = "Bits 0:7 - CTSU Channel Enable Control 0. 0: Not measurement target 1: Measurement target Note: CTSUCHAC0\\[0\\]
corresponds to TS00 and CTSUCHAC0\\[7\\]
corresponds to TS07. but the write value of CTSUCHAC0\\[2\\]
should be 0."]
    #[inline(always)]
    pub fn ctsuchac0(&self) -> CTSUCHAC0_R {
        CTSUCHAC0_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - CTSU Channel Enable Control 0. 0: Not measurement target 1: Measurement target Note: CTSUCHAC0\\[0\\]
corresponds to TS00 and CTSUCHAC0\\[7\\]
corresponds to TS07. but the write value of CTSUCHAC0\\[2\\]
should be 0."]
    #[inline(always)]
    #[must_use]
    pub fn ctsuchac0(&mut self) -> CTSUCHAC0_W<0> {
        CTSUCHAC0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CTSU Channel Enable Control Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctsuchac0](index.html) module"]
pub struct CTSUCHAC0_SPEC;
impl crate::RegisterSpec for CTSUCHAC0_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [ctsuchac0::R](R) reader structure"]
impl crate::Readable for CTSUCHAC0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctsuchac0::W](W) writer structure"]
impl crate::Writable for CTSUCHAC0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTSUCHAC0 to value 0"]
impl crate::Resettable for CTSUCHAC0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
