#[doc = "Register `MMPUEC%s` reader"]
pub struct R(crate::R<MMPUEC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MMPUEC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MMPUEC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MMPUEC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MMPUEC%s` writer"]
pub struct W(crate::W<MMPUEC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MMPUEC_SPEC>;
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
impl From<crate::W<MMPUEC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MMPUEC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MMPUEC` reader - Region end address registerAddress where the region end, for use in region determination.NOTE: The low-order 2 bits are fixed to 1."]
pub type MMPUEC_R = crate::FieldReader<u32, u32>;
#[doc = "Field `MMPUEC` writer - Region end address registerAddress where the region end, for use in region determination.NOTE: The low-order 2 bits are fixed to 1."]
pub type MMPUEC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MMPUEC_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Region end address registerAddress where the region end, for use in region determination.NOTE: The low-order 2 bits are fixed to 1."]
    #[inline(always)]
    pub fn mmpuec(&self) -> MMPUEC_R {
        MMPUEC_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Region end address registerAddress where the region end, for use in region determination.NOTE: The low-order 2 bits are fixed to 1."]
    #[inline(always)]
    #[must_use]
    pub fn mmpuec(&mut self) -> MMPUEC_W<0> {
        MMPUEC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Group C Region %s End Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mmpuec](index.html) module"]
pub struct MMPUEC_SPEC;
impl crate::RegisterSpec for MMPUEC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mmpuec::R](R) reader structure"]
impl crate::Readable for MMPUEC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mmpuec::W](W) writer structure"]
impl crate::Writable for MMPUEC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MMPUEC%s to value 0x03"]
impl crate::Resettable for MMPUEC_SPEC {
    const RESET_VALUE: Self::Ux = 0x03;
}
