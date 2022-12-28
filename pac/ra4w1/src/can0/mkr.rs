#[doc = "Register `MKR[%s]` reader"]
pub struct R(crate::R<MKR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MKR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MKR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MKR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MKR[%s]` writer"]
pub struct W(crate::W<MKR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MKR_SPEC>;
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
impl From<crate::W<MKR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MKR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EID` reader - Extended ID"]
pub type EID_R = crate::FieldReader<u32, u32>;
#[doc = "Field `EID` writer - Extended ID"]
pub type EID_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MKR_SPEC, u32, u32, 18, O>;
#[doc = "Field `SID` reader - Standard ID"]
pub type SID_R = crate::FieldReader<u16, u16>;
#[doc = "Field `SID` writer - Standard ID"]
pub type SID_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MKR_SPEC, u16, u16, 11, O>;
impl R {
    #[doc = "Bits 0:17 - Extended ID"]
    #[inline(always)]
    pub fn eid(&self) -> EID_R {
        EID_R::new(self.bits & 0x0003_ffff)
    }
    #[doc = "Bits 18:28 - Standard ID"]
    #[inline(always)]
    pub fn sid(&self) -> SID_R {
        SID_R::new(((self.bits >> 18) & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:17 - Extended ID"]
    #[inline(always)]
    #[must_use]
    pub fn eid(&mut self) -> EID_W<0> {
        EID_W::new(self)
    }
    #[doc = "Bits 18:28 - Standard ID"]
    #[inline(always)]
    #[must_use]
    pub fn sid(&mut self) -> SID_W<18> {
        SID_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Mask Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mkr](index.html) module"]
pub struct MKR_SPEC;
impl crate::RegisterSpec for MKR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mkr::R](R) reader structure"]
impl crate::Readable for MKR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mkr::W](W) writer structure"]
impl crate::Writable for MKR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MKR[%s]
to value 0"]
impl crate::Resettable for MKR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
