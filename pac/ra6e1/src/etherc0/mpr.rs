#[doc = "Register `MPR` reader"]
pub struct R(crate::R<MPR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MPR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MPR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MPR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MPR` writer"]
pub struct W(crate::W<MPR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MPR_SPEC>;
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
impl From<crate::W<MPR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MPR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MP` reader - Manual PAUSE Time Setting"]
pub type MP_R = crate::FieldReader<u16, u16>;
#[doc = "Field `MP` writer - Manual PAUSE Time Setting"]
pub type MP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MPR_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - Manual PAUSE Time Setting"]
    #[inline(always)]
    pub fn mp(&self) -> MP_R {
        MP_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Manual PAUSE Time Setting"]
    #[inline(always)]
    #[must_use]
    pub fn mp(&mut self) -> MP_W<0> {
        MP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Manual PAUSE Frame Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mpr](index.html) module"]
pub struct MPR_SPEC;
impl crate::RegisterSpec for MPR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mpr::R](R) reader structure"]
impl crate::Readable for MPR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mpr::W](W) writer structure"]
impl crate::Writable for MPR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MPR to value 0"]
impl crate::Resettable for MPR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
