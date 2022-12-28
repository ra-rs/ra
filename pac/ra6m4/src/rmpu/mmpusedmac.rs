#[doc = "Register `MMPUSEDMAC%s` reader"]
pub struct R(crate::R<MMPUSEDMAC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MMPUSEDMAC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MMPUSEDMAC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MMPUSEDMAC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MMPUSEDMAC%s` writer"]
pub struct W(crate::W<MMPUSEDMAC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MMPUSEDMAC_SPEC>;
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
impl From<crate::W<MMPUSEDMAC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MMPUSEDMAC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MMPUS` reader - Region start address register for EDMAC"]
pub type MMPUS_R = crate::FieldReader<u32, u32>;
#[doc = "Field `MMPUS` writer - Region start address register for EDMAC"]
pub type MMPUS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MMPUSEDMAC_SPEC, u32, u32, 27, O>;
impl R {
    #[doc = "Bits 5:31 - Region start address register for EDMAC"]
    #[inline(always)]
    pub fn mmpus(&self) -> MMPUS_R {
        MMPUS_R::new((self.bits >> 5) & 0x07ff_ffff)
    }
}
impl W {
    #[doc = "Bits 5:31 - Region start address register for EDMAC"]
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
#[doc = "MMPU Start Address Register for EDMAC\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mmpusedmac](index.html) module"]
pub struct MMPUSEDMAC_SPEC;
impl crate::RegisterSpec for MMPUSEDMAC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mmpusedmac::R](R) reader structure"]
impl crate::Readable for MMPUSEDMAC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mmpusedmac::W](W) writer structure"]
impl crate::Writable for MMPUSEDMAC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MMPUSEDMAC%s to value 0"]
impl crate::Resettable for MMPUSEDMAC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
