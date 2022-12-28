#[doc = "Register `MMPUEB%s` reader"]
pub struct R(crate::R<MMPUEB_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MMPUEB_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MMPUEB_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MMPUEB_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MMPUEB%s` writer"]
pub struct W(crate::W<MMPUEB_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MMPUEB_SPEC>;
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
impl From<crate::W<MMPUEB_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MMPUEB_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MMPUEB` reader - Region end address registerAddress where the region end, for use in region determination.NOTE: The low-order 2 bits are fixed to 1."]
pub type MMPUEB_R = crate::FieldReader<u32, u32>;
#[doc = "Field `MMPUEB` writer - Region end address registerAddress where the region end, for use in region determination.NOTE: The low-order 2 bits are fixed to 1."]
pub type MMPUEB_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MMPUEB_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Region end address registerAddress where the region end, for use in region determination.NOTE: The low-order 2 bits are fixed to 1."]
    #[inline(always)]
    pub fn mmpueb(&self) -> MMPUEB_R {
        MMPUEB_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Region end address registerAddress where the region end, for use in region determination.NOTE: The low-order 2 bits are fixed to 1."]
    #[inline(always)]
    #[must_use]
    pub fn mmpueb(&mut self) -> MMPUEB_W<0> {
        MMPUEB_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Group B Region %s End Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mmpueb](index.html) module"]
pub struct MMPUEB_SPEC;
impl crate::RegisterSpec for MMPUEB_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mmpueb::R](R) reader structure"]
impl crate::Readable for MMPUEB_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mmpueb::W](W) writer structure"]
impl crate::Writable for MMPUEB_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MMPUEB%s to value 0x03"]
impl crate::Resettable for MMPUEB_SPEC {
    const RESET_VALUE: Self::Ux = 0x03;
}
