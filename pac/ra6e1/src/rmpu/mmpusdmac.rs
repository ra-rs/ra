#[doc = "Register `MMPUSDMAC%s` reader"]
pub struct R(crate::R<MMPUSDMAC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MMPUSDMAC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MMPUSDMAC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MMPUSDMAC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MMPUSDMAC%s` writer"]
pub struct W(crate::W<MMPUSDMAC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MMPUSDMAC_SPEC>;
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
impl From<crate::W<MMPUSDMAC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MMPUSDMAC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MMPUS` reader - Region start address register"]
pub type MMPUS_R = crate::FieldReader<u32, u32>;
#[doc = "Field `MMPUS` writer - Region start address register"]
pub type MMPUS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MMPUSDMAC_SPEC, u32, u32, 27, O>;
impl R {
    #[doc = "Bits 5:31 - Region start address register"]
    #[inline(always)]
    pub fn mmpus(&self) -> MMPUS_R {
        MMPUS_R::new((self.bits >> 5) & 0x07ff_ffff)
    }
}
impl W {
    #[doc = "Bits 5:31 - Region start address register"]
    #[inline(always)]
    #[must_use]
    pub fn mmpus(&mut self) -> MMPUS_W<5> {
        MMPUS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MMPU Start Address Register for DMAC\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mmpusdmac](index.html) module"]
pub struct MMPUSDMAC_SPEC;
impl crate::RegisterSpec for MMPUSDMAC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mmpusdmac::R](R) reader structure"]
impl crate::Readable for MMPUSDMAC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mmpusdmac::W](W) writer structure"]
impl crate::Writable for MMPUSDMAC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MMPUSDMAC%s to value 0"]
impl crate::Resettable for MMPUSDMAC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
