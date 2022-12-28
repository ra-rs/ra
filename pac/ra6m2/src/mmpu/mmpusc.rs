#[doc = "Register `MMPUSC%s` reader"]
pub struct R(crate::R<MMPUSC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MMPUSC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MMPUSC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MMPUSC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MMPUSC%s` writer"]
pub struct W(crate::W<MMPUSC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MMPUSC_SPEC>;
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
impl From<crate::W<MMPUSC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MMPUSC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MMPUSC` reader - Address where the region starts, for use in region determination.NOTE: The low-order 2 bits are fixed to 0."]
pub type MMPUSC_R = crate::FieldReader<u32, u32>;
#[doc = "Field `MMPUSC` writer - Address where the region starts, for use in region determination.NOTE: The low-order 2 bits are fixed to 0."]
pub type MMPUSC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MMPUSC_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Address where the region starts, for use in region determination.NOTE: The low-order 2 bits are fixed to 0."]
    #[inline(always)]
    pub fn mmpusc(&self) -> MMPUSC_R {
        MMPUSC_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Address where the region starts, for use in region determination.NOTE: The low-order 2 bits are fixed to 0."]
    #[inline(always)]
    #[must_use]
    pub fn mmpusc(&mut self) -> MMPUSC_W<0> {
        MMPUSC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Group C Region %s Start Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mmpusc](index.html) module"]
pub struct MMPUSC_SPEC;
impl crate::RegisterSpec for MMPUSC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mmpusc::R](R) reader structure"]
impl crate::Readable for MMPUSC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mmpusc::W](W) writer structure"]
impl crate::Writable for MMPUSC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MMPUSC%s to value 0"]
impl crate::Resettable for MMPUSC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
