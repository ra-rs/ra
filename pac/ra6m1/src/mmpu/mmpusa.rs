#[doc = "Register `MMPUSA%s` reader"]
pub struct R(crate::R<MMPUSA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MMPUSA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MMPUSA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MMPUSA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MMPUSA%s` writer"]
pub struct W(crate::W<MMPUSA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MMPUSA_SPEC>;
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
impl From<crate::W<MMPUSA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MMPUSA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MMPUSA` reader - Region Stat Address : Address where the region starts, for use in region determination. NOTE: The low-order 2 bits are fixed to 0."]
pub type MMPUSA_R = crate::FieldReader<u32, u32>;
#[doc = "Field `MMPUSA` writer - Region Stat Address : Address where the region starts, for use in region determination. NOTE: The low-order 2 bits are fixed to 0."]
pub type MMPUSA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MMPUSA_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Region Stat Address : Address where the region starts, for use in region determination. NOTE: The low-order 2 bits are fixed to 0."]
    #[inline(always)]
    pub fn mmpusa(&self) -> MMPUSA_R {
        MMPUSA_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Region Stat Address : Address where the region starts, for use in region determination. NOTE: The low-order 2 bits are fixed to 0."]
    #[inline(always)]
    #[must_use]
    pub fn mmpusa(&mut self) -> MMPUSA_W<0> {
        MMPUSA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Group A Region %s Start Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mmpusa](index.html) module"]
pub struct MMPUSA_SPEC;
impl crate::RegisterSpec for MMPUSA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mmpusa::R](R) reader structure"]
impl crate::Readable for MMPUSA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mmpusa::W](W) writer structure"]
impl crate::Writable for MMPUSA_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MMPUSA%s to value 0"]
impl crate::Resettable for MMPUSA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
