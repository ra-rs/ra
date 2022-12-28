#[doc = "Register `MSPMPUSA` reader"]
pub struct R(crate::R<MSPMPUSA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MSPMPUSA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MSPMPUSA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MSPMPUSA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MSPMPUSA` writer"]
pub struct W(crate::W<MSPMPUSA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MSPMPUSA_SPEC>;
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
impl From<crate::W<MSPMPUSA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MSPMPUSA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MSPMPUSA` reader - Region start address register Address where the region starts, for use in region determination.NOTE: Range: 0x1FF00000-0x200FFFFC The low-order 2 bits are fixed to 0."]
pub type MSPMPUSA_R = crate::FieldReader<u32, u32>;
#[doc = "Field `MSPMPUSA` writer - Region start address register Address where the region starts, for use in region determination.NOTE: Range: 0x1FF00000-0x200FFFFC The low-order 2 bits are fixed to 0."]
pub type MSPMPUSA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MSPMPUSA_SPEC, u32, u32, 30, O>;
impl R {
    #[doc = "Bits 2:31 - Region start address register Address where the region starts, for use in region determination.NOTE: Range: 0x1FF00000-0x200FFFFC The low-order 2 bits are fixed to 0."]
    #[inline(always)]
    pub fn mspmpusa(&self) -> MSPMPUSA_R {
        MSPMPUSA_R::new((self.bits >> 2) & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bits 2:31 - Region start address register Address where the region starts, for use in region determination.NOTE: Range: 0x1FF00000-0x200FFFFC The low-order 2 bits are fixed to 0."]
    #[inline(always)]
    #[must_use]
    pub fn mspmpusa(&mut self) -> MSPMPUSA_W<2> {
        MSPMPUSA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Main Stack Pointer Monitor Start Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mspmpusa](index.html) module"]
pub struct MSPMPUSA_SPEC;
impl crate::RegisterSpec for MSPMPUSA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mspmpusa::R](R) reader structure"]
impl crate::Readable for MSPMPUSA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mspmpusa::W](W) writer structure"]
impl crate::Writable for MSPMPUSA_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MSPMPUSA to value 0"]
impl crate::Resettable for MSPMPUSA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
