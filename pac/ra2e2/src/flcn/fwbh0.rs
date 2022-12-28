#[doc = "Register `FWBH0` reader"]
pub struct R(crate::R<FWBH0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FWBH0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FWBH0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FWBH0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FWBH0` writer"]
pub struct W(crate::W<FWBH0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FWBH0_SPEC>;
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
impl From<crate::W<FWBH0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FWBH0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WDATA` reader - Flash Write Buffer H0"]
pub type WDATA_R = crate::FieldReader<u16, u16>;
#[doc = "Field `WDATA` writer - Flash Write Buffer H0"]
pub type WDATA_W<'a, const O: u8> = crate::FieldWriter<'a, u16, FWBH0_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - Flash Write Buffer H0"]
    #[inline(always)]
    pub fn wdata(&self) -> WDATA_R {
        WDATA_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - Flash Write Buffer H0"]
    #[inline(always)]
    #[must_use]
    pub fn wdata(&mut self) -> WDATA_W<0> {
        WDATA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Flash Write Buffer Register H0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fwbh0](index.html) module"]
pub struct FWBH0_SPEC;
impl crate::RegisterSpec for FWBH0_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [fwbh0::R](R) reader structure"]
impl crate::Readable for FWBH0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fwbh0::W](W) writer structure"]
impl crate::Writable for FWBH0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FWBH0 to value 0"]
impl crate::Resettable for FWBH0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
