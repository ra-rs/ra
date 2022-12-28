#[doc = "Register `BFRECDT` reader"]
pub struct R(crate::R<BFRECDT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BFRECDT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BFRECDT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BFRECDT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BFRECDT` writer"]
pub struct W(crate::W<BFRECDT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BFRECDT_SPEC>;
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
impl From<crate::W<BFRECDT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BFRECDT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FRECYC` reader - Bus Free Condition Detection Cycle"]
pub type FRECYC_R = crate::FieldReader<u16, u16>;
#[doc = "Field `FRECYC` writer - Bus Free Condition Detection Cycle"]
pub type FRECYC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BFRECDT_SPEC, u16, u16, 9, O>;
impl R {
    #[doc = "Bits 0:8 - Bus Free Condition Detection Cycle"]
    #[inline(always)]
    pub fn frecyc(&self) -> FRECYC_R {
        FRECYC_R::new((self.bits & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:8 - Bus Free Condition Detection Cycle"]
    #[inline(always)]
    #[must_use]
    pub fn frecyc(&mut self) -> FRECYC_W<0> {
        FRECYC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Bus Free Condition Detection Time Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bfrecdt](index.html) module"]
pub struct BFRECDT_SPEC;
impl crate::RegisterSpec for BFRECDT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bfrecdt::R](R) reader structure"]
impl crate::Readable for BFRECDT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bfrecdt::W](W) writer structure"]
impl crate::Writable for BFRECDT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BFRECDT to value 0"]
impl crate::Resettable for BFRECDT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
