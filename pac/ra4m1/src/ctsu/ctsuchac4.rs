#[doc = "Register `CTSUCHAC4` reader"]
pub struct R(crate::R<CTSUCHAC4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTSUCHAC4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTSUCHAC4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTSUCHAC4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTSUCHAC4` writer"]
pub struct W(crate::W<CTSUCHAC4_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTSUCHAC4_SPEC>;
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
impl From<crate::W<CTSUCHAC4_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTSUCHAC4_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CTSUCHAC4` reader - CTSU Channel Enable Control 4. 0: Not measurement target 1: Measurement target Note: CTSUCHAC4\\[0\\]
corresponds to TS32 and CTSUCHAC4\\[3\\]
corresponds to TS35. but the write value of CTSUCHAC0\\[4\\],CTSUCHAC4\\[5\\],CTSUCHAC4\\[6\\],CTSUCHAC4\\[7\\]
should be 0."]
pub type CTSUCHAC4_R = crate::FieldReader<u8, CTSUCHAC4_A>;
#[doc = "CTSU Channel Enable Control 4. 0: Not measurement target 1: Measurement target Note: CTSUCHAC4\\[0\\]
corresponds to TS32 and CTSUCHAC4\\[3\\]
corresponds to TS35. but the write value of CTSUCHAC0\\[4\\],CTSUCHAC4\\[5\\],CTSUCHAC4\\[6\\],CTSUCHAC4\\[7\\]
should be 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CTSUCHAC4_A(u8);
impl From<CTSUCHAC4_A> for u8 {
    #[inline(always)]
    fn from(val: CTSUCHAC4_A) -> Self {
        val.0 as _
    }
}
#[doc = "Field `CTSUCHAC4` writer - CTSU Channel Enable Control 4. 0: Not measurement target 1: Measurement target Note: CTSUCHAC4\\[0\\]
corresponds to TS32 and CTSUCHAC4\\[3\\]
corresponds to TS35. but the write value of CTSUCHAC0\\[4\\],CTSUCHAC4\\[5\\],CTSUCHAC4\\[6\\],CTSUCHAC4\\[7\\]
should be 0."]
pub type CTSUCHAC4_W<'a, const O: u8> =
    crate::FieldWriter<'a, u8, CTSUCHAC4_SPEC, u8, CTSUCHAC4_A, 4, O>;
impl R {
    #[doc = "Bits 0:3 - CTSU Channel Enable Control 4. 0: Not measurement target 1: Measurement target Note: CTSUCHAC4\\[0\\]
corresponds to TS32 and CTSUCHAC4\\[3\\]
corresponds to TS35. but the write value of CTSUCHAC0\\[4\\],CTSUCHAC4\\[5\\],CTSUCHAC4\\[6\\],CTSUCHAC4\\[7\\]
should be 0."]
    #[inline(always)]
    pub fn ctsuchac4(&self) -> CTSUCHAC4_R {
        CTSUCHAC4_R::new(self.bits & 0x0f)
    }
}
impl W {
    #[doc = "Bits 0:3 - CTSU Channel Enable Control 4. 0: Not measurement target 1: Measurement target Note: CTSUCHAC4\\[0\\]
corresponds to TS32 and CTSUCHAC4\\[3\\]
corresponds to TS35. but the write value of CTSUCHAC0\\[4\\],CTSUCHAC4\\[5\\],CTSUCHAC4\\[6\\],CTSUCHAC4\\[7\\]
should be 0."]
    #[inline(always)]
    #[must_use]
    pub fn ctsuchac4(&mut self) -> CTSUCHAC4_W<0> {
        CTSUCHAC4_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CTSU Channel Enable Control Register 4\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctsuchac4](index.html) module"]
pub struct CTSUCHAC4_SPEC;
impl crate::RegisterSpec for CTSUCHAC4_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [ctsuchac4::R](R) reader structure"]
impl crate::Readable for CTSUCHAC4_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctsuchac4::W](W) writer structure"]
impl crate::Writable for CTSUCHAC4_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTSUCHAC4 to value 0"]
impl crate::Resettable for CTSUCHAC4_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
