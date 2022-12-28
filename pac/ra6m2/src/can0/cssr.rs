#[doc = "Register `CSSR` reader"]
pub struct R(crate::R<CSSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CSSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CSSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CSSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CSSR` writer"]
pub struct W(crate::W<CSSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CSSR_SPEC>;
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
impl From<crate::W<CSSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CSSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CSSR` reader - When the value for the channel search is input, the channel number is output to MSSR."]
pub type CSSR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CSSR` writer - When the value for the channel search is input, the channel number is output to MSSR."]
pub type CSSR_W<'a, const O: u8> = crate::FieldWriter<'a, u8, CSSR_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - When the value for the channel search is input, the channel number is output to MSSR."]
    #[inline(always)]
    pub fn cssr(&self) -> CSSR_R {
        CSSR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - When the value for the channel search is input, the channel number is output to MSSR."]
    #[inline(always)]
    #[must_use]
    pub fn cssr(&mut self) -> CSSR_W<0> {
        CSSR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Channel Search Support Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cssr](index.html) module"]
pub struct CSSR_SPEC;
impl crate::RegisterSpec for CSSR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [cssr::R](R) reader structure"]
impl crate::Readable for CSSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cssr::W](W) writer structure"]
impl crate::Writable for CSSR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CSSR to value 0"]
impl crate::Resettable for CSSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
