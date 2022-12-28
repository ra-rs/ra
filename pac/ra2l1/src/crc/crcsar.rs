#[doc = "Register `CRCSAR` reader"]
pub struct R(crate::R<CRCSAR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CRCSAR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CRCSAR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CRCSAR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CRCSAR` writer"]
pub struct W(crate::W<CRCSAR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CRCSAR_SPEC>;
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
impl From<crate::W<CRCSAR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CRCSAR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CRCSA` reader - Register Snoop Address"]
pub type CRCSA_R = crate::FieldReader<u16, u16>;
#[doc = "Field `CRCSA` writer - Register Snoop Address"]
pub type CRCSA_W<'a, const O: u8> = crate::FieldWriter<'a, u16, CRCSAR_SPEC, u16, u16, 14, O>;
impl R {
    #[doc = "Bits 0:13 - Register Snoop Address"]
    #[inline(always)]
    pub fn crcsa(&self) -> CRCSA_R {
        CRCSA_R::new(self.bits & 0x3fff)
    }
}
impl W {
    #[doc = "Bits 0:13 - Register Snoop Address"]
    #[inline(always)]
    #[must_use]
    pub fn crcsa(&mut self) -> CRCSA_W<0> {
        CRCSA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Snoop Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [crcsar](index.html) module"]
pub struct CRCSAR_SPEC;
impl crate::RegisterSpec for CRCSAR_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [crcsar::R](R) reader structure"]
impl crate::Readable for CRCSAR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [crcsar::W](W) writer structure"]
impl crate::Writable for CRCSAR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CRCSAR to value 0"]
impl crate::Resettable for CRCSAR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
