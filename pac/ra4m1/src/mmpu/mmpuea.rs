#[doc = "Register `MMPUEA%s` reader"]
pub struct R(crate::R<MMPUEA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MMPUEA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MMPUEA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MMPUEA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MMPUEA%s` writer"]
pub struct W(crate::W<MMPUEA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MMPUEA_SPEC>;
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
impl From<crate::W<MMPUEA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MMPUEA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MMPUEA` reader - Region end address register Address where the region end, for use in region determination. NOTE: The low-order 2 bits are fixed to 1."]
pub type MMPUEA_R = crate::FieldReader<u32, u32>;
#[doc = "Field `MMPUEA` writer - Region end address register Address where the region end, for use in region determination. NOTE: The low-order 2 bits are fixed to 1."]
pub type MMPUEA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MMPUEA_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Region end address register Address where the region end, for use in region determination. NOTE: The low-order 2 bits are fixed to 1."]
    #[inline(always)]
    pub fn mmpuea(&self) -> MMPUEA_R {
        MMPUEA_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Region end address register Address where the region end, for use in region determination. NOTE: The low-order 2 bits are fixed to 1."]
    #[inline(always)]
    #[must_use]
    pub fn mmpuea(&mut self) -> MMPUEA_W<0> {
        MMPUEA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Group A Region %s End Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mmpuea](index.html) module"]
pub struct MMPUEA_SPEC;
impl crate::RegisterSpec for MMPUEA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mmpuea::R](R) reader structure"]
impl crate::Readable for MMPUEA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mmpuea::W](W) writer structure"]
impl crate::Writable for MMPUEA_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MMPUEA%s to value 0x03"]
impl crate::Resettable for MMPUEA_SPEC {
    const RESET_VALUE: Self::Ux = 0x03;
}
