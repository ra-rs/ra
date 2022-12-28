#[doc = "Register `BAVLCDT` reader"]
pub struct R(crate::R<BAVLCDT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BAVLCDT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BAVLCDT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BAVLCDT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BAVLCDT` writer"]
pub struct W(crate::W<BAVLCDT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BAVLCDT_SPEC>;
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
impl From<crate::W<BAVLCDT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BAVLCDT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `AVLCYC` reader - Bus Available Condition Detection Cycle"]
pub type AVLCYC_R = crate::FieldReader<u16, u16>;
#[doc = "Field `AVLCYC` writer - Bus Available Condition Detection Cycle"]
pub type AVLCYC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BAVLCDT_SPEC, u16, u16, 9, O>;
impl R {
    #[doc = "Bits 0:8 - Bus Available Condition Detection Cycle"]
    #[inline(always)]
    pub fn avlcyc(&self) -> AVLCYC_R {
        AVLCYC_R::new((self.bits & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:8 - Bus Available Condition Detection Cycle"]
    #[inline(always)]
    #[must_use]
    pub fn avlcyc(&mut self) -> AVLCYC_W<0> {
        AVLCYC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Bus Available Condition Detection Time Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bavlcdt](index.html) module"]
pub struct BAVLCDT_SPEC;
impl crate::RegisterSpec for BAVLCDT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bavlcdt::R](R) reader structure"]
impl crate::Readable for BAVLCDT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bavlcdt::W](W) writer structure"]
impl crate::Writable for BAVLCDT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BAVLCDT to value 0"]
impl crate::Resettable for BAVLCDT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
