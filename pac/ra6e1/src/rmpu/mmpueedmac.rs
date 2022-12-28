#[doc = "Register `MMPUEEDMAC%s` reader"]
pub struct R(crate::R<MMPUEEDMAC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MMPUEEDMAC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MMPUEEDMAC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MMPUEEDMAC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MMPUEEDMAC%s` writer"]
pub struct W(crate::W<MMPUEEDMAC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MMPUEEDMAC_SPEC>;
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
impl From<crate::W<MMPUEEDMAC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MMPUEEDMAC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MMPUE` reader - Region end address register for EDMAC"]
pub type MMPUE_R = crate::FieldReader<u32, u32>;
#[doc = "Field `MMPUE` writer - Region end address register for EDMAC"]
pub type MMPUE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MMPUEEDMAC_SPEC, u32, u32, 27, O>;
impl R {
    #[doc = "Bits 5:31 - Region end address register for EDMAC"]
    #[inline(always)]
    pub fn mmpue(&self) -> MMPUE_R {
        MMPUE_R::new((self.bits >> 5) & 0x07ff_ffff)
    }
}
impl W {
    #[doc = "Bits 5:31 - Region end address register for EDMAC"]
    #[inline(always)]
    #[must_use]
    pub fn mmpue(&mut self) -> MMPUE_W<5> {
        MMPUE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MMPU End Address Register for EDMAC\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mmpueedmac](index.html) module"]
pub struct MMPUEEDMAC_SPEC;
impl crate::RegisterSpec for MMPUEEDMAC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mmpueedmac::R](R) reader structure"]
impl crate::Readable for MMPUEEDMAC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mmpueedmac::W](W) writer structure"]
impl crate::Writable for MMPUEEDMAC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MMPUEEDMAC%s to value 0x1f"]
impl crate::Resettable for MMPUEEDMAC_SPEC {
    const RESET_VALUE: Self::Ux = 0x1f;
}
