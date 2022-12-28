#[doc = "Register `EXTBR` reader"]
pub struct R(crate::R<EXTBR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EXTBR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EXTBR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EXTBR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EXTBR` writer"]
pub struct W(crate::W<EXTBR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EXTBR_SPEC>;
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
impl From<crate::W<EXTBR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EXTBR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EBRLO` reader - Extended Bit Rate Low-Level Period Open-Drain"]
pub type EBRLO_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EBRLO` writer - Extended Bit Rate Low-Level Period Open-Drain"]
pub type EBRLO_W<'a, const O: u8> = crate::FieldWriter<'a, u32, EXTBR_SPEC, u8, u8, 8, O>;
#[doc = "Field `EBRHO` reader - Extended Bit Rate High-Level Period Open-Drain"]
pub type EBRHO_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EBRHO` writer - Extended Bit Rate High-Level Period Open-Drain"]
pub type EBRHO_W<'a, const O: u8> = crate::FieldWriter<'a, u32, EXTBR_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Extended Bit Rate Low-Level Period Open-Drain"]
    #[inline(always)]
    pub fn ebrlo(&self) -> EBRLO_R {
        EBRLO_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Extended Bit Rate High-Level Period Open-Drain"]
    #[inline(always)]
    pub fn ebrho(&self) -> EBRHO_R {
        EBRHO_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Extended Bit Rate Low-Level Period Open-Drain"]
    #[inline(always)]
    #[must_use]
    pub fn ebrlo(&mut self) -> EBRLO_W<0> {
        EBRLO_W::new(self)
    }
    #[doc = "Bits 8:15 - Extended Bit Rate High-Level Period Open-Drain"]
    #[inline(always)]
    #[must_use]
    pub fn ebrho(&mut self) -> EBRHO_W<8> {
        EBRHO_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Extended Bit Rate Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [extbr](index.html) module"]
pub struct EXTBR_SPEC;
impl crate::RegisterSpec for EXTBR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [extbr::R](R) reader structure"]
impl crate::Readable for EXTBR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [extbr::W](W) writer structure"]
impl crate::Writable for EXTBR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EXTBR to value 0xffff"]
impl crate::Resettable for EXTBR_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff;
}
