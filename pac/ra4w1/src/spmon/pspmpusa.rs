#[doc = "Register `PSPMPUSA` reader"]
pub struct R(crate::R<PSPMPUSA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PSPMPUSA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PSPMPUSA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PSPMPUSA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PSPMPUSA` writer"]
pub struct W(crate::W<PSPMPUSA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PSPMPUSA_SPEC>;
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
impl From<crate::W<PSPMPUSA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PSPMPUSA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PSPMPUSA` reader - Region start address register Address where the region starts, for use in region determination.NOTE: Range: 0x1FF00000-0x200FFFFC The low-order 2 bits are fixed to 0."]
pub type PSPMPUSA_R = crate::FieldReader<u32, u32>;
#[doc = "Field `PSPMPUSA` writer - Region start address register Address where the region starts, for use in region determination.NOTE: Range: 0x1FF00000-0x200FFFFC The low-order 2 bits are fixed to 0."]
pub type PSPMPUSA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PSPMPUSA_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Region start address register Address where the region starts, for use in region determination.NOTE: Range: 0x1FF00000-0x200FFFFC The low-order 2 bits are fixed to 0."]
    #[inline(always)]
    pub fn pspmpusa(&self) -> PSPMPUSA_R {
        PSPMPUSA_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Region start address register Address where the region starts, for use in region determination.NOTE: Range: 0x1FF00000-0x200FFFFC The low-order 2 bits are fixed to 0."]
    #[inline(always)]
    #[must_use]
    pub fn pspmpusa(&mut self) -> PSPMPUSA_W<0> {
        PSPMPUSA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Process Stack Pointer (PSP) Monitor Start Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pspmpusa](index.html) module"]
pub struct PSPMPUSA_SPEC;
impl crate::RegisterSpec for PSPMPUSA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pspmpusa::R](R) reader structure"]
impl crate::Readable for PSPMPUSA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pspmpusa::W](W) writer structure"]
impl crate::Writable for PSPMPUSA_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PSPMPUSA to value 0"]
impl crate::Resettable for PSPMPUSA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
