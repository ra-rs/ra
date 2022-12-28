#[doc = "Register `BRR` reader"]
pub struct R(crate::R<BRR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BRR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BRR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BRR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BRR` writer"]
pub struct W(crate::W<BRR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BRR_SPEC>;
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
impl From<crate::W<BRR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BRR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BRR` reader - BRR is an 8-bit register that adjusts the bit rate."]
pub type BRR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BRR` writer - BRR is an 8-bit register that adjusts the bit rate."]
pub type BRR_W<'a, const O: u8> = crate::FieldWriter<'a, u8, BRR_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - BRR is an 8-bit register that adjusts the bit rate."]
    #[inline(always)]
    pub fn brr(&self) -> BRR_R {
        BRR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - BRR is an 8-bit register that adjusts the bit rate."]
    #[inline(always)]
    #[must_use]
    pub fn brr(&mut self) -> BRR_W<0> {
        BRR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Bit Rate Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [brr](index.html) module"]
pub struct BRR_SPEC;
impl crate::RegisterSpec for BRR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [brr::R](R) reader structure"]
impl crate::Readable for BRR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [brr::W](W) writer structure"]
impl crate::Writable for BRR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BRR to value 0xff"]
impl crate::Resettable for BRR_SPEC {
    const RESET_VALUE: Self::Ux = 0xff;
}
