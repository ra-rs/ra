#[doc = "Register `BCNT1` reader"]
pub struct R(crate::R<BCNT1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BCNT1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BCNT1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BCNT1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BCNT1` writer"]
pub struct W(crate::W<BCNT1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BCNT1_SPEC>;
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
impl From<crate::W<BCNT1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BCNT1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BCNT1` reader - The BCNT1 counter is a readable/writable 32-bit binary counter b15 to b8."]
pub type BCNT1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BCNT1` writer - The BCNT1 counter is a readable/writable 32-bit binary counter b15 to b8."]
pub type BCNT1_W<'a, const O: u8> = crate::FieldWriter<'a, u8, BCNT1_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - The BCNT1 counter is a readable/writable 32-bit binary counter b15 to b8."]
    #[inline(always)]
    pub fn bcnt1(&self) -> BCNT1_R {
        BCNT1_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - The BCNT1 counter is a readable/writable 32-bit binary counter b15 to b8."]
    #[inline(always)]
    #[must_use]
    pub fn bcnt1(&mut self) -> BCNT1_W<0> {
        BCNT1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Binary Counter 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bcnt1](index.html) module"]
pub struct BCNT1_SPEC;
impl crate::RegisterSpec for BCNT1_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [bcnt1::R](R) reader structure"]
impl crate::Readable for BCNT1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bcnt1::W](W) writer structure"]
impl crate::Writable for BCNT1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BCNT1 to value 0"]
impl crate::Resettable for BCNT1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
