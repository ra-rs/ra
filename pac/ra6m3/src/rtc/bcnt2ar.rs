#[doc = "Register `BCNT2AR` reader"]
pub struct R(crate::R<BCNT2AR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BCNT2AR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BCNT2AR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BCNT2AR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BCNT2AR` writer"]
pub struct W(crate::W<BCNT2AR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BCNT2AR_SPEC>;
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
impl From<crate::W<BCNT2AR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BCNT2AR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BCNT2AR` reader - The BCNT2AR counter is a readable/writable 32-bit binary counter b23 to b16."]
pub type BCNT2AR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BCNT2AR` writer - The BCNT2AR counter is a readable/writable 32-bit binary counter b23 to b16."]
pub type BCNT2AR_W<'a, const O: u8> = crate::FieldWriter<'a, u8, BCNT2AR_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - The BCNT2AR counter is a readable/writable 32-bit binary counter b23 to b16."]
    #[inline(always)]
    pub fn bcnt2ar(&self) -> BCNT2AR_R {
        BCNT2AR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - The BCNT2AR counter is a readable/writable 32-bit binary counter b23 to b16."]
    #[inline(always)]
    #[must_use]
    pub fn bcnt2ar(&mut self) -> BCNT2AR_W<0> {
        BCNT2AR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Binary Counter 2 Alarm Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bcnt2ar](index.html) module"]
pub struct BCNT2AR_SPEC;
impl crate::RegisterSpec for BCNT2AR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [bcnt2ar::R](R) reader structure"]
impl crate::Readable for BCNT2AR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bcnt2ar::W](W) writer structure"]
impl crate::Writable for BCNT2AR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BCNT2AR to value 0"]
impl crate::Resettable for BCNT2AR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
